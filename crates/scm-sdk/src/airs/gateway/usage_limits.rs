use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;

/// Explicit OAuth usage limits operations; response formatting is redacted.
#[derive(Clone, Debug)]
pub struct AIGatewayUsageLimitsClient {
    pub(super) client: Client,
}
impl AIGatewayUsageLimitsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub async fn list(
        &self,
        query: &GatewayUsageLimitsClientListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayUsageLimitsClientListResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayUsageLimitsClientListOptionsSchema",
            &[
                "workspace_id",
                "status",
                "type",
                "page_size",
                "current_page",
            ],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/policies/usage-limits")
                    .named("airs.gateway.usage_limits.list")
                    .query(query),
                "GatewayUsageLimitsClientListResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayUsageLimitsClientCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayUsageLimitsClientCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/policies/usage-limits")
                    .named("airs.gateway.usage_limits.create"),
                body,
                "GatewayUsageLimitsClientCreateRequestSchema",
                "GatewayUsageLimitsClientCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        query: &GatewayUsageLimitsClientGetOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayUsageLimitsClientGetResponse> {
        common::resource_id(id)?;
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayUsageLimitsClientGetOptionsSchema",
            &["status", "include_usage"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/policies/usage-limits")
                    .segment(id)?
                    .named("airs.gateway.usage_limits.get")
                    .query(query),
                "GatewayUsageLimitsClientGetResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn update(
        &self,
        id: &str,
        body: &GatewayUsageLimitsClientUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayUsageLimitsClientUpdateResponse> {
        common::resource_id(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/policies/usage-limits")
                    .segment(id)?
                    .named("airs.gateway.usage_limits.update"),
                body,
                "GatewayUsageLimitsClientUpdateRequestSchema",
                "GatewayUsageLimitsClientUpdateResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayUsageLimitsClientDeleteResponse> {
        common::resource_id(id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/policies/usage-limits")
                    .segment(id)?
                    .named("airs.gateway.usage_limits.delete"),
                "GatewayUsageLimitsClientDeleteResponseSchema",
                options,
            )
            .await
    }
    pub async fn list_entities(
        &self,
        id: &str,
        query: &GatewayUsageLimitsClientListEntitiesOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayUsageLimitsClientListEntitiesResponse> {
        common::resource_id(id)?;
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayUsageLimitsClientListEntitiesOptionsSchema",
            &["status", "search", "page_size", "current_page"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/policies/usage-limits")
                    .segment(id)?
                    .segment("entities")?
                    .named("airs.gateway.usage_limits.list_entities")
                    .query(query),
                "GatewayUsageLimitsClientListEntitiesResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn reset_entity(
        &self,
        id: &str,
        entity_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayUsageLimitsClientResetEntityResponse> {
        common::resource_id(id)?;
        common::resource_id(entity_id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::PUT, "/policies/usage-limits")
                    .segment(id)?
                    .segment("entities")?
                    .segment(entity_id)?
                    .segment("reset")?
                    .named("airs.gateway.usage_limits.reset_entity"),
                "GatewayUsageLimitsClientResetEntityResponseSchema",
                options,
            )
            .await
    }
}
