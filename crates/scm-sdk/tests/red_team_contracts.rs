//! Frozen independent wire fixtures; expected responses never come from Rust models.
#![cfg(feature = "red-team")]
use bytes::Bytes;
use futures_util::{future::BoxFuture, stream};
use http::{HeaderMap, Method, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, Limits, OriginPolicy, RemoteOutcome, RequestOptions,
    Result, Secret,
    airs::red_team::{models::*, *},
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    pagination::{CollectOptions, Completion, Truncation},
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
const DATA: &str = "https://red-data.example/data-plane";
const MGMT: &str = "https://red-management.example/mgmt-plane";
const BROKER: &str = "https://red-broker.example/network-broker";
const FIXTURES: &str = include_str!("../src/fixtures/red-team-contracts.json");
struct Expected {
    method: Method,
    url: String,
    query: BTreeMap<String, Vec<String>>,
    body: Option<Value>,
    reply: Value,
    status: u16,
    fail: bool,
    delay: Duration,
    raw_reply: Option<Bytes>,
    operation: Option<String>,
}
#[derive(Default)]
struct Wire {
    expected: Mutex<VecDeque<Expected>>,
    requests: AtomicUsize,
}
impl Wire {
    fn push(
        &self,
        method: Method,
        base: &str,
        path: &str,
        query: BTreeMap<String, Vec<String>>,
        body: Option<Value>,
        reply: Value,
    ) {
        self.expected.lock().unwrap().push_back(Expected {
            method,
            url: format!("{base}{path}"),
            query,
            body,
            reply,
            status: 200,
            fail: false,
            delay: Duration::ZERO,
            raw_reply: None,
            operation: None,
        });
    }
    fn simple(&self, method: Method, base: &str, path: &str, body: Option<Value>, reply: Value) {
        self.push(method, base, path, BTreeMap::new(), body, reply);
    }
    fn fault(&self, status: u16, fail: bool) {
        let mut queue = self.expected.lock().unwrap();
        let row = queue.back_mut().unwrap();
        row.status = status;
        row.fail = fail;
    }
    fn delay(&self, value: Duration) {
        self.expected.lock().unwrap().back_mut().unwrap().delay = value;
    }
    fn count(&self) -> usize {
        self.requests.load(Ordering::SeqCst)
    }
    fn done(&self) {
        assert!(
            self.expected.lock().unwrap().is_empty(),
            "unconsumed independent fixture"
        );
    }
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.fetch_add(1, Ordering::SeqCst);
            let expected = self
                .expected
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected dispatch or mutation replay");
            assert_eq!(request.method(), expected.method);
            if let Some(operation) = &expected.operation {
                assert_eq!(request.operation(), operation);
            }
            let mut url = request.url().clone();
            url.set_query(None);
            assert_eq!(url.as_str(), expected.url);
            let mut query: BTreeMap<String, Vec<String>> = BTreeMap::new();
            for (key, value) in request.url().query_pairs() {
                query
                    .entry(key.into_owned())
                    .or_default()
                    .push(value.into_owned());
            }
            assert_eq!(query, expected.query);
            let mut names: Vec<_> = request
                .headers()
                .keys()
                .map(http::HeaderName::as_str)
                .collect();
            names.sort_unstable();
            let names_expected = if expected.body.is_some() {
                vec![
                    "authorization",
                    "content-type",
                    "service-name",
                    "user-agent",
                ]
            } else {
                vec!["authorization", "service-name", "user-agent"]
            };
            assert_eq!(names, names_expected);
            assert_eq!(request.headers().len(), names_expected.len());
            assert_eq!(
                request.headers()["authorization"],
                "Bearer synthetic-red-team-bearer"
            );
            assert!(request.headers()["authorization"].is_sensitive());
            assert_eq!(request.headers()["service-name"], "api");
            assert_eq!(
                request.headers()["user-agent"],
                concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
            );
            if let Some(body) = expected.body {
                assert_eq!(request.headers()["content-type"], "application/json");
                assert_eq!(
                    serde_json::from_slice::<Value>(request.body()).unwrap(),
                    body
                );
            } else {
                assert!(request.body().is_empty());
            }
            assert!(!format!("{request:?}").contains("synthetic-red-team-bearer"));
            if expected.fail {
                return Err(Error::new(ErrorKind::Transport));
            }
            tokio::time::sleep(expected.delay).await;
            Ok(TransportResponse::new(
                StatusCode::from_u16(expected.status).unwrap(),
                HeaderMap::new(),
                Box::pin(stream::iter([Ok(expected.raw_reply.unwrap_or_else(
                    || Bytes::from(serde_json::to_vec(&expected.reply).unwrap()),
                ))])),
            ))
        })
    }
}
struct Provider {
    context: AuthContext,
    calls: AtomicUsize,
    cancel_second: bool,
}
impl Provider {
    fn new(cancel_second: bool) -> Arc<Self> {
        Arc::new(Self {
            context: AuthContext::new(
                Endpoint::parse("https://issuer.example/token").unwrap(),
                "fixture-client",
                1,
                "123456789",
            )
            .unwrap(),
            calls: AtomicUsize::new(0),
            cancel_second,
        })
    }
}
impl TokenProvider for Provider {
    fn token<'a>(&'a self, options: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            let call = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
            if self.cancel_second && call == 2 {
                options.cancel();
            }
            TokenLease::new(
                Secret::new("synthetic-red-team-bearer").unwrap(),
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
fn clients(cancel_second: bool) -> (RedTeamClient, Arc<Wire>, Arc<Provider>) {
    let mut limits = Limits::default();
    limits.max_retries = 0;
    clients_with_limits(cancel_second, limits)
}
fn clients_with_limits(
    cancel_second: bool,
    limits: Limits,
) -> (RedTeamClient, Arc<Wire>, Arc<Provider>) {
    let wire = Arc::new(Wire::default());
    let provider = Provider::new(cancel_second);
    let data = Endpoint::parse(DATA).unwrap();
    let management = Endpoint::parse(MGMT).unwrap();
    let broker = Endpoint::parse(BROKER).unwrap();
    let raw = Client::builder(data.clone())
        .transport(wire.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .destination_policy(Arc::new(OriginPolicy::new([
            data,
            management.clone(),
            broker.clone(),
        ])))
        .limits(limits)
        .build()
        .unwrap();
    (
        RedTeamClient::new(raw, management, broker).unwrap(),
        wire,
        provider,
    )
}
// Keep the frozen corpus immutable; record each vendor-wire correction explicitly.
fn current_case(case: &Value) -> Value {
    let changes: Value = serde_json::from_str(include_str!(
        "../../../contracts/vendor/red-team-data-plane/0.93.0/legacy-fixture-corrections.json"
    ))
    .unwrap();
    let mut result = case.clone();
    for change in changes["corrections"].as_array().unwrap() {
        if case["call"]["className"] == change["class"]
            && case["call"]["member"] == change["method"]
        {
            let pointer = change["pointer"].as_str().unwrap();
            assert_eq!(result.pointer(pointer).unwrap(), &change["from"]);
            *result.pointer_mut(pointer).unwrap() = change["to"].clone();
        }
    }
    result
}
fn fixture(class: &str, method: &str) -> Value {
    let case = serde_json::from_str::<Value>(FIXTURES).unwrap()["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| {
            row["case"]["call"]["className"] == class && row["case"]["call"]["member"] == method
        })
        .unwrap()["case"]
        .clone();
    current_case(&case)
}
fn path(case: &Value) -> String {
    let mut path = case["path"].as_str().unwrap().to_owned();
    for name in [
        "job_id",
        "attack_id",
        "goal_id",
        "stream_id",
        "prompt_set_id",
        "prompt_id",
        "target_id",
    ] {
        path = path.replace(&format!("{{{name}}}"), ID);
    }
    path.replace("{target_type}", "APPLICATION")
}
fn arm(wire: &Wire, case: &Value) {
    let method = Method::from_bytes(case["method"].as_str().unwrap().as_bytes()).unwrap();
    let member = case["call"]["member"].as_str().unwrap();
    let base = if ["getDashboardOverview", "getManagementLanguages"].contains(&member) {
        MGMT
    } else {
        DATA
    };
    let mut query = BTreeMap::new();
    for (key, values) in case["queryWire"].as_object().unwrap() {
        query.insert(
            key.clone(),
            values
                .as_array()
                .unwrap()
                .iter()
                .map(|value| value.as_str().unwrap().into())
                .collect(),
        );
    }
    wire.push(
        method,
        base,
        &path(case),
        query,
        case.get("body").cloned(),
        case["responseBody"].clone(),
    );
}
fn qstr(query: &Value, key: &str) -> Option<String> {
    query.get(key).and_then(Value::as_str).map(str::to_owned)
}
fn listing(query: &Value) -> RedTeamListOptions {
    let mut value = RedTeamListOptions::default();
    value.skip = query["skip"].as_u64();
    value.limit = query["limit"].as_u64().map(|v| u32::try_from(v).unwrap());
    value.search = qstr(query, "search");
    value
}

#[tokio::test]
async fn all_thirty_eight_frozen_direct_fixtures_match_routes_queries_headers_bodies_and_responses()
{
    let (client, wire, _) = clients(false);
    let corpus: Value = serde_json::from_str(FIXTURES).unwrap();
    assert_eq!(
        corpus["baselineCommit"],
        "4a80dbdb3dc14365e1c25d838ac0bd139485d48c"
    );
    let rows = corpus["cases"].as_array().unwrap();
    assert_eq!(rows.len(), 38);
    macro_rules! parsed {
        ($body:expr) => {
            serde_json::from_value($body.clone()).unwrap()
        };
    }
    macro_rules! value {
        ($call:expr) => {
            serde_json::to_value($call.await.unwrap()).unwrap()
        };
    }
    for row in rows {
        let updated = current_case(&row["case"]);
        let case = &updated;
        arm(&wire, case);
        let query = &case["query"];
        let body = &case["body"];
        let actual = match (
            case["call"]["className"].as_str().unwrap(),
            case["call"]["member"].as_str().unwrap(),
        ) {
            ("RedTeamScansClient", "create") => value!(client.scans().create(&parsed!(body), None)),
            ("RedTeamScansClient", "list") => {
                let mut opts = RedTeamScanListOptions::from(listing(query));
                opts.status = qstr(query, "status");
                opts.job_type = qstr(query, "job_type");
                opts.target_id = qstr(query, "target_id");
                value!(client.scans().list(&opts, None))
            }
            ("RedTeamScansClient", "get") => value!(client.scans().get(ID, None)),
            ("RedTeamScansClient", "abort") => value!(client.scans().abort(ID, None)),
            ("RedTeamScansClient", "getCategories") => value!(client.scans().get_categories(None)),
            ("RedTeamClient", "getScanMetadata") => value!(client.get_scan_metadata(None)),
            ("RedTeamReportsClient", "listAttacks") => {
                let mut opts = AttackListOptions::default();
                opts.skip = query["skip"].as_u64();
                opts.limit = query["limit"].as_u64().map(|v| v as u32);
                opts.search = qstr(query, "search");
                opts.attack_status = qstr(query, "attack_status");
                opts.compliance = qstr(query, "compliance");
                opts.attack_modality = qstr(query, "attack_modality");
                opts.status = qstr(query, "status");
                opts.severity = qstr(query, "severity");
                opts.category = qstr(query, "category");
                opts.sub_category = qstr(query, "sub_category");
                opts.attack_type = qstr(query, "attack_type");
                opts.threat = query["threat"].as_bool();
                value!(client.reports().list_attacks(ID, &opts, None))
            }
            ("RedTeamReportsClient", "getAttackDetail") => {
                value!(client.reports().get_attack_detail(ID, ID, None))
            }
            ("RedTeamReportsClient", "getMultiTurnAttackDetail") => {
                value!(client.reports().get_multi_turn_attack_detail(ID, ID, None))
            }
            ("RedTeamReportsClient", "getStaticReport") => {
                value!(client.reports().get_static_report(ID, None))
            }
            ("RedTeamReportsClient", "getStaticRuntimePolicy") => {
                value!(client.reports().get_static_runtime_policy(ID, None))
            }
            ("RedTeamReportsClient", "getStaticRemediation") => {
                value!(client.reports().get_static_remediation(ID, None))
            }
            ("RedTeamReportsClient", "getDynamicReport") => {
                value!(client.reports().get_dynamic_report(ID, None))
            }
            ("RedTeamReportsClient", "getDynamicRuntimePolicy") => {
                value!(client.reports().get_dynamic_runtime_policy(ID, None))
            }
            ("RedTeamReportsClient", "getDynamicRemediation") => {
                value!(client.reports().get_dynamic_remediation(ID, None))
            }
            ("RedTeamReportsClient", "listGoals") => {
                let mut opts = GoalListOptions::default();
                opts.skip = query["skip"].as_u64();
                opts.limit = query["limit"].as_u64().map(|v| v as u32);
                opts.search = qstr(query, "search");
                opts.goal_category = qstr(query, "goal_category");
                opts.goal_type = qstr(query, "goal_type");
                opts.status = qstr(query, "status");
                opts.count = query["count"].as_bool();
                value!(client.reports().list_goals(ID, &opts, None))
            }
            ("RedTeamReportsClient", "listGoalStreams") => value!(
                client
                    .reports()
                    .list_goal_streams(ID, ID, &listing(query), None)
            ),
            ("RedTeamReportsClient", "getStreamDetail") => {
                value!(client.reports().get_stream_detail(ID, None))
            }
            ("RedTeamReportsClient", "downloadReport") => value!(client.reports().download_report(
                ID,
                query["file_format"].as_str().unwrap(),
                None
            )),
            ("RedTeamReportsClient", "generatePartialReport") => {
                value!(client.reports().generate_partial_report(ID, None))
            }
            ("RedTeamClient", "getQuota") => value!(client.get_quota(None)),
            ("RedTeamCustomAttackReportsClient", "getReport") => {
                value!(client.custom_attack_reports().get_report(ID, None))
            }
            ("RedTeamCustomAttackReportsClient", "getPromptSets") => {
                let mut opts = PromptSetsReportOptions::default();
                opts.skip = query["skip"].as_u64();
                opts.limit = query["limit"].as_u64().map(|v| v as u32);
                opts.search = qstr(query, "search");
                opts.property_filters = qstr(query, "property_filters");
                opts.is_threat = query["is_threat"].as_bool();
                value!(
                    client
                        .custom_attack_reports()
                        .get_prompt_sets(ID, &opts, None)
                )
            }
            ("RedTeamCustomAttackReportsClient", "getPromptsBySet") => {
                let mut opts = PromptsBySetListOptions::default();
                opts.skip = query["skip"].as_u64();
                opts.limit = query["limit"].as_u64().map(|v| v as u32);
                opts.search = qstr(query, "search");
                opts.is_threat = query["is_threat"].as_bool();
                value!(
                    client
                        .custom_attack_reports()
                        .get_prompts_by_set(ID, ID, &opts, None)
                )
            }
            ("RedTeamCustomAttackReportsClient", "getPromptDetail") => value!(
                client
                    .custom_attack_reports()
                    .get_prompt_detail(ID, ID, None)
            ),
            ("RedTeamCustomAttackReportsClient", "listCustomAttacks") => {
                let mut opts = CustomAttacksReportListOptions::default();
                opts.skip = query["skip"].as_u64();
                opts.limit = query["limit"].as_u64().map(|v| v as u32);
                opts.search = qstr(query, "search");
                opts.status = qstr(query, "status");
                opts.threat = query["threat"].as_bool();
                opts.prompt_set_id = qstr(query, "prompt_set_id");
                opts.property_value = qstr(query, "property_value");
                value!(
                    client
                        .custom_attack_reports()
                        .list_custom_attacks(ID, &opts, None)
                )
            }
            ("RedTeamCustomAttackReportsClient", "getAttackOutputs") => value!(
                client
                    .custom_attack_reports()
                    .get_attack_outputs(ID, ID, None)
            ),
            ("RedTeamCustomAttackReportsClient", "getPropertyStats") => {
                value!(client.custom_attack_reports().get_property_stats(ID, None))
            }
            ("RedTeamClient", "getErrorLogs") => {
                value!(client.get_error_logs(ID, &listing(query), None))
            }
            ("RedTeamClient", "getTargetProfileErrorLogs") => {
                value!(client.get_target_profile_error_logs(ID, &listing(query), None))
            }
            ("RedTeamClient", "getGoalCategories") => {
                value!(client.get_goal_categories("APPLICATION", None))
            }
            ("RedTeamClient", "getLanguages") => value!(client.get_languages(None)),
            ("RedTeamClient", "getManagementLanguages") => {
                value!(client.get_management_languages(None))
            }
            ("RedTeamClient", "getScanStatistics") => {
                value!(client.get_scan_statistics(&ScanStatisticsOptions::default(), None))
            }
            ("RedTeamClient", "getScoreTrend") => {
                let mut opts = ScoreTrendOptions::default();
                opts.date_range = qstr(query, "date_range");
                opts.start_date = qstr(query, "start_date");
                opts.end_date = qstr(query, "end_date");
                value!(client.get_score_trend(ID, &opts, None))
            }
            ("RedTeamClient", "updateSentiment") => {
                value!(client.update_sentiment(&parsed!(body), None))
            }
            ("RedTeamClient", "getSentiment") => value!(client.get_sentiment(ID, None)),
            ("RedTeamClient", "getDashboardOverview") => {
                value!(client.get_dashboard_overview(None))
            }
            key => panic!("uncovered frozen method {key:?}"),
        };
        assert_eq!(actual, case["responseBody"], "{}", case["call"]["member"]);
    }
    assert_eq!(wire.count(), 38);
    wire.done();
}

fn instance_body() -> InstanceRequest {
    serde_json::from_value(json!({
        "tsg_id": "123456789",
        "tenant_id": "tenant/a?b",
        "app_id": "airs-redteam",
        "region": "us-east-1",
        "future_request": true
    }))
    .unwrap()
}
fn device_body() -> DeviceRequest {
    serde_json::from_value(json!({
        "instance": {
            "tsg_id": "123456789",
            "tenant_id": "tenant/a?b",
            "app_id": "airs-redteam",
            "region": "us-east-1"
        },
        "devices": [
            {
                "serial_number": "SN-1",
                "device_name": "fixture"
            }
        ],
        "future_request": true
    }))
    .unwrap()
}
#[tokio::test]
async fn all_eight_instance_methods_match_source_literals_and_preserve_explicit_credentials() {
    let (client, wire, _) = clients(false);
    let instances = client.instances();
    let instance = json!({
        "tsg_id": "123456789",
        "tenant_id": "tenant/a?b",
        "app_id": "airs-redteam",
        "is_success": true,
        "future_response": true
    });
    let get = json!({
        "tsg_id": "123456789",
        "tenant_id": "tenant/a?b",
        "app_id": "airs-redteam",
        "region": "us-east-1",
        "future_response": true
    });
    let devices = json!({"devices":[{"serial_number":"SN-1","status":"FUTURE_ACCEPTED"}],"future_response":true});
    macro_rules! same {
        ($call:expr,$expected:expr) => {
            assert_eq!(
                serde_json::to_value($call.await.unwrap()).unwrap(),
                $expected
            )
        };
    }
    wire.simple(
        Method::POST,
        MGMT,
        "/v1/instances",
        Some(json!({
            "tsg_id": "123456789",
            "tenant_id": "tenant/a?b",
            "app_id": "airs-redteam",
            "region": "us-east-1",
            "future_request": true
        })),
        instance.clone(),
    );
    same!(instances.create_instance(&instance_body(), None), instance);
    wire.simple(
        Method::GET,
        MGMT,
        "/v1/instances/tenant%2Fa%3Fb",
        None,
        get.clone(),
    );
    same!(instances.get_instance("tenant/a?b", None), get);
    wire.simple(
        Method::PUT,
        MGMT,
        "/v1/instances/tenant%2Fa%3Fb",
        Some(json!({
            "tsg_id": "123456789",
            "tenant_id": "tenant/a?b",
            "app_id": "airs-redteam",
            "region": "us-east-1",
            "future_request": true
        })),
        instance.clone(),
    );
    same!(
        instances.update_instance("tenant/a?b", &instance_body(), None),
        instance
    );
    wire.simple(
        Method::DELETE,
        MGMT,
        "/v1/instances/tenant%2Fa%3Fb",
        None,
        instance.clone(),
    );
    same!(instances.delete_instance("tenant/a?b", None), instance);
    let expected_device = json!({
        "instance": {
            "tsg_id": "123456789",
            "tenant_id": "tenant/a?b",
            "app_id": "airs-redteam",
            "region": "us-east-1"
        },
        "devices": [
            {
                "serial_number": "SN-1",
                "device_name": "fixture"
            }
        ],
        "future_request": true
    });
    wire.simple(
        Method::POST,
        MGMT,
        "/v1/instances/tenant%2Fa%3Fb/devices",
        Some(expected_device.clone()),
        devices.clone(),
    );
    same!(
        instances.create_devices("tenant/a?b", &device_body(), None),
        devices
    );
    wire.simple(
        Method::PATCH,
        MGMT,
        "/v1/instances/tenant%2Fa%3Fb/devices",
        Some(expected_device),
        devices.clone(),
    );
    same!(
        instances.update_devices("tenant/a?b", &device_body(), None),
        devices
    );
    wire.push(
        Method::DELETE,
        MGMT,
        "/v1/instances/tenant%2Fa%3Fb/devices",
        BTreeMap::from([("serial_numbers".into(), vec!["SN-1,SN&2".into()])]),
        None,
        devices.clone(),
    );
    same!(
        instances.delete_devices("tenant/a?b", "SN-1,SN&2", None),
        devices
    );
    let credentials = json!({
        "token": "synthetic-registry-secret",
        "expiry": "2026-10-16T01:00:00Z",
        "future_response": true
    });
    wire.simple(
        Method::POST,
        MGMT,
        "/v1/registry-credentials",
        None,
        credentials.clone(),
    );
    let response = instances.get_registry_credentials(None).await.unwrap();
    assert!(!format!("{response:?}").contains("synthetic-registry-secret"));
    assert_eq!(serde_json::to_value(response).unwrap(), credentials);
    assert_eq!(wire.count(), 8);
    wire.done();
}

#[tokio::test]
async fn quota_summary_uses_get_without_changing_the_published_post_contract() {
    let (client, wire, _) = clients(false);
    let payload = fixture("RedTeamClient", "getQuota")["responseBody"].clone();
    wire.simple(
        Method::GET,
        DATA,
        "/v1/metering/quota",
        None,
        payload.clone(),
    );
    assert_eq!(
        serde_json::to_value(client.get_quota_summary(None).await.unwrap()).unwrap(),
        payload
    );
    wire.done();
}

fn scan_page(items: Vec<Value>, total: Option<Value>) -> Value {
    let mut page = json!({"pagination":{},"data":items});
    if let Some(total) = total {
        page["pagination"]["total_items"] = total;
    }
    page
}
fn job() -> Value {
    fixture("RedTeamScansClient", "get")["responseBody"].clone()
}
fn scan_expect(wire: &Wire, skip: u64, limit: u32, reply: Value) {
    wire.push(
        Method::GET,
        DATA,
        "/v1/scan",
        BTreeMap::from([
            ("skip".into(), vec![skip.to_string()]),
            ("limit".into(), vec![limit.to_string()]),
        ]),
        None,
        reply,
    );
}

#[tokio::test]
async fn scan_list_all_matches_actual_count_offsets_total_hints_defaults_and_source_empty_terminal()
{
    let (client, wire, _) = clients(false);
    scan_expect(&wire, 0, 50, scan_page(vec![job()], Some(json!(1))));
    let result = client
        .scans()
        .list_all(
            &RedTeamScanListOptions::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    assert!(result.is_complete());
    assert_eq!(result.items.len(), 1);
    wire.done();
    for total in [None, Some(Value::Null), Some(json!(3))] {
        let (client, wire, _) = clients(false);
        let mut opts = RedTeamScanListOptions::default();
        opts.limit = Some(2);
        scan_expect(&wire, 0, 2, scan_page(vec![job(), job()], total.clone()));
        scan_expect(&wire, 2, 2, scan_page(vec![job()], total));
        let result = client
            .scans()
            .list_all(&opts, CollectOptions::default(), None)
            .await
            .unwrap();
        assert!(result.is_complete());
        assert_eq!(result.items.len(), 3);
        assert_eq!(wire.count(), 2);
        wire.done();
    }
    let (client, wire, _) = clients(false);
    scan_expect(&wire, 0, 50, scan_page(vec![], Some(json!(100))));
    let result = client
        .scans()
        .list_all(
            &RedTeamScanListOptions::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    assert!(result.is_complete());
    assert!(result.items.is_empty());
    assert_eq!(wire.count(), 1);
}

#[tokio::test]
async fn scan_walk_is_lazy_cancellable_and_retains_truncation_and_failure_resume_positions() {
    let (client, wire, provider) = clients(false);
    let mut walk = client
        .scans()
        .list_pages(&RedTeamScanListOptions::default(), 5, None)
        .unwrap();
    assert_eq!(wire.count(), 0);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    scan_expect(&wire, 0, 50, scan_page(vec![job(), job()], Some(json!(3))));
    let first = walk.next_page().await.unwrap().unwrap();
    assert_eq!(first.items.len(), 2);
    assert_eq!(first.next, Some(2));
    drop(walk);
    assert_eq!(wire.count(), 1);
    let (client, wire, _) = clients(false);
    scan_expect(&wire, 0, 50, scan_page(vec![job(), job()], Some(json!(3))));
    let result = client
        .scans()
        .list_all(
            &RedTeamScanListOptions::default(),
            CollectOptions::default().with_max_items(1).unwrap(),
            None,
        )
        .await
        .unwrap();
    assert!(matches!(
        result.completion,
        Completion::Truncated(Truncation::Items)
    ));
    let position = result.resume.unwrap();
    assert_eq!(position.cursor, 0);
    assert_eq!(position.item_offset, 1);
    assert_eq!(wire.count(), 1);
    scan_expect(&wire, 0, 50, scan_page(vec![job(), job()], Some(json!(2))));
    let mut items = client
        .scans()
        .list_items(&RedTeamScanListOptions::default(), 5, None)
        .unwrap()
        .resume_from(position)
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    assert!(items.next().await.unwrap().is_none());
    wire.done();
    let (client, wire, _) = clients(false);
    scan_expect(&wire, 0, 50, scan_page(vec![job(), job()], Some(json!(3))));
    scan_expect(&wire, 2, 50, json!({"message":"synthetic-private-echo"}));
    wire.fault(503, false);
    let failure = client
        .scans()
        .list_all(
            &RedTeamScanListOptions::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap_err();
    assert_eq!(failure.partial().items.len(), 2);
    assert_eq!(failure.partial().resume.as_ref().unwrap().cursor, 2);
    assert_eq!(wire.count(), 2);
    assert!(!format!("{failure:?}").contains("synthetic-private-echo"));
    let (client, wire, provider) = clients(false);
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    let mut walk = client
        .scans()
        .list_pages(&RedTeamScanListOptions::default(), 5, Some(&options))
        .unwrap();
    options.cancel();
    assert_eq!(
        walk.next_page().await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(wire.count(), 0);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn metadata_fallback_requires_api_422_and_does_not_follow_arbitrary_download_urls() {
    let (client, wire, _) = clients(false);
    wire.simple(
        Method::GET,
        DATA,
        "/v1/scan/scan-metadata",
        None,
        json!({"message":"synthetic-private-echo"}),
    );
    wire.fault(422, false);
    wire.simple(
        Method::GET,
        MGMT,
        "/v1/template/target-metadata",
        None,
        json!({"future_metadata":[1,true,null]}),
    );
    assert_eq!(
        serde_json::to_value(client.get_scan_metadata(None).await.unwrap()).unwrap(),
        json!({"future_metadata":[1,true,null]})
    );
    assert_eq!(wire.count(), 2);
    wire.done();
    for status in [400, 403, 404, 500, 503] {
        let (client, wire, _) = clients(false);
        wire.simple(
            Method::GET,
            DATA,
            "/v1/scan/scan-metadata",
            None,
            json!({"message":"synthetic-private-echo"}),
        );
        wire.fault(status, false);
        let error = client.get_scan_metadata(None).await.unwrap_err();
        assert_eq!(error.status(), Some(status));
        assert_eq!(wire.count(), 1);
        wire.done();
    }
    let (client, wire, _) = clients(false);
    let response = json!({"url":"https://unlisted.example/report?signature=synthetic-private-echo","future":true});
    wire.push(
        Method::GET,
        DATA,
        &format!("/v1/report/{ID}/download"),
        BTreeMap::from([("file_format".into(), vec!["CSV".into()])]),
        None,
        response.clone(),
    );
    let result = client
        .reports()
        .download_report(ID, "CSV", None)
        .await
        .unwrap();
    assert_eq!(result.into_value(), response);
    assert_eq!(wire.count(), 1);
    wire.done();
}

#[tokio::test(start_paused = true)]
async fn metadata_fallback_shares_one_deadline_and_cancellation_can_prevent_second_dispatch() {
    let (client, wire, _) = clients(false);
    wire.simple(Method::GET, DATA, "/v1/scan/scan-metadata", None, json!({}));
    wire.fault(422, false);
    wire.delay(Duration::from_millis(600));
    wire.simple(
        Method::GET,
        MGMT,
        "/v1/template/target-metadata",
        None,
        json!({}),
    );
    wire.delay(Duration::from_millis(600));
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    let error = client.get_scan_metadata(Some(&options)).await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Deadline);
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
    assert_eq!(wire.count(), 2);
    let (client, wire, _) = clients(true);
    wire.simple(Method::GET, DATA, "/v1/scan/scan-metadata", None, json!({}));
    wire.fault(422, false);
    let error = client.get_scan_metadata(None).await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(wire.count(), 1);
    wire.done();
}

struct IssuerFailure {
    calls: AtomicUsize,
}
impl Transport for IssuerFailure {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            assert_eq!(request.url().as_str(), "https://issuer.example/token");
            Ok(TransportResponse::new(
                StatusCode::UNPROCESSABLE_ENTITY,
                HeaderMap::new(),
                Box::pin(stream::empty()),
            ))
        })
    }
}
#[tokio::test]
async fn issuer_422_never_triggers_metadata_fallback_and_constructor_never_widens_policy() {
    let transport = Arc::new(IssuerFailure {
        calls: AtomicUsize::new(0),
    });
    let data = Endpoint::parse(DATA).unwrap();
    let mgmt = Endpoint::parse(MGMT).unwrap();
    let broker = Endpoint::parse(BROKER).unwrap();
    let issuer = Endpoint::parse("https://issuer.example/token").unwrap();
    let context = AuthContext::new(issuer.clone(), "fixture-client", 1, "123456789").unwrap();
    let raw = Client::builder(data.clone())
        .transport(transport.clone())
        .destination_policy(Arc::new(OriginPolicy::new([
            data,
            mgmt.clone(),
            broker.clone(),
            issuer,
        ])))
        .oauth_credentials(
            context,
            Secret::new("synthetic-issuer-secret").unwrap(),
            true,
        )
        .build()
        .unwrap();
    let client = RedTeamClient::new(raw, mgmt, broker).unwrap();
    let error = client.get_scan_metadata(None).await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Authentication);
    assert_eq!(error.status(), Some(422));
    assert_eq!(transport.calls.load(Ordering::SeqCst), 1);
    let wire = Arc::new(Wire::default());
    let provider = Provider::new(false);
    let raw = Client::builder(Endpoint::parse(DATA).unwrap())
        .transport(wire.clone())
        .authentication(Authentication::oauth(provider.clone(), false))
        .build()
        .unwrap();
    assert_eq!(
        RedTeamClient::new(
            raw,
            Endpoint::parse(MGMT).unwrap(),
            Endpoint::parse(BROKER).unwrap()
        )
        .unwrap_err()
        .kind(),
        ErrorKind::Destination
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert_eq!(wire.count(), 0);
}

fn mutation_expected(wire: &Wire, name: &str) {
    let case = match name {
        "create_scan" => Some(fixture("RedTeamScansClient", "create")),
        "abort_scan" => Some(fixture("RedTeamScansClient", "abort")),
        "partial_report" => Some(fixture("RedTeamReportsClient", "generatePartialReport")),
        "quota_post" => Some(fixture("RedTeamClient", "getQuota")),
        "sentiment" => Some(fixture("RedTeamClient", "updateSentiment")),
        _ => None,
    };
    if let Some(case) = case {
        arm(wire, &case);
        return;
    }
    let instance = json!({
        "tsg_id": "123456789",
        "tenant_id": "tenant/a?b",
        "app_id": "airs-redteam",
        "region": "us-east-1",
        "future_request": true
    });
    let device = json!({
        "instance": {
            "tsg_id": "123456789",
            "tenant_id": "tenant/a?b",
            "app_id": "airs-redteam",
            "region": "us-east-1"
        },
        "devices": [
            {
                "serial_number": "SN-1",
                "device_name": "fixture"
            }
        ],
        "future_request": true
    });
    match name {
        "create_instance" => wire.simple(
            Method::POST,
            MGMT,
            "/v1/instances",
            Some(instance),
            json!({}),
        ),
        "update_instance" => wire.simple(
            Method::PUT,
            MGMT,
            "/v1/instances/tenant%2Fa%3Fb",
            Some(instance),
            json!({}),
        ),
        "delete_instance" => wire.simple(
            Method::DELETE,
            MGMT,
            "/v1/instances/tenant%2Fa%3Fb",
            None,
            json!({}),
        ),
        "create_devices" => wire.simple(
            Method::POST,
            MGMT,
            "/v1/instances/tenant%2Fa%3Fb/devices",
            Some(device),
            json!({}),
        ),
        "update_devices" => wire.simple(
            Method::PATCH,
            MGMT,
            "/v1/instances/tenant%2Fa%3Fb/devices",
            Some(device),
            json!({}),
        ),
        "delete_devices" => wire.push(
            Method::DELETE,
            MGMT,
            "/v1/instances/tenant%2Fa%3Fb/devices",
            BTreeMap::from([("serial_numbers".into(), vec!["SN-1,SN&2".into()])]),
            None,
            json!({}),
        ),
        "registry" => wire.simple(
            Method::POST,
            MGMT,
            "/v1/registry-credentials",
            None,
            json!({}),
        ),
        _ => panic!("unknown mutation"),
    }
}
#[tokio::test]
async fn all_twelve_owned_mutations_dispatch_once_and_keep_ambiguous_outcomes_unknown() {
    for name in [
        "create_scan",
        "abort_scan",
        "partial_report",
        "quota_post",
        "sentiment",
        "create_instance",
        "update_instance",
        "delete_instance",
        "create_devices",
        "update_devices",
        "delete_devices",
        "registry",
    ] {
        for failure in [false, true] {
            let (client, wire, _) = clients(false);
            mutation_expected(&wire, name);
            wire.fault(503, failure);
            wire.expected.lock().unwrap().back_mut().unwrap().reply =
                json!({"message":"synthetic-private-echo","token":"synthetic-private-echo"});
            let options = RequestOptions::with_timeout(Duration::from_secs(5))
                .unwrap()
                .with_retries(5)
                .unwrap();
            let result: Result<()> = match name {
                "create_scan" => client
                    .scans()
                    .create(
                        &serde_json::from_value(
                            fixture("RedTeamScansClient", "create")["body"].clone(),
                        )
                        .unwrap(),
                        Some(&options),
                    )
                    .await
                    .map(|_| ()),
                "abort_scan" => client.scans().abort(ID, Some(&options)).await.map(|_| ()),
                "partial_report" => client
                    .reports()
                    .generate_partial_report(ID, Some(&options))
                    .await
                    .map(|_| ()),
                "quota_post" => client.get_quota(Some(&options)).await.map(|_| ()),
                "sentiment" => client
                    .update_sentiment(
                        &serde_json::from_value(
                            fixture("RedTeamClient", "updateSentiment")["body"].clone(),
                        )
                        .unwrap(),
                        Some(&options),
                    )
                    .await
                    .map(|_| ()),
                "create_instance" => client
                    .instances()
                    .create_instance(&instance_body(), Some(&options))
                    .await
                    .map(|_| ()),
                "update_instance" => client
                    .instances()
                    .update_instance("tenant/a?b", &instance_body(), Some(&options))
                    .await
                    .map(|_| ()),
                "delete_instance" => client
                    .instances()
                    .delete_instance("tenant/a?b", Some(&options))
                    .await
                    .map(|_| ()),
                "create_devices" => client
                    .instances()
                    .create_devices("tenant/a?b", &device_body(), Some(&options))
                    .await
                    .map(|_| ()),
                "update_devices" => client
                    .instances()
                    .update_devices("tenant/a?b", &device_body(), Some(&options))
                    .await
                    .map(|_| ()),
                "delete_devices" => client
                    .instances()
                    .delete_devices("tenant/a?b", "SN-1,SN&2", Some(&options))
                    .await
                    .map(|_| ()),
                _ => client
                    .instances()
                    .get_registry_credentials(Some(&options))
                    .await
                    .map(|_| ()),
            };
            let error = result.unwrap_err();
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
            assert_eq!(wire.count(), 1);
            assert!(matches!(
                error.kind(),
                ErrorKind::Http | ErrorKind::Transport
            ));
            assert!(!format!("{error:?} {error}").contains("synthetic-private-echo"));
            wire.done();
        }
    }
}

#[tokio::test]
async fn invalid_identifiers_filters_and_request_models_fail_before_authentication_or_dispatch() {
    let (client, wire, provider) = clients(false);
    let invalid = "synthetic-private-invalid/id";
    let errors = vec![
        client.scans().get(invalid, None).await.unwrap_err(),
        client.scans().abort(invalid, None).await.unwrap_err(),
        client
            .reports()
            .get_attack_detail(ID, invalid, None)
            .await
            .unwrap_err(),
        client
            .reports()
            .get_multi_turn_attack_detail(invalid, ID, None)
            .await
            .unwrap_err(),
        client
            .reports()
            .get_static_report(invalid, None)
            .await
            .unwrap_err(),
        client
            .reports()
            .get_dynamic_report(invalid, None)
            .await
            .unwrap_err(),
        client
            .reports()
            .get_stream_detail(invalid, None)
            .await
            .unwrap_err(),
        client
            .reports()
            .list_goal_streams(ID, invalid, &RedTeamListOptions::default(), None)
            .await
            .unwrap_err(),
        client
            .reports()
            .download_report(invalid, "CSV", None)
            .await
            .unwrap_err(),
        client
            .reports()
            .generate_partial_report(invalid, None)
            .await
            .unwrap_err(),
        client
            .custom_attack_reports()
            .get_report(invalid, None)
            .await
            .unwrap_err(),
        client
            .custom_attack_reports()
            .get_prompt_detail(ID, invalid, None)
            .await
            .unwrap_err(),
        client
            .custom_attack_reports()
            .get_attack_outputs(invalid, ID, None)
            .await
            .unwrap_err(),
        client
            .custom_attack_reports()
            .get_property_stats(invalid, None)
            .await
            .unwrap_err(),
        client
            .get_error_logs(invalid, &RedTeamListOptions::default(), None)
            .await
            .unwrap_err(),
        client
            .get_target_profile_error_logs(invalid, &RedTeamListOptions::default(), None)
            .await
            .unwrap_err(),
        client
            .get_score_trend(invalid, &ScoreTrendOptions::default(), None)
            .await
            .unwrap_err(),
        client.get_sentiment(invalid, None).await.unwrap_err(),
    ];
    for error in errors {
        assert_eq!(error.kind(), ErrorKind::Validation);
        assert_eq!(error.remote_outcome(), RemoteOutcome::NotSent);
        assert!(!format!("{error:?} {error}").contains("synthetic-private"));
    }
    for tenant in ["", "..", "../escape", "%252e%252e", "a\\b"] {
        assert_eq!(
            client
                .instances()
                .get_instance(tenant, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Validation
        );
    }
    let mut options = RedTeamScanListOptions::default();
    options.limit = Some(0);
    assert_eq!(
        client
            .scans()
            .list(&options, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Validation
    );
    options.limit = Some(1);
    options.skip = Some(u64::MAX);
    assert!(client.scans().list_pages(&options, 10, None).is_err());
    options.skip = None;
    options.search = Some("synthetic-private\r\nvalue".into());
    assert!(client.scans().list(&options, None).await.is_err());
    assert!(
        client
            .reports()
            .download_report(ID, "CSV\r\nprivate", None)
            .await
            .is_err()
    );
    let mut body: JobCreateRequest =
        serde_json::from_value(fixture("RedTeamScansClient", "create")["body"].clone()).unwrap();
    body.name = "xx".into();
    assert!(client.scans().create(&body, None).await.is_err());
    let mut sentiment: SentimentRequest =
        serde_json::from_value(fixture("RedTeamClient", "updateSentiment")["body"].clone())
            .unwrap();
    sentiment.job_id = invalid.into();
    assert!(client.update_sentiment(&sentiment, None).await.is_err());
    let mut device = device_body();
    device.extensions.insert(
        "instance".into(),
        serde_json::from_value(json!("synthetic-private")).unwrap(),
    );
    assert!(
        client
            .instances()
            .create_devices("tenant", &device, None)
            .await
            .is_err()
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert_eq!(wire.count(), 0);
    wire.done();
}

#[tokio::test]
async fn optional_query_fields_missing_from_frozen_samples_are_transmitted_without_dropping_false_values()
 {
    let (client, wire, _) = clients(false);
    let search = "fixture & α";
    let listing = BTreeMap::from([
        ("skip".into(), vec!["3".into()]),
        ("limit".into(), vec!["7".into()]),
    ]);
    let mut common = RedTeamListOptions::default();
    common.skip = Some(3);
    common.limit = Some(7);
    let stats = ScanStatisticsOptions::default();
    wire.push(
        Method::GET,
        DATA,
        "/v1/dashboard/scan-statistics",
        BTreeMap::new(),
        None,
        fixture("RedTeamClient", "getScanStatistics")["responseBody"].clone(),
    );
    client.get_scan_statistics(&stats, None).await.unwrap();
    wire.push(
        Method::GET,
        DATA,
        &format!("/v1/report/dynamic/{ID}/goal/{ID}/list-streams"),
        BTreeMap::new(),
        None,
        fixture("RedTeamReportsClient", "listGoalStreams")["responseBody"].clone(),
    );
    client
        .reports()
        .list_goal_streams(ID, ID, &RedTeamListOptions::default(), None)
        .await
        .unwrap();
    for target in [false, true] {
        let path = if target {
            format!("/v1/error-log/target-profile/{ID}")
        } else {
            format!("/v1/error-log/job/{ID}")
        };
        let method = if target {
            "getTargetProfileErrorLogs"
        } else {
            "getErrorLogs"
        };
        wire.push(
            Method::GET,
            DATA,
            &path,
            if target {
                BTreeMap::from([("limit".into(), vec!["7".into()])])
            } else {
                listing.clone()
            },
            None,
            fixture("RedTeamClient", method)["responseBody"].clone(),
        );
        if target {
            client
                .get_target_profile_error_logs(
                    ID,
                    &{
                        let mut p = RedTeamListOptions::default();
                        p.limit = Some(7);
                        p
                    },
                    None,
                )
                .await
                .unwrap();
        } else {
            client.get_error_logs(ID, &common, None).await.unwrap();
        }
    }
    let mut attacks = AttackListOptions::default();
    attacks.threat = Some(false);
    wire.push(
        Method::GET,
        DATA,
        &format!("/v1/report/static/{ID}/list-attacks"),
        BTreeMap::from([("threat".into(), vec!["false".into()])]),
        None,
        fixture("RedTeamReportsClient", "listAttacks")["responseBody"].clone(),
    );
    client
        .reports()
        .list_attacks(ID, &attacks, None)
        .await
        .unwrap();
    let mut sets = PromptSetsReportOptions::default();
    sets.is_threat = Some(false);
    sets.property_filters = Some("category=fixture&severity=low".into());
    wire.push(
        Method::GET,
        DATA,
        &format!("/v1/custom-attacks/report/{ID}/prompt-sets"),
        BTreeMap::from([
            ("is_threat".into(), vec!["false".into()]),
            (
                "property_filters".into(),
                vec!["category=fixture&severity=low".into()],
            ),
        ]),
        None,
        fixture("RedTeamCustomAttackReportsClient", "getPromptSets")["responseBody"].clone(),
    );
    client
        .custom_attack_reports()
        .get_prompt_sets(ID, &sets, None)
        .await
        .unwrap();
    let mut prompts = PromptsBySetListOptions::default();
    prompts.is_threat = Some(false);
    wire.push(
        Method::GET,
        DATA,
        &format!("/v1/custom-attacks/report/{ID}/prompt-set/{ID}/prompts"),
        BTreeMap::from([("is_threat".into(), vec!["false".into()])]),
        None,
        fixture("RedTeamCustomAttackReportsClient", "getPromptsBySet")["responseBody"].clone(),
    );
    client
        .custom_attack_reports()
        .get_prompts_by_set(ID, ID, &prompts, None)
        .await
        .unwrap();
    let mut custom = CustomAttacksReportListOptions::default();
    custom.threat = Some(false);
    wire.push(
        Method::GET,
        DATA,
        &format!("/v1/custom-attacks/job/{ID}/list-custom-attacks"),
        BTreeMap::from([("threat".into(), vec!["false".into()])]),
        None,
        fixture("RedTeamCustomAttackReportsClient", "listCustomAttacks")["responseBody"].clone(),
    );
    client
        .custom_attack_reports()
        .list_custom_attacks(ID, &custom, None)
        .await
        .unwrap();
    wire.simple(
        Method::GET,
        DATA,
        "/v1/goal-categories/APPLICATION%2F%26%3F%CE%B1",
        None,
        fixture("RedTeamClient", "getGoalCategories")["responseBody"].clone(),
    );
    client
        .get_goal_categories("APPLICATION/&?α", None)
        .await
        .unwrap();
    assert_eq!(wire.count(), 9);
    wire.done();
    assert!(
        !format!("{common:?} {stats:?} {attacks:?} {sets:?} {prompts:?} {custom:?}")
            .contains(search)
    );
}

#[tokio::test]
async fn scan_walk_preserves_every_filter_on_successive_pages() {
    let (client, wire, _) = clients(false);
    let mut params = RedTeamScanListOptions::default();
    params.limit = Some(1);
    params.search = Some("fixture search".into());
    params.status = Some("COMPLETED".into());
    params.job_type = Some("STATIC".into());
    params.target_id = Some(ID.into());
    for skip in [0, 1] {
        wire.push(
            Method::GET,
            DATA,
            "/v1/scan",
            BTreeMap::from([
                ("skip".into(), vec![skip.to_string()]),
                ("limit".into(), vec!["1".into()]),
                ("search".into(), vec!["fixture search".into()]),
                ("status".into(), vec!["COMPLETED".into()]),
                ("job_type".into(), vec!["STATIC".into()]),
                ("target_id".into(), vec![ID.into()]),
            ]),
            None,
            scan_page(vec![job()], Some(json!(2))),
        );
    }
    let result = client
        .scans()
        .list_all(&params, CollectOptions::default(), None)
        .await
        .unwrap();
    assert!(result.is_complete());
    assert_eq!(result.items.len(), 2);
    wire.done();
}

#[tokio::test]
async fn every_unsupported_legacy_dataplane_parameter_fails_before_authentication() {
    let (client, wire, provider) = clients(false);
    macro_rules! rejected {
        ($call:expr) => {{
            let error = $call.await.unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Validation);
            assert_eq!(error.remote_outcome(), RemoteOutcome::NotSent);
            assert_eq!(wire.count(), 0);
            assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
        }};
    }
    let mut attacks = AttackListOptions::default();
    attacks.search = Some("caller-filter".into());
    rejected!(client.reports().list_attacks(ID, &attacks, None));
    for field in ["skip", "limit", "search"] {
        let mut options = RedTeamListOptions::default();
        match field {
            "skip" => options.skip = Some(0),
            "limit" => options.limit = Some(1),
            _ => options.search = Some("caller-filter".into()),
        }
        rejected!(client.reports().list_goal_streams(ID, ID, &options, None));
    }
    let mut sets = PromptSetsReportOptions::default();
    sets.search = Some("caller-filter".into());
    rejected!(
        client
            .custom_attack_reports()
            .get_prompt_sets(ID, &sets, None)
    );
    let mut prompts = PromptsBySetListOptions::default();
    prompts.search = Some("caller-filter".into());
    rejected!(
        client
            .custom_attack_reports()
            .get_prompts_by_set(ID, ID, &prompts, None)
    );
    let mut custom = CustomAttacksReportListOptions::default();
    custom.search = Some("caller-filter".into());
    rejected!(
        client
            .custom_attack_reports()
            .list_custom_attacks(ID, &custom, None)
    );
    let mut errors = RedTeamListOptions::default();
    errors.search = Some("caller-filter".into());
    rejected!(client.get_error_logs(ID, &errors, None));
    rejected!(client.get_target_profile_error_logs(ID, &errors, None));
    errors.search = None;
    errors.skip = Some(0);
    rejected!(client.get_target_profile_error_logs(ID, &errors, None));
    let mut stats = ScanStatisticsOptions::default();
    stats.date_range = Some("LAST_30_DAYS".into());
    rejected!(client.get_scan_statistics(&stats, None));
    stats.date_range = None;
    stats.target_id = Some(ID.into());
    rejected!(client.get_scan_statistics(&stats, None));
    let mut scans = RedTeamScanListOptions::default();
    scans.status = Some("FUTURE_STATE".into());
    rejected!(client.scans().list(&scans, None));
    scans.status = None;
    scans.limit = Some(101);
    rejected!(client.scans().list(&scans, None));
    scans.limit = None;
    scans.target_id = Some("not-a-uuid".into());
    rejected!(client.scans().list(&scans, None));
    let mut trend = ScoreTrendOptions::default();
    trend.start_date = Some("2026-02-30".into());
    rejected!(client.get_score_trend(ID, &trend, None));
}

fn vendor_specimen(schema: &str) -> Value {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../contracts/vendor/red-team-data-plane/0.93.0/model-fixtures.json"
    ))
    .unwrap();
    corpus["cases"]
        .as_array()
        .unwrap()
        .iter()
        .find(|row| row["name"] == format!("valid_{schema}"))
        .unwrap()["value"]
        .clone()
}

#[test]
fn exactly_four_new_presence_waivers_apply_only_to_response_validation() {
    use scm_sdk::validation::{self, Mode};
    for (schema, field) in [
        ("AttackDetailResponseSchema", "audio_technique"),
        ("AttackMultiTurnDetailResponseSchema", "audio_technique"),
        ("AttackOutputResponseSchema", "judge_reasoning"),
        ("AttackMultiTurnOutputResponseSchema", "judge_reasoning"),
    ] {
        let mut value = vendor_specimen(schema);
        value.as_object_mut().unwrap().remove(field);
        let strict = format!("vendor:red-team-dp:0.93.0:{schema}");
        let compatible = format!("{strict}:compatible");
        assert!(validation::parse(&compatible, value.clone(), Mode::Request).is_err());
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            assert!(validation::parse(&strict, value.clone(), mode).is_err());
            assert_eq!(
                validation::parse(&compatible, value.clone(), mode).unwrap(),
                value
            );
        }
    }
    let mut value = vendor_specimen("JobResponseSchema");
    value.as_object_mut().unwrap().remove("uuid");
    assert!(
        validation::parse(
            "vendor:red-team-dp:0.93.0:JobResponseSchema:compatible",
            value,
            Mode::Response
        )
        .is_err()
    );
}

#[test]
fn current_fields_are_typed_and_future_progress_and_verdicts_remain_explicit() {
    use scm_sdk::validation::{self, Mode};
    let mut value = vendor_specimen("JobResponseSchema");
    let model: JobResponse = serde_json::from_value(value.clone()).unwrap();
    assert!(matches!(model.runtime_metrics, Patch::Value(_)));
    assert!(matches!(model.progress, Patch::Value(_)));
    assert_eq!(serde_json::to_value(model).unwrap(), value);
    value["progress"] = json!({"kind":"FUTURE_PROGRESS", "opaque":{"preserved":true}});
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        let parsed = validation::parse(
            "vendor:red-team-dp:0.93.0:JobResponseSchema:compatible",
            value.clone(),
            mode,
        )
        .unwrap();
        let model: JobResponse = serde_json::from_value(parsed).unwrap();
        assert!(matches!(
            model.progress,
            Patch::Value(data_plane::JobResponseProgress::Unknown(_))
        ));
        assert_eq!(serde_json::to_value(model).unwrap(), value);
    }
    value["progress"] = json!({"kind":false});
    assert!(
        validation::parse(
            "vendor:red-team-dp:0.93.0:JobResponseSchema:compatible",
            value,
            Mode::Response
        )
        .is_err()
    );
    let mut detail = vendor_specimen("AttackDetailResponseSchema");
    detail["threat_details"] = json!({"raw_threat":null,"threat_override":null});
    let model: AttackDetailResponse = serde_json::from_value(detail).unwrap();
    let Optional::Value(provenance) = model.threat_details else {
        panic!("typed provenance")
    };
    assert!(matches!(provenance.raw_threat, Patch::Null));
    assert!(matches!(provenance.threat_override, Patch::Null));
    assert!(!format!("{provenance:?}").contains("raw_threat"));
}

#[tokio::test]
async fn current_scan_body_preserves_reserved_json_keys_without_javascript_normalization() {
    let (client, wire, _) = clients(false);
    let mut case = fixture("RedTeamScansClient", "create");
    case["body"]["extra_info"] = serde_json::from_str(
        r#"{"__proto__":{"preserved":true},"constructor":{"__proto__":"also data"}}"#,
    )
    .unwrap();
    let request: JobCreateRequest = serde_json::from_value(case["body"].clone()).unwrap();
    arm(&wire, &case);
    client.scans().create(&request, None).await.unwrap();
    wire.done();
    assert_eq!(wire.count(), 1);
}

// Current-spec operation corpus, independently validated by AJV before Rust runs.
const VENDOR_WIRE: &str =
    include_str!("../../../contracts/vendor/red-team-data-plane/0.93.0/wire-fixtures.json");
fn vendor_cases() -> Vec<Value> {
    serde_json::from_str::<Value>(VENDOR_WIRE).unwrap()["cases"]
        .as_array()
        .unwrap()
        .clone()
}
fn vendor_fixture(class: &str, method: &str) -> Value {
    vendor_cases()
        .into_iter()
        .find(|r| r["call"]["className"] == class && r["call"]["member"] == method)
        .unwrap()
}
fn raw_fixture(case: &Value) -> Option<Bytes> {
    case["rawResponseHex"].as_str().map(|hex| {
        Bytes::from(
            (0..hex.len())
                .step_by(2)
                .map(|i| u8::from_str_radix(&hex[i..i + 2], 16).unwrap())
                .collect::<Vec<_>>(),
        )
    })
}
fn arm_vendor(wire: &Wire, case: &Value) {
    let query = case["queryWire"]
        .as_object()
        .unwrap()
        .iter()
        .map(|(k, v)| {
            (
                k.clone(),
                v.as_array()
                    .unwrap()
                    .iter()
                    .map(|s| s.as_str().unwrap().to_owned())
                    .collect(),
            )
        })
        .collect();
    wire.push(
        Method::from_bytes(case["method"].as_str().unwrap().as_bytes()).unwrap(),
        DATA,
        case["path"].as_str().unwrap(),
        query,
        case.get("body").cloned(),
        case["responseBody"].clone(),
    );
    let mut queue = wire.expected.lock().unwrap();
    let row = queue.back_mut().unwrap();
    row.status = case["status"].as_u64().unwrap() as u16;
    row.raw_reply = raw_fixture(case);
    row.operation = Some(case["operation"].as_str().unwrap().to_owned());
}
async fn call_vendor(
    client: &RedTeamClient,
    case: &Value,
    options: Option<&RequestOptions>,
) -> Result<Value> {
    let params = &case["pathParameters"];
    let job = params["job_id"].as_str().unwrap_or(ID);
    let attack = params["attack_id"].as_str().unwrap_or(ID);
    let stream_id = params["stream_id"].as_str().unwrap_or(ID);
    let goal = params["goal_id"].as_str().unwrap_or(ID);
    let prompt_set = params["prompt_set_id"].as_str().unwrap_or(ID);
    let prompt = params["prompt_id"].as_str().unwrap_or(ID);
    let target = params["target_id"]
        .as_str()
        .unwrap_or_else(|| case["query"]["target_id"].as_str().unwrap_or(ID));
    let query = &case["query"];
    let body = &case["body"];
    macro_rules! parsed {
        ($body:expr) => {
            serde_json::from_value($body.clone()).map_err(|_| Error::new(ErrorKind::Validation))?
        };
    }
    macro_rules! value {
        ($call:expr) => {
            serde_json::to_value($call.await?).map_err(|_| Error::new(ErrorKind::Validation))?
        };
    }
    let actual = match (
        case["call"]["className"].as_str().unwrap(),
        case["call"]["member"].as_str().unwrap(),
    ) {
        ("RedTeamScansClient", "create") => value!(client.scans().create(&parsed!(body), options)),
        ("RedTeamScansClient", "list") => {
            let mut opts = RedTeamScanListOptions::from(listing(query));
            opts.status = qstr(query, "status");
            opts.job_type = qstr(query, "job_type");
            opts.target_id = qstr(query, "target_id");
            value!(client.scans().list(&opts, options))
        }
        ("RedTeamScansClient", "get") => value!(client.scans().get(job, options)),
        ("RedTeamScansClient", "abort") => value!(client.scans().abort(job, options)),
        ("RedTeamScansClient", "get_categories") => value!(client.scans().get_categories(options)),
        ("RedTeamClient", "get_scan_metadata") => value!(client.get_scan_metadata(options)),
        ("RedTeamReportsClient", "list_attacks") => {
            let mut opts = AttackListOptions::default();
            opts.skip = query["skip"].as_u64();
            opts.limit = query["limit"].as_u64().map(|v| v as u32);
            opts.search = qstr(query, "search");
            opts.attack_status = qstr(query, "attack_status");
            opts.compliance = qstr(query, "compliance");
            opts.attack_modality = qstr(query, "attack_modality");
            opts.status = qstr(query, "status");
            opts.severity = qstr(query, "severity");
            opts.category = qstr(query, "category");
            opts.sub_category = qstr(query, "sub_category");
            opts.attack_type = qstr(query, "attack_type");
            opts.threat = query["threat"].as_bool();
            value!(client.reports().list_attacks(job, &opts, options))
        }
        ("RedTeamReportsClient", "get_attack_detail") => {
            value!(client.reports().get_attack_detail(job, attack, options))
        }
        ("RedTeamReportsClient", "get_multi_turn_attack_detail") => {
            value!(
                client
                    .reports()
                    .get_multi_turn_attack_detail(job, attack, options)
            )
        }
        ("RedTeamReportsClient", "get_static_report") => {
            value!(client.reports().get_static_report(job, options))
        }
        ("RedTeamReportsClient", "get_static_runtime_policy") => {
            value!(client.reports().get_static_runtime_policy(job, options))
        }
        ("RedTeamReportsClient", "get_static_remediation") => {
            value!(client.reports().get_static_remediation(job, options))
        }
        ("RedTeamReportsClient", "get_dynamic_report") => {
            value!(client.reports().get_dynamic_report(job, options))
        }
        ("RedTeamReportsClient", "get_dynamic_runtime_policy") => {
            value!(client.reports().get_dynamic_runtime_policy(job, options))
        }
        ("RedTeamReportsClient", "get_dynamic_remediation") => {
            value!(client.reports().get_dynamic_remediation(job, options))
        }
        ("RedTeamReportsClient", "list_goals") => {
            let mut opts = GoalListOptions::default();
            opts.skip = query["skip"].as_u64();
            opts.limit = query["limit"].as_u64().map(|v| v as u32);
            opts.search = qstr(query, "search");
            opts.goal_category = qstr(query, "goal_category");
            opts.goal_type = qstr(query, "goal_type");
            opts.status = qstr(query, "status");
            opts.count = query["count"].as_bool();
            value!(client.reports().list_goals(job, &opts, options))
        }
        ("RedTeamReportsClient", "list_goal_streams") => value!(
            client
                .reports()
                .list_goal_streams(job, goal, &listing(query), options)
        ),
        ("RedTeamReportsClient", "get_stream_detail") => {
            value!(client.reports().get_stream_detail(stream_id, options))
        }
        ("RedTeamReportsClient", "download_report") => value!(client.reports().download_report(
            job,
            query["file_format"].as_str().unwrap(),
            options
        )),
        ("RedTeamReportsClient", "generate_partial_report") => {
            value!(client.reports().generate_partial_report(job, options))
        }
        ("RedTeamClient", "get_quota") => value!(client.get_quota(options)),
        ("RedTeamCustomAttackReportsClient", "get_report") => {
            value!(client.custom_attack_reports().get_report(job, options))
        }
        ("RedTeamCustomAttackReportsClient", "get_prompt_sets") => {
            let mut opts = PromptSetsReportOptions::default();
            opts.skip = query["skip"].as_u64();
            opts.limit = query["limit"].as_u64().map(|v| v as u32);
            opts.search = qstr(query, "search");
            opts.property_filters = qstr(query, "property_filters");
            opts.is_threat = query["is_threat"].as_bool();
            value!(
                client
                    .custom_attack_reports()
                    .get_prompt_sets(job, &opts, options)
            )
        }
        ("RedTeamCustomAttackReportsClient", "get_prompts_by_set") => {
            let mut opts = PromptsBySetListOptions::default();
            opts.skip = query["skip"].as_u64();
            opts.limit = query["limit"].as_u64().map(|v| v as u32);
            opts.search = qstr(query, "search");
            opts.is_threat = query["is_threat"].as_bool();
            value!(
                client
                    .custom_attack_reports()
                    .get_prompts_by_set(job, prompt_set, &opts, options)
            )
        }
        ("RedTeamCustomAttackReportsClient", "get_prompt_detail") => value!(
            client
                .custom_attack_reports()
                .get_prompt_detail(job, prompt, options)
        ),
        ("RedTeamCustomAttackReportsClient", "list_custom_attacks") => {
            let mut opts = CustomAttacksReportListOptions::default();
            opts.skip = query["skip"].as_u64();
            opts.limit = query["limit"].as_u64().map(|v| v as u32);
            opts.search = qstr(query, "search");
            opts.status = qstr(query, "status");
            opts.threat = query["threat"].as_bool();
            opts.prompt_set_id = qstr(query, "prompt_set_id");
            opts.property_value = qstr(query, "property_value");
            value!(
                client
                    .custom_attack_reports()
                    .list_custom_attacks(job, &opts, options)
            )
        }
        ("RedTeamCustomAttackReportsClient", "get_attack_outputs") => value!(
            client
                .custom_attack_reports()
                .get_attack_outputs(job, attack, options)
        ),
        ("RedTeamCustomAttackReportsClient", "get_property_stats") => {
            value!(
                client
                    .custom_attack_reports()
                    .get_property_stats(job, options)
            )
        }
        ("RedTeamClient", "get_error_logs") => {
            value!(client.get_error_logs(job, &listing(query), options))
        }
        ("RedTeamClient", "get_target_profile_error_logs") => {
            value!(client.get_target_profile_error_logs(target, &listing(query), options))
        }
        ("RedTeamClient", "get_goal_categories") => {
            value!(client.get_goal_categories("APPLICATION", options))
        }
        ("RedTeamClient", "get_languages") => value!(client.get_languages(options)),
        ("RedTeamClient", "get_management_languages") => {
            value!(client.get_management_languages(options))
        }
        ("RedTeamClient", "get_scan_statistics") => {
            value!(client.get_scan_statistics(&ScanStatisticsOptions::default(), options))
        }
        ("RedTeamClient", "get_score_trend") => {
            let mut opts = ScoreTrendOptions::default();
            opts.date_range = qstr(query, "date_range");
            opts.start_date = qstr(query, "start_date");
            opts.end_date = qstr(query, "end_date");
            value!(client.get_score_trend(target, &opts, options))
        }
        ("RedTeamClient", "update_sentiment") => {
            value!(client.update_sentiment(&parsed!(body), options))
        }
        ("RedTeamClient", "get_sentiment") => value!(client.get_sentiment(job, options)),
        ("RedTeamClient", "get_dashboard_overview") => {
            value!(client.get_dashboard_overview(options))
        }

        ("RedTeamScansClient", "set_runtime_profile") => value!(
            client
                .scans()
                .set_runtime_profile(job, &parsed!(body), options)
        ),
        ("RedTeamReportsClient", "get_static_asr") => {
            value!(client.reports().get_static_asr(job, options))
        }
        ("RedTeamReportsClient", "get_dynamic_asr") => {
            value!(client.reports().get_dynamic_asr(job, options))
        }
        ("RedTeamReportsClient", "regenerate") => value!(client.reports().regenerate(job, options)),
        ("RedTeamReportsClient", "get_status") => value!(client.reports().get_status(job, options)),
        ("RedTeamReportsClient", "override_attack") => value!(client.reports().override_attack(
            job,
            attack,
            &parsed!(body),
            options
        )),
        ("RedTeamReportsClient", "override_multi_turn_attack") => value!(
            client
                .reports()
                .override_multi_turn_attack(job, attack, &parsed!(body), options)
        ),
        ("RedTeamReportsClient", "override_stream") => value!(client.reports().override_stream(
            job,
            stream_id,
            &parsed!(body),
            options
        )),
        ("RedTeamCustomAttackReportsClient", "override_attack") => value!(
            client
                .custom_attack_reports()
                .override_attack(job, attack, &parsed!(body), options)
        ),
        ("RedTeamReportsClient", "get_download_url") => value!(client.reports().get_download_url(
            job,
            parsed!(&query["file_format"]),
            options
        )),
        ("RedTeamClient", "download_error_logs") => {
            serde_json::to_value(client.download_error_logs(job, options).await?.to_vec()).unwrap()
        }
        key => panic!("uncovered frozen method {key:?}"),
    };
    Ok(actual)
}
#[tokio::test]
async fn all_47_vendor_operations_match_independent_wire_contracts_under_both_response_policies() {
    let cases = vendor_cases();
    let mut ids = std::collections::BTreeSet::new();
    for without_defaults in [false, true] {
        let (client, wire, _) = clients(false);
        for case in &cases {
            ids.insert(case["operationId"].as_str().unwrap());
            arm_vendor(&wire, case);
            let mut options = RequestOptions::with_timeout(Duration::from_secs(5)).unwrap();
            if without_defaults {
                options = options.without_response_defaults();
            }
            let actual = call_vendor(&client, case, Some(&options))
                .await
                .unwrap_or_else(|e| panic!("{}: {e:?}", case["name"]));
            let expected = raw_fixture(case)
                .map(|b| serde_json::to_value(b.to_vec()).unwrap())
                .unwrap_or_else(|| case["responseBody"].clone());
            assert_eq!(actual, expected, "{}", case["name"]);
        }
        assert_eq!(wire.count(), cases.len());
        wire.done();
    }
    assert_eq!(ids.len(), 47);
}

const NEW_VENDOR_CALLS: &[(&str, &str)] = &[
    ("RedTeamScansClient", "set_runtime_profile"),
    ("RedTeamReportsClient", "get_static_asr"),
    ("RedTeamReportsClient", "get_dynamic_asr"),
    ("RedTeamReportsClient", "regenerate"),
    ("RedTeamReportsClient", "get_status"),
    ("RedTeamReportsClient", "override_attack"),
    ("RedTeamReportsClient", "override_multi_turn_attack"),
    ("RedTeamReportsClient", "override_stream"),
    ("RedTeamCustomAttackReportsClient", "override_attack"),
    ("RedTeamClient", "download_error_logs"),
    ("RedTeamReportsClient", "get_download_url"),
];
#[tokio::test]
async fn every_new_vendor_route_rejects_each_invalid_path_id_before_credentials() {
    for &(class, method) in NEW_VENDOR_CALLS {
        let case = vendor_fixture(class, method);
        for key in case["pathParameters"].as_object().unwrap().keys() {
            let (client, wire, provider) = clients(false);
            let mut bad = case.clone();
            bad["pathParameters"][key] = json!("synthetic-private-invalid-id/../?");
            let error = call_vendor(&client, &bad, None).await.unwrap_err();
            assert_eq!(
                error.kind(),
                ErrorKind::Validation,
                "{class}.{method}.{key}"
            );
            assert_eq!(error.remote_outcome(), RemoteOutcome::NotSent);
            assert_eq!(wire.count(), 0);
            assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
            assert!(!format!("{error:?} {error}").contains("synthetic-private"));
        }
    }
}
#[tokio::test]
async fn every_new_vendor_route_honors_cancellation_and_deadlines_before_and_after_dispatch() {
    for &(class, method) in NEW_VENDOR_CALLS {
        let case = vendor_fixture(class, method);
        for kind in [ErrorKind::Cancelled, ErrorKind::Deadline] {
            for dispatched in [false, true] {
                let (client, wire, provider) = clients(false);
                let options = RequestOptions::with_timeout(if kind == ErrorKind::Deadline {
                    if dispatched {
                        Duration::from_millis(100)
                    } else {
                        Duration::from_millis(1)
                    }
                } else {
                    Duration::from_secs(5)
                })
                .unwrap();
                if dispatched {
                    arm_vendor(&wire, &case);
                    wire.delay(Duration::from_secs(2));
                } else if kind == ErrorKind::Cancelled {
                    options.cancel();
                } else {
                    tokio::time::sleep(Duration::from_millis(3)).await;
                }
                let result = if dispatched && kind == ErrorKind::Cancelled {
                    let (result, ()) =
                        tokio::join!(call_vendor(&client, &case, Some(&options)), async {
                            while wire.count() == 0 {
                                tokio::task::yield_now().await;
                            }
                            options.cancel();
                        });
                    result
                } else {
                    call_vendor(&client, &case, Some(&options)).await
                };
                let error = result.unwrap_err();
                assert_eq!(
                    error.kind(),
                    kind,
                    "{class}.{method} dispatched={dispatched}"
                );
                assert_eq!(
                    error.remote_outcome(),
                    if dispatched {
                        RemoteOutcome::Unknown
                    } else {
                        RemoteOutcome::NotSent
                    }
                );
                assert_eq!(wire.count(), usize::from(dispatched));
                if !dispatched {
                    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
                }
                wire.done();
            }
        }
    }
}
#[tokio::test]
async fn six_new_vendor_writes_never_replay_transport_http_decode_or_limit_failures() {
    let mut writes = 0;
    for &(class, method) in NEW_VENDOR_CALLS {
        let case = vendor_fixture(class, method);
        if case["method"] == "GET" {
            continue;
        }
        writes += 1;
        for failure in ["transport", "http", "decode", "limit"] {
            let mut limits = Limits::default();
            limits.max_retries = 5;
            if failure == "limit" {
                limits.max_response_bytes = 64;
            }
            let (client, wire, _) = clients_with_limits(false, limits);
            arm_vendor(&wire, &case);
            {
                let mut queue = wire.expected.lock().unwrap();
                let reply = queue.back_mut().unwrap();
                if failure == "transport" {
                    reply.fail = true;
                }
                if failure == "http" {
                    reply.status = 503;
                }
                reply.raw_reply = Some(Bytes::from(if failure == "limit" {
                    vec![b'x'; 65]
                } else {
                    b"synthetic-private-echo-and-token".to_vec()
                }));
            }
            let options = RequestOptions::with_timeout(Duration::from_secs(5))
                .unwrap()
                .with_retries(5)
                .unwrap();
            let error = call_vendor(&client, &case, Some(&options))
                .await
                .unwrap_err();
            assert_eq!(
                error.remote_outcome(),
                RemoteOutcome::Unknown,
                "{class}.{method}.{failure}"
            );
            assert_eq!(
                error.kind(),
                match failure {
                    "transport" => ErrorKind::Transport,
                    "http" => ErrorKind::Http,
                    "decode" => ErrorKind::Decode,
                    _ => ErrorKind::Limit,
                }
            );
            assert_eq!(error.operation(), Some(case["operation"].as_str().unwrap()));
            assert!(!format!("{error:?} {error}").contains("synthetic-private"));
            assert_eq!(wire.count(), 1);
            wire.done();
        }
    }
    assert_eq!(writes, 6);
}
#[tokio::test]
async fn all_four_override_routes_validate_counts_nullable_verdicts_uuids_and_closed_objects() {
    for &(class, method) in NEW_VENDOR_CALLS
        .iter()
        .filter(|(_, method)| method.starts_with("override_"))
    {
        let base = vendor_fixture(class, method);
        for count in [0, 1, 100, 101] {
            for verdict in [json!(true), json!(false), Value::Null] {
                let (client, wire, provider) = clients(false);
                let mut case = base.clone();
                case["body"]["overrides"] = Value::Array(
                    (0..count)
                        .map(|_| json!({"output_id":ID,"threat_override":verdict}))
                        .collect(),
                );
                let valid = (1..=100).contains(&count);
                if valid {
                    arm_vendor(&wire, &case);
                }
                let result = call_vendor(&client, &case, None).await;
                if valid {
                    assert_eq!(result.unwrap(), case["responseBody"]);
                    wire.done();
                } else {
                    let error = result.unwrap_err();
                    assert_eq!(error.kind(), ErrorKind::Validation);
                    assert_eq!(error.remote_outcome(), RemoteOutcome::NotSent);
                    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
                }
                assert_eq!(wire.count(), usize::from(valid));
            }
        }
        for mutation in ["missing", "invalid_uuid", "root_extra", "item_extra"] {
            let (client, wire, provider) = clients(false);
            let mut case = base.clone();
            match mutation {
                "missing" => {
                    case["body"]["overrides"][0]
                        .as_object_mut()
                        .unwrap()
                        .remove("threat_override");
                }
                "invalid_uuid" => case["body"]["overrides"][0]["output_id"] = json!("invalid"),
                "root_extra" => case["body"]["extra"] = json!(true),
                _ => case["body"]["overrides"][0]["extra"] = json!(true),
            }
            assert_eq!(
                call_vendor(&client, &case, None).await.unwrap_err().kind(),
                ErrorKind::Validation
            );
            assert_eq!(wire.count(), 0);
            assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
        }
    }
}
#[tokio::test]
async fn runtime_profile_uses_unicode_codepoint_bounds_and_rejects_extra_fields_before_auth() {
    let base = vendor_fixture("RedTeamScansClient", "set_runtime_profile");
    for length in [0, 1, 256, 257] {
        let (client, wire, provider) = clients(false);
        let mut case = base.clone();
        case["body"]["runtime_profile_id"] = json!("🦀".repeat(length));
        let valid = (1..=256).contains(&length);
        if valid {
            arm_vendor(&wire, &case);
        }
        let result = call_vendor(&client, &case, None).await;
        if valid {
            result.unwrap();
            wire.done();
        } else {
            assert_eq!(result.unwrap_err().kind(), ErrorKind::Validation);
            assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
        }
        assert_eq!(wire.count(), usize::from(valid));
    }
    let (client, wire, provider) = clients(false);
    let mut case = base;
    case["body"]["extra"] = json!(true);
    assert_eq!(
        call_vendor(&client, &case, None).await.unwrap_err().kind(),
        ErrorKind::Validation
    );
    assert_eq!(wire.count(), 0);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
}
#[tokio::test]
async fn csv_download_preserves_empty_non_utf8_and_exact_limit_bytes() {
    let case = vendor_fixture("RedTeamClient", "download_error_logs");
    for bytes in [
        vec![],
        vec![0, 255, 254, b'\n'],
        vec![b'x'; 32],
        vec![b'x'; 33],
    ] {
        let mut limits = Limits::default();
        limits.max_response_bytes = 32;
        let (client, wire, _) = clients_with_limits(false, limits);
        arm_vendor(&wire, &case);
        wire.expected.lock().unwrap().back_mut().unwrap().raw_reply =
            Some(Bytes::from(bytes.clone()));
        let result = client
            .download_error_logs(case["pathParameters"]["job_id"].as_str().unwrap(), None)
            .await;
        if bytes.len() > 32 {
            assert_eq!(result.unwrap_err().kind(), ErrorKind::Limit);
        } else {
            assert_eq!(result.unwrap(), bytes);
        }
        assert_eq!(wire.count(), 1);
        wire.done();
    }
}
#[tokio::test]
async fn report_conflict_future_state_and_download_location_keep_explicit_semantics() {
    let (client, wire, provider) = clients(false);
    let case = vendor_fixture("RedTeamReportsClient", "regenerate");
    arm_vendor(&wire, &case);
    wire.fault(409, false);
    let options = RequestOptions::with_timeout(Duration::from_secs(5))
        .unwrap()
        .with_retries(5)
        .unwrap();
    let error = call_vendor(&client, &case, Some(&options))
        .await
        .unwrap_err();
    assert_eq!(error.status(), Some(409));
    assert_eq!(error.remote_outcome(), RemoteOutcome::Rejected);
    assert_eq!(wire.count(), 1);
    wire.done();
    let mut case = vendor_fixture("RedTeamReportsClient", "get_status");
    case["responseBody"]["report_state"] = json!("FUTURE_PRIVATE_STATE");
    arm_vendor(&wire, &case);
    let report = client
        .reports()
        .get_status(case["pathParameters"]["job_id"].as_str().unwrap(), None)
        .await
        .unwrap();
    assert!(matches!(&report.report_state,ReportState::Unknown(v) if v=="FUTURE_PRIVATE_STATE"));
    assert_eq!(serde_json::to_value(&report).unwrap(), case["responseBody"]);
    assert!(!format!("{report:?}").contains("FUTURE_PRIVATE_STATE"));
    let mut case = vendor_fixture("RedTeamReportsClient", "get_download_url");
    for format in [FileFormat::CSV, FileFormat::JSON, FileFormat::ALL] {
        case["queryWire"]["file_format"] = json!([format.as_str()]);
        arm_vendor(&wire, &case);
        let report = client
            .reports()
            .get_download_url(
                case["pathParameters"]["job_id"].as_str().unwrap(),
                format,
                None,
            )
            .await
            .unwrap();
        assert_eq!(serde_json::to_value(&report).unwrap(), case["responseBody"]);
        assert!(!format!("{report:?}").contains("synthetic-private-download"));
        assert!(!format!("{report:?}").contains("../../opaque"));
    }
    wire.done();
    assert_eq!(wire.count(), 5);
    let before = provider.calls.load(Ordering::SeqCst);
    for unknown in [
        FileFormat::Unknown("CSV".into()),
        FileFormat::Unknown("synthetic-private-format".into()),
    ] {
        let error = client
            .reports()
            .get_download_url(ID, unknown, None)
            .await
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Validation);
        assert_eq!(error.remote_outcome(), RemoteOutcome::NotSent);
        assert!(!format!("{error:?} {error}").contains("synthetic-private"));
    }
    assert_eq!(provider.calls.load(Ordering::SeqCst), before);
    assert_eq!(wire.count(), 5);
}

