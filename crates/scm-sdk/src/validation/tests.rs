use super::*;
use serde_json::json;
use std::time::Duration;

#[test]
fn ordered_contract_alternatives_share_work_and_preserve_source_order() {
    let registry = registry().unwrap();
    let literal = registry
        .graph
        .nodes
        .iter()
        .find(|(_, node)| node["kind"] == "ZodLiteral")
        .unwrap();
    let schema = literal.0.as_str();
    let value = literal.1["value"].clone();
    assert_eq!(
        parse_one_of_with_limits(
            &[schema],
            value.clone(),
            Mode::Response,
            &ValidationLimits::default()
        )
        .unwrap(),
        value
    );
    let limits = ValidationLimits {
        max_work: 1,
        ..Default::default()
    };
    let mismatched = if value == Value::Null {
        json!(false)
    } else {
        Value::Null
    };
    assert_eq!(
        parse_one_of_with_limits(&[schema, schema], mismatched, Mode::Response, &limits)
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    assert!(
        parse_one_of_with_limits(&[], json!({}), Mode::Response, &ValidationLimits::default())
            .is_err()
    );
}

#[cfg(feature = "gateway")]
#[test]
fn native_multipart_validates_every_required_blob_and_metadata_with_shared_limits() {
    let file = BinaryPart::new(bytes::Bytes::from_static(b"native-file")).unwrap();
    for (schema, field, metadata) in [
        ("s004590", "image", json!({"prompt":"edit"})),
        ("s004620", "image", json!({})),
        ("s004771", "file", json!({"model":"whisper-1"})),
        ("s004833", "file", json!({"model":"whisper-1"})),
        ("s004894", "file", json!({"purpose":"assistants"})),
    ] {
        let parsed = parse_multipart_with_limits(
            schema,
            metadata.clone(),
            &[(field, &file)],
            &ValidationLimits::default(),
            4096,
        )
        .unwrap();
        assert_eq!(parsed, metadata);
        assert!(
            parse_multipart_with_limits(
                schema,
                metadata.clone(),
                &[],
                &ValidationLimits::default(),
                4096
            )
            .is_err()
        );
        let mut spoof = metadata.clone();
        spoof[field] = json!({"bytes":[1,2]});
        assert!(parse(schema, spoof.clone(), Mode::Request).is_err());
        assert!(
            parse_multipart_with_limits(
                schema,
                spoof,
                &[(field, &file)],
                &ValidationLimits::default(),
                4096
            )
            .is_err()
        );
        assert!(
            parse_multipart_with_limits(
                schema,
                metadata.clone(),
                &[(field, &file), (field, &file)],
                &ValidationLimits::default(),
                4096
            )
            .is_err()
        );
        assert_eq!(
            parse_multipart_with_limits(
                schema,
                metadata,
                &[(field, &file)],
                &ValidationLimits::default(),
                1
            )
            .unwrap_err()
            .kind(),
            ErrorKind::Limit
        );
    }
    let metadata = json!({"prompt":"edit"});
    assert_eq!(
        parse_multipart_with_limits(
            "s004590",
            metadata.clone(),
            &[("image", &file), ("mask", &file)],
            &ValidationLimits::default(),
            4096
        )
        .unwrap(),
        metadata
    );
    assert!(
        parse_multipart_with_limits(
            "s004590",
            json!({"prompt":"edit", "unknown":true}),
            &[("image", &file)],
            &ValidationLimits::default(),
            4096
        )
        .is_err()
    );
    let limits = ValidationLimits {
        max_work: 1,
        ..Default::default()
    };
    assert_eq!(
        parse_multipart_with_limits(
            "s004590",
            json!({"prompt":"edit"}),
            &[("image", &file)],
            &limits,
            4096
        )
        .unwrap_err()
        .kind(),
        ErrorKind::Limit
    );
}

fn examples(binding: &str) -> (Value, Value) {
    match binding {
        "nonblank" => (json!("\u{feff}letter\u{00a0}"), json!("\u{feff}\u{00a0}")),
        "finite_json" => (
            json!([{"safe":[null,1,true,"value"]}]),
            json!({"nested":{"constructor":false}}),
        ),
        "finite_json_object" => (
            json!({"safe":[null,1,true,"value"]}),
            json!({"nested":{"prototype":false}}),
        ),
        "chart_bounds" => (
            json!({"costMin":1,"costMax":2}),
            json!({"totalUnitsMin":3,"totalUnitsMax":2}),
        ),
        "time_order" => (
            json!({"start_time":"2024-01-01T12:00:00+01:00","end_time":"2024-01-01T11:00:00Z"}),
            json!({"start_time":"2024-01-01T12:00:00Z","end_time":"2024-01-01T11:00:00Z"}),
        ),
        "nonempty_object" => (json!({"field":null}), json!({})),
        "secret_create" => (
            json!({"manager_type":"aws_sm","auth_config":{"aws_auth_type":"accessKey"}}),
            json!({"manager_type":"aws_sm","auth_config":{"azure_auth_mode":"default"}}),
        ),
        "secret_update" => (
            json!({"description":null}),
            json!({"allow_all_workspaces":true,"allowed_workspaces":[]}),
        ),
        "realtime_type" => (json!({"type":"event","data":"private"}), json!({"type":""})),
        "usage_reset_exclusive" => (
            json!({"periodic_reset":"weekly","periodic_reset_days":null}),
            json!({"periodic_reset":"weekly","periodic_reset_days":7}),
        ),
        "rotation_exclusive" => (
            json!({"rotation_period":"weekly","next_rotation_at":null}),
            json!({"rotation_period":"weekly","next_rotation_at":"2024-01-01T00:00:00Z"}),
        ),
        "guardrail_checks" => (
            json!({"target":"mcp_tools"}),
            json!({"target":"llm","checks":[]}),
        ),
        "profile_selector" => (json!({"profile_name":" "}), json!({"profile_name":""})),
        "tool_event" => (json!({"input":""}), json!({})),
        "identity" => (json!({"type":"event"}), json!(null)),
        "unwrap_data" => (
            json!({"data":[{"id":"mapping"}],"metadata":"private"}),
            json!({}),
        ),
        "message_object" => (json!("receipt"), json!(false)),
        _ => panic!("Every native semantic must have explicit behavioral examples"),
    }
}

#[test]
fn all_semantic_bindings_are_exercised() {
    let registry = registry().unwrap();
    assert_eq!(registry.semantics.len(), 69);
    let required: std::collections::BTreeSet<_> = registry
        .graph
        .nodes
        .iter()
        .filter(|(_, node)| node["requiresReviewedSemantics"] == true)
        .map(|(schema, _)| schema)
        .collect();
    assert_eq!(required, registry.semantics.keys().collect());
    for (schema, semantic) in &registry.semantics {
        match semantic.binding.as_str() {
            "default" => assert_eq!(
                parse_optional(schema, None, Mode::Request).unwrap(),
                Some(semantic.default.clone())
            ),
            "native_binary" => {
                assert!(parse(schema, json!({"bytes":[1,2,3]}), Mode::Request).is_err());
                #[cfg(feature = "models")]
                {
                    let file = BinaryPart::new(bytes::Bytes::from_static(b"binary")).unwrap();
                    validate_binary(schema, &file).unwrap();
                }
            }
            "native_transport" | "native_cancellation" => {
                assert!(parse(schema, json!({}), Mode::Request).is_err())
            }
            binding @ ("finite_json" | "finite_json_object") => {
                let (valid, invalid) = examples(binding);
                assert!(parse(schema, valid.clone(), Mode::Request).is_ok());
                assert!(parse(schema, invalid, Mode::Request).is_err());
                // Recursive bindings have only the bounded Context path.
                assert!(apply(binding, Some(valid)).is_err());
            }
            binding => {
                let (valid, invalid) = examples(binding);
                assert!(apply(binding, Some(valid)).is_ok(), "binding {schema}");
                if binding == "identity" {
                    assert!(apply(binding, None).is_err());
                } else {
                    assert!(apply(binding, Some(invalid)).is_err(), "binding {schema}");
                }
            }
        }
    }
}

#[test]
fn both_finite_json_bindings_charge_recursive_work() {
    let registry = registry().unwrap();
    let mut checked = 0;
    for (schema, semantic) in &registry.semantics {
        if !matches!(
            semantic.binding.as_str(),
            "finite_json" | "finite_json_object"
        ) {
            continue;
        }
        checked += 1;
        let value = json!({"nested":[1,2,{"field":[null,true,"value"]}]});
        for mode in [Mode::Request, Mode::Response] {
            let defaults = ValidationLimits::default();
            let mut core = Context {
                registry,
                mode,
                work: 0,
                copied: 0,
                limits: &defaults,
            };
            core.walk(
                child(&registry.graph.nodes[schema], "inner").unwrap(),
                Some(&value),
                0,
            )
            .unwrap();
            let limits = ValidationLimits {
                // Admit the entire inner parse and its one effect wrapper;
                // the finite-JSON traversal must consume additional budget.
                max_work: core.work + 1,
                ..defaults
            };
            assert_eq!(
                parse_with_limits(schema, value.clone(), mode, &limits)
                    .unwrap_err()
                    .kind(),
                ErrorKind::Limit,
                "binding {schema}"
            );
            assert_eq!(parse(schema, value.clone(), mode).unwrap(), value);
        }
    }
    assert_eq!(checked, 2);
}

#[test]
fn required_nullable_does_not_mean_missing_and_patch_defaults_are_exact() {
    assert_eq!(
        parse_optional("s000075", None, Mode::Request).unwrap(),
        Some(json!(30))
    );
    assert!(parse("s000075", Value::Null, Mode::Request).is_err());
    assert!(parse("AiProfileSchema", json!({}), Mode::Request).is_err());
    assert!(parse("ToolEventSchema", json!({}), Mode::Request).is_err());
    assert!(parse("ToolEventSchema", json!({"output":""}), Mode::Request).is_ok());
}

#[test]
fn response_evolution_preserves_unknown_values_but_not_invalid_known_structure() {
    assert!(
        parse(
            "GatewayDeploymentStatusSchema",
            json!("future-status"),
            Mode::Request
        )
        .is_err()
    );
    assert_eq!(
        parse(
            "GatewayDeploymentStatusSchema",
            json!("future-status"),
            Mode::Response
        )
        .unwrap(),
        json!("future-status")
    );
    let value = json!({"name":"workspace","scope_name":"tenant_scope","future":{"flag":true}});
    assert!(
        parse(
            "GatewayWorkspaceCreateRequestSchema",
            value.clone(),
            Mode::Request
        )
        .is_err()
    );
    assert_eq!(
        parse(
            "GatewayWorkspaceCreateRequestSchema",
            value.clone(),
            Mode::Response
        )
        .unwrap(),
        value
    );
    assert!(
        parse(
            "GatewayWorkspaceCreateRequestSchema",
            json!({"name":"workspace","future":true}),
            Mode::Response
        )
        .is_err()
    );
    assert!(parse("s008539", json!({"rule_type":"unknown"}), Mode::Response).is_err());
}

#[test]
fn utf16_lengths_trim_date_regex_and_decimal_steps_match_frozen_semantics() {
    assert!(
        parse(
            "AiProfileSchema",
            json!({"profile_name":"😀".repeat(50)}),
            Mode::Request
        )
        .is_ok()
    );
    assert!(
        parse(
            "AiProfileSchema",
            json!({"profile_name":"😀".repeat(51)}),
            Mode::Request
        )
        .is_err()
    );
    assert!(apply("nonblank", Some(json!("\u{0085}"))).is_ok()); // Not ECMAScript whitespace.
    assert!(datetime("2000-02-29T12:30Z"));
    assert!(!datetime("1900-02-29T12:30Z"));
    assert!(datetime("2024-01-01T12:30:01.12345+99:99")); // The pinned Zod regex permits this; Date.parse cannot compare it.
    assert!(timestamp("2024-01-01T12:30:01+99:99").is_none());
    assert!(!datetime("2024-01-01T24:00Z"));
    assert!(email("first.last+tag@example.co"));
    assert!(!email("first..last@example.co"));
    assert!(multiple_of(0.29, 0.01));
    assert!(!multiple_of(0.015, 0.01));
    assert!(!multiple_of(0.30000000000000004, 0.01));
    let precise = json!(u64::MAX);
    assert_eq!(
        parse("GatewayJsonValueSchema", precise.clone(), Mode::Request).unwrap(),
        precise
    );
}

#[test]
fn resource_limits_are_explicit_and_deep_values_drop_without_recursion() {
    let mut deep = json!(true);
    for _ in 0..10_000 {
        deep = Value::Array(vec![deep]);
    }
    assert_eq!(
        parse("GatewayJsonValueSchema", deep, Mode::Request)
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    let mut deep = json!(true);
    for _ in 0..10_000 {
        deep = Value::Array(vec![deep]);
    }
    let invalid_limits = ValidationLimits {
        max_depth: 0,
        ..ValidationLimits::default()
    };
    assert_eq!(
        parse_with_limits(
            "GatewayJsonValueSchema",
            deep,
            Mode::Request,
            &invalid_limits
        )
        .unwrap_err()
        .kind(),
        ErrorKind::Configuration
    );
    let mut within = json!(true);
    for _ in 0..60 {
        within = Value::Array(vec![within]);
    }
    assert!(parse("GatewayJsonValueSchema", within, Mode::Request).is_ok());
    let mut limits = ValidationLimits::for_bytes(20).unwrap();
    limits.max_nodes = 3;
    assert_eq!(
        parse_with_limits(
            "GatewayJsonValueSchema",
            json!([1, 2, 3]),
            Mode::Request,
            &limits
        )
        .unwrap_err()
        .kind(),
        ErrorKind::Limit
    );
    limits.max_nodes = 100;
    limits.max_work = 1;
    assert_eq!(
        parse_with_limits("GatewayJsonValueSchema", json!(1), Mode::Request, &limits)
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    let value = json!("x".repeat(8 * 1024 * 1024 + 1));
    assert!(parse("GatewayJsonValueSchema", value.clone(), Mode::Request).is_err());
    assert!(
        parse_with_limits(
            "GatewayJsonValueSchema",
            value,
            Mode::Request,
            &ValidationLimits::for_bytes(9 * 1024 * 1024).unwrap()
        )
        .is_ok()
    );
}

#[test]
fn configured_byte_budgets_scale_node_and_work_limits_monotonically() {
    let small = ValidationLimits::for_bytes(1024).unwrap();
    let default = ValidationLimits::for_bytes(8 * 1024 * 1024).unwrap();
    let doubled = ValidationLimits::for_bytes(16 * 1024 * 1024).unwrap();
    let maximum = ValidationLimits::for_bytes(512 * 1024 * 1024).unwrap();
    assert_eq!((small.max_nodes, small.max_work), (100_000, 1_000_000));
    assert_eq!((default.max_nodes, default.max_work), (100_000, 1_000_000));
    assert_eq!((doubled.max_nodes, doubled.max_work), (200_000, 2_000_000));
    assert_eq!(
        (maximum.max_nodes, maximum.max_work),
        (1_000_000, 16_000_000)
    );
    assert_eq!(maximum.max_input_bytes, 512 * 1024 * 1024);
    assert_eq!(maximum.max_copied_bytes, 512 * 1024 * 1024);
    assert!(ValidationLimits::for_bytes(0).is_err());
    assert!(ValidationLimits::for_bytes(512 * 1024 * 1024 + 1).is_err());
    let mut previous = (0, 0);
    for mib in 1..=512 {
        let limits = ValidationLimits::for_bytes(mib * 1024 * 1024).unwrap();
        assert!(limits.max_nodes >= previous.0);
        assert!(limits.max_work >= previous.1);
        previous = (limits.max_nodes, limits.max_work);
    }
}

#[test]
fn schema_wrappers_do_not_consume_json_depth() {
    let registry = registry().unwrap();
    let limits = ValidationLimits {
        max_depth: 1,
        ..ValidationLimits::default()
    };
    // This is an optional integer inside a default, a schema wrapper chain;
    // the JSON input is a scalar and has depth zero.
    let mut context = Context {
        registry,
        mode: Mode::Request,
        work: 0,
        copied: 0,
        limits: &limits,
    };
    assert_eq!(context.walk("s009825", None, 0).unwrap(), Some(json!(0)));
    // Objects at the depth boundary can omit optional fields without creating
    // a fictitious extra level of JSON nesting.
    assert_eq!(
        parse_array_with_limits("s000011", json!([{}]), Mode::Request, &limits).unwrap(),
        json!([{}])
    );
}

#[test]
fn anonymous_array_contracts_share_whole_operation_limits() {
    let value = json!(["first", "second"]);
    assert_eq!(
        parse_array("s000010", value.clone(), Mode::Request).unwrap(),
        value
    );
    assert!(parse_array("s000010", json!({}), Mode::Request).is_err());
    let mut limits = ValidationLimits {
        max_work: 3,
        ..ValidationLimits::default()
    };
    // Each row is valid within this budget in isolation, but the aggregate is not.
    assert!(parse_with_limits("s000010", json!("first"), Mode::Request, &limits).is_ok());
    assert_eq!(
        parse_array_with_limits("s000010", value.clone(), Mode::Request, &limits)
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
    limits.max_work = 100;
    limits.max_nodes = 2;
    assert_eq!(
        parse_array_with_limits("s000010", value, Mode::Request, &limits)
            .unwrap_err()
            .kind(),
        ErrorKind::Limit
    );
}

#[test]
fn typed_dates_cancellation_and_public_errors_are_safe() {
    let date = NativeDate::from_epoch_millis(0).unwrap();
    validate_date("s000025", &date).unwrap();
    assert!(serde_json::to_value(date).is_err());
    assert!(serde_json::from_str::<NativeDate>("0").is_err());
    assert!(parse("s000025", json!("2024-01-01T00:00:00Z"), Mode::Request).is_err());
    assert!(NativeDate::from_epoch_millis(i64::MAX).is_err());
    validate_cancellation(
        "s000014",
        &RequestOptions::with_timeout(Duration::from_secs(1)).unwrap(),
    )
    .unwrap();
    let value = json!({"profile_name":"confidential".repeat(100)});
    let error = parse("AiProfileSchema", value, Mode::Request).unwrap_err();
    assert!(!format!("{error:?} {error}").contains("confidential"));
}

#[test]
fn response_format_relaxation_keeps_malformed_units_fail_closed() {
    for mode in [Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults] {
        for unit in [json!("unknown"), json!(false), Value::Null] {
            for kind in ["uuid", "min"] {
                assert!(
                    check_string(
                        "text",
                        &[json!({"kind":kind,"value":3,"requestOnly":true,"unit":unit})],
                        mode
                    )
                    .is_err()
                );
            }
        }
    }
}

#[test]
fn response_length_relaxation_keeps_malformed_bounds_fail_closed() {
    for mode in [Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults] {
        for kind in ["min", "max"] {
            for value in [None, Some(json!("3")), Some(json!(-1)), Some(Value::Null)] {
                let mut check = json!({"kind": kind, "requestOnly": true});
                if let Some(value) = value {
                    check["value"] = value;
                }
                assert!(check_string("text", &[check], mode).is_err());
            }
        }
    }
}

#[test]
fn response_format_relaxation_keeps_malformed_regex_metadata_fail_closed() {
    for mode in [Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults] {
        for regex in [
            None,
            Some(Value::Null),
            Some(json!({})),
            Some(json!({"$regexp": 3})),
            Some(json!({"$regexp": "unregistered-g25-pattern"})),
        ] {
            let mut check = json!({"kind": "regex"});
            if let Some(regex) = regex {
                check["regex"] = regex;
            }
            assert!(check_string("text", &[check], mode).is_err());
        }
    }
}
