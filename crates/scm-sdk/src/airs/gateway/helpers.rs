//! Pure Gateway configuration and payload helpers. No service calls are made.
use super::models::GatewayDottedValueEntry;
pub use crate::constants::{
    AI_GATEWAY_REDACTED, AI_GATEWAY_SECRET_FIELDS, AIGatewaySecretOperation,
    GatewaySecretDirection, GatewaySecretPathSegment, GatewaySecretRule, GatewaySecretRules,
};
use crate::data::{JsonObject, JsonValue, Nullable};
pub use crate::helpers::generate_workspace_scope_name;
use crate::helpers::{MAX_HELPER_BYTES, MAX_HELPER_DEPTH, MAX_HELPER_NODES, bounded_json};
use crate::{Error, ErrorKind, Result};
use serde::Serialize;
use serde_json::Value;
use std::collections::{BTreeMap, HashSet};

#[derive(Clone)]
#[non_exhaustive]
pub struct CustomHostConfigurationOptions {
    pub host: String,
    pub headers: BTreeMap<String, String>,
}
impl CustomHostConfigurationOptions {
    pub fn new(host: impl Into<String>) -> Self {
        Self {
            host: host.into(),
            headers: BTreeMap::new(),
        }
    }
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        self.headers.insert(name.into(), value.into());
        self
    }
}
#[derive(Clone, PartialEq, Serialize)]
pub struct CustomHostConfiguration {
    pub provider_auth_type: CustomHostAuthType,
    pub custom_host: String,
    pub custom_headers: BTreeMap<String, String>,
}
#[derive(Clone, Copy, PartialEq, Eq, Serialize)]
pub enum CustomHostAuthType {
    #[serde(rename = "apiKey")]
    ApiKey,
}
crate::redaction::redacted_debug!(
    CustomHostConfigurationOptions,
    CustomHostConfiguration,
    CustomHostAuthType
);
/// Build the exact provider configuration, retaining the caller's URL spelling.
pub fn custom_host_configuration(
    options: &CustomHostConfigurationOptions,
) -> Result<CustomHostConfiguration> {
    if options.host.len() > crate::config::MAX_URL_BYTES || options.headers.len() > 128 {
        return Err(Error::new(ErrorKind::Limit));
    }
    let url = url::Url::parse(&options.host).map_err(|_| Error::new(ErrorKind::Validation))?;
    if !matches!(url.scheme(), "http" | "https") || url.host_str().is_none() {
        return Err(Error::new(ErrorKind::Validation));
    }
    let mut names = HashSet::new();
    let mut bytes = 0usize;
    for (name, value) in &options.headers {
        if name.len() > 128 || value.len() > 8192 {
            return Err(Error::new(ErrorKind::Limit));
        }
        let header = http::HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| Error::new(ErrorKind::Validation))?;
        http::HeaderValue::from_str(value).map_err(|_| Error::new(ErrorKind::Validation))?;
        if !names.insert(header) {
            return Err(Error::new(ErrorKind::Validation));
        }
        bytes = bytes
            .checked_add(name.len() + value.len())
            .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        if bytes > 16 * 1024 {
            return Err(Error::new(ErrorKind::Limit));
        }
    }
    Ok(CustomHostConfiguration {
        provider_auth_type: CustomHostAuthType::ApiKey,
        custom_host: options.host.clone(),
        custom_headers: options.headers.clone(),
    })
}
/// The standalone path helper rejects unsafe identifiers before constructing a path.
pub fn ai_gw_organisations_auth_settings_path(tsg_id: &str) -> Result<String> {
    super::common::numeric_id(tsg_id)?;
    Ok(format!("/organisations/{tsg_id}/auth-settings"))
}

