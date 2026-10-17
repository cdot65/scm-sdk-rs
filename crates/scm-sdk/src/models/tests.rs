use super::{agentguard::AgentGuardScanStats, dlp::*, scan::*, shared::*};
use crate::validation::Mode;
use serde_json::json;

#[test]
fn enum_unknown_values_roundtrip_and_never_format_payloads() {
    let outcome: Verdict = serde_json::from_value(json!("future-sensitive-outcome")).unwrap();
    assert!(matches!(outcome, Verdict::Unknown(_)));
    assert_eq!(
        serde_json::to_value(&outcome).unwrap(),
        json!("future-sensitive-outcome")
    );
    assert!(!format!("{outcome:?}").contains("future-sensitive"));
}

#[test]
fn known_fields_and_unknown_extensions_roundtrip_with_redacted_debug() {
    let value = json!({"report_id":"report-secret","scan_id":"scan-secret","category":"future-category","action":"future-action","timeout":false,"error":false,"errors":[],"unknown-sensitive-key":{"private":"prompt-secret"}});
    let response: ScanResponse = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(&response).unwrap(), value);
    assert!(!format!("{response:?}").contains("secret"));
    assert!(!format!("{:?}", response.extensions).contains("sensitive"));
}

#[test]
fn extensions_cannot_duplicate_or_override_known_fields() {
    let mut metadata = Metadata::builder()
        .app_name(Optional::Value("original".into()))
        .build()
        .unwrap();
    metadata.extensions.insert(
        "app_name".into(),
        JsonValue::from(json!("replacement-secret")),
    );
    assert!(serde_json::to_value(&metadata).is_err());
    metadata
        .extensions
        .insert("agent_meta".into(), JsonValue::from(json!(null)));
    assert!(serde_json::to_value(&metadata).is_err());
}

#[test]
fn optional_nonnull_fields_reject_null_and_preserve_omission() {
    assert!(serde_json::from_value::<ToolEvent>(json!({"input":null})).is_err());
    let event: ToolEvent = serde_json::from_value(json!({"output":"answer-secret"})).unwrap();
    assert!(event.input.is_unset());
    assert_eq!(
        serde_json::to_value(event).unwrap(),
        json!({"output":"answer-secret"})
    );
}

#[test]
fn required_nullable_fields_accept_null_and_reject_missing() {
    let value = json!({"unique_skills_scanned":{"count":0,"percent_change":null},"total_vulnerabilities_found":{"count":0,"percent_change":null},"top_vulnerability":null});
    let statistics: AgentGuardScanStats = serde_json::from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(statistics).unwrap(), value);
    let missing = json!({"unique_skills_scanned":{"count":0,"percent_change":null},"total_vulnerabilities_found":{"count":0,"percent_change":null}});
    assert!(serde_json::from_value::<AgentGuardScanStats>(missing).is_err());
}

#[test]
fn patch_fields_preserve_omission_clear_and_replacement() {
    let omitted = DataProfilePatchRequest::builder()
        .name("profile".into())
        .profile_type(DataProfileType::Basic)
        .build()
        .unwrap();
    assert_eq!(
        serde_json::to_value(omitted).unwrap(),
        json!({"name":"profile","profile_type":"basic"})
    );
    let cleared = DataProfilePatchRequest::builder()
        .name("profile".into())
        .profile_type(DataProfileType::Basic)
        .description(Patch::Null)
        .build()
        .unwrap();
    assert_eq!(
        serde_json::to_value(cleared).unwrap(),
        json!({"name":"profile","profile_type":"basic","description":null})
    );
    let replaced = DataProfilePatchRequest::builder()
        .name("profile".into())
        .profile_type(DataProfileType::Basic)
        .description(Patch::Value("replacement".into()))
        .build()
        .unwrap();
    assert_eq!(
        serde_json::to_value(replaced).unwrap(),
        json!({"name":"profile","profile_type":"basic","description":"replacement"})
    );
}

