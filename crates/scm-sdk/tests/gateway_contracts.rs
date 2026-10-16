//! Independent wire contracts: frozen TypeScript methods, exact independent fixtures,
//! and source-derived specimens checked against frozen Zod. No expected value is
//! derived from Rust validation.
#![cfg(feature = "gateway")]
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, Limits, RemoteOutcome, RequestOptions, Result, Secret,
    airs::gateway::{models::*, *},
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    config::OriginPolicy,
    transport::{Transport, TransportRequest, TransportResponse},
    validation::NativeDate,
};
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use std::{
    collections::{BTreeSet, VecDeque},
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
const ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const TOKEN: &str = "synthetic-gateway-bearer-secret";
const DATA: &str = "https://data.gateway.test/ai_gw/v2";
const ADMIN: &str = "https://admin.gateway.test/ai_gw/admin/v2";
const IAM: &str = "https://iam.gateway.test/iam/v1";
const FIXTURES: &str = include_str!("../src/fixtures/gateway-contracts.json");
struct Provider {
    context: AuthContext,
    calls: AtomicUsize,
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
            self.calls.fetch_add(1, Ordering::SeqCst);
            self.deadlines.lock().unwrap().push(options.deadline());
            TokenLease::new(Secret::new(TOKEN).unwrap(), 1, Duration::from_secs(3600))
        })
    }
}
#[derive(Clone)]
struct Expected {
    case: Value,
    status: u16,
    fail: bool,
    delay: Duration,
}
#[derive(Default)]
struct Wire {
    pending: Mutex<VecDeque<Expected>>,
    requests: Mutex<Vec<TransportRequest>>,
}
impl Wire {
    fn arm(&self, case: &Value, status: u16, fail: bool) {
        self.pending.lock().unwrap().push_back(Expected {
            case: case.clone(),
            status,
            fail,
            delay: Duration::ZERO,
        });
    }
    fn done(&self) {
        assert!(self.pending.lock().unwrap().is_empty());
    }
    fn count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            let expected = self
                .pending
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected Gateway dispatch");
            let case = &expected.case;
            let base = if case["plane"] == "admin" {
                ADMIN
            } else {
                DATA
            };
            let base = url::Url::parse(base).unwrap();
            assert_eq!(request.url().origin(), base.origin());
            assert_eq!(request.method().as_str(), case["method"].as_str().unwrap());
            assert_eq!(
                request.url().path(),
                format!("{}{}", base.path(), case["path"].as_str().unwrap())
            );
            let query = request
                .url()
                .query_pairs()
                .map(|(k, v)| json!([k, v]))
                .collect::<Vec<_>>();
            assert_eq!(Value::Array(query), case["query"], "{}", case["name"]);
            let has_body = !case["body"].is_null();
            let mut names = request
                .headers()
                .keys()
                .map(|name| name.as_str())
                .collect::<Vec<_>>();
            names.sort_unstable();
            assert_eq!(
                names,
                if has_body {
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
                format!("Bearer {TOKEN}")
            );
            assert_eq!(request.headers()["x-tsg-id"], "12345");
            assert_eq!(request.headers()["service-name"], "api");
            assert_eq!(
                request.headers()["user-agent"],
                concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
            );
            if has_body {
                assert_eq!(request.headers()["content-type"], "application/json");
                assert_eq!(
                    serde_json::from_slice::<Value>(request.body()).unwrap(),
                    case["body"]
                );
            } else {
                assert!(request.body().is_empty());
            }
            assert!(!format!("{request:?}").contains(TOKEN));
            self.requests.lock().unwrap().push(request);
            if expected.fail {
                return Err(Error::new(ErrorKind::Transport));
            }
            tokio::time::sleep(expected.delay).await;
            let bytes = if case["response"].is_null() {
                Bytes::new()
            } else {
                Bytes::from(serde_json::to_vec(&case["response"]).unwrap())
            };
            let mut headers = HeaderMap::new();
            headers.insert(
                "x-request-id",
                http::HeaderValue::from_static("gateway-correlation"),
            );
            Ok(TransportResponse::from_bytes(
                StatusCode::from_u16(expected.status).unwrap(),
                headers,
                bytes,
            ))
        })
    }
}
fn clients() -> (AIGatewayClient, Arc<Wire>, Arc<Provider>) {
    let wire = Arc::new(Wire::default());
    let provider = Arc::new(Provider {
        context: AuthContext::new(
            Endpoint::parse("https://issuer.gateway.test/token").unwrap(),
            "fixture-client",
            1,
            "12345",
        )
        .unwrap(),
        calls: AtomicUsize::new(0),
        deadlines: Mutex::new(Vec::new()),
    });
    let data = Endpoint::parse(DATA).unwrap();
    let admin = Endpoint::parse(ADMIN).unwrap();
    let iam = Endpoint::parse(IAM).unwrap();
    let mut limits = Limits::default();
    limits.max_retries = 0;
    let raw = Client::builder(data.clone())
        .transport(wire.clone())
        .authentication(Authentication::oauth(provider.clone(), false))
        .destination_policy(Arc::new(OriginPolicy::new([
            data,
            admin.clone(),
            iam.clone(),
        ])))
        .limits(limits)
        .build()
        .unwrap();
    (
        AIGatewayClient::new(raw, admin, iam).unwrap(),
        wire,
        provider,
    )
}
fn model<T: DeserializeOwned>(value: Value) -> T {
    serde_json::from_value(value).unwrap()
}
fn window() -> AIGatewayWindowOptions {
    let mut w = AIGatewayWindowOptions::new("ws-fixture");
    w.start = Some(NativeDate::from_epoch_millis(1_767_225_600_000).unwrap());
    w.end = Some(NativeDate::from_epoch_millis(1_767_312_000_000).unwrap());
    w
}
fn cases() -> Vec<Value> {
    serde_json::from_str(GOLDEN).unwrap()
}
fn case(name: &str) -> Value {
    cases()
        .into_iter()
        .find(|case| case["name"] == name)
        .unwrap()
}
async fn invoke(
    gateway: &AIGatewayClient,
    case: &Value,
    options: Option<&RequestOptions>,
) -> Result<Value> {
    match case["name"].as_str().unwrap() {
        "configs.list" => serde_json::to_value(
            gateway
                .configs()
                .list(&AIGatewayWorkspaceScopedListOptions::new(ID), options)
                .await?,
        ),
        "configs.get" => serde_json::to_value(gateway.configs().get(ID, options).await?),
        "configs.list_versions" => {
            serde_json::to_value(gateway.configs().list_versions(ID, options).await?)
        }
        "configs.create" => serde_json::to_value(
            gateway
                .configs()
                .create(&model(case["body"].clone()), options)
                .await?,
        ),
        "configs.update" => serde_json::to_value(
            gateway
                .configs()
                .update(ID, &model(case["body"].clone()), options)
                .await?,
        ),
        "configs.delete" => serde_json::to_value(gateway.configs().delete(ID, options).await?),
        "providers.list" => serde_json::to_value(
            gateway
                .providers()
                .list(&AIGatewayWorkspaceScopedListOptions::new(ID), options)
                .await?,
        ),
        "providers.get" => serde_json::to_value(gateway.providers().get(ID, options).await?),
        "providers.create" => serde_json::to_value(
            gateway
                .providers()
                .create(&model(case["body"].clone()), options)
                .await?,
        ),
        "providers.update" => serde_json::to_value(
            gateway
                .providers()
                .update(ID, &model(case["body"].clone()), options)
                .await?,
        ),
        "providers.delete" => serde_json::to_value(gateway.providers().delete(ID, options).await?),
        "deployments.list" => serde_json::to_value(gateway.deployments().list(options).await?),
        "deployments.get" => serde_json::to_value(gateway.deployments().get(ID, options).await?),
        "deployments.create" => serde_json::to_value(
            gateway
                .deployments()
                .create(&model(case["body"].clone()), options)
                .await?,
        ),
        "deployments.update" => serde_json::to_value(
            gateway
                .deployments()
                .update(ID, &model(case["body"].clone()), options)
                .await?,
        ),
        "deployments.ping" => serde_json::to_value(gateway.deployments().ping(ID, options).await?),
        "deployments.delete" => {
            serde_json::to_value(gateway.deployments().delete(ID, "12345", options).await?)
        }
        "organisations.get_info" => {
            serde_json::to_value(gateway.organisations().get_info("12345", options).await?)
        }
        "organisations.get_self" => {
            serde_json::to_value(gateway.organisations().get_self(options).await?)
        }
        "organisations.update_self" => serde_json::to_value(
            gateway
                .organisations()
                .update_self(&model(case["body"].clone()), options)
                .await?,
        ),
        "organisations.get_auth_settings" => serde_json::to_value(
            gateway
                .organisations()
                .get_auth_settings("12345", options)
                .await?,
        ),
        "organisations.update_auth_settings" => serde_json::to_value(
            gateway
                .organisations()
                .update_auth_settings("12345", &model(case["body"].clone()), options)
                .await?,
        ),
        "workspaces.list" => serde_json::to_value(
            gateway
                .workspaces()
                .list(&AIGatewayWorkspaceListOptions::default(), options)
                .await?,
        ),
        "workspaces.get" => serde_json::to_value(
            gateway
                .workspaces()
                .get(
                    "ws-fixture",
                    &AIGatewayWorkspaceGetOptions::default(),
                    options,
                )
                .await?,
        ),
        "workspaces.create" => serde_json::to_value(
            gateway
                .workspaces()
                .create(&model(case["body"].clone()), options)
                .await?,
        ),
        "workspaces.update" => serde_json::to_value(
            gateway
                .workspaces()
                .update("ws-fixture", &model(case["body"].clone()), options)
                .await?,
        ),
        "workspaces.delete" => {
            serde_json::to_value(gateway.workspaces().delete("ws-fixture", options).await?)
        }
        "log_exports.get" => serde_json::to_value(gateway.log_exports().get(ID, options).await?),
        "log_exports.update" => serde_json::to_value(
            gateway
                .log_exports()
                .update(ID, &model(case["body"].clone()), options)
                .await?,
        ),
        "log_exports.list" => serde_json::to_value(
            gateway
                .log_exports()
                .list(&model(case["queryObject"].clone()), options)
                .await?,
        ),
        "log_exports.create" => serde_json::to_value(
            gateway
                .log_exports()
                .create(&model(case["body"].clone()), options)
                .await?,
        ),
        "log_exports.start" => {
            serde_json::to_value(gateway.log_exports().start(ID, options).await?)
        }
        "log_exports.cancel" => {
            serde_json::to_value(gateway.log_exports().cancel(ID, options).await?)
        }
        "log_exports.download" => {
            serde_json::to_value(gateway.log_exports().download(ID, options).await?)
        }
        "rate_limits.create" => serde_json::to_value(
            gateway
                .rate_limits()
                .create(&model(case["body"].clone()), options)
                .await?,
        ),
        "rate_limits.list" => serde_json::to_value(
            gateway
                .rate_limits()
                .list(&model(case["queryObject"].clone()), options)
                .await?,
        ),
        "rate_limits.get" => serde_json::to_value(
            gateway
                .rate_limits()
                .get(ID, &model(case["queryObject"].clone()), options)
                .await?,
        ),
        "rate_limits.update" => serde_json::to_value(
            gateway
                .rate_limits()
                .update(ID, &model(case["body"].clone()), options)
                .await?,
        ),
        "rate_limits.delete" => {
            serde_json::to_value(gateway.rate_limits().delete(ID, options).await?)
        }
        "telemetry.error_category_trends" => serde_json::to_value(
            gateway
                .telemetry()
                .error_category_trends(&window(), options)
                .await?,
        ),
        "telemetry.grouped_errors" => serde_json::to_value(
            gateway
                .telemetry()
                .grouped_errors(&window(), options)
                .await?,
        ),
        "telemetry.cost" => serde_json::to_value(
            gateway
                .telemetry()
                .cost(&AIGatewayChartOptions::new(window()).unwrap(), options)
                .await?,
        ),
        "telemetry.requests" => serde_json::to_value(
            gateway
                .telemetry()
                .requests(&AIGatewayChartOptions::new(window()).unwrap(), options)
                .await?,
        ),
        "telemetry.latency" => serde_json::to_value(
            gateway
                .telemetry()
                .latency(&AIGatewayChartOptions::new(window()).unwrap(), options)
                .await?,
        ),
        "telemetry.tokens" => serde_json::to_value(
            gateway
                .telemetry()
                .tokens(&AIGatewayChartOptions::new(window()).unwrap(), options)
                .await?,
        ),
        "telemetry.errors" => {
            serde_json::to_value(gateway.telemetry().errors(&window(), options).await?)
        }
        "telemetry.users" => {
            serde_json::to_value(gateway.telemetry().users(&window(), options).await?)
        }
        "telemetry.cache_summary" => serde_json::to_value(
            gateway
                .telemetry()
                .cache_summary(&window(), options)
                .await?,
        ),
        "telemetry.cache_hit_trend" => serde_json::to_value(
            gateway
                .telemetry()
                .cache_hit_trend(&window(), options)
                .await?,
        ),
        "telemetry.user_trends" => {
            serde_json::to_value(gateway.telemetry().user_trends(&window(), options).await?)
        }
        "telemetry.error_trends" => {
            serde_json::to_value(gateway.telemetry().error_trends(&window(), options).await?)
        }
        "telemetry.rescued_retries" => serde_json::to_value(
            gateway
                .telemetry()
                .rescued_retries(&window(), options)
                .await?,
        ),
        "telemetry.feedback_trend" => serde_json::to_value(
            gateway
                .telemetry()
                .feedback_trend(&window(), options)
                .await?,
        ),
        "telemetry.feedback_weighted" => serde_json::to_value(
            gateway
                .telemetry()
                .feedback_weighted(&window(), options)
                .await?,
        ),
        "telemetry.feedback_score_distribution" => serde_json::to_value(
            gateway
                .telemetry()
                .feedback_score_distribution(&window(), options)
                .await?,
        ),
        "telemetry.feedback_models" => serde_json::to_value(
            gateway
                .telemetry()
                .feedback_models(&window(), options)
                .await?,
        ),
        "telemetry.filter_boundaries" => serde_json::to_value(
            gateway
                .telemetry()
                .filter_boundaries(&window(), options)
                .await?,
        ),
        "telemetry.by_user" => serde_json::to_value(
            gateway
                .telemetry()
                .by_user(&AIGatewayChartOptions::new(window()).unwrap(), options)
                .await?,
        ),
        "telemetry.group_by" => serde_json::to_value(
            gateway
                .telemetry()
                .group_by(
                    AIGatewayGroupDimension::Model,
                    &AIGatewayGroupOptions::new(AIGatewayChartOptions::new(window()).unwrap()),
                    options,
                )
                .await?,
        ),
        "telemetry.by_status_code" => serde_json::to_value(
            gateway
                .telemetry()
                .by_status_code(
                    &AIGatewayGroupOptions::new(AIGatewayChartOptions::new(window()).unwrap()),
                    options,
                )
                .await?,
        ),
        "telemetry.logs" => serde_json::to_value(
            gateway
                .telemetry()
                .logs(&AIGatewayLogsOptions::new(window()), options)
                .await?,
        ),
        _ => panic!("unimplemented independent test call"),
    }
    .map_err(|_| Error::new(ErrorKind::Decode))
}
#[tokio::test]
async fn all_sixty_one_methods_match_frozen_wire_contracts_and_zod_response_specimens() {
    let (gateway, wire, provider) = clients();
    let options = RequestOptions::with_timeout(Duration::from_secs(30)).unwrap();
    for case in cases() {
        wire.arm(
            &case,
            if case["response"].is_null() { 204 } else { 200 },
            false,
        );
        let output = invoke(&gateway, &case, Some(&options)).await.unwrap();
        assert_eq!(output, case["response"], "{}", case["name"]);
    }
    assert_eq!(wire.count(), 61);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 61);
    assert!(
        provider
            .deadlines
            .lock()
            .unwrap()
            .iter()
            .all(|deadline| *deadline == options.deadline())
    );
    wire.done();
}

