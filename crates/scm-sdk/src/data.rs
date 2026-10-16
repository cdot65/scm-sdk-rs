//! Explicit wire presence and finite JSON numbers with safe ordinary formatting.

use crate::{Error, ErrorKind, Result};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de::Error as _};
use serde_json::{Number, Value};
use std::fmt;

/// An omitted or present nonnull field. Omission requires serde(default) and
/// skip_serializing_if = "Optional::is_unset" on the containing field.
#[derive(Clone, PartialEq, Eq, Default)]
pub enum Optional<T> {
    #[default]
    Unset,
    Value(T),
}
impl<T> Optional<T> {
    pub fn is_unset(&self) -> bool {
        matches!(self, Self::Unset)
    }
    pub fn as_ref(&self) -> Optional<&T> {
        match self {
            Self::Unset => Optional::Unset,
            Self::Value(value) => Optional::Value(value),
        }
    }
    pub fn into_option(self) -> Option<T> {
        match self {
            Self::Unset => None,
            Self::Value(value) => Some(value),
        }
    }
}
impl<T> From<T> for Optional<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}
impl<T> fmt::Debug for Optional<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.is_unset() {
            "Optional::Unset"
        } else {
            "Optional::Value([REDACTED])"
        })
    }
}
impl<T: Serialize> Serialize for Optional<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::Unset => Err(serde::ser::Error::custom(
                "omitted field must be skipped by its container",
            )),
            Self::Value(value) => value.serialize(serializer),
        }
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Optional<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        if value.is_null() {
            return Err(D::Error::custom("nonnull field cannot be null"));
        }
        T::deserialize(value)
            .map(Self::Value)
            .map_err(|_| D::Error::custom("field value invalid"))
    }
}

/// A required field whose present value can be null. No Default implementation:
/// missing fields are rejected even though an explicit JSON null is accepted.
#[derive(Clone, PartialEq, Eq)]
pub enum Nullable<T> {
    Null,
    Value(T),
}
impl<T> Nullable<T> {
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
    pub fn as_ref(&self) -> Nullable<&T> {
        match self {
            Self::Null => Nullable::Null,
            Self::Value(value) => Nullable::Value(value),
        }
    }
}
impl<T> From<T> for Nullable<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}
impl<T> fmt::Debug for Nullable<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(if self.is_null() {
            "Nullable::Null"
        } else {
            "Nullable::Value([REDACTED])"
        })
    }
}
impl<T: Serialize> Serialize for Nullable<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::Null => serializer.serialize_none(),
            Self::Value(value) => value.serialize(serializer),
        }
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Nullable<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        if value.is_null() {
            Ok(Self::Null)
        } else {
            T::deserialize(value)
                .map(Self::Value)
                .map_err(|_| D::Error::custom("field value invalid"))
        }
    }
}

/// A patch field distinguishing omission, explicit clearing, and replacement.
#[derive(Clone, PartialEq, Eq, Default)]
pub enum Patch<T> {
    #[default]
    Unset,
    Null,
    Value(T),
}
impl<T> Patch<T> {
    pub fn is_unset(&self) -> bool {
        matches!(self, Self::Unset)
    }
    pub fn is_null(&self) -> bool {
        matches!(self, Self::Null)
    }
    pub fn as_ref(&self) -> Patch<&T> {
        match self {
            Self::Unset => Patch::Unset,
            Self::Null => Patch::Null,
            Self::Value(value) => Patch::Value(value),
        }
    }
}
impl<T> From<T> for Patch<T> {
    fn from(value: T) -> Self {
        Self::Value(value)
    }
}
impl<T> fmt::Debug for Patch<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str(match self {
            Self::Unset => "Patch::Unset",
            Self::Null => "Patch::Null",
            Self::Value(_) => "Patch::Value([REDACTED])",
        })
    }
}
impl<T: Serialize> Serialize for Patch<T> {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::Unset => Err(serde::ser::Error::custom(
                "omitted field must be skipped by its container",
            )),
            Self::Null => serializer.serialize_none(),
            Self::Value(value) => value.serialize(serializer),
        }
    }
}
impl<'de, T: Deserialize<'de>> Deserialize<'de> for Patch<T> {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        let value = Value::deserialize(deserializer)?;
        if value.is_null() {
            Ok(Self::Null)
        } else {
            T::deserialize(value)
                .map(Self::Value)
                .map_err(|_| D::Error::custom("field value invalid"))
        }
    }
}

