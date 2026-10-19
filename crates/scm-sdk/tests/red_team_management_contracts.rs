//! Independent management 0.86.0 fixtures. Configuration strings are data, never executed.
#![cfg(feature = "red-team")]
use bytes::Bytes;
use futures_util::{future::BoxFuture, stream};
use http::{HeaderMap, StatusCode};
use scm_sdk::{
    Client, Endpoint, RequestOptions, Result, Secret,
    airs::red_team::{RedTeamTargetsClient, TargetListOptions, models::*},
    auth::{AuthContext, Authentication, TokenLease, TokenProvider},
    pagination::CollectOptions,
    transport::{Transport, TransportRequest, TransportResponse},
    validation::{self, Mode},
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
const ID: &str = "550e8400-e29b-41d4-a716-446655440000";
const SECRET: &str = "synthetic-management-password-never-log";
fn target() -> Value {
    // Deliberately loose IDs/dates prove G25 remains active.
    json!({"uuid":"target-1","tsg_id":"12345","name":"synthetic-target","status":"FUTURE","active":false,"validated":false,"created_at":"2026-01-01","updated_at":"yesterday","future":null})
}
fn vertex() -> Value {
    json!({"project_id":"synthetic-project","location":"us-central1","engine_id":"12345","customer_sa_email":"synthetic@example.test"})
}
fn configured(mut provider: Value, multi: Value, redacted: bool) -> Value {
    if redacted && provider.get("customer_sa_email").is_some() {
        provider["customer_sa_email"] = json!("********");
    }
    let mut doc = target();
    doc["connection_type"] = json!("VERTEX_AI");
    doc["response_mode"] = json!("REALTIME");
    doc["connection_params"] = json!({"api_endpoint":"wss://target.example.test/ws","target_connection_config":provider,"multi_turn_config":multi,"audio_request_json":{"audio":"{AUDIO}","future":null},"audio_curl":SECRET,"future_connection":null});
    doc["auth_config"] = json!({"basic_username":"synthetic-user","basic_password":if redacted {"********"} else {SECRET},"basic_auth_header":null});
    doc
}
fn options(mode: Mode) -> RequestOptions {
    let result = RequestOptions::with_timeout(Duration::from_secs(5)).unwrap();
    if mode == Mode::ResponseWithoutDefaults {
        result.without_response_defaults()
    } else {
        result
    }
}
#[test]
fn vertex_client_session_and_new_values_preserve_both_response_modes() {
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        for connection in ["VERTEX_AI", "N8N", "FUTURE_PROVIDER"] {
            for redacted in [false, true] {
                let mut doc = configured(vertex(), json!({"type":"client_session"}), redacted);
                doc["connection_type"] = json!(connection);
                let parsed = validation::parse("TargetResponseSchema", doc.clone(), mode).unwrap();
                let typed = TargetResponse::parse(doc.clone(), mode).unwrap();
                assert!(!format!("{typed:?}").contains(SECRET));
                let actual = serde_json::to_value(typed).unwrap();
                assert_eq!(actual, parsed);
                assert_eq!(
                    actual["connection_params"]["multi_turn_config"],
                    json!({"type":"client_session"})
                );
                if mode == Mode::ResponseWithoutDefaults {
                    assert_eq!(actual, doc);
                }
            }
        }
    }
}
#[test]
fn explicit_client_session_wins_over_legacy_structural_fields() {
    let multi = json!({"type":"client_session","response_id_field":"id","request_id_field":"previous","future":null});
    let selected: StreamingConnectionParamsMultiTurnConfig =
        serde_json::from_value(multi.clone()).unwrap();
    assert!(matches!(
        selected,
        StreamingConnectionParamsMultiTurnConfig::Variant3(_)
    ));
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        let doc = configured(vertex(), multi.clone(), false);
        let actual = serde_json::to_value(TargetResponse::parse(doc, mode).unwrap()).unwrap();
        assert_eq!(actual["connection_params"]["multi_turn_config"], multi);
    }
}