#[tokio::test]
async fn all_twelve_packaged_independent_cases_drive_resource_wire_and_response_contracts() {
    let bundle: Value = serde_json::from_str(FIXTURES).unwrap();
    assert_eq!(
        bundle["baselineCommit"],
        "4a80dbdb3dc14365e1c25d838ac0bd139485d48c"
    );
    let fixtures = bundle["cases"].as_array().unwrap();
    assert_eq!(fixtures.len(), 12);
    let (gateway, wire, provider) = clients();
    let mut consumed = BTreeSet::new();
    let mut methods = BTreeSet::new();
    for entry in fixtures {
        let frozen = &entry["case"];
        let family = match frozen["call"]["className"].as_str().unwrap() {
            "AIGatewayLogExportsClient" => "log_exports",
            "AIGatewayRateLimitsClient" => "rate_limits",
            other => panic!("unconsumed frozen client: {other}"),
        };
        let name = format!("{family}.{}", frozen["call"]["member"].as_str().unwrap());
        assert!(methods.insert(name.clone()), "duplicate frozen method");
        assert!(consumed.insert((
            entry["source"]["path"].as_str().unwrap(),
            entry["source"]["pointer"].as_str().unwrap(),
        )));
        // The fixture stores parameterized OpenAPI paths. Bind only the two
        // explicit source parameters used by these independent cases.
        let path = frozen["path"]
            .as_str()
            .unwrap()
            .replace("{exportId}", ID)
            .replace("{rateLimitsPolicyId}", ID);
        assert!(!path.contains(['{', '}']), "unbound frozen path parameter");
        let ordered_keys: &[&str] = match name.as_str() {
            "log_exports.list" => &["workspace_id"],
            "rate_limits.list" => &[
                "workspace_id",
                "status",
                "type",
                "unit",
                "target",
                "page_size",
                "current_page",
            ],
            "rate_limits.get" => &["status"],
            _ => &[],
        };
        let mut query = Vec::new();
        let mut query_keys = BTreeSet::new();
        for key in ordered_keys {
            if let Some(values) = frozen["queryWire"].get(*key) {
                assert!(query_keys.insert(*key));
                for value in values.as_array().unwrap() {
                    query.push(json!([key, value.as_str().unwrap()]));
                }
            }
        }
        assert_eq!(
            query_keys.len(),
            frozen["queryWire"].as_object().unwrap().len()
        );
        let expected = json!({
            "name": name, "plane": "data", "method": frozen["method"],
            "path": path, "query": query, "queryObject": frozen["query"],
            "body": frozen["body"], "response": frozen["responseBody"],
        });
        // Keep the broader 61-method table tied to the independent fixture too;
        // changing its copied expectations cannot silently bypass this sweep.
        let inline = case(&name);
        for field in ["method", "plane", "path", "query", "body", "response"] {
            assert_eq!(inline[field], expected[field], "{name}: {field}");
        }
        assert_eq!(inline["expectedSource"], entry["source"]);
        wire.arm(
            &expected,
            frozen["responseStatus"].as_u64().unwrap() as u16,
            false,
        );
        assert_eq!(
            invoke(&gateway, &expected, None).await.unwrap(),
            frozen["responseBody"],
            "{name} independent response"
        );
    }
    assert_eq!(consumed.len(), fixtures.len(), "unused packaged fixture");
    assert_eq!(methods.len(), 12);
    assert_eq!(wire.count(), 12);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 12);
    wire.done();
}
#[tokio::test]
async fn every_owned_mutation_is_single_dispatch_for_transport_and_server_ambiguity() {
    let mutations = cases()
        .into_iter()
        .filter(|case| case["method"] != "GET")
        .collect::<Vec<_>>();
    assert_eq!(mutations.len(), 21);
    for case in mutations {
        for transport_failure in [true, false] {
            let (gateway, wire, _) = clients();
            wire.arm(&case, 503, transport_failure);
            let options = RequestOptions::with_timeout(Duration::from_secs(10))
                .unwrap()
                .with_retries(5)
                .unwrap();
            let error = invoke(&gateway, &case, Some(&options)).await.unwrap_err();
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
            assert_eq!(wire.count(), 1);
            assert!(!format!("{error:?} {error}").contains(TOKEN));
            wire.done();
        }
    }
}

