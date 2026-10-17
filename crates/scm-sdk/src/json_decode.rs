//! Owned JSON decoding compatible with Serde's flattened and untagged buffers.
//!
//! With feature unification, serde_json's arbitrary_precision Numbers may emit
//! i128/u128 from Value::deserialize_any, which Serde's intermediate Content
//! visitor cannot buffer. Route only those numeric tokens through the public
//! JSON reader instead. This retains exact opaque numbers without a whole-value
//! serialization or dependence on serde_json's private numeric conversion format.
//! SDK callers retain their existing byte, nesting, node, work and deadline bounds.
//!
//! Literal objects with either reserved Serde protocol key are rejected before
//! parsing or typed decoding. This feature-independent restriction prevents the
//! upstream Value visitor from silently reinterpreting ordinary JSON objects.

use serde::{
    Deserialize, Deserializer,
    de::{
        DeserializeSeed, EnumAccess, Error as _, IntoDeserializer, MapAccess, VariantAccess,
        Visitor,
        value::{MapDeserializer, SeqDeserializer, StringDeserializer},
    },
};
use serde_json::{Error, Value};
use std::fmt;

const MAX_BYTES: usize = 512 * 1024 * 1024;
const MAX_DEPTH: usize = 128;
const MAX_NODES: usize = 1_000_000;
const RESERVED_KEYS: [&[u8]; 2] = [
    b"$serde_json::private::Number",
    b"$serde_json::private::RawValue",
];

/// Reject literal protocol-key objects before serde_json can reinterpret them.
/// This restriction is independent of downstream serde_json feature unification.
pub(crate) fn from_slice(bytes: &[u8]) -> Result<Value, Error> {
    guard_wire(bytes)?;
    serde_json::from_slice(bytes)
}

pub(crate) fn from_str(text: &str) -> Result<Value, Error> {
    from_slice(text.as_bytes())
}

pub(crate) fn from_value<'de, T: Deserialize<'de>>(value: Value) -> Result<T, Error> {
    T::deserialize(OwnedValue(ensure_supported(value)?))
}

pub(crate) fn ensure_supported(value: Value) -> Result<Value, Error> {
    if let Err(error) = guard_value(&value) {
        // A caller-created Value may exceed the parser's depth limit. Rejecting
        // it must not recursively drop an arbitrarily deep tree.
        crate::validation::discard(value);
        return Err(error);
    }
    Ok(value)
}

fn rejected() -> Error {
    Error::custom("JSON representation is unsupported or exceeds limits")
}

fn reserved(key: &str) -> bool {
    RESERVED_KEYS.contains(&key.as_bytes())
}

// Actual JSON keys can spell ASCII characters with \u00XX or ordinary escapes.
// Compare against the two fixed ASCII names without allocating arbitrary keys.
fn encoded_key_matches(mut key: &[u8], expected: &[u8]) -> bool {
    for expected in expected {
        let Some((&first, remaining)) = key.split_first() else {
            return false;
        };
        key = remaining;
        let actual = if first == b'\\' {
            let Some((&escape, remaining)) = key.split_first() else {
                return false;
            };
            key = remaining;
            match escape {
                b'u' => {
                    if key.len() < 4 {
                        return false;
                    }
                    let mut code = 0u16;
                    for digit in &key[..4] {
                        let digit = match digit {
                            b'0'..=b'9' => digit - b'0',
                            b'a'..=b'f' => digit - b'a' + 10,
                            b'A'..=b'F' => digit - b'A' + 10,
                            _ => return false,
                        };
                        code = code * 16 + u16::from(digit);
                    }
                    key = &key[4..];
                    if code > 127 {
                        return false;
                    }
                    code as u8
                }
                b'"' | b'\\' | b'/' => escape,
                _ => return false,
            }
        } else {
            first
        };
        if actual != *expected {
            return false;
        }
    }
    key.is_empty()
}

fn whitespace(byte: u8) -> bool {
    matches!(byte, b' ' | b'\n' | b'\r' | b'\t')
}

