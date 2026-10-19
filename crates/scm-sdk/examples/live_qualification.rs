//! Explicit, bounded nonproduction qualification. This application never runs
//! as part of ordinary tests and never prints credentials or API payloads.
#[cfg(all(
    feature = "reqwest-transport",
    feature = "scan",
    feature = "management",
    feature = "model-security",
    feature = "red-team",
    feature = "gateway",
    feature = "agentguard"
))]
mod qualification {
    use futures_util::future::BoxFuture;
    use scm_sdk::{
        Client, Endpoint, Error, ErrorKind, Limits, OriginPolicy, RemoteOutcome, RequestOptions,
        Result, Secret, Session,
        airs::{
            agentguard::{AgentGuardClient, models::*},
            gateway::{
                AIGatewayClient, AIGatewayPlane, AIGatewayWorkspaceGetOptions,
                AIGatewayWorkspaceListOptions, AIGatewayWorkspaceProvisionOptions,
                AIGatewayWorkspaceStatus, GatewayWorkspaceProvisionRequest, ProvisionCleanup,
                ProvisionWorkspaceState,
            },
            management::{ManagementClient, PaginationOptions},
            model_security::{
                ModelSecurityClient, ModelSecurityModelListOptions, ModelSecurityRuleListOptions,
                ModelSecurityScanListOptions,
            },
            red_team::{RedTeamClient, TargetListOptions},
            scan::{Content, Region, Scanner, SyncScanOptions, models::*},
        },
        auth::{AuthContext, Authentication, OAuthProvider},
        data::{FiniteNumber, Optional},
        shared::iam::{IamScopesClient, models::*},
        transport::{ReqwestTransport, Transport, TransportRequest, TransportResponse},
    };
    use serde::{Deserialize, Serialize};
    use std::{
        fs::{self, OpenOptions},
        io::{Read, Write},
        path::{Path, PathBuf},
        sync::{
            Arc, Mutex,
            atomic::{AtomicUsize, Ordering},
        },
        time::Duration,
    };
    use zeroize::Zeroizing;

    const ISSUER: &str = "https://auth.apps.paloaltonetworks.com/oauth2/access_token";
    const MANAGEMENT: &str = "https://api.sase.paloaltonetworks.com/aisec";
    const MODEL_DATA: &str = "https://api.sase.paloaltonetworks.com/aims/data";
    const MODEL_MANAGEMENT: &str = "https://api.sase.paloaltonetworks.com/aims/mgmt";
    const RED_DATA: &str = "https://api.sase.paloaltonetworks.com/ai-red-teaming/data";
    const RED_MANAGEMENT: &str = "https://api.sase.paloaltonetworks.com/ai-red-teaming/mgmt";
    const RED_BROKER: &str = "https://api.sase.paloaltonetworks.com/ai-red-teaming/network-broker";
    const GATEWAY_DATA: &str = "https://api.strata.paloaltonetworks.com/ai_gw/v2";
    const GATEWAY_ADMIN: &str = "https://api.strata.paloaltonetworks.com/ai_gw/admin/v2";
    const IAM: &str = "https://api.strata.paloaltonetworks.com/iam/v1";
    const AGENT_DATA: &str = "https://api.apps.paloaltonetworks.com/aiag/data";
    const AGENT_MANAGEMENT: &str = "https://api.apps.paloaltonetworks.com/aiag/mgmt";

    #[derive(Deserialize)]
    #[serde(rename_all = "camelCase", deny_unknown_fields)]
    struct Registration {
        mgmt_client_id: String,
        #[serde(deserialize_with = "secret")]
        mgmt_client_secret: Secret,
        mgmt_tsg_id: String,
    }
    fn secret<'de, D: serde::Deserializer<'de>>(de: D) -> std::result::Result<Secret, D::Error> {
        Secret::new(String::deserialize(de)?)
            .map_err(|_| serde::de::Error::custom("invalid credential"))
    }
    struct Args {
        registration: PathBuf,
        expected_tsg: String,
        output: PathBuf,
        recovery: Option<PathBuf>,
        mutations: bool,
        read_only: bool,
    }
    impl Args {
        fn parse() -> Result<Self> {
            let mut args = std::env::args_os().skip(1);
            let mut registration = None;
            let mut expected_tsg = None;
            let mut output = None;
            let mut recovery = None;
            let mut mutations = false;
            let mut read_only = false;
            while let Some(flag) = args.next() {
                match flag.to_str() {
                    Some("--config") => registration = args.next().map(PathBuf::from),
                    Some("--expect-tsg") => {
                        expected_tsg = args.next().and_then(|v| v.into_string().ok());
                    }
                    Some("--output") => output = args.next().map(PathBuf::from),
                    Some("--recovery-output") => recovery = args.next().map(PathBuf::from),
                    Some("--mutations") => mutations = true,
                    Some("--read-only") => read_only = true,
                    _ => return Err(Error::new(ErrorKind::Configuration)),
                }
            }
            let config = Self {
                registration: registration.ok_or_else(|| Error::new(ErrorKind::Configuration))?,
                expected_tsg: expected_tsg.ok_or_else(|| Error::new(ErrorKind::Configuration))?,
                output: output.ok_or_else(|| Error::new(ErrorKind::Configuration))?,
                recovery,
                mutations,
                read_only,
            };
            if config.mutations && config.read_only
                || config.expected_tsg.is_empty()
                || !config.expected_tsg.bytes().all(|b| b.is_ascii_digit())
                || config.recovery.is_none()
                || config
                    .recovery
                    .as_ref()
                    .is_some_and(|v| v == &config.output)
                || config.output == config.registration
                || config
                    .recovery
                    .as_ref()
                    .is_some_and(|v| v == &config.registration)
            {
                return Err(Error::new(ErrorKind::Configuration));
            }
            Ok(config)
        }
    }

    // Closed protocol vocabulary only. service_code() also accepts arbitrary
    // shape-valid strings, which are not safe to copy into a public receipt.
    const REJECTION_CODES: &[&str] = &[
        "AISEC_SERVER_SIDE_ERROR",
        "AISEC_CLIENT_SIDE_ERROR",
        "AISEC_USER_REQUEST_PAYLOAD_ERROR",
        "AISEC_MISSING_VARIABLE",
        "AISEC_SDK_ERROR",
        "AISEC_OAUTH_ERROR",
        "AISEC_RESPONSE_VALIDATION",
        "invalid_request",
        "invalid_client",
        "invalid_grant",
        "unauthorized_client",
        "unsupported_grant_type",
        "invalid_scope",
    ];

    #[derive(Serialize)]
    struct RejectionDetails {
        code: Option<&'static str>,
        fields: Vec<&'static str>,
    }
    impl RejectionDetails {
        fn from_parts(code: Option<&str>, problem: Option<&scm_sdk::ProblemDetails>) -> Self {
            let code = code.and_then(|value| {
                REJECTION_CODES
                    .iter()
                    .copied()
                    .find(|known| *known == value)
            });
            let mut fields = Vec::new();
            for field in problem.and_then(|value| value.errors()).unwrap_or_default() {
                let name = field.property();
                if name != "[withheld]" && !fields.contains(&name) && fields.len() < 20 {
                    fields.push(name);
                }
            }
            Self { code, fields }
        }
    }
    fn rejection_details(error: &Error) -> Option<RejectionDetails> {
        matches!(error.status(), Some(400 | 405 | 409 | 422))
            .then(|| RejectionDetails::from_parts(error.service_code(), error.problem()))
    }

