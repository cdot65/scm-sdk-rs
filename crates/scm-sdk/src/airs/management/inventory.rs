use super::models::*;
use crate::{Client, Error, ErrorKind, RawRequest, RequestOptions, Result, engine::AuthKind};
use http::Method;
use serde_json::json;
#[derive(Clone, Copy, Debug, Default)]
#[non_exhaustive]
pub struct DeploymentProfileListOptions {
    pub unactivated: Option<bool>,
}
#[derive(Clone, Debug)]
pub struct DeploymentProfilesClient {
    pub(super) client: Client,
}
impl DeploymentProfilesClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self { client })
    }
    pub async fn list(
        &self,
        query: DeploymentProfileListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<DeploymentProfilesResponse> {
        let params = query
            .unactivated
            .map(|value| vec![("unactivated".into(), value.to_string())])
            .unwrap_or_default();
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/deploymentprofiles")
                    .named("airs.management.deployment_profiles.list")
                    .query(params),
                "DeploymentProfilesResponseSchema",
                options,
            )
            .await
    }
}
/// Legacy AIRS management inventory; canonical DLP resources use `crate::dlp`.
#[derive(Clone, Debug)]
pub struct DlpProfilesClient {
    pub(super) client: Client,
}
impl DlpProfilesClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self { client })
    }
    pub async fn list(&self, options: Option<&RequestOptions>) -> Result<DlpProfileListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/mgmt/dlpprofiles").named("airs.management.dlp_profiles.list"),
                "DlpProfileListResponseSchema",
                options,
            )
            .await
    }
}
/// Legacy query dialect. Empty hydrated responses are unavailable evidence, never proof of no activity.
#[derive(Clone)]
#[non_exhaustive]
pub struct ScanLogQueryOptions {
    pub time_interval: u64,
    pub time_unit: String,
    pub page_number: u64,
    pub page_size: u64,
    pub filter: String,
    pub page_token: Option<String>,
}
impl ScanLogQueryOptions {
    pub fn new(
        time_interval: u64,
        time_unit: impl Into<String>,
        page_number: u64,
        page_size: u64,
        filter: impl Into<String>,
    ) -> Self {
        Self {
            time_interval,
            time_unit: time_unit.into(),
            page_number,
            page_size,
            filter: filter.into(),
            page_token: None,
        }
    }
}
impl std::fmt::Debug for ScanLogQueryOptions {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ScanLogQueryOptions([REDACTED])")
    }
}
/// Legacy retrieval path retained for parity; prefer Dashboard session methods.
#[derive(Clone, Debug)]
pub struct ScanLogsClient {
    pub(super) client: Client,
}
impl ScanLogsClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        Ok(Self { client })
    }
    pub async fn query(
        &self,
        query: &ScanLogQueryOptions,
        options: Option<&RequestOptions>,
    ) -> Result<PaginatedScanResults> {
        if [query.time_interval, query.page_number, query.page_size]
            .iter()
            .any(|value| *value == 0 || *value > 9_007_199_254_740_991)
        {
            return Err(Error::new(ErrorKind::Validation));
        }
        let params = vec![
            ("time_interval".into(), query.time_interval.to_string()),
            ("time_unit".into(), query.time_unit.clone()),
            ("pageNumber".into(), query.page_number.to_string()),
            ("pageSize".into(), query.page_size.to_string()),
            ("filter".into(), query.filter.clone()),
        ];
        let mut request = RawRequest::new(Method::POST, "/v1/mgmt/scanlogs")
            .named("airs.management.scan_logs.query")
            .query(params);
        if let Some(token) = &query.page_token
            && !token.is_empty()
        {
            request = request.json_value(
                &json!({"page_token":token}),
                self.client.limits().max_request_bytes,
            )?;
        }
        self.client
            .send_contract(request, "PaginatedScanResultsSchema", options)
            .await
    }
}