// This is a bounded lexical guard, not a second JSON grammar. The public JSON
// parser validates syntax afterward. Every string followed by ':' is a key in
// any valid JSON document, so escaped, duplicate and non-first keys are covered.
fn guard_wire(bytes: &[u8]) -> Result<(), Error> {
    if bytes.len() > MAX_BYTES {
        return Err(rejected());
    }
    let max_work = bytes.len().checked_mul(4).ok_or_else(rejected)?;
    let (mut index, mut depth, mut nodes, mut work) = (0usize, 0usize, 0usize, 0usize);
    while index < bytes.len() {
        let start = index;
        match bytes[index] {
            b'"' => {
                index += 1;
                let content = index;
                while index < bytes.len() && bytes[index] != b'"' {
                    index += if bytes[index] == b'\\' { 2 } else { 1 };
                }
                if index >= bytes.len() {
                    return Err(rejected());
                }
                let key = &bytes[content..index];
                index += 1;
                let mut next = index;
                while next < bytes.len() && whitespace(bytes[next]) {
                    next += 1;
                }
                work = work.checked_add(next - index).ok_or_else(rejected)?;
                if bytes.get(next) == Some(&b':') {
                    // Each ASCII character has at most six encoded bytes.
                    // Longer keys cannot equal either reserved name.
                    if key.len() <= 6 * RESERVED_KEYS[1].len() {
                        work = work.checked_add(key.len() * 2).ok_or_else(rejected)?;
                        if RESERVED_KEYS
                            .iter()
                            .any(|name| encoded_key_matches(key, name))
                        {
                            return Err(rejected());
                        }
                    }
                } else {
                    nodes += 1;
                }
            }
            b'{' | b'[' => {
                depth += 1;
                nodes += 1;
                if depth > MAX_DEPTH {
                    return Err(rejected());
                }
                index += 1;
            }
            b'}' | b']' => {
                depth = depth.checked_sub(1).ok_or_else(rejected)?;
                index += 1;
            }
            b',' | b':' => index += 1,
            byte if whitespace(byte) => index += 1,
            _ => {
                nodes += 1;
                index += 1;
                while index < bytes.len()
                    && !whitespace(bytes[index])
                    && !matches!(bytes[index], b'{' | b'}' | b'[' | b']' | b',' | b':' | b'"')
                {
                    index += 1;
                }
            }
        }
        work = work.checked_add(index - start).ok_or_else(rejected)?;
        if nodes > MAX_NODES || work > max_work {
            return Err(rejected());
        }
    }
    if depth != 0 {
        return Err(rejected());
    }
    Ok(())
}

fn guard_value(value: &Value) -> Result<(), Error> {
    enum Children<'a> {
        Array(std::slice::Iter<'a, Value>),
        Object(serde_json::map::Values<'a>),
    }
    impl<'a> Children<'a> {
        fn next(&mut self) -> Option<&'a Value> {
            match self {
                Self::Array(values) => values.next(),
                Self::Object(values) => values.next(),
            }
        }
    }
    let mut stack = Vec::<Children<'_>>::new();
    let mut next = Some(value);
    let mut nodes = 0usize;
    loop {
        if let Some(value) = next.take() {
            nodes += 1;
            if nodes > MAX_NODES {
                return Err(rejected());
            }
            let children = match value {
                Value::Array(values) => {
                    if values.len() > MAX_NODES - nodes {
                        return Err(rejected());
                    }
                    Some(Children::Array(values.iter()))
                }
                Value::Object(values) => {
                    if values.len() > MAX_NODES - nodes {
                        return Err(rejected());
                    }
                    if values.keys().any(|key| reserved(key)) {
                        return Err(rejected());
                    }
                    Some(Children::Object(values.values()))
                }
                _ => None,
            };
            if let Some(children) = children {
                if stack.len() >= MAX_DEPTH {
                    return Err(rejected());
                }
                stack.push(children);
            }
        }
        while let Some(children) = stack.last_mut() {
            if let Some(value) = children.next() {
                next = Some(value);
                break;
            }
            stack.pop();
        }
        if next.is_none() {
            return Ok(());
        }
    }
}

struct OwnedValue(Value);

impl<'de> IntoDeserializer<'de, Error> for OwnedValue {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self {
        self
    }
}

