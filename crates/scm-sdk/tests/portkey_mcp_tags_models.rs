//! Portkey 8e5da0e MCP `tags` model and validation contracts.
//!
//! Versioned `portkey:8e5da0e:` request roots accept only the pinned tags
//! addition; frozen roots keep their historical acceptance byte for byte.
#![cfg(feature = "gateway")]
use scm_sdk::{
    airs::gateway::models::*,
    data::Patch,
    validation::{Mode, parse},
};
use serde_json::{Value, json};

const WRITES: [(&str, &str); 4] = [
    (
        "McpIntegrationCreateRequestSchema",
        "portkey:8e5da0e:McpIntegrationCreateRequestSchema",
    ),
    (
        "McpIntegrationUpdateRequestSchema",
        "portkey:8e5da0e:McpIntegrationUpdateRequestSchema",
    ),
    (
        "GatewayMcpServersClientCreateRequestSchema",
        "portkey:8e5da0e:GatewayMcpServersClientCreateRequestSchema",
    ),
    (
        "GatewayMcpServersClientUpdateRequestSchema",
        "portkey:8e5da0e:GatewayMcpServersClientUpdateRequestSchema",
    ),
];

fn minimal(root: &str) -> Value {
    match root {
        "McpIntegrationCreateRequestSchema" => json!({
            "name": "fixture", "organisation_id": "123", "slug": "fixture",
            "url": "https://mcp.example.test/sse", "auth_type": "none", "transport": "sse"
        }),
        "McpIntegrationUpdateRequestSchema" => json!({"name": "renamed"}),
        "GatewayMcpServersClientCreateRequestSchema" => {
            json!({"name": "fixture", "mcp_integration_id": "integration-id"})
        }
        "GatewayMcpServersClientUpdateRequestSchema" => json!({"name": "renamed"}),
        other => panic!("unexpected root {other}"),
    }
}

fn with_tags(root: &str, tags: Value) -> Value {
    let mut body = minimal(root);
    body["tags"] = tags;
    body
}

fn accepts(root: &str, value: Value) -> Value {
    parse(root, value, Mode::Request).unwrap_or_else(|error| panic!("{root} rejected: {error}"))
}

fn rejects(root: &str, value: Value) {
    assert!(
        parse(root, value.clone(), Mode::Request).is_err(),
        "{root} accepted {value}"
    );
}

#[test]
fn versioned_write_roots_accept_tags_value_null_and_omission() {
    for (frozen, current) in WRITES {
        let tags = json!({"env": "prod", "team_a-1": "", "Owner": "sec-ops"});
        assert_eq!(
            accepts(current, with_tags(frozen, tags.clone()))["tags"],
            tags
        );
        assert_eq!(
            accepts(current, with_tags(frozen, Value::Null))["tags"],
            Value::Null
        );
        assert_eq!(
            accepts(current, with_tags(frozen, json!({})))["tags"],
            json!({})
        );
        assert!(accepts(current, minimal(frozen)).get("tags").is_none());
    }
    // `{ "tags": null }` alone is a nonempty update: it clears tags upstream.
    for root in [
        "portkey:8e5da0e:McpIntegrationUpdateRequestSchema",
        "portkey:8e5da0e:GatewayMcpServersClientUpdateRequestSchema",
    ] {
        accepts(root, json!({"tags": null}));
    }
    rejects(
        "portkey:8e5da0e:McpIntegrationUpdateRequestSchema",
        json!({}),
    );
}

#[test]
fn versioned_write_roots_reject_invalid_keys_values_and_shapes() {
    for (frozen, current) in WRITES {
        for key in [
            "bad key",
            "dotted.key",
            "",
            "slash/key",
            "ümlaut",
            "colon:key",
        ] {
            rejects(current, with_tags(frozen, json!({ key: "value" })));
        }
        for value in [
            json!(1),
            json!(true),
            json!(null),
            json!(["a"]),
            json!({"a": "b"}),
        ] {
            rejects(current, with_tags(frozen, json!({ "env": value })));
        }
        for shape in [json!("env=prod"), json!(["env"]), json!(7)] {
            rejects(current, with_tags(frozen, shape));
        }
        // Every other frozen constraint is retained on the versioned root.
        let mut body = with_tags(frozen, json!({"env": "prod"}));
        body["unexpected"] = json!(true);
        rejects(current, body);
    }
}

