//! Native SCM Gateway resource contracts.
use super::{AIGatewayWorkspaceScopedListOptions, common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;
#[derive(Clone, Debug)]
pub struct AIGatewayConfigsClient {
    pub(super) client: Client,
}
impl AIGatewayConfigsClient {
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
        params: &AIGatewayWorkspaceScopedListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ListConfigsResponse> {
        common::uuid(&params.workspace_id)?;
        self.client
            .send_contract(
                RawRequest::read("/configs")
                    .query(vec![("workspace_id".into(), params.workspace_id.clone())])
                    .named("airs.gateway.configs.list"),
                "ListConfigsResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        config_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayConfigDetail> {
        common::uuid(config_id)?;
        self.client
            .send_contract(
                RawRequest::read("/configs")
                    .segment(config_id)?
                    .named("airs.gateway.configs.get"),
                "GatewayConfigDetailSchema",
                options,
            )
            .await
    }
    pub async fn list_versions(
        &self,
        config_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<ListConfigVersionsResponse> {
        common::uuid(config_id)?;
        self.client
            .send_contract(
                RawRequest::read("/configs")
                    .segment(config_id)?
                    .segment("versions")?
                    .named("airs.gateway.configs.list_versions"),
                "ListConfigVersionsResponseSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayConfigCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayConfigCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/configs").named("airs.gateway.configs.create"),
                body,
                "GatewayConfigCreateRequestSchema",
                "GatewayConfigCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        config_id: &str,
        body: &GatewayConfigUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::uuid(config_id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/configs")
                    .segment(config_id)?
                    .named("airs.gateway.configs.update"),
                body,
                "GatewayConfigUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(&self, config_id: &str, options: Option<&RequestOptions>) -> Result<()> {
        common::uuid(config_id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/configs")
                    .segment(config_id)?
                    .named("airs.gateway.configs.delete"),
                options,
            )
            .await
    }
}