pub fn redact_ai_gateway_secrets(
    operation: AIGatewaySecretOperation,
    input: &JsonValue,
    direction: Option<GatewaySecretDirection>,
) -> Result<JsonValue> {
    bounded_json(input.as_value())?;
    let rules = AI_GATEWAY_SECRET_FIELDS
        .iter()
        .find(|(name, _)| *name == operation)
        .map(|(_, rules)| rules)
        .ok_or_else(|| Error::new(ErrorKind::Validation))?;
    let mut output = input.as_value().clone();
    let mut work = 1_000_000usize;
    for rule in rules.direction(direction.unwrap_or_default()) {
        redact_path(&mut output, rule.path, &mut work)?;
    }
    Ok(JsonValue::new(output))
}
fn redact_path(value: &mut Value, path: &[&str], work: &mut usize) -> Result<()> {
    *work = work
        .checked_sub(1)
        .ok_or_else(|| Error::new(ErrorKind::Limit))?;
    let Some((segment, rest)) = path.split_first() else {
        *value = Value::String(AI_GATEWAY_REDACTED.into());
        return Ok(());
    };
    match value {
        Value::Array(values) if *segment == "*" => {
            for value in values {
                redact_path(value, rest, work)?;
            }
        }
        Value::Object(values) if *segment == "*" => {
            for value in values.values_mut() {
                redact_path(value, rest, work)?;
            }
        }
        Value::Object(values) => {
            if let Some(value) = values.get_mut(*segment) {
                redact_path(value, rest, work)?;
            }
        }
        _ => {}
    }
    Ok(())
}