#[tokio::test]
async fn filtered_telemetry_preserves_csv_lists_cents_zero_metadata_and_column_order() {
    let (gateway, wire, _) = clients();
    let mut chart = AIGatewayChartOptions::new(window()).unwrap();
    chart.filters = model(json!({
        "traceId":"trace+a&b", "metadata":{"environment":"a+b&c"},
        "statusCodes":[418,200,418],"apiKeyIds":[ID,ID],
        "aiOrgModels":["openai__model+a&b","azure-openai__deployment"],
        "totalUnitsMin":0,"totalUnitsMax":42,"costMin":0,"costMax":0.125
    }));
    let filter_query = json!([
        ["traceId", "trace+a&b"],
        ["metadata", "{\"environment\":\"a+b&c\"}"],
        ["statusCode", "418,200,418"],
        ["apiKeyIds", format!("{ID},{ID}")],
        ["aiOrgModel", "openai__model+a&b,azure-openai__deployment"],
        ["totalUnitsMin", "0"],
        ["totalUnitsMax", "42"],
        ["costMin", "0"],
        ["costMax", "0.125"]
    ]);
    for metric in ["cost", "requests", "latency", "tokens", "by_user"] {
        let mut expected = case(&format!("telemetry.{metric}"));
        expected["query"]
            .as_array_mut()
            .unwrap()
            .extend(filter_query.as_array().unwrap().clone());
        wire.arm(&expected, 200, false);
        let output = match metric {
            "cost" => serde_json::to_value(gateway.telemetry().cost(&chart, None).await.unwrap()),
            "requests" => {
                serde_json::to_value(gateway.telemetry().requests(&chart, None).await.unwrap())
            }
            "latency" => {
                serde_json::to_value(gateway.telemetry().latency(&chart, None).await.unwrap())
            }
            "tokens" => {
                serde_json::to_value(gateway.telemetry().tokens(&chart, None).await.unwrap())
            }
            _ => serde_json::to_value(gateway.telemetry().by_user(&chart, None).await.unwrap()),
        }
        .unwrap();
        assert_eq!(output, expected["response"]);
    }
    let mut group = AIGatewayGroupOptions::new(chart.clone());
    group.columns = Some(vec![
        AIGatewayGroupColumn::Cost,
        AIGatewayGroupColumn::TotalTokens,
        AIGatewayGroupColumn::Cost,
    ]);
    for (dimension, path) in [
        (AIGatewayGroupDimension::AiService, "ai_service"),
        (AIGatewayGroupDimension::Model, "model"),
        (AIGatewayGroupDimension::ApiKey, "api_key"),
        (AIGatewayGroupDimension::Provider, "provider"),
    ] {
        let mut expected = case("telemetry.group_by");
        expected["path"] = json!(format!("/logs/groups/{path}"));
        expected["query"]
            .as_array_mut()
            .unwrap()
            .extend(filter_query.as_array().unwrap().clone());
        expected["query"]
            .as_array_mut()
            .unwrap()
            .push(json!(["columns", "cost,total_tokens,cost"]));
        wire.arm(&expected, 200, false);
        let actual = gateway
            .telemetry()
            .group_by(dimension, &group, None)
            .await
            .unwrap();
        assert_eq!(serde_json::to_value(actual).unwrap(), expected["response"]);
    }
    let mut expected = case("telemetry.by_status_code");
    expected["query"]
        .as_array_mut()
        .unwrap()
        .extend(filter_query.as_array().unwrap().clone());
    expected["query"]
        .as_array_mut()
        .unwrap()
        .push(json!(["columns", "cost,total_tokens,cost"]));
    wire.arm(&expected, 200, false);
    gateway
        .telemetry()
        .by_status_code(&group, None)
        .await
        .unwrap();
    group.columns = Some(Vec::new());
    let mut expected = case("telemetry.group_by");
    expected["query"]
        .as_array_mut()
        .unwrap()
        .extend(filter_query.as_array().unwrap().clone());
    wire.arm(&expected, 200, false);
    gateway
        .telemetry()
        .group_by(AIGatewayGroupDimension::Model, &group, None)
        .await
        .unwrap();
    assert!(!format!("{chart:?} {group:?}").contains("trace+a&b"));
    assert_eq!(wire.count(), 11);
    wire.done();
}

