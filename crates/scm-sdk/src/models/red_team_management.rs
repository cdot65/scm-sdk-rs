//! Management 0.86.0 additive models derived from the MIT pan.dev specification.
//! Regenerate with tools/generate_models.py; see NOTICE.
#![allow(clippy::large_enum_variant)]
use super::support::*;
use crate::data::JsonValue;
use serde::{Deserialize, Serialize, ser::SerializeMap};
use std::fmt;
/// Native typed contract for frozen schema `smp_32d526d72ac63585e587daf7`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct MultiTurnClientSessionConfig {
    #[serde(rename = "type")]
    pub r#type: MultiTurnClientSessionConfigType,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for MultiTurnClientSessionConfig {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["type"];
        if self
            .extensions
            .keys()
            .any(|key| KNOWN.contains(&key.as_str()))
        {
            return Err(serde::ser::Error::custom(
                "extension collides with a known model field",
            ));
        }
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("type", &self.r#type)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl MultiTurnClientSessionConfig {
    pub fn builder() -> MultiTurnClientSessionConfigBuilder {
        MultiTurnClientSessionConfigBuilder::default()
    }
    pub const SCHEMA: &'static str = "smp_32d526d72ac63585e587daf7";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct MultiTurnClientSessionConfigBuilder {
    r#type: Option<MultiTurnClientSessionConfigType>,
    extensions: Extensions<JsonValue>,
}
impl MultiTurnClientSessionConfigBuilder {
    pub fn r#type(mut self, value: MultiTurnClientSessionConfigType) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<MultiTurnClientSessionConfig> {
        Ok(MultiTurnClientSessionConfig {
            r#type: self
                .r#type
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<MultiTurnClientSessionConfig> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        MultiTurnClientSessionConfig::parse(value, mode)
    }
}
impl fmt::Debug for MultiTurnClientSessionConfigBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MultiTurnClientSessionConfigBuilder([REDACTED])")
    }
}
impl fmt::Debug for MultiTurnClientSessionConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MultiTurnClientSessionConfig([REDACTED])")
    }
}

/// Native typed contract for frozen schema `smp_d339c1d9c989a597aa5ccb04`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MultiTurnClientSessionConfigType {
    ClientSession,
}
impl MultiTurnClientSessionConfigType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ClientSession => "client_session",
        }
    }
}
impl Serialize for MultiTurnClientSessionConfigType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for MultiTurnClientSessionConfigType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        match value.as_str() {
            "client_session" => Ok(Self::ClientSession),
            _ => Err(serde::de::Error::custom("literal discriminator mismatch")),
        }
    }
}
impl fmt::Debug for MultiTurnClientSessionConfigType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MultiTurnClientSessionConfigType([REDACTED])")
    }
}

