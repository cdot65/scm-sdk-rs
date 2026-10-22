//! Expected results were produced by actual frozen Zod execution, independently
//! of native validation. Native JS descriptors are not JSON conformance passes.
use std::{collections::BTreeSet, sync::OnceLock};

use scm_sdk::validation::{Mode, parse, parse_optional};
use serde_json::Value;

#[test]
fn all_23_frozen_defaults_validate_under_both_response_policies() {
    let semantics: Value =
        serde_json::from_str(include_str!("../src/validation/semantics.json")).unwrap();
    let mut count = 0;
    let mut aliases = 0;
    for (schema, binding) in semantics.as_object().unwrap() {
        if binding["binding"] != "default" {
            continue;
        }
        if schema == "s_ts035_s009653" {
            aliases += 1;
            assert_eq!(
                binding, &semantics["s009653"],
                "versioned list alias changed its default"
            );
        } else {
            count += 1;
        }
        let default = &binding["default"];
        for mode in [Mode::Request, Mode::Response] {
            assert_eq!(
                parse_optional(schema, None, mode).unwrap(),
                Some(default.clone()),
                "{schema} changed its original default"
            );
        }
        assert_eq!(
            parse_optional(schema, None, Mode::ResponseWithoutDefaults).unwrap(),
            None,
            "{schema} materialized an omitted default"
        );
        for sent in [default.clone(), Value::Null] {
            let full = parse(schema, sent.clone(), Mode::Response);
            let without = parse(schema, sent.clone(), Mode::ResponseWithoutDefaults);
            assert_eq!(full.is_ok(), without.is_ok(), "{schema} changed acceptance");
            if let (Ok(full), Ok(without)) = (full, without) {
                assert_eq!(full, without, "{schema} changed a supplied value");
                assert_eq!(without, sent, "{schema} changed a supplied value");
            }
        }
    }
    assert_eq!(count, 23);
    assert_eq!(aliases, 1);
}

#[test]
fn response_default_omission_charges_identical_copy_work_and_node_budgets() {
    use scm_sdk::validation::{ValidationLimits, parse_with_limits};
    let input = serde_json::json!({"basic_auth_header":null});
    let compare = |limits: &ValidationLimits| {
        let full = parse_with_limits(
            "BasicAuthAuthConfigSchema",
            input.clone(),
            Mode::Response,
            limits,
        );
        let without = parse_with_limits(
            "BasicAuthAuthConfigSchema",
            input.clone(),
            Mode::ResponseWithoutDefaults,
            limits,
        );
        assert_eq!(full.is_ok(), without.is_ok(), "{limits:?}");
        assert_eq!(
            full.as_ref().err().map(|e| e.kind()),
            without.as_ref().err().map(|e| e.kind()),
            "{limits:?}"
        );
        if let Ok(without) = without {
            assert_eq!(without, input);
        }
        full.is_ok()
    };
    for copied in 26..=70 {
        let mut limits = ValidationLimits::default();
        limits.max_input_bytes = 26;
        limits.max_copied_bytes = copied;
        // "HEADER" (6) + basic_auth_location (19) + basic_auth_header (17).
        assert_eq!(compare(&limits), copied >= 42);
    }
    for bound in 1..=64 {
        let mut limits = ValidationLimits::default();
        limits.max_work = bound;
        compare(&limits);
        let mut limits = ValidationLimits::default();
        limits.max_nodes = bound;
        compare(&limits);
    }
}

#[test]
fn response_default_choice_is_local_to_the_request_options_clone() {
    let original =
        scm_sdk::RequestOptions::with_timeout(std::time::Duration::from_secs(10)).unwrap();
    let without = original.clone().without_response_defaults();
    assert!(original.response_defaults());
    assert!(!without.response_defaults());
    assert_eq!(original.deadline(), without.deadline());
    assert_eq!(original.retry_limit(), without.retry_limit());
}

fn fixtures() -> &'static Value {
    static FIXTURES: OnceLock<Value> = OnceLock::new();
    FIXTURES.get_or_init(|| {
        serde_json::from_str(include_str!(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/../../contracts/native-fixtures/schemas.json"
        )))
        .expect("independent frozen Zod fixtures must be valid JSON")
    })
}

