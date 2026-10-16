//! Prisma AIRS Model Security, with separate OAuth data and management planes.
pub mod models {
    pub use crate::models::model_security::*;
}
#[cfg(feature = "model-security")]
mod client;
#[cfg(feature = "model-security")]
mod custom_rules;
#[cfg(feature = "model-security")]
mod groups;
#[cfg(feature = "model-security")]
mod inventory;
#[cfg(feature = "model-security")]
mod query;
#[cfg(feature = "model-security")]
mod scans;
#[cfg(feature = "model-security")]
use crate::{
    Client, Error, ErrorKind, Result,
    data::{FiniteNumber, Patch},
    engine::AuthKind,
};
#[cfg(feature = "model-security")]
pub use client::ModelSecurityClient;
#[cfg(feature = "model-security")]
pub use custom_rules::ModelSecurityCustomRulesClient;
#[cfg(feature = "model-security")]
pub use groups::ModelSecurityGroupsClient;
#[cfg(feature = "model-security")]
pub use inventory::{ModelSecurityModelsClient, ModelSecurityRulesClient};
#[cfg(feature = "model-security")]
pub use query::*;
#[cfg(feature = "model-security")]
pub use scans::{ModelSecurityScansClient, ScanCursor};
#[cfg(feature = "model-security")]
fn oauth(client: Client) -> Result<Client> {
    client.require_auth(AuthKind::OAuth)?;
    client.with_tsg_header(false)
}
#[cfg(feature = "model-security")]
fn uuid(value: &str) -> Result<()> {
    let bytes = value.as_bytes();
    if bytes.len() == 36
        && bytes.iter().enumerate().all(|(i, b)| {
            if [8, 13, 18, 23].contains(&i) {
                *b == b'-'
            } else {
                b.is_ascii_hexdigit()
            }
        })
    {
        Ok(())
    } else {
        Err(Error::new(ErrorKind::Validation))
    }
}
#[cfg(feature = "model-security")]
fn integer(value: &FiniteNumber) -> Result<u64> {
    match value.as_f64() {
        Some(v) if v >= 0.0 && v.fract() == 0.0 && v <= 9_007_199_254_740_991.0 => Ok(v as u64),
        _ => Err(Error::new(ErrorKind::Decode)),
    }
}
#[cfg(feature = "model-security")]
fn total(value: &Patch<FiniteNumber>) -> Result<Option<u64>> {
    match value {
        Patch::Unset | Patch::Null => Ok(None),
        Patch::Value(value) => integer(value).map(Some),
    }
}
#[cfg(feature = "model-security")]
fn next_skip(skip: u64, count: usize, limit: u32, total: Option<u64>) -> Result<Option<u64>> {
    if count == 0 {
        return Ok(None);
    }
    let next = skip
        .checked_add(u64::try_from(count).map_err(|_| Error::new(ErrorKind::Limit))?)
        .filter(|v| *v <= 9_007_199_254_740_991)
        .ok_or_else(|| Error::new(ErrorKind::Limit))?;
    let more = total.map_or(count == limit as usize, |total| next < total);
    Ok(more.then_some(next))
}