// Preserve JSON's explicit primitive conversion rules. In particular a typed
// f64/i128 visitor can accept a number directly even when deserialize_any must
// use the JSON reader to avoid Serde's ContentVisitor limitation.
macro_rules! primitive {
    ($($method:ident),+ $(,)?) => {$ (
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
            self.0.$method(visitor)
        }
    )+};
}

impl<'de> Deserializer<'de> for OwnedValue {
    type Error = Error;

    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Value::Null => visitor.visit_unit(),
            Value::Bool(value) => visitor.visit_bool(value),
            Value::String(value) => visitor.visit_string(value),
            Value::Number(number) => {
                if let Some(value) = number.as_u64() {
                    if value != 0 {
                        return visitor.visit_u64(value);
                    }
                } else if let Some(value) = number.as_i64()
                    && value != 0
                {
                    return visitor.visit_i64(value);
                }
                if number.as_i128().is_none() && number.as_u128().is_none() {
                    // Public Number conversion is safe when it cannot issue a
                    // 128-bit visit. In particular, keep existing f64 bits and
                    // exact arbitrary-precision fractional/overflow tokens.
                    return number.deserialize_any(visitor);
                }
                // This is a single numeric token, not a copy of the response.
                // Zero also uses the reader to preserve a possible -0 token.
                // The absolute cap matches the maximum configurable SDK body
                // size; enclosing operation/schema limits normally bound it lower.
                let bytes = crate::engine::serialize_bounded(&number, MAX_BYTES)
                    .map_err(|_| Error::custom("JSON number exceeds limits"))?;
                let mut reader = serde_json::Deserializer::from_reader(bytes.as_ref());
                let value = reader.deserialize_any(visitor)?;
                reader.end()?;
                Ok(value)
            }
            Value::Array(values) => {
                SeqDeserializer::<_, Error>::new(values.into_iter().map(OwnedValue))
                    .deserialize_any(visitor)
            }
            Value::Object(values) => MapDeserializer::<_, Error>::new(
                values
                    .into_iter()
                    .map(|(key, value)| (Key(key), OwnedValue(value))),
            )
            .deserialize_any(visitor),
        }
    }

    primitive!(
        deserialize_bool,
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_i128,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_u128,
        deserialize_f32,
        deserialize_f64,
        deserialize_char,
        deserialize_str,
        deserialize_string,
        deserialize_unit,
        deserialize_identifier,
        deserialize_ignored_any
    );

    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        if self.0.is_null() {
            visitor.visit_none()
        } else {
            visitor.visit_some(self)
        }
    }

    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error> {
        // Delegate newtype protocol selection to the public Value deserializer,
        // including downstream RawValue support, and adapt ordinary newtypes.
        self.0
            .deserialize_newtype_struct(name, NewtypeVisitor(visitor))
    }

    fn deserialize_enum<V: Visitor<'de>>(
        self,
        _name: &'static str,
        _variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        let (name, value) = match self.0 {
            Value::String(name) => (name, None),
            Value::Object(values) if values.len() == 1 => {
                let Some((name, value)) = values.into_iter().next() else {
                    return Err(Error::custom("invalid JSON enum representation"));
                };
                (name, Some(value))
            }
            _ => return Err(Error::custom("invalid JSON enum representation")),
        };
        visitor.visit_enum(EnumValue { name, value })
    }

    serde::forward_to_deserialize_any! {
        bytes byte_buf unit_struct seq tuple tuple_struct map struct
    }
}

struct NewtypeVisitor<V>(V);
impl<'de, V: Visitor<'de>> Visitor<'de> for NewtypeVisitor<V> {
    type Value = V::Value;
    fn expecting(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.expecting(f)
    }
    fn visit_newtype_struct<D: Deserializer<'de>>(self, d: D) -> Result<Self::Value, D::Error> {
        let value = Value::deserialize(d)?;
        self.0
            .visit_newtype_struct(OwnedValue(value))
            .map_err(|_| D::Error::custom("invalid JSON newtype value"))
    }
    fn visit_map<M: MapAccess<'de>>(self, map: M) -> Result<Self::Value, M::Error> {
        self.0.visit_map(map)
    }
}