#[test]
fn validated_build_checks_semantics_after_presence() {
    assert!(ToolEvent::builder().build().is_ok());
    assert!(ToolEvent::builder().build_validated(Mode::Request).is_err());
    assert!(
        ToolEvent::builder()
            .input(Optional::Value("prompt".into()))
            .build_validated(Mode::Request)
            .is_ok()
    );
}

#[test]
fn binary_file_is_bounded_header_safe_and_not_json() {
    let file = BinaryFile::new(bytes::Bytes::from_static(b"data-secret"))
        .unwrap()
        .with_filename("input.txt")
        .unwrap()
        .with_content_type("text/plain")
        .unwrap();
    assert!(file.validate(10).is_err());
    file.validate(11).unwrap();
    assert!(serde_json::to_value(&file).is_err());
    assert!(!format!("{file:?}").contains("data-secret"));
    for name in [
        "../file",
        "..",
        "bad\r\nX: injected",
        "bad\"quote",
        "bad\\path",
    ] {
        assert!(
            BinaryFile::new(bytes::Bytes::new())
                .unwrap()
                .with_filename(name)
                .is_err()
        );
    }
    for mime in ["bad", "text/plain\r\nX: injected", "текст/плейн"] {
        assert!(
            BinaryFile::new(bytes::Bytes::new())
                .unwrap()
                .with_content_type(mime)
                .is_err()
        );
    }
}

fn accepted_schema_value(name: &str) -> serde_json::Value {
    use std::sync::OnceLock;
    static CORPUS: OnceLock<serde_json::Value> = OnceLock::new();
    let corpus = CORPUS.get_or_init(|| {
        serde_json::from_str(include_str!("../fixtures/model-regressions.json")).unwrap()
    });
    assert_eq!(
        corpus["baselineCommit"],
        "4a80dbdb3dc14365e1c25d838ac0bd139485d48c"
    );
    let matches: Vec<_> = corpus["specimens"]
        .as_array()
        .unwrap()
        .iter()
        .filter(|item| item["name"] == name)
        .collect();
    assert_eq!(
        matches.len(),
        1,
        "independent schema fixture must be unique: {name}"
    );
    matches[0]["output"].clone()
}

#[test]
fn stream_event_literal_discriminators_preserve_native_variant_identity() {
    use super::gateway::GatewayInferenceResponseStreamEvent as Event;
    for (schema, expected) in [
        ("GatewayInferenceResponseCompletedEventSchema", 10),
        ("GatewayInferenceResponseCreatedEventSchema", 13),
        ("GatewayInferenceResponseFailedEventSchema", 21),
        ("GatewayInferenceResponseIncompleteEventSchema", 22),
    ] {
        let wire = accepted_schema_value(schema);
        let event: Event = serde_json::from_value(wire.clone()).unwrap();
        let actual = match &event {
            Event::Variant10(_) => 10,
            Event::Variant13(_) => 13,
            Event::Variant21(_) => 21,
            Event::Variant22(_) => 22,
            _ => 0,
        };
        assert_eq!(actual, expected, "wrong native variant for {schema}");
        assert_eq!(serde_json::to_value(event).unwrap(), wire);
    }
}

