//! Canonical SCM IAM scope resources, independent of the AIRS product tree.
pub mod models {
    pub use crate::models::iam::*;
}
#[cfg(feature = "iam")]
mod scopes;
#[cfg(feature = "iam")]
pub use scopes::{IamScopesClient, validate_scope_name};
