//! Workspace reads select one plane explicitly; every write uses the admin plane.
use super::{
    AIGatewayPlane, AIGatewayWorkspaceGetOptions, AIGatewayWorkspaceListOptions,
    AIGatewayWorkspaceStatus, common, models::*,
};
use crate::{Client, Endpoint, RawRequest, RequestOptions, Result};
use http::Method;
#[derive(Clone, Debug)]
pub struct AIGatewayWorkspacesClient {
    pub(super) client: Client,
    pub(super) admin: Client,
}
impl AIGatewayWorkspacesClient {
    pub fn new(client: Client, admin_endpoint: Endpoint) -> Result<Self> {
        let client = common::oauth(client)?;
        let admin = client.with_endpoint(admin_endpoint)?;
        Ok(Self { client, admin })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }
    fn plane(&self, plane: AIGatewayPlane) -> &Client {
        match plane {
            AIGatewayPlane::Data => &self.client,
            AIGatewayPlane::Admin => &self.admin,
        }
    }
    /// Omitting status preserves the server's active-only default.
    pub async fn list(
        &self,
        params: &AIGatewayWorkspaceListOptions,
        options: Option<&RequestOptions>,
    ) -> Result<ListWorkspacesResponse> {
        let query = params
            .status
            .map(|status| {
                vec![(
                    "status".into(),
                    match status {
                        AIGatewayWorkspaceStatus::Active => "active",
                        AIGatewayWorkspaceStatus::Archived => "archived",
                    }
                    .into(),
                )]
            })
            .unwrap_or_default();
        self.plane(params.plane)
            .send_contract(
                RawRequest::read("/workspaces")
                    .query(query)
                    .named("airs.gateway.workspaces.list"),
                "ListWorkspacesResponseSchema",
                options,
            )
            .await
    }
    pub async fn get(
        &self,
        workspace: &str,
        params: &AIGatewayWorkspaceGetOptions,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWorkspaceDetail> {
        common::workspace_ref(workspace)?;
        self.plane(params.plane)
            .send_contract(
                RawRequest::read("/workspaces")
                    .segment(workspace)?
                    .named("airs.gateway.workspaces.get"),
                "GatewayWorkspaceDetailSchema",
                options,
            )
            .await
    }
    /// scope_name must refer to an existing IAM scope. Provisioning is a separate workflow.
    pub async fn create(
        &self,
        body: &GatewayWorkspaceCreateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWorkspaceCreateResponse> {
        self.admin
            .send_model(
                RawRequest::new(Method::POST, "/workspaces")
                    .named("airs.gateway.workspaces.create"),
                body,
                "GatewayWorkspaceCreateRequestSchema",
                "GatewayWorkspaceCreateResponseSchema",
                options,
            )
            .await
    }
    /// Partial update: a successful empty receipt does not echo the resulting state.
    pub async fn update(
        &self,
        workspace: &str,
        body: &GatewayWorkspaceUpdateRequest,
        options: Option<&RequestOptions>,
    ) -> Result<GatewayWriteResponse> {
        common::workspace_ref(workspace)?;
        self.admin
            .send_model(
                RawRequest::new(Method::PUT, "/workspaces")
                    .segment(workspace)?
                    .named("airs.gateway.workspaces.update"),
                body,
                "GatewayWorkspaceUpdateRequestSchema",
                "GatewayWriteResponseSchema",
                options,
            )
            .await
    }
    /// Soft deletion archives the workspace. There is no implicit unarchive or hard delete.
    pub async fn delete(&self, workspace: &str, options: Option<&RequestOptions>) -> Result<()> {
        common::workspace_ref(workspace)?;
        self.admin
            .send_empty(
                RawRequest::new(Method::DELETE, "/workspaces")
                    .segment(workspace)?
                    .named("airs.gateway.workspaces.delete"),
                options,
            )
            .await
    }
}
