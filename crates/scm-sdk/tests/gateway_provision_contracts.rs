//! Independent workflow evidence from frozen workspaces-client.ts and IAM wire contracts.
#![cfg(feature = "gateway")]
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, Limits, RemoteOutcome, RequestOptions, Result, Secret,
    airs::gateway::{
        AIGatewayClient, AIGatewayWorkspaceProvisionOptions, AIGatewayWorkspacesClient,
        GatewayWorkspaceProvisionRequest, ProvisionCleanup, ProvisionScopeState, ProvisionStage,
        ProvisionWorkspaceState,
    },
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    config::OriginPolicy,
    data::Optional,
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

const SCOPE: &str = "ws_owned_synthetic_a1b2c3";
const SECRET: &str = "synthetic-provision-bearer-secret";
enum Reply {
    Json(Value),
    ScopeEcho(bool),
    WorkspaceEcho,
    Http(u16),
    Failure,
    Delay(Value, Duration),
}
struct Wire {
    replies: Mutex<VecDeque<Reply>>,
    requests: Mutex<Vec<TransportRequest>>,
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            let input: Value = serde_json::from_slice(request.body()).unwrap_or(Value::Null);
            let last_segment = request
                .url()
                .path_segments()
                .unwrap()
                .next_back()
                .unwrap()
                .to_owned();
            self.requests.lock().unwrap().push(request);
            let reply = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected workflow replay");
            let (status, body) = match reply {
                Reply::Json(body) => (200, body),
                Reply::ScopeEcho(bound) => {
                    let name = input["name"].as_str().unwrap_or(&last_segment);
                    let mut body = scope(bound);
                    body["name"] = json!(name);
                    body["id"] = json!(format!("{name}:12345"));
                    (200, body)
                }
                Reply::WorkspaceEcho => {
                    let mut body = workspace();
                    body["scope_name"] = input["scope_name"].clone();
                    (200, body)
                }
                Reply::Http(status) => (status, json!({"message":SECRET})),
                Reply::Failure => return Err(Error::new(ErrorKind::Transport)),
                Reply::Delay(body, delay) => {
                    tokio::time::sleep(delay).await;
                    (200, body)
                }
            };
            Ok(TransportResponse::from_bytes(
                StatusCode::from_u16(status).unwrap(),
                HeaderMap::new(),
                Bytes::from(serde_json::to_vec(&body).unwrap()),
            ))
        })
    }
}
struct Provider {
    context: AuthContext,
    calls: AtomicUsize,
    cancel_at: usize,
    deadlines: Mutex<Vec<tokio::time::Instant>>,
}
impl TokenProvider for Provider {
    fn context(&self) -> &AuthContext {
        &self.context
    }
    fn invalidate(&self, _: u64) -> bool {
        false
    }
    fn token<'a>(&'a self, options: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            let call = self.calls.fetch_add(1, Ordering::SeqCst) + 1;
            self.deadlines.lock().unwrap().push(options.deadline());
            if call == self.cancel_at {
                options.cancel();
            }
            TokenLease::new(Secret::new(SECRET).unwrap(), 1, Duration::from_secs(3600))
        })
    }
}
fn clients(replies: Vec<Reply>, cancel_at: usize) -> (AIGatewayClient, Arc<Wire>, Arc<Provider>) {
    let wire = Arc::new(Wire {
        replies: Mutex::new(replies.into()),
        requests: Mutex::new(Vec::new()),
    });
    let provider = Arc::new(Provider {
        context: AuthContext::new(
            Endpoint::parse("https://issuer.provision.test/token").unwrap(),
            "fixture-client",
            1,
            "12345",
        )
        .unwrap(),
        calls: AtomicUsize::new(0),
        cancel_at,
        deadlines: Mutex::new(Vec::new()),
    });
    let data = Endpoint::parse("https://data.provision.test/ai_gw/v2").unwrap();
    let admin = Endpoint::parse("https://admin.provision.test/ai_gw/admin/v2").unwrap();
    let iam = Endpoint::parse("https://iam.provision.test/iam/v1").unwrap();
    let client = Client::builder(data.clone())
        .transport(wire.clone())
        .authentication(Authentication::oauth(provider.clone(), false))
        .destination_policy(Arc::new(OriginPolicy::new([
            data.clone(),
            admin.clone(),
            iam.clone(),
        ])))
        .limits(Limits::default())
        .build()
        .unwrap();
    (
        AIGatewayClient::new(client, admin, iam).unwrap(),
        wire,
        provider,
    )
}
fn scope(bound: bool) -> Value {
    let mut resources = vec![
        json!({"resource_type":"other","resource_id":"preserved","metadata":[{"future":true}]}),
    ];
    if bound {
        resources
            .push(json!({"resource_type":"workspace","resource_id":"ws-created","metadata":[]}));
    }
    json!({"name":SCOPE,"description":"preserved description","resources":resources,"tsg_id":"12345","id":format!("{SCOPE}:12345")})
}
fn workspace() -> Value {
    json!({"id":"550e8400-e29b-41d4-a716-446655440000","name":"Owned workspace","slug":"ws-created",
        "description":null,"created_at":"2026-01-01T00:00:00Z","last_updated_at":"2026-01-01T00:00:00Z",
        "scope_name":SCOPE,"object":"workspace","future_response":true})
}
fn request() -> GatewayWorkspaceProvisionRequest {
    let mut value = GatewayWorkspaceProvisionRequest::new("Owned workspace");
    value.scope_name = Optional::Value(SCOPE.into());
    value.description = Optional::Value("scope creation description".into());
    value.icon = Optional::Value("icon".into());
    value
}
fn assert_request(request: &TransportRequest, method: &str, url: &str, body: Option<Value>) {
    assert_eq!(request.method().as_str(), method);
    assert_eq!(request.url().as_str(), url);
    let mut names = request
        .headers()
        .keys()
        .map(|name| name.as_str())
        .collect::<Vec<_>>();
    names.sort_unstable();
    assert_eq!(
        names,
        if body.is_some() {
            vec![
                "authorization",
                "content-type",
                "service-name",
                "user-agent",
                "x-tsg-id",
            ]
        } else {
            vec!["authorization", "service-name", "user-agent", "x-tsg-id"]
        }
    );
    assert_eq!(request.headers().len(), names.len());
    for name in names {
        assert_eq!(request.headers().get_all(name).iter().count(), 1);
        assert!(request.headers()[name].is_sensitive());
    }
    assert_eq!(
        request.headers()["authorization"],
        format!("Bearer {SECRET}")
    );
    assert_eq!(request.headers()["x-tsg-id"], "12345");
    assert_eq!(request.headers()["service-name"], "api");
    assert_eq!(
        request.headers()["user-agent"],
        concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
    );
    if let Some(body) = body {
        assert_eq!(
            serde_json::from_slice::<Value>(request.body()).unwrap(),
            body
        );
    } else {
        assert!(request.body().is_empty());
    }
}
#[tokio::test]
async fn new_scope_workspace_and_additive_binding_match_exact_source_sequence() {
    let (gateway, wire, provider) = clients(
        vec![
            Reply::Json(scope(false)),
            Reply::Json(workspace()),
            Reply::Json(scope(false)),
            Reply::Json(scope(true)),
        ],
        0,
    );
    let options = RequestOptions::with_timeout(Duration::from_secs(10))
        .unwrap()
        .with_retries(5)
        .unwrap();
    let result = gateway
        .workspaces()
        .provision(
            &request(),
            &AIGatewayWorkspaceProvisionOptions::default(),
            Some(&options),
        )
        .await
        .unwrap();
    assert!(result.scope_created);
    assert_eq!(serde_json::to_value(result.scope).unwrap(), scope(true));
    assert_eq!(serde_json::to_value(result.workspace).unwrap(), workspace());
    let requests = wire.requests.lock().unwrap();
    assert_eq!(requests.len(), 4);
    assert_request(
        &requests[0],
        "POST",
        "https://iam.provision.test/iam/v1/scopes",
        Some(json!({"name":SCOPE,"description":"scope creation description","resources":[]})),
    );
    assert_request(
        &requests[1],
        "POST",
        "https://admin.provision.test/ai_gw/admin/v2/workspaces",
        Some(
            json!({"name":"Owned workspace","scope_name":SCOPE,"description":"scope creation description","icon":"icon"}),
        ),
    );
    assert_request(
        &requests[2],
        "GET",
        &format!("https://iam.provision.test/iam/v1/scopes/{SCOPE}"),
        None,
    );
    assert_request(
        &requests[3],
        "PUT",
        &format!("https://iam.provision.test/iam/v1/scopes/{SCOPE}"),
        Some(
            json!({"name":SCOPE,"description":"preserved description","resources":[
        {"resource_type":"other","resource_id":"preserved","metadata":[{"future":true}]},
        {"resource_type":"workspace","resource_id":"ws-created","metadata":[]}]}),
        ),
    );
    assert!(
        provider
            .deadlines
            .lock()
            .unwrap()
            .iter()
            .all(|deadline| *deadline == options.deadline())
    );
}
#[tokio::test]
async fn reused_scope_is_never_created_or_deleted_and_existing_binding_is_idempotent() {
    let (gateway, wire, _) = clients(
        vec![
            Reply::Json(workspace()),
            Reply::Json(scope(true)),
            Reply::Json(scope(true)),
        ],
        0,
    );
    let result = gateway
        .workspaces()
        .provision(
            &request(),
            &AIGatewayWorkspaceProvisionOptions {
                existing_scope: true,
            },
            None,
        )
        .await
        .unwrap();
    assert!(!result.scope_created);
    assert_eq!(result.scope.resources.len(), 2);
    assert_eq!(
        wire.requests
            .lock()
            .unwrap()
            .iter()
            .map(|r| r.method().as_str())
            .collect::<Vec<_>>(),
        ["POST", "GET", "PUT"]
    );
    let (gateway, wire, _) = clients(vec![Reply::Http(403)], 0);
    let error = gateway
        .workspaces()
        .provision(
            &request(),
            &AIGatewayWorkspaceProvisionOptions {
                existing_scope: true,
            },
            None,
        )
        .await
        .unwrap_err();
    assert_eq!(error.recovery().scope_state, ProvisionScopeState::Existing);
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
}
#[tokio::test]
async fn ambiguous_creations_never_replay_or_delete_a_potential_dependency() {
    for reply in [
        Reply::Failure,
        Reply::Http(503),
        Reply::Json(json!({"invalid":true})),
    ] {
        let (gateway, wire, _) = clients(vec![Reply::Json(scope(false)), reply], 0);
        let options = RequestOptions::with_timeout(Duration::from_secs(10))
            .unwrap()
            .with_retries(5)
            .unwrap();
        let error = gateway
            .workspaces()
            .provision(
                &request(),
                &AIGatewayWorkspaceProvisionOptions::default(),
                Some(&options),
            )
            .await
            .unwrap_err();
        assert_eq!(error.error().remote_outcome(), RemoteOutcome::Unknown);
        assert_eq!(error.recovery().stage, ProvisionStage::CreateWorkspace);
        assert_eq!(error.recovery().scope_state, ProvisionScopeState::Created);
        assert_eq!(
            error.recovery().workspace_state,
            ProvisionWorkspaceState::Unknown
        );
        assert_eq!(error.recovery().cleanup, ProvisionCleanup::Retained);
        assert_eq!(wire.requests.lock().unwrap().len(), 2);
        assert!(!format!("{error:?} {error} {:?}", error.recovery()).contains(SECRET));
    }
    let (gateway, wire, _) = clients(vec![Reply::Failure], 0);
    let error = gateway
        .workspaces()
        .provision(
            &request(),
            &AIGatewayWorkspaceProvisionOptions::default(),
            None,
        )
        .await
        .unwrap_err();
    assert_eq!(error.recovery().stage, ProvisionStage::CreateScope);
    assert_eq!(error.recovery().scope_state, ProvisionScopeState::Unknown);
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
}
#[tokio::test]
async fn definite_workspace_rejection_can_clean_only_owned_scope_and_preserves_cleanup_failure() {
    for cleanup in [Reply::Json(json!({})), Reply::Http(405), Reply::Failure] {
        let (gateway, wire, _) = clients(
            vec![Reply::Json(scope(false)), Reply::Http(400), cleanup],
            0,
        );
        let error = gateway
            .workspaces()
            .provision(
                &request(),
                &AIGatewayWorkspaceProvisionOptions::default(),
                None,
            )
            .await
            .unwrap_err();
        assert_eq!(error.error().status(), Some(400));
        assert_eq!(
            error.recovery().workspace_state,
            ProvisionWorkspaceState::NotCreated
        );
        let requests = wire.requests.lock().unwrap();
        assert_eq!(requests.len(), 3);
        assert_request(
            &requests[2],
            "DELETE",
            &format!("https://iam.provision.test/iam/v1/scopes/{SCOPE}"),
            None,
        );
        if let Some(cleanup) = &error.recovery().cleanup_error {
            assert_eq!(error.recovery().cleanup, ProvisionCleanup::Failed);
            assert_eq!(
                error.recovery().scope_state,
                if cleanup.remote_outcome() == RemoteOutcome::Unknown {
                    ProvisionScopeState::Unknown
                } else {
                    ProvisionScopeState::Created
                }
            );
        } else {
            assert_eq!(error.recovery().cleanup, ProvisionCleanup::Deleted);
            assert_eq!(error.recovery().scope_state, ProvisionScopeState::Deleted);
        }
    }
}
#[tokio::test]
async fn binding_failure_keeps_confirmed_workspace_receipt_for_explicit_recovery() {
    for replies in [
        vec![
            Reply::Json(scope(false)),
            Reply::Json(workspace()),
            Reply::Http(403),
        ],
        vec![
            Reply::Json(scope(false)),
            Reply::Json(workspace()),
            Reply::Json(scope(false)),
            Reply::Failure,
        ],
    ] {
        let (gateway, wire, _) = clients(replies, 0);
        let error = gateway
            .workspaces()
            .provision(
                &request(),
                &AIGatewayWorkspaceProvisionOptions::default(),
                None,
            )
            .await
            .unwrap_err();
        assert_eq!(error.recovery().stage, ProvisionStage::BindWorkspace);
        assert_eq!(
            error.recovery().workspace_state,
            ProvisionWorkspaceState::Created
        );
        assert_eq!(error.recovery().workspace_outcome, None);
        assert_eq!(
            serde_json::to_value(error.recovery().workspace.as_ref().unwrap()).unwrap(),
            workspace()
        );
        assert_eq!(error.recovery().scope_name.as_deref(), Some(SCOPE));
        assert_eq!(error.recovery().cleanup, ProvisionCleanup::Retained);
        assert!(
            wire.requests
                .lock()
                .unwrap()
                .iter()
                .all(|r| r.method() != http::Method::DELETE)
        );
    }
}
#[tokio::test]
async fn preflight_checks_every_input_and_iam_configuration_before_any_authentication() {
    let (gateway, wire, provider) = clients(vec![], 0);
    let leaf = AIGatewayWorkspacesClient::new(
        gateway.client().clone(),
        Endpoint::parse("https://admin.provision.test/ai_gw/admin/v2").unwrap(),
    )
    .unwrap();
    assert_eq!(
        leaf.provision(
            &request(),
            &AIGatewayWorkspaceProvisionOptions::default(),
            None
        )
        .await
        .unwrap_err()
        .error()
        .kind(),
        ErrorKind::Configuration
    );
    let leaf = leaf
        .with_iam_endpoint(Endpoint::parse("https://iam.provision.test/iam/v1").unwrap())
        .unwrap();
    for mut request in [
        GatewayWorkspaceProvisionRequest::new(""),
        request(),
        request(),
    ] {
        if request.name == "Owned workspace" {
            request.scope_name = Optional::Value("../invalid".into());
        }
        assert_eq!(
            leaf.provision(
                &request,
                &AIGatewayWorkspaceProvisionOptions::default(),
                None
            )
            .await
            .unwrap_err()
            .error()
            .kind(),
            ErrorKind::Validation
        );
    }
    assert_eq!(
        gateway
            .workspaces()
            .provision(
                &GatewayWorkspaceProvisionRequest::new("valid"),
                &AIGatewayWorkspaceProvisionOptions {
                    existing_scope: true
                },
                None
            )
            .await
            .unwrap_err()
            .error()
            .kind(),
        ErrorKind::Validation
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert!(wire.requests.lock().unwrap().is_empty());
}
#[tokio::test]
async fn cancellation_between_steps_and_total_deadline_do_not_start_new_dispatches() {
    let (gateway, wire, provider) = clients(vec![Reply::Json(scope(false))], 2);
    let options = RequestOptions::with_timeout(Duration::from_secs(10)).unwrap();
    let error = gateway
        .workspaces()
        .provision(
            &request(),
            &AIGatewayWorkspaceProvisionOptions::default(),
            Some(&options),
        )
        .await
        .unwrap_err();
    assert_eq!(error.error().kind(), ErrorKind::Cancelled);
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
    assert!(
        provider
            .deadlines
            .lock()
            .unwrap()
            .iter()
            .all(|deadline| *deadline == options.deadline())
    );
    let (gateway, wire, _) = clients(vec![Reply::Delay(scope(false), Duration::from_secs(1))], 0);
    let options = RequestOptions::with_timeout(Duration::from_millis(5)).unwrap();
    let error = gateway
        .workspaces()
        .provision(
            &request(),
            &AIGatewayWorkspaceProvisionOptions::default(),
            Some(&options),
        )
        .await
        .unwrap_err();
    assert_eq!(error.error().kind(), ErrorKind::Deadline);
    assert_eq!(error.recovery().scope_state, ProvisionScopeState::Unknown);
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn generated_scope_name_is_used_consistently_and_mismatched_receipts_stop_recovery() {
    let (gateway, wire, _) = clients(
        vec![
            Reply::ScopeEcho(false),
            Reply::WorkspaceEcho,
            Reply::ScopeEcho(false),
            Reply::ScopeEcho(true),
        ],
        0,
    );
    let result = gateway
        .workspaces()
        .provision(
            &GatewayWorkspaceProvisionRequest::new("Owned workspace"),
            &AIGatewayWorkspaceProvisionOptions::default(),
            None,
        )
        .await
        .unwrap();
    assert!(result.scope.name.starts_with("ws_owned_workspace_"));
    assert_eq!(result.scope.name.len(), "ws_owned_workspace_".len() + 6);
    assert_eq!(result.workspace.scope_name, result.scope.name);
    let created: Value = {
        let requests = wire.requests.lock().unwrap();
        serde_json::from_slice(requests[0].body()).unwrap()
    };
    assert_eq!(
        created,
        json!({"name":result.scope.name,"description":"","resources":[]})
    );
    let mut wrong_scope = scope(false);
    wrong_scope["name"] = json!("different-owned-scope");
    let mut wrong_workspace = workspace();
    wrong_workspace["scope_name"] = json!("different-owned-scope");
    for replies in [
        vec![Reply::Json(wrong_scope)],
        vec![Reply::Json(scope(false)), Reply::Json(wrong_workspace)],
    ] {
        let count = replies.len();
        let (gateway, wire, _) = clients(replies, 0);
        let error = gateway
            .workspaces()
            .provision(
                &request(),
                &AIGatewayWorkspaceProvisionOptions::default(),
                None,
            )
            .await
            .unwrap_err();
        assert_eq!(error.error().kind(), ErrorKind::Protocol);
        assert_eq!(error.error().remote_outcome(), RemoteOutcome::Unknown);
        assert_eq!(wire.requests.lock().unwrap().len(), count);
    }
}
