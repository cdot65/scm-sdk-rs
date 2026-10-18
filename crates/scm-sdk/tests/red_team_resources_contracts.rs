//! Independent wire expectations transcribed from the frozen Red Team resources.
#![cfg(feature = "red-team")]
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, Limits, RemoteOutcome, RequestOptions, Result, Secret,
    airs::red_team::{
        AdapterListOptions, AdapterOperationOptions, AdapterStatusFilter, ChannelListOptions,
        PromptListOptions, PromptSetListOptions, RedTeamAdaptersClient, RedTeamCustomAttacksClient,
        RedTeamEulaClient, RedTeamNetworkBrokerClient, RedTeamTargetsClient, TargetListOptions,
        TargetOperationOptions, models::*,
    },
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    pagination::{CollectOptions, Completion, Truncation},
    transport::{Transport, TransportRequest, TransportResponse},
};
use serde::{Serialize, de::DeserializeOwned};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
macro_rules! configured {
    ($ty:ident { $($field:ident: $value:expr),* $(,)? }) => {{
        let mut configured = $ty::default();
        $(configured.$field = $value;)*
        configured
    }};
}
const ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const OTHER_ID: &str = "660e8400-e29b-41d4-a716-446655440000";
const TOKEN: &str = "synthetic-red-team-bearer-secret";
struct Provider {
    context: AuthContext,
    calls: AtomicUsize,
    deadlines: Mutex<Vec<tokio::time::Instant>>,
}
impl TokenProvider for Provider {
    fn token<'a>(&'a self, options: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.deadlines.lock().unwrap().push(options.deadline());
            TokenLease::new(Secret::new(TOKEN).unwrap(), 1, Duration::from_secs(3600))
        })
    }
    fn context(&self) -> &AuthContext {
        &self.context
    }
    fn invalidate(&self, _: u64) -> bool {
        false
    }
}
enum Reply {
    Json(Value),
    Empty,
    Raw(Vec<u8>),
    Failure,
}
struct Wire {
    requests: Mutex<Vec<TransportRequest>>,
    replies: Mutex<VecDeque<Reply>>,
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.lock().unwrap().push(request);
            match self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected resource dispatch")
            {
                Reply::Json(value) => Ok(TransportResponse::from_bytes(
                    StatusCode::OK,
                    HeaderMap::new(),
                    Bytes::from(serde_json::to_vec(&value).unwrap()),
                )),
                Reply::Empty => Ok(TransportResponse::from_bytes(
                    StatusCode::NO_CONTENT,
                    HeaderMap::new(),
                    Bytes::new(),
                )),
                Reply::Raw(value) => Ok(TransportResponse::from_bytes(
                    StatusCode::OK,
                    HeaderMap::new(),
                    Bytes::from(value),
                )),
                Reply::Failure => Err(Error::new(ErrorKind::Transport)),
            }
        })
    }
}
struct Resources {
    attacks: RedTeamCustomAttacksClient,
    targets: RedTeamTargetsClient,
    adapters: RedTeamAdaptersClient,
    broker: RedTeamNetworkBrokerClient,
    eula: RedTeamEulaClient,
}
fn setup(replies: Vec<Reply>) -> (Resources, Arc<Wire>, Arc<Provider>) {
    setup_limits(replies, Limits::default())
}
fn setup_limits(replies: Vec<Reply>, limits: Limits) -> (Resources, Arc<Wire>, Arc<Provider>) {
    let wire = Arc::new(Wire {
        requests: Mutex::new(Vec::new()),
        replies: Mutex::new(replies.into()),
    });
    let provider = Arc::new(Provider {
        context: AuthContext::new(
            Endpoint::parse("https://issuer.example/token").unwrap(),
            "red-team-client",
            1,
            "12345",
        )
        .unwrap(),
        calls: AtomicUsize::new(0),
        deadlines: Mutex::new(Vec::new()),
    });
    // Resource constructors must disable this caller-selected tenant header.
    let client = Client::builder(Endpoint::parse("https://red-team.example.test").unwrap())
        .limits(limits)
        .transport(wire.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .build()
        .unwrap();
    let resources = Resources {
        attacks: RedTeamCustomAttacksClient::new(client.clone()).unwrap(),
        targets: RedTeamTargetsClient::new(client.clone()).unwrap(),
        adapters: RedTeamAdaptersClient::new(client.clone()).unwrap(),
        broker: RedTeamNetworkBrokerClient::new(client.clone()).unwrap(),
        eula: RedTeamEulaClient::new(client).unwrap(),
    };
    (resources, wire, provider)
}
fn model<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).unwrap()
}
fn same<T: Serialize>(actual: T, expected: &Value) {
    assert_eq!(serde_json::to_value(actual).unwrap(), *expected)
}
fn check(
    request: &TransportRequest,
    method: &str,
    path: &str,
    query: &[(&str, &str)],
    body: Option<&Value>,
) {
    assert_eq!(request.method().as_str(), method);
    assert_eq!(request.url().host_str(), Some("red-team.example.test"));
    assert_eq!(request.url().path(), path);
    assert_eq!(
        request
            .url()
            .query_pairs()
            .map(|(k, v)| (k.into_owned(), v.into_owned()))
            .collect::<Vec<_>>(),
        query
            .iter()
            .map(|(k, v)| (k.to_string(), v.to_string()))
            .collect::<Vec<_>>()
    );
    let mut names = request
        .headers()
        .keys()
        .map(|key| key.as_str())
        .collect::<Vec<_>>();
    names.sort_unstable();
    let expected = if body.is_some() {
        vec![
            "authorization",
            "content-type",
            "service-name",
            "user-agent",
        ]
    } else {
        vec!["authorization", "service-name", "user-agent"]
    };
    assert_eq!(names, expected);
    for name in expected {
        assert_eq!(request.headers().get_all(name).iter().count(), 1)
    }
    assert!(request.headers().values().all(|value| value.is_sensitive()));
    assert_eq!(
        request.headers()["authorization"],
        format!("Bearer {TOKEN}")
    );
    assert_eq!(request.headers()["service-name"], "api");
    assert_eq!(
        request.headers()["user-agent"],
        concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
    );
    if let Some(expected) = body {
        assert_eq!(request.headers()["content-type"], "application/json");
        assert_eq!(
            serde_json::from_slice::<Value>(request.body()).unwrap(),
            *expected
        );
        assert_eq!(
            request.body().as_ref(),
            serde_json::to_vec(expected).unwrap()
        );
    } else {
        assert!(request.body().is_empty())
    }
    assert!(!format!("{request:?} {:?}", request.headers()).contains(TOKEN));
}
fn set() -> Value {
    json!({"uuid":ID,"name":"synthetic-set","active":true,"archive":false,"status":"FUTURE","created_at":"2026-01-01","updated_at":"2026-01-02","future":{"secret":"response-secret"}})
}
fn reference() -> Value {
    json!({"uuid":ID,"name":"synthetic-set","status":"FUTURE","active":true,"tsg_id":"12345","created_at":"2026-01-01","updated_at":"2026-01-02","version":null,"future":"response-secret"})
}
fn prompt() -> Value {
    json!({"uuid":OTHER_ID,"prompt":"synthetic-prompt","user_defined_goal":true,"status":"FUTURE","active":false,"prompt_set_id":ID,"created_at":"2026-01-01","updated_at":"2026-01-02","future":"response-secret"})
}
fn target() -> Value {
    json!({"uuid":ID,"tsg_id":"12345","name":"synthetic-target","status":"FUTURE","active":false,"validated":false,"created_at":"2026-01-01","updated_at":"2026-01-02","future":"response-secret"})
}
fn adapter() -> Value {
    json!({"uuid":ID,"tsg_id":"12345","name":"synthetic-adapter","script_b64":"cGFzcw==","status":"FUTURE","variables":[{"key":"secret","value":null,"type":"SECRET","is_redacted":true}],"future":"response-secret"})
}
fn adapter_row() -> Value {
    json!({"uuid":ID,"name":"synthetic-adapter","status":"FUTURE","created_at":"2026-01-01","updated_at":"2026-01-02","future":"response-secret"})
}
fn base() -> Value {
    json!({"message":"ok","status":201,"future":"response-secret"})
}
fn list(items: Vec<Value>, total: Option<Value>) -> Value {
    let mut page = json!({"pagination":{},"data":items,"future":"page-secret"});
    if let Some(total) = total {
        page["pagination"]["total_items"] = total;
    }
    page
}
fn adapter_body() -> Value {
    json!({"name":"synthetic-adapter","script_b64":"cGFzcw==","prompt":"synthetic-prompt","variables":[{"key":"stored-secret","type":"SECRET","value":null}],"network_broker_channel_uuid":ID})
}
fn validate_adapter_body() -> Value {
    json!({"script_b64":"cGFzcw==","prompt":"synthetic-prompt","network_broker_channel_uuid":ID,"adapter_uuid":null,"variables":[{"key":"stored-secret","type":"SECRET","value":null}]})
}
fn copilot_auth() -> Value {
    json!({"client_id":"synthetic-copilot","client_secret":"copilot-secret","tenant_id":"tenant-synthetic","target_uuid":null,"token_json_uuid":ID})
}
fn copilot_token() -> Value {
    json!({"auth_response":{"code":"callback-secret","state":"state-secret"},"token_json_uuid":ID,"client_id":"synthetic-copilot","client_secret":"copilot-secret","tenant_id":"tenant-synthetic"})
}

