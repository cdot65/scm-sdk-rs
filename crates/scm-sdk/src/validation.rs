//! Bounded native validation of the pinned schema graph.
//!
//! Requests retain the source's strict/strip/passthrough policy. Responses preserve
//! additional object fields and unfamiliar enum vocabulary while validating known
//! structure. Schema expressions are data; no JavaScript runtime is involved.

use crate::{Error, ErrorKind, Result, config::RequestOptions, transport::Transport};
use serde::Deserialize;
use serde_json::{Map, Value};
use std::{collections::BTreeMap, sync::OnceLock};

/// Request normalization or tolerant response decoding.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum Mode {
    Request,
    Response,
}

const MAX_DEPTH: usize = 64;

/// Caller-selected validation resource bounds, checked before schema work.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct ValidationLimits {
    pub max_depth: usize,
    pub max_nodes: usize,
    pub max_input_bytes: usize,
    pub max_work: usize,
    pub max_copied_bytes: usize,
}
impl Default for ValidationLimits {
    fn default() -> Self {
        Self {
            max_depth: 64,
            max_nodes: 100_000,
            max_input_bytes: 8 * 1024 * 1024,
            max_work: 1_000_000,
            max_copied_bytes: 32 * 1024 * 1024,
        }
    }
}
impl ValidationLimits {
    pub fn for_bytes(bytes: usize) -> Result<Self> {
        let scale = bytes.div_ceil(8 * 1024 * 1024).max(1);
        let value = Self {
            max_nodes: 100_000usize.saturating_mul(scale).min(1_000_000),
            max_work: 1_000_000usize.saturating_mul(scale).min(16_000_000),
            max_input_bytes: bytes,
            max_copied_bytes: bytes.saturating_mul(4).min(512 * 1024 * 1024),
            ..Self::default()
        };
        value.validate()?;
        Ok(value)
    }
    pub(crate) fn validate(&self) -> Result<()> {
        if !(1..=128).contains(&self.max_depth)
            || !(1..=1_000_000).contains(&self.max_nodes)
            || !(1..=512 * 1024 * 1024).contains(&self.max_input_bytes)
            || !(1..=16_000_000).contains(&self.max_work)
            || self.max_copied_bytes < self.max_input_bytes
            || self.max_copied_bytes > 2 * 1024 * 1024 * 1024
        {
            Err(Error::new(ErrorKind::Configuration))
        } else {
            Ok(())
        }
    }
}

#[derive(Deserialize)]
struct Graph {
    roots: BTreeMap<String, String>,
    nodes: BTreeMap<String, Value>,
}
#[derive(Deserialize)]
struct Semantic {
    binding: String,
    #[serde(default)]
    default: Value,
}
struct Registry {
    graph: Graph,
    semantics: BTreeMap<String, Semantic>,
}
fn registry() -> Result<&'static Registry> {
    static REGISTRY: OnceLock<std::result::Result<Registry, ()>> = OnceLock::new();
    REGISTRY
        .get_or_init(|| {
            let graph: Graph = serde_json::from_str(include_str!("validation/schema_graph.json"))
                .map_err(|_| ())?;
            let semantics =
                serde_json::from_str(include_str!("validation/semantics.json")).map_err(|_| ())?;
            Ok(Registry { graph, semantics })
        })
        .as_ref()
        .map_err(|_| invalid())
}
fn invalid() -> Error {
    Error::new(ErrorKind::Validation)
}
fn limit() -> Error {
    Error::new(ErrorKind::Limit)
}
fn sid<'a>(registry: &'a Registry, name: &'a str) -> Result<&'a str> {
    if registry.graph.nodes.contains_key(name) {
        Ok(name)
    } else {
        registry
            .graph
            .roots
            .get(name)
            .map(String::as_str)
            .ok_or_else(invalid)
    }
}
fn child<'a>(node: &'a Value, key: &str) -> Result<&'a str> {
    node.get(key)
        .and_then(|value| value.get("$ref"))
        .and_then(Value::as_str)
        .ok_or_else(invalid)
}

/// Validate and normalize an explicitly present JSON value.
pub fn parse(schema: &str, input: Value, mode: Mode) -> Result<Value> {
    parse_with_limits(schema, input, mode, &ValidationLimits::default())
}