#[test]
fn every_public_schema_and_manual_semantic_has_independent_oracle_evidence() {
    let corpus = fixtures();
    assert_eq!(
        corpus["baseline"]["commit"],
        "4a80dbdb3dc14365e1c25d838ac0bd139485d48c"
    );
    assert_eq!(corpus["counts"]["publicSchemaExports"], 1086);
    assert_eq!(corpus["counts"]["moduleRoots"], 1125);
    assert_eq!(corpus["counts"]["manualSemanticNodes"], 69);
    assert!(
        corpus["counts"]["unresolvedValidSpecimens"]
            .as_array()
            .unwrap()
            .is_empty()
    );
    let mut names = BTreeSet::new();
    let mut sids = BTreeSet::new();
    let mut declarations = 0;
    for root in corpus["roots"].as_array().unwrap() {
        assert!(sids.insert(root["sid"].as_str().unwrap()));
        for name in root["names"].as_array().unwrap() {
            assert!(names.insert(name.as_str().unwrap()));
        }
        declarations += root["moduleRoots"].as_array().unwrap().len();
        assert!(!root["jsonCases"].as_array().unwrap().is_empty());
        if root["coverage"]["acceptedJsonCases"] == 0 {
            assert_eq!(root["coverage"]["status"], "requires-native-input");
            assert!(
                root["nativeCases"]
                    .as_array()
                    .unwrap()
                    .iter()
                    .any(|case| case["zod"]["success"] == true)
            );
        }
        // Unconstrained JSON is accounted for through prototype-unsafe negative
        // specimens; no equal-to-valid sample is mislabeled as a rejection.
        assert!(root["coverage"]["rejectedJsonCases"].as_u64().unwrap() > 0);
    }
    assert_eq!(names.len(), 1086);
    assert_eq!(declarations, 1125);
    for semantic in corpus["semanticBindings"].as_array().unwrap() {
        assert!(sids.contains(semantic["oracleSid"].as_str().unwrap()));
        assert!(
            semantic["expression"].is_object(),
            "manual semantics require exact frozen expression evidence"
        );
    }
    for native in corpus["nativeBindings"].as_array().unwrap() {
        assert!(sids.contains(native["sid"].as_str().unwrap()));
        assert!(!native["mapping"].as_str().unwrap().is_empty());
    }
}

fn compare(mode: Mode, expected_key: &str) {
    let mut failures = Vec::new();
    let mut checked = 0;
    for root in fixtures()["roots"].as_array().unwrap() {
        let sid = root["sid"].as_str().unwrap();
        for (index, case) in root["jsonCases"].as_array().unwrap().iter().enumerate() {
            checked += 1;
            let expected = &case[expected_key];
            let actual = parse(sid, case["input"].clone(), mode);
            let agrees = match actual.as_ref() {
                Ok(output) => expected["success"] == true && output == &expected["output"],
                Err(_) => expected["success"] == false,
            };
            if !agrees && failures.len() < 30 {
                let actual = match actual {
                    Ok(value) => {
                        let mut value = value.to_string();
                        if value.len() > 400 {
                            let mut boundary = 400;
                            while !value.is_char_boundary(boundary) {
                                boundary -= 1;
                            }
                            value.truncate(boundary);
                        }
                        format!("accepted: {value}")
                    }
                    Err(error) => format!("rejected: {error:?}"),
                };
                failures.push(format!(
                    "{sid} {} case {index} {} expected {} actual {actual}",
                    root["names"], case["label"], expected["success"]
                ));
            }
        }
    }
    assert_eq!(
        checked as u64,
        fixtures()["counts"]["jsonCases"].as_u64().unwrap()
    );
    assert!(
        failures.is_empty(),
        "{expected_key} oracle mismatches:\n{}",
        failures.join("\n")
    );
}

#[test]
fn requests_match_original_frozen_zod_acceptance_and_normalized_outputs() {
    compare(Mode::Request, "zod");
}

#[test]
fn responses_match_independently_executed_documented_zod_policy() {
    compare(Mode::Response, "response");
}

// Every key the default response policy adds beyond `without` must be a key the
// input omitted; everything else is identical.
fn only_defaults_differ(full: &Value, without: &Value, input: Option<&Value>, path: &str) {
    match (full, without) {
        (Value::Object(full), Value::Object(without)) => {
            for (key, value) in without {
                let expected = full
                    .get(key)
                    .unwrap_or_else(|| panic!("{path}.{key} invented without defaults"));
                let input = input.and_then(|input| input.get(key));
                only_defaults_differ(expected, value, input, &format!("{path}.{key}"));
            }
            for key in full.keys().filter(|key| !without.contains_key(*key)) {
                assert!(
                    input.is_some_and(|input| input.is_object() && input.get(key).is_none()),
                    "{path}.{key} was present in the input but dropped"
                );
            }
        }
        (Value::Array(full), Value::Array(without)) => {
            assert_eq!(full.len(), without.len(), "{path} array length differs");
            for (index, (full, without)) in full.iter().zip(without).enumerate() {
                let input = input.and_then(|input| input.get(index));
                only_defaults_differ(full, without, input, &format!("{path}[{index}]"));
            }
        }
        (full, without) => assert_eq!(full, without, "{path} differs"),
    }
}