#[test]
fn request_new_variants_and_vertex_patterns_are_strict() {
    for connection in ["VERTEX_AI", "N8N"] {
        let doc = json!({"name":"synthetic-target","target_type":"APPLICATION","connection_type":connection,"response_mode":"REALTIME","connection_params":{"target_connection_config":vertex(),"multi_turn_config":{"type":"client_session"}}});
        validation::parse("TargetCreateRequestSchema", doc.clone(), Mode::Request).unwrap();
        let typed: TargetCreateRequest = serde_json::from_value(doc.clone()).unwrap();
        assert_eq!(serde_json::to_value(typed).unwrap(), doc);
    }
    for (field, value) in [
        ("connection_type", "FUTURE_PROVIDER"),
        ("response_mode", "FUTURE_MODE"),
    ] {
        let mut request = json!({"name":"synthetic-target","target_type":"APPLICATION"});
        request[field] = json!(value);
        assert!(validation::parse("TargetCreateRequestSchema", request, Mode::Request).is_err());
    }
    for (field, invalid) in [
        ("project_id", "INVALID_PROJECT!"),
        ("location", "INVALID_REGION!"),
        ("engine_id", "not-digits"),
    ] {
        let mut doc = configured(vertex(), json!({"type":"client_session"}), false);
        doc["connection_params"]["target_connection_config"][field] = json!(invalid);
        let request = json!({"name":"synthetic","target_type":"APPLICATION","connection_type":"VERTEX_AI","connection_params":doc["connection_params"]});
        let error =
            validation::parse("TargetCreateRequestSchema", request, Mode::Request).unwrap_err();
        assert!(!format!("{error:?} {error}").contains(SECRET));
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            let actual =
                serde_json::to_value(TargetResponse::parse(doc.clone(), mode).unwrap()).unwrap();
            assert_eq!(
                actual["connection_params"]["target_connection_config"][field],
                invalid
            );
        }
    }
}
#[test]
fn unknown_discriminators_and_required_vertex_types_stay_validated() {
    for multi in [
        json!({"type":"future_session"}),
        json!({"type":"client_session","future":null}),
    ] {
        let doc = configured(vertex(), multi.clone(), false);
        for mode in [Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults] {
            let result = validation::parse("TargetResponseSchema", doc.clone(), mode);
            if multi["type"] == "future_session" {
                assert!(result.is_err());
            } else if mode != Mode::Request {
                assert!(result.is_ok());
            }
        }
    }
    for replacement in [Value::Null, json!(17), json!([])] {
        let mut doc = configured(vertex(), json!({"type":"client_session"}), false);
        doc["connection_params"]["target_connection_config"]["engine_id"] = replacement;
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            assert!(validation::parse("TargetResponseSchema", doc.clone(), mode).is_err());
        }
    }
    let mut doc = configured(vertex(), json!({"type":"client_session"}), false);
    doc["connection_params"]["target_connection_config"]
        .as_object_mut()
        .unwrap()
        .remove("engine_id");
    assert!(TargetResponse::parse(doc, Mode::ResponseWithoutDefaults).is_err());
}
#[test]
fn missing_defaults_and_explicit_null_are_distinct() {
    let mut doc = target();
    doc["connection_params"] = json!({"api_endpoint":"wss://target.example.test/ws","request_headers":null,"request_json":{"nested":null},"audio_request_json":null,"audio_curl":null});
    doc["auth_config"] =
        json!({"basic_auth_header":null,"basic_username":null,"basic_password":null});
    let raw = serde_json::to_value(
        TargetResponse::parse(doc.clone(), Mode::ResponseWithoutDefaults).unwrap(),
    )
    .unwrap();
    assert_eq!(raw, doc);
    let parity = serde_json::to_value(TargetResponse::parse(doc, Mode::Response).unwrap()).unwrap();
    assert_eq!(parity["connection_params"]["ws_response_timeout"], 110);
    assert_eq!(parity["auth_config"]["basic_auth_location"], "HEADER");
    assert!(parity["auth_config"]["basic_password"].is_null());
}
#[test]
fn new_projected_fields_keep_previous_passthrough_values() {
    for (section, field, value) in [
        ("auth_config", "basic_username", json!(17)),
        ("auth_config", "basic_password", json!([SECRET])),
        ("connection_params", "audio_request_json", json!(17)),
        (
            "connection_params",
            "audio_curl",
            json!({"sensitive":SECRET}),
        ),
    ] {
        let mut doc = configured(vertex(), json!({"type":"client_session"}), false);
        doc[section][field] = value.clone();
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            let typed = TargetResponse::parse(doc.clone(), mode).unwrap();
            assert!(!format!("{typed:?}").contains(SECRET));
            assert_eq!(serde_json::to_value(typed).unwrap()[section][field], value);
        }
    }
}
#[test]
fn legacy_multi_turn_resolution_and_hybrid_provider_precedence_remain() {
    for multi in [
        json!({}),
        json!({"type":"stateless"}),
        json!({"response_id_field":"id"}),
    ] {
        let doc = configured(vertex(), multi.clone(), false);
        let actual =
            serde_json::to_value(TargetResponse::parse(doc, Mode::Response).unwrap()).unwrap();
        assert_eq!(actual["connection_params"]["multi_turn_config"], multi);
        let selected: StreamingConnectionParamsMultiTurnConfig =
            serde_json::from_value(multi).unwrap();
        assert!(matches!(
            selected,
            StreamingConnectionParamsMultiTurnConfig::Variant2(_)
        ));
    }
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        let doc = configured(
            vertex(),
            json!({"type":"stateful","response_id_field":"id","request_id_field":"previous_id"}),
            false,
        );
        let actual =
            serde_json::to_value(TargetResponse::parse(doc.clone(), mode).unwrap()).unwrap();
        assert_eq!(
            actual["connection_params"]["multi_turn_config"]["type"],
            "stateful"
        );
        let mut hybrid = vertex();
        hybrid["api_key"] = json!(SECRET);
        hybrid["model_name"] = json!("synthetic-model");
        let provider: StreamingConnectionParamsTargetConnectionConfig =
            serde_json::from_value(hybrid.clone()).unwrap();
        assert!(matches!(
            provider,
            StreamingConnectionParamsTargetConnectionConfig::Variant1(_)
        ));
        let doc = configured(hybrid.clone(), json!({"type":"stateless"}), false);
        let actual = serde_json::to_value(TargetResponse::parse(doc, mode).unwrap()).unwrap();
        assert_eq!(
            actual["connection_params"]["target_connection_config"],
            hybrid
        );
    }
}
#[test]
fn untagged_complete_stateful_config_keeps_frozen_branch_selection() {
    let doc = configured(
        vertex(),
        json!({"response_id_field":"id","request_id_field":"previous_id"}),
        false,
    );
    let parsed = TargetResponse::parse(doc.clone(), Mode::Response).unwrap();
    assert_eq!(
        serde_json::to_value(parsed).unwrap()["connection_params"]["multi_turn_config"],
        doc["connection_params"]["multi_turn_config"]
    );
    let selected: StreamingConnectionParamsMultiTurnConfig =
        serde_json::from_value(doc["connection_params"]["multi_turn_config"].clone()).unwrap();
    assert!(matches!(
        selected,
        StreamingConnectionParamsMultiTurnConfig::Variant1(_)
    ));
    assert_eq!(
        serde_json::to_value(
            TargetResponse::parse(doc.clone(), Mode::ResponseWithoutDefaults).unwrap()
        )
        .unwrap(),
        doc
    );
}

