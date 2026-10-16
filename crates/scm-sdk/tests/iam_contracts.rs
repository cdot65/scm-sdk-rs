//! Independent observed IAM contracts from frozen src/iam/scopes-client.ts.
#![cfg(feature = "iam")]
use bytes::Bytes;
use futures_util::{future::BoxFuture, stream};
use http::{HeaderMap, Method, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, Limits, RemoteOutcome, RequestOptions, Result, Secret,
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    shared::iam::{
        IamScopesClient,
        models::{IamScopeCreateInput, IamScopeUpdateInput},
        validate_scope_name,
    },
    transport::{Transport, TransportRequest, TransportResponse},
};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

enum Reply {
    Json(Value),
    Http(u16, Value),
    Failure,
    Delay(Value, Duration),
}
struct Wire {
    requests: Mutex<Vec<TransportRequest>>,
    replies: Mutex<VecDeque<Reply>>,
}
impl Wire {
    fn new(replies: Vec<Reply>) -> Arc<Self> {
        Arc::new(Self {
            requests: Mutex::new(Vec::new()),
            replies: Mutex::new(replies.into()),
        })
    }
    fn count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.lock().unwrap().push(request);
            let reply = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected replay");
            let (status, value) = match reply {
                Reply::Json(value) => (StatusCode::OK, value),
                Reply::Http(status, value) => (StatusCode::from_u16(status).unwrap(), value),
                Reply::Failure => return Err(Error::new(ErrorKind::Transport)),
                Reply::Delay(value, delay) => {
                    tokio::time::sleep(delay).await;
                    (StatusCode::OK, value)
                }
            };
            Ok(TransportResponse::new(
                status,
                HeaderMap::new(),
                Box::pin(stream::iter([Ok(Bytes::from(
                    serde_json::to_vec(&value).unwrap(),
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
                Secret::new("synthetic-iam-bearer").unwrap(),
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
fn client(wire: Arc<Wire>, provider: Arc<Provider>, limits: Limits) -> IamScopesClient {
    IamScopesClient::new(
        Client::builder(Endpoint::parse("https://iam.example/iam/v1").unwrap())
            .transport(wire)
            .authentication(Authentication::oauth(provider, false))
            .limits(limits)
            .build()
            .unwrap(),
    )
    .unwrap()
}
fn scope() -> Value {
    json!({"name":"ws_fixture_a1b2c3","description":"kept description","resources":[
        {"resource_type":"workspace","resource_id":"ws-existing","future_resource":true},
        {"resource_type":"other","resource_id":"other-id","metadata":[{"future":"retained"}]}],
        "tsg_id":"123456789","id":"ws_fixture_a1b2c3:123456789","future_scope":{"retained":true}})
}
fn create_input(value: Value) -> IamScopeCreateInput {
    serde_json::from_value(value).unwrap()
}
fn update_input(value: Value) -> IamScopeUpdateInput {
    serde_json::from_value(value).unwrap()
}
fn assert_wire(request: &TransportRequest, method: Method, path: &str, body: Option<Value>) {
    assert_eq!(request.method(), method);
    assert_eq!(
        request.url().as_str(),
        format!("https://iam.example/iam/v1{path}")
    );
    assert!(request.url().query().is_none());
    let mut names: Vec<_> = request
        .headers()
        .keys()
        .map(http::HeaderName::as_str)
        .collect();
    names.sort_unstable();
    let expected = if body.is_some() {
        vec![
            "authorization",
            "content-type",
            "service-name",
            "user-agent",
            "x-tsg-id",
        ]
    } else {
        vec!["authorization", "service-name", "user-agent", "x-tsg-id"]
    };
    assert_eq!(names, expected);
    assert_eq!(request.headers().len(), expected.len());
    assert_eq!(
        request.headers()["authorization"],
        "Bearer synthetic-iam-bearer"
    );
    assert_eq!(request.headers()["x-tsg-id"], "123456789");
    assert_eq!(request.headers()["service-name"], "api");
    assert_eq!(
        request.headers()["user-agent"],
        concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
    );
    match body {
        Some(body) => assert_eq!(
            serde_json::from_slice::<Value>(request.body()).unwrap(),
            body
        ),
        None => assert!(request.body().is_empty()),
    }
    assert!(!format!("{request:?}").contains("synthetic-iam-bearer"));
}

#[test]
fn scope_name_grammar_retains_all_frozen_ascii_forms() {
    for name in ["A", "_", "123", "_scope", "Scope.v2-1", "a..b"] {
        validate_scope_name(name).unwrap();
    }
}

#[tokio::test]
async fn list_and_get_retain_observed_unknown_fields_and_derive_tenant_header() {
    let payload = scope();
    let page = json!({"count":1,"items":[payload.clone()],"future_page":true});
    let wire = Wire::new(vec![
        Reply::Json(page.clone()),
        Reply::Json(payload.clone()),
    ]);
    let provider = Provider::new(false);
    let client = client(wire.clone(), provider.clone(), Limits::default());
    assert_eq!(
        serde_json::to_value(client.list(None).await.unwrap()).unwrap(),
        page
    );
    assert_eq!(
        serde_json::to_value(client.get("ws_fixture_a1b2c3", None).await.unwrap()).unwrap(),
        payload
    );
    let requests = wire.requests.lock().unwrap();
    assert_wire(&requests[0], Method::GET, "/scopes", None);
    assert_wire(&requests[1], Method::GET, "/scopes/ws_fixture_a1b2c3", None);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 2);
}

#[tokio::test]
async fn create_defaults_and_update_full_replacement_match_frozen_wire_bodies() {
    let wire = Wire::new(vec![
        Reply::Json(scope()),
        Reply::Json(scope()),
        Reply::Json(scope()),
    ]);
    let client = client(wire.clone(), Provider::new(false), Limits::default());
    client
        .create(&create_input(json!({"name":"fixture"})), None)
        .await
        .unwrap();
    client.create(&create_input(json!({"name":"fixture","resources":[{"resource_type":"workspace","resource_id":"ws-a"}]})),None).await.unwrap();
    client
        .update("fixture", &update_input(json!({})), None)
        .await
        .unwrap();
    let requests = wire.requests.lock().unwrap();
    assert_wire(
        &requests[0],
        Method::POST,
        "/scopes",
        Some(json!({"name":"fixture","description":"","resources":[]})),
    );
    assert_wire(
        &requests[1],
        Method::POST,
        "/scopes",
        Some(
            json!({"name":"fixture","description":"","resources":[{"resource_type":"workspace","resource_id":"ws-a","metadata":[]}]}),
        ),
    );
    assert_wire(
        &requests[2],
        Method::PUT,
        "/scopes/fixture",
        Some(json!({"name":"fixture","description":"","resources":[]})),
    );
}

#[tokio::test]
async fn bind_is_additive_and_idempotent_without_dropping_existing_metadata() {
    for already_bound in [false, true] {
        let mut initial = scope();
        if already_bound {
            initial["resources"]
                .as_array_mut()
                .unwrap()
                .push(json!({"resource_type":"workspace","resource_id":"ws-new","metadata":[]}));
        }
        let wire = Wire::new(vec![Reply::Json(initial), Reply::Json(scope())]);
        let client = client(wire.clone(), Provider::new(false), Limits::default());
        client
            .bind_workspace("ws_fixture_a1b2c3", "ws-new", None)
            .await
            .unwrap();
        let requests = wire.requests.lock().unwrap();
        assert_wire(&requests[0], Method::GET, "/scopes/ws_fixture_a1b2c3", None);
        assert_wire(
            &requests[1],
            Method::PUT,
            "/scopes/ws_fixture_a1b2c3",
            Some(
                json!({"name":"ws_fixture_a1b2c3","description":"kept description","resources":[
            {"resource_type":"workspace","resource_id":"ws-existing","metadata":[]},
            {"resource_type":"other","resource_id":"other-id","metadata":[{"future":"retained"}]},
            {"resource_type":"workspace","resource_id":"ws-new","metadata":[]}]}),
            ),
        );
    }
}

#[tokio::test]
async fn experimental_delete_uses_exact_route_and_preserves_service_rejection() {
    let wire = Wire::new(vec![
        Reply::Json(Value::Null),
        Reply::Http(405, json!({"message":"synthetic-private-body"})),
    ]);
    let client = client(wire.clone(), Provider::new(false), Limits::default());
    client.delete("fixture", None).await.unwrap();
    let error = client.delete("fixture", None).await.unwrap_err();
    assert_eq!(error.status(), Some(405));
    assert_eq!(error.remote_outcome(), RemoteOutcome::Rejected);
    assert!(!format!("{error:?} {error}").contains("synthetic-private-body"));
    assert_wire(
        &wire.requests.lock().unwrap()[0],
        Method::DELETE,
        "/scopes/fixture",
        None,
    );
}

#[tokio::test]
async fn invalid_names_inputs_and_workspace_slugs_fail_before_authentication_or_dispatch() {
    let wire = Wire::new(vec![]);
    let provider = Provider::new(false);
    let client = client(wire.clone(), provider.clone(), Limits::default());
    for name in [
        "",
        "..",
        "a/b",
        "a:b",
        "white space",
        "%2e%2e",
        "-leading",
        "é",
    ] {
        for error in [
            client.get(name, None).await.unwrap_err(),
            client
                .update(name, &update_input(json!({})), None)
                .await
                .unwrap_err(),
            client.delete(name, None).await.unwrap_err(),
            client
                .bind_workspace(name, "ws-good", None)
                .await
                .unwrap_err(),
        ] {
            assert_eq!(error.kind(), ErrorKind::Validation);
            assert_eq!(error.remote_outcome(), RemoteOutcome::NotSent);
        }
    }
    for slug in ["", "..", "ws/a", "ws a", "ws:a", "ws%2fa", "_leading"] {
        assert_eq!(
            client
                .bind_workspace("good", slug, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Validation
        );
    }
    for value in [
        json!({"name":""}),
        json!({"name":"good","resources":[{"resource_type":"","resource_id":"ws-a"}]}),
        json!({"name":"good","resources":[{"resource_type":"workspace","resource_id":"ws-a","unapproved":true}]}),
    ] {
        assert_eq!(
            client
                .create(&create_input(value), None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Validation
        );
    }
    assert_eq!(wire.count(), 0);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn all_writes_and_bind_put_preserve_unknown_outcome_without_replay() {
    for operation in ["create", "update", "delete", "bind"] {
        for response in [
            Reply::Failure,
            Reply::Http(503, json!({"message":"synthetic-private-body"})),
        ] {
            let mut replies = Vec::new();
            if operation == "bind" {
                replies.push(Reply::Json(scope()));
            }
            replies.push(response);
            let wire = Wire::new(replies);
            let client = client(wire.clone(), Provider::new(false), Limits::default());
            let options = RequestOptions::with_timeout(Duration::from_secs(1))
                .unwrap()
                .with_retries(5)
                .unwrap();
            let error = match operation {
                "create" => client
                    .create(&create_input(json!({"name":"good"})), Some(&options))
                    .await
                    .unwrap_err(),
                "update" => client
                    .update("good", &update_input(json!({})), Some(&options))
                    .await
                    .unwrap_err(),
                "delete" => client.delete("good", Some(&options)).await.unwrap_err(),
                _ => client
                    .bind_workspace("good", "ws-new", Some(&options))
                    .await
                    .unwrap_err(),
            };
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
            assert_eq!(wire.count(), if operation == "bind" { 2 } else { 1 });
            assert!(!format!("{error:?} {error}").contains("synthetic-private-body"));
        }
    }
}

#[tokio::test]
async fn cancellation_after_scope_read_prevents_workspace_write() {
    let wire = Wire::new(vec![Reply::Json(scope()), Reply::Json(scope())]);
    let client = client(wire.clone(), Provider::new(true), Limits::default());
    let error = client
        .bind_workspace("good", "ws-new", None)
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(wire.count(), 1);
}

#[tokio::test(start_paused = true)]
async fn bind_read_and_write_share_one_total_deadline() {
    let wire = Wire::new(vec![
        Reply::Delay(scope(), Duration::from_millis(600)),
        Reply::Delay(scope(), Duration::from_millis(600)),
    ]);
    let mut limits = Limits::default();
    limits.operation_timeout = Duration::from_secs(1);
    let client = client(wire.clone(), Provider::new(false), limits);
    let error = client
        .bind_workspace("good", "ws-new", None)
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Deadline);
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
    assert_eq!(wire.count(), 2);
}

#[tokio::test]
async fn malformed_scope_response_is_sanitized_and_authentication_plane_is_enforced() {
    let wire = Wire::new(vec![Reply::Json(json!({"name":"synthetic-private-body"}))]);
    let client = client(wire.clone(), Provider::new(false), Limits::default());
    let error = client.get("good", None).await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
    assert!(!format!("{error:?} {error}").contains("synthetic-private-body"));
    let raw = Client::builder(Endpoint::parse("https://iam.example/iam/v1").unwrap())
        .transport(wire)
        .authentication(
            Authentication::runtime_key(Secret::new("synthetic-runtime-secret").unwrap()).unwrap(),
        )
        .build()
        .unwrap();
    assert_eq!(
        IamScopesClient::new(raw).unwrap_err().kind(),
        ErrorKind::Configuration
    );
}