/// Preserve the source's undefined/default behavior without representing omitted
/// values as JSON null. This is also used by typed request builders.
pub fn parse_optional(schema: &str, input: Option<Value>, mode: Mode) -> Result<Option<Value>> {
    parse_optional_with_limits(schema, input, mode, &ValidationLimits::default())
}
/// Validate with bounds derived from the configured request/response budget.
pub fn parse_with_limits(
    schema: &str,
    input: Value,
    mode: Mode,
    limits: &ValidationLimits,
) -> Result<Value> {
    parse_optional_with_limits(schema, Some(input), mode, limits)?.ok_or_else(invalid)
}
/// Validate an anonymous array contract with one shared resource budget.
pub fn parse_array(schema: &str, input: Value, mode: Mode) -> Result<Value> {
    parse_array_with_limits(schema, input, mode, &ValidationLimits::default())
}
/// Whole-array preflight and normalization; per-row budgets are never reset.
pub fn parse_array_with_limits(
    schema: &str,
    input: Value,
    mode: Mode,
    limits: &ValidationLimits,
) -> Result<Value> {
    if let Err(error) = limits.validate() {
        discard(input);
        return Err(error);
    }
    if let Err(error) = preflight(&input, limits) {
        discard(input);
        return Err(error);
    }
    let registry = registry()?;
    let schema = sid(registry, schema)?;
    let values = input.as_array().ok_or_else(invalid)?;
    let mut context = Context {
        registry,
        mode,
        work: 0,
        copied: 0,
        limits,
    };
    context.tick(0)?;
    let mut result = Vec::with_capacity(values.len());
    for value in values {
        result.push(context.walk(schema, Some(value), 1)?.ok_or_else(invalid)?);
    }
    Ok(Value::Array(result))
}
/// Missing-aware normalization with explicit resource limits.
pub fn parse_optional_with_limits(
    schema: &str,
    input: Option<Value>,
    mode: Mode,
    limits: &ValidationLimits,
) -> Result<Option<Value>> {
    if let Err(error) = limits.validate() {
        if let Some(input) = input {
            discard(input);
        }
        return Err(error);
    }
    if let Some(value) = &input
        && let Err(error) = preflight(value, limits)
    {
        if let Some(value) = input {
            discard(value);
        }
        return Err(error);
    }
    let registry = registry()?;
    let schema = sid(registry, schema)?;
    let mut context = Context {
        registry,
        mode,
        work: 0,
        copied: 0,
        limits,
    };
    context.walk(schema, input.as_ref(), 0)
}

fn preflight(value: &Value, limits: &ValidationLimits) -> Result<()> {
    let mut stack = vec![(value, 0usize)];
    let mut nodes = 0usize;
    let mut bytes = 0usize;
    while let Some((value, depth)) = stack.pop() {
        nodes = nodes.checked_add(1).ok_or_else(limit)?;
        if nodes > limits.max_nodes || depth > limits.max_depth {
            return Err(limit());
        }
        match value {
            Value::String(text) => bytes = bytes.checked_add(text.len()).ok_or_else(limit)?,
            Value::Array(values) => {
                if values.len() > limits.max_nodes.saturating_sub(nodes) {
                    return Err(limit());
                }
                stack.extend(values.iter().map(|value| (value, depth + 1)));
            }
            Value::Object(values) => {
                if values.len() > limits.max_nodes.saturating_sub(nodes) {
                    return Err(limit());
                }
                for (key, value) in values {
                    bytes = bytes.checked_add(key.len()).ok_or_else(limit)?;
                    stack.push((value, depth + 1));
                }
            }
            _ => {}
        }
        if bytes > limits.max_input_bytes {
            return Err(limit());
        }
    }
    Ok(())
}
// An overdeep caller-created Value must not overflow the stack while being dropped.
fn discard(value: Value) {
    let mut values = vec![value];
    while let Some(value) = values.pop() {
        match value {
            Value::Array(children) => values.extend(children),
            Value::Object(children) => values.extend(children.into_values()),
            _ => {}
        }
    }
}

