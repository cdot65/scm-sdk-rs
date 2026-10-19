//! Native typed product and shared model namespaces.
mod generated;
mod support;
pub use support::BinaryFile;
pub mod agentguard;
pub mod dlp;
pub mod gateway;
pub mod iam;
pub mod management;
pub mod model_security;
pub mod red_team;
/// Current vendor dataplane model contracts.
pub mod red_team_data_plane;
/// Additive current management model contracts.
pub mod red_team_management;
pub mod scan;
pub mod shared;
#[cfg(test)]
mod tests;
