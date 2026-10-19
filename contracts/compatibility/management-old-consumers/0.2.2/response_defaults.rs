//! Offline demonstration; no credentials, runtime or service calls.
use scm_sdk::{RequestOptions, airs::red_team::models::TargetResponse, validation::Mode};
use serde_json::json;
use std::time::Duration;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let options =
        RequestOptions::with_timeout(Duration::from_secs(10))?.without_response_defaults();
    assert!(!options.response_defaults());
    // Pass Some(&options) to typed target operations. Direct model validation
    // uses the equivalent Mode below for this offline example.
    let sent = json!({
        "uuid":"00000000-0000-4000-8000-000000000001",
        "tsg_id":"example-tenant", "name":"example", "status":"FUTURE",
        "active":false, "validated":false,
        "created_at":"2026-01-01", "updated_at":"2026-01-02",
        "description":null,
        "connection_params":{"api_endpoint":"wss://target.example.test/ws","request_headers":null},
        "auth_config":{"basic_auth_header":null}
    });
    let target = TargetResponse::parse(sent.clone(), Mode::ResponseWithoutDefaults)?;
    assert_eq!(serde_json::to_value(target)?, sent);
    let parity = TargetResponse::parse(sent, Mode::Response)?;
    let parity = serde_json::to_value(parity)?;
    assert_eq!(parity["connection_params"]["ws_response_timeout"], 110);
    assert_eq!(parity["auth_config"]["basic_auth_location"], "HEADER");
    println!("Omitted fields and explicit nulls round-trip; default policy is unchanged.");
    Ok(())
}
