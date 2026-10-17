//! Shared model containers with redacted ordinary formatting.
use crate::data::{FiniteNumber, JsonValue};
use serde::{Deserialize, Deserializer, Serialize, Serializer};
use std::{collections::BTreeMap, fmt};

/// Additional wire fields. Ordinary formatting conceals both keys and values.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Extensions<T = JsonValue>(BTreeMap<String, T>);
impl<T> Default for Extensions<T> {
    fn default() -> Self {
        Self(BTreeMap::new())
    }
}
impl<T> Extensions<T> {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn insert(&mut self, key: String, value: T) -> Option<T> {
        self.0.insert(key, value)
    }
    pub fn get(&self, key: &str) -> Option<&T> {
        self.0.get(key)
    }
    pub fn iter(&self) -> impl Iterator<Item = (&String, &T)> {
        self.0.iter()
    }
    pub fn keys(&self) -> impl Iterator<Item = &String> {
        self.0.keys()
    }
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
    pub fn len(&self) -> usize {
        self.0.len()
    }
    pub fn into_map(self) -> BTreeMap<String, T> {
        self.0
    }
}
impl<T> fmt::Debug for Extensions<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Extensions([REDACTED])")
    }
}

/// A typed dictionary; key/value access is explicit and ordinary formatting is redacted.
pub type TypedMap<T> = Extensions<T>;

/// A typed top-level sequence with safe ordinary formatting.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Sequence<T>(Vec<T>);
impl<T> Sequence<T> {
    pub fn new(values: Vec<T>) -> Self {
        Self(values)
    }
    pub fn as_slice(&self) -> &[T] {
        &self.0
    }
    pub fn into_vec(self) -> Vec<T> {
        self.0
    }
    pub fn iter(&self) -> std::slice::Iter<'_, T> {
        self.0.iter()
    }
}
impl<T> From<Vec<T>> for Sequence<T> {
    fn from(value: Vec<T>) -> Self {
        Self(value)
    }
}
impl<T> fmt::Debug for Sequence<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Sequence([REDACTED])")
    }
}

/// A typed extension value or an explicitly unfamiliar wire value.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenValue<T> {
    Known(T),
    Unknown(JsonValue),
}
impl<T> fmt::Debug for OpenValue<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OpenValue([REDACTED])")
    }
}

/// Suggested string vocabulary with preserved future string values.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
pub enum OpenString<T> {
    Known(T),
    Unknown(String),
}
impl<T> fmt::Debug for OpenString<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("OpenString([REDACTED])")
    }
}

/// Validate and normalize a schema before decoding its native model.
pub fn parse<T: serde::de::DeserializeOwned>(
    schema: &str,
    value: serde_json::Value,
    mode: crate::validation::Mode,
) -> crate::Result<T> {
    let value = crate::json_decode::ensure_supported(value)
        .map_err(|_| crate::Error::new(crate::ErrorKind::Validation))?;
    let value = crate::validation::parse(schema, value, mode)?;
    crate::json_decode::from_value(value)
        .map_err(|_| crate::Error::new(crate::ErrorKind::Validation))
}

/// Convert a typed model within the standalone validator's default byte bound.
pub(super) fn wire_value<T: Serialize>(value: &T) -> crate::Result<serde_json::Value> {
    let bytes = crate::engine::serialize_bounded(
        value,
        crate::validation::ValidationLimits::default().max_input_bytes,
    )?;
    crate::json_decode::from_slice(&bytes)
        .map_err(|_| crate::Error::new(crate::ErrorKind::Validation))
}

