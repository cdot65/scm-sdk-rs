use super::{
    ModelSecurityCustomRulesClient, ModelSecurityGroupsClient, ModelSecurityModelsClient,
    ModelSecurityRulesClient, ModelSecurityScansClient, models::PyPIAuthResponse, oauth,
};
use crate::{Client, Endpoint, RawRequest, RequestOptions, Result};
/// All planes share one OAuth provider, session, limiter, transport and destination policy.
#[derive(Clone, Debug)]
pub struct ModelSecurityClient {
    data: Client,
    management: Client,
}
impl ModelSecurityClient {
    pub fn new(client: Client, data: Endpoint, management: Endpoint) -> Result<Self> {
        let client = oauth(client)?;
        Ok(Self {
            data: client.with_endpoint(data)?,
            management: client.with_endpoint(management)?,
        })
    }
    pub fn data_client(&self) -> &Client {
        &self.data
    }
    pub fn management_client(&self) -> &Client {
        &self.management
    }
    pub fn scans(&self) -> ModelSecurityScansClient {
        ModelSecurityScansClient {
            client: self.data.clone(),
        }
    }
    pub fn models(&self) -> ModelSecurityModelsClient {
        ModelSecurityModelsClient {
            client: self.data.clone(),
        }
    }
    pub fn security_groups(&self) -> ModelSecurityGroupsClient {
        ModelSecurityGroupsClient {
            client: self.management.clone(),
        }
    }
    pub fn security_rules(&self) -> ModelSecurityRulesClient {
        ModelSecurityRulesClient {
            client: self.management.clone(),
        }
    }
    pub fn custom_rules(&self) -> ModelSecurityCustomRulesClient {
        ModelSecurityCustomRulesClient {
            client: self.management.clone(),
        }
    }
    /// Return explicit PyPI credentials without following the credential-bearing URL.
    pub async fn get_pypi_auth(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<PyPIAuthResponse> {
        self.management
            .send_contract(
                RawRequest::read("/v1/pypi/authenticate")
                    .named("airs.model_security.get_pypi_auth"),
                "PyPIAuthResponseSchema",
                options,
            )
            .await
    }
}
