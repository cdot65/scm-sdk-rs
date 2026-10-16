use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;

#[derive(Clone, Debug)]
pub struct AIGatewayAdminGuardrailsClient {
    pub(super) client: Client,
}
impl AIGatewayAdminGuardrailsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub async fn list(
        &self,
        query: &GatewayAdminGuardrailListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayAdminGuardrailListResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayAdminGuardrailListOptionsSchema",
            &["workspaceId", "pageSize", "currentPage"],
            &options,
        )
        .await?;
        let query = query.into_iter().map(|(key, value)| {
            (
                match key.as_str() {
                    "workspaceId" => "workspace_id",
                    "pageSize" => "page_size",
                    _ => "current_page",
                }
                .into(),
                value,
            )
        });
        self.client
            .send_contract(
                RawRequest::read("/guardrails")
                    .named("airs.gateway.admin_guardrails.list")
                    .query(query),
                "GatewayAdminGuardrailListResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayAdminGuardrailDetail> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/guardrails")
                    .segment(id)?
                    .named("airs.gateway.admin_guardrails.get"),
                "GatewayAdminGuardrailDetailSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayAdminGuardrailCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayAdminGuardrailCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/guardrails")
                    .named("airs.gateway.admin_guardrails.create"),
                body,
                "GatewayAdminGuardrailCreateRequestSchema",
                "GatewayAdminGuardrailCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        id: &str,
        body: &GatewayGuardrailUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayAdminGuardrailUpdateResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/guardrails")
                    .segment(id)?
                    .named("airs.gateway.admin_guardrails.update"),
                body,
                "GatewayGuardrailUpdateRequestSchema",
                "GatewayAdminGuardrailUpdateResponseSchema",
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
                    .named("airs.gateway.admin_guardrails.delete"),
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
                    .named("airs.gateway.admin_guardrails.get_mcp_servers"),
                "GatewayMcpServerMappingsResponseSchema",
                options,
            )
            .await
    }
    pub async fn sync_mcp_servers(
        &self,
        id: &str,
        body: &GatewayAdminGuardrailMcpSyncRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayBulkSyncMcpServerMappingsResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/guardrails")
                    .segment(id)?
                    .segment("mcp-servers")?
                    .named("airs.gateway.admin_guardrails.sync_mcp_servers"),
                body,
                "GatewayAdminGuardrailMcpSyncRequestSchema",
                "GatewayBulkSyncMcpServerMappingsResponseSchema",
                options,
            )
            .await
    }
    pub async fn upsert_mcp_server(
        &self,
        id: &str,
        mcp_server_id: &str,
        body: &GatewayAdminGuardrailMcpMappingRequest,
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
                    .named("airs.gateway.admin_guardrails.upsert_mcp_server"),
                body,
                "GatewayAdminGuardrailMcpMappingRequestSchema",
                "GatewayUpsertMcpServerMappingResponseSchema",
                options,
            )
            .await
    }
}