struct Context<'a> {
    registry: &'static Registry,
    limits: &'a ValidationLimits,
    mode: Mode,
    work: usize,
    copied: usize,
}
impl Context<'_> {
    fn tick(&mut self, depth: usize) -> Result<()> {
        self.work = self.work.checked_add(1).ok_or_else(limit)?;
        if depth > self.limits.max_depth || self.work > self.limits.max_work {
            Err(limit())
        } else {
            Ok(())
        }
    }
    fn copy_text(&mut self, text: &str) -> Result<String> {
        self.copied = self.copied.checked_add(text.len()).ok_or_else(limit)?;
        if self.copied > self.limits.max_copied_bytes {
            return Err(limit());
        }
        Ok(text.to_owned())
    }
    fn clone_value(&mut self, value: &Value, depth: usize) -> Result<Value> {
        self.tick(depth)?;
        Ok(match value {
            Value::String(text) => Value::String(self.copy_text(text)?),
            Value::Array(values) => Value::Array(
                values
                    .iter()
                    .map(|value| self.clone_value(value, depth + 1))
                    .collect::<Result<_>>()?,
            ),
            Value::Object(values) => {
                let mut result = Map::new();
                for (key, value) in values {
                    result.insert(self.copy_text(key)?, self.clone_value(value, depth + 1)?);
                }
                Value::Object(result)
            }
            value => value.clone(),
        })
    }
    fn check_finite_json(&mut self, value: &Value, depth: usize) -> Result<()> {
        self.tick(depth)?;
        match value {
            Value::Object(values) => {
                for (key, value) in values {
                    if matches!(key.as_str(), "__proto__" | "constructor" | "prototype") {
                        return Err(invalid());
                    }
                    self.check_finite_json(value, depth + 1)?;
                }
            }
            Value::Array(values) => {
                for value in values {
                    self.check_finite_json(value, depth + 1)?;
                }
            }
            Value::Number(value) if !value.as_f64().is_some_and(f64::is_finite) => {
                return Err(invalid());
            }
            _ => {}
        }
        Ok(())
    }
    fn walk(&mut self, schema: &str, input: Option<&Value>, depth: usize) -> Result<Option<Value>> {
        let registry = self.registry;
        let mut schema = schema;
        let mut input = input;
        let mut effects = Vec::new();
        let mut terminal = None;
        let mut reached_core = false;
        // Source wrappers are traversed iteratively. They do not increase JSON
        // depth or grow the native call stack for optional/nullable/lazy chains.
        for _ in 0..64 {
            // A missing property has no JSON child. If a default supplies one,
            // the next wrapper/core step checks its actual output depth.
            self.tick(if input.is_none() {
                depth.saturating_sub(1)
            } else {
                depth
            })?;
            let node = registry.graph.nodes.get(schema).ok_or_else(invalid)?;
            match node
                .get("kind")
                .and_then(Value::as_str)
                .ok_or_else(invalid)?
            {
                "ZodOptional" if input.is_none() => {
                    terminal = Some(None);
                    reached_core = true;
                    break;
                }
                "ZodNullable" if input.is_some_and(Value::is_null) => {
                    terminal = Some(Some(Value::Null));
                    reached_core = true;
                    break;
                }
                "ZodOptional" | "ZodNullable" => schema = child(node, "inner")?,
                "ZodDefault" => {
                    input = input.or(Some(
                        &registry.semantics.get(schema).ok_or_else(invalid)?.default,
                    ));
                    schema = child(node, "inner")?;
                }
                "ZodLazy" => schema = child(node, "target")?,
                "ZodEffects" => {
                    effects.push(registry.semantics.get(schema).ok_or_else(invalid)?);
                    schema = child(node, "inner")?;
                }
                _ => {
                    reached_core = true;
                    break;
                }
            }
        }
        if !reached_core {
            return Err(limit());
        }
        let node = registry.graph.nodes.get(schema).ok_or_else(invalid)?;
        let kind = node
            .get("kind")
            .and_then(Value::as_str)
            .ok_or_else(invalid)?;
        let mut result = if let Some(value) = terminal {
            value
        } else {
            (|| -> Result<Option<Value>> {
                match kind {
                    "ZodPipeline" => {
                        let first = self.walk(child(node, "input")?, input, depth)?;
                        self.walk(child(node, "output")?, first.as_ref(), depth)
                    }
                    "ZodAny" | "ZodUnknown" => input
                        .map(|value| self.clone_value(value, depth))
                        .transpose(),
                    "ZodNever" | "ZodDate" => Err(invalid()),
                    "ZodObject" => self.object(node, input, depth),
                    "ZodArray" => {
                        let values = input.and_then(Value::as_array).ok_or_else(invalid)?;
                        for (key, exact) in [
                            ("minLength", false),
                            ("maxLength", false),
                            ("exactLength", true),
                        ] {
                            if let Some(length) = node
                                .get(key)
                                .and_then(|value| value.get("value"))
                                .and_then(Value::as_u64)
                            {
                                let length = usize::try_from(length).map_err(|_| invalid())?;
                                if (exact && values.len() != length)
                                    || (key == "minLength" && values.len() < length)
                                    || (key == "maxLength" && values.len() > length)
                                {
                                    return Err(invalid());
                                }
                            }
                        }
                        let mut result = Vec::with_capacity(values.len());
                        for value in values {
                            result.push(
                                self.walk(child(node, "element")?, Some(value), depth + 1)?
                                    .ok_or_else(invalid)?,
                            );
                        }
                        Ok(Some(Value::Array(result)))
                    }
                    "ZodRecord" => {
                        let values = input.and_then(Value::as_object).ok_or_else(invalid)?;
                        let mut result = Map::new();
                        for (key, value) in values {
                            let key_value = Value::String(self.copy_text(key)?);
                            self.walk(child(node, "key")?, Some(&key_value), depth + 1)?;
                            let normalized = self
                                .walk(child(node, "value")?, Some(value), depth + 1)?
                                .ok_or_else(invalid)?;
                            if self.mode == Mode::Response || key != "__proto__" {
                                result.insert(self.copy_text(key)?, normalized);
                            }
                        }
                        Ok(Some(Value::Object(result)))
                    }
                    "ZodUnion" => {
                        let variants = node
                            .get("variants")
                            .and_then(Value::as_array)
                            .ok_or_else(invalid)?;
                        if self.mode == Mode::Response
                            && input.is_some_and(Value::is_string)
                            && variants.iter().all(|variant| {
                                variant
                                    .get("$ref")
                                    .and_then(Value::as_str)
                                    .and_then(|sid| registry.graph.nodes.get(sid))
                                    .is_some_and(|node| {
                                        node["kind"] == "ZodLiteral" && node["value"].is_string()
                                    })
                            })
                        {
                            return input
                                .map(|value| self.clone_value(value, depth))
                                .transpose();
                        }
                        for variant in variants {
                            let variant = variant
                                .get("$ref")
                                .and_then(Value::as_str)
                                .ok_or_else(invalid)?;
                            match self.walk(variant, input, depth) {
                                Ok(value) => return Ok(value),
                                Err(error) if error.kind() == ErrorKind::Limit => {
                                    return Err(error);
                                }
                                Err(_) => {}
                            }
                        }
                        Err(invalid())
                    }
                    "ZodDiscriminatedUnion" => {
                        let values = input.and_then(Value::as_object).ok_or_else(invalid)?;
                        let discriminator = node
                            .get("discriminator")
                            .and_then(Value::as_str)
                            .ok_or_else(invalid)?;
                        let value = values.get(discriminator).ok_or_else(invalid)?;
                        let mapping = node
                            .get("discriminatorMapping")
                            .and_then(Value::as_array)
                            .ok_or_else(invalid)?;
                        let selected = mapping
                            .iter()
                            .find(|mapping| mapping.get("value") == Some(value))
                            .ok_or_else(invalid)?;
                        self.walk(child(selected, "schema")?, input, depth)
                    }
                    "ZodIntersection" => {
                        let left = self.walk(child(node, "left")?, input, depth)?;
                        let right = self.walk(child(node, "right")?, input, depth)?;
                        match (left, right) {
                            (None, None) => Ok(None),
                            (Some(left), Some(right)) => merge(left, right).map(Some),
                            _ => Err(invalid()),
                        }
                    }
                    "ZodEnum" | "ZodNativeEnum" => {
                        let value = input.ok_or_else(invalid)?;
                        let values = node
                            .get("values")
                            .and_then(Value::as_array)
                            .ok_or_else(invalid)?;
                        if values.contains(value)
                            || (self.mode == Mode::Response
                                && ((value.is_string() && values.iter().any(Value::is_string))
                                    || (value.is_number() && values.iter().any(Value::is_number))))
                        {
                            Ok(Some(self.clone_value(value, depth)?))
                        } else {
                            Err(invalid())
                        }
                    }
                    "ZodLiteral" => {
                        let value = input.ok_or_else(invalid)?;
                        if equal(value, node.get("value").ok_or_else(invalid)?) {
                            Ok(Some(self.clone_value(value, depth)?))
                        } else {
                            Err(invalid())
                        }
                    }
                    "ZodString" => {
                        let value = input.and_then(Value::as_str).ok_or_else(invalid)?;
                        check_string(
                            value,
                            node.get("checks")
                                .and_then(Value::as_array)
                                .ok_or_else(invalid)?,
                        )?;
                        Ok(Some(self.clone_value(input.ok_or_else(invalid)?, depth)?))
                    }
                    "ZodNumber" => {
                        let value = input.and_then(Value::as_f64).ok_or_else(invalid)?;
                        check_number(
                            value,
                            node.get("checks")
                                .and_then(Value::as_array)
                                .ok_or_else(invalid)?,
                        )?;
                        Ok(Some(self.clone_value(input.ok_or_else(invalid)?, depth)?))
                    }
                    "ZodBoolean" => {
                        if input.is_some_and(Value::is_boolean) {
                            Ok(input.cloned())
                        } else {
                            Err(invalid())
                        }
                    }
                    "ZodNull" => {
                        if input.is_some_and(Value::is_null) {
                            Ok(Some(Value::Null))
                        } else {
                            Err(invalid())
                        }
                    }
                    _ => Err(invalid()),
                }
            })()?
        };
        for effect in effects.into_iter().rev() {
            if matches!(
                effect.binding.as_str(),
                "finite_json" | "finite_json_object"
            ) {
                let value = result.as_ref().ok_or_else(invalid)?;
                if effect.binding == "finite_json_object" && !value.is_object() {
                    return Err(invalid());
                }
                self.check_finite_json(value, depth)?;
            } else {
                result = Some(apply(&effect.binding, result)?);
            }
        }
        Ok(result)
    }
    fn object(
        &mut self,
        node: &Value,
        input: Option<&Value>,
        depth: usize,
    ) -> Result<Option<Value>> {
        let values = input.and_then(Value::as_object).ok_or_else(invalid)?;
        let properties = node
            .get("properties")
            .and_then(Value::as_object)
            .ok_or_else(invalid)?;
        let mut result = Map::new();
        for (key, property) in properties {
            let schema = child(property, "schema")?;
            if let Some(value) = self.walk(schema, values.get(key), depth + 1)?
                && (self.mode == Mode::Response || key != "__proto__")
            {
                result.insert(self.copy_text(key)?, value);
            }
        }
        let catchall = child(node, "catchall")?;
        let catchall_never = self
            .registry
            .graph
            .nodes
            .get(catchall)
            .is_some_and(|node| node["kind"] == "ZodNever");
        for (key, value) in values {
            if properties.contains_key(key) {
                continue;
            }
            let value = if !catchall_never {
                self.walk(catchall, Some(value), depth + 1)?
                    .ok_or_else(invalid)?
            } else if self.mode == Mode::Response || node["unknownFields"] == "passthrough" {
                self.clone_value(value, depth + 1)?
            } else if node["unknownFields"] == "strict" {
                return Err(invalid());
            } else {
                continue;
            };
            if self.mode == Mode::Response || key != "__proto__" {
                result.insert(self.copy_text(key)?, value);
            }
        }
        Ok(Some(Value::Object(result)))
    }
}

