//! Independent wire and adversarial contracts for the separately pinned Portkey delta.
#![cfg(feature = "gateway")]
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, Limits, RemoteOutcome, RequestOptions, Result, Secret,
    airs::gateway::access_policies::*,
    auth::Authentication,
    data::{JsonValue, Optional, Patch},
    transport::{Transport, TransportRequest, TransportResponse},
};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
    time::Duration,
};
const ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const KEY: &str = "fixture-policy-secret";
struct Wire {
    requests: Mutex<Vec<TransportRequest>>,
    replies: Mutex<VecDeque<(u16, Value)>>,
    delay: Duration,
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.lock().unwrap().push(request);
            if !self.delay.is_zero() {
                tokio::time::sleep(self.delay).await;
            }
            let (status, value) = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected request/replay");
            if status == 0 {
                return Err(Error::new(ErrorKind::Transport));
            }
            Ok(TransportResponse::from_bytes(
                StatusCode::from_u16(status).unwrap(),
                HeaderMap::new(),
                Bytes::from(serde_json::to_vec(&value).unwrap()),
            ))
        })
    }
}
fn setup(
    replies: Vec<(u16, Value)>,
    limits: Limits,
    delay: Duration,
) -> (AccessPoliciesClient, Arc<Wire>) {
    let wire = Arc::new(Wire {
        requests: Mutex::new(vec![]),
        replies: Mutex::new(replies.into()),
        delay,
    });
    let client = Client::builder(Endpoint::parse("https://control.example/v1").unwrap())
        .authentication(Authentication::runtime_key(Secret::new(KEY).unwrap()).unwrap())
        .transport(wire.clone())
        .limits(limits)
        .build()
        .unwrap();
    (AccessPoliciesClient::new(client).unwrap(), wire)
}
fn body() -> CreateAccessPolicy {
    CreateAccessPolicy::new(
        "policy",
        vec![AccessPolicySubject::new(
            AccessPolicySubjectKind::User,
            "alice",
        )],
        vec![AccessPolicyTarget::provider("provider-slug")],
    )
}
fn update() -> UpdateAccessPolicy {
    let b = body();
    UpdateAccessPolicy::new(1, b.subjects, b.targets)
}
#[tokio::test]
async fn all_five_methods_have_literal_wire_contracts_and_faithful_json() {
    let detail = json!({"effect":"future-effect","description":null,"subjects":[{"type":"future-subject","name":null,"email":null,"future":{"secret":null}}],"targets":[{"type":"provider","provider_id":"provider-slug","extra":null},{"type":"future-target","future":{"x":null}}],"extra":null});
    let page = json!({"object":"future-list","total":1,"data":[{"description":null,"enabled":false,"effect":"unknown","extra":{"x":null}}]});
    let (client, wire) = setup(
        vec![
            (200, json!({"id":ID,"object":"access","extra":null})),
            (200, page.clone()),
            (200, detail.clone()),
            (200, json!({"future":null})),
            (200, json!({})),
        ],
        Limits::default(),
        Duration::ZERO,
    );
    let mut create = body();
    create.description = Patch::Null;
    create.enabled = Optional::Value(false);
    assert_eq!(
        serde_json::to_value(client.create(&create, None).await.unwrap()).unwrap(),
        json!({"id":ID,"object":"access","extra":null})
    );
    let query:AccessPolicyListOptions=serde_json::from_value(json!({"workspace_id":"workspace &/é","effect":"deny","enabled":false,"user_id":"u","agent_id":"a","server_id":"s","search":"term +/é","created_by":"owner","start_time":"2026-01-01T00:00:00Z","end_time":"2025-01-01T00:00:00+00:00","page_size":0,"current_page":0})).unwrap();
    assert_eq!(
        serde_json::to_value(client.list(&query, None).await.unwrap()).unwrap(),
        page
    );
    let opts = RequestOptions::with_timeout(Duration::from_secs(10))
        .unwrap()
        .without_response_defaults();
    assert_eq!(
        serde_json::to_value(client.get(ID, Some(&opts)).await.unwrap()).unwrap(),
        detail
    );
    assert_eq!(
        serde_json::to_value(client.update(ID, &update(), None).await.unwrap()).unwrap(),
        json!({"future":null})
    );
    assert_eq!(
        serde_json::to_value(client.delete(ID, None).await.unwrap()).unwrap(),
        json!({})
    );
    let requests = wire.requests.lock().unwrap();
    for (r, (verb, path)) in requests.iter().zip([
        ("POST", "/v1/policies/access"),
        ("GET", "/v1/policies/access"),
        (
            "GET",
            "/v1/policies/access/550e8400-e29b-41d4-a716-446655440000",
        ),
        (
            "PUT",
            "/v1/policies/access/550e8400-e29b-41d4-a716-446655440000",
        ),
        (
            "DELETE",
            "/v1/policies/access/550e8400-e29b-41d4-a716-446655440000",
        ),
    ]) {
        assert_eq!(r.method().as_str(), verb);
        assert_eq!(r.url().path(), path);
        assert_eq!(r.url().host_str(), Some("control.example"));
        assert_eq!(r.headers()["x-portkey-api-key"], KEY);
        assert!(r.headers()["x-portkey-api-key"].is_sensitive());
        for key in ["authorization", "x-tsg-id", "x-portkey-provider"] {
            assert!(!r.headers().contains_key(key));
        }
        assert!(!format!("{r:?}").contains(KEY));
    }
    assert_eq!(
        serde_json::from_slice::<Value>(requests[0].body()).unwrap(),
        json!({"name":"policy","subjects":[{"type":"user","id":"alice"}],"targets":[{"type":"provider","provider_id":"provider-slug"}],"description":null,"enabled":false})
    );
    assert_eq!(
        requests[1]
            .url()
            .query_pairs()
            .into_owned()
            .collect::<Vec<_>>(),
        vec![
            ("workspace_id", "workspace &/é"),
            ("effect", "deny"),
            ("enabled", "false"),
            ("user_id", "u"),
            ("agent_id", "a"),
            ("server_id", "s"),
            ("search", "term +/é"),
            ("created_by", "owner"),
            ("start_time", "2026-01-01T00:00:00Z"),
            ("end_time", "2025-01-01T00:00:00+00:00"),
            ("page_size", "0"),
            ("current_page", "0")
        ]
        .into_iter()
        .map(|(k, v)| (k.into(), v.into()))
        .collect::<Vec<_>>()
    );
    assert_eq!(
        serde_json::from_slice::<Value>(requests[3].body()).unwrap(),
        json!({"version":1,"subjects":[{"type":"user","id":"alice"}],"targets":[{"type":"provider","provider_id":"provider-slug"}]})
    );
    for i in [1, 2, 4] {
        assert!(requests[i].body().is_empty());
    }
}
#[tokio::test]
async fn invalid_requests_never_dispatch_and_errors_have_static_names() {
    let (client, wire) = setup(vec![], Limits::default(), Duration::ZERO);
    for mutate in [0, 1, 2, 3, 4, 5, 6, 7] {
        let mut b = body();
        match mutate {
            0 => b.name.clear(),
            1 => b.name = "é".repeat(65),
            2 => b.description = Patch::Value("x".repeat(257)),
            3 => b.subjects.clear(),
            4 => b.targets.clear(),
            5 => b.subjects[0].id = "ANY".into(),
            6 => {
                b.subjects = vec![AccessPolicySubject::new(
                    AccessPolicySubjectKind::Group,
                    "ANY",
                )]
            }
            _ => {
                b.targets = vec![AccessPolicyTarget::mcp_server(
                    "s",
                    AccessPolicySelection::Names(vec![]),
                )]
            }
        }
        let e = client.create(&b, None).await.unwrap_err();
        assert_eq!(e.kind(), ErrorKind::Validation);
        assert_eq!(e.operation(), Some("airs.gateway.access_policies.create"));
    }
    let mut b = body();
    b.extra
        .insert("effect".into(), JsonValue::new(json!("deny")));
    assert_eq!(
        client.create(&b, None).await.unwrap_err().kind(),
        ErrorKind::Validation
    );
    let mut u = update();
    u.version = 0;
    assert_eq!(
        client.update(ID, &u, None).await.unwrap_err().kind(),
        ErrorKind::Validation
    );
    for id in [
        "bad",
        "../other",
        "%2e%2e",
        "550e8400-e29b-41d4-a716-446655440000/extra",
    ] {
        assert_eq!(
            client.get(id, None).await.unwrap_err().kind(),
            ErrorKind::Validation
        );
    }
    for (field, value) in [
        ("user_id", json!("x".repeat(37))),
        ("agent_id", json!("x".repeat(37))),
        ("server_id", json!("x".repeat(101))),
        ("search", json!("x".repeat(65))),
        ("created_by", json!("x".repeat(256))),
        ("start_time", json!("yesterday")),
        ("end_time", json!("2026-99-99T00:00:00Z")),
    ] {
        let q: AccessPolicyListOptions = serde_json::from_value(json!({field:value})).unwrap();
        assert_eq!(
            client.list(&q, None).await.unwrap_err().kind(),
            ErrorKind::Validation
        );
    }
    assert!(wire.requests.lock().unwrap().is_empty());
}
#[tokio::test]
async fn inference_defaults_and_update_omission_are_server_owned() {
    let (client, wire) = setup(vec![(200, json!({})); 4], Limits::default(), Duration::ZERO);
    let mut b = body();
    b.name = "é".repeat(64);
    b.description = Patch::Value("é".repeat(256));
    b.subjects = vec![
        AccessPolicySubject::new(AccessPolicySubjectKind::User, "ANY"),
        AccessPolicySubject::new(AccessPolicySubjectKind::Agent, "ANY"),
    ];
    client.create(&b, None).await.unwrap();
    b.effect = Optional::Value(AccessPolicyEffect::Allow);
    assert!(client.create(&b, None).await.is_err());
    let mut u = update();
    u.subjects = vec![AccessPolicySubject::new(
        AccessPolicySubjectKind::User,
        "ANY",
    )];
    client.update(ID, &u, None).await.unwrap();
    u.effect = Optional::Value(AccessPolicyEffect::Allow);
    assert!(client.update(ID, &u, None).await.is_err());
    u.effect = Optional::Value(AccessPolicyEffect::Deny);
    client.update(ID, &u, None).await.unwrap();
    client
        .list(&AccessPolicyListOptions::default(), None)
        .await
        .unwrap();
    let r = wire.requests.lock().unwrap();
    let c: Value = serde_json::from_slice(r[0].body()).unwrap();
    assert!(c.get("effect").is_none());
    assert!(c.get("enabled").is_none());
    let u: Value = serde_json::from_slice(r[1].body()).unwrap();
    assert!(u.get("effect").is_none());
    assert!(u.get("name").is_none());
    assert!(u.get("description").is_none());
    assert!(r[3].url().query().is_none());
}
#[tokio::test]
async fn mutation_conflicts_failures_and_response_bounds_do_not_replay_or_leak() {
    for status in [409, 500, 0] {
        let (client, wire) = setup(
            vec![(status, json!({"message":KEY}))],
            Limits::default(),
            Duration::ZERO,
        );
        let error = client
            .update(
                ID,
                &update(),
                Some(
                    &RequestOptions::with_timeout(Duration::from_secs(10))
                        .unwrap()
                        .with_retries(5)
                        .unwrap(),
                ),
            )
            .await
            .unwrap_err();
        assert_eq!(wire.requests.lock().unwrap().len(), 1);
        assert!(!format!("{error:?} {error}").contains(KEY));
        if status == 409 {
            assert_eq!(error.status(), Some(409));
        }
        if status == 0 {
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        }
    }
    let mut limits = Limits::default();
    limits.max_response_bytes = 64;
    let (client, _) = setup(
        vec![(200, json!({"extra":"x".repeat(100)}))],
        limits,
        Duration::ZERO,
    );
    assert_eq!(
        client.get(ID, None).await.unwrap_err().kind(),
        ErrorKind::Limit
    );
    let (client, _) = setup(
        vec![(200, json!({"enabled":null}))],
        Limits::default(),
        Duration::ZERO,
    );
    assert_eq!(
        client.get(ID, None).await.unwrap_err().kind(),
        ErrorKind::Decode
    );
}
#[tokio::test]
async fn cancellation_deadlines_and_request_bounds_remain_enforced() {
    let (client, wire) = setup(vec![], Limits::default(), Duration::ZERO);
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    options.cancel();
    assert_eq!(
        client
            .create(&body(), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert!(wire.requests.lock().unwrap().is_empty());
    let (client, _) = setup(
        vec![(200, json!({}))],
        Limits::default(),
        Duration::from_secs(1),
    );
    let options = RequestOptions::with_timeout(Duration::from_millis(10)).unwrap();
    assert_eq!(
        client.get(ID, Some(&options)).await.unwrap_err().kind(),
        ErrorKind::Deadline
    );
    let mut limits = Limits::default();
    limits.max_request_bytes = 64;
    let (client, wire) = setup(vec![], limits, Duration::ZERO);
    assert_eq!(
        client.create(&body(), None).await.unwrap_err().kind(),
        ErrorKind::Limit
    );
    assert!(wire.requests.lock().unwrap().is_empty());
}
#[test]
fn response_presence_unknown_vocabulary_and_redaction_are_explicit() {
    for value in [
        json!({}),
        json!({"description":null,"created_by":null,"subjects":[{"type":"unknown","name":null}]}),
        json!({"effect":"deny-future","targets":[{"type":"unknown","payload":[null]}]}),
    ] {
        let model: AccessPolicy = serde_json::from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(&model).unwrap(), value);
        assert!(format!("{model:?}").contains("REDACTED"));
    }
    for value in [
        json!({"targets":[{"type":"provider"}]}),
        json!({"targets":[{"type":"mcp_server","server_id":"s"}]}),
        json!({"subjects":[{"type":3}]}),
        json!({"enabled":null}),
        json!({"version":1.5}),
    ] {
        assert!(serde_json::from_value::<AccessPolicy>(value).is_err());
    }
    assert!(!format!("{:?}", body()).contains("alice"));
    assert!(serde_json::from_value::<AccessPolicyListOptions>(json!({"page_size":-1})).is_err());
}

#[tokio::test]
async fn independent_openapi_fixture_corpus_matches_native_behavior() {
    let cases: Value = serde_json::from_str(include_str!(
        "../../../contracts/vendor/portkey-access/a27feb9/model-fixtures.json"
    ))
    .unwrap();
    for case in cases["cases"].as_array().unwrap() {
        let bytes = serde_json::to_vec(&case["value"]).unwrap();
        let (client, wire) = setup(vec![(200, json!({}))], Limits::default(), Duration::ZERO);
        let accepted = match case["schema"].as_str().unwrap() {
            "CreateAccessPolicyRequest" => {
                match serde_json::from_slice::<CreateAccessPolicy>(&bytes) {
                    Ok(body) => client.create(&body, None).await.is_ok(),
                    Err(_) => false,
                }
            }
            "UpdateAccessPolicyRequest" => {
                match serde_json::from_slice::<UpdateAccessPolicy>(&bytes) {
                    Ok(body) => client.update(ID, &body, None).await.is_ok(),
                    Err(_) => false,
                }
            }
            "AccessPolicy" => serde_json::from_slice::<AccessPolicy>(&bytes).is_ok(),
            _ => panic!("unbound fixture"),
        };
        assert_eq!(
            accepted,
            case["sdkValid"].as_bool().unwrap(),
            "{}",
            case["name"]
        );
        if !accepted {
            assert!(wire.requests.lock().unwrap().is_empty());
        }
    }
}

#[tokio::test]
async fn wrong_credential_planes_and_denied_origins_never_dispatch() {
    use scm_sdk::auth::{AuthContext, TokenLease, TokenProvider};
    struct NeverToken(AuthContext);
    impl TokenProvider for NeverToken {
        fn context(&self) -> &AuthContext {
            &self.0
        }
        fn invalidate(&self, _: u64) -> bool {
            false
        }
        fn token<'a>(&'a self, _: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
            Box::pin(async { panic!("must not acquire OAuth for Portkey") })
        }
    }
    let (_, wire) = setup(vec![], Limits::default(), Duration::ZERO);
    let oauth = Authentication::oauth(
        Arc::new(NeverToken(
            AuthContext::new(
                Endpoint::parse("https://issuer.example").unwrap(),
                "fixture",
                0,
                "1234",
            )
            .unwrap(),
        )),
        true,
    );
    for auth in [
        Authentication::None,
        Authentication::scan(Some(Secret::new(KEY).unwrap()), None).unwrap(),
        oauth,
    ] {
        let client = Client::builder(Endpoint::parse("https://control.example/v1").unwrap())
            .transport(wire.clone())
            .authentication(auth)
            .build()
            .unwrap();
        assert_eq!(
            AccessPoliciesClient::new(client).unwrap_err().kind(),
            ErrorKind::Configuration
        );
    }
    let client = Client::builder(Endpoint::parse("https://control.example/v1").unwrap())
        .transport(wire.clone())
        .authentication(Authentication::runtime_key(Secret::new(KEY).unwrap()).unwrap())
        .destination_policy(Arc::new(scm_sdk::OriginPolicy::new([])))
        .build();
    match client {
        Err(error) => assert_eq!(error.kind(), ErrorKind::Destination),
        Ok(client) => assert_eq!(
            AccessPoliciesClient::new(client)
                .unwrap()
                .get(ID, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Destination
        ),
    }
    assert!(wire.requests.lock().unwrap().is_empty());
}
