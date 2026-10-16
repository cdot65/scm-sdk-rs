use super::{common, models::*};
use crate::{Client, RawRequest, RequestOptions, Result};
use http::Method;

/// Explicit OAuth mcp servers operations; response formatting is redacted.
#[derive(Clone, Debug)]
pub struct AIGatewayMcpServersClient {
    pub(super) client: Client,
}
impl AIGatewayMcpServersClient {
    pub fn new(client: Client) -> Result<Self> {
        Ok(Self {
            client: common::oauth(client)?,
        })
    }
    pub async fn list(
        &self,
        query: &GatewayMcpServersClientListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientListResponse> {
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayMcpServersClientListOptionsSchema",
            &["workspace_id", "current_page", "page_size", "id", "search"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/mcp-servers")
                    .named("airs.gateway.mcp_servers.list")
                    .query(query),
                "GatewayMcpServersClientListResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn create(
        &self,
        body: &GatewayMcpServersClientCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientCreateResponse> {
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/mcp-servers")
                    .named("airs.gateway.mcp_servers.create"),
                body,
                "GatewayMcpServersClientCreateRequestSchema",
                "GatewayMcpServersClientCreateResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientGetResponse> {
        common::resource_id(id)?;
        self.client
            .send_contract(
                RawRequest::read("/mcp-servers")
                    .segment(id)?
                    .named("airs.gateway.mcp_servers.get"),
                "GatewayMcpServersClientGetResponseSchema",
                options,
            )
            .await
    }
    pub async fn update(
        &self,
        id: &str,
        body: &GatewayMcpServersClientUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientUpdateResponse> {
        common::resource_id(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/mcp-servers")
                    .segment(id)?
                    .named("airs.gateway.mcp_servers.update"),
                body,
                "GatewayMcpServersClientUpdateRequestSchema",
                "GatewayMcpServersClientUpdateResponseSchema",
                options,
            )
            .await
    }
    pub async fn delete(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientDeleteResponse> {
        common::resource_id(id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/mcp-servers")
                    .segment(id)?
                    .named("airs.gateway.mcp_servers.delete"),
                "GatewayMcpServersClientDeleteResponseSchema",
                options,
            )
            .await
    }
    pub async fn test(
        &self,
        id: &str,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientTestResponse> {
        common::resource_id(id)?;
        self.client
            .send_contract(
                RawRequest::new(Method::POST, "/mcp-servers")
                    .segment(id)?
                    .segment("test")?
                    .named("airs.gateway.mcp_servers.test"),
                "GatewayMcpServersClientTestResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_capabilities(
        &self,
        id: &str,
        query: &GatewayMcpServersClientGetCapabilitiesOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientGetCapabilitiesResponse> {
        common::resource_id(id)?;
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayMcpServersClientGetCapabilitiesOptionsSchema",
            &["page", "page_size", "type"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/mcp-servers")
                    .segment(id)?
                    .segment("capabilities")?
                    .named("airs.gateway.mcp_servers.get_capabilities")
                    .query(query),
                "GatewayMcpServersClientGetCapabilitiesResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn update_capabilities(
        &self,
        id: &str,
        body: &GatewayMcpServersClientUpdateCapabilitiesRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientUpdateCapabilitiesResponse> {
        common::resource_id(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/mcp-servers")
                    .segment(id)?
                    .segment("capabilities")?
                    .named("airs.gateway.mcp_servers.update_capabilities"),
                body,
                "GatewayMcpServersClientUpdateCapabilitiesRequestSchema",
                "GatewayMcpServersClientUpdateCapabilitiesResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_user_access(
        &self,
        id: &str,
        query: &GatewayMcpServersClientGetUserAccessOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientGetUserAccessResponse> {
        common::resource_id(id)?;
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayMcpServersClientGetUserAccessOptionsSchema",
            &["page", "page_size", "search"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/mcp-servers")
                    .segment(id)?
                    .segment("user-access")?
                    .named("airs.gateway.mcp_servers.get_user_access")
                    .query(query),
                "GatewayMcpServersClientGetUserAccessResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn update_user_access(
        &self,
        id: &str,
        body: &GatewayMcpServersClientUpdateUserAccessRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientUpdateUserAccessResponse> {
        common::resource_id(id)?;
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/mcp-servers")
                    .segment(id)?
                    .segment("user-access")?
                    .named("airs.gateway.mcp_servers.update_user_access"),
                body,
                "GatewayMcpServersClientUpdateUserAccessRequestSchema",
                "GatewayMcpServersClientUpdateUserAccessResponseSchema",
                options,
            )
            .await
    }
    pub async fn get_connections(
        &self,
        id: &str,
        query: &GatewayMcpServersClientGetConnectionsOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientGetConnectionsResponse> {
        common::resource_id(id)?;
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayMcpServersClientGetConnectionsOptionsSchema",
            &["user_id", "workspace_id", "current_page", "page_size"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::read("/mcp-servers")
                    .segment(id)?
                    .segment("connections")?
                    .named("airs.gateway.mcp_servers.get_connections")
                    .query(query),
                "GatewayMcpServersClientGetConnectionsResponseSchema",
                Some(&options),
            )
            .await
    }
    pub async fn delete_connections(
        &self,
        id: &str,
        query: &GatewayMcpServersClientDeleteConnectionsOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayMcpServersClientDeleteConnectionsResponse> {
        common::resource_id(id)?;
        let options = self.client.operation_options(options)?;
        let query = common::query(
            &self.client,
            query,
            "GatewayMcpServersClientDeleteConnectionsOptionsSchema",
            &["user_id", "workspace_id"],
            &options,
        )
        .await?;
        self.client
            .send_contract(
                RawRequest::new(Method::DELETE, "/mcp-servers")
                    .segment(id)?
                    .segment("connections")?
                    .named("airs.gateway.mcp_servers.delete_connections")
                    .query(query),
                "GatewayMcpServersClientDeleteConnectionsResponseSchema",
                Some(&options),
            )
            .await
    }
}
