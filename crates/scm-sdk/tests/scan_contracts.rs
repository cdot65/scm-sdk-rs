//! Scanner expectations come from the frozen upstream fixture, independently of
//! native method generation. Synthetic additions exercise future server values.
#![cfg(feature = "scan")]

use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use bytes::Bytes;
use futures_util::future::BoxFuture;
use hmac::{Hmac, Mac};
use http::{HeaderMap, StatusCode};
use scm_sdk::{
    Client, Endpoint, ErrorKind, RemoteOutcome, RequestOptions, Secret,
    airs::scan::{
        Content, Scanner, SyncScanOptions,
        models::{AiProfile, AsyncScanObject, ScanRequest},
    },
    auth::Authentication,
    transport::{Transport, TransportRequest, TransportResponse},
};
use serde_json::{Value, json};
use sha2::Sha256;

const API_KEY: &str = "independent-scan-fixture-key";
const API_TOKEN: &str = "independent-scan-fixture-token";

fn cases() -> Vec<Value> {
    let fixture: Value = serde_json::from_str(include_str!(concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../contracts/fixtures/typescript/test/openapi/fixtures/scan.json"
    )))
    .unwrap();
    assert_eq!(
        fixture["sha256"],
        "e97a5a75d5a6073c56af022452d65197c2c474597a569f628fe1d7de0f63e7c3"
    );
    let cases = fixture["cases"].as_array().unwrap().clone();
    assert_eq!(cases.len(), 4);
    cases
}

struct FixtureTransport {
    requests: Mutex<Vec<TransportRequest>>,
    replies: Mutex<VecDeque<Value>>,
}
impl Transport for FixtureTransport {
    fn send<'a>(
        &'a self,
        request: TransportRequest,
    ) -> BoxFuture<'a, scm_sdk::Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.lock().unwrap().push(request);
            let response = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected scanner dispatch");
            Ok(TransportResponse::from_bytes(
                StatusCode::OK,
                HeaderMap::new(),
                Bytes::from(serde_json::to_vec(&response).unwrap()),
            ))
        })
    }
}
fn scanner(replies: impl IntoIterator<Item = Value>) -> (Scanner, Arc<FixtureTransport>) {
    let transport = Arc::new(FixtureTransport {
        requests: Mutex::new(Vec::new()),
        replies: Mutex::new(replies.into_iter().collect()),
    });
    let client = Client::builder(Endpoint::parse("https://scan-fixture.example.test").unwrap())
        .authentication(
            Authentication::scan(
                Some(Secret::new(API_KEY).unwrap()),
                Some(Secret::new(API_TOKEN).unwrap()),
            )
            .unwrap(),
        )
        .transport(transport.clone())
        .build()
        .unwrap();
    (Scanner::new(client).unwrap(), transport)
}
fn ids(case: &Value, key: &str) -> Vec<String> {
    case["query"][key]
        .as_array()
        .unwrap()
        .iter()
        .map(|id| id.as_str().unwrap().to_owned())
        .collect()
}
fn independent_hash(bytes: &[u8]) -> String {
    let mut mac = Hmac::<Sha256>::new_from_slice(API_KEY.as_bytes()).unwrap();
    mac.update(bytes);
    mac.finalize()
        .into_bytes()
        .iter()
        .map(|byte| format!("{byte:02x}"))
        .collect()
}
fn assert_wire(request: &TransportRequest, case: &Value) {
    assert_eq!(request.method().as_str(), case["method"].as_str().unwrap());
    assert_eq!(request.url().path(), case["path"].as_str().unwrap());
    assert_eq!(request.url().host_str(), Some("scan-fixture.example.test"));
    let pairs: Vec<_> = request
        .url()
        .query_pairs()
        .map(|(key, value)| (key.into_owned(), value.into_owned()))
        .collect();
    let expected: Vec<_> = case["queryWire"]
        .as_object()
        .unwrap()
        .iter()
        .flat_map(|(key, values)| {
            values
                .as_array()
                .unwrap()
                .iter()
                .map(move |value| (key.clone(), value.as_str().unwrap().to_owned()))
        })
        .collect();
    assert_eq!(pairs, expected);
    let mut header_names: Vec<_> = request.headers().keys().map(|name| name.as_str()).collect();
    header_names.sort_unstable();
    let expected_headers: &[&str] = if case.get("body").is_some() {
        &[
            "authorization",
            "content-type",
            "service-name",
            "user-agent",
            "x-pan-token",
            "x-payload-hash",
        ]
    } else {
        &["authorization", "service-name", "user-agent", "x-pan-token"]
    };
    assert_eq!(header_names, expected_headers);
    for name in expected_headers {
        assert_eq!(request.headers().get_all(*name).iter().count(), 1);
    }
    assert_eq!(request.headers()["service-name"], "api");
    assert_eq!(
        request.headers()["user-agent"],
        concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
    );
    assert!(request.headers().values().all(|value| value.is_sensitive()));
    let headers_debug = format!("{:?}", request.headers());
    assert!(!headers_debug.contains(API_KEY));
    assert!(!headers_debug.contains(API_TOKEN));
    assert!(!headers_debug.contains(&independent_hash(request.body())));
    assert_eq!(request.headers()["x-pan-token"], API_KEY);
    assert_eq!(
        request.headers()["authorization"],
        format!("Bearer {API_TOKEN}")
    );
    assert!(request.headers()["x-pan-token"].is_sensitive());
    assert!(request.headers()["authorization"].is_sensitive());
    if let Some(body) = case.get("body") {
        assert_eq!(
            serde_json::from_slice::<Value>(request.body()).unwrap(),
            *body
        );
        assert_eq!(request.headers()["content-type"], "application/json");
        assert_eq!(
            request.headers()["x-payload-hash"],
            independent_hash(request.body())
        );
        assert!(request.headers()["x-payload-hash"].is_sensitive());
    } else {
        assert!(request.body().is_empty());
        assert!(!request.headers().contains_key("x-payload-hash"));
    }
}