struct VendorDestinationPolicy(std::sync::atomic::AtomicBool);
impl scm_sdk::config::DestinationPolicy for VendorDestinationPolicy {
    fn check(&self, _: &url::Url, _: scm_sdk::config::DestinationPurpose) -> Result<()> {
        if self.0.load(Ordering::SeqCst) {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Destination))
        }
    }
}
#[tokio::test]
async fn every_new_vendor_route_rechecks_destination_policy_before_authentication() {
    for &(class, method) in NEW_VENDOR_CALLS {
        let wire = Arc::new(Wire::default());
        let provider = Provider::new(false);
        let policy = Arc::new(VendorDestinationPolicy(std::sync::atomic::AtomicBool::new(
            true,
        )));
        let raw = Client::builder(Endpoint::parse(DATA).unwrap())
            .transport(wire.clone())
            .authentication(Authentication::oauth(provider.clone(), true))
            .destination_policy(policy.clone())
            .build()
            .unwrap();
        let client = RedTeamClient::new(
            raw,
            Endpoint::parse(MGMT).unwrap(),
            Endpoint::parse(BROKER).unwrap(),
        )
        .unwrap();
        policy.0.store(false, Ordering::SeqCst);
        let error = call_vendor(&client, &vendor_fixture(class, method), None)
            .await
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Destination);
        assert_eq!(error.remote_outcome(), RemoteOutcome::NotSent);
        assert_eq!(wire.count(), 0);
        assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    }
}
