// Mutating public non-exhaustive options also exercises invalid caller states.
#![allow(clippy::field_reassign_with_default)]
use futures_util::{StreamExt, stream};
use scm_sdk::pagination::{CollectOptions, Completion, Page, Truncation};
use scm_sdk::{Error, ErrorKind, RequestOptions, Session, constants, helpers::*};
use serde_json::{Value, json};
use std::{
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};

fn budget() -> RequestOptions {
    RequestOptions::with_timeout(Duration::from_secs(10)).unwrap()
}
fn constants_fixture() -> Value {
    serde_json::from_str(include_str!(
        "../../../contracts/native-fixtures/helpers.json"
    ))
    .unwrap()
}
fn constant_value(name: &str) -> Value {
    match name {
        "AGENT_GUARD_DATA_ENDPOINT" => {
            serde_json::to_value(constants::AGENT_GUARD_DATA_ENDPOINT).unwrap()
        }
        "AGENT_GUARD_MGMT_ENDPOINT" => {
            serde_json::to_value(constants::AGENT_GUARD_MGMT_ENDPOINT).unwrap()
        }
        "AGENT_GUARD_RULES_PATH" => {
            serde_json::to_value(constants::AGENT_GUARD_RULES_PATH).unwrap()
        }
        "AGENT_GUARD_SCANS_PATH" => {
            serde_json::to_value(constants::AGENT_GUARD_SCANS_PATH).unwrap()
        }
        "AGENT_GUARD_STATS_PATH" => {
            serde_json::to_value(constants::AGENT_GUARD_STATS_PATH).unwrap()
        }
        "AI_GATEWAY_DEPLOYMENT_STATUSES" => {
            serde_json::to_value(constants::AI_GATEWAY_DEPLOYMENT_STATUSES).unwrap()
        }
        "AI_GATEWAY_DEPLOYMENT_TYPES" => {
            serde_json::to_value(constants::AI_GATEWAY_DEPLOYMENT_TYPES).unwrap()
        }
        "AI_GATEWAY_KNOWN_API_KEY_SCOPES" => {
            serde_json::to_value(constants::AI_GATEWAY_KNOWN_API_KEY_SCOPES).unwrap()
        }
        "AI_GATEWAY_KNOWN_CACHE_MODES" => {
            serde_json::to_value(constants::AI_GATEWAY_KNOWN_CACHE_MODES).unwrap()
        }
        "AI_GATEWAY_KNOWN_CONFIG_STRATEGIES" => {
            serde_json::to_value(constants::AI_GATEWAY_KNOWN_CONFIG_STRATEGIES).unwrap()
        }
        "AI_GATEWAY_KNOWN_MCP_AUTH_TYPES" => {
            serde_json::to_value(constants::AI_GATEWAY_KNOWN_MCP_AUTH_TYPES).unwrap()
        }
        "AI_GATEWAY_KNOWN_MCP_TRANSPORTS" => {
            serde_json::to_value(constants::AI_GATEWAY_KNOWN_MCP_TRANSPORTS).unwrap()
        }
        "AI_GATEWAY_KNOWN_RATE_LIMIT_TYPES" => {
            serde_json::to_value(constants::AI_GATEWAY_KNOWN_RATE_LIMIT_TYPES).unwrap()
        }
        "AI_GATEWAY_KNOWN_RATE_LIMIT_UNITS" => {
            serde_json::to_value(constants::AI_GATEWAY_KNOWN_RATE_LIMIT_UNITS).unwrap()
        }
        "AI_GATEWAY_MUTABLE_MCP_CAPABILITY_TYPES" => {
            serde_json::to_value(constants::AI_GATEWAY_MUTABLE_MCP_CAPABILITY_TYPES).unwrap()
        }
        "AI_GATEWAY_REDACTED" => serde_json::to_value(constants::AI_GATEWAY_REDACTED).unwrap(),
        "AI_GATEWAY_SECRET_FIELDS" => serde_json::to_value(
            constants::AI_GATEWAY_SECRET_FIELDS
                .iter()
                .map(|(operation, rules)| (operation.as_str(), rules))
                .collect::<std::collections::BTreeMap<_, _>>(),
        )
        .unwrap(),
        "AI_GW_ADMIN_ENDPOINT" => serde_json::to_value(constants::AI_GW_ADMIN_ENDPOINT).unwrap(),
        "AI_GW_API_KEYS_SERVICE_PATH" => {
            serde_json::to_value(constants::AI_GW_API_KEYS_SERVICE_PATH).unwrap()
        }
        "AI_GW_API_KEYS_USER_PATH" => {
            serde_json::to_value(constants::AI_GW_API_KEYS_USER_PATH).unwrap()
        }
        "AI_GW_AUDIT_LOGS_PATH" => serde_json::to_value(constants::AI_GW_AUDIT_LOGS_PATH).unwrap(),
        "AI_GW_CHART_METRICS" => serde_json::to_value(constants::AI_GW_CHART_METRICS).unwrap(),
        "AI_GW_CHARTS_PATH" => serde_json::to_value(constants::AI_GW_CHARTS_PATH).unwrap(),
        "AI_GW_CONFIGS_PATH" => serde_json::to_value(constants::AI_GW_CONFIGS_PATH).unwrap(),
        "AI_GW_DATA_ENDPOINT" => serde_json::to_value(constants::AI_GW_DATA_ENDPOINT).unwrap(),
        "AI_GW_DEPLOYMENTS_PATH" => {
            serde_json::to_value(constants::AI_GW_DEPLOYMENTS_PATH).unwrap()
        }
        "AI_GW_GROUP_COLUMNS" => serde_json::to_value(constants::AI_GW_GROUP_COLUMNS).unwrap(),
        "AI_GW_GROUP_DIMENSIONS" => {
            serde_json::to_value(constants::AI_GW_GROUP_DIMENSIONS).unwrap()
        }
        "AI_GW_GROUPS_PATH" => serde_json::to_value(constants::AI_GW_GROUPS_PATH).unwrap(),
        "AI_GW_GUARDRAILS_PATH" => serde_json::to_value(constants::AI_GW_GUARDRAILS_PATH).unwrap(),
        "AI_GW_INTEGRATIONS_PATH" => {
            serde_json::to_value(constants::AI_GW_INTEGRATIONS_PATH).unwrap()
        }
        "AI_GW_LOGS_PATH" => serde_json::to_value(constants::AI_GW_LOGS_PATH).unwrap(),
        "AI_GW_MCP_INTEGRATIONS_PATH" => {
            serde_json::to_value(constants::AI_GW_MCP_INTEGRATIONS_PATH).unwrap()
        }
        "AI_GW_ORGANISATIONS_SELF_PATH" => {
            serde_json::to_value(constants::AI_GW_ORGANISATIONS_SELF_PATH).unwrap()
        }
        "AI_GW_PLUGINS_PATH" => serde_json::to_value(constants::AI_GW_PLUGINS_PATH).unwrap(),
        "AI_GW_PROVIDER_CATALOG_PATH" => {
            serde_json::to_value(constants::AI_GW_PROVIDER_CATALOG_PATH).unwrap()
        }
        "AI_GW_PROVIDERS_PATH" => serde_json::to_value(constants::AI_GW_PROVIDERS_PATH).unwrap(),
        "AI_GW_WORKSPACES_PATH" => serde_json::to_value(constants::AI_GW_WORKSPACES_PATH).unwrap(),
        "AI_SEC_API_ENDPOINT" => serde_json::to_value(constants::AI_SEC_API_ENDPOINT).unwrap(),
        "AI_SEC_API_KEY" => serde_json::to_value(constants::AI_SEC_API_KEY).unwrap(),
        "AI_SEC_API_TOKEN" => serde_json::to_value(constants::AI_SEC_API_TOKEN).unwrap(),
        "AIRS_ENDPOINTS" => serde_json::to_value(
            constants::AIRS_ENDPOINTS
                .iter()
                .copied()
                .collect::<std::collections::BTreeMap<_, _>>(),
        )
        .unwrap(),
        "ASYNC_SCAN_PATH" => serde_json::to_value(constants::ASYNC_SCAN_PATH).unwrap(),
        "BEARER" => serde_json::to_value(constants::BEARER).unwrap(),
        "DEFAULT_AI_GW_ADMIN_ENDPOINT" => {
            serde_json::to_value(constants::DEFAULT_AI_GW_ADMIN_ENDPOINT).unwrap()
        }
        "DEFAULT_AI_GW_DATA_ENDPOINT" => {
            serde_json::to_value(constants::DEFAULT_AI_GW_DATA_ENDPOINT).unwrap()
        }
        "DEFAULT_DLP_ENDPOINT" => serde_json::to_value(constants::DEFAULT_DLP_ENDPOINT).unwrap(),
        "DEFAULT_ENDPOINT" => serde_json::to_value(constants::DEFAULT_ENDPOINT).unwrap(),
        "DEFAULT_IAM_ENDPOINT" => serde_json::to_value(constants::DEFAULT_IAM_ENDPOINT).unwrap(),
        "DEFAULT_MGMT_ENDPOINT" => serde_json::to_value(constants::DEFAULT_MGMT_ENDPOINT).unwrap(),
        "DEFAULT_MODEL_SEC_DATA_ENDPOINT" => {
            serde_json::to_value(constants::DEFAULT_MODEL_SEC_DATA_ENDPOINT).unwrap()
        }
        "DEFAULT_MODEL_SEC_MGMT_ENDPOINT" => {
            serde_json::to_value(constants::DEFAULT_MODEL_SEC_MGMT_ENDPOINT).unwrap()
        }
        "DEFAULT_RED_TEAM_DATA_ENDPOINT" => {
            serde_json::to_value(constants::DEFAULT_RED_TEAM_DATA_ENDPOINT).unwrap()
        }
        "DEFAULT_RED_TEAM_MGMT_ENDPOINT" => {
            serde_json::to_value(constants::DEFAULT_RED_TEAM_MGMT_ENDPOINT).unwrap()
        }
        "DEFAULT_RED_TEAM_NETWORK_BROKER_ENDPOINT" => {
            serde_json::to_value(constants::DEFAULT_RED_TEAM_NETWORK_BROKER_ENDPOINT).unwrap()
        }
        "DEFAULT_TOKEN_ENDPOINT" => {
            serde_json::to_value(constants::DEFAULT_TOKEN_ENDPOINT).unwrap()
        }
        "DLP_DATA_FILTERING_PROFILES_PATH" => {
            serde_json::to_value(constants::DLP_DATA_FILTERING_PROFILES_PATH).unwrap()
        }
        "DLP_DATA_PATTERNS_PATH" => {
            serde_json::to_value(constants::DLP_DATA_PATTERNS_PATH).unwrap()
        }
        "DLP_DATA_PROFILES_PATH" => {
            serde_json::to_value(constants::DLP_DATA_PROFILES_PATH).unwrap()
        }
        "DLP_DICTIONARIES_PATH" => serde_json::to_value(constants::DLP_DICTIONARIES_PATH).unwrap(),
        "HEADER_API_KEY" => serde_json::to_value(constants::HEADER_API_KEY).unwrap(),
        "HEADER_AUTH_TOKEN" => serde_json::to_value(constants::HEADER_AUTH_TOKEN).unwrap(),
        "HTTP_FORCE_RETRY_STATUS_CODES" => {
            serde_json::to_value(constants::HTTP_FORCE_RETRY_STATUS_CODES).unwrap()
        }
        "IAM_ENDPOINT" => serde_json::to_value(constants::IAM_ENDPOINT).unwrap(),
        "IAM_SCOPES_PATH" => serde_json::to_value(constants::IAM_SCOPES_PATH).unwrap(),
        "MAX_AI_PROFILE_NAME_LENGTH" => {
            serde_json::to_value(constants::MAX_AI_PROFILE_NAME_LENGTH).unwrap()
        }
        "MAX_API_KEY_LENGTH" => serde_json::to_value(constants::MAX_API_KEY_LENGTH).unwrap(),
        "MAX_CONNECTION_POOL_SIZE" => {
            serde_json::to_value(constants::MAX_CONNECTION_POOL_SIZE).unwrap()
        }
        "MAX_CONTENT_CONTEXT_LENGTH" => {
            serde_json::to_value(constants::MAX_CONTENT_CONTEXT_LENGTH).unwrap()
        }
        "MAX_CONTENT_PROMPT_LENGTH" => {
            serde_json::to_value(constants::MAX_CONTENT_PROMPT_LENGTH).unwrap()
        }
        "MAX_CONTENT_RESPONSE_LENGTH" => {
            serde_json::to_value(constants::MAX_CONTENT_RESPONSE_LENGTH).unwrap()
        }
        "MAX_NUMBER_OF_BATCH_SCAN_OBJECTS" => {
            serde_json::to_value(constants::MAX_NUMBER_OF_BATCH_SCAN_OBJECTS).unwrap()
        }
        "MAX_NUMBER_OF_REPORT_IDS" => {
            serde_json::to_value(constants::MAX_NUMBER_OF_REPORT_IDS).unwrap()
        }
        "MAX_NUMBER_OF_RETRIES" => serde_json::to_value(constants::MAX_NUMBER_OF_RETRIES).unwrap(),
        "MAX_NUMBER_OF_SCAN_IDS" => {
            serde_json::to_value(constants::MAX_NUMBER_OF_SCAN_IDS).unwrap()
        }
        "MAX_REPORT_ID_STR_LENGTH" => {
            serde_json::to_value(constants::MAX_REPORT_ID_STR_LENGTH).unwrap()
        }
        "MAX_SCAN_ID_STR_LENGTH" => {
            serde_json::to_value(constants::MAX_SCAN_ID_STR_LENGTH).unwrap()
        }
        "MAX_SESSION_ID_STR_LENGTH" => {
            serde_json::to_value(constants::MAX_SESSION_ID_STR_LENGTH).unwrap()
        }
        "MAX_TOKEN_LENGTH" => serde_json::to_value(constants::MAX_TOKEN_LENGTH).unwrap(),
        "MAX_TRANSACTION_ID_STR_LENGTH" => {
            serde_json::to_value(constants::MAX_TRANSACTION_ID_STR_LENGTH).unwrap()
        }
        "MGMT_API_KEY_PATH" => serde_json::to_value(constants::MGMT_API_KEY_PATH).unwrap(),
        "MGMT_API_KEYS_TSG_PATH" => {
            serde_json::to_value(constants::MGMT_API_KEYS_TSG_PATH).unwrap()
        }
        "MGMT_CLIENT_ID" => serde_json::to_value(constants::MGMT_CLIENT_ID).unwrap(),
        "MGMT_CLIENT_SECRET" => serde_json::to_value(constants::MGMT_CLIENT_SECRET).unwrap(),
        "MGMT_CUSTOMER_APP_PATH" => {
            serde_json::to_value(constants::MGMT_CUSTOMER_APP_PATH).unwrap()
        }
        "MGMT_CUSTOMER_APPS_TSG_PATH" => {
            serde_json::to_value(constants::MGMT_CUSTOMER_APPS_TSG_PATH).unwrap()
        }
        "MGMT_DASHBOARD_APPLICATION_PATH" => {
            serde_json::to_value(constants::MGMT_DASHBOARD_APPLICATION_PATH).unwrap()
        }
        "MGMT_DASHBOARD_APPLICATION_VIOLATION_BREAKDOWN_PATH" => {
            serde_json::to_value(constants::MGMT_DASHBOARD_APPLICATION_VIOLATION_BREAKDOWN_PATH)
                .unwrap()
        }
        "MGMT_DASHBOARD_APPLICATIONS_OVERVIEW_PATH" => {
            serde_json::to_value(constants::MGMT_DASHBOARD_APPLICATIONS_OVERVIEW_PATH).unwrap()
        }
        "MGMT_DASHBOARD_APPLICATIONS_VIOLATIONS_TREND_PATH" => {
            serde_json::to_value(constants::MGMT_DASHBOARD_APPLICATIONS_VIOLATIONS_TREND_PATH)
                .unwrap()
        }
        "MGMT_DASHBOARD_APPS_LIST_PATH" => {
            serde_json::to_value(constants::MGMT_DASHBOARD_APPS_LIST_PATH).unwrap()
        }
        "MGMT_DASHBOARD_SESSION_PATH" => {
            serde_json::to_value(constants::MGMT_DASHBOARD_SESSION_PATH).unwrap()
        }
        "MGMT_DASHBOARD_SESSION_TRANSACTION_PATH" => {
            serde_json::to_value(constants::MGMT_DASHBOARD_SESSION_TRANSACTION_PATH).unwrap()
        }
        "MGMT_DASHBOARD_SESSIONS_CHART_PATH" => {
            serde_json::to_value(constants::MGMT_DASHBOARD_SESSIONS_CHART_PATH).unwrap()
        }
        "MGMT_DASHBOARD_SESSIONS_OVERVIEW_PATH" => {
            serde_json::to_value(constants::MGMT_DASHBOARD_SESSIONS_OVERVIEW_PATH).unwrap()
        }
        "MGMT_DASHBOARD_TOP_APPLICATIONS_VIOLATIONS_PATH" => {
            serde_json::to_value(constants::MGMT_DASHBOARD_TOP_APPLICATIONS_VIOLATIONS_PATH)
                .unwrap()
        }
        "MGMT_DEPLOYMENT_PROFILES_PATH" => {
            serde_json::to_value(constants::MGMT_DEPLOYMENT_PROFILES_PATH).unwrap()
        }
        "MGMT_DLP_PROFILES_PATH" => {
            serde_json::to_value(constants::MGMT_DLP_PROFILES_PATH).unwrap()
        }
        "MGMT_ENDPOINT" => serde_json::to_value(constants::MGMT_ENDPOINT).unwrap(),
        "MGMT_OAUTH_INVALIDATE_PATH" => {
            serde_json::to_value(constants::MGMT_OAUTH_INVALIDATE_PATH).unwrap()
        }
        "MGMT_OAUTH_TOKEN_PATH" => serde_json::to_value(constants::MGMT_OAUTH_TOKEN_PATH).unwrap(),
        "MGMT_PROFILE_PATH" => serde_json::to_value(constants::MGMT_PROFILE_PATH).unwrap(),
        "MGMT_PROFILES_TSG_PATH" => {
            serde_json::to_value(constants::MGMT_PROFILES_TSG_PATH).unwrap()
        }
        "MGMT_REPORT_SCAN_CONTENT_PATH" => {
            serde_json::to_value(constants::MGMT_REPORT_SCAN_CONTENT_PATH).unwrap()
        }
        "MGMT_SCAN_LOGS_PATH" => serde_json::to_value(constants::MGMT_SCAN_LOGS_PATH).unwrap(),
        "MGMT_TOKEN_ENDPOINT" => serde_json::to_value(constants::MGMT_TOKEN_ENDPOINT).unwrap(),
        "MGMT_TOPIC_FORCE_PATH" => serde_json::to_value(constants::MGMT_TOPIC_FORCE_PATH).unwrap(),
        "MGMT_TOPIC_PATH" => serde_json::to_value(constants::MGMT_TOPIC_PATH).unwrap(),
        "MGMT_TOPICS_TSG_PATH" => serde_json::to_value(constants::MGMT_TOPICS_TSG_PATH).unwrap(),
        "MGMT_TSG_ID" => serde_json::to_value(constants::MGMT_TSG_ID).unwrap(),
        "MODEL_SEC_CLIENT_ID" => serde_json::to_value(constants::MODEL_SEC_CLIENT_ID).unwrap(),
        "MODEL_SEC_CLIENT_SECRET" => {
            serde_json::to_value(constants::MODEL_SEC_CLIENT_SECRET).unwrap()
        }
        "MODEL_SEC_DATA_ENDPOINT" => {
            serde_json::to_value(constants::MODEL_SEC_DATA_ENDPOINT).unwrap()
        }
        "MODEL_SEC_EVALUATIONS_PATH" => {
            serde_json::to_value(constants::MODEL_SEC_EVALUATIONS_PATH).unwrap()
        }
        "MODEL_SEC_MGMT_ENDPOINT" => {
            serde_json::to_value(constants::MODEL_SEC_MGMT_ENDPOINT).unwrap()
        }
        "MODEL_SEC_MODEL_VERSIONS_PATH" => {
            serde_json::to_value(constants::MODEL_SEC_MODEL_VERSIONS_PATH).unwrap()
        }
        "MODEL_SEC_MODELS_PATH" => serde_json::to_value(constants::MODEL_SEC_MODELS_PATH).unwrap(),
        "MODEL_SEC_PYPI_AUTH_PATH" => {
            serde_json::to_value(constants::MODEL_SEC_PYPI_AUTH_PATH).unwrap()
        }
        "MODEL_SEC_SCANS_PATH" => serde_json::to_value(constants::MODEL_SEC_SCANS_PATH).unwrap(),
        "MODEL_SEC_SECURITY_GROUPS_PATH" => {
            serde_json::to_value(constants::MODEL_SEC_SECURITY_GROUPS_PATH).unwrap()
        }
        "MODEL_SEC_SECURITY_RULES_PATH" => {
            serde_json::to_value(constants::MODEL_SEC_SECURITY_RULES_PATH).unwrap()
        }
        "MODEL_SEC_TOKEN_ENDPOINT" => {
            serde_json::to_value(constants::MODEL_SEC_TOKEN_ENDPOINT).unwrap()
        }
        "MODEL_SEC_TSG_ID" => serde_json::to_value(constants::MODEL_SEC_TSG_ID).unwrap(),
        "MODEL_SEC_VIOLATIONS_PATH" => {
            serde_json::to_value(constants::MODEL_SEC_VIOLATIONS_PATH).unwrap()
        }
        "PAYLOAD_HASH" => serde_json::to_value(constants::PAYLOAD_HASH).unwrap(),
        "RED_TEAM_ADAPTER_PATH" => serde_json::to_value(constants::RED_TEAM_ADAPTER_PATH).unwrap(),
        "RED_TEAM_ADAPTER_VALIDATE_PATH" => {
            serde_json::to_value(constants::RED_TEAM_ADAPTER_VALIDATE_PATH).unwrap()
        }
        "RED_TEAM_CATEGORIES_PATH" => {
            serde_json::to_value(constants::RED_TEAM_CATEGORIES_PATH).unwrap()
        }
        "RED_TEAM_CHANNELS_PATH" => {
            serde_json::to_value(constants::RED_TEAM_CHANNELS_PATH).unwrap()
        }
        "RED_TEAM_CHANNELS_STATS_PATH" => {
            serde_json::to_value(constants::RED_TEAM_CHANNELS_STATS_PATH).unwrap()
        }
        "RED_TEAM_CLIENT_ID" => serde_json::to_value(constants::RED_TEAM_CLIENT_ID).unwrap(),
        "RED_TEAM_CLIENT_SECRET" => {
            serde_json::to_value(constants::RED_TEAM_CLIENT_SECRET).unwrap()
        }
        "RED_TEAM_CUSTOM_ATTACK_PATH" => {
            serde_json::to_value(constants::RED_TEAM_CUSTOM_ATTACK_PATH).unwrap()
        }
        "RED_TEAM_CUSTOM_ATTACKS_REPORT_PATH" => {
            serde_json::to_value(constants::RED_TEAM_CUSTOM_ATTACKS_REPORT_PATH).unwrap()
        }
        "RED_TEAM_DASHBOARD_PATH" => {
            serde_json::to_value(constants::RED_TEAM_DASHBOARD_PATH).unwrap()
        }
        "RED_TEAM_DATA_ENDPOINT" => {
            serde_json::to_value(constants::RED_TEAM_DATA_ENDPOINT).unwrap()
        }
        "RED_TEAM_ERROR_LOG_PATH" => {
            serde_json::to_value(constants::RED_TEAM_ERROR_LOG_PATH).unwrap()
        }
        "RED_TEAM_ERROR_LOG_TARGET_PROFILE_PATH" => {
            serde_json::to_value(constants::RED_TEAM_ERROR_LOG_TARGET_PROFILE_PATH).unwrap()
        }
        "RED_TEAM_EULA_PATH" => serde_json::to_value(constants::RED_TEAM_EULA_PATH).unwrap(),
        "RED_TEAM_INSTANCES_PATH" => {
            serde_json::to_value(constants::RED_TEAM_INSTANCES_PATH).unwrap()
        }
        "RED_TEAM_LANGUAGES_PATH" => {
            serde_json::to_value(constants::RED_TEAM_LANGUAGES_PATH).unwrap()
        }
        "RED_TEAM_MGMT_DASHBOARD_PATH" => {
            serde_json::to_value(constants::RED_TEAM_MGMT_DASHBOARD_PATH).unwrap()
        }
        "RED_TEAM_MGMT_ENDPOINT" => {
            serde_json::to_value(constants::RED_TEAM_MGMT_ENDPOINT).unwrap()
        }
        "RED_TEAM_NETWORK_BROKER_ENDPOINT" => {
            serde_json::to_value(constants::RED_TEAM_NETWORK_BROKER_ENDPOINT).unwrap()
        }
        "RED_TEAM_QUOTA_PATH" => serde_json::to_value(constants::RED_TEAM_QUOTA_PATH).unwrap(),
        "RED_TEAM_REGISTRY_CREDENTIALS_PATH" => {
            serde_json::to_value(constants::RED_TEAM_REGISTRY_CREDENTIALS_PATH).unwrap()
        }
        "RED_TEAM_REPORT_DYNAMIC_PATH" => {
            serde_json::to_value(constants::RED_TEAM_REPORT_DYNAMIC_PATH).unwrap()
        }
        "RED_TEAM_REPORT_PATH" => serde_json::to_value(constants::RED_TEAM_REPORT_PATH).unwrap(),
        "RED_TEAM_REPORT_STATIC_PATH" => {
            serde_json::to_value(constants::RED_TEAM_REPORT_STATIC_PATH).unwrap()
        }
        "RED_TEAM_SCAN_PATH" => serde_json::to_value(constants::RED_TEAM_SCAN_PATH).unwrap(),
        "RED_TEAM_SENTIMENT_PATH" => {
            serde_json::to_value(constants::RED_TEAM_SENTIMENT_PATH).unwrap()
        }
        "RED_TEAM_TARGET_PATH" => serde_json::to_value(constants::RED_TEAM_TARGET_PATH).unwrap(),
        "RED_TEAM_TARGET_VALIDATE_AUTH_PATH" => {
            serde_json::to_value(constants::RED_TEAM_TARGET_VALIDATE_AUTH_PATH).unwrap()
        }
        "RED_TEAM_TEMPLATE_PATH" => {
            serde_json::to_value(constants::RED_TEAM_TEMPLATE_PATH).unwrap()
        }
        "RED_TEAM_TOKEN_ENDPOINT" => {
            serde_json::to_value(constants::RED_TEAM_TOKEN_ENDPOINT).unwrap()
        }
        "RED_TEAM_TSG_ID" => serde_json::to_value(constants::RED_TEAM_TSG_ID).unwrap(),
        "SCAN_REPORTS_PATH" => serde_json::to_value(constants::SCAN_REPORTS_PATH).unwrap(),
        "SCAN_RESULTS_PATH" => serde_json::to_value(constants::SCAN_RESULTS_PATH).unwrap(),
        "SDK_VERSION" => serde_json::to_value(constants::SDK_VERSION).unwrap(),
        "SYNC_SCAN_PATH" => serde_json::to_value(constants::SYNC_SCAN_PATH).unwrap(),
        "TSG_ID_HEADER" => serde_json::to_value(constants::TSG_ID_HEADER).unwrap(),
        "USER_AGENT" => serde_json::to_value(constants::USER_AGENT).unwrap(),
        _ => panic!("unmapped frozen constant"),
    }
}
#[test]
fn all_163_frozen_constant_exports_have_values_or_explicit_native_configuration_adaptations() {
    let fixture = constants_fixture();
    let rows = fixture["constants"].as_array().unwrap();
    assert_eq!(rows.len(), 163);
    for row in rows {
        let name = row["name"].as_str().unwrap();
        match name {
            "globalConfiguration" => {
                assert_eq!(
                    row["nativeAdaptation"],
                    "isolated caller-owned configuration"
                )
            }
            "SDK_VERSION" => {
                assert_eq!(constant_value(name), env!("CARGO_PKG_VERSION"));
                assert_eq!(constants::BASELINE_SDK_VERSION, row["value"]);
            }
            "USER_AGENT" => assert_eq!(
                constant_value(name),
                format!("PAN-AIRS/{}-rust-sdk", env!("CARGO_PKG_VERSION"))
            ),
            _ => assert_eq!(constant_value(name), row["value"], "{name}"),
        }
    }
}