/// Native typed contract for frozen schema `smp_bc45d353b11579547ea9cf9e`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct VertexAIConnectionParams {
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "engine_id")]
    pub engine_id: String,
    #[serde(rename = "customer_sa_email")]
    pub customer_sa_email: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for VertexAIConnectionParams {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["project_id", "location", "engine_id", "customer_sa_email"];
        if self
            .extensions
            .keys()
            .any(|key| KNOWN.contains(&key.as_str()))
        {
            return Err(serde::ser::Error::custom(
                "extension collides with a known model field",
            ));
        }
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("project_id", &self.project_id)?;
        map.serialize_entry("location", &self.location)?;
        map.serialize_entry("engine_id", &self.engine_id)?;
        map.serialize_entry("customer_sa_email", &self.customer_sa_email)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl VertexAIConnectionParams {
    pub fn builder() -> VertexAIConnectionParamsBuilder {
        VertexAIConnectionParamsBuilder::default()
    }
    pub const SCHEMA: &'static str = "smp_bc45d353b11579547ea9cf9e";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct VertexAIConnectionParamsBuilder {
    project_id: Option<String>,
    location: Option<String>,
    engine_id: Option<String>,
    customer_sa_email: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl VertexAIConnectionParamsBuilder {
    pub fn project_id(mut self, value: String) -> Self {
        self.project_id = Some(value);
        self
    }
    pub fn location(mut self, value: String) -> Self {
        self.location = Some(value);
        self
    }
    pub fn engine_id(mut self, value: String) -> Self {
        self.engine_id = Some(value);
        self
    }
    pub fn customer_sa_email(mut self, value: String) -> Self {
        self.customer_sa_email = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<VertexAIConnectionParams> {
        Ok(VertexAIConnectionParams {
            project_id: self
                .project_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            location: self
                .location
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            engine_id: self
                .engine_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            customer_sa_email: self
                .customer_sa_email
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<VertexAIConnectionParams> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        VertexAIConnectionParams::parse(value, mode)
    }
}
impl fmt::Debug for VertexAIConnectionParamsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VertexAIConnectionParamsBuilder([REDACTED])")
    }
}
impl fmt::Debug for VertexAIConnectionParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VertexAIConnectionParams([REDACTED])")
    }
}

/// Native typed contract for frozen schema `smp_4bcd2c718055eb4ea89dcd95`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
/// Declaration-only typed view of the vendor masked shape. Target GET responses
/// use the shared `VertexAIConnectionParams` union branch; response validation
/// preserves masked strings and does not enforce request patterns.
pub struct VertexAIConnectionRedactParams {
    #[serde(rename = "project_id")]
    pub project_id: String,
    #[serde(rename = "location")]
    pub location: String,
    #[serde(rename = "engine_id")]
    pub engine_id: String,
    #[serde(rename = "customer_sa_email")]
    pub customer_sa_email: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for VertexAIConnectionRedactParams {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["project_id", "location", "engine_id", "customer_sa_email"];
        if self
            .extensions
            .keys()
            .any(|key| KNOWN.contains(&key.as_str()))
        {
            return Err(serde::ser::Error::custom(
                "extension collides with a known model field",
            ));
        }
        let mut map = serializer.serialize_map(None)?;
        map.serialize_entry("project_id", &self.project_id)?;
        map.serialize_entry("location", &self.location)?;
        map.serialize_entry("engine_id", &self.engine_id)?;
        map.serialize_entry("customer_sa_email", &self.customer_sa_email)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl VertexAIConnectionRedactParams {
    pub fn builder() -> VertexAIConnectionRedactParamsBuilder {
        VertexAIConnectionRedactParamsBuilder::default()
    }
    pub const SCHEMA: &'static str = "smp_4bcd2c718055eb4ea89dcd95";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct VertexAIConnectionRedactParamsBuilder {
    project_id: Option<String>,
    location: Option<String>,
    engine_id: Option<String>,
    customer_sa_email: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl VertexAIConnectionRedactParamsBuilder {
    pub fn project_id(mut self, value: String) -> Self {
        self.project_id = Some(value);
        self
    }
    pub fn location(mut self, value: String) -> Self {
        self.location = Some(value);
        self
    }
    pub fn engine_id(mut self, value: String) -> Self {
        self.engine_id = Some(value);
        self
    }
    pub fn customer_sa_email(mut self, value: String) -> Self {
        self.customer_sa_email = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<VertexAIConnectionRedactParams> {
        Ok(VertexAIConnectionRedactParams {
            project_id: self
                .project_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            location: self
                .location
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            engine_id: self
                .engine_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            customer_sa_email: self
                .customer_sa_email
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<VertexAIConnectionRedactParams> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        VertexAIConnectionRedactParams::parse(value, mode)
    }
}
impl fmt::Debug for VertexAIConnectionRedactParamsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VertexAIConnectionRedactParamsBuilder([REDACTED])")
    }
}
impl fmt::Debug for VertexAIConnectionRedactParams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("VertexAIConnectionRedactParams([REDACTED])")
    }
}

#[cfg(feature = "red-team")]
pub(crate) const TARGET_PROFILING_FILTER_VALUES: &[&str] =
    &["NOT_STARTED", "IN_PROGRESS", "COMPLETED", "FAILED"];
