//! Canonical DLP resources, authenticated with an explicit SCM OAuth client.
pub mod models {
    pub use crate::models::dlp::*;
}
#[cfg(feature = "dlp")]
mod common;
#[cfg(feature = "dlp")]
mod data_patterns;
#[cfg(feature = "dlp")]
mod data_profiles;
#[cfg(feature = "dlp")]
mod dictionaries;
#[cfg(feature = "dlp")]
mod filtering_profiles;
#[cfg(feature = "dlp")]
pub use common::{
    DataFilteringProfileListParams, DataPatternListParams, DataProfileListParams,
    DictionaryListParams, FilteringStatus, SpringListParams,
};
#[cfg(feature = "dlp")]
pub use data_patterns::DataPatternsClient;
#[cfg(feature = "dlp")]
pub use data_profiles::DataProfilesClient;
#[cfg(feature = "dlp")]
pub use dictionaries::{DictionariesClient, DictionaryUpload};
#[cfg(feature = "dlp")]
pub use filtering_profiles::DataFilteringProfilesClient;

/// DLP facade using the caller's endpoint, OAuth identity and transport policy.
#[cfg(feature = "dlp")]
#[derive(Clone, Debug)]
pub struct DlpClient {
    client: crate::Client,
}
#[cfg(feature = "dlp")]
impl DlpClient {
    /// Create canonical DLP resources from an OAuth client with an explicit DLP base.
    ///
    /// Enable `dlp`. The endpoint belongs to DLP and is not inferred from an AIRS
    /// management base. The caller retains runtime, transport and credential ownership.
    ///
    /// ```no_run
    /// use scm_sdk::{Client, Result, dlp::{DataPatternListParams, DlpClient}};
    /// async fn pattern_page(oauth: Client) -> Result<()> {
    ///     let dlp = DlpClient::new(oauth)?;
    ///     let query = DataPatternListParams::new().page(0).size(20);
    ///     let _page = dlp.data_patterns().list(&query, None).await?;
    ///     Ok(())
    /// }
    /// ```
    pub fn new(client: crate::Client) -> crate::Result<Self> {
        client.require_auth(crate::engine::AuthKind::OAuth)?;
        Ok(Self { client })
    }
    /// Borrow the configured DLP client and its shared session.
    pub fn client(&self) -> &crate::Client {
        &self.client
    }
    /// Access data pattern definitions.
    pub fn data_patterns(&self) -> DataPatternsClient {
        DataPatternsClient {
            client: self.client.clone(),
        }
    }
    /// Access DLP data profiles.
    pub fn data_profiles(&self) -> DataProfilesClient {
        DataProfilesClient {
            client: self.client.clone(),
        }
    }
    /// Access dictionaries, including explicit bounded file uploads.
    pub fn dictionaries(&self) -> DictionariesClient {
        DictionariesClient {
            client: self.client.clone(),
        }
    }
    /// Access data filtering profiles.
    pub fn data_filtering_profiles(&self) -> DataFilteringProfilesClient {
        DataFilteringProfilesClient {
            client: self.client.clone(),
        }
    }
}