#[tokio::test]
async fn four_scanner_methods_match_independent_frozen_wire_contracts() {
    let cases = cases();
    let (scanner, transport) = scanner(cases.iter().map(|case| case["responseBody"].clone()));
    let body = &cases[0]["body"];
    let profile: AiProfile = serde_json::from_value(body["ai_profile"].clone()).unwrap();
    let content =
        Content::from_json(serde_json::from_value(body["contents"][0].clone()).unwrap()).unwrap();
    let mut options = SyncScanOptions::default();
    options.tr_id = Some(body["tr_id"].as_str().unwrap().into());
    options.session_id = Some(body["session_id"].as_str().unwrap().into());
    options.metadata = Some(serde_json::from_value(body["metadata"].clone()).unwrap());
    let response = scanner
        .sync_scan(&profile, &content, Some(&options))
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(response).unwrap(),
        cases[0]["responseBody"]
    );
    let batch: Vec<AsyncScanObject> = serde_json::from_value(cases[1]["body"].clone()).unwrap();
    let receipt = scanner.async_scan(&batch, None).await.unwrap();
    assert_eq!(
        serde_json::to_value(receipt).unwrap(),
        cases[1]["responseBody"]
    );
    let results = scanner
        .query_by_scan_ids(&ids(&cases[2], "scan_ids"), None)
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(results).unwrap(),
        cases[2]["responseBody"]
    );
    let reports = scanner
        .query_by_report_ids(&ids(&cases[3], "report_ids"), None)
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(reports).unwrap(),
        cases[3]["responseBody"]
    );
    let requests = transport.requests.lock().unwrap();
    assert_eq!(requests.len(), 4);
    for (request, case) in requests.iter().zip(cases.iter()) {
        assert_wire(request, case);
    }
}