fn equal(left: &Value, right: &Value) -> bool {
    match (left.as_f64(), right.as_f64()) {
        (Some(left), Some(right)) => left == right,
        _ => left == right,
    }
}
fn merge(left: Value, right: Value) -> Result<Value> {
    match (left, right) {
        (Value::Object(mut left), Value::Object(right)) => {
            for (key, right) in right {
                if let Some(old) = left.remove(&key) {
                    left.insert(key, merge(old, right)?);
                } else {
                    left.insert(key, right);
                }
            }
            Ok(Value::Object(left))
        }
        (Value::Array(left), Value::Array(right)) if left.len() == right.len() => Ok(Value::Array(
            left.into_iter()
                .zip(right)
                .map(|(left, right)| merge(left, right))
                .collect::<Result<_>>()?,
        )),
        (left, right) if equal(&left, &right) => Ok(left),
        _ => Err(invalid()),
    }
}

fn js_space(value: char) -> bool {
    matches!(value,'\u{0009}'..='\u{000d}'|'\u{0020}'|'\u{00a0}'|'\u{1680}'|'\u{2000}'..='\u{200a}'|'\u{2028}'|'\u{2029}'|'\u{202f}'|'\u{205f}'|'\u{3000}'|'\u{feff}')
}
fn object(value: &Value) -> Result<&Map<String, Value>> {
    value.as_object().ok_or_else(invalid)
}
fn not_null(map: &Map<String, Value>, key: &str) -> bool {
    map.get(key).is_some_and(|value| !value.is_null())
}
fn apply(binding: &str, value: Option<Value>) -> Result<Value> {
    let value = value.ok_or_else(invalid)?;
    let valid = match binding {
        "native_transport" | "native_cancellation" | "native_binary" => false,
        "nonblank" => value
            .as_str()
            .is_some_and(|value| !value.trim_matches(js_space).is_empty()),
        "chart_bounds" => {
            let value = object(&value)?;
            [("totalUnitsMin", "totalUnitsMax"), ("costMin", "costMax")]
                .into_iter()
                .all(|(lower, upper)| {
                    match (
                        value.get(lower).and_then(Value::as_f64),
                        value.get(upper).and_then(Value::as_f64),
                    ) {
                        (Some(lower), Some(upper)) => lower <= upper,
                        _ => true,
                    }
                })
        }
        "time_order" => {
            let value = object(&value)?;
            match (
                value.get("start_time").and_then(Value::as_str),
                value.get("end_time").and_then(Value::as_str),
            ) {
                (Some(start), Some(end)) => match (timestamp(start), timestamp(end)) {
                    (Some(start), Some(end)) => start <= end,
                    _ => false,
                },
                _ => true,
            }
        }
        "nonempty_object" => value.as_object().is_some_and(|value| !value.is_empty()),
        "secret_create" => {
            let value = object(&value)?;
            let allowed = !(value.get("allow_all_workspaces") == Some(&Value::Bool(true))
                && value.contains_key("allowed_workspaces"));
            let key = match value.get("manager_type").and_then(Value::as_str) {
                Some("aws_sm") => "aws_auth_type",
                Some("azure_kv") => "azure_auth_mode",
                _ => "vault_auth_type",
            };
            allowed
                && value
                    .get("auth_config")
                    .and_then(Value::as_object)
                    .is_some_and(|config| config.contains_key(key))
        }
        "secret_update" => {
            let value = object(&value)?;
            !(value.is_empty()
                || value.get("allow_all_workspaces") == Some(&Value::Bool(true))
                    && value.contains_key("allowed_workspaces"))
        }
        "realtime_type" => value
            .get("type")
            .and_then(Value::as_str)
            .is_some_and(|value| !value.is_empty()),
        "usage_reset_exclusive" => {
            let value = object(&value)?;
            !not_null(value, "periodic_reset") || !not_null(value, "periodic_reset_days")
        }
        "rotation_exclusive" => {
            let value = object(&value)?;
            !not_null(value, "rotation_period") || !not_null(value, "next_rotation_at")
        }
        "guardrail_checks" => {
            let value = object(&value)?;
            value.get("target").and_then(Value::as_str) == Some("mcp_tools")
                || (value.contains_key("checks") && value.contains_key("actions"))
        }
        "profile_selector" => {
            let value = object(&value)?;
            ["profile_id", "profile_name"].into_iter().any(|key| {
                value
                    .get(key)
                    .and_then(Value::as_str)
                    .is_some_and(|value| !value.is_empty())
            })
        }
        "tool_event" => {
            let value = object(&value)?;
            value.contains_key("input") || value.contains_key("output")
        }
        "identity" => true,
        "unwrap_data" => {
            return match value {
                Value::Object(mut values) => values.remove("data").ok_or_else(invalid),
                _ => Err(invalid()),
            };
        }
        "message_object" => {
            return match value {
                Value::String(message) => {
                    let mut values = Map::new();
                    values.insert("message".to_owned(), Value::String(message));
                    Ok(Value::Object(values))
                }
                _ => Err(invalid()),
            };
        }
        _ => false,
    };
    if valid { Ok(value) } else { Err(invalid()) }
}