#[tokio::test]
async fn arbitrary_stream_collection_stops_before_polling_after_item_cap_and_resumes_byte_caps() {
    let calls = Arc::new(AtomicUsize::new(0));
    let seen = calls.clone();
    let source = stream::iter([
        Ok("one".to_string()),
        Ok("two".to_string()),
        Ok("three".to_string()),
    ])
    .inspect(move |_| {
        seen.fetch_add(1, Ordering::SeqCst);
    });
    let mut listing = StreamListing::new(source);
    let session = Session::new();
    let options = budget();
    let result = collect_all(
        &mut listing,
        CollectOptions::default().with_max_items(1).unwrap(),
        &session,
        &options,
    )
    .await
    .unwrap();
    assert_eq!(result.items, ["one"]);
    assert_eq!(result.completion, Completion::Truncated(Truncation::Items));
    assert_eq!(calls.load(Ordering::SeqCst), 1);
    let result = collect_all(
        &mut listing,
        CollectOptions::default().with_max_bytes(2).unwrap(),
        &session,
        &options,
    )
    .await
    .unwrap();
    assert!(result.items.is_empty());
    assert_eq!(result.completion, Completion::Truncated(Truncation::Bytes));
    assert_eq!(calls.load(Ordering::SeqCst), 2);
    let result = collect_all(&mut listing, CollectOptions::default(), &session, &options)
        .await
        .unwrap();
    assert_eq!(result.items, ["two", "three"]);
    assert!(result.is_complete());
    assert_eq!(calls.load(Ordering::SeqCst), 3);
    assert!(!format!("{listing:?}").contains("three"));
}
#[tokio::test(start_paused = true)]
async fn arbitrary_stream_errors_and_cancellation_keep_partial_results_with_a_shared_budget() {
    let source = stream::iter([Ok(1), Err(Error::new(ErrorKind::Transport)), Ok(2)]);
    let mut listing = StreamListing::new(source);
    let session = Session::new();
    let options = budget();
    let result = collect_all(&mut listing, CollectOptions::default(), &session, &options)
        .await
        .unwrap_err();
    assert_eq!(result.partial().items, [1]);
    assert_eq!(result.partial().completion, Completion::Interrupted);
    assert_eq!(result.error().kind(), ErrorKind::Transport);
    let result = collect_all(&mut listing, CollectOptions::default(), &session, &options)
        .await
        .unwrap();
    assert_eq!(result.items, [2]);
    assert!(result.is_complete());
    let options = budget();
    options.cancel();
    let mut listing = StreamListing::new(stream::iter([Ok(1)]));
    let result = collect_all(&mut listing, CollectOptions::default(), &session, &options)
        .await
        .unwrap_err();
    assert!(result.partial().items.is_empty());
    assert_eq!(result.error().kind(), ErrorKind::Cancelled);
    let options = RequestOptions::with_timeout(Duration::from_millis(1)).unwrap();
    let mut listing = StreamListing::new(stream::pending::<scm_sdk::Result<i32>>());
    let result = collect_all(&mut listing, CollectOptions::default(), &session, &options)
        .await
        .unwrap_err();
    assert_eq!(result.error().kind(), ErrorKind::Deadline);
}
#[tokio::test]
async fn generic_skip_and_spring_walks_preserve_partial_caps_and_fail_before_invalid_fetches() {
    let calls = Arc::new(Mutex::new(Vec::new()));
    let seen = calls.clone();
    let mut walk = WalkAllOptions::default();
    walk.limit = 2;
    walk.collect = CollectOptions::default().with_max_items(1).unwrap();
    let result = collect_skip_pages(
        move |skip, limit, options| {
            seen.lock().unwrap().push((skip, limit, options.deadline()));
            async move {
                Ok(SkipPage {
                    items: vec![1, 2],
                    total: Some(4.into()),
                })
            }
        },
        walk,
        Session::new(),
        budget(),
    )
    .await
    .unwrap();
    assert_eq!(result.items, [1]);
    assert_eq!(result.resume.unwrap().item_offset, 1);
    assert_eq!(calls.lock().unwrap().len(), 1);
    let calls = Arc::new(AtomicUsize::new(0));
    let seen = calls.clone();
    let mut walk = WalkAllOptions::default();
    walk.limit = 0;
    assert!(
        collect_skip_pages(
            move |_, _, _| {
                seen.fetch_add(1, Ordering::SeqCst);
                async {
                    Ok(SkipPage::<i32> {
                        items: vec![],
                        total: None,
                    })
                }
            },
            walk,
            Session::new(),
            budget()
        )
        .await
        .is_err()
    );
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    let result = collect_skip_pages(
        |_, _, _| async {
            Ok(SkipPage::<i32> {
                items: vec![],
                total: Some((-1).into()),
            })
        },
        WalkAllOptions::default(),
        Session::new(),
        budget(),
    )
    .await
    .unwrap_err();
    assert_eq!(result.error().kind(), ErrorKind::Pagination);
    let result = collect_spring_pages(
        |page, _, _| async move {
            if page == 0 {
                Ok(SpringPage {
                    items: vec![1],
                    last: false,
                })
            } else {
                Err(Error::new(ErrorKind::Transport))
            }
        },
        SpringWalkOptions::default(),
        Session::new(),
        budget(),
    )
    .await
    .unwrap_err();
    assert_eq!(result.partial().items, [1]);
    assert_eq!(result.partial().resume.as_ref().unwrap().cursor, 1);
}
#[tokio::test]
async fn generic_cursor_iterator_rejects_repeated_positions_and_checks_buffered_cancellation() {
    let options = budget();
    let mut items = paginate(
        0u64,
        |_, _| async {
            Ok(Page {
                items: vec![1, 2],
                next: Some(0),
            })
        },
        3,
        Session::new(),
        options.clone(),
    )
    .unwrap();
    assert_eq!(items.next().await.unwrap(), Some(1));
    options.cancel();
    assert_eq!(items.next().await.unwrap_err().kind(), ErrorKind::Cancelled);
    let mut items = paginate(
        0u64,
        |_, _| async {
            Ok(Page {
                items: vec![1],
                next: Some(0),
            })
        },
        3,
        Session::new(),
        budget(),
    )
    .unwrap();
    assert_eq!(items.next().await.unwrap(), Some(1));
    assert_eq!(
        items.next().await.unwrap_err().kind(),
        ErrorKind::Pagination
    );
}
#[test]
fn listing_and_nullable_composition_preserve_explicit_states_and_use_the_inner_parser() {
    let mut options = ListingOptions::default();
    options.skip = Some(0);
    options.limit = Some(0);
    options.search = Some(String::new());
    assert_eq!(
        serialize_listing(Some(&options)).unwrap(),
        vec![
            ("skip".into(), "0".into()),
            ("limit".into(), "0".into()),
            ("search".into(), "".into())
        ]
    );
    let calls = AtomicUsize::new(0);
    for input in [None, Some(&Value::Null)] {
        assert!(
            json_nullable::<String>(input, |_| {
                calls.fetch_add(1, Ordering::SeqCst);
                Ok("wrong".into())
            })
            .is_ok()
        );
    }
    assert_eq!(calls.load(Ordering::SeqCst), 0);
    assert_eq!(
        json_nullable(Some(&json!("value")), |value| Ok(value
            .as_str()
            .unwrap()
            .to_owned()))
        .unwrap(),
        scm_sdk::data::Patch::Value("value".to_string())
    );
    let mut invalid = ListingOptions::default();
    invalid.skip = Some(u64::MAX);
    assert!(serialize_listing(Some(&invalid)).is_err());
    invalid.skip = None;
    invalid.search = Some("secret\nvalue".into());
    assert!(serialize_listing(Some(&invalid)).is_err());
    assert!(!format!("{invalid:?}").contains("secret"));
}
#[test]
fn init_returns_independent_immutable_clients_and_rejects_invalid_configuration() {
    use scm_sdk::{Client, Endpoint, Secret, auth::Authentication};
    let one = init(
        Client::builder(Endpoint::parse("https://one.example.test").unwrap()).authentication(
            Authentication::runtime_key(Secret::new("synthetic-one").unwrap()).unwrap(),
        ),
    );
    let two = init(
        Client::builder(Endpoint::parse("https://two.example.test").unwrap()).authentication(
            Authentication::runtime_key(Secret::new("synthetic-two").unwrap()).unwrap(),
        ),
    );
    // Default transport can be disabled in this test configuration; constructors then
    // fail independently without reading process globals or dispatching requests.
    assert_eq!(one.is_ok(), cfg!(feature = "reqwest-transport"));
    assert_eq!(two.is_ok(), cfg!(feature = "reqwest-transport"));
}