struct Provider {
    context: AuthContext,
    calls: AtomicUsize,
}
impl TokenProvider for Provider {
    fn token<'a>(&'a self, _: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            TokenLease::new(
                Secret::new("synthetic-management-bearer").unwrap(),
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
struct Wire {
    replies: Mutex<VecDeque<Value>>,
    requests: Mutex<Vec<TransportRequest>>,
}
impl Transport for Wire {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            assert!(!format!("{request:?}").contains(SECRET));
            self.requests.lock().unwrap().push(request);
            let value = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected request");
            Ok(TransportResponse::new(
                StatusCode::OK,
                HeaderMap::new(),
                Box::pin(stream::iter([Ok(Bytes::from(
                    serde_json::to_vec(&value).unwrap(),
                ))])),
            ))
        })
    }
}
fn setup(replies: Vec<Value>) -> (RedTeamTargetsClient, Arc<Wire>, Arc<Provider>) {
    let wire = Arc::new(Wire {
        replies: Mutex::new(replies.into()),
        requests: Mutex::new(Vec::new()),
    });
    let provider = Arc::new(Provider {
        context: AuthContext::new(
            Endpoint::parse("https://issuer.example/token").unwrap(),
            "synthetic-client",
            1,
            "12345",
        )
        .unwrap(),
        calls: AtomicUsize::new(0),
    });
    let client = Client::builder(Endpoint::parse("https://management.example/mgmt-plane").unwrap())
        .transport(wire.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .build()
        .unwrap();
    (RedTeamTargetsClient::new(client).unwrap(), wire, provider)
}
#[tokio::test]
async fn all_five_operations_preserve_declared_sensitive_response_shapes() {
    for provider in [
        vertex(),
        json!({"api_key":SECRET,"model_name":"synthetic-model"}),
        json!({"auth_type":"ACCESS_TOKEN","access_token":SECRET,"workspace_url":"https://workspace.example.test","model_name":"synthetic-model"}),
        json!({"access_id":"synthetic-id","access_secret":SECRET,"region":"us-east-1","model_id":"synthetic-model"}),
        json!({"client_id":"synthetic-client","client_secret":SECRET,"tenant_id":"synthetic-tenant","schema_name":"synthetic-schema","environment_id":"synthetic-env"}),
    ] {
        for auth in [
            json!({"basic_username":"synthetic-user","basic_password":SECRET,"basic_auth_header":null}),
            json!({"basic_auth_header":{"Authorization":SECRET}}),
            json!({"auth_header":{"Authorization":SECRET}}),
            json!({"oauth2_token_url":"https://issuer.example.test/token","oauth2_headers":{"Authorization":SECRET},"oauth2_body_params":{"client_secret":SECRET},"oauth2_inject_header":{"Authorization":"Bearer {token}"}}),
        ] {
            for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
                let mut responses = Vec::new();
                for redacted in [true, false, false, false, false] {
                    let mut response =
                        configured(provider.clone(), json!({"type":"client_session"}), redacted);
                    response["auth_config"] = auth.clone();
                    response["connection_type"] = json!(if provider.get("project_id").is_some() {
                        "VERTEX_AI"
                    } else if provider.get("api_key").is_some() {
                        "OPENAI"
                    } else if provider.get("workspace_url").is_some() {
                        "DATABRICKS"
                    } else if provider.get("access_id").is_some() {
                        "BEDROCK"
                    } else {
                        "MS_COPILOT_STUDIO"
                    });
                    response["auth_type"] = json!(if auth.get("oauth2_token_url").is_some() {
                        "OAUTH2"
                    } else if auth.get("auth_header").is_some() {
                        "HEADERS"
                    } else {
                        "BASIC_AUTH"
                    });
                    if redacted {
                        // Model an upstream masked GET without altering mutation responses.
                        response = serde_json::from_str(
                            &serde_json::to_string(&response)
                                .unwrap()
                                .replace(SECRET, "********"),
                        )
                        .unwrap();
                    }
                    responses.push(response);
                }
                let (client, wire, _) = setup(responses.clone());
                let opts = options(mode);
                let create =
                    serde_json::from_value(json!({"name":"synthetic","target_type":"APPLICATION"}))
                        .unwrap();
                let update =
                    serde_json::from_value(json!({"name":"synthetic","target_type":"APPLICATION"}))
                        .unwrap();
                let probe = serde_json::from_value(
                    json!({"name":"synthetic","uuid":ID,"probe_fields":["multi_turn"]}),
                )
                .unwrap();
                let profile =
                    serde_json::from_value(json!({"target_background":{"industry":"synthetic"}}))
                        .unwrap();
                let actual = [
                    client.get(ID, Some(&opts)).await.unwrap(),
                    client.create(&create, None, Some(&opts)).await.unwrap(),
                    client.update(ID, &update, None, Some(&opts)).await.unwrap(),
                    client.probe(&probe, Some(&opts)).await.unwrap(),
                    client
                        .update_profile(ID, &profile, Some(&opts))
                        .await
                        .unwrap(),
                ];
                for (actual, expected) in actual.into_iter().zip(responses) {
                    assert!(!format!("{actual:?}").contains(SECRET));
                    assert_eq!(
                        serde_json::to_value(actual).unwrap(),
                        validation::parse("TargetResponseSchema", expected, mode).unwrap()
                    );
                }
                let requests = wire.requests.lock().unwrap();
                let names: Vec<_> = requests.iter().map(TransportRequest::operation).collect();
                assert_eq!(
                    names,
                    [
                        "red_team.targets.get",
                        "red_team.targets.create",
                        "red_team.targets.update",
                        "red_team.targets.probe",
                        "red_team.targets.update_profile"
                    ]
                );
                for (request, (method, suffix)) in requests.iter().zip([
                    ("GET", format!("/v1/target/{ID}")),
                    ("POST", "/v1/target".to_owned()),
                    ("PUT", format!("/v1/target/{ID}")),
                    ("POST", "/v1/target/probe".to_owned()),
                    ("PUT", format!("/v1/target/{ID}/profile")),
                ]) {
                    assert_eq!(request.method().as_str(), method);
                    assert_eq!(request.url().path(), format!("/mgmt-plane{suffix}"));
                }
                assert!(wire.replies.lock().unwrap().is_empty());
            }
        }
    }
}
fn page(id: &str) -> Value {
    let mut item = target();
    item["uuid"] = json!(id);
    json!({"pagination":{"total_items":2},"data":[item]})
}
#[tokio::test]
async fn profiling_filter_propagates_through_every_page_and_collection_surface() {
    for profiling_status in ["NOT_STARTED", "IN_PROGRESS", "COMPLETED", "FAILED"] {
        for surface in ["list", "pages", "items", "all"] {
            let (client, wire, _) = setup(if surface == "list" {
                vec![page("first")]
            } else {
                vec![page("first"), page("second")]
            });
            let mut params = TargetListOptions::default();
            params.limit = Some(1);
            params.profiling_status = Some(profiling_status.into());
            params.adapter_uuid = Some(ID.into());
            params.target_type = Some("APPLICATION".into());
            params.search = Some("synthetic & target".into());
            params.status = Some("QUEUED".into());
            match surface {
                "list" => {
                    client.list(&params, None).await.unwrap();
                }
                "pages" => {
                    let mut pages = client.list_pages(&params, 3, None).unwrap();
                    assert!(pages.next_page().await.unwrap().is_some());
                    assert!(pages.next_page().await.unwrap().is_some());
                    assert!(pages.next_page().await.unwrap().is_none());
                }
                "items" => {
                    let mut items = client.list_items(&params, 3, None).unwrap();
                    assert!(items.next().await.unwrap().is_some());
                    assert!(items.next().await.unwrap().is_some());
                    assert!(items.next().await.unwrap().is_none());
                }
                _ => {
                    assert_eq!(
                        client
                            .list_all(&params, CollectOptions::default(), None)
                            .await
                            .unwrap()
                            .items
                            .len(),
                        2
                    );
                }
            }
            let requests = wire.requests.lock().unwrap();
            for (index, request) in requests.iter().enumerate() {
                let pairs: Vec<_> = request
                    .url()
                    .query_pairs()
                    .map(|(key, value)| (key.into_owned(), value.into_owned()))
                    .collect();
                let mut expected = std::collections::BTreeMap::from([
                    ("adapter_uuid".to_owned(), ID.to_owned()),
                    ("target_type".to_owned(), "APPLICATION".to_owned()),
                    ("search".to_owned(), "synthetic & target".to_owned()),
                    ("status".to_owned(), "QUEUED".to_owned()),
                    ("profiling_status".to_owned(), profiling_status.to_owned()),
                    ("limit".to_owned(), "1".to_owned()),
                ]);
                if surface != "list" {
                    expected.insert("skip".into(), index.to_string());
                }
                // Compare counts too: collecting into a map alone would hide duplicates.
                assert_eq!(pairs.len(), expected.len());
                assert_eq!(
                    pairs
                        .into_iter()
                        .collect::<std::collections::BTreeMap<_, _>>(),
                    expected
                );
            }
            assert!(wire.replies.lock().unwrap().is_empty());
        }
    }
}
#[tokio::test]
async fn invalid_profiling_filter_never_acquires_token_or_dispatches() {
    for status in ["QUEUED", "PARTIALLY_COMPLETE", "", "FUTURE", "ACTIVE"] {
        let (client, wire, provider) = setup(vec![]);
        let mut params = TargetListOptions::default();
        params.profiling_status = Some(status.into());
        assert!(client.list(&params, None).await.is_err());
        assert!(client.list_pages(&params, 2, None).is_err());
        assert!(client.list_items(&params, 2, None).is_err());
        assert!(
            client
                .list_all(&params, CollectOptions::default(), None)
                .await
                .is_err()
        );
        assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
        assert!(wire.requests.lock().unwrap().is_empty());
    }
}

