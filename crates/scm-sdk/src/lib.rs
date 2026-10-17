//! Native async Strata Cloud Manager SDK.
//!
//! The application owns the Tokio runtime, credential persistence and enforcement policy.
//! SDK formatting contains metadata; API data remains available through explicit access.
#![forbid(unsafe_code)]
#![doc = concat!("\n## Explicit credentials\n\n```no_run\n", include_str!("../examples/foundation.rs"), "\n```")]

pub mod auth;
pub mod config;
pub mod configuration;
pub mod constants;
pub mod data;
pub mod environment;
pub mod error;
pub mod helpers;
pub mod pagination;
#[cfg(feature = "realtime")]
pub mod realtime;
pub mod secret;
pub mod streaming;
pub mod transport;
pub mod validation;

#[cfg(feature = "models")]
pub mod airs;
#[cfg(feature = "models")]
pub mod dlp;
#[cfg(feature = "models")]
mod models;
#[cfg(feature = "models")]
pub mod shared;

#[cfg(any(feature = "dlp", feature = "red-team", feature = "gateway"))]
mod multipart;

mod json_decode;
mod observability;
mod redaction;

#[cfg(feature = "gateway")]
pub mod routing;

mod engine;

pub use config::{Endpoint, Limits, OriginPolicy, RequestOptions, Session};
pub use engine::{Client, ClientBuilder, RawRequest};
pub use error::{
    Error, ErrorKind, ErrorMetadata, ErrorType, FailureKind, ProblemDetails, ProblemFieldError,
    RemoteOutcome, Result,
};
#[cfg(feature = "gateway")]
pub use routing::{GatewayRouting, RuntimeReplay};
pub use secret::Secret;
