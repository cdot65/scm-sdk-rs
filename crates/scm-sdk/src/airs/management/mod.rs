//! AIRS management, authenticated with SCM OAuth service credentials.
pub mod models {
    pub use crate::models::management::*;
}
#[cfg(feature = "management")]
mod profiles;
#[cfg(feature = "management")]
pub use profiles::{ManagementClient, PaginationOptions, ProfilesClient};
