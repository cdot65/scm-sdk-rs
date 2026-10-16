//! Native SCM Gateway resource contracts.
use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;
#[derive(Clone, Debug)]
pub struct AIGatewayLogExportsClient {
    pub(super) client: Client,
}
impl AIGatewayLogExportsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub async fn list(
        &self,
        params: &GatewayLogExportsClientListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayLogExportsClientListResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            params,
            "GatewayLogExportsClientListOptionsSchema",
            &["workspace_id"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/logs/exports")
                    .query(query)
                    .named("airs.gateway.log_exports.list"),
                "GatewayLogExportsClientListResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayLogExportsClientCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayLogExportsClientCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/logs/exports")
                    .named("airs.gateway.log_exports.create"),
                body,
                "GatewayLogExportsClientCreateRequestSchema",
                "GatewayLogExportsClientCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        export_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayLogExportsClientGetResponse> {
        common::resource_id(export_id)?;
        self.client
            .send_contract(
                RawRequest::read("/logs/exports")
                    .segment(export_id)?
                    .named("airs.gateway.log_exports.get"),
                "GatewayLogExportsClientGetResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        export_id: &str,
        body: &GatewayLogExportsClientUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayLogExportsClientUpdateResponse> {
        common::resource_id(export_id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/logs/exports")
                    .segment(export_id)?
                    .named("airs.gateway.log_exports.update"),
                body,
                "GatewayLogExportsClientUpdateRequestSchema",
                "GatewayLogExportsClientUpdateResponseSchema",
                options,
            )
            .await
    }
    /// Experimental: the frozen source did not establish successful live execution.
    pub async fn start(
        &self,
        export_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayLogExportsClientStartResponse> {
        common::resource_id(export_id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::POST, "/logs/exports")
                    .segment(export_id)?
                    .segment("start")?
                    .named("airs.gateway.log_exports.start"),
                "GatewayLogExportsClientStartResponseSchema",
                options,
            )
            .await
    }
    /// Experimental: requires a running owned export; no success is inferred locally.
    pub async fn cancel(
        &self,
        export_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayLogExportsClientCancelResponse> {
        common::resource_id(export_id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::POST, "/logs/exports")
                    .segment(export_id)?
                    .segment("cancel")?
                    .named("airs.gateway.log_exports.cancel"),
                "GatewayLogExportsClientCancelResponseSchema",
                options,
            )
            .await
    }
    /// Experimental. Returns a signed URL descriptor without fetching that URL.
    pub async fn download(
        &self,
        export_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayLogExportsClientDownloadResponse> {
        common::resource_id(export_id)?;
        self.client
            .send_contract(
                RawRequest::read("/logs/exports")
                    .segment(export_id)?
                    .segment("download")?
                    .named("airs.gateway.log_exports.download"),
                "GatewayLogExportsClientDownloadResponseSchema",
                options,
            )
            .await
    }
}
