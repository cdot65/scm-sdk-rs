//! Literal wire contracts for the Portkey 8e5da0e MCP `tags` delta.
#![cfg(feature = "gateway")]
use bytes::Bytes;
use futures_util::{future::BoxFuture, stream};
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use scm_sdk::{
    Client, Endpoint, ErrorKind, OriginPolicy, RequestOptions, Result, Secret,
    airs::gateway::{models::*, *},
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    data::Patch,
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
const DATA: &str = "https://gateway-data.example/ai_gw/v2";
const ADMIN: &str = "https://gateway-admin.example/ai_gw/admin/v2";

struct Fixture {
    base: &'static str,
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    body: Option<Value>,
    reply: Value,
}

#[derive(Default)]
struct Wire {
    fixtures: Mutex<VecDeque<Fixture>>,
    calls: AtomicUsize,
}

impl Wire {
    fn expect(
        &self,
        base: &'static str,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<Value>,
        reply: Value,
    ) {
        self.fixtures.lock().unwrap().push_back(Fixture {
            base,
            method,
            path: path.into(),
            query: query
                .iter()
                .map(|(k, v)| ((*k).into(), (*v).into()))
                .collect(),
            body,
            reply,
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
                .expect("unexpected request");
            let mut url = request.url().clone();
            url.set_query(None);
            assert_eq!(url.as_str(), format!("{}{}", expected.base, expected.path));
            assert_eq!(request.method(), expected.method);
            let query: Vec<_> = request.url().query_pairs().into_owned().collect();
            assert_eq!(query, expected.query);
            assert_eq!(
                request.headers()["authorization"],
                "Bearer fixture-gateway-token"
            );
            assert_eq!(request.headers()["x-tsg-id"], "123456789");
            match expected.body {
                Some(body) => {
                    assert_eq!(request.headers()["content-type"], "application/json");
                    assert_eq!(
                        serde_json::from_slice::<Value>(request.body()).unwrap(),
                        body
                    );
                }
                None => assert!(request.body().is_empty()),
            }
            let mut headers = HeaderMap::new();
            headers.insert("content-type", HeaderValue::from_static("application/json"));
            Ok(TransportResponse::new(
                StatusCode::OK,
                headers,
                Box::pin(stream::iter([Ok(Bytes::from(
                    serde_json::to_vec(&expected.reply).unwrap(),
                ))])),
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

struct Clients {
    servers: AIGatewayMcpServersClient,
    integrations: AIGatewayMcpIntegrationsClient,
    wire: Arc<Wire>,
    token: Arc<Token>,
}

fn clients() -> Clients {
    let data = Endpoint::parse(DATA).unwrap();
    let admin = Endpoint::parse(ADMIN).unwrap();
    let wire = Arc::new(Wire::default());
    let token = Arc::new(Token {
        context: AuthContext::new(
            Endpoint::parse("https://issuer.example/token").unwrap(),
            "gateway-client",
            1,
            "123456789",
        )
        .unwrap(),
        calls: AtomicUsize::new(0),
    });
    let client = Client::builder(data.clone())
        .transport(wire.clone())
        .destination_policy(Arc::new(OriginPolicy::new([data, admin.clone()])))
        .authentication(Authentication::oauth(token.clone(), false))
        .build()
        .unwrap();
    Clients {
        servers: AIGatewayMcpServersClient::new(client.clone()).unwrap(),
        integrations: AIGatewayMcpIntegrationsClient::new(client.with_endpoint(admin).unwrap())
            .unwrap(),
        wire,
        token,
    }
}

fn body<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).unwrap()
}

fn integration_create() -> Value {
    json!({"name": "fixture", "organisation_id": "123", "slug": "fixture",
        "url": "https://never-follow.example/mcp", "auth_type": "none", "transport": "sse"})
}

fn with(mut value: Value, tags: Option<Value>) -> Value {
    if let Some(tags) = tags {
        value["tags"] = tags;
    }
    value
}

fn detail(tags: Value) -> Value {
    json!({"id": ID, "name": "fixture", "description": null, "owner_id": "owner", "status": "active",
        "created_at": "created", "last_updated_at": "updated", "configurations": {},
        "global_workspace_access": null, "workspace_id": null, "slug": "fixture",
        "url": "https://never-follow.example/mcp", "auth_type": "none", "transport": "sse",
        "type": "mcp", "secret_mappings": null, "object": "mcp", "tags": tags})
}

#[tokio::test]
async fn tags_write_bodies_preserve_omission_null_and_value_for_all_four_writes() {
    let c = clients();
    for tags in [
        None,
        Some(Value::Null),
        Some(json!({})),
        Some(json!({"env": "prod", "team_a-1": "sec"})),
    ] {
        let b = with(integration_create(), tags.clone());
        c.wire.expect(
            ADMIN,
            Method::POST,
            "/mcp-integrations",
            &[],
            Some(b.clone()),
            json!({}),
        );
        c.integrations.create(&body(b), None).await.unwrap();

        let b = with(json!({"description": null}), tags.clone());
        c.wire.expect(
            ADMIN,
            Method::PUT,
            &format!("/mcp-integrations/{ID}"),
            &[],
            Some(b.clone()),
            json!({}),
        );
        c.integrations.update(ID, &body(b), None).await.unwrap();

        let b = with(
            json!({"name": "fixture", "mcp_integration_id": "opaque"}),
            tags.clone(),
        );
        c.wire.expect(
            DATA,
            Method::POST,
            "/mcp-servers",
            &[],
            Some(b.clone()),
            json!({"id": ID}),
        );
        c.servers.create(&body(b), None).await.unwrap();

        let b = with(json!({}), tags.clone());
        c.wire.expect(
            DATA,
            Method::PUT,
            "/mcp-servers/opaque",
            &[],
            Some(b.clone()),
            json!({}),
        );
        c.servers.update("opaque", &body(b), None).await.unwrap();
    }
    // Typed construction: `Patch::Null` clears tags; a collected map replaces them.
    let mut update: GatewayMcpServersClientUpdateRequest = body(json!({}));
    update.tags = Patch::Null;
    c.wire.expect(
        DATA,
        Method::PUT,
        "/mcp-servers/opaque",
        &[],
        Some(json!({"tags": null})),
        json!({}),
    );
    c.servers.update("opaque", &update, None).await.unwrap();
    update.tags = Patch::Value([("env", "prod".to_owned())].into_iter().collect());
    c.wire.expect(
        DATA,
        Method::PUT,
        "/mcp-servers/opaque",
        &[],
        Some(json!({"tags": {"env": "prod"}})),
        json!({}),
    );
    c.servers.update("opaque", &update, None).await.unwrap();
    // A tags-only integration update is a valid nonempty replacement.
    c.wire.expect(
        ADMIN,
        Method::PUT,
        &format!("/mcp-integrations/{ID}"),
        &[],
        Some(json!({"tags": null})),
        json!({}),
    );
    c.integrations
        .update(ID, &body(json!({"tags": null})), None)
        .await
        .unwrap();
    c.wire.done();
}

#[tokio::test]
async fn invalid_tags_and_filters_are_rejected_before_authentication() {
    let c = clients();
    let mut rejected = 0;
    for tags in [
        json!({"bad key": "x"}),
        json!({"dot.key": "x"}),
        json!({"": "x"}),
        json!({"env": 1}),
        json!({"env": null}),
        json!({"env": ["a"]}),
        json!("env=prod"),
        json!(["env"]),
    ] {
        let create: Result<McpIntegrationCreateRequest> =
            serde_json::from_value(with(integration_create(), Some(tags.clone())))
                .map_err(|_| scm_sdk::Error::new(ErrorKind::Validation));
        if let Ok(create) = create {
            assert_eq!(
                c.integrations
                    .create(&create, None)
                    .await
                    .unwrap_err()
                    .kind(),
                ErrorKind::Validation
            );
            rejected += 1;
        }
        let server: Result<GatewayMcpServersClientUpdateRequest> =
            serde_json::from_value(json!({ "tags": tags }))
                .map_err(|_| scm_sdk::Error::new(ErrorKind::Validation));
        if let Ok(server) = server {
            assert_eq!(
                c.servers
                    .update("opaque", &server, None)
                    .await
                    .unwrap_err()
                    .kind(),
                ErrorKind::Validation
            );
            rejected += 1;
        }
    }
    assert!(
        rejected >= 8,
        "typed models must carry invalid wire values to validation"
    );
    let oversized = format!("{{\"k\":\"{}\"}}", "x".repeat(20 * 1024));
    for filter in [
        "{\"env\":1}",
        "[\"env\"]",
        "\"env\"",
        "not json",
        "{\"env\":\"a\",\"env\":\"b\"}",
        "{\"env\":null}",
        oversized.as_str(),
    ] {
        let query: GatewayMcpServersClientListOptions = body(json!({ "tags": filter }));
        assert_eq!(
            c.servers.list(&query, None).await.unwrap_err().kind(),
            ErrorKind::Validation
        );
        let query: GatewayMcpIntegrationsClientListOptions = body(json!({ "tags": filter }));
        assert_eq!(
            c.integrations
                .list_with_options(&query, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Validation
        );
    }
    for invalid in [
        json!({"page_size": 0}),
        json!({"page_size": 1001}),
        json!({"current_page": -1}),
        json!({"type": "global"}),
        json!({"organisation_id": "not-a-uuid"}),
    ] {
        let query: GatewayMcpIntegrationsClientListOptions = body(invalid);
        assert_eq!(
            c.integrations
                .list_with_options(&query, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Validation
        );
    }
    // A filter supplied through the open extensions map is validated identically.
    for filter in [json!("not json"), json!("{\"env\":1}"), json!(7)] {
        let query = GatewayMcpServersClientListOptions::builder()
            .extension("tags", filter.clone().into())
            .build()
            .unwrap();
        assert_eq!(
            c.servers.list(&query, None).await.unwrap_err().kind(),
            ErrorKind::Validation
        );
        let query = GatewayMcpIntegrationsClientListOptions::builder()
            .extension("tags", filter.into())
            .build()
            .unwrap();
        assert_eq!(
            c.integrations
                .list_with_options(&query, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Validation
        );
    }
    assert_eq!(
        mcp_tags_filter([("env", "a"), ("env", "b")])
            .unwrap_err()
            .kind(),
        ErrorKind::Validation
    );
    assert_eq!(c.wire.calls.load(Ordering::SeqCst), 0);
    assert_eq!(c.token.calls.load(Ordering::SeqCst), 0);
}

#[tokio::test]
async fn list_filters_send_exact_queries_and_frozen_list_is_unchanged() {
    let c = clients();
    let filter = mcp_tags_filter([("team", "sec"), ("env", "prod")]).unwrap();
    assert_eq!(filter, r#"{"env":"prod","team":"sec"}"#);
    let empty: Vec<(&str, &str)> = Vec::new();
    assert_eq!(mcp_tags_filter(empty).unwrap(), "{}");

    let query = json!({"workspace_id": "ws", "current_page": 0, "page_size": 100, "id": "opaque", "search": "", "tags": filter});
    c.wire.expect(
        DATA,
        Method::GET,
        "/mcp-servers",
        &[
            ("workspace_id", "ws"),
            ("current_page", "0"),
            ("page_size", "100"),
            ("id", "opaque"),
            ("search", ""),
            ("tags", r#"{"env":"prod","team":"sec"}"#),
        ],
        None,
        json!({"data": [], "total": 0}),
    );
    c.servers.list(&body(query), None).await.unwrap();
    // Without tags, the server list keeps its exact frozen query.
    c.wire.expect(
        DATA,
        Method::GET,
        "/mcp-servers",
        &[("page_size", "10")],
        None,
        json!({"data": [], "total": 0}),
    );
    c.servers
        .list(&body(json!({"page_size": 10})), None)
        .await
        .unwrap();

    let reply = json!({"object": "list", "total": 0, "data": []});
    // Frozen `list` sends no query at all.
    c.wire.expect(
        ADMIN,
        Method::GET,
        "/mcp-integrations",
        &[],
        None,
        reply.clone(),
    );
    c.integrations.list(None).await.unwrap();
    c.wire.expect(
        ADMIN,
        Method::GET,
        "/mcp-integrations",
        &[],
        None,
        reply.clone(),
    );
    c.integrations
        .list_with_options(&body(json!({})), None)
        .await
        .unwrap();
    let full = json!({"organisation_id": "8f8a6f1e-5a36-4c47-9f1a-5d2b3c4d5e6f", "type": "organisation", "workspace_id": "ws",
        "page_size": 1000, "current_page": 2, "search": "fixture", "tags": r#"{"env":"prod"}"#});
    c.wire.expect(
        ADMIN,
        Method::GET,
        "/mcp-integrations",
        &[
            ("organisation_id", "8f8a6f1e-5a36-4c47-9f1a-5d2b3c4d5e6f"),
            ("type", "organisation"),
            ("workspace_id", "ws"),
            ("page_size", "1000"),
            ("current_page", "2"),
            ("search", "fixture"),
            ("tags", r#"{"env":"prod"}"#),
        ],
        None,
        reply,
    );
    c.integrations
        .list_with_options(&body(full), None)
        .await
        .unwrap();
    c.wire.done();
}

#[tokio::test]
async fn responses_type_conforming_tags_and_preserve_nonconforming_values() {
    let c = clients();
    c.wire.expect(
        ADMIN,
        Method::GET,
        &format!("/mcp-integrations/{ID}"),
        &[],
        None,
        detail(json!({"env": "prod"})),
    );
    let typed = c.integrations.get(ID, None).await.unwrap();
    match &typed.tags {
        Patch::Value(tags) => assert_eq!(tags.get("env").map(String::as_str), Some("prod")),
        _ => panic!("expected typed tags"),
    }
    assert!(
        !format!("{typed:?}").contains("prod"),
        "ordinary formatting stays redacted"
    );

    let raw = json!({"id": "opaque", "tags": {"count": 3}});
    c.wire.expect(
        DATA,
        Method::GET,
        "/mcp-servers/opaque",
        &[],
        None,
        raw.clone(),
    );
    let server = c.servers.get("opaque", None).await.unwrap();
    assert!(matches!(server.tags, Patch::Unset));
    assert_eq!(
        serde_json::to_value(&server).unwrap(),
        raw,
        "nonconforming tags survive unchanged"
    );

    let item = json!({"id": "opaque", "tags": null});
    c.wire.expect(
        DATA,
        Method::GET,
        "/mcp-servers",
        &[],
        None,
        json!({"data": [item], "total": 1}),
    );
    let listed = c.servers.list(&body(json!({})), None).await.unwrap();
    assert_eq!(
        serde_json::to_value(&listed).unwrap()["data"][0]["tags"],
        Value::Null
    );
    c.wire.done();
}
