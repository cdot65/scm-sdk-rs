//! Explicit, bounded live acceptance for Portkey 8e5da0e MCP `tags`.
//!
//! Creates one uniquely named disposable MCP integration and, when a workspace
//! is selected, one MCP server referencing it. The workflow verifies typed tags,
//! the tags list filter, replacement and null clearing for both fixtures, then
//! deletes every owned fixture. A create whose remote outcome is unknown is
//! reconciled through the fixture's unique tag; unresolved fixtures are reported
//! as uncertain and recorded with their run nonce in a private recovery file.
//! The sanitized JSON report records only operation names, outcomes, HTTP status
//! codes, short service codes and booleans: no identifiers, payloads or
//! credentials. Nothing runs without explicit command-line selection.
//!
//! ```text
//! cargo run --example mcp_tags_live --features gateway,reqwest-transport -- \
//!   --config <external registration.json> --expect-tsg <tenant> \
//!   --workspace <nonproduction workspace slug> --output <report.json> \
//!   --recovery-output <private recovery.json>
//! ```

/// Pure workflow rules, compiled for every feature set and unit tested offline.
mod rules {
    /// Fixture accounting: confirmed creates (including reconciled ones), creates
    /// whose remote state is still unknown, and confirmed deletions.
    #[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
    pub struct Accounting {
        pub created: u32,
        pub uncertain: u32,
        pub deleted: u32,
    }

    impl Accounting {
        pub fn cleanup_state(self) -> &'static str {
            if self.created == 0 && self.uncertain == 0 {
                "not_needed"
            } else if self.uncertain == 0 && self.deleted == self.created {
                "complete"
            } else {
                "incomplete"
            }
        }
    }

    /// The typed tags view of one response: value, explicit null, or absent.
    #[derive(Debug, PartialEq, Eq)]
    pub enum TagsView {
        Value(Vec<(String, String)>),
        Null,
        Absent,
    }

    /// Clearing is verified only by an explicit null, an empty map, or genuine
    /// omission. A nonconforming value preserved in extensions is never "cleared".
    pub fn cleared(view: &TagsView, retained_in_extensions: bool) -> bool {
        !retained_in_extensions
            && match view {
                TagsView::Null | TagsView::Absent => true,
                TagsView::Value(tags) => tags.is_empty(),
            }
    }

    /// An exact replacement is verified only by a typed value with no retained raw value.
    pub fn equals(
        view: &TagsView,
        retained_in_extensions: bool,
        expected: &[(String, String)],
    ) -> bool {
        !retained_in_extensions && matches!(view, TagsView::Value(tags) if tags == expected)
    }
}

#[cfg(test)]
mod tests {
    use super::rules::*;

    fn pairs(values: &[(&str, &str)]) -> Vec<(String, String)> {
        values
            .iter()
            .map(|(k, v)| ((*k).into(), (*v).into()))
            .collect()
    }

    #[test]
    fn clearing_accepts_only_explicit_cleared_forms() {
        assert!(cleared(&TagsView::Null, false));
        assert!(cleared(&TagsView::Value(Vec::new()), false));
        assert!(cleared(&TagsView::Absent, false));
        assert!(!cleared(&TagsView::Value(pairs(&[("env", "prod")])), false));
        // `{"count": 3}` decodes as absent with the raw value kept in extensions.
        assert!(!cleared(&TagsView::Absent, true));
        assert!(!cleared(&TagsView::Null, true));
    }

    #[test]
    fn replacement_requires_the_exact_typed_value() {
        let expected = pairs(&[("phase", "replaced")]);
        assert!(equals(&TagsView::Value(expected.clone()), false, &expected));
        assert!(!equals(
            &TagsView::Value(pairs(&[("phase", "create")])),
            false,
            &expected
        ));
        assert!(!equals(&TagsView::Value(expected.clone()), true, &expected));
        assert!(!equals(&TagsView::Absent, false, &expected));
    }

