//! Typed Portkey access-policy contracts. Ordinary formatting is redacted.
use crate::data::{JsonObject, JsonValue, Optional, Patch};
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Optional upstream extension fields; known field names cannot be overridden.
pub type AccessPolicyExtensions = BTreeMap<String, JsonValue>;
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AccessPolicyEffect {
    Allow,
    Deny,
}
#[derive(Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum AccessPolicySubjectKind {
    User,
    Group,
    Agent,
}
#[derive(Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessPolicySubject {
    #[serde(rename = "type")]
    pub kind: AccessPolicySubjectKind,
    pub id: String,
    #[serde(flatten)]
    pub extra: AccessPolicyExtensions,
}
impl AccessPolicySubject {
    pub fn new(kind: AccessPolicySubjectKind, id: impl Into<String>) -> Self {
        Self {
            kind,
            id: id.into(),
            extra: BTreeMap::new(),
        }
    }
}
/// ALL or a nonempty set of nonempty names; constraints are checked before sending.
#[derive(Clone, PartialEq, Eq)]
#[non_exhaustive]
pub enum AccessPolicySelection {
    All,
    Names(Vec<String>),
}
impl Serialize for AccessPolicySelection {
    fn serialize<S: serde::Serializer>(&self, s: S) -> std::result::Result<S::Ok, S::Error> {
        match self {
            Self::All => s.serialize_str("ALL"),
            Self::Names(v) => v.serialize(s),
        }
    }
}
impl<'de> Deserialize<'de> for AccessPolicySelection {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        match value {
            serde_json::Value::String(s) if s == "ALL" => Ok(Self::All),
            serde_json::Value::Array(v) if !v.is_empty() => v
                .into_iter()
                .map(|x| {
                    x.as_str()
                        .filter(|s| !s.is_empty())
                        .map(str::to_owned)
                        .ok_or_else(|| serde::de::Error::custom("invalid selection"))
                })
                .collect::<std::result::Result<Vec<_>, D::Error>>()
                .map(Self::Names),
            _ => Err(serde::de::Error::custom("invalid selection")),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
#[non_exhaustive]
pub enum AccessPolicyTarget {
    #[serde(rename = "mcp_server")]
    McpServer {
        server_id: String,
        tools: AccessPolicySelection,
        #[serde(flatten)]
        extra: AccessPolicyExtensions,
    },
    #[serde(rename = "provider")]
    Provider {
        provider_id: String,
        #[serde(default, skip_serializing_if = "Optional::is_unset")]
        models: Optional<AccessPolicySelection>,
        #[serde(flatten)]
        extra: AccessPolicyExtensions,
    },
}
impl AccessPolicyTarget {
    pub fn mcp_server(id: impl Into<String>, tools: AccessPolicySelection) -> Self {
        Self::McpServer {
            server_id: id.into(),
            tools,
            extra: BTreeMap::new(),
        }
    }
    pub fn provider(id: impl Into<String>) -> Self {
        Self::Provider {
            provider_id: id.into(),
            models: Optional::Unset,
            extra: BTreeMap::new(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct CreateAccessPolicy {
    pub name: String,
    pub subjects: Vec<AccessPolicySubject>,
    pub targets: Vec<AccessPolicyTarget>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub workspace_id: Optional<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub description: Patch<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub effect: Optional<AccessPolicyEffect>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub enabled: Optional<bool>,
    #[serde(flatten)]
    pub extra: AccessPolicyExtensions,
}
impl CreateAccessPolicy {
    pub fn new(
        name: impl Into<String>,
        subjects: Vec<AccessPolicySubject>,
        targets: Vec<AccessPolicyTarget>,
    ) -> Self {
        Self {
            name: name.into(),
            subjects,
            targets,
            workspace_id: Optional::Unset,
            description: Patch::Unset,
            effect: Optional::Unset,
            enabled: Optional::Unset,
            extra: BTreeMap::new(),
        }
    }
}
#[derive(Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct UpdateAccessPolicy {
    pub version: u64,
    pub subjects: Vec<AccessPolicySubject>,
    pub targets: Vec<AccessPolicyTarget>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub name: Optional<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub description: Patch<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub effect: Optional<AccessPolicyEffect>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub enabled: Optional<bool>,
    #[serde(flatten)]
    pub extra: AccessPolicyExtensions,
}
impl UpdateAccessPolicy {
    pub fn new(
        version: u64,
        subjects: Vec<AccessPolicySubject>,
        targets: Vec<AccessPolicyTarget>,
    ) -> Self {
        Self {
            version,
            subjects,
            targets,
            name: Optional::Unset,
            description: Patch::Unset,
            effect: Optional::Unset,
            enabled: Optional::Unset,
            extra: BTreeMap::new(),
        }
    }
}
#[derive(Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessPolicyListOptions {
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub workspace_id: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub effect: Optional<AccessPolicyEffect>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub enabled: Optional<bool>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub user_id: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub agent_id: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub server_id: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub search: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub created_by: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub start_time: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub end_time: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub page_size: Optional<u64>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub current_page: Optional<u64>,
}
// Optional response fields preserve absence; Patch fields also preserve explicit null.
#[derive(Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessPolicySubjectDetails {
    #[serde(rename = "type", default, skip_serializing_if = "Optional::is_unset")]
    pub kind: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub id: Optional<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub name: Patch<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub email: Patch<String>,
    #[serde(flatten)]
    pub extra: AccessPolicyExtensions,
}
crate::redaction::redacted_debug!(AccessPolicySubjectDetails);
// Subject kinds remain open response strings, never authorization decisions.
#[derive(Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessPolicyCreated {
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub id: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub object: Optional<String>,
    #[serde(flatten)]
    pub extra: AccessPolicyExtensions,
}
crate::redaction::redacted_debug!(AccessPolicyCreated);
#[derive(Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessPolicy {
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub id: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub name: Optional<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub description: Patch<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub enabled: Optional<bool>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub effect: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub subjects: Optional<Vec<AccessPolicySubjectDetails>>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub targets: Optional<Vec<AccessPolicyTargetDetails>>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub version: Optional<i64>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub status: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub workspace_id: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub organisation_id: Optional<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub created_by: Patch<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub created_at: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub last_updated_at: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub object: Optional<String>,
    #[serde(flatten)]
    pub extra: AccessPolicyExtensions,
}
crate::redaction::redacted_debug!(AccessPolicy);
#[derive(Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessPolicyListItem {
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub id: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub name: Optional<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub description: Patch<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub enabled: Optional<bool>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub effect: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub allow_all_users: Optional<bool>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub allow_all_agents: Optional<bool>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub target_server_count: Optional<i64>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub target_provider_count: Optional<i64>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub version: Optional<i64>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub status: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub workspace_id: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub organisation_id: Optional<String>,
    #[serde(default, skip_serializing_if = "Patch::is_unset")]
    pub created_by: Patch<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub created_at: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub last_updated_at: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub object: Optional<String>,
    #[serde(flatten)]
    pub extra: AccessPolicyExtensions,
}
crate::redaction::redacted_debug!(AccessPolicyListItem);
#[derive(Clone, Default, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessPolicyPage {
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub object: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub total: Optional<i64>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub data: Optional<Vec<AccessPolicyListItem>>,
    #[serde(flatten)]
    pub extra: AccessPolicyExtensions,
}
crate::redaction::redacted_debug!(AccessPolicyPage);
/// Open selector vocabulary returned by future service versions; no implicit ALL.
#[derive(Clone, Serialize, Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum AccessPolicyReturnedSelection {
    Value(String),
    Names(Vec<String>),
}
#[derive(Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessPolicyMcpServerDetails {
    #[serde(rename = "type")]
    pub kind: String,
    pub server_id: String,
    pub tools: AccessPolicyReturnedSelection,
    #[serde(flatten)]
    pub extra: AccessPolicyExtensions,
}
#[derive(Clone, Serialize, Deserialize)]
#[non_exhaustive]
pub struct AccessPolicyProviderDetails {
    #[serde(rename = "type")]
    pub kind: String,
    pub provider_id: String,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub models: Optional<AccessPolicyReturnedSelection>,
    #[serde(flatten)]
    pub extra: AccessPolicyExtensions,
}
/// Unknown target kinds are preserved as data, never treated as an authorization result.
#[derive(Clone, Serialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum AccessPolicyTargetDetails {
    McpServer(AccessPolicyMcpServerDetails),
    Provider(AccessPolicyProviderDetails),
    Unknown(JsonObject),
}
impl<'de> Deserialize<'de> for AccessPolicyTargetDetails {
    fn deserialize<D: serde::Deserializer<'de>>(d: D) -> std::result::Result<Self, D::Error> {
        let value = serde_json::Value::deserialize(d)?;
        let kind = value
            .get("type")
            .and_then(serde_json::Value::as_str)
            .ok_or_else(|| serde::de::Error::custom("invalid target kind"))?;
        match kind {
            "mcp_server" => crate::json_decode::from_value(value).map(Self::McpServer),
            "provider" => crate::json_decode::from_value(value).map(Self::Provider),
            _ => crate::json_decode::from_value(value).map(Self::Unknown),
        }
        .map_err(|_| serde::de::Error::custom("invalid target"))
    }
}
crate::redaction::redacted_debug!(
    AccessPolicySubject,
    AccessPolicySelection,
    AccessPolicyTarget,
    CreateAccessPolicy,
    UpdateAccessPolicy,
    AccessPolicyListOptions,
    AccessPolicyReturnedSelection,
    AccessPolicyMcpServerDetails,
    AccessPolicyProviderDetails,
    AccessPolicyTargetDetails
);