#[tokio::test]
async fn prompt_set_methods_preserve_routes_queries_and_unknown_responses() {
    let version = json!({"uuid":ID,"status":"FUTURE","is_latest":false,"version":null,"future":"response-secret"});
    let active = json!({"data":[reference()],"future":"response-secret"});
    let page = list(vec![set(), set()], Some(json!(2)));
    let (r, wire, _) = setup(vec![
        Reply::Json(set()),
        Reply::Json(page.clone()),
        Reply::Json(set()),
        Reply::Json(set()),
        Reply::Json(set()),
        Reply::Json(reference()),
        Reply::Json(version.clone()),
        Reply::Json(version.clone()),
        Reply::Json(active.clone()),
    ]);
    let create = json!({"name":"synthetic-set","language":"en","property_names":["severity"],"future_request":"extension"});
    same(
        r.attacks
            .create_prompt_set(&model(create.clone()), None)
            .await
            .unwrap(),
        &set(),
    );
    let params = configured!(PromptSetListOptions {
        skip: Some(3),
        limit: Some(7),
        search: Some("query & space".into()),
        language: Some("en".into()),
        status: Some("READY".into()),
        active: Some(false),
        archive: Some(true),
    });
    same(
        r.attacks.list_prompt_sets(&params, None).await.unwrap(),
        &page,
    );
    same(r.attacks.get_prompt_set(ID, None).await.unwrap(), &set());
    let update = json!({"name":null,"description":"updated","archive":false});
    same(
        r.attacks
            .update_prompt_set(ID, &model(update.clone()), None)
            .await
            .unwrap(),
        &set(),
    );
    let archive = json!({"archive":true});
    same(
        r.attacks
            .archive_prompt_set(ID, &model(archive.clone()), None)
            .await
            .unwrap(),
        &set(),
    );
    same(
        r.attacks.get_prompt_set_reference(ID, None).await.unwrap(),
        &reference(),
    );
    same(
        r.attacks
            .get_prompt_set_version_info(ID, Some("gen /%literal"), None)
            .await
            .unwrap(),
        &version,
    );
    same(
        r.attacks
            .get_prompt_set_version_info(ID, None, None)
            .await
            .unwrap(),
        &version,
    );
    let value = r.attacks.list_active_prompt_sets(None).await.unwrap();
    assert!(!format!("{value:?}").contains("response-secret"));
    same(value, &active);
    let q = wire.requests.lock().unwrap();
    let path = format!("/v1/custom-attack/custom-prompt-set/{ID}");
    check(
        &q[0],
        "POST",
        "/v1/custom-attack/custom-prompt-set",
        &[],
        Some(&create),
    );
    check(
        &q[1],
        "GET",
        "/v1/custom-attack/list-custom-prompt-sets",
        &[
            ("skip", "3"),
            ("limit", "7"),
            ("search", "query & space"),
            ("language", "en"),
            ("status", "READY"),
            ("active", "false"),
            ("archive", "true"),
        ],
        None,
    );
    check(&q[2], "GET", &path, &[], None);
    check(&q[3], "PUT", &path, &[], Some(&update));
    check(
        &q[4],
        "PUT",
        &format!("{path}/archive"),
        &[],
        Some(&archive),
    );
    check(&q[5], "GET", &format!("{path}/reference"), &[], None);
    check(
        &q[6],
        "GET",
        &format!("{path}/version-info"),
        &[("version", "gen /%literal")],
        None,
    );
    check(&q[7], "GET", &format!("{path}/version-info"), &[], None);
    check(
        &q[8],
        "GET",
        "/v1/custom-attack/active-custom-prompt-sets",
        &[],
        None,
    );
}

#[tokio::test]
async fn prompt_and_property_methods_preserve_path_encoding_repeated_queries_and_empty_success() {
    let page = list(vec![prompt()], Some(json!(1)));
    let names = json!({"data":["severity","category"],"future":"response-secret"});
    let values =
        json!({"name":"severity /%literal","values":["low","high"],"future":"response-secret"});
    let multiple =
        json!({"data":{"severity":["low"],"category":["PII"]},"future":"response-secret"});
    let (r, wire, _) = setup(vec![
        Reply::Json(prompt()),
        Reply::Json(page.clone()),
        Reply::Json(prompt()),
        Reply::Json(prompt()),
        Reply::Empty,
        Reply::Json(base()),
        Reply::Json(names.clone()),
        Reply::Json(names.clone()),
        Reply::Json(base()),
        Reply::Empty,
        Reply::Json(values.clone()),
        Reply::Json(multiple.clone()),
        Reply::Json(json!({})),
        Reply::Json(base()),
    ]);
    let create = json!({"prompt":"synthetic-prompt","prompt_set_id":ID,"goal":null,"properties":{"severity":"high"}});
    same(
        r.attacks
            .create_prompt(&model(create.clone()), None)
            .await
            .unwrap(),
        &prompt(),
    );
    let params = configured!(PromptListOptions {
        skip: Some(0),
        limit: Some(5),
        search: Some("synthetic".into()),
        status: Some("READY".into()),
        active: Some(false),
    });
    same(
        r.attacks.list_prompts(ID, &params, None).await.unwrap(),
        &page,
    );
    same(
        r.attacks.get_prompt(ID, OTHER_ID, None).await.unwrap(),
        &prompt(),
    );
    let update = json!({"prompt":null,"goal":null,"properties":null});
    same(
        r.attacks
            .update_prompt(ID, OTHER_ID, &model(update.clone()), None)
            .await
            .unwrap(),
        &prompt(),
    );
    assert!(
        r.attacks
            .delete_prompt(ID, OTHER_ID, None)
            .await
            .unwrap()
            .is_none()
    );
    same(
        r.attacks
            .delete_prompt(ID, OTHER_ID, None)
            .await
            .unwrap()
            .unwrap(),
        &base(),
    );
    same(r.attacks.get_property_names(None).await.unwrap(), &names);
    let name = json!({"name":"severity"});
    same(
        r.attacks
            .create_property_name(&model(name.clone()), None)
            .await
            .unwrap()
            .unwrap(),
        &names,
    );
    same(
        r.attacks
            .create_property_name(&model(name.clone()), None)
            .await
            .unwrap()
            .unwrap(),
        &base(),
    );
    assert!(
        r.attacks
            .create_property_name(&model(name.clone()), None)
            .await
            .unwrap()
            .is_none()
    );
    same(
        r.attacks
            .get_property_values("severity /%literal", None)
            .await
            .unwrap(),
        &values,
    );
    same(
        r.attacks
            .get_property_values_multiple(
                &["severity".into(), "category".into(), "severity".into()],
                None,
            )
            .await
            .unwrap(),
        &multiple,
    );
    same(
        r.attacks
            .get_property_values_multiple(&[], None)
            .await
            .unwrap(),
        &json!({}),
    );
    let value = json!({"property_name":"severity","property_value":"critical"});
    same(
        r.attacks
            .create_property_value(&model(value.clone()), None)
            .await
            .unwrap(),
        &base(),
    );
    let q = wire.requests.lock().unwrap();
    let path = format!("/v1/custom-attack/custom-prompt-set/{ID}/custom-prompt/{OTHER_ID}");
    check(
        &q[0],
        "POST",
        "/v1/custom-attack/custom-prompt-set/custom-prompt",
        &[],
        Some(&create),
    );
    check(
        &q[1],
        "GET",
        &format!("/v1/custom-attack/custom-prompt-set/{ID}/list-custom-prompts"),
        &[
            ("skip", "0"),
            ("limit", "5"),
            ("search", "synthetic"),
            ("status", "READY"),
            ("active", "false"),
        ],
        None,
    );
    check(&q[2], "GET", &path, &[], None);
    check(&q[3], "PUT", &path, &[], Some(&update));
    check(&q[4], "DELETE", &path, &[], None);
    check(&q[5], "DELETE", &path, &[], None);
    check(&q[6], "GET", "/v1/custom-attack/property-names", &[], None);
    for request in &q[7..10] {
        check(
            request,
            "POST",
            "/v1/custom-attack/property-names",
            &[],
            Some(&name),
        );
    }
    check(
        &q[10],
        "GET",
        "/v1/custom-attack/property-values/severity%20%2F%25literal",
        &[],
        None,
    );
    check(
        &q[11],
        "GET",
        "/v1/custom-attack/property-values",
        &[
            ("property_names", "severity"),
            ("property_names", "category"),
            ("property_names", "severity"),
        ],
        None,
    );
    check(
        &q[12],
        "GET",
        "/v1/custom-attack/property-values",
        &[],
        None,
    );
    check(
        &q[13],
        "POST",
        "/v1/custom-attack/property-values",
        &[],
        Some(&value),
    );
}