/// Merge compatible independently normalized intersection representations.
fn merge_intersection(
    left: serde_json::Value,
    right: serde_json::Value,
    depth: usize,
    work: &mut usize,
) -> crate::Result<serde_json::Value> {
    use serde_json::Value;
    if depth > crate::validation::ValidationLimits::default().max_depth || *work == 0 {
        return Err(crate::Error::new(crate::ErrorKind::Limit));
    }
    *work -= 1;
    match (left, right) {
        (Value::Object(mut left), Value::Object(right)) => {
            for (key, right) in right {
                let value = if let Some(old) = left.remove(&key) {
                    merge_intersection(old, right, depth + 1, work)?
                } else {
                    right
                };
                left.insert(key, value);
            }
            Ok(Value::Object(left))
        }
        (Value::Array(left), Value::Array(right)) if left.len() == right.len() => Ok(Value::Array(
            left.into_iter()
                .zip(right)
                .map(|(left, right)| merge_intersection(left, right, depth + 1, work))
                .collect::<crate::Result<_>>()?,
        )),
        (left, right)
            if match (left.as_f64(), right.as_f64()) {
                (Some(left), Some(right)) => left == right,
                _ => left == right,
            } =>
        {
            Ok(left)
        }
        _ => Err(crate::Error::new(crate::ErrorKind::Validation)),
    }
}

fn bound_intersection_value(
    value: &serde_json::Value,
    limits: &crate::validation::ValidationLimits,
) -> crate::Result<()> {
    use serde_json::Value;
    let mut pending = vec![(value, 0usize)];
    let mut nodes = 0usize;
    while let Some((value, depth)) = pending.pop() {
        nodes += 1;
        if depth > limits.max_depth || nodes > limits.max_nodes {
            return Err(crate::Error::new(crate::ErrorKind::Limit));
        }
        match value {
            Value::Array(values) => {
                if nodes
                    .saturating_add(pending.len())
                    .saturating_add(values.len())
                    > limits.max_nodes
                {
                    return Err(crate::Error::new(crate::ErrorKind::Limit));
                }
                pending.extend(values.iter().map(|value| (value, depth + 1)));
            }
            Value::Object(values) => {
                if nodes
                    .saturating_add(pending.len())
                    .saturating_add(values.len())
                    > limits.max_nodes
                {
                    return Err(crate::Error::new(crate::ErrorKind::Limit));
                }
                pending.extend(values.values().map(|value| (value, depth + 1)));
            }
            Value::Number(value) if value.as_f64().is_none_or(|value| !value.is_finite()) => {
                return Err(crate::Error::new(crate::ErrorKind::Validation));
            }
            _ => {}
        }
    }
    Ok(())
}
fn intersection_wire<A: Serialize, B: Serialize>(
    left: &A,
    right: &B,
) -> crate::Result<serde_json::Value> {
    let left = wire_value(left)?;
    let right = wire_value(right)?;
    let limits = crate::validation::ValidationLimits::default();
    bound_intersection_value(&left, &limits)?;
    bound_intersection_value(&right, &limits)?;
    let mut work = limits.max_work;
    let merged = merge_intersection(left, right, 0, &mut work)?;
    bound_intersection_value(&merged, &limits)?;
    // Independently bounded halves can merge into a larger object.
    wire_value(&merged)
}

