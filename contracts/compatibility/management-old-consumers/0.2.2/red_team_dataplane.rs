//! Current dataplane types, explicit request options, and application-owned async calls.
//! The main function is offline. Integrate the helpers into your configured application.
#[cfg(feature = "red-team")]
use scm_sdk::{
    RequestOptions, Result,
    airs::red_team::{
        RedTeamClient,
        models::{FileFormat, RegenerateResponse, ThreatOverrideItem, ThreatOverrideRequest},
    },
    data::Nullable,
};

/// Read a signed location as opaque API data. No storage request follows.
#[cfg(feature = "red-team")]
pub async fn inspect_report(client: &RedTeamClient, job_id: &str) -> Result<()> {
    let options = RequestOptions::with_timeout(std::time::Duration::from_secs(10))?
        .without_response_defaults();
    let status = client.reports().get_status(job_id, Some(&options)).await?;
    let descriptor = client
        .reports()
        .get_download_url(job_id, FileFormat::CSV, Some(&options))
        .await?;
    let csv = client.download_error_logs(job_id, Some(&options)).await?;
    // Application policy chooses how to use the status and where to store data.
    // Ordinary SDK model formatting redacts signed locations and response data.
    let _ = (status, descriptor, csv);
    Ok(())
}

/// Queue once and return the background job; polling and reconciliation belong to the caller.
#[cfg(feature = "red-team")]
pub async fn regenerate_report(client: &RedTeamClient, job_id: &str) -> Result<RegenerateResponse> {
    let options = RequestOptions::with_timeout(std::time::Duration::from_secs(10))?;
    client.reports().regenerate(job_id, Some(&options)).await
}

#[cfg(feature = "red-team")]
fn main() -> Result<()> {
    // Explicit null clears a verdict; true/false set it. Omission is invalid.
    let edit = ThreatOverrideItem::builder()
        .output_id("550e8400-e29b-41d4-a716-446655440011".into())
        .threat_override(Nullable::Null)
        .build()?;
    let request = ThreatOverrideRequest::builder()
        .overrides(vec![edit])
        .build_validated(scm_sdk::validation::Mode::Request)?;
    assert_eq!(request.overrides.len(), 1);
    Ok(())
}
#[cfg(not(feature = "red-team"))]
fn main() {}