fn patterns() -> &'static BTreeMap<String, regex::Regex> {
    static PATTERNS: OnceLock<BTreeMap<String, regex::Regex>> = OnceLock::new();
    PATTERNS.get_or_init(||{
        let whitespace=r"[\x{0009}-\x{000d}\x{0020}\x{00a0}\x{1680}\x{2000}-\x{200a}\x{2028}\x{2029}\x{202f}\x{205f}\x{3000}\x{feff}]";
        [r"^[A-Za-z0-9][A-Za-z0-9_-]*$",r"^[^,\s]+__[^,\s]+$",r"^[a-zA-Z0-9_-]+$",r"^\d+$"].into_iter().filter_map(|source|{
            let native=source.replace(r"\d","[0-9]").replace(r"\s",whitespace);
            regex::RegexBuilder::new(&native).size_limit(64*1024).build().ok().map(|regex|(source.to_owned(),regex))
        }).collect()
    })
}
fn email(value: &str) -> bool {
    let Some((local, domain)) = value.split_once('@') else {
        return false;
    };
    let local_character = |byte: u8| byte.is_ascii_alphanumeric() || b"_'+-.".contains(&byte);
    if local.is_empty()
        || local.starts_with('.')
        || local.contains("..")
        || !local.bytes().all(local_character)
        || !local
            .as_bytes()
            .last()
            .is_some_and(|byte| byte.is_ascii_alphanumeric() || b"_+-".contains(byte))
    {
        return false;
    }
    let Some((labels, top_level)) = domain.rsplit_once('.') else {
        return false;
    };
    top_level.len() >= 2
        && top_level.bytes().all(|byte| byte.is_ascii_alphabetic())
        && labels.split('.').all(|part| {
            !part.is_empty()
                && part.as_bytes()[0].is_ascii_alphanumeric()
                && part
                    .bytes()
                    .all(|byte| byte.is_ascii_alphanumeric() || byte == b'-')
        })
}
fn datetime_pattern() -> Option<&'static regex::Regex> {
    static REGEX: OnceLock<Option<regex::Regex>> = OnceLock::new();
    REGEX.get_or_init(||regex::Regex::new(r"^([0-9]{4})-([0-9]{2})-([0-9]{2})T([01][0-9]|2[0-3]):([0-5][0-9])(?::([0-5][0-9])(?:\.([0-9]+))?)?(Z|[+-][0-9]{2}:?[0-9]{2})$").ok()).as_ref()
}
fn datetime(value: &str) -> bool {
    let Some(captures) = datetime_pattern().and_then(|pattern| pattern.captures(value)) else {
        return false;
    };
    let year = captures[1].parse::<i32>().ok();
    let month = captures[2]
        .parse::<u8>()
        .ok()
        .and_then(|value| time::Month::try_from(value).ok());
    let day = captures[3].parse::<u8>().ok();
    match (year, month, day) {
        (Some(year), Some(month), Some(day)) => {
            time::Date::from_calendar_date(year, month, day).is_ok()
        }
        _ => false,
    }
}
fn timestamp(value: &str) -> Option<i128> {
    let captures = datetime_pattern()?.captures(value)?;
    let date = time::Date::from_calendar_date(
        captures[1].parse().ok()?,
        time::Month::try_from(captures[2].parse::<u8>().ok()?).ok()?,
        captures[3].parse().ok()?,
    )
    .ok()?;
    let seconds = captures
        .get(6)
        .map(|value| value.as_str().parse::<u8>())
        .transpose()
        .ok()?
        .unwrap_or(0);
    let millis = captures
        .get(7)
        .map(|value| {
            let mut text = value.as_str().chars().take(3).collect::<String>();
            while text.len() < 3 {
                text.push('0');
            }
            text.parse::<u16>()
        })
        .transpose()
        .ok()?
        .unwrap_or(0);
    let zone = &captures[8];
    let offset = if zone == "Z" {
        time::UtcOffset::UTC
    } else {
        let sign = if zone.starts_with('-') { -1 } else { 1 };
        let digits = zone[1..].replace(':', "");
        time::UtcOffset::from_hms(
            digits[..2].parse::<i8>().ok()? * sign,
            digits[2..].parse::<i8>().ok()? * sign,
            0,
        )
        .ok()?
    };
    Some(
        date.with_hms_milli(
            captures[4].parse().ok()?,
            captures[5].parse().ok()?,
            seconds,
            millis,
        )
        .ok()?
        .assume_offset(offset)
        .unix_timestamp_nanos()
            / 1_000_000,
    )
}
fn check_string(value: &str, checks: &[Value]) -> Result<()> {
    let length = value.encode_utf16().count() as u64;
    for check in checks {
        let valid = match check
            .get("kind")
            .and_then(Value::as_str)
            .ok_or_else(invalid)?
        {
            "min" => length >= check["value"].as_u64().ok_or_else(invalid)?,
            "max" => length <= check["value"].as_u64().ok_or_else(invalid)?,
            "uuid" => {
                let bytes = value.as_bytes();
                bytes.len() == 36
                    && bytes.iter().enumerate().all(|(index, byte)| {
                        if [8, 13, 18, 23].contains(&index) {
                            *byte == b'-'
                        } else {
                            byte.is_ascii_hexdigit()
                        }
                    })
            }
            "email" => email(value),
            "url" => url::Url::parse(value).is_ok(),
            "datetime" => datetime(value),
            "regex" => {
                let source = check
                    .get("regex")
                    .and_then(|value| value.get("$regexp"))
                    .and_then(Value::as_str)
                    .ok_or_else(invalid)?;
                patterns()
                    .get(source)
                    .is_some_and(|pattern| pattern.is_match(value))
            }
            _ => false,
        };
        if !valid {
            return Err(invalid());
        }
    }
    Ok(())
}
fn check_number(value: f64, checks: &[Value]) -> Result<()> {
    if !value.is_finite() {
        return Err(invalid());
    }
    for check in checks {
        let valid = match check
            .get("kind")
            .and_then(Value::as_str)
            .ok_or_else(invalid)?
        {
            "int" => value.fract() == 0.0,
            "finite" => value.is_finite(),
            "min" => {
                let boundary = check["value"].as_f64().ok_or_else(invalid)?;
                if check["inclusive"] == true {
                    value >= boundary
                } else {
                    value > boundary
                }
            }
            "max" => {
                let boundary = check["value"].as_f64().ok_or_else(invalid)?;
                if check["inclusive"] == true {
                    value <= boundary
                } else {
                    value < boundary
                }
            }
            "multipleOf" => multiple_of(value, check["value"].as_f64().ok_or_else(invalid)?),
            _ => false,
        };
        if !valid {
            return Err(invalid());
        }
    }
    Ok(())
}
fn multiple_of(value: f64, step: f64) -> bool {
    fn decimal_count(value: f64) -> usize {
        ryu_js::Buffer::new()
            .format_finite(value)
            .split_once('.')
            .map(|(_, fraction)| fraction.len())
            .unwrap_or(0)
    }
    let decimals = decimal_count(value).max(decimal_count(step));
    if decimals > 100 {
        return false;
    }
    let integer = |value: f64| {
        let text = ryu_js::Buffer::new()
            .format_to_fixed(value, decimals as u8)
            .replace('.', "");
        let prefix = text
            .chars()
            .enumerate()
            .take_while(|(index, value)| {
                value.is_ascii_digit() || (*index == 0 && matches!(value, '-' | '+'))
            })
            .map(|(_, value)| value)
            .collect::<String>();
        prefix.parse::<f64>().ok()
    };
    match (integer(value), integer(step)) {
        (Some(value), Some(step)) => step != 0.0 && value % step == 0.0,
        _ => false,
    }
}