#[tokio::test]
async fn scan_results_preserve_future_values_rows_duplicates_and_order() {
    let cases = cases();
    let mut first = cases[2]["responseBody"][0].clone();
    first["status"] = json!("future-pending-status");
    first["result"]["action"] = json!("future-action");
    first["result"]["category"] = json!("future-category");
    first["result"]["errors"][0]["feature"] = json!("future-detector");
    first["future_row_field"] = json!({"private":"row-secret"});
    first["result"]["future_detection"] = json!([1, 2, 3]);
    let mut second = first.clone();
    second["req_id"] = json!(2);
    second["status"] = json!("future-complete-status");
    let wire = json!([second, first, first]);
    let (scanner, transport) = scanner([wire.clone()]);
    let rows = scanner
        .query_by_scan_ids(&ids(&cases[2], "scan_ids"), None)
        .await
        .unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(serde_json::to_value(&rows).unwrap(), wire);
    assert!(!format!("{rows:?}").contains("row-secret"));
    assert_wire(&transport.requests.lock().unwrap()[0], &cases[2]);
}

#[tokio::test]
async fn report_results_preserve_future_verdicts_and_unordered_duplicate_rows() {
    let cases = cases();
    let mut first = cases[3]["responseBody"][0].clone();
    first["detection_results"][0]["verdict"] = json!("future-verdict");
    first["detection_results"][0]["action"] = json!("future-action");
    first["future_report_field"] = json!({"private":"report-secret"});
    let mut second = first.clone();
    second["req_id"] = json!(2);
    let wire = json!([second, first, first]);
    let (scanner, transport) = scanner([wire.clone()]);
    let rows = scanner
        .query_by_report_ids(&ids(&cases[3], "report_ids"), None)
        .await
        .unwrap();
    assert_eq!(rows.len(), 3);
    assert_eq!(serde_json::to_value(&rows).unwrap(), wire);
    assert!(!format!("{rows:?}").contains("report-secret"));
    assert_wire(&transport.requests.lock().unwrap()[0], &cases[3]);
}

#[tokio::test]
async fn complete_typed_scan_request_preserves_the_frozen_body() {
    let case = cases().remove(0);
    let (scanner, transport) = scanner([case["responseBody"].clone()]);
    let body: ScanRequest = serde_json::from_value(case["body"].clone()).unwrap();
    scanner.scan_request(&body, None).await.unwrap();
    assert_wire(&transport.requests.lock().unwrap()[0], &case);
}

