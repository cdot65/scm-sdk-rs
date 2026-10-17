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
    /// Scope one OAuth client to explicitly admitted data and management endpoints.
    ///
    /// Enable `model-security`. Both endpoints must already be allowed by the
    /// client's destination policy; transport, session and OAuth provider are shared.
    ///
    /// ```no_run
    /// use scm_sdk::{Client, Endpoint, Result,
    ///     airs::model_security::{ModelSecurityClient, ModelSecurityScanListOptions}};
    /// async fn scan_page(oauth: Client, data: Endpoint, management: Endpoint) -> Result<()> {
    ///     let security = ModelSecurityClient::new(oauth, data, management)?;
    ///     let query = ModelSecurityScanListOptions::default();
    ///     let _page = security.scans().list(&query, None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client, data: Endpoint, management: Endpoint) -> Result<Self> {
        let client = oauth(client)?;
        Ok(Self {
            data: client.with_endpoint(data)?,
            management: client.with_endpoint(management)?,
        })
    }
    /// Borrow the client scoped to model scan and inventory reads.
    pub fn data_client(&self) -> &Client {
        &self.data
    }
    /// Borrow the client scoped to rules and security group management.
    pub fn management_client(&self) -> &Client {
        &self.management
    }
    /// Access model scans and bounded result pagination.
    pub fn scans(&self) -> ModelSecurityScansClient {
        ModelSecurityScansClient {
            client: self.data.clone(),
        }
    }
    /// Access scanned models, versions and file inventory.
    pub fn models(&self) -> ModelSecurityModelsClient {
        ModelSecurityModelsClient {
            client: self.data.clone(),
        }
    }
    /// Access security groups and their rule instances.
    pub fn security_groups(&self) -> ModelSecurityGroupsClient {
        ModelSecurityGroupsClient {
            client: self.management.clone(),
        }
    }
    /// Access security rule definitions and versions.
    pub fn security_rules(&self) -> ModelSecurityRulesClient {
        ModelSecurityRulesClient {
            client: self.management.clone(),
        }
    }
    /// Access custom rules and their version lifecycle.
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