#[derive(Clone, PartialEq, Eq, Hash)]
enum Token {
    Property(String),
    Index(usize),
}
fn property(tokens: &mut Vec<Token>, value: &mut String) -> Result<()> {
    if value.is_empty() || matches!(value.as_str(), "__proto__" | "constructor" | "prototype") {
        return Err(Error::new(ErrorKind::Validation));
    }
    tokens.push(Token::Property(std::mem::take(value)));
    Ok(())
}
fn path_tokens(path: &str) -> Result<Vec<Token>> {
    if path.is_empty() {
        return Err(Error::new(ErrorKind::Validation));
    }
    if path.len() > 4096 {
        return Err(Error::new(ErrorKind::Limit));
    }
    let chars = path.chars().collect::<Vec<_>>();
    let mut tokens = Vec::new();
    let mut value = String::new();
    let mut after_dot = false;
    let mut after_index = false;
    let mut i = 0;
    while i < chars.len() {
        let ch = chars[i];
        match ch {
            '\\' => {
                i += 1;
                let escaped = chars
                    .get(i)
                    .copied()
                    .ok_or_else(|| Error::new(ErrorKind::Validation))?;
                if !matches!(escaped, '.' | '[' | ']' | '\\') || after_index {
                    return Err(Error::new(ErrorKind::Validation));
                }
                value.push(escaped);
                after_dot = false;
            }
            '.' => {
                if !value.is_empty() {
                    property(&mut tokens, &mut value)?;
                } else if !after_index {
                    return Err(Error::new(ErrorKind::Validation));
                }
                after_dot = true;
                after_index = false;
            }
            '[' => {
                if after_dot {
                    return Err(Error::new(ErrorKind::Validation));
                }
                if !value.is_empty() {
                    property(&mut tokens, &mut value)?;
                } else if tokens.is_empty() {
                    return Err(Error::new(ErrorKind::Validation));
                }
                let close = (i + 1..chars.len())
                    .find(|at| chars[*at] == ']')
                    .ok_or_else(|| Error::new(ErrorKind::Validation))?;
                let raw = chars[i + 1..close].iter().collect::<String>();
                if raw.is_empty()
                    || !raw.bytes().all(|v| v.is_ascii_digit())
                    || raw.len() > 1 && raw.starts_with('0')
                {
                    return Err(Error::new(ErrorKind::Validation));
                }
                let index = raw
                    .parse::<usize>()
                    .map_err(|_| Error::new(ErrorKind::Limit))?;
                if index >= MAX_HELPER_NODES {
                    return Err(Error::new(ErrorKind::Limit));
                }
                tokens.push(Token::Index(index));
                i = close;
                after_index = true;
                after_dot = false;
            }
            ']' => return Err(Error::new(ErrorKind::Validation)),
            _ => {
                if after_index {
                    return Err(Error::new(ErrorKind::Validation));
                }
                value.push(ch);
                after_dot = false;
            }
        }
        if tokens.len() > MAX_HELPER_DEPTH {
            return Err(Error::new(ErrorKind::Limit));
        }
        i += 1;
    }
    if !value.is_empty() {
        property(&mut tokens, &mut value)?;
    } else if after_dot {
        return Err(Error::new(ErrorKind::Validation));
    }
    if tokens.is_empty() {
        return Err(Error::new(ErrorKind::Validation));
    }
    if tokens.len() > MAX_HELPER_DEPTH {
        return Err(Error::new(ErrorKind::Limit));
    }
    Ok(tokens)
}
enum Tree {
    Scalar(Value),
    Object(BTreeMap<String, Tree>),
    Array(BTreeMap<usize, Tree>),
}
impl Tree {
    fn from_value(value: Value, nodes: &mut usize) -> Result<Self> {
        *nodes = nodes
            .checked_sub(1)
            .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        Ok(match value {
            Value::Object(values) => Self::Object(
                values
                    .into_iter()
                    .map(|(k, v)| Self::from_value(v, nodes).map(|v| (k, v)))
                    .collect::<Result<_>>()?,
            ),
            Value::Array(values) => Self::Array(
                values
                    .into_iter()
                    .enumerate()
                    .map(|(k, v)| Self::from_value(v, nodes).map(|v| (k, v)))
                    .collect::<Result<_>>()?,
            ),
            value => Self::Scalar(value),
        })
    }
    fn assign(
        &mut self,
        path: &[Token],
        value: Tree,
        replace: bool,
        nodes: &mut usize,
    ) -> Result<()> {
        let (first, rest) = path
            .split_first()
            .ok_or_else(|| Error::new(ErrorKind::Validation))?;
        if matches!(first, Token::Index(_)) {
            return self.assign_index(path, value, replace, nodes);
        }
        let (Self::Object(map), Token::Property(key)) = (self, first) else {
            return Err(Error::new(ErrorKind::Validation));
        };
        if rest.is_empty() {
            if !replace && map.contains_key(key) {
                return Err(Error::new(ErrorKind::Validation));
            }
            map.insert(key.clone(), value);
            return Ok(());
        }
        let child = map.entry(key.clone()).or_insert_with(|| {
            if matches!(rest.first(), Some(Token::Index(_))) {
                Tree::Array(BTreeMap::new())
            } else {
                Tree::Object(BTreeMap::new())
            }
        });
        *nodes = nodes
            .checked_sub(1)
            .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        child.assign(rest, value, replace, nodes)
    }
    fn assign_index(
        &mut self,
        path: &[Token],
        value: Tree,
        replace: bool,
        nodes: &mut usize,
    ) -> Result<()> {
        let Self::Array(map) = self else {
            return Err(Error::new(ErrorKind::Validation));
        };
        let (Token::Index(index), rest) = path
            .split_first()
            .ok_or_else(|| Error::new(ErrorKind::Validation))?
        else {
            return Err(Error::new(ErrorKind::Validation));
        };
        if rest.is_empty() {
            if !replace && map.contains_key(index) {
                return Err(Error::new(ErrorKind::Validation));
            }
            map.insert(*index, value);
            return Ok(());
        }
        let child = map.entry(*index).or_insert_with(|| {
            if matches!(rest.first(), Some(Token::Index(_))) {
                Tree::Array(BTreeMap::new())
            } else {
                Tree::Object(BTreeMap::new())
            }
        });
        *nodes = nodes
            .checked_sub(1)
            .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        child.assign(rest, value, replace, nodes)
    }
    fn into_value(self, depth: usize) -> Result<Value> {
        if depth > MAX_HELPER_DEPTH {
            return Err(Error::new(ErrorKind::Limit));
        }
        Ok(match self {
            Self::Scalar(value) => value,
            Self::Object(values) => Value::Object(
                values
                    .into_iter()
                    .map(|(k, v)| v.into_value(depth + 1).map(|v| (k, v)))
                    .collect::<Result<_>>()?,
            ),
            Self::Array(values) => {
                let mut result = Vec::with_capacity(values.len());
                for (index, value) in values {
                    if index != result.len() {
                        return Err(Error::new(ErrorKind::Validation));
                    }
                    result.push(value.into_value(depth + 1)?);
                }
                Value::Array(result)
            }
        })
    }
}
fn normalized_value(input: &Value, remaining: &mut usize) -> Result<Value> {
    bounded_json(input)?;
    let bytes = crate::engine::serialize_bounded(input, *remaining)?;
    *remaining = remaining
        .checked_sub(bytes.len())
        .ok_or_else(|| Error::new(ErrorKind::Limit))?;
    crate::validation::parse(
        "GatewayJsonValueSchema",
        serde_json::from_slice(&bytes).map_err(|_| Error::new(ErrorKind::Validation))?,
        crate::validation::Mode::Request,
    )
}
fn finish_tree(tree: Tree) -> Result<JsonObject> {
    let value = tree.into_value(0)?;
    bounded_json(&value)?;
    let Value::Object(map) = value else {
        return Err(Error::new(ErrorKind::Validation));
    };
    Ok(JsonObject::new(map))
}
pub fn build_dotted_object(entries: &[GatewayDottedValueEntry]) -> Result<JsonObject> {
    if entries.len() > 10_000 {
        return Err(Error::new(ErrorKind::Limit));
    }
    let mut tree = Tree::Object(BTreeMap::new());
    let mut identities = HashSet::new();
    let mut remaining = MAX_HELPER_BYTES;
    let mut nodes = MAX_HELPER_NODES;
    for entry in entries {
        if entry
            .extensions
            .keys()
            .any(|key| matches!(key.as_str(), "path" | "value"))
        {
            return Err(Error::new(ErrorKind::Validation));
        }
        let tokens = path_tokens(&entry.path)?;
        if !identities.insert(tokens.clone()) {
            return Err(Error::new(ErrorKind::Validation));
        }
        remaining = remaining
            .checked_sub(entry.path.len())
            .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        let value = match &entry.value {
            Nullable::Null => Value::Null,
            Nullable::Value(value) => {
                normalized_value(value.as_inner().as_value(), &mut remaining)?
            }
        };
        tree.assign(
            &tokens,
            Tree::from_value(value, &mut nodes)?,
            false,
            &mut nodes,
        )?;
    }
    finish_tree(tree)
}
pub fn set_dotted_value(input: &JsonObject, path: &str, value: &JsonValue) -> Result<JsonObject> {
    let tokens = path_tokens(path)?;
    let mut remaining = MAX_HELPER_BYTES;
    let mut nodes = MAX_HELPER_NODES;
    // Validate each borrowed branch before serializing/cloning the object.
    if input.as_map().len() > MAX_HELPER_NODES {
        return Err(Error::new(ErrorKind::Limit));
    }
    for value in input.as_map().values() {
        bounded_json(value)?;
    }
    let bytes = crate::engine::serialize_bounded(input, remaining)?;
    remaining -= bytes.len();
    let object = crate::validation::parse(
        "GatewayJsonObjectSchema",
        serde_json::from_slice(&bytes).map_err(|_| Error::new(ErrorKind::Validation))?,
        crate::validation::Mode::Request,
    )?;
    let mut tree = Tree::from_value(object, &mut nodes)?;
    let value = Tree::from_value(
        normalized_value(value.as_value(), &mut remaining)?,
        &mut nodes,
    )?;
    tree.assign(&tokens, value, true, &mut nodes)?;
    finish_tree(tree)
}