#[tokio::test]
async fn targets_crud_probe_profile_and_metadata_match_frozen_contracts() {
    let profile = json!({"target_id":ID,"target_version":2,"status":"FUTURE","other_details":null,"future":"response-secret"});
    let page = list(vec![target(), target()], Some(json!(2)));
    let metadata = json!({"future_field":{"type":"secret-schema"}});
    let templates = json!({"OPENAI":{},"HUGGING_FACE":{},"DATABRICKS":{},"BEDROCK":{},"REST":{},"STREAMING":{},"WEBSOCKET":{},"FUTURE_PROVIDER":{"secret":"response-secret"}});
    let (r, wire, _) = setup(vec![
        Reply::Json(target()),
        Reply::Json(target()),
        Reply::Json(page.clone()),
        Reply::Json(target()),
        Reply::Json(target()),
        Reply::Json(target()),
        Reply::Empty,
        Reply::Json(base()),
        Reply::Json(target()),
        Reply::Json(profile.clone()),
        Reply::Json(target()),
        Reply::Json(json!({"validated":false,"token_preview":null,"future":"response-secret"})),
        Reply::Json(metadata.clone()),
        Reply::Json(templates.clone()),
    ]);
    let body = json!({"name":"synthetic-target","description":null,"target_type":"APPLICATION"});
    let operation = configured!(TargetOperationOptions {
        validate: Some(false),
    });
    same(
        r.targets
            .create(&model(body.clone()), None, None)
            .await
            .unwrap(),
        &target(),
    );
    same(
        r.targets
            .create(&model(body.clone()), Some(&operation), None)
            .await
            .unwrap(),
        &target(),
    );
    let params = configured!(TargetListOptions {
        skip: Some(1),
        limit: Some(4),
        search: Some("target".into()),
        adapter_uuid: Some(OTHER_ID.into()),
        target_type: Some("APPLICATION".into()),
        status: Some("READY".into()),
    });
    same(r.targets.list(&params, None).await.unwrap(), &page);
    same(r.targets.get(ID, None).await.unwrap(), &target());
    same(
        r.targets
            .update(ID, &model(body.clone()), None, None)
            .await
            .unwrap(),
        &target(),
    );
    same(
        r.targets
            .update(
                ID,
                &model(body.clone()),
                Some(&configured!(TargetOperationOptions {
                    validate: Some(true),
                })),
                None,
            )
            .await
            .unwrap(),
        &target(),
    );
    assert!(r.targets.delete(ID, None).await.unwrap().is_none());
    same(r.targets.delete(ID, None).await.unwrap().unwrap(), &base());
    let probe = json!({"name":"synthetic-target","uuid":ID,"probe_fields":["multi_turn"]});
    same(
        r.targets.probe(&model(probe.clone()), None).await.unwrap(),
        &target(),
    );
    same(r.targets.get_profile(ID, None).await.unwrap(), &profile);
    let update = json!({"target_background":{"industry":"synthetic"},"additional_context":null});
    same(
        r.targets
            .update_profile(ID, &model(update.clone()), None)
            .await
            .unwrap(),
        &target(),
    );
    let auth =
        json!({"auth_type":"HEADERS","auth_config":{"Authorization":"synthetic-target-secret"}});
    let result = r
        .targets
        .validate_auth(&model(auth.clone()), None)
        .await
        .unwrap();
    assert!(!result.validated);
    let result = r.targets.get_target_metadata(None).await.unwrap();
    assert!(!format!("{result:?}").contains("secret-schema"));
    same(result, &metadata);
    same(
        r.targets.get_target_templates(None).await.unwrap(),
        &templates,
    );
    let q = wire.requests.lock().unwrap();
    let path = format!("/v1/target/{ID}");
    check(&q[0], "POST", "/v1/target", &[], Some(&body));
    check(
        &q[1],
        "POST",
        "/v1/target",
        &[("validate", "false")],
        Some(&body),
    );
    check(
        &q[2],
        "GET",
        "/v1/target",
        &[
            ("skip", "1"),
            ("limit", "4"),
            ("search", "target"),
            ("adapter_uuid", OTHER_ID),
            ("target_type", "APPLICATION"),
            ("status", "READY"),
        ],
        None,
    );
    check(&q[3], "GET", &path, &[], None);
    check(&q[4], "PUT", &path, &[], Some(&body));
    check(&q[5], "PUT", &path, &[("validate", "true")], Some(&body));
    check(&q[6], "DELETE", &path, &[], None);
    check(&q[7], "DELETE", &path, &[], None);
    check(&q[8], "POST", "/v1/target/probe", &[], Some(&probe));
    check(&q[9], "GET", &format!("{path}/profile"), &[], None);
    check(
        &q[10],
        "PUT",
        &format!("{path}/profile"),
        &[],
        Some(&update),
    );
    check(&q[11], "POST", "/v1/target/validate-auth", &[], Some(&auth));
    check(&q[12], "GET", "/v1/template/target-metadata", &[], None);
    check(&q[13], "GET", "/v1/template/target-templates", &[], None);
}

#[tokio::test]
async fn target_profiling_and_copilot_credentials_use_exact_bodies_and_safe_formatting() {
    let auth_result = json!({"auth_url":"https://login.example/authorize","token_json_uuid":ID,"state":"state-secret","future":"response-secret"});
    let token_result = json!({"token_json_uuid":ID,"future":"response-secret"});
    let (r, wire, _) = setup(vec![
        Reply::Json(json!({"message":"profiling started"})),
        Reply::Json(auth_result.clone()),
        Reply::Json(token_result.clone()),
        Reply::Empty,
    ]);
    r.targets.start_profiling(ID, None).await.unwrap();
    let auth: MSCopilotStudioAuthUrlRequest = model(copilot_auth());
    let token: MSCopilotStudioTokenRequest = model(copilot_token());
    assert!(!format!("{auth:?} {token:?} {:?}", r.targets).contains("copilot-secret"));
    same(
        r.targets.get_copilot_auth_url(&auth, None).await.unwrap(),
        &auth_result,
    );
    same(
        r.targets
            .exchange_copilot_token(&token, None)
            .await
            .unwrap(),
        &token_result,
    );
    r.targets.delete_copilot_token(ID, None).await.unwrap();
    let q = wire.requests.lock().unwrap();
    check(
        &q[0],
        "POST",
        &format!("/v1/target/{ID}/profile"),
        &[],
        None,
    );
    check(
        &q[1],
        "POST",
        "/v1/target/ms-copilot-studio/auth-url",
        &[],
        Some(&copilot_auth()),
    );
    check(
        &q[2],
        "POST",
        "/v1/target/ms-copilot-studio/token",
        &[],
        Some(&copilot_token()),
    );
    check(
        &q[3],
        "DELETE",
        &format!("/v1/target/ms-copilot-studio/token/{ID}"),
        &[],
        None,
    );
    assert!(!format!("{:?}", *q).contains("copilot-secret"));
    assert!(!format!("{:?}", *q).contains("callback-secret"));
}