#[tokio::test]
async fn invalid_scan_inputs_are_rejected_without_dispatch() {
    let cases = cases();
    let (scanner, transport) = scanner([]);
    let content = Content::builder().prompt("valid prompt").build().unwrap();
    let missing_profile = AiProfile::builder().build().unwrap();
    assert_eq!(
        scanner
            .sync_scan(&missing_profile, &content, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Validation
    );
    let mut request: ScanRequest = serde_json::from_value(cases[0]["body"].clone()).unwrap();
    request.contents.clear();
    assert_eq!(
        scanner
            .scan_request(&request, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Validation
    );
    let mut batch: Vec<AsyncScanObject> = serde_json::from_value(cases[1]["body"].clone()).unwrap();
    assert_eq!(
        scanner.async_scan(&[], None).await.unwrap_err().kind(),
        ErrorKind::Validation
    );
    assert_eq!(
        scanner
            .async_scan(&vec![batch[0].clone(); 21], None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Validation
    );
    batch[0].scan_req.contents.clear();
    assert_eq!(
        scanner.async_scan(&batch, None).await.unwrap_err().kind(),
        ErrorKind::Validation
    );
    let valid_id = "550e8400-e29b-41d4-a716-446655440000".to_owned();
    for invalid in [vec![], vec!["not-a-uuid".into()], vec![valid_id; 6]] {
        assert_eq!(
            scanner
                .query_by_scan_ids(&invalid, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Validation
        );
    }
    for invalid in [
        vec![],
        vec!["unexpected,second-id".into()],
        vec!["r".into(); 6],
    ] {
        assert_eq!(
            scanner
                .query_by_report_ids(&invalid, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Validation
        );
    }
    assert!(transport.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn malformed_known_response_fields_are_rejected_safely() {
    let case = cases().remove(0);
    let mut invalid = case["responseBody"].clone();
    invalid["timeout"] = json!("private-invalid-value");
    let (scanner, transport) = scanner([invalid]);
    let request: ScanRequest = serde_json::from_value(case["body"].clone()).unwrap();
    let error = scanner.scan_request(&request, None).await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert!(!format!("{error:?} {error}").contains("private-invalid-value"));
    assert_eq!(transport.requests.lock().unwrap().len(), 1);
}

struct AmbiguousFailureTransport {
    requests: Mutex<Vec<TransportRequest>>,
}
impl Transport for AmbiguousFailureTransport {
    fn send<'a>(
        &'a self,
        request: TransportRequest,
    ) -> BoxFuture<'a, scm_sdk::Result<TransportResponse>> {
        Box::pin(async move {
            // The SDK has dispatched a request. A transport failure cannot prove
            // that the remote scan was never accepted or billed.
            self.requests.lock().unwrap().push(request);
            Err(scm_sdk::Error::new(ErrorKind::Transport))
        })
    }
}

#[tokio::test]
async fn sync_and_async_ambiguous_failures_remain_unknown_and_are_never_replayed() {
    let cases = cases();
    let transport = Arc::new(AmbiguousFailureTransport {
        requests: Mutex::new(Vec::new()),
    });
    let client = Client::builder(Endpoint::parse("https://scan-fixture.example.test").unwrap())
        .authentication(
            Authentication::scan(
                Some(Secret::new(API_KEY).unwrap()),
                Some(Secret::new(API_TOKEN).unwrap()),
            )
            .unwrap(),
        )
        .transport(transport.clone())
        .build()
        .unwrap();
    let scanner = Scanner::new(client).unwrap();
    let options = RequestOptions::with_timeout(std::time::Duration::from_secs(5))
        .unwrap()
        .with_retries(5)
        .unwrap();
    let body = &cases[0]["body"];
    let profile: AiProfile = serde_json::from_value(body["ai_profile"].clone()).unwrap();
    let content =
        Content::from_json(serde_json::from_value(body["contents"][0].clone()).unwrap()).unwrap();
    let mut sync_options = SyncScanOptions::default();
    sync_options.tr_id = Some(body["tr_id"].as_str().unwrap().into());
    sync_options.session_id = Some(body["session_id"].as_str().unwrap().into());
    sync_options.metadata = Some(serde_json::from_value(body["metadata"].clone()).unwrap());
    sync_options.request = Some(options.clone());
    let sync = scanner
        .sync_scan(&profile, &content, Some(&sync_options))
        .await
        .unwrap_err();
    assert_eq!(sync.kind(), ErrorKind::Transport);
    assert_eq!(sync.remote_outcome(), RemoteOutcome::Unknown);
    assert_eq!(transport.requests.lock().unwrap().len(), 1);
    let batch: Vec<AsyncScanObject> = serde_json::from_value(cases[1]["body"].clone()).unwrap();
    let asynchronous = scanner
        .async_scan(&batch, Some(&options))
        .await
        .unwrap_err();
    assert_eq!(asynchronous.kind(), ErrorKind::Transport);
    assert_eq!(asynchronous.remote_outcome(), RemoteOutcome::Unknown);
    let requests = transport.requests.lock().unwrap();
    assert_eq!(
        requests.len(),
        2,
        "each paid POST must be dispatched exactly once despite the retry override"
    );
    assert_wire(&requests[0], &cases[0]);
    assert_wire(&requests[1], &cases[1]);
    let diagnostics = format!("{sync:?} {sync} {asynchronous:?} {asynchronous}");
    assert!(!diagnostics.contains(API_KEY));
    assert!(!diagnostics.contains(API_TOKEN));
}