#[test]
fn frozen_roots_keep_historical_acceptance() {
    for (frozen, current) in WRITES {
        accepts(frozen, minimal(frozen));
        // The frozen 0.34.0 roots are strict and predate tags.
        rejects(frozen, with_tags(frozen, json!({"env": "prod"})));
        assert_eq!(
            accepts(frozen, minimal(frozen)),
            accepts(current, minimal(frozen))
        );
    }
    rejects(
        "GatewayMcpServersClientListOptionsSchema",
        json!({"tags": "{\"env\":\"prod\"}"}),
    );
}

#[test]
fn list_option_roots_carry_the_pinned_query_contract() {
    let servers = "portkey:8e5da0e:GatewayMcpServersClientListOptionsSchema";
    let filter = json!({"tags": "{\"env\":\"prod\"}", "page_size": 100, "current_page": 0});
    assert_eq!(accepts(servers, filter.clone()), filter);
    rejects(servers, json!({"tags": {"env": "prod"}}));
    rejects(servers, json!({"page_size": 101}));

    let integrations = "GatewayMcpIntegrationsClientListOptionsSchema";
    let full = json!({
        "organisation_id": "8f8a6f1e-5a36-4c47-9f1a-5d2b3c4d5e6f", "type": "workspace",
        "workspace_id": "ws-1", "page_size": 1000, "current_page": 0, "search": "fixture",
        "tags": "{\"env\":\"prod\"}"
    });
    assert_eq!(accepts(integrations, full.clone()), full);
    accepts(integrations, json!({}));
    for kind in ["workspace", "organisation", "all"] {
        accepts(integrations, json!({ "type": kind }));
    }
    for invalid in [
        json!({"page_size": 0}),
        json!({"page_size": 1001}),
        json!({"page_size": 1.5}),
        json!({"current_page": -1}),
        json!({"type": "global"}),
        json!({"organisation_id": "not-a-uuid"}),
        json!({"tags": {"env": "prod"}}),
        json!({"tags": null}),
        json!({"unknown": "x"}),
    ] {
        rejects(integrations, invalid);
    }
}

#[test]
fn typed_request_models_preserve_omission_null_and_value() {
    let mut create = serde_json::from_value::<GatewayMcpServersClientCreateRequest>(minimal(
        "GatewayMcpServersClientCreateRequestSchema",
    ))
    .unwrap();
    assert!(matches!(create.tags, Patch::Unset));
    assert!(serde_json::to_value(&create).unwrap().get("tags").is_none());
    create.tags = Patch::Null;
    assert_eq!(serde_json::to_value(&create).unwrap()["tags"], Value::Null);
    let tags: TypedMapString = serde_json::from_value(json!({"env": "prod"})).unwrap();
    create.tags = Patch::Value(tags);
    assert_eq!(
        serde_json::to_value(&create).unwrap()["tags"],
        json!({"env": "prod"})
    );

    let update: McpIntegrationUpdateRequest =
        serde_json::from_value(json!({"tags": null})).unwrap();
    assert!(matches!(update.tags, Patch::Null));
    assert_eq!(
        serde_json::to_value(&update).unwrap(),
        json!({"tags": null})
    );
}

fn detail(tags: Value) -> Value {
    json!({"id": "i", "name": "fixture", "description": null, "owner_id": "owner", "status": "active",
        "created_at": "created", "last_updated_at": "updated", "configurations": {},
        "global_workspace_access": null, "workspace_id": null, "slug": "fixture",
        "url": "https://never-follow.example/mcp", "auth_type": "none", "transport": "sse",
        "type": "mcp", "secret_mappings": null, "object": "mcp", "tags": tags})
}

type TypedMapString = scm_sdk::data::TypedMap<String>;

