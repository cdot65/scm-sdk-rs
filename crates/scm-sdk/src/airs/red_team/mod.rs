//! Prisma AIRS Red Teaming, with explicit data, management and broker endpoints.
pub mod models {
    pub use crate::models::red_team::*;
    /// Exact current vendor schemas, alongside the compatible existing models.
    pub use crate::models::red_team_data_plane as data_plane;
}
#[cfg(feature = "red-team")]
mod adapters;
#[cfg(feature = "red-team")]
mod common;
#[cfg(feature = "red-team")]
mod custom_attack_reports;
#[cfg(feature = "red-team")]
mod custom_attacks;
#[cfg(feature = "red-team")]
mod eula;
#[cfg(feature = "red-team")]
mod facade;
#[cfg(feature = "red-team")]
mod instances;
#[cfg(feature = "red-team")]
mod network_broker;
#[cfg(feature = "red-team")]
mod reports;
#[cfg(feature = "red-team")]
mod scans;
#[cfg(feature = "red-team")]
mod targets;
#[cfg(feature = "red-team")]
pub use adapters::*;
#[cfg(feature = "red-team")]
pub use common::RedTeamListOptions;
#[cfg(feature = "red-team")]
pub use custom_attack_reports::*;
#[cfg(feature = "red-team")]
pub use custom_attacks::*;
#[cfg(feature = "red-team")]
pub use eula::*;
#[cfg(feature = "red-team")]
pub use facade::*;
#[cfg(feature = "red-team")]
pub use instances::*;
#[cfg(feature = "red-team")]
pub use network_broker::*;
#[cfg(feature = "red-team")]
pub use reports::*;
#[cfg(feature = "red-team")]
pub use scans::*;
#[cfg(feature = "red-team")]
pub use targets::*;
