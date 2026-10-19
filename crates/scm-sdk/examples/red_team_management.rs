//! Offline management 0.86.0 consumer: no credentials or service calls.
//! Run with `cargo run --example red_team_management --features red-team`.
use scm_sdk::{
    RequestOptions,
    airs::red_team::{
        TargetListOptions,
        models::{
            BasicAuthAuthConfig, MultiTurnClientSessionConfig, RestConnectionParams,
            StreamingConnectionParamsMultiTurnConfig,
            StreamingConnectionParamsTargetConnectionConfig, TargetCreateRequestResponseMode,
            TargetResponse, VertexAIConnectionParams,
        },
    },
    data::Patch,
    validation::Mode,
};
use serde_json::json;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vertex = VertexAIConnectionParams::builder()
        .project_id("example-project".into())
        .location("us-central1".into())
        .engine_id("12345".into())
        .customer_sa_email("service@example.invalid".into())
        .build()?;
    let provider = StreamingConnectionParamsTargetConnectionConfig::Variant5(vertex);
    // Explicitly opt into client_session; an absent tag keeps legacy selection.
    let session = StreamingConnectionParamsMultiTurnConfig::Variant3(
        MultiTurnClientSessionConfig::parse(json!({"type":"client_session"}), Mode::Request)?,
    );
    // Current values are on schema-bound enums; frozen TypeScript constants stay intact.
    let response_mode = TargetCreateRequestResponseMode::REALTIME;
    let sent = json!({
        "uuid":"example-target", "tsg_id":"example-tenant", "name":"example",
        "status":"ACTIVE", "active":true, "validated":false,
        "created_at":"2026-01-01", "updated_at":"2026-01-02",
        "connection_type":"VERTEX_AI", "response_mode":response_mode,
        "connection_params":{
            "api_endpoint":"wss://target.example.invalid/ws",
            "target_connection_config":provider, "multi_turn_config":session,
            "audio_request_json":{"audio":"{AUDIO}"}, "audio_curl":"example audio template"
        },
        "auth_config":{"basic_username":"example-user","basic_password":"example-only"}
    });
    for mode in [Mode::Response, Mode::ResponseWithoutDefaults] {
        let target = TargetResponse::parse(sent.clone(), mode)?;
        let actual = serde_json::to_value(target)?;
        assert_eq!(actual["response_mode"], "REALTIME");
        assert_eq!(
            actual["connection_params"]["multi_turn_config"]["type"],
            "client_session"
        );
        if mode == Mode::ResponseWithoutDefaults {
            assert_eq!(actual, sent);
        } else {
            assert_eq!(actual["connection_params"]["ws_response_timeout"], 110);
            assert_eq!(actual["auth_config"]["basic_auth_location"], "HEADER");
        }
        // Four optional projections provide typed access without losing raw values.
        let connection = RestConnectionParams::parse(sent["connection_params"].clone(), mode)?;
        assert!(matches!(connection.audio_request_json, Patch::Value(_)));
        assert!(matches!(connection.audio_curl, Patch::Value(_)));
        let auth = BasicAuthAuthConfig::parse(sent["auth_config"].clone(), mode)?;
        assert!(matches!(auth.basic_username, Patch::Value(_)));
        assert!(matches!(auth.basic_password, Patch::Value(_)));
        let raw = json!({"audio_request_json":17,"audio_curl":["future-shape"]});
        let fallback = RestConnectionParams::parse(raw.clone(), mode)?;
        assert!(matches!(fallback.audio_request_json, Patch::Unset));
        assert!(matches!(fallback.audio_curl, Patch::Unset));
        assert_eq!(serde_json::to_value(fallback)?, raw);
        let raw_auth = json!({"basic_username":17,"basic_password":["future-shape"]});
        let fallback_auth = BasicAuthAuthConfig::parse(raw_auth.clone(), mode)?;
        assert!(matches!(fallback_auth.basic_username, Patch::Unset));
        assert!(matches!(fallback_auth.basic_password, Patch::Unset));
        let preserved_auth = serde_json::to_value(fallback_auth)?;
        assert_eq!(preserved_auth["basic_username"], raw_auth["basic_username"]);
        assert_eq!(preserved_auth["basic_password"], raw_auth["basic_password"]);
    }
    // Pass these options to targets.list/list_all: the filter is kept on each page.
    // INIT, QUEUED and PARTIALLY_COMPLETE remain response values, not filter values.
    let mut listing = TargetListOptions::default();
    listing.profiling_status = Some("COMPLETED".into());
    listing.limit = Some(25);
    let request =
        RequestOptions::with_timeout(Duration::from_secs(10))?.without_response_defaults();
    assert_eq!(listing.profiling_status.as_deref(), Some("COMPLETED"));
    assert!(!request.response_defaults());
    println!(
        "Management models, raw fallback, filters and response default policies verified offline."
    );
    Ok(())
}