#[test]
fn responses_type_conforming_tags_and_preserve_nonconforming_json() {
    let typed: McpIntegrationDetail =
        serde_json::from_value(detail(json!({"env": "prod"}))).unwrap();
    match &typed.tags {
        Patch::Value(tags) => assert_eq!(tags.get("env").map(String::as_str), Some("prod")),
        other => panic!(
            "expected typed tags, got {:?}",
            matches!(other, Patch::Null)
        ),
    }
    // Upstream response values are strings; anything else stays raw, untyped.
    let server: GatewayMcpServer =
        serde_json::from_value(json!({"id": "s", "tags": {"count": 3}})).unwrap();
    assert!(matches!(server.tags, Patch::Unset));
    assert_eq!(
        serde_json::to_value(server.extensions.get("tags")).unwrap(),
        json!({"count": 3})
    );
    assert_eq!(
        serde_json::to_value(&server).unwrap()["tags"],
        json!({"count": 3})
    );

    let item: GatewayMcpServerListItem =
        serde_json::from_value(json!({"id": "s", "tags": null})).unwrap();
    assert!(matches!(item.tags, Patch::Null));
    let listed: McpIntegration = serde_json::from_value(
        json!({"id": "i", "organisation_id": "o", "name": "n", "owner_id": "u",
            "status": "active", "type": "mcp", "url": "https://never-follow.example/mcp",
            "auth_type": "none", "transport": "sse", "configurations": "{}", "created_at": "c",
            "last_updated_at": "u", "tags": {"weird key!": "x"}}),
    )
    .unwrap();
    assert!(
        matches!(listed.tags, Patch::Value(_)),
        "responses accept any string keys"
    );

    // Historical tolerant response roots accept both forms unchanged.
    for tags in [json!({"env": "prod"}), json!({"count": 3}), Value::Null] {
        let value = detail(tags.clone());
        assert_eq!(
            parse("McpIntegrationDetailSchema", value, Mode::Response).unwrap()["tags"],
            tags
        );
        let value = json!({"id": "x", "tags": tags.clone()});
        assert_eq!(
            parse(
                "GatewayMcpServersClientGetResponseSchema",
                value,
                Mode::Response
            )
            .unwrap()["tags"],
            tags
        );
    }
}

#[test]
fn typed_model_validation_paths_accept_exactly_what_clients_accept() {
    fn check<T: serde::de::DeserializeOwned>(
        parse_typed: fn(Value, Mode) -> scm_sdk::Result<T>,
        base: Value,
    ) {
        for tags in [json!({"env": "prod"}), Value::Null, json!({})] {
            let mut body = base.clone();
            body["tags"] = tags;
            assert!(parse_typed(body, Mode::Request).is_ok());
        }
        assert!(parse_typed(base.clone(), Mode::Request).is_ok());
        for invalid in [json!({"bad key": "x"}), json!({"env": 1}), json!("env")] {
            let mut body = base.clone();
            body["tags"] = invalid;
            assert!(parse_typed(body, Mode::Request).is_err());
        }
    }
    check(
        McpIntegrationCreateRequest::parse,
        minimal("McpIntegrationCreateRequestSchema"),
    );
    check(
        McpIntegrationUpdateRequest::parse,
        minimal("McpIntegrationUpdateRequestSchema"),
    );
    check(
        GatewayMcpServersClientCreateRequest::parse,
        minimal("GatewayMcpServersClientCreateRequestSchema"),
    );
    check(
        GatewayMcpServersClientUpdateRequest::parse,
        minimal("GatewayMcpServersClientUpdateRequestSchema"),
    );
    assert!(
        GatewayMcpServersClientListOptions::parse(
            json!({"tags": "{\"env\":\"prod\"}"}),
            Mode::Request
        )
        .is_ok()
    );
    assert!(GatewayMcpServersClientListOptions::parse(json!({"tags": 1}), Mode::Request).is_err());

    // Builders validate through the same versioned roots.
    let update = McpIntegrationUpdateRequest::builder()
        .tags(Patch::Null)
        .build_validated(Mode::Request)
        .unwrap();
    assert!(matches!(update.tags, Patch::Null));
    let tags: TypedMapString = [("env", "prod".to_owned())].into_iter().collect();
    let server = GatewayMcpServersClientUpdateRequest::builder()
        .tags(Patch::Value(tags))
        .build_validated(Mode::Request)
        .unwrap();
    assert_eq!(
        serde_json::to_value(&server).unwrap(),
        json!({"tags": {"env": "prod"}})
    );
    let bad: TypedMapString = [("bad key", "x".to_owned())].into_iter().collect();
    assert!(
        GatewayMcpServersClientUpdateRequest::builder()
            .tags(Patch::Value(bad))
            .build_validated(Mode::Request)
            .is_err()
    );
    // The frozen standalone schema names still reject tags.
    assert!(
        parse(
            "McpIntegrationUpdateRequestSchema",
            json!({"tags": null}),
            Mode::Request
        )
        .is_err()
    );
}
