//! Independent TypeScript 0.35.0 outcomes and additive response compatibility.
#![cfg(feature = "models")]
use scm_sdk::{
    airs::management::models::*,
    data::Optional,
    validation::{Mode, parse},
};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};

fn typed_round_trip<T: serde::de::DeserializeOwned + serde::Serialize>(
    value: Value,
    id: &str,
) -> Value {
    let model: T = serde_json::from_value(value)
        .unwrap_or_else(|error| panic!("typed oracle decode {id}: {error}"));
    serde_json::to_value(model).unwrap_or_else(|error| panic!("typed oracle encode {id}: {error}"))
}

#[test]
fn every_current_profile_root_matches_actual_typescript_outcomes() {
    let corpus: Value = serde_json::from_str(include_str!(
        "../../../contracts/typescript-035-oracle.json"
    ))
    .unwrap();
    assert_eq!(
        corpus["provenance"]["sourceCommit"],
        "780c985cac90144ea1884fe6cffa58b5deec9ac4"
    );
    // Registry keys and replay modes are explicit for every oracle root.
    // Only the two new roots share exact Zod semantics in response modes.
    let root_contracts: [(&str, (&str, &[Mode])); 8] = [
        (
            "ContentTypeConfigurationSchema",
            (
                "ContentTypeConfigurationSchema",
                &[Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults],
            ),
        ),
        (
            "ContentTypeConfigurationsSchema",
            (
                "ContentTypeConfigurationsSchema",
                &[Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults],
            ),
        ),
        (
            "ModelConfigurationSchema",
            (
                "typescript:0.35.0:ModelConfigurationSchema",
                &[Mode::Request],
            ),
        ),
        (
            "AiSecurityProfileSchema",
            (
                "typescript:0.35.0:AiSecurityProfileSchema",
                &[Mode::Request],
            ),
        ),
        (
            "PolicySchema",
            ("typescript:0.35.0:PolicySchema", &[Mode::Request]),
        ),
        (
            "SecurityProfileSchema",
            ("typescript:0.35.0:SecurityProfileSchema", &[Mode::Request]),
        ),
        (
            "CreateSecurityProfileRequestSchema",
            (
                "typescript:0.35.0:CreateSecurityProfileRequestSchema",
                &[Mode::Request],
            ),
        ),
        (
            "SecurityProfileListResponseSchema",
            (
                "typescript:0.35.0:SecurityProfileListResponseSchema",
                &[Mode::Request],
            ),
        ),
    ];
    let root_contracts: BTreeMap<_, _> = root_contracts.into_iter().collect();
    assert_eq!(root_contracts.len(), 8);
    let expected: BTreeSet<_> = root_contracts.keys().copied().collect();
    let mut roots: BTreeMap<&str, [usize; 2]> = BTreeMap::new();
    let mut ids = BTreeSet::new();
    for case in corpus["cases"].as_array().unwrap() {
        let id = case["id"].as_str().unwrap();
        assert!(ids.insert(id), "duplicate oracle case {id}");
        let name = case["schema"].as_str().unwrap();
        let (schema, modes) = root_contracts
            .get(name)
            .unwrap_or_else(|| panic!("unmapped oracle root: {name}"));
        let accepted = case["zod"]["success"].as_bool().unwrap();
        roots.entry(name).or_default()[usize::from(accepted)] += 1;
        for mode in *modes {
            match parse(schema, case["input"].clone(), *mode) {
                Ok(value) => {
                    assert!(accepted, "unexpected acceptance: {id}");
                    assert_eq!(value, case["zod"]["output"], "oracle output: {id}");
                    // Round-trip normalized Zod output, including its intentional
                    // list default, rather than asking serde to insert defaults.
                    let typed = match name {
                        "ContentTypeConfigurationSchema" => {
                            typed_round_trip::<ContentTypeConfiguration>(value, id)
                        }
                        "ContentTypeConfigurationsSchema" => {
                            typed_round_trip::<ContentTypeConfigurations>(value, id)
                        }
                        "ModelConfigurationSchema" => {
                            typed_round_trip::<ModelConfiguration>(value, id)
                        }
                        "AiSecurityProfileSchema" => {
                            typed_round_trip::<AiSecurityProfile>(value, id)
                        }
                        "PolicySchema" => typed_round_trip::<Policy>(value, id),
                        "SecurityProfileSchema" => typed_round_trip::<SecurityProfile>(value, id),
                        "CreateSecurityProfileRequestSchema" => {
                            typed_round_trip::<CreateSecurityProfileRequest>(value, id)
                        }
                        "SecurityProfileListResponseSchema" => {
                            typed_round_trip::<SecurityProfileListResponse>(value, id)
                        }
                        _ => panic!("unmapped typed oracle root: {name}"),
                    };
                    assert_eq!(typed, case["zod"]["output"], "typed oracle output: {id}");
                }
                Err(_) => assert!(!accepted, "unexpected rejection: {id}"),
            }
        }
    }
    assert_eq!(roots.keys().copied().collect::<BTreeSet<_>>(), expected);
    assert!(roots.values().all(|counts| counts.iter().all(|n| *n > 0)));
}