#[cfg(feature = "gateway")]
mod gateway {
    use super::*;
    use scm_sdk::airs::gateway::models::GatewayDottedValueEntry;
    use scm_sdk::airs::gateway::*;
    use scm_sdk::data::{JsonObject, JsonValue};
    fn invalid() -> Error {
        Error::new(ErrorKind::Validation)
    }
    async fn invoke(case: &Value) -> scm_sdk::Result<Value> {
        let input = &case["input"];
        match case["helper"].as_str().unwrap() {
            "buildDottedObject" => {
                let entries: Vec<GatewayDottedValueEntry> =
                    serde_json::from_value(input.clone()).map_err(|_| invalid())?;
                serde_json::to_value(build_dotted_object(&entries)?).map_err(|_| invalid())
            }
            "setDottedValue" => serde_json::to_value(set_dotted_value(
                &serde_json::from_value::<JsonObject>(input["object"].clone())
                    .map_err(|_| invalid())?,
                input["path"].as_str().unwrap(),
                &JsonValue::new(input["value"].clone()),
            )?)
            .map_err(|_| invalid()),
            "customHostConfiguration" => {
                let mut options =
                    CustomHostConfigurationOptions::new(input["host"].as_str().unwrap());
                if let Some(headers) = input["headers"].as_object() {
                    for (name, value) in headers {
                        options
                            .headers
                            .insert(name.clone(), value.as_str().unwrap().to_owned());
                    }
                }
                serde_json::to_value(custom_host_configuration(&options)?).map_err(|_| invalid())
            }
            "generateWorkspaceScopeName" => {
                let value = generate_workspace_scope_name(input["name"].as_str().unwrap())?;
                let suffix = &value[value.len() - 6..];
                Ok(
                    json!({"prefix":&value[..value.len()-6],"validSuffix":suffix.len()==6 && suffix.bytes().all(|v|v.is_ascii_lowercase()||v.is_ascii_digit())}),
                )
            }
            "serializeListing" => {
                let options = if input.is_null() {
                    None
                } else {
                    let mut options = ListingOptions::default();
                    options.skip = input["skip"].as_u64();
                    options.limit = input["limit"].as_u64().map(|v| v as u32);
                    options.search = input["search"].as_str().map(str::to_owned);
                    Some(options)
                };
                Ok(serde_json::to_value(
                    serialize_listing(options.as_ref())?
                        .into_iter()
                        .collect::<std::collections::BTreeMap<_, _>>(),
                )
                .unwrap())
            }
            "aiGwOrganisationsAuthSettingsPath" => Ok(json!(
                ai_gw_organisations_auth_settings_path(input["tsgId"].as_str().unwrap())?
            )),
            "redactAIGatewaySecrets" => {
                let operation =
                    AIGatewaySecretOperation::parse(input["operation"].as_str().unwrap())?;
                let direction =
                    serde_json::from_value(input["direction"].clone()).map_err(|_| invalid())?;
                serde_json::to_value(redact_ai_gateway_secrets(
                    operation,
                    &JsonValue::new(input["payload"].clone()),
                    Some(direction),
                )?)
                .map_err(|_| invalid())
            }
            "jsonNullable" => {
                let value = match input["presence"].as_str().unwrap() {
                    "unset" => None,
                    "null" => Some(&Value::Null),
                    _ => Some(&input["value"]),
                };
                let result = json_nullable(value, |value| {
                    value
                        .as_str()
                        .filter(|v| !v.is_empty())
                        .map(str::to_owned)
                        .ok_or_else(invalid)
                })?;
                Ok(match result {
                    scm_sdk::data::Patch::Unset => json!({"presence":"unset"}),
                    scm_sdk::data::Patch::Null => json!({"presence":"null"}),
                    scm_sdk::data::Patch::Value(value) => json!({"presence":"value","value":value}),
                })
            }
            "pageSchema" => serde_json::to_value(page_schema(input, |value| {
                if value["id"].is_string() {
                    Ok(value.clone())
                } else {
                    Err(invalid())
                }
            })?)
            .map_err(|_| invalid()),
            "collectAll" => {
                let items = input["items"].as_array().unwrap().clone();
                let mut listing = StreamListing::new(stream::iter(items.into_iter().map(Ok)));
                let result = collect_all(
                    &mut listing,
                    CollectOptions::default()
                        .with_max_items(input["max"].as_u64().unwrap() as usize)?,
                    &Session::new(),
                    &budget(),
                )
                .await
                .map_err(|error| error.into_error())?;
                Ok(json!(result.items))
            }
            "collectSkipPages" => {
                let pages = Arc::new(Mutex::new(
                    input["pages"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .cloned()
                        .collect::<std::collections::VecDeque<_>>(),
                ));
                let mut options = WalkAllOptions::default();
                options.limit = input["limit"].as_u64().unwrap() as u32;
                let result = collect_skip_pages(
                    move |_, _, _| {
                        let page = pages
                            .lock()
                            .unwrap()
                            .pop_front()
                            .unwrap_or_else(|| json!({"items":[]}));
                        async move {
                            Ok(SkipPage {
                                items: page["items"].as_array().unwrap().clone(),
                                total: page["total"]
                                    .as_f64()
                                    .map(scm_sdk::data::FiniteNumber::try_from)
                                    .transpose()?,
                            })
                        }
                    },
                    options,
                    Session::new(),
                    budget(),
                )
                .await
                .map_err(|e| e.into_error())?;
                Ok(json!(result.items))
            }
            "collectSpringPages" => {
                let pages = Arc::new(Mutex::new(
                    input["pages"]
                        .as_array()
                        .unwrap()
                        .iter()
                        .cloned()
                        .collect::<std::collections::VecDeque<_>>(),
                ));
                let mut options = SpringWalkOptions::default();
                options.size = input["size"].as_u64().unwrap() as u32;
                let result = collect_spring_pages(
                    move |_, _, _| {
                        let page = pages.lock().unwrap().pop_front().unwrap();
                        async move {
                            Ok(SpringPage {
                                items: page["items"].as_array().unwrap().clone(),
                                last: page["last"].as_bool().unwrap(),
                            })
                        }
                    },
                    options,
                    Session::new(),
                    budget(),
                )
                .await
                .map_err(|e| e.into_error())?;
                Ok(json!(result.items))
            }
            "paginate" => {
                let pages = input["pages"].as_array().unwrap().clone();
                let mut items = paginate(
                    0usize,
                    move |cursor, _| {
                        let page = pages[cursor].clone();
                        async move {
                            Ok(Page {
                                items: page["items"].as_array().unwrap().clone(),
                                next: page["next"].as_u64().map(|v| v as usize),
                            })
                        }
                    },
                    3,
                    Session::new(),
                    budget(),
                )?;
                let mut values = Vec::new();
                while let Some(value) = items.next().await? {
                    values.push(value);
                }
                Ok(json!(values))
            }
            _ => panic!("unknown helper fixture"),
        }
    }
    #[tokio::test]
    async fn all_157_independent_typescript_helper_cases_match_native_contracts() {
        let fixture = constants_fixture();
        let cases = fixture["cases"].as_array().unwrap();
        assert_eq!(cases.len(), 157);
        for case in cases {
            let result = invoke(case).await;
            if case["expected"]["success"] == true {
                assert_eq!(
                    result
                        .unwrap_or_else(|e| panic!("{} {}: {e:?}", case["helper"], case["label"])),
                    case["expected"]["output"],
                    "{} {}",
                    case["helper"],
                    case["label"]
                );
            } else {
                assert!(result.is_err(), "{} {}", case["helper"], case["label"]);
            }
        }
    }
    #[test]
    fn helper_boundaries_redact_errors_and_preserve_original_inputs() {
        let input = JsonValue::new(
            serde_json::from_str(
                r#"{"__proto__":{"safe":1},"key":"synthetic-secret","safe":"retained"}"#,
            )
            .unwrap(),
        );
        let output =
            redact_ai_gateway_secrets(AIGatewaySecretOperation::IntegrationsCreate, &input, None)
                .unwrap();
        assert_eq!(input.as_value()["key"], "synthetic-secret");
        assert_eq!(output.as_value()["key"], "[REDACTED]");
        assert_eq!(output.as_value()["__proto__"], json!({"safe":1}));
        assert!(AIGatewaySecretOperation::parse("unknown.operation").is_err());
        for path in [
            "a[100000]".to_string(),
            "a[999999999999999999999999999999999]".to_string(),
            "a".repeat(4097),
            (0..66).map(|_| "a").collect::<Vec<_>>().join("."),
        ] {
            let entry = GatewayDottedValueEntry::builder()
                .path(path)
                .value(scm_sdk::data::Nullable::Null)
                .build()
                .unwrap();
            let error = build_dotted_object(&[entry]).unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Limit);
            assert!(!format!("{error:?}").contains("999999"));
        }
        let mut deep = Value::Null;
        for _ in 0..65 {
            deep = json!({"a":deep});
        }
        assert_eq!(
            redact_ai_gateway_secrets(
                AIGatewaySecretOperation::IntegrationsCreate,
                &JsonValue::new(deep),
                None
            )
            .unwrap_err()
            .kind(),
            ErrorKind::Limit
        );
        let original = JsonObject::new(serde_json::from_value(json!({"a":[1]})).unwrap());
        assert!(set_dotted_value(&original, "a[2]", &JsonValue::new(json!(2))).is_err());
        assert_eq!(original.as_map().get("a").unwrap(), &json!([1]));
        for header in ["bad\nname", "bad header"] {
            assert!(
                custom_host_configuration(
                    &CustomHostConfigurationOptions::new("https://example.test")
                        .header(header, "secret")
                )
                .is_err()
            );
        }
        assert!(
            custom_host_configuration(
                &CustomHostConfigurationOptions::new("https://example.test")
                    .header("authorization", "secret\r\nInjected: yes")
            )
            .is_err()
        );
        assert!(
            custom_host_configuration(
                &CustomHostConfigurationOptions::new("https://example.test")
                    .header("X-Key", "one")
                    .header("x-key", "two")
            )
            .is_err()
        );
        assert!(ai_gw_organisations_auth_settings_path("../escape").is_err());
        assert!(generate_workspace_scope_name(&"x".repeat(4097)).is_err());
    }
    #[test]
    fn typed_secret_rule_metadata_and_all_new_helper_debug_output_are_safe() {
        assert_eq!(AI_GATEWAY_SECRET_FIELDS.len(), 21);
        assert_eq!(
            AI_GATEWAY_SECRET_FIELDS
                .iter()
                .map(|(_, rules)| rules.request.len() + rules.response.len())
                .sum::<usize>(),
            49
        );
        let options =
            CustomHostConfigurationOptions::new("https://user:synthetic-secret@example.test")
                .header("Authorization", "synthetic-secret");
        let configuration = custom_host_configuration(&options).unwrap();
        for text in [
            format!("{options:?}"),
            format!("{configuration:?}"),
            format!("{:?}", AI_GATEWAY_SECRET_FIELDS[0]),
        ] {
            assert!(!text.contains("synthetic-secret"));
        }
        assert_eq!(
            serde_json::to_value(configuration).unwrap()["custom_headers"]["Authorization"],
            "synthetic-secret"
        );
        for payload in [
            Value::Null,
            json!("scalar"),
            json!({"other":{"key":"retained"}}),
        ] {
            let output = redact_ai_gateway_secrets(
                AIGatewaySecretOperation::IntegrationsCreate,
                &JsonValue::new(payload.clone()),
                None,
            )
            .unwrap();
            assert_eq!(output.as_value(), &payload);
        }
    }
}