    #[derive(Serialize)]
    #[serde(rename_all = "camelCase")]
    struct Probe {
        operation: &'static str,
        classification: &'static str,
        status: Option<u16>,
        error_kind: Option<&'static str>,
        remote_outcome: Option<&'static str>,
        workflow_accepted: bool,
        #[serde(skip_serializing_if = "Option::is_none")]
        rejection_details: Option<RejectionDetails>,
    }
    #[derive(Serialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Report {
        schema_version: u32,
        tenant_scope_matched: bool,
        client: &'static str,
        crate_version: &'static str,
        automatic_retries: u32,
        maximum_transport_attempts: usize,
        per_operation_timeout_seconds: u32,
        scan_authentication: Option<&'static str>,
        mutations_enabled: bool,
        cleanup_state: &'static str,
        probes: Vec<Probe>,
        cleanup: Vec<Probe>,
        service_requests: usize,
        token_exchanges: usize,
        limitations: Vec<&'static str>,
    }
    #[derive(Serialize, Default)]
    #[serde(rename_all = "camelCase")]
    struct Recovery {
        snapshot_sequence: u64,
        scope_uncertain: bool,
        scope_name: Option<String>,
        scope_id: Option<String>,
        workspace_id: Option<String>,
        workspace_slug: Option<String>,
        scan_ids: Vec<String>,
        report_ids: Vec<String>,
        scope_deleted: bool,
        workspace_archived: bool,
        workspace_uncertain: bool,
    }
    struct Wire {
        token_exchange: bool,
        status: Option<u16>,
    }
    type WireLog = Arc<Mutex<Vec<Wire>>>;
    #[derive(Clone)]
    struct AuditTransport {
        inner: ReqwestTransport,
        wire: WireLog,
        attempts: Arc<AtomicUsize>,
        read_only: bool,
    }
    impl Transport for AuditTransport {
        fn send<'a>(
            &'a self,
            request: TransportRequest,
        ) -> BoxFuture<'a, Result<TransportResponse>> {
            let token = request.url().host_str() == Some("auth.apps.paloaltonetworks.com");
            Box::pin(async move {
                if self.read_only
                    && !read_only_request_allowed(request.method().as_str(), request.url())
                {
                    return Err(Error::new(ErrorKind::Configuration));
                }
                if self.attempts.fetch_add(1, Ordering::Relaxed)
                    >= if self.read_only { 5 } else { 80 }
                {
                    return Err(Error::new(ErrorKind::Limit));
                }
                let result = self.inner.send(request).await;
                if let Ok(mut wire) = self.wire.lock()
                    && wire.len() < 80
                {
                    wire.push(Wire {
                        token_exchange: token,
                        status: result.as_ref().ok().map(|v| v.status().as_u16()),
                    });
                }
                result
            })
        }
    }
    fn read_only_request_allowed(method: &str, url: &url::Url) -> bool {
        (url.as_str() == ISSUER && method == "POST")
            || (url.scheme() == "https"
                && url.host_str() == Some("api.sase.paloaltonetworks.com")
                && url.port_or_known_default() == Some(443)
                && url.path() == "/ai-red-teaming/mgmt/v1/target"
                && method == "GET"
                && url.query_pairs().count() == 2
                && url
                    .query_pairs()
                    .any(|(key, value)| key == "limit" && value == "1")
                && url.query_pairs().any(|(key, value)| {
                    key == "profiling_status"
                        && PROFILING_PROBES.iter().any(|(filter, _)| *filter == value)
                }))
    }
    const PROFILING_PROBES: [(&str, &str); 4] = [
        ("NOT_STARTED", "red_team.targets.list.profiling_not_started"),
        ("IN_PROGRESS", "red_team.targets.list.profiling_in_progress"),
        ("COMPLETED", "red_team.targets.list.profiling_completed"),
        ("FAILED", "red_team.targets.list.profiling_failed"),
    ];
    const READ_ONLY_LIMITATIONS: [&str; 3] = [
        "Read-only qualification attempts OAuth and four single-page target lists with limit 1; all other workflows remain live-unqualified.",
        "Filter acceptance means HTTP success and native typed decoding only; filtering effectiveness is not established, including for empty or ambiguous results.",
        "Vertex AI, client_session and audio workflows remain experimental and live-unverified without provider credentials, fixtures and execution prerequisites.",
    ];
    fn finish_read_only(args: &Args, audit: &AuditTransport, report: &mut Report) -> Result<()> {
        let wire = audit
            .wire
            .lock()
            .map_err(|_| Error::new(ErrorKind::Configuration))?;
        report.token_exchanges = wire.iter().filter(|v| v.token_exchange).count();
        report.service_requests = wire.iter().filter(|v| !v.token_exchange).count();
        drop(wire);
        write_private(&args.output, report)
    }
    impl AuditTransport {
        fn latest_status(&self) -> Option<u16> {
            self.wire
                .lock()
                .ok()
                .and_then(|v| v.last().and_then(|v| v.status))
        }
    }
    fn options() -> Result<RequestOptions> {
        RequestOptions::with_timeout(Duration::from_secs(15))?.with_retries(0)
    }
    fn failure_class(error: &Error) -> &'static str {
        match error.status() {
            Some(401) => "authentication_prerequisite",
            Some(403) => "permission_prerequisite",
            Some(404) => "not_found_or_unavailable",
            Some(429) => "rate_limit_prerequisite",
            // A rejected submitted request does not establish the cause. A
            // native request-shape defect remains possible until reconciled.
            Some(400 | 405 | 409 | 422) => "service_contract_rejection",
            Some(500..=599) => "upstream_failure",
            _ => match error.kind() {
                ErrorKind::Transport | ErrorKind::Io | ErrorKind::Deadline => {
                    "network_prerequisite"
                }
                ErrorKind::Decode => "service_contract_mismatch",
                _ => "unexpected_client_failure",
            },
        }
    }
    fn error_kind(kind: ErrorKind) -> &'static str {
        match kind {
            ErrorKind::Configuration => "configuration",
            ErrorKind::Validation => "validation",
            ErrorKind::Destination => "destination",
            ErrorKind::Authentication => "authentication",
            ErrorKind::Transport => "transport",
            ErrorKind::Http => "http",
            ErrorKind::Decode => "decode",
            ErrorKind::Protocol => "protocol",
            ErrorKind::Limit => "limit",
            ErrorKind::Cancelled => "cancelled",
            ErrorKind::Deadline => "deadline",
            ErrorKind::Io => "io",
            ErrorKind::Pagination => "pagination",
            _ => "unknown",
        }
    }
    fn outcome(value: RemoteOutcome) -> &'static str {
        match value {
            RemoteOutcome::NotSent => "not_sent",
            RemoteOutcome::Rejected => "rejected",
            RemoteOutcome::Unknown => "unknown",
            _ => "unknown",
        }
    }
    fn record<T>(
        rows: &mut Vec<Probe>,
        audit: &AuditTransport,
        operation: &'static str,
        result: Result<T>,
    ) -> Option<T> {
        let rejection_details = result.as_ref().err().and_then(rejection_details);
        let (classification, status, error_kind, remote_outcome, accepted) = match &result {
            Ok(_) => ("verified", audit.latest_status(), None, None, true),
            Err(error) => (
                failure_class(error),
                error.status(),
                Some(error_kind(error.kind())),
                Some(outcome(error.remote_outcome())),
                false,
            ),
        };
        rows.push(Probe {
            operation,
            classification,
            status,
            error_kind,
            remote_outcome,
            workflow_accepted: accepted,
            rejection_details,
        });
        result.ok()
    }
    fn skipped(rows: &mut Vec<Probe>, operation: &'static str) {
        rows.push(Probe {
            operation,
            classification: "not_attempted_missing_prerequisite",
            status: None,
            error_kind: None,
            remote_outcome: None,
            workflow_accepted: false,
            rejection_details: None,
        });
    }
    fn require_receipt(rows: &mut [Probe], matches: bool) {
        if !matches && let Some(last) = rows.last_mut() {
            last.workflow_accepted = false;
            last.classification = "incomplete_or_uncorrelated_receipt";
        }
    }
    fn scan_results_match(rows: &[ScanIdResult], scan: &str, report: Option<&str>) -> bool {
        rows.iter().any(|row| {
            row.scan_id
                .as_ref()
                .into_option()
                .is_some_and(|id| id == scan)
                && row.result.as_ref().into_option().is_some_and(|result| {
                    result.scan_id == scan
                        && report.is_none_or(|id| result.report_id == id)
                        && !result.report_id.is_empty()
                        && !result.error
                        && !result.timeout
                        && result.errors.is_empty()
                })
        })
    }
    fn report_matches(rows: &[ThreatScanReport], scan: &str, report: &str) -> bool {
        rows.iter().any(|row| {
            row.scan_id
                .as_ref()
                .into_option()
                .is_some_and(|id| id == scan)
                && row
                    .report_id
                    .as_ref()
                    .into_option()
                    .is_some_and(|id| id == report)
                && !row.detection_results.is_unset()
        })
    }
    fn scope_resources(
        scope: IamScope,
        omit_workspace: Option<&str>,
    ) -> Result<Vec<IamScopeResourceInput>> {
        scope
            .resources
            .into_iter()
            .filter(|r| {
                !omit_workspace
                    .is_some_and(|slug| r.resource_type == "workspace" && r.resource_id == slug)
            })
            .map(|r| {
                let mut builder = IamScopeResourceInput::builder()
                    .resource_type(r.resource_type)
                    .resource_id(r.resource_id)
                    .metadata(r.metadata);
                for (key, value) in r.extensions.iter() {
                    builder = builder.extension(key.clone(), value.clone());
                }
                builder.build()
            })
            .collect()
    }
    fn confirmed_absent(rows: &mut Vec<Probe>, operation: &'static str, status: Option<u16>) {
        rows.push(Probe {
            operation,
            classification: "verified_absent",
            status,
            error_kind: None,
            remote_outcome: None,
            workflow_accepted: true,
            rejection_details: None,
        });
    }
    fn load_registration(path: &Path) -> Result<Registration> {
        let file = fs::File::open(path).map_err(|_| Error::new(ErrorKind::Configuration))?;
        let mut bytes = Zeroizing::new(Vec::new());
        file.take(65_537)
            .read_to_end(&mut bytes)
            .map_err(|_| Error::new(ErrorKind::Configuration))?;
        if bytes.len() > 65_536 {
            return Err(Error::new(ErrorKind::Limit));
        }
        serde_json::from_slice(&bytes).map_err(|_| Error::new(ErrorKind::Configuration))
    }
    fn write_private(path: &Path, value: &impl Serialize) -> Result<()> {
        let bytes = Zeroizing::new(
            serde_json::to_vec_pretty(value).map_err(|_| Error::new(ErrorKind::Configuration))?,
        );
        // Refuse symlinks and existing output files: a caller selects new paths.
        let mut options = OpenOptions::new();
        options.write(true).create_new(true);
        #[cfg(unix)]
        {
            use std::os::unix::fs::OpenOptionsExt;
            options.mode(0o600);
        }
        let mut file = options.open(path).map_err(|_| Error::new(ErrorKind::Io))?;
        file.write_all(&bytes)
            .map_err(|_| Error::new(ErrorKind::Io))?;
        file.sync_all().map_err(|_| Error::new(ErrorKind::Io))
    }
    fn persist_recovery(path: Option<&Path>, recovery: &mut Recovery) -> Result<()> {
        if let Some(path) = path {
            // Each update is a new uniquely owned file, preserving earlier facts.
            recovery.snapshot_sequence = recovery
                .snapshot_sequence
                .checked_add(1)
                .ok_or_else(|| Error::new(ErrorKind::Limit))?;
            let counter = recovery.snapshot_sequence;
            let mut name = path.as_os_str().to_owned();
            name.push(format!(".{counter}.json"));
            write_private(Path::new(&name), recovery)?;
        }
        Ok(())
    }
    fn unique_name() -> Result<String> {
        let mut nonce = [0u8; 16];
        getrandom::fill(&mut nonce).map_err(|_| Error::new(ErrorKind::Io))?;
        use std::fmt::Write;
        let mut value = String::from("sdkq_");
        for byte in nonce {
            write!(&mut value, "{byte:02x}").map_err(|_| Error::new(ErrorKind::Io))?;
        }
        Ok(value)
    }

    pub async fn run() -> Result<()> {
        let args = Args::parse()?;
        let registration = load_registration(&args.registration)?;
        if registration.mgmt_tsg_id != args.expected_tsg {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let origins = [
            ISSUER,
            MANAGEMENT,
            MODEL_DATA,
            GATEWAY_DATA,
            AGENT_DATA,
            scm_sdk::constants::DEFAULT_DLP_ENDPOINT,
            Region::Us.endpoint(),
        ]
        .into_iter()
        .map(Endpoint::parse)
        .collect::<Result<Vec<_>>>()?;
        let policy = Arc::new(OriginPolicy::new(origins));
        let audit = AuditTransport {
            inner: ReqwestTransport::new()?,
            wire: Arc::new(Mutex::new(Vec::new())),
            attempts: Arc::new(AtomicUsize::new(0)),
            read_only: args.read_only,
        };
        let session = Session::new();
        let provider = Arc::new(OAuthProvider::new(
            AuthContext::new(
                Endpoint::parse(ISSUER)?,
                registration.mgmt_client_id,
                0,
                registration.mgmt_tsg_id,
            )?,
            registration.mgmt_client_secret,
            Arc::new(audit.clone()),
            policy.clone(),
            session.clone(),
        )?);
        let mut limits = Limits::default();
        limits.operation_timeout = Duration::from_secs(15);
        limits.max_response_bytes = 2 * 1024 * 1024;
        limits.max_request_bytes = 256 * 1024;
        limits.max_retries = 0;
        limits.max_concurrency = 2;
        let client = Client::builder(Endpoint::parse(MANAGEMENT)?)
            .authentication(Authentication::oauth(provider.clone(), false))
            .transport(Arc::new(audit.clone()))
            .destination_policy(policy)
            .session(session)
            .limits(limits)
            .build()?;
        let mut report = Report {
            schema_version: if args.read_only { 2 } else { 1 },
            tenant_scope_matched: true,
            client: "native_scm_sdk",
            crate_version: env!("CARGO_PKG_VERSION"),
            automatic_retries: 0,
            maximum_transport_attempts: if args.read_only { 5 } else { 80 },
            per_operation_timeout_seconds: 15,
            mutations_enabled: args.mutations,
            cleanup_state: "not_needed",
            limitations: vec![
                "Only the explicitly recorded workflows were attempted; remaining methods are live-unqualified.",
                "Synthetic scans retain server audit records; the frozen scan API has no deletion method.",
                "Workspace deletion is archival; permanent deletion is unavailable in the frozen API.",
            ],
            ..Report::default()
        };
        if args.read_only {
            report.limitations = READ_ONLY_LIMITATIONS.to_vec();
        }
        let mut recovery = Recovery::default();
        let token = record(
            &mut report.probes,
            &audit,
            "oauth.get_token",
            provider.get_token(Some(&options()?)).await,
        );
        if args.read_only {
            if token.is_none() {
                for (_, operation) in PROFILING_PROBES {
                    skipped(&mut report.probes, operation);
                }
            } else {
                let red = RedTeamClient::new(
                    client.with_endpoint(Endpoint::parse(RED_DATA)?)?,
                    Endpoint::parse(RED_MANAGEMENT)?,
                    Endpoint::parse(RED_BROKER)?,
                )?;
                for (filter, operation) in PROFILING_PROBES {
                    let mut listing = TargetListOptions::default();
                    listing.limit = Some(1);
                    listing.profiling_status = Some(filter.to_owned());
                    record(
                        &mut report.probes,
                        &audit,
                        operation,
                        red.targets()
                            .list(&listing, Some(&options()?.without_response_defaults()))
                            .await,
                    );
                }
            }
            return finish_read_only(&args, &audit, &mut report);
        }
        if token.is_none() {
            let wire = audit
                .wire
                .lock()
                .map_err(|_| Error::new(ErrorKind::Configuration))?;
            report.token_exchanges = wire.iter().filter(|v| v.token_exchange).count();
            report.service_requests = wire.iter().filter(|v| !v.token_exchange).count();
            drop(wire);
            write_private(&args.output, &report)?;
            return Ok(());
        }
        let management = ManagementClient::new(client.clone())?;
        let mut page = PaginationOptions::default();
        page.limit = 1;
        page.latest = Some(true);
        let profiles = record(
            &mut report.probes,
            &audit,
            "management.profiles.list_for_token",
            management
                .profiles()
                .list_for_token(page, Some(&options()?))
                .await,
        );
        let mut key_page = page;
        key_page.limit = 100;
        let keys = record(
            &mut report.probes,
            &audit,
            "management.api_keys.list_for_token",
            management
                .api_keys()
                .list_for_token(key_page, Some(&options()?))
                .await,
        );
        record(
            &mut report.probes,
            &audit,
            "management.profiles.list",
            management.profiles().list(page, Some(&options()?)).await,
        );
        let dlp = management.dlp(Endpoint::parse(scm_sdk::constants::DEFAULT_DLP_ENDPOINT)?)?;
        record(
            &mut report.probes,
            &audit,
            "dlp.data_profiles.list",
            dlp.data_profiles()
                .list(
                    &scm_sdk::dlp::DataProfileListParams::new().size(1),
                    Some(&options()?),
                )
                .await,
        );

        let model = ModelSecurityClient::new(
            client.clone(),
            Endpoint::parse(MODEL_DATA)?,
            Endpoint::parse(MODEL_MANAGEMENT)?,
        )?;
        let mut model_page = ModelSecurityModelListOptions::default();
        model_page.limit = Some(1);
        record(
            &mut report.probes,
            &audit,
            "model_security.models.list_models",
            model
                .models()
                .list_models(&model_page, Some(&options()?))
                .await,
        );
        let mut scan_page = ModelSecurityScanListOptions::default();
        scan_page.limit = Some(1);
        record(
            &mut report.probes,
            &audit,
            "model_security.scans.list",
            model.scans().list(&scan_page, Some(&options()?)).await,
        );
        let mut rule_page = ModelSecurityRuleListOptions::default();
        rule_page.limit = Some(1);
        record(
            &mut report.probes,
            &audit,
            "model_security.rules.list",
            model
                .security_rules()
                .list(&rule_page, Some(&options()?))
                .await,
        );

        let red = RedTeamClient::new(
            client.with_endpoint(Endpoint::parse(RED_DATA)?)?,
            Endpoint::parse(RED_MANAGEMENT)?,
            Endpoint::parse(RED_BROKER)?,
        )?;
        record(
            &mut report.probes,
            &audit,
            "red_team.eula.get_status",
            red.eula().get_status(Some(&options()?)).await,
        );
        record(
            &mut report.probes,
            &audit,
            "red_team.get_scan_metadata",
            red.get_scan_metadata(Some(&options()?)).await,
        );
        record(
            &mut report.probes,
            &audit,
            "red_team.get_management_languages",
            red.get_management_languages(Some(&options()?)).await,
        );

        let agent = AgentGuardClient::new(
            client.clone(),
            Endpoint::parse(AGENT_DATA)?,
            Endpoint::parse(AGENT_MANAGEMENT)?,
        )?;
        let agent_page = AgentGuardScanListOptions::builder()
            .limit(Optional::Value(FiniteNumber::from(1_u32)))
            .build()?;
        record(
            &mut report.probes,
            &audit,
            "agentguard.list_scans",
            agent.list_scans(&agent_page, Some(&options()?)).await,
        );
        record(
            &mut report.probes,
            &audit,
            "agentguard.get_scan_stats",
            agent
                .get_scan_stats(
                    &AgentGuardStatsOptions::builder().build()?,
                    Some(&options()?),
                )
                .await,
        );
        record(
            &mut report.probes,
            &audit,
            "agentguard.list_rules",
            agent
                .list_rules(
                    &AgentGuardPageOptions::builder()
                        .limit(Optional::Value(FiniteNumber::from(1_u32)))
                        .build()?,
                    Some(&options()?),
                )
                .await,
        );

        let gateway = AIGatewayClient::new(
            client.with_endpoint(Endpoint::parse(GATEWAY_DATA)?)?,
            Endpoint::parse(GATEWAY_ADMIN)?,
            Endpoint::parse(IAM)?,
        )?;
        record(
            &mut report.probes,
            &audit,
            "gateway.workspaces.list",
            gateway
                .workspaces()
                .list(&AIGatewayWorkspaceListOptions::default(), Some(&options()?))
                .await,
        );
        let iam = gateway.iam_scopes();
        record(
            &mut report.probes,
            &audit,
            "iam.scopes.list",
            iam.list(Some(&options()?)).await,
        );

        if let Some(profiles) = profiles {
            if let Some(profile) = profiles
                .ai_profiles
                .into_option()
                .and_then(|v| v.into_iter().next())
            {
                // Only a returned key with an explicit matching tenant is selected.
                let key = keys
                    .and_then(|v| v.api_keys.into_option())
                    .and_then(|v| {
                        v.into_iter().find(|v| {
                            !v.revoked
                                && v.tsg_id
                                    .as_ref()
                                    .into_option()
                                    .is_some_and(|v| v == &args.expected_tsg)
                                && v.api_key
                                    .as_ref()
                                    .into_option()
                                    .is_some_and(|v| !v.is_empty())
                        })
                    })
                    .and_then(|v| v.api_key.into_option());
                let scan_auth = match key {
                    Some(key) => {
                        report.scan_authentication = Some("api_key_hmac");
                        Authentication::scan(Some(Secret::new(key)?), None)?
                    }
                    None => {
                        report.scan_authentication = Some("oauth_bearer");
                        Authentication::scan(None, token.map(|v| v.secret().clone()))?
                    }
                };
                let mut scan_limits = Limits::default();
                scan_limits.operation_timeout = Duration::from_secs(15);
                scan_limits.max_retries = 0;
                let scan_client = Client::builder(Endpoint::parse(Region::Us.endpoint())?)
                    .authentication(scan_auth)
                    .transport(Arc::new(audit.clone()))
                    .limits(scan_limits)
                    .build()?;
                let scanner = Scanner::new(scan_client)?;
                let ai_profile = AiProfile::builder()
                    .profile_name(Optional::Value(profile.profile_name))
                    .build()?;
                let content = Content::builder()
                    .prompt("A harmless native Rust SDK qualification check.")
                    .build()?;
                let mut sync_options = SyncScanOptions::default();
                sync_options.request = Some(options()?);
                if let Some(response) = record(
                    &mut report.probes,
                    &audit,
                    "scan.sync_scan",
                    scanner
                        .sync_scan(&ai_profile, &content, Some(&sync_options))
                        .await,
                ) {
                    if (response.error || response.timeout)
                        && let Some(last) = report.probes.last_mut()
                    {
                        last.workflow_accepted = false;
                        last.classification = "service_reported_scan_failure";
                    }
                    recovery.scan_ids.push(response.scan_id.clone());
                    recovery.report_ids.push(response.report_id.clone());
                    persist_recovery(args.recovery.as_deref(), &mut recovery)?;
                    if let Some(rows) = record(
                        &mut report.probes,
                        &audit,
                        "scan.query_scan_results",
                        scanner
                            .query_by_scan_ids(
                                std::slice::from_ref(&response.scan_id),
                                Some(&options()?),
                            )
                            .await,
                    ) {
                        require_receipt(
                            &mut report.probes,
                            scan_results_match(&rows, &response.scan_id, Some(&response.report_id)),
                        );
                    }
                    if let Some(rows) = record(
                        &mut report.probes,
                        &audit,
                        "scan.query_report",
                        scanner
                            .query_by_report_ids(
                                std::slice::from_ref(&response.report_id),
                                Some(&options()?),
                            )
                            .await,
                    ) {
                        require_receipt(
                            &mut report.probes,
                            report_matches(&rows, &response.scan_id, &response.report_id),
                        );
                    }
                }
                let request = ScanRequest::builder()
                    .ai_profile(ai_profile)
                    .contents(vec![content.to_json().clone()])
                    .build()?;
                let batch = AsyncScanObject::builder()
                    .req_id(FiniteNumber::from(1_u32))
                    .scan_req(request)
                    .build()?;
                if let Some(response) = record(
                    &mut report.probes,
                    &audit,
                    "scan.async_scan",
                    scanner.async_scan(&[batch], Some(&options()?)).await,
                ) {
                    recovery.scan_ids.push(response.scan_id.clone());
                    persist_recovery(args.recovery.as_deref(), &mut recovery)?;
                    if let Some(rows) = record(
                        &mut report.probes,
                        &audit,
                        "scan.query_async_results",
                        scanner
                            .query_by_scan_ids(
                                std::slice::from_ref(&response.scan_id),
                                Some(&options()?),
                            )
                            .await,
                    ) {
                        require_receipt(
                            &mut report.probes,
                            scan_results_match(&rows, &response.scan_id, None),
                        );
                    }
                }
            } else {
                skipped(&mut report.probes, "scan.sync_scan");
            }
        } else {
            skipped(&mut report.probes, "scan.sync_scan");
        }

        if args.mutations {
            qualify_scope(
                &iam,
                &audit,
                &mut report,
                &mut recovery,
                args.recovery.as_deref(),
            )
            .await?;
            if recovery.scope_name.is_none() || recovery.scope_deleted {
                qualify_provision(
                    &gateway,
                    &audit,
                    &mut report,
                    &mut recovery,
                    args.recovery.as_deref(),
                )
                .await?;
            } else {
                skipped(&mut report.probes, "gateway.workspaces.provision");
            }
        }
        report.cleanup_state = if recovery.workspace_uncertain
            || (recovery.scope_name.is_some() && !recovery.scope_deleted)
            || (recovery.workspace_id.is_some() && !recovery.workspace_archived)
        {
            "requires_reconciliation"
        } else if report
            .cleanup
            .iter()
            .any(|row| row.operation == "iam.scopes.verify_absent" && row.workflow_accepted)
        {
            "confirmed_retired"
        } else {
            "not_needed"
        };
        let wire = audit
            .wire
            .lock()
            .map_err(|_| Error::new(ErrorKind::Configuration))?;
        report.token_exchanges = wire.iter().filter(|v| v.token_exchange).count();
        report.service_requests = wire.iter().filter(|v| !v.token_exchange).count();
        drop(wire);
        write_private(&args.output, &report)?;
        println!(
            "qualification_report_written probes={} cleanup={}",
            report.probes.len(),
            report.cleanup.len()
        );
        Ok(())
    }

    async fn fixture_absent(
        iam: &IamScopesClient,
        name: &str,
        audit: &AuditTransport,
        rows: &mut Vec<Probe>,
        operation: &'static str,
    ) -> Result<bool> {
        // The deployed IAM service returns 400 for an absent name. Never infer
        // absence from that ambiguous error: require its unpaginated listing to
        // account for every item and contain no matching name. The fresh 128-bit
        // nonce minimizes collisions; the API offers no transactional preflight.
        match iam.list(Some(&options()?)).await {
            Ok(value)
                if value.count.as_u64() == u64::try_from(value.items.len()).ok()
                    && !value.items.iter().any(|scope| scope.name == name) =>
            {
                confirmed_absent(rows, operation, audit.latest_status());
                Ok(true)
            }
            Err(error) => {
                record::<()>(rows, audit, operation, Err(error));
                Ok(false)
            }
            Ok(_) => {
                rows.push(Probe {
                    operation,
                    classification: "not_attempted_existing_resource",
                    status: audit.latest_status(),
                    error_kind: None,
                    remote_outcome: None,
                    workflow_accepted: false,
                    rejection_details: None,
                });
                Ok(false)
            }
        }
    }

    async fn delete_owned_scope(
        iam: &IamScopesClient,
        name: &str,
        audit: &AuditTransport,
        report: &mut Report,
        recovery: &mut Recovery,
        path: Option<&Path>,
    ) -> Result<()> {
        let reconciled = iam.get(name, Some(&options()?)).await;
        match reconciled {
            Ok(value)
                if value.name == name
                    && recovery.scope_id.as_deref() == Some(value.id.as_str())
                    && value.resources.is_empty() =>
            {
                recovery.scope_uncertain = false;
                record(
                    &mut report.cleanup,
                    audit,
                    "iam.scopes.delete",
                    iam.delete(name, Some(&options()?)).await,
                );
                recovery.scope_deleted = fixture_absent(
                    iam,
                    name,
                    audit,
                    &mut report.cleanup,
                    "iam.scopes.verify_absent",
                )
                .await?;
            }
            Err(error) if error.status() == Some(404) && !recovery.scope_uncertain => {
                recovery.scope_deleted = true;
                confirmed_absent(&mut report.cleanup, "iam.scopes.verify_absent", Some(404));
            }
            Err(_) if !recovery.scope_uncertain => {
                recovery.scope_deleted = fixture_absent(
                    iam,
                    name,
                    audit,
                    &mut report.cleanup,
                    "iam.scopes.verify_absent",
                )
                .await?;
            }
            Err(error) => {
                record::<()>(
                    &mut report.cleanup,
                    audit,
                    "iam.scopes.reconcile",
                    Err(error),
                );
            }
            Ok(_) => {
                skipped(&mut report.cleanup, "iam.scopes.delete");
            }
        }
        persist_recovery(path, recovery)
    }

    async fn qualify_scope(
        iam: &IamScopesClient,
        audit: &AuditTransport,
        report: &mut Report,
        recovery: &mut Recovery,
        path: Option<&Path>,
    ) -> Result<()> {
        let name = unique_name()?;
        if !fixture_absent(
            iam,
            &name,
            audit,
            &mut report.probes,
            "iam.scopes.owned_fixture_preflight",
        )
        .await?
        {
            skipped(&mut report.probes, "iam.scopes.create");
            return Ok(());
        }
        recovery.scope_name = Some(name.clone());
        recovery.scope_deleted = false;
        recovery.scope_uncertain = true;
        recovery.scope_id = None;
        persist_recovery(path, recovery)?;
        let input = IamScopeCreateInput::builder()
            .name(name.clone())
            .description(Optional::Value(
                "Disposable scm-sdk qualification fixture".into(),
            ))
            .build()?;
        let creation = iam.create(&input, Some(&options()?)).await;
        recovery.scope_uncertain = creation
            .as_ref()
            .is_err_and(|e| e.remote_outcome() == RemoteOutcome::Unknown);
        if creation
            .as_ref()
            .is_err_and(|e| e.remote_outcome() != RemoteOutcome::Unknown)
        {
            // A definite rejection never grants ownership, even if another
            // actor created the same name after the preflight.
            record(&mut report.probes, audit, "iam.scopes.create", creation);
            recovery.scope_name = None;
            return persist_recovery(path, recovery);
        }
        if let Ok(value) = &creation {
            if value.name != name {
                recovery.scope_uncertain = true;
                report.probes.push(Probe {
                    operation: "iam.scopes.create",
                    classification: "service_contract_mismatch",
                    status: audit.latest_status(),
                    error_kind: Some("protocol"),
                    remote_outcome: Some("unknown"),
                    workflow_accepted: false,
                    rejection_details: None,
                });
                return persist_recovery(path, recovery);
            }
            recovery.scope_id = Some(value.id.clone());
        }
        persist_recovery(path, recovery)?;
        let created = record(&mut report.probes, audit, "iam.scopes.create", creation);
        if created.is_some()
            && let Some(scope) = record(
                &mut report.probes,
                audit,
                "iam.scopes.get",
                iam.get(&name, Some(&options()?)).await,
            )
        {
            let owned =
                scope.name == name && recovery.scope_id.as_deref() == Some(scope.id.as_str());
            require_receipt(&mut report.probes, owned);
            if owned {
                let resources = scope_resources(scope, None)?;
                let update = IamScopeUpdateInput::builder()
                    .description(Optional::Value(
                        "Disposable scm-sdk updated qualification fixture".into(),
                    ))
                    .resources(Optional::Value(resources))
                    .build()?;
                if let Some(updated) = record(
                    &mut report.probes,
                    audit,
                    "iam.scopes.update",
                    iam.update(&name, &update, Some(&options()?)).await,
                ) {
                    require_receipt(
                        &mut report.probes,
                        updated.name == name
                            && recovery.scope_id.as_deref() == Some(updated.id.as_str()),
                    );
                }
            }
        }
        delete_owned_scope(iam, &name, audit, report, recovery, path).await
    }

    async fn qualify_provision(
        gateway: &AIGatewayClient,
        audit: &AuditTransport,
        report: &mut Report,
        recovery: &mut Recovery,
        path: Option<&Path>,
    ) -> Result<()> {
        let name = unique_name()?;
        if !fixture_absent(
            &gateway.iam_scopes(),
            &name,
            audit,
            &mut report.probes,
            "gateway.provision.owned_fixture_preflight",
        )
        .await?
        {
            skipped(&mut report.probes, "gateway.workspaces.provision");
            return Ok(());
        }
        let mut request = GatewayWorkspaceProvisionRequest::new(name.clone());
        request.scope_name = Optional::Value(name.clone());
        request.description = Optional::Value("Disposable scm-sdk qualification workspace".into());
        recovery.scope_name = Some(name.clone());
        recovery.scope_deleted = false;
        recovery.scope_uncertain = true;
        recovery.scope_id = None;
        recovery.workspace_id = None;
        recovery.workspace_slug = None;
        recovery.workspace_archived = false;
        recovery.workspace_uncertain = true;
        persist_recovery(path, recovery)?;
        let result = gateway
            .workspaces()
            .provision(
                &request,
                &AIGatewayWorkspaceProvisionOptions::default(),
                Some(&options()?),
            )
            .await;
        let workspace = match result {
            Ok(value) => {
                recovery.scope_uncertain = false;
                recovery.workspace_uncertain = false;
                recovery.scope_id = Some(value.scope.id);
                record(
                    &mut report.probes,
                    audit,
                    "gateway.workspaces.provision",
                    Ok(()),
                );
                Some(value.workspace)
            }
            Err(error) => {
                record::<()>(
                    &mut report.probes,
                    audit,
                    "gateway.workspaces.provision",
                    Err(error.error().clone()),
                );
                recovery.scope_deleted = error.recovery().cleanup == ProvisionCleanup::Deleted;
                recovery.scope_uncertain = error.recovery().scope_state
                    == scm_sdk::airs::gateway::ProvisionScopeState::Unknown;
                recovery.workspace_uncertain =
                    error.recovery().workspace_state == ProvisionWorkspaceState::Unknown;
                persist_recovery(path, recovery)?;
                if error.recovery().scope_state
                    == scm_sdk::airs::gateway::ProvisionScopeState::NotCreated
                {
                    recovery.scope_name = None;
                    persist_recovery(path, recovery)?;
                    return Ok(());
                }
                if recovery.scope_deleted {
                    recovery.scope_deleted = fixture_absent(
                        &gateway.iam_scopes(),
                        &name,
                        audit,
                        &mut report.cleanup,
                        "iam.scopes.verify_absent",
                    )
                    .await?;
                    return persist_recovery(path, recovery);
                }
                if error.recovery().workspace_state == ProvisionWorkspaceState::Unknown {
                    report.cleanup.push(Probe {
                        operation: "gateway.provision.recovery",
                        classification: "retained_for_reconciliation",
                        status: error.error().status(),
                        error_kind: None,
                        remote_outcome: Some("unknown"),
                        workflow_accepted: false,
                        rejection_details: None,
                    });
                    return Ok(());
                }
                // The native recovery state proves a successful matching scope
                // creation. Preserve its current ID before any later cleanup;
                // an unknown create does not authorize deleting by name.
                if error.recovery().scope_state
                    == scm_sdk::airs::gateway::ProvisionScopeState::Created
                    && let Some(scope) = record(
                        &mut report.cleanup,
                        audit,
                        "iam.scopes.reconcile",
                        gateway.iam_scopes().get(&name, Some(&options()?)).await,
                    )
                    && scope.name == name
                {
                    recovery.scope_id = Some(scope.id);
                    persist_recovery(path, recovery)?;
                }
                error.recovery().workspace.clone()
            }
        };
        if let Some(workspace) = workspace {
            if workspace.name != name || workspace.scope_name != name {
                return Err(Error::new(ErrorKind::Configuration));
            }
            recovery.workspace_id = Some(workspace.id.clone());
            recovery.workspace_slug = Some(workspace.slug.clone());
            persist_recovery(path, recovery)?;
            let mut get = AIGatewayWorkspaceGetOptions::default();
            get.plane = AIGatewayPlane::Admin;
            if let Some(receipt) = record(
                &mut report.probes,
                audit,
                "gateway.workspaces.get",
                gateway
                    .workspaces()
                    .get(&workspace.id, &get, Some(&options()?))
                    .await,
            ) {
                require_receipt(
                    &mut report.probes,
                    receipt.id == workspace.id
                        && receipt.name == name
                        && receipt.slug == workspace.slug,
                );
            }
            record(
                &mut report.cleanup,
                audit,
                "gateway.workspaces.delete",
                gateway
                    .workspaces()
                    .delete(&workspace.id, Some(&options()?))
                    .await,
            );
            let mut archived = AIGatewayWorkspaceListOptions::default();
            archived.plane = AIGatewayPlane::Admin;
            archived.status = Some(AIGatewayWorkspaceStatus::Archived);
            if let Some(receipt) = record(
                &mut report.cleanup,
                audit,
                "gateway.workspaces.verify_archived",
                gateway
                    .workspaces()
                    .list(&archived, Some(&options()?))
                    .await,
            ) {
                recovery.workspace_archived = receipt.data.iter().any(|v| {
                    v.id == workspace.id
                        && v.name == name
                        && v.scope_name == name
                        && v.status == "archived"
                });
                if !recovery.workspace_archived
                    && let Some(last) = report.cleanup.last_mut()
                {
                    last.workflow_accepted = false;
                    last.classification = "archive_not_confirmed";
                }
            }
            persist_recovery(path, recovery)?;
            if recovery.workspace_archived {
                // Remove only the binding owned by this newly created scope, after
                // confirmed workspace retirement. Existing scopes are untouched.
                if let Some(scope) = record(
                    &mut report.cleanup,
                    audit,
                    "iam.scopes.reconcile",
                    gateway.iam_scopes().get(&name, Some(&options()?)).await,
                ) && recovery.scope_id.as_deref() == Some(scope.id.as_str())
                    && scope.name == name
                {
                    let description = scope.description.clone();
                    let resources = scope_resources(scope, Some(&workspace.slug))?;
                    let update = IamScopeUpdateInput::builder()
                        .description(Optional::Value(description))
                        .resources(Optional::Value(resources))
                        .build()?;
                    record(
                        &mut report.cleanup,
                        audit,
                        "iam.provision_scope.clear_owned_binding",
                        gateway
                            .iam_scopes()
                            .update(&name, &update, Some(&options()?))
                            .await,
                    );
                }
                delete_owned_scope(&gateway.iam_scopes(), &name, audit, report, recovery, path)
                    .await?;
            }
        } else {
            // No workspace was created (including a confirmed rejection). The
            // native workflow may already have deleted its newly owned scope.
            delete_owned_scope(&gateway.iam_scopes(), &name, audit, report, recovery, path).await?;
        }
        Ok(())
    }

    #[cfg(test)]
    mod tests {
        use super::*;
        use serde_json::json;

        #[test]
        fn read_only_transport_rejects_mutations_and_unreviewed_routes() {
            let token = url::Url::parse(ISSUER).unwrap();
            let targets = url::Url::parse(&format!(
                "{RED_MANAGEMENT}/v1/target?limit=1&profiling_status=FAILED"
            ))
            .unwrap();
            assert!(read_only_request_allowed("POST", &token));
            assert!(read_only_request_allowed("GET", &targets));
            for query in [
                "limit=10&profiling_status=FAILED",
                "limit=1&profiling_status=UNKNOWN",
                "limit=1&profiling_status=FAILED&skip=1",
                "limit=1&limit=1",
                "",
            ] {
                let mut changed = targets.clone();
                changed.set_query(Some(query));
                assert!(!read_only_request_allowed("GET", &changed));
            }
            for method in ["POST", "PUT", "PATCH", "DELETE"] {
                assert!(!read_only_request_allowed(method, &targets));
            }
            for value in [
                "https://example.invalid/v1/target",
                "http://api.sase.paloaltonetworks.com/ai-red-teaming/mgmt/v1/target",
                "https://api.sase.paloaltonetworks.com:8443/ai-red-teaming/mgmt/v1/target",
                "https://api.sase.paloaltonetworks.com/ai-red-teaming/mgmt/v1/target/other",
                "https://auth.apps.paloaltonetworks.com/oauth2/other",
            ] {
                let url = url::Url::parse(value).unwrap();
                assert!(!read_only_request_allowed("GET", &url));
                assert!(!read_only_request_allowed("POST", &url));
            }
            assert_eq!(
                PROFILING_PROBES.map(|row| row.0),
                ["NOT_STARTED", "IN_PROGRESS", "COMPLETED", "FAILED"]
            );
        }

        #[test]
        fn rejection_details_never_publish_unreviewed_codes_or_problem_text() {
            let problem = scm_sdk::ProblemDetails::from_value(&json!({
                "title":"private-echoed-title",
                "detail":"private-echoed-detail",
                "errors":[
                    {"property":"name", "reason":"private-echoed-reason"},
                    {"property":"private-echoed-property", "reason":"is required"},
                    {"property":"name", "reason":"is required"},
                    {"property":"type", "reason":"is required"}
                ],
                "future":"private-echoed-extension"
            }))
            .unwrap();
            let details = RejectionDetails::from_parts(Some("private-echoed-code"), Some(&problem));
            assert_eq!(
                serde_json::to_value(&details).unwrap(),
                json!({"code":null,"fields":["name","type"]})
            );
            assert!(!serde_json::to_string(&details).unwrap().contains("private"));
            for code in REJECTION_CODES {
                let details = RejectionDetails::from_parts(Some(code), None);
                assert_eq!(details.code, Some(*code));
                assert!(details.fields.is_empty());
            }
            for code in [
                None,
                Some("invalid_request_secret"),
                Some("INVALID_REQUEST"),
            ] {
                assert!(RejectionDetails::from_parts(code, None).code.is_none());
            }
        }

        #[test]
        fn rejection_details_are_optional_bounded_and_do_not_establish_a_cause() {
            let errors: Vec<_> = (0..40)
                .map(|_| json!({"property":"name","reason":"is required"}))
                .collect();
            let problem = scm_sdk::ProblemDetails::from_value(&json!({"errors":errors})).unwrap();
            for status in [400, 405, 409, 422] {
                let metadata = scm_sdk::ErrorMetadata::new()
                    .with_problem(problem.clone())
                    .with_status(status)
                    .unwrap();
                let error = Error::custom("private-application-message", None, metadata).unwrap();
                assert_eq!(failure_class(&error), "service_contract_rejection");
                let details = rejection_details(&error).unwrap();
                assert_eq!(details.code, None);
                assert_eq!(details.fields, ["name"]);
                let probe = Probe {
                    operation: "iam.scopes.create",
                    classification: failure_class(&error),
                    status: Some(status),
                    error_kind: Some(error_kind(error.kind())),
                    remote_outcome: Some(outcome(error.remote_outcome())),
                    workflow_accepted: false,
                    rejection_details: Some(details),
                };
                let value = serde_json::to_value(probe).unwrap();
                assert_eq!(value["workflowAccepted"], false);
                assert_eq!(
                    value["rejectionDetails"],
                    json!({"code":null,"fields":["name"]})
                );
                assert!(!value.to_string().contains("private"));
            }
            for status in [200, 403, 429, 500] {
                let error = Error::custom(
                    "private-application-message",
                    None,
                    scm_sdk::ErrorMetadata::new().with_status(status).unwrap(),
                )
                .unwrap();
                assert!(rejection_details(&error).is_none());
            }
            let mut rows = Vec::new();
            skipped(&mut rows, "iam.scopes.create");
            assert!(
                serde_json::to_value(&rows[0])
                    .unwrap()
                    .get("rejectionDetails")
                    .is_none()
            );
        }

        #[test]
        fn result_acceptance_requires_correlated_completed_non_error_scan() {
            let complete = json!({"scan_id":"owned-scan", "report_id":"owned-report",
                "category":"unknown-future-value", "action":"unknown-future-value",
                "error":false, "timeout":false, "errors":[]});
            let row = json!({"scan_id":"owned-scan", "result":complete});
            let valid: Vec<ScanIdResult> = serde_json::from_value(json!([row.clone()])).unwrap();
            assert!(scan_results_match(
                &valid,
                "owned-scan",
                Some("owned-report")
            ));
            for bad in [
                json!([]),
                json!([{}]),
                json!([{"scan_id":"owned-scan", "status":"pending"}]),
                json!([{"scan_id":"other", "result":complete}]),
            ] {
                let rows: Vec<ScanIdResult> = serde_json::from_value(bad).unwrap();
                assert!(!scan_results_match(
                    &rows,
                    "owned-scan",
                    Some("owned-report")
                ));
            }
            for field in ["error", "timeout"] {
                let mut failed = row.clone();
                failed["result"][field] = json!(true);
                let rows: Vec<ScanIdResult> = serde_json::from_value(json!([failed])).unwrap();
                assert!(!scan_results_match(
                    &rows,
                    "owned-scan",
                    Some("owned-report")
                ));
            }
            assert!(!scan_results_match(
                &valid,
                "owned-scan",
                Some("other-report")
            ));
        }

        #[test]
        fn report_acceptance_requires_both_ids_and_a_detection_collection() {
            for value in [
                json!([]),
                json!([{}]),
                json!([{"scan_id":"owned-scan", "report_id":"owned-report"}]),
                json!([{"scan_id":"other", "report_id":"owned-report", "detection_results":[]}]),
            ] {
                let rows: Vec<ThreatScanReport> = serde_json::from_value(value).unwrap();
                assert!(!report_matches(&rows, "owned-scan", "owned-report"));
            }
            let rows: Vec<ThreatScanReport> =
                serde_json::from_value(json!([{"scan_id":"owned-scan",
                "report_id":"owned-report", "detection_results":[]}]))
                .unwrap();
            assert!(report_matches(&rows, "owned-scan", "owned-report"));
        }

        #[test]
        fn binding_cleanup_preserves_every_unrelated_resource_and_extension() {
            let value = json!({"name":"owned", "id":"owned-id", "tsg_id":"synthetic",
                "description":"synthetic", "resources":[
                    {"resource_type":"workspace", "resource_id":"owned-workspace", "metadata":[]},
                    {"resource_type":"workspace", "resource_id":"other-workspace", "metadata":[{"future":true}], "future":"retained"},
                    {"resource_type":"other-kind", "resource_id":"owned-workspace"}]});
            let scope: IamScope = serde_json::from_value(value.clone()).unwrap();
            let kept = scope_resources(scope, Some("owned-workspace")).unwrap();
            assert_eq!(
                serde_json::to_value(kept).unwrap(),
                json!([value["resources"][1].clone(), value["resources"][2].clone()])
            );
            let scope = serde_json::from_value(value.clone()).unwrap();
            assert_eq!(
                serde_json::to_value(scope_resources(scope, None).unwrap()).unwrap(),
                value["resources"]
            );
        }

        #[test]
        fn uncorrelated_success_is_never_accepted_as_a_workflow() {
            let mut rows = vec![Probe {
                operation: "iam.scopes.get",
                classification: "verified",
                status: Some(200),
                error_kind: None,
                remote_outcome: None,
                workflow_accepted: true,
                rejection_details: None,
            }];
            require_receipt(&mut rows, false);
            assert!(!rows[0].workflow_accepted);
            assert_eq!(rows[0].classification, "incomplete_or_uncorrelated_receipt");
        }
    }
}

#[cfg(all(
    feature = "reqwest-transport",
    feature = "scan",
    feature = "management",
    feature = "model-security",
    feature = "red-team",
    feature = "gateway",
    feature = "agentguard"
))]
#[tokio::main(flavor = "current_thread")]
async fn main() {
    if !matches!(
        tokio::time::timeout(std::time::Duration::from_secs(600), qualification::run()).await,
        Ok(Ok(()))
    ) {
        eprintln!(
            "qualification_failed: inspect the private recovery record; no credential or payload was printed"
        );
        std::process::exit(1);
    }
}

#[cfg(not(all(
    feature = "reqwest-transport",
    feature = "scan",
    feature = "management",
    feature = "model-security",
    feature = "red-team",
    feature = "gateway",
    feature = "agentguard"
)))]
fn main() {
    eprintln!("Enable all product features and reqwest-transport for explicit live qualification.");
}
