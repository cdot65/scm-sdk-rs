//! Explicit EULA reads and acceptance. Construction never accepts an agreement.
use super::models::{EulaAcceptRequest, EulaContentResponse, EulaResponse};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;

#[derive(Clone, Debug)]
pub struct RedTeamEulaClient {
    pub(super) client: Client,
}
impl RedTeamEulaClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(crate::engine::AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(false)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    pub async fn get_content(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<EulaContentResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/eula/content").named("red_team.eula.get_content"),
                "EulaContentResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_status(&self, options: Option<&RequestOptions>) -> Result<EulaResponse> {
        self.client
            .send_contract(
                RawRequest::read("/v1/eula/status").named("red_team.eula.get_status"),
                "EulaResponseSchema",
                options,
            )
            .await
    }
    pub async fn accept(
        &self,
        body: &EulaAcceptRequest,
        options: Option<&RequestOptions>,
    ) -> Result<EulaResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/v1/eula/accept").named("red_team.eula.accept"),
                body,
                "EulaAcceptRequestSchema",
                "EulaResponseSchema",
                options,
            )
            .await
    }
}
