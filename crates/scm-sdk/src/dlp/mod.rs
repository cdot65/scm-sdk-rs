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
    pub fn new(client: crate::Client) -> crate::Result<Self> {
        client.require_auth(crate::engine::AuthKind::OAuth)?;
        Ok(Self { client })
    }
    pub fn client(&self) -> &crate::Client {
        &self.client
    }
    pub fn data_patterns(&self) -> DataPatternsClient {
        DataPatternsClient {
            client: self.client.clone(),
        }
    }
    pub fn data_profiles(&self) -> DataProfilesClient {
        DataProfilesClient {
            client: self.client.clone(),
        }
    }
    pub fn dictionaries(&self) -> DictionariesClient {
        DictionariesClient {
            client: self.client.clone(),
        }
    }
    pub fn data_filtering_profiles(&self) -> DataFilteringProfilesClient {
        DataFilteringProfilesClient {
            client: self.client.clone(),
        }
    }
}