    #[test]
    fn uncertain_or_undeleted_fixtures_never_report_complete_cleanup() {
        let state = |created, uncertain, deleted| {
            Accounting {
                created,
                uncertain,
                deleted,
            }
            .cleanup_state()
        };
        assert_eq!(state(0, 0, 0), "not_needed");
        assert_eq!(state(2, 0, 2), "complete");
        assert_eq!(state(2, 0, 1), "incomplete");
        assert_eq!(state(0, 1, 0), "incomplete");
        assert_eq!(state(1, 1, 1), "incomplete");
    }
}

#[cfg(all(feature = "gateway", feature = "reqwest-transport"))]
mod live {
    use super::rules::{Accounting, TagsView, cleared, equals};
    use scm_sdk::{
        Client, Endpoint, Error, ErrorKind, Limits, OriginPolicy, RemoteOutcome, RequestOptions,
        Result, Secret, Session,
        airs::gateway::{
            AIGatewayMcpIntegrationsClient, AIGatewayMcpServersClient,
            AIGatewayWorkspaceGetOptions, AIGatewayWorkspacesClient, mcp_tags_filter, models::*,
        },
        auth::{AuthContext, Authentication, OAuthProvider},
        constants::{DEFAULT_AI_GW_ADMIN_ENDPOINT, DEFAULT_AI_GW_DATA_ENDPOINT},
        data::{Extensions, JsonValue, Patch, TypedMap},
        transport::ReqwestTransport,
    };
    use serde::{Deserialize, Serialize};
    use serde_json::{Value, json};
    use std::{fs::OpenOptions, io::Write, path::PathBuf, sync::Arc, time::Duration};

    const ISSUER: &str = "https://auth.apps.paloaltonetworks.com/oauth2/access_token";
    const MAX_STEPS: usize = 32;

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct Registration {
        mgmt_client_id: String,
        mgmt_client_secret: String,
        mgmt_tsg_id: String,
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Step {
        operation: &'static str,
        outcome: &'static str,
        #[serde(skip_serializing_if = "Option::is_none")]
        error_kind: Option<String>,
        #[serde(skip_serializing_if = "Option::is_none")]
        status: Option<u16>,
        #[serde(skip_serializing_if = "Option::is_none")]
        verified: Option<bool>,
        /// Service error code, kept only when it is a short code token.
        #[serde(skip_serializing_if = "Option::is_none")]
        service_code: Option<String>,
    }

    #[derive(Serialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Report {
        schema_version: u32,
        crate_version: &'static str,
        upstream_commit: &'static str,
        tenant_scope_matched: bool,
        server_workflow_selected: bool,
        steps: Vec<Step>,
        fixtures_created: u32,
        fixtures_uncertain: u32,
        fixtures_deleted: u32,
        cleanup_state: &'static str,
        limitations: Vec<&'static str>,
    }

    impl Report {
        fn record<T>(
            &mut self,
            operation: &'static str,
            result: &Result<T>,
            verified: Option<bool>,
        ) {
            if self.steps.len() >= MAX_STEPS {
                return;
            }
            let (outcome, error_kind, status, service_code) = match result {
                Ok(_) if verified == Some(false) => ("verification_failed", None, None, None),
                Ok(_) => ("ok", None, None, None),
                Err(error) => {
                    // Private diagnostics only: the runner keeps stderr out of the receipt.
                    if let Some(problem) =
                        error.problem().and_then(|p| serde_json::to_string(p).ok())
                    {
                        eprintln!("{operation}: problem {problem}");
                    }
                    let code = error.service_code().filter(|code| {
                        !code.is_empty()
                            && code.len() <= 64
                            && code.bytes().all(|b| {
                                b.is_ascii_alphanumeric() || matches!(b, b'_' | b'.' | b':' | b'-')
                            })
                    });
                    (
                        "error",
                        Some(format!("{:?}", error.kind())),
                        error.status(),
                        code.map(str::to_owned),
                    )
                }
            };
            // A failed request verifies nothing: the receipt schema pairs `verified`
            // only with ok/verification_failed outcomes.
            let verified = if outcome == "error" { None } else { verified };
            self.steps.push(Step {
                operation,
                outcome,
                error_kind,
                status,
                verified,
                service_code,
            });
        }

        fn finish(&mut self, accounting: Accounting) {
            self.fixtures_created = accounting.created;
            self.fixtures_uncertain = accounting.uncertain;
            self.fixtures_deleted = accounting.deleted;
            self.cleanup_state = accounting.cleanup_state();
        }
    }

