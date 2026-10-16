use super::{common, models::*};
use crate::{Client, Error, ErrorKind, RawRequest, RequestOptions, Result};
use http::Method;

#[derive(Clone, Debug)]
pub struct AIGatewayMcpIntegrationsClient {
    pub(super) client: Client,
}
impl AIGatewayMcpIntegrationsClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub async fn list(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<ListMcpIntegrationsResponse> {
        self.client
            .send_contract(
                RawRequest::read("/mcp-integrations").named("airs.gateway.mcp_integrations.list"),
                "ListMcpIntegrationsResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<McpIntegrationDetail> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/mcp-integrations")
                    .segment(id)?
                    .named("airs.gateway.mcp_integrations.get"),
                "McpIntegrationDetailSchema",
                options,
            )
            .await
    }
    pub async fn create(
        &self,
        body: &McpIntegrationCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/mcp-integrations")
                    .named("airs.gateway.mcp_integrations.create"),
                body,
                "McpIntegrationCreateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        id: &str,
        body: &McpIntegrationUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/mcp-integrations")
                    .segment(id)?
                    .named("airs.gateway.mcp_integrations.update"),
                body,
                "McpIntegrationUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(&self, id: &str, options: Option<&RequestOptions>) -> Result<()> {
        common::uuid(id)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/mcp-integrations")
                    .segment(id)?
                    .named("airs.gateway.mcp_integrations.delete"),
                options,
            )
            .await
    }
    pub async fn get_capabilities(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<McpIntegrationCapabilitiesResponse> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/mcp-integrations")
                    .segment(id)?
                    .segment("capabilities")?
                    .named("airs.gateway.mcp_integrations.get_capabilities"),
                "McpIntegrationCapabilitiesResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_metadata(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<McpIntegrationMetadata> {
        common::uuid(id)?;
        self.client
            .send_contract(
                RawRequest::read("/mcp-integrations")
                    .segment(id)?
                    .segment("metadata")?
                    .named("airs.gateway.mcp_integrations.get_metadata"),
                "McpIntegrationMetadataSchema",
                options,
            )
            .await
    }
    pub async fn set_capabilities(
        &self,
        id: &str,
        body: &McpIntegrationCapabilitiesBulkUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<McpIntegrationCapabilitiesUpdateResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/mcp-integrations")
                    .segment(id)?
                    .segment("capabilities")?
                    .named("airs.gateway.mcp_integrations.set_capabilities"),
                body,
                "McpIntegrationCapabilitiesBulkUpdateRequestSchema",
                "McpIntegrationCapabilitiesUpdateResponseSchema",
                options,
            )
            .await
    }
    pub async fn set_workspaces(
        &self,
        id: &str,
        body: &McpIntegrationWorkspacesBulkUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<McpIntegrationWorkspacesUpdateResponse> {
        common::uuid(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/mcp-integrations")
                    .segment(id)?
                    .segment("workspaces")?
                    .named("airs.gateway.mcp_integrations.set_workspaces"),
                body,
                "McpIntegrationWorkspacesBulkUpdateRequestSchema",
                "McpIntegrationWorkspacesUpdateResponseSchema",
                options,
            )
            .await
    }
    /// Preserve the source ordered union without a second HTTP dispatch.
    pub async fn get_workspaces(
        &self,
        id: &str,
        query: &McpIntegrationWorkspacesOptions,
        options: Option<&RequestOptions>,
    ) -> Result<McpIntegrationWorkspacesResponse> {
        common::uuid(id)?;
        let query = match &query.version {
            None => Vec::new(),
            Some(value) if value.len() <= crate::config::MAX_URL_BYTES => {
                vec![("version".into(), value.clone())]
            }
            Some(_) => return Err(Error::new(ErrorKind::Limit)),
        };
        self.client
            .send_contract_one_of(
                RawRequest::read("/mcp-integrations")
                    .segment(id)?
                    .segment("workspaces")?
                    .named("airs.gateway.mcp_integrations.get_workspaces")
                    .query(query),
                &[
                    "GatewayMcpIntegrationWorkspacesListResponseSchema",
                    "GatewayMcpIntegrationWorkspacesLegacyResponseSchema",
                ],
                options,
            )
            .await
    }
}

#[derive(Clone, Default)]
#[non_exhaustive]
pub struct McpIntegrationWorkspacesOptions {
    pub version: Option<String>,
}
crate::redaction::redacted_debug!(McpIntegrationWorkspacesOptions);

/// Variant order matches the overlapping frozen Zod union.
#[derive(Clone, PartialEq, serde::Serialize, serde::Deserialize)]
#[serde(untagged)]
#[non_exhaustive]
pub enum McpIntegrationWorkspacesResponse {
    List(GatewayMcpIntegrationWorkspacesListResponse),
    Legacy(GatewayMcpIntegrationWorkspacesLegacyResponse),
}
crate::redaction::redacted_debug!(McpIntegrationWorkspacesResponse);
