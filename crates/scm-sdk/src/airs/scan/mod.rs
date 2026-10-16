//! AIRS content scanning, asynchronous batches, and correlated report queries.
pub mod models {
    pub use crate::models::scan::*;
}
#[cfg(feature = "scan")]
mod content;
#[cfg(feature = "scan")]
mod scanner;
#[cfg(feature = "scan")]
pub use content::{Content, ContentBuilder};
#[cfg(feature = "scan")]
pub use scanner::{Region, Scanner, SyncScanOptions};
