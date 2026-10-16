//! Independent literal wire contracts for all 79 Gateway control/public-pricing methods.
#![cfg(feature = "gateway")]
use bytes::Bytes;
use futures_util::{future::BoxFuture, stream};
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, OriginPolicy, RemoteOutcome, RequestOptions, Result,
    Secret,
    airs::gateway::{models::*, *},
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    data::{JsonValue, Optional, Patch},
    transport::{Transport, TransportRequest, TransportResponse},
    validation::NativeDate,
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
const ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const OTHER: &str = "660e8400-e29b-41d4-a716-446655440000";
#[derive(Clone, Copy)]
enum Plane {
    Data,
    Admin,
    Public,
}
enum Reply {
    Json(u16, Value),
    Empty,
    Failure,
    Text(&'static str),
}
struct Fixture {
    plane: Plane,
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    body: Option<Value>,
    reply: Reply,
    delay: Duration,
}
#[derive(Default)]
struct Wire {
    fixtures: Mutex<VecDeque<Fixture>>,
    calls: AtomicUsize,
}
impl Wire {
    fn expect(
        &self,
        plane: Plane,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<Value>,
        reply: Reply,
    ) {
        self.fixtures.lock().unwrap().push_back(Fixture {
            plane,
            method,
            path: path.into(),
            query: query
                .iter()
                .map(|(k, v)| ((*k).into(), (*v).into()))
                .collect(),
            body,
            reply,
            delay: Duration::ZERO,
        });
    }
    fn done(&self) {
        assert!(
            self.fixtures.lock().unwrap().is_empty(),
            "unconsumed contract fixture"
        );
    }
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let expected = self
                .fixtures
                .lock()
                .unwrap()
                .pop_front()
                .unwrap_or_else(|| {
                    panic!(
                        "unexpected fixture request: {} {}",
                        request.method(),
                        request.url().path()
                    )
                });
            let base = match expected.plane {
                Plane::Data => "https://gateway-data.example/ai_gw/v2",
                Plane::Admin => "https://gateway-admin.example/ai_gw/admin/v2",
                Plane::Public => "https://public-pricing.example",
            };
            let mut url = request.url().clone();
            url.set_query(None);
            assert_eq!(url.as_str(), format!("{base}{}", expected.path));
            assert_eq!(request.method(), expected.method);
            let query: Vec<_> = request.url().query_pairs().into_owned().collect();
            assert_eq!(query, expected.query);
            let headers = request.headers();
            let mut names: Vec<_> = headers.keys().map(|h| h.as_str()).collect();
            names.sort_unstable();
            let public = matches!(expected.plane, Plane::Public);
            let mut expected_names = if public {
                vec!["accept", "service-name", "user-agent"]
            } else {
                vec!["authorization", "service-name", "user-agent", "x-tsg-id"]
            };
            if expected.body.is_some() {
                expected_names.push("content-type");
            }
            expected_names.sort_unstable();
            assert_eq!(names, expected_names);
            assert_eq!(headers.len(), expected_names.len());
            if public {
                assert_eq!(headers["accept"], "application/json");
                assert!(!headers.contains_key("authorization"));
                assert!(!headers.contains_key("x-tsg-id"));
            } else {
                assert_eq!(headers["authorization"], "Bearer fixture-gateway-token");
                assert!(headers["authorization"].is_sensitive());
                assert_eq!(headers["x-tsg-id"], "123456789");
            }
            assert_eq!(headers["service-name"], "api");
            assert_eq!(
                headers["user-agent"],
                concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
            );
            if let Some(body) = expected.body {
                assert_eq!(headers["content-type"], "application/json");
                assert_eq!(
                    serde_json::from_slice::<Value>(request.body()).unwrap(),
                    body
                );
            } else {
                assert!(request.body().is_empty());
            }
            if !expected.delay.is_zero() {
                tokio::time::sleep(expected.delay).await;
            }
            let (status, body) = match expected.reply {
                Reply::Json(status, value) => (
                    StatusCode::from_u16(status).unwrap(),
                    Bytes::from(serde_json::to_vec(&value).unwrap()),
                ),
                Reply::Empty => (StatusCode::NO_CONTENT, Bytes::new()),
                Reply::Failure => return Err(Error::new(ErrorKind::Transport)),
                Reply::Text(text) => (StatusCode::OK, Bytes::from_static(text.as_bytes())),
            };
            let mut headers = HeaderMap::new();
            headers.insert("content-type", HeaderValue::from_static("application/json"));
            Ok(TransportResponse::new(
                status,
                headers,
                Box::pin(stream::iter([Ok(body)])),
            ))
        })
    }
}
struct Token {
    context: AuthContext,
    calls: AtomicUsize,
}
impl Token {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            context: AuthContext::new(
                Endpoint::parse("https://issuer.example/token").unwrap(),
                "gateway-client",
                1,
                "123456789",
            )
            .unwrap(),
            calls: AtomicUsize::new(0),
        })
    }
}
impl TokenProvider for Token {
    fn token<'a>(&'a self, _: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            TokenLease::new(
                Secret::new("fixture-gateway-token").unwrap(),
                1,
                Duration::from_secs(3600),
            )
        })
    }
    fn context(&self) -> &AuthContext {
        &self.context
    }
    fn invalidate(&self, _: u64) -> bool {
        false
    }
}
fn endpoints() -> (Endpoint, Endpoint, Endpoint) {
    (
        Endpoint::parse("https://gateway-data.example/ai_gw/v2").unwrap(),
        Endpoint::parse("https://gateway-admin.example/ai_gw/admin/v2").unwrap(),
        Endpoint::parse("https://public-pricing.example").unwrap(),
    )
}
struct Control {
    data: Client,
    admin: Client,
    api: AIGatewayApiKeysClient,
    integrations: AIGatewayIntegrationsClient,
    mcp: AIGatewayMcpServersClient,
    mcp_integrations: AIGatewayMcpIntegrationsClient,
    guardrails: AIGatewayGuardrailsClient,
    admin_guardrails: AIGatewayAdminGuardrailsClient,
    usage: AIGatewayUsageLimitsClient,
    secrets: AIGatewaySecretReferencesClient,
    plugins: AIGatewayPluginsClient,
    audit: AIGatewayAuditLogsClient,
    pricing: AIGatewayModelPricingClient,
}
fn clients() -> (Control, Arc<Wire>, Arc<Token>) {
    let (data, admin, public) = endpoints();
    let wire = Arc::new(Wire::default());
    let token = Token::new();
    let policy = Arc::new(OriginPolicy::new([
        data.clone(),
        admin.clone(),
        public.clone(),
    ]));
    let data = Client::builder(data)
        .transport(wire.clone())
        .destination_policy(policy.clone())
        .authentication(Authentication::oauth(token.clone(), false))
        .build()
        .unwrap();
    let admin = data.with_endpoint(admin).unwrap();
    let pricing = Client::builder(public)
        .transport(wire.clone())
        .destination_policy(policy)
        .build()
        .unwrap();
    (
        Control {
            api: AIGatewayApiKeysClient::new(data.clone()).unwrap(),
            integrations: AIGatewayIntegrationsClient::new(admin.clone()).unwrap(),
            mcp: AIGatewayMcpServersClient::new(data.clone()).unwrap(),
            mcp_integrations: AIGatewayMcpIntegrationsClient::new(admin.clone()).unwrap(),
            guardrails: AIGatewayGuardrailsClient::new(data.clone())
                .unwrap()
                .with_admin_endpoint(admin.endpoint().clone())
                .unwrap(),
            admin_guardrails: AIGatewayAdminGuardrailsClient::new(admin.clone()).unwrap(),
            usage: AIGatewayUsageLimitsClient::new(data.clone()).unwrap(),
            secrets: AIGatewaySecretReferencesClient::new(admin.clone()).unwrap(),
            plugins: AIGatewayPluginsClient::new(admin.clone()).unwrap(),
            audit: AIGatewayAuditLogsClient::new(admin.clone()).unwrap(),
            pricing: AIGatewayModelPricingClient::new(pricing).unwrap(),
            data,
            admin,
        },
        wire,
        token,
    )
}
fn body<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).unwrap()
}
macro_rules! check {
    ($wire:expr,$plane:expr,$method:expr,$path:expr,$query:expr,$body:expr,$reply:expr,$call:expr) => {{
        let reply: Value = $reply;
        $wire.expect(
            $plane,
            $method,
            $path,
            $query,
            $body,
            Reply::Json(200, reply.clone()),
        );
        let actual = $call.await.unwrap();
        assert_eq!(serde_json::to_value(&actual).unwrap(), reply);
        assert!(!format!("{actual:?}").contains("response-secret"));
        actual
    }};
}
macro_rules! check_void {
    ($wire:expr,$plane:expr,$method:expr,$path:expr,$query:expr,$call:expr) => {{
        $wire.expect(
            $plane,
            $method,
            $path,
            $query,
            None,
            Reply::Text("successful non-JSON deletion"),
        );
        $call.await.unwrap();
    }};
}
fn workspace() -> AIGatewayWorkspaceScopedListOptions {
    AIGatewayWorkspaceScopedListOptions::new(ID)
}
fn list(data: Value) -> Value {
    json!({"object":"list","total":1,"data":data,"future":"response-secret"})
}
fn key_body(user: bool) -> Value {
    let mut value = json!({"name":"fixture","scopes":["completions.write"],"organisation_id":"123456789","workspace_id":ID,"type":"workspace"});
    if user {
        value["user_id"] = json!(OTHER);
    }
    value
}
fn integration_body() -> Value {
    json!({"organisation_id":"123456789","ai_provider_id":OTHER,"name":"fixture","slug":"fixture","key":"request-secret"})
}
fn integration() -> Value {
    json!({"id":ID,"name":"fixture","owner_id":"owner","status":"FUTURE","created_at":"created","last_updated_at":null,"slug":"fixture","tags":null,"description":null,"workspace_id":null,"ai_provider_id":OTHER,"object":"integration","future":"response-secret"})
}
fn mcp_body() -> Value {
    json!({"organisation_id":"123456789","name":"fixture","slug":"fixture","url":"https://never-follow.example/mcp","auth_type":"none","transport":"http"})
}
fn mcp_detail() -> Value {
    json!({"id":ID,"name":"fixture","description":null,"owner_id":"owner","status":"FUTURE","created_at":"created","last_updated_at":"updated","configurations":{},"global_workspace_access":null,"workspace_id":null,"slug":"fixture","url":"https://never-follow.example/mcp","auth_type":"FUTURE","transport":"FUTURE","type":"mcp","secret_mappings":null,"object":"mcp","future":"response-secret"})
}
fn guardrail() -> Value {
    json!({"id":ID,"name":"fixture","slug":"fixture","organisation_id":"123456789","status":"active","owner_id":"owner","updated_by":null,"created_at":"created","last_updated_at":"updated","workspace_id":OTHER,"object":"guardrail","checks":[],"actions":{"deny":false,"async":false,"sequential":false},"version_id":"version","future":"response-secret"})
}
fn mapping() -> Value {
    json!({"id":ID,"guardrail_id":ID,"mcp_server_id":OTHER,"run_on":["future-phase"],"capability_ids":["opaque"],"future":"response-secret"})
}
fn usage_body() -> Value {
    json!({"name":"fixture","conditions":[{"key":"workspace","value":"fixture"}],"group_by":[{"key":"workspace"}],"type":"cost","credit_limit":0})
}
fn secret_body() -> Value {
    json!({"name":"fixture","manager_type":"aws_sm","auth_config":{"aws_auth_type":"accessKey","aws_access_key_id":"request-secret","aws_secret_access_key":"request-secret","aws_region":"us-east-1"},"secret_path":"never-resolve","allow_all_workspaces":false,"allowed_workspaces":["fixture"]})
}
// Only responses come from this independently frozen OpenAPI corpus. The wire
// routes/query/header/body expectations below are literal source-derived fixtures.
fn extension_reply(class: &str, method: &str) -> Value {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../contracts/fixtures/typescript/test/openapi/fixtures/gateway.json"
    ))
    .unwrap();
    corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|case| case["call"]["className"] == class && case["call"]["member"] == method)
        .unwrap()["responseBody"]
        .clone()
}
#[tokio::test]
async fn all_twelve_api_key_methods_preserve_separate_routes_and_rotation_defaults() {
    let (c, w, _) = clients();
    for user in [false, true] {
        let path = if user {
            "/api-keys/user"
        } else {
            "/api-keys/service"
        };
        check!(
            w,
            Plane::Data,
            Method::GET,
            path,
            &[("workspace_id", ID)],
            None,
            list(json!([{"id":ID}])),
            async {
                if user {
                    c.api.list_user(&workspace(), None).await
                } else {
                    c.api.list_service(&workspace(), None).await
                }
            }
        );
        check!(
            w,
            Plane::Data,
            Method::GET,
            &format!("{path}/{ID}"),
            &[],
            None,
            json!({"id":ID,"future":"response-secret"}),
            async {
                if user {
                    c.api.get_user(ID, None).await
                } else {
                    c.api.get_service(ID, None).await
                }
            }
        );
        let b = key_body(user);
        let receipt = check!(
            w,
            Plane::Data,
            Method::POST,
            path,
            &[],
            Some(b.clone()),
            json!({"key":"response-secret","id":ID}),
            async {
                if user {
                    c.api.create_user(&body(b), None).await
                } else {
                    c.api.create_service(&body(b), None).await
                }
            }
        );
        assert!(!format!("{receipt:?}").contains("response-secret"));
        let b = json!({"name":"updated","usage_limits":null,"expires_at":null});
        check!(
            w,
            Plane::Data,
            Method::PUT,
            &format!("{path}/{ID}"),
            &[],
            Some(b.clone()),
            json!({}),
            async {
                if user {
                    c.api.update_user(ID, &body(b), None).await
                } else {
                    c.api.update_service(ID, &body(b), None).await
                }
            }
        );
        check!(
            w,
            Plane::Data,
            Method::POST,
            &format!("{path}/{ID}/rotate"),
            &[],
            Some(json!({})),
            json!({"id":ID,"key":"response-secret","key_transition_expires_at":"expires"}),
            async {
                if user {
                    c.api.rotate_user(ID, None, None).await
                } else {
                    c.api.rotate_service(ID, None, None).await
                }
            }
        );
        check_void!(
            w,
            Plane::Data,
            Method::DELETE,
            &format!("{path}/{ID}"),
            &[],
            async {
                if user {
                    c.api.delete_user(ID, None).await
                } else {
                    c.api.delete_service(ID, None).await
                }
            }
        );
    }
    w.done();
}
#[tokio::test]
async fn all_twelve_integration_methods_preserve_catalog_resolution_and_model_workspace_contracts()
{
    let (c, w, t) = clients();
    let catalog = json!({"success":true,"data":[{"id":OTHER,"slug":"Open-AI","name":"OpenAI","status":"active"}]});
    check!(
        w,
        Plane::Admin,
        Method::GET,
        "/utils/static-resources/ai-providers",
        &[],
        None,
        catalog,
        c.integrations.catalog(None)
    );
    let before = t.calls.load(Ordering::SeqCst);
    assert_eq!(
        c.integrations
            .resolve_provider_id("\u{feff}550E8400-E29B-41D4-A716-446655440000\u{00a0}", None)
            .await
            .unwrap(),
        ID
    );
    assert_eq!(before, t.calls.load(Ordering::SeqCst));
    w.expect(
        Plane::Admin,
        Method::GET,
        "/utils/static-resources/ai-providers",
        &[],
        None,
        Reply::Json(
            200,
            json!({"data":[{"id":OTHER,"slug":"Open-AI","name":"OpenAI","status":"active"}]}),
        ),
    );
    assert_eq!(
        c.integrations
            .resolve_provider_id(" open-ai ", None)
            .await
            .unwrap(),
        OTHER
    );
    check!(
        w,
        Plane::Admin,
        Method::GET,
        "/integrations",
        &[],
        None,
        list(json!([integration()])),
        c.integrations.list(None)
    );
    check!(
        w,
        Plane::Admin,
        Method::GET,
        &format!("/integrations/{ID}"),
        &[],
        None,
        integration(),
        c.integrations.get(ID, None)
    );
    let b = integration_body();
    check!(
        w,
        Plane::Admin,
        Method::POST,
        "/integrations",
        &[],
        Some(b.clone()),
        json!({"id":ID}),
        c.integrations.create(&body(b), None)
    );
    let b = json!({"description":"updated","key":"request-secret","pricing_adjustments":null});
    check!(
        w,
        Plane::Admin,
        Method::PUT,
        &format!("/integrations/{ID}"),
        &[],
        Some(b.clone()),
        json!({}),
        c.integrations.update(ID, &body(b), None)
    );
    let reply = json!({"models":[],"allow_all_models":false,"object":"models"});
    check!(
        w,
        Plane::Admin,
        Method::GET,
        &format!("/integrations/{ID}/models"),
        &[],
        None,
        reply,
        c.integrations.get_models(ID, None)
    );
    let b = json!({"models":[{"slug":"custom-model","enabled":false}],"allow_all_models":false});
    check!(
        w,
        Plane::Admin,
        Method::PUT,
        &format!("/integrations/{ID}/models"),
        &[],
        Some(b.clone()),
        json!({}),
        c.integrations.set_models(ID, &body(b), None)
    );
    check!(
        w,
        Plane::Admin,
        Method::DELETE,
        "/integrations/integration-slug/models",
        &[("slugs", "custom-one,custom-two")],
        None,
        json!({}),
        c.integrations.delete_models(
            "integration-slug",
            &["custom-one".into(), "custom-two".into()],
            None
        )
    );
    let reply = json!({"workspaces":[],"global_workspace_access":{"enabled":false,"rate_limits":null,"usage_limits":null},"object":"workspaces"});
    check!(
        w,
        Plane::Admin,
        Method::GET,
        &format!("/integrations/{ID}/workspaces"),
        &[],
        None,
        reply,
        c.integrations.get_workspaces(ID, None)
    );
    let b = json!({"global_workspace_access":{"enabled":false},"override_existing_workspace_access":false});
    check!(
        w,
        Plane::Admin,
        Method::PUT,
        &format!("/integrations/{ID}/workspaces"),
        &[],
        Some(b.clone()),
        json!({}),
        c.integrations.set_workspaces(ID, &body(b), None)
    );
    check_void!(
        w,
        Plane::Admin,
        Method::DELETE,
        &format!("/integrations/{ID}"),
        &[("organisation_id", "123456789")],
        c.integrations.delete(ID, "123456789", None)
    );
    w.done();
}
#[tokio::test]
async fn all_twelve_mcp_server_methods_preserve_opaque_paths_queries_and_partial_updates() {
    let (c, w, _) = clients();
    let id = "server /λ";
    let path = "/mcp-servers/server%20%2F%CE%BB";
    let q = json!({"workspace_id":"workspace-slug","current_page":0,"page_size":100,"id":"opaque","search":""});
    check!(
        w,
        Plane::Data,
        Method::GET,
        "/mcp-servers",
        &[
            ("workspace_id", "workspace-slug"),
            ("current_page", "0"),
            ("page_size", "100"),
            ("id", "opaque"),
            ("search", "")
        ],
        None,
        extension_reply("AIGatewayMcpServersClient", "list"),
        c.mcp.list(&body(q), None)
    );
    let b = json!({"workspace_id":"workspace-slug","name":"fixture","description":null,"mcp_integration_id":"opaque","slug":"sdk"});
    check!(
        w,
        Plane::Data,
        Method::POST,
        "/mcp-servers",
        &[],
        Some(b.clone()),
        json!({"id":ID}),
        c.mcp.create(&body(b), None)
    );
    check!(
        w,
        Plane::Data,
        Method::GET,
        path,
        &[],
        None,
        extension_reply("AIGatewayMcpServersClient", "get"),
        c.mcp.get(id, None)
    );
    let b = json!({"description":null});
    check!(
        w,
        Plane::Data,
        Method::PUT,
        path,
        &[],
        Some(b.clone()),
        json!({}),
        c.mcp.update(id, &body(b), None)
    );
    check!(
        w,
        Plane::Data,
        Method::DELETE,
        path,
        &[],
        None,
        json!({}),
        c.mcp.delete(id, None)
    );
    check!(
        w,
        Plane::Data,
        Method::POST,
        &format!("{path}/test"),
        &[],
        None,
        extension_reply("AIGatewayMcpServersClient", "test"),
        c.mcp.test(id, None)
    );
    let q = json!({"page":1,"page_size":500,"type":"future-tool"});
    check!(
        w,
        Plane::Data,
        Method::GET,
        &format!("{path}/capabilities"),
        &[("page", "1"), ("page_size", "500"), ("type", "future-tool")],
        None,
        extension_reply("AIGatewayMcpServersClient", "getCapabilities"),
        c.mcp.get_capabilities(id, &body(q), None)
    );
    let b = json!({"capabilities":[{"name":"lookup","type":"tool","enabled":false}]});
    check!(
        w,
        Plane::Data,
        Method::PUT,
        &format!("{path}/capabilities"),
        &[],
        Some(b.clone()),
        json!({"success":false}),
        c.mcp.update_capabilities(id, &body(b), None)
    );
    let q = json!({"page":2,"page_size":500,"search":""});
    check!(
        w,
        Plane::Data,
        Method::GET,
        &format!("{path}/user-access"),
        &[("page", "2"), ("page_size", "500"), ("search", "")],
        None,
        extension_reply("AIGatewayMcpServersClient", "getUserAccess"),
        c.mcp.get_user_access(id, &body(q), None)
    );
    let b =
        json!({"user_access":[{"user_id":"opaque","enabled":false}],"default_user_access":"allow"});
    check!(
        w,
        Plane::Data,
        Method::PUT,
        &format!("{path}/user-access"),
        &[],
        Some(b.clone()),
        json!({"success":false}),
        c.mcp.update_user_access(id, &body(b), None)
    );
    let q = json!({"user_id":"user","workspace_id":"workspace","current_page":0,"page_size":500});
    check!(
        w,
        Plane::Data,
        Method::GET,
        &format!("{path}/connections"),
        &[
            ("user_id", "user"),
            ("workspace_id", "workspace"),
            ("current_page", "0"),
            ("page_size", "500")
        ],
        None,
        extension_reply("AIGatewayMcpServersClient", "getConnections"),
        c.mcp.get_connections(id, &body(q), None)
    );
    let q = json!({"user_id":"user","workspace_id":"workspace"});
    check!(
        w,
        Plane::Data,
        Method::DELETE,
        &format!("{path}/connections"),
        &[("user_id", "user"), ("workspace_id", "workspace")],
        None,
        json!({"success":false}),
        c.mcp.delete_connections(id, &body(q), None)
    );
    w.done();
}
#[tokio::test]
async fn all_ten_mcp_integration_methods_preserve_admin_shapes_and_ordered_workspace_union() {
    let (c, w, _) = clients();
    check!(
        w,
        Plane::Admin,
        Method::GET,
        "/mcp-integrations",
        &[],
        None,
        list(json!([])),
        c.mcp_integrations.list(None)
    );
    check!(
        w,
        Plane::Admin,
        Method::GET,
        &format!("/mcp-integrations/{ID}"),
        &[],
        None,
        mcp_detail(),
        c.mcp_integrations.get(ID, None)
    );
    let b = mcp_body();
    check!(
        w,
        Plane::Admin,
        Method::POST,
        "/mcp-integrations",
        &[],
        Some(b.clone()),
        json!({}),
        c.mcp_integrations.create(&body(b), None)
    );
    let b =
        json!({"description":null,"configurations":{"headers":{"Authorization":"request-secret"}}});
    check!(
        w,
        Plane::Admin,
        Method::PUT,
        &format!("/mcp-integrations/{ID}"),
        &[],
        Some(b.clone()),
        json!({}),
        c.mcp_integrations.update(ID, &body(b), None)
    );
    check_void!(
        w,
        Plane::Admin,
        Method::DELETE,
        &format!("/mcp-integrations/{ID}"),
        &[],
        c.mcp_integrations.delete(ID, None)
    );
    check!(
        w,
        Plane::Admin,
        Method::GET,
        &format!("/mcp-integrations/{ID}/capabilities"),
        &[],
        None,
        json!({"object":"list","counts":{},"total":0,"has_more":false,"data":[]}),
        c.mcp_integrations.get_capabilities(ID, None)
    );
    let reply = json!({"server_name":"fixture","server_version":"version","title":null,"description":null,"website_url":null,"icons":null,"protocol_version":null,"capability_flags":{},"instructions":null,"sync_status":"FUTURE","last_synced_at":null,"sync_error":null,"object":"metadata"});
    check!(
        w,
        Plane::Admin,
        Method::GET,
        &format!("/mcp-integrations/{ID}/metadata"),
        &[],
        None,
        reply,
        c.mcp_integrations.get_metadata(ID, None)
    );
    let b = json!({"capabilities":[{"name":"lookup","type":"tool","enabled":false}]});
    check!(
        w,
        Plane::Admin,
        Method::PUT,
        &format!("/mcp-integrations/{ID}/capabilities"),
        &[],
        Some(b.clone()),
        json!({"success":false}),
        c.mcp_integrations.set_capabilities(ID, &body(b), None)
    );
    let b = json!({"workspaces":[{"id":"workspace-slug","enabled":false}],"global_workspace_access":null,"override_existing_workspace_access":false});
    check!(
        w,
        Plane::Admin,
        Method::PUT,
        &format!("/mcp-integrations/{ID}/workspaces"),
        &[],
        Some(b.clone()),
        json!({}),
        c.mcp_integrations.set_workspaces(ID, &body(b), None)
    );
    let mut q = McpIntegrationWorkspacesOptions::default();
    q.version = Some(String::new());
    let list = check!(
        w,
        Plane::Admin,
        Method::GET,
        &format!("/mcp-integrations/{ID}/workspaces"),
        &[("version", "")],
        None,
        json!({"data":[{"id":"workspace-slug","enabled":false}],"total":1,"future":"response-secret"}),
        c.mcp_integrations.get_workspaces(ID, &q, None)
    );
    assert!(matches!(list, McpIntegrationWorkspacesResponse::List(_)));
    // An invalid first-branch data field is unknown to the second, exactly as
    // in the frozen overlapping Zod union. Only one API dispatch is made.
    let legacy = check!(
        w,
        Plane::Admin,
        Method::GET,
        &format!("/mcp-integrations/{ID}/workspaces"),
        &[],
        None,
        json!({"data":"future-shape","workspaces":[{"id":"legacy"}]}),
        c.mcp_integrations
            .get_workspaces(ID, &McpIntegrationWorkspacesOptions::default(), None)
    );
    assert!(matches!(
        legacy,
        McpIntegrationWorkspacesResponse::Legacy(_)
    ));
    w.done();
}
#[tokio::test]
async fn all_nine_data_guardrail_methods_keep_admin_catalog_and_distinct_mapping_schema() {
    let (c, w, _) = clients();
    check!(
        w,
        Plane::Admin,
        Method::GET,
        "/utils/static-resources/schema",
        &[("resource", "guardrails")],
        None,
        json!({"object":"schema","evals":[]}),
        c.guardrails.get_catalog(None)
    );
    check!(
        w,
        Plane::Data,
        Method::GET,
        "/guardrails",
        &[("workspace_id", ID)],
        None,
        list(json!([guardrail()])),
        c.guardrails.list(&workspace(), None)
    );
    check!(
        w,
        Plane::Data,
        Method::GET,
        &format!("/guardrails/{ID}"),
        &[],
        None,
        guardrail(),
        c.guardrails.get(ID, None)
    );
    let b = json!({"workspace_id":ID,"organisation_id":"123456789","name":"fixture","target":"mcp_tools"});
    check!(
        w,
        Plane::Data,
        Method::POST,
        "/guardrails",
        &[],
        Some(b.clone()),
        json!({"id":ID,"version_id":"version","slug":"fixture","object":"guardrail"}),
        c.guardrails.create(&body(b), None)
    );
    let b = json!({"name":"updated"});
    check!(
        w,
        Plane::Data,
        Method::PUT,
        &format!("/guardrails/{ID}"),
        &[],
        Some(b.clone()),
        json!({}),
        c.guardrails.update(ID, &body(b), None)
    );
    check_void!(
        w,
        Plane::Data,
        Method::DELETE,
        &format!("/guardrails/{ID}"),
        &[],
        c.guardrails.delete(ID, None)
    );
    check!(
        w,
        Plane::Data,
        Method::GET,
        &format!("/guardrails/{ID}/mcp-servers"),
        &[],
        None,
        json!([mapping()]),
        c.guardrails.get_mcp_servers(ID, None)
    );
    let b = json!({"mcp_servers":{"opaque-server":{"run_on":["future-phase"],"mcp_integration_capability_ids":["opaque-capability"]}}});
    check!(
        w,
        Plane::Data,
        Method::PUT,
        &format!("/guardrails/{ID}/mcp-servers"),
        &[],
        Some(b.clone()),
        json!({"changed":true,"added":1,"updated":0,"removed":0}),
        c.guardrails.sync_mcp_servers(ID, &body(b), None)
    );
    let b =
        json!({"run_on":["future-phase"],"mcp_integration_capability_ids":["opaque-capability"]});
    check!(
        w,
        Plane::Data,
        Method::PUT,
        &format!("/guardrails/{ID}/mcp-servers/{OTHER}"),
        &[],
        Some(b.clone()),
        json!({"map_id":"opaque-map"}),
        c.guardrails.upsert_mcp_server(ID, OTHER, &body(b), None)
    );
    w.done();
}
#[tokio::test]
async fn all_eight_admin_guardrail_methods_preserve_admin_identity_and_paging_keys() {
    let (c, w, _) = clients();
    let q = json!({"workspaceId":ID,"pageSize":1000,"currentPage":0});
    check!(
        w,
        Plane::Admin,
        Method::GET,
        "/guardrails",
        &[
            ("workspace_id", ID),
            ("page_size", "1000"),
            ("current_page", "0")
        ],
        None,
        json!({"data":[guardrail()],"total":1}),
        c.admin_guardrails.list(&body(q), None)
    );
    check!(
        w,
        Plane::Admin,
        Method::GET,
        &format!("/guardrails/{ID}"),
        &[],
        None,
        guardrail(),
        c.admin_guardrails.get(ID, None)
    );
    let b = json!({"organisation_id":OTHER,"name":"fixture","target":"mcp_tools"});
    check!(
        w,
        Plane::Admin,
        Method::POST,
        "/guardrails",
        &[],
        Some(b.clone()),
        json!({"id":ID,"version_id":"version","slug":"fixture"}),
        c.admin_guardrails.create(&body(b), None)
    );
    let b = json!({"name":"updated"});
    check!(
        w,
        Plane::Admin,
        Method::PUT,
        &format!("/guardrails/{ID}"),
        &[],
        Some(b.clone()),
        json!({"id":ID,"slug":"fixture"}),
        c.admin_guardrails.update(ID, &body(b), None)
    );
    check_void!(
        w,
        Plane::Admin,
        Method::DELETE,
        &format!("/guardrails/{ID}"),
        &[],
        c.admin_guardrails.delete(ID, None)
    );
    check!(
        w,
        Plane::Admin,
        Method::GET,
        &format!("/guardrails/{ID}/mcp-servers"),
        &[],
        None,
        json!([mapping()]),
        c.admin_guardrails.get_mcp_servers(ID, None)
    );
    let b = json!({"mcp_servers":{OTHER:{"run_on":["input","output"],"mcp_integration_capability_ids":[ID]}}});
    check!(
        w,
        Plane::Admin,
        Method::PUT,
        &format!("/guardrails/{ID}/mcp-servers"),
        &[],
        Some(b.clone()),
        json!({"changed":false,"added":0,"updated":0,"removed":0}),
        c.admin_guardrails.sync_mcp_servers(ID, &body(b), None)
    );
    let b = json!({"run_on":["input"],"mcp_integration_capability_ids":[ID]});
    check!(
        w,
        Plane::Admin,
        Method::PUT,
        &format!("/guardrails/{ID}/mcp-servers/{OTHER}"),
        &[],
        Some(b.clone()),
        json!({"map_id":ID}),
        c.admin_guardrails
            .upsert_mcp_server(ID, OTHER, &body(b), None)
    );
    w.done();
}
#[tokio::test]
async fn all_seven_usage_policy_methods_preserve_zero_page_size_and_no_body_reset() {
    let (c, w, _) = clients();
    let id = "policy /λ";
    let path = "/policies/usage-limits/policy%20%2F%CE%BB";
    let q = json!({"workspace_id":"workspace","status":"active","type":"cost","page_size":0,"current_page":0});
    check!(
        w,
        Plane::Data,
        Method::GET,
        "/policies/usage-limits",
        &[
            ("workspace_id", "workspace"),
            ("status", "active"),
            ("type", "cost"),
            ("page_size", "0"),
            ("current_page", "0")
        ],
        None,
        extension_reply("AIGatewayUsageLimitsClient", "list"),
        c.usage.list(&body(q), None)
    );
    let b = usage_body();
    check!(
        w,
        Plane::Data,
        Method::POST,
        "/policies/usage-limits",
        &[],
        Some(b.clone()),
        json!({"id":ID}),
        c.usage.create(&body(b), None)
    );
    let q = json!({"status":"active","include_usage":false});
    check!(
        w,
        Plane::Data,
        Method::GET,
        path,
        &[("status", "active"), ("include_usage", "false")],
        None,
        extension_reply("AIGatewayUsageLimitsClient", "get"),
        c.usage.get(id, &body(q), None)
    );
    let b = json!({"description":null,"alert_threshold":null,"periodic_reset_days":null,"next_usage_reset_at":null});
    check!(
        w,
        Plane::Data,
        Method::PUT,
        path,
        &[],
        Some(b.clone()),
        json!({}),
        c.usage.update(id, &body(b), None)
    );
    check!(
        w,
        Plane::Data,
        Method::DELETE,
        path,
        &[],
        None,
        json!({}),
        c.usage.delete(id, None)
    );
    let q = json!({"status":"active","search":"","page_size":100,"current_page":0});
    check!(
        w,
        Plane::Data,
        Method::GET,
        &format!("{path}/entities"),
        &[
            ("status", "active"),
            ("search", ""),
            ("page_size", "100"),
            ("current_page", "0")
        ],
        None,
        extension_reply("AIGatewayUsageLimitsClient", "listEntities"),
        c.usage.list_entities(id, &body(q), None)
    );
    check!(
        w,
        Plane::Data,
        Method::PUT,
        &format!("{path}/entities/entity%20%2F%CE%BB/reset"),
        &[],
        None,
        json!({}),
        c.usage.reset_entity(id, "entity /λ", None)
    );
    w.done();
}
#[tokio::test]
async fn all_five_secret_reference_methods_keep_credentials_local_and_nullable_updates() {
    let (c, w, _) = clients();
    let id = "secret /λ";
    let path = "/secret-references/secret%20%2F%CE%BB";
    let q = json!({"manager_type":"aws_sm","tags":"env:fixture","search":"fixture","current_page":0,"page_size":100});
    check!(
        w,
        Plane::Admin,
        Method::GET,
        "/secret-references",
        &[
            ("manager_type", "aws_sm"),
            ("tags", "env:fixture"),
            ("search", "fixture"),
            ("current_page", "0"),
            ("page_size", "100")
        ],
        None,
        extension_reply("AIGatewaySecretReferencesClient", "list"),
        c.secrets.list(&body(q), None)
    );
    let b = secret_body();
    let input: GatewaySecretReferencesClientCreateRequest = body(b.clone());
    assert!(!format!("{input:?}").contains("request-secret"));
    check!(
        w,
        Plane::Admin,
        Method::POST,
        "/secret-references",
        &[],
        Some(b),
        json!({"id":ID,"slug":"fixture","object":"secret-reference"}),
        c.secrets.create(&input, None)
    );
    check!(
        w,
        Plane::Admin,
        Method::GET,
        path,
        &[],
        None,
        extension_reply("AIGatewaySecretReferencesClient", "get"),
        c.secrets.get(id, None)
    );
    let b = json!({"description":null,"secret_key":null,"tags":null,"allow_all_workspaces":false});
    check!(
        w,
        Plane::Admin,
        Method::PUT,
        path,
        &[],
        Some(b.clone()),
        json!({}),
        c.secrets.update(id, &body(b), None)
    );
    check!(
        w,
        Plane::Admin,
        Method::DELETE,
        path,
        &[],
        None,
        json!({}),
        c.secrets.delete(id, None)
    );
    assert_eq!(w.calls.load(Ordering::SeqCst), 5);
    w.done();
}
#[tokio::test]
async fn both_plugin_methods_preserve_secret_bodies_without_debug_disclosure() {
    let (c, w, _) = clients();
    check!(
        w,
        Plane::Admin,
        Method::GET,
        "/plugins",
        &[],
        None,
        list(json!([])),
        c.plugins.list(None)
    );
    let b = json!({"organisation_id":"123456789","integration_id":ID,"credentials":{"AIRS_API_KEY":"request-secret"}});
    let input: GatewayPluginCreateRequest = body(b.clone());
    assert!(!format!("{input:?}").contains("request-secret"));
    check!(
        w,
        Plane::Admin,
        Method::POST,
        "/plugins",
        &[],
        Some(b),
        json!({}),
        c.plugins.create(&input, None)
    );
    w.done();
}
#[tokio::test]
async fn audit_dates_use_exact_js_utc_milliseconds_and_full_range_without_redacting_data() {
    let (c, w, _) = clients();
    for (start, end, first, last) in [
        (
            0,
            -1,
            "1970-01-01T00:00:00.000Z",
            "1969-12-31T23:59:59.999Z",
        ),
        (
            -8_640_000_000_000_000,
            8_640_000_000_000_000,
            "-271821-04-20T00:00:00.000Z",
            "+275760-09-13T00:00:00.000Z",
        ),
    ] {
        let q = AIGatewayAuditLogListOptions::new(
            NativeDate::from_epoch_millis(start).unwrap(),
            NativeDate::from_epoch_millis(end).unwrap(),
        );
        let reply = json!({"records":[{"timestamp":"created","method":"POST","uri":"/integrations","request_id":"request","request_body":"request-secret","query_params":"","request_headers":"[REDACTED]","user_id":"user","user_type":"type","organisation_id":"tenant","workspace_id":"workspace","response_status_code":200,"resource_type":"integration","action":"create","client_ip":"127.0.0.1","country":"fixture"}],"future":"response-secret"});
        let result = check!(
            w,
            Plane::Admin,
            Method::GET,
            "/audit-logs",
            &[("start_time", first), ("end_time", last)],
            None,
            reply,
            c.audit.list(&q, None)
        );
        assert_eq!(result.records[0].request_body, "request-secret");
        assert!(!format!("{result:?}").contains("request-secret"));
    }
    w.done();
}
#[tokio::test]
async fn public_pricing_preserves_unevaluated_recursive_prices_and_exact_segments() {
    let (c, w, t) = clients();
    let reply = json!({"currency":"USD","pay_as_you_go":{"request_token":{"price":0.000123,"future":"response-secret"},"additional_units":{"unit":{"price":3}}},"calculate":{"request":{"operation":"sum","operands":[{"value":"usage.request_tokens"},{"operation":"multiply","operands":[{"value":"never.execute()"}]}]}},"future":{"keep":true}});
    check!(
        w,
        Plane::Public,
        Method::GET,
        "/model-configs/pricing/provider%20%2F%CE%BB/model%20%2Fversion",
        &[],
        None,
        reply,
        c.pricing.get("provider /λ", "model /version", None)
    );
    assert_eq!(t.calls.load(Ordering::SeqCst), 0);
    w.done();
}
struct Mutation {
    name: &'static str,
    plane: Plane,
    method: Method,
    path: String,
    query: Vec<(&'static str, &'static str)>,
    body: Option<Value>,
}
fn mutations() -> Vec<Mutation> {
    let mut values = Vec::new();
    macro_rules! entry {
        ($name:expr,$plane:expr,$method:expr,$path:expr,$query:expr,$body:expr) => {
            values.push(Mutation {
                name: $name,
                plane: $plane,
                method: $method,
                path: ($path).into(),
                query: $query,
                body: $body,
            });
        };
    }
    for (kind, user) in [("service", false), ("user", true)] {
        let p = format!("/api-keys/{kind}");
        entry!(
            if user {
                "key_create_user"
            } else {
                "key_create_service"
            },
            Plane::Data,
            Method::POST,
            p.clone(),
            vec![],
            Some(key_body(user))
        );
        entry!(
            if user {
                "key_update_user"
            } else {
                "key_update_service"
            },
            Plane::Data,
            Method::PUT,
            format!("{p}/{ID}"),
            vec![],
            Some(json!({"name":"updated"}))
        );
        entry!(
            if user {
                "key_rotate_user"
            } else {
                "key_rotate_service"
            },
            Plane::Data,
            Method::POST,
            format!("{p}/{ID}/rotate"),
            vec![],
            Some(json!({}))
        );
        entry!(
            if user {
                "key_delete_user"
            } else {
                "key_delete_service"
            },
            Plane::Data,
            Method::DELETE,
            format!("{p}/{ID}"),
            vec![],
            None
        );
    }
    entry!(
        "integration_create",
        Plane::Admin,
        Method::POST,
        "/integrations",
        vec![],
        Some(integration_body())
    );
    entry!(
        "integration_update",
        Plane::Admin,
        Method::PUT,
        format!("/integrations/{ID}"),
        vec![],
        Some(json!({"name":"updated"}))
    );
    entry!(
        "integration_delete",
        Plane::Admin,
        Method::DELETE,
        format!("/integrations/{ID}"),
        vec![("organisation_id", "123456789")],
        None
    );
    entry!(
        "integration_delete_models",
        Plane::Admin,
        Method::DELETE,
        "/integrations/integration-slug/models",
        vec![("slugs", "custom")],
        None
    );
    entry!(
        "integration_set_models",
        Plane::Admin,
        Method::PUT,
        format!("/integrations/{ID}/models"),
        vec![],
        Some(json!({"models":[{"slug":"custom","enabled":false}]}))
    );
    entry!(
        "integration_set_workspaces",
        Plane::Admin,
        Method::PUT,
        format!("/integrations/{ID}/workspaces"),
        vec![],
        Some(json!({"global_workspace_access":{"enabled":false}}))
    );
    entry!(
        "mcp_create",
        Plane::Data,
        Method::POST,
        "/mcp-servers",
        vec![],
        Some(json!({"name":"fixture","mcp_integration_id":"opaque"}))
    );
    entry!(
        "mcp_update",
        Plane::Data,
        Method::PUT,
        format!("/mcp-servers/{ID}"),
        vec![],
        Some(json!({}))
    );
    entry!(
        "mcp_delete",
        Plane::Data,
        Method::DELETE,
        format!("/mcp-servers/{ID}"),
        vec![],
        None
    );
    entry!(
        "mcp_test",
        Plane::Data,
        Method::POST,
        format!("/mcp-servers/{ID}/test"),
        vec![],
        None
    );
    entry!(
        "mcp_capabilities",
        Plane::Data,
        Method::PUT,
        format!("/mcp-servers/{ID}/capabilities"),
        vec![],
        Some(json!({"capabilities":[{"name":"lookup","type":"tool","enabled":false}]}))
    );
    entry!(
        "mcp_user_access",
        Plane::Data,
        Method::PUT,
        format!("/mcp-servers/{ID}/user-access"),
        vec![],
        Some(json!({"user_access":[{"user_id":"opaque","enabled":false}]}))
    );
    entry!(
        "mcp_connections",
        Plane::Data,
        Method::DELETE,
        format!("/mcp-servers/{ID}/connections"),
        vec![],
        None
    );
    entry!(
        "mcp_integration_create",
        Plane::Admin,
        Method::POST,
        "/mcp-integrations",
        vec![],
        Some(mcp_body())
    );
    entry!(
        "mcp_integration_update",
        Plane::Admin,
        Method::PUT,
        format!("/mcp-integrations/{ID}"),
        vec![],
        Some(json!({"description":null}))
    );
    entry!(
        "mcp_integration_delete",
        Plane::Admin,
        Method::DELETE,
        format!("/mcp-integrations/{ID}"),
        vec![],
        None
    );
    entry!(
        "mcp_integration_capabilities",
        Plane::Admin,
        Method::PUT,
        format!("/mcp-integrations/{ID}/capabilities"),
        vec![],
        Some(json!({"capabilities":[{"name":"lookup","type":"tool","enabled":false}]}))
    );
    entry!(
        "mcp_integration_workspaces",
        Plane::Admin,
        Method::PUT,
        format!("/mcp-integrations/{ID}/workspaces"),
        vec![],
        Some(json!({"global_workspace_access":null}))
    );
    for admin in [false, true] {
        let plane = if admin { Plane::Admin } else { Plane::Data };
        let b = if admin {
            json!({"name":"fixture","target":"mcp_tools","organisation_id":OTHER})
        } else {
            json!({"name":"fixture","target":"mcp_tools","workspace_id":ID})
        };
        entry!(
            if admin {
                "admin_guardrail_create"
            } else {
                "guardrail_create"
            },
            plane,
            Method::POST,
            "/guardrails",
            vec![],
            Some(b)
        );
        entry!(
            if admin {
                "admin_guardrail_update"
            } else {
                "guardrail_update"
            },
            plane,
            Method::PUT,
            format!("/guardrails/{ID}"),
            vec![],
            Some(json!({"name":"updated"}))
        );
        entry!(
            if admin {
                "admin_guardrail_delete"
            } else {
                "guardrail_delete"
            },
            plane,
            Method::DELETE,
            format!("/guardrails/{ID}"),
            vec![],
            None
        );
        entry!(
            if admin {
                "admin_guardrail_sync"
            } else {
                "guardrail_sync"
            },
            plane,
            Method::PUT,
            format!("/guardrails/{ID}/mcp-servers"),
            vec![],
            Some(json!({"mcp_servers":{}}))
        );
        entry!(
            if admin {
                "admin_guardrail_upsert"
            } else {
                "guardrail_upsert"
            },
            plane,
            Method::PUT,
            format!("/guardrails/{ID}/mcp-servers/{OTHER}"),
            vec![],
            Some(json!({}))
        );
    }
    entry!(
        "usage_create",
        Plane::Data,
        Method::POST,
        "/policies/usage-limits",
        vec![],
        Some(usage_body())
    );
    entry!(
        "usage_update",
        Plane::Data,
        Method::PUT,
        format!("/policies/usage-limits/{ID}"),
        vec![],
        Some(json!({}))
    );
    entry!(
        "usage_delete",
        Plane::Data,
        Method::DELETE,
        format!("/policies/usage-limits/{ID}"),
        vec![],
        None
    );
    entry!(
        "usage_reset",
        Plane::Data,
        Method::PUT,
        format!("/policies/usage-limits/{ID}/entities/{OTHER}/reset"),
        vec![],
        None
    );
    entry!(
        "secret_create",
        Plane::Admin,
        Method::POST,
        "/secret-references",
        vec![],
        Some(secret_body())
    );
    entry!(
        "secret_update",
        Plane::Admin,
        Method::PUT,
        format!("/secret-references/{ID}"),
        vec![],
        Some(json!({"secret_key":null}))
    );
    entry!(
        "secret_delete",
        Plane::Admin,
        Method::DELETE,
        format!("/secret-references/{ID}"),
        vec![],
        None
    );
    entry!(
        "plugin_create",
        Plane::Admin,
        Method::POST,
        "/plugins",
        vec![],
        Some(
            json!({"organisation_id":"123456789","integration_id":ID,"credentials":{"AIRS_API_KEY":"request-secret"}})
        )
    );
    assert_eq!(values.len(), 44);
    values
}
async fn invoke_mutation(c: &Control, m: &Mutation) -> Result<()> {
    let b = m.body.clone().unwrap_or_else(|| json!({}));
    match m.name {
        "key_create_service" => c.api.create_service(&body(b), None).await.map(|_| ()),
        "key_create_user" => c.api.create_user(&body(b), None).await.map(|_| ()),
        "key_update_service" => c.api.update_service(ID, &body(b), None).await.map(|_| ()),
        "key_update_user" => c.api.update_user(ID, &body(b), None).await.map(|_| ()),
        "key_rotate_service" => c.api.rotate_service(ID, None, None).await.map(|_| ()),
        "key_rotate_user" => c.api.rotate_user(ID, None, None).await.map(|_| ()),
        "key_delete_service" => c.api.delete_service(ID, None).await,
        "key_delete_user" => c.api.delete_user(ID, None).await,
        "integration_create" => c.integrations.create(&body(b), None).await.map(|_| ()),
        "integration_update" => c.integrations.update(ID, &body(b), None).await.map(|_| ()),
        "integration_delete" => c.integrations.delete(ID, "123456789", None).await,
        "integration_delete_models" => c
            .integrations
            .delete_models("integration-slug", &["custom".into()], None)
            .await
            .map(|_| ()),
        "integration_set_models" => c
            .integrations
            .set_models(ID, &body(b), None)
            .await
            .map(|_| ()),
        "integration_set_workspaces" => c
            .integrations
            .set_workspaces(ID, &body(b), None)
            .await
            .map(|_| ()),
        "mcp_create" => c.mcp.create(&body(b), None).await.map(|_| ()),
        "mcp_update" => c.mcp.update(ID, &body(b), None).await.map(|_| ()),
        "mcp_delete" => c.mcp.delete(ID, None).await.map(|_| ()),
        "mcp_test" => c.mcp.test(ID, None).await.map(|_| ()),
        "mcp_capabilities" => c
            .mcp
            .update_capabilities(ID, &body(b), None)
            .await
            .map(|_| ()),
        "mcp_user_access" => c
            .mcp
            .update_user_access(ID, &body(b), None)
            .await
            .map(|_| ()),
        "mcp_connections" => c
            .mcp
            .delete_connections(ID, &body(json!({})), None)
            .await
            .map(|_| ()),
        "mcp_integration_create" => c.mcp_integrations.create(&body(b), None).await.map(|_| ()),
        "mcp_integration_update" => c
            .mcp_integrations
            .update(ID, &body(b), None)
            .await
            .map(|_| ()),
        "mcp_integration_delete" => c.mcp_integrations.delete(ID, None).await,
        "mcp_integration_capabilities" => c
            .mcp_integrations
            .set_capabilities(ID, &body(b), None)
            .await
            .map(|_| ()),
        "mcp_integration_workspaces" => c
            .mcp_integrations
            .set_workspaces(ID, &body(b), None)
            .await
            .map(|_| ()),
        "guardrail_create" => c.guardrails.create(&body(b), None).await.map(|_| ()),
        "guardrail_update" => c.guardrails.update(ID, &body(b), None).await.map(|_| ()),
        "guardrail_delete" => c.guardrails.delete(ID, None).await,
        "guardrail_sync" => c
            .guardrails
            .sync_mcp_servers(ID, &body(b), None)
            .await
            .map(|_| ()),
        "guardrail_upsert" => c
            .guardrails
            .upsert_mcp_server(ID, OTHER, &body(b), None)
            .await
            .map(|_| ()),
        "admin_guardrail_create" => c.admin_guardrails.create(&body(b), None).await.map(|_| ()),
        "admin_guardrail_update" => c
            .admin_guardrails
            .update(ID, &body(b), None)
            .await
            .map(|_| ()),
        "admin_guardrail_delete" => c.admin_guardrails.delete(ID, None).await,
        "admin_guardrail_sync" => c
            .admin_guardrails
            .sync_mcp_servers(ID, &body(b), None)
            .await
            .map(|_| ()),
        "admin_guardrail_upsert" => c
            .admin_guardrails
            .upsert_mcp_server(ID, OTHER, &body(b), None)
            .await
            .map(|_| ()),
        "usage_create" => c.usage.create(&body(b), None).await.map(|_| ()),
        "usage_update" => c.usage.update(ID, &body(b), None).await.map(|_| ()),
        "usage_delete" => c.usage.delete(ID, None).await.map(|_| ()),
        "usage_reset" => c.usage.reset_entity(ID, OTHER, None).await.map(|_| ()),
        "secret_create" => c.secrets.create(&body(b), None).await.map(|_| ()),
        "secret_update" => c.secrets.update(ID, &body(b), None).await.map(|_| ()),
        "secret_delete" => c.secrets.delete(ID, None).await.map(|_| ()),
        "plugin_create" => c.plugins.create(&body(b), None).await.map(|_| ()),
        _ => panic!("unbound mutation fixture"),
    }
}
#[tokio::test]
async fn all_forty_four_mutations_preserve_unknown_outcomes_without_replay() {
    let (c, w, _) = clients();
    for m in mutations() {
        w.expect(
            m.plane,
            m.method.clone(),
            &m.path,
            &m.query,
            m.body.clone(),
            Reply::Failure,
        );
        let e = invoke_mutation(&c, &m).await.unwrap_err();
        assert_eq!(e.kind(), ErrorKind::Transport, "{}", m.name);
        assert_eq!(e.remote_outcome(), RemoteOutcome::Unknown, "{}", m.name);
    }
    assert_eq!(w.calls.load(Ordering::SeqCst), 44);
    w.done();
}
#[tokio::test]
async fn all_thirty_eight_typed_mutations_retain_unknown_decode_failures_without_replay() {
    let (c, w, _) = clients();
    for m in mutations().into_iter().filter(|m| {
        !matches!(
            m.name,
            "key_delete_service"
                | "key_delete_user"
                | "integration_delete"
                | "mcp_integration_delete"
                | "guardrail_delete"
                | "admin_guardrail_delete"
        )
    }) {
        w.expect(
            m.plane,
            m.method.clone(),
            &m.path,
            &m.query,
            m.body.clone(),
            Reply::Text("not-json-response-secret"),
        );
        let e = invoke_mutation(&c, &m).await.unwrap_err();
        assert_eq!(e.kind(), ErrorKind::Decode, "{}", m.name);
        assert_eq!(e.status(), Some(200));
        assert_eq!(e.remote_outcome(), RemoteOutcome::Unknown, "{}", m.name);
        assert!(!format!("{e:?} {e}").contains("response-secret"));
    }
    assert_eq!(w.calls.load(Ordering::SeqCst), 38);
    w.done();
}
fn rejected(error: Error) {
    assert!(matches!(
        error.kind(),
        ErrorKind::Validation | ErrorKind::Limit
    ));
    assert!(!format!("{error:?} {error}").contains("request-secret"));
}
#[tokio::test]
async fn invalid_ids_query_dialects_strict_bodies_and_cross_field_inputs_fail_before_auth() {
    let (c, w, t) = clients();
    rejected(c.api.get_service("bad", None).await.unwrap_err());
    rejected(
        c.api
            .list_user(&AIGatewayWorkspaceScopedListOptions::new("slug"), None)
            .await
            .unwrap_err(),
    );
    rejected(c.integrations.get("slug", None).await.unwrap_err());
    rejected(
        c.integrations
            .delete(ID, "123/not-tenant", None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.integrations
            .delete_models("bad/ref", &["model".into()], None)
            .await
            .unwrap_err(),
    );
    for slugs in [
        vec![],
        vec![String::new()],
        vec!["a,b".into()],
        vec!["x".repeat(513)],
    ] {
        rejected(
            c.integrations
                .delete_models("slug", &slugs, None)
                .await
                .unwrap_err(),
        );
    }
    rejected(
        c.integrations
            .resolve_provider_id("\u{feff}\u{2029}", None)
            .await
            .unwrap_err(),
    );
    rejected(c.mcp.get("..", None).await.unwrap_err());
    rejected(c.mcp.get(&"😀".repeat(257), None).await.unwrap_err());
    rejected(
        c.usage
            .reset_entity("policy", "\u{feff}", None)
            .await
            .unwrap_err(),
    );
    rejected(c.secrets.get("%2e%2e", None).await.unwrap_err());
    rejected(c.mcp_integrations.get("slug", None).await.unwrap_err());
    rejected(c.guardrails.get("slug", None).await.unwrap_err());
    rejected(
        c.admin_guardrails
            .upsert_mcp_server(ID, "slug", &body(json!({})), None)
            .await
            .unwrap_err(),
    );
    for q in [
        json!({"page_size":101}),
        json!({"current_page":-1}),
        json!({"search":"x".repeat(256)}),
        json!({"unexpected":"request-secret"}),
    ] {
        rejected(c.mcp.list(&body(q), None).await.unwrap_err());
    }
    for q in [
        json!({"page":0}),
        json!({"page_size":501}),
        json!({"type":""}),
    ] {
        rejected(
            c.mcp
                .get_capabilities(ID, &body(q), None)
                .await
                .unwrap_err(),
        );
    }
    rejected(
        c.mcp
            .get_user_access(ID, &body(json!({"page":0})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.mcp
            .get_connections(ID, &body(json!({"current_page":-1})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.mcp
            .delete_connections(ID, &body(json!({"unknown":true})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.usage
            .list(&body(json!({"page_size":-1})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.usage
            .get(ID, &body(json!({"status":""})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.usage
            .list_entities(ID, &body(json!({"page_size":101})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.secrets
            .list(&body(json!({"manager_type":"other"})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.secrets
            .list(&body(json!({"search":""})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.admin_guardrails
            .list(&body(json!({"pageSize":1001})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.admin_guardrails
            .list(&body(json!({"workspaceId":"slug"})), None)
            .await
            .unwrap_err(),
    );
    let mut b = key_body(false);
    b["organisation_id"] = json!(ID);
    rejected(c.api.create_service(&body(b), None).await.unwrap_err());
    let mut b = key_body(true);
    b["user_id"] = json!("slug");
    rejected(c.api.create_user(&body(b), None).await.unwrap_err());
    rejected(
        c.api
            .update_service(ID, &body(json!({})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.api
            .rotate_user(ID, Some(&body(json!({"key_transition_period_ms":1}))), None)
            .await
            .unwrap_err(),
    );
    let b = json!({"rotation_policy":{"rotation_period":"weekly","next_rotation_at":"2026-09-01T00:00:00Z"}});
    rejected(c.api.update_user(ID, &body(b), None).await.unwrap_err());
    let mut b = integration_body();
    b["ai_provider_id"] = json!("slug");
    rejected(c.integrations.create(&body(b), None).await.unwrap_err());
    rejected(
        c.integrations
            .update(ID, &body(json!({})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.integrations
            .set_models(ID, &body(json!({"models":[]})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.integrations
            .set_workspaces(ID, &body(json!({})), None)
            .await
            .unwrap_err(),
    );
    let mut b = mcp_body();
    b["url"] = json!("not-a-url");
    rejected(c.mcp_integrations.create(&body(b), None).await.unwrap_err());
    rejected(
        c.mcp_integrations
            .update(ID, &body(json!({})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.mcp_integrations
            .set_capabilities(ID, &body(json!({"capabilities":[]})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.mcp_integrations
            .set_workspaces(ID, &body(json!({})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.mcp
            .create(
                &body(json!({"name":"fixture","mcp_integration_id":"opaque","slug":"x"})),
                None,
            )
            .await
            .unwrap_err(),
    );
    rejected(
        c.mcp
            .update(ID, &body(json!({"extra":"request-secret"})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.mcp
            .update_capabilities(
                ID,
                &body(json!({"capabilities":[{"name":"lookup","type":"","enabled":false}]})),
                None,
            )
            .await
            .unwrap_err(),
    );
    rejected(
        c.mcp
            .update_user_access(
                ID,
                &body(json!({"user_access":[],"default_user_access":""})),
                None,
            )
            .await
            .unwrap_err(),
    );
    rejected(
        c.guardrails
            .create(&body(json!({"workspace_id":ID,"name":"fixture"})), None)
            .await
            .unwrap_err(),
    );
    rejected(c.guardrails.create(&body(json!({"workspace_id":ID,"name":"fixture","target":"mcp_tools","organisation_id":OTHER})),None).await.unwrap_err());
    rejected(
        c.guardrails
            .update(ID, &body(json!({})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.admin_guardrails
            .create(
                &body(json!({"name":"fixture","organisation_id":"123456789"})),
                None,
            )
            .await
            .unwrap_err(),
    );
    rejected(
        c.admin_guardrails
            .upsert_mcp_server(ID, OTHER, &body(json!({"run_on":["future-phase"]})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.admin_guardrails
            .sync_mcp_servers(ID, &body(json!({"mcp_servers":{"opaque-server":{}}})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.usage
            .create(
                &body(json!({"conditions":[],"group_by":[],"type":"cost","credit_limit":0})),
                None,
            )
            .await
            .unwrap_err(),
    );
    rejected(
        c.usage
            .update(ID, &body(json!({"periodic_reset_days":366})), None)
            .await
            .unwrap_err(),
    );
    let mut b = secret_body();
    b["manager_type"] = json!("azure_kv");
    rejected(c.secrets.create(&body(b), None).await.unwrap_err());
    let mut b = secret_body();
    b["allow_all_workspaces"] = json!(true);
    rejected(c.secrets.create(&body(b), None).await.unwrap_err());
    rejected(
        c.secrets
            .update(ID, &body(json!({})), None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.secrets
            .update(
                ID,
                &body(json!({"allow_all_workspaces":true,"allowed_workspaces":["fixture"]})),
                None,
            )
            .await
            .unwrap_err(),
    );
    rejected(
        c.plugins
            .create(
                &body(json!({"organisation_id":"123456789","integration_id":ID,"credentials":{}})),
                None,
            )
            .await
            .unwrap_err(),
    );
    rejected(c.pricing.get("..", "model", None).await.unwrap_err());
    rejected(
        c.pricing
            .get("provider", "\u{feff}", None)
            .await
            .unwrap_err(),
    );
    assert_eq!(t.calls.load(Ordering::SeqCst), 0);
    assert_eq!(w.calls.load(Ordering::SeqCst), 0);
    w.done();
}
#[tokio::test]
async fn oversized_queries_and_provider_model_names_are_rejected_without_auth_or_transport() {
    let (c, w, t) = clients();
    let large = "request-secret".repeat(20_000);
    let q = body(json!({"search":large.clone()}));
    rejected(c.mcp.get_user_access(ID, &q, None).await.unwrap_err());
    let q = body(json!({"search":large.clone()}));
    rejected(c.usage.list_entities(ID, &q, None).await.unwrap_err());
    let mut q = McpIntegrationWorkspacesOptions::default();
    q.version = Some(large.clone());
    rejected(
        c.mcp_integrations
            .get_workspaces(ID, &q, None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.integrations
            .resolve_provider_id(&large, None)
            .await
            .unwrap_err(),
    );
    rejected(
        c.integrations
            .delete_models("slug", &vec!["x".repeat(512); 33], None)
            .await
            .unwrap_err(),
    );
    rejected(c.pricing.get("provider", &large, None).await.unwrap_err());
    assert_eq!(t.calls.load(Ordering::SeqCst), 0);
    assert_eq!(w.calls.load(Ordering::SeqCst), 0);
    w.done();
}
#[tokio::test]
async fn constructor_authentication_destination_scoping_and_option_formatting_are_safe() {
    let (c, w, t) = clients();
    let none = Client::builder(endpoints().0)
        .transport(w.clone())
        .build()
        .unwrap();
    assert!(AIGatewayApiKeysClient::new(none.clone()).is_err());
    assert!(AIGatewayIntegrationsClient::new(none.clone()).is_err());
    assert!(AIGatewayMcpServersClient::new(none.clone()).is_err());
    assert!(AIGatewayMcpIntegrationsClient::new(none.clone()).is_err());
    assert!(AIGatewayGuardrailsClient::new(none.clone()).is_err());
    assert!(AIGatewayAdminGuardrailsClient::new(none.clone()).is_err());
    assert!(AIGatewayUsageLimitsClient::new(none.clone()).is_err());
    assert!(AIGatewaySecretReferencesClient::new(none.clone()).is_err());
    assert!(AIGatewayPluginsClient::new(none.clone()).is_err());
    assert!(AIGatewayAuditLogsClient::new(none).is_err());
    assert!(AIGatewayModelPricingClient::new(c.data.clone()).is_err());
    let runtime = Client::builder(endpoints().2)
        .transport(w.clone())
        .authentication(
            Authentication::runtime_key(Secret::new("runtime-secret").unwrap()).unwrap(),
        )
        .build()
        .unwrap();
    assert!(AIGatewayModelPricingClient::new(runtime).is_err());
    let no_catalog = AIGatewayGuardrailsClient::new(c.data.clone()).unwrap();
    assert_eq!(
        no_catalog.get_catalog(None).await.unwrap_err().kind(),
        ErrorKind::Configuration
    );
    assert!(
        no_catalog
            .with_admin_endpoint(Endpoint::parse("https://unadmitted.example/admin").unwrap())
            .is_err()
    );
    let mut q = McpIntegrationWorkspacesOptions::default();
    q.version = Some("request-secret".into());
    assert!(!format!("{q:?}").contains("request-secret"));
    let q = AIGatewayWorkspaceScopedListOptions::new("request-secret");
    assert!(!format!("{q:?}").contains("request-secret"));
    let query: AIGatewayAuditLogListOptions = AIGatewayAuditLogListOptions::new(
        NativeDate::from_epoch_millis(0).unwrap(),
        NativeDate::from_epoch_millis(1).unwrap(),
    );
    assert!(!format!("{query:?}").contains("1970"));
    let mut config = AIGatewayModelPricingClientOptions::default();
    config.read_retries = 6;
    assert!(c.pricing.clone().with_options(config).is_err());
    let mut config = AIGatewayModelPricingClientOptions::default();
    config.timeout = Duration::ZERO;
    assert!(c.pricing.clone().with_options(config).is_err());
    assert!(!format!("{:?}", c.admin).contains("fixture-gateway-token"));
    assert_eq!(t.calls.load(Ordering::SeqCst), 0);
    assert_eq!(w.calls.load(Ordering::SeqCst), 0);
    w.done();
}
#[tokio::test]
async fn public_pricing_defaults_to_zero_retries_and_explicit_overrides_remain_bounded() {
    let (c, w, t) = clients();
    let path = "/model-configs/pricing/provider/model";
    // Core Client defaults permit safe-read retries. Pricing independently defaults to zero.
    w.expect(Plane::Public, Method::GET, path, &[], None, Reply::Failure);
    assert_eq!(
        c.pricing
            .get("provider", "model", None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Transport
    );
    assert_eq!(w.calls.load(Ordering::SeqCst), 1);
    let mut config = AIGatewayModelPricingClientOptions::default();
    config.read_retries = 1;
    let pricing = c.pricing.clone().with_options(config).unwrap();
    w.expect(Plane::Public, Method::GET, path, &[], None, Reply::Failure);
    w.expect(
        Plane::Public,
        Method::GET,
        path,
        &[],
        None,
        Reply::Json(200, json!({})),
    );
    pricing.get("provider", "model", None).await.unwrap();
    assert_eq!(w.calls.load(Ordering::SeqCst), 3);
    let options = RequestOptions::with_timeout(Duration::from_secs(2))
        .unwrap()
        .with_retries(0)
        .unwrap();
    w.expect(Plane::Public, Method::GET, path, &[], None, Reply::Failure);
    pricing
        .get("provider", "model", Some(&options))
        .await
        .unwrap_err();
    assert_eq!(w.calls.load(Ordering::SeqCst), 4);
    let options = RequestOptions::with_timeout(Duration::from_secs(2))
        .unwrap()
        .with_retries(1)
        .unwrap();
    w.expect(Plane::Public, Method::GET, path, &[], None, Reply::Failure);
    w.expect(
        Plane::Public,
        Method::GET,
        path,
        &[],
        None,
        Reply::Json(200, json!({})),
    );
    c.pricing
        .get("provider", "model", Some(&options))
        .await
        .unwrap();
    assert_eq!(w.calls.load(Ordering::SeqCst), 6);
    assert_eq!(t.calls.load(Ordering::SeqCst), 0);
    w.done();
}
#[tokio::test]
async fn public_pricing_never_follows_redirects_and_honors_deadline_and_cancellation() {
    let (c, w, t) = clients();
    let path = "/model-configs/pricing/provider/model";
    w.expect(
        Plane::Public,
        Method::GET,
        path,
        &[],
        None,
        Reply::Json(
            302,
            json!({"location":"https://never-follow.example/request-secret"}),
        ),
    );
    let e = c.pricing.get("provider", "model", None).await.unwrap_err();
    assert_eq!(e.kind(), ErrorKind::Destination);
    assert_eq!(e.status(), Some(302));
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    options.cancel();
    assert_eq!(
        c.pricing
            .get("provider", "model", Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    w.expect(
        Plane::Public,
        Method::GET,
        path,
        &[],
        None,
        Reply::Json(200, json!({})),
    );
    w.fixtures.lock().unwrap().back_mut().unwrap().delay = Duration::from_millis(100);
    let options = RequestOptions::with_timeout(Duration::from_millis(5)).unwrap();
    assert_eq!(
        c.pricing
            .get("provider", "model", Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Deadline
    );
    let mut config = AIGatewayModelPricingClientOptions::default();
    config.timeout = Duration::from_millis(5);
    let pricing = c.pricing.clone().with_options(config).unwrap();
    w.expect(
        Plane::Public,
        Method::GET,
        path,
        &[],
        None,
        Reply::Json(200, json!({})),
    );
    w.fixtures.lock().unwrap().back_mut().unwrap().delay = Duration::from_millis(100);
    assert_eq!(
        pricing
            .get("provider", "model", None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Deadline
    );
    assert_eq!(t.calls.load(Ordering::SeqCst), 0);
    assert_eq!(w.calls.load(Ordering::SeqCst), 3);
    w.done();
}
#[tokio::test]
async fn empty_optional_successes_required_structures_and_workspace_union_errors_are_distinct() {
    let (c, w, _) = clients();
    w.expect(
        Plane::Data,
        Method::PUT,
        &format!("/mcp-servers/{ID}"),
        &[],
        Some(json!({})),
        Reply::Empty,
    );
    let result = c.mcp.update(ID, &body(json!({})), None).await.unwrap();
    assert_eq!(serde_json::to_value(result).unwrap(), json!({}));
    w.expect(
        Plane::Data,
        Method::PUT,
        &format!("/policies/usage-limits/{ID}"),
        &[],
        Some(json!({})),
        Reply::Empty,
    );
    c.usage.update(ID, &body(json!({})), None).await.unwrap();
    w.expect(
        Plane::Public,
        Method::GET,
        "/model-configs/pricing/provider/model",
        &[],
        None,
        Reply::Empty,
    );
    c.pricing.get("provider", "model", None).await.unwrap();
    w.expect(
        Plane::Admin,
        Method::GET,
        &format!("/mcp-integrations/{ID}/workspaces"),
        &[],
        None,
        Reply::Empty,
    );
    assert!(matches!(
        c.mcp_integrations
            .get_workspaces(ID, &McpIntegrationWorkspacesOptions::default(), None)
            .await
            .unwrap(),
        McpIntegrationWorkspacesResponse::List(_)
    ));
    w.expect(
        Plane::Data,
        Method::GET,
        "/api-keys/service",
        &[("workspace_id", ID)],
        None,
        Reply::Empty,
    );
    assert_eq!(
        c.api
            .list_service(&workspace(), None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Decode
    );
    w.expect(
        Plane::Admin,
        Method::GET,
        &format!("/mcp-integrations/{ID}/workspaces"),
        &[],
        None,
        Reply::Json(
            200,
            json!({"data":"invalid-first","workspaces":"invalid-second"}),
        ),
    );
    let e = c
        .mcp_integrations
        .get_workspaces(ID, &McpIntegrationWorkspacesOptions::default(), None)
        .await
        .unwrap_err();
    assert_eq!(e.kind(), ErrorKind::Decode);
    assert_eq!(e.status(), Some(200));
    w.expect(
        Plane::Public,
        Method::GET,
        "/model-configs/pricing/provider/model",
        &[],
        None,
        Reply::Json(200, json!({"currency":"EUR"})),
    );
    assert_eq!(
        c.pricing
            .get("provider", "model", None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Decode
    );
    w.expect(
        Plane::Public,
        Method::GET,
        "/model-configs/pricing/provider/model",
        &[],
        None,
        Reply::Json(
            200,
            json!({"calculate":{"request":{"operation":"sum","operands":[{"value":123}]}}}),
        ),
    );
    assert_eq!(
        c.pricing
            .get("provider", "model", None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Decode
    );
    w.done();
}
#[tokio::test]
async fn typed_patch_rotation_and_extension_collision_states_remain_explicit() {
    let (c, w, t) = clients();
    let mut input: GatewayApiKeyUpdateRequest = body(json!({"name":"updated"}));
    input.usage_limits = Patch::Null;
    check!(
        w,
        Plane::Data,
        Method::PUT,
        &format!("/api-keys/service/{ID}"),
        &[],
        Some(json!({"name":"updated","usage_limits":null})),
        json!({}),
        c.api.update_service(ID, &input, None)
    );
    let mut input = GatewayApiKeyRotateRequest::builder().build().unwrap();
    input.key_transition_period_ms = Optional::Value(1_800_000.into());
    check!(
        w,
        Plane::Data,
        Method::POST,
        &format!("/api-keys/service/{ID}/rotate"),
        &[],
        Some(json!({"key_transition_period_ms":1_800_000})),
        json!({"id":ID,"key":"response-secret","key_transition_expires_at":"expiry"}),
        c.api.rotate_service(ID, Some(&input), None)
    );
    let before = t.calls.load(Ordering::SeqCst);
    let mut input: GatewayIntegrationCreateRequest = body(integration_body());
    input.extensions.insert(
        "ai_provider_id".into(),
        JsonValue::new(json!("request-secret")),
    );
    rejected(c.integrations.create(&input, None).await.unwrap_err());
    assert_eq!(before, t.calls.load(Ordering::SeqCst));
    w.done();
}
#[tokio::test]
async fn provider_resolution_selects_first_match_and_unknown_errors_omit_input_values() {
    let (c, w, _) = clients();
    w.expect(Plane::Admin,Method::GET,"/utils/static-resources/ai-providers",&[],None,Reply::Json(200,json!({"data":[{"id":ID,"slug":"OPEN-AI","name":"first","status":"active"},{"id":OTHER,"slug":"open-ai","name":"second","status":"active"}]})));
    assert_eq!(
        c.integrations
            .resolve_provider_id("Open-AI", None)
            .await
            .unwrap(),
        ID
    );
    w.expect(
        Plane::Admin,
        Method::GET,
        "/utils/static-resources/ai-providers",
        &[],
        None,
        Reply::Json(200, json!({"data":[]})),
    );
    let e = c
        .integrations
        .resolve_provider_id("request-secret", None)
        .await
        .unwrap_err();
    assert_eq!(e.kind(), ErrorKind::Validation);
    assert!(!format!("{e:?} {e}").contains("request-secret"));
    w.done();
}
