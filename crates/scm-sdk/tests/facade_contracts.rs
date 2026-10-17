//! Facade accessors must preserve the actual endpoint and credential plane.
#![cfg(all(feature = "gateway", feature = "management", feature = "scan"))]
#![allow(clippy::field_reassign_with_default)]
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, StatusCode};
use scm_sdk::{
    Client, Endpoint, Limits, RequestOptions, Result, Secret,
    airs::{gateway::*, management::*, scan::Content},
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    config::OriginPolicy,
    transport::{Transport, TransportRequest, TransportResponse},
    validation::NativeDate,
};
use serde::de::DeserializeOwned;
use serde_json::json;
use std::{
    future::Future,
    sync::{Arc, Mutex},
    time::Duration,
};

const ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const BEARER: &str = "synthetic-facade-bearer";
#[derive(Default)]
struct Wire(Mutex<Vec<TransportRequest>>);
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.0.lock().unwrap().push(request);
            Ok(TransportResponse::from_bytes(
                StatusCode::BAD_REQUEST,
                HeaderMap::new(),
                Bytes::from_static(b"{}"),
            ))
        })
    }
}
struct Provider(AuthContext);
impl TokenProvider for Provider {
    fn context(&self) -> &AuthContext {
        &self.0
    }
    fn invalidate(&self, _: u64) -> bool {
        false
    }
    fn token<'a>(&'a self, _: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async {
            TokenLease::new(Secret::new(BEARER).unwrap(), 1, Duration::from_secs(3600))
        })
    }
}
fn limits() -> Limits {
    let mut limits = Limits::default();
    limits.max_retries = 0;
    limits
}
fn raw(wire: Arc<Wire>) -> Client {
    let destinations = [
        "https://data.facade.test",
        "https://admin.facade.test",
        "https://iam.facade.test",
        "https://dlp.facade.test",
    ]
    .map(|url| Endpoint::parse(url).unwrap());
    let context = AuthContext::new(
        Endpoint::parse("https://issuer.facade.test/token").unwrap(),
        "client",
        1,
        "12345",
    )
    .unwrap();
    Client::builder(destinations[0].clone())
        .transport(wire)
        .authentication(Authentication::oauth(Arc::new(Provider(context)), true))
        .destination_policy(Arc::new(OriginPolicy::new(destinations)))
        .limits(limits())
        .build()
        .unwrap()
}
fn empty<T: DeserializeOwned>() -> T {
    serde_json::from_value(json!({})).unwrap()
}
async fn dispatched<T>(
    wire: &Wire,
    future: impl Future<Output = Result<T>>,
    origin: &str,
    path: &str,
    tenant: bool,
    runtime: bool,
) {
    let error = match future.await {
        Ok(_) => panic!("synthetic rejection must propagate"),
        Err(error) => error,
    };
    assert_eq!(error.status(), Some(400));
    let mut sent = wire.0.lock().unwrap();
    assert_eq!(sent.len(), 1, "one dispatch per facade accessor");
    let request = sent.pop().unwrap();
    assert_eq!(
        request.url().origin().ascii_serialization(),
        origin,
        "origin on {path}"
    );
    assert_eq!(request.url().path(), path);
    assert_eq!(
        request.headers().contains_key("x-tsg-id"),
        tenant,
        "tenant header on {path}"
    );
    if tenant {
        assert_eq!(request.headers()["x-tsg-id"], "12345");
    }
    if runtime {
        assert!(!request.headers().contains_key("authorization"));
        assert_eq!(
            request.headers()["x-portkey-api-key"],
            "synthetic-facade-runtime-key"
        );
    } else {
        assert_eq!(
            request.headers()["authorization"],
            format!("Bearer {BEARER}")
        );
        assert!(!request.headers().contains_key("x-portkey-api-key"));
    }
}

