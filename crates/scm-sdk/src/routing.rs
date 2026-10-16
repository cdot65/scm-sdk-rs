//! Explicit Gateway routing headers and acknowledgement of billable replay.
use crate::{Error, ErrorKind, RawRequest, Result};
use http::{HeaderMap, HeaderName, HeaderValue};

/// Validated runtime headers. Values may contain provider credentials or metadata;
/// ordinary formatting always redacts them.
#[derive(Clone, Default)]
pub struct GatewayRouting {
    headers: HeaderMap,
    bytes: usize,
}
crate::redaction::redacted_debug!(GatewayRouting);
impl GatewayRouting {
    pub fn new() -> Self {
        Self::default()
    }

    /// Allow only the frozen runtime routing family and OpenAI's version header.
    /// Duplicate names are rejected case insensitively before credentials are read.
    pub fn header(mut self, name: &str, value: &str) -> Result<Self> {
        if name.len() > 128 || value.len() > 8192 {
            return Err(Error::new(ErrorKind::Limit));
        }
        let name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| Error::new(ErrorKind::Validation))?;
        let allowed = name.as_str() == "openai-beta"
            || name
                .as_str()
                .strip_prefix("x-portkey-")
                .is_some_and(|suffix| {
                    !suffix.is_empty()
                        && suffix.bytes().all(|byte| {
                            byte.is_ascii_lowercase() || byte.is_ascii_digit() || byte == b'-'
                        })
                        && suffix != "api-key"
                });
        if !allowed || self.headers.contains_key(&name) {
            return Err(Error::new(ErrorKind::Validation));
        }
        let bytes = self
            .bytes
            .checked_add(name.as_str().len())
            .and_then(|n| n.checked_add(value.len()))
            .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        if self.headers.len() >= 32 || bytes > 12 * 1024 {
            return Err(Error::new(ErrorKind::Limit));
        }
        let mut value =
            HeaderValue::from_str(value).map_err(|_| Error::new(ErrorKind::Validation))?;
        value.set_sensitive(true);
        self.headers.insert(name, value);
        self.bytes = bytes;
        Ok(self)
    }

    pub(crate) fn apply(&self, request: &mut RawRequest) -> Result<()> {
        request.routing_headers(&self.headers)
    }
}

/// Runtime retries can duplicate remote computation and charges even when the
/// first response was lost. Applications must explicitly acknowledge that risk.
/// A request is never replayed after successful response headers arrive.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum RuntimeReplay {
    #[default]
    Never,
    /// Validated through `bounded`; kept private behind a separate permit type.
    Explicit(RuntimeReplayPermit),
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct RuntimeReplayPermit {
    retries: usize,
}
impl RuntimeReplay {
    /// At most five additional attempts, within the original operation deadline.
    /// Only network failure and HTTP 500/502/503/504 are eligible; 429 is excluded.
    pub fn bounded(retries: usize) -> Result<Self> {
        if retries > 5 {
            return Err(Error::new(ErrorKind::Configuration));
        }
        Ok(Self::Explicit(RuntimeReplayPermit { retries }))
    }
    pub(crate) fn apply(self, request: &mut RawRequest) -> Result<()> {
        request.runtime_replay(match self {
            Self::Never => None,
            Self::Explicit(permit) => Some(permit.retries),
        });
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn routing_is_case_insensitive_bounded_and_redacted() {
        for name in [
            "authorization",
            "X-Portkey-Api-Key",
            "host",
            "x-tsg-id",
            "cookie",
        ] {
            assert!(GatewayRouting::new().header(name, "secret").is_err());
        }
        let routing = GatewayRouting::new()
            .header("X-Portkey-Provider", "private")
            .unwrap();
        assert!(!format!("{routing:?}").contains("private"));
        assert!(routing.header("x-portkey-provider", "other").is_err());
        assert!(
            GatewayRouting::new()
                .header("openai-beta", "x\r\nY:z")
                .is_err()
        );
        assert!(
            GatewayRouting::new()
                .header("openai-beta", &"x".repeat(8193))
                .is_err()
        );
        assert!(RuntimeReplay::bounded(6).is_err());
        assert_eq!(RuntimeReplay::default(), RuntimeReplay::Never);
    }
}
