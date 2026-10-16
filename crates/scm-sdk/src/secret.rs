//! Explicit access to credential values with redacted formatting.

use crate::{Error, ErrorKind, Result};
use std::{fmt, sync::Arc};
use zeroize::Zeroizing;

/// A nonempty secret, shared between clones and zeroized when its last owner drops.
///
/// This cannot erase copies made by an HTTP implementation or caller.
#[derive(Clone)]
pub struct Secret(Arc<Zeroizing<String>>);

impl Secret {
    /// Create a secret without including rejected input in an error.
    pub fn new(value: impl Into<String>) -> Result<Self> {
        let value = Zeroizing::new(value.into());
        if value.is_empty() || value.len() > 16 * 1024 {
            return Err(Error::new(ErrorKind::Configuration));
        }
        Ok(Self(Arc::new(value)))
    }

    /// Explicitly access the secret. Callers own any resulting logging or copies.
    pub fn expose(&self) -> &str {
        self.0.as_str()
    }
}

impl fmt::Debug for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("Secret([REDACTED])")
    }
}
impl fmt::Display for Secret {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("[REDACTED]")
    }
}
