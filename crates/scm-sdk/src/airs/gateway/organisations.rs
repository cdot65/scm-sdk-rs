//! Native SCM Gateway resource contracts.
use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;
#[derive(Clone, Debug)]
pub struct AIGatewayOrganisationsClient {
    pub(super) client: Client,
}
impl AIGatewayOrganisationsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub async fn get_info(
        &self,
        tsg_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayOrganisationInfo> {
        common::numeric_id(tsg_id)?;
        self.client
            .send_contract(
                RawRequest::read("/organisations")
                    .segment(tsg_id)?
                    .segment("info")?
                    .named("airs.gateway.organisations.get_info"),
                "GatewayOrganisationInfoSchema",
                options,
            )
            .await
    }
    pub async fn get_self(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<OrganisationSelfResponse> {
        self.client
            .send_contract(
                RawRequest::read("/organisations/self")
                    .named("airs.gateway.organisations.get_self"),
                "OrganisationSelfResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_self(
        &self,
        body: &GatewayOrganisationUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/organisations/self")
                    .named("airs.gateway.organisations.update_self"),
                body,
                "GatewayOrganisationUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_auth_settings(
        &self,
        tsg_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<AuthSettingsResponse> {
        common::numeric_id(tsg_id)?;
        self.client
            .send_contract(
                RawRequest::read("/organisations")
                    .segment(tsg_id)?
                    .segment("auth-settings")?
                    .named("airs.gateway.organisations.get_auth_settings"),
                "AuthSettingsResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_auth_settings(
        &self,
        tsg_id: &str,
        body: &GatewayOrganisationAuthSettingsUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::numeric_id(tsg_id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/organisations")
                    .segment(tsg_id)?
                    .segment("auth-settings")?
                    .named("airs.gateway.organisations.update_auth_settings"),
                body,
                "GatewayOrganisationAuthSettingsUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
}