struct EnumValue {
    name: String,
    value: Option<Value>,
}
impl<'de> EnumAccess<'de> for EnumValue {
    type Error = Error;
    type Variant = EnumPayload;
    fn variant_seed<S: DeserializeSeed<'de>>(
        self,
        seed: S,
    ) -> Result<(S::Value, EnumPayload), Error> {
        let name = seed.deserialize(StringDeserializer::<Error>::new(self.name))?;
        Ok((name, EnumPayload(self.value)))
    }
}
struct EnumPayload(Option<Value>);
impl<'de> VariantAccess<'de> for EnumPayload {
    type Error = Error;
    fn unit_variant(self) -> Result<(), Error> {
        match self.0 {
            None => Ok(()),
            Some(value) => from_value(value),
        }
    }
    fn newtype_variant_seed<S: DeserializeSeed<'de>>(self, seed: S) -> Result<S::Value, Error> {
        seed.deserialize(OwnedValue(
            self.0
                .ok_or_else(|| Error::custom("missing JSON enum payload"))?,
        ))
    }
    fn tuple_variant<V: Visitor<'de>>(self, len: usize, visitor: V) -> Result<V::Value, Error> {
        match self.0 {
            Some(Value::Array(values)) => {
                OwnedValue(Value::Array(values)).deserialize_tuple(len, visitor)
            }
            _ => Err(Error::custom("invalid JSON tuple variant")),
        }
    }
    fn struct_variant<V: Visitor<'de>>(
        self,
        fields: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        match self.0 {
            Some(Value::Object(values)) => {
                OwnedValue(Value::Object(values)).deserialize_struct("", fields, visitor)
            }
            _ => Err(Error::custom("invalid JSON struct variant")),
        }
    }
}

// JSON object keys stay strings for ordinary visitors, while numeric/bool key
// consumers retain serde_json's key grammar. Values use OwnedValue recursively.
struct Key(String);
impl<'de> IntoDeserializer<'de, Error> for Key {
    type Deserializer = Self;
    fn into_deserializer(self) -> Self {
        self
    }
}
macro_rules! numeric_key {
    ($($method:ident),+ $(,)?) => {$ (
        fn $method<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
            if !matches!(self.0.as_bytes().first(), Some(b'0'..=b'9' | b'-'))
                || self.0.bytes().any(|byte| byte.is_ascii_whitespace())
            {
                return Err(Error::custom("invalid numeric JSON key"));
            }
            let mut reader = serde_json::Deserializer::from_reader(self.0.as_bytes());
            let value = reader.$method(visitor)?;
            reader.end()?;
            Ok(value)
        }
    )+};
}
impl<'de> Deserializer<'de> for Key {
    type Error = Error;
    fn deserialize_any<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_string(self.0)
    }
    numeric_key!(
        deserialize_i8,
        deserialize_i16,
        deserialize_i32,
        deserialize_i64,
        deserialize_i128,
        deserialize_u8,
        deserialize_u16,
        deserialize_u32,
        deserialize_u64,
        deserialize_u128,
        deserialize_f32,
        deserialize_f64
    );
    fn deserialize_bool<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        match self.0.as_str() {
            "true" => visitor.visit_bool(true),
            "false" => visitor.visit_bool(false),
            _ => Err(Error::custom("invalid boolean JSON key")),
        }
    }
    fn deserialize_option<V: Visitor<'de>>(self, visitor: V) -> Result<V::Value, Error> {
        visitor.visit_some(self)
    }
    fn deserialize_newtype_struct<V: Visitor<'de>>(
        self,
        _name: &'static str,
        visitor: V,
    ) -> Result<V::Value, Error> {
        visitor.visit_newtype_struct(self)
    }
    fn deserialize_enum<V: Visitor<'de>>(
        self,
        name: &'static str,
        variants: &'static [&'static str],
        visitor: V,
    ) -> Result<V::Value, Error> {
        StringDeserializer::<Error>::new(self.0).deserialize_enum(name, variants, visitor)
    }
    serde::forward_to_deserialize_any! {
        char str string bytes byte_buf unit unit_struct seq tuple tuple_struct
        map struct identifier ignored_any
    }
}

#[cfg(test)]
mod tests;
