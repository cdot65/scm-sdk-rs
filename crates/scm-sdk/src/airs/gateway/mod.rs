//! Gateway runtime and SCM control-plane APIs use distinct credentials and endpoints.
pub mod models {
    pub use crate::models::gateway::*;
}
#[cfg(feature = "gateway")]
mod runtime;
#[cfg(feature = "gateway")]
pub use runtime::RuntimeClient;
