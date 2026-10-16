//! Native SCM Gateway resource contracts.
use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;
#[derive(Clone, Debug)]
pub struct AIGatewayRateLimitsClient {
    pub(super) client: Client,
}
impl AIGatewayRateLimitsClient {
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
        params: &GatewayRateLimitsClientListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayRateLimitsClientListResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            params,
            "GatewayRateLimitsClientListOptionsSchema",
            &[
                "workspace_id",
                "status",
                "type",
                "unit",
                "target",
                "page_size",
                "current_page",
            ],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/policies/rate-limits")
                    .query(query)
                    .named("airs.gateway.rate_limits.list"),
                "GatewayRateLimitsClientListResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayRateLimitsClientCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayRateLimitsClientCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/policies/rate-limits")
                    .named("airs.gateway.rate_limits.create"),
                body,
                "GatewayRateLimitsClientCreateRequestSchema",
                "GatewayRateLimitsClientCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        policy_id: &str,
        params: &GatewayRateLimitsClientGetOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayRateLimitsClientGetResponse> {
        common::resource_id(policy_id)?;
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            params,
            "GatewayRateLimitsClientGetOptionsSchema",
            &["status"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/policies/rate-limits")
                    .segment(policy_id)?
                    .query(query)
                    .named("airs.gateway.rate_limits.get"),
                "GatewayRateLimitsClientGetResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn update(
        &self,
        policy_id: &str,
        body: &GatewayRateLimitsClientUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayRateLimitsClientUpdateResponse> {
        common::resource_id(policy_id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/policies/rate-limits")
                    .segment(policy_id)?
                    .named("airs.gateway.rate_limits.update"),
                body,
                "GatewayRateLimitsClientUpdateRequestSchema",
                "GatewayRateLimitsClientUpdateResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        policy_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayRateLimitsClientDeleteResponse> {
        common::resource_id(policy_id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/policies/rate-limits")
                    .segment(policy_id)?
                    .named("airs.gateway.rate_limits.delete"),
                "GatewayRateLimitsClientDeleteResponseSchema",
                options,
            )
            .await
    }
}