#[test]
fn five_projections_preserve_missing_null_and_unexpected_response_json() {
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        for value in [
            json!(null),
            json!(17),
            json!(["future"]),
            json!({"unexpected":true}),
        ] {
            let ai = json!({"content-type-mode":value,"content-type-configurations":value,"future":false});
            assert_eq!(
                serde_json::to_value(AiSecurityProfile::parse(ai.clone(), mode).unwrap()).unwrap(),
                ai
            );
            let model = json!({"enable-full-conversation-inspection":value});
            assert_eq!(
                serde_json::to_value(ModelConfiguration::parse(model.clone(), mode).unwrap())
                    .unwrap(),
                model
            );
            let profile = json!({"profile_name":"fixture","dlp_tenant_id":value,"policy":{}});
            assert_eq!(
                serde_json::to_value(SecurityProfile::parse(profile.clone(), mode).unwrap())
                    .unwrap(),
                profile
            );
            assert_eq!(
                serde_json::to_value(
                    CreateSecurityProfileRequest::parse(profile.clone(), mode).unwrap()
                )
                .unwrap(),
                profile
            );
        }
        let empty = AiSecurityProfile::parse(json!({}), mode).unwrap();
        assert!(empty.content_type_mode.is_unset());
        assert!(empty.content_type_configurations.is_unset());
        assert_eq!(serde_json::to_value(empty).unwrap(), json!({}));
        // Invalid nested known directions fall back at the outer projection, losslessly.
        for directions in [
            json!({"prompt":null}),
            json!({"response":{"model-protection":17}}),
            json!({"tool-call":false}),
        ] {
            let input = json!({"content-type-configurations":directions});
            let model = AiSecurityProfile::parse(input.clone(), mode).unwrap();
            assert!(model.content_type_configurations.is_unset());
            assert_eq!(serde_json::to_value(model).unwrap(), input);
        }
    }
}

#[test]
fn typed_fields_take_precedence_without_losing_other_extensions() {
    let direction = ContentTypeConfiguration::builder()
        .build_validated(Mode::Request)
        .unwrap();
    let directions = ContentTypeConfigurations::builder()
        .prompt(Optional::Value(direction))
        .extension("future-direction", json!(null).into())
        .build_validated(Mode::Request)
        .unwrap();
    let ai = AiSecurityProfile::builder()
        .content_type_mode(Optional::Value("per_content_type".into()))
        .content_type_configurations(Optional::Value(directions))
        .extension("content-type-mode", json!(17).into())
        .extension("content-type-configurations", json!(false).into())
        .extension("future", json!(true).into())
        .build()
        .unwrap();
    assert_eq!(
        serde_json::to_value(&ai).unwrap(),
        json!({"content-type-mode":"per_content_type",
        "content-type-configurations":{"prompt":{},"future-direction":null},"future":true})
    );
    assert!(!format!("{ai:?}").contains("per_content_type"));
    let model = ModelConfiguration::builder()
        .enable_full_conversation_inspection(Optional::Value(false))
        .extension("enable-full-conversation-inspection", json!(null).into())
        .build()
        .unwrap();
    assert_eq!(
        serde_json::to_value(model).unwrap(),
        json!({"enable-full-conversation-inspection":false})
    );
    let response = SecurityProfile::builder()
        .profile_name("fixture".into())
        .dlp_tenant_id(Optional::Value("100".into()))
        .extension("dlp_tenant_id", json!(17).into())
        .build()
        .unwrap();
    assert_eq!(
        serde_json::to_value(response).unwrap()["dlp_tenant_id"],
        "100"
    );
    let request = CreateSecurityProfileRequest::builder()
        .profile_name("fixture".into())
        .policy(Policy::builder().build().unwrap())
        .dlp_tenant_id(Optional::Value("100".into()))
        .extension("dlp_tenant_id", json!(17).into())
        .build()
        .unwrap();
    assert_eq!(
        serde_json::to_value(request).unwrap()["dlp_tenant_id"],
        "100"
    );
}

#[test]
fn versioned_list_default_preserves_the_response_policy() {
    let schema = "typescript:0.35.0:SecurityProfileListResponseSchema";
    assert_eq!(
        parse(schema, json!({}), Mode::Request).unwrap(),
        json!({"ai_profiles":[]})
    );
    assert_eq!(
        parse(schema, json!({}), Mode::Response).unwrap(),
        json!({"ai_profiles":[]})
    );
    assert_eq!(
        parse(schema, json!({}), Mode::ResponseWithoutDefaults).unwrap(),
        json!({})
    );
}