#[test]
fn chat_message_roles_preserve_native_variant_identity() {
    use super::gateway::{
        GatewayInferenceChatCompletionRequestMessage as Message,
        GatewayInferenceInputChatCompletionRequestMessage as Input,
    };
    for (role, expected) in [
        ("System", 1),
        ("Developer", 2),
        ("User", 3),
        ("Assistant", 4),
        ("Tool", 5),
        ("Function", 6),
    ] {
        let wire = accepted_schema_value(&format!(
            "GatewayInferenceChatCompletionRequest{role}MessageSchema"
        ));
        let message: Message = serde_json::from_value(wire.clone()).unwrap();
        let actual = match &message {
            Message::Variant1(_) => 1,
            Message::Variant2(_) => 2,
            Message::Variant3(_) => 3,
            Message::Variant4(_) => 4,
            Message::Variant5(_) => 5,
            Message::Variant6(_) => 6,
            _ => 0,
        };
        assert_eq!(actual, expected, "wrong native chat role {role}");
        assert_eq!(serde_json::to_value(message).unwrap(), wire);
        let wire = accepted_schema_value(&format!(
            "GatewayInferenceInputChatCompletionRequest{role}MessageSchema"
        ));
        let input: Input = serde_json::from_value(wire.clone()).unwrap();
        let actual = match &input {
            Input::Variant1(_) => 1,
            Input::Variant2(_) => 2,
            Input::Variant3(_) => 3,
            Input::Variant4(_) => 4,
            Input::Variant5(_) => 5,
            Input::Variant6(_) => 6,
            _ => 0,
        };
        assert_eq!(actual, expected, "wrong native input chat role {role}");
        assert_eq!(serde_json::to_value(input).unwrap(), wire);
    }
}

#[test]
fn literal_decoders_reject_mismatches_and_whole_unions_retain_unknown_values() {
    use super::gateway::{
        GatewayInferenceChatCompletionRequestSystemMessageRole,
        GatewayInferenceResponseCompletedEvent, GatewayInferenceResponseCompletedEventType,
        GatewayInferenceResponseStreamEvent,
    };
    assert!(
        serde_json::from_value::<GatewayInferenceResponseCompletedEventType>(json!(
            "response.failed"
        ))
        .is_err()
    );
    assert!(
        serde_json::from_value::<GatewayInferenceChatCompletionRequestSystemMessageRole>(json!(
            "assistant"
        ))
        .is_err()
    );
    let failed = accepted_schema_value("GatewayInferenceResponseFailedEventSchema");
    assert!(serde_json::from_value::<GatewayInferenceResponseCompletedEvent>(failed).is_err());
    let future = json!({"type":"future.event", "private":"future-secret"});
    let event: GatewayInferenceResponseStreamEvent =
        serde_json::from_value(future.clone()).unwrap();
    assert!(matches!(
        &event,
        GatewayInferenceResponseStreamEvent::Unknown(_)
    ));
    assert_eq!(serde_json::to_value(&event).unwrap(), future);
    assert!(!format!("{event:?}").contains("future-secret"));
}

#[test]
fn numeric_literal_unions_choose_the_matching_literal() {
    use super::gateway::{
        GatewayDeploymentAuthSettingsInputIsDataserviceHosted as Hosted,
        GatewayDeploymentAuthSettingsInputIsDataserviceHostedVariant1 as Zero,
        GatewayInferenceStaticChunkingStrategyMaxChunkSizeTokens as ChunkSize,
        GatewayIntegrationModelsResponseModelsItemIsCustom as Custom,
    };
    assert!(matches!(
        serde_json::from_value::<Hosted>(json!(0)).unwrap(),
        Hosted::Variant1(_)
    ));
    assert!(matches!(
        serde_json::from_value::<Hosted>(json!(1)).unwrap(),
        Hosted::Variant2(_)
    ));
    assert!(matches!(
        serde_json::from_value::<Hosted>(json!(1.0)).unwrap(),
        Hosted::Variant2(_)
    ));
    assert!(matches!(
        serde_json::from_value::<Custom>(json!(false)).unwrap(),
        Custom::Variant1(_)
    ));
    assert!(matches!(
        serde_json::from_value::<Custom>(json!(0)).unwrap(),
        Custom::Variant2(_)
    ));
    assert!(matches!(
        serde_json::from_value::<Custom>(json!(1)).unwrap(),
        Custom::Variant3(_)
    ));
    assert!(matches!(
        serde_json::from_value::<ChunkSize>(json!(0)).unwrap(),
        ChunkSize::Variant2(_)
    ));
    assert!(matches!(
        serde_json::from_value::<ChunkSize>(json!(100)).unwrap(),
        ChunkSize::Variant1(_)
    ));
    assert!(serde_json::from_value::<Zero>(json!(1)).is_err());
    assert!(Zero::new(1.into()).is_err());
    assert!(Zero::new(0.into()).is_ok());
}

