use super::models::{
    IamScope, IamScopeCreateInput, IamScopeCreateRequest, IamScopeListResponse,
    IamScopeResourceInput, IamScopeUpdateInput, IamScopeUpdateRequest,
};
use crate::{Client, Error, ErrorKind, RawRequest, RequestOptions, Result, data::Optional};
use http::Method;

/// SCM's scope-name path grammar. Errors never echo the supplied name.
pub fn validate_scope_name(name: &str) -> Result<()> {
    let mut bytes = name.bytes();
    if !bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphanumeric() || byte == b'_')
        || !bytes.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'.' | b'-'))
    {
        return Err(Error::new(ErrorKind::Validation));
    }
    crate::config::encode_segment(name).map(|_| ())
}

/// Tenant IAM scopes. Configure the client endpoint explicitly, including `/iam/v1`.
/// Reads and writes retain the caller's transport, destination policy and session.
#[derive(Clone, Debug)]
pub struct IamScopesClient {
    client: Client,
}
impl IamScopesClient {
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(crate::engine::AuthKind::OAuth)?;
        Ok(Self {
            client: client.with_tsg_header(true)?,
        })
    }
    pub fn client(&self) -> &Client {
        &self.client
    }

    /// List scopes. The observed API is not paginated; count is retained verbatim.
    pub async fn list(&self, options: Option<&RequestOptions>) -> Result<IamScopeListResponse> {
        self.client
            .send_contract(
                RawRequest::read("/scopes").named("shared.iam.scopes.list"),
                "IamScopeListResponseSchema",
                options,
            )
            .await
    }
    /// Fetch by name; the display `name:tsg` ID is not a valid path key.
    pub async fn get(&self, name: &str, options: Option<&RequestOptions>) -> Result<IamScope> {
        validate_scope_name(name)?;
        self.client
            .send_contract(
                RawRequest::read("/scopes")
                    .segment(name)?
                    .named("shared.iam.scopes.get"),
                "IamScopeSchema",
                options,
            )
            .await
    }
    /// Create an unbound scope by omitting resources. Description defaults to empty.
    pub async fn create(
        &self,
        input: &IamScopeCreateInput,
        options: Option<&RequestOptions>,
    ) -> Result<IamScope> {
        let body = IamScopeCreateRequest {
            name: input.name.clone(),
            description: input.description.clone().into_option().unwrap_or_default(),
            resources: default_metadata(input.resources.clone().into_option().unwrap_or_default()),
            extensions: Default::default(),
        };
        self.client
            .send_model(
                RawRequest::new(Method::POST, "/scopes").named("shared.iam.scopes.create"),
                &body,
                "IamScopeCreateRequestSchema",
                "IamScopeSchema",
                options,
            )
            .await
    }
    /// Full replacement: omitted description/resources become `""`/`[]`.
    /// An ambiguous PUT is never automatically replayed.
    pub async fn update(
        &self,
        name: &str,
        input: &IamScopeUpdateInput,
        options: Option<&RequestOptions>,
    ) -> Result<IamScope> {
        validate_scope_name(name)?;
        let body = IamScopeUpdateRequest {
            name: name.into(),
            description: input.description.clone().into_option().unwrap_or_default(),
            resources: default_metadata(input.resources.clone().into_option().unwrap_or_default()),
            extensions: Default::default(),
        };
        self.client
            .send_model(
                RawRequest::new(Method::PUT, "/scopes")
                    .segment(name)?
                    .named("shared.iam.scopes.update"),
                &body,
                "IamScopeUpdateRequestSchema",
                "IamScopeSchema",
                options,
            )
            .await
    }
    /// Read, append a missing workspace binding, then replace the scope.
    /// Existing bindings survive and duplicate bindings are not added. Concurrent
    /// writers can overwrite each other: the upstream API exposes no transaction.
    /// Both steps share one deadline and cancellation token; the PUT has no replay.
    pub async fn bind_workspace(
        &self,
        name: &str,
        workspace_slug: &str,
        options: Option<&RequestOptions>,
    ) -> Result<IamScope> {
        validate_scope_name(name)?;
        validate_workspace_slug(workspace_slug)?;
        let options = self.client.operation_options(options)?;
        let current = self.get(name, Some(&options)).await?;
        let mut resources: Vec<_> = current
            .resources
            .into_iter()
            .map(|resource| IamScopeResourceInput {
                resource_type: resource.resource_type,
                resource_id: resource.resource_id,
                metadata: resource.metadata,
                extensions: Default::default(),
            })
            .collect();
        if !resources.iter().any(|resource| {
            resource.resource_type == "workspace" && resource.resource_id == workspace_slug
        }) {
            resources.push(IamScopeResourceInput {
                resource_type: "workspace".into(),
                resource_id: workspace_slug.into(),
                metadata: Optional::Value(Vec::new()),
                extensions: Default::default(),
            });
        }
        let input = IamScopeUpdateInput {
            description: Optional::Value(current.description),
            resources: Optional::Value(resources),
            extensions: Default::default(),
        };
        self.update(name, &input, Some(&options)).await
    }
    /// Experimental: the frozen source infers DELETE from REST shape; no live
    /// deletion was verified. A 404/405 remains an explicit service rejection.
    pub async fn delete(&self, name: &str, options: Option<&RequestOptions>) -> Result<()> {
        validate_scope_name(name)?;
        self.client
            .send_empty(
                RawRequest::new(Method::DELETE, "/scopes")
                    .segment(name)?
                    .named("shared.iam.scopes.delete"),
                options,
            )
            .await
    }
}
fn default_metadata(mut resources: Vec<IamScopeResourceInput>) -> Vec<IamScopeResourceInput> {
    for resource in &mut resources {
        if resource.metadata.is_unset() {
            resource.metadata = Optional::Value(Vec::new());
        }
    }
    resources
}
fn validate_workspace_slug(slug: &str) -> Result<()> {
    let mut bytes = slug.bytes();
    if !bytes
        .next()
        .is_some_and(|byte| byte.is_ascii_alphanumeric())
        || !bytes.all(|byte| byte.is_ascii_alphanumeric() || matches!(byte, b'_' | b'-'))
    {
        return Err(Error::new(ErrorKind::Validation));
    }
    crate::config::encode_segment(slug).map(|_| ())
}
