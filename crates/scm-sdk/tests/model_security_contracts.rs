//! Independent literal wire fixtures from frozen Model Security clients and schemas.
#![cfg(feature = "model-security")]
use bytes::Bytes;
use futures_util::{future::BoxFuture, stream};
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, OriginPolicy, RemoteOutcome, RequestOptions, Result,
    Secret,
    airs::model_security::{models::*, *},
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    data::{JsonValue, Patch},
    pagination::{Completion, Resume, Truncation},
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
const OTHER: &str = "660e8400-e29b-41d4-a716-446655440000";
#[derive(Clone, Copy)]
enum Plane {
    Data,
    Management,
}
enum Reply {
    Json(u16, Value),
    Empty,
    Failure,
    Text(&'static str),
}
struct Fixture {
    plane: Plane,
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    body: Option<Value>,
    reply: Reply,
    delay: Duration,
}
#[derive(Default)]
struct Wire {
    fixtures: Mutex<VecDeque<Fixture>>,
    calls: AtomicUsize,
}
impl Wire {
    fn expect(
        &self,
        plane: Plane,
        method: Method,
        path: &str,
        query: &[(&str, &str)],
        body: Option<Value>,
        reply: Reply,
    ) {
        self.fixtures.lock().unwrap().push_back(Fixture {
            plane,
            method,
            path: path.into(),
            query: query
                .iter()
                .map(|(k, v)| ((*k).into(), (*v).into()))
                .collect(),
            body,
            reply,
            delay: Duration::ZERO,
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
                .expect("unexpected request or mutation replay");
            let base = match expected.plane {
                Plane::Data => "https://model-data.example/aims/data",
                Plane::Management => "https://model-management.example/aims/mgmt",
            };
            let mut url = request.url().clone();
            url.set_query(None);
            assert_eq!(url.as_str(), format!("{base}{}", expected.path));
            assert_eq!(request.method(), expected.method);
            let query: Vec<_> = request.url().query_pairs().into_owned().collect();
            assert_eq!(query, expected.query);
            let headers = request.headers();
            let mut names: Vec<_> = headers.keys().map(|h| h.as_str()).collect();
            names.sort_unstable();
            let expected_names = if expected.body.is_some() {
                vec![
                    "authorization",
                    "content-type",
                    "service-name",
                    "user-agent",
                ]
            } else {
                vec!["authorization", "service-name", "user-agent"]
            };
            assert_eq!(names, expected_names);
            assert_eq!(headers.len(), expected_names.len());
            assert_eq!(headers["authorization"], "Bearer fixture-model-token");
            assert!(headers["authorization"].is_sensitive());
            assert_eq!(headers["service-name"], "api");
            assert_eq!(
                headers["user-agent"],
                concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
            );
            if let Some(body) = expected.body {
                assert_eq!(headers["content-type"], "application/json");
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
            let (status, body) = match expected.reply {
                Reply::Json(status, value) => (
                    StatusCode::from_u16(status).unwrap(),
                    Bytes::from(serde_json::to_vec(&value).unwrap()),
                ),
                Reply::Empty => (StatusCode::NO_CONTENT, Bytes::new()),
                Reply::Failure => return Err(Error::new(ErrorKind::Transport)),
                Reply::Text(text) => (StatusCode::OK, Bytes::from_static(text.as_bytes())),
            };
            let mut headers = HeaderMap::new();
            headers.insert("content-type", HeaderValue::from_static("application/json"));
            Ok(TransportResponse::new(
                status,
                headers,
                Box::pin(stream::iter([Ok(body)])),
            ))
        })
    }
}
struct Token {
    context: AuthContext,
    calls: AtomicUsize,
}
impl Token {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            context: AuthContext::new(
                Endpoint::parse("https://issuer.example/token").unwrap(),
                "model-client",
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
                Secret::new("fixture-model-token").unwrap(),
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
        Endpoint::parse("https://model-data.example/aims/data").unwrap(),
        Endpoint::parse("https://model-management.example/aims/mgmt").unwrap(),
    )
}
fn clients() -> (ModelSecurityClient, Arc<Wire>, Arc<Token>) {
    let (data, management) = endpoints();
    let wire = Arc::new(Wire::default());
    let token = Token::new();
    let client = Client::builder(management.clone())
        .transport(wire.clone())
        .destination_policy(Arc::new(OriginPolicy::new([
            data.clone(),
            management.clone(),
        ])))
        .authentication(Authentication::oauth(token.clone(), true))
        .build()
        .unwrap();
    (
        ModelSecurityClient::new(client, data, management).unwrap(),
        wire,
        token,
    )
}
fn body<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).unwrap()
}
fn scan() -> Value {
    json!({"uuid":ID,"tsg_id":"123456789","created_at":"created","updated_at":"updated","model_uri":"s3://fixture/model","owner":"owner","scan_origin":"MODEL_SECURITY_SDK","security_group_uuid":OTHER,"security_group_name":"group","eval_outcome":"FUTURE_OUTCOME","source_type":"FUTURE_SOURCE","future":{"keep":true}})
}
fn model() -> Value {
    json!({"uuid":ID,"tsg_id":"123456789","created_at":"created","updated_at":"updated","name":"fixture/model"})
}
fn version() -> Value {
    json!({"uuid":ID,"tsg_id":"123456789","created_at":"created","updated_at":"updated","revision":"main","model_uuid":OTHER})
}
fn file() -> Value {
    json!({"uuid":ID,"tsg_id":"123456789","created_at":"created","updated_at":"updated","path":"/model.bin","parent_path":"/","type":"FILE","result":"FUTURE_RESULT","model_version_uuid":OTHER})
}
fn evaluation() -> Value {
    json!({"uuid":ID,"tsg_id":"123456789","created_at":"created","updated_at":"updated","result":"FUTURE_RESULT","violation_count":0,"rule_instance_uuid":OTHER,"scan_uuid":OTHER,"rule_name":"rule","rule_description":"description","rule_instance_state":"FUTURE_STATE"})
}
fn violation() -> Value {
    json!({"uuid":ID,"tsg_id":"123456789","created_at":"created","updated_at":"updated","description":"violation","rule_instance_uuid":OTHER,"rule_name":"rule","rule_description":"description","rule_instance_state":"FUTURE_STATE"})
}
fn group() -> Value {
    json!({"uuid":ID,"tsg_id":"123456789","created_at":"created","updated_at":"updated","name":"group","description":"description","source_type":"FUTURE_SOURCE","state":"FUTURE_STATE","is_tombstone":false})
}
fn rule() -> Value {
    json!({"uuid":ID,"name":"rule","description":"description","rule_type":"FUTURE_TYPE","compatible_sources":[],"default_state":"FUTURE_STATE","remediation":{"description":"fix","steps":[],"url":"https://example.test/fix"},"editable_fields":[],"constant_values":{},"default_values":{}})
}
fn instance() -> Value {
    json!({"uuid":ID,"tsg_id":"123456789","security_group_uuid":OTHER,"state":"FUTURE_STATE"})
}
fn custom_rule() -> Value {
    json!({"uuid":ID,"tsg_id":"123456789","name":"custom","description":"description","compatible_sources":["LOCAL"],"violation_message":"violation","remediation_message":null,"is_archived":false,"created_by":null,"updated_by":null})
}
fn scan_body() -> Value {
    json!({"model_uri":"s3://fixture/model","security_group_uuid":OTHER})
}
fn labels_body() -> Value {
    json!({"labels":[{"key":"env","value":"prod"}]})
}
fn group_body() -> Value {
    json!({"name":"group","source_type":"LOCAL"})
}
fn custom_body() -> Value {
    json!({"name":"custom","compatible_sources":["LOCAL"],"condition":{"type":"label","key":"env","operator":"exists"},"violation_message":"violation"})
}
fn assignment_body() -> Value {
    json!({"assignments":[{"security_group_uuid":OTHER,"state":"BLOCKING"}]})
}
const WALK: &[(&str, &str)] = &[("skip", "0"), ("limit", "50")];
#[tokio::test]
async fn all_fourteen_scan_methods_match_independent_data_plane_contracts() {
    let (client, wire, _) = clients();
    let scans = client.scans();
    wire.expect(
        Plane::Data,
        Method::POST,
        "/v1/scans",
        &[],
        Some(scan_body()),
        Reply::Json(200, scan()),
    );
    assert_eq!(
        serde_json::to_value(scans.create(&body(scan_body()), None).await.unwrap()).unwrap(),
        scan()
    );
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"scans":[]})),
    );
    scans
        .list(&ModelSecurityScanListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        WALK,
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":1},"scans":[scan()]}),
        ),
    );
    assert!(
        scans
            .list_all(ModelSecurityScanListAllOptions::default(), None)
            .await
            .unwrap()
            .is_complete()
    );
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/scans/{ID}"),
        &[],
        None,
        Reply::Json(200, scan()),
    );
    scans.get(ID, None).await.unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/scans/{ID}/evaluations"),
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"evaluations":[evaluation()]})),
    );
    scans
        .get_evaluations(ID, &ModelSecurityEvaluationListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/scans/{ID}/files"),
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"files":[file()]})),
    );
    scans
        .get_files(ID, &ModelSecurityFileListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::POST,
        &format!("/v1/scans/{ID}/labels"),
        &[],
        Some(labels_body()),
        Reply::Empty,
    );
    scans
        .add_labels(ID, &body(labels_body()), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::PUT,
        &format!("/v1/scans/{ID}/labels"),
        &[],
        Some(labels_body()),
        Reply::Empty,
    );
    scans
        .set_labels(ID, &body(labels_body()), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::DELETE,
        &format!("/v1/scans/{ID}/labels"),
        &[("keys", "env"), ("keys", "owner")],
        None,
        Reply::Empty,
    );
    scans
        .delete_labels(ID, &["env".into(), "owner".into()], None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/scans/{ID}/rule-violations"),
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"violations":[violation()]})),
    );
    scans
        .get_violations(ID, &ModelSecurityViolationListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans/label-keys",
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"keys":["env"]})),
    );
    scans
        .get_label_keys(&ModelSecurityLabelListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans/label-keys/private%20label/values",
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"values":["prod"]})),
    );
    scans
        .get_label_values(
            "private label",
            &ModelSecurityLabelListOptions::default(),
            None,
        )
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/evaluations/{ID}"),
        &[],
        None,
        Reply::Json(200, evaluation()),
    );
    scans.get_evaluation(ID, None).await.unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/violations/{ID}"),
        &[],
        None,
        Reply::Json(200, violation()),
    );
    scans.get_violation(ID, None).await.unwrap();
    wire.done();
}
#[tokio::test]
async fn all_eight_model_methods_use_distinct_model_and_version_paths() {
    let (client, wire, _) = clients();
    let models = client.models();
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/models",
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"models":[]})),
    );
    models
        .list_models(&ModelSecurityModelListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/models",
        WALK,
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":1},"models":[model()]}),
        ),
    );
    assert!(
        models
            .list_all_models(ModelSecurityModelListAllOptions::default(), None)
            .await
            .unwrap()
            .is_complete()
    );
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/models/{ID}"),
        &[],
        None,
        Reply::Json(200, model()),
    );
    models.get_model(ID, None).await.unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/models/{ID}/model-versions"),
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"model_versions":[version()]})),
    );
    models
        .list_model_versions(ID, &ModelSecurityModelVersionListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/models/{ID}/model-versions"),
        WALK,
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":1},"model_versions":[version()]}),
        ),
    );
    assert!(
        models
            .list_all_model_versions(ID, ModelSecurityModelVersionListAllOptions::default(), None)
            .await
            .unwrap()
            .is_complete()
    );
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/model-versions/{ID}"),
        &[],
        None,
        Reply::Json(200, version()),
    );
    models.get_model_version(ID, None).await.unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/model-versions/{ID}/files"),
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"files":[file()]})),
    );
    models
        .list_model_version_files(
            ID,
            &ModelSecurityModelVersionFileListOptions::default(),
            None,
        )
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/model-versions/{ID}/files"),
        WALK,
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":1},"files":[file()]}),
        ),
    );
    assert!(
        models
            .list_all_model_version_files(
                ID,
                ModelSecurityModelVersionFileListAllOptions::default(),
                None
            )
            .await
            .unwrap()
            .is_complete()
    );
    wire.done();
}
#[tokio::test]
async fn all_ten_group_methods_preserve_put_patch_states_and_snapshot_route() {
    let (client, wire, _) = clients();
    let groups = client.security_groups();
    wire.expect(
        Plane::Management,
        Method::POST,
        "/v1/security-groups",
        &[],
        Some(json!({"name":"group","source_type":"LOCAL","description":""})),
        Reply::Json(200, group()),
    );
    groups.create(&body(group_body()), None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/security-groups",
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"security_groups":[]})),
    );
    groups
        .list(&ModelSecurityGroupListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/security-groups",
        WALK,
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":1},"security_groups":[group()]}),
        ),
    );
    assert!(
        groups
            .list_all(ModelSecurityGroupListAllOptions::default(), None)
            .await
            .unwrap()
            .is_complete()
    );
    wire.expect(
        Plane::Management,
        Method::GET,
        &format!("/v1/security-groups/{ID}"),
        &[],
        None,
        Reply::Json(200, group()),
    );
    groups.get(ID, None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::PUT,
        &format!("/v1/security-groups/{ID}"),
        &[],
        Some(json!({"description":null})),
        Reply::Json(200, group()),
    );
    groups
        .update(ID, &body(json!({"description":null})), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Management,
        Method::DELETE,
        &format!("/v1/security-groups/{ID}"),
        &[],
        None,
        Reply::Empty,
    );
    groups.delete(ID, None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::GET,
        &format!("/v1/security-groups/{ID}/rule-instances"),
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"rule_instances":[instance()]})),
    );
    groups
        .list_rule_instances(ID, &ModelSecurityRuleInstanceListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Management,
        Method::GET,
        &format!("/v1/security-groups/{ID}/rule-instances/{OTHER}"),
        &[],
        None,
        Reply::Json(200, instance()),
    );
    groups.get_rule_instance(ID, OTHER, None).await.unwrap();
    let update = json!({"security_group_uuid":ID,"state":null,"field_values":null});
    wire.expect(
        Plane::Management,
        Method::PUT,
        &format!("/v1/security-groups/{ID}/rule-instances/{OTHER}"),
        &[],
        Some(update.clone()),
        Reply::Json(200, instance()),
    );
    groups
        .update_rule_instance(ID, OTHER, &body(update), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Management,
        Method::GET,
        &format!("/v1/security-groups/{ID}/rule-instances/versions"),
        &[],
        None,
        Reply::Json(200, json!({"versions":[],"pagination":{}})),
    );
    groups
        .list_rule_instance_versions(ID, &SnapshotVersionListOptions::default(), None)
        .await
        .unwrap();
    wire.done();
}
#[tokio::test]
async fn all_four_template_rule_methods_preserve_global_version_history() {
    let (client, wire, _) = clients();
    let rules = client.security_rules();
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/security-rules",
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"rules":[rule()]})),
    );
    rules
        .list(&ModelSecurityRuleListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/security-rules",
        WALK,
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":1},"rules":[rule()]}),
        ),
    );
    assert!(
        rules
            .list_all(ModelSecurityRuleListAllOptions::default(), None)
            .await
            .unwrap()
            .is_complete()
    );
    wire.expect(
        Plane::Management,
        Method::GET,
        &format!("/v1/security-rules/{ID}"),
        &[],
        None,
        Reply::Json(200, rule()),
    );
    rules.get(ID, None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/security-rules/versions",
        &[],
        None,
        Reply::Json(200, json!({"versions":[],"pagination":{}})),
    );
    rules
        .list_versions(&SnapshotVersionListOptions::default(), None)
        .await
        .unwrap();
    wire.done();
}
#[tokio::test]
async fn all_twelve_custom_rule_methods_preserve_strict_requests_and_207_results() {
    let (client, wire, _) = clients();
    let rules = client.custom_rules();
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/custom-rules",
        &[],
        None,
        Reply::Json(200, json!({"pagination":{},"custom_rules":[]})),
    );
    rules
        .list(&CustomRuleListOptions::default(), None)
        .await
        .unwrap();
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/custom-rules",
        &[("limit", "50"), ("skip", "0")],
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":1},"custom_rules":[custom_rule()]}),
        ),
    );
    assert!(
        rules
            .list_all(CustomRuleListAllOptions::default(), None)
            .await
            .unwrap()
            .is_complete()
    );
    wire.expect(
        Plane::Management,
        Method::POST,
        "/v1/custom-rules",
        &[],
        Some(custom_body()),
        Reply::Json(200, custom_rule()),
    );
    rules.create(&body(custom_body()), None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::GET,
        &format!("/v1/custom-rules/{ID}"),
        &[],
        None,
        Reply::Json(200, custom_rule()),
    );
    rules.get(ID, None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::PUT,
        &format!("/v1/custom-rules/{ID}"),
        &[],
        Some(json!({"name":"renamed","remediation_message":null})),
        Reply::Json(200, custom_rule()),
    );
    rules
        .update(
            ID,
            &body(json!({"name":"renamed","remediation_message":null})),
            None,
        )
        .await
        .unwrap();
    wire.expect(
        Plane::Management,
        Method::POST,
        &format!("/v1/custom-rules/{ID}/archive"),
        &[],
        None,
        Reply::Empty,
    );
    rules.archive(ID, None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::POST,
        &format!("/v1/custom-rules/{ID}/unarchive"),
        &[],
        None,
        Reply::Empty,
    );
    rules.unarchive(ID, None).await.unwrap();
    wire.expect(Plane::Management,Method::GET,&format!("/v1/custom-rules/{ID}/security-groups"),&[],None,Reply::Json(200,json!({"pagination":{},"security_groups":[{"uuid":OTHER,"name":"group","source_type":"LOCAL","rule_state":"BLOCKING"}]})));
    rules
        .list_security_groups(ID, &CustomRuleSecurityGroupListOptions::default(), None)
        .await
        .unwrap();
    let outcomes = json!({"results":[{"security_group_uuid":OTHER,"status":"SUCCESS","rule_instance_uuid":ID},{"security_group_uuid":ID,"status":"FAILURE","error":"private server detail"}]});
    wire.expect(
        Plane::Management,
        Method::POST,
        &format!("/v1/custom-rules/{ID}/security-groups"),
        &[],
        Some(assignment_body()),
        Reply::Json(207, outcomes.clone()),
    );
    let assigned = rules
        .assign_security_groups(ID, &body(assignment_body()), None)
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(&assigned).unwrap(), outcomes);
    assert!(!format!("{assigned:?}").contains("private server detail"));
    wire.expect(
        Plane::Management,
        Method::DELETE,
        &format!("/v1/custom-rules/{ID}/security-groups/{OTHER}"),
        &[],
        None,
        Reply::Empty,
    );
    rules.remove_assignment(ID, OTHER, None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/custom-rules/versions",
        &[],
        None,
        Reply::Json(200, json!({"versions":[],"pagination":{}})),
    );
    rules
        .list_versions(&SnapshotVersionListOptions::default(), None)
        .await
        .unwrap();
    let mut all = SnapshotVersionListAllOptions::default();
    all.limit = Some(2);
    all.next_token = Some("initial".into());
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/custom-rules/versions",
        &[("limit", "2"), ("next_token", "initial")],
        None,
        Reply::Json(
            200,
            json!({"versions":[{"generation":1,"timestamp":"one"}],"pagination":{"next_token":""}}),
        ),
    );
    wire.expect(Plane::Management,Method::GET,"/v1/custom-rules/versions",&[("limit","2"),("next_token","")],None,Reply::Json(200,json!({"versions":[{"generation":2,"timestamp":"two"}],"pagination":{"next_token":null}})));
    let versions = rules.list_all_versions(all, None).await.unwrap();
    assert!(versions.is_complete());
    assert_eq!(versions.items.len(), 2);
    wire.done();
}
#[tokio::test]
async fn pypi_credentials_are_returned_faithfully_without_following_or_formatting_the_url() {
    let (client, wire, _) = clients();
    let url = "https://_token:synthetic-private-credential@artifact.example/pypi";
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/pypi/authenticate",
        &[],
        None,
        Reply::Json(200, json!({"url":url,"expires_at":"2030-01-01T00:00:00Z"})),
    );
    let result = client.get_pypi_auth(None).await.unwrap();
    assert_eq!(result.url, url);
    assert!(!format!("{result:?}").contains("synthetic-private-credential"));
    assert_eq!(wire.calls.load(Ordering::SeqCst), 1);
    wire.done();
}
#[tokio::test]
async fn all_listing_filters_preserve_exact_order_repeated_arrays_and_zero_false() {
    let (client, wire, _) = clients();
    let mut scan_query = ModelSecurityScanListOptions::default();
    scan_query.skip = Some(0);
    scan_query.limit = Some(3);
    scan_query.search = Some("general".into());
    scan_query.is_background_refresh = Some(false);
    scan_query.model_version_uuid = Some(OTHER.into());
    scan_query.sort_by = Some("created_at".into());
    scan_query.sort_order = Some("desc".into());
    scan_query.search_query = Some("needle".into());
    scan_query.eval_outcomes = Some(vec!["ALLOWED".into(), "BLOCKED".into(), "ALLOWED".into()]);
    scan_query.source_types = Some(vec!["S3".into(), "LOCAL".into()]);
    scan_query.security_group_uuid = Some(ID.into());
    scan_query.start_time = Some("start".into());
    scan_query.end_time = Some("end".into());
    scan_query.labels_query = Some("env=prod".into());
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[
            ("skip", "0"),
            ("limit", "3"),
            ("search", "general"),
            ("isBackgroundRefresh", "false"),
            ("model_version_uuid", OTHER),
            ("sort_by", "created_at"),
            ("sort_order", "desc"),
            ("search_query", "needle"),
            ("eval_outcomes", "ALLOWED"),
            ("eval_outcomes", "BLOCKED"),
            ("eval_outcomes", "ALLOWED"),
            ("source_types", "S3"),
            ("source_types", "LOCAL"),
            ("security_group_uuid", ID),
            ("start_time", "start"),
            ("end_time", "end"),
            ("labels_query", "env=prod"),
        ],
        None,
        Reply::Json(200, json!({"pagination":{},"scans":[]})),
    );
    client.scans().list(&scan_query, None).await.unwrap();
    let mut query = ModelSecurityEvaluationListOptions::default();
    query.skip = Some(0);
    query.limit = Some(3);
    query.search = Some("general".into());
    query.sort_field = Some("created_at".into());
    query.sort_order = Some("desc".into());
    query.result = Some("FAILED".into());
    query.rule_instance_uuid = Some(OTHER.into());
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/scans/{ID}/evaluations"),
        &[
            ("skip", "0"),
            ("limit", "3"),
            ("search", "general"),
            ("sort_field", "created_at"),
            ("sort_order", "desc"),
            ("result", "FAILED"),
            ("rule_instance_uuid", OTHER),
        ],
        None,
        Reply::Json(200, json!({"pagination":{},"evaluations":[]})),
    );
    client
        .scans()
        .get_evaluations(ID, &query, None)
        .await
        .unwrap();
    let mut query = ModelSecurityFileListOptions::default();
    query.skip = Some(0);
    query.limit = Some(3);
    query.search = Some("general".into());
    query.recursive = Some(false);
    query.sort_field = Some("path".into());
    query.sort_dir = Some("asc".into());
    query.type_ = Some("FILE".into());
    query.result = Some("SUCCESS".into());
    query.query_path = Some("/root/private name".into());
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/scans/{ID}/files"),
        &[
            ("skip", "0"),
            ("limit", "3"),
            ("search", "general"),
            ("recursive", "false"),
            ("sort_field", "path"),
            ("sort_dir", "asc"),
            ("type", "FILE"),
            ("result", "SUCCESS"),
            ("query_path", "/root/private name"),
        ],
        None,
        Reply::Json(200, json!({"pagination":{},"files":[]})),
    );
    client.scans().get_files(ID, &query, None).await.unwrap();
    let mut query = ModelSecurityModelListOptions::default();
    query.skip = Some(0);
    query.limit = Some(3);
    query.search = Some("general".into());
    query.is_background_refresh = Some(false);
    query.search_query = Some("specific".into());
    query.sort_field = Some("name".into());
    query.sort_order = Some("asc".into());
    query.latest_version_outcomes = Some(vec!["ALLOWED".into(), "BLOCKED".into()]);
    query.latest_version_formats = Some(vec!["pickle".into(), "onnx".into()]);
    query.latest_version_source_types = Some(vec!["S3".into(), "LOCAL".into()]);
    query.latest_version_scan_time_before = Some("before".into());
    query.start_time = Some("start".into());
    query.end_time = Some("end".into());
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/models",
        &[
            ("skip", "0"),
            ("limit", "3"),
            ("search", "general"),
            ("isBackgroundRefresh", "false"),
            ("search_query", "specific"),
            ("sort_field", "name"),
            ("sort_order", "asc"),
            ("latest_version_outcomes", "ALLOWED"),
            ("latest_version_outcomes", "BLOCKED"),
            ("latest_version_formats", "pickle"),
            ("latest_version_formats", "onnx"),
            ("latest_version_source_types", "S3"),
            ("latest_version_source_types", "LOCAL"),
            ("latest_version_scan_time_before", "before"),
            ("start_time", "start"),
            ("end_time", "end"),
        ],
        None,
        Reply::Json(200, json!({"pagination":{},"models":[]})),
    );
    client.models().list_models(&query, None).await.unwrap();
    let mut query = ModelSecurityModelVersionListOptions::default();
    query.skip = Some(0);
    query.limit = Some(2);
    query.search = Some("general".into());
    query.sort_order = Some("desc".into());
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/models/{ID}/model-versions"),
        &[
            ("skip", "0"),
            ("limit", "2"),
            ("search", "general"),
            ("sort_order", "desc"),
        ],
        None,
        Reply::Json(200, json!({"pagination":{},"model_versions":[]})),
    );
    client
        .models()
        .list_model_versions(ID, &query, None)
        .await
        .unwrap();
    let mut query = ModelSecurityLabelListOptions::default();
    query.skip = Some(0);
    query.limit = Some(2);
    query.search = Some("general".into());
    wire.expect(
        Plane::Data,
        Method::GET,
        &format!("/v1/model-versions/{ID}/files"),
        &[("skip", "0"), ("limit", "2"), ("search", "general")],
        None,
        Reply::Json(200, json!({"pagination":{},"files":[]})),
    );
    client
        .models()
        .list_model_version_files(ID, &query, None)
        .await
        .unwrap();
    let mut query = ModelSecurityGroupListOptions::default();
    query.skip = Some(0);
    query.limit = Some(3);
    query.search = Some("general".into());
    query.sort_field = Some("name".into());
    query.sort_dir = Some("asc".into());
    query.source_types = Some(vec!["LOCAL".into(), "S3".into()]);
    query.search_query = Some("specific".into());
    query.enabled_rules = Some(vec![ID.into(), OTHER.into()]);
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/security-groups",
        &[
            ("skip", "0"),
            ("limit", "3"),
            ("search", "general"),
            ("sort_field", "name"),
            ("sort_dir", "asc"),
            ("source_types", "LOCAL"),
            ("source_types", "S3"),
            ("search_query", "specific"),
            ("enabled_rules", ID),
            ("enabled_rules", OTHER),
        ],
        None,
        Reply::Json(200, json!({"pagination":{},"security_groups":[]})),
    );
    client.security_groups().list(&query, None).await.unwrap();
    let mut query = ModelSecurityRuleInstanceListOptions::default();
    query.skip = Some(0);
    query.limit = Some(2);
    query.search = Some("general".into());
    query.is_custom = Some(false);
    query.generation = Some(0);
    query.security_rule_uuid = Some(OTHER.into());
    query.state = Some("ALLOWING".into());
    wire.expect(
        Plane::Management,
        Method::GET,
        &format!("/v1/security-groups/{ID}/rule-instances"),
        &[
            ("skip", "0"),
            ("limit", "2"),
            ("search", "general"),
            ("is_custom", "false"),
            ("generation", "0"),
            ("security_rule_uuid", OTHER),
            ("state", "ALLOWING"),
        ],
        None,
        Reply::Json(200, json!({"pagination":{},"rule_instances":[]})),
    );
    client
        .security_groups()
        .list_rule_instances(ID, &query, None)
        .await
        .unwrap();
    let mut query = ModelSecurityRuleListOptions::default();
    query.skip = Some(0);
    query.limit = Some(2);
    query.search = Some("general".into());
    query.source_type = Some("LOCAL".into());
    query.search_query = Some("specific".into());
    query.generation = Some(0);
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/security-rules",
        &[
            ("skip", "0"),
            ("limit", "2"),
            ("search", "general"),
            ("source_type", "LOCAL"),
            ("search_query", "specific"),
            ("generation", "0"),
        ],
        None,
        Reply::Json(200, json!({"pagination":{},"rules":[]})),
    );
    client.security_rules().list(&query, None).await.unwrap();
    let mut query = CustomRuleListOptions::default();
    query.limit = Some(5);
    query.skip = Some(0);
    query.sort_field = Some("updated_at".into());
    query.sort_dir = Some("desc".into());
    query.is_archived = Some(false);
    query.search_query = Some("specific".into());
    query.source_types = Some(vec!["LOCAL".into(), "S3".into()]);
    query.generation = Some(0);
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/custom-rules",
        &[
            ("limit", "5"),
            ("skip", "0"),
            ("sort_field", "updated_at"),
            ("sort_dir", "desc"),
            ("is_archived", "false"),
            ("search_query", "specific"),
            ("source_types", "LOCAL"),
            ("source_types", "S3"),
            ("generation", "0"),
        ],
        None,
        Reply::Json(200, json!({"pagination":{},"custom_rules":[]})),
    );
    client.custom_rules().list(&query, None).await.unwrap();
    let mut query = CustomRuleSecurityGroupListOptions::default();
    query.skip = Some(0);
    query.limit = Some(2);
    wire.expect(
        Plane::Management,
        Method::GET,
        &format!("/v1/custom-rules/{ID}/security-groups"),
        &[("skip", "0"), ("limit", "2")],
        None,
        Reply::Json(200, json!({"pagination":{},"security_groups":[]})),
    );
    client
        .custom_rules()
        .list_security_groups(ID, &query, None)
        .await
        .unwrap();
    wire.done();
}
#[tokio::test]
async fn all_fourteen_mutations_preserve_unknown_outcomes_and_never_replay() {
    let (client, wire, _) = clients();
    let scans = client.scans();
    let groups = client.security_groups();
    let custom = client.custom_rules();
    macro_rules! ambiguous {
        ($plane:expr,$method:expr,$path:expr,$query:expr,$body:expr,$call:expr) => {{
            wire.expect($plane, $method, $path, $query, $body, Reply::Failure);
            let error = $call.await.unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Transport);
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        }};
    }
    ambiguous!(
        Plane::Data,
        Method::POST,
        "/v1/scans",
        &[],
        Some(scan_body()),
        scans.create(&body(scan_body()), None)
    );
    ambiguous!(
        Plane::Data,
        Method::POST,
        &format!("/v1/scans/{ID}/labels"),
        &[],
        Some(labels_body()),
        scans.add_labels(ID, &body(labels_body()), None)
    );
    ambiguous!(
        Plane::Data,
        Method::PUT,
        &format!("/v1/scans/{ID}/labels"),
        &[],
        Some(labels_body()),
        scans.set_labels(ID, &body(labels_body()), None)
    );
    ambiguous!(
        Plane::Data,
        Method::DELETE,
        &format!("/v1/scans/{ID}/labels"),
        &[("keys", "env")],
        None,
        scans.delete_labels(ID, &["env".into()], None)
    );
    ambiguous!(
        Plane::Management,
        Method::POST,
        "/v1/security-groups",
        &[],
        Some(json!({"name":"group","source_type":"LOCAL","description":""})),
        groups.create(&body(group_body()), None)
    );
    ambiguous!(
        Plane::Management,
        Method::PUT,
        &format!("/v1/security-groups/{ID}"),
        &[],
        Some(json!({"description":null})),
        groups.update(ID, &body(json!({"description":null})), None)
    );
    ambiguous!(
        Plane::Management,
        Method::DELETE,
        &format!("/v1/security-groups/{ID}"),
        &[],
        None,
        groups.delete(ID, None)
    );
    let update = json!({"security_group_uuid":ID});
    ambiguous!(
        Plane::Management,
        Method::PUT,
        &format!("/v1/security-groups/{ID}/rule-instances/{OTHER}"),
        &[],
        Some(update.clone()),
        groups.update_rule_instance(ID, OTHER, &body(update), None)
    );
    ambiguous!(
        Plane::Management,
        Method::POST,
        "/v1/custom-rules",
        &[],
        Some(custom_body()),
        custom.create(&body(custom_body()), None)
    );
    ambiguous!(
        Plane::Management,
        Method::PUT,
        &format!("/v1/custom-rules/{ID}"),
        &[],
        Some(json!({"description":null})),
        custom.update(ID, &body(json!({"description":null})), None)
    );
    ambiguous!(
        Plane::Management,
        Method::POST,
        &format!("/v1/custom-rules/{ID}/archive"),
        &[],
        None,
        custom.archive(ID, None)
    );
    ambiguous!(
        Plane::Management,
        Method::POST,
        &format!("/v1/custom-rules/{ID}/unarchive"),
        &[],
        None,
        custom.unarchive(ID, None)
    );
    ambiguous!(
        Plane::Management,
        Method::POST,
        &format!("/v1/custom-rules/{ID}/security-groups"),
        &[],
        Some(assignment_body()),
        custom.assign_security_groups(ID, &body(assignment_body()), None)
    );
    ambiguous!(
        Plane::Management,
        Method::DELETE,
        &format!("/v1/custom-rules/{ID}/security-groups/{OTHER}"),
        &[],
        None,
        custom.remove_assignment(ID, OTHER, None)
    );
    assert_eq!(wire.calls.load(Ordering::SeqCst), 14);
    wire.done();
}
#[tokio::test]
async fn capped_scan_totals_are_latched_until_short_or_empty_end_pages() {
    let (client, wire, _) = clients();
    let mut options = ModelSecurityScanListAllOptions::default();
    options.limit = Some(2);
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[("skip", "0"), ("limit", "2")],
        None,
        Reply::Json(
            200,
            json!({"count_capped":true,"pagination":{"total_items":2},"scans":[scan(),scan()]}),
        ),
    );
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[("skip", "2"), ("limit", "2")],
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":2},"scans":[scan(),scan()]}),
        ),
    );
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[("skip", "4"), ("limit", "2")],
        None,
        Reply::Json(
            200,
            json!({"count_capped":false,"pagination":{"total_items":2},"scans":[scan()]}),
        ),
    );
    let result = client.scans().list_all(options, None).await.unwrap();
    assert!(result.is_complete());
    assert_eq!(result.items.len(), 5);
    let mut options = ModelSecurityScanListAllOptions::default();
    options.limit = Some(2);
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[("skip", "0"), ("limit", "2")],
        None,
        Reply::Json(
            200,
            json!({"count_capped":true,"pagination":{"total_items":2},"scans":[scan(),scan()]}),
        ),
    );
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[("skip", "2"), ("limit", "2")],
        None,
        Reply::Json(200, json!({"pagination":{},"scans":[]})),
    );
    assert!(
        client
            .scans()
            .list_all(options, None)
            .await
            .unwrap()
            .is_complete()
    );
    wire.done();
}
#[tokio::test]
async fn mid_page_capped_resume_survives_serialized_snapshot_and_missing_replayed_flag() {
    let (client, wire, _) = clients();
    let mut options = ModelSecurityScanListAllOptions::default();
    options.limit = Some(2);
    options.collect.max_items = 1;
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[("skip", "0"), ("limit", "2")],
        None,
        Reply::Json(
            200,
            json!({"count_capped":true,"pagination":{"total_items":2},"scans":[scan(),scan()]}),
        ),
    );
    let first = client.scans().list_all(options, None).await.unwrap();
    assert_eq!(first.completion, Completion::Truncated(Truncation::Items));
    let saved = first.resume.unwrap();
    assert_eq!(saved.item_offset, 1);
    assert!(saved.cursor.count_capped());
    let snapshot = serde_json::to_string(&saved.cursor).unwrap();
    let restored: ScanCursor = serde_json::from_str(&snapshot).unwrap();
    assert!(restored.count_capped());
    assert_eq!(restored.skip(), 0);
    assert!(
        serde_json::from_value::<ScanCursor>(json!({"skip":u64::MAX,"count_capped":true})).is_err()
    );
    let mut query = ModelSecurityScanListOptions::default();
    query.limit = Some(2);
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[("skip", "0"), ("limit", "2")],
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":2},"scans":[scan(),scan()]}),
        ),
    );
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[("skip", "2"), ("limit", "2")],
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":2},"scans":[scan()]}),
        ),
    );
    let rest = client
        .scans()
        .pages(query, 10, None)
        .unwrap()
        .items()
        .resume_from(Resume {
            cursor: restored,
            item_offset: saved.item_offset,
        })
        .unwrap()
        .collect(scm_sdk::pagination::CollectOptions::default())
        .await
        .unwrap();
    assert!(rest.is_complete());
    assert_eq!(rest.items.len(), 2);
    wire.done();
}
#[tokio::test]
async fn invalid_identifiers_queries_strict_bodies_and_oversized_inputs_are_pre_auth() {
    let (client, wire, token) = clients();
    let scans = client.scans();
    let groups = client.security_groups();
    let custom = client.custom_rules();
    assert!(scans.get("bad", None).await.is_err());
    assert!(
        scans
            .get_evaluations("bad", &ModelSecurityEvaluationListOptions::default(), None)
            .await
            .is_err()
    );
    assert!(scans.get_evaluation("bad", None).await.is_err());
    assert!(scans.get_violation("bad", None).await.is_err());
    assert!(scans.delete_labels("bad", &[], None).await.is_err());
    assert!(
        scans
            .get_label_values("../escape", &ModelSecurityLabelListOptions::default(), None)
            .await
            .is_err()
    );
    assert!(client.models().get_model("bad", None).await.is_err());
    assert!(
        client
            .models()
            .get_model_version("bad", None)
            .await
            .is_err()
    );
    assert!(groups.get_rule_instance(ID, "bad", None).await.is_err());
    assert!(custom.remove_assignment(ID, "bad", None).await.is_err());
    let mut query = CustomRuleListOptions::default();
    query.limit = Some(101);
    assert!(custom.list(&query, None).await.is_err());
    query.limit = Some(1);
    query.generation = Some(u64::MAX);
    assert!(custom.list(&query, None).await.is_err());
    let mut query = SnapshotVersionListOptions::default();
    query.limit = Some(0);
    assert!(custom.list_versions(&query, None).await.is_err());
    let mut query = ModelSecurityModelListOptions::default();
    query.search = Some("private-query".repeat(2048));
    assert_eq!(
        client
            .models()
            .list_models(&query, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    let mut query = ModelSecurityScanListOptions::default();
    query.eval_outcomes = Some(vec!["x".into(); 257]);
    assert_eq!(
        scans.list(&query, None).await.unwrap_err().kind(),
        ErrorKind::Limit
    );
    let mut invalid = scan_body();
    invalid["security_group_uuid"] = json!("bad");
    assert!(scans.create(&body(invalid), None).await.is_err());
    assert!(
        scans
            .add_labels(ID, &body(json!({"labels":[]})), None)
            .await
            .is_err()
    );
    let mut invalid = group_body();
    invalid["source_type"] = json!("FUTURE_REQUEST_SOURCE");
    assert!(groups.create(&body(invalid), None).await.is_err());
    let mut invalid: CustomRuleCreateRequest = body(custom_body());
    invalid
        .extensions
        .insert("unexpected".into(), JsonValue::new(json!("private")));
    assert!(custom.create(&invalid, None).await.is_err());
    let mut invalid = custom_body();
    invalid["condition"] =
        json!({"operator":"and","conditions":[{"type":"label","key":"env","operator":"exists"}]});
    assert!(custom.create(&body(invalid), None).await.is_err());
    assert!(
        custom
            .assign_security_groups(ID, &body(json!({"assignments":[]})), None)
            .await
            .is_err()
    );
    let invalid =
        json!({"assignments":vec![json!({"security_group_uuid":OTHER,"state":"BLOCKING"});51]});
    assert!(
        custom
            .assign_security_groups(ID, &body(invalid), None)
            .await
            .is_err()
    );
    let mut update: ModelSecurityGroupUpdateRequest = body(json!({}));
    update.name = Patch::Value(String::new());
    assert!(groups.update(ID, &update, None).await.is_err());
    assert_eq!(token.calls.load(Ordering::SeqCst), 0);
    assert_eq!(wire.calls.load(Ordering::SeqCst), 0);
    wire.done();
}
#[tokio::test]
async fn void_mutations_ignore_non_json_success_bodies_while_required_models_reject_empty() {
    let (client, wire, _) = clients();
    wire.expect(
        Plane::Data,
        Method::DELETE,
        &format!("/v1/scans/{ID}/labels"),
        &[("keys", "env")],
        None,
        Reply::Text("private success"),
    );
    client
        .scans()
        .delete_labels(ID, &["env".into()], None)
        .await
        .unwrap();
    wire.expect(
        Plane::Management,
        Method::DELETE,
        &format!("/v1/security-groups/{ID}"),
        &[],
        None,
        Reply::Text("private success"),
    );
    client.security_groups().delete(ID, None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::POST,
        &format!("/v1/custom-rules/{ID}/archive"),
        &[],
        None,
        Reply::Text("private success"),
    );
    client.custom_rules().archive(ID, None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::POST,
        &format!("/v1/custom-rules/{ID}/unarchive"),
        &[],
        None,
        Reply::Text("private success"),
    );
    client.custom_rules().unarchive(ID, None).await.unwrap();
    wire.expect(
        Plane::Management,
        Method::DELETE,
        &format!("/v1/custom-rules/{ID}/security-groups/{OTHER}"),
        &[],
        None,
        Reply::Text("private success"),
    );
    client
        .custom_rules()
        .remove_assignment(ID, OTHER, None)
        .await
        .unwrap();
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/scans",
        &[],
        None,
        Reply::Empty,
    );
    assert_eq!(
        client
            .scans()
            .list(&ModelSecurityScanListOptions::default(), None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Decode
    );
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/custom-rules/versions",
        &[],
        None,
        Reply::Empty,
    );
    assert_eq!(
        client
            .custom_rules()
            .list_versions(&SnapshotVersionListOptions::default(), None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Decode
    );
    wire.done();
}
#[tokio::test]
async fn opaque_version_cursor_limits_keep_rows_and_resume_without_duplication() {
    let (client, wire, _) = clients();
    let mut limits = scm_sdk::pagination::CursorLimits::default();
    limits.max_bytes = 8;
    let token = "private-opaque-next-token";
    wire.expect(Plane::Management,Method::GET,"/v1/custom-rules/versions",&[],None,Reply::Json(200,json!({"versions":[{"generation":1,"timestamp":"one"}],"pagination":{"next_token":token}})));
    let failed = client
        .custom_rules()
        .version_pages(SnapshotVersionListOptions::default(), 10, None)
        .unwrap()
        .with_cursor_limits(limits)
        .unwrap()
        .collect(scm_sdk::pagination::CollectOptions::default())
        .await
        .unwrap_err();
    assert_eq!(failed.error().kind(), ErrorKind::Limit);
    assert_eq!(failed.partial().items.len(), 1);
    assert!(!format!("{failed:?}").contains(token));
    let saved = failed.partial().resume.clone().unwrap();
    assert_eq!(saved.cursor, None);
    assert_eq!(saved.item_offset, 1);
    wire.expect(Plane::Management,Method::GET,"/v1/custom-rules/versions",&[],None,Reply::Json(200,json!({"versions":[{"generation":1,"timestamp":"one"}],"pagination":{"next_token":token}})));
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/custom-rules/versions",
        &[("next_token", token)],
        None,
        Reply::Json(
            200,
            json!({"versions":[{"generation":2,"timestamp":"two"}],"pagination":{}}),
        ),
    );
    let resumed = client
        .custom_rules()
        .version_pages(SnapshotVersionListOptions::default(), 10, None)
        .unwrap()
        .items()
        .resume_from(saved)
        .unwrap()
        .collect(scm_sdk::pagination::CollectOptions::default())
        .await
        .unwrap();
    assert!(resumed.is_complete());
    assert_eq!(resumed.items.len(), 1);
    assert_eq!(resumed.items[0].generation.as_u64(), Some(2));
    wire.done();
}
#[tokio::test(start_paused = true)]
async fn version_walks_share_one_deadline_and_cancellation_including_buffered_rows() {
    let (client, wire, _) = clients();
    wire.expect(Plane::Management,Method::GET,"/v1/custom-rules/versions",&[],None,Reply::Json(200,json!({"versions":[{"generation":1,"timestamp":"one"}],"pagination":{"next_token":"next"}})));
    wire.fixtures.lock().unwrap().back_mut().unwrap().delay = Duration::from_millis(40);
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/custom-rules/versions",
        &[("next_token", "next")],
        None,
        Reply::Json(
            200,
            json!({"versions":[{"generation":2,"timestamp":"two"}],"pagination":{}}),
        ),
    );
    wire.fixtures.lock().unwrap().back_mut().unwrap().delay = Duration::from_millis(40);
    let options = RequestOptions::with_timeout(Duration::from_millis(60)).unwrap();
    let failed = client
        .custom_rules()
        .list_all_versions(SnapshotVersionListAllOptions::default(), Some(&options))
        .await
        .unwrap_err();
    assert_eq!(failed.error().kind(), ErrorKind::Deadline);
    assert_eq!(failed.partial().items.len(), 1);
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    wire.expect(Plane::Management,Method::GET,"/v1/custom-rules/versions",&[],None,Reply::Json(200,json!({"versions":[{"generation":1,"timestamp":"one"},{"generation":2,"timestamp":"two"}],"pagination":{}})));
    let mut items = client
        .custom_rules()
        .version_pages(SnapshotVersionListOptions::default(), 10, Some(&options))
        .unwrap()
        .items();
    assert!(items.next().await.unwrap().is_some());
    options.cancel();
    assert_eq!(items.next().await.unwrap_err().kind(), ErrorKind::Cancelled);
    assert_eq!(items.resume().unwrap().item_offset, 1);
    wire.done();
}
#[test]
fn constructors_enforce_oauth_and_destination_policy_and_options_are_redacted() {
    let (data, management) = endpoints();
    let wire = Arc::new(Wire::default());
    let client = Client::builder(data.clone())
        .transport(wire.clone())
        .build()
        .unwrap();
    assert!(ModelSecurityClient::new(client.clone(), data.clone(), management.clone()).is_err());
    assert!(ModelSecurityScansClient::new(client.clone()).is_err());
    assert!(ModelSecurityModelsClient::new(client.clone()).is_err());
    assert!(ModelSecurityGroupsClient::new(client.clone()).is_err());
    assert!(ModelSecurityRulesClient::new(client.clone()).is_err());
    assert!(ModelSecurityCustomRulesClient::new(client).is_err());
    let token = Token::new();
    let client = Client::builder(data.clone())
        .transport(wire.clone())
        .authentication(Authentication::oauth(token.clone(), true))
        .build()
        .unwrap();
    assert!(ModelSecurityClient::new(client, data, management).is_err());
    assert_eq!(token.calls.load(Ordering::SeqCst), 0);
    let mut query = ModelSecurityScanListOptions::default();
    query.search = Some("private query".into());
    let mut token_query = SnapshotVersionListOptions::default();
    token_query.next_token = Some("private cursor".into());
    assert!(!format!("{query:?} {token_query:?}").contains("private"));
    wire.done();
}
#[tokio::test]
async fn all_page_collections_expose_item_caps_and_resumable_positions() {
    let (client, wire, _) = clients();
    let mut options = ModelSecurityModelListAllOptions::default();
    options.collect.max_items = 1;
    wire.expect(
        Plane::Data,
        Method::GET,
        "/v1/models",
        WALK,
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":2},"models":[model(),model()]}),
        ),
    );
    let collection = client
        .models()
        .list_all_models(options, None)
        .await
        .unwrap();
    assert_eq!(
        collection.completion,
        Completion::Truncated(Truncation::Items)
    );
    let resume = collection.resume.unwrap();
    assert_eq!((resume.cursor, resume.item_offset), (0, 1));
    let mut options = CustomRuleListAllOptions::default();
    options.collect.max_items = 1;
    wire.expect(
        Plane::Management,
        Method::GET,
        "/v1/custom-rules",
        &[("limit", "50"), ("skip", "0")],
        None,
        Reply::Json(
            200,
            json!({"pagination":{"total_items":2},"custom_rules":[custom_rule(),custom_rule()]}),
        ),
    );
    assert!(
        !client
            .custom_rules()
            .list_all(options, None)
            .await
            .unwrap()
            .is_complete()
    );
    wire.done();
}
#[tokio::test]
async fn all_page_filters_survive_each_request_and_unknown_totals_use_actual_item_counts() {
    let (client, wire, _) = clients();
    let mut query = ModelSecurityScanListAllOptions::default();
    query.limit = Some(2);
    query.is_background_refresh = Some(false);
    query.eval_outcomes = Some(vec!["ALLOWED".into(), "BLOCKED".into()]);
    for (skip, items) in [("0", vec![scan(), scan()]), ("2", vec![])] {
        wire.expect(
            Plane::Data,
            Method::GET,
            "/v1/scans",
            &[
                ("skip", skip),
                ("limit", "2"),
                ("isBackgroundRefresh", "false"),
                ("eval_outcomes", "ALLOWED"),
                ("eval_outcomes", "BLOCKED"),
            ],
            None,
            Reply::Json(
                200,
                json!({"pagination":{"total_items":null},"scans":items}),
            ),
        );
    }
    let collected = client.scans().list_all(query, None).await.unwrap();
    assert!(collected.is_complete());
    assert_eq!(collected.items.len(), 2);
    wire.done();
}
