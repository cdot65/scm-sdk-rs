//! AIRS management, authenticated with immutable SCM OAuth service credentials.
pub mod models {
    pub use crate::models::management::*;
}
#[cfg(feature = "management")]
mod api_keys;
#[cfg(feature = "management")]
mod customer_apps;
#[cfg(feature = "management")]
mod dashboard;
#[cfg(feature = "management")]
mod inventory;
#[cfg(feature = "management")]
mod oauth_management;
#[cfg(feature = "management")]
mod profiles;
#[cfg(feature = "management")]
mod topics;
#[cfg(feature = "management")]
use crate::{
    Client, Endpoint, Error, ErrorKind, Result,
    data::{FiniteNumber, Optional},
    engine::AuthKind,
};
#[cfg(feature = "management")]
pub use api_keys::{ApiKeyListAllOptions, ApiKeysClient};
#[cfg(feature = "management")]
pub use customer_apps::{CustomerAppListAllOptions, CustomerAppsClient};
#[cfg(feature = "management")]
pub use dashboard::*;
#[cfg(feature = "management")]
pub use inventory::*;
#[cfg(feature = "management")]
pub use oauth_management::{GetTokenOptions, OAuthManagementClient};
#[cfg(feature = "management")]
pub use profiles::{PaginationOptions, ProfileListAllOptions, ProfilesClient};
#[cfg(feature = "management")]
pub use topics::{TopicListAllOptions, TopicListOptions, TopicsClient};

/// Product resource clients share one identity, transport, session and destination policy.
#[cfg(feature = "management")]
#[derive(Clone, Debug)]
pub struct ManagementClient {
    client: Client,
    dashboard_client: Client,
}
#[cfg(feature = "management")]
impl ManagementClient {
    /// Create AIRS management resource clients from a configured SCM OAuth client.
    ///
    /// Enable `management`. The supplied endpoint is the AIRS management base;
    /// dashboard requests derive their tenant header from the same OAuth identity.
    ///
    /// ```no_run
    /// use scm_sdk::{Client, Result, airs::management::{ManagementClient, PaginationOptions}};
    /// async fn profile_page(oauth: Client) -> Result<()> {
    ///     let management = ManagementClient::new(oauth)?;
    ///     let _page = management.profiles().list(PaginationOptions::default(), None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: Client) -> Result<Self> {
        client.require_auth(AuthKind::OAuth)?;
        let dashboard_client = client.with_tsg_header(true)?;
        Ok(Self {
            client,
            dashboard_client,
        })
    }
    /// Borrow the shared AIRS management client and its cancellation session.
    pub fn client(&self) -> &Client {
        &self.client
    }
    /// Override only the dashboard origin/base path, subject to the shared policy.
    pub fn with_dashboard_endpoint(mut self, endpoint: Endpoint) -> Result<Self> {
        self.dashboard_client = self.dashboard_client.with_endpoint(endpoint)?;
        Ok(self)
    }
    /// Access security profiles using the configured management endpoint.
    pub fn profiles(&self) -> ProfilesClient {
        ProfilesClient {
            client: self.client.clone(),
        }
    }
    /// Access custom topics and topic associations.
    pub fn topics(&self) -> TopicsClient {
        TopicsClient {
            client: self.client.clone(),
        }
    }
    /// Access AIRS scan API keys for this OAuth tenant.
    pub fn api_keys(&self) -> ApiKeysClient {
        ApiKeysClient {
            client: self.client.clone(),
        }
    }
    /// Access customer applications for this OAuth tenant.
    pub fn customer_apps(&self) -> CustomerAppsClient {
        CustomerAppsClient {
            client: self.client.clone(),
        }
    }
    /// Access dashboard reads through the explicitly scoped dashboard endpoint.
    pub fn dashboard(&self) -> DashboardClient {
        DashboardClient {
            client: self.dashboard_client.clone(),
        }
    }
    /// Access deployment profile inventory.
    pub fn deployment_profiles(&self) -> DeploymentProfilesClient {
        DeploymentProfilesClient {
            client: self.client.clone(),
        }
    }
    /// Access the legacy AIRS management DLP profile listing.
    pub fn dlp_profiles(&self) -> DlpProfilesClient {
        DlpProfilesClient {
            client: self.client.clone(),
        }
    }
    /// Access scan-log inventory with the management client's request bounds.
    pub fn scan_logs(&self) -> ScanLogsClient {
        ScanLogsClient {
            client: self.client.clone(),
        }
    }
    /// Access explicit token-management operations using the shared OAuth identity.
    pub fn oauth(&self) -> Result<OAuthManagementClient> {
        OAuthManagementClient::new(self.client.clone())
    }
    /// Canonical DLP requires its own explicitly admitted endpoint.
    pub fn dlp(&self, endpoint: Endpoint) -> Result<crate::dlp::DlpClient> {
        crate::dlp::DlpClient::new(self.client.with_endpoint(endpoint)?)
    }
}
#[cfg(feature = "management")]
fn tenant(client: &Client) -> Result<&str> {
    client
        .oauth_context()
        .map(|context| context.tsg_id())
        .ok_or_else(|| Error::new(ErrorKind::Authentication))
}
#[cfg(feature = "management")]
fn uuid(value: &str) -> Result<()> {
    if crate::data::is_uuid(value) {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Validation))
    }
}
#[cfg(feature = "management")]
const ECMASCRIPT_WHITESPACE: &[char] = &[
    '\u{0009}', '\u{000a}', '\u{000b}', '\u{000c}', '\u{000d}', '\u{0020}', '\u{00a0}', '\u{1680}',
    '\u{2000}', '\u{2001}', '\u{2002}', '\u{2003}', '\u{2004}', '\u{2005}', '\u{2006}', '\u{2007}',
    '\u{2008}', '\u{2009}', '\u{200a}', '\u{2028}', '\u{2029}', '\u{202f}', '\u{205f}', '\u{3000}',
    '\u{feff}',
];
#[cfg(feature = "management")]
fn nonblank(value: &str) -> Result<()> {
    if value.trim_matches(ECMASCRIPT_WHITESPACE).is_empty() {
        Err(Error::new(ErrorKind::Validation))
    } else {
        Ok(())
    }
}
#[cfg(feature = "management")]
fn next_offset(value: Optional<FiniteNumber>) -> Result<Option<u64>> {
    match value {
        Optional::Unset => Ok(None),
        Optional::Value(value) => match value.as_f64() {
            Some(0.0) => Ok(None),
            Some(n) if n > 0.0 && n.fract() == 0.0 && n <= 9_007_199_254_740_991.0 => {
                Ok(Some(n as u64))
            }
            _ => Err(Error::new(ErrorKind::Decode)),
        },
    }
}
#[cfg(feature = "management")]
fn optional_items<T>(value: Optional<Vec<T>>) -> Vec<T> {
    value.into_option().unwrap_or_default()
}
