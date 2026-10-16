//! Literal, independently authored contracts from frozen AgentGuard client and schemas.
#![cfg(feature = "agentguard")]
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, Method, StatusCode};
use scm_sdk::{
    Client, Endpoint, ErrorKind, OriginPolicy, RequestOptions, Result, Secret,
    airs::agentguard::{AgentGuardClient, models::AgentGuardStatsOptions},
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    transport::{Transport, TransportRequest, TransportResponse},
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
struct Fixture {
    management: bool,
    path: String,
    query: Vec<(String, String)>,
    response: Value,
    status: StatusCode,
    delay: Duration,
}
#[derive(Default)]
struct Wire {
    fixtures: Mutex<VecDeque<Fixture>>,
    calls: AtomicUsize,
}
impl Wire {
    fn expect(&self, management: bool, path: &str, query: &[(&str, &str)], response: Value) {
        self.fixtures.lock().unwrap().push_back(Fixture {
            management,
            path: path.into(),
            query: query
                .iter()
                .map(|(k, v)| ((*k).into(), (*v).into()))
                .collect(),
            response,
            status: StatusCode::OK,
            delay: Duration::ZERO,
        });
    }
    fn done(&self) {
        assert!(self.fixtures.lock().unwrap().is_empty());
    }
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let f = self
                .fixtures
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected AgentGuard request");
            let base = if f.management {
                "https://agent-management.example/aiag/mgmt"
            } else {
                "https://agent-data.example/aiag/data"
            };
            assert_eq!(request.method(), Method::GET);
            let mut url = request.url().clone();
            url.set_query(None);
            assert_eq!(url.as_str(), format!("{base}{}", f.path));
            assert_eq!(
                request.url().query_pairs().into_owned().collect::<Vec<_>>(),
                f.query
            );
            let mut names: Vec<_> = request.headers().keys().map(|h| h.as_str()).collect();
            names.sort_unstable();
            assert_eq!(
                names,
                ["authorization", "service-name", "user-agent", "x-tsg-id"]
            );
            assert_eq!(request.headers().len(), 4);
            assert_eq!(
                request.headers()["authorization"],
                "Bearer fixture-agent-token"
            );
            assert!(request.headers()["authorization"].is_sensitive());
            assert_eq!(request.headers()["x-tsg-id"], "123456789");
            assert_eq!(request.headers()["service-name"], "api");
            assert_eq!(
                request.headers()["user-agent"],
                concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
            );
            assert!(request.body().is_empty());
            tokio::time::sleep(f.delay).await;
            Ok(TransportResponse::from_bytes(
                f.status,
                HeaderMap::new(),
                Bytes::from(serde_json::to_vec(&f.response).unwrap()),
            ))
        })
    }
}
struct Token {
    context: AuthContext,
    calls: AtomicUsize,
}
impl TokenProvider for Token {
    fn token<'a>(&'a self, _: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            TokenLease::new(
                Secret::new("fixture-agent-token").unwrap(),
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
fn endpoints() -> (Endpoint, Endpoint) {
    (
        Endpoint::parse("https://agent-data.example/aiag/data").unwrap(),
        Endpoint::parse("https://agent-management.example/aiag/mgmt").unwrap(),
    )
}
fn clients() -> (AgentGuardClient, Arc<Wire>, Arc<Token>) {
    let (data, management) = endpoints();
    let wire = Arc::new(Wire::default());
    let token = Arc::new(Token {
        context: AuthContext::new(
            Endpoint::parse("https://issuer.example/token").unwrap(),
            "agent-client",
            1,
            "123456789",
        )
        .unwrap(),
        calls: AtomicUsize::new(0),
    });
    let client = Client::builder(data.clone())
        .transport(wire.clone())
        .destination_policy(Arc::new(OriginPolicy::new([
            data.clone(),
            management.clone(),
        ])))
        .authentication(Authentication::oauth(token.clone(), false))
        .build()
        .unwrap();
    (
        AgentGuardClient::new(client, data, management).unwrap(),
        wire,
        token,
    )
}
fn body<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).unwrap()
}
fn scan_list() -> Value {
    json!({"pagination":{"total_items":9,"future_page":true},"scans":[{"uuid":ID,"tsg_id":"123456789","created_at":"created","updated_at":"updated","name":"sensitive-skill","git_url":null,"artifact_type":"FUTURE_ARTIFACT","status":"FUTURE_STATUS","error_message":null,"summary":{"vulnerability_count":2,"attack_chain_count":1,"future_count":7},"eval_outcome":null,"eval_summary":null,"eval_details":null,"duration_seconds":null,"fingerprint":"sensitive-fingerprint","scanner_version":null,"device_metadata":{"future":"sensitive-device"},"parent_uuid":null,"is_batch":true,"child_scan_uuids":["child-id"],"batch_summary":{"sensitive_batch":true},"future_scan":{"keep":true}}],"future_list":true})
}
fn stats() -> Value {
    json!({"unique_skills_scanned":{"count":3,"percent_change":null,"future_metric":true},"total_vulnerabilities_found":{"count":2,"percent_change":-5.5},"top_vulnerability":null,"future_stats":true})
}
fn vulnerabilities() -> Value {
    json!({"pagination":{"total_items":1},"vulnerabilities":[{"uuid":ID,"tsg_id":"123456789","created_at":"created","updated_at":"updated","scan_uuid":ID,"title":"sensitive-title","description":"sensitive-description","vulnerability_type":"FUTURE_TYPE","file_path":"/sensitive/file","node_id":null,"node_type":null,"node_name":null,"remediation_description":null,"original_code":"sensitive-original-code","suggested_code":null,"attack_chain_count":0,"attack_chain_uuids":[],"type_description":"sensitive-type-description","future_finding":{"keep":true}}]})
}
fn rules() -> Value {
    json!({"pagination":{"total_items":1},"rules":[{"uuid":ID,"name":"catalog-default","vulnerability_types":["FUTURE_TYPE"],"default_state":"FUTURE_STATE","description":"description","remediation":"remediation","future_rule":true}]})
}
#[tokio::test]
async fn all_four_read_operations_preserve_exact_planes_headers_order_defaults_and_sensitive_models()
 {
    let (c, w, t) = clients();
    w.expect(
        false,
        "/v1/scans",
        &[
            ("skip", "0"),
            ("limit", "7"),
            ("isBackgroundRefresh", "false"),
            ("start_time", "2026-10-01T01:00:00+01:00"),
            ("end_time", "2026-10-02T00:00:00Z"),
        ],
        scan_list(),
    );
    let scan=c.list_scans(&body(json!({"skip":0,"limit":7,"isBackgroundRefresh":false,"start_time":"2026-10-01T01:00:00+01:00","end_time":"2026-10-02T00:00:00Z"})),None).await.unwrap();
    assert_eq!(serde_json::to_value(&scan).unwrap(), scan_list());
    assert!(!format!("{scan:?}").contains("sensitive"));
    w.expect(
        false,
        "/v1/stats/scans",
        &[("time_period", "30_DAYS")],
        stats(),
    );
    let response = c.get_scan_stats(&body(json!({})), None).await.unwrap();
    assert_eq!(serde_json::to_value(&response).unwrap(), stats());
    w.expect(
        false,
        &format!("/v1/scans/{ID}/vulnerabilities"),
        &[],
        vulnerabilities(),
    );
    let response = c.list_scan_vulnerabilities(ID, None).await.unwrap();
    assert_eq!(serde_json::to_value(&response).unwrap(), vulnerabilities());
    assert!(!format!("{response:?}").contains("sensitive"));
    w.expect(true, "/v1/rules", &[("skip", "4"), ("limit", "2")], rules());
    let response = c
        .list_rules(&body(json!({"skip":4,"limit":2})), None)
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(&response).unwrap(), rules());
    assert_eq!(response.pagination.total_items.as_u64(), Some(1));
    w.done();
    assert_eq!(w.calls.load(Ordering::SeqCst), 4);
    assert_eq!(t.calls.load(Ordering::SeqCst), 4);
    assert!(!format!("{c:?}").contains("agent-client"));
}
#[tokio::test]
async fn omitted_query_fields_stay_absent_and_only_stats_gets_its_observed_default() {
    let (c, w, _) = clients();
    w.expect(
        false,
        "/v1/scans",
        &[],
        json!({"pagination":{"total_items":0},"scans":[]}),
    );
    c.list_scans(&body(json!({})), None).await.unwrap();
    w.expect(
        true,
        "/v1/rules",
        &[],
        json!({"pagination":{"total_items":0},"rules":[]}),
    );
    c.list_rules(&body(json!({})), None).await.unwrap();
    w.expect(
        false,
        "/v1/stats/scans",
        &[("time_period", "30_DAYS")],
        stats(),
    );
    c.get_scan_stats(&body(json!({"time_period":"30_DAYS"})), None)
        .await
        .unwrap();
    w.done();
}
#[tokio::test]
async fn invalid_queries_uuid_and_bounded_serialization_fail_before_authentication() {
    let (c, w, t) = clients();
    for query in [
        json!({"skip":-1}),
        json!({"skip":1.5}),
        json!({"limit":0}),
        json!({"limit":9007199254740992u64}),
        json!({"start_time":"not-a-date"}),
        json!({"start_time":"2026-10-03T00:00:00Z","end_time":"2026-10-02T00:00:00Z"}),
        json!({"future":"request-secret"}),
    ] {
        let e = c.list_scans(&body(query), None).await.unwrap_err();
        assert_eq!(e.kind(), ErrorKind::Validation);
        assert!(!format!("{e:?} {e}").contains("request-secret"));
    }
    for query in [
        json!({"skip":-1}),
        json!({"limit":0}),
        json!({"future":true}),
    ] {
        assert_eq!(
            c.list_rules(&body(query), None).await.unwrap_err().kind(),
            ErrorKind::Validation
        );
    }
    assert_eq!(
        c.get_scan_stats(&body(json!({"future":"request-secret"})), None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Validation
    );
    assert!(
        serde_json::from_value::<AgentGuardStatsOptions>(json!({"time_period":"60_DAYS"})).is_err()
    );
    for id in [
        "../request-secret",
        "00000000-0000-0000-0000-00000000000g",
        "",
    ] {
        assert_eq!(
            c.list_scan_vulnerabilities(id, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Validation
        );
    }
    assert_eq!(
        c.list_scans(&body(json!({"start_time":"x".repeat(32*1024)})), None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    assert_eq!(w.calls.load(Ordering::SeqCst), 0);
    assert_eq!(t.calls.load(Ordering::SeqCst), 0);
}
#[tokio::test]
async fn malformed_known_response_fields_missing_nulls_and_negative_counts_are_rejected() {
    let (c, w, _) = clients();
    for mut response in [scan_list(), scan_list(), scan_list()] {
        let index = w.calls.load(Ordering::SeqCst);
        match index {
            0 => {
                response["scans"][0]
                    .as_object_mut()
                    .unwrap()
                    .remove("git_url");
            }
            1 => response["scans"][0]["duration_seconds"] = json!(-1),
            _ => response["scans"][0]["summary"]["vulnerability_count"] = json!(-1),
        }
        w.expect(false, "/v1/scans", &[], response);
        assert_eq!(
            c.list_scans(&body(json!({})), None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Decode
        );
    }
    w.expect(false,"/v1/stats/scans",&[("time_period","30_DAYS")],json!({"unique_skills_scanned":{"count":0,"percent_change":null},"total_vulnerabilities_found":{"count":0,"percent_change":null}}));
    assert_eq!(
        c.get_scan_stats(&body(json!({})), None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Decode
    );
    w.done();
}
#[test]
fn constructors_require_oauth_and_explicit_admission_for_both_planes() {
    let (data, management) = endpoints();
    assert!(
        AgentGuardClient::new(
            Client::builder(data.clone())
                .transport(Arc::new(Wire::default()))
                .build()
                .unwrap(),
            data.clone(),
            management.clone()
        )
        .is_err()
    );
    let (_, _, token) = clients();
    let client = Client::builder(data.clone())
        .transport(Arc::new(Wire::default()))
        .authentication(Authentication::oauth(token, false))
        .build()
        .unwrap();
    assert_eq!(
        AgentGuardClient::new(client, data, management)
            .unwrap_err()
            .kind(),
        ErrorKind::Destination
    );
}
#[tokio::test(start_paused = true)]
async fn request_deadline_and_session_cancellation_bound_dispatch_and_read_only_errors() {
    let (c, w, t) = clients();
    let cancelled = c.data_client().session();
    cancelled.cancel();
    assert_eq!(
        c.list_rules(&body(json!({})), None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(t.calls.load(Ordering::SeqCst), 0);
    assert_eq!(w.calls.load(Ordering::SeqCst), 0);
    let (c, w, _) = clients();
    w.expect(
        false,
        "/v1/stats/scans",
        &[("time_period", "30_DAYS")],
        stats(),
    );
    w.fixtures.lock().unwrap().front_mut().unwrap().delay = Duration::from_secs(10);
    let options = RequestOptions::with_timeout(Duration::from_secs(1))
        .unwrap()
        .with_retries(0)
        .unwrap();
    assert_eq!(
        c.get_scan_stats(&body(json!({})), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Deadline
    );
    w.done();
    let (c, w, _) = clients();
    w.expect(
        false,
        "/v1/stats/scans",
        &[("time_period", "30_DAYS")],
        json!({"request-secret":"private"}),
    );
    w.fixtures.lock().unwrap().front_mut().unwrap().status = StatusCode::SERVICE_UNAVAILABLE;
    let e = c
        .get_scan_stats(
            &body(json!({})),
            Some(
                &RequestOptions::with_timeout(Duration::from_secs(10))
                    .unwrap()
                    .with_retries(0)
                    .unwrap(),
            ),
        )
        .await
        .unwrap_err();
    assert_eq!(e.status(), Some(503));
    assert!(!format!("{e:?} {e}").contains("request-secret"));
    w.done();
}
