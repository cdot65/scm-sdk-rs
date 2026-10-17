//! Distinct SCM OAuth, runtime-key, and public Gateway credential planes.
#[cfg(feature = "realtime")]
pub use crate::realtime::{
    GatewayRealtimeConnection, GatewayRealtimeOptions, GatewayRealtimeReceiver,
    GatewayRealtimeSender, GatewayWebSocket, GatewayWebSocketFactory, WebSocketConnector,
};
pub mod models {
    pub use crate::models::gateway::*;
}
#[cfg(feature = "gateway")]
mod admin_guardrails;
#[cfg(feature = "gateway")]
mod api_keys;
#[cfg(feature = "gateway")]
mod audit_logs;
#[cfg(feature = "gateway")]
mod common;
#[cfg(feature = "gateway")]
mod configs;
#[cfg(feature = "gateway")]
mod deployments;
#[cfg(feature = "gateway")]
mod guardrails;
#[cfg(feature = "gateway")]
mod helpers;
#[cfg(feature = "gateway")]
mod integrations;
#[cfg(feature = "gateway")]
mod log_exports;
#[cfg(feature = "gateway")]
mod mcp_integrations;
#[cfg(feature = "gateway")]
mod mcp_servers;
#[cfg(feature = "gateway")]
mod model_pricing;
#[cfg(feature = "gateway")]
mod organisations;
#[cfg(feature = "gateway")]
mod plugins;
#[cfg(feature = "gateway")]
mod providers;
#[cfg(feature = "gateway")]
mod provision;
#[cfg(feature = "gateway")]
mod rate_limits;
#[cfg(feature = "gateway")]
mod runtime;
#[cfg(feature = "gateway")]
mod secret_references;
#[cfg(feature = "gateway")]
mod telemetry;
#[cfg(feature = "gateway")]
mod usage_limits;
#[cfg(feature = "gateway")]
mod workspaces;
#[cfg(feature = "gateway")]
pub use crate::routing::{GatewayRouting, RuntimeReplay};
#[cfg(feature = "gateway")]
pub use admin_guardrails::*;
#[cfg(feature = "gateway")]
pub use api_keys::*;
#[cfg(feature = "gateway")]
pub use audit_logs::*;
#[cfg(feature = "gateway")]
pub use common::{
    AIGatewayPlane, AIGatewayWorkspaceGetOptions, AIGatewayWorkspaceListOptions,
    AIGatewayWorkspaceScopedListOptions, AIGatewayWorkspaceStatus,
};
#[cfg(feature = "gateway")]
pub use configs::*;
#[cfg(feature = "gateway")]
pub use deployments::*;
#[cfg(feature = "gateway")]
pub use guardrails::*;
#[cfg(feature = "gateway")]
pub use helpers::*;
#[cfg(feature = "gateway")]
pub use integrations::*;
#[cfg(feature = "gateway")]
pub use log_exports::*;
#[cfg(feature = "gateway")]
pub use mcp_integrations::*;
#[cfg(feature = "gateway")]
pub use mcp_servers::*;
#[cfg(feature = "gateway")]
pub use model_pricing::*;
#[cfg(feature = "gateway")]
pub use organisations::*;
#[cfg(feature = "gateway")]
pub use plugins::*;
#[cfg(feature = "gateway")]
pub use providers::*;
#[cfg(feature = "gateway")]
pub use provision::*;
#[cfg(feature = "gateway")]
pub use rate_limits::*;
#[cfg(feature = "gateway")]
pub use runtime::*;
#[cfg(feature = "gateway")]
pub use secret_references::*;
#[cfg(feature = "gateway")]
pub use telemetry::*;
#[cfg(feature = "gateway")]
pub use usage_limits::*;
#[cfg(feature = "gateway")]
pub use workspaces::*;
#[cfg(feature = "gateway")]
#[derive(Clone, Debug)]
/// SCM OAuth control-plane resources with separately admitted admin and IAM bases.
pub struct AIGatewayClient {
    data: crate::Client,
    admin: crate::Client,
    iam: crate::shared::iam::IamScopesClient,
    runtime: Option<RuntimeClient>,
}
#[cfg(feature = "gateway")]
impl AIGatewayClient {
    /// Share one scoped SCM identity across explicitly admitted control-plane endpoints.
    ///
    /// Enable `gateway`. The OAuth client selects the data base; its policy must
    /// also admit admin and IAM. Runtime inference has separate credentials.
    ///
    /// ```no_run
    /// use scm_sdk::{Client, Endpoint, Result,
    ///     airs::gateway::{AIGatewayClient, AIGatewayWorkspaceScopedListOptions}};
    /// async fn service_keys(
    ///     oauth: Client, admin: Endpoint, iam: Endpoint, workspace_id: &str,
    /// ) -> Result<()> {
    ///     let gateway = AIGatewayClient::new(oauth, admin, iam)?;
    ///     let query = AIGatewayWorkspaceScopedListOptions::new(workspace_id);
    ///     let _keys = gateway.api_keys().list_service(&query, None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(
        data: crate::Client,
        admin_endpoint: crate::Endpoint,
        iam_endpoint: crate::Endpoint,
    ) -> crate::Result<Self> {
        let data = common::oauth(data)?;
        let admin = data.with_endpoint(admin_endpoint)?;
        let iam = crate::shared::iam::IamScopesClient::new(data.with_endpoint(iam_endpoint)?)?;
        Ok(Self {
            data,
            admin,
            iam,
            runtime: None,
        })
    }
    /// Borrow the OAuth data-plane client and its shared session.
    pub fn client(&self) -> &crate::Client {
        &self.data
    }
    /// Attach a separately constructed runtime-key client. SCM OAuth is never copied into it.
    pub fn with_runtime(mut self, runtime: RuntimeClient) -> Self {
        self.runtime = Some(runtime);
        self
    }
    /// Return the attached runtime client, or a configuration error if none was attached.
    pub fn inference(&self) -> crate::Result<RuntimeClient> {
        self.runtime
            .clone()
            .ok_or_else(|| crate::Error::new(crate::ErrorKind::Configuration))
    }
    /// Return the same separately authenticated client used for runtime resources.
    pub fn runtime_resources(&self) -> crate::Result<RuntimeClient> {
        self.inference()
    }
    /// Access tenant scope operations at the explicitly selected IAM base.
    pub fn iam_scopes(&self) -> crate::shared::iam::IamScopesClient {
        self.iam.clone()
    }
    /// Access workspace reads and explicit admin-plane provisioning workflows.
    pub fn workspaces(&self) -> AIGatewayWorkspacesClient {
        AIGatewayWorkspacesClient {
            client: self.data.clone(),
            admin: self.admin.clone(),
            iam: Some(self.iam.clone()),
        }
    }
    pub fn guardrails(&self) -> AIGatewayGuardrailsClient {
        AIGatewayGuardrailsClient {
            client: self.data.clone(),
            admin: Some(self.admin.clone()),
        }
    }
    pub fn telemetry(&self) -> AIGatewayTelemetryClient {
        AIGatewayTelemetryClient {
            client: self.data.clone(),
        }
    }
    pub fn configs(&self) -> AIGatewayConfigsClient {
        AIGatewayConfigsClient {
            client: self.data.clone(),
        }
    }
    pub fn deployments(&self) -> AIGatewayDeploymentsClient {
        AIGatewayDeploymentsClient {
            client: self.admin.clone(),
        }
    }
    pub fn organisations(&self) -> AIGatewayOrganisationsClient {
        AIGatewayOrganisationsClient {
            client: self.admin.clone(),
        }
    }
    pub fn providers(&self) -> AIGatewayProvidersClient {
        AIGatewayProvidersClient {
            client: self.data.clone(),
        }
    }
    pub fn rate_limits(&self) -> AIGatewayRateLimitsClient {
        AIGatewayRateLimitsClient {
            client: self.data.clone(),
        }
    }
    pub fn log_exports(&self) -> AIGatewayLogExportsClient {
        AIGatewayLogExportsClient {
            client: self.data.clone(),
        }
    }
    pub fn api_keys(&self) -> AIGatewayApiKeysClient {
        AIGatewayApiKeysClient {
            client: self.data.clone(),
        }
    }
    pub fn integrations(&self) -> AIGatewayIntegrationsClient {
        AIGatewayIntegrationsClient {
            client: self.admin.clone(),
        }
    }
    pub fn mcp_servers(&self) -> AIGatewayMcpServersClient {
        AIGatewayMcpServersClient {
            client: self.data.clone(),
        }
    }
    pub fn mcp_integrations(&self) -> AIGatewayMcpIntegrationsClient {
        AIGatewayMcpIntegrationsClient {
            client: self.admin.clone(),
        }
    }
    pub fn admin_guardrails(&self) -> AIGatewayAdminGuardrailsClient {
        AIGatewayAdminGuardrailsClient {
            client: self.admin.clone(),
        }
    }
    pub fn usage_limits(&self) -> AIGatewayUsageLimitsClient {
        AIGatewayUsageLimitsClient {
            client: self.data.clone(),
        }
    }
    pub fn secret_references(&self) -> AIGatewaySecretReferencesClient {
        AIGatewaySecretReferencesClient {
            client: self.admin.clone(),
        }
    }
    pub fn plugins(&self) -> AIGatewayPluginsClient {
        AIGatewayPluginsClient {
            client: self.admin.clone(),
        }
    }
    pub fn audit_logs(&self) -> AIGatewayAuditLogsClient {
        AIGatewayAuditLogsClient {
            client: self.admin.clone(),
        }
    }
}
#[cfg(feature = "gateway")]
pub type GatewayClient = AIGatewayClient;