#[tokio::test]
async fn every_gateway_facade_accessor_dispatches_on_its_exact_credential_plane() {
    let wire = Arc::new(Wire::default());
    let runtime = RuntimeClient::new(
        Client::builder(Endpoint::parse("https://runtime.facade.test").unwrap())
            .transport(wire.clone())
            .authentication(
                Authentication::runtime_key(Secret::new("synthetic-facade-runtime-key").unwrap())
                    .unwrap(),
            )
            .limits(limits())
            .build()
            .unwrap(),
    )
    .unwrap();
    let gateway = AIGatewayClient::new(
        raw(wire.clone()),
        Endpoint::parse("https://admin.facade.test").unwrap(),
        Endpoint::parse("https://iam.facade.test").unwrap(),
    )
    .unwrap()
    .with_runtime(runtime);
    let scoped = AIGatewayWorkspaceScopedListOptions::new(ID);
    let data = "https://data.facade.test";
    let admin = "https://admin.facade.test";
    dispatched(
        &wire,
        gateway.configs().list(&scoped, None),
        data,
        "/configs",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.providers().list(&scoped, None),
        data,
        "/providers",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.api_keys().list_service(&scoped, None),
        data,
        "/api-keys/service",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.guardrails().list(&scoped, None),
        data,
        "/guardrails",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.rate_limits().list(&empty(), None),
        data,
        "/policies/rate-limits",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.log_exports().list(&empty(), None),
        data,
        "/logs/exports",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.mcp_servers().list(&empty(), None),
        data,
        "/mcp-servers",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.usage_limits().list(&empty(), None),
        data,
        "/policies/usage-limits",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.secret_references().list(&empty(), None),
        admin,
        "/secret-references",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.plugins().list(None),
        admin,
        "/plugins",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway
            .workspaces()
            .list(&AIGatewayWorkspaceListOptions::default(), None),
        data,
        "/workspaces",
        true,
        false,
    )
    .await;
    let mut window = AIGatewayWindowOptions::new("workspace");
    window.start = Some(NativeDate::from_epoch_millis(0).unwrap());
    window.end = Some(NativeDate::from_epoch_millis(1).unwrap());
    dispatched(
        &wire,
        gateway
            .telemetry()
            .requests(&AIGatewayChartOptions::new(window).unwrap(), None),
        data,
        "/logs/charts/requests",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.deployments().list(None),
        admin,
        "/deployments",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.organisations().get_auth_settings("12345", None),
        admin,
        "/organisations/12345/auth-settings",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.integrations().list(None),
        admin,
        "/integrations",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.mcp_integrations().list(None),
        admin,
        "/mcp-integrations",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.admin_guardrails().list(&empty(), None),
        admin,
        "/guardrails",
        true,
        false,
    )
    .await;
    let dates = AIGatewayAuditLogListOptions::new(
        NativeDate::from_epoch_millis(0).unwrap(),
        NativeDate::from_epoch_millis(1).unwrap(),
    );
    dispatched(
        &wire,
        gateway.audit_logs().list(&dates, None),
        admin,
        "/audit-logs",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.iam_scopes().list(None),
        "https://iam.facade.test",
        "/scopes",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        gateway.inference().unwrap().list_models(&empty(), None),
        "https://runtime.facade.test",
        "/models",
        false,
        true,
    )
    .await;
    dispatched(
        &wire,
        gateway
            .runtime_resources()
            .unwrap()
            .list_models(&empty(), None),
        "https://runtime.facade.test",
        "/models",
        false,
        true,
    )
    .await;
}

#[tokio::test]
async fn every_management_and_dlp_facade_accessor_preserves_shared_identity_and_explicit_origins() {
    let wire = Arc::new(Wire::default());
    let management = ManagementClient::new(raw(wire.clone())).unwrap();
    let data = "https://data.facade.test";
    dispatched(
        &wire,
        management
            .profiles()
            .list_for_token(PaginationOptions::default(), None),
        data,
        "/v1/mgmt/profiles",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        management
            .topics()
            .list_for_token(PaginationOptions::default(), None),
        data,
        "/v1/mgmt/topics",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        management
            .api_keys()
            .list_for_token(PaginationOptions::default(), None),
        data,
        "/v1/mgmt/apikeys",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        management
            .customer_apps()
            .list_for_token(PaginationOptions::default(), None),
        data,
        "/v1/mgmt/customerapps",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        management
            .deployment_profiles()
            .list(DeploymentProfileListOptions::default(), None),
        data,
        "/v1/mgmt/deploymentprofiles",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        management.dlp_profiles().list(None),
        data,
        "/v1/mgmt/dlpprofiles",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        management
            .dashboard()
            .apps_list(&DashboardTimeRangeQuery::default(), None),
        data,
        "/v1/mgmt/dashboard/v2/apps/appslist",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        management
            .scan_logs()
            .query(&ScanLogQueryOptions::new(1, "HOUR", 1, 1, "all"), None),
        data,
        "/v1/mgmt/scanlogs",
        true,
        false,
    )
    .await;
    let body = serde_json::from_value(json!({"client_id":"client", "customer_app":"app"})).unwrap();
    dispatched(
        &wire,
        management
            .oauth()
            .unwrap()
            .get_access_token(&GetTokenOptions::new(body), None),
        data,
        "/v1/mgmt/oauth/client_credential/accesstoken",
        false,
        false,
    )
    .await;
    let dlp = management
        .dlp(Endpoint::parse("https://dlp.facade.test").unwrap())
        .unwrap();
    dispatched(
        &wire,
        dlp.data_patterns().list(&Default::default(), None),
        "https://dlp.facade.test",
        "/v2/api/data-patterns",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        dlp.data_profiles().list(&Default::default(), None),
        "https://dlp.facade.test",
        "/v2/api/data-profiles",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        dlp.dictionaries().list(&Default::default(), None),
        "https://dlp.facade.test",
        "/v2/api/dictionaries",
        true,
        false,
    )
    .await;
    dispatched(
        &wire,
        dlp.data_filtering_profiles()
            .list(&Default::default(), None),
        "https://dlp.facade.test",
        "/v2/api/data-filtering-profiles",
        true,
        false,
    )
    .await;
}

#[test]
fn every_content_accessor_returns_the_validated_explicit_payload_and_byte_length() {
    let tool = scm_sdk::airs::scan::models::ToolEvent::builder()
        .input(scm_sdk::data::Optional::Value("tool".into()))
        .build()
        .unwrap();
    let content = Content::builder()
        .prompt("prompt")
        .response("response")
        .context("context")
        .code_prompt("code prompt")
        .code_response("code response")
        .tool_event(tool)
        .build()
        .unwrap();
    assert_eq!(content.prompt(), Some("prompt"));
    assert_eq!(content.response(), Some("response"));
    assert_eq!(content.context(), Some("context"));
    assert_eq!(content.code_prompt(), Some("code prompt"));
    assert_eq!(content.code_response(), Some("code response"));
    assert!(content.tool_event().is_some());
    assert_eq!(content.length(), 6 + 8 + 7 + 11 + 13);
    let debug = format!("{content:?}");
    assert!(!debug.contains("prompt"));
    assert!(!debug.contains("response"));
}
