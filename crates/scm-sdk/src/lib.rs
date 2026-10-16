//! Native async Strata Cloud Manager SDK.
//!
//! The application owns the Tokio runtime, credential persistence and enforcement policy.
//! SDK formatting contains metadata; API data remains available through explicit access.
#![forbid(unsafe_code)]
#![doc = concat!("\n## Explicit credentials\n\n```no_run\n", include_str!("../examples/foundation.rs"), "\n```")]

pub mod auth;
pub mod config;
pub mod error;
pub mod secret;
pub mod transport;

mod engine;

pub use config::{Endpoint, Limits, OriginPolicy, RequestOptions, Session};
pub use engine::{Client, ClientBuilder, RawRequest};
pub use error::{Error, ErrorKind, RemoteOutcome, Result};
pub use secret::Secret;
