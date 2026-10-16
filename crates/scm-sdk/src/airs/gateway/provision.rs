//! Bounded workspace provisioning with explicit recovery after uncertain writes.
use super::{AIGatewayWorkspacesClient, models::*};
use crate::shared::iam::{
    models::{IamScope, IamScopeCreateInput, IamScopeCreateRequest},
    validate_scope_name,
};
use crate::{Error, ErrorKind, RemoteOutcome, RequestOptions, data::Optional};
use serde::{Deserialize, Serialize};
use std::fmt;

/// Workspace creation fields with an optional IAM scope name.
#[derive(Clone, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GatewayWorkspaceProvisionRequest {
    pub name: String,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub scope_name: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub description: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub icon: Optional<String>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub defaults: Optional<GatewayDefaultsInput>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub users: Optional<Vec<String>>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub usage_limits: Optional<Vec<GatewayUsageLimitInput>>,
    #[serde(default, skip_serializing_if = "Optional::is_unset")]
    pub rate_limits: Optional<Vec<GatewayRateLimitInput>>,
}
impl GatewayWorkspaceProvisionRequest {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            scope_name: Optional::Unset,
            description: Optional::Unset,
            icon: Optional::Unset,
            defaults: Optional::Unset,
            users: Optional::Unset,
            usage_limits: Optional::Unset,
            rate_limits: Optional::Unset,
        }
    }
}
#[derive(Clone, Copy, Default)]
pub struct AIGatewayWorkspaceProvisionOptions {
    /// Reuse a caller-owned scope. An explicit scope name is required.
    pub existing_scope: bool,
}
#[derive(Clone)]
pub struct GatewayWorkspaceProvisionResult {
    pub scope: IamScope,
    pub workspace: GatewayWorkspaceCreateResponse,
    pub scope_created: bool,
}
/// The operation that failed; completed resources are retained for recovery.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvisionStage {
    Preflight,
    CreateScope,
    CreateWorkspace,
    BindWorkspace,
}
/// Scope ownership and creation certainty, independent of workspace certainty.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvisionScopeState {
    NotCreated,
    Existing,
    Created,
    Unknown,
    Deleted,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvisionCleanup {
    NotNeeded,
    Retained,
    Deleted,
    Failed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ProvisionWorkspaceState {
    NotCreated,
    Unknown,
    Created,
}
/// Explicit recovery data. Ordinary formatting never prints identifiers or bodies.
#[derive(Clone)]
pub struct ProvisionRecovery {
    pub stage: ProvisionStage,
    pub scope_name: Option<String>,
    pub scope_state: ProvisionScopeState,
    /// Present only when workspace creation failed; creation certainty is separate.
    pub workspace_outcome: Option<RemoteOutcome>,
    pub workspace_state: ProvisionWorkspaceState,
    pub workspace: Option<GatewayWorkspaceCreateResponse>,
    pub cleanup: ProvisionCleanup,
    pub cleanup_error: Option<Error>,
}
/// A safe underlying failure together with bounded facts for manual reconciliation.
#[derive(Clone)]
pub struct ProvisionError {
    error: Error,
    recovery: Box<ProvisionRecovery>,
}
impl ProvisionError {
    pub fn error(&self) -> &Error {
        &self.error
    }
    pub fn recovery(&self) -> &ProvisionRecovery {
        &self.recovery
    }
    pub fn into_parts(self) -> (Error, ProvisionRecovery) {
        (self.error, *self.recovery)
    }
}
impl fmt::Debug for ProvisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProvisionError")
            .field("error", &self.error)
            .field("stage", &self.recovery.stage)
            .field("scope_state", &self.recovery.scope_state)
            .field("workspace_outcome", &self.recovery.workspace_outcome)
            .field("workspace_state", &self.recovery.workspace_state)
            .field("cleanup", &self.recovery.cleanup)
            .finish()
    }
}
impl fmt::Display for ProvisionError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Gateway provisioning failed at {:?}: {}",
            self.recovery.stage, self.error
        )
    }
}
impl std::error::Error for ProvisionError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        Some(&self.error)
    }
}
crate::redaction::redacted_debug!(
    GatewayWorkspaceProvisionRequest,
    AIGatewayWorkspaceProvisionOptions,
    GatewayWorkspaceProvisionResult,
    ProvisionRecovery
);