#[tokio::test]
async fn malformed_target_response_errors_never_echo_credentials() {
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        let mut response = configured(vertex(), json!({"type":"client_session"}), false);
        response["active"] = json!(SECRET);
        let (client, _, _) = setup(vec![response]);
        let error = client.get(ID, Some(&options(mode))).await.unwrap_err();
        assert_eq!(error.operation(), Some("red_team.targets.get"));
        assert!(!format!("{error:?} {error}").contains(SECRET));
    }
}

#[test]
fn typed_projection_values_override_raw_collisions_and_unset_restores_them() {
    use scm_sdk::data::Patch;
    let raw = json!({"basic_password":{"raw_secret":SECRET}});
    let mut basic: BasicAuthAuthConfig = serde_json::from_value(raw.clone()).unwrap();
    assert!(basic.basic_password.is_unset());
    basic.basic_password = Patch::Value("replacement".into());
    assert_eq!(
        serde_json::to_value(&basic).unwrap()["basic_password"],
        "replacement"
    );
    basic.basic_password = Patch::Null;
    assert!(serde_json::to_value(&basic).unwrap()["basic_password"].is_null());
    basic.basic_password = Patch::Unset;
    assert_eq!(serde_json::to_value(&basic).unwrap(), raw);
    assert!(!format!("{basic:?}").contains(SECRET));

    let raw = json!({"audio_curl":{"raw_secret":SECRET}});
    let mut audio: RestConnectionParams = serde_json::from_value(raw.clone()).unwrap();
    assert!(audio.audio_curl.is_unset());
    audio.audio_curl = Patch::Value("replacement".into());
    assert_eq!(
        serde_json::to_value(&audio).unwrap()["audio_curl"],
        "replacement"
    );
    audio.audio_curl = Patch::Null;
    assert!(serde_json::to_value(&audio).unwrap()["audio_curl"].is_null());
    audio.audio_curl = Patch::Unset;
    assert_eq!(serde_json::to_value(&audio).unwrap(), raw);
    assert!(!format!("{audio:?}").contains(SECRET));
}
