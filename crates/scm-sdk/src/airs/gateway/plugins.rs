use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;

/// Explicit OAuth plugins operations; response formatting is redacted.
#[derive(Clone, Debug)]
pub struct AIGatewayPluginsClient {
    pub(super) client: Client,
}
impl AIGatewayPluginsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub async fn list(&self, options: Option<&RequestOptions>) -> Result<ListPluginsResponse> {
        self.client
            .send_contract(
                RawRequest::read("/plugins").named("airs.gateway.plugins.list"),
                "ListPluginsResponseSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayPluginCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/plugins").named("airs.gateway.plugins.create"),
                body,
                "GatewayPluginCreateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
}
