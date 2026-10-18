//! Experimental Portkey-compatible control-plane access policies (a27feb9).
//! Configure an explicit control-plane endpoint and key. SCM OAuth is not accepted.
use super::common;
use crate::{
    Client, Error, ErrorKind, RawRequest, RequestOptions, Result, data::JsonObject,
    engine::AuthKind,
};
use http::Method;
use serde::{Serialize, de::DeserializeOwned};
use serde_json::Value;
#[path = "access_policy_models.rs"]
pub mod models;
pub use models::*;
const PATH: &str = "/policies/access";
fn invalid() -> Error {
    Error::new(ErrorKind::Validation)
}
fn length(value: &Value, min: usize, max: usize) -> Result<()> {
    let n = value.as_str().ok_or_else(invalid)?.chars().count();
    if (min..=max).contains(&n) {
        Ok(())
    } else {
        Err(invalid())
    }
}
fn extensions(extra: &AccessPolicyExtensions, known: &[&str]) -> Result<()> {
    if extra.keys().any(|k| known.contains(&k.as_str())) {
        Err(invalid())
    } else {
        Ok(())
    }
}
fn target_extensions(targets: &[AccessPolicyTarget]) -> Result<()> {
    for target in targets {
        match target {
            AccessPolicyTarget::McpServer { extra, .. } => {
                extensions(extra, &["type", "server_id", "tools"])?
            }
            AccessPolicyTarget::Provider { extra, .. } => {
                extensions(extra, &["type", "provider_id", "models"])?
            }
        }
    }
    Ok(())
}
fn subject_extensions(subjects: &[AccessPolicySubject]) -> Result<()> {
    for subject in subjects {
        extensions(&subject.extra, &["type", "id"])?;
    }
    Ok(())
}
fn selection(value: &Value) -> Result<()> {
    match value {
        Value::String(v) if v == "ALL" => Ok(()),
        Value::Array(v) if !v.is_empty() => {
            for name in v {
                length(name, 1, usize::MAX)?;
            }
            Ok(())
        }
        _ => Err(invalid()),
    }
}
fn validate(value: &Value, update: bool) -> Result<()> {
    if let Some(name) = value.get("name") {
        length(name, 1, 64)?;
    }
    if let Some(description) = value.get("description").filter(|v| !v.is_null()) {
        length(description, 0, 256)?;
    }
    if update
        && value
            .get("version")
            .and_then(Value::as_u64)
            .is_none_or(|v| v < 1)
    {
        return Err(invalid());
    }
    let subjects = value
        .get("subjects")
        .and_then(Value::as_array)
        .filter(|v| !v.is_empty())
        .ok_or_else(invalid)?;
    let targets = value
        .get("targets")
        .and_then(Value::as_array)
        .filter(|v| !v.is_empty())
        .ok_or_else(invalid)?;
    let mut specific = false;
    let mut all_users = false;
    let mut all_agents = false;
    for subject in subjects {
        let kind = subject
            .get("type")
            .and_then(Value::as_str)
            .ok_or_else(invalid)?;
        let id = subject
            .get("id")
            .and_then(Value::as_str)
            .ok_or_else(invalid)?;
        if id == "ANY" {
            match kind {
                "user" => all_users = true,
                "agent" => all_agents = true,
                _ => return Err(invalid()),
            }
        } else {
            specific = true;
        }
    }
    let effect = value.get("effect").and_then(Value::as_str);
    let allow =
        effect == Some("allow") || (!update && effect.is_none() && !(all_users && all_agents));
    if allow && !specific {
        return Err(invalid());
    }
    for target in targets {
        if target.get("type").and_then(Value::as_str) == Some("mcp_server") {
            selection(target.get("tools").ok_or_else(invalid)?)?;
        } else if let Some(models) = target.get("models") {
            selection(models)?;
        }
    }
    Ok(())
}
/// Explicit key-authenticated control-plane client. New calls are live-unverified.
///
/// The caller supplies the endpoint; there is no automatic Portkey or SCM origin.
/// Mutations do not replay, and a 409 conflict is returned without fetching or merging.
#[derive(Clone, Debug)]
pub struct AccessPoliciesClient {
    client: Client,
}
impl AccessPoliciesClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::Runtime)?;
        Ok(Self { client })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    async fn input<T: Serialize>(
        &self,
        body: &T,
        options: &RequestOptions,
        validate: impl FnOnce(&Value) -> Result<()>,
    ) -> Result<bytes::Bytes> {
        options
            .run(&self.client.session(), async {
                let bytes =
                    crate::engine::serialize_bounded(body, self.client.limits().max_request_bytes)?;
                let value = crate::json_decode::from_slice(&bytes).map_err(|_| invalid())?;
                validate(&value)?;
                Ok(bytes)
            })
            .await
    }
    async fn execute<R: DeserializeOwned>(
        &self,
        request: RawRequest,
        options: &RequestOptions,
    ) -> Result<R> {
        self.client.send_json(request, Some(options)).await
    }
    /// Create a policy; omitted effect/enabled values are left for the service.
    pub async fn create(
        &self,
        body: &CreateAccessPolicy,
        options: Option<&RequestOptions>,
    ) -> Result<AccessPolicyCreated> {
        const OP: &str = "airs.gateway.access_policies.create";
        async {
            let options = self.client.operation_options(options)?;
            let bytes = self
                .input(body, &options, |v| {
                    extensions(
                        &body.extra,
                        &[
                            "name",
                            "subjects",
                            "targets",
                            "workspace_id",
                            "description",
                            "effect",
                            "enabled",
                        ],
                    )?;
                    subject_extensions(&body.subjects)?;
                    target_extensions(&body.targets)?;
                    validate(v, false)
                })
                .await?;
            self.execute(
                RawRequest::new(Method::POST, PATH)
                    .named(OP)
                    .body(bytes, "application/json")?,
                &options,
            )
            .await
        }
        .await
        .map_err(|e: Error| e.with_operation(OP))
    }
    /// List one explicitly numbered page; absent pagination values remain absent.
    pub async fn list(
        &self,
        query: &AccessPolicyListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<AccessPolicyPage> {
        const OP: &str = "airs.gateway.access_policies.list";
        async {
            let options = self.client.operation_options(options)?;
            let query = options
                .run(&self.client.session(), async {
                    let bytes =
                        crate::engine::serialize_bounded(query, crate::config::MAX_URL_BYTES)?;
                    let value: Value =
                        crate::json_decode::from_slice(&bytes).map_err(|_| invalid())?;
                    for (name, max) in [
                        ("user_id", 36),
                        ("agent_id", 36),
                        ("server_id", 100),
                        ("search", 64),
                        ("created_by", 255),
                    ] {
                        if let Some(v) = value.get(name) {
                            length(v, 0, max)?;
                        }
                    }
                    for name in ["start_time", "end_time"] {
                        if let Some(v) = value.get(name) {
                            time::OffsetDateTime::parse(
                                v.as_str().ok_or_else(invalid)?,
                                &time::format_description::well_known::Rfc3339,
                            )
                            .map_err(|_| invalid())?;
                        }
                    }
                    let mut pairs = Vec::new();
                    for name in [
                        "workspace_id",
                        "effect",
                        "enabled",
                        "user_id",
                        "agent_id",
                        "server_id",
                        "search",
                        "created_by",
                        "start_time",
                        "end_time",
                        "page_size",
                        "current_page",
                    ] {
                        if let Some(v) = value.get(name) {
                            let text = match v {
                                Value::String(s) => s.clone(),
                                Value::Bool(b) => b.to_string(),
                                Value::Number(n) => n.to_string(),
                                _ => return Err(invalid()),
                            };
                            pairs.push((name.to_owned(), text));
                        }
                    }
                    Ok(pairs)
                })
                .await?;
            self.execute(RawRequest::read(PATH).named(OP).query(query), &options)
                .await
        }
        .await
        .map_err(|e: Error| e.with_operation(OP))
    }
    pub async fn get(&self, id: &str, options: Option<&RequestOptions>) -> Result<AccessPolicy> {
        const OP: &str = "airs.gateway.access_policies.get";
        async {
            let options = self.client.operation_options(options)?;
            options
                .run(&self.client.session(), async { common::uuid(id) })
                .await?;
            self.execute(RawRequest::read(PATH).segment(id)?.named(OP), &options)
                .await
        }
        .await
        .map_err(|e: Error| e.with_operation(OP))
    }
    /// Replace subjects/targets at the given version; omitted fields retain stored values.
    pub async fn update(
        &self,
        id: &str,
        body: &UpdateAccessPolicy,
        options: Option<&RequestOptions>,
    ) -> Result<JsonObject> {
        const OP: &str = "airs.gateway.access_policies.update";
        async {
            let options = self.client.operation_options(options)?;
            let bytes = self
                .input(body, &options, |v| {
                    common::uuid(id)?;
                    extensions(
                        &body.extra,
                        &[
                            "version",
                            "name",
                            "subjects",
                            "targets",
                            "description",
                            "effect",
                            "enabled",
                        ],
                    )?;
                    subject_extensions(&body.subjects)?;
                    target_extensions(&body.targets)?;
                    validate(v, true)
                })
                .await?;
            self.execute(
                RawRequest::new(Method::PUT, PATH)
                    .segment(id)?
                    .named(OP)
                    .body(bytes, "application/json")?,
                &options,
            )
            .await
        }
        .await
        .map_err(|e: Error| e.with_operation(OP))
    }
    pub async fn delete(&self, id: &str, options: Option<&RequestOptions>) -> Result<JsonObject> {
        const OP: &str = "airs.gateway.access_policies.delete";
        async {
            let options = self.client.operation_options(options)?;
            options
                .run(&self.client.session(), async { common::uuid(id) })
                .await?;
            self.execute(
                RawRequest::new(Method::DELETE, PATH).segment(id)?.named(OP),
                &options,
            )
            .await
        }
        .await
        .map_err(|e: Error| e.with_operation(OP))
    }
}