/// A wire value satisfying both halves of an intersection. Compatible objects
/// merge fields recursively; arrays merge elementwise and must have equal length.
#[derive(Clone, PartialEq)]
pub struct Intersection<A, B> {
    left: A,
    right: B,
}
impl<A, B> Intersection<A, B> {
    pub fn left(&self) -> &A {
        &self.left
    }
    pub fn right(&self) -> &B {
        &self.right
    }
}
impl<A: Serialize, B: Serialize> Intersection<A, B> {
    pub fn new(left: A, right: B) -> crate::Result<Self> {
        intersection_wire(&left, &right)?;
        Ok(Self { left, right })
    }
}
impl<A: Serialize, B: Serialize> Serialize for Intersection<A, B> {
    fn serialize<S: Serializer>(&self, s: S) -> Result<S::Ok, S::Error> {
        intersection_wire(&self.left, &self.right)
            .map_err(|_| {
                serde::ser::Error::custom("intersection representations conflict or exceed limits")
            })?
            .serialize(s)
    }
}
impl<'de, A: serde::de::DeserializeOwned + Serialize, B: serde::de::DeserializeOwned + Serialize>
    Deserialize<'de> for Intersection<A, B>
{
    fn deserialize<D: Deserializer<'de>>(d: D) -> Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        let value = wire_value(&value)
            .map_err(|_| serde::de::Error::custom("intersection input exceeds limits"))?;
        bound_intersection_value(&value, &crate::validation::ValidationLimits::default())
            .map_err(|_| serde::de::Error::custom("intersection input exceeds limits"))?;
        let left = crate::json_decode::from_value(value.clone())
            .map_err(|_| serde::de::Error::custom("left intersection invalid"))?;
        let right = crate::json_decode::from_value(value)
            .map_err(|_| serde::de::Error::custom("right intersection invalid"))?;
        Self::new(left, right).map_err(|_| {
            serde::de::Error::custom("intersection representations conflict or exceed limits")
        })
    }
}
impl<A, B> fmt::Debug for Intersection<A, B> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Intersection([REDACTED])")
    }
}

/// A contract with no accepted JSON values.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
pub enum Never {}
impl fmt::Debug for Never {
    fn fmt(&self, _: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {}
    }
}

/// Owned binary input. JSON encoding is deliberately rejected; multipart adapters own the wire representation.
#[derive(Clone, PartialEq)]
pub struct BinaryFile {
    bytes: bytes::Bytes,
    filename: Option<String>,
    content_type: Option<String>,
}
impl BinaryFile {
    pub fn new(bytes: impl Into<bytes::Bytes>) -> crate::Result<Self> {
        Ok(Self {
            bytes: bytes.into(),
            filename: None,
            content_type: None,
        })
    }
    pub fn with_filename(mut self, name: impl Into<String>) -> crate::Result<Self> {
        let name = name.into();
        if name.is_empty()
            || name.len() > 255
            || name == "."
            || name == ".."
            || name
                .chars()
                .any(|c| c.is_control() || matches!(c, '/' | '\\' | '"'))
        {
            return Err(crate::Error::new(crate::ErrorKind::Validation));
        }
        self.filename = Some(name);
        Ok(self)
    }
    pub fn with_content_type(mut self, content_type: impl Into<String>) -> crate::Result<Self> {
        let content_type = content_type.into();
        if content_type.len() > 255
            || !content_type.is_ascii()
            || content_type.bytes().any(|c| c < 32 || c == 127)
            || content_type.parse::<mime::Mime>().is_err()
        {
            return Err(crate::Error::new(crate::ErrorKind::Validation));
        }
        self.content_type = Some(content_type);
        Ok(self)
    }
    pub fn bytes(&self) -> &bytes::Bytes {
        &self.bytes
    }
    pub fn filename(&self) -> Option<&str> {
        self.filename.as_deref()
    }
    pub fn content_type(&self) -> Option<&str> {
        self.content_type.as_deref()
    }
    pub fn validate(&self, max_bytes: usize) -> crate::Result<()> {
        if self.bytes.len() > max_bytes {
            Err(crate::Error::new(crate::ErrorKind::Limit))
        } else {
            Ok(())
        }
    }
}
impl fmt::Debug for BinaryFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BinaryFile([REDACTED])")
    }
}
impl Serialize for BinaryFile {
    fn serialize<S: Serializer>(&self, _: S) -> Result<S::Ok, S::Error> {
        Err(serde::ser::Error::custom(
            "binary input requires a multipart adapter",
        ))
    }
}
impl<'de> Deserialize<'de> for BinaryFile {
    fn deserialize<D: Deserializer<'de>>(_: D) -> Result<Self, D::Error> {
        Err(serde::de::Error::custom("binary input is not a JSON value"))
    }
}

/// An arbitrary finite JSON number's model alias.
pub type Number = FiniteNumber;
