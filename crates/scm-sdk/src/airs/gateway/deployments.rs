//! Native SCM Gateway resource contracts.
use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;
#[derive(Clone, Debug)]
pub struct AIGatewayDeploymentsClient {
    pub(super) client: Client,
}
impl AIGatewayDeploymentsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub async fn list(&self, options: Option<&RequestOptions>) -> Result<ListDeploymentsResponse> {
        self.client
            .send_contract(
                RawRequest::read("/deployments").named("airs.gateway.deployments.list"),
                "ListDeploymentsResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        deployment_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayDeploymentDetail> {
        common::uuid(deployment_id)?;
        self.client
            .send_contract(
                RawRequest::read("/deployments")
                    .segment(deployment_id)?
                    .named("airs.gateway.deployments.get"),
                "GatewayDeploymentDetailSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayDeploymentCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayDeploymentCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/deployments")
                    .named("airs.gateway.deployments.create"),
                body,
                "GatewayDeploymentCreateRequestSchema",
                "GatewayDeploymentCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        deployment_id: &str,
        body: &GatewayDeploymentUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::uuid(deployment_id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/deployments")
                    .segment(deployment_id)?
                    .named("airs.gateway.deployments.update"),
                body,
                "GatewayDeploymentUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn ping(
        &self,
        deployment_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayDeploymentPingResponse> {
        common::uuid(deployment_id)?;
        self.client
            .send_contract(
                RawRequest::read("/deployments")
                    .segment(deployment_id)?
                    .segment("ping")?
                    .named("airs.gateway.deployments.ping"),
                "GatewayDeploymentPingResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        deployment_id: &str,
        organisation_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<()> {
        common::uuid(deployment_id)?;
        common::numeric_id(organisation_id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/deployments")
                    .segment(deployment_id)?
                    .query(vec![("organisation_id".into(), organisation_id.into())])
                    .named("airs.gateway.deployments.delete"),
                options,
            )
            .await
    }
}