#[test]
fn dlp_explicit_discriminator_selects_the_matching_rule() {
    let tree = accepted_schema_value("DefaultTreeDetectionRuleSchema");
    let multi = accepted_schema_value("MultiProfileDetectionRuleSchema");
    let decoded: DetectionRule = serde_json::from_value(tree.clone()).unwrap();
    assert!(matches!(&decoded, DetectionRule::Variant1(_)));
    assert_eq!(serde_json::to_value(decoded).unwrap(), tree);
    let decoded: DetectionRule = serde_json::from_value(multi.clone()).unwrap();
    assert!(matches!(&decoded, DetectionRule::Variant2(_)));
    assert_eq!(serde_json::to_value(decoded).unwrap(), multi);
}

#[test]
fn intersections_merge_independently_stripped_object_fields() {
    use serde::{Deserialize, Serialize};
    #[derive(Serialize, Deserialize)]
    struct Left {
        left: String,
    }
    #[derive(Serialize, Deserialize)]
    struct Right {
        right: String,
    }
    let wire = json!({"left":"left-secret","right":"right-secret"});
    let intersection: Intersection<Left, Right> = serde_json::from_value(wire.clone()).unwrap();
    assert_eq!(intersection.left().left, "left-secret");
    assert_eq!(intersection.right().right, "right-secret");
    assert_eq!(serde_json::to_value(&intersection).unwrap(), wire);
    assert!(!format!("{intersection:?}").contains("secret"));
}

#[test]
fn intersections_merge_nested_objects_and_arrays_and_reject_conflicts() {
    let left = JsonValue::from(json!({"shared":{"left":1,"same":true},"items":[{"left":1},2]}));
    let right =
        JsonValue::from(json!({"shared":{"right":2,"same":true},"items":[{"right":2},2.0]}));
    let merged = Intersection::new(left, right).unwrap();
    assert_eq!(
        serde_json::to_value(merged).unwrap(),
        json!({"shared":{"left":1,"right":2,"same":true},"items":[{"left":1,"right":2},2]})
    );
    for (left, right) in [
        (
            json!({"shared":{"private":"secret-a"}}),
            json!({"shared":{"private":"secret-b"}}),
        ),
        (json!([1]), json!([1, 2])),
        (json!([{"nested":1}]), json!([{"nested":2}])),
    ] {
        let error = Intersection::new(JsonValue::from(left), JsonValue::from(right)).unwrap_err();
        assert_eq!(error.kind(), crate::ErrorKind::Validation);
        assert!(!format!("{error:?} {error}").contains("secret"));
    }
    let mutable = Intersection::new(std::cell::Cell::new(1u8), std::cell::Cell::new(1u8)).unwrap();
    mutable.right().set(2);
    assert!(serde_json::to_value(mutable).is_err());
}

#[test]
fn intersection_merge_enforces_depth_node_and_combined_wire_bounds() {
    let mut deep = json!(null);
    for _ in 0..65 {
        deep = json!([deep]);
    }
    let error = Intersection::new(JsonValue::from(deep), JsonValue::from(json!({}))).unwrap_err();
    assert_eq!(error.kind(), crate::ErrorKind::Limit);
    let within = JsonValue::from(serde_json::Value::Array(vec![json!(null); 60_000]));
    assert!(Intersection::new(within.clone(), within).is_ok());
    let many = JsonValue::from(serde_json::Value::Array(vec![json!(null); 100_001]));
    let error = Intersection::new(many, JsonValue::from(json!([]))).unwrap_err();
    assert_eq!(error.kind(), crate::ErrorKind::Limit);
    let left = JsonValue::from(json!({"left":"a".repeat(5*1024*1024)}));
    let right = JsonValue::from(json!({"right":"b".repeat(5*1024*1024)}));
    let error = Intersection::new(left, right).unwrap_err();
    assert_eq!(error.kind(), crate::ErrorKind::Limit);
}