impl AIGatewayWorkspacesClient {
    /// Create/reuse an IAM scope, create a workspace, then add its IAM binding.
    /// All steps and any safe cleanup share one absolute deadline/cancellation.
    /// An uncertain workspace creation retains the scope for reconciliation.
    pub async fn provision(
        &self,
        request: &GatewayWorkspaceProvisionRequest,
        params: &AIGatewayWorkspaceProvisionOptions,
        options: Option<&RequestOptions>,
    ) -> std::result::Result<GatewayWorkspaceProvisionResult, ProvisionError> {
        let mut recovery = ProvisionRecovery {
            stage: ProvisionStage::Preflight,
            scope_name: None,
            scope_state: ProvisionScopeState::NotCreated,
            workspace_outcome: None,
            workspace_state: ProvisionWorkspaceState::NotCreated,
            workspace: None,
            cleanup: ProvisionCleanup::NotNeeded,
            cleanup_error: None,
        };
        let preflight = async {
            let iam = self
                .iam
                .as_ref()
                .ok_or_else(|| Error::new(ErrorKind::Configuration))?;
            let options = self.admin.operation_options(options)?;
            options
                .run(&self.admin.session(), async {
                    crate::engine::serialize_bounded(request, self.admin.limits().max_request_bytes)
                        .map(|_| ())
                })
                .await?;
            let scope_name = match &request.scope_name {
                Optional::Value(name) => name.clone(),
                Optional::Unset if params.existing_scope => {
                    return Err(Error::new(ErrorKind::Validation));
                }
                Optional::Unset => crate::helpers::generate_workspace_scope_name(&request.name)?,
            };
            validate_scope_name(&scope_name)?;
            let body = GatewayWorkspaceCreateRequest {
                name: request.name.clone(),
                scope_name: scope_name.clone(),
                description: request.description.clone(),
                icon: request.icon.clone(),
                defaults: request.defaults.clone(),
                users: request.users.clone(),
                usage_limits: request.usage_limits.clone(),
                rate_limits: request.rate_limits.clone(),
                extensions: Default::default(),
            };
            // Validate the entire eventual write before creating any scope.
            self.admin
                .validated_json(&body, "GatewayWorkspaceCreateRequestSchema", &options)
                .await?;
            let description = request
                .description
                .clone()
                .into_option()
                .unwrap_or_default();
            let scope_body = IamScopeCreateRequest {
                name: scope_name,
                description,
                resources: Vec::new(),
                extensions: Default::default(),
            };
            iam.client()
                .validated_json(&scope_body, "IamScopeCreateRequestSchema", &options)
                .await?;
            Ok((iam, options, body, scope_body))
        }
        .await;
        let (iam, options, body, scope_body) = preflight.map_err(|error| ProvisionError {
            error,
            recovery: Box::new(recovery.clone()),
        })?;
        recovery.scope_name = Some(body.scope_name.clone());
        recovery.scope_state = if params.existing_scope {
            ProvisionScopeState::Existing
        } else {
            ProvisionScopeState::NotCreated
        };
        if !params.existing_scope {
            recovery.stage = ProvisionStage::CreateScope;
            let input = IamScopeCreateInput {
                name: scope_body.name,
                description: Optional::Value(scope_body.description),
                resources: Optional::Value(Vec::new()),
                extensions: Default::default(),
            };
            match iam.create(&input, Some(&options)).await {
                Ok(scope) if scope.name == body.scope_name => {}
                Ok(_) => {
                    recovery.scope_state = ProvisionScopeState::Unknown;
                    return Err(ProvisionError {
                        error: Error::new(ErrorKind::Protocol).with_outcome(RemoteOutcome::Unknown),
                        recovery: Box::new(recovery),
                    });
                }
                Err(error) => {
                    recovery.scope_state = if error.remote_outcome() == RemoteOutcome::Unknown {
                        ProvisionScopeState::Unknown
                    } else {
                        ProvisionScopeState::NotCreated
                    };
                    return Err(ProvisionError {
                        error,
                        recovery: Box::new(recovery),
                    });
                }
            }
            recovery.scope_state = ProvisionScopeState::Created;
        }
        recovery.stage = ProvisionStage::CreateWorkspace;
        let workspace = match self.create(&body, Some(&options)).await {
            Ok(workspace) if workspace.scope_name == body.scope_name => workspace,
            Ok(workspace) => {
                recovery.workspace = Some(workspace);
                recovery.workspace_state = ProvisionWorkspaceState::Unknown;
                recovery.workspace_outcome = Some(RemoteOutcome::Unknown);
                recovery.cleanup = ProvisionCleanup::Retained;
                return Err(ProvisionError {
                    error: Error::new(ErrorKind::Protocol).with_outcome(RemoteOutcome::Unknown),
                    recovery: Box::new(recovery),
                });
            }
            Err(error) => {
                recovery.workspace_outcome = Some(error.remote_outcome());
                recovery.workspace_state = if error.remote_outcome() == RemoteOutcome::Unknown {
                    ProvisionWorkspaceState::Unknown
                } else {
                    ProvisionWorkspaceState::NotCreated
                };
                // Never delete a possibly referenced scope after an ambiguous
                // create. Existing scopes are never owned by this workflow.
                if !params.existing_scope && error.remote_outcome() != RemoteOutcome::Unknown {
                    match iam.delete(&body.scope_name, Some(&options)).await {
                        Ok(()) => {
                            recovery.scope_state = ProvisionScopeState::Deleted;
                            recovery.cleanup = ProvisionCleanup::Deleted;
                        }
                        Err(cleanup_error) => {
                            recovery.cleanup = ProvisionCleanup::Failed;
                            if cleanup_error.remote_outcome() == RemoteOutcome::Unknown {
                                recovery.scope_state = ProvisionScopeState::Unknown;
                            }
                            recovery.cleanup_error = Some(cleanup_error);
                        }
                    }
                } else {
                    recovery.cleanup = ProvisionCleanup::Retained;
                }
                return Err(ProvisionError {
                    error,
                    recovery: Box::new(recovery),
                });
            }
        };
        recovery.workspace_outcome = None;
        recovery.workspace_state = ProvisionWorkspaceState::Created;
        recovery.workspace = Some(workspace.clone());
        recovery.stage = ProvisionStage::BindWorkspace;
        recovery.cleanup = ProvisionCleanup::Retained;
        let scope = iam
            .bind_workspace(&body.scope_name, &workspace.slug, Some(&options))
            .await
            .map_err(|error| ProvisionError {
                error,
                recovery: Box::new(recovery),
            })?;
        Ok(GatewayWorkspaceProvisionResult {
            scope,
            workspace,
            scope_created: !params.existing_scope,
        })
    }
}