#[tokio::test]
async fn adapter_methods_pin_validate_default_replacement_null_secrets_and_empty_delete() {
    let config = json!({"default_script_b64":"cGFzcw==","default_test_prompt":"synthetic","future":"response-secret"});
    let page = list(vec![adapter_row(), adapter_row()], Some(json!(2)));
    let outcome = json!({"validated":false,"stdout":null,"stderr":"validation-secret","traceback":null,"future":"response-secret"});
    let (r, wire, _) = setup(vec![
        Reply::Json(config.clone()),
        Reply::Json(adapter()),
        Reply::Json(adapter()),
        Reply::Json(page.clone()),
        Reply::Json(adapter()),
        Reply::Json(adapter()),
        Reply::Json(adapter()),
        Reply::Empty,
        Reply::Json(base()),
        Reply::Json(outcome.clone()),
    ]);
    same(r.adapters.get_config(None).await.unwrap(), &config);
    let no = configured!(AdapterOperationOptions {
        validate: Some(false),
    });
    let body = adapter_body();
    same(
        r.adapters
            .create(&model(body.clone()), None, None)
            .await
            .unwrap(),
        &adapter(),
    );
    same(
        r.adapters
            .create(&model(body.clone()), Some(&no), None)
            .await
            .unwrap(),
        &adapter(),
    );
    let params = configured!(AdapterListOptions {
        skip: Some(2),
        limit: Some(6),
        search: Some("adapter".into()),
        status: Some(AdapterStatusFilter::Active),
        include_target_count: Some(false),
    });
    same(r.adapters.list(&params, None).await.unwrap(), &page);
    same(r.adapters.get(ID, None).await.unwrap(), &adapter());
    same(
        r.adapters
            .update(ID, &model(body.clone()), None, None)
            .await
            .unwrap(),
        &adapter(),
    );
    same(
        r.adapters
            .update(ID, &model(body.clone()), Some(&no), None)
            .await
            .unwrap(),
        &adapter(),
    );
    assert!(r.adapters.delete(ID, None).await.unwrap().is_none());
    same(r.adapters.delete(ID, None).await.unwrap().unwrap(), &base());
    let result = r
        .adapters
        .validate(&model(validate_adapter_body()), None)
        .await
        .unwrap();
    assert!(!result.validated);
    assert!(!format!("{result:?}").contains("validation-secret"));
    same(result, &outcome);
    let q = wire.requests.lock().unwrap();
    let path = format!("/v1/adapters/{ID}");
    check(&q[0], "GET", "/v1/adapters/config", &[], None);
    check(
        &q[1],
        "POST",
        "/v1/adapters",
        &[("validate", "true")],
        Some(&body),
    );
    check(
        &q[2],
        "POST",
        "/v1/adapters",
        &[("validate", "false")],
        Some(&body),
    );
    check(
        &q[3],
        "GET",
        "/v1/adapters",
        &[
            ("skip", "2"),
            ("limit", "6"),
            ("search", "adapter"),
            ("status", "ACTIVE"),
            ("include_target_count", "false"),
        ],
        None,
    );
    check(&q[4], "GET", &path, &[], None);
    check(&q[5], "PUT", &path, &[("validate", "true")], Some(&body));
    check(&q[6], "PUT", &path, &[("validate", "false")], Some(&body));
    check(&q[7], "DELETE", &path, &[], None);
    check(&q[8], "DELETE", &path, &[], None);
    check(
        &q[9],
        "POST",
        "/v1/adapters/validate",
        &[],
        Some(&validate_adapter_body()),
    );
}

#[tokio::test]
async fn broker_methods_preserve_repeated_status_defaults_and_ordinary_json_patch() {
    let channel = json!({"uuid":ID,"name":"synthetic-channel","status":"FUTURE","features":{"future_capability":false},"future":"response-secret"});
    let page = json!({"pagination":{"total_items":null},"data":[channel.clone()],"future":"response-secret"});
    let stats = json!({"client_version":"9.9.9","future":"response-secret"});
    let (r, wire, _) = setup(vec![
        Reply::Json(page.clone()),
        Reply::Json(json!({"future":"response-secret"})),
        Reply::Json(channel.clone()),
        Reply::Json(stats.clone()),
        Reply::Json(channel.clone()),
        Reply::Json(channel.clone()),
    ]);
    let params = configured!(ChannelListOptions {
        skip: Some(0),
        limit: Some(8),
        search: Some("broker".into()),
        status: vec!["ONLINE".into(), "DRAFT".into(), "ONLINE".into()],
        include_all_if_empty: Some(false),
    });
    same(r.broker.list_channels(&params, None).await.unwrap(), &page);
    same(
        r.broker
            .list_channels(&ChannelListOptions::default(), None)
            .await
            .unwrap(),
        &json!({"data":[],"future":"response-secret"}),
    );
    let create = json!({"name":"synthetic-channel","description":"synthetic"});
    same(
        r.broker
            .create_channel(&model(create.clone()), None)
            .await
            .unwrap(),
        &channel,
    );
    same(r.broker.get_channel_stats(None).await.unwrap(), &stats);
    same(r.broker.get_channel(ID, None).await.unwrap(), &channel);
    let update = json!({"description":"updated"});
    same(
        r.broker
            .update_channel(ID, &model(update.clone()), None)
            .await
            .unwrap(),
        &channel,
    );
    let q = wire.requests.lock().unwrap();
    check(
        &q[0],
        "GET",
        "/v1/channels",
        &[
            ("skip", "0"),
            ("limit", "8"),
            ("search", "broker"),
            ("status", "ONLINE"),
            ("status", "DRAFT"),
            ("status", "ONLINE"),
            ("include_all_if_empty", "false"),
        ],
        None,
    );
    check(&q[1], "GET", "/v1/channels", &[], None);
    check(&q[2], "POST", "/v1/channels", &[], Some(&create));
    check(&q[3], "GET", "/v1/channels/stats", &[], None);
    check(&q[4], "GET", &format!("/v1/channels/{ID}"), &[], None);
    check(
        &q[5],
        "PATCH",
        &format!("/v1/channels/{ID}"),
        &[],
        Some(&update),
    );
}

#[tokio::test]
async fn eula_methods_read_content_status_and_accept_only_when_called() {
    let content = json!({"content":"synthetic agreement","future":"response-secret"});
    let status = json!({"is_accepted":false,"accepted_at":null,"future":"response-secret"});
    let accepted =
        json!({"is_accepted":true,"accepted_at":"2026-01-01","future":"response-secret"});
    let (r, wire, provider) = setup(vec![
        Reply::Json(content.clone()),
        Reply::Json(status.clone()),
        Reply::Json(accepted.clone()),
    ]);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    same(r.eula.get_content(None).await.unwrap(), &content);
    same(r.eula.get_status(None).await.unwrap(), &status);
    let body = json!({"eula_content":"synthetic agreement","accepted_at":null});
    same(
        r.eula.accept(&model(body.clone()), None).await.unwrap(),
        &accepted,
    );
    let q = wire.requests.lock().unwrap();
    check(&q[0], "GET", "/v1/eula/content", &[], None);
    check(&q[1], "GET", "/v1/eula/status", &[], None);
    check(&q[2], "POST", "/v1/eula/accept", &[], Some(&body));
}

#[tokio::test]
async fn csv_upload_preserves_exact_bytes_mime_forced_filename_and_text_download() {
    let csv = b"prompt,goal\r\n\"synthetic\",\"goal\"\n\0\xff";
    let (r, wire, _) = setup(vec![
        Reply::Json(base()),
        Reply::Json(base()),
        Reply::Raw(b"\xef\xbb\xbfprompt,goal\ninvalid:\xff".to_vec()),
    ]);
    let file = BinaryFile::new(Bytes::copy_from_slice(csv))
        .unwrap()
        .with_filename("ignored-name.txt")
        .unwrap()
        .with_content_type("text/csv")
        .unwrap();
    same(
        r.attacks.upload_prompts_csv(ID, &file, None).await.unwrap(),
        &base(),
    );
    same(
        r.attacks
            .upload_prompts_csv(
                ID,
                &BinaryFile::new(Bytes::from_static(b"plain")).unwrap(),
                None,
            )
            .await
            .unwrap(),
        &base(),
    );
    assert_eq!(
        r.attacks.download_template(ID, None).await.unwrap(),
        "prompt,goal\ninvalid:\u{fffd}"
    );
    let q = wire.requests.lock().unwrap();
    for (request, data, mime) in [
        (&q[0], csv.as_slice(), "text/csv"),
        (&q[1], b"plain".as_slice(), "application/octet-stream"),
    ] {
        assert_eq!(request.method(), "POST");
        assert_eq!(request.url().host_str(), Some("red-team.example.test"));
        assert_eq!(
            request.url().path(),
            "/v1/custom-attack/upload-custom-prompts-csv"
        );
        assert_eq!(
            request.url().query_pairs().collect::<Vec<_>>(),
            vec![("prompt_set_uuid".into(), ID.into())]
        );
        let mut names = request
            .headers()
            .keys()
            .map(|key| key.as_str())
            .collect::<Vec<_>>();
        names.sort_unstable();
        assert_eq!(
            names,
            [
                "authorization",
                "content-type",
                "service-name",
                "user-agent"
            ]
        );
        assert!(request.headers().values().all(|v| v.is_sensitive()));
        assert_eq!(
            request.headers()["authorization"],
            format!("Bearer {TOKEN}")
        );
        assert_eq!(request.headers()["service-name"], "api");
        assert_eq!(
            request.headers()["user-agent"],
            concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
        );
        for name in names {
            assert_eq!(request.headers().get_all(name).iter().count(), 1);
        }
        let boundary = request.headers()["content-type"]
            .to_str()
            .unwrap()
            .strip_prefix("multipart/form-data; boundary=")
            .unwrap();
        let mut expected = format!(
            concat!(
                "--{boundary}\r\n",
                "Content-Disposition: form-data; name=\"file\"; filename=\"prompts.csv\"\r\n",
                "Content-Type: {mime}\r\n\r\n"
            ),
            boundary = boundary,
            mime = mime
        )
        .into_bytes();
        expected.extend_from_slice(data);
        expected.extend_from_slice(format!("\r\n--{boundary}--\r\n").as_bytes());
        assert_eq!(request.body().as_ref(), expected);
        assert!(!format!("{request:?}").contains("synthetic\",\"goal"));
    }
    check(
        &q[2],
        "GET",
        &format!("/v1/custom-attack/download-template/{ID}"),
        &[],
        None,
    );
}