/// A finite JSON number. Integer constraints belong to the schema validator.
#[derive(Clone, PartialEq, Eq)]
pub struct FiniteNumber(Number);
impl FiniteNumber {
    pub fn as_f64(&self) -> Option<f64> {
        self.0.as_f64()
    }
    pub fn as_i64(&self) -> Option<i64> {
        self.0.as_i64()
    }
    pub fn as_u64(&self) -> Option<u64> {
        self.0.as_u64()
    }
    pub fn is_i64(&self) -> bool {
        self.0.is_i64()
    }
    pub fn is_u64(&self) -> bool {
        self.0.is_u64()
    }
    pub fn is_f64(&self) -> bool {
        self.0.is_f64()
    }
    pub fn as_number(&self) -> &Number {
        &self.0
    }
    pub fn into_number(self) -> Number {
        self.0
    }
}
impl fmt::Debug for FiniteNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FiniteNumber([REDACTED])")
    }
}
impl Serialize for FiniteNumber {
    fn serialize<S: Serializer>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error> {
        self.0.serialize(serializer)
    }
}
impl<'de> Deserialize<'de> for FiniteNumber {
    fn deserialize<D: Deserializer<'de>>(deserializer: D) -> std::result::Result<Self, D::Error> {
        Self::try_from(Number::deserialize(deserializer)?)
            .map_err(|_| D::Error::custom("number must be finite"))
    }
}
impl TryFrom<f64> for FiniteNumber {
    type Error = Error;
    fn try_from(value: f64) -> Result<Self> {
        Number::from_f64(value)
            .map(Self)
            .ok_or_else(|| Error::new(ErrorKind::Validation))
    }
}
impl TryFrom<f32> for FiniteNumber {
    type Error = Error;
    fn try_from(value: f32) -> Result<Self> {
        Self::try_from(f64::from(value))
    }
}
impl TryFrom<Number> for FiniteNumber {
    type Error = Error;
    fn try_from(value: Number) -> Result<Self> {
        if value.as_f64().is_some_and(f64::is_finite) {
            Ok(Self(value))
        } else {
            Err(Error::new(ErrorKind::Validation))
        }
    }
}
macro_rules! integer {($($ty:ty),*)=>{$(impl From<$ty> for FiniteNumber{fn from(value:$ty)->Self{Self(Number::from(value))}})*};}
integer!(i8, i16, i32, i64, u8, u16, u32, u64, isize, usize);

/// Faithful arbitrary JSON whose ordinary formatting excludes its contents.
/// Schema validation decides where extension values and prototype keys are allowed.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(transparent)]
pub struct JsonValue(Value);
impl JsonValue {
    pub fn new(value: Value) -> Self {
        Self(value)
    }
    pub fn as_value(&self) -> &Value {
        &self.0
    }
    pub fn into_value(self) -> Value {
        self.0
    }
}
impl From<Value> for JsonValue {
    fn from(value: Value) -> Self {
        Self(value)
    }
}
impl From<JsonValue> for Value {
    fn from(value: JsonValue) -> Self {
        value.0
    }
}
impl fmt::Debug for JsonValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JsonValue([REDACTED])")
    }
}

/// A faithful arbitrary JSON object with redacted ordinary formatting.
#[derive(Clone, PartialEq, Eq, Serialize, Deserialize, Default)]
#[serde(transparent)]
pub struct JsonObject(serde_json::Map<String, Value>);
impl JsonObject {
    pub fn new(value: serde_json::Map<String, Value>) -> Self {
        Self(value)
    }
    pub fn as_map(&self) -> &serde_json::Map<String, Value> {
        &self.0
    }
    pub fn into_map(self) -> serde_json::Map<String, Value> {
        self.0
    }
}
impl From<serde_json::Map<String, Value>> for JsonObject {
    fn from(value: serde_json::Map<String, Value>) -> Self {
        Self(value)
    }
}
impl fmt::Debug for JsonObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JsonObject([REDACTED])")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[derive(Serialize, Deserialize)]
    struct Fields {
        #[serde(default, skip_serializing_if = "Optional::is_unset")]
        optional: Optional<String>,
        required: Nullable<String>,
        #[serde(default, skip_serializing_if = "Patch::is_unset")]
        patch: Patch<String>,
    }
    #[test]
    fn omission_null_and_value_are_distinct() {
        struct NoDefault;
        assert!(Optional::<NoDefault>::default().is_unset());
        assert!(Patch::<NoDefault>::default().is_unset());
        assert!(serde_json::from_str::<Fields>("{}").is_err());
        let fields: Fields = serde_json::from_str(r#"{"required":null}"#).unwrap();
        assert!(fields.optional.is_unset());
        assert!(fields.required.is_null());
        assert!(fields.patch.is_unset());
        assert_eq!(
            serde_json::to_value(fields).unwrap(),
            serde_json::json!({"required":null})
        );
        assert!(serde_json::from_str::<Fields>(r#"{"required":null,"optional":null}"#).is_err());
        let fields: Fields = serde_json::from_str(r#"{"required":"kept","patch":null}"#).unwrap();
        assert!(fields.patch.is_null());
        assert_eq!(
            serde_json::to_value(fields).unwrap(),
            serde_json::json!({"required":"kept","patch":null})
        );
        assert!(serde_json::to_value(Patch::<String>::Unset).is_err());
    }
    #[test]
    fn numbers_never_turn_nonfinite_into_null() {
        for value in [f64::NAN, f64::INFINITY, f64::NEG_INFINITY] {
            assert!(FiniteNumber::try_from(value).is_err());
        }
        assert_eq!(FiniteNumber::from(u64::MAX).as_u64(), Some(u64::MAX));
        assert!(serde_json::from_str::<FiniteNumber>("1e999").is_err());
        // A downstream crate may enable serde_json's arbitrary_precision feature.
        // Its Number representation must not weaken this type's finite contract.
        if let Ok(value) = serde_json::from_str::<Number>("1e999") {
            assert!(FiniteNumber::try_from(value).is_err());
        }
        assert_eq!(
            serde_json::to_value(FiniteNumber::try_from(1.5).unwrap()).unwrap(),
            serde_json::json!(1.5)
        );
    }
    #[test]
    fn ordinary_formatting_excludes_values() {
        let output = format!(
            "{:?} {:?} {:?} {:?} {:?} {:?}",
            Optional::Value("confidential"),
            Nullable::Value("confidential"),
            Patch::Value("confidential"),
            FiniteNumber::from(123),
            JsonValue::new(serde_json::json!({"confidential":123})),
            JsonObject::new(
                serde_json::from_value(serde_json::json!({"confidential":123})).unwrap()
            )
        );
        assert!(!output.contains("confidential"));
        assert!(!output.contains("123"));
    }
}
