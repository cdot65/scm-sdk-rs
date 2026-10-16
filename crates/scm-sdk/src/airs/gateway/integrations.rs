use super::{common, models::*};
use crate::{Client, Error, ErrorKind, RawRequest, RequestOptions, Result};
use http::Method;

#[derive(Clone, Debug)]
pub struct AIGatewayIntegrationsClient {
    pub(super) client: Client,
}
impl AIGatewayIntegrationsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub async fn list(&self, options: Option<&RequestOptions>) -> Result<ListIntegrationsResponse> {
        self.client
            .send_contract(
                RawRequest::read("/integrations").named("airs.gateway.integrations.list"),
                "ListIntegrationsResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayIntegration> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/integrations")
                    .segment(id)?
                    .named("airs.gateway.integrations.get"),
                "GatewayIntegrationSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayIntegrationCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/integrations")
                    .named("airs.gateway.integrations.create"),
                body,
                "GatewayIntegrationCreateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        id: &str,
        body: &GatewayIntegrationUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/integrations")
                    .segment(id)?
                    .named("airs.gateway.integrations.update"),
                body,
                "GatewayIntegrationUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        id: &str,
        organisation_id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<()> {
        common::uuid(id)?;
        common::numeric_id(organisation_id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/integrations")
                    .segment(id)?
                    .named("airs.gateway.integrations.delete")
                    .query([("organisation_id".into(), organisation_id.into())]),
                options,
            )
            .await
    }
    pub async fn catalog(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<ListCatalogProvidersResponse> {
        self.client
            .send_contract(
                RawRequest::read("/utils/static-resources/ai-providers")
                    .named("airs.gateway.integrations.catalog"),
                "ListCatalogProvidersResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_models(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayIntegrationModelsResponse> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/integrations")
                    .segment(id)?
                    .segment("models")?
                    .named("airs.gateway.integrations.get_models"),
                "GatewayIntegrationModelsResponseSchema",
                options,
            )
            .await
    }
    pub async fn set_models(
        &self,
        id: &str,
        body: &GatewayIntegrationModelsBulkUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/integrations")
                    .segment(id)?
                    .segment("models")?
                    .named("airs.gateway.integrations.set_models"),
                body,
                "GatewayIntegrationModelsBulkUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_workspaces(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayIntegrationWorkspacesResponse> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/integrations")
                    .segment(id)?
                    .segment("workspaces")?
                    .named("airs.gateway.integrations.get_workspaces"),
                "GatewayIntegrationWorkspacesResponseSchema",
                options,
            )
            .await
    }
    pub async fn set_workspaces(
        &self,
        id: &str,
        body: &GatewayIntegrationWorkspacesBulkUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/integrations")
                    .segment(id)?
                    .segment("workspaces")?
                    .named("airs.gateway.integrations.set_workspaces"),
                body,
                "GatewayIntegrationWorkspacesBulkUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    /// UUID references resolve locally; slugs match the first catalog row case-insensitively.
    pub async fn resolve_provider_id(
        &self,
        reference: &str,
        options: Option<&RequestOptions>,
    ) -> Result<String> {
        if reference.len() > crate::config::MAX_URL_BYTES {
            return Err(Error::new(ErrorKind::Limit));
        }
        let wanted = common::js_trim(reference);
        if common::uuid(wanted).is_ok() {
            return Ok(wanted.to_ascii_lowercase());
        }
        if wanted.is_empty() {
            return Err(Error::new(ErrorKind::Validation));
        }
        let options = self.client.operation_options(options)?;
        let providers = self.catalog(Some(&options)).await?.data;
        options
            .run(&self.client.session(), async {
                let wanted = wanted.to_lowercase();
                providers
                    .into_iter()
                    .find(|provider| provider.slug.to_lowercase() == wanted)
                    .map(|provider| provider.id)
                    .ok_or_else(|| Error::new(ErrorKind::Validation))
            })
            .await
    }
    /// Delete only explicitly named custom models, preserving the comma query dialect.
    pub async fn delete_models(
        &self,
        id: &str,
        slugs: &[String],
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::workspace_ref(id)?;
        if slugs.is_empty() {
            return Err(Error::new(ErrorKind::Validation));
        }
        let mut units = 0usize;
        let mut bytes = 0usize;
        for (index, slug) in slugs.iter().enumerate() {
            let length = slug.encode_utf16().take(513).count();
            if !(1..=512).contains(&length) || slug.contains(',') {
                return Err(Error::new(ErrorKind::Validation));
            }
            let comma = usize::from(index > 0);
            units = units
                .checked_add(length + comma)
                .filter(|n| *n <= 16_384)
                .ok_or_else(|| Error::new(ErrorKind::Validation))?;
            bytes = bytes
                .checked_add(slug.len() + comma)
                .filter(|n| *n <= crate::config::MAX_URL_BYTES)
                .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        }
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/integrations")
                    .segment(id)?
                    .segment("models")?
                    .named("airs.gateway.integrations.delete_models")
                    .query([("slugs".into(), slugs.join(","))]),
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
}