#[test]
fn responses_without_defaults_keep_oracle_acceptance_and_omit_only_defaults() {
    let mut checked = 0;
    let mut omitted = 0;
    for root in fixtures()["roots"].as_array().unwrap() {
        let sid = root["sid"].as_str().unwrap();
        for (index, case) in root["jsonCases"].as_array().unwrap().iter().enumerate() {
            checked += 1;
            let expected = &case["response"];
            let actual = parse(sid, case["input"].clone(), Mode::ResponseWithoutDefaults);
            match actual {
                Ok(output) => {
                    assert_eq!(expected["success"], true, "{sid} case {index} accepted");
                    if output != expected["output"] {
                        omitted += 1;
                    }
                    only_defaults_differ(
                        &expected["output"],
                        &output,
                        Some(&case["input"]),
                        &format!("{sid} case {index}"),
                    );
                }
                Err(_) => assert_eq!(expected["success"], false, "{sid} case {index} rejected"),
            }
        }
    }
    assert_eq!(
        checked as u64,
        fixtures()["counts"]["jsonCases"].as_u64().unwrap()
    );
    // The corpus exercises actual defaulted omissions, not only identical outputs.
    assert!(omitted > 0);
}

#[test]
fn red_team_target_defaults_are_validated_but_not_inserted_without_defaults() {
    use serde_json::json;
    let ws = json!({"api_endpoint":"wss://target.example.test/ws","future":"kept"});
    let mut defaulted = ws.clone();
    defaulted["ws_response_timeout"] = json!(110);
    assert_eq!(
        parse("ConnectionParamsSchema", ws.clone(), Mode::Response).unwrap(),
        defaulted
    );
    assert_eq!(
        parse(
            "ConnectionParamsSchema",
            ws.clone(),
            Mode::ResponseWithoutDefaults
        )
        .unwrap(),
        ws
    );
    let sent = json!({"api_endpoint":"wss://target.example.test/ws","ws_response_timeout":5});
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        assert_eq!(
            parse("ConnectionParamsSchema", sent.clone(), mode).unwrap(),
            sent
        );
    }
    let basic = json!({"basic_auth_header":{"Authorization":"synthetic-header"}});
    let mut defaulted = basic.clone();
    defaulted["basic_auth_location"] = json!("HEADER");
    assert_eq!(
        parse("AuthConfigSchema", basic.clone(), Mode::Response).unwrap(),
        defaulted
    );
    assert_eq!(
        parse(
            "AuthConfigSchema",
            basic.clone(),
            Mode::ResponseWithoutDefaults
        )
        .unwrap(),
        basic
    );
    let oauth = json!({"oauth2_token_url":"https://issuer.example.test/token","oauth2_inject_header":{"Authorization":"Bearer {token}"}});
    let full = parse("OAuth2AuthConfigSchema", oauth.clone(), Mode::Response).unwrap();
    assert_eq!(full["oauth2_expiry_minutes"], 60);
    assert_eq!(full["oauth2_token_response_key"], "access_token");
    assert_eq!(
        parse(
            "OAuth2AuthConfigSchema",
            oauth.clone(),
            Mode::ResponseWithoutDefaults
        )
        .unwrap(),
        oauth
    );
    // A defaulted field that is present is still checked against its schema, and
    // a document that is invalid for every union variant is still rejected.
    for (schema, invalid) in [
        (
            "WebSocketConnectionParamsSchema",
            json!({"ws_response_timeout":"110"}),
        ),
        (
            "BasicAuthAuthConfigSchema",
            json!({"basic_auth_location":5}),
        ),
        (
            "OAuth2AuthConfigSchema",
            json!({"oauth2_inject_header":{"Authorization":"Bearer {token}"}}),
        ),
        ("AuthConfigSchema", json!({"basic_auth_location":5})),
    ] {
        for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
            assert!(
                parse(schema, invalid.clone(), mode).is_err(),
                "{schema} accepted an invalid document in {mode:?}"
            );
        }
    }
}

