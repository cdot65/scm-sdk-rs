//! Generated from the pinned Red Team dataplane 0.93.0 MIT specification.
//! Regenerate with tools/generate_models.py; see package NOTICE.
#![allow(clippy::large_enum_variant)]
use super::support::*;
use crate::data::{FiniteNumber, JsonValue, Nullable, Optional, Patch};
use serde::{Deserialize, Serialize, ser::SerializeMap};
use std::fmt;
/// Native typed contract for vendor schema `sdp_q_a4c4751e69d5be1285be2591`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AgenticProgress {
    #[serde(rename = "kind", default)]
    pub kind: Optional<AgenticProgressKind>,
    #[serde(rename = "stages", default)]
    pub stages: Optional<Vec<AgenticStage>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AgenticProgress {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["kind", "stages"];
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
        if !self.kind.is_unset() {
            map.serialize_entry("kind", &self.kind)?;
        }
        if !self.stages.is_unset() {
            map.serialize_entry("stages", &self.stages)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AgenticProgress {
    pub fn builder() -> AgenticProgressBuilder {
        AgenticProgressBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_a4c4751e69d5be1285be2591";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AgenticProgressBuilder {
    kind: Optional<AgenticProgressKind>,
    stages: Optional<Vec<AgenticStage>>,
    extensions: Extensions<JsonValue>,
}
impl AgenticProgressBuilder {
    pub fn kind(mut self, value: Optional<AgenticProgressKind>) -> Self {
        self.kind = value;
        self
    }
    pub fn stages(mut self, value: Optional<Vec<AgenticStage>>) -> Self {
        self.stages = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AgenticProgress> {
        Ok(AgenticProgress {
            kind: self.kind,
            stages: self.stages,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<AgenticProgress> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AgenticProgress::parse(value, mode)
    }
}
impl fmt::Debug for AgenticProgressBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgenticProgressBuilder([REDACTED])")
    }
}
impl fmt::Debug for AgenticProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgenticProgress([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_4e22f51e52502b3fb42b80da`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AgenticProgressKind {
    Agentic,
}
impl AgenticProgressKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Agentic => "agentic",
        }
    }
}
impl Serialize for AgenticProgressKind {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AgenticProgressKind {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        match value.as_str() {
            "agentic" => Ok(Self::Agentic),
            _ => Err(serde::de::Error::custom("literal discriminator mismatch")),
        }
    }
}
impl fmt::Debug for AgenticProgressKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgenticProgressKind([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_fdf2ec8a7b17774e048fa247`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AgenticStage {
    #[serde(rename = "stage")]
    pub stage: AgenticStageName,
    #[serde(rename = "status")]
    pub status: AgenticStatus,
    #[serde(rename = "sub_steps", default)]
    pub sub_steps: Optional<Vec<AgenticSubStep>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AgenticStage {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["stage", "status", "sub_steps"];
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
        map.serialize_entry("stage", &self.stage)?;
        map.serialize_entry("status", &self.status)?;
        if !self.sub_steps.is_unset() {
            map.serialize_entry("sub_steps", &self.sub_steps)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AgenticStage {
    pub fn builder() -> AgenticStageBuilder {
        AgenticStageBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_fdf2ec8a7b17774e048fa247";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AgenticStageBuilder {
    stage: Option<AgenticStageName>,
    status: Option<AgenticStatus>,
    sub_steps: Optional<Vec<AgenticSubStep>>,
    extensions: Extensions<JsonValue>,
}
impl AgenticStageBuilder {
    pub fn stage(mut self, value: AgenticStageName) -> Self {
        self.stage = Some(value);
        self
    }
    pub fn status(mut self, value: AgenticStatus) -> Self {
        self.status = Some(value);
        self
    }
    pub fn sub_steps(mut self, value: Optional<Vec<AgenticSubStep>>) -> Self {
        self.sub_steps = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AgenticStage> {
        Ok(AgenticStage {
            stage: self
                .stage
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            status: self
                .status
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            sub_steps: self.sub_steps,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<AgenticStage> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AgenticStage::parse(value, mode)
    }
}
impl fmt::Debug for AgenticStageBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgenticStageBuilder([REDACTED])")
    }
}
impl fmt::Debug for AgenticStage {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgenticStage([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c8b2c9e2e810ebeceac27d16`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AgenticStageName {
    TargetProfiling,
    GoalGeneration,
    AttackExecution,
    Unknown(String),
}
impl AgenticStageName {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TargetProfiling => "target_profiling",
            Self::GoalGeneration => "goal_generation",
            Self::AttackExecution => "attack_execution",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AgenticStageName {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AgenticStageName {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "target_profiling" => Self::TargetProfiling,
            "goal_generation" => Self::GoalGeneration,
            "attack_execution" => Self::AttackExecution,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AgenticStageName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgenticStageName([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_5428a10f512ef870b2efb847`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AgenticStatus {
    Pending,
    InProgress,
    Complete,
    PartiallyComplete,
    NotStarted,
    Skipped,
    Unknown(String),
}
impl AgenticStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Pending => "pending",
            Self::InProgress => "in_progress",
            Self::Complete => "complete",
            Self::PartiallyComplete => "partially_complete",
            Self::NotStarted => "not_started",
            Self::Skipped => "skipped",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AgenticStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AgenticStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "pending" => Self::Pending,
            "in_progress" => Self::InProgress,
            "complete" => Self::Complete,
            "partially_complete" => Self::PartiallyComplete,
            "not_started" => Self::NotStarted,
            "skipped" => Self::Skipped,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AgenticStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgenticStatus([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_79ea86b463928764c618ddcf`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AgenticSubStep {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "completed", default)]
    pub completed: Patch<FiniteNumber>,
    #[serde(rename = "total", default)]
    pub total: Patch<FiniteNumber>,
    #[serde(rename = "status")]
    pub status: AgenticStatus,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AgenticSubStep {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["label", "completed", "total", "status"];
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
        map.serialize_entry("label", &self.label)?;
        if !self.completed.is_unset() {
            map.serialize_entry("completed", &self.completed)?;
        }
        if !self.total.is_unset() {
            map.serialize_entry("total", &self.total)?;
        }
        map.serialize_entry("status", &self.status)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AgenticSubStep {
    pub fn builder() -> AgenticSubStepBuilder {
        AgenticSubStepBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_79ea86b463928764c618ddcf";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AgenticSubStepBuilder {
    label: Option<String>,
    completed: Patch<FiniteNumber>,
    total: Patch<FiniteNumber>,
    status: Option<AgenticStatus>,
    extensions: Extensions<JsonValue>,
}
impl AgenticSubStepBuilder {
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn completed(mut self, value: Patch<FiniteNumber>) -> Self {
        self.completed = value;
        self
    }
    pub fn total(mut self, value: Patch<FiniteNumber>) -> Self {
        self.total = value;
        self
    }
    pub fn status(mut self, value: AgenticStatus) -> Self {
        self.status = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AgenticSubStep> {
        Ok(AgenticSubStep {
            label: self
                .label
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            completed: self.completed,
            total: self.total,
            status: self
                .status
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<AgenticSubStep> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AgenticSubStep::parse(value, mode)
    }
}
impl fmt::Debug for AgenticSubStepBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgenticSubStepBuilder([REDACTED])")
    }
}
impl fmt::Debug for AgenticSubStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AgenticSubStep([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_0af4adfa08d739b1230c99b7`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ApiEndpointType {
    PUBLIC,
    PRIVATE,
    NETWORKBROKER,
    Unknown(String),
}
impl ApiEndpointType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PUBLIC => "PUBLIC",
            Self::PRIVATE => "PRIVATE",
            Self::NETWORKBROKER => "NETWORK_BROKER",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ApiEndpointType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ApiEndpointType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "PUBLIC" => Self::PUBLIC,
            "PRIVATE" => Self::PRIVATE,
            "NETWORK_BROKER" => Self::NETWORKBROKER,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ApiEndpointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ApiEndpointType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_f722ab6c76eec9c75dbcdce3`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AsrCategory {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "total")]
    pub total: FiniteNumber,
    #[serde(rename = "successful")]
    pub successful: FiniteNumber,
    #[serde(rename = "failed")]
    pub failed: FiniteNumber,
    #[serde(rename = "asr")]
    pub asr: FiniteNumber,
    #[serde(rename = "sub_categories", default)]
    pub sub_categories: Optional<Vec<AsrStats>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AsrCategory {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "id",
            "display_name",
            "total",
            "successful",
            "failed",
            "asr",
            "sub_categories",
        ];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("total", &self.total)?;
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("failed", &self.failed)?;
        map.serialize_entry("asr", &self.asr)?;
        if !self.sub_categories.is_unset() {
            map.serialize_entry("sub_categories", &self.sub_categories)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AsrCategory {
    pub fn builder() -> AsrCategoryBuilder {
        AsrCategoryBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_f722ab6c76eec9c75dbcdce3";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AsrCategoryBuilder {
    id: Option<String>,
    display_name: Option<String>,
    total: Option<FiniteNumber>,
    successful: Option<FiniteNumber>,
    failed: Option<FiniteNumber>,
    asr: Option<FiniteNumber>,
    sub_categories: Optional<Vec<AsrStats>>,
    extensions: Extensions<JsonValue>,
}
impl AsrCategoryBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn total(mut self, value: FiniteNumber) -> Self {
        self.total = Some(value);
        self
    }
    pub fn successful(mut self, value: FiniteNumber) -> Self {
        self.successful = Some(value);
        self
    }
    pub fn failed(mut self, value: FiniteNumber) -> Self {
        self.failed = Some(value);
        self
    }
    pub fn asr(mut self, value: FiniteNumber) -> Self {
        self.asr = Some(value);
        self
    }
    pub fn sub_categories(mut self, value: Optional<Vec<AsrStats>>) -> Self {
        self.sub_categories = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AsrCategory> {
        Ok(AsrCategory {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total: self
                .total
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            successful: self
                .successful
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            failed: self
                .failed
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            asr: self
                .asr
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            sub_categories: self.sub_categories,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<AsrCategory> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AsrCategory::parse(value, mode)
    }
}
impl fmt::Debug for AsrCategoryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AsrCategoryBuilder([REDACTED])")
    }
}
impl fmt::Debug for AsrCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AsrCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_9df843dae617cb5fe5abcf72`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AsrResponse {
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "job_type")]
    pub job_type: JobType,
    #[serde(rename = "asr")]
    pub asr: FiniteNumber,
    #[serde(rename = "categories", default)]
    pub categories: Optional<Vec<AsrCategory>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AsrResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["job_id", "job_type", "asr", "categories"];
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
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("job_type", &self.job_type)?;
        map.serialize_entry("asr", &self.asr)?;
        if !self.categories.is_unset() {
            map.serialize_entry("categories", &self.categories)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AsrResponse {
    pub fn builder() -> AsrResponseBuilder {
        AsrResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_9df843dae617cb5fe5abcf72";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AsrResponseBuilder {
    job_id: Option<String>,
    job_type: Option<JobType>,
    asr: Option<FiniteNumber>,
    categories: Optional<Vec<AsrCategory>>,
    extensions: Extensions<JsonValue>,
}
impl AsrResponseBuilder {
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn job_type(mut self, value: JobType) -> Self {
        self.job_type = Some(value);
        self
    }
    pub fn asr(mut self, value: FiniteNumber) -> Self {
        self.asr = Some(value);
        self
    }
    pub fn categories(mut self, value: Optional<Vec<AsrCategory>>) -> Self {
        self.categories = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AsrResponse> {
        Ok(AsrResponse {
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_type: self
                .job_type
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            asr: self
                .asr
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            categories: self.categories,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<AsrResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AsrResponse::parse(value, mode)
    }
}
impl fmt::Debug for AsrResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AsrResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for AsrResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AsrResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_62c8057c0a6350d5505197a9`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AsrStats {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "total")]
    pub total: FiniteNumber,
    #[serde(rename = "successful")]
    pub successful: FiniteNumber,
    #[serde(rename = "failed")]
    pub failed: FiniteNumber,
    #[serde(rename = "asr")]
    pub asr: FiniteNumber,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AsrStats {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["id", "display_name", "total", "successful", "failed", "asr"];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("total", &self.total)?;
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("failed", &self.failed)?;
        map.serialize_entry("asr", &self.asr)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AsrStats {
    pub fn builder() -> AsrStatsBuilder {
        AsrStatsBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_62c8057c0a6350d5505197a9";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AsrStatsBuilder {
    id: Option<String>,
    display_name: Option<String>,
    total: Option<FiniteNumber>,
    successful: Option<FiniteNumber>,
    failed: Option<FiniteNumber>,
    asr: Option<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl AsrStatsBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn total(mut self, value: FiniteNumber) -> Self {
        self.total = Some(value);
        self
    }
    pub fn successful(mut self, value: FiniteNumber) -> Self {
        self.successful = Some(value);
        self
    }
    pub fn failed(mut self, value: FiniteNumber) -> Self {
        self.failed = Some(value);
        self
    }
    pub fn asr(mut self, value: FiniteNumber) -> Self {
        self.asr = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AsrStats> {
        Ok(AsrStats {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total: self
                .total
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            successful: self
                .successful
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            failed: self
                .failed
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            asr: self
                .asr
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<AsrStats> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AsrStats::parse(value, mode)
    }
}
impl fmt::Debug for AsrStatsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AsrStatsBuilder([REDACTED])")
    }
}
impl fmt::Debug for AsrStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AsrStats([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_a56ec471d25a69d21f8d6d80`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackAudio {
    #[serde(rename = "audio_name")]
    pub audio_name: String,
    #[serde(rename = "signed_url", default)]
    pub signed_url: Patch<String>,
    #[serde(rename = "audio_format", default)]
    pub audio_format: Patch<AttackAudioAudioFormat>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackAudio {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["audio_name", "signed_url", "audio_format"];
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
        map.serialize_entry("audio_name", &self.audio_name)?;
        if !self.signed_url.is_unset() {
            map.serialize_entry("signed_url", &self.signed_url)?;
        }
        if !self.audio_format.is_unset() {
            map.serialize_entry("audio_format", &self.audio_format)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackAudio {
    pub fn builder() -> AttackAudioBuilder {
        AttackAudioBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_a56ec471d25a69d21f8d6d80";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackAudioBuilder {
    audio_name: Option<String>,
    signed_url: Patch<String>,
    audio_format: Patch<AttackAudioAudioFormat>,
    extensions: Extensions<JsonValue>,
}
impl AttackAudioBuilder {
    pub fn audio_name(mut self, value: String) -> Self {
        self.audio_name = Some(value);
        self
    }
    pub fn signed_url(mut self, value: Patch<String>) -> Self {
        self.signed_url = value;
        self
    }
    pub fn audio_format(mut self, value: Patch<AttackAudioAudioFormat>) -> Self {
        self.audio_format = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackAudio> {
        Ok(AttackAudio {
            audio_name: self
                .audio_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            signed_url: self.signed_url,
            audio_format: self.audio_format,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<AttackAudio> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackAudio::parse(value, mode)
    }
}
impl fmt::Debug for AttackAudioBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackAudioBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackAudio {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackAudio([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_9d60d9890bd16096a7d11d1d`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttackAudioAudioFormat {
    WAV,
    MP3,
    PCM16,
    Unknown(String),
}
impl AttackAudioAudioFormat {
    pub fn as_str(&self) -> &str {
        match self {
            Self::WAV => "WAV",
            Self::MP3 => "MP3",
            Self::PCM16 => "PCM16",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AttackAudioAudioFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AttackAudioAudioFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "WAV" => Self::WAV,
            "MP3" => Self::MP3,
            "PCM16" => Self::PCM16,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AttackAudioAudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackAudioAudioFormat([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_01ab4dc9408116a84a16c5ee`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackDetailResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "target_id")]
    pub target_id: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "status", default)]
    pub status: Optional<AttackStatus>,
    #[serde(rename = "marked_safe", default)]
    pub marked_safe: Patch<bool>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Optional<AttackDetailResponseExtraInfo>,
    #[serde(rename = "threat", default)]
    pub threat: Patch<bool>,
    #[serde(rename = "attack_type", default)]
    pub attack_type: Optional<AttackType>,
    #[serde(rename = "multi_turn", default)]
    pub multi_turn: Optional<bool>,
    #[serde(rename = "asr", default)]
    pub asr: Patch<FiniteNumber>,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(rename = "prompt_mapping_id")]
    pub prompt_mapping_id: String,
    #[serde(rename = "prompt_id")]
    pub prompt_id: String,
    #[serde(rename = "category")]
    pub category: Category,
    #[serde(rename = "sub_category")]
    pub sub_category: AttackDetailResponseSubCategory,
    #[serde(rename = "severity", default)]
    pub severity: Optional<String>,
    #[serde(rename = "threat_details", default)]
    pub threat_details: Optional<ThreatDetails>,
    #[serde(rename = "category_display_name")]
    pub category_display_name: String,
    #[serde(rename = "sub_category_display_name")]
    pub sub_category_display_name: String,
    #[serde(rename = "compliance_frameworks")]
    pub compliance_frameworks: Vec<ComplianceWithTechniqueModel>,
    #[serde(rename = "outputs", default)]
    pub outputs: Optional<Vec<AttackOutputResponse>>,
    #[serde(rename = "goal")]
    pub goal: Nullable<String>,
    #[serde(rename = "attack_modality", default)]
    pub attack_modality: Optional<Modality>,
    #[serde(rename = "file", default)]
    pub file: Patch<AttackFile>,
    #[serde(rename = "audio", default)]
    pub audio: Patch<AttackAudio>,
    #[serde(rename = "audio_technique")]
    pub audio_technique: Nullable<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackDetailResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "job_id",
            "target_id",
            "prompt",
            "status",
            "marked_safe",
            "extra_info",
            "threat",
            "attack_type",
            "multi_turn",
            "asr",
            "version",
            "error",
            "prompt_mapping_id",
            "prompt_id",
            "category",
            "sub_category",
            "severity",
            "threat_details",
            "category_display_name",
            "sub_category_display_name",
            "compliance_frameworks",
            "outputs",
            "goal",
            "attack_modality",
            "file",
            "audio",
            "audio_technique",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("target_id", &self.target_id)?;
        map.serialize_entry("prompt", &self.prompt)?;
        if !self.status.is_unset() {
            map.serialize_entry("status", &self.status)?;
        }
        if !self.marked_safe.is_unset() {
            map.serialize_entry("marked_safe", &self.marked_safe)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.attack_type.is_unset() {
            map.serialize_entry("attack_type", &self.attack_type)?;
        }
        if !self.multi_turn.is_unset() {
            map.serialize_entry("multi_turn", &self.multi_turn)?;
        }
        if !self.asr.is_unset() {
            map.serialize_entry("asr", &self.asr)?;
        }
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        map.serialize_entry("prompt_mapping_id", &self.prompt_mapping_id)?;
        map.serialize_entry("prompt_id", &self.prompt_id)?;
        map.serialize_entry("category", &self.category)?;
        map.serialize_entry("sub_category", &self.sub_category)?;
        if !self.severity.is_unset() {
            map.serialize_entry("severity", &self.severity)?;
        }
        if !self.threat_details.is_unset() {
            map.serialize_entry("threat_details", &self.threat_details)?;
        }
        map.serialize_entry("category_display_name", &self.category_display_name)?;
        map.serialize_entry("sub_category_display_name", &self.sub_category_display_name)?;
        map.serialize_entry("compliance_frameworks", &self.compliance_frameworks)?;
        if !self.outputs.is_unset() {
            map.serialize_entry("outputs", &self.outputs)?;
        }
        map.serialize_entry("goal", &self.goal)?;
        if !self.attack_modality.is_unset() {
            map.serialize_entry("attack_modality", &self.attack_modality)?;
        }
        if !self.file.is_unset() {
            map.serialize_entry("file", &self.file)?;
        }
        if !self.audio.is_unset() {
            map.serialize_entry("audio", &self.audio)?;
        }
        map.serialize_entry("audio_technique", &self.audio_technique)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackDetailResponse {
    pub fn builder() -> AttackDetailResponseBuilder {
        AttackDetailResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_01ab4dc9408116a84a16c5ee";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackDetailResponseBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    job_id: Option<String>,
    target_id: Option<String>,
    prompt: Option<String>,
    status: Optional<AttackStatus>,
    marked_safe: Patch<bool>,
    extra_info: Optional<AttackDetailResponseExtraInfo>,
    threat: Patch<bool>,
    attack_type: Optional<AttackType>,
    multi_turn: Optional<bool>,
    asr: Patch<FiniteNumber>,
    version: Patch<FiniteNumber>,
    error: Patch<bool>,
    prompt_mapping_id: Option<String>,
    prompt_id: Option<String>,
    category: Option<Category>,
    sub_category: Option<AttackDetailResponseSubCategory>,
    severity: Optional<String>,
    threat_details: Optional<ThreatDetails>,
    category_display_name: Option<String>,
    sub_category_display_name: Option<String>,
    compliance_frameworks: Option<Vec<ComplianceWithTechniqueModel>>,
    outputs: Optional<Vec<AttackOutputResponse>>,
    goal: Option<Nullable<String>>,
    attack_modality: Optional<Modality>,
    file: Patch<AttackFile>,
    audio: Patch<AttackAudio>,
    audio_technique: Option<Nullable<String>>,
    extensions: Extensions<JsonValue>,
}
impl AttackDetailResponseBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn target_id(mut self, value: String) -> Self {
        self.target_id = Some(value);
        self
    }
    pub fn prompt(mut self, value: String) -> Self {
        self.prompt = Some(value);
        self
    }
    pub fn status(mut self, value: Optional<AttackStatus>) -> Self {
        self.status = value;
        self
    }
    pub fn marked_safe(mut self, value: Patch<bool>) -> Self {
        self.marked_safe = value;
        self
    }
    pub fn extra_info(mut self, value: Optional<AttackDetailResponseExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn threat(mut self, value: Patch<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn attack_type(mut self, value: Optional<AttackType>) -> Self {
        self.attack_type = value;
        self
    }
    pub fn multi_turn(mut self, value: Optional<bool>) -> Self {
        self.multi_turn = value;
        self
    }
    pub fn asr(mut self, value: Patch<FiniteNumber>) -> Self {
        self.asr = value;
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn prompt_mapping_id(mut self, value: String) -> Self {
        self.prompt_mapping_id = Some(value);
        self
    }
    pub fn prompt_id(mut self, value: String) -> Self {
        self.prompt_id = Some(value);
        self
    }
    pub fn category(mut self, value: Category) -> Self {
        self.category = Some(value);
        self
    }
    pub fn sub_category(mut self, value: AttackDetailResponseSubCategory) -> Self {
        self.sub_category = Some(value);
        self
    }
    pub fn severity(mut self, value: Optional<String>) -> Self {
        self.severity = value;
        self
    }
    pub fn threat_details(mut self, value: Optional<ThreatDetails>) -> Self {
        self.threat_details = value;
        self
    }
    pub fn category_display_name(mut self, value: String) -> Self {
        self.category_display_name = Some(value);
        self
    }
    pub fn sub_category_display_name(mut self, value: String) -> Self {
        self.sub_category_display_name = Some(value);
        self
    }
    pub fn compliance_frameworks(mut self, value: Vec<ComplianceWithTechniqueModel>) -> Self {
        self.compliance_frameworks = Some(value);
        self
    }
    pub fn outputs(mut self, value: Optional<Vec<AttackOutputResponse>>) -> Self {
        self.outputs = value;
        self
    }
    pub fn goal(mut self, value: Nullable<String>) -> Self {
        self.goal = Some(value);
        self
    }
    pub fn attack_modality(mut self, value: Optional<Modality>) -> Self {
        self.attack_modality = value;
        self
    }
    pub fn file(mut self, value: Patch<AttackFile>) -> Self {
        self.file = value;
        self
    }
    pub fn audio(mut self, value: Patch<AttackAudio>) -> Self {
        self.audio = value;
        self
    }
    pub fn audio_technique(mut self, value: Nullable<String>) -> Self {
        self.audio_technique = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackDetailResponse> {
        Ok(AttackDetailResponse {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target_id: self
                .target_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt: self
                .prompt
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            status: self.status,
            marked_safe: self.marked_safe,
            extra_info: self.extra_info,
            threat: self.threat,
            attack_type: self.attack_type,
            multi_turn: self.multi_turn,
            asr: self.asr,
            version: self.version,
            error: self.error,
            prompt_mapping_id: self
                .prompt_mapping_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt_id: self
                .prompt_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            category: self
                .category
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            sub_category: self
                .sub_category
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            severity: self.severity,
            threat_details: self.threat_details,
            category_display_name: self
                .category_display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            sub_category_display_name: self
                .sub_category_display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            compliance_frameworks: self
                .compliance_frameworks
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            outputs: self.outputs,
            goal: self
                .goal
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            attack_modality: self.attack_modality,
            file: self.file,
            audio: self.audio,
            audio_technique: self
                .audio_technique
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<AttackDetailResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackDetailResponse::parse(value, mode)
    }
}
impl fmt::Debug for AttackDetailResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackDetailResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackDetailResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackDetailResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_560ca0de77c2ae65d7274b12`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackDetailResponseExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackDetailResponseExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackDetailResponseExtraInfo {
    pub fn builder() -> AttackDetailResponseExtraInfoBuilder {
        AttackDetailResponseExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_560ca0de77c2ae65d7274b12";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackDetailResponseExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl AttackDetailResponseExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackDetailResponseExtraInfo> {
        Ok(AttackDetailResponseExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<AttackDetailResponseExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackDetailResponseExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for AttackDetailResponseExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackDetailResponseExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackDetailResponseExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackDetailResponseExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_59b64c926d25f8c8e3eb7c33`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttackDetailResponseSubCategory {
    ADVERSARIALSUFFIX,
    EVASION,
    INDIRECTPROMPTINJECTION,
    JAILBREAK,
    MULTITURN,
    PROMPTINJECTION,
    REMOTECODEEXECUTION,
    SYSTEMPROMPTLEAK,
    TOOLLEAK,
    MALWAREGENERATION,
    BIAS,
    CBRN,
    CYBERCRIME,
    DRUGS,
    HATETOXICABUSE,
    NONVIOLENTCRIMES,
    POLITICAL,
    SELFHARM,
    SEXUAL,
    VIOLENTCRIMESWEAPONS,
    COMPETITORENDORSEMENTS,
    BRANDTARNISHINGSELFCRITICISM,
    DISCRIMINATINGCLAIMS,
    POLITICALENDORSEMENTS,
    OWASP,
    MITREATLAS,
    NIST,
    DASFV2,
    Unknown(String),
}
impl AttackDetailResponseSubCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ADVERSARIALSUFFIX => "ADVERSARIAL_SUFFIX",
            Self::EVASION => "EVASION",
            Self::INDIRECTPROMPTINJECTION => "INDIRECT_PROMPT_INJECTION",
            Self::JAILBREAK => "JAILBREAK",
            Self::MULTITURN => "MULTI_TURN",
            Self::PROMPTINJECTION => "PROMPT_INJECTION",
            Self::REMOTECODEEXECUTION => "REMOTE_CODE_EXECUTION",
            Self::SYSTEMPROMPTLEAK => "SYSTEM_PROMPT_LEAK",
            Self::TOOLLEAK => "TOOL_LEAK",
            Self::MALWAREGENERATION => "MALWARE_GENERATION",
            Self::BIAS => "BIAS",
            Self::CBRN => "CBRN",
            Self::CYBERCRIME => "CYBERCRIME",
            Self::DRUGS => "DRUGS",
            Self::HATETOXICABUSE => "HATE_TOXIC_ABUSE",
            Self::NONVIOLENTCRIMES => "NON_VIOLENT_CRIMES",
            Self::POLITICAL => "POLITICAL",
            Self::SELFHARM => "SELF_HARM",
            Self::SEXUAL => "SEXUAL",
            Self::VIOLENTCRIMESWEAPONS => "VIOLENT_CRIMES_WEAPONS",
            Self::COMPETITORENDORSEMENTS => "COMPETITOR_ENDORSEMENTS",
            Self::BRANDTARNISHINGSELFCRITICISM => "BRAND_TARNISHING_SELF_CRITICISM",
            Self::DISCRIMINATINGCLAIMS => "DISCRIMINATING_CLAIMS",
            Self::POLITICALENDORSEMENTS => "POLITICAL_ENDORSEMENTS",
            Self::OWASP => "OWASP",
            Self::MITREATLAS => "MITRE_ATLAS",
            Self::NIST => "NIST",
            Self::DASFV2 => "DASF_V2",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AttackDetailResponseSubCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AttackDetailResponseSubCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "ADVERSARIAL_SUFFIX" => Self::ADVERSARIALSUFFIX,
            "EVASION" => Self::EVASION,
            "INDIRECT_PROMPT_INJECTION" => Self::INDIRECTPROMPTINJECTION,
            "JAILBREAK" => Self::JAILBREAK,
            "MULTI_TURN" => Self::MULTITURN,
            "PROMPT_INJECTION" => Self::PROMPTINJECTION,
            "REMOTE_CODE_EXECUTION" => Self::REMOTECODEEXECUTION,
            "SYSTEM_PROMPT_LEAK" => Self::SYSTEMPROMPTLEAK,
            "TOOL_LEAK" => Self::TOOLLEAK,
            "MALWARE_GENERATION" => Self::MALWAREGENERATION,
            "BIAS" => Self::BIAS,
            "CBRN" => Self::CBRN,
            "CYBERCRIME" => Self::CYBERCRIME,
            "DRUGS" => Self::DRUGS,
            "HATE_TOXIC_ABUSE" => Self::HATETOXICABUSE,
            "NON_VIOLENT_CRIMES" => Self::NONVIOLENTCRIMES,
            "POLITICAL" => Self::POLITICAL,
            "SELF_HARM" => Self::SELFHARM,
            "SEXUAL" => Self::SEXUAL,
            "VIOLENT_CRIMES_WEAPONS" => Self::VIOLENTCRIMESWEAPONS,
            "COMPETITOR_ENDORSEMENTS" => Self::COMPETITORENDORSEMENTS,
            "BRAND_TARNISHING_SELF_CRITICISM" => Self::BRANDTARNISHINGSELFCRITICISM,
            "DISCRIMINATING_CLAIMS" => Self::DISCRIMINATINGCLAIMS,
            "POLITICAL_ENDORSEMENTS" => Self::POLITICALENDORSEMENTS,
            "OWASP" => Self::OWASP,
            "MITRE_ATLAS" => Self::MITREATLAS,
            "NIST" => Self::NIST,
            "DASF_V2" => Self::DASFV2,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AttackDetailResponseSubCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackDetailResponseSubCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_cbb94ac978f413cc58c53e23`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackFile {
    #[serde(rename = "file_name")]
    pub file_name: String,
    #[serde(rename = "signed_url", default)]
    pub signed_url: Patch<String>,
    #[serde(rename = "file_type")]
    pub file_type: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackFile {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["file_name", "signed_url", "file_type"];
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
        map.serialize_entry("file_name", &self.file_name)?;
        if !self.signed_url.is_unset() {
            map.serialize_entry("signed_url", &self.signed_url)?;
        }
        map.serialize_entry("file_type", &self.file_type)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackFile {
    pub fn builder() -> AttackFileBuilder {
        AttackFileBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_cbb94ac978f413cc58c53e23";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackFileBuilder {
    file_name: Option<String>,
    signed_url: Patch<String>,
    file_type: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl AttackFileBuilder {
    pub fn file_name(mut self, value: String) -> Self {
        self.file_name = Some(value);
        self
    }
    pub fn signed_url(mut self, value: Patch<String>) -> Self {
        self.signed_url = value;
        self
    }
    pub fn file_type(mut self, value: String) -> Self {
        self.file_type = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackFile> {
        Ok(AttackFile {
            file_name: self
                .file_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            signed_url: self.signed_url,
            file_type: self
                .file_type
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<AttackFile> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackFile::parse(value, mode)
    }
}
impl fmt::Debug for AttackFileBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackFileBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackFile {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackFile([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_e7e565098cad280e13256b6d`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackList {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "target_id")]
    pub target_id: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "status", default)]
    pub status: Optional<AttackStatus>,
    #[serde(rename = "marked_safe", default)]
    pub marked_safe: Patch<bool>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Optional<AttackListExtraInfo>,
    #[serde(rename = "threat", default)]
    pub threat: Patch<bool>,
    #[serde(rename = "attack_type", default)]
    pub attack_type: Optional<AttackType>,
    #[serde(rename = "multi_turn", default)]
    pub multi_turn: Optional<bool>,
    #[serde(rename = "asr", default)]
    pub asr: Patch<FiniteNumber>,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(rename = "prompt_mapping_id")]
    pub prompt_mapping_id: String,
    #[serde(rename = "prompt_id")]
    pub prompt_id: String,
    #[serde(rename = "category")]
    pub category: Category,
    #[serde(rename = "sub_category")]
    pub sub_category: AttackListSubCategory,
    #[serde(rename = "severity", default)]
    pub severity: Optional<String>,
    #[serde(rename = "category_display_name")]
    pub category_display_name: String,
    #[serde(rename = "sub_category_display_name")]
    pub sub_category_display_name: String,
    #[serde(rename = "attack_modality", default)]
    pub attack_modality: Optional<Modality>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackList {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "job_id",
            "target_id",
            "prompt",
            "status",
            "marked_safe",
            "extra_info",
            "threat",
            "attack_type",
            "multi_turn",
            "asr",
            "version",
            "error",
            "prompt_mapping_id",
            "prompt_id",
            "category",
            "sub_category",
            "severity",
            "category_display_name",
            "sub_category_display_name",
            "attack_modality",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("target_id", &self.target_id)?;
        map.serialize_entry("prompt", &self.prompt)?;
        if !self.status.is_unset() {
            map.serialize_entry("status", &self.status)?;
        }
        if !self.marked_safe.is_unset() {
            map.serialize_entry("marked_safe", &self.marked_safe)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.attack_type.is_unset() {
            map.serialize_entry("attack_type", &self.attack_type)?;
        }
        if !self.multi_turn.is_unset() {
            map.serialize_entry("multi_turn", &self.multi_turn)?;
        }
        if !self.asr.is_unset() {
            map.serialize_entry("asr", &self.asr)?;
        }
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        map.serialize_entry("prompt_mapping_id", &self.prompt_mapping_id)?;
        map.serialize_entry("prompt_id", &self.prompt_id)?;
        map.serialize_entry("category", &self.category)?;
        map.serialize_entry("sub_category", &self.sub_category)?;
        if !self.severity.is_unset() {
            map.serialize_entry("severity", &self.severity)?;
        }
        map.serialize_entry("category_display_name", &self.category_display_name)?;
        map.serialize_entry("sub_category_display_name", &self.sub_category_display_name)?;
        if !self.attack_modality.is_unset() {
            map.serialize_entry("attack_modality", &self.attack_modality)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackList {
    pub fn builder() -> AttackListBuilder {
        AttackListBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_e7e565098cad280e13256b6d";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackListBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    job_id: Option<String>,
    target_id: Option<String>,
    prompt: Option<String>,
    status: Optional<AttackStatus>,
    marked_safe: Patch<bool>,
    extra_info: Optional<AttackListExtraInfo>,
    threat: Patch<bool>,
    attack_type: Optional<AttackType>,
    multi_turn: Optional<bool>,
    asr: Patch<FiniteNumber>,
    version: Patch<FiniteNumber>,
    error: Patch<bool>,
    prompt_mapping_id: Option<String>,
    prompt_id: Option<String>,
    category: Option<Category>,
    sub_category: Option<AttackListSubCategory>,
    severity: Optional<String>,
    category_display_name: Option<String>,
    sub_category_display_name: Option<String>,
    attack_modality: Optional<Modality>,
    extensions: Extensions<JsonValue>,
}
impl AttackListBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn target_id(mut self, value: String) -> Self {
        self.target_id = Some(value);
        self
    }
    pub fn prompt(mut self, value: String) -> Self {
        self.prompt = Some(value);
        self
    }
    pub fn status(mut self, value: Optional<AttackStatus>) -> Self {
        self.status = value;
        self
    }
    pub fn marked_safe(mut self, value: Patch<bool>) -> Self {
        self.marked_safe = value;
        self
    }
    pub fn extra_info(mut self, value: Optional<AttackListExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn threat(mut self, value: Patch<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn attack_type(mut self, value: Optional<AttackType>) -> Self {
        self.attack_type = value;
        self
    }
    pub fn multi_turn(mut self, value: Optional<bool>) -> Self {
        self.multi_turn = value;
        self
    }
    pub fn asr(mut self, value: Patch<FiniteNumber>) -> Self {
        self.asr = value;
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn prompt_mapping_id(mut self, value: String) -> Self {
        self.prompt_mapping_id = Some(value);
        self
    }
    pub fn prompt_id(mut self, value: String) -> Self {
        self.prompt_id = Some(value);
        self
    }
    pub fn category(mut self, value: Category) -> Self {
        self.category = Some(value);
        self
    }
    pub fn sub_category(mut self, value: AttackListSubCategory) -> Self {
        self.sub_category = Some(value);
        self
    }
    pub fn severity(mut self, value: Optional<String>) -> Self {
        self.severity = value;
        self
    }
    pub fn category_display_name(mut self, value: String) -> Self {
        self.category_display_name = Some(value);
        self
    }
    pub fn sub_category_display_name(mut self, value: String) -> Self {
        self.sub_category_display_name = Some(value);
        self
    }
    pub fn attack_modality(mut self, value: Optional<Modality>) -> Self {
        self.attack_modality = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackList> {
        Ok(AttackList {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target_id: self
                .target_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt: self
                .prompt
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            status: self.status,
            marked_safe: self.marked_safe,
            extra_info: self.extra_info,
            threat: self.threat,
            attack_type: self.attack_type,
            multi_turn: self.multi_turn,
            asr: self.asr,
            version: self.version,
            error: self.error,
            prompt_mapping_id: self
                .prompt_mapping_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt_id: self
                .prompt_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            category: self
                .category
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            sub_category: self
                .sub_category
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            severity: self.severity,
            category_display_name: self
                .category_display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            sub_category_display_name: self
                .sub_category_display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            attack_modality: self.attack_modality,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<AttackList> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackList::parse(value, mode)
    }
}
impl fmt::Debug for AttackListBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackListBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackList([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_dce54e09949454b98c2993e1`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackListExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackListExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackListExtraInfo {
    pub fn builder() -> AttackListExtraInfoBuilder {
        AttackListExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_dce54e09949454b98c2993e1";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackListExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl AttackListExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackListExtraInfo> {
        Ok(AttackListExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<AttackListExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackListExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for AttackListExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackListExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackListExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackListExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_dff5e7fb110f810ff19c3983`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackListResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "data")]
    pub data: Vec<AttackList>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackListResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["pagination", "data"];
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
        map.serialize_entry("pagination", &self.pagination)?;
        map.serialize_entry("data", &self.data)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackListResponse {
    pub fn builder() -> AttackListResponseBuilder {
        AttackListResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_dff5e7fb110f810ff19c3983";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackListResponseBuilder {
    pagination: Option<Pagination>,
    data: Option<Vec<AttackList>>,
    extensions: Extensions<JsonValue>,
}
impl AttackListResponseBuilder {
    pub fn pagination(mut self, value: Pagination) -> Self {
        self.pagination = Some(value);
        self
    }
    pub fn data(mut self, value: Vec<AttackList>) -> Self {
        self.data = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackListResponse> {
        Ok(AttackListResponse {
            pagination: self
                .pagination
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            data: self
                .data
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<AttackListResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackListResponse::parse(value, mode)
    }
}
impl fmt::Debug for AttackListResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackListResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackListResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackListResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_05691f00c0beb83ec35109cb`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttackListSubCategory {
    ADVERSARIALSUFFIX,
    EVASION,
    INDIRECTPROMPTINJECTION,
    JAILBREAK,
    MULTITURN,
    PROMPTINJECTION,
    REMOTECODEEXECUTION,
    SYSTEMPROMPTLEAK,
    TOOLLEAK,
    MALWAREGENERATION,
    BIAS,
    CBRN,
    CYBERCRIME,
    DRUGS,
    HATETOXICABUSE,
    NONVIOLENTCRIMES,
    POLITICAL,
    SELFHARM,
    SEXUAL,
    VIOLENTCRIMESWEAPONS,
    COMPETITORENDORSEMENTS,
    BRANDTARNISHINGSELFCRITICISM,
    DISCRIMINATINGCLAIMS,
    POLITICALENDORSEMENTS,
    OWASP,
    MITREATLAS,
    NIST,
    DASFV2,
    Unknown(String),
}
impl AttackListSubCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ADVERSARIALSUFFIX => "ADVERSARIAL_SUFFIX",
            Self::EVASION => "EVASION",
            Self::INDIRECTPROMPTINJECTION => "INDIRECT_PROMPT_INJECTION",
            Self::JAILBREAK => "JAILBREAK",
            Self::MULTITURN => "MULTI_TURN",
            Self::PROMPTINJECTION => "PROMPT_INJECTION",
            Self::REMOTECODEEXECUTION => "REMOTE_CODE_EXECUTION",
            Self::SYSTEMPROMPTLEAK => "SYSTEM_PROMPT_LEAK",
            Self::TOOLLEAK => "TOOL_LEAK",
            Self::MALWAREGENERATION => "MALWARE_GENERATION",
            Self::BIAS => "BIAS",
            Self::CBRN => "CBRN",
            Self::CYBERCRIME => "CYBERCRIME",
            Self::DRUGS => "DRUGS",
            Self::HATETOXICABUSE => "HATE_TOXIC_ABUSE",
            Self::NONVIOLENTCRIMES => "NON_VIOLENT_CRIMES",
            Self::POLITICAL => "POLITICAL",
            Self::SELFHARM => "SELF_HARM",
            Self::SEXUAL => "SEXUAL",
            Self::VIOLENTCRIMESWEAPONS => "VIOLENT_CRIMES_WEAPONS",
            Self::COMPETITORENDORSEMENTS => "COMPETITOR_ENDORSEMENTS",
            Self::BRANDTARNISHINGSELFCRITICISM => "BRAND_TARNISHING_SELF_CRITICISM",
            Self::DISCRIMINATINGCLAIMS => "DISCRIMINATING_CLAIMS",
            Self::POLITICALENDORSEMENTS => "POLITICAL_ENDORSEMENTS",
            Self::OWASP => "OWASP",
            Self::MITREATLAS => "MITRE_ATLAS",
            Self::NIST => "NIST",
            Self::DASFV2 => "DASF_V2",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AttackListSubCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AttackListSubCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "ADVERSARIAL_SUFFIX" => Self::ADVERSARIALSUFFIX,
            "EVASION" => Self::EVASION,
            "INDIRECT_PROMPT_INJECTION" => Self::INDIRECTPROMPTINJECTION,
            "JAILBREAK" => Self::JAILBREAK,
            "MULTI_TURN" => Self::MULTITURN,
            "PROMPT_INJECTION" => Self::PROMPTINJECTION,
            "REMOTE_CODE_EXECUTION" => Self::REMOTECODEEXECUTION,
            "SYSTEM_PROMPT_LEAK" => Self::SYSTEMPROMPTLEAK,
            "TOOL_LEAK" => Self::TOOLLEAK,
            "MALWARE_GENERATION" => Self::MALWAREGENERATION,
            "BIAS" => Self::BIAS,
            "CBRN" => Self::CBRN,
            "CYBERCRIME" => Self::CYBERCRIME,
            "DRUGS" => Self::DRUGS,
            "HATE_TOXIC_ABUSE" => Self::HATETOXICABUSE,
            "NON_VIOLENT_CRIMES" => Self::NONVIOLENTCRIMES,
            "POLITICAL" => Self::POLITICAL,
            "SELF_HARM" => Self::SELFHARM,
            "SEXUAL" => Self::SEXUAL,
            "VIOLENT_CRIMES_WEAPONS" => Self::VIOLENTCRIMESWEAPONS,
            "COMPETITOR_ENDORSEMENTS" => Self::COMPETITORENDORSEMENTS,
            "BRAND_TARNISHING_SELF_CRITICISM" => Self::BRANDTARNISHINGSELFCRITICISM,
            "DISCRIMINATING_CLAIMS" => Self::DISCRIMINATINGCLAIMS,
            "POLITICAL_ENDORSEMENTS" => Self::POLITICALENDORSEMENTS,
            "OWASP" => Self::OWASP,
            "MITRE_ATLAS" => Self::MITREATLAS,
            "NIST" => Self::NIST,
            "DASF_V2" => Self::DASFV2,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AttackListSubCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackListSubCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_e0d8608da704c2f0430a76d8`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackMultiTurnDetailResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "target_id")]
    pub target_id: String,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "status", default)]
    pub status: Optional<AttackStatus>,
    #[serde(rename = "marked_safe", default)]
    pub marked_safe: Patch<bool>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Optional<AttackMultiTurnDetailResponseExtraInfo>,
    #[serde(rename = "threat", default)]
    pub threat: Patch<bool>,
    #[serde(rename = "attack_type", default)]
    pub attack_type: Optional<AttackType>,
    #[serde(rename = "multi_turn", default)]
    pub multi_turn: Optional<bool>,
    #[serde(rename = "asr", default)]
    pub asr: Patch<FiniteNumber>,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(rename = "prompt_mapping_id")]
    pub prompt_mapping_id: String,
    #[serde(rename = "prompt_id")]
    pub prompt_id: String,
    #[serde(rename = "category")]
    pub category: Category,
    #[serde(rename = "sub_category")]
    pub sub_category: AttackMultiTurnDetailResponseSubCategory,
    #[serde(rename = "severity", default)]
    pub severity: Optional<String>,
    #[serde(rename = "threat_details", default)]
    pub threat_details: Optional<ThreatDetails>,
    #[serde(rename = "category_display_name")]
    pub category_display_name: String,
    #[serde(rename = "sub_category_display_name")]
    pub sub_category_display_name: String,
    #[serde(rename = "compliance_frameworks")]
    pub compliance_frameworks: Vec<ComplianceWithTechniqueModel>,
    #[serde(rename = "outputs", default)]
    pub outputs: Optional<Vec<Vec<AttackMultiTurnOutputResponse>>>,
    #[serde(rename = "goal")]
    pub goal: Nullable<String>,
    #[serde(rename = "attack_modality", default)]
    pub attack_modality: Optional<Modality>,
    #[serde(rename = "file", default)]
    pub file: Patch<AttackFile>,
    #[serde(rename = "audio", default)]
    pub audio: Patch<AttackAudio>,
    #[serde(rename = "audio_technique")]
    pub audio_technique: Nullable<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackMultiTurnDetailResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "job_id",
            "target_id",
            "prompt",
            "status",
            "marked_safe",
            "extra_info",
            "threat",
            "attack_type",
            "multi_turn",
            "asr",
            "version",
            "error",
            "prompt_mapping_id",
            "prompt_id",
            "category",
            "sub_category",
            "severity",
            "threat_details",
            "category_display_name",
            "sub_category_display_name",
            "compliance_frameworks",
            "outputs",
            "goal",
            "attack_modality",
            "file",
            "audio",
            "audio_technique",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("target_id", &self.target_id)?;
        map.serialize_entry("prompt", &self.prompt)?;
        if !self.status.is_unset() {
            map.serialize_entry("status", &self.status)?;
        }
        if !self.marked_safe.is_unset() {
            map.serialize_entry("marked_safe", &self.marked_safe)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.attack_type.is_unset() {
            map.serialize_entry("attack_type", &self.attack_type)?;
        }
        if !self.multi_turn.is_unset() {
            map.serialize_entry("multi_turn", &self.multi_turn)?;
        }
        if !self.asr.is_unset() {
            map.serialize_entry("asr", &self.asr)?;
        }
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        map.serialize_entry("prompt_mapping_id", &self.prompt_mapping_id)?;
        map.serialize_entry("prompt_id", &self.prompt_id)?;
        map.serialize_entry("category", &self.category)?;
        map.serialize_entry("sub_category", &self.sub_category)?;
        if !self.severity.is_unset() {
            map.serialize_entry("severity", &self.severity)?;
        }
        if !self.threat_details.is_unset() {
            map.serialize_entry("threat_details", &self.threat_details)?;
        }
        map.serialize_entry("category_display_name", &self.category_display_name)?;
        map.serialize_entry("sub_category_display_name", &self.sub_category_display_name)?;
        map.serialize_entry("compliance_frameworks", &self.compliance_frameworks)?;
        if !self.outputs.is_unset() {
            map.serialize_entry("outputs", &self.outputs)?;
        }
        map.serialize_entry("goal", &self.goal)?;
        if !self.attack_modality.is_unset() {
            map.serialize_entry("attack_modality", &self.attack_modality)?;
        }
        if !self.file.is_unset() {
            map.serialize_entry("file", &self.file)?;
        }
        if !self.audio.is_unset() {
            map.serialize_entry("audio", &self.audio)?;
        }
        map.serialize_entry("audio_technique", &self.audio_technique)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackMultiTurnDetailResponse {
    pub fn builder() -> AttackMultiTurnDetailResponseBuilder {
        AttackMultiTurnDetailResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_e0d8608da704c2f0430a76d8";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackMultiTurnDetailResponseBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    job_id: Option<String>,
    target_id: Option<String>,
    prompt: Option<String>,
    status: Optional<AttackStatus>,
    marked_safe: Patch<bool>,
    extra_info: Optional<AttackMultiTurnDetailResponseExtraInfo>,
    threat: Patch<bool>,
    attack_type: Optional<AttackType>,
    multi_turn: Optional<bool>,
    asr: Patch<FiniteNumber>,
    version: Patch<FiniteNumber>,
    error: Patch<bool>,
    prompt_mapping_id: Option<String>,
    prompt_id: Option<String>,
    category: Option<Category>,
    sub_category: Option<AttackMultiTurnDetailResponseSubCategory>,
    severity: Optional<String>,
    threat_details: Optional<ThreatDetails>,
    category_display_name: Option<String>,
    sub_category_display_name: Option<String>,
    compliance_frameworks: Option<Vec<ComplianceWithTechniqueModel>>,
    outputs: Optional<Vec<Vec<AttackMultiTurnOutputResponse>>>,
    goal: Option<Nullable<String>>,
    attack_modality: Optional<Modality>,
    file: Patch<AttackFile>,
    audio: Patch<AttackAudio>,
    audio_technique: Option<Nullable<String>>,
    extensions: Extensions<JsonValue>,
}
impl AttackMultiTurnDetailResponseBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn target_id(mut self, value: String) -> Self {
        self.target_id = Some(value);
        self
    }
    pub fn prompt(mut self, value: String) -> Self {
        self.prompt = Some(value);
        self
    }
    pub fn status(mut self, value: Optional<AttackStatus>) -> Self {
        self.status = value;
        self
    }
    pub fn marked_safe(mut self, value: Patch<bool>) -> Self {
        self.marked_safe = value;
        self
    }
    pub fn extra_info(mut self, value: Optional<AttackMultiTurnDetailResponseExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn threat(mut self, value: Patch<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn attack_type(mut self, value: Optional<AttackType>) -> Self {
        self.attack_type = value;
        self
    }
    pub fn multi_turn(mut self, value: Optional<bool>) -> Self {
        self.multi_turn = value;
        self
    }
    pub fn asr(mut self, value: Patch<FiniteNumber>) -> Self {
        self.asr = value;
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn prompt_mapping_id(mut self, value: String) -> Self {
        self.prompt_mapping_id = Some(value);
        self
    }
    pub fn prompt_id(mut self, value: String) -> Self {
        self.prompt_id = Some(value);
        self
    }
    pub fn category(mut self, value: Category) -> Self {
        self.category = Some(value);
        self
    }
    pub fn sub_category(mut self, value: AttackMultiTurnDetailResponseSubCategory) -> Self {
        self.sub_category = Some(value);
        self
    }
    pub fn severity(mut self, value: Optional<String>) -> Self {
        self.severity = value;
        self
    }
    pub fn threat_details(mut self, value: Optional<ThreatDetails>) -> Self {
        self.threat_details = value;
        self
    }
    pub fn category_display_name(mut self, value: String) -> Self {
        self.category_display_name = Some(value);
        self
    }
    pub fn sub_category_display_name(mut self, value: String) -> Self {
        self.sub_category_display_name = Some(value);
        self
    }
    pub fn compliance_frameworks(mut self, value: Vec<ComplianceWithTechniqueModel>) -> Self {
        self.compliance_frameworks = Some(value);
        self
    }
    pub fn outputs(mut self, value: Optional<Vec<Vec<AttackMultiTurnOutputResponse>>>) -> Self {
        self.outputs = value;
        self
    }
    pub fn goal(mut self, value: Nullable<String>) -> Self {
        self.goal = Some(value);
        self
    }
    pub fn attack_modality(mut self, value: Optional<Modality>) -> Self {
        self.attack_modality = value;
        self
    }
    pub fn file(mut self, value: Patch<AttackFile>) -> Self {
        self.file = value;
        self
    }
    pub fn audio(mut self, value: Patch<AttackAudio>) -> Self {
        self.audio = value;
        self
    }
    pub fn audio_technique(mut self, value: Nullable<String>) -> Self {
        self.audio_technique = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackMultiTurnDetailResponse> {
        Ok(AttackMultiTurnDetailResponse {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target_id: self
                .target_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt: self
                .prompt
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            status: self.status,
            marked_safe: self.marked_safe,
            extra_info: self.extra_info,
            threat: self.threat,
            attack_type: self.attack_type,
            multi_turn: self.multi_turn,
            asr: self.asr,
            version: self.version,
            error: self.error,
            prompt_mapping_id: self
                .prompt_mapping_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt_id: self
                .prompt_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            category: self
                .category
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            sub_category: self
                .sub_category
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            severity: self.severity,
            threat_details: self.threat_details,
            category_display_name: self
                .category_display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            sub_category_display_name: self
                .sub_category_display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            compliance_frameworks: self
                .compliance_frameworks
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            outputs: self.outputs,
            goal: self
                .goal
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            attack_modality: self.attack_modality,
            file: self.file,
            audio: self.audio,
            audio_technique: self
                .audio_technique
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<AttackMultiTurnDetailResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackMultiTurnDetailResponse::parse(value, mode)
    }
}
impl fmt::Debug for AttackMultiTurnDetailResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackMultiTurnDetailResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackMultiTurnDetailResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackMultiTurnDetailResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_3b3eaaaad562e802c6bf4191`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackMultiTurnDetailResponseExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackMultiTurnDetailResponseExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackMultiTurnDetailResponseExtraInfo {
    pub fn builder() -> AttackMultiTurnDetailResponseExtraInfoBuilder {
        AttackMultiTurnDetailResponseExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_3b3eaaaad562e802c6bf4191";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackMultiTurnDetailResponseExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl AttackMultiTurnDetailResponseExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackMultiTurnDetailResponseExtraInfo> {
        Ok(AttackMultiTurnDetailResponseExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<AttackMultiTurnDetailResponseExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackMultiTurnDetailResponseExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for AttackMultiTurnDetailResponseExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackMultiTurnDetailResponseExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackMultiTurnDetailResponseExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackMultiTurnDetailResponseExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b8755d998e58c71e581ce079`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttackMultiTurnDetailResponseSubCategory {
    ADVERSARIALSUFFIX,
    EVASION,
    INDIRECTPROMPTINJECTION,
    JAILBREAK,
    MULTITURN,
    PROMPTINJECTION,
    REMOTECODEEXECUTION,
    SYSTEMPROMPTLEAK,
    TOOLLEAK,
    MALWAREGENERATION,
    BIAS,
    CBRN,
    CYBERCRIME,
    DRUGS,
    HATETOXICABUSE,
    NONVIOLENTCRIMES,
    POLITICAL,
    SELFHARM,
    SEXUAL,
    VIOLENTCRIMESWEAPONS,
    COMPETITORENDORSEMENTS,
    BRANDTARNISHINGSELFCRITICISM,
    DISCRIMINATINGCLAIMS,
    POLITICALENDORSEMENTS,
    OWASP,
    MITREATLAS,
    NIST,
    DASFV2,
    Unknown(String),
}
impl AttackMultiTurnDetailResponseSubCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ADVERSARIALSUFFIX => "ADVERSARIAL_SUFFIX",
            Self::EVASION => "EVASION",
            Self::INDIRECTPROMPTINJECTION => "INDIRECT_PROMPT_INJECTION",
            Self::JAILBREAK => "JAILBREAK",
            Self::MULTITURN => "MULTI_TURN",
            Self::PROMPTINJECTION => "PROMPT_INJECTION",
            Self::REMOTECODEEXECUTION => "REMOTE_CODE_EXECUTION",
            Self::SYSTEMPROMPTLEAK => "SYSTEM_PROMPT_LEAK",
            Self::TOOLLEAK => "TOOL_LEAK",
            Self::MALWAREGENERATION => "MALWARE_GENERATION",
            Self::BIAS => "BIAS",
            Self::CBRN => "CBRN",
            Self::CYBERCRIME => "CYBERCRIME",
            Self::DRUGS => "DRUGS",
            Self::HATETOXICABUSE => "HATE_TOXIC_ABUSE",
            Self::NONVIOLENTCRIMES => "NON_VIOLENT_CRIMES",
            Self::POLITICAL => "POLITICAL",
            Self::SELFHARM => "SELF_HARM",
            Self::SEXUAL => "SEXUAL",
            Self::VIOLENTCRIMESWEAPONS => "VIOLENT_CRIMES_WEAPONS",
            Self::COMPETITORENDORSEMENTS => "COMPETITOR_ENDORSEMENTS",
            Self::BRANDTARNISHINGSELFCRITICISM => "BRAND_TARNISHING_SELF_CRITICISM",
            Self::DISCRIMINATINGCLAIMS => "DISCRIMINATING_CLAIMS",
            Self::POLITICALENDORSEMENTS => "POLITICAL_ENDORSEMENTS",
            Self::OWASP => "OWASP",
            Self::MITREATLAS => "MITRE_ATLAS",
            Self::NIST => "NIST",
            Self::DASFV2 => "DASF_V2",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AttackMultiTurnDetailResponseSubCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AttackMultiTurnDetailResponseSubCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "ADVERSARIAL_SUFFIX" => Self::ADVERSARIALSUFFIX,
            "EVASION" => Self::EVASION,
            "INDIRECT_PROMPT_INJECTION" => Self::INDIRECTPROMPTINJECTION,
            "JAILBREAK" => Self::JAILBREAK,
            "MULTI_TURN" => Self::MULTITURN,
            "PROMPT_INJECTION" => Self::PROMPTINJECTION,
            "REMOTE_CODE_EXECUTION" => Self::REMOTECODEEXECUTION,
            "SYSTEM_PROMPT_LEAK" => Self::SYSTEMPROMPTLEAK,
            "TOOL_LEAK" => Self::TOOLLEAK,
            "MALWARE_GENERATION" => Self::MALWAREGENERATION,
            "BIAS" => Self::BIAS,
            "CBRN" => Self::CBRN,
            "CYBERCRIME" => Self::CYBERCRIME,
            "DRUGS" => Self::DRUGS,
            "HATE_TOXIC_ABUSE" => Self::HATETOXICABUSE,
            "NON_VIOLENT_CRIMES" => Self::NONVIOLENTCRIMES,
            "POLITICAL" => Self::POLITICAL,
            "SELF_HARM" => Self::SELFHARM,
            "SEXUAL" => Self::SEXUAL,
            "VIOLENT_CRIMES_WEAPONS" => Self::VIOLENTCRIMESWEAPONS,
            "COMPETITOR_ENDORSEMENTS" => Self::COMPETITORENDORSEMENTS,
            "BRAND_TARNISHING_SELF_CRITICISM" => Self::BRANDTARNISHINGSELFCRITICISM,
            "DISCRIMINATING_CLAIMS" => Self::DISCRIMINATINGCLAIMS,
            "POLITICAL_ENDORSEMENTS" => Self::POLITICALENDORSEMENTS,
            "OWASP" => Self::OWASP,
            "MITRE_ATLAS" => Self::MITREATLAS,
            "NIST" => Self::NIST,
            "DASF_V2" => Self::DASFV2,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AttackMultiTurnDetailResponseSubCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackMultiTurnDetailResponseSubCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c42f730011b86eb5e46dba9c`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackMultiTurnOutputResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "attack_id")]
    pub attack_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "target_id")]
    pub target_id: String,
    #[serde(rename = "output")]
    pub output: String,
    #[serde(rename = "threat", default)]
    pub threat: Patch<bool>,
    #[serde(rename = "marked_safe", default)]
    pub marked_safe: Patch<bool>,
    #[serde(rename = "response_audio", default)]
    pub response_audio: Patch<AttackAudio>,
    #[serde(rename = "transcript", default)]
    pub transcript: Patch<String>,
    #[serde(rename = "transcript_source", default)]
    pub transcript_source: Patch<AttackMultiTurnOutputResponseTranscriptSource>,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "turn")]
    pub turn: FiniteNumber,
    #[serde(rename = "generation", default)]
    pub generation: Optional<FiniteNumber>,
    #[serde(rename = "multi_turn", default)]
    pub multi_turn: Optional<bool>,
    #[serde(rename = "threat_details", default)]
    pub threat_details: Optional<ThreatDetails>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(rename = "error_message", default)]
    pub error_message: Patch<String>,
    #[serde(rename = "judge_reasoning")]
    pub judge_reasoning: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackMultiTurnOutputResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "attack_id",
            "job_id",
            "target_id",
            "output",
            "threat",
            "marked_safe",
            "response_audio",
            "transcript",
            "transcript_source",
            "prompt",
            "turn",
            "generation",
            "multi_turn",
            "threat_details",
            "error",
            "error_message",
            "judge_reasoning",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("attack_id", &self.attack_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("target_id", &self.target_id)?;
        map.serialize_entry("output", &self.output)?;
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.marked_safe.is_unset() {
            map.serialize_entry("marked_safe", &self.marked_safe)?;
        }
        if !self.response_audio.is_unset() {
            map.serialize_entry("response_audio", &self.response_audio)?;
        }
        if !self.transcript.is_unset() {
            map.serialize_entry("transcript", &self.transcript)?;
        }
        if !self.transcript_source.is_unset() {
            map.serialize_entry("transcript_source", &self.transcript_source)?;
        }
        map.serialize_entry("prompt", &self.prompt)?;
        map.serialize_entry("turn", &self.turn)?;
        if !self.generation.is_unset() {
            map.serialize_entry("generation", &self.generation)?;
        }
        if !self.multi_turn.is_unset() {
            map.serialize_entry("multi_turn", &self.multi_turn)?;
        }
        if !self.threat_details.is_unset() {
            map.serialize_entry("threat_details", &self.threat_details)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        if !self.error_message.is_unset() {
            map.serialize_entry("error_message", &self.error_message)?;
        }
        map.serialize_entry("judge_reasoning", &self.judge_reasoning)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackMultiTurnOutputResponse {
    pub fn builder() -> AttackMultiTurnOutputResponseBuilder {
        AttackMultiTurnOutputResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_c42f730011b86eb5e46dba9c";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackMultiTurnOutputResponseBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    attack_id: Option<String>,
    job_id: Option<String>,
    target_id: Option<String>,
    output: Option<String>,
    threat: Patch<bool>,
    marked_safe: Patch<bool>,
    response_audio: Patch<AttackAudio>,
    transcript: Patch<String>,
    transcript_source: Patch<AttackMultiTurnOutputResponseTranscriptSource>,
    prompt: Option<String>,
    turn: Option<FiniteNumber>,
    generation: Optional<FiniteNumber>,
    multi_turn: Optional<bool>,
    threat_details: Optional<ThreatDetails>,
    error: Patch<bool>,
    error_message: Patch<String>,
    judge_reasoning: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl AttackMultiTurnOutputResponseBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn attack_id(mut self, value: String) -> Self {
        self.attack_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn target_id(mut self, value: String) -> Self {
        self.target_id = Some(value);
        self
    }
    pub fn output(mut self, value: String) -> Self {
        self.output = Some(value);
        self
    }
    pub fn threat(mut self, value: Patch<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn marked_safe(mut self, value: Patch<bool>) -> Self {
        self.marked_safe = value;
        self
    }
    pub fn response_audio(mut self, value: Patch<AttackAudio>) -> Self {
        self.response_audio = value;
        self
    }
    pub fn transcript(mut self, value: Patch<String>) -> Self {
        self.transcript = value;
        self
    }
    pub fn transcript_source(
        mut self,
        value: Patch<AttackMultiTurnOutputResponseTranscriptSource>,
    ) -> Self {
        self.transcript_source = value;
        self
    }
    pub fn prompt(mut self, value: String) -> Self {
        self.prompt = Some(value);
        self
    }
    pub fn turn(mut self, value: FiniteNumber) -> Self {
        self.turn = Some(value);
        self
    }
    pub fn generation(mut self, value: Optional<FiniteNumber>) -> Self {
        self.generation = value;
        self
    }
    pub fn multi_turn(mut self, value: Optional<bool>) -> Self {
        self.multi_turn = value;
        self
    }
    pub fn threat_details(mut self, value: Optional<ThreatDetails>) -> Self {
        self.threat_details = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn error_message(mut self, value: Patch<String>) -> Self {
        self.error_message = value;
        self
    }
    pub fn judge_reasoning(mut self, value: String) -> Self {
        self.judge_reasoning = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackMultiTurnOutputResponse> {
        Ok(AttackMultiTurnOutputResponse {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            attack_id: self
                .attack_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target_id: self
                .target_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            output: self
                .output
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            threat: self.threat,
            marked_safe: self.marked_safe,
            response_audio: self.response_audio,
            transcript: self.transcript,
            transcript_source: self.transcript_source,
            prompt: self
                .prompt
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            turn: self
                .turn
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            generation: self.generation,
            multi_turn: self.multi_turn,
            threat_details: self.threat_details,
            error: self.error,
            error_message: self.error_message,
            judge_reasoning: self
                .judge_reasoning
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<AttackMultiTurnOutputResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackMultiTurnOutputResponse::parse(value, mode)
    }
}
impl fmt::Debug for AttackMultiTurnOutputResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackMultiTurnOutputResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackMultiTurnOutputResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackMultiTurnOutputResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_0c120bce73f89c5cf785ad38`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttackMultiTurnOutputResponseTranscriptSource {
    TARGET,
    STT,
    Unknown(String),
}
impl AttackMultiTurnOutputResponseTranscriptSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TARGET => "TARGET",
            Self::STT => "STT",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AttackMultiTurnOutputResponseTranscriptSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AttackMultiTurnOutputResponseTranscriptSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "TARGET" => Self::TARGET,
            "STT" => Self::STT,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AttackMultiTurnOutputResponseTranscriptSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackMultiTurnOutputResponseTranscriptSource([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_f1b98a1688c99e03d1afedbd`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct AttackOutputResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "attack_id")]
    pub attack_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "target_id")]
    pub target_id: String,
    #[serde(rename = "output")]
    pub output: String,
    #[serde(rename = "threat", default)]
    pub threat: Patch<bool>,
    #[serde(rename = "marked_safe", default)]
    pub marked_safe: Patch<bool>,
    #[serde(rename = "response_audio", default)]
    pub response_audio: Patch<AttackAudio>,
    #[serde(rename = "transcript", default)]
    pub transcript: Patch<String>,
    #[serde(rename = "transcript_source", default)]
    pub transcript_source: Patch<AttackOutputResponseTranscriptSource>,
    #[serde(rename = "threat_details", default)]
    pub threat_details: Optional<ThreatDetails>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(rename = "error_message", default)]
    pub error_message: Patch<String>,
    #[serde(rename = "judge_reasoning")]
    pub judge_reasoning: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for AttackOutputResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "attack_id",
            "job_id",
            "target_id",
            "output",
            "threat",
            "marked_safe",
            "response_audio",
            "transcript",
            "transcript_source",
            "threat_details",
            "error",
            "error_message",
            "judge_reasoning",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("attack_id", &self.attack_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("target_id", &self.target_id)?;
        map.serialize_entry("output", &self.output)?;
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.marked_safe.is_unset() {
            map.serialize_entry("marked_safe", &self.marked_safe)?;
        }
        if !self.response_audio.is_unset() {
            map.serialize_entry("response_audio", &self.response_audio)?;
        }
        if !self.transcript.is_unset() {
            map.serialize_entry("transcript", &self.transcript)?;
        }
        if !self.transcript_source.is_unset() {
            map.serialize_entry("transcript_source", &self.transcript_source)?;
        }
        if !self.threat_details.is_unset() {
            map.serialize_entry("threat_details", &self.threat_details)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        if !self.error_message.is_unset() {
            map.serialize_entry("error_message", &self.error_message)?;
        }
        map.serialize_entry("judge_reasoning", &self.judge_reasoning)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl AttackOutputResponse {
    pub fn builder() -> AttackOutputResponseBuilder {
        AttackOutputResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_f1b98a1688c99e03d1afedbd";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct AttackOutputResponseBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    attack_id: Option<String>,
    job_id: Option<String>,
    target_id: Option<String>,
    output: Option<String>,
    threat: Patch<bool>,
    marked_safe: Patch<bool>,
    response_audio: Patch<AttackAudio>,
    transcript: Patch<String>,
    transcript_source: Patch<AttackOutputResponseTranscriptSource>,
    threat_details: Optional<ThreatDetails>,
    error: Patch<bool>,
    error_message: Patch<String>,
    judge_reasoning: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl AttackOutputResponseBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn attack_id(mut self, value: String) -> Self {
        self.attack_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn target_id(mut self, value: String) -> Self {
        self.target_id = Some(value);
        self
    }
    pub fn output(mut self, value: String) -> Self {
        self.output = Some(value);
        self
    }
    pub fn threat(mut self, value: Patch<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn marked_safe(mut self, value: Patch<bool>) -> Self {
        self.marked_safe = value;
        self
    }
    pub fn response_audio(mut self, value: Patch<AttackAudio>) -> Self {
        self.response_audio = value;
        self
    }
    pub fn transcript(mut self, value: Patch<String>) -> Self {
        self.transcript = value;
        self
    }
    pub fn transcript_source(mut self, value: Patch<AttackOutputResponseTranscriptSource>) -> Self {
        self.transcript_source = value;
        self
    }
    pub fn threat_details(mut self, value: Optional<ThreatDetails>) -> Self {
        self.threat_details = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn error_message(mut self, value: Patch<String>) -> Self {
        self.error_message = value;
        self
    }
    pub fn judge_reasoning(mut self, value: String) -> Self {
        self.judge_reasoning = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<AttackOutputResponse> {
        Ok(AttackOutputResponse {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            attack_id: self
                .attack_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target_id: self
                .target_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            output: self
                .output
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            threat: self.threat,
            marked_safe: self.marked_safe,
            response_audio: self.response_audio,
            transcript: self.transcript,
            transcript_source: self.transcript_source,
            threat_details: self.threat_details,
            error: self.error,
            error_message: self.error_message,
            judge_reasoning: self
                .judge_reasoning
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<AttackOutputResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        AttackOutputResponse::parse(value, mode)
    }
}
impl fmt::Debug for AttackOutputResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackOutputResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for AttackOutputResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackOutputResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_03fd766dfd18c319a12d92eb`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttackOutputResponseTranscriptSource {
    TARGET,
    STT,
    Unknown(String),
}
impl AttackOutputResponseTranscriptSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TARGET => "TARGET",
            Self::STT => "STT",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AttackOutputResponseTranscriptSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AttackOutputResponseTranscriptSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "TARGET" => Self::TARGET,
            "STT" => Self::STT,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AttackOutputResponseTranscriptSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackOutputResponseTranscriptSource([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_9c54750581690fb70a0409ab`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttackStatus {
    INIT,
    ATTACK,
    DETECTION,
    REPORT,
    COMPLETED,
    FAILED,
    Unknown(String),
}
impl AttackStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::INIT => "INIT",
            Self::ATTACK => "ATTACK",
            Self::DETECTION => "DETECTION",
            Self::REPORT => "REPORT",
            Self::COMPLETED => "COMPLETED",
            Self::FAILED => "FAILED",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AttackStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AttackStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "INIT" => Self::INIT,
            "ATTACK" => Self::ATTACK,
            "DETECTION" => Self::DETECTION,
            "REPORT" => Self::REPORT,
            "COMPLETED" => Self::COMPLETED,
            "FAILED" => Self::FAILED,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AttackStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackStatus([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_d69834879ab5461534e0335d`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttackType {
    NORMAL,
    CUSTOM,
    Unknown(String),
}
impl AttackType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NORMAL => "NORMAL",
            Self::CUSTOM => "CUSTOM",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AttackType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AttackType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "NORMAL" => Self::NORMAL,
            "CUSTOM" => Self::CUSTOM,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AttackType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttackType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_2f056c14d98cca06cf8de502`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AttemptsUnit {
    Attacks,
    Streams,
    Unknown(String),
}
impl AttemptsUnit {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Attacks => "attacks",
            Self::Streams => "streams",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AttemptsUnit {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AttemptsUnit {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "attacks" => Self::Attacks,
            "streams" => Self::Streams,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AttemptsUnit {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AttemptsUnit([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b2b7c4a204dc564d5cd1b6ff`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AudioFormat {
    WAV,
    MP3,
    PCM16,
    Unknown(String),
}
impl AudioFormat {
    pub fn as_str(&self) -> &str {
        match self {
            Self::WAV => "WAV",
            Self::MP3 => "MP3",
            Self::PCM16 => "PCM16",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AudioFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AudioFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "WAV" => Self::WAV,
            "MP3" => Self::MP3,
            "PCM16" => Self::PCM16,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AudioFormat([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_d6971ffd4897344273865789`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AuthType {
    HEADERS,
    BASICAUTH,
    OAUTH2,
    CUSTOMTARGETADAPTER,
    Unknown(String),
}
impl AuthType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::HEADERS => "HEADERS",
            Self::BASICAUTH => "BASIC_AUTH",
            Self::OAUTH2 => "OAUTH2",
            Self::CUSTOMTARGETADAPTER => "CUSTOM_TARGET_ADAPTER",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for AuthType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for AuthType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "HEADERS" => Self::HEADERS,
            "BASIC_AUTH" => Self::BASICAUTH,
            "OAUTH2" => Self::OAUTH2,
            "CUSTOM_TARGET_ADAPTER" => Self::CUSTOMTARGETADAPTER,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for AuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("AuthType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c92f28b8fd987860b675bcc7`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BackgroundJobStatus {
    INIT,
    QUEUED,
    RUNNING,
    COMPLETED,
    PARTIALLYCOMPLETE,
    FAILED,
    ABORTED,
    Unknown(String),
}
impl BackgroundJobStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::INIT => "INIT",
            Self::QUEUED => "QUEUED",
            Self::RUNNING => "RUNNING",
            Self::COMPLETED => "COMPLETED",
            Self::PARTIALLYCOMPLETE => "PARTIALLY_COMPLETE",
            Self::FAILED => "FAILED",
            Self::ABORTED => "ABORTED",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for BackgroundJobStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for BackgroundJobStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "INIT" => Self::INIT,
            "QUEUED" => Self::QUEUED,
            "RUNNING" => Self::RUNNING,
            "COMPLETED" => Self::COMPLETED,
            "PARTIALLY_COMPLETE" => Self::PARTIALLYCOMPLETE,
            "FAILED" => Self::FAILED,
            "ABORTED" => Self::ABORTED,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for BackgroundJobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BackgroundJobStatus([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_cb88d91c37a197fbe9a27f11`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BaseSubtype {
    REMOTECODEEXECUTION,
    SYSTEMPROMPTLEAK,
    TOOLLEAK,
    MALWAREGENERATION,
    BIAS,
    CBRN,
    CYBERCRIME,
    DRUGS,
    HATETOXICABUSE,
    NONVIOLENTCRIMES,
    POLITICAL,
    SELFHARM,
    SEXUAL,
    VIOLENTCRIMESWEAPONS,
    SENSITIVEDATALEAKAGE,
    COMPETITORENDORSEMENTS,
    BRANDTARNISHINGSELFCRITICISM,
    Unknown(String),
}
impl BaseSubtype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::REMOTECODEEXECUTION => "REMOTE_CODE_EXECUTION",
            Self::SYSTEMPROMPTLEAK => "SYSTEM_PROMPT_LEAK",
            Self::TOOLLEAK => "TOOL_LEAK",
            Self::MALWAREGENERATION => "MALWARE_GENERATION",
            Self::BIAS => "BIAS",
            Self::CBRN => "CBRN",
            Self::CYBERCRIME => "CYBERCRIME",
            Self::DRUGS => "DRUGS",
            Self::HATETOXICABUSE => "HATE_TOXIC_ABUSE",
            Self::NONVIOLENTCRIMES => "NON_VIOLENT_CRIMES",
            Self::POLITICAL => "POLITICAL",
            Self::SELFHARM => "SELF_HARM",
            Self::SEXUAL => "SEXUAL",
            Self::VIOLENTCRIMESWEAPONS => "VIOLENT_CRIMES_WEAPONS",
            Self::SENSITIVEDATALEAKAGE => "SENSITIVE_DATA_LEAKAGE",
            Self::COMPETITORENDORSEMENTS => "COMPETITOR_ENDORSEMENTS",
            Self::BRANDTARNISHINGSELFCRITICISM => "BRAND_TARNISHING_SELF_CRITICISM",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for BaseSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for BaseSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "REMOTE_CODE_EXECUTION" => Self::REMOTECODEEXECUTION,
            "SYSTEM_PROMPT_LEAK" => Self::SYSTEMPROMPTLEAK,
            "TOOL_LEAK" => Self::TOOLLEAK,
            "MALWARE_GENERATION" => Self::MALWAREGENERATION,
            "BIAS" => Self::BIAS,
            "CBRN" => Self::CBRN,
            "CYBERCRIME" => Self::CYBERCRIME,
            "DRUGS" => Self::DRUGS,
            "HATE_TOXIC_ABUSE" => Self::HATETOXICABUSE,
            "NON_VIOLENT_CRIMES" => Self::NONVIOLENTCRIMES,
            "POLITICAL" => Self::POLITICAL,
            "SELF_HARM" => Self::SELFHARM,
            "SEXUAL" => Self::SEXUAL,
            "VIOLENT_CRIMES_WEAPONS" => Self::VIOLENTCRIMESWEAPONS,
            "SENSITIVE_DATA_LEAKAGE" => Self::SENSITIVEDATALEAKAGE,
            "COMPETITOR_ENDORSEMENTS" => Self::COMPETITORENDORSEMENTS,
            "BRAND_TARNISHING_SELF_CRITICISM" => Self::BRANDTARNISHINGSELFCRITICISM,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for BaseSubtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BaseSubtype([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_92a61da72b73667b1707fd02`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum BrandSubCategory {
    COMPETITORENDORSEMENTS,
    BRANDTARNISHINGSELFCRITICISM,
    DISCRIMINATINGCLAIMS,
    POLITICALENDORSEMENTS,
    Unknown(String),
}
impl BrandSubCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::COMPETITORENDORSEMENTS => "COMPETITOR_ENDORSEMENTS",
            Self::BRANDTARNISHINGSELFCRITICISM => "BRAND_TARNISHING_SELF_CRITICISM",
            Self::DISCRIMINATINGCLAIMS => "DISCRIMINATING_CLAIMS",
            Self::POLITICALENDORSEMENTS => "POLITICAL_ENDORSEMENTS",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for BrandSubCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for BrandSubCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "COMPETITOR_ENDORSEMENTS" => Self::COMPETITORENDORSEMENTS,
            "BRAND_TARNISHING_SELF_CRITICISM" => Self::BRANDTARNISHINGSELFCRITICISM,
            "DISCRIMINATING_CLAIMS" => Self::DISCRIMINATINGCLAIMS,
            "POLITICAL_ENDORSEMENTS" => Self::POLITICALENDORSEMENTS,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for BrandSubCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("BrandSubCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c735d2d7e2eb94a9c149816b`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Category {
    SECURITY,
    SAFETY,
    COMPLIANCE,
    BRAND,
    Unknown(String),
}
impl Category {
    pub fn as_str(&self) -> &str {
        match self {
            Self::SECURITY => "SECURITY",
            Self::SAFETY => "SAFETY",
            Self::COMPLIANCE => "COMPLIANCE",
            Self::BRAND => "BRAND",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for Category {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for Category {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "SECURITY" => Self::SECURITY,
            "SAFETY" => Self::SAFETY,
            "COMPLIANCE" => Self::COMPLIANCE,
            "BRAND" => Self::BRAND,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for Category {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Category([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_ae6f9e9ae6f8143afc5fab1e`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct CategoryModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "preselect", default)]
    pub preselect: Optional<bool>,
    #[serde(rename = "sub_categories")]
    pub sub_categories: Vec<SubCategoryModel>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for CategoryModel {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "id",
            "display_name",
            "description",
            "preselect",
            "sub_categories",
        ];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("description", &self.description)?;
        if !self.preselect.is_unset() {
            map.serialize_entry("preselect", &self.preselect)?;
        }
        map.serialize_entry("sub_categories", &self.sub_categories)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CategoryModel {
    pub fn builder() -> CategoryModelBuilder {
        CategoryModelBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_ae6f9e9ae6f8143afc5fab1e";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct CategoryModelBuilder {
    id: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    preselect: Optional<bool>,
    sub_categories: Option<Vec<SubCategoryModel>>,
    extensions: Extensions<JsonValue>,
}
impl CategoryModelBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn preselect(mut self, value: Optional<bool>) -> Self {
        self.preselect = value;
        self
    }
    pub fn sub_categories(mut self, value: Vec<SubCategoryModel>) -> Self {
        self.sub_categories = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<CategoryModel> {
        Ok(CategoryModel {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            preselect: self.preselect,
            sub_categories: self
                .sub_categories
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<CategoryModel> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        CategoryModel::parse(value, mode)
    }
}
impl fmt::Debug for CategoryModelBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CategoryModelBuilder([REDACTED])")
    }
}
impl fmt::Debug for CategoryModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CategoryModel([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_dea9b68baee2a18ea5fbc53d`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct CategoryReport {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "preselect", default)]
    pub preselect: Optional<bool>,
    #[serde(rename = "sub_categories")]
    pub sub_categories: Vec<SubCategoryStats>,
    #[serde(rename = "asr")]
    pub asr: FiniteNumber,
    #[serde(rename = "total_prompts")]
    pub total_prompts: FiniteNumber,
    #[serde(rename = "total_attacks")]
    pub total_attacks: FiniteNumber,
    #[serde(rename = "successful")]
    pub successful: FiniteNumber,
    #[serde(rename = "failed")]
    pub failed: FiniteNumber,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for CategoryReport {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "id",
            "display_name",
            "description",
            "preselect",
            "sub_categories",
            "asr",
            "total_prompts",
            "total_attacks",
            "successful",
            "failed",
        ];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("description", &self.description)?;
        if !self.preselect.is_unset() {
            map.serialize_entry("preselect", &self.preselect)?;
        }
        map.serialize_entry("sub_categories", &self.sub_categories)?;
        map.serialize_entry("asr", &self.asr)?;
        map.serialize_entry("total_prompts", &self.total_prompts)?;
        map.serialize_entry("total_attacks", &self.total_attacks)?;
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("failed", &self.failed)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CategoryReport {
    pub fn builder() -> CategoryReportBuilder {
        CategoryReportBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_dea9b68baee2a18ea5fbc53d";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct CategoryReportBuilder {
    id: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    preselect: Optional<bool>,
    sub_categories: Option<Vec<SubCategoryStats>>,
    asr: Option<FiniteNumber>,
    total_prompts: Option<FiniteNumber>,
    total_attacks: Option<FiniteNumber>,
    successful: Option<FiniteNumber>,
    failed: Option<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl CategoryReportBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn preselect(mut self, value: Optional<bool>) -> Self {
        self.preselect = value;
        self
    }
    pub fn sub_categories(mut self, value: Vec<SubCategoryStats>) -> Self {
        self.sub_categories = Some(value);
        self
    }
    pub fn asr(mut self, value: FiniteNumber) -> Self {
        self.asr = Some(value);
        self
    }
    pub fn total_prompts(mut self, value: FiniteNumber) -> Self {
        self.total_prompts = Some(value);
        self
    }
    pub fn total_attacks(mut self, value: FiniteNumber) -> Self {
        self.total_attacks = Some(value);
        self
    }
    pub fn successful(mut self, value: FiniteNumber) -> Self {
        self.successful = Some(value);
        self
    }
    pub fn failed(mut self, value: FiniteNumber) -> Self {
        self.failed = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<CategoryReport> {
        Ok(CategoryReport {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            preselect: self.preselect,
            sub_categories: self
                .sub_categories
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            asr: self
                .asr
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_prompts: self
                .total_prompts
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_attacks: self
                .total_attacks
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            successful: self
                .successful
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            failed: self
                .failed
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<CategoryReport> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        CategoryReport::parse(value, mode)
    }
}
impl fmt::Debug for CategoryReportBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CategoryReportBuilder([REDACTED])")
    }
}
impl fmt::Debug for CategoryReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CategoryReport([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_4e2da160f6075970aacc2c3f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ClaraJobMetadata {
    #[serde(rename = "scan_name")]
    pub scan_name: String,
    #[serde(rename = "categories", default)]
    pub categories: Optional<TypedMap<Vec<ClaraJobMetadataCategoriesValueItem>>>,
    #[serde(rename = "language", default)]
    pub language: Patch<()>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ClaraJobMetadata {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["scan_name", "categories", "language"];
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
        map.serialize_entry("scan_name", &self.scan_name)?;
        if !self.categories.is_unset() {
            map.serialize_entry("categories", &self.categories)?;
        }
        if !self.language.is_unset() {
            map.serialize_entry("language", &self.language)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ClaraJobMetadata {
    pub fn builder() -> ClaraJobMetadataBuilder {
        ClaraJobMetadataBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_4e2da160f6075970aacc2c3f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ClaraJobMetadataBuilder {
    scan_name: Option<String>,
    categories: Optional<TypedMap<Vec<ClaraJobMetadataCategoriesValueItem>>>,
    language: Patch<()>,
    extensions: Extensions<JsonValue>,
}
impl ClaraJobMetadataBuilder {
    pub fn scan_name(mut self, value: String) -> Self {
        self.scan_name = Some(value);
        self
    }
    pub fn categories(
        mut self,
        value: Optional<TypedMap<Vec<ClaraJobMetadataCategoriesValueItem>>>,
    ) -> Self {
        self.categories = value;
        self
    }
    pub fn language(mut self, value: Patch<()>) -> Self {
        self.language = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ClaraJobMetadata> {
        Ok(ClaraJobMetadata {
            scan_name: self
                .scan_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            categories: self.categories,
            language: self.language,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<ClaraJobMetadata> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ClaraJobMetadata::parse(value, mode)
    }
}
impl fmt::Debug for ClaraJobMetadataBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ClaraJobMetadataBuilder([REDACTED])")
    }
}
impl fmt::Debug for ClaraJobMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ClaraJobMetadata([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_cc724ce3f8040856dfdc9723`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ClaraJobMetadataCategoriesValueItem {
    ADVERSARIALSUFFIX,
    EVASION,
    INDIRECTPROMPTINJECTION,
    JAILBREAK,
    MULTITURN,
    PROMPTINJECTION,
    REMOTECODEEXECUTION,
    SYSTEMPROMPTLEAK,
    TOOLLEAK,
    MALWAREGENERATION,
    BIAS,
    CBRN,
    CYBERCRIME,
    DRUGS,
    HATETOXICABUSE,
    NONVIOLENTCRIMES,
    POLITICAL,
    SELFHARM,
    SEXUAL,
    VIOLENTCRIMESWEAPONS,
    COMPETITORENDORSEMENTS,
    BRANDTARNISHINGSELFCRITICISM,
    DISCRIMINATINGCLAIMS,
    POLITICALENDORSEMENTS,
    OWASP,
    MITREATLAS,
    NIST,
    DASFV2,
    Unknown(String),
}
impl ClaraJobMetadataCategoriesValueItem {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ADVERSARIALSUFFIX => "ADVERSARIAL_SUFFIX",
            Self::EVASION => "EVASION",
            Self::INDIRECTPROMPTINJECTION => "INDIRECT_PROMPT_INJECTION",
            Self::JAILBREAK => "JAILBREAK",
            Self::MULTITURN => "MULTI_TURN",
            Self::PROMPTINJECTION => "PROMPT_INJECTION",
            Self::REMOTECODEEXECUTION => "REMOTE_CODE_EXECUTION",
            Self::SYSTEMPROMPTLEAK => "SYSTEM_PROMPT_LEAK",
            Self::TOOLLEAK => "TOOL_LEAK",
            Self::MALWAREGENERATION => "MALWARE_GENERATION",
            Self::BIAS => "BIAS",
            Self::CBRN => "CBRN",
            Self::CYBERCRIME => "CYBERCRIME",
            Self::DRUGS => "DRUGS",
            Self::HATETOXICABUSE => "HATE_TOXIC_ABUSE",
            Self::NONVIOLENTCRIMES => "NON_VIOLENT_CRIMES",
            Self::POLITICAL => "POLITICAL",
            Self::SELFHARM => "SELF_HARM",
            Self::SEXUAL => "SEXUAL",
            Self::VIOLENTCRIMESWEAPONS => "VIOLENT_CRIMES_WEAPONS",
            Self::COMPETITORENDORSEMENTS => "COMPETITOR_ENDORSEMENTS",
            Self::BRANDTARNISHINGSELFCRITICISM => "BRAND_TARNISHING_SELF_CRITICISM",
            Self::DISCRIMINATINGCLAIMS => "DISCRIMINATING_CLAIMS",
            Self::POLITICALENDORSEMENTS => "POLITICAL_ENDORSEMENTS",
            Self::OWASP => "OWASP",
            Self::MITREATLAS => "MITRE_ATLAS",
            Self::NIST => "NIST",
            Self::DASFV2 => "DASF_V2",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ClaraJobMetadataCategoriesValueItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ClaraJobMetadataCategoriesValueItem {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "ADVERSARIAL_SUFFIX" => Self::ADVERSARIALSUFFIX,
            "EVASION" => Self::EVASION,
            "INDIRECT_PROMPT_INJECTION" => Self::INDIRECTPROMPTINJECTION,
            "JAILBREAK" => Self::JAILBREAK,
            "MULTI_TURN" => Self::MULTITURN,
            "PROMPT_INJECTION" => Self::PROMPTINJECTION,
            "REMOTE_CODE_EXECUTION" => Self::REMOTECODEEXECUTION,
            "SYSTEM_PROMPT_LEAK" => Self::SYSTEMPROMPTLEAK,
            "TOOL_LEAK" => Self::TOOLLEAK,
            "MALWARE_GENERATION" => Self::MALWAREGENERATION,
            "BIAS" => Self::BIAS,
            "CBRN" => Self::CBRN,
            "CYBERCRIME" => Self::CYBERCRIME,
            "DRUGS" => Self::DRUGS,
            "HATE_TOXIC_ABUSE" => Self::HATETOXICABUSE,
            "NON_VIOLENT_CRIMES" => Self::NONVIOLENTCRIMES,
            "POLITICAL" => Self::POLITICAL,
            "SELF_HARM" => Self::SELFHARM,
            "SEXUAL" => Self::SEXUAL,
            "VIOLENT_CRIMES_WEAPONS" => Self::VIOLENTCRIMESWEAPONS,
            "COMPETITOR_ENDORSEMENTS" => Self::COMPETITORENDORSEMENTS,
            "BRAND_TARNISHING_SELF_CRITICISM" => Self::BRANDTARNISHINGSELFCRITICISM,
            "DISCRIMINATING_CLAIMS" => Self::DISCRIMINATINGCLAIMS,
            "POLITICAL_ENDORSEMENTS" => Self::POLITICALENDORSEMENTS,
            "OWASP" => Self::OWASP,
            "MITRE_ATLAS" => Self::MITREATLAS,
            "NIST" => Self::NIST,
            "DASF_V2" => Self::DASFV2,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ClaraJobMetadataCategoriesValueItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ClaraJobMetadataCategoriesValueItem([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_05be42ef39f26473abd27020`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ComplianceReport {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "link")]
    pub link: String,
    #[serde(rename = "techniques")]
    pub techniques: Vec<ComplianceTechnique>,
    #[serde(rename = "score", default)]
    pub score: Optional<FiniteNumber>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ComplianceReport {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "id",
            "display_name",
            "description",
            "active",
            "version",
            "link",
            "techniques",
            "score",
        ];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("active", &self.active)?;
        map.serialize_entry("version", &self.version)?;
        map.serialize_entry("link", &self.link)?;
        map.serialize_entry("techniques", &self.techniques)?;
        if !self.score.is_unset() {
            map.serialize_entry("score", &self.score)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ComplianceReport {
    pub fn builder() -> ComplianceReportBuilder {
        ComplianceReportBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_05be42ef39f26473abd27020";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ComplianceReportBuilder {
    id: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    active: Option<bool>,
    version: Option<String>,
    link: Option<String>,
    techniques: Option<Vec<ComplianceTechnique>>,
    score: Optional<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl ComplianceReportBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }
    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
    pub fn link(mut self, value: String) -> Self {
        self.link = Some(value);
        self
    }
    pub fn techniques(mut self, value: Vec<ComplianceTechnique>) -> Self {
        self.techniques = Some(value);
        self
    }
    pub fn score(mut self, value: Optional<FiniteNumber>) -> Self {
        self.score = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ComplianceReport> {
        Ok(ComplianceReport {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            active: self
                .active
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            version: self
                .version
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            link: self
                .link
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            techniques: self
                .techniques
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            score: self.score,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<ComplianceReport> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ComplianceReport::parse(value, mode)
    }
}
impl fmt::Debug for ComplianceReportBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ComplianceReportBuilder([REDACTED])")
    }
}
impl fmt::Debug for ComplianceReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ComplianceReport([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_9b2d9c9e88e1691d081aa31a`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ComplianceSubCategory {
    OWASP,
    MITREATLAS,
    NIST,
    DASFV2,
    Unknown(String),
}
impl ComplianceSubCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::OWASP => "OWASP",
            Self::MITREATLAS => "MITRE_ATLAS",
            Self::NIST => "NIST",
            Self::DASFV2 => "DASF_V2",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ComplianceSubCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ComplianceSubCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "OWASP" => Self::OWASP,
            "MITRE_ATLAS" => Self::MITREATLAS,
            "NIST" => Self::NIST,
            "DASF_V2" => Self::DASFV2,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ComplianceSubCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ComplianceSubCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_a373266736a5d20cee2b6e8a`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ComplianceTechnique {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "compliance_id")]
    pub compliance_id: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "link")]
    pub link: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "successful", default)]
    pub successful: Optional<FiniteNumber>,
    #[serde(rename = "failed", default)]
    pub failed: Optional<FiniteNumber>,
    #[serde(rename = "total", default)]
    pub total: Optional<FiniteNumber>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ComplianceTechnique {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "id",
            "display_name",
            "compliance_id",
            "description",
            "link",
            "version",
            "active",
            "successful",
            "failed",
            "total",
        ];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("compliance_id", &self.compliance_id)?;
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("link", &self.link)?;
        map.serialize_entry("version", &self.version)?;
        map.serialize_entry("active", &self.active)?;
        if !self.successful.is_unset() {
            map.serialize_entry("successful", &self.successful)?;
        }
        if !self.failed.is_unset() {
            map.serialize_entry("failed", &self.failed)?;
        }
        if !self.total.is_unset() {
            map.serialize_entry("total", &self.total)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ComplianceTechnique {
    pub fn builder() -> ComplianceTechniqueBuilder {
        ComplianceTechniqueBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_a373266736a5d20cee2b6e8a";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ComplianceTechniqueBuilder {
    id: Option<String>,
    display_name: Option<String>,
    compliance_id: Option<String>,
    description: Option<String>,
    link: Option<String>,
    version: Option<String>,
    active: Option<bool>,
    successful: Optional<FiniteNumber>,
    failed: Optional<FiniteNumber>,
    total: Optional<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl ComplianceTechniqueBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn compliance_id(mut self, value: String) -> Self {
        self.compliance_id = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn link(mut self, value: String) -> Self {
        self.link = Some(value);
        self
    }
    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }
    pub fn successful(mut self, value: Optional<FiniteNumber>) -> Self {
        self.successful = value;
        self
    }
    pub fn failed(mut self, value: Optional<FiniteNumber>) -> Self {
        self.failed = value;
        self
    }
    pub fn total(mut self, value: Optional<FiniteNumber>) -> Self {
        self.total = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ComplianceTechnique> {
        Ok(ComplianceTechnique {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            compliance_id: self
                .compliance_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            link: self
                .link
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            version: self
                .version
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            active: self
                .active
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            successful: self.successful,
            failed: self.failed,
            total: self.total,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ComplianceTechnique> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ComplianceTechnique::parse(value, mode)
    }
}
impl fmt::Debug for ComplianceTechniqueBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ComplianceTechniqueBuilder([REDACTED])")
    }
}
impl fmt::Debug for ComplianceTechnique {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ComplianceTechnique([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_1313aa3b60f5fb7a396c0ee8`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ComplianceWithTechniqueModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "link")]
    pub link: String,
    #[serde(rename = "techniques")]
    pub techniques: Vec<TechniqueModel>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ComplianceWithTechniqueModel {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "id",
            "display_name",
            "description",
            "active",
            "version",
            "link",
            "techniques",
        ];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("active", &self.active)?;
        map.serialize_entry("version", &self.version)?;
        map.serialize_entry("link", &self.link)?;
        map.serialize_entry("techniques", &self.techniques)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ComplianceWithTechniqueModel {
    pub fn builder() -> ComplianceWithTechniqueModelBuilder {
        ComplianceWithTechniqueModelBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_1313aa3b60f5fb7a396c0ee8";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ComplianceWithTechniqueModelBuilder {
    id: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    active: Option<bool>,
    version: Option<String>,
    link: Option<String>,
    techniques: Option<Vec<TechniqueModel>>,
    extensions: Extensions<JsonValue>,
}
impl ComplianceWithTechniqueModelBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }
    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
    pub fn link(mut self, value: String) -> Self {
        self.link = Some(value);
        self
    }
    pub fn techniques(mut self, value: Vec<TechniqueModel>) -> Self {
        self.techniques = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ComplianceWithTechniqueModel> {
        Ok(ComplianceWithTechniqueModel {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            active: self
                .active
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            version: self
                .version
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            link: self
                .link
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            techniques: self
                .techniques
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ComplianceWithTechniqueModel> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ComplianceWithTechniqueModel::parse(value, mode)
    }
}
impl fmt::Debug for ComplianceWithTechniqueModelBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ComplianceWithTechniqueModelBuilder([REDACTED])")
    }
}
impl fmt::Debug for ComplianceWithTechniqueModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ComplianceWithTechniqueModel([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_e9600f8178586e47b5a6a4cd`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct CountByName {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "count")]
    pub count: FiniteNumber,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for CountByName {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["name", "count"];
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
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("count", &self.count)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CountByName {
    pub fn builder() -> CountByNameBuilder {
        CountByNameBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_e9600f8178586e47b5a6a4cd";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct CountByNameBuilder {
    name: Option<String>,
    count: Option<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl CountByNameBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn count(mut self, value: FiniteNumber) -> Self {
        self.count = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<CountByName> {
        Ok(CountByName {
            name: self
                .name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            count: self
                .count
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<CountByName> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        CountByName::parse(value, mode)
    }
}
impl fmt::Debug for CountByNameBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CountByNameBuilder([REDACTED])")
    }
}
impl fmt::Debug for CountByName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CountByName([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_107dc8732d79d3153cff6c16`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum CountedQuotaEnum {
    HELD,
    COUNTED,
    NOTCOUNTED,
    Unknown(String),
}
impl CountedQuotaEnum {
    pub fn as_str(&self) -> &str {
        match self {
            Self::HELD => "HELD",
            Self::COUNTED => "COUNTED",
            Self::NOTCOUNTED => "NOT_COUNTED",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for CountedQuotaEnum {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for CountedQuotaEnum {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "HELD" => Self::HELD,
            "COUNTED" => Self::COUNTED,
            "NOT_COUNTED" => Self::NOTCOUNTED,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for CountedQuotaEnum {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CountedQuotaEnum([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_1b6745db0ce247044800df54`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct CustomAttackOutputResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "custom_attack_id")]
    pub custom_attack_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "target_id")]
    pub target_id: String,
    #[serde(rename = "output")]
    pub output: String,
    #[serde(rename = "threat", default)]
    pub threat: Patch<bool>,
    #[serde(rename = "marked_safe", default)]
    pub marked_safe: Patch<bool>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(rename = "error_message", default)]
    pub error_message: Patch<String>,
    #[serde(rename = "judge_reasoning", default)]
    pub judge_reasoning: Optional<String>,
    #[serde(rename = "threat_details", default)]
    pub threat_details: Optional<ThreatDetails>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for CustomAttackOutputResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "custom_attack_id",
            "job_id",
            "target_id",
            "output",
            "threat",
            "marked_safe",
            "error",
            "error_message",
            "judge_reasoning",
            "threat_details",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("custom_attack_id", &self.custom_attack_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("target_id", &self.target_id)?;
        map.serialize_entry("output", &self.output)?;
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.marked_safe.is_unset() {
            map.serialize_entry("marked_safe", &self.marked_safe)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        if !self.error_message.is_unset() {
            map.serialize_entry("error_message", &self.error_message)?;
        }
        if !self.judge_reasoning.is_unset() {
            map.serialize_entry("judge_reasoning", &self.judge_reasoning)?;
        }
        if !self.threat_details.is_unset() {
            map.serialize_entry("threat_details", &self.threat_details)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CustomAttackOutputResponse {
    pub fn builder() -> CustomAttackOutputResponseBuilder {
        CustomAttackOutputResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_1b6745db0ce247044800df54";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct CustomAttackOutputResponseBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    custom_attack_id: Option<String>,
    job_id: Option<String>,
    target_id: Option<String>,
    output: Option<String>,
    threat: Patch<bool>,
    marked_safe: Patch<bool>,
    error: Patch<bool>,
    error_message: Patch<String>,
    judge_reasoning: Optional<String>,
    threat_details: Optional<ThreatDetails>,
    extensions: Extensions<JsonValue>,
}
impl CustomAttackOutputResponseBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn custom_attack_id(mut self, value: String) -> Self {
        self.custom_attack_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn target_id(mut self, value: String) -> Self {
        self.target_id = Some(value);
        self
    }
    pub fn output(mut self, value: String) -> Self {
        self.output = Some(value);
        self
    }
    pub fn threat(mut self, value: Patch<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn marked_safe(mut self, value: Patch<bool>) -> Self {
        self.marked_safe = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn error_message(mut self, value: Patch<String>) -> Self {
        self.error_message = value;
        self
    }
    pub fn judge_reasoning(mut self, value: Optional<String>) -> Self {
        self.judge_reasoning = value;
        self
    }
    pub fn threat_details(mut self, value: Optional<ThreatDetails>) -> Self {
        self.threat_details = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<CustomAttackOutputResponse> {
        Ok(CustomAttackOutputResponse {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            custom_attack_id: self
                .custom_attack_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target_id: self
                .target_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            output: self
                .output
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            threat: self.threat,
            marked_safe: self.marked_safe,
            error: self.error,
            error_message: self.error_message,
            judge_reasoning: self.judge_reasoning,
            threat_details: self.threat_details,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<CustomAttackOutputResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        CustomAttackOutputResponse::parse(value, mode)
    }
}
impl fmt::Debug for CustomAttackOutputResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomAttackOutputResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for CustomAttackOutputResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomAttackOutputResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b03a0d6faf7caf85d7be7048`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct CustomAttackReportResponse {
    #[serde(rename = "total_prompts")]
    pub total_prompts: FiniteNumber,
    #[serde(rename = "total_attacks")]
    pub total_attacks: FiniteNumber,
    #[serde(rename = "total_threats")]
    pub total_threats: FiniteNumber,
    #[serde(rename = "failed_attacks")]
    pub failed_attacks: FiniteNumber,
    #[serde(rename = "score")]
    pub score: FiniteNumber,
    #[serde(rename = "asr")]
    pub asr: FiniteNumber,
    #[serde(rename = "custom_attack_reports", default)]
    pub custom_attack_reports: Optional<Vec<PromptSetSummary>>,
    #[serde(rename = "property_statistics", default)]
    pub property_statistics: Optional<Vec<PropertyStatistic>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for CustomAttackReportResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "total_prompts",
            "total_attacks",
            "total_threats",
            "failed_attacks",
            "score",
            "asr",
            "custom_attack_reports",
            "property_statistics",
        ];
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
        map.serialize_entry("total_prompts", &self.total_prompts)?;
        map.serialize_entry("total_attacks", &self.total_attacks)?;
        map.serialize_entry("total_threats", &self.total_threats)?;
        map.serialize_entry("failed_attacks", &self.failed_attacks)?;
        map.serialize_entry("score", &self.score)?;
        map.serialize_entry("asr", &self.asr)?;
        if !self.custom_attack_reports.is_unset() {
            map.serialize_entry("custom_attack_reports", &self.custom_attack_reports)?;
        }
        if !self.property_statistics.is_unset() {
            map.serialize_entry("property_statistics", &self.property_statistics)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CustomAttackReportResponse {
    pub fn builder() -> CustomAttackReportResponseBuilder {
        CustomAttackReportResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_b03a0d6faf7caf85d7be7048";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct CustomAttackReportResponseBuilder {
    total_prompts: Option<FiniteNumber>,
    total_attacks: Option<FiniteNumber>,
    total_threats: Option<FiniteNumber>,
    failed_attacks: Option<FiniteNumber>,
    score: Option<FiniteNumber>,
    asr: Option<FiniteNumber>,
    custom_attack_reports: Optional<Vec<PromptSetSummary>>,
    property_statistics: Optional<Vec<PropertyStatistic>>,
    extensions: Extensions<JsonValue>,
}
impl CustomAttackReportResponseBuilder {
    pub fn total_prompts(mut self, value: FiniteNumber) -> Self {
        self.total_prompts = Some(value);
        self
    }
    pub fn total_attacks(mut self, value: FiniteNumber) -> Self {
        self.total_attacks = Some(value);
        self
    }
    pub fn total_threats(mut self, value: FiniteNumber) -> Self {
        self.total_threats = Some(value);
        self
    }
    pub fn failed_attacks(mut self, value: FiniteNumber) -> Self {
        self.failed_attacks = Some(value);
        self
    }
    pub fn score(mut self, value: FiniteNumber) -> Self {
        self.score = Some(value);
        self
    }
    pub fn asr(mut self, value: FiniteNumber) -> Self {
        self.asr = Some(value);
        self
    }
    pub fn custom_attack_reports(mut self, value: Optional<Vec<PromptSetSummary>>) -> Self {
        self.custom_attack_reports = value;
        self
    }
    pub fn property_statistics(mut self, value: Optional<Vec<PropertyStatistic>>) -> Self {
        self.property_statistics = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<CustomAttackReportResponse> {
        Ok(CustomAttackReportResponse {
            total_prompts: self
                .total_prompts
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_attacks: self
                .total_attacks
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_threats: self
                .total_threats
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            failed_attacks: self
                .failed_attacks
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            score: self
                .score
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            asr: self
                .asr
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            custom_attack_reports: self.custom_attack_reports,
            property_statistics: self.property_statistics,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<CustomAttackReportResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        CustomAttackReportResponse::parse(value, mode)
    }
}
impl fmt::Debug for CustomAttackReportResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomAttackReportResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for CustomAttackReportResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomAttackReportResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_e89a4147f7f219d3da01040f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct CustomAttacksListResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "data")]
    pub data: Vec<PromptDetailResponse>,
    #[serde(rename = "total_attacks")]
    pub total_attacks: FiniteNumber,
    #[serde(rename = "total_threats")]
    pub total_threats: FiniteNumber,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for CustomAttacksListResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["pagination", "data", "total_attacks", "total_threats"];
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
        map.serialize_entry("pagination", &self.pagination)?;
        map.serialize_entry("data", &self.data)?;
        map.serialize_entry("total_attacks", &self.total_attacks)?;
        map.serialize_entry("total_threats", &self.total_threats)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CustomAttacksListResponse {
    pub fn builder() -> CustomAttacksListResponseBuilder {
        CustomAttacksListResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_e89a4147f7f219d3da01040f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct CustomAttacksListResponseBuilder {
    pagination: Option<Pagination>,
    data: Option<Vec<PromptDetailResponse>>,
    total_attacks: Option<FiniteNumber>,
    total_threats: Option<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl CustomAttacksListResponseBuilder {
    pub fn pagination(mut self, value: Pagination) -> Self {
        self.pagination = Some(value);
        self
    }
    pub fn data(mut self, value: Vec<PromptDetailResponse>) -> Self {
        self.data = Some(value);
        self
    }
    pub fn total_attacks(mut self, value: FiniteNumber) -> Self {
        self.total_attacks = Some(value);
        self
    }
    pub fn total_threats(mut self, value: FiniteNumber) -> Self {
        self.total_threats = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<CustomAttacksListResponse> {
        Ok(CustomAttacksListResponse {
            pagination: self
                .pagination
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            data: self
                .data
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_attacks: self
                .total_attacks
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_threats: self
                .total_threats
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<CustomAttacksListResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        CustomAttacksListResponse::parse(value, mode)
    }
}
impl fmt::Debug for CustomAttacksListResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomAttacksListResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for CustomAttacksListResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomAttacksListResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_70996d94c36373d887cccf1f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct CustomJobMetadata {
    #[serde(rename = "rate_limit_enabled", default)]
    pub rate_limit_enabled: Optional<bool>,
    #[serde(rename = "rate_limit", default)]
    pub rate_limit: Patch<FiniteNumber>,
    #[serde(rename = "rate_limit_error_code", default)]
    pub rate_limit_error_code: Patch<FiniteNumber>,
    #[serde(rename = "rate_limit_error_message", default)]
    pub rate_limit_error_message: Patch<String>,
    #[serde(rename = "rate_limit_error_json", default)]
    pub rate_limit_error_json: Patch<CustomJobMetadataRateLimitErrorJson>,
    #[serde(rename = "content_filter_enabled", default)]
    pub content_filter_enabled: Optional<bool>,
    #[serde(rename = "content_filter_error_code", default)]
    pub content_filter_error_code: Patch<FiniteNumber>,
    #[serde(rename = "content_filter_error_message", default)]
    pub content_filter_error_message: Patch<String>,
    #[serde(rename = "content_filter_error_json", default)]
    pub content_filter_error_json: Patch<CustomJobMetadataContentFilterErrorJson>,
    #[serde(rename = "language", default)]
    pub language: Patch<LanguageOption>,
    #[serde(rename = "runtime_profile_id", default)]
    pub runtime_profile_id: Patch<String>,
    #[serde(rename = "custom_prompt_sets")]
    pub custom_prompt_sets: Vec<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for CustomJobMetadata {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "rate_limit_enabled",
            "rate_limit",
            "rate_limit_error_code",
            "rate_limit_error_message",
            "rate_limit_error_json",
            "content_filter_enabled",
            "content_filter_error_code",
            "content_filter_error_message",
            "content_filter_error_json",
            "language",
            "runtime_profile_id",
            "custom_prompt_sets",
        ];
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
        if !self.rate_limit_enabled.is_unset() {
            map.serialize_entry("rate_limit_enabled", &self.rate_limit_enabled)?;
        }
        if !self.rate_limit.is_unset() {
            map.serialize_entry("rate_limit", &self.rate_limit)?;
        }
        if !self.rate_limit_error_code.is_unset() {
            map.serialize_entry("rate_limit_error_code", &self.rate_limit_error_code)?;
        }
        if !self.rate_limit_error_message.is_unset() {
            map.serialize_entry("rate_limit_error_message", &self.rate_limit_error_message)?;
        }
        if !self.rate_limit_error_json.is_unset() {
            map.serialize_entry("rate_limit_error_json", &self.rate_limit_error_json)?;
        }
        if !self.content_filter_enabled.is_unset() {
            map.serialize_entry("content_filter_enabled", &self.content_filter_enabled)?;
        }
        if !self.content_filter_error_code.is_unset() {
            map.serialize_entry("content_filter_error_code", &self.content_filter_error_code)?;
        }
        if !self.content_filter_error_message.is_unset() {
            map.serialize_entry(
                "content_filter_error_message",
                &self.content_filter_error_message,
            )?;
        }
        if !self.content_filter_error_json.is_unset() {
            map.serialize_entry("content_filter_error_json", &self.content_filter_error_json)?;
        }
        if !self.language.is_unset() {
            map.serialize_entry("language", &self.language)?;
        }
        if !self.runtime_profile_id.is_unset() {
            map.serialize_entry("runtime_profile_id", &self.runtime_profile_id)?;
        }
        map.serialize_entry("custom_prompt_sets", &self.custom_prompt_sets)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CustomJobMetadata {
    pub fn builder() -> CustomJobMetadataBuilder {
        CustomJobMetadataBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_70996d94c36373d887cccf1f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct CustomJobMetadataBuilder {
    rate_limit_enabled: Optional<bool>,
    rate_limit: Patch<FiniteNumber>,
    rate_limit_error_code: Patch<FiniteNumber>,
    rate_limit_error_message: Patch<String>,
    rate_limit_error_json: Patch<CustomJobMetadataRateLimitErrorJson>,
    content_filter_enabled: Optional<bool>,
    content_filter_error_code: Patch<FiniteNumber>,
    content_filter_error_message: Patch<String>,
    content_filter_error_json: Patch<CustomJobMetadataContentFilterErrorJson>,
    language: Patch<LanguageOption>,
    runtime_profile_id: Patch<String>,
    custom_prompt_sets: Option<Vec<String>>,
    extensions: Extensions<JsonValue>,
}
impl CustomJobMetadataBuilder {
    pub fn rate_limit_enabled(mut self, value: Optional<bool>) -> Self {
        self.rate_limit_enabled = value;
        self
    }
    pub fn rate_limit(mut self, value: Patch<FiniteNumber>) -> Self {
        self.rate_limit = value;
        self
    }
    pub fn rate_limit_error_code(mut self, value: Patch<FiniteNumber>) -> Self {
        self.rate_limit_error_code = value;
        self
    }
    pub fn rate_limit_error_message(mut self, value: Patch<String>) -> Self {
        self.rate_limit_error_message = value;
        self
    }
    pub fn rate_limit_error_json(
        mut self,
        value: Patch<CustomJobMetadataRateLimitErrorJson>,
    ) -> Self {
        self.rate_limit_error_json = value;
        self
    }
    pub fn content_filter_enabled(mut self, value: Optional<bool>) -> Self {
        self.content_filter_enabled = value;
        self
    }
    pub fn content_filter_error_code(mut self, value: Patch<FiniteNumber>) -> Self {
        self.content_filter_error_code = value;
        self
    }
    pub fn content_filter_error_message(mut self, value: Patch<String>) -> Self {
        self.content_filter_error_message = value;
        self
    }
    pub fn content_filter_error_json(
        mut self,
        value: Patch<CustomJobMetadataContentFilterErrorJson>,
    ) -> Self {
        self.content_filter_error_json = value;
        self
    }
    pub fn language(mut self, value: Patch<LanguageOption>) -> Self {
        self.language = value;
        self
    }
    pub fn runtime_profile_id(mut self, value: Patch<String>) -> Self {
        self.runtime_profile_id = value;
        self
    }
    pub fn custom_prompt_sets(mut self, value: Vec<String>) -> Self {
        self.custom_prompt_sets = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<CustomJobMetadata> {
        Ok(CustomJobMetadata {
            rate_limit_enabled: self.rate_limit_enabled,
            rate_limit: self.rate_limit,
            rate_limit_error_code: self.rate_limit_error_code,
            rate_limit_error_message: self.rate_limit_error_message,
            rate_limit_error_json: self.rate_limit_error_json,
            content_filter_enabled: self.content_filter_enabled,
            content_filter_error_code: self.content_filter_error_code,
            content_filter_error_message: self.content_filter_error_message,
            content_filter_error_json: self.content_filter_error_json,
            language: self.language,
            runtime_profile_id: self.runtime_profile_id,
            custom_prompt_sets: self
                .custom_prompt_sets
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<CustomJobMetadata> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        CustomJobMetadata::parse(value, mode)
    }
}
impl fmt::Debug for CustomJobMetadataBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomJobMetadataBuilder([REDACTED])")
    }
}
impl fmt::Debug for CustomJobMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomJobMetadata([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_8bc2dcce459338e5249f4911`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct CustomJobMetadataContentFilterErrorJson {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for CustomJobMetadataContentFilterErrorJson {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CustomJobMetadataContentFilterErrorJson {
    pub fn builder() -> CustomJobMetadataContentFilterErrorJsonBuilder {
        CustomJobMetadataContentFilterErrorJsonBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_8bc2dcce459338e5249f4911";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct CustomJobMetadataContentFilterErrorJsonBuilder {
    extensions: Extensions<JsonValue>,
}
impl CustomJobMetadataContentFilterErrorJsonBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<CustomJobMetadataContentFilterErrorJson> {
        Ok(CustomJobMetadataContentFilterErrorJson {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<CustomJobMetadataContentFilterErrorJson> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        CustomJobMetadataContentFilterErrorJson::parse(value, mode)
    }
}
impl fmt::Debug for CustomJobMetadataContentFilterErrorJsonBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomJobMetadataContentFilterErrorJsonBuilder([REDACTED])")
    }
}
impl fmt::Debug for CustomJobMetadataContentFilterErrorJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomJobMetadataContentFilterErrorJson([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b5766660f8daaaa59507e5fc`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct CustomJobMetadataRateLimitErrorJson {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for CustomJobMetadataRateLimitErrorJson {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CustomJobMetadataRateLimitErrorJson {
    pub fn builder() -> CustomJobMetadataRateLimitErrorJsonBuilder {
        CustomJobMetadataRateLimitErrorJsonBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_b5766660f8daaaa59507e5fc";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct CustomJobMetadataRateLimitErrorJsonBuilder {
    extensions: Extensions<JsonValue>,
}
impl CustomJobMetadataRateLimitErrorJsonBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<CustomJobMetadataRateLimitErrorJson> {
        Ok(CustomJobMetadataRateLimitErrorJson {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<CustomJobMetadataRateLimitErrorJson> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        CustomJobMetadataRateLimitErrorJson::parse(value, mode)
    }
}
impl fmt::Debug for CustomJobMetadataRateLimitErrorJsonBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomJobMetadataRateLimitErrorJsonBuilder([REDACTED])")
    }
}
impl fmt::Debug for CustomJobMetadataRateLimitErrorJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomJobMetadataRateLimitErrorJson([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_ec59d547e743cfc6a4361221`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct CustomTopicGuardrails {
    #[serde(rename = "action", default)]
    pub action: Optional<GuardrailAction>,
    #[serde(rename = "blocked_topics", default)]
    pub blocked_topics: Optional<Vec<String>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for CustomTopicGuardrails {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["action", "blocked_topics"];
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
        if !self.action.is_unset() {
            map.serialize_entry("action", &self.action)?;
        }
        if !self.blocked_topics.is_unset() {
            map.serialize_entry("blocked_topics", &self.blocked_topics)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl CustomTopicGuardrails {
    pub fn builder() -> CustomTopicGuardrailsBuilder {
        CustomTopicGuardrailsBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_ec59d547e743cfc6a4361221";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct CustomTopicGuardrailsBuilder {
    action: Optional<GuardrailAction>,
    blocked_topics: Optional<Vec<String>>,
    extensions: Extensions<JsonValue>,
}
impl CustomTopicGuardrailsBuilder {
    pub fn action(mut self, value: Optional<GuardrailAction>) -> Self {
        self.action = value;
        self
    }
    pub fn blocked_topics(mut self, value: Optional<Vec<String>>) -> Self {
        self.blocked_topics = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<CustomTopicGuardrails> {
        Ok(CustomTopicGuardrails {
            action: self.action,
            blocked_topics: self.blocked_topics,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<CustomTopicGuardrails> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        CustomTopicGuardrails::parse(value, mode)
    }
}
impl fmt::Debug for CustomTopicGuardrailsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomTopicGuardrailsBuilder([REDACTED])")
    }
}
impl fmt::Debug for CustomTopicGuardrails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("CustomTopicGuardrails([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_fde39cdb73bce46981b0baf4`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum DateRangeFilter {
    LAST7DAYS,
    LAST15DAYS,
    LAST30DAYS,
    ALL,
    Unknown(String),
}
impl DateRangeFilter {
    pub fn as_str(&self) -> &str {
        match self {
            Self::LAST7DAYS => "LAST_7_DAYS",
            Self::LAST15DAYS => "LAST_15_DAYS",
            Self::LAST30DAYS => "LAST_30_DAYS",
            Self::ALL => "ALL",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for DateRangeFilter {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for DateRangeFilter {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "LAST_7_DAYS" => Self::LAST7DAYS,
            "LAST_15_DAYS" => Self::LAST15DAYS,
            "LAST_30_DAYS" => Self::LAST30DAYS,
            "ALL" => Self::ALL,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for DateRangeFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DateRangeFilter([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_de272ffee7ce54655060aa54`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct DynamicJobMetadata {
    #[serde(rename = "rate_limit_enabled", default)]
    pub rate_limit_enabled: Optional<bool>,
    #[serde(rename = "rate_limit", default)]
    pub rate_limit: Patch<FiniteNumber>,
    #[serde(rename = "rate_limit_error_code", default)]
    pub rate_limit_error_code: Patch<FiniteNumber>,
    #[serde(rename = "rate_limit_error_message", default)]
    pub rate_limit_error_message: Patch<String>,
    #[serde(rename = "rate_limit_error_json", default)]
    pub rate_limit_error_json: Patch<DynamicJobMetadataRateLimitErrorJson>,
    #[serde(rename = "content_filter_enabled", default)]
    pub content_filter_enabled: Optional<bool>,
    #[serde(rename = "content_filter_error_code", default)]
    pub content_filter_error_code: Patch<FiniteNumber>,
    #[serde(rename = "content_filter_error_message", default)]
    pub content_filter_error_message: Patch<String>,
    #[serde(rename = "content_filter_error_json", default)]
    pub content_filter_error_json: Patch<DynamicJobMetadataContentFilterErrorJson>,
    #[serde(rename = "language", default)]
    pub language: Patch<LanguageOption>,
    #[serde(rename = "runtime_profile_id", default)]
    pub runtime_profile_id: Patch<String>,
    #[serde(rename = "stream_breadth", default)]
    pub stream_breadth: Optional<FiniteNumber>,
    #[serde(rename = "stream_depth", default)]
    pub stream_depth: Optional<FiniteNumber>,
    #[serde(rename = "max_tokens", default)]
    pub max_tokens: Optional<FiniteNumber>,
    #[serde(rename = "context_size", default)]
    pub context_size: Optional<FiniteNumber>,
    #[serde(rename = "attack_goals", default)]
    pub attack_goals: Optional<Vec<String>>,
    #[serde(rename = "base_model", default)]
    pub base_model: Patch<String>,
    #[serde(rename = "use_case", default)]
    pub use_case: Patch<String>,
    #[serde(rename = "system_prompt", default)]
    pub system_prompt: Patch<String>,
    #[serde(rename = "goal_categories", default)]
    pub goal_categories: Patch<Vec<GoalCategory>>,
    #[serde(rename = "multi_turn_enabled", default)]
    pub multi_turn_enabled: Patch<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for DynamicJobMetadata {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "rate_limit_enabled",
            "rate_limit",
            "rate_limit_error_code",
            "rate_limit_error_message",
            "rate_limit_error_json",
            "content_filter_enabled",
            "content_filter_error_code",
            "content_filter_error_message",
            "content_filter_error_json",
            "language",
            "runtime_profile_id",
            "stream_breadth",
            "stream_depth",
            "max_tokens",
            "context_size",
            "attack_goals",
            "base_model",
            "use_case",
            "system_prompt",
            "goal_categories",
            "multi_turn_enabled",
        ];
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
        if !self.rate_limit_enabled.is_unset() {
            map.serialize_entry("rate_limit_enabled", &self.rate_limit_enabled)?;
        }
        if !self.rate_limit.is_unset() {
            map.serialize_entry("rate_limit", &self.rate_limit)?;
        }
        if !self.rate_limit_error_code.is_unset() {
            map.serialize_entry("rate_limit_error_code", &self.rate_limit_error_code)?;
        }
        if !self.rate_limit_error_message.is_unset() {
            map.serialize_entry("rate_limit_error_message", &self.rate_limit_error_message)?;
        }
        if !self.rate_limit_error_json.is_unset() {
            map.serialize_entry("rate_limit_error_json", &self.rate_limit_error_json)?;
        }
        if !self.content_filter_enabled.is_unset() {
            map.serialize_entry("content_filter_enabled", &self.content_filter_enabled)?;
        }
        if !self.content_filter_error_code.is_unset() {
            map.serialize_entry("content_filter_error_code", &self.content_filter_error_code)?;
        }
        if !self.content_filter_error_message.is_unset() {
            map.serialize_entry(
                "content_filter_error_message",
                &self.content_filter_error_message,
            )?;
        }
        if !self.content_filter_error_json.is_unset() {
            map.serialize_entry("content_filter_error_json", &self.content_filter_error_json)?;
        }
        if !self.language.is_unset() {
            map.serialize_entry("language", &self.language)?;
        }
        if !self.runtime_profile_id.is_unset() {
            map.serialize_entry("runtime_profile_id", &self.runtime_profile_id)?;
        }
        if !self.stream_breadth.is_unset() {
            map.serialize_entry("stream_breadth", &self.stream_breadth)?;
        }
        if !self.stream_depth.is_unset() {
            map.serialize_entry("stream_depth", &self.stream_depth)?;
        }
        if !self.max_tokens.is_unset() {
            map.serialize_entry("max_tokens", &self.max_tokens)?;
        }
        if !self.context_size.is_unset() {
            map.serialize_entry("context_size", &self.context_size)?;
        }
        if !self.attack_goals.is_unset() {
            map.serialize_entry("attack_goals", &self.attack_goals)?;
        }
        if !self.base_model.is_unset() {
            map.serialize_entry("base_model", &self.base_model)?;
        }
        if !self.use_case.is_unset() {
            map.serialize_entry("use_case", &self.use_case)?;
        }
        if !self.system_prompt.is_unset() {
            map.serialize_entry("system_prompt", &self.system_prompt)?;
        }
        if !self.goal_categories.is_unset() {
            map.serialize_entry("goal_categories", &self.goal_categories)?;
        }
        if !self.multi_turn_enabled.is_unset() {
            map.serialize_entry("multi_turn_enabled", &self.multi_turn_enabled)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DynamicJobMetadata {
    pub fn builder() -> DynamicJobMetadataBuilder {
        DynamicJobMetadataBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_de272ffee7ce54655060aa54";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct DynamicJobMetadataBuilder {
    rate_limit_enabled: Optional<bool>,
    rate_limit: Patch<FiniteNumber>,
    rate_limit_error_code: Patch<FiniteNumber>,
    rate_limit_error_message: Patch<String>,
    rate_limit_error_json: Patch<DynamicJobMetadataRateLimitErrorJson>,
    content_filter_enabled: Optional<bool>,
    content_filter_error_code: Patch<FiniteNumber>,
    content_filter_error_message: Patch<String>,
    content_filter_error_json: Patch<DynamicJobMetadataContentFilterErrorJson>,
    language: Patch<LanguageOption>,
    runtime_profile_id: Patch<String>,
    stream_breadth: Optional<FiniteNumber>,
    stream_depth: Optional<FiniteNumber>,
    max_tokens: Optional<FiniteNumber>,
    context_size: Optional<FiniteNumber>,
    attack_goals: Optional<Vec<String>>,
    base_model: Patch<String>,
    use_case: Patch<String>,
    system_prompt: Patch<String>,
    goal_categories: Patch<Vec<GoalCategory>>,
    multi_turn_enabled: Patch<bool>,
    extensions: Extensions<JsonValue>,
}
impl DynamicJobMetadataBuilder {
    pub fn rate_limit_enabled(mut self, value: Optional<bool>) -> Self {
        self.rate_limit_enabled = value;
        self
    }
    pub fn rate_limit(mut self, value: Patch<FiniteNumber>) -> Self {
        self.rate_limit = value;
        self
    }
    pub fn rate_limit_error_code(mut self, value: Patch<FiniteNumber>) -> Self {
        self.rate_limit_error_code = value;
        self
    }
    pub fn rate_limit_error_message(mut self, value: Patch<String>) -> Self {
        self.rate_limit_error_message = value;
        self
    }
    pub fn rate_limit_error_json(
        mut self,
        value: Patch<DynamicJobMetadataRateLimitErrorJson>,
    ) -> Self {
        self.rate_limit_error_json = value;
        self
    }
    pub fn content_filter_enabled(mut self, value: Optional<bool>) -> Self {
        self.content_filter_enabled = value;
        self
    }
    pub fn content_filter_error_code(mut self, value: Patch<FiniteNumber>) -> Self {
        self.content_filter_error_code = value;
        self
    }
    pub fn content_filter_error_message(mut self, value: Patch<String>) -> Self {
        self.content_filter_error_message = value;
        self
    }
    pub fn content_filter_error_json(
        mut self,
        value: Patch<DynamicJobMetadataContentFilterErrorJson>,
    ) -> Self {
        self.content_filter_error_json = value;
        self
    }
    pub fn language(mut self, value: Patch<LanguageOption>) -> Self {
        self.language = value;
        self
    }
    pub fn runtime_profile_id(mut self, value: Patch<String>) -> Self {
        self.runtime_profile_id = value;
        self
    }
    pub fn stream_breadth(mut self, value: Optional<FiniteNumber>) -> Self {
        self.stream_breadth = value;
        self
    }
    pub fn stream_depth(mut self, value: Optional<FiniteNumber>) -> Self {
        self.stream_depth = value;
        self
    }
    pub fn max_tokens(mut self, value: Optional<FiniteNumber>) -> Self {
        self.max_tokens = value;
        self
    }
    pub fn context_size(mut self, value: Optional<FiniteNumber>) -> Self {
        self.context_size = value;
        self
    }
    pub fn attack_goals(mut self, value: Optional<Vec<String>>) -> Self {
        self.attack_goals = value;
        self
    }
    pub fn base_model(mut self, value: Patch<String>) -> Self {
        self.base_model = value;
        self
    }
    pub fn use_case(mut self, value: Patch<String>) -> Self {
        self.use_case = value;
        self
    }
    pub fn system_prompt(mut self, value: Patch<String>) -> Self {
        self.system_prompt = value;
        self
    }
    pub fn goal_categories(mut self, value: Patch<Vec<GoalCategory>>) -> Self {
        self.goal_categories = value;
        self
    }
    pub fn multi_turn_enabled(mut self, value: Patch<bool>) -> Self {
        self.multi_turn_enabled = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<DynamicJobMetadata> {
        Ok(DynamicJobMetadata {
            rate_limit_enabled: self.rate_limit_enabled,
            rate_limit: self.rate_limit,
            rate_limit_error_code: self.rate_limit_error_code,
            rate_limit_error_message: self.rate_limit_error_message,
            rate_limit_error_json: self.rate_limit_error_json,
            content_filter_enabled: self.content_filter_enabled,
            content_filter_error_code: self.content_filter_error_code,
            content_filter_error_message: self.content_filter_error_message,
            content_filter_error_json: self.content_filter_error_json,
            language: self.language,
            runtime_profile_id: self.runtime_profile_id,
            stream_breadth: self.stream_breadth,
            stream_depth: self.stream_depth,
            max_tokens: self.max_tokens,
            context_size: self.context_size,
            attack_goals: self.attack_goals,
            base_model: self.base_model,
            use_case: self.use_case,
            system_prompt: self.system_prompt,
            goal_categories: self.goal_categories,
            multi_turn_enabled: self.multi_turn_enabled,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<DynamicJobMetadata> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        DynamicJobMetadata::parse(value, mode)
    }
}
impl fmt::Debug for DynamicJobMetadataBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicJobMetadataBuilder([REDACTED])")
    }
}
impl fmt::Debug for DynamicJobMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicJobMetadata([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_7abc0acaf70ac53f1dfa4882`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct DynamicJobMetadataContentFilterErrorJson {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for DynamicJobMetadataContentFilterErrorJson {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DynamicJobMetadataContentFilterErrorJson {
    pub fn builder() -> DynamicJobMetadataContentFilterErrorJsonBuilder {
        DynamicJobMetadataContentFilterErrorJsonBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_7abc0acaf70ac53f1dfa4882";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct DynamicJobMetadataContentFilterErrorJsonBuilder {
    extensions: Extensions<JsonValue>,
}
impl DynamicJobMetadataContentFilterErrorJsonBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<DynamicJobMetadataContentFilterErrorJson> {
        Ok(DynamicJobMetadataContentFilterErrorJson {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<DynamicJobMetadataContentFilterErrorJson> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        DynamicJobMetadataContentFilterErrorJson::parse(value, mode)
    }
}
impl fmt::Debug for DynamicJobMetadataContentFilterErrorJsonBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicJobMetadataContentFilterErrorJsonBuilder([REDACTED])")
    }
}
impl fmt::Debug for DynamicJobMetadataContentFilterErrorJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicJobMetadataContentFilterErrorJson([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_1dd0d5f864975f28537a6a28`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct DynamicJobMetadataRateLimitErrorJson {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for DynamicJobMetadataRateLimitErrorJson {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DynamicJobMetadataRateLimitErrorJson {
    pub fn builder() -> DynamicJobMetadataRateLimitErrorJsonBuilder {
        DynamicJobMetadataRateLimitErrorJsonBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_1dd0d5f864975f28537a6a28";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct DynamicJobMetadataRateLimitErrorJsonBuilder {
    extensions: Extensions<JsonValue>,
}
impl DynamicJobMetadataRateLimitErrorJsonBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<DynamicJobMetadataRateLimitErrorJson> {
        Ok(DynamicJobMetadataRateLimitErrorJson {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<DynamicJobMetadataRateLimitErrorJson> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        DynamicJobMetadataRateLimitErrorJson::parse(value, mode)
    }
}
impl fmt::Debug for DynamicJobMetadataRateLimitErrorJsonBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicJobMetadataRateLimitErrorJsonBuilder([REDACTED])")
    }
}
impl fmt::Debug for DynamicJobMetadataRateLimitErrorJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicJobMetadataRateLimitErrorJson([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_4c50f0262af035ce021e89d0`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct DynamicJobReport {
    #[serde(rename = "total_goals", default)]
    pub total_goals: Optional<FiniteNumber>,
    #[serde(rename = "total_streams", default)]
    pub total_streams: Optional<FiniteNumber>,
    #[serde(rename = "total_threats", default)]
    pub total_threats: Optional<FiniteNumber>,
    #[serde(rename = "goals_achieved", default)]
    pub goals_achieved: Optional<FiniteNumber>,
    #[serde(rename = "report_summary", default)]
    pub report_summary: Patch<String>,
    #[serde(rename = "score", default)]
    pub score: Optional<FiniteNumber>,
    #[serde(rename = "asr", default)]
    pub asr: Optional<FiniteNumber>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for DynamicJobReport {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "total_goals",
            "total_streams",
            "total_threats",
            "goals_achieved",
            "report_summary",
            "score",
            "asr",
        ];
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
        if !self.total_goals.is_unset() {
            map.serialize_entry("total_goals", &self.total_goals)?;
        }
        if !self.total_streams.is_unset() {
            map.serialize_entry("total_streams", &self.total_streams)?;
        }
        if !self.total_threats.is_unset() {
            map.serialize_entry("total_threats", &self.total_threats)?;
        }
        if !self.goals_achieved.is_unset() {
            map.serialize_entry("goals_achieved", &self.goals_achieved)?;
        }
        if !self.report_summary.is_unset() {
            map.serialize_entry("report_summary", &self.report_summary)?;
        }
        if !self.score.is_unset() {
            map.serialize_entry("score", &self.score)?;
        }
        if !self.asr.is_unset() {
            map.serialize_entry("asr", &self.asr)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DynamicJobReport {
    pub fn builder() -> DynamicJobReportBuilder {
        DynamicJobReportBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_4c50f0262af035ce021e89d0";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct DynamicJobReportBuilder {
    total_goals: Optional<FiniteNumber>,
    total_streams: Optional<FiniteNumber>,
    total_threats: Optional<FiniteNumber>,
    goals_achieved: Optional<FiniteNumber>,
    report_summary: Patch<String>,
    score: Optional<FiniteNumber>,
    asr: Optional<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl DynamicJobReportBuilder {
    pub fn total_goals(mut self, value: Optional<FiniteNumber>) -> Self {
        self.total_goals = value;
        self
    }
    pub fn total_streams(mut self, value: Optional<FiniteNumber>) -> Self {
        self.total_streams = value;
        self
    }
    pub fn total_threats(mut self, value: Optional<FiniteNumber>) -> Self {
        self.total_threats = value;
        self
    }
    pub fn goals_achieved(mut self, value: Optional<FiniteNumber>) -> Self {
        self.goals_achieved = value;
        self
    }
    pub fn report_summary(mut self, value: Patch<String>) -> Self {
        self.report_summary = value;
        self
    }
    pub fn score(mut self, value: Optional<FiniteNumber>) -> Self {
        self.score = value;
        self
    }
    pub fn asr(mut self, value: Optional<FiniteNumber>) -> Self {
        self.asr = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<DynamicJobReport> {
        Ok(DynamicJobReport {
            total_goals: self.total_goals,
            total_streams: self.total_streams,
            total_threats: self.total_threats,
            goals_achieved: self.goals_achieved,
            report_summary: self.report_summary,
            score: self.score,
            asr: self.asr,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<DynamicJobReport> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        DynamicJobReport::parse(value, mode)
    }
}
impl fmt::Debug for DynamicJobReportBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicJobReportBuilder([REDACTED])")
    }
}
impl fmt::Debug for DynamicJobReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicJobReport([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_7a5452f832cb51c4c070a8e7`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct DynamicJobReportStats {
    #[serde(rename = "total_goals", default)]
    pub total_goals: Optional<FiniteNumber>,
    #[serde(rename = "total_streams", default)]
    pub total_streams: Optional<FiniteNumber>,
    #[serde(rename = "total_threats", default)]
    pub total_threats: Optional<FiniteNumber>,
    #[serde(rename = "goals_achieved", default)]
    pub goals_achieved: Optional<FiniteNumber>,
    #[serde(rename = "report_summary", default)]
    pub report_summary: Patch<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for DynamicJobReportStats {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "total_goals",
            "total_streams",
            "total_threats",
            "goals_achieved",
            "report_summary",
        ];
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
        if !self.total_goals.is_unset() {
            map.serialize_entry("total_goals", &self.total_goals)?;
        }
        if !self.total_streams.is_unset() {
            map.serialize_entry("total_streams", &self.total_streams)?;
        }
        if !self.total_threats.is_unset() {
            map.serialize_entry("total_threats", &self.total_threats)?;
        }
        if !self.goals_achieved.is_unset() {
            map.serialize_entry("goals_achieved", &self.goals_achieved)?;
        }
        if !self.report_summary.is_unset() {
            map.serialize_entry("report_summary", &self.report_summary)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DynamicJobReportStats {
    pub fn builder() -> DynamicJobReportStatsBuilder {
        DynamicJobReportStatsBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_7a5452f832cb51c4c070a8e7";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct DynamicJobReportStatsBuilder {
    total_goals: Optional<FiniteNumber>,
    total_streams: Optional<FiniteNumber>,
    total_threats: Optional<FiniteNumber>,
    goals_achieved: Optional<FiniteNumber>,
    report_summary: Patch<String>,
    extensions: Extensions<JsonValue>,
}
impl DynamicJobReportStatsBuilder {
    pub fn total_goals(mut self, value: Optional<FiniteNumber>) -> Self {
        self.total_goals = value;
        self
    }
    pub fn total_streams(mut self, value: Optional<FiniteNumber>) -> Self {
        self.total_streams = value;
        self
    }
    pub fn total_threats(mut self, value: Optional<FiniteNumber>) -> Self {
        self.total_threats = value;
        self
    }
    pub fn goals_achieved(mut self, value: Optional<FiniteNumber>) -> Self {
        self.goals_achieved = value;
        self
    }
    pub fn report_summary(mut self, value: Patch<String>) -> Self {
        self.report_summary = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<DynamicJobReportStats> {
        Ok(DynamicJobReportStats {
            total_goals: self.total_goals,
            total_streams: self.total_streams,
            total_threats: self.total_threats,
            goals_achieved: self.goals_achieved,
            report_summary: self.report_summary,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<DynamicJobReportStats> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        DynamicJobReportStats::parse(value, mode)
    }
}
impl fmt::Debug for DynamicJobReportStatsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicJobReportStatsBuilder([REDACTED])")
    }
}
impl fmt::Debug for DynamicJobReportStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicJobReportStats([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_da6a92a450e0fd7aaad18b41`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct DynamicRemediationDetail {
    #[serde(rename = "remediation")]
    pub remediation: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "resource_links", default)]
    pub resource_links: Optional<Vec<String>>,
    #[serde(rename = "priority_level", default)]
    pub priority_level: Optional<String>,
    #[serde(rename = "ease_of_implementation_level", default)]
    pub ease_of_implementation_level: Optional<String>,
    #[serde(rename = "effectiveness_level", default)]
    pub effectiveness_level: Optional<String>,
    #[serde(rename = "categories", default)]
    pub categories: Optional<Vec<GoalCategoryDetail>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for DynamicRemediationDetail {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "remediation",
            "description",
            "resource_links",
            "priority_level",
            "ease_of_implementation_level",
            "effectiveness_level",
            "categories",
        ];
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
        map.serialize_entry("remediation", &self.remediation)?;
        map.serialize_entry("description", &self.description)?;
        if !self.resource_links.is_unset() {
            map.serialize_entry("resource_links", &self.resource_links)?;
        }
        if !self.priority_level.is_unset() {
            map.serialize_entry("priority_level", &self.priority_level)?;
        }
        if !self.ease_of_implementation_level.is_unset() {
            map.serialize_entry(
                "ease_of_implementation_level",
                &self.ease_of_implementation_level,
            )?;
        }
        if !self.effectiveness_level.is_unset() {
            map.serialize_entry("effectiveness_level", &self.effectiveness_level)?;
        }
        if !self.categories.is_unset() {
            map.serialize_entry("categories", &self.categories)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DynamicRemediationDetail {
    pub fn builder() -> DynamicRemediationDetailBuilder {
        DynamicRemediationDetailBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_da6a92a450e0fd7aaad18b41";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct DynamicRemediationDetailBuilder {
    remediation: Option<String>,
    description: Option<String>,
    resource_links: Optional<Vec<String>>,
    priority_level: Optional<String>,
    ease_of_implementation_level: Optional<String>,
    effectiveness_level: Optional<String>,
    categories: Optional<Vec<GoalCategoryDetail>>,
    extensions: Extensions<JsonValue>,
}
impl DynamicRemediationDetailBuilder {
    pub fn remediation(mut self, value: String) -> Self {
        self.remediation = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn resource_links(mut self, value: Optional<Vec<String>>) -> Self {
        self.resource_links = value;
        self
    }
    pub fn priority_level(mut self, value: Optional<String>) -> Self {
        self.priority_level = value;
        self
    }
    pub fn ease_of_implementation_level(mut self, value: Optional<String>) -> Self {
        self.ease_of_implementation_level = value;
        self
    }
    pub fn effectiveness_level(mut self, value: Optional<String>) -> Self {
        self.effectiveness_level = value;
        self
    }
    pub fn categories(mut self, value: Optional<Vec<GoalCategoryDetail>>) -> Self {
        self.categories = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<DynamicRemediationDetail> {
        Ok(DynamicRemediationDetail {
            remediation: self
                .remediation
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            resource_links: self.resource_links,
            priority_level: self.priority_level,
            ease_of_implementation_level: self.ease_of_implementation_level,
            effectiveness_level: self.effectiveness_level,
            categories: self.categories,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<DynamicRemediationDetail> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        DynamicRemediationDetail::parse(value, mode)
    }
}
impl fmt::Debug for DynamicRemediationDetailBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicRemediationDetailBuilder([REDACTED])")
    }
}
impl fmt::Debug for DynamicRemediationDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicRemediationDetail([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_53e1842b2e55d78878c69f9d`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct DynamicRemediationResponse {
    #[serde(rename = "remediations", default)]
    pub remediations: Optional<Vec<DynamicRemediationDetail>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for DynamicRemediationResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["remediations"];
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
        if !self.remediations.is_unset() {
            map.serialize_entry("remediations", &self.remediations)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl DynamicRemediationResponse {
    pub fn builder() -> DynamicRemediationResponseBuilder {
        DynamicRemediationResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_53e1842b2e55d78878c69f9d";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct DynamicRemediationResponseBuilder {
    remediations: Optional<Vec<DynamicRemediationDetail>>,
    extensions: Extensions<JsonValue>,
}
impl DynamicRemediationResponseBuilder {
    pub fn remediations(mut self, value: Optional<Vec<DynamicRemediationDetail>>) -> Self {
        self.remediations = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<DynamicRemediationResponse> {
        Ok(DynamicRemediationResponse {
            remediations: self.remediations,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<DynamicRemediationResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        DynamicRemediationResponse::parse(value, mode)
    }
}
impl fmt::Debug for DynamicRemediationResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicRemediationResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for DynamicRemediationResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("DynamicRemediationResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_66a562119b1620ea1b360008`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ErrorLogList {
    #[serde(rename = "job_id", default)]
    pub job_id: Patch<String>,
    #[serde(rename = "target_id", default)]
    pub target_id: Patch<String>,
    #[serde(rename = "target_version", default)]
    pub target_version: Patch<FiniteNumber>,
    #[serde(rename = "attack_id", default)]
    pub attack_id: Patch<String>,
    #[serde(rename = "error_type", default)]
    pub error_type: Patch<ErrorLogListErrorType>,
    #[serde(rename = "error_source", default)]
    pub error_source: Patch<ErrorLogListErrorSource>,
    #[serde(rename = "error_message", default)]
    pub error_message: Patch<String>,
    #[serde(rename = "target_object", default)]
    pub target_object: Patch<ErrorLogListTargetObject>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Patch<ErrorLogListExtraInfo>,
    #[serde(rename = "version", default)]
    pub version: Optional<FiniteNumber>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ErrorLogList {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "job_id",
            "target_id",
            "target_version",
            "attack_id",
            "error_type",
            "error_source",
            "error_message",
            "target_object",
            "extra_info",
            "version",
            "created_at",
            "updated_at",
        ];
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
        if !self.job_id.is_unset() {
            map.serialize_entry("job_id", &self.job_id)?;
        }
        if !self.target_id.is_unset() {
            map.serialize_entry("target_id", &self.target_id)?;
        }
        if !self.target_version.is_unset() {
            map.serialize_entry("target_version", &self.target_version)?;
        }
        if !self.attack_id.is_unset() {
            map.serialize_entry("attack_id", &self.attack_id)?;
        }
        if !self.error_type.is_unset() {
            map.serialize_entry("error_type", &self.error_type)?;
        }
        if !self.error_source.is_unset() {
            map.serialize_entry("error_source", &self.error_source)?;
        }
        if !self.error_message.is_unset() {
            map.serialize_entry("error_message", &self.error_message)?;
        }
        if !self.target_object.is_unset() {
            map.serialize_entry("target_object", &self.target_object)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        map.serialize_entry("created_at", &self.created_at)?;
        map.serialize_entry("updated_at", &self.updated_at)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ErrorLogList {
    pub fn builder() -> ErrorLogListBuilder {
        ErrorLogListBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_66a562119b1620ea1b360008";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ErrorLogListBuilder {
    job_id: Patch<String>,
    target_id: Patch<String>,
    target_version: Patch<FiniteNumber>,
    attack_id: Patch<String>,
    error_type: Patch<ErrorLogListErrorType>,
    error_source: Patch<ErrorLogListErrorSource>,
    error_message: Patch<String>,
    target_object: Patch<ErrorLogListTargetObject>,
    extra_info: Patch<ErrorLogListExtraInfo>,
    version: Optional<FiniteNumber>,
    created_at: Option<String>,
    updated_at: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl ErrorLogListBuilder {
    pub fn job_id(mut self, value: Patch<String>) -> Self {
        self.job_id = value;
        self
    }
    pub fn target_id(mut self, value: Patch<String>) -> Self {
        self.target_id = value;
        self
    }
    pub fn target_version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.target_version = value;
        self
    }
    pub fn attack_id(mut self, value: Patch<String>) -> Self {
        self.attack_id = value;
        self
    }
    pub fn error_type(mut self, value: Patch<ErrorLogListErrorType>) -> Self {
        self.error_type = value;
        self
    }
    pub fn error_source(mut self, value: Patch<ErrorLogListErrorSource>) -> Self {
        self.error_source = value;
        self
    }
    pub fn error_message(mut self, value: Patch<String>) -> Self {
        self.error_message = value;
        self
    }
    pub fn target_object(mut self, value: Patch<ErrorLogListTargetObject>) -> Self {
        self.target_object = value;
        self
    }
    pub fn extra_info(mut self, value: Patch<ErrorLogListExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn version(mut self, value: Optional<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }
    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ErrorLogList> {
        Ok(ErrorLogList {
            job_id: self.job_id,
            target_id: self.target_id,
            target_version: self.target_version,
            attack_id: self.attack_id,
            error_type: self.error_type,
            error_source: self.error_source,
            error_message: self.error_message,
            target_object: self.target_object,
            extra_info: self.extra_info,
            version: self.version,
            created_at: self
                .created_at
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<ErrorLogList> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ErrorLogList::parse(value, mode)
    }
}
impl fmt::Debug for ErrorLogListBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorLogListBuilder([REDACTED])")
    }
}
impl fmt::Debug for ErrorLogList {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorLogList([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_5026d3042d4a8fa3525447d9`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorLogListErrorSource {
    TARGET,
    JOB,
    SYSTEM,
    VALIDATION,
    TARGETPROFILING,
    Unknown(String),
}
impl ErrorLogListErrorSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TARGET => "TARGET",
            Self::JOB => "JOB",
            Self::SYSTEM => "SYSTEM",
            Self::VALIDATION => "VALIDATION",
            Self::TARGETPROFILING => "TARGET_PROFILING",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ErrorLogListErrorSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ErrorLogListErrorSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "TARGET" => Self::TARGET,
            "JOB" => Self::JOB,
            "SYSTEM" => Self::SYSTEM,
            "VALIDATION" => Self::VALIDATION,
            "TARGET_PROFILING" => Self::TARGETPROFILING,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ErrorLogListErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorLogListErrorSource([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_a093798b167b1b04dc4f01bc`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorLogListErrorType {
    CONTENTFILTER,
    RATELIMIT,
    AUTHENTICATION,
    NETWORK,
    VALIDATION,
    NETWORKCHANNEL,
    TRANSLATION,
    CUSTOMTARGETADAPTER,
    UNKNOWN,
    Unknown(String),
}
impl ErrorLogListErrorType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CONTENTFILTER => "CONTENT_FILTER",
            Self::RATELIMIT => "RATE_LIMIT",
            Self::AUTHENTICATION => "AUTHENTICATION",
            Self::NETWORK => "NETWORK",
            Self::VALIDATION => "VALIDATION",
            Self::NETWORKCHANNEL => "NETWORK_CHANNEL",
            Self::TRANSLATION => "TRANSLATION",
            Self::CUSTOMTARGETADAPTER => "CUSTOM_TARGET_ADAPTER",
            Self::UNKNOWN => "UNKNOWN",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ErrorLogListErrorType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ErrorLogListErrorType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "CONTENT_FILTER" => Self::CONTENTFILTER,
            "RATE_LIMIT" => Self::RATELIMIT,
            "AUTHENTICATION" => Self::AUTHENTICATION,
            "NETWORK" => Self::NETWORK,
            "VALIDATION" => Self::VALIDATION,
            "NETWORK_CHANNEL" => Self::NETWORKCHANNEL,
            "TRANSLATION" => Self::TRANSLATION,
            "CUSTOM_TARGET_ADAPTER" => Self::CUSTOMTARGETADAPTER,
            "UNKNOWN" => Self::UNKNOWN,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ErrorLogListErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorLogListErrorType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_86cdfecd16d276ccfdb334ef`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ErrorLogListExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ErrorLogListExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ErrorLogListExtraInfo {
    pub fn builder() -> ErrorLogListExtraInfoBuilder {
        ErrorLogListExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_86cdfecd16d276ccfdb334ef";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ErrorLogListExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl ErrorLogListExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ErrorLogListExtraInfo> {
        Ok(ErrorLogListExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ErrorLogListExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ErrorLogListExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for ErrorLogListExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorLogListExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for ErrorLogListExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorLogListExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_13b83a5d7ad5b797b42e0d3b`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ErrorLogListResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "data")]
    pub data: Vec<ErrorLogList>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ErrorLogListResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["pagination", "data"];
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
        map.serialize_entry("pagination", &self.pagination)?;
        map.serialize_entry("data", &self.data)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ErrorLogListResponse {
    pub fn builder() -> ErrorLogListResponseBuilder {
        ErrorLogListResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_13b83a5d7ad5b797b42e0d3b";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ErrorLogListResponseBuilder {
    pagination: Option<Pagination>,
    data: Option<Vec<ErrorLogList>>,
    extensions: Extensions<JsonValue>,
}
impl ErrorLogListResponseBuilder {
    pub fn pagination(mut self, value: Pagination) -> Self {
        self.pagination = Some(value);
        self
    }
    pub fn data(mut self, value: Vec<ErrorLogList>) -> Self {
        self.data = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ErrorLogListResponse> {
        Ok(ErrorLogListResponse {
            pagination: self
                .pagination
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            data: self
                .data
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ErrorLogListResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ErrorLogListResponse::parse(value, mode)
    }
}
impl fmt::Debug for ErrorLogListResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorLogListResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for ErrorLogListResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorLogListResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_6165c9cda31d2c5a43d54950`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ErrorLogListTargetObject {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ErrorLogListTargetObject {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ErrorLogListTargetObject {
    pub fn builder() -> ErrorLogListTargetObjectBuilder {
        ErrorLogListTargetObjectBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_6165c9cda31d2c5a43d54950";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ErrorLogListTargetObjectBuilder {
    extensions: Extensions<JsonValue>,
}
impl ErrorLogListTargetObjectBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ErrorLogListTargetObject> {
        Ok(ErrorLogListTargetObject {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ErrorLogListTargetObject> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ErrorLogListTargetObject::parse(value, mode)
    }
}
impl fmt::Debug for ErrorLogListTargetObjectBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorLogListTargetObjectBuilder([REDACTED])")
    }
}
impl fmt::Debug for ErrorLogListTargetObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorLogListTargetObject([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b115ca25c61f157973c1ec81`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorSource {
    TARGET,
    JOB,
    SYSTEM,
    VALIDATION,
    TARGETPROFILING,
    Unknown(String),
}
impl ErrorSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TARGET => "TARGET",
            Self::JOB => "JOB",
            Self::SYSTEM => "SYSTEM",
            Self::VALIDATION => "VALIDATION",
            Self::TARGETPROFILING => "TARGET_PROFILING",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ErrorSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ErrorSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "TARGET" => Self::TARGET,
            "JOB" => Self::JOB,
            "SYSTEM" => Self::SYSTEM,
            "VALIDATION" => Self::VALIDATION,
            "TARGET_PROFILING" => Self::TARGETPROFILING,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ErrorSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorSource([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_ed8bce48329ea083842f36bd`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorType {
    CONTENTFILTER,
    RATELIMIT,
    AUTHENTICATION,
    NETWORK,
    VALIDATION,
    NETWORKCHANNEL,
    TRANSLATION,
    CUSTOMTARGETADAPTER,
    UNKNOWN,
    Unknown(String),
}
impl ErrorType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CONTENTFILTER => "CONTENT_FILTER",
            Self::RATELIMIT => "RATE_LIMIT",
            Self::AUTHENTICATION => "AUTHENTICATION",
            Self::NETWORK => "NETWORK",
            Self::VALIDATION => "VALIDATION",
            Self::NETWORKCHANNEL => "NETWORK_CHANNEL",
            Self::TRANSLATION => "TRANSLATION",
            Self::CUSTOMTARGETADAPTER => "CUSTOM_TARGET_ADAPTER",
            Self::UNKNOWN => "UNKNOWN",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ErrorType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ErrorType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "CONTENT_FILTER" => Self::CONTENTFILTER,
            "RATE_LIMIT" => Self::RATELIMIT,
            "AUTHENTICATION" => Self::AUTHENTICATION,
            "NETWORK" => Self::NETWORK,
            "VALIDATION" => Self::VALIDATION,
            "NETWORK_CHANNEL" => Self::NETWORKCHANNEL,
            "TRANSLATION" => Self::TRANSLATION,
            "CUSTOM_TARGET_ADAPTER" => Self::CUSTOMTARGETADAPTER,
            "UNKNOWN" => Self::UNKNOWN,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ErrorType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ErrorType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b4706577925113883673532e`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum EtaState {
    Calculating,
    Ready,
    Unknown(String),
}
impl EtaState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Calculating => "calculating",
            Self::Ready => "ready",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for EtaState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for EtaState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "calculating" => Self::Calculating,
            "ready" => Self::Ready,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for EtaState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("EtaState([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_faf609de6fc8f0bb647a8f47`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum FileFormat {
    CSV,
    JSON,
    ALL,
    Unknown(String),
}
impl FileFormat {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CSV => "CSV",
            Self::JSON => "JSON",
            Self::ALL => "ALL",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for FileFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for FileFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "CSV" => Self::CSV,
            "JSON" => Self::JSON,
            "ALL" => Self::ALL,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for FileFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("FileFormat([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_7e69aa21d388d8dbb82e76ec`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Goal {
    #[serde(rename = "goal")]
    pub goal: String,
    #[serde(rename = "goal_english", default)]
    pub goal_english: Patch<String>,
    #[serde(rename = "safe_response")]
    pub safe_response: String,
    #[serde(rename = "jailbroken_response")]
    pub jailbroken_response: String,
    #[serde(rename = "goal_metadata", default)]
    pub goal_metadata: Optional<GoalGoalMetadata>,
    #[serde(rename = "custom_goal", default)]
    pub custom_goal: Optional<bool>,
    #[serde(rename = "goal_type", default)]
    pub goal_type: Optional<GoalType>,
    #[serde(rename = "goal_category", default)]
    pub goal_category: Patch<GoalGoalCategory>,
    #[serde(rename = "goal_subtype", default)]
    pub goal_subtype: Patch<GoalGoalSubtype>,
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "goal_to_show", default)]
    pub goal_to_show: Patch<String>,
    #[serde(rename = "threat", default)]
    pub threat: Optional<bool>,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Patch<GoalExtraInfo>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for Goal {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "goal",
            "goal_english",
            "safe_response",
            "jailbroken_response",
            "goal_metadata",
            "custom_goal",
            "goal_type",
            "goal_category",
            "goal_subtype",
            "uuid",
            "tsg_id",
            "job_id",
            "goal_to_show",
            "threat",
            "version",
            "extra_info",
            "error",
        ];
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
        map.serialize_entry("goal", &self.goal)?;
        if !self.goal_english.is_unset() {
            map.serialize_entry("goal_english", &self.goal_english)?;
        }
        map.serialize_entry("safe_response", &self.safe_response)?;
        map.serialize_entry("jailbroken_response", &self.jailbroken_response)?;
        if !self.goal_metadata.is_unset() {
            map.serialize_entry("goal_metadata", &self.goal_metadata)?;
        }
        if !self.custom_goal.is_unset() {
            map.serialize_entry("custom_goal", &self.custom_goal)?;
        }
        if !self.goal_type.is_unset() {
            map.serialize_entry("goal_type", &self.goal_type)?;
        }
        if !self.goal_category.is_unset() {
            map.serialize_entry("goal_category", &self.goal_category)?;
        }
        if !self.goal_subtype.is_unset() {
            map.serialize_entry("goal_subtype", &self.goal_subtype)?;
        }
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        if !self.goal_to_show.is_unset() {
            map.serialize_entry("goal_to_show", &self.goal_to_show)?;
        }
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl Goal {
    pub fn builder() -> GoalBuilder {
        GoalBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_7e69aa21d388d8dbb82e76ec";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct GoalBuilder {
    goal: Option<String>,
    goal_english: Patch<String>,
    safe_response: Option<String>,
    jailbroken_response: Option<String>,
    goal_metadata: Optional<GoalGoalMetadata>,
    custom_goal: Optional<bool>,
    goal_type: Optional<GoalType>,
    goal_category: Patch<GoalGoalCategory>,
    goal_subtype: Patch<GoalGoalSubtype>,
    uuid: Option<String>,
    tsg_id: Option<String>,
    job_id: Option<String>,
    goal_to_show: Patch<String>,
    threat: Optional<bool>,
    version: Patch<FiniteNumber>,
    extra_info: Patch<GoalExtraInfo>,
    error: Patch<bool>,
    extensions: Extensions<JsonValue>,
}
impl GoalBuilder {
    pub fn goal(mut self, value: String) -> Self {
        self.goal = Some(value);
        self
    }
    pub fn goal_english(mut self, value: Patch<String>) -> Self {
        self.goal_english = value;
        self
    }
    pub fn safe_response(mut self, value: String) -> Self {
        self.safe_response = Some(value);
        self
    }
    pub fn jailbroken_response(mut self, value: String) -> Self {
        self.jailbroken_response = Some(value);
        self
    }
    pub fn goal_metadata(mut self, value: Optional<GoalGoalMetadata>) -> Self {
        self.goal_metadata = value;
        self
    }
    pub fn custom_goal(mut self, value: Optional<bool>) -> Self {
        self.custom_goal = value;
        self
    }
    pub fn goal_type(mut self, value: Optional<GoalType>) -> Self {
        self.goal_type = value;
        self
    }
    pub fn goal_category(mut self, value: Patch<GoalGoalCategory>) -> Self {
        self.goal_category = value;
        self
    }
    pub fn goal_subtype(mut self, value: Patch<GoalGoalSubtype>) -> Self {
        self.goal_subtype = value;
        self
    }
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn goal_to_show(mut self, value: Patch<String>) -> Self {
        self.goal_to_show = value;
        self
    }
    pub fn threat(mut self, value: Optional<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn extra_info(mut self, value: Patch<GoalExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<Goal> {
        Ok(Goal {
            goal: self
                .goal
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            goal_english: self.goal_english,
            safe_response: self
                .safe_response
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            jailbroken_response: self
                .jailbroken_response
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            goal_metadata: self.goal_metadata,
            custom_goal: self.custom_goal,
            goal_type: self.goal_type,
            goal_category: self.goal_category,
            goal_subtype: self.goal_subtype,
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            goal_to_show: self.goal_to_show,
            threat: self.threat,
            version: self.version,
            extra_info: self.extra_info,
            error: self.error,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<Goal> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        Goal::parse(value, mode)
    }
}
impl fmt::Debug for GoalBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalBuilder([REDACTED])")
    }
}
impl fmt::Debug for Goal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Goal([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_1b27a90443884fbccad90a5c`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GoalCategory {
    GOALMANIPULATION,
    PRIVILEGEMISUSE,
    TOOLMISUSE,
    TOOLCHAINING,
    TOXICCONTENTGENERATION,
    MEMORYPOISONING,
    CUSTOM,
    Unknown(String),
}
impl GoalCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GOALMANIPULATION => "GOAL_MANIPULATION",
            Self::PRIVILEGEMISUSE => "PRIVILEGE_MISUSE",
            Self::TOOLMISUSE => "TOOL_MISUSE",
            Self::TOOLCHAINING => "TOOL_CHAINING",
            Self::TOXICCONTENTGENERATION => "TOXIC_CONTENT_GENERATION",
            Self::MEMORYPOISONING => "MEMORY_POISONING",
            Self::CUSTOM => "CUSTOM",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for GoalCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GoalCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "GOAL_MANIPULATION" => Self::GOALMANIPULATION,
            "PRIVILEGE_MISUSE" => Self::PRIVILEGEMISUSE,
            "TOOL_MISUSE" => Self::TOOLMISUSE,
            "TOOL_CHAINING" => Self::TOOLCHAINING,
            "TOXIC_CONTENT_GENERATION" => Self::TOXICCONTENTGENERATION,
            "MEMORY_POISONING" => Self::MEMORYPOISONING,
            "CUSTOM" => Self::CUSTOM,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for GoalCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_749a61d5faad394bc0521e56`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct GoalCategoryDetail {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for GoalCategoryDetail {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["category", "display_name", "description"];
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
        map.serialize_entry("category", &self.category)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("description", &self.description)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl GoalCategoryDetail {
    pub fn builder() -> GoalCategoryDetailBuilder {
        GoalCategoryDetailBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_749a61d5faad394bc0521e56";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct GoalCategoryDetailBuilder {
    category: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl GoalCategoryDetailBuilder {
    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<GoalCategoryDetail> {
        Ok(GoalCategoryDetail {
            category: self
                .category
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<GoalCategoryDetail> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        GoalCategoryDetail::parse(value, mode)
    }
}
impl fmt::Debug for GoalCategoryDetailBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalCategoryDetailBuilder([REDACTED])")
    }
}
impl fmt::Debug for GoalCategoryDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalCategoryDetail([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_2957a38c98e845d479fcefdf`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct GoalCategoryListResponse {
    #[serde(rename = "target_type")]
    pub target_type: String,
    #[serde(rename = "categories")]
    pub categories: Vec<GoalCategoryOption>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for GoalCategoryListResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["target_type", "categories"];
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
        map.serialize_entry("target_type", &self.target_type)?;
        map.serialize_entry("categories", &self.categories)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl GoalCategoryListResponse {
    pub fn builder() -> GoalCategoryListResponseBuilder {
        GoalCategoryListResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_2957a38c98e845d479fcefdf";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct GoalCategoryListResponseBuilder {
    target_type: Option<String>,
    categories: Option<Vec<GoalCategoryOption>>,
    extensions: Extensions<JsonValue>,
}
impl GoalCategoryListResponseBuilder {
    pub fn target_type(mut self, value: String) -> Self {
        self.target_type = Some(value);
        self
    }
    pub fn categories(mut self, value: Vec<GoalCategoryOption>) -> Self {
        self.categories = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<GoalCategoryListResponse> {
        Ok(GoalCategoryListResponse {
            target_type: self
                .target_type
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            categories: self
                .categories
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<GoalCategoryListResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        GoalCategoryListResponse::parse(value, mode)
    }
}
impl fmt::Debug for GoalCategoryListResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalCategoryListResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for GoalCategoryListResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalCategoryListResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_ac5c391296c942f276fee294`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct GoalCategoryOption {
    #[serde(rename = "category")]
    pub category: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for GoalCategoryOption {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["category", "display_name", "description"];
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
        map.serialize_entry("category", &self.category)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("description", &self.description)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl GoalCategoryOption {
    pub fn builder() -> GoalCategoryOptionBuilder {
        GoalCategoryOptionBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_ac5c391296c942f276fee294";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct GoalCategoryOptionBuilder {
    category: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl GoalCategoryOptionBuilder {
    pub fn category(mut self, value: String) -> Self {
        self.category = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<GoalCategoryOption> {
        Ok(GoalCategoryOption {
            category: self
                .category
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<GoalCategoryOption> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        GoalCategoryOption::parse(value, mode)
    }
}
impl fmt::Debug for GoalCategoryOptionBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalCategoryOptionBuilder([REDACTED])")
    }
}
impl fmt::Debug for GoalCategoryOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalCategoryOption([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_179dfcc5e4a2a6cee0613565`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct GoalExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for GoalExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl GoalExtraInfo {
    pub fn builder() -> GoalExtraInfoBuilder {
        GoalExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_179dfcc5e4a2a6cee0613565";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct GoalExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl GoalExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<GoalExtraInfo> {
        Ok(GoalExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<GoalExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        GoalExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for GoalExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for GoalExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_1b62d56de2e660bfc40f3344`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GoalGoalCategory {
    GOALMANIPULATION,
    PRIVILEGEMISUSE,
    TOOLMISUSE,
    TOOLCHAINING,
    TOXICCONTENTGENERATION,
    MEMORYPOISONING,
    CUSTOM,
    Unknown(String),
}
impl GoalGoalCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GOALMANIPULATION => "GOAL_MANIPULATION",
            Self::PRIVILEGEMISUSE => "PRIVILEGE_MISUSE",
            Self::TOOLMISUSE => "TOOL_MISUSE",
            Self::TOOLCHAINING => "TOOL_CHAINING",
            Self::TOXICCONTENTGENERATION => "TOXIC_CONTENT_GENERATION",
            Self::MEMORYPOISONING => "MEMORY_POISONING",
            Self::CUSTOM => "CUSTOM",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for GoalGoalCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GoalGoalCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "GOAL_MANIPULATION" => Self::GOALMANIPULATION,
            "PRIVILEGE_MISUSE" => Self::PRIVILEGEMISUSE,
            "TOOL_MISUSE" => Self::TOOLMISUSE,
            "TOOL_CHAINING" => Self::TOOLCHAINING,
            "TOXIC_CONTENT_GENERATION" => Self::TOXICCONTENTGENERATION,
            "MEMORY_POISONING" => Self::MEMORYPOISONING,
            "CUSTOM" => Self::CUSTOM,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for GoalGoalCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalGoalCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_8da00aa35a4bdc7e21c6186e`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct GoalGoalMetadata {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for GoalGoalMetadata {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl GoalGoalMetadata {
    pub fn builder() -> GoalGoalMetadataBuilder {
        GoalGoalMetadataBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_8da00aa35a4bdc7e21c6186e";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct GoalGoalMetadataBuilder {
    extensions: Extensions<JsonValue>,
}
impl GoalGoalMetadataBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<GoalGoalMetadata> {
        Ok(GoalGoalMetadata {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<GoalGoalMetadata> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        GoalGoalMetadata::parse(value, mode)
    }
}
impl fmt::Debug for GoalGoalMetadataBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalGoalMetadataBuilder([REDACTED])")
    }
}
impl fmt::Debug for GoalGoalMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalGoalMetadata([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_1c83b9f694b76aa450998397`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GoalGoalSubtype {
    MASSDATALEAKAGE,
    UNAUTHORIZEDDATAACCESS,
    PROCESSFLOODING,
    DATAMODIFICATION,
    DATAINJECTION,
    DATADELETION,
    ROLEIDENTITYMODIFICATION,
    UNAUTHORIZEDSIDEEFFECTEXECUTION,
    FRAUDULENTTRANSACTIONEXECUTION,
    RESOURCEEXHAUSTION,
    SCOPEGROUNDED,
    SCOPEUNRELATED,
    POLICY,
    COMMITMENT,
    OBJECTIVE,
    ROLEMISUSE,
    CROSSUSERDATAMISUSE,
    BROADDATAMISUSE,
    SELFPRIVILEGEESCALATION,
    CREDENTIALTHEFT,
    UNAUTHORIZEDTRANSACTION,
    CODEEXECUTION,
    DATAEXFILTRATION,
    DATADESTRUCTION,
    PRIVILEGEESCALATION,
    KNOWLEDGEPOISONING,
    MEMORYPOISONING,
    REMOTECODEEXECUTION,
    SYSTEMPROMPTLEAK,
    TOOLLEAK,
    MALWAREGENERATION,
    BIAS,
    CBRN,
    CYBERCRIME,
    DRUGS,
    HATETOXICABUSE,
    NONVIOLENTCRIMES,
    POLITICAL,
    SELFHARM,
    SEXUAL,
    VIOLENTCRIMESWEAPONS,
    SENSITIVEDATALEAKAGE,
    COMPETITORENDORSEMENTS,
    BRANDTARNISHINGSELFCRITICISM,
    Unknown(String),
}
impl GoalGoalSubtype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MASSDATALEAKAGE => "MASS_DATA_LEAKAGE",
            Self::UNAUTHORIZEDDATAACCESS => "UNAUTHORIZED_DATA_ACCESS",
            Self::PROCESSFLOODING => "PROCESS_FLOODING",
            Self::DATAMODIFICATION => "DATA_MODIFICATION",
            Self::DATAINJECTION => "DATA_INJECTION",
            Self::DATADELETION => "DATA_DELETION",
            Self::ROLEIDENTITYMODIFICATION => "ROLE_IDENTITY_MODIFICATION",
            Self::UNAUTHORIZEDSIDEEFFECTEXECUTION => "UNAUTHORIZED_SIDE_EFFECT_EXECUTION",
            Self::FRAUDULENTTRANSACTIONEXECUTION => "FRAUDULENT_TRANSACTION_EXECUTION",
            Self::RESOURCEEXHAUSTION => "RESOURCE_EXHAUSTION",
            Self::SCOPEGROUNDED => "SCOPE_GROUNDED",
            Self::SCOPEUNRELATED => "SCOPE_UNRELATED",
            Self::POLICY => "POLICY",
            Self::COMMITMENT => "COMMITMENT",
            Self::OBJECTIVE => "OBJECTIVE",
            Self::ROLEMISUSE => "ROLE_MISUSE",
            Self::CROSSUSERDATAMISUSE => "CROSS_USER_DATA_MISUSE",
            Self::BROADDATAMISUSE => "BROAD_DATA_MISUSE",
            Self::SELFPRIVILEGEESCALATION => "SELF_PRIVILEGE_ESCALATION",
            Self::CREDENTIALTHEFT => "CREDENTIAL_THEFT",
            Self::UNAUTHORIZEDTRANSACTION => "UNAUTHORIZED_TRANSACTION",
            Self::CODEEXECUTION => "CODE_EXECUTION",
            Self::DATAEXFILTRATION => "DATA_EXFILTRATION",
            Self::DATADESTRUCTION => "DATA_DESTRUCTION",
            Self::PRIVILEGEESCALATION => "PRIVILEGE_ESCALATION",
            Self::KNOWLEDGEPOISONING => "KNOWLEDGE_POISONING",
            Self::MEMORYPOISONING => "MEMORY_POISONING",
            Self::REMOTECODEEXECUTION => "REMOTE_CODE_EXECUTION",
            Self::SYSTEMPROMPTLEAK => "SYSTEM_PROMPT_LEAK",
            Self::TOOLLEAK => "TOOL_LEAK",
            Self::MALWAREGENERATION => "MALWARE_GENERATION",
            Self::BIAS => "BIAS",
            Self::CBRN => "CBRN",
            Self::CYBERCRIME => "CYBERCRIME",
            Self::DRUGS => "DRUGS",
            Self::HATETOXICABUSE => "HATE_TOXIC_ABUSE",
            Self::NONVIOLENTCRIMES => "NON_VIOLENT_CRIMES",
            Self::POLITICAL => "POLITICAL",
            Self::SELFHARM => "SELF_HARM",
            Self::SEXUAL => "SEXUAL",
            Self::VIOLENTCRIMESWEAPONS => "VIOLENT_CRIMES_WEAPONS",
            Self::SENSITIVEDATALEAKAGE => "SENSITIVE_DATA_LEAKAGE",
            Self::COMPETITORENDORSEMENTS => "COMPETITOR_ENDORSEMENTS",
            Self::BRANDTARNISHINGSELFCRITICISM => "BRAND_TARNISHING_SELF_CRITICISM",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for GoalGoalSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GoalGoalSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "MASS_DATA_LEAKAGE" => Self::MASSDATALEAKAGE,
            "UNAUTHORIZED_DATA_ACCESS" => Self::UNAUTHORIZEDDATAACCESS,
            "PROCESS_FLOODING" => Self::PROCESSFLOODING,
            "DATA_MODIFICATION" => Self::DATAMODIFICATION,
            "DATA_INJECTION" => Self::DATAINJECTION,
            "DATA_DELETION" => Self::DATADELETION,
            "ROLE_IDENTITY_MODIFICATION" => Self::ROLEIDENTITYMODIFICATION,
            "UNAUTHORIZED_SIDE_EFFECT_EXECUTION" => Self::UNAUTHORIZEDSIDEEFFECTEXECUTION,
            "FRAUDULENT_TRANSACTION_EXECUTION" => Self::FRAUDULENTTRANSACTIONEXECUTION,
            "RESOURCE_EXHAUSTION" => Self::RESOURCEEXHAUSTION,
            "SCOPE_GROUNDED" => Self::SCOPEGROUNDED,
            "SCOPE_UNRELATED" => Self::SCOPEUNRELATED,
            "POLICY" => Self::POLICY,
            "COMMITMENT" => Self::COMMITMENT,
            "OBJECTIVE" => Self::OBJECTIVE,
            "ROLE_MISUSE" => Self::ROLEMISUSE,
            "CROSS_USER_DATA_MISUSE" => Self::CROSSUSERDATAMISUSE,
            "BROAD_DATA_MISUSE" => Self::BROADDATAMISUSE,
            "SELF_PRIVILEGE_ESCALATION" => Self::SELFPRIVILEGEESCALATION,
            "CREDENTIAL_THEFT" => Self::CREDENTIALTHEFT,
            "UNAUTHORIZED_TRANSACTION" => Self::UNAUTHORIZEDTRANSACTION,
            "CODE_EXECUTION" => Self::CODEEXECUTION,
            "DATA_EXFILTRATION" => Self::DATAEXFILTRATION,
            "DATA_DESTRUCTION" => Self::DATADESTRUCTION,
            "PRIVILEGE_ESCALATION" => Self::PRIVILEGEESCALATION,
            "KNOWLEDGE_POISONING" => Self::KNOWLEDGEPOISONING,
            "MEMORY_POISONING" => Self::MEMORYPOISONING,
            "REMOTE_CODE_EXECUTION" => Self::REMOTECODEEXECUTION,
            "SYSTEM_PROMPT_LEAK" => Self::SYSTEMPROMPTLEAK,
            "TOOL_LEAK" => Self::TOOLLEAK,
            "MALWARE_GENERATION" => Self::MALWAREGENERATION,
            "BIAS" => Self::BIAS,
            "CBRN" => Self::CBRN,
            "CYBERCRIME" => Self::CYBERCRIME,
            "DRUGS" => Self::DRUGS,
            "HATE_TOXIC_ABUSE" => Self::HATETOXICABUSE,
            "NON_VIOLENT_CRIMES" => Self::NONVIOLENTCRIMES,
            "POLITICAL" => Self::POLITICAL,
            "SELF_HARM" => Self::SELFHARM,
            "SEXUAL" => Self::SEXUAL,
            "VIOLENT_CRIMES_WEAPONS" => Self::VIOLENTCRIMESWEAPONS,
            "SENSITIVE_DATA_LEAKAGE" => Self::SENSITIVEDATALEAKAGE,
            "COMPETITOR_ENDORSEMENTS" => Self::COMPETITORENDORSEMENTS,
            "BRAND_TARNISHING_SELF_CRITICISM" => Self::BRANDTARNISHINGSELFCRITICISM,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for GoalGoalSubtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalGoalSubtype([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_593cfed0cfd40583917a0a5f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct GoalListResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "data")]
    pub data: Vec<GoalResponse>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for GoalListResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["pagination", "data"];
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
        map.serialize_entry("pagination", &self.pagination)?;
        map.serialize_entry("data", &self.data)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl GoalListResponse {
    pub fn builder() -> GoalListResponseBuilder {
        GoalListResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_593cfed0cfd40583917a0a5f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct GoalListResponseBuilder {
    pagination: Option<Pagination>,
    data: Option<Vec<GoalResponse>>,
    extensions: Extensions<JsonValue>,
}
impl GoalListResponseBuilder {
    pub fn pagination(mut self, value: Pagination) -> Self {
        self.pagination = Some(value);
        self
    }
    pub fn data(mut self, value: Vec<GoalResponse>) -> Self {
        self.data = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<GoalListResponse> {
        Ok(GoalListResponse {
            pagination: self
                .pagination
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            data: self
                .data
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<GoalListResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        GoalListResponse::parse(value, mode)
    }
}
impl fmt::Debug for GoalListResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalListResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for GoalListResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalListResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_f24d8775f5b2881535a18ef5`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GoalManipulationSubtype {
    SCOPEGROUNDED,
    SCOPEUNRELATED,
    POLICY,
    COMMITMENT,
    OBJECTIVE,
    Unknown(String),
}
impl GoalManipulationSubtype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::SCOPEGROUNDED => "SCOPE_GROUNDED",
            Self::SCOPEUNRELATED => "SCOPE_UNRELATED",
            Self::POLICY => "POLICY",
            Self::COMMITMENT => "COMMITMENT",
            Self::OBJECTIVE => "OBJECTIVE",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for GoalManipulationSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GoalManipulationSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "SCOPE_GROUNDED" => Self::SCOPEGROUNDED,
            "SCOPE_UNRELATED" => Self::SCOPEUNRELATED,
            "POLICY" => Self::POLICY,
            "COMMITMENT" => Self::COMMITMENT,
            "OBJECTIVE" => Self::OBJECTIVE,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for GoalManipulationSubtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalManipulationSubtype([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_82859d4f62f1c0e881de24f9`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct GoalResponse {
    #[serde(rename = "goal")]
    pub goal: String,
    #[serde(rename = "safe_response")]
    pub safe_response: String,
    #[serde(rename = "jailbroken_response")]
    pub jailbroken_response: String,
    #[serde(rename = "goal_metadata", default)]
    pub goal_metadata: Optional<GoalResponseGoalMetadata>,
    #[serde(rename = "custom_goal", default)]
    pub custom_goal: Optional<bool>,
    #[serde(rename = "goal_type", default)]
    pub goal_type: Optional<GoalType>,
    #[serde(rename = "goal_category", default)]
    pub goal_category: Patch<GoalResponseGoalCategory>,
    #[serde(rename = "goal_subtype", default)]
    pub goal_subtype: Patch<GoalResponseGoalSubtype>,
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "goal_to_show", default)]
    pub goal_to_show: Patch<String>,
    #[serde(rename = "threat", default)]
    pub threat: Optional<bool>,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Patch<GoalResponseExtraInfo>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for GoalResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "goal",
            "safe_response",
            "jailbroken_response",
            "goal_metadata",
            "custom_goal",
            "goal_type",
            "goal_category",
            "goal_subtype",
            "uuid",
            "tsg_id",
            "job_id",
            "goal_to_show",
            "threat",
            "version",
            "extra_info",
            "error",
        ];
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
        map.serialize_entry("goal", &self.goal)?;
        map.serialize_entry("safe_response", &self.safe_response)?;
        map.serialize_entry("jailbroken_response", &self.jailbroken_response)?;
        if !self.goal_metadata.is_unset() {
            map.serialize_entry("goal_metadata", &self.goal_metadata)?;
        }
        if !self.custom_goal.is_unset() {
            map.serialize_entry("custom_goal", &self.custom_goal)?;
        }
        if !self.goal_type.is_unset() {
            map.serialize_entry("goal_type", &self.goal_type)?;
        }
        if !self.goal_category.is_unset() {
            map.serialize_entry("goal_category", &self.goal_category)?;
        }
        if !self.goal_subtype.is_unset() {
            map.serialize_entry("goal_subtype", &self.goal_subtype)?;
        }
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        if !self.goal_to_show.is_unset() {
            map.serialize_entry("goal_to_show", &self.goal_to_show)?;
        }
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl GoalResponse {
    pub fn builder() -> GoalResponseBuilder {
        GoalResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_82859d4f62f1c0e881de24f9";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct GoalResponseBuilder {
    goal: Option<String>,
    safe_response: Option<String>,
    jailbroken_response: Option<String>,
    goal_metadata: Optional<GoalResponseGoalMetadata>,
    custom_goal: Optional<bool>,
    goal_type: Optional<GoalType>,
    goal_category: Patch<GoalResponseGoalCategory>,
    goal_subtype: Patch<GoalResponseGoalSubtype>,
    uuid: Option<String>,
    tsg_id: Option<String>,
    job_id: Option<String>,
    goal_to_show: Patch<String>,
    threat: Optional<bool>,
    version: Patch<FiniteNumber>,
    extra_info: Patch<GoalResponseExtraInfo>,
    error: Patch<bool>,
    extensions: Extensions<JsonValue>,
}
impl GoalResponseBuilder {
    pub fn goal(mut self, value: String) -> Self {
        self.goal = Some(value);
        self
    }
    pub fn safe_response(mut self, value: String) -> Self {
        self.safe_response = Some(value);
        self
    }
    pub fn jailbroken_response(mut self, value: String) -> Self {
        self.jailbroken_response = Some(value);
        self
    }
    pub fn goal_metadata(mut self, value: Optional<GoalResponseGoalMetadata>) -> Self {
        self.goal_metadata = value;
        self
    }
    pub fn custom_goal(mut self, value: Optional<bool>) -> Self {
        self.custom_goal = value;
        self
    }
    pub fn goal_type(mut self, value: Optional<GoalType>) -> Self {
        self.goal_type = value;
        self
    }
    pub fn goal_category(mut self, value: Patch<GoalResponseGoalCategory>) -> Self {
        self.goal_category = value;
        self
    }
    pub fn goal_subtype(mut self, value: Patch<GoalResponseGoalSubtype>) -> Self {
        self.goal_subtype = value;
        self
    }
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn goal_to_show(mut self, value: Patch<String>) -> Self {
        self.goal_to_show = value;
        self
    }
    pub fn threat(mut self, value: Optional<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn extra_info(mut self, value: Patch<GoalResponseExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<GoalResponse> {
        Ok(GoalResponse {
            goal: self
                .goal
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            safe_response: self
                .safe_response
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            jailbroken_response: self
                .jailbroken_response
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            goal_metadata: self.goal_metadata,
            custom_goal: self.custom_goal,
            goal_type: self.goal_type,
            goal_category: self.goal_category,
            goal_subtype: self.goal_subtype,
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            goal_to_show: self.goal_to_show,
            threat: self.threat,
            version: self.version,
            extra_info: self.extra_info,
            error: self.error,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<GoalResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        GoalResponse::parse(value, mode)
    }
}
impl fmt::Debug for GoalResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for GoalResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_38188a491d91952790eea324`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct GoalResponseExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for GoalResponseExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl GoalResponseExtraInfo {
    pub fn builder() -> GoalResponseExtraInfoBuilder {
        GoalResponseExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_38188a491d91952790eea324";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct GoalResponseExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl GoalResponseExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<GoalResponseExtraInfo> {
        Ok(GoalResponseExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<GoalResponseExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        GoalResponseExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for GoalResponseExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalResponseExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for GoalResponseExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalResponseExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c49e7379095eab88d6889f48`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GoalResponseGoalCategory {
    GOALMANIPULATION,
    PRIVILEGEMISUSE,
    TOOLMISUSE,
    TOOLCHAINING,
    TOXICCONTENTGENERATION,
    MEMORYPOISONING,
    CUSTOM,
    Unknown(String),
}
impl GoalResponseGoalCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GOALMANIPULATION => "GOAL_MANIPULATION",
            Self::PRIVILEGEMISUSE => "PRIVILEGE_MISUSE",
            Self::TOOLMISUSE => "TOOL_MISUSE",
            Self::TOOLCHAINING => "TOOL_CHAINING",
            Self::TOXICCONTENTGENERATION => "TOXIC_CONTENT_GENERATION",
            Self::MEMORYPOISONING => "MEMORY_POISONING",
            Self::CUSTOM => "CUSTOM",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for GoalResponseGoalCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GoalResponseGoalCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "GOAL_MANIPULATION" => Self::GOALMANIPULATION,
            "PRIVILEGE_MISUSE" => Self::PRIVILEGEMISUSE,
            "TOOL_MISUSE" => Self::TOOLMISUSE,
            "TOOL_CHAINING" => Self::TOOLCHAINING,
            "TOXIC_CONTENT_GENERATION" => Self::TOXICCONTENTGENERATION,
            "MEMORY_POISONING" => Self::MEMORYPOISONING,
            "CUSTOM" => Self::CUSTOM,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for GoalResponseGoalCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalResponseGoalCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b2fc3401336bfed1b0e08508`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct GoalResponseGoalMetadata {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for GoalResponseGoalMetadata {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl GoalResponseGoalMetadata {
    pub fn builder() -> GoalResponseGoalMetadataBuilder {
        GoalResponseGoalMetadataBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_b2fc3401336bfed1b0e08508";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct GoalResponseGoalMetadataBuilder {
    extensions: Extensions<JsonValue>,
}
impl GoalResponseGoalMetadataBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<GoalResponseGoalMetadata> {
        Ok(GoalResponseGoalMetadata {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<GoalResponseGoalMetadata> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        GoalResponseGoalMetadata::parse(value, mode)
    }
}
impl fmt::Debug for GoalResponseGoalMetadataBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalResponseGoalMetadataBuilder([REDACTED])")
    }
}
impl fmt::Debug for GoalResponseGoalMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalResponseGoalMetadata([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_da645730acc26b689137bf5b`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GoalResponseGoalSubtype {
    MASSDATALEAKAGE,
    UNAUTHORIZEDDATAACCESS,
    PROCESSFLOODING,
    DATAMODIFICATION,
    DATAINJECTION,
    DATADELETION,
    ROLEIDENTITYMODIFICATION,
    UNAUTHORIZEDSIDEEFFECTEXECUTION,
    FRAUDULENTTRANSACTIONEXECUTION,
    RESOURCEEXHAUSTION,
    SCOPEGROUNDED,
    SCOPEUNRELATED,
    POLICY,
    COMMITMENT,
    OBJECTIVE,
    ROLEMISUSE,
    CROSSUSERDATAMISUSE,
    BROADDATAMISUSE,
    SELFPRIVILEGEESCALATION,
    CREDENTIALTHEFT,
    UNAUTHORIZEDTRANSACTION,
    CODEEXECUTION,
    DATAEXFILTRATION,
    DATADESTRUCTION,
    PRIVILEGEESCALATION,
    KNOWLEDGEPOISONING,
    MEMORYPOISONING,
    REMOTECODEEXECUTION,
    SYSTEMPROMPTLEAK,
    TOOLLEAK,
    MALWAREGENERATION,
    BIAS,
    CBRN,
    CYBERCRIME,
    DRUGS,
    HATETOXICABUSE,
    NONVIOLENTCRIMES,
    POLITICAL,
    SELFHARM,
    SEXUAL,
    VIOLENTCRIMESWEAPONS,
    SENSITIVEDATALEAKAGE,
    COMPETITORENDORSEMENTS,
    BRANDTARNISHINGSELFCRITICISM,
    Unknown(String),
}
impl GoalResponseGoalSubtype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MASSDATALEAKAGE => "MASS_DATA_LEAKAGE",
            Self::UNAUTHORIZEDDATAACCESS => "UNAUTHORIZED_DATA_ACCESS",
            Self::PROCESSFLOODING => "PROCESS_FLOODING",
            Self::DATAMODIFICATION => "DATA_MODIFICATION",
            Self::DATAINJECTION => "DATA_INJECTION",
            Self::DATADELETION => "DATA_DELETION",
            Self::ROLEIDENTITYMODIFICATION => "ROLE_IDENTITY_MODIFICATION",
            Self::UNAUTHORIZEDSIDEEFFECTEXECUTION => "UNAUTHORIZED_SIDE_EFFECT_EXECUTION",
            Self::FRAUDULENTTRANSACTIONEXECUTION => "FRAUDULENT_TRANSACTION_EXECUTION",
            Self::RESOURCEEXHAUSTION => "RESOURCE_EXHAUSTION",
            Self::SCOPEGROUNDED => "SCOPE_GROUNDED",
            Self::SCOPEUNRELATED => "SCOPE_UNRELATED",
            Self::POLICY => "POLICY",
            Self::COMMITMENT => "COMMITMENT",
            Self::OBJECTIVE => "OBJECTIVE",
            Self::ROLEMISUSE => "ROLE_MISUSE",
            Self::CROSSUSERDATAMISUSE => "CROSS_USER_DATA_MISUSE",
            Self::BROADDATAMISUSE => "BROAD_DATA_MISUSE",
            Self::SELFPRIVILEGEESCALATION => "SELF_PRIVILEGE_ESCALATION",
            Self::CREDENTIALTHEFT => "CREDENTIAL_THEFT",
            Self::UNAUTHORIZEDTRANSACTION => "UNAUTHORIZED_TRANSACTION",
            Self::CODEEXECUTION => "CODE_EXECUTION",
            Self::DATAEXFILTRATION => "DATA_EXFILTRATION",
            Self::DATADESTRUCTION => "DATA_DESTRUCTION",
            Self::PRIVILEGEESCALATION => "PRIVILEGE_ESCALATION",
            Self::KNOWLEDGEPOISONING => "KNOWLEDGE_POISONING",
            Self::MEMORYPOISONING => "MEMORY_POISONING",
            Self::REMOTECODEEXECUTION => "REMOTE_CODE_EXECUTION",
            Self::SYSTEMPROMPTLEAK => "SYSTEM_PROMPT_LEAK",
            Self::TOOLLEAK => "TOOL_LEAK",
            Self::MALWAREGENERATION => "MALWARE_GENERATION",
            Self::BIAS => "BIAS",
            Self::CBRN => "CBRN",
            Self::CYBERCRIME => "CYBERCRIME",
            Self::DRUGS => "DRUGS",
            Self::HATETOXICABUSE => "HATE_TOXIC_ABUSE",
            Self::NONVIOLENTCRIMES => "NON_VIOLENT_CRIMES",
            Self::POLITICAL => "POLITICAL",
            Self::SELFHARM => "SELF_HARM",
            Self::SEXUAL => "SEXUAL",
            Self::VIOLENTCRIMESWEAPONS => "VIOLENT_CRIMES_WEAPONS",
            Self::SENSITIVEDATALEAKAGE => "SENSITIVE_DATA_LEAKAGE",
            Self::COMPETITORENDORSEMENTS => "COMPETITOR_ENDORSEMENTS",
            Self::BRANDTARNISHINGSELFCRITICISM => "BRAND_TARNISHING_SELF_CRITICISM",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for GoalResponseGoalSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GoalResponseGoalSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "MASS_DATA_LEAKAGE" => Self::MASSDATALEAKAGE,
            "UNAUTHORIZED_DATA_ACCESS" => Self::UNAUTHORIZEDDATAACCESS,
            "PROCESS_FLOODING" => Self::PROCESSFLOODING,
            "DATA_MODIFICATION" => Self::DATAMODIFICATION,
            "DATA_INJECTION" => Self::DATAINJECTION,
            "DATA_DELETION" => Self::DATADELETION,
            "ROLE_IDENTITY_MODIFICATION" => Self::ROLEIDENTITYMODIFICATION,
            "UNAUTHORIZED_SIDE_EFFECT_EXECUTION" => Self::UNAUTHORIZEDSIDEEFFECTEXECUTION,
            "FRAUDULENT_TRANSACTION_EXECUTION" => Self::FRAUDULENTTRANSACTIONEXECUTION,
            "RESOURCE_EXHAUSTION" => Self::RESOURCEEXHAUSTION,
            "SCOPE_GROUNDED" => Self::SCOPEGROUNDED,
            "SCOPE_UNRELATED" => Self::SCOPEUNRELATED,
            "POLICY" => Self::POLICY,
            "COMMITMENT" => Self::COMMITMENT,
            "OBJECTIVE" => Self::OBJECTIVE,
            "ROLE_MISUSE" => Self::ROLEMISUSE,
            "CROSS_USER_DATA_MISUSE" => Self::CROSSUSERDATAMISUSE,
            "BROAD_DATA_MISUSE" => Self::BROADDATAMISUSE,
            "SELF_PRIVILEGE_ESCALATION" => Self::SELFPRIVILEGEESCALATION,
            "CREDENTIAL_THEFT" => Self::CREDENTIALTHEFT,
            "UNAUTHORIZED_TRANSACTION" => Self::UNAUTHORIZEDTRANSACTION,
            "CODE_EXECUTION" => Self::CODEEXECUTION,
            "DATA_EXFILTRATION" => Self::DATAEXFILTRATION,
            "DATA_DESTRUCTION" => Self::DATADESTRUCTION,
            "PRIVILEGE_ESCALATION" => Self::PRIVILEGEESCALATION,
            "KNOWLEDGE_POISONING" => Self::KNOWLEDGEPOISONING,
            "MEMORY_POISONING" => Self::MEMORYPOISONING,
            "REMOTE_CODE_EXECUTION" => Self::REMOTECODEEXECUTION,
            "SYSTEM_PROMPT_LEAK" => Self::SYSTEMPROMPTLEAK,
            "TOOL_LEAK" => Self::TOOLLEAK,
            "MALWARE_GENERATION" => Self::MALWAREGENERATION,
            "BIAS" => Self::BIAS,
            "CBRN" => Self::CBRN,
            "CYBERCRIME" => Self::CYBERCRIME,
            "DRUGS" => Self::DRUGS,
            "HATE_TOXIC_ABUSE" => Self::HATETOXICABUSE,
            "NON_VIOLENT_CRIMES" => Self::NONVIOLENTCRIMES,
            "POLITICAL" => Self::POLITICAL,
            "SELF_HARM" => Self::SELFHARM,
            "SEXUAL" => Self::SEXUAL,
            "VIOLENT_CRIMES_WEAPONS" => Self::VIOLENTCRIMESWEAPONS,
            "SENSITIVE_DATA_LEAKAGE" => Self::SENSITIVEDATALEAKAGE,
            "COMPETITOR_ENDORSEMENTS" => Self::COMPETITORENDORSEMENTS,
            "BRAND_TARNISHING_SELF_CRITICISM" => Self::BRANDTARNISHINGSELFCRITICISM,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for GoalResponseGoalSubtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalResponseGoalSubtype([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_8f370e74cf5b91fe9992c292`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GoalType {
    BASE,
    TOOLMISUSE,
    GOALMANIPULATION,
    PRIVILEGEMISUSE,
    TOOLCHAINING,
    MEMORYPOISONING,
    Unknown(String),
}
impl GoalType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BASE => "BASE",
            Self::TOOLMISUSE => "TOOL_MISUSE",
            Self::GOALMANIPULATION => "GOAL_MANIPULATION",
            Self::PRIVILEGEMISUSE => "PRIVILEGE_MISUSE",
            Self::TOOLCHAINING => "TOOL_CHAINING",
            Self::MEMORYPOISONING => "MEMORY_POISONING",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for GoalType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GoalType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "BASE" => Self::BASE,
            "TOOL_MISUSE" => Self::TOOLMISUSE,
            "GOAL_MANIPULATION" => Self::GOALMANIPULATION,
            "PRIVILEGE_MISUSE" => Self::PRIVILEGEMISUSE,
            "TOOL_CHAINING" => Self::TOOLCHAINING,
            "MEMORY_POISONING" => Self::MEMORYPOISONING,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for GoalType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_932e5fb1d222a3580d4e1008`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GoalTypeQueryParam {
    AGENT,
    HUMANAUGMENTED,
    Unknown(String),
}
impl GoalTypeQueryParam {
    pub fn as_str(&self) -> &str {
        match self {
            Self::AGENT => "AGENT",
            Self::HUMANAUGMENTED => "HUMAN_AUGMENTED",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for GoalTypeQueryParam {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GoalTypeQueryParam {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "AGENT" => Self::AGENT,
            "HUMAN_AUGMENTED" => Self::HUMANAUGMENTED,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for GoalTypeQueryParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GoalTypeQueryParam([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c481d488c3530b24c6ff294a`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum GuardrailAction {
    ALLOW,
    BLOCK,
    Unknown(String),
}
impl GuardrailAction {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ALLOW => "ALLOW",
            Self::BLOCK => "BLOCK",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for GuardrailAction {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for GuardrailAction {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "ALLOW" => Self::ALLOW,
            "BLOCK" => Self::BLOCK,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for GuardrailAction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GuardrailAction([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_53ac78138530b59e0d1e7acc`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct HTTPValidationError {
    #[serde(rename = "detail", default)]
    pub detail: Optional<Vec<ValidationError>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for HTTPValidationError {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["detail"];
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
        if !self.detail.is_unset() {
            map.serialize_entry("detail", &self.detail)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl HTTPValidationError {
    pub fn builder() -> HTTPValidationErrorBuilder {
        HTTPValidationErrorBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_53ac78138530b59e0d1e7acc";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct HTTPValidationErrorBuilder {
    detail: Optional<Vec<ValidationError>>,
    extensions: Extensions<JsonValue>,
}
impl HTTPValidationErrorBuilder {
    pub fn detail(mut self, value: Optional<Vec<ValidationError>>) -> Self {
        self.detail = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<HTTPValidationError> {
        Ok(HTTPValidationError {
            detail: self.detail,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<HTTPValidationError> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        HTTPValidationError::parse(value, mode)
    }
}
impl fmt::Debug for HTTPValidationErrorBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("HTTPValidationErrorBuilder([REDACTED])")
    }
}
impl fmt::Debug for HTTPValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("HTTPValidationError([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_427a022ad8185148c54fb938`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct JobAbortResponse {
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "message")]
    pub message: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for JobAbortResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["job_id", "message"];
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
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("message", &self.message)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl JobAbortResponse {
    pub fn builder() -> JobAbortResponseBuilder {
        JobAbortResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_427a022ad8185148c54fb938";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct JobAbortResponseBuilder {
    job_id: Option<String>,
    message: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl JobAbortResponseBuilder {
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn message(mut self, value: String) -> Self {
        self.message = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<JobAbortResponse> {
        Ok(JobAbortResponse {
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            message: self
                .message
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<JobAbortResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        JobAbortResponse::parse(value, mode)
    }
}
impl fmt::Debug for JobAbortResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobAbortResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for JobAbortResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobAbortResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_e29acefc9d3bc872bfbca7d6`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct JobCreateRequest {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "target")]
    pub target: TargetJobRequest,
    #[serde(rename = "job_type")]
    pub job_type: JobType,
    #[serde(rename = "job_metadata")]
    pub job_metadata: JobCreateRequestJobMetadata,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Patch<JobCreateRequestExtraInfo>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for JobCreateRequest {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "name",
            "target",
            "job_type",
            "job_metadata",
            "version",
            "extra_info",
        ];
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
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("target", &self.target)?;
        map.serialize_entry("job_type", &self.job_type)?;
        map.serialize_entry("job_metadata", &self.job_metadata)?;
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl JobCreateRequest {
    pub fn builder() -> JobCreateRequestBuilder {
        JobCreateRequestBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_e29acefc9d3bc872bfbca7d6";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct JobCreateRequestBuilder {
    name: Option<String>,
    target: Option<TargetJobRequest>,
    job_type: Option<JobType>,
    job_metadata: Option<JobCreateRequestJobMetadata>,
    version: Patch<FiniteNumber>,
    extra_info: Patch<JobCreateRequestExtraInfo>,
    extensions: Extensions<JsonValue>,
}
impl JobCreateRequestBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn target(mut self, value: TargetJobRequest) -> Self {
        self.target = Some(value);
        self
    }
    pub fn job_type(mut self, value: JobType) -> Self {
        self.job_type = Some(value);
        self
    }
    pub fn job_metadata(mut self, value: JobCreateRequestJobMetadata) -> Self {
        self.job_metadata = Some(value);
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn extra_info(mut self, value: Patch<JobCreateRequestExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<JobCreateRequest> {
        Ok(JobCreateRequest {
            name: self
                .name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target: self
                .target
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_type: self
                .job_type
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_metadata: self
                .job_metadata
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            version: self.version,
            extra_info: self.extra_info,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<JobCreateRequest> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        JobCreateRequest::parse(value, mode)
    }
}
impl fmt::Debug for JobCreateRequestBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobCreateRequestBuilder([REDACTED])")
    }
}
impl fmt::Debug for JobCreateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobCreateRequest([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_145da0379078d13d055b4c93`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct JobCreateRequestExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for JobCreateRequestExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl JobCreateRequestExtraInfo {
    pub fn builder() -> JobCreateRequestExtraInfoBuilder {
        JobCreateRequestExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_145da0379078d13d055b4c93";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct JobCreateRequestExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl JobCreateRequestExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<JobCreateRequestExtraInfo> {
        Ok(JobCreateRequestExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<JobCreateRequestExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        JobCreateRequestExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for JobCreateRequestExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobCreateRequestExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for JobCreateRequestExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobCreateRequestExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_0129b27bff583f74dbefbb48`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum JobCreateRequestJobMetadata {
    Variant1(StaticJobMetadata),
    Variant2(DynamicJobMetadata),
    Variant3(CustomJobMetadata),
    Variant4(ClaraJobMetadata),
    Unknown(JsonValue),
}
impl fmt::Debug for JobCreateRequestJobMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobCreateRequestJobMetadata([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_95cc90ed45310a1ba7dac2d1`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct JobListResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "data")]
    pub data: Vec<JobResponse>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for JobListResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["pagination", "data"];
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
        map.serialize_entry("pagination", &self.pagination)?;
        map.serialize_entry("data", &self.data)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl JobListResponse {
    pub fn builder() -> JobListResponseBuilder {
        JobListResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_95cc90ed45310a1ba7dac2d1";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct JobListResponseBuilder {
    pagination: Option<Pagination>,
    data: Option<Vec<JobResponse>>,
    extensions: Extensions<JsonValue>,
}
impl JobListResponseBuilder {
    pub fn pagination(mut self, value: Pagination) -> Self {
        self.pagination = Some(value);
        self
    }
    pub fn data(mut self, value: Vec<JobResponse>) -> Self {
        self.data = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<JobListResponse> {
        Ok(JobListResponse {
            pagination: self
                .pagination
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            data: self
                .data
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<JobListResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        JobListResponse::parse(value, mode)
    }
}
impl fmt::Debug for JobListResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobListResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for JobListResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobListResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_ba536a7a0815f4246dc680ed`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct JobResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "target")]
    pub target: TargetReference,
    #[serde(rename = "job_type")]
    pub job_type: JobType,
    #[serde(rename = "job_metadata")]
    pub job_metadata: JobResponseJobMetadata,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Patch<JobResponseExtraInfo>,
    #[serde(rename = "target_id")]
    pub target_id: String,
    #[serde(rename = "target_type")]
    pub target_type: String,
    #[serde(rename = "total", default)]
    pub total: Patch<FiniteNumber>,
    #[serde(rename = "completed", default)]
    pub completed: Patch<FiniteNumber>,
    #[serde(rename = "status", default)]
    pub status: Optional<JobStatus>,
    #[serde(rename = "score", default)]
    pub score: Patch<FiniteNumber>,
    #[serde(rename = "asr", default)]
    pub asr: Patch<FiniteNumber>,
    #[serde(rename = "time_record", default)]
    pub time_record: Patch<JobTimeRecord>,
    #[serde(rename = "created_at", default)]
    pub created_at: Patch<String>,
    #[serde(rename = "updated_at", default)]
    pub updated_at: Patch<String>,
    #[serde(rename = "created_by_user_id", default)]
    pub created_by_user_id: Patch<String>,
    #[serde(rename = "report_stats", default)]
    pub report_stats: Patch<JobResponseReportStats>,
    #[serde(rename = "metering_quota_uuid", default)]
    pub metering_quota_uuid: Patch<String>,
    #[serde(rename = "counted_towards_quota", default)]
    pub counted_towards_quota: Optional<CountedQuotaEnum>,
    #[serde(rename = "invocation_id", default)]
    pub invocation_id: Patch<String>,
    #[serde(rename = "runtime_metrics", default)]
    pub runtime_metrics: Patch<JobRuntimeMetrics>,
    #[serde(rename = "progress", default)]
    pub progress: Patch<JobResponseProgress>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for JobResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "name",
            "target",
            "job_type",
            "job_metadata",
            "version",
            "extra_info",
            "target_id",
            "target_type",
            "total",
            "completed",
            "status",
            "score",
            "asr",
            "time_record",
            "created_at",
            "updated_at",
            "created_by_user_id",
            "report_stats",
            "metering_quota_uuid",
            "counted_towards_quota",
            "invocation_id",
            "runtime_metrics",
            "progress",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("target", &self.target)?;
        map.serialize_entry("job_type", &self.job_type)?;
        map.serialize_entry("job_metadata", &self.job_metadata)?;
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        map.serialize_entry("target_id", &self.target_id)?;
        map.serialize_entry("target_type", &self.target_type)?;
        if !self.total.is_unset() {
            map.serialize_entry("total", &self.total)?;
        }
        if !self.completed.is_unset() {
            map.serialize_entry("completed", &self.completed)?;
        }
        if !self.status.is_unset() {
            map.serialize_entry("status", &self.status)?;
        }
        if !self.score.is_unset() {
            map.serialize_entry("score", &self.score)?;
        }
        if !self.asr.is_unset() {
            map.serialize_entry("asr", &self.asr)?;
        }
        if !self.time_record.is_unset() {
            map.serialize_entry("time_record", &self.time_record)?;
        }
        if !self.created_at.is_unset() {
            map.serialize_entry("created_at", &self.created_at)?;
        }
        if !self.updated_at.is_unset() {
            map.serialize_entry("updated_at", &self.updated_at)?;
        }
        if !self.created_by_user_id.is_unset() {
            map.serialize_entry("created_by_user_id", &self.created_by_user_id)?;
        }
        if !self.report_stats.is_unset() {
            map.serialize_entry("report_stats", &self.report_stats)?;
        }
        if !self.metering_quota_uuid.is_unset() {
            map.serialize_entry("metering_quota_uuid", &self.metering_quota_uuid)?;
        }
        if !self.counted_towards_quota.is_unset() {
            map.serialize_entry("counted_towards_quota", &self.counted_towards_quota)?;
        }
        if !self.invocation_id.is_unset() {
            map.serialize_entry("invocation_id", &self.invocation_id)?;
        }
        if !self.runtime_metrics.is_unset() {
            map.serialize_entry("runtime_metrics", &self.runtime_metrics)?;
        }
        if !self.progress.is_unset() {
            map.serialize_entry("progress", &self.progress)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl JobResponse {
    pub fn builder() -> JobResponseBuilder {
        JobResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_ba536a7a0815f4246dc680ed";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct JobResponseBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    name: Option<String>,
    target: Option<TargetReference>,
    job_type: Option<JobType>,
    job_metadata: Option<JobResponseJobMetadata>,
    version: Patch<FiniteNumber>,
    extra_info: Patch<JobResponseExtraInfo>,
    target_id: Option<String>,
    target_type: Option<String>,
    total: Patch<FiniteNumber>,
    completed: Patch<FiniteNumber>,
    status: Optional<JobStatus>,
    score: Patch<FiniteNumber>,
    asr: Patch<FiniteNumber>,
    time_record: Patch<JobTimeRecord>,
    created_at: Patch<String>,
    updated_at: Patch<String>,
    created_by_user_id: Patch<String>,
    report_stats: Patch<JobResponseReportStats>,
    metering_quota_uuid: Patch<String>,
    counted_towards_quota: Optional<CountedQuotaEnum>,
    invocation_id: Patch<String>,
    runtime_metrics: Patch<JobRuntimeMetrics>,
    progress: Patch<JobResponseProgress>,
    extensions: Extensions<JsonValue>,
}
impl JobResponseBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn target(mut self, value: TargetReference) -> Self {
        self.target = Some(value);
        self
    }
    pub fn job_type(mut self, value: JobType) -> Self {
        self.job_type = Some(value);
        self
    }
    pub fn job_metadata(mut self, value: JobResponseJobMetadata) -> Self {
        self.job_metadata = Some(value);
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn extra_info(mut self, value: Patch<JobResponseExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn target_id(mut self, value: String) -> Self {
        self.target_id = Some(value);
        self
    }
    pub fn target_type(mut self, value: String) -> Self {
        self.target_type = Some(value);
        self
    }
    pub fn total(mut self, value: Patch<FiniteNumber>) -> Self {
        self.total = value;
        self
    }
    pub fn completed(mut self, value: Patch<FiniteNumber>) -> Self {
        self.completed = value;
        self
    }
    pub fn status(mut self, value: Optional<JobStatus>) -> Self {
        self.status = value;
        self
    }
    pub fn score(mut self, value: Patch<FiniteNumber>) -> Self {
        self.score = value;
        self
    }
    pub fn asr(mut self, value: Patch<FiniteNumber>) -> Self {
        self.asr = value;
        self
    }
    pub fn time_record(mut self, value: Patch<JobTimeRecord>) -> Self {
        self.time_record = value;
        self
    }
    pub fn created_at(mut self, value: Patch<String>) -> Self {
        self.created_at = value;
        self
    }
    pub fn updated_at(mut self, value: Patch<String>) -> Self {
        self.updated_at = value;
        self
    }
    pub fn created_by_user_id(mut self, value: Patch<String>) -> Self {
        self.created_by_user_id = value;
        self
    }
    pub fn report_stats(mut self, value: Patch<JobResponseReportStats>) -> Self {
        self.report_stats = value;
        self
    }
    pub fn metering_quota_uuid(mut self, value: Patch<String>) -> Self {
        self.metering_quota_uuid = value;
        self
    }
    pub fn counted_towards_quota(mut self, value: Optional<CountedQuotaEnum>) -> Self {
        self.counted_towards_quota = value;
        self
    }
    pub fn invocation_id(mut self, value: Patch<String>) -> Self {
        self.invocation_id = value;
        self
    }
    pub fn runtime_metrics(mut self, value: Patch<JobRuntimeMetrics>) -> Self {
        self.runtime_metrics = value;
        self
    }
    pub fn progress(mut self, value: Patch<JobResponseProgress>) -> Self {
        self.progress = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<JobResponse> {
        Ok(JobResponse {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            name: self
                .name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target: self
                .target
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_type: self
                .job_type
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_metadata: self
                .job_metadata
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            version: self.version,
            extra_info: self.extra_info,
            target_id: self
                .target_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target_type: self
                .target_type
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total: self.total,
            completed: self.completed,
            status: self.status,
            score: self.score,
            asr: self.asr,
            time_record: self.time_record,
            created_at: self.created_at,
            updated_at: self.updated_at,
            created_by_user_id: self.created_by_user_id,
            report_stats: self.report_stats,
            metering_quota_uuid: self.metering_quota_uuid,
            counted_towards_quota: self.counted_towards_quota,
            invocation_id: self.invocation_id,
            runtime_metrics: self.runtime_metrics,
            progress: self.progress,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<JobResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        JobResponse::parse(value, mode)
    }
}
impl fmt::Debug for JobResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for JobResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_df4eb45d0ba297fe9f950766`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct JobResponseExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for JobResponseExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl JobResponseExtraInfo {
    pub fn builder() -> JobResponseExtraInfoBuilder {
        JobResponseExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_df4eb45d0ba297fe9f950766";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct JobResponseExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl JobResponseExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<JobResponseExtraInfo> {
        Ok(JobResponseExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<JobResponseExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        JobResponseExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for JobResponseExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobResponseExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for JobResponseExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobResponseExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_f8d2927de047b543503d2660`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum JobResponseJobMetadata {
    Variant1(StaticJobMetadata),
    Variant2(DynamicJobMetadata),
    Variant3(CustomJobMetadata),
    Variant4(ClaraJobMetadata),
    Unknown(JsonValue),
}
impl fmt::Debug for JobResponseJobMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobResponseJobMetadata([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_34800ae52d6410c842bea62f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum JobResponseProgress {
    Variant1(StandardProgress),
    Variant2(AgenticProgress),
    Unknown(JsonValue),
}
impl fmt::Debug for JobResponseProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobResponseProgress([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_fddb6759fee0e30d2627be04`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum JobResponseReportStats {
    Variant1(StaticJobReportStats),
    Variant2(DynamicJobReportStats),
    Variant3(JobResponseReportStatsVariant3),
    Variant4(()),
    Unknown(JsonValue),
}
impl fmt::Debug for JobResponseReportStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobResponseReportStats([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_a5ae1a83f2fb6c700f09a0f3`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct JobResponseReportStatsVariant3 {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for JobResponseReportStatsVariant3 {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl JobResponseReportStatsVariant3 {
    pub fn builder() -> JobResponseReportStatsVariant3Builder {
        JobResponseReportStatsVariant3Builder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_a5ae1a83f2fb6c700f09a0f3";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct JobResponseReportStatsVariant3Builder {
    extensions: Extensions<JsonValue>,
}
impl JobResponseReportStatsVariant3Builder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<JobResponseReportStatsVariant3> {
        Ok(JobResponseReportStatsVariant3 {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<JobResponseReportStatsVariant3> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        JobResponseReportStatsVariant3::parse(value, mode)
    }
}
impl fmt::Debug for JobResponseReportStatsVariant3Builder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobResponseReportStatsVariant3Builder([REDACTED])")
    }
}
impl fmt::Debug for JobResponseReportStatsVariant3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobResponseReportStatsVariant3([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_e9aec08c39ca75d06ed32bf2`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct JobRuntimeMetrics {
    #[serde(rename = "error_count")]
    pub error_count: FiniteNumber,
    #[serde(rename = "error_percentage")]
    pub error_percentage: FiniteNumber,
    #[serde(rename = "eta_seconds", default)]
    pub eta_seconds: Patch<FiniteNumber>,
    #[serde(rename = "eta_state")]
    pub eta_state: EtaState,
    #[serde(rename = "attempts_completed")]
    pub attempts_completed: FiniteNumber,
    #[serde(rename = "attempts_total")]
    pub attempts_total: FiniteNumber,
    #[serde(rename = "attempts_unit")]
    pub attempts_unit: AttemptsUnit,
    #[serde(rename = "currently_attacking", default)]
    pub currently_attacking: Patch<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for JobRuntimeMetrics {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "error_count",
            "error_percentage",
            "eta_seconds",
            "eta_state",
            "attempts_completed",
            "attempts_total",
            "attempts_unit",
            "currently_attacking",
        ];
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
        map.serialize_entry("error_count", &self.error_count)?;
        map.serialize_entry("error_percentage", &self.error_percentage)?;
        if !self.eta_seconds.is_unset() {
            map.serialize_entry("eta_seconds", &self.eta_seconds)?;
        }
        map.serialize_entry("eta_state", &self.eta_state)?;
        map.serialize_entry("attempts_completed", &self.attempts_completed)?;
        map.serialize_entry("attempts_total", &self.attempts_total)?;
        map.serialize_entry("attempts_unit", &self.attempts_unit)?;
        if !self.currently_attacking.is_unset() {
            map.serialize_entry("currently_attacking", &self.currently_attacking)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl JobRuntimeMetrics {
    pub fn builder() -> JobRuntimeMetricsBuilder {
        JobRuntimeMetricsBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_e9aec08c39ca75d06ed32bf2";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct JobRuntimeMetricsBuilder {
    error_count: Option<FiniteNumber>,
    error_percentage: Option<FiniteNumber>,
    eta_seconds: Patch<FiniteNumber>,
    eta_state: Option<EtaState>,
    attempts_completed: Option<FiniteNumber>,
    attempts_total: Option<FiniteNumber>,
    attempts_unit: Option<AttemptsUnit>,
    currently_attacking: Patch<String>,
    extensions: Extensions<JsonValue>,
}
impl JobRuntimeMetricsBuilder {
    pub fn error_count(mut self, value: FiniteNumber) -> Self {
        self.error_count = Some(value);
        self
    }
    pub fn error_percentage(mut self, value: FiniteNumber) -> Self {
        self.error_percentage = Some(value);
        self
    }
    pub fn eta_seconds(mut self, value: Patch<FiniteNumber>) -> Self {
        self.eta_seconds = value;
        self
    }
    pub fn eta_state(mut self, value: EtaState) -> Self {
        self.eta_state = Some(value);
        self
    }
    pub fn attempts_completed(mut self, value: FiniteNumber) -> Self {
        self.attempts_completed = Some(value);
        self
    }
    pub fn attempts_total(mut self, value: FiniteNumber) -> Self {
        self.attempts_total = Some(value);
        self
    }
    pub fn attempts_unit(mut self, value: AttemptsUnit) -> Self {
        self.attempts_unit = Some(value);
        self
    }
    pub fn currently_attacking(mut self, value: Patch<String>) -> Self {
        self.currently_attacking = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<JobRuntimeMetrics> {
        Ok(JobRuntimeMetrics {
            error_count: self
                .error_count
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            error_percentage: self
                .error_percentage
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            eta_seconds: self.eta_seconds,
            eta_state: self
                .eta_state
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            attempts_completed: self
                .attempts_completed
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            attempts_total: self
                .attempts_total
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            attempts_unit: self
                .attempts_unit
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            currently_attacking: self.currently_attacking,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<JobRuntimeMetrics> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        JobRuntimeMetrics::parse(value, mode)
    }
}
impl fmt::Debug for JobRuntimeMetricsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobRuntimeMetricsBuilder([REDACTED])")
    }
}
impl fmt::Debug for JobRuntimeMetrics {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobRuntimeMetrics([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_90cc6673c529eec1dc05cceb`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum JobStatus {
    INIT,
    QUEUED,
    RUNNING,
    COMPLETED,
    PARTIALLYCOMPLETE,
    FAILED,
    ABORTED,
    Unknown(String),
}
impl JobStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::INIT => "INIT",
            Self::QUEUED => "QUEUED",
            Self::RUNNING => "RUNNING",
            Self::COMPLETED => "COMPLETED",
            Self::PARTIALLYCOMPLETE => "PARTIALLY_COMPLETE",
            Self::FAILED => "FAILED",
            Self::ABORTED => "ABORTED",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for JobStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for JobStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "INIT" => Self::INIT,
            "QUEUED" => Self::QUEUED,
            "RUNNING" => Self::RUNNING,
            "COMPLETED" => Self::COMPLETED,
            "PARTIALLY_COMPLETE" => Self::PARTIALLYCOMPLETE,
            "FAILED" => Self::FAILED,
            "ABORTED" => Self::ABORTED,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for JobStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobStatus([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_808d883df0c91c6c6860f24b`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum JobStatusFilter {
    QUEUED,
    RUNNING,
    COMPLETED,
    PARTIALLYCOMPLETE,
    FAILED,
    ABORTED,
    Unknown(String),
}
impl JobStatusFilter {
    pub fn as_str(&self) -> &str {
        match self {
            Self::QUEUED => "QUEUED",
            Self::RUNNING => "RUNNING",
            Self::COMPLETED => "COMPLETED",
            Self::PARTIALLYCOMPLETE => "PARTIALLY_COMPLETE",
            Self::FAILED => "FAILED",
            Self::ABORTED => "ABORTED",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for JobStatusFilter {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for JobStatusFilter {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "QUEUED" => Self::QUEUED,
            "RUNNING" => Self::RUNNING,
            "COMPLETED" => Self::COMPLETED,
            "PARTIALLY_COMPLETE" => Self::PARTIALLYCOMPLETE,
            "FAILED" => Self::FAILED,
            "ABORTED" => Self::ABORTED,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for JobStatusFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobStatusFilter([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_fc542d2346666288ebec016d`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct JobTimeRecord {
    #[serde(rename = "queued_at", default)]
    pub queued_at: Patch<String>,
    #[serde(rename = "started_at", default)]
    pub started_at: Patch<String>,
    #[serde(rename = "completed_at", default)]
    pub completed_at: Patch<String>,
    #[serde(rename = "time_taken", default)]
    pub time_taken: Patch<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for JobTimeRecord {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["queued_at", "started_at", "completed_at", "time_taken"];
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
        if !self.queued_at.is_unset() {
            map.serialize_entry("queued_at", &self.queued_at)?;
        }
        if !self.started_at.is_unset() {
            map.serialize_entry("started_at", &self.started_at)?;
        }
        if !self.completed_at.is_unset() {
            map.serialize_entry("completed_at", &self.completed_at)?;
        }
        if !self.time_taken.is_unset() {
            map.serialize_entry("time_taken", &self.time_taken)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl JobTimeRecord {
    pub fn builder() -> JobTimeRecordBuilder {
        JobTimeRecordBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_fc542d2346666288ebec016d";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct JobTimeRecordBuilder {
    queued_at: Patch<String>,
    started_at: Patch<String>,
    completed_at: Patch<String>,
    time_taken: Patch<String>,
    extensions: Extensions<JsonValue>,
}
impl JobTimeRecordBuilder {
    pub fn queued_at(mut self, value: Patch<String>) -> Self {
        self.queued_at = value;
        self
    }
    pub fn started_at(mut self, value: Patch<String>) -> Self {
        self.started_at = value;
        self
    }
    pub fn completed_at(mut self, value: Patch<String>) -> Self {
        self.completed_at = value;
        self
    }
    pub fn time_taken(mut self, value: Patch<String>) -> Self {
        self.time_taken = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<JobTimeRecord> {
        Ok(JobTimeRecord {
            queued_at: self.queued_at,
            started_at: self.started_at,
            completed_at: self.completed_at,
            time_taken: self.time_taken,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<JobTimeRecord> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        JobTimeRecord::parse(value, mode)
    }
}
impl fmt::Debug for JobTimeRecordBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobTimeRecordBuilder([REDACTED])")
    }
}
impl fmt::Debug for JobTimeRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobTimeRecord([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_9ab9be8948be85feadd818ae`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum JobType {
    STATIC,
    DYNAMIC,
    CUSTOM,
    CLARA,
    Unknown(String),
}
impl JobType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::STATIC => "STATIC",
            Self::DYNAMIC => "DYNAMIC",
            Self::CUSTOM => "CUSTOM",
            Self::CLARA => "CLARA",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for JobType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for JobType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "STATIC" => Self::STATIC,
            "DYNAMIC" => Self::DYNAMIC,
            "CUSTOM" => Self::CUSTOM,
            "CLARA" => Self::CLARA,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for JobType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("JobType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_889e775e33a042d1b88d5c36`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct LanguageDropdownOption {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "country_code", default)]
    pub country_code: Patch<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for LanguageDropdownOption {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["code", "name", "country_code"];
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
        map.serialize_entry("code", &self.code)?;
        map.serialize_entry("name", &self.name)?;
        if !self.country_code.is_unset() {
            map.serialize_entry("country_code", &self.country_code)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl LanguageDropdownOption {
    pub fn builder() -> LanguageDropdownOptionBuilder {
        LanguageDropdownOptionBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_889e775e33a042d1b88d5c36";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct LanguageDropdownOptionBuilder {
    code: Option<String>,
    name: Option<String>,
    country_code: Patch<String>,
    extensions: Extensions<JsonValue>,
}
impl LanguageDropdownOptionBuilder {
    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
        self
    }
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn country_code(mut self, value: Patch<String>) -> Self {
        self.country_code = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<LanguageDropdownOption> {
        Ok(LanguageDropdownOption {
            code: self
                .code
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            name: self
                .name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            country_code: self.country_code,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<LanguageDropdownOption> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        LanguageDropdownOption::parse(value, mode)
    }
}
impl fmt::Debug for LanguageDropdownOptionBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("LanguageDropdownOptionBuilder([REDACTED])")
    }
}
impl fmt::Debug for LanguageDropdownOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("LanguageDropdownOption([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_4ccdeabe68c0e4b2ff767236`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct LanguageOption {
    #[serde(rename = "code")]
    pub code: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for LanguageOption {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["code", "name"];
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
        map.serialize_entry("code", &self.code)?;
        map.serialize_entry("name", &self.name)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl LanguageOption {
    pub fn builder() -> LanguageOptionBuilder {
        LanguageOptionBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_4ccdeabe68c0e4b2ff767236";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct LanguageOptionBuilder {
    code: Option<String>,
    name: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl LanguageOptionBuilder {
    pub fn code(mut self, value: String) -> Self {
        self.code = Some(value);
        self
    }
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<LanguageOption> {
        Ok(LanguageOption {
            code: self
                .code
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            name: self
                .name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<LanguageOption> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        LanguageOption::parse(value, mode)
    }
}
impl fmt::Debug for LanguageOptionBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("LanguageOptionBuilder([REDACTED])")
    }
}
impl fmt::Debug for LanguageOption {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("LanguageOption([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_7ea67ff2c043b88e12b3f6b4`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct MaliciousCodeDetection {
    #[serde(rename = "action", default)]
    pub action: Optional<GuardrailAction>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for MaliciousCodeDetection {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["action"];
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
        if !self.action.is_unset() {
            map.serialize_entry("action", &self.action)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl MaliciousCodeDetection {
    pub fn builder() -> MaliciousCodeDetectionBuilder {
        MaliciousCodeDetectionBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_7ea67ff2c043b88e12b3f6b4";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct MaliciousCodeDetectionBuilder {
    action: Optional<GuardrailAction>,
    extensions: Extensions<JsonValue>,
}
impl MaliciousCodeDetectionBuilder {
    pub fn action(mut self, value: Optional<GuardrailAction>) -> Self {
        self.action = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<MaliciousCodeDetection> {
        Ok(MaliciousCodeDetection {
            action: self.action,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<MaliciousCodeDetection> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        MaliciousCodeDetection::parse(value, mode)
    }
}
impl fmt::Debug for MaliciousCodeDetectionBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MaliciousCodeDetectionBuilder([REDACTED])")
    }
}
impl fmt::Debug for MaliciousCodeDetection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MaliciousCodeDetection([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_24eefca5d65c6e3b1954d205`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct MaliciousURLDetection {
    #[serde(rename = "action", default)]
    pub action: Optional<GuardrailAction>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for MaliciousURLDetection {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["action"];
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
        if !self.action.is_unset() {
            map.serialize_entry("action", &self.action)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl MaliciousURLDetection {
    pub fn builder() -> MaliciousURLDetectionBuilder {
        MaliciousURLDetectionBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_24eefca5d65c6e3b1954d205";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct MaliciousURLDetectionBuilder {
    action: Optional<GuardrailAction>,
    extensions: Extensions<JsonValue>,
}
impl MaliciousURLDetectionBuilder {
    pub fn action(mut self, value: Optional<GuardrailAction>) -> Self {
        self.action = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<MaliciousURLDetection> {
        Ok(MaliciousURLDetection {
            action: self.action,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<MaliciousURLDetection> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        MaliciousURLDetection::parse(value, mode)
    }
}
impl fmt::Debug for MaliciousURLDetectionBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MaliciousURLDetectionBuilder([REDACTED])")
    }
}
impl fmt::Debug for MaliciousURLDetection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MaliciousURLDetection([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b934510a67ec0730415c9443`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum MemoryPoisoningSubtype {
    MEMORYPOISONING,
    Unknown(String),
}
impl MemoryPoisoningSubtype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MEMORYPOISONING => "MEMORY_POISONING",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for MemoryPoisoningSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for MemoryPoisoningSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "MEMORY_POISONING" => Self::MEMORYPOISONING,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for MemoryPoisoningSubtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("MemoryPoisoningSubtype([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_0df350acfa74987fc1e41d39`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum Modality {
    TEXT,
    FILE,
    AUDIO,
    Unknown(String),
}
impl Modality {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TEXT => "TEXT",
            Self::FILE => "FILE",
            Self::AUDIO => "AUDIO",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for Modality {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for Modality {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "TEXT" => Self::TEXT,
            "FILE" => Self::FILE,
            "AUDIO" => Self::AUDIO,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for Modality {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Modality([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c93ba97d3da3b05bb17c1a67`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Pagination {
    #[serde(rename = "total_items", default)]
    pub total_items: Patch<FiniteNumber>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for Pagination {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["total_items"];
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
        if !self.total_items.is_unset() {
            map.serialize_entry("total_items", &self.total_items)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl Pagination {
    pub fn builder() -> PaginationBuilder {
        PaginationBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_c93ba97d3da3b05bb17c1a67";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct PaginationBuilder {
    total_items: Patch<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl PaginationBuilder {
    pub fn total_items(mut self, value: Patch<FiniteNumber>) -> Self {
        self.total_items = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<Pagination> {
        Ok(Pagination {
            total_items: self.total_items,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<Pagination> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        Pagination::parse(value, mode)
    }
}
impl fmt::Debug for PaginationBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PaginationBuilder([REDACTED])")
    }
}
impl fmt::Debug for Pagination {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Pagination([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_0c719de7a500433a05c332f9`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PolicyType {
    PROMPTINJECTION,
    TOXICCONTENT,
    CUSTOMTOPICGUARDRAILS,
    MALICIOUSCODEDETECTION,
    MALICIOUSURLDETECTION,
    SENSITIVEDATAPROTECTION,
    Unknown(String),
}
impl PolicyType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PROMPTINJECTION => "PROMPT_INJECTION",
            Self::TOXICCONTENT => "TOXIC_CONTENT",
            Self::CUSTOMTOPICGUARDRAILS => "CUSTOM_TOPIC_GUARDRAILS",
            Self::MALICIOUSCODEDETECTION => "MALICIOUS_CODE_DETECTION",
            Self::MALICIOUSURLDETECTION => "MALICIOUS_URL_DETECTION",
            Self::SENSITIVEDATAPROTECTION => "SENSITIVE_DATA_PROTECTION",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for PolicyType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PolicyType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "PROMPT_INJECTION" => Self::PROMPTINJECTION,
            "TOXIC_CONTENT" => Self::TOXICCONTENT,
            "CUSTOM_TOPIC_GUARDRAILS" => Self::CUSTOMTOPICGUARDRAILS,
            "MALICIOUS_CODE_DETECTION" => Self::MALICIOUSCODEDETECTION,
            "MALICIOUS_URL_DETECTION" => Self::MALICIOUSURLDETECTION,
            "SENSITIVE_DATA_PROTECTION" => Self::SENSITIVEDATAPROTECTION,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for PolicyType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PolicyType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_684a90fba0e19c4627831ebd`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PrerequisiteModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for PrerequisiteModel {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["id", "display_name", "description"];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("description", &self.description)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PrerequisiteModel {
    pub fn builder() -> PrerequisiteModelBuilder {
        PrerequisiteModelBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_684a90fba0e19c4627831ebd";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct PrerequisiteModelBuilder {
    id: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl PrerequisiteModelBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<PrerequisiteModel> {
        Ok(PrerequisiteModel {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<PrerequisiteModel> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        PrerequisiteModel::parse(value, mode)
    }
}
impl fmt::Debug for PrerequisiteModelBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PrerequisiteModelBuilder([REDACTED])")
    }
}
impl fmt::Debug for PrerequisiteModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PrerequisiteModel([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_d847377a217fae2f98eb9feb`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum PrivilegeMisuseSubtype {
    ROLEMISUSE,
    CROSSUSERDATAMISUSE,
    BROADDATAMISUSE,
    SELFPRIVILEGEESCALATION,
    Unknown(String),
}
impl PrivilegeMisuseSubtype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ROLEMISUSE => "ROLE_MISUSE",
            Self::CROSSUSERDATAMISUSE => "CROSS_USER_DATA_MISUSE",
            Self::BROADDATAMISUSE => "BROAD_DATA_MISUSE",
            Self::SELFPRIVILEGEESCALATION => "SELF_PRIVILEGE_ESCALATION",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for PrivilegeMisuseSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for PrivilegeMisuseSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "ROLE_MISUSE" => Self::ROLEMISUSE,
            "CROSS_USER_DATA_MISUSE" => Self::CROSSUSERDATAMISUSE,
            "BROAD_DATA_MISUSE" => Self::BROADDATAMISUSE,
            "SELF_PRIVILEGE_ESCALATION" => Self::SELFPRIVILEGEESCALATION,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for PrivilegeMisuseSubtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PrivilegeMisuseSubtype([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_afd997cbdd0d24a1dee30e57`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProfilingStatus {
    NOTSTARTED,
    INPROGRESS,
    COMPLETED,
    FAILED,
    PARTIALLYCOMPLETE,
    Unknown(String),
}
impl ProfilingStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NOTSTARTED => "NOT_STARTED",
            Self::INPROGRESS => "IN_PROGRESS",
            Self::COMPLETED => "COMPLETED",
            Self::FAILED => "FAILED",
            Self::PARTIALLYCOMPLETE => "PARTIALLY_COMPLETE",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ProfilingStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ProfilingStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "NOT_STARTED" => Self::NOTSTARTED,
            "IN_PROGRESS" => Self::INPROGRESS,
            "COMPLETED" => Self::COMPLETED,
            "FAILED" => Self::FAILED,
            "PARTIALLY_COMPLETE" => Self::PARTIALLYCOMPLETE,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ProfilingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ProfilingStatus([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_3e4c2bdb504031656775963a`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PromptDetailResponse {
    #[serde(rename = "prompt_id")]
    pub prompt_id: String,
    #[serde(rename = "prompt_text")]
    pub prompt_text: String,
    #[serde(rename = "goal", default)]
    pub goal: Patch<String>,
    #[serde(rename = "user_defined_goal", default)]
    pub user_defined_goal: Optional<bool>,
    #[serde(rename = "properties", default)]
    pub properties: Optional<Vec<PropertyAssignment>>,
    #[serde(rename = "attack_id", default)]
    pub attack_id: Patch<String>,
    #[serde(rename = "threat", default)]
    pub threat: Patch<bool>,
    #[serde(rename = "threat_details", default)]
    pub threat_details: Optional<ThreatDetails>,
    #[serde(rename = "attack_outputs", default)]
    pub attack_outputs: Optional<Vec<String>>,
    #[serde(rename = "asr", default)]
    pub asr: Patch<FiniteNumber>,
    #[serde(rename = "prompt_set_id", default)]
    pub prompt_set_id: Patch<String>,
    #[serde(rename = "prompt_set_name", default)]
    pub prompt_set_name: Patch<String>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for PromptDetailResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "prompt_id",
            "prompt_text",
            "goal",
            "user_defined_goal",
            "properties",
            "attack_id",
            "threat",
            "threat_details",
            "attack_outputs",
            "asr",
            "prompt_set_id",
            "prompt_set_name",
            "error",
        ];
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
        map.serialize_entry("prompt_id", &self.prompt_id)?;
        map.serialize_entry("prompt_text", &self.prompt_text)?;
        if !self.goal.is_unset() {
            map.serialize_entry("goal", &self.goal)?;
        }
        if !self.user_defined_goal.is_unset() {
            map.serialize_entry("user_defined_goal", &self.user_defined_goal)?;
        }
        if !self.properties.is_unset() {
            map.serialize_entry("properties", &self.properties)?;
        }
        if !self.attack_id.is_unset() {
            map.serialize_entry("attack_id", &self.attack_id)?;
        }
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.threat_details.is_unset() {
            map.serialize_entry("threat_details", &self.threat_details)?;
        }
        if !self.attack_outputs.is_unset() {
            map.serialize_entry("attack_outputs", &self.attack_outputs)?;
        }
        if !self.asr.is_unset() {
            map.serialize_entry("asr", &self.asr)?;
        }
        if !self.prompt_set_id.is_unset() {
            map.serialize_entry("prompt_set_id", &self.prompt_set_id)?;
        }
        if !self.prompt_set_name.is_unset() {
            map.serialize_entry("prompt_set_name", &self.prompt_set_name)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PromptDetailResponse {
    pub fn builder() -> PromptDetailResponseBuilder {
        PromptDetailResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_3e4c2bdb504031656775963a";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct PromptDetailResponseBuilder {
    prompt_id: Option<String>,
    prompt_text: Option<String>,
    goal: Patch<String>,
    user_defined_goal: Optional<bool>,
    properties: Optional<Vec<PropertyAssignment>>,
    attack_id: Patch<String>,
    threat: Patch<bool>,
    threat_details: Optional<ThreatDetails>,
    attack_outputs: Optional<Vec<String>>,
    asr: Patch<FiniteNumber>,
    prompt_set_id: Patch<String>,
    prompt_set_name: Patch<String>,
    error: Patch<bool>,
    extensions: Extensions<JsonValue>,
}
impl PromptDetailResponseBuilder {
    pub fn prompt_id(mut self, value: String) -> Self {
        self.prompt_id = Some(value);
        self
    }
    pub fn prompt_text(mut self, value: String) -> Self {
        self.prompt_text = Some(value);
        self
    }
    pub fn goal(mut self, value: Patch<String>) -> Self {
        self.goal = value;
        self
    }
    pub fn user_defined_goal(mut self, value: Optional<bool>) -> Self {
        self.user_defined_goal = value;
        self
    }
    pub fn properties(mut self, value: Optional<Vec<PropertyAssignment>>) -> Self {
        self.properties = value;
        self
    }
    pub fn attack_id(mut self, value: Patch<String>) -> Self {
        self.attack_id = value;
        self
    }
    pub fn threat(mut self, value: Patch<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn threat_details(mut self, value: Optional<ThreatDetails>) -> Self {
        self.threat_details = value;
        self
    }
    pub fn attack_outputs(mut self, value: Optional<Vec<String>>) -> Self {
        self.attack_outputs = value;
        self
    }
    pub fn asr(mut self, value: Patch<FiniteNumber>) -> Self {
        self.asr = value;
        self
    }
    pub fn prompt_set_id(mut self, value: Patch<String>) -> Self {
        self.prompt_set_id = value;
        self
    }
    pub fn prompt_set_name(mut self, value: Patch<String>) -> Self {
        self.prompt_set_name = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<PromptDetailResponse> {
        Ok(PromptDetailResponse {
            prompt_id: self
                .prompt_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt_text: self
                .prompt_text
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            goal: self.goal,
            user_defined_goal: self.user_defined_goal,
            properties: self.properties,
            attack_id: self.attack_id,
            threat: self.threat,
            threat_details: self.threat_details,
            attack_outputs: self.attack_outputs,
            asr: self.asr,
            prompt_set_id: self.prompt_set_id,
            prompt_set_name: self.prompt_set_name,
            error: self.error,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<PromptDetailResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        PromptDetailResponse::parse(value, mode)
    }
}
impl fmt::Debug for PromptDetailResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptDetailResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for PromptDetailResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptDetailResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b93ea13b5d339f860050fa5f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PromptInjectionGuardrail {
    #[serde(rename = "action", default)]
    pub action: Optional<GuardrailAction>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for PromptInjectionGuardrail {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["action"];
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
        if !self.action.is_unset() {
            map.serialize_entry("action", &self.action)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PromptInjectionGuardrail {
    pub fn builder() -> PromptInjectionGuardrailBuilder {
        PromptInjectionGuardrailBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_b93ea13b5d339f860050fa5f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct PromptInjectionGuardrailBuilder {
    action: Optional<GuardrailAction>,
    extensions: Extensions<JsonValue>,
}
impl PromptInjectionGuardrailBuilder {
    pub fn action(mut self, value: Optional<GuardrailAction>) -> Self {
        self.action = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<PromptInjectionGuardrail> {
        Ok(PromptInjectionGuardrail {
            action: self.action,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<PromptInjectionGuardrail> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        PromptInjectionGuardrail::parse(value, mode)
    }
}
impl fmt::Debug for PromptInjectionGuardrailBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptInjectionGuardrailBuilder([REDACTED])")
    }
}
impl fmt::Debug for PromptInjectionGuardrail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptInjectionGuardrail([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_19579cb1aee323e7f2227f31`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PromptSetSummary {
    #[serde(rename = "prompt_set_id")]
    pub prompt_set_id: String,
    #[serde(rename = "prompt_set_name")]
    pub prompt_set_name: String,
    #[serde(rename = "total_prompts")]
    pub total_prompts: FiniteNumber,
    #[serde(rename = "total_attacks")]
    pub total_attacks: FiniteNumber,
    #[serde(rename = "total_threats")]
    pub total_threats: FiniteNumber,
    #[serde(rename = "failed_attacks")]
    pub failed_attacks: FiniteNumber,
    #[serde(rename = "errored_attacks", default)]
    pub errored_attacks: Optional<FiniteNumber>,
    #[serde(rename = "has_errors", default)]
    pub has_errors: Optional<bool>,
    #[serde(rename = "threat_rate")]
    pub threat_rate: FiniteNumber,
    #[serde(rename = "property_names", default)]
    pub property_names: Optional<Vec<String>>,
    #[serde(rename = "property_statistics", default)]
    pub property_statistics: Optional<Vec<PropertyStatistic>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for PromptSetSummary {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "prompt_set_id",
            "prompt_set_name",
            "total_prompts",
            "total_attacks",
            "total_threats",
            "failed_attacks",
            "errored_attacks",
            "has_errors",
            "threat_rate",
            "property_names",
            "property_statistics",
        ];
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
        map.serialize_entry("prompt_set_id", &self.prompt_set_id)?;
        map.serialize_entry("prompt_set_name", &self.prompt_set_name)?;
        map.serialize_entry("total_prompts", &self.total_prompts)?;
        map.serialize_entry("total_attacks", &self.total_attacks)?;
        map.serialize_entry("total_threats", &self.total_threats)?;
        map.serialize_entry("failed_attacks", &self.failed_attacks)?;
        if !self.errored_attacks.is_unset() {
            map.serialize_entry("errored_attacks", &self.errored_attacks)?;
        }
        if !self.has_errors.is_unset() {
            map.serialize_entry("has_errors", &self.has_errors)?;
        }
        map.serialize_entry("threat_rate", &self.threat_rate)?;
        if !self.property_names.is_unset() {
            map.serialize_entry("property_names", &self.property_names)?;
        }
        if !self.property_statistics.is_unset() {
            map.serialize_entry("property_statistics", &self.property_statistics)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PromptSetSummary {
    pub fn builder() -> PromptSetSummaryBuilder {
        PromptSetSummaryBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_19579cb1aee323e7f2227f31";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct PromptSetSummaryBuilder {
    prompt_set_id: Option<String>,
    prompt_set_name: Option<String>,
    total_prompts: Option<FiniteNumber>,
    total_attacks: Option<FiniteNumber>,
    total_threats: Option<FiniteNumber>,
    failed_attacks: Option<FiniteNumber>,
    errored_attacks: Optional<FiniteNumber>,
    has_errors: Optional<bool>,
    threat_rate: Option<FiniteNumber>,
    property_names: Optional<Vec<String>>,
    property_statistics: Optional<Vec<PropertyStatistic>>,
    extensions: Extensions<JsonValue>,
}
impl PromptSetSummaryBuilder {
    pub fn prompt_set_id(mut self, value: String) -> Self {
        self.prompt_set_id = Some(value);
        self
    }
    pub fn prompt_set_name(mut self, value: String) -> Self {
        self.prompt_set_name = Some(value);
        self
    }
    pub fn total_prompts(mut self, value: FiniteNumber) -> Self {
        self.total_prompts = Some(value);
        self
    }
    pub fn total_attacks(mut self, value: FiniteNumber) -> Self {
        self.total_attacks = Some(value);
        self
    }
    pub fn total_threats(mut self, value: FiniteNumber) -> Self {
        self.total_threats = Some(value);
        self
    }
    pub fn failed_attacks(mut self, value: FiniteNumber) -> Self {
        self.failed_attacks = Some(value);
        self
    }
    pub fn errored_attacks(mut self, value: Optional<FiniteNumber>) -> Self {
        self.errored_attacks = value;
        self
    }
    pub fn has_errors(mut self, value: Optional<bool>) -> Self {
        self.has_errors = value;
        self
    }
    pub fn threat_rate(mut self, value: FiniteNumber) -> Self {
        self.threat_rate = Some(value);
        self
    }
    pub fn property_names(mut self, value: Optional<Vec<String>>) -> Self {
        self.property_names = value;
        self
    }
    pub fn property_statistics(mut self, value: Optional<Vec<PropertyStatistic>>) -> Self {
        self.property_statistics = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<PromptSetSummary> {
        Ok(PromptSetSummary {
            prompt_set_id: self
                .prompt_set_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt_set_name: self
                .prompt_set_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_prompts: self
                .total_prompts
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_attacks: self
                .total_attacks
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_threats: self
                .total_threats
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            failed_attacks: self
                .failed_attacks
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            errored_attacks: self.errored_attacks,
            has_errors: self.has_errors,
            threat_rate: self
                .threat_rate
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            property_names: self.property_names,
            property_statistics: self.property_statistics,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<PromptSetSummary> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        PromptSetSummary::parse(value, mode)
    }
}
impl fmt::Debug for PromptSetSummaryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptSetSummaryBuilder([REDACTED])")
    }
}
impl fmt::Debug for PromptSetSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptSetSummary([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_2241a5dede2bf35ecf64113f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PromptSetsReportResponse {
    #[serde(rename = "prompt_sets")]
    pub prompt_sets: Vec<PromptSetSummary>,
    #[serde(rename = "total_prompt_sets")]
    pub total_prompt_sets: FiniteNumber,
    #[serde(rename = "applied_filters", default)]
    pub applied_filters: Optional<PromptSetsReportResponseAppliedFilters>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for PromptSetsReportResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["prompt_sets", "total_prompt_sets", "applied_filters"];
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
        map.serialize_entry("prompt_sets", &self.prompt_sets)?;
        map.serialize_entry("total_prompt_sets", &self.total_prompt_sets)?;
        if !self.applied_filters.is_unset() {
            map.serialize_entry("applied_filters", &self.applied_filters)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PromptSetsReportResponse {
    pub fn builder() -> PromptSetsReportResponseBuilder {
        PromptSetsReportResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_2241a5dede2bf35ecf64113f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct PromptSetsReportResponseBuilder {
    prompt_sets: Option<Vec<PromptSetSummary>>,
    total_prompt_sets: Option<FiniteNumber>,
    applied_filters: Optional<PromptSetsReportResponseAppliedFilters>,
    extensions: Extensions<JsonValue>,
}
impl PromptSetsReportResponseBuilder {
    pub fn prompt_sets(mut self, value: Vec<PromptSetSummary>) -> Self {
        self.prompt_sets = Some(value);
        self
    }
    pub fn total_prompt_sets(mut self, value: FiniteNumber) -> Self {
        self.total_prompt_sets = Some(value);
        self
    }
    pub fn applied_filters(
        mut self,
        value: Optional<PromptSetsReportResponseAppliedFilters>,
    ) -> Self {
        self.applied_filters = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<PromptSetsReportResponse> {
        Ok(PromptSetsReportResponse {
            prompt_sets: self
                .prompt_sets
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_prompt_sets: self
                .total_prompt_sets
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            applied_filters: self.applied_filters,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<PromptSetsReportResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        PromptSetsReportResponse::parse(value, mode)
    }
}
impl fmt::Debug for PromptSetsReportResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptSetsReportResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for PromptSetsReportResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptSetsReportResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_a1d8d52f51eee9c573dd4be6`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PromptSetsReportResponseAppliedFilters {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for PromptSetsReportResponseAppliedFilters {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PromptSetsReportResponseAppliedFilters {
    pub fn builder() -> PromptSetsReportResponseAppliedFiltersBuilder {
        PromptSetsReportResponseAppliedFiltersBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_a1d8d52f51eee9c573dd4be6";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct PromptSetsReportResponseAppliedFiltersBuilder {
    extensions: Extensions<JsonValue>,
}
impl PromptSetsReportResponseAppliedFiltersBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<PromptSetsReportResponseAppliedFilters> {
        Ok(PromptSetsReportResponseAppliedFilters {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<PromptSetsReportResponseAppliedFilters> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        PromptSetsReportResponseAppliedFilters::parse(value, mode)
    }
}
impl fmt::Debug for PromptSetsReportResponseAppliedFiltersBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptSetsReportResponseAppliedFiltersBuilder([REDACTED])")
    }
}
impl fmt::Debug for PromptSetsReportResponseAppliedFilters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PromptSetsReportResponseAppliedFilters([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_9c5deaab185641f530cc0111`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PropertyAssignment {
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "value")]
    pub value: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for PropertyAssignment {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["name", "value"];
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
        map.serialize_entry("name", &self.name)?;
        map.serialize_entry("value", &self.value)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PropertyAssignment {
    pub fn builder() -> PropertyAssignmentBuilder {
        PropertyAssignmentBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_9c5deaab185641f530cc0111";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct PropertyAssignmentBuilder {
    name: Option<String>,
    value: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl PropertyAssignmentBuilder {
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<PropertyAssignment> {
        Ok(PropertyAssignment {
            name: self
                .name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            value: self
                .value
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<PropertyAssignment> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        PropertyAssignment::parse(value, mode)
    }
}
impl fmt::Debug for PropertyAssignmentBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PropertyAssignmentBuilder([REDACTED])")
    }
}
impl fmt::Debug for PropertyAssignment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PropertyAssignment([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b508788b21733bc934e38bab`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PropertyStatistic {
    #[serde(rename = "property_name")]
    pub property_name: String,
    #[serde(rename = "values")]
    pub values: Vec<PropertyValueStatistic>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for PropertyStatistic {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["property_name", "values"];
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
        map.serialize_entry("property_name", &self.property_name)?;
        map.serialize_entry("values", &self.values)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PropertyStatistic {
    pub fn builder() -> PropertyStatisticBuilder {
        PropertyStatisticBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_b508788b21733bc934e38bab";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct PropertyStatisticBuilder {
    property_name: Option<String>,
    values: Option<Vec<PropertyValueStatistic>>,
    extensions: Extensions<JsonValue>,
}
impl PropertyStatisticBuilder {
    pub fn property_name(mut self, value: String) -> Self {
        self.property_name = Some(value);
        self
    }
    pub fn values(mut self, value: Vec<PropertyValueStatistic>) -> Self {
        self.values = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<PropertyStatistic> {
        Ok(PropertyStatistic {
            property_name: self
                .property_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            values: self
                .values
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<PropertyStatistic> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        PropertyStatistic::parse(value, mode)
    }
}
impl fmt::Debug for PropertyStatisticBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PropertyStatisticBuilder([REDACTED])")
    }
}
impl fmt::Debug for PropertyStatistic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PropertyStatistic([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_8e03f2ab6bd121b66a164bf9`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct PropertyValueStatistic {
    #[serde(rename = "value")]
    pub value: String,
    #[serde(rename = "successful_attack_count")]
    pub successful_attack_count: FiniteNumber,
    #[serde(rename = "total_attack_count")]
    pub total_attack_count: FiniteNumber,
    #[serde(rename = "success_rate")]
    pub success_rate: FiniteNumber,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for PropertyValueStatistic {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "value",
            "successful_attack_count",
            "total_attack_count",
            "success_rate",
        ];
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
        map.serialize_entry("value", &self.value)?;
        map.serialize_entry("successful_attack_count", &self.successful_attack_count)?;
        map.serialize_entry("total_attack_count", &self.total_attack_count)?;
        map.serialize_entry("success_rate", &self.success_rate)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl PropertyValueStatistic {
    pub fn builder() -> PropertyValueStatisticBuilder {
        PropertyValueStatisticBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_8e03f2ab6bd121b66a164bf9";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct PropertyValueStatisticBuilder {
    value: Option<String>,
    successful_attack_count: Option<FiniteNumber>,
    total_attack_count: Option<FiniteNumber>,
    success_rate: Option<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl PropertyValueStatisticBuilder {
    pub fn value(mut self, value: String) -> Self {
        self.value = Some(value);
        self
    }
    pub fn successful_attack_count(mut self, value: FiniteNumber) -> Self {
        self.successful_attack_count = Some(value);
        self
    }
    pub fn total_attack_count(mut self, value: FiniteNumber) -> Self {
        self.total_attack_count = Some(value);
        self
    }
    pub fn success_rate(mut self, value: FiniteNumber) -> Self {
        self.success_rate = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<PropertyValueStatistic> {
        Ok(PropertyValueStatistic {
            value: self
                .value
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            successful_attack_count: self
                .successful_attack_count
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total_attack_count: self
                .total_attack_count
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            success_rate: self
                .success_rate
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<PropertyValueStatistic> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        PropertyValueStatistic::parse(value, mode)
    }
}
impl fmt::Debug for PropertyValueStatisticBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PropertyValueStatisticBuilder([REDACTED])")
    }
}
impl fmt::Debug for PropertyValueStatistic {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("PropertyValueStatistic([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_683be522b37768f163c9feab`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct QuotaDetails {
    #[serde(rename = "allocated")]
    pub allocated: FiniteNumber,
    #[serde(rename = "unlimited")]
    pub unlimited: bool,
    #[serde(rename = "consumed")]
    pub consumed: FiniteNumber,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for QuotaDetails {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["allocated", "unlimited", "consumed"];
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
        map.serialize_entry("allocated", &self.allocated)?;
        map.serialize_entry("unlimited", &self.unlimited)?;
        map.serialize_entry("consumed", &self.consumed)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl QuotaDetails {
    pub fn builder() -> QuotaDetailsBuilder {
        QuotaDetailsBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_683be522b37768f163c9feab";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct QuotaDetailsBuilder {
    allocated: Option<FiniteNumber>,
    unlimited: Option<bool>,
    consumed: Option<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl QuotaDetailsBuilder {
    pub fn allocated(mut self, value: FiniteNumber) -> Self {
        self.allocated = Some(value);
        self
    }
    pub fn unlimited(mut self, value: bool) -> Self {
        self.unlimited = Some(value);
        self
    }
    pub fn consumed(mut self, value: FiniteNumber) -> Self {
        self.consumed = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<QuotaDetails> {
        Ok(QuotaDetails {
            allocated: self
                .allocated
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            unlimited: self
                .unlimited
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            consumed: self
                .consumed
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<QuotaDetails> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        QuotaDetails::parse(value, mode)
    }
}
impl fmt::Debug for QuotaDetailsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("QuotaDetailsBuilder([REDACTED])")
    }
}
impl fmt::Debug for QuotaDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("QuotaDetails([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_48b46bd8cf26038c2533adc3`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct QuotaSummary {
    #[serde(rename = "static")]
    pub r#static: QuotaDetails,
    #[serde(rename = "dynamic")]
    pub dynamic: QuotaDetails,
    #[serde(rename = "custom")]
    pub custom: QuotaDetails,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for QuotaSummary {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["static", "dynamic", "custom"];
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
        map.serialize_entry("static", &self.r#static)?;
        map.serialize_entry("dynamic", &self.dynamic)?;
        map.serialize_entry("custom", &self.custom)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl QuotaSummary {
    pub fn builder() -> QuotaSummaryBuilder {
        QuotaSummaryBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_48b46bd8cf26038c2533adc3";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct QuotaSummaryBuilder {
    r#static: Option<QuotaDetails>,
    dynamic: Option<QuotaDetails>,
    custom: Option<QuotaDetails>,
    extensions: Extensions<JsonValue>,
}
impl QuotaSummaryBuilder {
    pub fn r#static(mut self, value: QuotaDetails) -> Self {
        self.r#static = Some(value);
        self
    }
    pub fn dynamic(mut self, value: QuotaDetails) -> Self {
        self.dynamic = Some(value);
        self
    }
    pub fn custom(mut self, value: QuotaDetails) -> Self {
        self.custom = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<QuotaSummary> {
        Ok(QuotaSummary {
            r#static: self
                .r#static
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            dynamic: self
                .dynamic
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            custom: self
                .custom
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<QuotaSummary> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        QuotaSummary::parse(value, mode)
    }
}
impl fmt::Debug for QuotaSummaryBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("QuotaSummaryBuilder([REDACTED])")
    }
}
impl fmt::Debug for QuotaSummary {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("QuotaSummary([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_ff3c9d9b50c6b7e833a47a1f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct RegenerateResponse {
    #[serde(rename = "background_job_id")]
    pub background_job_id: String,
    #[serde(rename = "status")]
    pub status: BackgroundJobStatus,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for RegenerateResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["background_job_id", "status"];
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
        map.serialize_entry("background_job_id", &self.background_job_id)?;
        map.serialize_entry("status", &self.status)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RegenerateResponse {
    pub fn builder() -> RegenerateResponseBuilder {
        RegenerateResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_ff3c9d9b50c6b7e833a47a1f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct RegenerateResponseBuilder {
    background_job_id: Option<String>,
    status: Option<BackgroundJobStatus>,
    extensions: Extensions<JsonValue>,
}
impl RegenerateResponseBuilder {
    pub fn background_job_id(mut self, value: String) -> Self {
        self.background_job_id = Some(value);
        self
    }
    pub fn status(mut self, value: BackgroundJobStatus) -> Self {
        self.status = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<RegenerateResponse> {
        Ok(RegenerateResponse {
            background_job_id: self
                .background_job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            status: self
                .status
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<RegenerateResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        RegenerateResponse::parse(value, mode)
    }
}
impl fmt::Debug for RegenerateResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RegenerateResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for RegenerateResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RegenerateResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_e072b4c1c931f98a4d4f5591`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct RemediationDetail {
    #[serde(rename = "remediation")]
    pub remediation: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "resource_links", default)]
    pub resource_links: Optional<Vec<String>>,
    #[serde(rename = "priority_level", default)]
    pub priority_level: Optional<String>,
    #[serde(rename = "ease_of_implementation_level", default)]
    pub ease_of_implementation_level: Optional<String>,
    #[serde(rename = "effectiveness_level", default)]
    pub effectiveness_level: Optional<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for RemediationDetail {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "remediation",
            "description",
            "resource_links",
            "priority_level",
            "ease_of_implementation_level",
            "effectiveness_level",
        ];
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
        map.serialize_entry("remediation", &self.remediation)?;
        map.serialize_entry("description", &self.description)?;
        if !self.resource_links.is_unset() {
            map.serialize_entry("resource_links", &self.resource_links)?;
        }
        if !self.priority_level.is_unset() {
            map.serialize_entry("priority_level", &self.priority_level)?;
        }
        if !self.ease_of_implementation_level.is_unset() {
            map.serialize_entry(
                "ease_of_implementation_level",
                &self.ease_of_implementation_level,
            )?;
        }
        if !self.effectiveness_level.is_unset() {
            map.serialize_entry("effectiveness_level", &self.effectiveness_level)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RemediationDetail {
    pub fn builder() -> RemediationDetailBuilder {
        RemediationDetailBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_e072b4c1c931f98a4d4f5591";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct RemediationDetailBuilder {
    remediation: Option<String>,
    description: Option<String>,
    resource_links: Optional<Vec<String>>,
    priority_level: Optional<String>,
    ease_of_implementation_level: Optional<String>,
    effectiveness_level: Optional<String>,
    extensions: Extensions<JsonValue>,
}
impl RemediationDetailBuilder {
    pub fn remediation(mut self, value: String) -> Self {
        self.remediation = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn resource_links(mut self, value: Optional<Vec<String>>) -> Self {
        self.resource_links = value;
        self
    }
    pub fn priority_level(mut self, value: Optional<String>) -> Self {
        self.priority_level = value;
        self
    }
    pub fn ease_of_implementation_level(mut self, value: Optional<String>) -> Self {
        self.ease_of_implementation_level = value;
        self
    }
    pub fn effectiveness_level(mut self, value: Optional<String>) -> Self {
        self.effectiveness_level = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<RemediationDetail> {
        Ok(RemediationDetail {
            remediation: self
                .remediation
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            resource_links: self.resource_links,
            priority_level: self.priority_level,
            ease_of_implementation_level: self.ease_of_implementation_level,
            effectiveness_level: self.effectiveness_level,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<RemediationDetail> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        RemediationDetail::parse(value, mode)
    }
}
impl fmt::Debug for RemediationDetailBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RemediationDetailBuilder([REDACTED])")
    }
}
impl fmt::Debug for RemediationDetail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RemediationDetail([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_3410047582cae6eb7f7db338`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct RemediationResponse {
    #[serde(rename = "remediations", default)]
    pub remediations: Optional<Vec<RemediationDetail>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for RemediationResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["remediations"];
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
        if !self.remediations.is_unset() {
            map.serialize_entry("remediations", &self.remediations)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RemediationResponse {
    pub fn builder() -> RemediationResponseBuilder {
        RemediationResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_3410047582cae6eb7f7db338";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct RemediationResponseBuilder {
    remediations: Optional<Vec<RemediationDetail>>,
    extensions: Extensions<JsonValue>,
}
impl RemediationResponseBuilder {
    pub fn remediations(mut self, value: Optional<Vec<RemediationDetail>>) -> Self {
        self.remediations = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<RemediationResponse> {
        Ok(RemediationResponse {
            remediations: self.remediations,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<RemediationResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        RemediationResponse::parse(value, mode)
    }
}
impl fmt::Debug for RemediationResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RemediationResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for RemediationResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RemediationResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_2a824e9a684fd725eb5fd5ae`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ReportDownload {
    #[serde(rename = "filename")]
    pub filename: String,
    #[serde(rename = "download_url")]
    pub download_url: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ReportDownload {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["filename", "download_url"];
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
        map.serialize_entry("filename", &self.filename)?;
        map.serialize_entry("download_url", &self.download_url)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ReportDownload {
    pub fn builder() -> ReportDownloadBuilder {
        ReportDownloadBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_2a824e9a684fd725eb5fd5ae";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ReportDownloadBuilder {
    filename: Option<String>,
    download_url: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl ReportDownloadBuilder {
    pub fn filename(mut self, value: String) -> Self {
        self.filename = Some(value);
        self
    }
    pub fn download_url(mut self, value: String) -> Self {
        self.download_url = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ReportDownload> {
        Ok(ReportDownload {
            filename: self
                .filename
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            download_url: self
                .download_url
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<ReportDownload> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ReportDownload::parse(value, mode)
    }
}
impl fmt::Debug for ReportDownloadBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ReportDownloadBuilder([REDACTED])")
    }
}
impl fmt::Debug for ReportDownload {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ReportDownload([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_e8cd1bccb7db58f3f6014037`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ReportState {
    ACTIVE,
    STALE,
    GENERATING,
    FAILED,
    Unknown(String),
}
impl ReportState {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ACTIVE => "ACTIVE",
            Self::STALE => "STALE",
            Self::GENERATING => "GENERATING",
            Self::FAILED => "FAILED",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ReportState {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ReportState {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "ACTIVE" => Self::ACTIVE,
            "STALE" => Self::STALE,
            "GENERATING" => Self::GENERATING,
            "FAILED" => Self::FAILED,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ReportState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ReportState([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_62ce9efa377c23dfc599fcf5`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ReportStatusResponse {
    #[serde(rename = "report_state")]
    pub report_state: ReportState,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ReportStatusResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["report_state"];
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
        map.serialize_entry("report_state", &self.report_state)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ReportStatusResponse {
    pub fn builder() -> ReportStatusResponseBuilder {
        ReportStatusResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_62ce9efa377c23dfc599fcf5";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ReportStatusResponseBuilder {
    report_state: Option<ReportState>,
    extensions: Extensions<JsonValue>,
}
impl ReportStatusResponseBuilder {
    pub fn report_state(mut self, value: ReportState) -> Self {
        self.report_state = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ReportStatusResponse> {
        Ok(ReportStatusResponse {
            report_state: self
                .report_state
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ReportStatusResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ReportStatusResponse::parse(value, mode)
    }
}
impl fmt::Debug for ReportStatusResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ReportStatusResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for ReportStatusResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ReportStatusResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_a38ab830abe0b5d8f781ed3a`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ResponseMode {
    REST,
    STREAMING,
    WEBSOCKET,
    WEBSOCKETSTREAMING,
    REALTIME,
    Unknown(String),
}
impl ResponseMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::REST => "REST",
            Self::STREAMING => "STREAMING",
            Self::WEBSOCKET => "WEBSOCKET",
            Self::WEBSOCKETSTREAMING => "WEBSOCKET_STREAMING",
            Self::REALTIME => "REALTIME",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ResponseMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ResponseMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "REST" => Self::REST,
            "STREAMING" => Self::STREAMING,
            "WEBSOCKET" => Self::WEBSOCKET,
            "WEBSOCKET_STREAMING" => Self::WEBSOCKETSTREAMING,
            "REALTIME" => Self::REALTIME,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ResponseMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ResponseMode([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_abef946d8888fde087f15a0f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct RiskLevel {
    #[serde(rename = "risk_rating")]
    pub risk_rating: RiskRating,
    #[serde(rename = "total")]
    pub total: FiniteNumber,
    #[serde(rename = "targets_by_type", default)]
    pub targets_by_type: Optional<Vec<CountByName>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for RiskLevel {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["risk_rating", "total", "targets_by_type"];
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
        map.serialize_entry("risk_rating", &self.risk_rating)?;
        map.serialize_entry("total", &self.total)?;
        if !self.targets_by_type.is_unset() {
            map.serialize_entry("targets_by_type", &self.targets_by_type)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RiskLevel {
    pub fn builder() -> RiskLevelBuilder {
        RiskLevelBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_abef946d8888fde087f15a0f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct RiskLevelBuilder {
    risk_rating: Option<RiskRating>,
    total: Option<FiniteNumber>,
    targets_by_type: Optional<Vec<CountByName>>,
    extensions: Extensions<JsonValue>,
}
impl RiskLevelBuilder {
    pub fn risk_rating(mut self, value: RiskRating) -> Self {
        self.risk_rating = Some(value);
        self
    }
    pub fn total(mut self, value: FiniteNumber) -> Self {
        self.total = Some(value);
        self
    }
    pub fn targets_by_type(mut self, value: Optional<Vec<CountByName>>) -> Self {
        self.targets_by_type = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<RiskLevel> {
        Ok(RiskLevel {
            risk_rating: self
                .risk_rating
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total: self
                .total
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            targets_by_type: self.targets_by_type,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<RiskLevel> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        RiskLevel::parse(value, mode)
    }
}
impl fmt::Debug for RiskLevelBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RiskLevelBuilder([REDACTED])")
    }
}
impl fmt::Debug for RiskLevel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RiskLevel([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_9ed9a1ea4d573a62451deb40`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum RiskRating {
    CRITICAL,
    HIGH,
    MEDIUM,
    LOW,
    Unknown(String),
}
impl RiskRating {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CRITICAL => "CRITICAL",
            Self::HIGH => "HIGH",
            Self::MEDIUM => "MEDIUM",
            Self::LOW => "LOW",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for RiskRating {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for RiskRating {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "CRITICAL" => Self::CRITICAL,
            "HIGH" => Self::HIGH,
            "MEDIUM" => Self::MEDIUM,
            "LOW" => Self::LOW,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for RiskRating {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RiskRating([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_8bf8e951aba649bf8a571a01`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct RuntimeProfileUpdateRequest {
    #[serde(rename = "runtime_profile_id")]
    pub runtime_profile_id: String,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for RuntimeProfileUpdateRequest {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["runtime_profile_id"];
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
        map.serialize_entry("runtime_profile_id", &self.runtime_profile_id)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RuntimeProfileUpdateRequest {
    pub fn builder() -> RuntimeProfileUpdateRequestBuilder {
        RuntimeProfileUpdateRequestBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_8bf8e951aba649bf8a571a01";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct RuntimeProfileUpdateRequestBuilder {
    runtime_profile_id: Option<String>,
    extensions: Extensions<JsonValue>,
}
impl RuntimeProfileUpdateRequestBuilder {
    pub fn runtime_profile_id(mut self, value: String) -> Self {
        self.runtime_profile_id = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<RuntimeProfileUpdateRequest> {
        Ok(RuntimeProfileUpdateRequest {
            runtime_profile_id: self
                .runtime_profile_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<RuntimeProfileUpdateRequest> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        RuntimeProfileUpdateRequest::parse(value, mode)
    }
}
impl fmt::Debug for RuntimeProfileUpdateRequestBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RuntimeProfileUpdateRequestBuilder([REDACTED])")
    }
}
impl fmt::Debug for RuntimeProfileUpdateRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RuntimeProfileUpdateRequest([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_959b327a69ea5bd1a0a465af`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct RuntimeSecurityPolicy {
    #[serde(rename = "policy_id")]
    pub policy_id: PolicyType,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "config")]
    pub config: RuntimeSecurityPolicyConfig,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for RuntimeSecurityPolicy {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["policy_id", "display_name", "config"];
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
        map.serialize_entry("policy_id", &self.policy_id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("config", &self.config)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RuntimeSecurityPolicy {
    pub fn builder() -> RuntimeSecurityPolicyBuilder {
        RuntimeSecurityPolicyBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_959b327a69ea5bd1a0a465af";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct RuntimeSecurityPolicyBuilder {
    policy_id: Option<PolicyType>,
    display_name: Option<String>,
    config: Option<RuntimeSecurityPolicyConfig>,
    extensions: Extensions<JsonValue>,
}
impl RuntimeSecurityPolicyBuilder {
    pub fn policy_id(mut self, value: PolicyType) -> Self {
        self.policy_id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn config(mut self, value: RuntimeSecurityPolicyConfig) -> Self {
        self.config = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<RuntimeSecurityPolicy> {
        Ok(RuntimeSecurityPolicy {
            policy_id: self
                .policy_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            config: self
                .config
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<RuntimeSecurityPolicy> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        RuntimeSecurityPolicy::parse(value, mode)
    }
}
impl fmt::Debug for RuntimeSecurityPolicyBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RuntimeSecurityPolicyBuilder([REDACTED])")
    }
}
impl fmt::Debug for RuntimeSecurityPolicy {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RuntimeSecurityPolicy([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_5ed56a6843edff5ba5b35402`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum RuntimeSecurityPolicyConfig {
    Variant1(PromptInjectionGuardrail),
    Variant2(ToxicContentGuardrail),
    Variant3(CustomTopicGuardrails),
    Variant4(MaliciousCodeDetection),
    Variant5(MaliciousURLDetection),
    Variant6(SensitiveDataProtection),
    Unknown(JsonValue),
}
impl fmt::Debug for RuntimeSecurityPolicyConfig {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RuntimeSecurityPolicyConfig([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_9286451e7f1371c2020dece9`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct RuntimeSecurityProfileResponse {
    #[serde(rename = "runtime_security_profile", default)]
    pub runtime_security_profile: Patch<Vec<RuntimeSecurityPolicy>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for RuntimeSecurityProfileResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["runtime_security_profile"];
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
        if !self.runtime_security_profile.is_unset() {
            map.serialize_entry("runtime_security_profile", &self.runtime_security_profile)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl RuntimeSecurityProfileResponse {
    pub fn builder() -> RuntimeSecurityProfileResponseBuilder {
        RuntimeSecurityProfileResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_9286451e7f1371c2020dece9";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct RuntimeSecurityProfileResponseBuilder {
    runtime_security_profile: Patch<Vec<RuntimeSecurityPolicy>>,
    extensions: Extensions<JsonValue>,
}
impl RuntimeSecurityProfileResponseBuilder {
    pub fn runtime_security_profile(mut self, value: Patch<Vec<RuntimeSecurityPolicy>>) -> Self {
        self.runtime_security_profile = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<RuntimeSecurityProfileResponse> {
        Ok(RuntimeSecurityProfileResponse {
            runtime_security_profile: self.runtime_security_profile,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<RuntimeSecurityProfileResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        RuntimeSecurityProfileResponse::parse(value, mode)
    }
}
impl fmt::Debug for RuntimeSecurityProfileResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RuntimeSecurityProfileResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for RuntimeSecurityProfileResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("RuntimeSecurityProfileResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_016adf1101a01c54775be947`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SafetySubCategory {
    BIAS,
    CBRN,
    CYBERCRIME,
    DRUGS,
    HATETOXICABUSE,
    NONVIOLENTCRIMES,
    POLITICAL,
    SELFHARM,
    SEXUAL,
    VIOLENTCRIMESWEAPONS,
    Unknown(String),
}
impl SafetySubCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::BIAS => "BIAS",
            Self::CBRN => "CBRN",
            Self::CYBERCRIME => "CYBERCRIME",
            Self::DRUGS => "DRUGS",
            Self::HATETOXICABUSE => "HATE_TOXIC_ABUSE",
            Self::NONVIOLENTCRIMES => "NON_VIOLENT_CRIMES",
            Self::POLITICAL => "POLITICAL",
            Self::SELFHARM => "SELF_HARM",
            Self::SEXUAL => "SEXUAL",
            Self::VIOLENTCRIMESWEAPONS => "VIOLENT_CRIMES_WEAPONS",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for SafetySubCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for SafetySubCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "BIAS" => Self::BIAS,
            "CBRN" => Self::CBRN,
            "CYBERCRIME" => Self::CYBERCRIME,
            "DRUGS" => Self::DRUGS,
            "HATE_TOXIC_ABUSE" => Self::HATETOXICABUSE,
            "NON_VIOLENT_CRIMES" => Self::NONVIOLENTCRIMES,
            "POLITICAL" => Self::POLITICAL,
            "SELF_HARM" => Self::SELFHARM,
            "SEXUAL" => Self::SEXUAL,
            "VIOLENT_CRIMES_WEAPONS" => Self::VIOLENTCRIMESWEAPONS,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for SafetySubCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SafetySubCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_7bfe6979b30f30ed4cc64d09`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ScanProgressBreakdown {
    #[serde(rename = "label")]
    pub label: String,
    #[serde(rename = "sub_categories", default)]
    pub sub_categories: Optional<Vec<String>>,
    #[serde(rename = "completed")]
    pub completed: FiniteNumber,
    #[serde(rename = "total")]
    pub total: FiniteNumber,
    #[serde(rename = "status")]
    pub status: ScanRowStatus,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ScanProgressBreakdown {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["label", "sub_categories", "completed", "total", "status"];
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
        map.serialize_entry("label", &self.label)?;
        if !self.sub_categories.is_unset() {
            map.serialize_entry("sub_categories", &self.sub_categories)?;
        }
        map.serialize_entry("completed", &self.completed)?;
        map.serialize_entry("total", &self.total)?;
        map.serialize_entry("status", &self.status)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ScanProgressBreakdown {
    pub fn builder() -> ScanProgressBreakdownBuilder {
        ScanProgressBreakdownBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_7bfe6979b30f30ed4cc64d09";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ScanProgressBreakdownBuilder {
    label: Option<String>,
    sub_categories: Optional<Vec<String>>,
    completed: Option<FiniteNumber>,
    total: Option<FiniteNumber>,
    status: Option<ScanRowStatus>,
    extensions: Extensions<JsonValue>,
}
impl ScanProgressBreakdownBuilder {
    pub fn label(mut self, value: String) -> Self {
        self.label = Some(value);
        self
    }
    pub fn sub_categories(mut self, value: Optional<Vec<String>>) -> Self {
        self.sub_categories = value;
        self
    }
    pub fn completed(mut self, value: FiniteNumber) -> Self {
        self.completed = Some(value);
        self
    }
    pub fn total(mut self, value: FiniteNumber) -> Self {
        self.total = Some(value);
        self
    }
    pub fn status(mut self, value: ScanRowStatus) -> Self {
        self.status = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ScanProgressBreakdown> {
        Ok(ScanProgressBreakdown {
            label: self
                .label
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            sub_categories: self.sub_categories,
            completed: self
                .completed
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total: self
                .total
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            status: self
                .status
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ScanProgressBreakdown> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ScanProgressBreakdown::parse(value, mode)
    }
}
impl fmt::Debug for ScanProgressBreakdownBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScanProgressBreakdownBuilder([REDACTED])")
    }
}
impl fmt::Debug for ScanProgressBreakdown {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScanProgressBreakdown([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c8df018d7e468b9eb67e941e`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ScanRowStatus {
    Queued,
    AttackInProgress,
    PartiallyComplete,
    Complete,
    NotStarted,
    Unknown(String),
}
impl ScanRowStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Queued => "queued",
            Self::AttackInProgress => "attack_in_progress",
            Self::PartiallyComplete => "partially_complete",
            Self::Complete => "complete",
            Self::NotStarted => "not_started",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ScanRowStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ScanRowStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "queued" => Self::Queued,
            "attack_in_progress" => Self::AttackInProgress,
            "partially_complete" => Self::PartiallyComplete,
            "complete" => Self::Complete,
            "not_started" => Self::NotStarted,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ScanRowStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScanRowStatus([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_14ba04f6059b605f4d3f8791`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ScanStatisticsResponse {
    #[serde(rename = "total_scans")]
    pub total_scans: FiniteNumber,
    #[serde(rename = "targets_scanned")]
    pub targets_scanned: FiniteNumber,
    #[serde(rename = "targets_scanned_by_type", default)]
    pub targets_scanned_by_type: Optional<Vec<CountByName>>,
    #[serde(rename = "scan_status", default)]
    pub scan_status: Optional<Vec<CountByName>>,
    #[serde(rename = "risk_profile", default)]
    pub risk_profile: Optional<Vec<RiskLevel>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ScanStatisticsResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "total_scans",
            "targets_scanned",
            "targets_scanned_by_type",
            "scan_status",
            "risk_profile",
        ];
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
        map.serialize_entry("total_scans", &self.total_scans)?;
        map.serialize_entry("targets_scanned", &self.targets_scanned)?;
        if !self.targets_scanned_by_type.is_unset() {
            map.serialize_entry("targets_scanned_by_type", &self.targets_scanned_by_type)?;
        }
        if !self.scan_status.is_unset() {
            map.serialize_entry("scan_status", &self.scan_status)?;
        }
        if !self.risk_profile.is_unset() {
            map.serialize_entry("risk_profile", &self.risk_profile)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ScanStatisticsResponse {
    pub fn builder() -> ScanStatisticsResponseBuilder {
        ScanStatisticsResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_14ba04f6059b605f4d3f8791";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ScanStatisticsResponseBuilder {
    total_scans: Option<FiniteNumber>,
    targets_scanned: Option<FiniteNumber>,
    targets_scanned_by_type: Optional<Vec<CountByName>>,
    scan_status: Optional<Vec<CountByName>>,
    risk_profile: Optional<Vec<RiskLevel>>,
    extensions: Extensions<JsonValue>,
}
impl ScanStatisticsResponseBuilder {
    pub fn total_scans(mut self, value: FiniteNumber) -> Self {
        self.total_scans = Some(value);
        self
    }
    pub fn targets_scanned(mut self, value: FiniteNumber) -> Self {
        self.targets_scanned = Some(value);
        self
    }
    pub fn targets_scanned_by_type(mut self, value: Optional<Vec<CountByName>>) -> Self {
        self.targets_scanned_by_type = value;
        self
    }
    pub fn scan_status(mut self, value: Optional<Vec<CountByName>>) -> Self {
        self.scan_status = value;
        self
    }
    pub fn risk_profile(mut self, value: Optional<Vec<RiskLevel>>) -> Self {
        self.risk_profile = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ScanStatisticsResponse> {
        Ok(ScanStatisticsResponse {
            total_scans: self
                .total_scans
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            targets_scanned: self
                .targets_scanned
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            targets_scanned_by_type: self.targets_scanned_by_type,
            scan_status: self.scan_status,
            risk_profile: self.risk_profile,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ScanStatisticsResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ScanStatisticsResponse::parse(value, mode)
    }
}
impl fmt::Debug for ScanStatisticsResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScanStatisticsResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for ScanStatisticsResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScanStatisticsResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_42be594bad7ab5f5331c51dc`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ScoreTrendResponse {
    #[serde(rename = "labels")]
    pub labels: Vec<String>,
    #[serde(rename = "iso_labels", default)]
    pub iso_labels: Optional<Vec<String>>,
    #[serde(rename = "series")]
    pub series: Vec<ScoreTrendSeries>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ScoreTrendResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["labels", "iso_labels", "series"];
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
        map.serialize_entry("labels", &self.labels)?;
        if !self.iso_labels.is_unset() {
            map.serialize_entry("iso_labels", &self.iso_labels)?;
        }
        map.serialize_entry("series", &self.series)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ScoreTrendResponse {
    pub fn builder() -> ScoreTrendResponseBuilder {
        ScoreTrendResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_42be594bad7ab5f5331c51dc";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ScoreTrendResponseBuilder {
    labels: Option<Vec<String>>,
    iso_labels: Optional<Vec<String>>,
    series: Option<Vec<ScoreTrendSeries>>,
    extensions: Extensions<JsonValue>,
}
impl ScoreTrendResponseBuilder {
    pub fn labels(mut self, value: Vec<String>) -> Self {
        self.labels = Some(value);
        self
    }
    pub fn iso_labels(mut self, value: Optional<Vec<String>>) -> Self {
        self.iso_labels = value;
        self
    }
    pub fn series(mut self, value: Vec<ScoreTrendSeries>) -> Self {
        self.series = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ScoreTrendResponse> {
        Ok(ScoreTrendResponse {
            labels: self
                .labels
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            iso_labels: self.iso_labels,
            series: self
                .series
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ScoreTrendResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ScoreTrendResponse::parse(value, mode)
    }
}
impl fmt::Debug for ScoreTrendResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScoreTrendResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for ScoreTrendResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScoreTrendResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_77d9c23cd02a8d129d9083d3`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ScoreTrendSeries {
    #[serde(rename = "label")]
    pub label: JobType,
    #[serde(rename = "data")]
    pub data: Vec<ScoreTrendSeriesDataItem>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ScoreTrendSeries {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["label", "data"];
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
        map.serialize_entry("label", &self.label)?;
        map.serialize_entry("data", &self.data)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ScoreTrendSeries {
    pub fn builder() -> ScoreTrendSeriesBuilder {
        ScoreTrendSeriesBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_77d9c23cd02a8d129d9083d3";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ScoreTrendSeriesBuilder {
    label: Option<JobType>,
    data: Option<Vec<ScoreTrendSeriesDataItem>>,
    extensions: Extensions<JsonValue>,
}
impl ScoreTrendSeriesBuilder {
    pub fn label(mut self, value: JobType) -> Self {
        self.label = Some(value);
        self
    }
    pub fn data(mut self, value: Vec<ScoreTrendSeriesDataItem>) -> Self {
        self.data = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ScoreTrendSeries> {
        Ok(ScoreTrendSeries {
            label: self
                .label
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            data: self
                .data
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<ScoreTrendSeries> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ScoreTrendSeries::parse(value, mode)
    }
}
impl fmt::Debug for ScoreTrendSeriesBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScoreTrendSeriesBuilder([REDACTED])")
    }
}
impl fmt::Debug for ScoreTrendSeries {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScoreTrendSeries([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b9428b2255e2bd7c52159e22`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ScoreTrendSeriesDataItem {
    Variant1(ScoreTrendSeriesDataItemVariant1),
    Variant2(()),
    Unknown(JsonValue),
}
impl fmt::Debug for ScoreTrendSeriesDataItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScoreTrendSeriesDataItem([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_13cf47003520e015997a9124`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ScoreTrendSeriesDataItemVariant1(FiniteNumber);
impl ScoreTrendSeriesDataItemVariant1 {
    pub fn new(value: FiniteNumber) -> crate::Result<Self> {
        crate::validation::parse(
            "sdp_q_13cf47003520e015997a9124",
            serde_json::to_value(&value)
                .map_err(|_| crate::Error::new(crate::ErrorKind::Validation))?,
            crate::validation::Mode::Response,
        )?;
        Ok(Self(value))
    }
    pub fn as_inner(&self) -> &FiniteNumber {
        &self.0
    }
    pub fn into_inner(self) -> FiniteNumber {
        self.0
    }
}
impl<'de> Deserialize<'de> for ScoreTrendSeriesDataItemVariant1 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = FiniteNumber::deserialize(d)?;
        Self::new(value).map_err(|_| serde::de::Error::custom("number contract mismatch"))
    }
}
impl fmt::Debug for ScoreTrendSeriesDataItemVariant1 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ScoreTrendSeriesDataItemVariant1([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_62939f57db9f8c957da58ded`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SecuritySubCategory {
    ADVERSARIALSUFFIX,
    EVASION,
    INDIRECTPROMPTINJECTION,
    JAILBREAK,
    MULTITURN,
    PROMPTINJECTION,
    REMOTECODEEXECUTION,
    SYSTEMPROMPTLEAK,
    TOOLLEAK,
    MALWAREGENERATION,
    Unknown(String),
}
impl SecuritySubCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ADVERSARIALSUFFIX => "ADVERSARIAL_SUFFIX",
            Self::EVASION => "EVASION",
            Self::INDIRECTPROMPTINJECTION => "INDIRECT_PROMPT_INJECTION",
            Self::JAILBREAK => "JAILBREAK",
            Self::MULTITURN => "MULTI_TURN",
            Self::PROMPTINJECTION => "PROMPT_INJECTION",
            Self::REMOTECODEEXECUTION => "REMOTE_CODE_EXECUTION",
            Self::SYSTEMPROMPTLEAK => "SYSTEM_PROMPT_LEAK",
            Self::TOOLLEAK => "TOOL_LEAK",
            Self::MALWAREGENERATION => "MALWARE_GENERATION",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for SecuritySubCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for SecuritySubCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "ADVERSARIAL_SUFFIX" => Self::ADVERSARIALSUFFIX,
            "EVASION" => Self::EVASION,
            "INDIRECT_PROMPT_INJECTION" => Self::INDIRECTPROMPTINJECTION,
            "JAILBREAK" => Self::JAILBREAK,
            "MULTI_TURN" => Self::MULTITURN,
            "PROMPT_INJECTION" => Self::PROMPTINJECTION,
            "REMOTE_CODE_EXECUTION" => Self::REMOTECODEEXECUTION,
            "SYSTEM_PROMPT_LEAK" => Self::SYSTEMPROMPTLEAK,
            "TOOL_LEAK" => Self::TOOLLEAK,
            "MALWARE_GENERATION" => Self::MALWAREGENERATION,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for SecuritySubCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SecuritySubCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_71629a4ad7aa936ff11bd77b`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct SensitiveDataProtection {
    #[serde(rename = "action", default)]
    pub action: Optional<GuardrailAction>,
    #[serde(rename = "masking_sensitive_data", default)]
    pub masking_sensitive_data: Optional<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for SensitiveDataProtection {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["action", "masking_sensitive_data"];
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
        if !self.action.is_unset() {
            map.serialize_entry("action", &self.action)?;
        }
        if !self.masking_sensitive_data.is_unset() {
            map.serialize_entry("masking_sensitive_data", &self.masking_sensitive_data)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SensitiveDataProtection {
    pub fn builder() -> SensitiveDataProtectionBuilder {
        SensitiveDataProtectionBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_71629a4ad7aa936ff11bd77b";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct SensitiveDataProtectionBuilder {
    action: Optional<GuardrailAction>,
    masking_sensitive_data: Optional<bool>,
    extensions: Extensions<JsonValue>,
}
impl SensitiveDataProtectionBuilder {
    pub fn action(mut self, value: Optional<GuardrailAction>) -> Self {
        self.action = value;
        self
    }
    pub fn masking_sensitive_data(mut self, value: Optional<bool>) -> Self {
        self.masking_sensitive_data = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<SensitiveDataProtection> {
        Ok(SensitiveDataProtection {
            action: self.action,
            masking_sensitive_data: self.masking_sensitive_data,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<SensitiveDataProtection> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        SensitiveDataProtection::parse(value, mode)
    }
}
impl fmt::Debug for SensitiveDataProtectionBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SensitiveDataProtectionBuilder([REDACTED])")
    }
}
impl fmt::Debug for SensitiveDataProtection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SensitiveDataProtection([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_39c90319d2a729a72aa94434`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct SentimentRequest {
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "up_vote", default)]
    pub up_vote: Optional<bool>,
    #[serde(rename = "down_vote", default)]
    pub down_vote: Optional<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for SentimentRequest {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["job_id", "up_vote", "down_vote"];
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
        map.serialize_entry("job_id", &self.job_id)?;
        if !self.up_vote.is_unset() {
            map.serialize_entry("up_vote", &self.up_vote)?;
        }
        if !self.down_vote.is_unset() {
            map.serialize_entry("down_vote", &self.down_vote)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SentimentRequest {
    pub fn builder() -> SentimentRequestBuilder {
        SentimentRequestBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_39c90319d2a729a72aa94434";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct SentimentRequestBuilder {
    job_id: Option<String>,
    up_vote: Optional<bool>,
    down_vote: Optional<bool>,
    extensions: Extensions<JsonValue>,
}
impl SentimentRequestBuilder {
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn up_vote(mut self, value: Optional<bool>) -> Self {
        self.up_vote = value;
        self
    }
    pub fn down_vote(mut self, value: Optional<bool>) -> Self {
        self.down_vote = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<SentimentRequest> {
        Ok(SentimentRequest {
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            up_vote: self.up_vote,
            down_vote: self.down_vote,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<SentimentRequest> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        SentimentRequest::parse(value, mode)
    }
}
impl fmt::Debug for SentimentRequestBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SentimentRequestBuilder([REDACTED])")
    }
}
impl fmt::Debug for SentimentRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SentimentRequest([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c9325b07c2a0f532cb38c8d2`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct SentimentResponse {
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "up_vote", default)]
    pub up_vote: Optional<bool>,
    #[serde(rename = "down_vote", default)]
    pub down_vote: Optional<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for SentimentResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["job_id", "up_vote", "down_vote"];
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
        map.serialize_entry("job_id", &self.job_id)?;
        if !self.up_vote.is_unset() {
            map.serialize_entry("up_vote", &self.up_vote)?;
        }
        if !self.down_vote.is_unset() {
            map.serialize_entry("down_vote", &self.down_vote)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SentimentResponse {
    pub fn builder() -> SentimentResponseBuilder {
        SentimentResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_c9325b07c2a0f532cb38c8d2";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct SentimentResponseBuilder {
    job_id: Option<String>,
    up_vote: Optional<bool>,
    down_vote: Optional<bool>,
    extensions: Extensions<JsonValue>,
}
impl SentimentResponseBuilder {
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn up_vote(mut self, value: Optional<bool>) -> Self {
        self.up_vote = value;
        self
    }
    pub fn down_vote(mut self, value: Optional<bool>) -> Self {
        self.down_vote = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<SentimentResponse> {
        Ok(SentimentResponse {
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            up_vote: self.up_vote,
            down_vote: self.down_vote,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<SentimentResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        SentimentResponse::parse(value, mode)
    }
}
impl fmt::Debug for SentimentResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SentimentResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for SentimentResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SentimentResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_82de231fce3dbb6dbcc63ad0`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum SeverityFilter {
    LOW,
    MEDIUM,
    HIGH,
    CRITICAL,
    Unknown(String),
}
impl SeverityFilter {
    pub fn as_str(&self) -> &str {
        match self {
            Self::LOW => "LOW",
            Self::MEDIUM => "MEDIUM",
            Self::HIGH => "HIGH",
            Self::CRITICAL => "CRITICAL",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for SeverityFilter {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for SeverityFilter {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "LOW" => Self::LOW,
            "MEDIUM" => Self::MEDIUM,
            "HIGH" => Self::HIGH,
            "CRITICAL" => Self::CRITICAL,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for SeverityFilter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SeverityFilter([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_f365b95067211b1fb1eca51a`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct SeverityReport {
    #[serde(rename = "stats")]
    pub stats: Vec<SeverityStats>,
    #[serde(rename = "successful", default)]
    pub successful: Optional<FiniteNumber>,
    #[serde(rename = "failed", default)]
    pub failed: Optional<FiniteNumber>,
    #[serde(rename = "total_attacks", default)]
    pub total_attacks: Optional<FiniteNumber>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for SeverityReport {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["stats", "successful", "failed", "total_attacks"];
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
        map.serialize_entry("stats", &self.stats)?;
        if !self.successful.is_unset() {
            map.serialize_entry("successful", &self.successful)?;
        }
        if !self.failed.is_unset() {
            map.serialize_entry("failed", &self.failed)?;
        }
        if !self.total_attacks.is_unset() {
            map.serialize_entry("total_attacks", &self.total_attacks)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SeverityReport {
    pub fn builder() -> SeverityReportBuilder {
        SeverityReportBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_f365b95067211b1fb1eca51a";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct SeverityReportBuilder {
    stats: Option<Vec<SeverityStats>>,
    successful: Optional<FiniteNumber>,
    failed: Optional<FiniteNumber>,
    total_attacks: Optional<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl SeverityReportBuilder {
    pub fn stats(mut self, value: Vec<SeverityStats>) -> Self {
        self.stats = Some(value);
        self
    }
    pub fn successful(mut self, value: Optional<FiniteNumber>) -> Self {
        self.successful = value;
        self
    }
    pub fn failed(mut self, value: Optional<FiniteNumber>) -> Self {
        self.failed = value;
        self
    }
    pub fn total_attacks(mut self, value: Optional<FiniteNumber>) -> Self {
        self.total_attacks = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<SeverityReport> {
        Ok(SeverityReport {
            stats: self
                .stats
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            successful: self.successful,
            failed: self.failed,
            total_attacks: self.total_attacks,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<SeverityReport> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        SeverityReport::parse(value, mode)
    }
}
impl fmt::Debug for SeverityReportBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SeverityReportBuilder([REDACTED])")
    }
}
impl fmt::Debug for SeverityReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SeverityReport([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c774d4b70528cba19fac20d0`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct SeverityStats {
    #[serde(rename = "severity")]
    pub severity: String,
    #[serde(rename = "successful", default)]
    pub successful: Optional<FiniteNumber>,
    #[serde(rename = "failed", default)]
    pub failed: Optional<FiniteNumber>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for SeverityStats {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["severity", "successful", "failed"];
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
        map.serialize_entry("severity", &self.severity)?;
        if !self.successful.is_unset() {
            map.serialize_entry("successful", &self.successful)?;
        }
        if !self.failed.is_unset() {
            map.serialize_entry("failed", &self.failed)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SeverityStats {
    pub fn builder() -> SeverityStatsBuilder {
        SeverityStatsBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_c774d4b70528cba19fac20d0";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct SeverityStatsBuilder {
    severity: Option<String>,
    successful: Optional<FiniteNumber>,
    failed: Optional<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl SeverityStatsBuilder {
    pub fn severity(mut self, value: String) -> Self {
        self.severity = Some(value);
        self
    }
    pub fn successful(mut self, value: Optional<FiniteNumber>) -> Self {
        self.successful = value;
        self
    }
    pub fn failed(mut self, value: Optional<FiniteNumber>) -> Self {
        self.failed = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<SeverityStats> {
        Ok(SeverityStats {
            severity: self
                .severity
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            successful: self.successful,
            failed: self.failed,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<SeverityStats> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        SeverityStats::parse(value, mode)
    }
}
impl fmt::Debug for SeverityStatsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SeverityStatsBuilder([REDACTED])")
    }
}
impl fmt::Debug for SeverityStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SeverityStats([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_129e546491ee48eb7c96048d`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StandardProgress {
    #[serde(rename = "kind", default)]
    pub kind: Optional<StandardProgressKind>,
    #[serde(rename = "rows", default)]
    pub rows: Optional<Vec<ScanProgressBreakdown>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StandardProgress {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["kind", "rows"];
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
        if !self.kind.is_unset() {
            map.serialize_entry("kind", &self.kind)?;
        }
        if !self.rows.is_unset() {
            map.serialize_entry("rows", &self.rows)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StandardProgress {
    pub fn builder() -> StandardProgressBuilder {
        StandardProgressBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_129e546491ee48eb7c96048d";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StandardProgressBuilder {
    kind: Optional<StandardProgressKind>,
    rows: Optional<Vec<ScanProgressBreakdown>>,
    extensions: Extensions<JsonValue>,
}
impl StandardProgressBuilder {
    pub fn kind(mut self, value: Optional<StandardProgressKind>) -> Self {
        self.kind = value;
        self
    }
    pub fn rows(mut self, value: Optional<Vec<ScanProgressBreakdown>>) -> Self {
        self.rows = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StandardProgress> {
        Ok(StandardProgress {
            kind: self.kind,
            rows: self.rows,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<StandardProgress> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StandardProgress::parse(value, mode)
    }
}
impl fmt::Debug for StandardProgressBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StandardProgressBuilder([REDACTED])")
    }
}
impl fmt::Debug for StandardProgress {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StandardProgress([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_90d4ffedbfba6df8498af4cb`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StandardProgressKind {
    Standard,
}
impl StandardProgressKind {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Standard => "standard",
        }
    }
}
impl Serialize for StandardProgressKind {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for StandardProgressKind {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        match value.as_str() {
            "standard" => Ok(Self::Standard),
            _ => Err(serde::de::Error::custom("literal discriminator mismatch")),
        }
    }
}
impl fmt::Debug for StandardProgressKind {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StandardProgressKind([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_994ea8742a05abe3930b03c4`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StaticJobMetadata {
    #[serde(rename = "rate_limit_enabled", default)]
    pub rate_limit_enabled: Optional<bool>,
    #[serde(rename = "rate_limit", default)]
    pub rate_limit: Patch<FiniteNumber>,
    #[serde(rename = "rate_limit_error_code", default)]
    pub rate_limit_error_code: Patch<FiniteNumber>,
    #[serde(rename = "rate_limit_error_message", default)]
    pub rate_limit_error_message: Patch<String>,
    #[serde(rename = "rate_limit_error_json", default)]
    pub rate_limit_error_json: Patch<StaticJobMetadataRateLimitErrorJson>,
    #[serde(rename = "content_filter_enabled", default)]
    pub content_filter_enabled: Optional<bool>,
    #[serde(rename = "content_filter_error_code", default)]
    pub content_filter_error_code: Patch<FiniteNumber>,
    #[serde(rename = "content_filter_error_message", default)]
    pub content_filter_error_message: Patch<String>,
    #[serde(rename = "content_filter_error_json", default)]
    pub content_filter_error_json: Patch<StaticJobMetadataContentFilterErrorJson>,
    #[serde(rename = "language", default)]
    pub language: Patch<LanguageOption>,
    #[serde(rename = "runtime_profile_id", default)]
    pub runtime_profile_id: Patch<String>,
    #[serde(rename = "categories")]
    pub categories: TypedMap<Vec<StaticJobMetadataCategoriesValueItem>>,
    #[serde(rename = "include_file_attacks", default)]
    pub include_file_attacks: Optional<bool>,
    #[serde(rename = "include_audio_attacks", default)]
    pub include_audio_attacks: Optional<bool>,
    #[serde(rename = "include_text_attacks", default)]
    pub include_text_attacks: Optional<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StaticJobMetadata {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "rate_limit_enabled",
            "rate_limit",
            "rate_limit_error_code",
            "rate_limit_error_message",
            "rate_limit_error_json",
            "content_filter_enabled",
            "content_filter_error_code",
            "content_filter_error_message",
            "content_filter_error_json",
            "language",
            "runtime_profile_id",
            "categories",
            "include_file_attacks",
            "include_audio_attacks",
            "include_text_attacks",
        ];
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
        if !self.rate_limit_enabled.is_unset() {
            map.serialize_entry("rate_limit_enabled", &self.rate_limit_enabled)?;
        }
        if !self.rate_limit.is_unset() {
            map.serialize_entry("rate_limit", &self.rate_limit)?;
        }
        if !self.rate_limit_error_code.is_unset() {
            map.serialize_entry("rate_limit_error_code", &self.rate_limit_error_code)?;
        }
        if !self.rate_limit_error_message.is_unset() {
            map.serialize_entry("rate_limit_error_message", &self.rate_limit_error_message)?;
        }
        if !self.rate_limit_error_json.is_unset() {
            map.serialize_entry("rate_limit_error_json", &self.rate_limit_error_json)?;
        }
        if !self.content_filter_enabled.is_unset() {
            map.serialize_entry("content_filter_enabled", &self.content_filter_enabled)?;
        }
        if !self.content_filter_error_code.is_unset() {
            map.serialize_entry("content_filter_error_code", &self.content_filter_error_code)?;
        }
        if !self.content_filter_error_message.is_unset() {
            map.serialize_entry(
                "content_filter_error_message",
                &self.content_filter_error_message,
            )?;
        }
        if !self.content_filter_error_json.is_unset() {
            map.serialize_entry("content_filter_error_json", &self.content_filter_error_json)?;
        }
        if !self.language.is_unset() {
            map.serialize_entry("language", &self.language)?;
        }
        if !self.runtime_profile_id.is_unset() {
            map.serialize_entry("runtime_profile_id", &self.runtime_profile_id)?;
        }
        map.serialize_entry("categories", &self.categories)?;
        if !self.include_file_attacks.is_unset() {
            map.serialize_entry("include_file_attacks", &self.include_file_attacks)?;
        }
        if !self.include_audio_attacks.is_unset() {
            map.serialize_entry("include_audio_attacks", &self.include_audio_attacks)?;
        }
        if !self.include_text_attacks.is_unset() {
            map.serialize_entry("include_text_attacks", &self.include_text_attacks)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StaticJobMetadata {
    pub fn builder() -> StaticJobMetadataBuilder {
        StaticJobMetadataBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_994ea8742a05abe3930b03c4";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StaticJobMetadataBuilder {
    rate_limit_enabled: Optional<bool>,
    rate_limit: Patch<FiniteNumber>,
    rate_limit_error_code: Patch<FiniteNumber>,
    rate_limit_error_message: Patch<String>,
    rate_limit_error_json: Patch<StaticJobMetadataRateLimitErrorJson>,
    content_filter_enabled: Optional<bool>,
    content_filter_error_code: Patch<FiniteNumber>,
    content_filter_error_message: Patch<String>,
    content_filter_error_json: Patch<StaticJobMetadataContentFilterErrorJson>,
    language: Patch<LanguageOption>,
    runtime_profile_id: Patch<String>,
    categories: Option<TypedMap<Vec<StaticJobMetadataCategoriesValueItem>>>,
    include_file_attacks: Optional<bool>,
    include_audio_attacks: Optional<bool>,
    include_text_attacks: Optional<bool>,
    extensions: Extensions<JsonValue>,
}
impl StaticJobMetadataBuilder {
    pub fn rate_limit_enabled(mut self, value: Optional<bool>) -> Self {
        self.rate_limit_enabled = value;
        self
    }
    pub fn rate_limit(mut self, value: Patch<FiniteNumber>) -> Self {
        self.rate_limit = value;
        self
    }
    pub fn rate_limit_error_code(mut self, value: Patch<FiniteNumber>) -> Self {
        self.rate_limit_error_code = value;
        self
    }
    pub fn rate_limit_error_message(mut self, value: Patch<String>) -> Self {
        self.rate_limit_error_message = value;
        self
    }
    pub fn rate_limit_error_json(
        mut self,
        value: Patch<StaticJobMetadataRateLimitErrorJson>,
    ) -> Self {
        self.rate_limit_error_json = value;
        self
    }
    pub fn content_filter_enabled(mut self, value: Optional<bool>) -> Self {
        self.content_filter_enabled = value;
        self
    }
    pub fn content_filter_error_code(mut self, value: Patch<FiniteNumber>) -> Self {
        self.content_filter_error_code = value;
        self
    }
    pub fn content_filter_error_message(mut self, value: Patch<String>) -> Self {
        self.content_filter_error_message = value;
        self
    }
    pub fn content_filter_error_json(
        mut self,
        value: Patch<StaticJobMetadataContentFilterErrorJson>,
    ) -> Self {
        self.content_filter_error_json = value;
        self
    }
    pub fn language(mut self, value: Patch<LanguageOption>) -> Self {
        self.language = value;
        self
    }
    pub fn runtime_profile_id(mut self, value: Patch<String>) -> Self {
        self.runtime_profile_id = value;
        self
    }
    pub fn categories(
        mut self,
        value: TypedMap<Vec<StaticJobMetadataCategoriesValueItem>>,
    ) -> Self {
        self.categories = Some(value);
        self
    }
    pub fn include_file_attacks(mut self, value: Optional<bool>) -> Self {
        self.include_file_attacks = value;
        self
    }
    pub fn include_audio_attacks(mut self, value: Optional<bool>) -> Self {
        self.include_audio_attacks = value;
        self
    }
    pub fn include_text_attacks(mut self, value: Optional<bool>) -> Self {
        self.include_text_attacks = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StaticJobMetadata> {
        Ok(StaticJobMetadata {
            rate_limit_enabled: self.rate_limit_enabled,
            rate_limit: self.rate_limit,
            rate_limit_error_code: self.rate_limit_error_code,
            rate_limit_error_message: self.rate_limit_error_message,
            rate_limit_error_json: self.rate_limit_error_json,
            content_filter_enabled: self.content_filter_enabled,
            content_filter_error_code: self.content_filter_error_code,
            content_filter_error_message: self.content_filter_error_message,
            content_filter_error_json: self.content_filter_error_json,
            language: self.language,
            runtime_profile_id: self.runtime_profile_id,
            categories: self
                .categories
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            include_file_attacks: self.include_file_attacks,
            include_audio_attacks: self.include_audio_attacks,
            include_text_attacks: self.include_text_attacks,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StaticJobMetadata> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StaticJobMetadata::parse(value, mode)
    }
}
impl fmt::Debug for StaticJobMetadataBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobMetadataBuilder([REDACTED])")
    }
}
impl fmt::Debug for StaticJobMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobMetadata([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_673cbc31bd5675488238a778`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StaticJobMetadataCategoriesValueItem {
    ADVERSARIALSUFFIX,
    EVASION,
    INDIRECTPROMPTINJECTION,
    JAILBREAK,
    MULTITURN,
    PROMPTINJECTION,
    REMOTECODEEXECUTION,
    SYSTEMPROMPTLEAK,
    TOOLLEAK,
    MALWAREGENERATION,
    BIAS,
    CBRN,
    CYBERCRIME,
    DRUGS,
    HATETOXICABUSE,
    NONVIOLENTCRIMES,
    POLITICAL,
    SELFHARM,
    SEXUAL,
    VIOLENTCRIMESWEAPONS,
    COMPETITORENDORSEMENTS,
    BRANDTARNISHINGSELFCRITICISM,
    DISCRIMINATINGCLAIMS,
    POLITICALENDORSEMENTS,
    OWASP,
    MITREATLAS,
    NIST,
    DASFV2,
    Unknown(String),
}
impl StaticJobMetadataCategoriesValueItem {
    pub fn as_str(&self) -> &str {
        match self {
            Self::ADVERSARIALSUFFIX => "ADVERSARIAL_SUFFIX",
            Self::EVASION => "EVASION",
            Self::INDIRECTPROMPTINJECTION => "INDIRECT_PROMPT_INJECTION",
            Self::JAILBREAK => "JAILBREAK",
            Self::MULTITURN => "MULTI_TURN",
            Self::PROMPTINJECTION => "PROMPT_INJECTION",
            Self::REMOTECODEEXECUTION => "REMOTE_CODE_EXECUTION",
            Self::SYSTEMPROMPTLEAK => "SYSTEM_PROMPT_LEAK",
            Self::TOOLLEAK => "TOOL_LEAK",
            Self::MALWAREGENERATION => "MALWARE_GENERATION",
            Self::BIAS => "BIAS",
            Self::CBRN => "CBRN",
            Self::CYBERCRIME => "CYBERCRIME",
            Self::DRUGS => "DRUGS",
            Self::HATETOXICABUSE => "HATE_TOXIC_ABUSE",
            Self::NONVIOLENTCRIMES => "NON_VIOLENT_CRIMES",
            Self::POLITICAL => "POLITICAL",
            Self::SELFHARM => "SELF_HARM",
            Self::SEXUAL => "SEXUAL",
            Self::VIOLENTCRIMESWEAPONS => "VIOLENT_CRIMES_WEAPONS",
            Self::COMPETITORENDORSEMENTS => "COMPETITOR_ENDORSEMENTS",
            Self::BRANDTARNISHINGSELFCRITICISM => "BRAND_TARNISHING_SELF_CRITICISM",
            Self::DISCRIMINATINGCLAIMS => "DISCRIMINATING_CLAIMS",
            Self::POLITICALENDORSEMENTS => "POLITICAL_ENDORSEMENTS",
            Self::OWASP => "OWASP",
            Self::MITREATLAS => "MITRE_ATLAS",
            Self::NIST => "NIST",
            Self::DASFV2 => "DASF_V2",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for StaticJobMetadataCategoriesValueItem {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for StaticJobMetadataCategoriesValueItem {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "ADVERSARIAL_SUFFIX" => Self::ADVERSARIALSUFFIX,
            "EVASION" => Self::EVASION,
            "INDIRECT_PROMPT_INJECTION" => Self::INDIRECTPROMPTINJECTION,
            "JAILBREAK" => Self::JAILBREAK,
            "MULTI_TURN" => Self::MULTITURN,
            "PROMPT_INJECTION" => Self::PROMPTINJECTION,
            "REMOTE_CODE_EXECUTION" => Self::REMOTECODEEXECUTION,
            "SYSTEM_PROMPT_LEAK" => Self::SYSTEMPROMPTLEAK,
            "TOOL_LEAK" => Self::TOOLLEAK,
            "MALWARE_GENERATION" => Self::MALWAREGENERATION,
            "BIAS" => Self::BIAS,
            "CBRN" => Self::CBRN,
            "CYBERCRIME" => Self::CYBERCRIME,
            "DRUGS" => Self::DRUGS,
            "HATE_TOXIC_ABUSE" => Self::HATETOXICABUSE,
            "NON_VIOLENT_CRIMES" => Self::NONVIOLENTCRIMES,
            "POLITICAL" => Self::POLITICAL,
            "SELF_HARM" => Self::SELFHARM,
            "SEXUAL" => Self::SEXUAL,
            "VIOLENT_CRIMES_WEAPONS" => Self::VIOLENTCRIMESWEAPONS,
            "COMPETITOR_ENDORSEMENTS" => Self::COMPETITORENDORSEMENTS,
            "BRAND_TARNISHING_SELF_CRITICISM" => Self::BRANDTARNISHINGSELFCRITICISM,
            "DISCRIMINATING_CLAIMS" => Self::DISCRIMINATINGCLAIMS,
            "POLITICAL_ENDORSEMENTS" => Self::POLITICALENDORSEMENTS,
            "OWASP" => Self::OWASP,
            "MITRE_ATLAS" => Self::MITREATLAS,
            "NIST" => Self::NIST,
            "DASF_V2" => Self::DASFV2,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for StaticJobMetadataCategoriesValueItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobMetadataCategoriesValueItem([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_68adbb1c2fe73fda7ae11e40`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StaticJobMetadataContentFilterErrorJson {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StaticJobMetadataContentFilterErrorJson {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StaticJobMetadataContentFilterErrorJson {
    pub fn builder() -> StaticJobMetadataContentFilterErrorJsonBuilder {
        StaticJobMetadataContentFilterErrorJsonBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_68adbb1c2fe73fda7ae11e40";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StaticJobMetadataContentFilterErrorJsonBuilder {
    extensions: Extensions<JsonValue>,
}
impl StaticJobMetadataContentFilterErrorJsonBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StaticJobMetadataContentFilterErrorJson> {
        Ok(StaticJobMetadataContentFilterErrorJson {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StaticJobMetadataContentFilterErrorJson> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StaticJobMetadataContentFilterErrorJson::parse(value, mode)
    }
}
impl fmt::Debug for StaticJobMetadataContentFilterErrorJsonBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobMetadataContentFilterErrorJsonBuilder([REDACTED])")
    }
}
impl fmt::Debug for StaticJobMetadataContentFilterErrorJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobMetadataContentFilterErrorJson([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_6f4c024b4661524c17f4c733`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StaticJobMetadataRateLimitErrorJson {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StaticJobMetadataRateLimitErrorJson {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StaticJobMetadataRateLimitErrorJson {
    pub fn builder() -> StaticJobMetadataRateLimitErrorJsonBuilder {
        StaticJobMetadataRateLimitErrorJsonBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_6f4c024b4661524c17f4c733";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StaticJobMetadataRateLimitErrorJsonBuilder {
    extensions: Extensions<JsonValue>,
}
impl StaticJobMetadataRateLimitErrorJsonBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StaticJobMetadataRateLimitErrorJson> {
        Ok(StaticJobMetadataRateLimitErrorJson {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StaticJobMetadataRateLimitErrorJson> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StaticJobMetadataRateLimitErrorJson::parse(value, mode)
    }
}
impl fmt::Debug for StaticJobMetadataRateLimitErrorJsonBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobMetadataRateLimitErrorJsonBuilder([REDACTED])")
    }
}
impl fmt::Debug for StaticJobMetadataRateLimitErrorJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobMetadataRateLimitErrorJson([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_db1a79919e1178c1b403404d`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StaticJobRemediation {
    #[serde(rename = "mapping_remediation_id", default)]
    pub mapping_remediation_id: Patch<String>,
    #[serde(rename = "subcategories", default)]
    pub subcategories: Patch<Vec<String>>,
    #[serde(rename = "effectiveness", default)]
    pub effectiveness: Optional<FiniteNumber>,
    #[serde(rename = "ease_of_implementation", default)]
    pub ease_of_implementation: Optional<FiniteNumber>,
    #[serde(rename = "priority", default)]
    pub priority: Optional<FiniteNumber>,
    #[serde(rename = "remediation")]
    pub remediation: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "resource_links", default)]
    pub resource_links: Optional<Vec<String>>,
    #[serde(rename = "categories", default)]
    pub categories: Patch<Vec<String>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StaticJobRemediation {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "mapping_remediation_id",
            "subcategories",
            "effectiveness",
            "ease_of_implementation",
            "priority",
            "remediation",
            "description",
            "resource_links",
            "categories",
        ];
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
        if !self.mapping_remediation_id.is_unset() {
            map.serialize_entry("mapping_remediation_id", &self.mapping_remediation_id)?;
        }
        if !self.subcategories.is_unset() {
            map.serialize_entry("subcategories", &self.subcategories)?;
        }
        if !self.effectiveness.is_unset() {
            map.serialize_entry("effectiveness", &self.effectiveness)?;
        }
        if !self.ease_of_implementation.is_unset() {
            map.serialize_entry("ease_of_implementation", &self.ease_of_implementation)?;
        }
        if !self.priority.is_unset() {
            map.serialize_entry("priority", &self.priority)?;
        }
        map.serialize_entry("remediation", &self.remediation)?;
        map.serialize_entry("description", &self.description)?;
        if !self.resource_links.is_unset() {
            map.serialize_entry("resource_links", &self.resource_links)?;
        }
        if !self.categories.is_unset() {
            map.serialize_entry("categories", &self.categories)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StaticJobRemediation {
    pub fn builder() -> StaticJobRemediationBuilder {
        StaticJobRemediationBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_db1a79919e1178c1b403404d";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StaticJobRemediationBuilder {
    mapping_remediation_id: Patch<String>,
    subcategories: Patch<Vec<String>>,
    effectiveness: Optional<FiniteNumber>,
    ease_of_implementation: Optional<FiniteNumber>,
    priority: Optional<FiniteNumber>,
    remediation: Option<String>,
    description: Option<String>,
    resource_links: Optional<Vec<String>>,
    categories: Patch<Vec<String>>,
    extensions: Extensions<JsonValue>,
}
impl StaticJobRemediationBuilder {
    pub fn mapping_remediation_id(mut self, value: Patch<String>) -> Self {
        self.mapping_remediation_id = value;
        self
    }
    pub fn subcategories(mut self, value: Patch<Vec<String>>) -> Self {
        self.subcategories = value;
        self
    }
    pub fn effectiveness(mut self, value: Optional<FiniteNumber>) -> Self {
        self.effectiveness = value;
        self
    }
    pub fn ease_of_implementation(mut self, value: Optional<FiniteNumber>) -> Self {
        self.ease_of_implementation = value;
        self
    }
    pub fn priority(mut self, value: Optional<FiniteNumber>) -> Self {
        self.priority = value;
        self
    }
    pub fn remediation(mut self, value: String) -> Self {
        self.remediation = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn resource_links(mut self, value: Optional<Vec<String>>) -> Self {
        self.resource_links = value;
        self
    }
    pub fn categories(mut self, value: Patch<Vec<String>>) -> Self {
        self.categories = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StaticJobRemediation> {
        Ok(StaticJobRemediation {
            mapping_remediation_id: self.mapping_remediation_id,
            subcategories: self.subcategories,
            effectiveness: self.effectiveness,
            ease_of_implementation: self.ease_of_implementation,
            priority: self.priority,
            remediation: self
                .remediation
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            resource_links: self.resource_links,
            categories: self.categories,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StaticJobRemediation> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StaticJobRemediation::parse(value, mode)
    }
}
impl fmt::Debug for StaticJobRemediationBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobRemediationBuilder([REDACTED])")
    }
}
impl fmt::Debug for StaticJobRemediation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobRemediation([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_9b513c20c2a2bdc4004f9b2b`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StaticJobRemediationRecommendation {
    #[serde(rename = "runtime_security_policy_configuration", default)]
    pub runtime_security_policy_configuration: Patch<Vec<RuntimeSecurityPolicy>>,
    #[serde(rename = "other_measures", default)]
    pub other_measures: Optional<Vec<StaticJobRemediation>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StaticJobRemediationRecommendation {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["runtime_security_policy_configuration", "other_measures"];
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
        if !self.runtime_security_policy_configuration.is_unset() {
            map.serialize_entry(
                "runtime_security_policy_configuration",
                &self.runtime_security_policy_configuration,
            )?;
        }
        if !self.other_measures.is_unset() {
            map.serialize_entry("other_measures", &self.other_measures)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StaticJobRemediationRecommendation {
    pub fn builder() -> StaticJobRemediationRecommendationBuilder {
        StaticJobRemediationRecommendationBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_9b513c20c2a2bdc4004f9b2b";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StaticJobRemediationRecommendationBuilder {
    runtime_security_policy_configuration: Patch<Vec<RuntimeSecurityPolicy>>,
    other_measures: Optional<Vec<StaticJobRemediation>>,
    extensions: Extensions<JsonValue>,
}
impl StaticJobRemediationRecommendationBuilder {
    pub fn runtime_security_policy_configuration(
        mut self,
        value: Patch<Vec<RuntimeSecurityPolicy>>,
    ) -> Self {
        self.runtime_security_policy_configuration = value;
        self
    }
    pub fn other_measures(mut self, value: Optional<Vec<StaticJobRemediation>>) -> Self {
        self.other_measures = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StaticJobRemediationRecommendation> {
        Ok(StaticJobRemediationRecommendation {
            runtime_security_policy_configuration: self.runtime_security_policy_configuration,
            other_measures: self.other_measures,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StaticJobRemediationRecommendation> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StaticJobRemediationRecommendation::parse(value, mode)
    }
}
impl fmt::Debug for StaticJobRemediationRecommendationBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobRemediationRecommendationBuilder([REDACTED])")
    }
}
impl fmt::Debug for StaticJobRemediationRecommendation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobRemediationRecommendation([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_110fd0d872d98de3b6f812f3`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StaticJobReport {
    #[serde(rename = "asr", default)]
    pub asr: Patch<FiniteNumber>,
    #[serde(rename = "score", default)]
    pub score: Patch<FiniteNumber>,
    #[serde(rename = "security_report", default)]
    pub security_report: Patch<CategoryReport>,
    #[serde(rename = "safety_report", default)]
    pub safety_report: Patch<CategoryReport>,
    #[serde(rename = "brand_report", default)]
    pub brand_report: Patch<CategoryReport>,
    #[serde(rename = "compliance_report", default)]
    pub compliance_report: Patch<Vec<ComplianceReport>>,
    #[serde(rename = "severity_report")]
    pub severity_report: SeverityReport,
    #[serde(rename = "report_summary", default)]
    pub report_summary: Patch<String>,
    #[serde(rename = "recommendations", default)]
    pub recommendations: Patch<StaticJobRemediationRecommendation>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StaticJobReport {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "asr",
            "score",
            "security_report",
            "safety_report",
            "brand_report",
            "compliance_report",
            "severity_report",
            "report_summary",
            "recommendations",
        ];
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
        if !self.asr.is_unset() {
            map.serialize_entry("asr", &self.asr)?;
        }
        if !self.score.is_unset() {
            map.serialize_entry("score", &self.score)?;
        }
        if !self.security_report.is_unset() {
            map.serialize_entry("security_report", &self.security_report)?;
        }
        if !self.safety_report.is_unset() {
            map.serialize_entry("safety_report", &self.safety_report)?;
        }
        if !self.brand_report.is_unset() {
            map.serialize_entry("brand_report", &self.brand_report)?;
        }
        if !self.compliance_report.is_unset() {
            map.serialize_entry("compliance_report", &self.compliance_report)?;
        }
        map.serialize_entry("severity_report", &self.severity_report)?;
        if !self.report_summary.is_unset() {
            map.serialize_entry("report_summary", &self.report_summary)?;
        }
        if !self.recommendations.is_unset() {
            map.serialize_entry("recommendations", &self.recommendations)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StaticJobReport {
    pub fn builder() -> StaticJobReportBuilder {
        StaticJobReportBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_110fd0d872d98de3b6f812f3";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StaticJobReportBuilder {
    asr: Patch<FiniteNumber>,
    score: Patch<FiniteNumber>,
    security_report: Patch<CategoryReport>,
    safety_report: Patch<CategoryReport>,
    brand_report: Patch<CategoryReport>,
    compliance_report: Patch<Vec<ComplianceReport>>,
    severity_report: Option<SeverityReport>,
    report_summary: Patch<String>,
    recommendations: Patch<StaticJobRemediationRecommendation>,
    extensions: Extensions<JsonValue>,
}
impl StaticJobReportBuilder {
    pub fn asr(mut self, value: Patch<FiniteNumber>) -> Self {
        self.asr = value;
        self
    }
    pub fn score(mut self, value: Patch<FiniteNumber>) -> Self {
        self.score = value;
        self
    }
    pub fn security_report(mut self, value: Patch<CategoryReport>) -> Self {
        self.security_report = value;
        self
    }
    pub fn safety_report(mut self, value: Patch<CategoryReport>) -> Self {
        self.safety_report = value;
        self
    }
    pub fn brand_report(mut self, value: Patch<CategoryReport>) -> Self {
        self.brand_report = value;
        self
    }
    pub fn compliance_report(mut self, value: Patch<Vec<ComplianceReport>>) -> Self {
        self.compliance_report = value;
        self
    }
    pub fn severity_report(mut self, value: SeverityReport) -> Self {
        self.severity_report = Some(value);
        self
    }
    pub fn report_summary(mut self, value: Patch<String>) -> Self {
        self.report_summary = value;
        self
    }
    pub fn recommendations(mut self, value: Patch<StaticJobRemediationRecommendation>) -> Self {
        self.recommendations = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StaticJobReport> {
        Ok(StaticJobReport {
            asr: self.asr,
            score: self.score,
            security_report: self.security_report,
            safety_report: self.safety_report,
            brand_report: self.brand_report,
            compliance_report: self.compliance_report,
            severity_report: self
                .severity_report
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            report_summary: self.report_summary,
            recommendations: self.recommendations,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<StaticJobReport> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StaticJobReport::parse(value, mode)
    }
}
impl fmt::Debug for StaticJobReportBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobReportBuilder([REDACTED])")
    }
}
impl fmt::Debug for StaticJobReport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobReport([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_d3099306721ea98924b71191`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StaticJobReportStats {
    #[serde(rename = "output_completion_percentage")]
    pub output_completion_percentage: FiniteNumber,
    #[serde(rename = "partial_report_unlocked", default)]
    pub partial_report_unlocked: Optional<bool>,
    #[serde(rename = "partial_report_unlocked_at", default)]
    pub partial_report_unlocked_at: Patch<String>,
    #[serde(rename = "report_summary", default)]
    pub report_summary: Patch<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StaticJobReportStats {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "output_completion_percentage",
            "partial_report_unlocked",
            "partial_report_unlocked_at",
            "report_summary",
        ];
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
        map.serialize_entry(
            "output_completion_percentage",
            &self.output_completion_percentage,
        )?;
        if !self.partial_report_unlocked.is_unset() {
            map.serialize_entry("partial_report_unlocked", &self.partial_report_unlocked)?;
        }
        if !self.partial_report_unlocked_at.is_unset() {
            map.serialize_entry(
                "partial_report_unlocked_at",
                &self.partial_report_unlocked_at,
            )?;
        }
        if !self.report_summary.is_unset() {
            map.serialize_entry("report_summary", &self.report_summary)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StaticJobReportStats {
    pub fn builder() -> StaticJobReportStatsBuilder {
        StaticJobReportStatsBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_d3099306721ea98924b71191";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StaticJobReportStatsBuilder {
    output_completion_percentage: Option<FiniteNumber>,
    partial_report_unlocked: Optional<bool>,
    partial_report_unlocked_at: Patch<String>,
    report_summary: Patch<String>,
    extensions: Extensions<JsonValue>,
}
impl StaticJobReportStatsBuilder {
    pub fn output_completion_percentage(mut self, value: FiniteNumber) -> Self {
        self.output_completion_percentage = Some(value);
        self
    }
    pub fn partial_report_unlocked(mut self, value: Optional<bool>) -> Self {
        self.partial_report_unlocked = value;
        self
    }
    pub fn partial_report_unlocked_at(mut self, value: Patch<String>) -> Self {
        self.partial_report_unlocked_at = value;
        self
    }
    pub fn report_summary(mut self, value: Patch<String>) -> Self {
        self.report_summary = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StaticJobReportStats> {
        Ok(StaticJobReportStats {
            output_completion_percentage: self
                .output_completion_percentage
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            partial_report_unlocked: self.partial_report_unlocked,
            partial_report_unlocked_at: self.partial_report_unlocked_at,
            report_summary: self.report_summary,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StaticJobReportStats> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StaticJobReportStats::parse(value, mode)
    }
}
impl fmt::Debug for StaticJobReportStatsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobReportStatsBuilder([REDACTED])")
    }
}
impl fmt::Debug for StaticJobReportStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StaticJobReportStats([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_81933daf506d02ff992addce`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StatusQueryParam {
    SUCCESSFUL,
    FAILED,
    ERROR,
    Unknown(String),
}
impl StatusQueryParam {
    pub fn as_str(&self) -> &str {
        match self {
            Self::SUCCESSFUL => "SUCCESSFUL",
            Self::FAILED => "FAILED",
            Self::ERROR => "ERROR",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for StatusQueryParam {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for StatusQueryParam {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "SUCCESSFUL" => Self::SUCCESSFUL,
            "FAILED" => Self::FAILED,
            "ERROR" => Self::ERROR,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for StatusQueryParam {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StatusQueryParam([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_dd27749ec1b73be62647de58`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct Stream {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "target_id")]
    pub target_id: String,
    #[serde(rename = "stream_idx", default)]
    pub stream_idx: Optional<FiniteNumber>,
    #[serde(rename = "iteration", default)]
    pub iteration: Optional<FiniteNumber>,
    #[serde(rename = "goal_id")]
    pub goal_id: String,
    #[serde(rename = "goal", default)]
    pub goal: Patch<StreamGoal>,
    #[serde(rename = "marked_safe", default)]
    pub marked_safe: Optional<bool>,
    #[serde(rename = "stream_type", default)]
    pub stream_type: Patch<StreamStreamType>,
    #[serde(rename = "threat", default)]
    pub threat: Optional<bool>,
    #[serde(rename = "first_threat_iteration", default)]
    pub first_threat_iteration: Patch<StreamIterationData>,
    #[serde(rename = "created_at", default)]
    pub created_at: Patch<String>,
    #[serde(rename = "updated_at", default)]
    pub updated_at: Patch<String>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Optional<StreamExtraInfo>,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(rename = "error_message", default)]
    pub error_message: Patch<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for Stream {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "job_id",
            "target_id",
            "stream_idx",
            "iteration",
            "goal_id",
            "goal",
            "marked_safe",
            "stream_type",
            "threat",
            "first_threat_iteration",
            "created_at",
            "updated_at",
            "extra_info",
            "version",
            "error",
            "error_message",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("target_id", &self.target_id)?;
        if !self.stream_idx.is_unset() {
            map.serialize_entry("stream_idx", &self.stream_idx)?;
        }
        if !self.iteration.is_unset() {
            map.serialize_entry("iteration", &self.iteration)?;
        }
        map.serialize_entry("goal_id", &self.goal_id)?;
        if !self.goal.is_unset() {
            map.serialize_entry("goal", &self.goal)?;
        }
        if !self.marked_safe.is_unset() {
            map.serialize_entry("marked_safe", &self.marked_safe)?;
        }
        if !self.stream_type.is_unset() {
            map.serialize_entry("stream_type", &self.stream_type)?;
        }
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.first_threat_iteration.is_unset() {
            map.serialize_entry("first_threat_iteration", &self.first_threat_iteration)?;
        }
        if !self.created_at.is_unset() {
            map.serialize_entry("created_at", &self.created_at)?;
        }
        if !self.updated_at.is_unset() {
            map.serialize_entry("updated_at", &self.updated_at)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        if !self.error_message.is_unset() {
            map.serialize_entry("error_message", &self.error_message)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl Stream {
    pub fn builder() -> StreamBuilder {
        StreamBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_dd27749ec1b73be62647de58";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StreamBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    job_id: Option<String>,
    target_id: Option<String>,
    stream_idx: Optional<FiniteNumber>,
    iteration: Optional<FiniteNumber>,
    goal_id: Option<String>,
    goal: Patch<StreamGoal>,
    marked_safe: Optional<bool>,
    stream_type: Patch<StreamStreamType>,
    threat: Optional<bool>,
    first_threat_iteration: Patch<StreamIterationData>,
    created_at: Patch<String>,
    updated_at: Patch<String>,
    extra_info: Optional<StreamExtraInfo>,
    version: Patch<FiniteNumber>,
    error: Patch<bool>,
    error_message: Patch<String>,
    extensions: Extensions<JsonValue>,
}
impl StreamBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn target_id(mut self, value: String) -> Self {
        self.target_id = Some(value);
        self
    }
    pub fn stream_idx(mut self, value: Optional<FiniteNumber>) -> Self {
        self.stream_idx = value;
        self
    }
    pub fn iteration(mut self, value: Optional<FiniteNumber>) -> Self {
        self.iteration = value;
        self
    }
    pub fn goal_id(mut self, value: String) -> Self {
        self.goal_id = Some(value);
        self
    }
    pub fn goal(mut self, value: Patch<StreamGoal>) -> Self {
        self.goal = value;
        self
    }
    pub fn marked_safe(mut self, value: Optional<bool>) -> Self {
        self.marked_safe = value;
        self
    }
    pub fn stream_type(mut self, value: Patch<StreamStreamType>) -> Self {
        self.stream_type = value;
        self
    }
    pub fn threat(mut self, value: Optional<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn first_threat_iteration(mut self, value: Patch<StreamIterationData>) -> Self {
        self.first_threat_iteration = value;
        self
    }
    pub fn created_at(mut self, value: Patch<String>) -> Self {
        self.created_at = value;
        self
    }
    pub fn updated_at(mut self, value: Patch<String>) -> Self {
        self.updated_at = value;
        self
    }
    pub fn extra_info(mut self, value: Optional<StreamExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn error_message(mut self, value: Patch<String>) -> Self {
        self.error_message = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<Stream> {
        Ok(Stream {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target_id: self
                .target_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            stream_idx: self.stream_idx,
            iteration: self.iteration,
            goal_id: self
                .goal_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            goal: self.goal,
            marked_safe: self.marked_safe,
            stream_type: self.stream_type,
            threat: self.threat,
            first_threat_iteration: self.first_threat_iteration,
            created_at: self.created_at,
            updated_at: self.updated_at,
            extra_info: self.extra_info,
            version: self.version,
            error: self.error,
            error_message: self.error_message,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<Stream> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        Stream::parse(value, mode)
    }
}
impl fmt::Debug for StreamBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamBuilder([REDACTED])")
    }
}
impl fmt::Debug for Stream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Stream([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_aa0dff27a2f696d29cc6382f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StreamDetailResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "target_id")]
    pub target_id: String,
    #[serde(rename = "stream_idx", default)]
    pub stream_idx: Optional<FiniteNumber>,
    #[serde(rename = "iteration", default)]
    pub iteration: Optional<FiniteNumber>,
    #[serde(rename = "goal_id")]
    pub goal_id: String,
    #[serde(rename = "goal", default)]
    pub goal: Patch<String>,
    #[serde(rename = "marked_safe", default)]
    pub marked_safe: Optional<bool>,
    #[serde(rename = "stream_type", default)]
    pub stream_type: Patch<StreamDetailResponseStreamType>,
    #[serde(rename = "threat", default)]
    pub threat: Optional<bool>,
    #[serde(rename = "first_threat_iteration", default)]
    pub first_threat_iteration: Patch<StreamIterationData>,
    #[serde(rename = "created_at", default)]
    pub created_at: Patch<String>,
    #[serde(rename = "updated_at", default)]
    pub updated_at: Patch<String>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Optional<StreamDetailResponseExtraInfo>,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(rename = "error_message", default)]
    pub error_message: Patch<String>,
    #[serde(rename = "threat_details", default)]
    pub threat_details: Optional<ThreatDetails>,
    #[serde(rename = "iterations", default)]
    pub iterations: Optional<Vec<StreamIterationDataResponse>>,
    #[serde(rename = "goal_category", default)]
    pub goal_category: Patch<StreamDetailResponseGoalCategory>,
    #[serde(rename = "goal_metadata", default)]
    pub goal_metadata: Patch<StreamDetailResponseGoalMetadata>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StreamDetailResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "job_id",
            "target_id",
            "stream_idx",
            "iteration",
            "goal_id",
            "goal",
            "marked_safe",
            "stream_type",
            "threat",
            "first_threat_iteration",
            "created_at",
            "updated_at",
            "extra_info",
            "version",
            "error",
            "error_message",
            "threat_details",
            "iterations",
            "goal_category",
            "goal_metadata",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("target_id", &self.target_id)?;
        if !self.stream_idx.is_unset() {
            map.serialize_entry("stream_idx", &self.stream_idx)?;
        }
        if !self.iteration.is_unset() {
            map.serialize_entry("iteration", &self.iteration)?;
        }
        map.serialize_entry("goal_id", &self.goal_id)?;
        if !self.goal.is_unset() {
            map.serialize_entry("goal", &self.goal)?;
        }
        if !self.marked_safe.is_unset() {
            map.serialize_entry("marked_safe", &self.marked_safe)?;
        }
        if !self.stream_type.is_unset() {
            map.serialize_entry("stream_type", &self.stream_type)?;
        }
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.first_threat_iteration.is_unset() {
            map.serialize_entry("first_threat_iteration", &self.first_threat_iteration)?;
        }
        if !self.created_at.is_unset() {
            map.serialize_entry("created_at", &self.created_at)?;
        }
        if !self.updated_at.is_unset() {
            map.serialize_entry("updated_at", &self.updated_at)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        if !self.error_message.is_unset() {
            map.serialize_entry("error_message", &self.error_message)?;
        }
        if !self.threat_details.is_unset() {
            map.serialize_entry("threat_details", &self.threat_details)?;
        }
        if !self.iterations.is_unset() {
            map.serialize_entry("iterations", &self.iterations)?;
        }
        if !self.goal_category.is_unset() {
            map.serialize_entry("goal_category", &self.goal_category)?;
        }
        if !self.goal_metadata.is_unset() {
            map.serialize_entry("goal_metadata", &self.goal_metadata)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StreamDetailResponse {
    pub fn builder() -> StreamDetailResponseBuilder {
        StreamDetailResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_aa0dff27a2f696d29cc6382f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StreamDetailResponseBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    job_id: Option<String>,
    target_id: Option<String>,
    stream_idx: Optional<FiniteNumber>,
    iteration: Optional<FiniteNumber>,
    goal_id: Option<String>,
    goal: Patch<String>,
    marked_safe: Optional<bool>,
    stream_type: Patch<StreamDetailResponseStreamType>,
    threat: Optional<bool>,
    first_threat_iteration: Patch<StreamIterationData>,
    created_at: Patch<String>,
    updated_at: Patch<String>,
    extra_info: Optional<StreamDetailResponseExtraInfo>,
    version: Patch<FiniteNumber>,
    error: Patch<bool>,
    error_message: Patch<String>,
    threat_details: Optional<ThreatDetails>,
    iterations: Optional<Vec<StreamIterationDataResponse>>,
    goal_category: Patch<StreamDetailResponseGoalCategory>,
    goal_metadata: Patch<StreamDetailResponseGoalMetadata>,
    extensions: Extensions<JsonValue>,
}
impl StreamDetailResponseBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn target_id(mut self, value: String) -> Self {
        self.target_id = Some(value);
        self
    }
    pub fn stream_idx(mut self, value: Optional<FiniteNumber>) -> Self {
        self.stream_idx = value;
        self
    }
    pub fn iteration(mut self, value: Optional<FiniteNumber>) -> Self {
        self.iteration = value;
        self
    }
    pub fn goal_id(mut self, value: String) -> Self {
        self.goal_id = Some(value);
        self
    }
    pub fn goal(mut self, value: Patch<String>) -> Self {
        self.goal = value;
        self
    }
    pub fn marked_safe(mut self, value: Optional<bool>) -> Self {
        self.marked_safe = value;
        self
    }
    pub fn stream_type(mut self, value: Patch<StreamDetailResponseStreamType>) -> Self {
        self.stream_type = value;
        self
    }
    pub fn threat(mut self, value: Optional<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn first_threat_iteration(mut self, value: Patch<StreamIterationData>) -> Self {
        self.first_threat_iteration = value;
        self
    }
    pub fn created_at(mut self, value: Patch<String>) -> Self {
        self.created_at = value;
        self
    }
    pub fn updated_at(mut self, value: Patch<String>) -> Self {
        self.updated_at = value;
        self
    }
    pub fn extra_info(mut self, value: Optional<StreamDetailResponseExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn error_message(mut self, value: Patch<String>) -> Self {
        self.error_message = value;
        self
    }
    pub fn threat_details(mut self, value: Optional<ThreatDetails>) -> Self {
        self.threat_details = value;
        self
    }
    pub fn iterations(mut self, value: Optional<Vec<StreamIterationDataResponse>>) -> Self {
        self.iterations = value;
        self
    }
    pub fn goal_category(mut self, value: Patch<StreamDetailResponseGoalCategory>) -> Self {
        self.goal_category = value;
        self
    }
    pub fn goal_metadata(mut self, value: Patch<StreamDetailResponseGoalMetadata>) -> Self {
        self.goal_metadata = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StreamDetailResponse> {
        Ok(StreamDetailResponse {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            target_id: self
                .target_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            stream_idx: self.stream_idx,
            iteration: self.iteration,
            goal_id: self
                .goal_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            goal: self.goal,
            marked_safe: self.marked_safe,
            stream_type: self.stream_type,
            threat: self.threat,
            first_threat_iteration: self.first_threat_iteration,
            created_at: self.created_at,
            updated_at: self.updated_at,
            extra_info: self.extra_info,
            version: self.version,
            error: self.error,
            error_message: self.error_message,
            threat_details: self.threat_details,
            iterations: self.iterations,
            goal_category: self.goal_category,
            goal_metadata: self.goal_metadata,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StreamDetailResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StreamDetailResponse::parse(value, mode)
    }
}
impl fmt::Debug for StreamDetailResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamDetailResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for StreamDetailResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamDetailResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_8e49fff631b083a7688840b8`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StreamDetailResponseExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StreamDetailResponseExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StreamDetailResponseExtraInfo {
    pub fn builder() -> StreamDetailResponseExtraInfoBuilder {
        StreamDetailResponseExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_8e49fff631b083a7688840b8";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StreamDetailResponseExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl StreamDetailResponseExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StreamDetailResponseExtraInfo> {
        Ok(StreamDetailResponseExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StreamDetailResponseExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StreamDetailResponseExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for StreamDetailResponseExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamDetailResponseExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for StreamDetailResponseExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamDetailResponseExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_8e046dcf8599057ec4fbe5eb`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StreamDetailResponseGoalCategory {
    GOALMANIPULATION,
    PRIVILEGEMISUSE,
    TOOLMISUSE,
    TOOLCHAINING,
    TOXICCONTENTGENERATION,
    MEMORYPOISONING,
    CUSTOM,
    Unknown(String),
}
impl StreamDetailResponseGoalCategory {
    pub fn as_str(&self) -> &str {
        match self {
            Self::GOALMANIPULATION => "GOAL_MANIPULATION",
            Self::PRIVILEGEMISUSE => "PRIVILEGE_MISUSE",
            Self::TOOLMISUSE => "TOOL_MISUSE",
            Self::TOOLCHAINING => "TOOL_CHAINING",
            Self::TOXICCONTENTGENERATION => "TOXIC_CONTENT_GENERATION",
            Self::MEMORYPOISONING => "MEMORY_POISONING",
            Self::CUSTOM => "CUSTOM",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for StreamDetailResponseGoalCategory {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for StreamDetailResponseGoalCategory {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "GOAL_MANIPULATION" => Self::GOALMANIPULATION,
            "PRIVILEGE_MISUSE" => Self::PRIVILEGEMISUSE,
            "TOOL_MISUSE" => Self::TOOLMISUSE,
            "TOOL_CHAINING" => Self::TOOLCHAINING,
            "TOXIC_CONTENT_GENERATION" => Self::TOXICCONTENTGENERATION,
            "MEMORY_POISONING" => Self::MEMORYPOISONING,
            "CUSTOM" => Self::CUSTOM,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for StreamDetailResponseGoalCategory {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamDetailResponseGoalCategory([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_47d17648380bf2c61041064d`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StreamDetailResponseGoalMetadata {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StreamDetailResponseGoalMetadata {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StreamDetailResponseGoalMetadata {
    pub fn builder() -> StreamDetailResponseGoalMetadataBuilder {
        StreamDetailResponseGoalMetadataBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_47d17648380bf2c61041064d";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StreamDetailResponseGoalMetadataBuilder {
    extensions: Extensions<JsonValue>,
}
impl StreamDetailResponseGoalMetadataBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StreamDetailResponseGoalMetadata> {
        Ok(StreamDetailResponseGoalMetadata {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StreamDetailResponseGoalMetadata> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StreamDetailResponseGoalMetadata::parse(value, mode)
    }
}
impl fmt::Debug for StreamDetailResponseGoalMetadataBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamDetailResponseGoalMetadataBuilder([REDACTED])")
    }
}
impl fmt::Debug for StreamDetailResponseGoalMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamDetailResponseGoalMetadata([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_7fd9aa443c91e53282ae82c7`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StreamDetailResponseStreamType {
    NORMAL,
    ADVERSARIAL,
    Unknown(String),
}
impl StreamDetailResponseStreamType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NORMAL => "NORMAL",
            Self::ADVERSARIAL => "ADVERSARIAL",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for StreamDetailResponseStreamType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for StreamDetailResponseStreamType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "NORMAL" => Self::NORMAL,
            "ADVERSARIAL" => Self::ADVERSARIAL,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for StreamDetailResponseStreamType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamDetailResponseStreamType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_e4cf921688d948459d2a07ce`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StreamExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StreamExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StreamExtraInfo {
    pub fn builder() -> StreamExtraInfoBuilder {
        StreamExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_e4cf921688d948459d2a07ce";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StreamExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl StreamExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StreamExtraInfo> {
        Ok(StreamExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<StreamExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StreamExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for StreamExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for StreamExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_65203060caad080a15f0bb9e`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum StreamGoal {
    Variant1(Goal),
    Variant2(String),
    Variant3(()),
    Unknown(JsonValue),
}
impl fmt::Debug for StreamGoal {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamGoal([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_9de2de0499f4eb17a075895c`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StreamIterationData {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "stream_id")]
    pub stream_id: String,
    #[serde(rename = "goal_id")]
    pub goal_id: String,
    #[serde(rename = "iteration")]
    pub iteration: FiniteNumber,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "prompt_english", default)]
    pub prompt_english: Patch<String>,
    #[serde(rename = "techniques")]
    pub techniques: String,
    #[serde(rename = "improvement")]
    pub improvement: String,
    #[serde(rename = "prompts_objective")]
    pub prompts_objective: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "output", default)]
    pub output: Patch<String>,
    #[serde(rename = "output_english", default)]
    pub output_english: Patch<String>,
    #[serde(rename = "score", default)]
    pub score: Patch<FiniteNumber>,
    #[serde(rename = "judge_reasoning", default)]
    pub judge_reasoning: Patch<String>,
    #[serde(rename = "judge_explanation", default)]
    pub judge_explanation: Patch<String>,
    #[serde(rename = "threat", default)]
    pub threat: Optional<bool>,
    #[serde(rename = "created_at", default)]
    pub created_at: Patch<String>,
    #[serde(rename = "updated_at", default)]
    pub updated_at: Patch<String>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Patch<StreamIterationDataExtraInfo>,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(rename = "error_message", default)]
    pub error_message: Patch<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StreamIterationData {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "job_id",
            "stream_id",
            "goal_id",
            "iteration",
            "prompt",
            "prompt_english",
            "techniques",
            "improvement",
            "prompts_objective",
            "summary",
            "output",
            "output_english",
            "score",
            "judge_reasoning",
            "judge_explanation",
            "threat",
            "created_at",
            "updated_at",
            "extra_info",
            "version",
            "error",
            "error_message",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("stream_id", &self.stream_id)?;
        map.serialize_entry("goal_id", &self.goal_id)?;
        map.serialize_entry("iteration", &self.iteration)?;
        map.serialize_entry("prompt", &self.prompt)?;
        if !self.prompt_english.is_unset() {
            map.serialize_entry("prompt_english", &self.prompt_english)?;
        }
        map.serialize_entry("techniques", &self.techniques)?;
        map.serialize_entry("improvement", &self.improvement)?;
        map.serialize_entry("prompts_objective", &self.prompts_objective)?;
        map.serialize_entry("summary", &self.summary)?;
        if !self.output.is_unset() {
            map.serialize_entry("output", &self.output)?;
        }
        if !self.output_english.is_unset() {
            map.serialize_entry("output_english", &self.output_english)?;
        }
        if !self.score.is_unset() {
            map.serialize_entry("score", &self.score)?;
        }
        if !self.judge_reasoning.is_unset() {
            map.serialize_entry("judge_reasoning", &self.judge_reasoning)?;
        }
        if !self.judge_explanation.is_unset() {
            map.serialize_entry("judge_explanation", &self.judge_explanation)?;
        }
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.created_at.is_unset() {
            map.serialize_entry("created_at", &self.created_at)?;
        }
        if !self.updated_at.is_unset() {
            map.serialize_entry("updated_at", &self.updated_at)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        if !self.error_message.is_unset() {
            map.serialize_entry("error_message", &self.error_message)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StreamIterationData {
    pub fn builder() -> StreamIterationDataBuilder {
        StreamIterationDataBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_9de2de0499f4eb17a075895c";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StreamIterationDataBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    job_id: Option<String>,
    stream_id: Option<String>,
    goal_id: Option<String>,
    iteration: Option<FiniteNumber>,
    prompt: Option<String>,
    prompt_english: Patch<String>,
    techniques: Option<String>,
    improvement: Option<String>,
    prompts_objective: Option<String>,
    summary: Option<String>,
    output: Patch<String>,
    output_english: Patch<String>,
    score: Patch<FiniteNumber>,
    judge_reasoning: Patch<String>,
    judge_explanation: Patch<String>,
    threat: Optional<bool>,
    created_at: Patch<String>,
    updated_at: Patch<String>,
    extra_info: Patch<StreamIterationDataExtraInfo>,
    version: Patch<FiniteNumber>,
    error: Patch<bool>,
    error_message: Patch<String>,
    extensions: Extensions<JsonValue>,
}
impl StreamIterationDataBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn stream_id(mut self, value: String) -> Self {
        self.stream_id = Some(value);
        self
    }
    pub fn goal_id(mut self, value: String) -> Self {
        self.goal_id = Some(value);
        self
    }
    pub fn iteration(mut self, value: FiniteNumber) -> Self {
        self.iteration = Some(value);
        self
    }
    pub fn prompt(mut self, value: String) -> Self {
        self.prompt = Some(value);
        self
    }
    pub fn prompt_english(mut self, value: Patch<String>) -> Self {
        self.prompt_english = value;
        self
    }
    pub fn techniques(mut self, value: String) -> Self {
        self.techniques = Some(value);
        self
    }
    pub fn improvement(mut self, value: String) -> Self {
        self.improvement = Some(value);
        self
    }
    pub fn prompts_objective(mut self, value: String) -> Self {
        self.prompts_objective = Some(value);
        self
    }
    pub fn summary(mut self, value: String) -> Self {
        self.summary = Some(value);
        self
    }
    pub fn output(mut self, value: Patch<String>) -> Self {
        self.output = value;
        self
    }
    pub fn output_english(mut self, value: Patch<String>) -> Self {
        self.output_english = value;
        self
    }
    pub fn score(mut self, value: Patch<FiniteNumber>) -> Self {
        self.score = value;
        self
    }
    pub fn judge_reasoning(mut self, value: Patch<String>) -> Self {
        self.judge_reasoning = value;
        self
    }
    pub fn judge_explanation(mut self, value: Patch<String>) -> Self {
        self.judge_explanation = value;
        self
    }
    pub fn threat(mut self, value: Optional<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn created_at(mut self, value: Patch<String>) -> Self {
        self.created_at = value;
        self
    }
    pub fn updated_at(mut self, value: Patch<String>) -> Self {
        self.updated_at = value;
        self
    }
    pub fn extra_info(mut self, value: Patch<StreamIterationDataExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn error_message(mut self, value: Patch<String>) -> Self {
        self.error_message = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StreamIterationData> {
        Ok(StreamIterationData {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            stream_id: self
                .stream_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            goal_id: self
                .goal_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            iteration: self
                .iteration
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt: self
                .prompt
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt_english: self.prompt_english,
            techniques: self
                .techniques
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            improvement: self
                .improvement
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompts_objective: self
                .prompts_objective
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            summary: self
                .summary
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            output: self.output,
            output_english: self.output_english,
            score: self.score,
            judge_reasoning: self.judge_reasoning,
            judge_explanation: self.judge_explanation,
            threat: self.threat,
            created_at: self.created_at,
            updated_at: self.updated_at,
            extra_info: self.extra_info,
            version: self.version,
            error: self.error,
            error_message: self.error_message,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StreamIterationData> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StreamIterationData::parse(value, mode)
    }
}
impl fmt::Debug for StreamIterationDataBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamIterationDataBuilder([REDACTED])")
    }
}
impl fmt::Debug for StreamIterationData {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamIterationData([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_c4bb042a0a3d4babec569901`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StreamIterationDataExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StreamIterationDataExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StreamIterationDataExtraInfo {
    pub fn builder() -> StreamIterationDataExtraInfoBuilder {
        StreamIterationDataExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_c4bb042a0a3d4babec569901";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StreamIterationDataExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl StreamIterationDataExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StreamIterationDataExtraInfo> {
        Ok(StreamIterationDataExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StreamIterationDataExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StreamIterationDataExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for StreamIterationDataExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamIterationDataExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for StreamIterationDataExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamIterationDataExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_2a58f1ff1afeb284cfc59b5f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StreamIterationDataResponse {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "job_id")]
    pub job_id: String,
    #[serde(rename = "stream_id")]
    pub stream_id: String,
    #[serde(rename = "goal_id")]
    pub goal_id: String,
    #[serde(rename = "iteration")]
    pub iteration: FiniteNumber,
    #[serde(rename = "prompt")]
    pub prompt: String,
    #[serde(rename = "techniques")]
    pub techniques: String,
    #[serde(rename = "improvement")]
    pub improvement: String,
    #[serde(rename = "prompts_objective")]
    pub prompts_objective: String,
    #[serde(rename = "summary")]
    pub summary: String,
    #[serde(rename = "output", default)]
    pub output: Patch<String>,
    #[serde(rename = "score", default)]
    pub score: Patch<FiniteNumber>,
    #[serde(rename = "judge_reasoning", default)]
    pub judge_reasoning: Patch<String>,
    #[serde(rename = "threat", default)]
    pub threat: Optional<bool>,
    #[serde(rename = "created_at", default)]
    pub created_at: Patch<String>,
    #[serde(rename = "updated_at", default)]
    pub updated_at: Patch<String>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Patch<StreamIterationDataResponseExtraInfo>,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "error", default)]
    pub error: Patch<bool>,
    #[serde(rename = "error_message", default)]
    pub error_message: Patch<String>,
    #[serde(rename = "threat_details", default)]
    pub threat_details: Optional<ThreatDetails>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StreamIterationDataResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "job_id",
            "stream_id",
            "goal_id",
            "iteration",
            "prompt",
            "techniques",
            "improvement",
            "prompts_objective",
            "summary",
            "output",
            "score",
            "judge_reasoning",
            "threat",
            "created_at",
            "updated_at",
            "extra_info",
            "version",
            "error",
            "error_message",
            "threat_details",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("job_id", &self.job_id)?;
        map.serialize_entry("stream_id", &self.stream_id)?;
        map.serialize_entry("goal_id", &self.goal_id)?;
        map.serialize_entry("iteration", &self.iteration)?;
        map.serialize_entry("prompt", &self.prompt)?;
        map.serialize_entry("techniques", &self.techniques)?;
        map.serialize_entry("improvement", &self.improvement)?;
        map.serialize_entry("prompts_objective", &self.prompts_objective)?;
        map.serialize_entry("summary", &self.summary)?;
        if !self.output.is_unset() {
            map.serialize_entry("output", &self.output)?;
        }
        if !self.score.is_unset() {
            map.serialize_entry("score", &self.score)?;
        }
        if !self.judge_reasoning.is_unset() {
            map.serialize_entry("judge_reasoning", &self.judge_reasoning)?;
        }
        if !self.threat.is_unset() {
            map.serialize_entry("threat", &self.threat)?;
        }
        if !self.created_at.is_unset() {
            map.serialize_entry("created_at", &self.created_at)?;
        }
        if !self.updated_at.is_unset() {
            map.serialize_entry("updated_at", &self.updated_at)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.error.is_unset() {
            map.serialize_entry("error", &self.error)?;
        }
        if !self.error_message.is_unset() {
            map.serialize_entry("error_message", &self.error_message)?;
        }
        if !self.threat_details.is_unset() {
            map.serialize_entry("threat_details", &self.threat_details)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StreamIterationDataResponse {
    pub fn builder() -> StreamIterationDataResponseBuilder {
        StreamIterationDataResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_2a58f1ff1afeb284cfc59b5f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StreamIterationDataResponseBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    job_id: Option<String>,
    stream_id: Option<String>,
    goal_id: Option<String>,
    iteration: Option<FiniteNumber>,
    prompt: Option<String>,
    techniques: Option<String>,
    improvement: Option<String>,
    prompts_objective: Option<String>,
    summary: Option<String>,
    output: Patch<String>,
    score: Patch<FiniteNumber>,
    judge_reasoning: Patch<String>,
    threat: Optional<bool>,
    created_at: Patch<String>,
    updated_at: Patch<String>,
    extra_info: Patch<StreamIterationDataResponseExtraInfo>,
    version: Patch<FiniteNumber>,
    error: Patch<bool>,
    error_message: Patch<String>,
    threat_details: Optional<ThreatDetails>,
    extensions: Extensions<JsonValue>,
}
impl StreamIterationDataResponseBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn job_id(mut self, value: String) -> Self {
        self.job_id = Some(value);
        self
    }
    pub fn stream_id(mut self, value: String) -> Self {
        self.stream_id = Some(value);
        self
    }
    pub fn goal_id(mut self, value: String) -> Self {
        self.goal_id = Some(value);
        self
    }
    pub fn iteration(mut self, value: FiniteNumber) -> Self {
        self.iteration = Some(value);
        self
    }
    pub fn prompt(mut self, value: String) -> Self {
        self.prompt = Some(value);
        self
    }
    pub fn techniques(mut self, value: String) -> Self {
        self.techniques = Some(value);
        self
    }
    pub fn improvement(mut self, value: String) -> Self {
        self.improvement = Some(value);
        self
    }
    pub fn prompts_objective(mut self, value: String) -> Self {
        self.prompts_objective = Some(value);
        self
    }
    pub fn summary(mut self, value: String) -> Self {
        self.summary = Some(value);
        self
    }
    pub fn output(mut self, value: Patch<String>) -> Self {
        self.output = value;
        self
    }
    pub fn score(mut self, value: Patch<FiniteNumber>) -> Self {
        self.score = value;
        self
    }
    pub fn judge_reasoning(mut self, value: Patch<String>) -> Self {
        self.judge_reasoning = value;
        self
    }
    pub fn threat(mut self, value: Optional<bool>) -> Self {
        self.threat = value;
        self
    }
    pub fn created_at(mut self, value: Patch<String>) -> Self {
        self.created_at = value;
        self
    }
    pub fn updated_at(mut self, value: Patch<String>) -> Self {
        self.updated_at = value;
        self
    }
    pub fn extra_info(mut self, value: Patch<StreamIterationDataResponseExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn error(mut self, value: Patch<bool>) -> Self {
        self.error = value;
        self
    }
    pub fn error_message(mut self, value: Patch<String>) -> Self {
        self.error_message = value;
        self
    }
    pub fn threat_details(mut self, value: Optional<ThreatDetails>) -> Self {
        self.threat_details = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StreamIterationDataResponse> {
        Ok(StreamIterationDataResponse {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            job_id: self
                .job_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            stream_id: self
                .stream_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            goal_id: self
                .goal_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            iteration: self
                .iteration
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompt: self
                .prompt
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            techniques: self
                .techniques
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            improvement: self
                .improvement
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            prompts_objective: self
                .prompts_objective
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            summary: self
                .summary
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            output: self.output,
            score: self.score,
            judge_reasoning: self.judge_reasoning,
            threat: self.threat,
            created_at: self.created_at,
            updated_at: self.updated_at,
            extra_info: self.extra_info,
            version: self.version,
            error: self.error,
            error_message: self.error_message,
            threat_details: self.threat_details,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StreamIterationDataResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StreamIterationDataResponse::parse(value, mode)
    }
}
impl fmt::Debug for StreamIterationDataResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamIterationDataResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for StreamIterationDataResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamIterationDataResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_2479267cb046605b08027e9a`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StreamIterationDataResponseExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StreamIterationDataResponseExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StreamIterationDataResponseExtraInfo {
    pub fn builder() -> StreamIterationDataResponseExtraInfoBuilder {
        StreamIterationDataResponseExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_2479267cb046605b08027e9a";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StreamIterationDataResponseExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl StreamIterationDataResponseExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StreamIterationDataResponseExtraInfo> {
        Ok(StreamIterationDataResponseExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StreamIterationDataResponseExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StreamIterationDataResponseExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for StreamIterationDataResponseExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamIterationDataResponseExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for StreamIterationDataResponseExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamIterationDataResponseExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_e87be8d95ea1855f170badf2`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct StreamListResponse {
    #[serde(rename = "pagination")]
    pub pagination: Pagination,
    #[serde(rename = "data")]
    pub data: Vec<Stream>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for StreamListResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["pagination", "data"];
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
        map.serialize_entry("pagination", &self.pagination)?;
        map.serialize_entry("data", &self.data)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl StreamListResponse {
    pub fn builder() -> StreamListResponseBuilder {
        StreamListResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_e87be8d95ea1855f170badf2";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct StreamListResponseBuilder {
    pagination: Option<Pagination>,
    data: Option<Vec<Stream>>,
    extensions: Extensions<JsonValue>,
}
impl StreamListResponseBuilder {
    pub fn pagination(mut self, value: Pagination) -> Self {
        self.pagination = Some(value);
        self
    }
    pub fn data(mut self, value: Vec<Stream>) -> Self {
        self.data = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<StreamListResponse> {
        Ok(StreamListResponse {
            pagination: self
                .pagination
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            data: self
                .data
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<StreamListResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        StreamListResponse::parse(value, mode)
    }
}
impl fmt::Debug for StreamListResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamListResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for StreamListResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamListResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_fe3b08aca6b0c448a946399f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StreamStreamType {
    NORMAL,
    ADVERSARIAL,
    Unknown(String),
}
impl StreamStreamType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NORMAL => "NORMAL",
            Self::ADVERSARIAL => "ADVERSARIAL",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for StreamStreamType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for StreamStreamType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "NORMAL" => Self::NORMAL,
            "ADVERSARIAL" => Self::ADVERSARIAL,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for StreamStreamType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamStreamType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_f5d52c73759ca3c08550a081`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum StreamType {
    NORMAL,
    ADVERSARIAL,
    Unknown(String),
}
impl StreamType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NORMAL => "NORMAL",
            Self::ADVERSARIAL => "ADVERSARIAL",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for StreamType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for StreamType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "NORMAL" => Self::NORMAL,
            "ADVERSARIAL" => Self::ADVERSARIAL,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for StreamType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("StreamType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_debeb7391c922909bc3dda62`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct SubCategoryModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "preselect", default)]
    pub preselect: Optional<bool>,
    #[serde(rename = "prerequisites", default)]
    pub prerequisites: Patch<Vec<PrerequisiteModel>>,
    #[serde(rename = "active", default)]
    pub active: Optional<bool>,
    #[serde(rename = "file_supported", default)]
    pub file_supported: Optional<bool>,
    #[serde(rename = "audio_supported", default)]
    pub audio_supported: Optional<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for SubCategoryModel {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "id",
            "display_name",
            "description",
            "preselect",
            "prerequisites",
            "active",
            "file_supported",
            "audio_supported",
        ];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("description", &self.description)?;
        if !self.preselect.is_unset() {
            map.serialize_entry("preselect", &self.preselect)?;
        }
        if !self.prerequisites.is_unset() {
            map.serialize_entry("prerequisites", &self.prerequisites)?;
        }
        if !self.active.is_unset() {
            map.serialize_entry("active", &self.active)?;
        }
        if !self.file_supported.is_unset() {
            map.serialize_entry("file_supported", &self.file_supported)?;
        }
        if !self.audio_supported.is_unset() {
            map.serialize_entry("audio_supported", &self.audio_supported)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SubCategoryModel {
    pub fn builder() -> SubCategoryModelBuilder {
        SubCategoryModelBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_debeb7391c922909bc3dda62";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct SubCategoryModelBuilder {
    id: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    preselect: Optional<bool>,
    prerequisites: Patch<Vec<PrerequisiteModel>>,
    active: Optional<bool>,
    file_supported: Optional<bool>,
    audio_supported: Optional<bool>,
    extensions: Extensions<JsonValue>,
}
impl SubCategoryModelBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn preselect(mut self, value: Optional<bool>) -> Self {
        self.preselect = value;
        self
    }
    pub fn prerequisites(mut self, value: Patch<Vec<PrerequisiteModel>>) -> Self {
        self.prerequisites = value;
        self
    }
    pub fn active(mut self, value: Optional<bool>) -> Self {
        self.active = value;
        self
    }
    pub fn file_supported(mut self, value: Optional<bool>) -> Self {
        self.file_supported = value;
        self
    }
    pub fn audio_supported(mut self, value: Optional<bool>) -> Self {
        self.audio_supported = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<SubCategoryModel> {
        Ok(SubCategoryModel {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            preselect: self.preselect,
            prerequisites: self.prerequisites,
            active: self.active,
            file_supported: self.file_supported,
            audio_supported: self.audio_supported,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<SubCategoryModel> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        SubCategoryModel::parse(value, mode)
    }
}
impl fmt::Debug for SubCategoryModelBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SubCategoryModelBuilder([REDACTED])")
    }
}
impl fmt::Debug for SubCategoryModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SubCategoryModel([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_3a3473b9aadd3d9552b8538d`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct SubCategoryStats {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "preselect", default)]
    pub preselect: Optional<bool>,
    #[serde(rename = "prerequisites", default)]
    pub prerequisites: Patch<Vec<PrerequisiteModel>>,
    #[serde(rename = "active", default)]
    pub active: Optional<bool>,
    #[serde(rename = "file_supported", default)]
    pub file_supported: Optional<bool>,
    #[serde(rename = "audio_supported", default)]
    pub audio_supported: Optional<bool>,
    #[serde(rename = "successful")]
    pub successful: FiniteNumber,
    #[serde(rename = "failed")]
    pub failed: FiniteNumber,
    #[serde(rename = "total", default)]
    pub total: Optional<FiniteNumber>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for SubCategoryStats {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "id",
            "display_name",
            "description",
            "preselect",
            "prerequisites",
            "active",
            "file_supported",
            "audio_supported",
            "successful",
            "failed",
            "total",
        ];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("description", &self.description)?;
        if !self.preselect.is_unset() {
            map.serialize_entry("preselect", &self.preselect)?;
        }
        if !self.prerequisites.is_unset() {
            map.serialize_entry("prerequisites", &self.prerequisites)?;
        }
        if !self.active.is_unset() {
            map.serialize_entry("active", &self.active)?;
        }
        if !self.file_supported.is_unset() {
            map.serialize_entry("file_supported", &self.file_supported)?;
        }
        if !self.audio_supported.is_unset() {
            map.serialize_entry("audio_supported", &self.audio_supported)?;
        }
        map.serialize_entry("successful", &self.successful)?;
        map.serialize_entry("failed", &self.failed)?;
        if !self.total.is_unset() {
            map.serialize_entry("total", &self.total)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl SubCategoryStats {
    pub fn builder() -> SubCategoryStatsBuilder {
        SubCategoryStatsBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_3a3473b9aadd3d9552b8538d";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct SubCategoryStatsBuilder {
    id: Option<String>,
    display_name: Option<String>,
    description: Option<String>,
    preselect: Optional<bool>,
    prerequisites: Patch<Vec<PrerequisiteModel>>,
    active: Optional<bool>,
    file_supported: Optional<bool>,
    audio_supported: Optional<bool>,
    successful: Option<FiniteNumber>,
    failed: Option<FiniteNumber>,
    total: Optional<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl SubCategoryStatsBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn preselect(mut self, value: Optional<bool>) -> Self {
        self.preselect = value;
        self
    }
    pub fn prerequisites(mut self, value: Patch<Vec<PrerequisiteModel>>) -> Self {
        self.prerequisites = value;
        self
    }
    pub fn active(mut self, value: Optional<bool>) -> Self {
        self.active = value;
        self
    }
    pub fn file_supported(mut self, value: Optional<bool>) -> Self {
        self.file_supported = value;
        self
    }
    pub fn audio_supported(mut self, value: Optional<bool>) -> Self {
        self.audio_supported = value;
        self
    }
    pub fn successful(mut self, value: FiniteNumber) -> Self {
        self.successful = Some(value);
        self
    }
    pub fn failed(mut self, value: FiniteNumber) -> Self {
        self.failed = Some(value);
        self
    }
    pub fn total(mut self, value: Optional<FiniteNumber>) -> Self {
        self.total = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<SubCategoryStats> {
        Ok(SubCategoryStats {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            preselect: self.preselect,
            prerequisites: self.prerequisites,
            active: self.active,
            file_supported: self.file_supported,
            audio_supported: self.audio_supported,
            successful: self
                .successful
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            failed: self
                .failed
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            total: self.total,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<SubCategoryStats> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        SubCategoryStats::parse(value, mode)
    }
}
impl fmt::Debug for SubCategoryStatsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SubCategoryStatsBuilder([REDACTED])")
    }
}
impl fmt::Debug for SubCategoryStats {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("SubCategoryStats([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_bbfd54e59c9456ecec2ca951`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TargetAdditionalContext {
    #[serde(rename = "base_model", default)]
    pub base_model: Patch<String>,
    #[serde(rename = "core_architecture", default)]
    pub core_architecture: Patch<String>,
    #[serde(rename = "system_prompt", default)]
    pub system_prompt: Patch<String>,
    #[serde(rename = "languages_supported", default)]
    pub languages_supported: Patch<Vec<String>>,
    #[serde(rename = "banned_keywords", default)]
    pub banned_keywords: Patch<Vec<String>>,
    #[serde(rename = "tools_accessible", default)]
    pub tools_accessible: Patch<Vec<String>>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for TargetAdditionalContext {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "base_model",
            "core_architecture",
            "system_prompt",
            "languages_supported",
            "banned_keywords",
            "tools_accessible",
        ];
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
        if !self.base_model.is_unset() {
            map.serialize_entry("base_model", &self.base_model)?;
        }
        if !self.core_architecture.is_unset() {
            map.serialize_entry("core_architecture", &self.core_architecture)?;
        }
        if !self.system_prompt.is_unset() {
            map.serialize_entry("system_prompt", &self.system_prompt)?;
        }
        if !self.languages_supported.is_unset() {
            map.serialize_entry("languages_supported", &self.languages_supported)?;
        }
        if !self.banned_keywords.is_unset() {
            map.serialize_entry("banned_keywords", &self.banned_keywords)?;
        }
        if !self.tools_accessible.is_unset() {
            map.serialize_entry("tools_accessible", &self.tools_accessible)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TargetAdditionalContext {
    pub fn builder() -> TargetAdditionalContextBuilder {
        TargetAdditionalContextBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_bbfd54e59c9456ecec2ca951";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct TargetAdditionalContextBuilder {
    base_model: Patch<String>,
    core_architecture: Patch<String>,
    system_prompt: Patch<String>,
    languages_supported: Patch<Vec<String>>,
    banned_keywords: Patch<Vec<String>>,
    tools_accessible: Patch<Vec<String>>,
    extensions: Extensions<JsonValue>,
}
impl TargetAdditionalContextBuilder {
    pub fn base_model(mut self, value: Patch<String>) -> Self {
        self.base_model = value;
        self
    }
    pub fn core_architecture(mut self, value: Patch<String>) -> Self {
        self.core_architecture = value;
        self
    }
    pub fn system_prompt(mut self, value: Patch<String>) -> Self {
        self.system_prompt = value;
        self
    }
    pub fn languages_supported(mut self, value: Patch<Vec<String>>) -> Self {
        self.languages_supported = value;
        self
    }
    pub fn banned_keywords(mut self, value: Patch<Vec<String>>) -> Self {
        self.banned_keywords = value;
        self
    }
    pub fn tools_accessible(mut self, value: Patch<Vec<String>>) -> Self {
        self.tools_accessible = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<TargetAdditionalContext> {
        Ok(TargetAdditionalContext {
            base_model: self.base_model,
            core_architecture: self.core_architecture,
            system_prompt: self.system_prompt,
            languages_supported: self.languages_supported,
            banned_keywords: self.banned_keywords,
            tools_accessible: self.tools_accessible,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<TargetAdditionalContext> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        TargetAdditionalContext::parse(value, mode)
    }
}
impl fmt::Debug for TargetAdditionalContextBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetAdditionalContextBuilder([REDACTED])")
    }
}
impl fmt::Debug for TargetAdditionalContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetAdditionalContext([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_64241205e20c0f4df28a4e1f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TargetBackground {
    #[serde(rename = "industry", default)]
    pub industry: Patch<String>,
    #[serde(rename = "use_case", default)]
    pub use_case: Patch<String>,
    #[serde(rename = "competitors", default)]
    pub competitors: Patch<Vec<String>>,
    #[serde(rename = "agentic_profiling_enabled", default)]
    pub agentic_profiling_enabled: Optional<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for TargetBackground {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "industry",
            "use_case",
            "competitors",
            "agentic_profiling_enabled",
        ];
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
        if !self.industry.is_unset() {
            map.serialize_entry("industry", &self.industry)?;
        }
        if !self.use_case.is_unset() {
            map.serialize_entry("use_case", &self.use_case)?;
        }
        if !self.competitors.is_unset() {
            map.serialize_entry("competitors", &self.competitors)?;
        }
        if !self.agentic_profiling_enabled.is_unset() {
            map.serialize_entry("agentic_profiling_enabled", &self.agentic_profiling_enabled)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TargetBackground {
    pub fn builder() -> TargetBackgroundBuilder {
        TargetBackgroundBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_64241205e20c0f4df28a4e1f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct TargetBackgroundBuilder {
    industry: Patch<String>,
    use_case: Patch<String>,
    competitors: Patch<Vec<String>>,
    agentic_profiling_enabled: Optional<bool>,
    extensions: Extensions<JsonValue>,
}
impl TargetBackgroundBuilder {
    pub fn industry(mut self, value: Patch<String>) -> Self {
        self.industry = value;
        self
    }
    pub fn use_case(mut self, value: Patch<String>) -> Self {
        self.use_case = value;
        self
    }
    pub fn competitors(mut self, value: Patch<Vec<String>>) -> Self {
        self.competitors = value;
        self
    }
    pub fn agentic_profiling_enabled(mut self, value: Optional<bool>) -> Self {
        self.agentic_profiling_enabled = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<TargetBackground> {
        Ok(TargetBackground {
            industry: self.industry,
            use_case: self.use_case,
            competitors: self.competitors,
            agentic_profiling_enabled: self.agentic_profiling_enabled,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<TargetBackground> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        TargetBackground::parse(value, mode)
    }
}
impl fmt::Debug for TargetBackgroundBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetBackgroundBuilder([REDACTED])")
    }
}
impl fmt::Debug for TargetBackground {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetBackground([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_d987178d94b8ca044155bcea`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetConnectionType {
    DATABRICKS,
    BEDROCK,
    OPENAI,
    HUGGINGFACE,
    CUSTOM,
    REST,
    STREAMING,
    WEBSOCKET,
    WEBSOCKETSTREAMING,
    MSCOPILOTSTUDIO,
    VERTEXAI,
    CUSTOMTARGETADAPTER,
    N8N,
    Unknown(String),
}
impl TargetConnectionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::DATABRICKS => "DATABRICKS",
            Self::BEDROCK => "BEDROCK",
            Self::OPENAI => "OPENAI",
            Self::HUGGINGFACE => "HUGGING_FACE",
            Self::CUSTOM => "CUSTOM",
            Self::REST => "REST",
            Self::STREAMING => "STREAMING",
            Self::WEBSOCKET => "WEBSOCKET",
            Self::WEBSOCKETSTREAMING => "WEBSOCKET_STREAMING",
            Self::MSCOPILOTSTUDIO => "MS_COPILOT_STUDIO",
            Self::VERTEXAI => "VERTEX_AI",
            Self::CUSTOMTARGETADAPTER => "CUSTOM_TARGET_ADAPTER",
            Self::N8N => "N8N",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TargetConnectionType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TargetConnectionType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "DATABRICKS" => Self::DATABRICKS,
            "BEDROCK" => Self::BEDROCK,
            "OPENAI" => Self::OPENAI,
            "HUGGING_FACE" => Self::HUGGINGFACE,
            "CUSTOM" => Self::CUSTOM,
            "REST" => Self::REST,
            "STREAMING" => Self::STREAMING,
            "WEBSOCKET" => Self::WEBSOCKET,
            "WEBSOCKET_STREAMING" => Self::WEBSOCKETSTREAMING,
            "MS_COPILOT_STUDIO" => Self::MSCOPILOTSTUDIO,
            "VERTEX_AI" => Self::VERTEXAI,
            "CUSTOM_TARGET_ADAPTER" => Self::CUSTOMTARGETADAPTER,
            "N8N" => Self::N8N,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TargetConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetConnectionType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_4d9b62142d7ee4d7886602f4`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TargetJobRequest {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for TargetJobRequest {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["uuid", "version"];
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
        map.serialize_entry("uuid", &self.uuid)?;
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TargetJobRequest {
    pub fn builder() -> TargetJobRequestBuilder {
        TargetJobRequestBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_4d9b62142d7ee4d7886602f4";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct TargetJobRequestBuilder {
    uuid: Option<String>,
    version: Patch<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl TargetJobRequestBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<TargetJobRequest> {
        Ok(TargetJobRequest {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            version: self.version,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<TargetJobRequest> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        TargetJobRequest::parse(value, mode)
    }
}
impl fmt::Debug for TargetJobRequestBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetJobRequestBuilder([REDACTED])")
    }
}
impl fmt::Debug for TargetJobRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetJobRequest([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_a2aa0eec5a51ea78947d2434`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TargetMetadata {
    #[serde(rename = "multi_turn", default)]
    pub multi_turn: Optional<bool>,
    #[serde(rename = "multi_turn_error_message", default)]
    pub multi_turn_error_message: Patch<String>,
    #[serde(rename = "supports_multimodal_files", default)]
    pub supports_multimodal_files: Optional<bool>,
    #[serde(rename = "supports_multimodal_files_error_message", default)]
    pub supports_multimodal_files_error_message: Patch<String>,
    #[serde(rename = "audio_probe_passed", default)]
    pub audio_probe_passed: Patch<bool>,
    #[serde(rename = "audio_probe_error", default)]
    pub audio_probe_error: Patch<String>,
    #[serde(rename = "audio_multi_turn_probe_passed", default)]
    pub audio_multi_turn_probe_passed: Patch<bool>,
    #[serde(rename = "audio_multi_turn_probe_error", default)]
    pub audio_multi_turn_probe_error: Patch<String>,
    #[serde(rename = "rate_limit", default)]
    pub rate_limit: Patch<FiniteNumber>,
    #[serde(rename = "rate_limit_enabled", default)]
    pub rate_limit_enabled: Optional<bool>,
    #[serde(rename = "rate_limit_error_code", default)]
    pub rate_limit_error_code: Patch<FiniteNumber>,
    #[serde(rename = "rate_limit_error_json", default)]
    pub rate_limit_error_json: Patch<TargetMetadataRateLimitErrorJson>,
    #[serde(rename = "rate_limit_error_message", default)]
    pub rate_limit_error_message: Patch<String>,
    #[serde(rename = "content_filter_enabled", default)]
    pub content_filter_enabled: Optional<bool>,
    #[serde(rename = "content_filter_error_code", default)]
    pub content_filter_error_code: Patch<FiniteNumber>,
    #[serde(rename = "content_filter_error_json", default)]
    pub content_filter_error_json: Patch<TargetMetadataContentFilterErrorJson>,
    #[serde(rename = "content_filter_error_message", default)]
    pub content_filter_error_message: Patch<String>,
    #[serde(rename = "probe_message", default)]
    pub probe_message: Optional<String>,
    #[serde(rename = "request_timeout", default)]
    pub request_timeout: Optional<FiniteNumber>,
    #[serde(rename = "supports_audio", default)]
    pub supports_audio: Optional<bool>,
    #[serde(rename = "audio_format", default)]
    pub audio_format: Patch<TargetMetadataAudioFormat>,
    #[serde(rename = "supports_text", default)]
    pub supports_text: Optional<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for TargetMetadata {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "multi_turn",
            "multi_turn_error_message",
            "supports_multimodal_files",
            "supports_multimodal_files_error_message",
            "audio_probe_passed",
            "audio_probe_error",
            "audio_multi_turn_probe_passed",
            "audio_multi_turn_probe_error",
            "rate_limit",
            "rate_limit_enabled",
            "rate_limit_error_code",
            "rate_limit_error_json",
            "rate_limit_error_message",
            "content_filter_enabled",
            "content_filter_error_code",
            "content_filter_error_json",
            "content_filter_error_message",
            "probe_message",
            "request_timeout",
            "supports_audio",
            "audio_format",
            "supports_text",
        ];
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
        if !self.multi_turn.is_unset() {
            map.serialize_entry("multi_turn", &self.multi_turn)?;
        }
        if !self.multi_turn_error_message.is_unset() {
            map.serialize_entry("multi_turn_error_message", &self.multi_turn_error_message)?;
        }
        if !self.supports_multimodal_files.is_unset() {
            map.serialize_entry("supports_multimodal_files", &self.supports_multimodal_files)?;
        }
        if !self.supports_multimodal_files_error_message.is_unset() {
            map.serialize_entry(
                "supports_multimodal_files_error_message",
                &self.supports_multimodal_files_error_message,
            )?;
        }
        if !self.audio_probe_passed.is_unset() {
            map.serialize_entry("audio_probe_passed", &self.audio_probe_passed)?;
        }
        if !self.audio_probe_error.is_unset() {
            map.serialize_entry("audio_probe_error", &self.audio_probe_error)?;
        }
        if !self.audio_multi_turn_probe_passed.is_unset() {
            map.serialize_entry(
                "audio_multi_turn_probe_passed",
                &self.audio_multi_turn_probe_passed,
            )?;
        }
        if !self.audio_multi_turn_probe_error.is_unset() {
            map.serialize_entry(
                "audio_multi_turn_probe_error",
                &self.audio_multi_turn_probe_error,
            )?;
        }
        if !self.rate_limit.is_unset() {
            map.serialize_entry("rate_limit", &self.rate_limit)?;
        }
        if !self.rate_limit_enabled.is_unset() {
            map.serialize_entry("rate_limit_enabled", &self.rate_limit_enabled)?;
        }
        if !self.rate_limit_error_code.is_unset() {
            map.serialize_entry("rate_limit_error_code", &self.rate_limit_error_code)?;
        }
        if !self.rate_limit_error_json.is_unset() {
            map.serialize_entry("rate_limit_error_json", &self.rate_limit_error_json)?;
        }
        if !self.rate_limit_error_message.is_unset() {
            map.serialize_entry("rate_limit_error_message", &self.rate_limit_error_message)?;
        }
        if !self.content_filter_enabled.is_unset() {
            map.serialize_entry("content_filter_enabled", &self.content_filter_enabled)?;
        }
        if !self.content_filter_error_code.is_unset() {
            map.serialize_entry("content_filter_error_code", &self.content_filter_error_code)?;
        }
        if !self.content_filter_error_json.is_unset() {
            map.serialize_entry("content_filter_error_json", &self.content_filter_error_json)?;
        }
        if !self.content_filter_error_message.is_unset() {
            map.serialize_entry(
                "content_filter_error_message",
                &self.content_filter_error_message,
            )?;
        }
        if !self.probe_message.is_unset() {
            map.serialize_entry("probe_message", &self.probe_message)?;
        }
        if !self.request_timeout.is_unset() {
            map.serialize_entry("request_timeout", &self.request_timeout)?;
        }
        if !self.supports_audio.is_unset() {
            map.serialize_entry("supports_audio", &self.supports_audio)?;
        }
        if !self.audio_format.is_unset() {
            map.serialize_entry("audio_format", &self.audio_format)?;
        }
        if !self.supports_text.is_unset() {
            map.serialize_entry("supports_text", &self.supports_text)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TargetMetadata {
    pub fn builder() -> TargetMetadataBuilder {
        TargetMetadataBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_a2aa0eec5a51ea78947d2434";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct TargetMetadataBuilder {
    multi_turn: Optional<bool>,
    multi_turn_error_message: Patch<String>,
    supports_multimodal_files: Optional<bool>,
    supports_multimodal_files_error_message: Patch<String>,
    audio_probe_passed: Patch<bool>,
    audio_probe_error: Patch<String>,
    audio_multi_turn_probe_passed: Patch<bool>,
    audio_multi_turn_probe_error: Patch<String>,
    rate_limit: Patch<FiniteNumber>,
    rate_limit_enabled: Optional<bool>,
    rate_limit_error_code: Patch<FiniteNumber>,
    rate_limit_error_json: Patch<TargetMetadataRateLimitErrorJson>,
    rate_limit_error_message: Patch<String>,
    content_filter_enabled: Optional<bool>,
    content_filter_error_code: Patch<FiniteNumber>,
    content_filter_error_json: Patch<TargetMetadataContentFilterErrorJson>,
    content_filter_error_message: Patch<String>,
    probe_message: Optional<String>,
    request_timeout: Optional<FiniteNumber>,
    supports_audio: Optional<bool>,
    audio_format: Patch<TargetMetadataAudioFormat>,
    supports_text: Optional<bool>,
    extensions: Extensions<JsonValue>,
}
impl TargetMetadataBuilder {
    pub fn multi_turn(mut self, value: Optional<bool>) -> Self {
        self.multi_turn = value;
        self
    }
    pub fn multi_turn_error_message(mut self, value: Patch<String>) -> Self {
        self.multi_turn_error_message = value;
        self
    }
    pub fn supports_multimodal_files(mut self, value: Optional<bool>) -> Self {
        self.supports_multimodal_files = value;
        self
    }
    pub fn supports_multimodal_files_error_message(mut self, value: Patch<String>) -> Self {
        self.supports_multimodal_files_error_message = value;
        self
    }
    pub fn audio_probe_passed(mut self, value: Patch<bool>) -> Self {
        self.audio_probe_passed = value;
        self
    }
    pub fn audio_probe_error(mut self, value: Patch<String>) -> Self {
        self.audio_probe_error = value;
        self
    }
    pub fn audio_multi_turn_probe_passed(mut self, value: Patch<bool>) -> Self {
        self.audio_multi_turn_probe_passed = value;
        self
    }
    pub fn audio_multi_turn_probe_error(mut self, value: Patch<String>) -> Self {
        self.audio_multi_turn_probe_error = value;
        self
    }
    pub fn rate_limit(mut self, value: Patch<FiniteNumber>) -> Self {
        self.rate_limit = value;
        self
    }
    pub fn rate_limit_enabled(mut self, value: Optional<bool>) -> Self {
        self.rate_limit_enabled = value;
        self
    }
    pub fn rate_limit_error_code(mut self, value: Patch<FiniteNumber>) -> Self {
        self.rate_limit_error_code = value;
        self
    }
    pub fn rate_limit_error_json(mut self, value: Patch<TargetMetadataRateLimitErrorJson>) -> Self {
        self.rate_limit_error_json = value;
        self
    }
    pub fn rate_limit_error_message(mut self, value: Patch<String>) -> Self {
        self.rate_limit_error_message = value;
        self
    }
    pub fn content_filter_enabled(mut self, value: Optional<bool>) -> Self {
        self.content_filter_enabled = value;
        self
    }
    pub fn content_filter_error_code(mut self, value: Patch<FiniteNumber>) -> Self {
        self.content_filter_error_code = value;
        self
    }
    pub fn content_filter_error_json(
        mut self,
        value: Patch<TargetMetadataContentFilterErrorJson>,
    ) -> Self {
        self.content_filter_error_json = value;
        self
    }
    pub fn content_filter_error_message(mut self, value: Patch<String>) -> Self {
        self.content_filter_error_message = value;
        self
    }
    pub fn probe_message(mut self, value: Optional<String>) -> Self {
        self.probe_message = value;
        self
    }
    pub fn request_timeout(mut self, value: Optional<FiniteNumber>) -> Self {
        self.request_timeout = value;
        self
    }
    pub fn supports_audio(mut self, value: Optional<bool>) -> Self {
        self.supports_audio = value;
        self
    }
    pub fn audio_format(mut self, value: Patch<TargetMetadataAudioFormat>) -> Self {
        self.audio_format = value;
        self
    }
    pub fn supports_text(mut self, value: Optional<bool>) -> Self {
        self.supports_text = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<TargetMetadata> {
        Ok(TargetMetadata {
            multi_turn: self.multi_turn,
            multi_turn_error_message: self.multi_turn_error_message,
            supports_multimodal_files: self.supports_multimodal_files,
            supports_multimodal_files_error_message: self.supports_multimodal_files_error_message,
            audio_probe_passed: self.audio_probe_passed,
            audio_probe_error: self.audio_probe_error,
            audio_multi_turn_probe_passed: self.audio_multi_turn_probe_passed,
            audio_multi_turn_probe_error: self.audio_multi_turn_probe_error,
            rate_limit: self.rate_limit,
            rate_limit_enabled: self.rate_limit_enabled,
            rate_limit_error_code: self.rate_limit_error_code,
            rate_limit_error_json: self.rate_limit_error_json,
            rate_limit_error_message: self.rate_limit_error_message,
            content_filter_enabled: self.content_filter_enabled,
            content_filter_error_code: self.content_filter_error_code,
            content_filter_error_json: self.content_filter_error_json,
            content_filter_error_message: self.content_filter_error_message,
            probe_message: self.probe_message,
            request_timeout: self.request_timeout,
            supports_audio: self.supports_audio,
            audio_format: self.audio_format,
            supports_text: self.supports_text,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<TargetMetadata> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        TargetMetadata::parse(value, mode)
    }
}
impl fmt::Debug for TargetMetadataBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetMetadataBuilder([REDACTED])")
    }
}
impl fmt::Debug for TargetMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetMetadata([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_299b0d36a244900b95b5ea9a`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetMetadataAudioFormat {
    WAV,
    MP3,
    PCM16,
    Unknown(String),
}
impl TargetMetadataAudioFormat {
    pub fn as_str(&self) -> &str {
        match self {
            Self::WAV => "WAV",
            Self::MP3 => "MP3",
            Self::PCM16 => "PCM16",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TargetMetadataAudioFormat {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TargetMetadataAudioFormat {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "WAV" => Self::WAV,
            "MP3" => Self::MP3,
            "PCM16" => Self::PCM16,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TargetMetadataAudioFormat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetMetadataAudioFormat([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_154aa3431ab9c91126c7fea3`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TargetMetadataContentFilterErrorJson {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for TargetMetadataContentFilterErrorJson {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TargetMetadataContentFilterErrorJson {
    pub fn builder() -> TargetMetadataContentFilterErrorJsonBuilder {
        TargetMetadataContentFilterErrorJsonBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_154aa3431ab9c91126c7fea3";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct TargetMetadataContentFilterErrorJsonBuilder {
    extensions: Extensions<JsonValue>,
}
impl TargetMetadataContentFilterErrorJsonBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<TargetMetadataContentFilterErrorJson> {
        Ok(TargetMetadataContentFilterErrorJson {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<TargetMetadataContentFilterErrorJson> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        TargetMetadataContentFilterErrorJson::parse(value, mode)
    }
}
impl fmt::Debug for TargetMetadataContentFilterErrorJsonBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetMetadataContentFilterErrorJsonBuilder([REDACTED])")
    }
}
impl fmt::Debug for TargetMetadataContentFilterErrorJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetMetadataContentFilterErrorJson([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_27f1e141776e7dbbdf3eb383`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TargetMetadataRateLimitErrorJson {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for TargetMetadataRateLimitErrorJson {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TargetMetadataRateLimitErrorJson {
    pub fn builder() -> TargetMetadataRateLimitErrorJsonBuilder {
        TargetMetadataRateLimitErrorJsonBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_27f1e141776e7dbbdf3eb383";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct TargetMetadataRateLimitErrorJsonBuilder {
    extensions: Extensions<JsonValue>,
}
impl TargetMetadataRateLimitErrorJsonBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<TargetMetadataRateLimitErrorJson> {
        Ok(TargetMetadataRateLimitErrorJson {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<TargetMetadataRateLimitErrorJson> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        TargetMetadataRateLimitErrorJson::parse(value, mode)
    }
}
impl fmt::Debug for TargetMetadataRateLimitErrorJsonBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetMetadataRateLimitErrorJsonBuilder([REDACTED])")
    }
}
impl fmt::Debug for TargetMetadataRateLimitErrorJson {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetMetadataRateLimitErrorJson([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_d92297e92e3d83f670e7099e`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TargetReference {
    #[serde(rename = "uuid")]
    pub uuid: String,
    #[serde(rename = "tsg_id")]
    pub tsg_id: String,
    #[serde(rename = "name")]
    pub name: String,
    #[serde(rename = "description", default)]
    pub description: Patch<String>,
    #[serde(rename = "target_type", default)]
    pub target_type: Patch<TargetReferenceTargetType>,
    #[serde(rename = "connection_type", default)]
    pub connection_type: Patch<TargetReferenceConnectionType>,
    #[serde(rename = "api_endpoint_type", default)]
    pub api_endpoint_type: Patch<TargetReferenceApiEndpointType>,
    #[serde(rename = "response_mode", default)]
    pub response_mode: Patch<TargetReferenceResponseMode>,
    #[serde(rename = "auth_type", default)]
    pub auth_type: Patch<TargetReferenceAuthType>,
    #[serde(rename = "session_supported", default)]
    pub session_supported: Optional<bool>,
    #[serde(rename = "extra_info", default)]
    pub extra_info: Patch<TargetReferenceExtraInfo>,
    #[serde(rename = "adapter_uuid", default)]
    pub adapter_uuid: Patch<String>,
    #[serde(rename = "status")]
    pub status: TargetStatus,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(rename = "validated")]
    pub validated: bool,
    #[serde(rename = "version", default)]
    pub version: Patch<FiniteNumber>,
    #[serde(rename = "secret_version", default)]
    pub secret_version: Patch<String>,
    #[serde(rename = "created_by_user_id", default)]
    pub created_by_user_id: Patch<String>,
    #[serde(rename = "updated_by_user_id", default)]
    pub updated_by_user_id: Patch<String>,
    #[serde(rename = "created_at")]
    pub created_at: String,
    #[serde(rename = "updated_at")]
    pub updated_at: String,
    #[serde(rename = "profiling_status", default)]
    pub profiling_status: Patch<TargetReferenceProfilingStatus>,
    #[serde(rename = "canonical_id", default)]
    pub canonical_id: Patch<String>,
    #[serde(rename = "target_metadata", default)]
    pub target_metadata: Optional<TargetMetadata>,
    #[serde(rename = "adapter_secret_version", default)]
    pub adapter_secret_version: Patch<String>,
    #[serde(rename = "target_background", default)]
    pub target_background: Patch<TargetBackground>,
    #[serde(rename = "profiling_progress", default)]
    pub profiling_progress: Patch<FiniteNumber>,
    #[serde(rename = "additional_context", default)]
    pub additional_context: Patch<TargetAdditionalContext>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for TargetReference {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "uuid",
            "tsg_id",
            "name",
            "description",
            "target_type",
            "connection_type",
            "api_endpoint_type",
            "response_mode",
            "auth_type",
            "session_supported",
            "extra_info",
            "adapter_uuid",
            "status",
            "active",
            "validated",
            "version",
            "secret_version",
            "created_by_user_id",
            "updated_by_user_id",
            "created_at",
            "updated_at",
            "profiling_status",
            "canonical_id",
            "target_metadata",
            "adapter_secret_version",
            "target_background",
            "profiling_progress",
            "additional_context",
        ];
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
        map.serialize_entry("uuid", &self.uuid)?;
        map.serialize_entry("tsg_id", &self.tsg_id)?;
        map.serialize_entry("name", &self.name)?;
        if !self.description.is_unset() {
            map.serialize_entry("description", &self.description)?;
        }
        if !self.target_type.is_unset() {
            map.serialize_entry("target_type", &self.target_type)?;
        }
        if !self.connection_type.is_unset() {
            map.serialize_entry("connection_type", &self.connection_type)?;
        }
        if !self.api_endpoint_type.is_unset() {
            map.serialize_entry("api_endpoint_type", &self.api_endpoint_type)?;
        }
        if !self.response_mode.is_unset() {
            map.serialize_entry("response_mode", &self.response_mode)?;
        }
        if !self.auth_type.is_unset() {
            map.serialize_entry("auth_type", &self.auth_type)?;
        }
        if !self.session_supported.is_unset() {
            map.serialize_entry("session_supported", &self.session_supported)?;
        }
        if !self.extra_info.is_unset() {
            map.serialize_entry("extra_info", &self.extra_info)?;
        }
        if !self.adapter_uuid.is_unset() {
            map.serialize_entry("adapter_uuid", &self.adapter_uuid)?;
        }
        map.serialize_entry("status", &self.status)?;
        map.serialize_entry("active", &self.active)?;
        map.serialize_entry("validated", &self.validated)?;
        if !self.version.is_unset() {
            map.serialize_entry("version", &self.version)?;
        }
        if !self.secret_version.is_unset() {
            map.serialize_entry("secret_version", &self.secret_version)?;
        }
        if !self.created_by_user_id.is_unset() {
            map.serialize_entry("created_by_user_id", &self.created_by_user_id)?;
        }
        if !self.updated_by_user_id.is_unset() {
            map.serialize_entry("updated_by_user_id", &self.updated_by_user_id)?;
        }
        map.serialize_entry("created_at", &self.created_at)?;
        map.serialize_entry("updated_at", &self.updated_at)?;
        if !self.profiling_status.is_unset() {
            map.serialize_entry("profiling_status", &self.profiling_status)?;
        }
        if !self.canonical_id.is_unset() {
            map.serialize_entry("canonical_id", &self.canonical_id)?;
        }
        if !self.target_metadata.is_unset() {
            map.serialize_entry("target_metadata", &self.target_metadata)?;
        }
        if !self.adapter_secret_version.is_unset() {
            map.serialize_entry("adapter_secret_version", &self.adapter_secret_version)?;
        }
        if !self.target_background.is_unset() {
            map.serialize_entry("target_background", &self.target_background)?;
        }
        if !self.profiling_progress.is_unset() {
            map.serialize_entry("profiling_progress", &self.profiling_progress)?;
        }
        if !self.additional_context.is_unset() {
            map.serialize_entry("additional_context", &self.additional_context)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TargetReference {
    pub fn builder() -> TargetReferenceBuilder {
        TargetReferenceBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_d92297e92e3d83f670e7099e";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct TargetReferenceBuilder {
    uuid: Option<String>,
    tsg_id: Option<String>,
    name: Option<String>,
    description: Patch<String>,
    target_type: Patch<TargetReferenceTargetType>,
    connection_type: Patch<TargetReferenceConnectionType>,
    api_endpoint_type: Patch<TargetReferenceApiEndpointType>,
    response_mode: Patch<TargetReferenceResponseMode>,
    auth_type: Patch<TargetReferenceAuthType>,
    session_supported: Optional<bool>,
    extra_info: Patch<TargetReferenceExtraInfo>,
    adapter_uuid: Patch<String>,
    status: Option<TargetStatus>,
    active: Option<bool>,
    validated: Option<bool>,
    version: Patch<FiniteNumber>,
    secret_version: Patch<String>,
    created_by_user_id: Patch<String>,
    updated_by_user_id: Patch<String>,
    created_at: Option<String>,
    updated_at: Option<String>,
    profiling_status: Patch<TargetReferenceProfilingStatus>,
    canonical_id: Patch<String>,
    target_metadata: Optional<TargetMetadata>,
    adapter_secret_version: Patch<String>,
    target_background: Patch<TargetBackground>,
    profiling_progress: Patch<FiniteNumber>,
    additional_context: Patch<TargetAdditionalContext>,
    extensions: Extensions<JsonValue>,
}
impl TargetReferenceBuilder {
    pub fn uuid(mut self, value: String) -> Self {
        self.uuid = Some(value);
        self
    }
    pub fn tsg_id(mut self, value: String) -> Self {
        self.tsg_id = Some(value);
        self
    }
    pub fn name(mut self, value: String) -> Self {
        self.name = Some(value);
        self
    }
    pub fn description(mut self, value: Patch<String>) -> Self {
        self.description = value;
        self
    }
    pub fn target_type(mut self, value: Patch<TargetReferenceTargetType>) -> Self {
        self.target_type = value;
        self
    }
    pub fn connection_type(mut self, value: Patch<TargetReferenceConnectionType>) -> Self {
        self.connection_type = value;
        self
    }
    pub fn api_endpoint_type(mut self, value: Patch<TargetReferenceApiEndpointType>) -> Self {
        self.api_endpoint_type = value;
        self
    }
    pub fn response_mode(mut self, value: Patch<TargetReferenceResponseMode>) -> Self {
        self.response_mode = value;
        self
    }
    pub fn auth_type(mut self, value: Patch<TargetReferenceAuthType>) -> Self {
        self.auth_type = value;
        self
    }
    pub fn session_supported(mut self, value: Optional<bool>) -> Self {
        self.session_supported = value;
        self
    }
    pub fn extra_info(mut self, value: Patch<TargetReferenceExtraInfo>) -> Self {
        self.extra_info = value;
        self
    }
    pub fn adapter_uuid(mut self, value: Patch<String>) -> Self {
        self.adapter_uuid = value;
        self
    }
    pub fn status(mut self, value: TargetStatus) -> Self {
        self.status = Some(value);
        self
    }
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }
    pub fn validated(mut self, value: bool) -> Self {
        self.validated = Some(value);
        self
    }
    pub fn version(mut self, value: Patch<FiniteNumber>) -> Self {
        self.version = value;
        self
    }
    pub fn secret_version(mut self, value: Patch<String>) -> Self {
        self.secret_version = value;
        self
    }
    pub fn created_by_user_id(mut self, value: Patch<String>) -> Self {
        self.created_by_user_id = value;
        self
    }
    pub fn updated_by_user_id(mut self, value: Patch<String>) -> Self {
        self.updated_by_user_id = value;
        self
    }
    pub fn created_at(mut self, value: String) -> Self {
        self.created_at = Some(value);
        self
    }
    pub fn updated_at(mut self, value: String) -> Self {
        self.updated_at = Some(value);
        self
    }
    pub fn profiling_status(mut self, value: Patch<TargetReferenceProfilingStatus>) -> Self {
        self.profiling_status = value;
        self
    }
    pub fn canonical_id(mut self, value: Patch<String>) -> Self {
        self.canonical_id = value;
        self
    }
    pub fn target_metadata(mut self, value: Optional<TargetMetadata>) -> Self {
        self.target_metadata = value;
        self
    }
    pub fn adapter_secret_version(mut self, value: Patch<String>) -> Self {
        self.adapter_secret_version = value;
        self
    }
    pub fn target_background(mut self, value: Patch<TargetBackground>) -> Self {
        self.target_background = value;
        self
    }
    pub fn profiling_progress(mut self, value: Patch<FiniteNumber>) -> Self {
        self.profiling_progress = value;
        self
    }
    pub fn additional_context(mut self, value: Patch<TargetAdditionalContext>) -> Self {
        self.additional_context = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<TargetReference> {
        Ok(TargetReference {
            uuid: self
                .uuid
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            tsg_id: self
                .tsg_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            name: self
                .name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self.description,
            target_type: self.target_type,
            connection_type: self.connection_type,
            api_endpoint_type: self.api_endpoint_type,
            response_mode: self.response_mode,
            auth_type: self.auth_type,
            session_supported: self.session_supported,
            extra_info: self.extra_info,
            adapter_uuid: self.adapter_uuid,
            status: self
                .status
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            active: self
                .active
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            validated: self
                .validated
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            version: self.version,
            secret_version: self.secret_version,
            created_by_user_id: self.created_by_user_id,
            updated_by_user_id: self.updated_by_user_id,
            created_at: self
                .created_at
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            updated_at: self
                .updated_at
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            profiling_status: self.profiling_status,
            canonical_id: self.canonical_id,
            target_metadata: self.target_metadata,
            adapter_secret_version: self.adapter_secret_version,
            target_background: self.target_background,
            profiling_progress: self.profiling_progress,
            additional_context: self.additional_context,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<TargetReference> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        TargetReference::parse(value, mode)
    }
}
impl fmt::Debug for TargetReferenceBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetReferenceBuilder([REDACTED])")
    }
}
impl fmt::Debug for TargetReference {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetReference([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_8dbb658ebe16305ff6adcf69`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetReferenceApiEndpointType {
    PUBLIC,
    PRIVATE,
    NETWORKBROKER,
    Unknown(String),
}
impl TargetReferenceApiEndpointType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::PUBLIC => "PUBLIC",
            Self::PRIVATE => "PRIVATE",
            Self::NETWORKBROKER => "NETWORK_BROKER",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TargetReferenceApiEndpointType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TargetReferenceApiEndpointType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "PUBLIC" => Self::PUBLIC,
            "PRIVATE" => Self::PRIVATE,
            "NETWORK_BROKER" => Self::NETWORKBROKER,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TargetReferenceApiEndpointType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetReferenceApiEndpointType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_95564f61c09d2acf9ed2e6c9`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetReferenceAuthType {
    HEADERS,
    BASICAUTH,
    OAUTH2,
    CUSTOMTARGETADAPTER,
    Unknown(String),
}
impl TargetReferenceAuthType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::HEADERS => "HEADERS",
            Self::BASICAUTH => "BASIC_AUTH",
            Self::OAUTH2 => "OAUTH2",
            Self::CUSTOMTARGETADAPTER => "CUSTOM_TARGET_ADAPTER",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TargetReferenceAuthType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TargetReferenceAuthType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "HEADERS" => Self::HEADERS,
            "BASIC_AUTH" => Self::BASICAUTH,
            "OAUTH2" => Self::OAUTH2,
            "CUSTOM_TARGET_ADAPTER" => Self::CUSTOMTARGETADAPTER,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TargetReferenceAuthType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetReferenceAuthType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_14abaddfbda2a4ac126f0ffd`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetReferenceConnectionType {
    DATABRICKS,
    BEDROCK,
    OPENAI,
    HUGGINGFACE,
    CUSTOM,
    REST,
    STREAMING,
    WEBSOCKET,
    WEBSOCKETSTREAMING,
    MSCOPILOTSTUDIO,
    VERTEXAI,
    CUSTOMTARGETADAPTER,
    N8N,
    Unknown(String),
}
impl TargetReferenceConnectionType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::DATABRICKS => "DATABRICKS",
            Self::BEDROCK => "BEDROCK",
            Self::OPENAI => "OPENAI",
            Self::HUGGINGFACE => "HUGGING_FACE",
            Self::CUSTOM => "CUSTOM",
            Self::REST => "REST",
            Self::STREAMING => "STREAMING",
            Self::WEBSOCKET => "WEBSOCKET",
            Self::WEBSOCKETSTREAMING => "WEBSOCKET_STREAMING",
            Self::MSCOPILOTSTUDIO => "MS_COPILOT_STUDIO",
            Self::VERTEXAI => "VERTEX_AI",
            Self::CUSTOMTARGETADAPTER => "CUSTOM_TARGET_ADAPTER",
            Self::N8N => "N8N",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TargetReferenceConnectionType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TargetReferenceConnectionType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "DATABRICKS" => Self::DATABRICKS,
            "BEDROCK" => Self::BEDROCK,
            "OPENAI" => Self::OPENAI,
            "HUGGING_FACE" => Self::HUGGINGFACE,
            "CUSTOM" => Self::CUSTOM,
            "REST" => Self::REST,
            "STREAMING" => Self::STREAMING,
            "WEBSOCKET" => Self::WEBSOCKET,
            "WEBSOCKET_STREAMING" => Self::WEBSOCKETSTREAMING,
            "MS_COPILOT_STUDIO" => Self::MSCOPILOTSTUDIO,
            "VERTEX_AI" => Self::VERTEXAI,
            "CUSTOM_TARGET_ADAPTER" => Self::CUSTOMTARGETADAPTER,
            "N8N" => Self::N8N,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TargetReferenceConnectionType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetReferenceConnectionType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_ecae7803321cf3598fc44fd3`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TargetReferenceExtraInfo {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for TargetReferenceExtraInfo {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TargetReferenceExtraInfo {
    pub fn builder() -> TargetReferenceExtraInfoBuilder {
        TargetReferenceExtraInfoBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_ecae7803321cf3598fc44fd3";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct TargetReferenceExtraInfoBuilder {
    extensions: Extensions<JsonValue>,
}
impl TargetReferenceExtraInfoBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<TargetReferenceExtraInfo> {
        Ok(TargetReferenceExtraInfo {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<TargetReferenceExtraInfo> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        TargetReferenceExtraInfo::parse(value, mode)
    }
}
impl fmt::Debug for TargetReferenceExtraInfoBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetReferenceExtraInfoBuilder([REDACTED])")
    }
}
impl fmt::Debug for TargetReferenceExtraInfo {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetReferenceExtraInfo([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_30b0d07f8f408aaf908043d8`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetReferenceProfilingStatus {
    NOTSTARTED,
    INPROGRESS,
    COMPLETED,
    FAILED,
    PARTIALLYCOMPLETE,
    Unknown(String),
}
impl TargetReferenceProfilingStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::NOTSTARTED => "NOT_STARTED",
            Self::INPROGRESS => "IN_PROGRESS",
            Self::COMPLETED => "COMPLETED",
            Self::FAILED => "FAILED",
            Self::PARTIALLYCOMPLETE => "PARTIALLY_COMPLETE",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TargetReferenceProfilingStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TargetReferenceProfilingStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "NOT_STARTED" => Self::NOTSTARTED,
            "IN_PROGRESS" => Self::INPROGRESS,
            "COMPLETED" => Self::COMPLETED,
            "FAILED" => Self::FAILED,
            "PARTIALLY_COMPLETE" => Self::PARTIALLYCOMPLETE,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TargetReferenceProfilingStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetReferenceProfilingStatus([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_fb85dcf631a0b4d91dabcde4`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetReferenceResponseMode {
    REST,
    STREAMING,
    WEBSOCKET,
    WEBSOCKETSTREAMING,
    REALTIME,
    Unknown(String),
}
impl TargetReferenceResponseMode {
    pub fn as_str(&self) -> &str {
        match self {
            Self::REST => "REST",
            Self::STREAMING => "STREAMING",
            Self::WEBSOCKET => "WEBSOCKET",
            Self::WEBSOCKETSTREAMING => "WEBSOCKET_STREAMING",
            Self::REALTIME => "REALTIME",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TargetReferenceResponseMode {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TargetReferenceResponseMode {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "REST" => Self::REST,
            "STREAMING" => Self::STREAMING,
            "WEBSOCKET" => Self::WEBSOCKET,
            "WEBSOCKET_STREAMING" => Self::WEBSOCKETSTREAMING,
            "REALTIME" => Self::REALTIME,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TargetReferenceResponseMode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetReferenceResponseMode([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_319c1603a6b377370e1049a8`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetReferenceTargetType {
    APPLICATION,
    AGENT,
    MODEL,
    Unknown(String),
}
impl TargetReferenceTargetType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::APPLICATION => "APPLICATION",
            Self::AGENT => "AGENT",
            Self::MODEL => "MODEL",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TargetReferenceTargetType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TargetReferenceTargetType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "APPLICATION" => Self::APPLICATION,
            "AGENT" => Self::AGENT,
            "MODEL" => Self::MODEL,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TargetReferenceTargetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetReferenceTargetType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_795fb2f4d47a34f8fe1cef3b`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetStatus {
    DRAFT,
    VALIDATING,
    VALIDATED,
    ACTIVE,
    INACTIVE,
    FAILED,
    PENDINGAUTH,
    Unknown(String),
}
impl TargetStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::DRAFT => "DRAFT",
            Self::VALIDATING => "VALIDATING",
            Self::VALIDATED => "VALIDATED",
            Self::ACTIVE => "ACTIVE",
            Self::INACTIVE => "INACTIVE",
            Self::FAILED => "FAILED",
            Self::PENDINGAUTH => "PENDING_AUTH",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TargetStatus {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TargetStatus {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "DRAFT" => Self::DRAFT,
            "VALIDATING" => Self::VALIDATING,
            "VALIDATED" => Self::VALIDATED,
            "ACTIVE" => Self::ACTIVE,
            "INACTIVE" => Self::INACTIVE,
            "FAILED" => Self::FAILED,
            "PENDING_AUTH" => Self::PENDINGAUTH,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TargetStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetStatus([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b9ef58a5822d989d1a440ecd`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TargetType {
    APPLICATION,
    AGENT,
    MODEL,
    Unknown(String),
}
impl TargetType {
    pub fn as_str(&self) -> &str {
        match self {
            Self::APPLICATION => "APPLICATION",
            Self::AGENT => "AGENT",
            Self::MODEL => "MODEL",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TargetType {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TargetType {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "APPLICATION" => Self::APPLICATION,
            "AGENT" => Self::AGENT,
            "MODEL" => Self::MODEL,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TargetType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TargetType([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_50cd35ae891d1ef1a2a1b94f`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TechniqueModel {
    #[serde(rename = "id")]
    pub id: String,
    #[serde(rename = "display_name")]
    pub display_name: String,
    #[serde(rename = "compliance_id")]
    pub compliance_id: String,
    #[serde(rename = "description")]
    pub description: String,
    #[serde(rename = "link")]
    pub link: String,
    #[serde(rename = "version")]
    pub version: String,
    #[serde(rename = "active")]
    pub active: bool,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for TechniqueModel {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "id",
            "display_name",
            "compliance_id",
            "description",
            "link",
            "version",
            "active",
        ];
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
        map.serialize_entry("id", &self.id)?;
        map.serialize_entry("display_name", &self.display_name)?;
        map.serialize_entry("compliance_id", &self.compliance_id)?;
        map.serialize_entry("description", &self.description)?;
        map.serialize_entry("link", &self.link)?;
        map.serialize_entry("version", &self.version)?;
        map.serialize_entry("active", &self.active)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TechniqueModel {
    pub fn builder() -> TechniqueModelBuilder {
        TechniqueModelBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_50cd35ae891d1ef1a2a1b94f";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct TechniqueModelBuilder {
    id: Option<String>,
    display_name: Option<String>,
    compliance_id: Option<String>,
    description: Option<String>,
    link: Option<String>,
    version: Option<String>,
    active: Option<bool>,
    extensions: Extensions<JsonValue>,
}
impl TechniqueModelBuilder {
    pub fn id(mut self, value: String) -> Self {
        self.id = Some(value);
        self
    }
    pub fn display_name(mut self, value: String) -> Self {
        self.display_name = Some(value);
        self
    }
    pub fn compliance_id(mut self, value: String) -> Self {
        self.compliance_id = Some(value);
        self
    }
    pub fn description(mut self, value: String) -> Self {
        self.description = Some(value);
        self
    }
    pub fn link(mut self, value: String) -> Self {
        self.link = Some(value);
        self
    }
    pub fn version(mut self, value: String) -> Self {
        self.version = Some(value);
        self
    }
    pub fn active(mut self, value: bool) -> Self {
        self.active = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<TechniqueModel> {
        Ok(TechniqueModel {
            id: self
                .id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            display_name: self
                .display_name
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            compliance_id: self
                .compliance_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            description: self
                .description
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            link: self
                .link
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            version: self
                .version
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            active: self
                .active
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<TechniqueModel> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        TechniqueModel::parse(value, mode)
    }
}
impl fmt::Debug for TechniqueModelBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TechniqueModelBuilder([REDACTED])")
    }
}
impl fmt::Debug for TechniqueModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TechniqueModel([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_ff52649c866790e970b9f30b`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct TenantLanguagesResponse {
    #[serde(rename = "multilingual_enabled")]
    pub multilingual_enabled: bool,
    #[serde(rename = "supported_job_types")]
    pub supported_job_types: Vec<String>,
    #[serde(rename = "languages")]
    pub languages: Vec<LanguageDropdownOption>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for TenantLanguagesResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["multilingual_enabled", "supported_job_types", "languages"];
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
        map.serialize_entry("multilingual_enabled", &self.multilingual_enabled)?;
        map.serialize_entry("supported_job_types", &self.supported_job_types)?;
        map.serialize_entry("languages", &self.languages)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl TenantLanguagesResponse {
    pub fn builder() -> TenantLanguagesResponseBuilder {
        TenantLanguagesResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_ff52649c866790e970b9f30b";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct TenantLanguagesResponseBuilder {
    multilingual_enabled: Option<bool>,
    supported_job_types: Option<Vec<String>>,
    languages: Option<Vec<LanguageDropdownOption>>,
    extensions: Extensions<JsonValue>,
}
impl TenantLanguagesResponseBuilder {
    pub fn multilingual_enabled(mut self, value: bool) -> Self {
        self.multilingual_enabled = Some(value);
        self
    }
    pub fn supported_job_types(mut self, value: Vec<String>) -> Self {
        self.supported_job_types = Some(value);
        self
    }
    pub fn languages(mut self, value: Vec<LanguageDropdownOption>) -> Self {
        self.languages = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<TenantLanguagesResponse> {
        Ok(TenantLanguagesResponse {
            multilingual_enabled: self
                .multilingual_enabled
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            supported_job_types: self
                .supported_job_types
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            languages: self
                .languages
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<TenantLanguagesResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        TenantLanguagesResponse::parse(value, mode)
    }
}
impl fmt::Debug for TenantLanguagesResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TenantLanguagesResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for TenantLanguagesResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TenantLanguagesResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_a4b6cc53cf5c910e00af9586`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ThreatDetails {
    #[serde(rename = "raw_threat", default)]
    pub raw_threat: Patch<bool>,
    #[serde(rename = "threat_override", default)]
    pub threat_override: Patch<bool>,
    #[serde(rename = "overridden_by", default)]
    pub overridden_by: Patch<String>,
    #[serde(rename = "overridden_at", default)]
    pub overridden_at: Patch<String>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ThreatDetails {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[
            "raw_threat",
            "threat_override",
            "overridden_by",
            "overridden_at",
        ];
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
        if !self.raw_threat.is_unset() {
            map.serialize_entry("raw_threat", &self.raw_threat)?;
        }
        if !self.threat_override.is_unset() {
            map.serialize_entry("threat_override", &self.threat_override)?;
        }
        if !self.overridden_by.is_unset() {
            map.serialize_entry("overridden_by", &self.overridden_by)?;
        }
        if !self.overridden_at.is_unset() {
            map.serialize_entry("overridden_at", &self.overridden_at)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ThreatDetails {
    pub fn builder() -> ThreatDetailsBuilder {
        ThreatDetailsBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_a4b6cc53cf5c910e00af9586";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ThreatDetailsBuilder {
    raw_threat: Patch<bool>,
    threat_override: Patch<bool>,
    overridden_by: Patch<String>,
    overridden_at: Patch<String>,
    extensions: Extensions<JsonValue>,
}
impl ThreatDetailsBuilder {
    pub fn raw_threat(mut self, value: Patch<bool>) -> Self {
        self.raw_threat = value;
        self
    }
    pub fn threat_override(mut self, value: Patch<bool>) -> Self {
        self.threat_override = value;
        self
    }
    pub fn overridden_by(mut self, value: Patch<String>) -> Self {
        self.overridden_by = value;
        self
    }
    pub fn overridden_at(mut self, value: Patch<String>) -> Self {
        self.overridden_at = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ThreatDetails> {
        Ok(ThreatDetails {
            raw_threat: self.raw_threat,
            threat_override: self.threat_override,
            overridden_by: self.overridden_by,
            overridden_at: self.overridden_at,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<ThreatDetails> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ThreatDetails::parse(value, mode)
    }
}
impl fmt::Debug for ThreatDetailsBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ThreatDetailsBuilder([REDACTED])")
    }
}
impl fmt::Debug for ThreatDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ThreatDetails([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b969594343cd8dbc9ee56766`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ThreatOverrideItem {
    #[serde(rename = "output_id")]
    pub output_id: String,
    #[serde(rename = "threat_override")]
    pub threat_override: Nullable<bool>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ThreatOverrideItem {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["output_id", "threat_override"];
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
        map.serialize_entry("output_id", &self.output_id)?;
        map.serialize_entry("threat_override", &self.threat_override)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ThreatOverrideItem {
    pub fn builder() -> ThreatOverrideItemBuilder {
        ThreatOverrideItemBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_b969594343cd8dbc9ee56766";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ThreatOverrideItemBuilder {
    output_id: Option<String>,
    threat_override: Option<Nullable<bool>>,
    extensions: Extensions<JsonValue>,
}
impl ThreatOverrideItemBuilder {
    pub fn output_id(mut self, value: String) -> Self {
        self.output_id = Some(value);
        self
    }
    pub fn threat_override(mut self, value: Nullable<bool>) -> Self {
        self.threat_override = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ThreatOverrideItem> {
        Ok(ThreatOverrideItem {
            output_id: self
                .output_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            threat_override: self
                .threat_override
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ThreatOverrideItem> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ThreatOverrideItem::parse(value, mode)
    }
}
impl fmt::Debug for ThreatOverrideItemBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ThreatOverrideItemBuilder([REDACTED])")
    }
}
impl fmt::Debug for ThreatOverrideItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ThreatOverrideItem([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_116cf41e8886d41bc77443d0`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ThreatOverrideRequest {
    #[serde(rename = "overrides")]
    pub overrides: Vec<ThreatOverrideItem>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ThreatOverrideRequest {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["overrides"];
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
        map.serialize_entry("overrides", &self.overrides)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ThreatOverrideRequest {
    pub fn builder() -> ThreatOverrideRequestBuilder {
        ThreatOverrideRequestBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_116cf41e8886d41bc77443d0";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ThreatOverrideRequestBuilder {
    overrides: Option<Vec<ThreatOverrideItem>>,
    extensions: Extensions<JsonValue>,
}
impl ThreatOverrideRequestBuilder {
    pub fn overrides(mut self, value: Vec<ThreatOverrideItem>) -> Self {
        self.overrides = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ThreatOverrideRequest> {
        Ok(ThreatOverrideRequest {
            overrides: self
                .overrides
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ThreatOverrideRequest> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ThreatOverrideRequest::parse(value, mode)
    }
}
impl fmt::Debug for ThreatOverrideRequestBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ThreatOverrideRequestBuilder([REDACTED])")
    }
}
impl fmt::Debug for ThreatOverrideRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ThreatOverrideRequest([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_dcea6eece2e8097a540b0553`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ThreatOverrideResponse {
    #[serde(rename = "parent_id")]
    pub parent_id: String,
    #[serde(rename = "parent_effective_threat")]
    pub parent_effective_threat: Nullable<bool>,
    #[serde(rename = "updated_count")]
    pub updated_count: FiniteNumber,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ThreatOverrideResponse {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["parent_id", "parent_effective_threat", "updated_count"];
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
        map.serialize_entry("parent_id", &self.parent_id)?;
        map.serialize_entry("parent_effective_threat", &self.parent_effective_threat)?;
        map.serialize_entry("updated_count", &self.updated_count)?;
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ThreatOverrideResponse {
    pub fn builder() -> ThreatOverrideResponseBuilder {
        ThreatOverrideResponseBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_dcea6eece2e8097a540b0553";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ThreatOverrideResponseBuilder {
    parent_id: Option<String>,
    parent_effective_threat: Option<Nullable<bool>>,
    updated_count: Option<FiniteNumber>,
    extensions: Extensions<JsonValue>,
}
impl ThreatOverrideResponseBuilder {
    pub fn parent_id(mut self, value: String) -> Self {
        self.parent_id = Some(value);
        self
    }
    pub fn parent_effective_threat(mut self, value: Nullable<bool>) -> Self {
        self.parent_effective_threat = Some(value);
        self
    }
    pub fn updated_count(mut self, value: FiniteNumber) -> Self {
        self.updated_count = Some(value);
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ThreatOverrideResponse> {
        Ok(ThreatOverrideResponse {
            parent_id: self
                .parent_id
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            parent_effective_threat: self
                .parent_effective_threat
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            updated_count: self
                .updated_count
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ThreatOverrideResponse> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ThreatOverrideResponse::parse(value, mode)
    }
}
impl fmt::Debug for ThreatOverrideResponseBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ThreatOverrideResponseBuilder([REDACTED])")
    }
}
impl fmt::Debug for ThreatOverrideResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ThreatOverrideResponse([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_2197d7481a087e7cc9d5fed1`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ToolChainingSubtype {
    CREDENTIALTHEFT,
    UNAUTHORIZEDTRANSACTION,
    CODEEXECUTION,
    DATAEXFILTRATION,
    DATADESTRUCTION,
    PRIVILEGEESCALATION,
    KNOWLEDGEPOISONING,
    DATAMODIFICATION,
    Unknown(String),
}
impl ToolChainingSubtype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::CREDENTIALTHEFT => "CREDENTIAL_THEFT",
            Self::UNAUTHORIZEDTRANSACTION => "UNAUTHORIZED_TRANSACTION",
            Self::CODEEXECUTION => "CODE_EXECUTION",
            Self::DATAEXFILTRATION => "DATA_EXFILTRATION",
            Self::DATADESTRUCTION => "DATA_DESTRUCTION",
            Self::PRIVILEGEESCALATION => "PRIVILEGE_ESCALATION",
            Self::KNOWLEDGEPOISONING => "KNOWLEDGE_POISONING",
            Self::DATAMODIFICATION => "DATA_MODIFICATION",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ToolChainingSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ToolChainingSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "CREDENTIAL_THEFT" => Self::CREDENTIALTHEFT,
            "UNAUTHORIZED_TRANSACTION" => Self::UNAUTHORIZEDTRANSACTION,
            "CODE_EXECUTION" => Self::CODEEXECUTION,
            "DATA_EXFILTRATION" => Self::DATAEXFILTRATION,
            "DATA_DESTRUCTION" => Self::DATADESTRUCTION,
            "PRIVILEGE_ESCALATION" => Self::PRIVILEGEESCALATION,
            "KNOWLEDGE_POISONING" => Self::KNOWLEDGEPOISONING,
            "DATA_MODIFICATION" => Self::DATAMODIFICATION,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ToolChainingSubtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ToolChainingSubtype([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_76b99e4b7396ccb06910745a`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum ToolMisuseSubtype {
    MASSDATALEAKAGE,
    UNAUTHORIZEDDATAACCESS,
    PROCESSFLOODING,
    DATAMODIFICATION,
    DATAINJECTION,
    DATADELETION,
    ROLEIDENTITYMODIFICATION,
    UNAUTHORIZEDSIDEEFFECTEXECUTION,
    FRAUDULENTTRANSACTIONEXECUTION,
    RESOURCEEXHAUSTION,
    Unknown(String),
}
impl ToolMisuseSubtype {
    pub fn as_str(&self) -> &str {
        match self {
            Self::MASSDATALEAKAGE => "MASS_DATA_LEAKAGE",
            Self::UNAUTHORIZEDDATAACCESS => "UNAUTHORIZED_DATA_ACCESS",
            Self::PROCESSFLOODING => "PROCESS_FLOODING",
            Self::DATAMODIFICATION => "DATA_MODIFICATION",
            Self::DATAINJECTION => "DATA_INJECTION",
            Self::DATADELETION => "DATA_DELETION",
            Self::ROLEIDENTITYMODIFICATION => "ROLE_IDENTITY_MODIFICATION",
            Self::UNAUTHORIZEDSIDEEFFECTEXECUTION => "UNAUTHORIZED_SIDE_EFFECT_EXECUTION",
            Self::FRAUDULENTTRANSACTIONEXECUTION => "FRAUDULENT_TRANSACTION_EXECUTION",
            Self::RESOURCEEXHAUSTION => "RESOURCE_EXHAUSTION",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for ToolMisuseSubtype {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for ToolMisuseSubtype {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "MASS_DATA_LEAKAGE" => Self::MASSDATALEAKAGE,
            "UNAUTHORIZED_DATA_ACCESS" => Self::UNAUTHORIZEDDATAACCESS,
            "PROCESS_FLOODING" => Self::PROCESSFLOODING,
            "DATA_MODIFICATION" => Self::DATAMODIFICATION,
            "DATA_INJECTION" => Self::DATAINJECTION,
            "DATA_DELETION" => Self::DATADELETION,
            "ROLE_IDENTITY_MODIFICATION" => Self::ROLEIDENTITYMODIFICATION,
            "UNAUTHORIZED_SIDE_EFFECT_EXECUTION" => Self::UNAUTHORIZEDSIDEEFFECTEXECUTION,
            "FRAUDULENT_TRANSACTION_EXECUTION" => Self::FRAUDULENTTRANSACTIONEXECUTION,
            "RESOURCE_EXHAUSTION" => Self::RESOURCEEXHAUSTION,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for ToolMisuseSubtype {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ToolMisuseSubtype([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_d7d16d4229a74701d599e95b`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ToxicContentGuardrail {
    #[serde(rename = "moderate", default)]
    pub moderate: Optional<GuardrailAction>,
    #[serde(rename = "high", default)]
    pub high: Optional<GuardrailAction>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ToxicContentGuardrail {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["moderate", "high"];
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
        if !self.moderate.is_unset() {
            map.serialize_entry("moderate", &self.moderate)?;
        }
        if !self.high.is_unset() {
            map.serialize_entry("high", &self.high)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ToxicContentGuardrail {
    pub fn builder() -> ToxicContentGuardrailBuilder {
        ToxicContentGuardrailBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_d7d16d4229a74701d599e95b";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ToxicContentGuardrailBuilder {
    moderate: Optional<GuardrailAction>,
    high: Optional<GuardrailAction>,
    extensions: Extensions<JsonValue>,
}
impl ToxicContentGuardrailBuilder {
    pub fn moderate(mut self, value: Optional<GuardrailAction>) -> Self {
        self.moderate = value;
        self
    }
    pub fn high(mut self, value: Optional<GuardrailAction>) -> Self {
        self.high = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ToxicContentGuardrail> {
        Ok(ToxicContentGuardrail {
            moderate: self.moderate,
            high: self.high,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ToxicContentGuardrail> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ToxicContentGuardrail::parse(value, mode)
    }
}
impl fmt::Debug for ToxicContentGuardrailBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ToxicContentGuardrailBuilder([REDACTED])")
    }
}
impl fmt::Debug for ToxicContentGuardrail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ToxicContentGuardrail([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_81cae2975ed2695c5c7d6465`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum TranscriptSource {
    TARGET,
    STT,
    Unknown(String),
}
impl TranscriptSource {
    pub fn as_str(&self) -> &str {
        match self {
            Self::TARGET => "TARGET",
            Self::STT => "STT",
            Self::Unknown(value) => value,
        }
    }
}
impl Serialize for TranscriptSource {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        s.serialize_str(self.as_str())
    }
}
impl<'de> Deserialize<'de> for TranscriptSource {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = String::deserialize(d)?;
        Ok(match value.as_str() {
            "TARGET" => Self::TARGET,
            "STT" => Self::STT,
            _ => Self::Unknown(value),
        })
    }
}
impl fmt::Debug for TranscriptSource {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("TranscriptSource([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_329bdfca5421aed70d6593b7`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ValidationError {
    #[serde(rename = "loc")]
    pub loc: Vec<ValidationErrorLocItem>,
    #[serde(rename = "msg")]
    pub msg: String,
    #[serde(rename = "type")]
    pub r#type: String,
    #[serde(rename = "input", default)]
    pub input: Patch<JsonValue>,
    #[serde(rename = "ctx", default)]
    pub ctx: Optional<ValidationErrorCtx>,
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ValidationError {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &["loc", "msg", "type", "input", "ctx"];
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
        map.serialize_entry("loc", &self.loc)?;
        map.serialize_entry("msg", &self.msg)?;
        map.serialize_entry("type", &self.r#type)?;
        if !self.input.is_unset() {
            map.serialize_entry("input", &self.input)?;
        }
        if !self.ctx.is_unset() {
            map.serialize_entry("ctx", &self.ctx)?;
        }
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ValidationError {
    pub fn builder() -> ValidationErrorBuilder {
        ValidationErrorBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_329bdfca5421aed70d6593b7";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ValidationErrorBuilder {
    loc: Option<Vec<ValidationErrorLocItem>>,
    msg: Option<String>,
    r#type: Option<String>,
    input: Patch<JsonValue>,
    ctx: Optional<ValidationErrorCtx>,
    extensions: Extensions<JsonValue>,
}
impl ValidationErrorBuilder {
    pub fn loc(mut self, value: Vec<ValidationErrorLocItem>) -> Self {
        self.loc = Some(value);
        self
    }
    pub fn msg(mut self, value: String) -> Self {
        self.msg = Some(value);
        self
    }
    pub fn r#type(mut self, value: String) -> Self {
        self.r#type = Some(value);
        self
    }
    pub fn input(mut self, value: Patch<JsonValue>) -> Self {
        self.input = value;
        self
    }
    pub fn ctx(mut self, value: Optional<ValidationErrorCtx>) -> Self {
        self.ctx = value;
        self
    }
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ValidationError> {
        Ok(ValidationError {
            loc: self
                .loc
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            msg: self
                .msg
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            r#type: self
                .r#type
                .ok_or_else(|| crate::Error::new(crate::ErrorKind::Validation))?,
            input: self.input,
            ctx: self.ctx,
            extensions: self.extensions,
        })
    }
    pub fn build_validated(self, mode: crate::validation::Mode) -> crate::Result<ValidationError> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ValidationError::parse(value, mode)
    }
}
impl fmt::Debug for ValidationErrorBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ValidationErrorBuilder([REDACTED])")
    }
}
impl fmt::Debug for ValidationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ValidationError([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_3cb90a921ad6d580a31f5897`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Deserialize)]
#[non_exhaustive]
pub struct ValidationErrorCtx {
    #[serde(flatten)]
    pub extensions: Extensions<JsonValue>,
}
impl Serialize for ValidationErrorCtx {
    fn serialize<S: serde::Serializer>(
        &self,
        serializer: S,
    ) -> std::result::Result<S::Ok, S::Error> {
        const KNOWN: &[&str] = &[];
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
        for (key, value) in self.extensions.iter() {
            map.serialize_entry(key, value)?;
        }
        map.end()
    }
}
impl ValidationErrorCtx {
    pub fn builder() -> ValidationErrorCtxBuilder {
        ValidationErrorCtxBuilder::default()
    }
    pub const SCHEMA: &'static str = "sdp_q_3cb90a921ad6d580a31f5897";
    pub fn parse(value: serde_json::Value, mode: crate::validation::Mode) -> crate::Result<Self> {
        super::support::parse(Self::SCHEMA, value, mode)
    }
}
#[derive(Default)]
pub struct ValidationErrorCtxBuilder {
    extensions: Extensions<JsonValue>,
}
impl ValidationErrorCtxBuilder {
    pub fn extension(mut self, key: impl Into<String>, value: JsonValue) -> Self {
        self.extensions.insert(key.into(), value);
        self
    }
    pub fn build(self) -> crate::Result<ValidationErrorCtx> {
        Ok(ValidationErrorCtx {
            extensions: self.extensions,
        })
    }
    pub fn build_validated(
        self,
        mode: crate::validation::Mode,
    ) -> crate::Result<ValidationErrorCtx> {
        let model = self.build()?;
        let value = super::support::wire_value(&model)?;
        ValidationErrorCtx::parse(value, mode)
    }
}
impl fmt::Debug for ValidationErrorCtxBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ValidationErrorCtxBuilder([REDACTED])")
    }
}
impl fmt::Debug for ValidationErrorCtx {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ValidationErrorCtx([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_b5ca96537d2c6e4dd8bdbb13`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum ValidationErrorLocItem {
    Variant1(String),
    Variant2(ValidationErrorLocItemVariant2),
    Unknown(JsonValue),
}
impl fmt::Debug for ValidationErrorLocItem {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ValidationErrorLocItem([REDACTED])")
    }
}

/// Native typed contract for vendor schema `sdp_q_97e5d58d5e325d9a81aa84a6`. Validate wire constraints with `validation::parse`.
#[derive(Clone, PartialEq, Serialize)]
#[serde(transparent)]
pub struct ValidationErrorLocItemVariant2(FiniteNumber);
impl ValidationErrorLocItemVariant2 {
    pub fn new(value: FiniteNumber) -> crate::Result<Self> {
        crate::validation::parse(
            "sdp_q_97e5d58d5e325d9a81aa84a6",
            serde_json::to_value(&value)
                .map_err(|_| crate::Error::new(crate::ErrorKind::Validation))?,
            crate::validation::Mode::Response,
        )?;
        Ok(Self(value))
    }
    pub fn as_inner(&self) -> &FiniteNumber {
        &self.0
    }
    pub fn into_inner(self) -> FiniteNumber {
        self.0
    }
}
impl<'de> Deserialize<'de> for ValidationErrorLocItemVariant2 {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = FiniteNumber::deserialize(d)?;
        Self::new(value).map_err(|_| serde::de::Error::custom("number contract mismatch"))
    }
}
impl fmt::Debug for ValidationErrorLocItemVariant2 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ValidationErrorLocItemVariant2([REDACTED])")
    }
}
