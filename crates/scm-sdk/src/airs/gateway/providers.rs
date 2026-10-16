//! Native SCM Gateway resource contracts.
use super::{AIGatewayWorkspaceScopedListOptions, common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;
#[derive(Clone, Debug)]
pub struct AIGatewayProvidersClient {
    pub(super) client: Client,
}
impl AIGatewayProvidersClient {
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
    ) -> Result<ListProvidersResponse> {
        common::uuid(&params.workspace_id)?;
        self.client
            .send_contract(
                RawRequest::read("/providers")
                    .query(vec![("workspace_id".into(), params.workspace_id.clone())])
                    .named("airs.gateway.providers.list"),
                "ListProvidersResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        provider_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayProviderDetail> {
        common::uuid(provider_id)?;
        self.client
            .send_contract(
                RawRequest::read("/providers")
                    .segment(provider_id)?
                    .named("airs.gateway.providers.get"),
                "GatewayProviderDetailSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayProviderCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayProviderCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/providers").named("airs.gateway.providers.create"),
                body,
                "GatewayProviderCreateRequestSchema",
                "GatewayProviderCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        provider_id: &str,
        body: &GatewayProviderUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::uuid(provider_id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/providers")
                    .segment(provider_id)?
                    .named("airs.gateway.providers.update"),
                body,
                "GatewayProviderUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(&self, provider_id: &str, options: Option<&RequestOptions>) -> Result<()> {
        common::uuid(provider_id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/providers")
                    .segment(provider_id)?
                    .named("airs.gateway.providers.delete"),
                options,
            )
            .await
    }
}
