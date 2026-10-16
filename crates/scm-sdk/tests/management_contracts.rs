//! Literal, independent management fixtures from frozen src/management and model contracts.
#![cfg(feature = "management")]
use bytes::Bytes;
use futures_util::{future::BoxFuture, stream};
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, OriginPolicy, RemoteOutcome, RequestOptions, Result,
    Secret,
    airs::management::{models::*, *},
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    data::{FiniteNumber, Optional},
    pagination::{Completion, Truncation},
    transport::{Transport, TransportRequest, TransportResponse},
};
use serde_json::{Value, json};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
const ID: &str = "550e8400-e29b-41d4-a716-446655440000";
struct Token {
    context: AuthContext,
    calls: AtomicUsize,
}
impl Token {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            context: AuthContext::new(
                Endpoint::parse("https://issuer.example/token").unwrap(),
                "fixture-client",
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
                Secret::new("fixture-bearer").unwrap(),
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
struct Fixture {
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    body: Option<Value>,
    reply: Option<Value>,
    tenant: bool,
    failure: bool,
    delay: Duration,
}
#[derive(Default)]
struct Wire {
    fixtures: Mutex<VecDeque<Fixture>>,
    requests: AtomicUsize,
}
impl Wire {
    fn expect(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<Value>,
        reply: Value,
    ) {
        self.push(method, path, query, body, Some(reply), (false, false));
    }
    fn dashboard(&self, path: &str, query: &[(&str, &str)], reply: Option<Value>) {
        self.push(Method::GET, path, query, None, reply, (true, false));
    }
    fn push(
        &self,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<Value>,
        reply: Option<Value>,
        flags: (bool, bool),
    ) {
        self.fixtures.lock().unwrap().push_back(Fixture {
            method,
            path: path.into(),
            query: query
                .iter()
                .map(|(k, v)| ((*k).into(), (*v).into()))
                .collect(),
            body,
            reply,
            tenant: flags.0,
            failure: flags.1,
            delay: Duration::ZERO,
        });
    }
    fn done(&self) {
        assert!(
            self.fixtures.lock().unwrap().is_empty(),
            "unconsumed independently authored fixture"
        );
    }
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.fetch_add(1, Ordering::SeqCst);
            let expected = self
                .fixtures
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected network work or mutation replay");
            assert_eq!(request.method(), expected.method);
            let expected_url = if expected.path.starts_with("https://") {
                expected.path
            } else {
                format!("https://management.example/aisec{}", expected.path)
            };
            let mut actual_url = request.url().clone();
            actual_url.set_query(None);
            assert_eq!(actual_url.as_str(), expected_url);
            let ordered: Vec<(String, String)> = request.url().query_pairs().into_owned().collect();
            assert_eq!(ordered, expected.query);
            let actual: BTreeMap<String, String> = ordered.into_iter().collect();
            assert_eq!(actual, expected.query.into_iter().collect());
            assert_eq!(
                request.url().query_pairs().count(),
                actual.len(),
                "duplicate query keys"
            );
            let headers = request.headers();
            let mut names: Vec<_> = headers.keys().map(|name| name.as_str()).collect();
            names.sort_unstable();
            let mut wanted = vec!["authorization", "service-name", "user-agent"];
            if expected.tenant {
                wanted.push("x-tsg-id");
                assert_eq!(headers["x-tsg-id"], "123456789");
            }
            if expected.body.is_some() {
                wanted.push("content-type");
                assert_eq!(headers["content-type"], "application/json");
            }
            wanted.sort_unstable();
            assert_eq!(names, wanted);
            assert_eq!(headers.len(), wanted.len());
            assert_eq!(headers["authorization"], "Bearer fixture-bearer");
            assert!(headers["authorization"].is_sensitive());
            assert_eq!(headers["service-name"], "api");
            assert_eq!(
                headers["user-agent"],
                concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
            );
            if let Some(body) = expected.body {
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
            if expected.failure {
                return Err(Error::new(ErrorKind::Transport));
            }
            let body = expected
                .reply
                .map(|value| Bytes::from(serde_json::to_vec(&value).unwrap()))
                .unwrap_or_default();
            let mut response_headers = HeaderMap::new();
            response_headers.insert("content-type", HeaderValue::from_static("application/json"));
            Ok(TransportResponse::new(
                StatusCode::OK,
                response_headers,
                Box::pin(stream::iter([Ok(body)])),
            ))
        })
    }
}
fn clients() -> (ManagementClient, Arc<Wire>, Arc<Token>) {
    let wire = Arc::new(Wire::default());
    let token = Token::new();
    let client = Client::builder(Endpoint::parse("https://management.example/aisec").unwrap())
        .transport(wire.clone())
        .authentication(Authentication::oauth(token.clone(), false))
        .build()
        .unwrap();
    (ManagementClient::new(client).unwrap(), wire, token)
}
fn profile(id: &str, name: &str, revision: i64) -> Value {
    json!({"profile_id":id,"profile_name":name,"revision":revision,"future":{"retained":true}})
}
fn topic(id: &str, name: &str, revision: i64) -> Value {
    json!({"topic_id":id,"topic_name":name,"revision":revision,"description":"fixture","examples":[]})
}
fn key() -> Value {
    json!({"api_key_id":"key-id","api_key_last8":"last1234","auth_code":"fixture-code","expiration":"2030-01-01","revoked":false})
}
fn app() -> Value {
    json!({"customer_appId":"app-id","tsg_id":"123456789","app_name":"fixture app","cloud_provider":"aws","environment":"prod"})
}
fn create_profile() -> CreateSecurityProfileRequest {
    CreateSecurityProfileRequest::parse(
        json!({"profile_name":"fixture","policy":{}}),
        scm_sdk::validation::Mode::Request,
    )
    .unwrap()
}
fn create_topic() -> CreateCustomTopicRequest {
    CreateCustomTopicRequest::parse(
        json!({"topic_name":"fixture"}),
        scm_sdk::validation::Mode::Request,
    )
    .unwrap()
}
fn create_key_json() -> Value {
    json!({"auth_code":"fixture-code","cust_app":"fixture app","revoked":false,"created_by":"fixture@example.test","api_key_name":"fixture key","rotation_time_interval":30,"rotation_time_unit":"days"})
}
fn create_key() -> ApiKeyCreateRequest {
    ApiKeyCreateRequest::parse(create_key_json(), scm_sdk::validation::Mode::Request).unwrap()
}
fn regenerate_key() -> ApiKeyRegenerateRequest {
    ApiKeyRegenerateRequest::parse(
        json!({"rotation_time_interval":30,"rotation_time_unit":"days"}),
        scm_sdk::validation::Mode::Request,
    )
    .unwrap()
}
const PAGE: &[(&str, &str)] = &[("offset", "0"), ("limit", "100")];
#[tokio::test]
async fn profile_methods_match_literal_routes_schemas_defaults_and_helpers() {
    let (management, wire, _) = clients();
    let client = management.profiles();
    let item = profile(ID, "fixture", 2);
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles",
        PAGE,
        None,
        json!({"ai_profiles":[item.clone()]}),
    );
    client
        .list_for_token(PaginationOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Method::POST,
        "/v1/mgmt/profile",
        &[],
        Some(json!({"profile_name":"fixture","policy":{}})),
        item.clone(),
    );
    client.create(&create_profile(), None).await.unwrap();
    let mut page = PaginationOptions::default();
    page.offset = 10;
    page.limit = 20;
    page.latest = Some(false);
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        &[("offset", "10"), ("limit", "20"), ("latest", "false")],
        None,
        json!({"ai_profiles":[]}),
    );
    client.list(page, None).await.unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        PAGE,
        None,
        json!({"ai_profiles":[item.clone()],"next_offset":2}),
    );
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        &[("offset", "2"), ("limit", "100")],
        None,
        json!({"ai_profiles":[profile("other","fixture",5)],"next_offset":0}),
    );
    let all = client
        .list_all(ProfileListAllOptions::default(), None)
        .await
        .unwrap();
    assert!(all.is_complete());
    assert_eq!(all.items.len(), 2);
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        PAGE,
        None,
        json!({"ai_profiles":[item.clone()]}),
    );
    assert_eq!(client.get(ID, None).await.unwrap().profile_name, "fixture");
    wire.expect(Method::GET,"/v1/mgmt/profiles/tsg/123456789",PAGE,None,json!({"ai_profiles":[item.clone(),profile("latest","fixture",5),profile("tie","fixture",5)]}));
    assert!(
        matches!(client.get_by_name("fixture",None).await.unwrap().profile_id,Optional::Value(id) if id=="latest")
    );
    wire.expect(
        Method::PUT,
        &format!("/v1/mgmt/profile/uuid/{ID}"),
        &[],
        Some(json!({"profile_name":"fixture","policy":{}})),
        item,
    );
    client.update(ID, &create_profile(), None).await.unwrap();
    wire.expect(
        Method::DELETE,
        &format!("/v1/mgmt/profile/{ID}"),
        &[],
        None,
        json!("deleted"),
    );
    assert_eq!(
        serde_json::to_value(client.delete(ID, None).await.unwrap()).unwrap(),
        json!({"message":"deleted"})
    );
    wire.expect(
        Method::DELETE,
        &format!("/v1/mgmt/profile/{ID}/force"),
        &[("updated_by", "fixture@example.test")],
        None,
        json!({"message":"deleted"}),
    );
    client
        .force_delete(ID, "fixture@example.test", None)
        .await
        .unwrap();
    wire.done();
}
#[tokio::test]
async fn topic_methods_match_literal_routes_and_latest_revision_workflow() {
    let (management, wire, _) = clients();
    let client = management.topics();
    let item = topic(ID, "fixture", 2);
    wire.expect(
        Method::GET,
        "/v1/mgmt/topics",
        PAGE,
        None,
        json!({"custom_topics":[item.clone()]}),
    );
    client
        .list_for_token(PaginationOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Method::POST,
        "/v1/mgmt/topic",
        &[],
        Some(json!({"topic_name":"fixture"})),
        item.clone(),
    );
    client.create(&create_topic(), None).await.unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/topics/tsg/123456789",
        PAGE,
        None,
        json!({"custom_topics":[item.clone()]}),
    );
    client
        .list(TopicListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/topics/tsg/123456789",
        PAGE,
        None,
        json!({"custom_topics":[item.clone()],"next_offset":2}),
    );
    wire.expect(
        Method::GET,
        "/v1/mgmt/topics/tsg/123456789",
        &[("offset", "2"), ("limit", "100")],
        None,
        json!({"custom_topics":[topic("later","fixture",4)]}),
    );
    assert_eq!(
        client
            .list_all(TopicListAllOptions::default(), None)
            .await
            .unwrap()
            .items
            .len(),
        2
    );
    wire.expect(
        Method::GET,
        "/v1/mgmt/topics/tsg/123456789",
        PAGE,
        None,
        json!({"custom_topics":[item.clone()]}),
    );
    client.get(ID, None).await.unwrap();
    wire.expect(Method::GET,"/v1/mgmt/topics/tsg/123456789",PAGE,None,json!({"custom_topics":[item.clone(),topic("latest","fixture",4),topic("tie","fixture",4)]}));
    assert!(
        matches!(client.get_by_name("fixture",None).await.unwrap().topic_id,Optional::Value(id) if id=="latest")
    );
    wire.expect(
        Method::PUT,
        &format!("/v1/mgmt/topic/uuid/{ID}"),
        &[],
        Some(json!({"topic_name":"fixture"})),
        item.clone(),
    );
    client.update(ID, &create_topic(), None).await.unwrap();
    wire.expect(
        Method::DELETE,
        &format!("/v1/mgmt/topic/{ID}"),
        &[],
        None,
        json!("deleted"),
    );
    client.delete(ID, None).await.unwrap();
    wire.expect(
        Method::DELETE,
        &format!("/v1/mgmt/topic/{ID}/force"),
        &[("updated_by", "fixture@example.test")],
        None,
        json!("deleted"),
    );
    client
        .force_delete_with_audit(ID, "fixture@example.test", None)
        .await
        .unwrap();
    wire.expect(
        Method::DELETE,
        &format!("/v1/mgmt/topic/{ID}/force"),
        &[("updated_by", "fixture@example.test")],
        None,
        json!("deleted"),
    );
    client
        .force_delete(ID, Some("fixture@example.test"), None)
        .await
        .unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/topics/tsg/123456789",
        &[("offset", "0"), ("limit", "200")],
        None,
        json!({"custom_topics":[topic("old-a","A",1),topic("b","B",2)],"next_offset":2}),
    );
    wire.expect(
        Method::GET,
        "/v1/mgmt/topics/tsg/123456789",
        &[("offset", "2"), ("limit", "200")],
        None,
        json!({"custom_topics":[topic("new-a","A",3),topic("tie-a","A",3)]}),
    );
    let mut latest = TopicListOptions::default();
    latest.latest_only = true;
    latest.limit = 1;
    let selected = client.list(latest, None).await.unwrap();
    let topics = selected.custom_topics.into_option().unwrap();
    assert!(matches!(&topics[0].topic_id,Optional::Value(id) if id=="new-a"));
    assert_eq!(
        selected.next_offset.into_option().unwrap().as_u64(),
        Some(1)
    );
    wire.done();
}
#[tokio::test]
async fn api_key_methods_match_literal_routes_bodies_and_optional_empty_inventory() {
    let (management, wire, _) = clients();
    let client = management.api_keys();
    wire.expect(Method::GET, "/v1/mgmt/apikeys", PAGE, None, json!({}));
    client
        .list_for_token(PaginationOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Method::POST,
        "/v1/mgmt/apikey",
        &[],
        Some(create_key_json()),
        key(),
    );
    client.create(&create_key(), None).await.unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/apikeys/tsg/123456789",
        PAGE,
        None,
        json!({"api_keys":[key()]}),
    );
    client
        .list(PaginationOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/apikeys/tsg/123456789",
        PAGE,
        None,
        json!({}),
    );
    assert!(
        client
            .list_all(ApiKeyListAllOptions::default(), None)
            .await
            .unwrap()
            .items
            .is_empty()
    );
    wire.expect(
        Method::DELETE,
        "/v1/mgmt/apikey/delete/fixture%20key",
        &[("updated_by", "fixture@example.test")],
        None,
        json!("deleted"),
    );
    client
        .delete("fixture key", "fixture@example.test", None)
        .await
        .unwrap();
    wire.expect(
        Method::POST,
        "/v1/mgmt/apikey/regenerate/key-id",
        &[],
        Some(json!({"rotation_time_interval":30,"rotation_time_unit":"days"})),
        key(),
    );
    client
        .regenerate("key-id", &regenerate_key(), None)
        .await
        .unwrap();
    wire.done();
}
#[tokio::test]
async fn customer_app_methods_match_distinct_token_and_tenant_routes() {
    let (management, wire, _) = clients();
    let client = management.customer_apps();
    wire.expect(
        Method::GET,
        "/v1/mgmt/customerapps",
        PAGE,
        None,
        json!({"customer_apps":[app()]}),
    );
    client
        .list_for_token(PaginationOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/customerapp",
        &[("app_name", "fixture app")],
        None,
        app(),
    );
    client.get("fixture app", None).await.unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/customerapp/tsg/123456789",
        PAGE,
        None,
        json!({"customer_apps":[app()]}),
    );
    client
        .list(PaginationOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/customerapp/tsg/123456789",
        PAGE,
        None,
        json!({"customer_apps":[app()],"next_offset":2}),
    );
    wire.expect(
        Method::GET,
        "/v1/mgmt/customerapp/tsg/123456789",
        &[("offset", "2"), ("limit", "100")],
        None,
        json!({}),
    );
    assert_eq!(
        client
            .list_all(CustomerAppListAllOptions::default(), None)
            .await
            .unwrap()
            .items
            .len(),
        1
    );
    let body = CustomerApp::parse(app(), scm_sdk::validation::Mode::Request).unwrap();
    wire.expect(
        Method::PUT,
        "/v1/mgmt/customerapp",
        &[("customer_app_id", "app-id")],
        Some(app()),
        app(),
    );
    client.update("app-id", &body, None).await.unwrap();
    wire.expect(
        Method::DELETE,
        "/v1/mgmt/customerapp",
        &[
            ("app_name", "fixture app"),
            ("updated_by", "fixture@example.test"),
        ],
        None,
        json!("deleted"),
    );
    assert_eq!(
        serde_json::to_value(
            client
                .delete("fixture app", "fixture@example.test", None)
                .await
                .unwrap()
        )
        .unwrap(),
        json!({"message":"deleted"})
    );
    wire.done();
}
#[tokio::test]
async fn deployment_legacy_dlp_and_scan_log_methods_match_frozen_dialects() {
    let (management, wire, _) = clients();
    let mut deployment = DeploymentProfileListOptions::default();
    deployment.unactivated = Some(false);
    wire.expect(
        Method::GET,
        "/v1/mgmt/deploymentprofiles",
        &[("unactivated", "false")],
        None,
        json!({"deployment_profiles":[],"status":"ok"}),
    );
    management
        .deployment_profiles()
        .list(deployment, None)
        .await
        .unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/dlpprofiles",
        &[],
        None,
        json!({"dlp_profiles":[]}),
    );
    management.dlp_profiles().list(None).await.unwrap();
    let mut query = ScanLogQueryOptions::new(24, "hour", 1, 10, "threat");
    query.page_token = Some("fixture-page".into());
    wire.expect(
        Method::POST,
        "/v1/mgmt/scanlogs",
        &[
            ("time_interval", "24"),
            ("time_unit", "hour"),
            ("pageNumber", "1"),
            ("pageSize", "10"),
            ("filter", "threat"),
        ],
        Some(json!({"page_token":"fixture-page"})),
        json!({"page_number":1,"page_size":10,"total_pages":1}),
    );
    management.scan_logs().query(&query, None).await.unwrap();
    query.page_token = Some(String::new());
    wire.expect(
        Method::POST,
        "/v1/mgmt/scanlogs",
        &[
            ("time_interval", "24"),
            ("time_unit", "hour"),
            ("pageNumber", "1"),
            ("pageSize", "10"),
            ("filter", "threat"),
        ],
        None,
        json!({}),
    );
    management.scan_logs().query(&query, None).await.unwrap();
    wire.done();
}
#[tokio::test]
async fn all_twenty_two_dashboard_methods_preserve_queries_headers_and_raw_values() {
    let (management, wire, _) = clients();
    let dashboard = management.dashboard();
    let app = DashboardAppQuery::new("app-id", "fixture app");
    let time = DashboardTimeRangeQuery::default();
    let overview = DashboardSessionsOverviewQuery::default();
    let session = DashboardSessionQuery::new("session-id", "app-id", "fixture app");
    let transaction =
        DashboardSessionTransactionQuery::new("session-id", "app-id", "fixture app", "scan-id", 2);
    let content = DashboardScanContentQuery::new("scan-id", 2);
    let applications = DashboardApplicationsOverviewQuery::default();
    let app_query = &[
        ("appid", "app-id"),
        ("appname", "fixture app"),
        ("time_interval", "30"),
        ("time_unit", "days"),
    ];
    let time_query = &[("time_interval", "1"), ("time_unit", "day")];
    let window = &[("time_interval", "30"), ("time_unit", "days")];
    macro_rules! pair{($path:expr,$query:expr,$reply:expr,$typed:expr,$raw:expr)=>{{
        let expected=$reply;wire.dashboard($path,$query,Some(expected.clone()));let result=$typed.await.unwrap();assert_eq!(serde_json::to_value(result).unwrap(),expected);
        let opaque=json!({"unrecognized_response":[null,"sensitive fixture"]});wire.dashboard($path,$query,Some(opaque.clone()));assert_eq!($raw.await.unwrap().unwrap().into_value(),opaque);
    }};}
    pair!(
        "/v1/mgmt/dashboard/v2/apps/application",
        app_query,
        json!({"token_stats":null,"session_stats":null}),
        dashboard.application(&app, None),
        dashboard.application_raw(&app, None)
    );
    pair!(
        "/v1/mgmt/dashboard/v2/apps/applicationviolationbreakdown",
        app_query,
        json!({"detection_type_violation_breakdown":[]}),
        dashboard.application_violation_breakdown(&app, None),
        dashboard.application_violation_breakdown_raw(&app, None)
    );
    pair!(
        "/v1/mgmt/dashboard/v2/apps/topapplicationsviolations",
        time_query,
        json!({"applications":[]}),
        dashboard.top_applications_violations(&time, None),
        dashboard.top_applications_violations_raw(&time, None)
    );
    pair!(
        "/v1/mgmt/dashboard/v2/apps/applicationsviolationstrend",
        time_query,
        json!({"violations":[]}),
        dashboard.applications_violations_trend(&time, None),
        dashboard.applications_violations_trend_raw(&time, None)
    );
    pair!(
        "/v1/mgmt/dashboard/v2/apps/appslist",
        window,
        json!({"applications":[]}),
        dashboard.apps_list(&time, None),
        dashboard.apps_list_raw(&time, None)
    );
    pair!(
        "/v1/mgmt/dashboard/v2/sessions/sessionschart",
        time_query,
        json!({"bucket_size_seconds":60,"buckets":[]}),
        dashboard.sessions_chart(&time, None),
        dashboard.sessions_chart_raw(&time, None)
    );
    pair!(
        "/v1/mgmt/dashboard/v2/sessions/sessionsoverview",
        &[
            ("time_interval", "1"),
            ("time_unit", "day"),
            ("limit", "25"),
            ("offset", "0")
        ],
        json!({"items":[],"pagination":{}}),
        dashboard.sessions_overview(&overview, None),
        dashboard.sessions_overview_raw(&overview, None)
    );
    pair!(
        "/v1/mgmt/dashboard/v2/sessions/session",
        &[
            ("session_id", "session-id"),
            ("app_id", "app-id"),
            ("app_name", "fixture app"),
            ("offset", "0"),
            ("limit", "25"),
            ("time_interval", "30"),
            ("time_unit", "days")
        ],
        json!({"application":{"id":"app-id","name":"fixture app"},"id":"session-id","session_actions":[],"pagination":{}}),
        dashboard.session(&session, None),
        dashboard.session_raw(&session, None)
    );
    pair!(
        "/v1/mgmt/dashboard/v2/sessions/sessiontransaction",
        &[
            ("session_id", "session-id"),
            ("app_id", "app-id"),
            ("app_name", "fixture app"),
            ("scan_id", "scan-id"),
            ("scan_sub_req_id", "2"),
            ("time_interval", "30"),
            ("time_unit", "days")
        ],
        json!({"application_id":"app-id","application_name":"fixture app","session_id":"session-id","scan_id":"scan-id","scan_sub_req_id":2,"detection_type_violation_breakdown":[],"violation_breakdown":{}}),
        dashboard.session_transaction(&transaction, None),
        dashboard.session_transaction_raw(&transaction, None)
    );
    pair!(
        "/v1/mgmt/reports/scancontent",
        &[("scan_id", "scan-id"), ("scan_sub_req_id", "2")],
        json!({"report_id":"report-id","scan_id":"scan-id","sub_scan_req_id":2,"transaction_id":"transaction-id","scan_contents":null}),
        dashboard.scan_content(&content, None),
        dashboard.scan_content_raw(&content, None)
    );
    pair!(
        "/v1/mgmt/dashboard/v2/apps/applicationsoverview",
        &[
            ("time_interval", "30"),
            ("time_unit", "days"),
            ("limit", "25"),
            ("offset", "0")
        ],
        json!({"items":[]}),
        dashboard.applications_overview(&applications, None),
        dashboard.applications_overview_raw(&applications, None)
    );
    assert_eq!(wire.requests.load(Ordering::SeqCst), 22);
    wire.done();
}
#[tokio::test]
async fn raw_dashboard_empty_null_and_typed_missing_body_remain_distinct() {
    let (management, wire, _) = clients();
    let dashboard = management.dashboard();
    let query = DashboardAppQuery::new("id", "name");
    let params = &[
        ("appid", "id"),
        ("appname", "name"),
        ("time_interval", "30"),
        ("time_unit", "days"),
    ];
    wire.dashboard("/v1/mgmt/dashboard/v2/apps/application", params, None);
    assert!(
        dashboard
            .application_raw(&query, None)
            .await
            .unwrap()
            .is_none()
    );
    wire.dashboard(
        "/v1/mgmt/dashboard/v2/apps/application",
        params,
        Some(Value::Null),
    );
    assert!(
        dashboard
            .application_raw(&query, None)
            .await
            .unwrap()
            .unwrap()
            .as_value()
            .is_null()
    );
    wire.dashboard("/v1/mgmt/dashboard/v2/apps/application", params, None);
    assert_eq!(
        dashboard
            .application(&query, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Decode
    );
    wire.done();
}
#[tokio::test]
async fn invalid_parameters_and_models_fail_before_authentication_or_transport() {
    let (management, wire, token) = clients();
    let mut page = PaginationOptions::default();
    page.limit = 0;
    assert!(management.profiles().list(page, None).await.is_err());
    assert!(
        management
            .topics()
            .list_for_token(page, None)
            .await
            .is_err()
    );
    assert!(management.api_keys().list(page, None).await.is_err());
    assert!(management.customer_apps().list(page, None).await.is_err());
    assert!(
        management
            .profiles()
            .delete("not-uuid", None)
            .await
            .is_err()
    );
    assert!(
        management
            .profiles()
            .update("not-uuid", &create_profile(), None)
            .await
            .is_err()
    );
    assert!(
        management
            .profiles()
            .force_delete("not-uuid", "email", None)
            .await
            .is_err()
    );
    assert!(
        management
            .topics()
            .update("not-uuid", &create_topic(), None)
            .await
            .is_err()
    );
    assert!(management.topics().delete("not-uuid", None).await.is_err());
    assert!(
        management
            .topics()
            .force_delete(ID, None, None)
            .await
            .is_err()
    );
    assert!(
        management
            .topics()
            .force_delete_with_audit(ID, "\u{feff}\u{00a0}", None)
            .await
            .is_err()
    );
    assert!(
        management
            .api_keys()
            .delete("../escape", "email", None)
            .await
            .is_err()
    );
    assert!(
        management
            .api_keys()
            .regenerate("%252e%252e", &regenerate_key(), None)
            .await
            .is_err()
    );
    let mut bad_profile = create_profile();
    bad_profile.revision = Optional::Value(FiniteNumber::try_from(1.5).unwrap());
    assert!(
        management
            .profiles()
            .create(&bad_profile, None)
            .await
            .is_err()
    );
    assert!(
        management
            .profiles()
            .update(ID, &bad_profile, None)
            .await
            .is_err()
    );
    let mut bad_topic = create_topic();
    bad_topic.revision = Optional::Value(FiniteNumber::try_from(1.5).unwrap());
    assert!(management.topics().create(&bad_topic, None).await.is_err());
    let mut bad_key = create_key();
    bad_key.rotation_time_interval = FiniteNumber::try_from(1.5).unwrap();
    assert!(management.api_keys().create(&bad_key, None).await.is_err());
    let mut bad_regenerate = regenerate_key();
    bad_regenerate.rotation_time_interval = FiniteNumber::try_from(1.5).unwrap();
    assert!(
        management
            .api_keys()
            .regenerate("id", &bad_regenerate, None)
            .await
            .is_err()
    );
    let dashboard = management.dashboard();
    assert!(
        dashboard
            .application(&DashboardAppQuery::new("\u{feff}", "private"), None)
            .await
            .is_err()
    );
    let mut time = DashboardTimeRangeQuery::default();
    time.time_interval = Some(0);
    assert!(dashboard.sessions_chart(&time, None).await.is_err());
    let mut overview = DashboardSessionsOverviewQuery::default();
    overview.limit = Some(0);
    assert!(dashboard.sessions_overview(&overview, None).await.is_err());
    let mut session = DashboardSessionQuery::new("id", "id", "private");
    session.offset = Some(u64::MAX);
    assert!(dashboard.session(&session, None).await.is_err());
    assert!(
        dashboard
            .scan_content(&DashboardScanContentQuery::new("id", u64::MAX), None)
            .await
            .is_err()
    );
    let mut applications = DashboardApplicationsOverviewQuery::default();
    applications.time_interval = Some(2);
    assert!(
        dashboard
            .applications_overview(&applications, None)
            .await
            .is_err()
    );
    assert!(
        management
            .scan_logs()
            .query(&ScanLogQueryOptions::new(0, "hour", 1, 10, "all"), None)
            .await
            .is_err()
    );
    assert_eq!(token.calls.load(Ordering::SeqCst), 0);
    assert_eq!(wire.requests.load(Ordering::SeqCst), 0);
    wire.done();
}
#[tokio::test]
async fn every_management_mutation_reports_uncertainty_without_replay() {
    let (management, wire, _) = clients();
    let profiles = management.profiles();
    let topics = management.topics();
    let keys = management.api_keys();
    let apps = management.customer_apps();
    macro_rules! ambiguous {
        ($method:expr,$path:expr,$query:expr,$body:expr,$call:expr) => {{
            wire.push($method, $path, $query, $body, None, (false, true));
            let error = $call.await.unwrap_err();
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
            assert_eq!(error.kind(), ErrorKind::Transport);
        }};
    }
    ambiguous!(
        Method::POST,
        "/v1/mgmt/profile",
        &[],
        Some(json!({"profile_name":"fixture","policy":{}})),
        profiles.create(&create_profile(), None)
    );
    ambiguous!(
        Method::PUT,
        &format!("/v1/mgmt/profile/uuid/{ID}"),
        &[],
        Some(json!({"profile_name":"fixture","policy":{}})),
        profiles.update(ID, &create_profile(), None)
    );
    ambiguous!(
        Method::DELETE,
        &format!("/v1/mgmt/profile/{ID}"),
        &[],
        None,
        profiles.delete(ID, None)
    );
    ambiguous!(
        Method::DELETE,
        &format!("/v1/mgmt/profile/{ID}/force"),
        &[("updated_by", "email")],
        None,
        profiles.force_delete(ID, "email", None)
    );
    ambiguous!(
        Method::POST,
        "/v1/mgmt/topic",
        &[],
        Some(json!({"topic_name":"fixture"})),
        topics.create(&create_topic(), None)
    );
    ambiguous!(
        Method::PUT,
        &format!("/v1/mgmt/topic/uuid/{ID}"),
        &[],
        Some(json!({"topic_name":"fixture"})),
        topics.update(ID, &create_topic(), None)
    );
    ambiguous!(
        Method::DELETE,
        &format!("/v1/mgmt/topic/{ID}"),
        &[],
        None,
        topics.delete(ID, None)
    );
    ambiguous!(
        Method::DELETE,
        &format!("/v1/mgmt/topic/{ID}/force"),
        &[("updated_by", "email")],
        None,
        topics.force_delete_with_audit(ID, "email", None)
    );
    ambiguous!(
        Method::DELETE,
        &format!("/v1/mgmt/topic/{ID}/force"),
        &[("updated_by", "email")],
        None,
        topics.force_delete(ID, Some("email"), None)
    );
    ambiguous!(
        Method::POST,
        "/v1/mgmt/apikey",
        &[],
        Some(create_key_json()),
        keys.create(&create_key(), None)
    );
    ambiguous!(
        Method::POST,
        "/v1/mgmt/apikey/regenerate/id",
        &[],
        Some(json!({"rotation_time_interval":30,"rotation_time_unit":"days"})),
        keys.regenerate("id", &regenerate_key(), None)
    );
    ambiguous!(
        Method::DELETE,
        "/v1/mgmt/apikey/delete/name",
        &[("updated_by", "email")],
        None,
        keys.delete("name", "email", None)
    );
    let body = CustomerApp::parse(app(), scm_sdk::validation::Mode::Request).unwrap();
    ambiguous!(
        Method::PUT,
        "/v1/mgmt/customerapp",
        &[("customer_app_id", "id")],
        Some(app()),
        apps.update("id", &body, None)
    );
    ambiguous!(
        Method::DELETE,
        "/v1/mgmt/customerapp",
        &[("app_name", "name"), ("updated_by", "email")],
        None,
        apps.delete("name", "email", None)
    );
    ambiguous!(
        Method::POST,
        "/v1/mgmt/scanlogs",
        &[
            ("time_interval", "1"),
            ("time_unit", "day"),
            ("pageNumber", "1"),
            ("pageSize", "10"),
            ("filter", "all")
        ],
        None,
        management
            .scan_logs()
            .query(&ScanLogQueryOptions::new(1, "day", 1, 10, "all"), None)
    );
    assert_eq!(wire.requests.load(Ordering::SeqCst), 15);
    wire.done();
}
#[tokio::test]
async fn every_inventory_collection_exposes_truncation_and_resume() {
    let (management, wire, _) = clients();
    let mut profiles = ProfileListAllOptions::default();
    profiles.collect.max_items = 1;
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        PAGE,
        None,
        json!({"ai_profiles":[profile("one","one",1),profile("two","two",1)],"next_offset":2}),
    );
    let result = management
        .profiles()
        .list_all(profiles, None)
        .await
        .unwrap();
    assert_eq!(result.completion, Completion::Truncated(Truncation::Items));
    assert_eq!(result.items.len(), 1);
    let resume = result.resume.unwrap();
    assert_eq!((resume.cursor, resume.item_offset), (0, 1));
    let mut topics = TopicListAllOptions::default();
    topics.collect.max_items = 1;
    wire.expect(
        Method::GET,
        "/v1/mgmt/topics/tsg/123456789",
        PAGE,
        None,
        json!({"custom_topics":[topic("one","one",1),topic("two","two",1)],"next_offset":2}),
    );
    assert!(
        !management
            .topics()
            .list_all(topics, None)
            .await
            .unwrap()
            .is_complete()
    );
    let mut keys = ApiKeyListAllOptions::default();
    keys.collect.max_items = 1;
    wire.expect(
        Method::GET,
        "/v1/mgmt/apikeys/tsg/123456789",
        PAGE,
        None,
        json!({"api_keys":[key(),key()],"next_offset":2}),
    );
    assert!(
        !management
            .api_keys()
            .list_all(keys, None)
            .await
            .unwrap()
            .is_complete()
    );
    let mut apps = CustomerAppListAllOptions::default();
    apps.collect.max_items = 1;
    wire.expect(
        Method::GET,
        "/v1/mgmt/customerapp/tsg/123456789",
        PAGE,
        None,
        json!({"customer_apps":[app(),app()],"next_offset":2}),
    );
    assert!(
        !management
            .customer_apps()
            .list_all(apps, None)
            .await
            .unwrap()
            .is_complete()
    );
    wire.done();
}
#[tokio::test]
async fn profile_and_topic_lookups_never_claim_missing_or_latest_after_truncation() {
    let (management, wire, _) = clients();
    let profiles: Vec<_> = (0..10_001)
        .map(|index| profile(&index.to_string(), "name", index))
        .collect();
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        PAGE,
        None,
        json!({"ai_profiles":profiles}),
    );
    assert_eq!(
        management
            .profiles()
            .get("missing", None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    let topics: Vec<_> = (0..10_001)
        .map(|index| topic(&index.to_string(), "name", index))
        .collect();
    wire.expect(
        Method::GET,
        "/v1/mgmt/topics/tsg/123456789",
        PAGE,
        None,
        json!({"custom_topics":topics}),
    );
    assert_eq!(
        management
            .topics()
            .get_by_name("name", None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    wire.done();
}
#[tokio::test]
async fn invalid_and_repeated_offsets_preserve_partial_collection_failure() {
    let (management, wire, _) = clients();
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        PAGE,
        None,
        json!({"ai_profiles":[profile("one","one",1)],"next_offset":2}),
    );
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        &[("offset", "2"), ("limit", "100")],
        None,
        json!({"ai_profiles":[profile("two","two",1)],"next_offset":2}),
    );
    let failed = management
        .profiles()
        .list_all(ProfileListAllOptions::default(), None)
        .await
        .unwrap_err();
    assert_eq!(failed.error().kind(), ErrorKind::Pagination);
    assert_eq!(failed.partial().items.len(), 2);
    for offset in [json!(-1), json!(1.5), json!(9_007_199_254_740_992u64)] {
        wire.expect(
            Method::GET,
            "/v1/mgmt/topics/tsg/123456789",
            PAGE,
            None,
            json!({"custom_topics":[topic("one","one",1)],"next_offset":offset}),
        );
        assert_eq!(
            management
                .topics()
                .list_all(TopicListAllOptions::default(), None)
                .await
                .unwrap_err()
                .error()
                .kind(),
            ErrorKind::Decode
        );
    }
    wire.done();
}
#[tokio::test(start_paused = true)]
async fn page_workflow_uses_one_deadline_and_retains_partial_results() {
    let (management, wire, _) = clients();
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        PAGE,
        None,
        json!({"ai_profiles":[profile("one","one",1)],"next_offset":2}),
    );
    wire.fixtures.lock().unwrap().back_mut().unwrap().delay = Duration::from_millis(40);
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        &[("offset", "2"), ("limit", "100")],
        None,
        json!({"ai_profiles":[profile("two","two",1)]}),
    );
    wire.fixtures.lock().unwrap().back_mut().unwrap().delay = Duration::from_millis(40);
    let options = RequestOptions::with_timeout(Duration::from_millis(60)).unwrap();
    let start = tokio::time::Instant::now();
    let failure = management
        .profiles()
        .list_all(ProfileListAllOptions::default(), Some(&options))
        .await
        .unwrap_err();
    assert_eq!(failure.error().kind(), ErrorKind::Deadline);
    assert_eq!(failure.partial().items.len(), 1);
    assert!(tokio::time::Instant::now() - start <= Duration::from_millis(60));
    wire.done();
}
#[tokio::test]
async fn endpoint_overrides_preserve_policy_and_do_not_reroute_other_resources() {
    let (management, wire, token) = clients();
    assert!(
        management
            .clone()
            .with_dashboard_endpoint(Endpoint::parse("https://denied.example/aisec").unwrap())
            .is_err()
    );
    assert_eq!(token.calls.load(Ordering::SeqCst), 0);
    let endpoint = Endpoint::parse("https://management.example/aisec").unwrap();
    let dashboard_endpoint = Endpoint::parse("https://dashboard.example/dashboard-base").unwrap();
    let client = Client::builder(endpoint.clone())
        .transport(wire.clone())
        .destination_policy(Arc::new(OriginPolicy::new([
            endpoint,
            dashboard_endpoint.clone(),
        ])))
        .authentication(Authentication::oauth(token.clone(), false))
        .build()
        .unwrap();
    let management = ManagementClient::new(client)
        .unwrap()
        .with_dashboard_endpoint(dashboard_endpoint)
        .unwrap();
    wire.dashboard(
        "https://dashboard.example/dashboard-base/v1/mgmt/dashboard/v2/apps/appslist",
        &[("time_interval", "30"), ("time_unit", "days")],
        Some(json!({"applications":[]})),
    );
    management
        .dashboard()
        .apps_list(&DashboardTimeRangeQuery::default(), None)
        .await
        .unwrap();
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles",
        PAGE,
        None,
        json!({"ai_profiles":[]}),
    );
    management
        .profiles()
        .list_for_token(PaginationOptions::default(), None)
        .await
        .unwrap();
    #[cfg(feature = "dlp")]
    assert!(
        management
            .dlp(Endpoint::parse("https://api.dlp.paloaltonetworks.com").unwrap())
            .is_err()
    );
    wire.done();
}
#[test]
fn constructors_and_query_formatting_preserve_authentication_and_secrecy() {
    let client = Client::builder(Endpoint::parse("https://management.example/aisec").unwrap())
        .transport(Arc::new(Wire::default()))
        .build()
        .unwrap();
    assert!(ManagementClient::new(client.clone()).is_err());
    assert!(ProfilesClient::new(client.clone()).is_err());
    assert!(TopicsClient::new(client.clone()).is_err());
    assert!(ApiKeysClient::new(client.clone()).is_err());
    assert!(CustomerAppsClient::new(client.clone()).is_err());
    assert!(DashboardClient::new(client.clone()).is_err());
    assert!(DeploymentProfilesClient::new(client.clone()).is_err());
    assert!(DlpProfilesClient::new(client.clone()).is_err());
    assert!(ScanLogsClient::new(client).is_err());
    let confidential = "confidential-id-and-query";
    let output = format!(
        "{:?} {:?} {:?} {:?} {:?}",
        DashboardAppQuery::new(confidential, confidential),
        DashboardSessionQuery::new(confidential, confidential, confidential),
        DashboardSessionTransactionQuery::new(
            confidential,
            confidential,
            confidential,
            confidential,
            1
        ),
        DashboardScanContentQuery::new(confidential, 1),
        ScanLogQueryOptions::new(1, confidential, 1, 1, confidential)
    );
    assert!(!output.contains(confidential));
}
#[tokio::test]
async fn actual_empty_http_bodies_match_frozen_optional_and_required_schemas() {
    let (management, wire, _) = clients();
    wire.push(
        Method::GET,
        "/v1/mgmt/apikeys",
        PAGE,
        None,
        None,
        (false, false),
    );
    assert!(
        management
            .api_keys()
            .list_for_token(PaginationOptions::default(), None)
            .await
            .unwrap()
            .api_keys
            .is_unset()
    );
    wire.push(
        Method::GET,
        "/v1/mgmt/customerapps",
        PAGE,
        None,
        None,
        (false, false),
    );
    assert!(
        management
            .customer_apps()
            .list_for_token(PaginationOptions::default(), None)
            .await
            .unwrap()
            .customer_apps
            .is_unset()
    );
    wire.push(
        Method::GET,
        "/v1/mgmt/profiles",
        PAGE,
        None,
        None,
        (false, false),
    );
    assert!(
        management
            .profiles()
            .list_for_token(PaginationOptions::default(), None)
            .await
            .unwrap()
            .ai_profiles
            .into_option()
            .unwrap()
            .is_empty()
    );
    wire.push(
        Method::GET,
        "/v1/mgmt/topics",
        PAGE,
        None,
        None,
        (false, false),
    );
    assert!(
        management
            .topics()
            .list_for_token(PaginationOptions::default(), None)
            .await
            .unwrap()
            .custom_topics
            .into_option()
            .unwrap()
            .is_empty()
    );
    wire.push(
        Method::GET,
        "/v1/mgmt/dlpprofiles",
        &[],
        None,
        None,
        (false, false),
    );
    assert!(
        management
            .dlp_profiles()
            .list(None)
            .await
            .unwrap()
            .dlp_profiles
            .is_unset()
    );
    wire.push(
        Method::POST,
        "/v1/mgmt/scanlogs",
        &[
            ("time_interval", "1"),
            ("time_unit", "day"),
            ("pageNumber", "1"),
            ("pageSize", "10"),
            ("filter", "all"),
        ],
        None,
        None,
        (false, false),
    );
    assert_eq!(
        serde_json::to_value(
            management
                .scan_logs()
                .query(&ScanLogQueryOptions::new(1, "day", 1, 10, "all"), None)
                .await
                .unwrap()
        )
        .unwrap(),
        json!({})
    );
    wire.push(
        Method::DELETE,
        &format!("/v1/mgmt/profile/{ID}"),
        &[],
        None,
        None,
        (false, false),
    );
    assert_eq!(
        serde_json::to_value(management.profiles().delete(ID, None).await.unwrap()).unwrap(),
        json!({})
    );
    wire.push(
        Method::DELETE,
        &format!("/v1/mgmt/topic/{ID}"),
        &[],
        None,
        None,
        (false, false),
    );
    assert_eq!(
        serde_json::to_value(management.topics().delete(ID, None).await.unwrap()).unwrap(),
        json!({})
    );
    wire.push(
        Method::DELETE,
        "/v1/mgmt/apikey/delete/name",
        &[("updated_by", "email")],
        None,
        None,
        (false, false),
    );
    assert_eq!(
        serde_json::to_value(
            management
                .api_keys()
                .delete("name", "email", None)
                .await
                .unwrap()
        )
        .unwrap(),
        json!({})
    );
    wire.push(
        Method::GET,
        "/v1/mgmt/deploymentprofiles",
        &[],
        None,
        None,
        (false, false),
    );
    assert_eq!(
        management
            .deployment_profiles()
            .list(DeploymentProfileListOptions::default(), None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Decode
    );
    wire.push(
        Method::DELETE,
        "/v1/mgmt/customerapp",
        &[("app_name", "name"), ("updated_by", "email")],
        None,
        None,
        (false, false),
    );
    let error = management
        .customer_apps()
        .delete("name", "email", None)
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
    wire.done();
}
#[tokio::test]
async fn dashboard_nondefault_windows_paging_and_zero_subrequest_keep_wire_spelling() {
    let (management, wire, _) = clients();
    let dashboard = management.dashboard();
    let mut app = DashboardAppQuery::new("a", "private / unicode 日本");
    app.time_interval = Some(7);
    app.time_unit = Some("days".into());
    wire.dashboard(
        "/v1/mgmt/dashboard/v2/apps/application",
        &[
            ("appid", "a"),
            ("appname", "private / unicode 日本"),
            ("time_interval", "7"),
            ("time_unit", "days"),
        ],
        Some(json!({"token_stats":null,"session_stats":null})),
    );
    dashboard.application(&app, None).await.unwrap();
    let mut overview = DashboardSessionsOverviewQuery::default();
    overview.time_interval = Some(24);
    overview.time_unit = Some("hour".into());
    overview.offset = Some(4);
    overview.limit = Some(8);
    wire.dashboard(
        "/v1/mgmt/dashboard/v2/sessions/sessionsoverview",
        &[
            ("time_interval", "24"),
            ("time_unit", "hour"),
            ("limit", "8"),
            ("offset", "4"),
        ],
        Some(json!({"items":[],"pagination":{}})),
    );
    dashboard.sessions_overview(&overview, None).await.unwrap();
    let query = DashboardScanContentQuery::new("scan-id", 0);
    wire.dashboard("/v1/mgmt/reports/scancontent",&[("scan_id","scan-id"),("scan_sub_req_id","0")],Some(json!({"report_id":"report","scan_id":"scan-id","sub_scan_req_id":0,"transaction_id":"transaction","scan_contents":null})));
    dashboard.scan_content(&query, None).await.unwrap();
    let mut applications = DashboardApplicationsOverviewQuery::default();
    applications.time_interval = Some(1);
    applications.time_unit = Some("hour".into());
    applications.offset = Some(5);
    applications.limit = Some(6);
    wire.dashboard(
        "/v1/mgmt/dashboard/v2/apps/applicationsoverview",
        &[
            ("time_interval", "1"),
            ("time_unit", "hour"),
            ("limit", "6"),
            ("offset", "5"),
        ],
        Some(json!({"items":[]})),
    );
    dashboard
        .applications_overview(&applications, None)
        .await
        .unwrap();
    wire.done();
}
#[tokio::test]
async fn pages_are_lazy_and_caller_cancellation_stops_the_whole_walk() {
    let (management, wire, token) = clients();
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    let mut pages = management
        .profiles()
        .pages(PaginationOptions::default(), 10, Some(&options))
        .unwrap();
    assert_eq!(wire.requests.load(Ordering::SeqCst), 0);
    wire.expect(
        Method::GET,
        "/v1/mgmt/profiles/tsg/123456789",
        PAGE,
        None,
        json!({"ai_profiles":[profile("one","one",1)],"next_offset":2}),
    );
    assert_eq!(pages.next_page().await.unwrap().unwrap().items.len(), 1);
    options.cancel();
    assert_eq!(
        pages.next_page().await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(token.calls.load(Ordering::SeqCst), 1);
    wire.done();
}
#[tokio::test]
async fn oversized_dashboard_queries_reject_before_copy_authentication_and_network() {
    let (management, wire, token) = clients();
    let query = DashboardAppQuery::new("id", "confidential-query".repeat(2048));
    let error = management
        .dashboard()
        .application(&query, None)
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Limit);
    assert!(!format!("{error:?} {error}").contains("confidential-query"));
    assert_eq!(token.calls.load(Ordering::SeqCst), 0);
    assert_eq!(wire.requests.load(Ordering::SeqCst), 0);
    wire.done();
}