/// A native JavaScript-Date equivalent represented as epoch milliseconds.
#[derive(Clone, Copy, PartialEq, Eq)]
pub struct NativeDate {
    epoch_millis: i64,
}
impl NativeDate {
    pub fn from_epoch_millis(value: i64) -> Result<Self> {
        if value.unsigned_abs() > 8_640_000_000_000_000 {
            Err(invalid())
        } else {
            Ok(Self {
                epoch_millis: value,
            })
        }
    }
    pub fn epoch_millis(&self) -> i64 {
        self.epoch_millis
    }
}
impl std::fmt::Debug for NativeDate {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("NativeDate([REDACTED])")
    }
}
impl serde::Serialize for NativeDate {
    fn serialize<S: serde::Serializer>(&self, _: S) -> std::result::Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom(
            "native date requires a typed adapter",
        ))
    }
}
impl<'de> Deserialize<'de> for NativeDate {
    fn deserialize<D: serde::Deserializer<'de>>(_: D) -> std::result::Result<Self, D::Error> {
        Err(serde::de::Error::custom(
            "native date cannot be decoded from JSON",
        ))
    }
}

/// The canonical native file/Blob input; JSON encoding is rejected.
#[cfg(feature = "models")]
pub use crate::models::BinaryFile as BinaryPart;

fn boundary(schema: &str, expected: &str) -> Result<()> {
    let registry = registry()?;
    let mut schema = sid(registry, schema)?;
    for _ in 0..MAX_DEPTH {
        let node = registry.graph.nodes.get(schema).ok_or_else(invalid)?;
        if expected == "date" && node["kind"] == "ZodDate" {
            return Ok(());
        }
        if let Some(semantic) = registry.semantics.get(schema) {
            return if semantic.binding == expected {
                Ok(())
            } else {
                Err(invalid())
            };
        }
        schema = child(node, "inner")?;
    }
    Err(limit())
}
#[cfg(feature = "models")]
pub fn validate_binary(schema: &str, value: &BinaryPart) -> Result<()> {
    boundary(schema, "native_binary")?;
    value.validate(512 * 1024 * 1024)
}
pub fn validate_transport(schema: &str, _value: &dyn Transport) -> Result<()> {
    boundary(schema, "native_transport")
}
pub fn validate_cancellation(schema: &str, _value: &RequestOptions) -> Result<()> {
    boundary(schema, "native_cancellation")
}
pub fn validate_date(schema: &str, _value: &NativeDate) -> Result<()> {
    boundary(schema, "date")
}

#[cfg(test)]
mod tests;
