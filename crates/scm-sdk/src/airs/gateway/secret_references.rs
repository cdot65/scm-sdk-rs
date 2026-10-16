use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;

/// Explicit OAuth secret references operations; response formatting is redacted.
#[derive(Clone, Debug)]
pub struct AIGatewaySecretReferencesClient {
    pub(super) client: Client,
}
impl AIGatewaySecretReferencesClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub async fn list(
        &self,
        query: &GatewaySecretReferencesClientListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewaySecretReferencesClientListResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewaySecretReferencesClientListOptionsSchema",
            &[
                "manager_type",
                "tags",
                "search",
                "current_page",
                "page_size",
            ],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/secret-references")
                    .named("airs.gateway.secret_references.list")
                    .query(query),
                "GatewaySecretReferencesClientListResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewaySecretReferencesClientCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewaySecretReferencesClientCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/secret-references")
                    .named("airs.gateway.secret_references.create"),
                body,
                "GatewaySecretReferencesClientCreateRequestSchema",
                "GatewaySecretReferencesClientCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewaySecretReferencesClientGetResponse> {
        common::resource_id(id)?;
        self.client
            .send_contract(
                RawRequest::read("/secret-references")
                    .segment(id)?
                    .named("airs.gateway.secret_references.get"),
                "GatewaySecretReferencesClientGetResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        id: &str,
        body: &GatewaySecretReferencesClientUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewaySecretReferencesClientUpdateResponse> {
        common::resource_id(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/secret-references")
                    .segment(id)?
                    .named("airs.gateway.secret_references.update"),
                body,
                "GatewaySecretReferencesClientUpdateRequestSchema",
                "GatewaySecretReferencesClientUpdateResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewaySecretReferencesClientDeleteResponse> {
        common::resource_id(id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/secret-references")
                    .segment(id)?
                    .named("airs.gateway.secret_references.delete"),
                "GatewaySecretReferencesClientDeleteResponseSchema",
                options,
            )
            .await
    }
}