#[tokio::test]
async fn telemetry_windows_offsets_defaults_and_zero_based_logs_have_exact_source_names() {
    let (gateway, wire, _) = clients();
    let mut logs = AIGatewayLogsOptions::new(window());
    logs.window.offset_minutes = 120;
    logs.page_size = Some(0);
    logs.current_page = Some(0);
    logs.status_code = Some(0);
    logs.trace_id = Some("trace+a&b".into());
    let mut expected = case("telemetry.logs");
    expected["query"] = json!([
        ["organisationId", "12345"],
        ["workspaceSlug", "ws-fixture"],
        ["timeOfGenerationMin", "2026-01-01T02:00:00+02:00"],
        ["timeOfGenerationMax", "2026-01-02T02:00:00+02:00"],
        ["pageSize", "0"],
        ["currentPage", "0"],
        ["traceId", "trace+a&b"],
        ["statusCode", "0"]
    ]);
    wire.arm(&expected, 200, false);
    gateway.telemetry().logs(&logs, None).await.unwrap();
    let mut defaults = window();
    defaults.start = None;
    let mut expected = case("telemetry.errors");
    expected["query"][2][1] = json!("2025-12-26T00:00:00+00:00");
    wire.arm(&expected, 200, false);
    gateway.telemetry().errors(&defaults, None).await.unwrap();
    defaults.days = Some(0.0);
    expected["query"][2][1] = json!("2026-01-02T00:00:00+00:00");
    wire.arm(&expected, 200, false);
    gateway.telemetry().errors(&defaults, None).await.unwrap();
    defaults.start = window().start;
    defaults.days = Some(-1.0);
    let expected = case("telemetry.errors");
    wire.arm(&expected, 200, false);
    gateway.telemetry().errors(&defaults, None).await.unwrap();
    assert_eq!(wire.count(), 4);
    wire.done();
}

#[tokio::test]
async fn explicit_workspace_planes_and_archived_state_never_infer_permission_fallback() {
    let (gateway, wire, _) = clients();
    let mut params = AIGatewayWorkspaceListOptions::default();
    params.plane = AIGatewayPlane::Admin;
    params.status = Some(AIGatewayWorkspaceStatus::Archived);
    let mut expected = case("workspaces.list");
    expected["plane"] = json!("admin");
    expected["query"] = json!([["status", "archived"]]);
    wire.arm(&expected, 200, false);
    gateway.workspaces().list(&params, None).await.unwrap();
    let mut get = AIGatewayWorkspaceGetOptions::default();
    get.plane = AIGatewayPlane::Admin;
    let mut expected = case("workspaces.get");
    expected["plane"] = json!("admin");
    wire.arm(&expected, 200, false);
    gateway
        .workspaces()
        .get("ws-fixture", &get, None)
        .await
        .unwrap();
    let mut expected = case("workspaces.get");
    expected["response"] = json!({"errorCode":"AB03","private":"upstream-secret"});
    wire.arm(&expected, 403, false);
    let error = gateway
        .workspaces()
        .get("ws-fixture", &AIGatewayWorkspaceGetOptions::default(), None)
        .await
        .unwrap_err();
    assert_eq!(
        (error.kind(), error.status(), error.remote_outcome()),
        (ErrorKind::Http, Some(403), RemoteOutcome::Rejected)
    );
    assert!(!format!("{error:?} {error}").contains("upstream-secret"));
    assert_eq!(wire.count(), 3);
    wire.done();
}

#[tokio::test]
async fn nullable_receipts_and_unknown_telemetry_values_preserve_the_wire_without_guessing() {
    let (gateway, wire, _) = clients();
    let mut expected = case("telemetry.latency");
    expected["response"] = json!({"success":true,"future":"response-secret","data":{
        "records":[{"x":"2026-01-01","y":0,"p50":0,"p90":0,"p99":0,"future":true}],
        "total":null,"p50":null,"p90":null,"p99":null,"isQuotaExceeded":false,"future":42
    }});
    wire.arm(&expected, 200, false);
    let actual = gateway
        .telemetry()
        .latency(&AIGatewayChartOptions::new(window()).unwrap(), None)
        .await
        .unwrap();
    assert!(!format!("{actual:?}").contains("response-secret"));
    assert_eq!(serde_json::to_value(actual).unwrap(), expected["response"]);
    let mut expected = case("configs.update");
    expected["response"] = Value::Null;
    wire.arm(&expected, 204, false);
    let actual = gateway
        .configs()
        .update(ID, &model(expected["body"].clone()), None)
        .await
        .unwrap();
    assert_eq!(serde_json::to_value(actual).unwrap(), json!({}));
    let mut expected = case("log_exports.download");
    expected["response"] = json!({
        "signed_url": "https://unadmitted.example.test/private?signature=download-secret",
        "future": true
    });
    wire.arm(&expected, 200, false);
    let actual = gateway.log_exports().download(ID, None).await.unwrap();
    assert!(!format!("{actual:?}").contains("download-secret"));
    assert_eq!(serde_json::to_value(actual).unwrap(), expected["response"]);
    assert_eq!(wire.count(), 3);
    wire.done();
}

