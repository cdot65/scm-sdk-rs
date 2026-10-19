//! Additive management models retain the frozen response and union contracts.
#![cfg(feature = "red-team")]
use scm_sdk::{
    airs::red_team::models::{
        BasicAuthAuthConfig, MultiTurnClientSessionConfig, RestConnectionParams,
        StreamingConnectionParamsMultiTurnConfig, StreamingConnectionParamsTargetConnectionConfig,
        VertexAIConnectionParams, VertexAIConnectionRedactParams,
    },
    validation::{self, Mode},
};
use serde_json::{Value, json};

fn vertex() -> Value {
    json!({"project_id":"project-123","location":"us-central1","engine_id":"1234","customer_sa_email":"service@example.invalid"})
}

#[test]
fn vertex_request_patterns_are_strict_and_responses_preserve_strings() {
    let value = vertex();
    let typed = VertexAIConnectionParams::parse(value.clone(), Mode::Request).unwrap();
    assert_eq!(serde_json::to_value(typed).unwrap(), value);
    for (field, wrong) in [
        ("project_id", "BAD ID"),
        ("location", "a/b"),
        ("engine_id", "opaque-engine"),
    ] {
        let mut value = vertex();
        value[field] = json!(wrong);
        assert!(VertexAIConnectionParams::parse(value.clone(), Mode::Request).is_err());
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            let typed = VertexAIConnectionParams::parse(value.clone(), mode).unwrap();
            assert_eq!(serde_json::to_value(typed).unwrap(), value);
            let redacted = VertexAIConnectionRedactParams::parse(value.clone(), mode).unwrap();
            assert_eq!(serde_json::to_value(redacted).unwrap(), value);
        }
    }
    let mut wrong_type = vertex();
    wrong_type["project_id"] = json!(123);
    assert!(VertexAIConnectionParams::parse(wrong_type, Mode::Response).is_err());
}

#[test]
fn provider_union_appends_vertex_and_preserves_old_hybrid_precedence() {
    let value = vertex();
    let parsed = validation::parse("s010462", value.clone(), Mode::Request).unwrap();
    let typed: StreamingConnectionParamsTargetConnectionConfig =
        serde_json::from_value(parsed).unwrap();
    assert!(matches!(
        typed,
        StreamingConnectionParamsTargetConnectionConfig::Variant5(_)
    ));
    assert_eq!(serde_json::to_value(typed).unwrap(), value);
    let mut hybrid = vertex();
    hybrid["api_key"] = json!("private-fixture-key");
    hybrid["model_name"] = json!("existing-model");
    let typed: StreamingConnectionParamsTargetConnectionConfig = serde_json::from_value(
        validation::parse("s010462", hybrid.clone(), Mode::Request).unwrap(),
    )
    .unwrap();
    assert!(matches!(
        typed,
        StreamingConnectionParamsTargetConnectionConfig::Variant1(_)
    ));
    assert_eq!(serde_json::to_value(typed).unwrap(), hybrid);
}

#[test]
fn client_session_requires_tag_and_legacy_missing_tag_order_stays_intact() {
    assert!(MultiTurnClientSessionConfig::parse(json!({}), Mode::Request).is_err());
    for mode in [Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults] {
        for (value, expected) in [
            (json!({}), 2),
            (
                json!({"response_id_field":"reply.id","request_id_field":"previous"}),
                1,
            ),
            (json!({"type":"client_session","unknown_field":7}), 3),
        ] {
            let parsed = validation::parse("s010504", value.clone(), mode).unwrap();
            assert_eq!(parsed, value);
            let typed: StreamingConnectionParamsMultiTurnConfig =
                serde_json::from_value(parsed).unwrap();
            let actual = match &typed {
                StreamingConnectionParamsMultiTurnConfig::Variant1(_) => 1,
                StreamingConnectionParamsMultiTurnConfig::Variant2(_) => 2,
                StreamingConnectionParamsMultiTurnConfig::Variant3(_) => 3,
                _ => 0,
            };
            assert_eq!(actual, expected);
            assert_eq!(serde_json::to_value(typed).unwrap(), value);
        }
    }
}

#[test]
fn formerly_unknown_audio_and_basic_fields_keep_all_json_values() {
    for value in [
        json!(null),
        json!(9),
        json!(["secret"]),
        json!({"secret":"fixture"}),
        json!("fixture"),
    ] {
        for mode in [Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults] {
            let raw = json!({"audio_request_json":value,"audio_curl":value});
            let typed = RestConnectionParams::parse(raw.clone(), mode).unwrap();
            assert_eq!(serde_json::to_value(&typed).unwrap(), raw);
            assert!(!format!("{typed:?}").contains("fixture"));
            let raw = json!({"basic_username":value,"basic_password":value});
            let normalized =
                validation::parse("BasicAuthAuthConfigSchema", raw.clone(), mode).unwrap();
            let typed = BasicAuthAuthConfig::parse(raw, mode).unwrap();
            assert_eq!(serde_json::to_value(&typed).unwrap(), normalized);
            assert!(!format!("{typed:?}").contains("fixture"));
        }
    }
}

#[test]
fn runtime_connection_and_response_mode_vocabulary_are_distinct() {
    for value in ["VERTEX_AI", "N8N"] {
        assert_eq!(
            validation::parse("s012061", json!(value), Mode::Request).unwrap(),
            json!(value)
        );
    }
    assert!(validation::parse("s012061", json!("REALTIME"), Mode::Request).is_err());
    assert_eq!(
        validation::parse("s012067", json!("REALTIME"), Mode::Request).unwrap(),
        json!("REALTIME")
    );
}

#[test]
fn current_schema_enums_have_named_variants_and_frozen_constants_keep_unknowns() {
    use scm_sdk::airs::red_team::models::{
        ResponseMode, TargetConnectionType, TargetCreateRequestConnectionType,
        TargetCreateRequestResponseMode,
    };
    assert!(matches!(
        serde_json::from_value::<TargetCreateRequestConnectionType>(json!("VERTEX_AI")).unwrap(),
        TargetCreateRequestConnectionType::VERTEXAI
    ));
    assert!(matches!(
        serde_json::from_value::<TargetCreateRequestConnectionType>(json!("N8N")).unwrap(),
        TargetCreateRequestConnectionType::N8N
    ));
    assert!(matches!(
        serde_json::from_value::<TargetCreateRequestResponseMode>(json!("REALTIME")).unwrap(),
        TargetCreateRequestResponseMode::REALTIME
    ));
    assert!(
        matches!(serde_json::from_value::<TargetCreateRequestConnectionType>(json!("future-provider")).unwrap(), TargetCreateRequestConnectionType::Unknown(value) if value == "future-provider")
    );
    assert!(
        matches!(serde_json::from_value::<TargetCreateRequestResponseMode>(json!("future-mode")).unwrap(), TargetCreateRequestResponseMode::Unknown(value) if value == "future-mode")
    );
    assert!(
        matches!(serde_json::from_value::<TargetConnectionType>(json!("VERTEX_AI")).unwrap(), TargetConnectionType::Unknown(value) if value == "VERTEX_AI")
    );
    assert!(
        matches!(serde_json::from_value::<ResponseMode>(json!("REALTIME")).unwrap(), ResponseMode::Unknown(value) if value == "REALTIME")
    );
}
