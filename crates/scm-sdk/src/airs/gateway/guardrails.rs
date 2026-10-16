use super::{AIGatewayWorkspaceScopedListOptions, common, models::*};
use crate::{Client, Endpoint, Error, ErrorKind, RawRequest, RequestOptions, Result};
use http::Method;

#[derive(Clone, Debug)]
pub struct AIGatewayGuardrailsClient {
    pub(super) client: Client,
    pub(super) admin: Option<Client>,
}
impl AIGatewayGuardrailsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
            admin: None,
        })
    }
    /// Supply the explicitly admitted admin endpoint for the capability catalog.
    pub fn with_admin_endpoint(mut self, endpoint: Endpoint) -> Result<Self> {
        self.admin = Some(self.client.with_endpoint(endpoint)?);
        Ok(self)
    }
    pub async fn get_catalog(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayGuardrailCatalogResponse> {
        self.admin
            .as_ref()
            .ok_or_else(|| Error::new(ErrorKind::Configuration))?
            .send_contract(
                RawRequest::read("/utils/static-resources/schema")
                    .named("airs.gateway.guardrails.get_catalog")
                    .query([("resource".into(), "guardrails".into())]),
                "GatewayGuardrailCatalogResponseSchema",
                options,
            )
            .await
    }
    pub async fn list(
        &self,
        query: &AIGatewayWorkspaceScopedListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ListGuardrailsResponse> {
        common::uuid(&query.workspace_id)?;
        let query = [("workspace_id".into(), query.workspace_id.clone())];
        self.client
            .send_contract(
                RawRequest::read("/guardrails")
                    .named("airs.gateway.guardrails.list")
                    .query(query),
                "ListGuardrailsResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayGuardrailDetail> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/guardrails")
                    .segment(id)?
                    .named("airs.gateway.guardrails.get"),
                "GatewayGuardrailDetailSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayGuardrailCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayGuardrailCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/guardrails")
                    .named("airs.gateway.guardrails.create"),
                body,
                "GatewayGuardrailCreateRequestSchema",
                "GatewayGuardrailCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        id: &str,
        body: &GatewayGuardrailUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/guardrails")
                    .segment(id)?
                    .named("airs.gateway.guardrails.update"),
                body,
                "GatewayGuardrailUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(&self, id: &str, options: Option<&RequestOptions>) -> Result<()> {
        common::uuid(id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/guardrails")
                    .segment(id)?
                    .named("airs.gateway.guardrails.delete"),
                options,
            )
            .await
    }
    pub async fn get_mcp_servers(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Vec<GatewayMcpServerMapping>> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/guardrails")
                    .segment(id)?
                    .segment("mcp-servers")?
                    .named("airs.gateway.guardrails.get_mcp_servers"),
                "GatewayMcpServerMappingsResponseSchema",
                options,
            )
            .await
    }
    pub async fn sync_mcp_servers(
        &self,
        id: &str,
        body: &GatewayBulkSyncMcpServerMappingsRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayBulkSyncMcpServerMappingsResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/guardrails")
                    .segment(id)?
                    .segment("mcp-servers")?
                    .named("airs.gateway.guardrails.sync_mcp_servers"),
                body,
                "GatewayBulkSyncMcpServerMappingsRequestSchema",
                "GatewayBulkSyncMcpServerMappingsResponseSchema",
                options,
            )
            .await
    }
    pub async fn upsert_mcp_server(
        &self,
        id: &str,
        mcp_server_id: &str,
        body: &GatewayUpsertMcpServerMappingRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayUpsertMcpServerMappingResponse> {
        common::uuid(id)?;
        common::uuid(mcp_server_id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/guardrails")
                    .segment(id)?
                    .segment("mcp-servers")?
                    .segment(mcp_server_id)?
                    .named("airs.gateway.guardrails.upsert_mcp_server"),
                body,
                "GatewayUpsertMcpServerMappingRequestSchema",
                "GatewayUpsertMcpServerMappingResponseSchema",
                options,
            )
            .await
    }
}
