//! Independent management OAuth wire contracts; literal responses are not model output.
#![cfg(feature = "management")]
use bytes::Bytes;
use futures_util::{future::BoxFuture, stream};
use http::{HeaderMap, Method, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, RemoteOutcome, RequestOptions, Result, Secret,
    airs::management::{GetTokenOptions, OAuthManagementClient, models::ClientIdAndCustomerApp},
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
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
    Raw(Bytes),
    Http(u16, Value),
    Failure,
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
            let (status, body) = match self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected mutation replay")
            {
                Reply::Json(value) => (
                    StatusCode::OK,
                    Bytes::from(serde_json::to_vec(&value).unwrap()),
                ),
                Reply::Raw(bytes) => (StatusCode::OK, bytes),
                Reply::Http(status, value) => (
                    StatusCode::from_u16(status).unwrap(),
                    Bytes::from(serde_json::to_vec(&value).unwrap()),
                ),
                Reply::Failure => return Err(Error::new(ErrorKind::Transport)),
            };
            Ok(TransportResponse::new(
                status,
                HeaderMap::new(),
                Box::pin(stream::iter([Ok(body)])),
            ))
        })
    }
}
struct Provider {
    context: AuthContext,
    calls: AtomicUsize,
}
impl Provider {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            context: AuthContext::new(
                Endpoint::parse("https://issuer.example/token").unwrap(),
                "fixture-service-client",
                1,
                "123456789",
            )
            .unwrap(),
            calls: AtomicUsize::new(0),
        })
    }
}
impl TokenProvider for Provider {
    fn token<'a>(&'a self, _: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            TokenLease::new(
                Secret::new("synthetic-service-bearer").unwrap(),
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
fn client(wire: Arc<Wire>, provider: Arc<Provider>) -> OAuthManagementClient {
    OAuthManagementClient::new(
        Client::builder(Endpoint::parse("https://management.example").unwrap())
            .authentication(Authentication::oauth(provider, true))
            .transport(wire)
            .build()
            .unwrap(),
    )
    .unwrap()
}
fn body() -> ClientIdAndCustomerApp {
    serde_json::from_value(json!({"client_id":"fixture-client","customer_app":"fixture-app","future_request":{"retained":true}})).unwrap()
}
fn token_response() -> Value {
    json!({"token_type":"Bearer","issued_at":"123","client_id":"fixture-client",
    "access_token":"synthetic-created-token","expires_in":"86400","status":"approved","future_token":{"retained":true}})
}
fn assert_wire(request: &TransportRequest, path_and_query: &str) {
    assert_eq!(request.method(), Method::POST);
    assert_eq!(
        request.url().as_str(),
        format!("https://management.example{path_and_query}")
    );
    let mut names: Vec<_> = request
        .headers()
        .keys()
        .map(http::HeaderName::as_str)
        .collect();
    names.sort_unstable();
    let expected = [
        "authorization",
        "content-type",
        "service-name",
        "user-agent",
    ];
    assert_eq!(names, expected);
    assert_eq!(request.headers().len(), expected.len());
    assert_eq!(
        request.headers()["authorization"],
        "Bearer synthetic-service-bearer"
    );
    assert_eq!(request.headers()["content-type"], "application/json");
    assert_eq!(request.headers()["service-name"], "api");
    assert_eq!(
        request.headers()["user-agent"],
        concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
    );
    assert_eq!(
        serde_json::from_slice::<Value>(request.body()).unwrap(),
        json!({"client_id":"fixture-client","customer_app":"fixture-app","future_request":{"retained":true}})
    );
    assert!(!format!("{request:?}").contains("synthetic-service-bearer"));
}

#[tokio::test]
async fn token_request_preserves_exact_ttl_dialect_body_and_unknown_response_fields() {
    // Frozen test/openapi/fixtures/management.json supplies this method/path,
    // tokenTtlInterval/tokenTtlUnit query spelling and string-token response schema.
    let payload = token_response();
    let wire = Wire::new(vec![Reply::Json(payload.clone())]);
    let client = client(wire.clone(), Provider::new());
    let input = GetTokenOptions::new(body())
        .with_token_ttl_interval(3u32.into())
        .with_token_ttl_unit("hours");
    let response = client.get_access_token(&input, None).await.unwrap();
    assert_eq!(serde_json::to_value(&response).unwrap(), payload);
    assert_wire(
        &wire.requests.lock().unwrap()[0],
        "/v1/mgmt/oauth/client_credential/accesstoken?tokenTtlInterval=3&tokenTtlUnit=hours",
    );
    assert!(!format!("{response:?} {input:?} {client:?}").contains("synthetic-created-token"));
    assert!(!format!("{input:?}").contains("fixture-client"));
}

#[tokio::test]
async fn token_ttl_omission_fractional_interval_and_encoded_unit_match_source_strings() {
    let wire = Wire::new(vec![
        Reply::Json(token_response()),
        Reply::Json(token_response()),
    ]);
    let client = client(wire.clone(), Provider::new());
    client
        .get_access_token(&GetTokenOptions::new(body()), None)
        .await
        .unwrap();
    client
        .get_access_token(
            &GetTokenOptions::new(body())
                .with_token_ttl_interval(1.5f64.try_into().unwrap())
                .with_token_ttl_unit("hours & minutes"),
            None,
        )
        .await
        .unwrap();
    let requests = wire.requests.lock().unwrap();
    assert_wire(&requests[0], "/v1/mgmt/oauth/client_credential/accesstoken");
    assert_wire(
        &requests[1],
        "/v1/mgmt/oauth/client_credential/accesstoken?tokenTtlInterval=1.5&tokenTtlUnit=hours+%26+minutes",
    );
}

#[tokio::test]
async fn invalidation_encodes_secret_query_exactly_and_returns_json_string() {
    let wire = Wire::new(vec![Reply::Json(json!("token invalidated"))]);
    let client = client(wire.clone(), Provider::new());
    let token = Secret::new("synthetic-old+token/&?=secret").unwrap();
    assert_eq!(
        client
            .invalidate_token(&token, &body(), None)
            .await
            .unwrap(),
        "token invalidated"
    );
    let requests = wire.requests.lock().unwrap();
    assert_wire(
        &requests[0],
        "/v1/mgmt/oauth/invalidateToken?token=synthetic-old%2Btoken%2F%26%3F%3Dsecret",
    );
    assert!(!format!("{:?} {token:?} {client:?}", requests[0]).contains(token.expose()));
    assert!(!format!("{:?}", requests[0]).contains("synthetic-old"));
}

#[tokio::test]
async fn arbitrary_echoes_and_wrong_response_kinds_are_safe_unknown_decode_failures() {
    for invalidate in [false, true] {
        for reply in [
            Reply::Json(json!({"access_token":17,"message":"synthetic-sensitive-echo"})),
            Reply::Raw(Bytes::from_static(b"synthetic-sensitive-echo")),
        ] {
            let wire = Wire::new(vec![reply]);
            let client = client(wire.clone(), Provider::new());
            let error = if invalidate {
                client
                    .invalidate_token(
                        &Secret::new("synthetic-sensitive-echo").unwrap(),
                        &body(),
                        None,
                    )
                    .await
                    .unwrap_err()
            } else {
                client
                    .get_access_token(&GetTokenOptions::new(body()), None)
                    .await
                    .unwrap_err()
            };
            assert_eq!(error.kind(), ErrorKind::Decode);
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
            assert!(!format!("{error:?} {error}").contains("synthetic-sensitive-echo"));
            assert_eq!(wire.count(), 1);
        }
    }
}

#[tokio::test]
async fn both_token_mutations_are_never_replayed_after_uncertain_failure() {
    for invalidate in [false, true] {
        for reply in [
            Reply::Failure,
            Reply::Http(503, json!({"message":"synthetic-sensitive-echo"})),
        ] {
            let wire = Wire::new(vec![reply]);
            let client = client(wire.clone(), Provider::new());
            let options = RequestOptions::with_timeout(Duration::from_secs(1))
                .unwrap()
                .with_retries(5)
                .unwrap();
            let error = if invalidate {
                client
                    .invalidate_token(
                        &Secret::new("synthetic-sensitive-echo").unwrap(),
                        &body(),
                        Some(&options),
                    )
                    .await
                    .unwrap_err()
            } else {
                client
                    .get_access_token(&GetTokenOptions::new(body()), Some(&options))
                    .await
                    .unwrap_err()
            };
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
            assert_eq!(wire.count(), 1);
            assert!(!format!("{error:?} {error}").contains("synthetic-sensitive-echo"));
        }
    }
}

#[tokio::test]
async fn invalid_body_collision_and_cancelled_options_fail_before_authentication_or_network() {
    let wire = Wire::new(vec![]);
    let provider = Provider::new();
    let client = client(wire.clone(), provider.clone());
    let mut invalid = body();
    invalid.extensions.insert(
        "client_id".into(),
        serde_json::from_value(json!("synthetic-sensitive-echo")).unwrap(),
    );
    let error = client
        .get_access_token(&GetTokenOptions::new(invalid.clone()), None)
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Validation);
    let error = client
        .invalidate_token(
            &Secret::new("synthetic-sensitive-echo").unwrap(),
            &invalid,
            None,
        )
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Validation);
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    options.cancel();
    assert_eq!(
        client
            .get_access_token(&GetTokenOptions::new(body()), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert_eq!(wire.count(), 0);
}

#[test]
fn oauth_management_constructor_rejects_runtime_key_authentication() {
    let wire = Wire::new(vec![]);
    let raw = Client::builder(Endpoint::parse("https://management.example").unwrap())
        .transport(wire.clone())
        .authentication(
            Authentication::runtime_key(Secret::new("synthetic-runtime-secret").unwrap()).unwrap(),
        )
        .build()
        .unwrap();
    let error = OAuthManagementClient::new(raw).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Configuration);
    assert_eq!(error.remote_outcome(), RemoteOutcome::NotSent);
    assert_eq!(wire.count(), 0);
}