    /// Private recovery record: never exported to the receipt.
    #[derive(Serialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Recovery {
        fixture: String,
        integration_id: Option<String>,
        integration_uncertain: bool,
        server_id: Option<String>,
        server_uncertain: bool,
        workspace_id: Option<String>,
    }

    struct Args {
        registration: PathBuf,
        expected_tsg: String,
        workspace: Option<String>,
        output: PathBuf,
        recovery: PathBuf,
    }

    fn args() -> Result<Args> {
        let configuration = || Error::new(ErrorKind::Configuration);
        let mut values = std::env::args().skip(1);
        let (mut registration, mut expected_tsg, mut workspace, mut output, mut recovery) =
            (None, None, None, None, None);
        while let Some(flag) = values.next() {
            match flag.as_str() {
                "--config" => registration = values.next().map(PathBuf::from),
                "--expect-tsg" => expected_tsg = values.next(),
                "--workspace" => workspace = values.next(),
                "--output" => output = values.next().map(PathBuf::from),
                "--recovery-output" => recovery = values.next().map(PathBuf::from),
                _ => return Err(configuration()),
            }
        }
        let args = Args {
            registration: registration.ok_or_else(configuration)?,
            expected_tsg: expected_tsg
                .filter(|v| !v.is_empty() && v.bytes().all(|b| b.is_ascii_digit()))
                .ok_or_else(configuration)?,
            workspace,
            output: output.ok_or_else(configuration)?,
            recovery: recovery.ok_or_else(configuration)?,
        };
        if args.output == args.registration
            || args.recovery == args.registration
            || args.recovery == args.output
        {
            return Err(configuration());
        }
        Ok(args)
    }

    fn options() -> Result<RequestOptions> {
        RequestOptions::with_timeout(Duration::from_secs(20))
    }

    fn view(tags: &Patch<TypedMap<String>>) -> TagsView {
        match tags {
            Patch::Value(tags) => {
                TagsView::Value(tags.iter().map(|(k, v)| (k.clone(), v.clone())).collect())
            }
            Patch::Null => TagsView::Null,
            Patch::Unset => TagsView::Absent,
        }
    }

    fn retained(extensions: &Extensions<JsonValue>) -> bool {
        extensions.get("tags").is_some()
    }

    fn pairs(tags: &TypedMap<String>) -> Vec<(String, String)> {
        tags.iter().map(|(k, v)| (k.clone(), v.clone())).collect()
    }

    /// Recovery ids are private: refuse symlinks, non-regular files and any
    /// existing destination readable by others; create owner-only (0600).
    #[cfg(unix)]
    fn open_private(path: &PathBuf) -> Result<std::fs::File> {
        use std::os::unix::fs::{OpenOptionsExt, PermissionsExt};
        let unsafe_mode = |mode: u32| mode & 0o077 != 0;
        if let Ok(existing) = std::fs::symlink_metadata(path)
            && (!existing.file_type().is_file() || unsafe_mode(existing.permissions().mode()))
        {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .mode(0o600)
            .open(path)
            .map_err(|_| Error::new(ErrorKind::Configuration))?;
        let mode = file
            .metadata()
            .map_err(|_| Error::new(ErrorKind::Configuration))?
            .permissions()
            .mode();
        if unsafe_mode(mode) {
            return Err(Error::new(ErrorKind::Configuration));
        }
        Ok(file)
    }

    /// Owner-only file modes are not portable here; refuse rather than write
    /// recovery identifiers with unknown permissions. Live acceptance is Linux-only.
    #[cfg(not(unix))]
    fn open_private(_path: &PathBuf) -> Result<std::fs::File> {
        Err(Error::new(ErrorKind::Configuration))
    }

    fn write_private(path: &PathBuf, value: &impl Serialize) -> Result<()> {
        let bytes =
            serde_json::to_vec_pretty(value).map_err(|_| Error::new(ErrorKind::Configuration))?;
        let mut file = open_private(path)?;
        file.write_all(&bytes)
            .map_err(|_| Error::new(ErrorKind::Configuration))
    }

    fn rows(value: &impl Serialize) -> Vec<Value> {
        serde_json::to_value(value)
            .ok()
            .and_then(|v| v["data"].as_array().cloned())
            .unwrap_or_default()
    }

    fn decode<T: serde::de::DeserializeOwned>(value: Value) -> Result<T> {
        serde_json::from_value(value).map_err(|_| Error::new(ErrorKind::Validation))
    }

    /// Offline producer fixture: an unknown-outcome integration create whose
    /// reconciliation request fails, and a server whose reconciliation finds nothing.
    #[cfg(test)]
    pub fn report_for_test() -> Value {
        let mut report = Report {
            schema_version: 2,
            crate_version: env!("CARGO_PKG_VERSION"),
            upstream_commit: "8e5da0e756bc49ee3e82ca13df693b39fa541627",
            tenant_scope_matched: true,
            server_workflow_selected: true,
            limitations: vec!["Offline producer fixture."],
            ..Report::default()
        };
        let failed: Result<()> = Err(Error::new(ErrorKind::Transport));
        report.record("mcp_integrations.create", &failed, None);
        report.record("mcp_integrations.create.reconcile", &failed, Some(false));
        report.record(
            "mcp_servers.create.reconcile",
            &Ok::<(), Error>(()),
            Some(false),
        );
        report.finish(Accounting {
            created: 0,
            uncertain: 2,
            deleted: 0,
        });
        serde_json::to_value(&report).unwrap()
    }

    pub async fn run() -> Result<()> {
        let args = args()?;
        let raw =
            std::fs::read(&args.registration).map_err(|_| Error::new(ErrorKind::Configuration))?;
        let registration: Registration =
            serde_json::from_slice(&raw).map_err(|_| Error::new(ErrorKind::Configuration))?;
        drop(raw);
        let mut report = Report {
            schema_version: 2,
            crate_version: env!("CARGO_PKG_VERSION"),
            upstream_commit: "8e5da0e756bc49ee3e82ca13df693b39fa541627",
            tenant_scope_matched: registration.mgmt_tsg_id == args.expected_tsg,
            server_workflow_selected: args.workspace.is_some(),
            cleanup_state: "not_needed",
            limitations: vec![
                "Only the recorded MCP tags steps were attempted; other MCP workflows remain live-unqualified.",
                "The fixture URL is the reserved example.com host: the service rejects unresolvable hosts as unsafe. The SDK never contacts it; service-side discovery is not asserted.",
            ],
            ..Report::default()
        };
        if !report.tenant_scope_matched {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let origins = [
            ISSUER,
            DEFAULT_AI_GW_DATA_ENDPOINT,
            DEFAULT_AI_GW_ADMIN_ENDPOINT,
        ]
        .into_iter()
        .map(Endpoint::parse)
        .collect::<Result<Vec<_>>>()?;
        let policy = Arc::new(OriginPolicy::new(origins));
        let transport = Arc::new(ReqwestTransport::new()?);
        let session = Session::new();
        let provider = Arc::new(OAuthProvider::new(
            AuthContext::new(
                Endpoint::parse(ISSUER)?,
                registration.mgmt_client_id,
                0,
                registration.mgmt_tsg_id.clone(),
            )?,
            Secret::new(registration.mgmt_client_secret)?,
            transport.clone(),
            policy.clone(),
            session.clone(),
        )?);
        let mut limits = Limits::default();
        limits.max_retries = 0;
        limits.max_concurrency = 1;
        let data = Client::builder(Endpoint::parse(DEFAULT_AI_GW_DATA_ENDPOINT)?)
            .authentication(Authentication::oauth(provider, false))
            .transport(transport)
            .destination_policy(policy)
            .session(session)
            .limits(limits)
            .build()?;
        let admin_endpoint = Endpoint::parse(DEFAULT_AI_GW_ADMIN_ENDPOINT)?;
        let admin = data.with_endpoint(admin_endpoint.clone())?;
        let integrations = AIGatewayMcpIntegrationsClient::new(admin)?;
        let servers = AIGatewayMcpServersClient::new(data.clone())?;
        let workspaces = AIGatewayWorkspacesClient::new(data, admin_endpoint)?;

        let nonce = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .map_err(|_| Error::new(ErrorKind::Configuration))?
            .as_nanos();
        let fixture = format!("sdk-tags-{:x}{:x}", nonce, std::process::id());
        let tags = |value: &str| -> TypedMap<String> {
            [
                ("sdk_fixture", fixture.clone()),
                ("phase", value.to_owned()),
            ]
            .into_iter()
            .collect()
        };
        let filter = mcp_tags_filter([("sdk_fixture", fixture.as_str())])?;
        let mut accounting = Accounting::default();
        let mut recovery = Recovery {
            fixture: fixture.clone(),
            ..Recovery::default()
        };
        // Record the nonce before the first mutation so any orphan stays findable.
        write_private(&args.recovery, &recovery)?;

        // 1. Integration with tags. The frozen request schema requires a numeric
        // organisation_id; the service accepts the tenant identifier.
        let mut create: McpIntegrationCreateRequest = decode(json!({
            "name": fixture, "organisation_id": registration.mgmt_tsg_id, "slug": fixture,
            "url": "https://example.com/mcp", "auth_type": "none", "transport": "http",
            "description": "Disposable scm-sdk MCP tags acceptance fixture"
        }))?;
        create.tags = Patch::Value(tags("create"));
        let created = integrations.create(&create, Some(&options()?)).await;
        report.record("mcp_integrations.create", &created, None);
        let mut integration_id = created
            .as_ref()
            .ok()
            .and_then(|r| r.id.clone().into_option());
        let integration_uncertain =
            matches!(&created, Err(e) if e.remote_outcome() == RemoteOutcome::Unknown);
        if created.is_ok() {
            accounting.created += 1;
        }

        // 2. The tags filter locates the fixture; it also reconciles an ambiguous create.
        let query: GatewayMcpIntegrationsClientListOptions =
            decode(json!({"tags": filter, "page_size": 10}))?;
        let listed = integrations
            .list_with_options(&query, Some(&options()?))
            .await;
        let found = listed.as_ref().ok().map(|page| {
            let rows = rows(page);
            let mine: Vec<&Value> = rows
                .iter()
                .filter(|row| row["slug"] == json!(fixture))
                .collect();
            if integration_id.is_none() {
                integration_id = mine
                    .first()
                    .and_then(|row| row["id"].as_str())
                    .map(str::to_owned);
            }
            (
                !mine.is_empty(),
                rows.iter()
                    .all(|row| row["tags"]["sdk_fixture"] == json!(fixture)),
            )
        });
        if integration_uncertain {
            if integration_id.is_some() {
                accounting.created += 1;
            } else {
                accounting.uncertain += 1;
            }
            report.record(
                "mcp_integrations.create.reconcile",
                &listed,
                Some(integration_id.is_some()),
            );
        }
        if created.is_ok() || integration_uncertain {
            report.record(
                "mcp_integrations.list_with_options.tags_filter_includes_fixture",
                &listed,
                found.map(|f| f.0),
            );
            report.record(
                "mcp_integrations.list_with_options.tags_filter_excludes_others",
                &listed,
                found.map(|f| f.1),
            );
        }
        recovery.integration_id = integration_id.clone();
        recovery.integration_uncertain = integration_uncertain && integration_id.is_none();
        write_private(&args.recovery, &recovery)?;

        if let Some(id) = integration_id.clone() {
            // 3. Read back typed tags.
            let detail = integrations.get(&id, Some(&options()?)).await;
            let verified = detail.as_ref().ok().map(|d| {
                equals(
                    &view(&d.tags),
                    retained(&d.extensions),
                    &pairs(&tags("create")),
                )
            });
            report.record("mcp_integrations.get.tags_created", &detail, verified);

            // 4. Replace, then clear with null.
            let mut update: McpIntegrationUpdateRequest = decode(json!({}))?;
            update.tags = Patch::Value(tags("replaced"));
            let replaced = integrations.update(&id, &update, Some(&options()?)).await;
            report.record("mcp_integrations.update.tags_replace", &replaced, None);
            let detail = integrations.get(&id, Some(&options()?)).await;
            let verified = detail.as_ref().ok().map(|d| {
                equals(
                    &view(&d.tags),
                    retained(&d.extensions),
                    &pairs(&tags("replaced")),
                )
            });
            report.record("mcp_integrations.get.tags_replaced", &detail, verified);
            update.tags = Patch::Null;
            let cleared_result = integrations.update(&id, &update, Some(&options()?)).await;
            report.record(
                "mcp_integrations.update.tags_null_clear",
                &cleared_result,
                None,
            );
            let detail = integrations.get(&id, Some(&options()?)).await;
            let verified = detail
                .as_ref()
                .ok()
                .map(|d| cleared(&view(&d.tags), retained(&d.extensions)));
            report.record("mcp_integrations.get.tags_cleared", &detail, verified);

            // 5. Optional server workflow in an explicitly selected workspace.
            let mut server_id = None;
            if let Some(slug) = args.workspace.as_deref() {
                let workspace = workspaces
                    .get(
                        slug,
                        &AIGatewayWorkspaceGetOptions::default(),
                        Some(&options()?),
                    )
                    .await;
                report.record("workspaces.get.selected", &workspace, None);
                if let Ok(workspace) = workspace {
                    recovery.workspace_id = Some(workspace.id.clone());
                    // A new integration must be enabled for the workspace that references it.
                    let access: McpIntegrationWorkspacesBulkUpdateRequest =
                        decode(json!({"workspaces": [{"id": workspace.id, "enabled": true}]}))?;
                    let enabled = integrations
                        .set_workspaces(&id, &access, Some(&options()?))
                        .await;
                    report.record(
                        "mcp_integrations.set_workspaces.enable_selected",
                        &enabled,
                        None,
                    );

                    let mut server: GatewayMcpServersClientCreateRequest = decode(json!({
                        "workspace_id": workspace.id, "name": fixture, "mcp_integration_id": id,
                        "description": "Disposable scm-sdk MCP tags acceptance fixture"
                    }))?;
                    server.tags = Patch::Value(tags("server"));
                    let created = servers.create(&server, Some(&options()?)).await;
                    report.record("mcp_servers.create", &created, None);
                    server_id = created
                        .as_ref()
                        .ok()
                        .and_then(|r| r.id.clone().into_option());
                    let server_uncertain =
                        matches!(&created, Err(e) if e.remote_outcome() == RemoteOutcome::Unknown);
                    if created.is_ok() {
                        accounting.created += 1;
                    }
                    let server_query: GatewayMcpServersClientListOptions = decode(
                        json!({"workspace_id": workspace.id, "tags": filter, "page_size": 10}),
                    )?;
                    if server_uncertain || (created.is_ok() && server_id.is_none()) {
                        let listed = servers.list(&server_query, Some(&options()?)).await;
                        if let Ok(page) = &listed {
                            server_id = rows(page)
                                .iter()
                                .find(|row| row["name"] == json!(fixture))
                                .and_then(|row| row["id"].as_str())
                                .map(str::to_owned);
                        }
                        if server_uncertain {
                            if server_id.is_some() {
                                accounting.created += 1;
                            } else {
                                accounting.uncertain += 1;
                            }
                            report.record(
                                "mcp_servers.create.reconcile",
                                &listed,
                                Some(server_id.is_some()),
                            );
                        }
                    }
                    recovery.server_id = server_id.clone();
                    recovery.server_uncertain = server_uncertain && server_id.is_none();
                    write_private(&args.recovery, &recovery)?;
                    if let Some(sid) = server_id.clone() {
                        let got = servers.get(&sid, Some(&options()?)).await;
                        let verified = got.as_ref().ok().map(|s| {
                            equals(
                                &view(&s.tags),
                                retained(&s.extensions),
                                &pairs(&tags("server")),
                            )
                        });
                        report.record("mcp_servers.get.tags_created", &got, verified);
                        let listed = servers.list(&server_query, Some(&options()?)).await;
                        let verified = listed
                            .as_ref()
                            .ok()
                            .map(|page| rows(page).iter().any(|row| row["id"] == json!(sid)));
                        report.record(
                            "mcp_servers.list.tags_filter_includes_fixture",
                            &listed,
                            verified,
                        );

                        let mut update: GatewayMcpServersClientUpdateRequest = decode(json!({}))?;
                        update.tags = Patch::Value(tags("server-replaced"));
                        let replaced = servers.update(&sid, &update, Some(&options()?)).await;
                        report.record("mcp_servers.update.tags_replace", &replaced, None);
                        let got = servers.get(&sid, Some(&options()?)).await;
                        let verified = got.as_ref().ok().map(|s| {
                            equals(
                                &view(&s.tags),
                                retained(&s.extensions),
                                &pairs(&tags("server-replaced")),
                            )
                        });
                        report.record("mcp_servers.get.tags_replaced", &got, verified);
                        update.tags = Patch::Null;
                        let cleared_result = servers.update(&sid, &update, Some(&options()?)).await;
                        report.record("mcp_servers.update.tags_null_clear", &cleared_result, None);
                        let got = servers.get(&sid, Some(&options()?)).await;
                        let verified = got
                            .as_ref()
                            .ok()
                            .map(|s| cleared(&view(&s.tags), retained(&s.extensions)));
                        report.record("mcp_servers.get.tags_cleared", &got, verified);
                    }
                }
            }

            // 6. Cleanup every owned fixture, server first.
            if let Some(sid) = server_id {
                let deleted = servers.delete(&sid, Some(&options()?)).await;
                report.record("mcp_servers.delete", &deleted, None);
                if deleted.is_ok() {
                    accounting.deleted += 1;
                    recovery.server_id = None;
                }
            }
            let deleted = integrations.delete(&id, Some(&options()?)).await;
            report.record("mcp_integrations.delete", &deleted, None);
            if deleted.is_ok() {
                accounting.deleted += 1;
                recovery.integration_id = None;
            }
        }
        report.finish(accounting);
        write_private(&args.recovery, &recovery)?;
        let mut file = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(&args.output)
            .map_err(|_| Error::new(ErrorKind::Configuration))?;
        file.write_all(
            serde_json::to_string_pretty(&report)
                .map_err(|_| Error::new(ErrorKind::Configuration))?
                .as_bytes(),
        )
        .map_err(|_| Error::new(ErrorKind::Configuration))?;
        if report.cleanup_state == "incomplete" {
            eprintln!(
                "cleanup_incomplete: inspect the private recovery record for the run nonce and fixture ids"
            );
        }
        Ok(())
    }
}

#[cfg(all(test, feature = "gateway", feature = "reqwest-transport"))]
mod report_tests {
    use super::live::report_for_test;

    #[test]
    fn failed_reconciliation_produces_a_receipt_valid_error_step() {
        let report = report_for_test();
        let steps = report["steps"].as_array().unwrap();
        let reconcile = steps
            .iter()
            .find(|s| s["operation"] == "mcp_integrations.create.reconcile")
            .unwrap();
        assert_eq!(reconcile["outcome"], "error");
        assert!(reconcile.get("verified").is_none());
        assert_eq!(reconcile["errorKind"], "Transport");
        let unresolved = steps
            .iter()
            .find(|s| s["operation"] == "mcp_servers.create.reconcile")
            .unwrap();
        assert_eq!(unresolved["outcome"], "verification_failed");
        assert_eq!(unresolved["verified"], false);
        assert_eq!(report["fixturesUncertain"], 2);
        assert_eq!(report["cleanupState"], "incomplete");
        // Shared with tools/test_mcp_tags_live.py, which proves the runner's receipt
        // validator accepts exactly this producer output (crateVersion is added there).
        let mut report = report;
        report.as_object_mut().unwrap().remove("crateVersion");
        let expected: serde_json::Value =
            serde_json::from_str(include_str!("mcp_tags_live_uncertain_report.json")).unwrap();
        assert_eq!(report, expected);
    }
}

#[cfg(all(feature = "gateway", feature = "reqwest-transport"))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    if live::run().await.is_err() {
        eprintln!(
            "mcp_tags_live_failed: configuration or report write failed; no credential or payload was printed"
        );
        std::process::exit(1);
    }
}

#[cfg(not(all(feature = "gateway", feature = "reqwest-transport")))]
fn main() {
    eprintln!("Enable gateway and reqwest-transport for explicit MCP tags live acceptance.");
}
