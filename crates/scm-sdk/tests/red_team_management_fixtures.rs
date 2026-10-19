//! Independent vendor-derived fixtures; these assertions make no live-service claim.
#![cfg(feature = "red-team")]
use scm_sdk::{
    airs::red_team::models::*,
    data::Patch,
    validation::{self, Mode},
};
use serde_json::{Value, json};

fn corpus() -> Value {
    serde_json::from_str(include_str!(
        "../../../contracts/native-fixtures/red-team-management.json"
    ))
    .unwrap()
}
fn presence<T>(value: &Patch<T>) -> &'static str {
    match value {
        Patch::Unset => "unset",
        Patch::Null => "null",
        Patch::Value(_) => "value",
    }
}
fn typed(row: &Value, normalized: Value) -> (Value, String) {
    macro_rules! model {
        ($model:ty, $first:ident, $second:ident) => {{
            let decoded: $model = serde_json::from_value(normalized).unwrap();
            let state = match row["field"].as_str().unwrap() {
                stringify!($first) => presence(&decoded.$first),
                stringify!($second) => presence(&decoded.$second),
                other => panic!("Unbound fixture projection {other}"),
            };
            assert!(!format!("{decoded:?}").contains("synthetic-private-fixture"));
            (serde_json::to_value(&decoded).unwrap(), state.to_owned())
        }};
    }
    match row["rustType"].as_str().unwrap() {
        "BasicAuthAuthConfig" => model!(BasicAuthAuthConfig, basic_username, basic_password),
        "AuthConfigRequestVariant3" => {
            model!(AuthConfigRequestVariant3, basic_username, basic_password)
        }
        "RestConnectionParams" => model!(RestConnectionParams, audio_request_json, audio_curl),
        "StreamingConnectionParams" => {
            model!(StreamingConnectionParams, audio_request_json, audio_curl)
        }
        "WebSocketConnectionParams" => {
            model!(WebSocketConnectionParams, audio_request_json, audio_curl)
        }
        "ConnectionParamsRequestVariant2" => model!(
            ConnectionParamsRequestVariant2,
            audio_request_json,
            audio_curl
        ),
        other => panic!("Unbound fixture model {other}"),
    }
}

#[test]
fn all_twelve_projection_occurrences_preserve_independent_raw_oracles() {
    let corpus = corpus();
    assert_eq!(
        corpus["projectionOccurrences"].as_array().unwrap().len(),
        12
    );
    let rows = corpus["projectionCases"].as_array().unwrap();
    assert_eq!(rows.len(), 84);
    for row in rows {
        for mode in [Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults] {
            let expected = if mode == Mode::ResponseWithoutDefaults {
                &row["withoutDefaults"]
            } else {
                &row["expected"]
            };
            let normalized =
                validation::parse(row["schema"].as_str().unwrap(), row["input"].clone(), mode)
                    .unwrap();
            assert_eq!(
                &normalized, expected,
                "validator changed independent fixture {row} in {mode:?}"
            );
            let (actual, state) = typed(row, normalized);
            assert_eq!(
                &actual, expected,
                "typed roundtrip changed independent fixture {row}"
            );
            assert_eq!(state, row["typedPresence"].as_str().unwrap());
        }
    }
}

#[test]
fn vendor_union_required_types_formats_and_legacy_order_are_independent() {
    let corpus = corpus();
    assert_eq!(corpus["unionTails"].as_array().unwrap().len(), 2);
    for row in corpus["unionCases"].as_array().unwrap() {
        for mode in [Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults] {
            let accepted = row[if mode == Mode::Request {
                "requestAccepted"
            } else {
                "responseAccepted"
            }]
            .as_bool()
            .unwrap();
            let parsed =
                validation::parse(row["schema"].as_str().unwrap(), row["input"].clone(), mode);
            assert_eq!(
                parsed.is_ok(),
                accepted,
                "Unexpected acceptance {row} in {mode:?}"
            );
            if !accepted {
                continue;
            }
            let parsed = parsed.unwrap();
            assert_eq!(parsed, row["input"]);
            let (variant, actual) = if row["schema"] == "s010462" {
                let typed: StreamingConnectionParamsTargetConnectionConfig =
                    serde_json::from_value(parsed).unwrap();
                let variant = match &typed {
                    StreamingConnectionParamsTargetConnectionConfig::Variant5(_) => 5,
                    _ => 0,
                };
                (variant, serde_json::to_value(typed).unwrap())
            } else {
                let typed: StreamingConnectionParamsMultiTurnConfig =
                    serde_json::from_value(parsed).unwrap();
                let variant = match &typed {
                    StreamingConnectionParamsMultiTurnConfig::Variant1(_) => 1,
                    StreamingConnectionParamsMultiTurnConfig::Variant2(_) => 2,
                    StreamingConnectionParamsMultiTurnConfig::Variant3(_) => 3,
                    _ => 0,
                };
                (variant, serde_json::to_value(typed).unwrap())
            };
            assert_eq!(json!(variant), row["variant"]);
            assert_eq!(actual, row["input"]);
        }
    }
}

#[test]
fn all_vendor_enum_additions_and_unknown_response_values_roundtrip() {
    let corpus = corpus();
    assert_eq!(corpus["enumAdditions"].as_array().unwrap().len(), 3);
    for row in corpus["enumAdditions"].as_array().unwrap() {
        for mode in [Mode::Request, Mode::Response, Mode::ResponseWithoutDefaults] {
            let schema = row["schema"].as_str().unwrap();
            assert_eq!(
                validation::parse(schema, row["value"].clone(), mode).unwrap(),
                row["value"]
            );
            let unknown = json!("future-vendor-value");
            let result = validation::parse(schema, unknown.clone(), mode);
            if mode == Mode::Request {
                assert!(result.is_err());
            } else {
                assert_eq!(result.unwrap(), unknown);
            }
        }
    }
}