#[tokio::test]
async fn invalid_ids_models_filters_ranges_and_url_budgets_fail_before_authentication() {
    let (gateway, wire, provider) = clients();
    for bad in ["", "../private", "%252e%252e%252fprivate", "not-a-uuid"] {
        assert!(gateway.configs().get(bad, None).await.is_err());
        assert!(gateway.configs().list_versions(bad, None).await.is_err());
        assert!(gateway.providers().get(bad, None).await.is_err());
        assert!(gateway.deployments().get(bad, None).await.is_err());
        assert!(gateway.deployments().ping(bad, None).await.is_err());
    }
    for bad in ["", "../private", "ws/other", "\u{feff}"] {
        assert!(
            gateway
                .workspaces()
                .get(bad, &AIGatewayWorkspaceGetOptions::default(), None)
                .await
                .is_err()
        );
    }
    for bad in ["", "../private", "12a", "123/45"] {
        assert!(gateway.organisations().get_info(bad, None).await.is_err());
        assert!(
            gateway
                .organisations()
                .get_auth_settings(bad, None)
                .await
                .is_err()
        );
    }
    for bad in ["", ".", "..", "%252e%252e%252fprivate", "\u{feff}"] {
        assert!(gateway.log_exports().get(bad, None).await.is_err());
        assert!(gateway.rate_limits().delete(bad, None).await.is_err());
    }
    assert!(
        gateway
            .log_exports()
            .get(&"😀".repeat(257), None)
            .await
            .is_err()
    );
    for body in [
        json!({}),
        json!({"workspace_id":ID,"name":"bad","config":{},"unverified":true}),
    ] {
        assert!(
            gateway
                .configs()
                .update(ID, &model(body), None)
                .await
                .is_err()
        );
    }
    assert!(
        gateway
            .workspaces()
            .update("ws-fixture", &model(json!({})), None)
            .await
            .is_err()
    );
    for filters in [
        json!({"statusCodes":[]}),
        json!({"statusCodes":[-1]}),
        json!({"statusCodes":[1.5]}),
        json!({"apiKeyIds":["bad"]}),
        json!({"aiOrgModels":["without-provider"]}),
        json!({"costMin":0.2,"costMax":0.1}),
        json!({"totalUnitsMin":2,"totalUnitsMax":1}),
        json!({"spanId":"unverified-secret"}),
        json!({"traceId":"\u{feff}"}),
        json!({"metadata":{"key":"x".repeat(20_000)}}),
    ] {
        let mut chart = AIGatewayChartOptions::new(window()).unwrap();
        chart.filters = model(filters);
        let error = gateway.telemetry().cost(&chart, None).await.unwrap_err();
        assert!(!format!("{error:?} {error}").contains("unverified-secret"));
    }
    let mut invalid_slug = window();
    invalid_slug.workspace_slug = "../secret".into();
    assert!(
        gateway
            .telemetry()
            .errors(&invalid_slug, None)
            .await
            .is_err()
    );
    let mut bad = window();
    bad.days = Some(f64::NAN);
    assert!(gateway.telemetry().errors(&bad, None).await.is_err());
    bad = window();
    bad.start = bad.end;
    bad.end = window().start;
    assert!(gateway.telemetry().errors(&bad, None).await.is_err());
    bad = window();
    bad.offset_minutes = 1440;
    assert!(gateway.telemetry().errors(&bad, None).await.is_err());
    bad = window();
    bad.end = Some(NativeDate::from_epoch_millis(8_640_000_000_000_000).unwrap());
    assert!(gateway.telemetry().errors(&bad, None).await.is_err());
    let mut logs = AIGatewayLogsOptions::new(window());
    logs.current_page = Some(9_007_199_254_740_992);
    assert!(gateway.telemetry().logs(&logs, None).await.is_err());
    let mut query = case("rate_limits.list")["queryObject"].clone();
    query["current_page"] = json!(-1);
    assert!(
        gateway
            .rate_limits()
            .list(&model(query), None)
            .await
            .is_err()
    );
    let params: GatewayLogExportsClientListOptions =
        model(json!({"workspace_id":"x".repeat(20_000)}));
    assert_eq!(
        gateway
            .log_exports()
            .list(&params, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert_eq!(wire.count(), 0);
    wire.done();
}

#[tokio::test]
async fn caller_cancellation_deadlines_decode_metadata_and_facade_credential_planes_are_explicit() {
    let (gateway, wire, provider) = clients();
    assert!(gateway.inference().is_err());
    assert!(gateway.runtime_resources().is_err());
    let options = RequestOptions::with_timeout(Duration::from_secs(10)).unwrap();
    options.cancel();
    assert_eq!(
        gateway
            .deployments()
            .list(Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    let options = RequestOptions::with_timeout(Duration::from_nanos(1)).unwrap();
    tokio::time::sleep(Duration::from_millis(1)).await;
    assert_eq!(
        gateway
            .configs()
            .list(
                &AIGatewayWorkspaceScopedListOptions::new(ID),
                Some(&options)
            )
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Deadline
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    let mut expected = case("configs.get");
    expected["response"] = json!({"private":"decode-secret"});
    wire.arm(&expected, 200, false);
    let error = gateway.configs().get(ID, None).await.unwrap_err();
    assert_eq!(
        (error.kind(), error.status(), error.remote_outcome()),
        (ErrorKind::Decode, Some(200), RemoteOutcome::Unknown)
    );
    assert!(!format!("{error:?} {error}").contains("decode-secret"));
    assert_eq!(wire.count(), 1);
    wire.done();
    gateway.client().session().cancel();
    assert_eq!(
        gateway
            .organisations()
            .get_self(None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(wire.count(), 1);
}

#[test]
fn scm_constructors_reject_foreign_credentials_and_policy_is_never_broadened() {
    let wire = Arc::new(Wire::default());
    for authentication in [
        Authentication::None,
        Authentication::runtime_key(Secret::new("runtime-secret").unwrap()).unwrap(),
        Authentication::scan(Some(Secret::new("scan-secret").unwrap()), None).unwrap(),
    ] {
        let raw = Client::builder(Endpoint::parse(DATA).unwrap())
            .transport(wire.clone())
            .authentication(authentication)
            .build()
            .unwrap();
        assert!(
            AIGatewayClient::new(
                raw.clone(),
                Endpoint::parse(ADMIN).unwrap(),
                Endpoint::parse(IAM).unwrap()
            )
            .is_err()
        );
        assert!(AIGatewayTelemetryClient::new(raw.clone()).is_err());
        assert!(AIGatewayConfigsClient::new(raw.clone()).is_err());
        assert!(AIGatewayWorkspacesClient::new(raw, Endpoint::parse(ADMIN).unwrap()).is_err());
    }
    let (gateway, _, _) = clients();
    assert!(
        AIGatewayClient::new(
            gateway.client().clone(),
            Endpoint::parse("https://unadmitted.test").unwrap(),
            Endpoint::parse(IAM).unwrap()
        )
        .is_err()
    );
    assert!(!format!("{gateway:?}").contains(TOKEN));
}

#[tokio::test]
async fn attached_runtime_clients_keep_their_independent_key_and_destination() {
    struct RuntimeWire(AtomicUsize);
    impl Transport for RuntimeWire {
        fn send<'a>(
            &'a self,
            request: TransportRequest,
        ) -> BoxFuture<'a, Result<TransportResponse>> {
            Box::pin(async move {
                self.0.fetch_add(1, Ordering::SeqCst);
                assert_eq!(request.url().host_str(), Some("runtime.gateway.test"));
                assert_eq!(request.url().path(), "/v1/files/fixture/content");
                assert_eq!(request.method(), http::Method::GET);
                assert!(!request.headers().contains_key("authorization"));
                assert!(!request.headers().contains_key("x-tsg-id"));
                assert_eq!(
                    request.headers()["x-portkey-api-key"],
                    "synthetic-runtime-key"
                );
                assert!(!format!("{request:?}").contains("synthetic-runtime-key"));
                Ok(TransportResponse::from_bytes(
                    StatusCode::OK,
                    HeaderMap::new(),
                    Bytes::from_static(&[0, 255, 7]),
                ))
            })
        }
    }
    let (gateway, wire, provider) = clients();
    let runtime_wire = Arc::new(RuntimeWire(AtomicUsize::new(0)));
    let raw = Client::builder(Endpoint::parse("https://runtime.gateway.test/v1").unwrap())
        .transport(runtime_wire.clone())
        .authentication(
            Authentication::runtime_key(Secret::new("synthetic-runtime-key").unwrap()).unwrap(),
        )
        .build()
        .unwrap();
    let gateway = gateway.with_runtime(RuntimeClient::new(raw).unwrap());
    assert!(
        gateway
            .inference()
            .unwrap()
            .client()
            .oauth_context()
            .is_none()
    );
    gateway.client().session().cancel();
    assert_eq!(
        gateway
            .runtime_resources()
            .unwrap()
            .download_file("fixture", None)
            .await
            .unwrap(),
        Bytes::from_static(&[0, 255, 7])
    );
    assert_eq!(runtime_wire.0.load(Ordering::SeqCst), 1);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert_eq!(wire.count(), 0);
}

// Literal source-derived requests and responses. Actual frozen Zod acceptance
// and normalization are reproducibly checked by tools/verify-gateway-specimens.mts;
// its source/input hashes and results live in contracts/native-fixtures/gateway-specimens.json.
const GOLDEN: &str = r#"[
  {
    "name": "configs.list",
    "method": "GET",
    "plane": "data",
    "path": "/configs",
    "query": [
      [
        "workspace_id",
        "550e8400-e29b-41d4-a716-446655440000"
      ]
    ],
    "body": null,
    "response": {
      "object": "fixture",
      "total": 0,
      "data": []
    },
    "requestSchema": null,
    "responseSchema": "ListConfigsResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "configs.get",
    "method": "GET",
    "plane": "data",
    "path": "/configs/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "body": null,
    "response": {
      "id": "fixture",
      "name": "fixture",
      "slug": "fixture",
      "organisation_id": "fixture",
      "is_default": 0,
      "status": "fixture",
      "owner_id": "fixture",
      "updated_by": "fixture",
      "created_at": "fixture",
      "last_updated_at": "fixture",
      "workspace_id": "fixture",
      "object": "fixture",
      "config": "fixture",
      "format": "fixture",
      "type": "fixture",
      "version_id": "fixture"
    },
    "requestSchema": null,
    "responseSchema": "GatewayConfigDetailSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "configs.list_versions",
    "method": "GET",
    "plane": "data",
    "path": "/configs/550e8400-e29b-41d4-a716-446655440000/versions",
    "query": [],
    "body": null,
    "response": {
      "object": "fixture",
      "total": 0,
      "data": []
    },
    "requestSchema": null,
    "responseSchema": "ListConfigVersionsResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "configs.create",
    "method": "POST",
    "plane": "data",
    "path": "/configs",
    "query": [],
    "body": {
      "name": "fixture",
      "workspace_id": "550e8400-e29b-41d4-a716-446655440000",
      "config": {}
    },
    "response": {
      "id": "fixture",
      "version_id": "fixture",
      "slug": "fixture",
      "object": "fixture"
    },
    "requestSchema": "GatewayConfigCreateRequestSchema",
    "responseSchema": "GatewayConfigCreateResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "configs.update",
    "method": "PUT",
    "plane": "data",
    "path": "/configs/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "body": {
      "name": "fixture",
      "workspace_id": "550e8400-e29b-41d4-a716-446655440000",
      "config": {
        "retry": {
          "attempts": 0,
          "on_status_codes": [
            100
          ]
        },
        "cache": {
          "mode": "fixture",
          "max_age": 0
        },
        "strategy": {
          "mode": "fixture"
        },
        "targets": [
          {
            "provider": "fixture",
            "virtual_key": "fixture"
          }
        ],
        "provider": "fixture",
        "virtual_key": "fixture"
      },
      "status": "fixture"
    },
    "response": {},
    "requestSchema": "GatewayConfigUpdateRequestSchema",
    "responseSchema": "GatewayWriteResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "configs.delete",
    "method": "DELETE",
    "plane": "data",
    "path": "/configs/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "body": null,
    "response": null,
    "requestSchema": null,
    "responseSchema": null,
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "providers.list",
    "method": "GET",
    "plane": "data",
    "path": "/providers",
    "query": [
      [
        "workspace_id",
        "550e8400-e29b-41d4-a716-446655440000"
      ]
    ],
    "body": null,
    "response": {
      "object": "fixture",
      "total": 0,
      "data": []
    },
    "requestSchema": null,
    "responseSchema": "ListProvidersResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "providers.get",
    "method": "GET",
    "plane": "data",
    "path": "/providers/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "body": null,
    "response": {
      "id": "fixture",
      "ai_provider_name": "fixture",
      "model_config": {},
      "key": "fixture",
      "masked_api_key": "fixture",
      "slug": "fixture",
      "name": "fixture",
      "usage_limits": null,
      "status": "fixture",
      "note": null,
      "created_at": "fixture",
      "expires_at": null,
      "last_reset_at": null,
      "rate_limits": [],
      "integration_id": "fixture",
      "tags": null,
      "object": "fixture"
    },
    "requestSchema": null,
    "responseSchema": "GatewayProviderDetailSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "providers.create",
    "method": "POST",
    "plane": "data",
    "path": "/providers",
    "query": [],
    "body": {
      "workspace_id": "550e8400-e29b-41d4-a716-446655440000",
      "ai_provider_id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "fixture",
      "integration_id": "550e8400-e29b-41d4-a716-446655440000",
      "slug": "fixture"
    },
    "response": {
      "id": "fixture",
      "slug": "fixture",
      "object": "fixture"
    },
    "requestSchema": "GatewayProviderCreateRequestSchema",
    "responseSchema": "GatewayProviderCreateResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "providers.update",
    "method": "PUT",
    "plane": "data",
    "path": "/providers/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "body": {
      "name": "fixture"
    },
    "response": {},
    "requestSchema": "GatewayProviderUpdateRequestSchema",
    "responseSchema": "GatewayWriteResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "providers.delete",
    "method": "DELETE",
    "plane": "data",
    "path": "/providers/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "body": null,
    "response": null,
    "requestSchema": null,
    "responseSchema": null,
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "deployments.list",
    "method": "GET",
    "plane": "admin",
    "path": "/deployments",
    "query": [],
    "body": null,
    "response": {
      "object": "fixture",
      "total": 0,
      "data": []
    },
    "requestSchema": null,
    "responseSchema": "ListDeploymentsResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "deployments.get",
    "method": "GET",
    "plane": "admin",
    "path": "/deployments/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "body": null,
    "response": {
      "id": "fixture",
      "name": "fixture",
      "slug": "fixture",
      "type": "fixture",
      "status": "fixture",
      "created_at": "fixture",
      "last_updated_at": "fixture",
      "last_synced_at": null,
      "last_resynced_at": null,
      "is_default": 0,
      "created_by": "fixture",
      "object": "fixture",
      "deployment_config": null
    },
    "requestSchema": null,
    "responseSchema": "GatewayDeploymentDetailSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "deployments.create",
    "method": "POST",
    "plane": "admin",
    "path": "/deployments",
    "query": [],
    "body": {
      "name": "fixture",
      "type": "non_production",
      "organisation_id": "1"
    },
    "response": {
      "id": "fixture",
      "client_auth": "fixture",
      "credentials": {
        "username": "fixture",
        "password": "fixture"
      },
      "organisation_id": "fixture",
      "object": "fixture"
    },
    "requestSchema": "GatewayDeploymentCreateRequestSchema",
    "responseSchema": "GatewayDeploymentCreateResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "deployments.update",
    "method": "PUT",
    "plane": "admin",
    "path": "/deployments/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "body": {
      "name": "fixture",
      "type": "non_production",
      "status": "active",
      "deployment_config": {
        "fixture_key": "fixture"
      },
      "is_default": false,
      "rotate_auth": false,
      "override_existing": false,
      "auth_settings": {
        "gateway_base_url": "https://example.invalid/resource",
        "mcp_gateway_base_url": "https://example.invalid/resource",
        "is_dataservice_hosted": 0,
        "is_playground_proxy_allowed": 0,
        "workspaces_allowed": [
          "fixture"
        ],
        "jwt_subs_allowed": [
          "fixture"
        ],
        "jwt_sub_workspace_mapping": {
          "fixture": "fixture"
        },
        "allow_all_workspaces": false,
        "remove_workspaces_allowed": [
          "fixture"
        ],
        "remove_subs_allowed": [
          "fixture"
        ]
      },
      "tags": {
        "fixture_key": "fixture"
      }
    },
    "response": {},
    "requestSchema": "GatewayDeploymentUpdateRequestSchema",
    "responseSchema": "GatewayWriteResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "deployments.ping",
    "method": "GET",
    "plane": "admin",
    "path": "/deployments/550e8400-e29b-41d4-a716-446655440000/ping",
    "query": [],
    "body": null,
    "response": {
      "status": "fixture",
      "gateway_base_url": "fixture",
      "outbound": {
        "status": "fixture"
      },
      "inbound": {
        "status": "fixture"
      },
      "object": "fixture"
    },
    "requestSchema": null,
    "responseSchema": "GatewayDeploymentPingResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "deployments.delete",
    "method": "DELETE",
    "plane": "admin",
    "path": "/deployments/550e8400-e29b-41d4-a716-446655440000",
    "query": [
      [
        "organisation_id",
        "12345"
      ]
    ],
    "body": null,
    "response": null,
    "requestSchema": null,
    "responseSchema": null,
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "organisations.get_info",
    "method": "GET",
    "plane": "admin",
    "path": "/organisations/12345/info",
    "query": [],
    "body": null,
    "response": {
      "id": "fixture",
      "name": "fixture",
      "object": "fixture",
      "created_at": "fixture",
      "last_updated_at": "fixture",
      "is_first_generation_done": 0,
      "disable_portkey_gateway": 0,
      "settings": {
        "dashboard_gateway_url": "fixture",
        "dashboard_mcp_gateway_url": "fixture",
        "debug_log": 0,
        "user_api_key_count": 0,
        "user_api_key_ttl": 0,
        "user_api_key_max_ttl": 0,
        "user_key_metadata": 0,
        "user_key_metadata_override": 0,
        "user_api_key_rotation_period": null,
        "user_api_key_auto_create": false,
        "default_workspace_user_api_key_auto_create": false
      },
      "benefits": {
        "limits": {},
        "featureFlags": {}
      },
      "subscription": {
        "type": "fixture",
        "name": "fixture"
      }
    },
    "requestSchema": null,
    "responseSchema": "GatewayOrganisationInfoSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "organisations.get_self",
    "method": "GET",
    "plane": "admin",
    "path": "/organisations/self",
    "query": [],
    "body": null,
    "response": {
      "success": false,
      "data": {}
    },
    "requestSchema": null,
    "responseSchema": "OrganisationSelfResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "organisations.update_self",
    "method": "PUT",
    "plane": "admin",
    "path": "/organisations/self",
    "query": [],
    "body": {
      "type": "fixture.event"
    },
    "response": {},
    "requestSchema": "GatewayOrganisationUpdateRequestSchema",
    "responseSchema": "GatewayWriteResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "organisations.get_auth_settings",
    "method": "GET",
    "plane": "admin",
    "path": "/organisations/12345/auth-settings",
    "query": [],
    "body": null,
    "response": {
      "success": false,
      "data": {}
    },
    "requestSchema": null,
    "responseSchema": "AuthSettingsResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "organisations.update_auth_settings",
    "method": "PUT",
    "plane": "admin",
    "path": "/organisations/12345/auth-settings",
    "query": [],
    "body": {
      "type": "fixture.event"
    },
    "response": {},
    "requestSchema": "GatewayOrganisationAuthSettingsUpdateRequestSchema",
    "responseSchema": "GatewayWriteResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "workspaces.list",
    "method": "GET",
    "plane": "data",
    "path": "/workspaces",
    "query": [],
    "body": null,
    "response": {
      "object": "fixture",
      "total": 0,
      "data": []
    },
    "requestSchema": null,
    "responseSchema": "ListWorkspacesResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "workspaces.get",
    "method": "GET",
    "plane": "data",
    "path": "/workspaces/ws-fixture",
    "query": [],
    "body": null,
    "response": {
      "id": "fixture",
      "name": "fixture",
      "description": null,
      "created_at": "fixture",
      "last_updated_at": "fixture",
      "is_default": 0,
      "slug": "fixture",
      "icon": null,
      "defaults": null,
      "usage_limits": null,
      "rate_limits": null
    },
    "requestSchema": null,
    "responseSchema": "GatewayWorkspaceDetailSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "workspaces.create",
    "method": "POST",
    "plane": "admin",
    "path": "/workspaces",
    "query": [],
    "body": {
      "name": "fixture",
      "scope_name": "fixture"
    },
    "response": {
      "id": "fixture",
      "name": "fixture",
      "slug": "fixture",
      "description": null,
      "created_at": "fixture",
      "last_updated_at": "fixture",
      "scope_name": "fixture",
      "object": "fixture"
    },
    "requestSchema": "GatewayWorkspaceCreateRequestSchema",
    "responseSchema": "GatewayWorkspaceCreateResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "workspaces.update",
    "method": "PUT",
    "plane": "admin",
    "path": "/workspaces/ws-fixture",
    "query": [],
    "body": {
      "name": "fixture"
    },
    "response": {},
    "requestSchema": "GatewayWorkspaceUpdateRequestSchema",
    "responseSchema": "GatewayWriteResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "workspaces.delete",
    "method": "DELETE",
    "plane": "admin",
    "path": "/workspaces/ws-fixture",
    "query": [],
    "body": null,
    "response": null,
    "requestSchema": null,
    "responseSchema": null,
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "log_exports.get",
    "method": "GET",
    "plane": "data",
    "path": "/logs/exports/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "queryObject": {},
    "body": null,
    "response": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "organisation_id": "550e8400-e29b-41d4-a716-446655440000",
      "filters": {
        "time_of_generation_min": "2026-09-06T00:00:00Z",
        "time_of_generation_max": "2026-09-06T00:00:00Z",
        "total_units_min": 1,
        "total_units_max": 1,
        "cost_min": 1,
        "cost_max": 1,
        "ai_model": "x",
        "prompt_token_min": 1,
        "prompt_token_max": 1,
        "completion_token_min": 1,
        "completion_token_max": 1,
        "status_code": "x",
        "metadata": {},
        "ai_org_model": "x",
        "weighted_feedback_min": 1,
        "weighted_feedback_max": 1,
        "virtual_keys": "x",
        "trace_id": "x",
        "configs": "x",
        "workspace_slug": "x",
        "prompt_slug": "x",
        "page_size": 1,
        "current_page": 1
      },
      "requested_data": [
        "id"
      ],
      "status": "draft",
      "description": "x",
      "created_at": "2026-09-06T00:00:00Z",
      "last_updated_at": "2026-09-06T00:00:00Z",
      "created_by": "550e8400-e29b-41d4-a716-446655440000",
      "workspace_id": "550e8400-e29b-41d4-a716-446655440000",
      "object": "export"
    },
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/17"
    }
  },
  {
    "name": "log_exports.update",
    "method": "PUT",
    "plane": "data",
    "path": "/logs/exports/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "queryObject": {},
    "body": {
      "workspace_id": "x",
      "filters": {
        "time_of_generation_min": "2026-09-06T00:00:00Z",
        "time_of_generation_max": "2026-09-06T00:00:00Z",
        "total_units_min": 1,
        "total_units_max": 1,
        "cost_min": 1,
        "cost_max": 1,
        "ai_model": "x",
        "prompt_token_min": 1,
        "prompt_token_max": 1,
        "completion_token_min": 1,
        "completion_token_max": 1,
        "status_code": "x",
        "metadata": {},
        "ai_org_model": "x",
        "weighted_feedback_min": 1,
        "weighted_feedback_max": 1,
        "virtual_keys": "x",
        "trace_id": "x",
        "configs": "x",
        "workspace_slug": "x",
        "prompt_slug": "x",
        "page_size": 1,
        "current_page": 1
      },
      "requested_data": [
        "id"
      ],
      "description": "Offline contract fixture"
    },
    "response": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "total": 1,
      "object": "export"
    },
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/18"
    }
  },
  {
    "name": "log_exports.list",
    "method": "GET",
    "plane": "data",
    "path": "/logs/exports",
    "query": [
      [
        "workspace_id",
        "550e8400-e29b-41d4-a716-446655440000"
      ]
    ],
    "queryObject": {
      "workspace_id": "550e8400-e29b-41d4-a716-446655440000"
    },
    "body": null,
    "response": {
      "object": "list",
      "total": 1,
      "data": [
        {
          "id": "550e8400-e29b-41d4-a716-446655440000",
          "organisation_id": "550e8400-e29b-41d4-a716-446655440000",
          "filters": {
            "time_of_generation_min": "2026-09-06T00:00:00Z",
            "time_of_generation_max": "2026-09-06T00:00:00Z",
            "total_units_min": 1,
            "total_units_max": 1,
            "cost_min": 1,
            "cost_max": 1,
            "ai_model": "x",
            "prompt_token_min": 1,
            "prompt_token_max": 1,
            "completion_token_min": 1,
            "completion_token_max": 1,
            "status_code": "x",
            "metadata": {},
            "ai_org_model": "x",
            "weighted_feedback_min": 1,
            "weighted_feedback_max": 1,
            "virtual_keys": "x",
            "trace_id": "x",
            "configs": "x",
            "workspace_slug": "x",
            "prompt_slug": "x",
            "page_size": 1,
            "current_page": 1
          },
          "requested_data": [
            "id"
          ],
          "status": "draft",
          "description": "x",
          "created_at": "2026-09-06T00:00:00Z",
          "last_updated_at": "2026-09-06T00:00:00Z",
          "created_by": "550e8400-e29b-41d4-a716-446655440000",
          "workspace_id": "550e8400-e29b-41d4-a716-446655440000",
          "object": "export"
        }
      ]
    },
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/19"
    }
  },
  {
    "name": "log_exports.create",
    "method": "POST",
    "plane": "data",
    "path": "/logs/exports",
    "query": [],
    "queryObject": {},
    "body": {
      "workspace_id": "x",
      "filters": {
        "time_of_generation_min": "2026-09-06T00:00:00Z",
        "time_of_generation_max": "2026-09-06T00:00:00Z",
        "total_units_min": 1,
        "total_units_max": 1,
        "cost_min": 1,
        "cost_max": 1,
        "ai_model": "x",
        "prompt_token_min": 1,
        "prompt_token_max": 1,
        "completion_token_min": 1,
        "completion_token_max": 1,
        "status_code": "x",
        "metadata": {},
        "ai_org_model": "x",
        "weighted_feedback_min": 1,
        "weighted_feedback_max": 1,
        "virtual_keys": "x",
        "trace_id": "x",
        "configs": "x",
        "workspace_slug": "x",
        "prompt_slug": "x",
        "page_size": 1,
        "current_page": 1
      },
      "requested_data": [
        "id"
      ],
      "description": "Offline contract fixture"
    },
    "response": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "total": 1,
      "object": "export"
    },
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/20"
    }
  },
  {
    "name": "log_exports.start",
    "method": "POST",
    "plane": "data",
    "path": "/logs/exports/550e8400-e29b-41d4-a716-446655440000/start",
    "query": [],
    "queryObject": {},
    "body": null,
    "response": {
      "message": "x",
      "object": "export"
    },
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/21"
    }
  },
  {
    "name": "log_exports.cancel",
    "method": "POST",
    "plane": "data",
    "path": "/logs/exports/550e8400-e29b-41d4-a716-446655440000/cancel",
    "query": [],
    "queryObject": {},
    "body": null,
    "response": {
      "message": "x",
      "object": "export"
    },
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/22"
    }
  },
  {
    "name": "log_exports.download",
    "method": "GET",
    "plane": "data",
    "path": "/logs/exports/550e8400-e29b-41d4-a716-446655440000/download",
    "query": [],
    "queryObject": {},
    "body": null,
    "response": {
      "signed_url": "https://example.test"
    },
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/23"
    }
  },
  {
    "name": "rate_limits.create",
    "method": "POST",
    "plane": "data",
    "path": "/policies/rate-limits",
    "query": [],
    "queryObject": {},
    "body": {
      "name": "x",
      "conditions": [
        {
          "key": "x",
          "value": "x",
          "excludes": "x"
        }
      ],
      "group_by": [
        {
          "key": "x"
        }
      ],
      "type": "requests",
      "unit": "rpm",
      "value": 1,
      "target": "llm",
      "workspace_id": "x",
      "organisation_id": "550e8400-e29b-41d4-a716-446655440000"
    },
    "response": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "object": "x"
    },
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/31"
    }
  },
  {
    "name": "rate_limits.list",
    "method": "GET",
    "plane": "data",
    "path": "/policies/rate-limits",
    "query": [
      [
        "workspace_id",
        "550e8400-e29b-41d4-a716-446655440000"
      ],
      [
        "status",
        "active"
      ],
      [
        "type",
        "requests"
      ],
      [
        "unit",
        "rpm"
      ],
      [
        "target",
        "llm"
      ],
      [
        "page_size",
        "0"
      ],
      [
        "current_page",
        "0"
      ]
    ],
    "queryObject": {
      "workspace_id": "550e8400-e29b-41d4-a716-446655440000",
      "status": "active",
      "type": "requests",
      "unit": "rpm",
      "target": "llm",
      "page_size": 0,
      "current_page": 0
    },
    "body": null,
    "response": {
      "object": "x",
      "data": [
        {
          "id": "550e8400-e29b-41d4-a716-446655440000",
          "name": "x",
          "conditions": [
            {
              "key": "x",
              "value": "x",
              "excludes": "x"
            }
          ],
          "group_by": [
            {
              "key": "x"
            }
          ],
          "type": "requests",
          "unit": "rpm",
          "value": 1,
          "status": "active",
          "workspace_id": "550e8400-e29b-41d4-a716-446655440000",
          "organisation_id": "550e8400-e29b-41d4-a716-446655440000",
          "created_at": "2026-09-06T00:00:00Z",
          "last_updated_at": "2026-09-06T00:00:00Z",
          "target": "llm"
        }
      ],
      "total": 1
    },
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/32"
    }
  },
  {
    "name": "rate_limits.get",
    "method": "GET",
    "plane": "data",
    "path": "/policies/rate-limits/550e8400-e29b-41d4-a716-446655440000",
    "query": [
      [
        "status",
        "active"
      ]
    ],
    "queryObject": {
      "status": "active"
    },
    "body": null,
    "response": {
      "id": "550e8400-e29b-41d4-a716-446655440000",
      "name": "x",
      "conditions": [
        {
          "key": "x",
          "value": "x",
          "excludes": "x"
        }
      ],
      "group_by": [
        {
          "key": "x"
        }
      ],
      "type": "requests",
      "unit": "rpm",
      "value": 1,
      "status": "active",
      "workspace_id": "550e8400-e29b-41d4-a716-446655440000",
      "organisation_id": "550e8400-e29b-41d4-a716-446655440000",
      "created_at": "2026-09-06T00:00:00Z",
      "last_updated_at": "2026-09-06T00:00:00Z",
      "target": "llm",
      "object": "x"
    },
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/33"
    }
  },
  {
    "name": "rate_limits.update",
    "method": "PUT",
    "plane": "data",
    "path": "/policies/rate-limits/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "queryObject": {},
    "body": {
      "name": "x",
      "unit": "rpm",
      "value": 1,
      "conditions": [
        {
          "key": "x",
          "value": "x",
          "excludes": "x"
        }
      ]
    },
    "response": {},
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/34"
    }
  },
  {
    "name": "rate_limits.delete",
    "method": "DELETE",
    "plane": "data",
    "path": "/policies/rate-limits/550e8400-e29b-41d4-a716-446655440000",
    "query": [],
    "queryObject": {},
    "body": null,
    "response": {},
    "expectedSource": {
      "path": "contracts/fixtures/typescript/test/openapi/fixtures/gateway.json",
      "sha256": "3b1cd910fa88259340478ce103eb3bb1760d31c92514adbe694edf2b5ffad9d6",
      "pointer": "/cases/35"
    }
  },
  {
    "name": "telemetry.error_category_trends",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/error-category-trends",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "summary": {
          "totalErrors": 0,
          "totalUniqueErrorCodes": 0
        },
        "trend": [],
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "ErrorCategoryTrendsResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.grouped_errors",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/grouped-errors",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "trend": [],
        "total": 0,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "GroupedErrorsResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.cost",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/cost",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": 0,
        "avg": 0,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "CostChartResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.requests",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/requests",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": null,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "CountChartResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.latency",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/latency",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": null,
        "p50": null,
        "p90": null,
        "p99": null,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "LatencyChartResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.tokens",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/tokens",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": 0,
        "avg": 0,
        "total_request_units": 0,
        "total_response_units": 0,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "TokensChartResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.errors",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/errors",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": null,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "CountChartResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.users",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/users",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": null,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "CountChartResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.cache_summary",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/cache-summary",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "summary": {
          "cacheHits": 0,
          "avgCacheLatency": null,
          "totalRequests": 0,
          "cacheSpeedup": 0
        },
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "CacheSummaryResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.cache_hit_trend",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/cache-hit-trend",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "trend": [],
        "total": 0,
        "summary": {
          "totalCacheHits": 0,
          "hitRate": 0
        },
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "CacheHitTrendResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.user_trends",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/user-trends",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "summary": {
          "total": 0,
          "unique": 0,
          "avg": 0
        },
        "trend": [],
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "UserTrendsResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.error_trends",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/error-trends",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "summary": {
          "errorPercent": 0
        },
        "trend": [],
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "ErrorTrendsResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.rescued_retries",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/rescued-retries",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "trend": [],
        "total": 0,
        "trends": [],
        "retryTotal": 0,
        "fallbackTotal": 0,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "RescuedRetriesResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.feedback_trend",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/feedback-trend",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": null,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "CountChartResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.feedback_weighted",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/feedback-weighted",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": null,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "CountChartResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.feedback_score_distribution",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/feedback-score-distribution",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": null,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "FeedbackScoreDistributionResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.feedback_models",
    "method": "GET",
    "plane": "data",
    "path": "/logs/charts/feedback-models",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "FeedbackModelsResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.filter_boundaries",
    "method": "GET",
    "plane": "data",
    "path": "/analytics/filter-boundaries",
    "query": [
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "total_units_max": null,
        "total_units_min": null,
        "cost_max": null,
        "cost_min": null,
        "unique_ai_models": [],
        "unique_status_codes": [],
        "unique_environments": [],
        "unique_prompt_ids": [],
        "unique_virtual_keys": [],
        "unique_configs": [],
        "unique_api_keys": [],
        "unique_metadata_keys": [],
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "GatewayFilterBoundariesResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.by_user",
    "method": "GET",
    "plane": "data",
    "path": "/logs/groups/users",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": 0,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "UserGroupResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.group_by",
    "method": "GET",
    "plane": "data",
    "path": "/logs/groups/model",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "object": "fixture",
      "is_quota_exceeded": false,
      "total": 0,
      "data": []
    },
    "responseSchema": "GroupListResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.by_status_code",
    "method": "GET",
    "plane": "data",
    "path": "/logs/groups/status_code",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "object": "fixture",
      "is_quota_exceeded": false,
      "total": 0,
      "data": []
    },
    "responseSchema": "GroupListResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  },
  {
    "name": "telemetry.logs",
    "method": "GET",
    "plane": "data",
    "path": "/logs",
    "query": [
      [
        "organisationId",
        "12345"
      ],
      [
        "workspaceSlug",
        "ws-fixture"
      ],
      [
        "timeOfGenerationMin",
        "2026-01-01T00:00:00+00:00"
      ],
      [
        "timeOfGenerationMax",
        "2026-01-02T00:00:00+00:00"
      ]
    ],
    "body": null,
    "response": {
      "success": false,
      "data": {
        "records": [],
        "total": 0,
        "capturedTotal": 0,
        "isQuotaExceeded": false
      }
    },
    "responseSchema": "GatewayLogsResponseSchema",
    "expectedSource": "literal source-derived specimen; verified by frozen Zod checker"
  }
]"#;