#[tokio::test]
async fn all_four_walks_default_to_skip_zero_limit_fifty_and_preserve_duplicates() {
    let pages = [
        list(vec![set(), set()], None),
        list(vec![prompt(), prompt()], None),
        list(vec![target(), target()], None),
        list(vec![adapter_row(), adapter_row()], None),
    ];
    let (r, wire, _) = setup(pages.iter().cloned().map(Reply::Json).collect());
    let sets = r
        .attacks
        .list_all_prompt_sets(
            &PromptSetListOptions::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    assert!(sets.is_complete());
    same(sets.items, &pages[0]["data"]);
    let prompts = r
        .attacks
        .list_all_prompts(
            ID,
            &PromptListOptions::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    assert!(prompts.is_complete());
    same(prompts.items, &pages[1]["data"]);
    let targets = r
        .targets
        .list_all(
            &TargetListOptions::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    assert!(targets.is_complete());
    same(targets.items, &pages[2]["data"]);
    let adapters = r
        .adapters
        .list_all(
            &AdapterListOptions::default(),
            CollectOptions::default(),
            None,
        )
        .await
        .unwrap();
    assert!(adapters.is_complete());
    same(adapters.items, &pages[3]["data"]);
    let q = wire.requests.lock().unwrap();
    for (request, path) in q.iter().zip([
        "/v1/custom-attack/list-custom-prompt-sets",
        &format!("/v1/custom-attack/custom-prompt-set/{ID}/list-custom-prompts"),
        "/v1/target",
        "/v1/adapters",
    ]) {
        check(
            request,
            "GET",
            path,
            &[("skip", "0"), ("limit", "50")],
            None,
        );
    }
}

#[tokio::test]
async fn direct_lists_omit_paging_defaults_and_filtered_walks_keep_all_options_and_budget() {
    let (r, wire, provider) = setup(vec![
        Reply::Json(list(vec![], None)),
        Reply::Json(list(vec![], None)),
        Reply::Json(list(vec![], None)),
        Reply::Json(list(vec![], None)),
        Reply::Json(list(vec![set(), set()], Some(json!(3)))),
        Reply::Json(list(vec![set()], Some(json!(3)))),
        Reply::Json(list(vec![prompt(), prompt()], Some(json!(3)))),
        Reply::Json(list(vec![prompt()], Some(json!(3)))),
        Reply::Json(list(vec![target(), target()], Some(json!(3)))),
        Reply::Json(list(vec![target()], Some(json!(3)))),
        Reply::Json(list(vec![adapter_row(), adapter_row()], Some(json!(3)))),
        Reply::Json(list(vec![adapter_row()], Some(json!(3)))),
    ]);
    r.attacks
        .list_prompt_sets(&PromptSetListOptions::default(), None)
        .await
        .unwrap();
    r.attacks
        .list_prompts(ID, &PromptListOptions::default(), None)
        .await
        .unwrap();
    r.targets
        .list(&TargetListOptions::default(), None)
        .await
        .unwrap();
    r.adapters
        .list(&AdapterListOptions::default(), None)
        .await
        .unwrap();
    let sets = configured!(PromptSetListOptions {
        limit: Some(2),
        search: Some("same filter".into()),
        language: Some("en".into()),
        status: Some("READY".into()),
        active: Some(false),
        archive: Some(true),
    });
    let prompts = configured!(PromptListOptions {
        limit: Some(2),
        search: Some("same filter".into()),
        status: Some("READY".into()),
        active: Some(false),
    });
    let targets = configured!(TargetListOptions {
        limit: Some(2),
        search: Some("same filter".into()),
        adapter_uuid: Some(ID.into()),
        target_type: Some("APPLICATION".into()),
        status: Some("READY".into()),
    });
    let adapters = configured!(AdapterListOptions {
        limit: Some(2),
        search: Some("same filter".into()),
        status: Some(AdapterStatusFilter::Draft),
        include_target_count: Some(true),
    });
    let options = RequestOptions::with_timeout(Duration::from_secs(5)).unwrap();
    assert_eq!(
        r.attacks
            .list_all_prompt_sets(&sets, CollectOptions::default(), Some(&options))
            .await
            .unwrap()
            .items
            .len(),
        3
    );
    assert_eq!(
        r.attacks
            .list_all_prompts(ID, &prompts, CollectOptions::default(), Some(&options))
            .await
            .unwrap()
            .items
            .len(),
        3
    );
    assert_eq!(
        r.targets
            .list_all(&targets, CollectOptions::default(), Some(&options))
            .await
            .unwrap()
            .items
            .len(),
        3
    );
    assert_eq!(
        r.adapters
            .list_all(&adapters, CollectOptions::default(), Some(&options))
            .await
            .unwrap()
            .items
            .len(),
        3
    );
    let q = wire.requests.lock().unwrap();
    for request in &q[..4] {
        assert!(request.url().query().is_none());
    }
    for (i, extras) in [
        vec![
            ("language", "en"),
            ("status", "READY"),
            ("active", "false"),
            ("archive", "true"),
        ],
        vec![("status", "READY"), ("active", "false")],
        vec![
            ("adapter_uuid", ID),
            ("target_type", "APPLICATION"),
            ("status", "READY"),
        ],
        vec![("status", "DRAFT"), ("include_target_count", "true")],
    ]
    .iter()
    .enumerate()
    {
        for (j, skip) in ["0", "2"].iter().enumerate() {
            let mut expected = vec![("skip", *skip), ("limit", "2"), ("search", "same filter")];
            expected.extend(extras.iter().copied());
            check(
                &q[4 + i * 2 + j],
                "GET",
                q[4 + i * 2 + j].url().path(),
                &expected,
                None,
            );
        }
    }
    let deadlines = provider.deadlines.lock().unwrap();
    assert_eq!(deadlines.len(), 12);
    assert!(
        deadlines[4..]
            .iter()
            .all(|deadline| *deadline == options.deadline())
    );
}

#[tokio::test]
async fn skip_walks_use_actual_counts_null_totals_and_empty_terminal_pages() {
    for total in [None, Some(Value::Null)] {
        let (r, wire, _) = setup(vec![
            Reply::Json(list(vec![set(), set()], total.clone())),
            Reply::Json(list(vec![], Some(json!(999)))),
        ]);
        let params = configured!(PromptSetListOptions {
            skip: Some(5),
            limit: Some(2),
        });
        let result = r
            .attacks
            .list_all_prompt_sets(&params, CollectOptions::default(), None)
            .await
            .unwrap();
        assert!(result.is_complete());
        assert_eq!(result.items.len(), 2);
        let q = wire.requests.lock().unwrap();
        check(
            &q[0],
            "GET",
            "/v1/custom-attack/list-custom-prompt-sets",
            &[("skip", "5"), ("limit", "2")],
            None,
        );
        check(
            &q[1],
            "GET",
            "/v1/custom-attack/list-custom-prompt-sets",
            &[("skip", "7"), ("limit", "2")],
            None,
        );
    }
    // A short page still continues when an explicit total says there are more rows.
    let (r, wire, _) = setup(vec![
        Reply::Json(list(vec![target()], Some(json!(2)))),
        Reply::Json(list(vec![target()], Some(json!(2)))),
    ]);
    let params = configured!(TargetListOptions { limit: Some(50) });
    assert_eq!(
        r.targets
            .list_all(&params, CollectOptions::default(), None)
            .await
            .unwrap()
            .items
            .len(),
        2
    );
    let q = wire.requests.lock().unwrap();
    assert_eq!(
        q[1].url()
            .query_pairs()
            .find(|(k, _)| k == "skip")
            .unwrap()
            .1,
        "1"
    );
}

#[tokio::test]
async fn page_item_helpers_caps_resume_and_partial_failures_are_explicit() {
    let rows = list(vec![adapter_row(), adapter_row()], Some(json!(2)));
    let (r, wire, _) = setup(vec![Reply::Json(rows.clone()), Reply::Json(rows.clone())]);
    let cap = configured!(CollectOptions { max_items: 1 });
    let first = r
        .adapters
        .list_all(&AdapterListOptions::default(), cap, None)
        .await
        .unwrap();
    assert_eq!(first.completion, Completion::Truncated(Truncation::Items));
    let resume = first.resume.unwrap();
    assert_eq!((resume.cursor, resume.item_offset), (0, 1));
    let rest = r
        .adapters
        .list_items(&AdapterListOptions::default(), 10, None)
        .unwrap()
        .resume_from(resume)
        .unwrap()
        .collect(CollectOptions::default())
        .await
        .unwrap();
    assert!(rest.is_complete());
    assert_eq!(rest.items.len(), 1);
    assert_eq!(wire.requests.lock().unwrap().len(), 2);
    let (r, _, _) = setup(vec![Reply::Json(rows)]);
    let result = r
        .adapters
        .list_all(
            &AdapterListOptions::default(),
            configured!(CollectOptions { max_bytes: 2 }),
            None,
        )
        .await
        .unwrap();
    assert_eq!(result.completion, Completion::Truncated(Truncation::Bytes));
    assert!(result.items.is_empty());
    assert_eq!(result.resume.unwrap().item_offset, 0);
    let (r, wire, _) = setup(vec![Reply::Json(list(vec![prompt()], Some(json!(10))))]);
    let params = configured!(PromptListOptions { limit: Some(1) });
    let result = r
        .attacks
        .list_all_prompts(
            ID,
            &params,
            configured!(CollectOptions { max_pages: 1 }),
            None,
        )
        .await
        .unwrap();
    assert_eq!(result.completion, Completion::Truncated(Truncation::Pages));
    assert_eq!(result.resume.unwrap().cursor, 1);
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
    let (r, wire, _) = setup(vec![
        Reply::Json(list(vec![set()], Some(json!(2)))),
        Reply::Failure,
    ]);
    let options = RequestOptions::with_timeout(Duration::from_secs(5))
        .unwrap()
        .with_retries(0)
        .unwrap();
    let error = r
        .attacks
        .list_all_prompt_sets(
            &PromptSetListOptions::default(),
            CollectOptions::default(),
            Some(&options),
        )
        .await
        .unwrap_err();
    assert_eq!(error.error().kind(), ErrorKind::Transport);
    assert_eq!(error.partial().items.len(), 1);
    assert_eq!(error.partial().resume.as_ref().unwrap().cursor, 1);
    assert!(!format!("{error:?}").contains("response-secret"));
    assert_eq!(wire.requests.lock().unwrap().len(), 2);
    for total in [-1_i64, 9_007_199_254_740_992] {
        let (r, _, _) = setup(vec![Reply::Json(list(vec![target()], Some(json!(total))))]);
        assert_eq!(
            r.targets
                .list_all(
                    &TargetListOptions::default(),
                    CollectOptions::default(),
                    None
                )
                .await
                .unwrap_err()
                .error()
                .kind(),
            ErrorKind::Pagination
        );
    }
}

#[tokio::test]
async fn all_native_page_and_item_helpers_execute_with_no_extra_terminal_dispatch() {
    let (r, wire, _) = setup(vec![
        Reply::Json(list(vec![set()], Some(json!(1)))),
        Reply::Json(list(vec![set()], Some(json!(1)))),
        Reply::Json(list(vec![prompt()], Some(json!(1)))),
        Reply::Json(list(vec![prompt()], Some(json!(1)))),
        Reply::Json(list(vec![target()], Some(json!(1)))),
        Reply::Json(list(vec![target()], Some(json!(1)))),
        Reply::Json(list(vec![adapter_row()], Some(json!(1)))),
        Reply::Json(list(vec![adapter_row()], Some(json!(1)))),
    ]);
    let mut pages = r
        .attacks
        .prompt_set_pages(&PromptSetListOptions::default(), 2, None)
        .unwrap();
    assert_eq!(pages.next_page().await.unwrap().unwrap().items.len(), 1);
    assert!(pages.next_page().await.unwrap().is_none());
    let mut items = r
        .attacks
        .prompt_set_items(&PromptSetListOptions::default(), 2, None)
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    assert!(items.next().await.unwrap().is_none());
    let mut pages = r
        .attacks
        .prompt_pages(ID, &PromptListOptions::default(), 2, None)
        .unwrap();
    assert_eq!(pages.next_page().await.unwrap().unwrap().items.len(), 1);
    assert!(pages.next_page().await.unwrap().is_none());
    let mut items = r
        .attacks
        .prompt_items(ID, &PromptListOptions::default(), 2, None)
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    assert!(items.next().await.unwrap().is_none());
    let mut pages = r
        .targets
        .list_pages(&TargetListOptions::default(), 2, None)
        .unwrap();
    assert_eq!(pages.next_page().await.unwrap().unwrap().items.len(), 1);
    assert!(pages.next_page().await.unwrap().is_none());
    let mut items = r
        .targets
        .list_items(&TargetListOptions::default(), 2, None)
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    assert!(items.next().await.unwrap().is_none());
    let mut pages = r
        .adapters
        .list_pages(&AdapterListOptions::default(), 2, None)
        .unwrap();
    assert_eq!(pages.next_page().await.unwrap().unwrap().items.len(), 1);
    assert!(pages.next_page().await.unwrap().is_none());
    let mut items = r
        .adapters
        .list_items(&AdapterListOptions::default(), 2, None)
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    assert!(items.next().await.unwrap().is_none());
    assert_eq!(wire.requests.lock().unwrap().len(), 8);
}

#[tokio::test]
async fn cancellation_before_dispatch_and_between_buffered_items_preserves_budget() {
    let (r, wire, provider) = setup(vec![]);
    let options = RequestOptions::with_timeout(Duration::from_secs(5)).unwrap();
    options.cancel();
    assert_eq!(
        r.eula.get_content(Some(&options)).await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(
        r.attacks
            .upload_prompts_csv(
                ID,
                &BinaryFile::new(Bytes::from_static(b"synthetic")).unwrap(),
                Some(&options)
            )
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(
        r.targets
            .create(&model(json!({"name":"synthetic"})), None, Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert!(wire.requests.lock().unwrap().is_empty());
    let (r, wire, provider) = setup(vec![Reply::Json(list(
        vec![target(), target()],
        Some(json!(2)),
    ))]);
    let options = RequestOptions::with_timeout(Duration::from_secs(5)).unwrap();
    let mut items = r
        .targets
        .list_items(&TargetListOptions::default(), 10, Some(&options))
        .unwrap();
    assert!(items.next().await.unwrap().is_some());
    options.cancel();
    assert_eq!(items.next().await.unwrap_err().kind(), ErrorKind::Cancelled);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
}

#[tokio::test]
async fn invalid_identifiers_models_queries_and_extensions_fail_before_credentials() {
    let (r, wire, provider) = setup(vec![]);
    for bad in [
        "bad",
        "../escape",
        "550e8400-e29b-41d4-a716-446655440000/extra",
        "550e8400-e29b-41d4-a716-44665544000%",
    ] {
        assert!(r.attacks.get_prompt_set(bad, None).await.is_err());
        assert!(
            r.attacks
                .update_prompt_set(bad, &model(json!({})), None)
                .await
                .is_err()
        );
        assert!(
            r.attacks
                .archive_prompt_set(bad, &model(json!({"archive":true})), None)
                .await
                .is_err()
        );
        assert!(r.attacks.get_prompt_set_reference(bad, None).await.is_err());
        assert!(
            r.attacks
                .get_prompt_set_version_info(bad, None, None)
                .await
                .is_err()
        );
        assert!(r.attacks.download_template(bad, None).await.is_err());
        assert!(
            r.attacks
                .upload_prompts_csv(bad, &BinaryFile::new(Bytes::new()).unwrap(), None)
                .await
                .is_err()
        );
        assert!(
            r.attacks
                .list_prompts(bad, &PromptListOptions::default(), None)
                .await
                .is_err()
        );
        assert!(r.attacks.get_prompt(bad, ID, None).await.is_err());
        assert!(r.attacks.get_prompt(ID, bad, None).await.is_err());
        assert!(
            r.attacks
                .update_prompt(ID, bad, &model(json!({})), None)
                .await
                .is_err()
        );
        assert!(r.attacks.delete_prompt(bad, ID, None).await.is_err());
        assert!(
            r.attacks
                .prompt_pages(bad, &PromptListOptions::default(), 1, None)
                .is_err()
        );
        assert!(r.targets.start_profiling(bad, None).await.is_err());
        assert!(r.targets.delete_copilot_token(bad, None).await.is_err());
        assert!(r.targets.get(bad, None).await.is_err());
        assert!(
            r.targets
                .update(bad, &model(json!({"name":"synthetic"})), None, None)
                .await
                .is_err()
        );
        assert!(r.targets.delete(bad, None).await.is_err());
        assert!(r.targets.get_profile(bad, None).await.is_err());
        assert!(
            r.targets
                .update_profile(bad, &model(json!({})), None)
                .await
                .is_err()
        );
        assert!(r.adapters.get(bad, None).await.is_err());
        assert!(
            r.adapters
                .update(bad, &model(adapter_body()), None, None)
                .await
                .is_err()
        );
        assert!(r.adapters.delete(bad, None).await.is_err());
        assert!(r.broker.get_channel(bad, None).await.is_err());
        assert!(
            r.broker
                .update_channel(bad, &model(json!({})), None)
                .await
                .is_err()
        );
    }
    assert!(
        r.attacks
            .create_prompt_set(&model(json!({"name":"x".repeat(256)})), None)
            .await
            .is_err()
    );
    assert!(
        r.attacks
            .create_prompt(
                &model(json!({"prompt":"synthetic","prompt_set_id":"bad"})),
                None
            )
            .await
            .is_err()
    );
    assert!(
        r.attacks
            .create_property_name(&model(json!({"name":"x".repeat(257)})), None)
            .await
            .is_err()
    );
    assert!(
        r.attacks
            .create_property_value(
                &model(json!({"property_name":"synthetic","property_value":"x".repeat(257)})),
                None
            )
            .await
            .is_err()
    );
    assert!(
        r.attacks
            .list_prompt_sets(&configured!(PromptSetListOptions { limit: Some(0) }), None)
            .await
            .is_err()
    );
    assert!(
        r.attacks
            .list_prompts(
                ID,
                &configured!(PromptListOptions {
                    skip: Some(9_007_199_254_740_992),
                }),
                None
            )
            .await
            .is_err()
    );
    assert!(
        r.attacks
            .get_prompt_set_version_info(ID, Some("bad\r\nfilter"), None)
            .await
            .is_err()
    );
    assert!(
        r.attacks
            .get_property_values("../escape", None)
            .await
            .is_err()
    );
    assert!(
        r.attacks
            .get_property_values_multiple(&vec!["synthetic".into(); 257], None)
            .await
            .is_err()
    );
    let mut target: TargetCreateRequest = model(json!({"name":"synthetic"}));
    target.extensions.insert(
        "rogue".into(),
        scm_sdk::data::JsonValue::new(json!("request-secret")),
    );
    assert!(r.targets.create(&target, None, None).await.is_err());
    assert!(
        r.targets
            .list(
                &configured!(TargetListOptions {
                    adapter_uuid: Some("bad".into()),
                }),
                None
            )
            .await
            .is_err()
    );
    assert!(
        r.targets
            .list(
                &configured!(TargetListOptions {
                    status: Some("bad\nfilter".into()),
                }),
                None
            )
            .await
            .is_err()
    );
    let mut auth = copilot_auth();
    auth["token_json_uuid"] = json!("bad");
    assert!(
        r.targets
            .get_copilot_auth_url(&model(auth), None)
            .await
            .is_err()
    );
    let mut exchange = copilot_token();
    exchange["token_json_uuid"] = json!("bad");
    assert!(
        r.targets
            .exchange_copilot_token(&model(exchange), None)
            .await
            .is_err()
    );
    let mut body = adapter_body();
    body["name"] = json!("x".repeat(256));
    assert!(r.adapters.create(&model(body), None, None).await.is_err());
    let mut body = validate_adapter_body();
    body["network_broker_channel_uuid"] = json!("bad");
    assert!(r.adapters.validate(&model(body), None).await.is_err());
    assert!(
        r.adapters
            .list(
                &configured!(AdapterListOptions {
                    search: Some("x".repeat(4097)),
                }),
                None
            )
            .await
            .is_err()
    );
    assert!(
        r.broker
            .create_channel(&model(json!({"name":""})), None)
            .await
            .is_err()
    );
    assert!(
        r.broker
            .update_channel(ID, &model(json!({"name":""})), None)
            .await
            .is_err()
    );
    assert!(
        r.broker
            .list_channels(
                &configured!(ChannelListOptions {
                    status: vec!["ONLINE".into(); 253],
                }),
                None
            )
            .await
            .is_err()
    );
    let mut accept: EulaAcceptRequest = model(json!({"eula_content":"synthetic"}));
    accept.extensions.insert(
        "eula_content".into(),
        scm_sdk::data::JsonValue::new(json!("request-secret")),
    );
    let error = r.eula.accept(&accept, None).await.unwrap_err();
    assert!(!format!("{error:?} {error}").contains("request-secret"));
    let result = r
        .targets
        .list_all(
            &TargetListOptions::default(),
            configured!(CollectOptions { max_items: 0 }),
            None,
        )
        .await;
    assert!(result.is_err());
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert!(wire.requests.lock().unwrap().is_empty());
}

#[tokio::test]
async fn csv_file_envelope_download_and_response_shape_limits_are_bounded() {
    let limits = configured!(Limits {
        max_request_bytes: 1024,
        max_response_bytes: 1024,
    });
    let (r, wire, provider) = setup_limits(vec![], limits);
    for length in [1025, 950] {
        let file = BinaryFile::new(Bytes::from(vec![b'x'; length])).unwrap();
        assert_eq!(
            r.attacks
                .upload_prompts_csv(ID, &file, None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Limit
        );
    }
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert!(wire.requests.lock().unwrap().is_empty());
    let (r, wire, _) = setup_limits(
        vec![Reply::Raw(vec![b'x'; 1025])],
        configured!(Limits {
            max_response_bytes: 1024,
        }),
    );
    assert_eq!(
        r.attacks
            .download_template(ID, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    assert_eq!(wire.requests.lock().unwrap().len(), 1);
    let (r, wire, _) = setup(vec![
        Reply::Json(json!("upstream-secret")),
        Reply::Json(json!(["upstream-secret"])),
        Reply::Json(json!({"uuid":ID,"name":42,"private":"upstream-secret"})),
    ]);
    for error in [
        r.targets.get_target_metadata(None).await.unwrap_err(),
        r.targets.get_target_metadata(None).await.unwrap_err(),
        r.targets.get(ID, None).await.unwrap_err(),
    ] {
        assert_eq!(error.kind(), ErrorKind::Decode);
        assert!(!format!("{error:?} {error}").contains("upstream-secret"));
    }
    assert_eq!(wire.requests.lock().unwrap().len(), 3);
}

#[tokio::test]
async fn all_twenty_six_resource_mutations_keep_unknown_outcome_without_replay() {
    let (r, wire, _) = setup((0..26).map(|_| Reply::Failure).collect());
    let options = RequestOptions::with_timeout(Duration::from_secs(10))
        .unwrap()
        .with_retries(5)
        .unwrap();
    let options = Some(&options);
    let file = BinaryFile::new(Bytes::from_static(b"prompt\nsynthetic")).unwrap();
    let errors=[
        r.attacks.create_prompt_set(&model(json!({"name":"synthetic"})),options).await.unwrap_err(),
        r.attacks.update_prompt_set(ID,&model(json!({"name":null})),options).await.unwrap_err(),
        r.attacks.archive_prompt_set(ID,&model(json!({"archive":true})),options).await.unwrap_err(),
        r.attacks.upload_prompts_csv(ID,&file,options).await.unwrap_err(),
        r.attacks.create_prompt(&model(json!({"prompt":"synthetic","prompt_set_id":ID})),options).await.unwrap_err(),
        r.attacks.update_prompt(ID,OTHER_ID,&model(json!({"goal":null})),options).await.unwrap_err(),
        r.attacks.delete_prompt(ID,OTHER_ID,options).await.unwrap_err(),
        r.attacks.create_property_name(&model(json!({"name":"synthetic"})),options).await.unwrap_err(),
        r.attacks.create_property_value(&model(json!({"property_name":"synthetic","property_value":"synthetic"})),options).await.unwrap_err(),
        r.targets.start_profiling(ID,options).await.unwrap_err(),
        r.targets.get_copilot_auth_url(&model(copilot_auth()),options).await.unwrap_err(),
        r.targets.exchange_copilot_token(&model(copilot_token()),options).await.unwrap_err(),
        r.targets.delete_copilot_token(ID,options).await.unwrap_err(),
        r.targets.create(&model(json!({"name":"synthetic"})),None,options).await.unwrap_err(),
        r.targets.update(ID,&model(json!({"name":"synthetic"})),None,options).await.unwrap_err(),
        r.targets.delete(ID,options).await.unwrap_err(),
        r.targets.probe(&model(json!({"name":"synthetic","uuid":ID})),options).await.unwrap_err(),
        r.targets.update_profile(ID,&model(json!({"additional_context":null})),options).await.unwrap_err(),
        r.targets.validate_auth(&model(json!({"auth_type":"HEADERS","auth_config":{"auth_header":{"Authorization":"target-secret"}}})),options).await.unwrap_err(),
        r.adapters.create(&model(adapter_body()),None,options).await.unwrap_err(),
        r.adapters.update(ID,&model(adapter_body()),None,options).await.unwrap_err(),
        r.adapters.delete(ID,options).await.unwrap_err(),
        r.adapters.validate(&model(validate_adapter_body()),options).await.unwrap_err(),
        r.broker.create_channel(&model(json!({"name":"synthetic"})),options).await.unwrap_err(),
        r.broker.update_channel(ID,&model(json!({"description":"synthetic"})),options).await.unwrap_err(),
        r.eula.accept(&model(json!({"eula_content":"synthetic"})),options).await.unwrap_err(),
    ];
    for error in errors {
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Transport, RemoteOutcome::Unknown)
        );
        for secret in [TOKEN, "copilot-secret", "callback-secret", "target-secret"] {
            assert!(!format!("{error:?} {error}").contains(secret));
        }
    }
    assert_eq!(wire.requests.lock().unwrap().len(), 26);
}

#[test]
fn all_resource_constructors_require_oauth_and_have_safe_debug() {
    for authentication in [
        Authentication::None,
        Authentication::scan(Some(Secret::new("synthetic-scan-secret").unwrap()), None).unwrap(),
        Authentication::runtime_key(Secret::new("synthetic-runtime-secret").unwrap()).unwrap(),
    ] {
        let wire = Arc::new(Wire {
            requests: Mutex::new(Vec::new()),
            replies: Mutex::new(VecDeque::new()),
        });
        let client = Client::builder(Endpoint::parse("https://red-team.example.test").unwrap())
            .transport(wire)
            .authentication(authentication)
            .build()
            .unwrap();
        assert!(RedTeamCustomAttacksClient::new(client.clone()).is_err());
        assert!(RedTeamTargetsClient::new(client.clone()).is_err());
        assert!(RedTeamAdaptersClient::new(client.clone()).is_err());
        assert!(RedTeamNetworkBrokerClient::new(client.clone()).is_err());
        assert!(RedTeamEulaClient::new(client).is_err());
    }
    let options = configured!(TargetListOptions {
        search: Some("filter-secret".into()),
    });
    assert!(!format!("{options:?}").contains("filter-secret"));
}

#[tokio::test]
async fn nullable_request_fields_keep_omission_null_and_value_in_exact_json_bytes() {
    let (r, wire, _) = setup((0..9).map(|_| Reply::Json(set())).collect());
    for key in ["name", "description", "property_names"] {
        for value in [
            None,
            Some(Value::Null),
            Some(if key == "property_names" {
                json!(["severity"])
            } else {
                json!("synthetic")
            }),
        ] {
            let mut body = json!({});
            if let Some(value) = value {
                body[key] = value;
            }
            r.attacks
                .update_prompt_set(ID, &model(body.clone()), None)
                .await
                .unwrap();
            let requests = wire.requests.lock().unwrap();
            check(
                requests.last().unwrap(),
                "PUT",
                &format!("/v1/custom-attack/custom-prompt-set/{ID}"),
                &[],
                Some(&body),
            );
        }
    }
    assert_eq!(wire.requests.lock().unwrap().len(), 9);
}

#[tokio::test]
async fn expired_csv_budget_precedes_encoding_validation_and_credentials() {
    let (r, wire, provider) = setup_limits(
        vec![],
        configured!(Limits {
            max_request_bytes: 1024
        }),
    );
    let options = RequestOptions::with_timeout(Duration::from_nanos(1)).unwrap();
    tokio::time::sleep(Duration::from_millis(2)).await;
    // This would exceed the file bound if encoding/validation were polled first.
    let file = BinaryFile::new(Bytes::from(vec![b'x'; 1025])).unwrap();
    assert_eq!(
        r.attacks
            .upload_prompts_csv(ID, &file, Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Deadline
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert!(wire.requests.lock().unwrap().is_empty());
}

fn configured_target() -> Value {
    let mut target = target();
    target["connection_params"] =
        json!({"api_endpoint":"wss://target.example.test/ws","future":"kept"});
    target["auth_config"] = json!({"basic_auth_header":{"Authorization":"target-secret"}});
    target
}

#[tokio::test]
async fn target_documents_without_response_defaults_contain_only_server_sent_fields() {
    let sent = configured_target();
    let mut oauth = target();
    oauth["auth_config"] = json!({"oauth2_token_url":"https://issuer.example.test/token","oauth2_inject_header":{"Authorization":"Bearer {token}"}});
    let page = list(vec![sent.clone(), sent.clone()], Some(json!(2)));
    let mut invalid = target();
    invalid["auth_config"] = json!({"basic_auth_location":5});
    let (r, wire, _) = setup(vec![
        Reply::Json(sent.clone()),
        Reply::Json(sent.clone()),
        Reply::Json(page.clone()),
        Reply::Json(oauth.clone()),
        Reply::Json(sent.clone()),
        Reply::Json(sent.clone()),
        Reply::Json(sent.clone()),
        Reply::Json(page.clone()),
        Reply::Json(invalid.clone()),
        Reply::Json(invalid),
    ]);
    // Source parity remains the default: frozen schema defaults are inserted.
    let parity = serde_json::to_value(r.targets.get(ID, None).await.unwrap()).unwrap();
    assert_eq!(parity["connection_params"]["ws_response_timeout"], 110);
    assert_eq!(parity["auth_config"]["basic_auth_location"], "HEADER");
    assert_ne!(parity, sent);

    let options = RequestOptions::with_timeout(Duration::from_secs(5))
        .unwrap()
        .without_response_defaults();
    assert!(!options.response_defaults());
    assert!(
        RequestOptions::with_timeout(Duration::from_secs(5))
            .unwrap()
            .response_defaults()
    );
    let options = Some(&options);
    same(r.targets.get(ID, options).await.unwrap(), &sent);
    same(
        r.targets
            .list(&TargetListOptions::default(), options)
            .await
            .unwrap(),
        &page,
    );
    let body = json!({"name":"synthetic-target","target_type":"APPLICATION"});
    same(
        r.targets
            .create(&model(body.clone()), None, options)
            .await
            .unwrap(),
        &oauth,
    );
    same(
        r.targets
            .update(ID, &model(body.clone()), None, options)
            .await
            .unwrap(),
        &sent,
    );
    let probe = json!({"name":"synthetic-target","uuid":ID,"probe_fields":["multi_turn"]});
    same(
        r.targets.probe(&model(probe), options).await.unwrap(),
        &sent,
    );
    let update = json!({"target_background":{"industry":"synthetic"}});
    same(
        r.targets
            .update_profile(ID, &model(update), options)
            .await
            .unwrap(),
        &sent,
    );
    // Paginated walks carry the per-operation choice to every page.
    let collected = r
        .targets
        .list_all(
            &TargetListOptions::default(),
            CollectOptions::default(),
            options,
        )
        .await
        .unwrap();
    assert_eq!(collected.items.len(), 2);
    for item in collected.items {
        same(item, &sent);
    }

    // Schema-invalid documents are still rejected under both policies, with the
    // typed operation's own name.
    for options in [None, options] {
        let error = r.targets.get(ID, options).await.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Decode);
        assert_eq!(error.operation(), Some("red_team.targets.get"));
    }
    assert_eq!(wire.requests.lock().unwrap().len(), 10);
}