#[test]
fn omitted_roots_match_actual_zod_undefined_and_default_results() {
    let mut checked = 0;
    for root in fixtures()["roots"].as_array().unwrap() {
        let sid = root["sid"].as_str().unwrap();
        let expected = root["nativeCases"]
            .as_array()
            .unwrap()
            .iter()
            .find(|case| case["input"]["nativeType"] == "undefined")
            .expect("every root requires an actual undefined probe");
        checked += 1;
        // Without defaults an accepted omission stays omitted.
        let actual = parse_optional(sid, None, Mode::ResponseWithoutDefaults);
        if expected["zod"]["success"] == false {
            assert!(actual.is_err(), "{sid} accepted an omitted required value");
        } else {
            assert_eq!(actual.unwrap(), None, "{sid} materialized a default");
        }
        for mode in [Mode::Request, Mode::Response] {
            let actual = parse_optional(sid, None, mode);
            if expected["zod"]["success"] == false {
                assert!(actual.is_err(), "{sid} accepted an omitted required value");
            } else if expected["zod"]["output"]["nativeType"] == "undefined" {
                assert_eq!(actual.unwrap(), None, "{sid} did not preserve omission");
            } else {
                assert_eq!(
                    actual.unwrap(),
                    Some(expected["zod"]["output"].clone()),
                    "{sid} default differs"
                );
            }
        }
    }
    assert_eq!(
        checked,
        fixtures()["counts"]["oracleEntries"].as_u64().unwrap()
    );
}

#[test]
fn native_boundaries_accept_rust_values_and_reject_json_class_spoofs() {
    #[cfg(feature = "models")]
    use bytes::Bytes;
    use futures_util::future::BoxFuture;
    #[cfg(feature = "models")]
    use scm_sdk::validation::{BinaryPart, validate_binary};
    use scm_sdk::{
        Error, ErrorKind, RequestOptions, Result,
        transport::{Transport, TransportRequest, TransportResponse},
        validation::{NativeDate, validate_cancellation, validate_date, validate_transport},
    };
    use std::{
        sync::atomic::{AtomicUsize, Ordering},
        time::Duration,
    };

    struct NeverNetwork(AtomicUsize);
    impl Transport for NeverNetwork {
        fn send<'a>(&'a self, _: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
            self.0.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { Err(Error::new(ErrorKind::Protocol)) })
        }
    }
    let transport = NeverNetwork(AtomicUsize::new(0));
    let cancellation = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    let date = NativeDate::from_epoch_millis(0).unwrap();
    #[cfg(feature = "models")]
    let binary = BinaryPart::new(Bytes::from_static(b"fixture"))
        .unwrap()
        .with_filename("fixture.bin")
        .unwrap()
        .with_content_type("application/octet-stream")
        .unwrap();
    assert_eq!(date.epoch_millis(), 0);
    #[cfg(feature = "models")]
    assert_eq!(binary.bytes().as_ref(), b"fixture");
    let mut checked = 0;
    for binding in fixtures()["nativeBindings"].as_array().unwrap() {
        let sid = binding["sid"].as_str().unwrap();
        let expression = binding["expression"]["text"].as_str().unwrap_or("");
        if binding["kind"] == "ZodDate" {
            validate_date(sid, &date).unwrap();
            #[cfg(feature = "models")]
            assert!(validate_binary(sid, &binary).is_err());
        } else if expression.contains("Blob") {
            #[cfg(feature = "models")]
            validate_binary(sid, &binary).unwrap();
            assert!(validate_date(sid, &date).is_err());
        } else if expression.contains("AbortSignal") {
            validate_cancellation(sid, &cancellation).unwrap();
            cancellation.cancel();
            validate_cancellation(sid, &cancellation).unwrap();
        } else {
            assert!(expression.contains("globalThis.fetch"));
            validate_transport(sid, &transport).unwrap();
        }
        checked += 1;
        for spoof in [
            Value::Null,
            serde_json::json!({"nativeType":"Blob","bytes":[1,2,3]}),
            serde_json::json!("2026-01-02T03:04:05Z"),
            serde_json::json!({"aborted":false}),
        ] {
            for mode in [Mode::Request, Mode::Response] {
                assert!(
                    parse(sid, spoof.clone(), mode).is_err(),
                    "{sid} accepted a JSON class spoof"
                );
            }
        }
    }
    assert_eq!(checked, 10);
    assert_eq!(transport.0.load(Ordering::SeqCst), 0);
}
