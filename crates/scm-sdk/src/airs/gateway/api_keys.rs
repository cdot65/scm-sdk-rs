use super::{AIGatewayWorkspaceScopedListOptions, common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;

/// Workspace service and user keys are distinct source routes; no route fallback.
#[derive(Clone, Debug)]
pub struct AIGatewayApiKeysClient {
    pub(super) client: Client,
}
impl AIGatewayApiKeysClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub async fn list_service(
        &self,
        query: &AIGatewayWorkspaceScopedListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ListApiKeysResponse> {
        common::uuid(&query.workspace_id)?;
        self.client
            .send_contract(
                RawRequest::read("/api-keys/service")
                    .named("airs.gateway.api_keys.list_service")
                    .query([("workspace_id".into(), query.workspace_id.clone())]),
                "ListApiKeysResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_service(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayApiKey> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/api-keys/service")
                    .segment(id)?
                    .named("airs.gateway.api_keys.get_service"),
                "GatewayApiKeySchema",
                options,
            )
            .await
    }
    pub async fn delete_service(&self, id: &str, options: Option<&RequestOptions>) -> Result<()> {
        common::uuid(id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/api-keys/service")
                    .segment(id)?
                    .named("airs.gateway.api_keys.delete_service"),
                options,
            )
            .await
    }
    pub async fn create_service(
        &self,
        body: &GatewayServiceApiKeyCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayApiKeyCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/api-keys/service")
                    .named("airs.gateway.api_keys.create_service"),
                body,
                "GatewayServiceApiKeyCreateRequestSchema",
                "GatewayApiKeyCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_service(
        &self,
        id: &str,
        body: &GatewayApiKeyUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/api-keys/service")
                    .segment(id)?
                    .named("airs.gateway.api_keys.update_service"),
                body,
                "GatewayApiKeyUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn rotate_service(
        &self,
        id: &str,
        body: Option<&GatewayApiKeyRotateRequest>,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayApiKeyRotateResponse> {
        common::uuid(id)?;
        let empty = GatewayApiKeyRotateRequest::builder().build()?;
        let body = body.unwrap_or(&empty);
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/api-keys/service")
                    .segment(id)?
                    .segment("rotate")?
                    .named("airs.gateway.api_keys.rotate_service"),
                body,
                "GatewayApiKeyRotateRequestSchema",
                "GatewayApiKeyRotateResponseSchema",
                options,
            )
            .await
    }
    pub async fn list_user(
        &self,
        query: &AIGatewayWorkspaceScopedListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ListApiKeysResponse> {
        common::uuid(&query.workspace_id)?;
        self.client
            .send_contract(
                RawRequest::read("/api-keys/user")
                    .named("airs.gateway.api_keys.list_user")
                    .query([("workspace_id".into(), query.workspace_id.clone())]),
                "ListApiKeysResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_user(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayApiKey> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/api-keys/user")
                    .segment(id)?
                    .named("airs.gateway.api_keys.get_user"),
                "GatewayApiKeySchema",
                options,
            )
            .await
    }
    pub async fn delete_user(&self, id: &str, options: Option<&RequestOptions>) -> Result<()> {
        common::uuid(id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/api-keys/user")
                    .segment(id)?
                    .named("airs.gateway.api_keys.delete_user"),
                options,
            )
            .await
    }
    pub async fn create_user(
        &self,
        body: &GatewayUserApiKeyCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayApiKeyCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/api-keys/user")
                    .named("airs.gateway.api_keys.create_user"),
                body,
                "GatewayUserApiKeyCreateRequestSchema",
                "GatewayApiKeyCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn update_user(
        &self,
        id: &str,
        body: &GatewayApiKeyUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/api-keys/user")
                    .segment(id)?
                    .named("airs.gateway.api_keys.update_user"),
                body,
                "GatewayApiKeyUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn rotate_user(
        &self,
        id: &str,
        body: Option<&GatewayApiKeyRotateRequest>,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayApiKeyRotateResponse> {
        common::uuid(id)?;
        let empty = GatewayApiKeyRotateRequest::builder().build()?;
        let body = body.unwrap_or(&empty);
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/api-keys/user")
                    .segment(id)?
                    .segment("rotate")?
                    .named("airs.gateway.api_keys.rotate_user"),
                body,
                "GatewayApiKeyRotateRequestSchema",
                "GatewayApiKeyRotateResponseSchema",
                options,
            )
            .await
    }
}
