//! Validated destinations, budgets and caller-owned cancellation.

use crate::{Error, ErrorKind, Result};
use std::{collections::BTreeSet, fmt, future::Future, time::Duration};
use tokio::time::Instant;
use tokio_util::sync::CancellationToken;
use url::Url;

pub(crate) const MAX_URL_BYTES: usize = 16 * 1024;
const MAX_PATH_SEGMENT_BYTES: usize = 4096;

/// An immutable HTTPS endpoint. Local HTTP requires the explicit loopback constructor.
#[derive(Clone)]
pub struct Endpoint {
    url: Url,
    origin: String,
    loopback_http: bool,
}

impl Endpoint {
    pub fn parse(value: &str) -> Result<Self> {
        Self::parse_inner(value, false)
    }
    /// Permit HTTP only for a literal loopback address or localhost.
    pub fn loopback(value: &str) -> Result<Self> {
        Self::parse_inner(value, true)
    }
    fn parse_inner(value: &str, local: bool) -> Result<Self> {
        if value.len() > MAX_URL_BYTES {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let url = Url::parse(value).map_err(|_| Error::new(ErrorKind::Configuration))?;
        if !valid_url(&url, local) || url.query().is_some() {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let origin = url.origin().ascii_serialization();
        Ok(Self {
            url,
            origin,
            loopback_http: local,
        })
    }
    /// The explicitly configured URL, including its path.
    pub fn url(&self) -> &Url {
        &self.url
    }
    pub fn origin(&self) -> &str {
        &self.origin
    }
    /// Require the same origin and secure scheme before credentials are attached.
    pub fn admit(&self, url: &Url) -> Result<()> {
        if !valid_url(url, self.loopback_http) || url.origin().ascii_serialization() != self.origin
        {
            return Err(Error::new(ErrorKind::Destination));
        }
        Ok(())
    }
    pub(crate) fn resolve(
        &self,
        path: &str,
        segments: &[String],
        query: &[(String, String)],
    ) -> Result<Url> {
        if path.len() > MAX_URL_BYTES
            || query.len() > 256
            || query
                .iter()
                .try_fold(0usize, |n, (k, v)| {
                    n.checked_add(k.len())?.checked_add(v.len())
                })
                .is_none_or(|n| n > MAX_URL_BYTES)
        {
            return Err(Error::new(ErrorKind::Limit));
        }
        if path.starts_with("//") || path.contains(['?', '#', '\\']) || path.contains("://") {
            return Err(Error::new(ErrorKind::Validation));
        }
        for part in path.split('/') {
            // Encoded separators can become route traversal after server decoding.
            // Encoded '%' is rejected too, preventing recursive decoding bypasses.
            let lowered = part.to_ascii_lowercase();
            if ["%2f", "%5c", "%25"]
                .iter()
                .any(|value| lowered.contains(value))
            {
                return Err(Error::new(ErrorKind::Validation));
            }
            let normalized = lowered.replace("%2e", ".");
            if matches!(normalized.as_str(), "." | "..") {
                return Err(Error::new(ErrorKind::Validation));
            }
        }
        let mut url = self.url.clone();
        let mut full = format!(
            "{}/{}",
            url.path().trim_end_matches('/'),
            path.trim_start_matches('/')
        );
        for segment in segments {
            full.push('/');
            full.push_str(&encode_segment(segment)?);
            if full.len() > MAX_URL_BYTES {
                return Err(Error::new(ErrorKind::Limit));
            }
        }
        url.set_path(&full);
        if !query.is_empty() {
            url.query_pairs_mut()
                .extend_pairs(query.iter().map(|(k, v)| (k, v)));
        }
        self.admit(&url)?;
        if url.as_str().len() > MAX_URL_BYTES {
            return Err(Error::new(ErrorKind::Limit));
        }
        Ok(url)
    }
}

/// Encode identifier data exactly once, while rejecting traversal at every
/// supported decoding depth. Slashes in identifiers remain encoded on the wire.
pub(crate) fn encode_segment(value: &str) -> Result<String> {
    if value.is_empty() || value.len() > MAX_PATH_SEGMENT_BYTES {
        return Err(Error::new(ErrorKind::Validation));
    }
    let mut decoded = value.as_bytes().to_vec();
    for depth in 0..=8 {
        if decoded.iter().any(|b| b.is_ascii_control() || *b == b'\\')
            || decoded
                .split(|b| *b == b'/')
                .any(|p| p == b"." || p == b"..")
        {
            return Err(Error::new(ErrorKind::Validation));
        }
        let mut next = Vec::with_capacity(decoded.len());
        let mut i = 0;
        while i < decoded.len() {
            if decoded[i] == b'%'
                && i + 2 < decoded.len()
                && let (Some(a), Some(b)) = (hex(decoded[i + 1]), hex(decoded[i + 2]))
            {
                next.push(a * 16 + b);
                i += 3;
            } else {
                next.push(decoded[i]);
                i += 1;
            }
        }
        if next == decoded {
            break;
        }
        if depth == 8 {
            return Err(Error::new(ErrorKind::Validation));
        }
        decoded = next;
    }
    const HEX: &[u8; 16] = b"0123456789ABCDEF";
    let mut encoded = String::with_capacity(value.len());
    for byte in value.bytes() {
        if byte.is_ascii_alphanumeric() || matches!(byte, b'-' | b'_' | b'.' | b'~') {
            encoded.push(byte as char);
        } else {
            encoded.push('%');
            encoded.push(HEX[(byte >> 4) as usize] as char);
            encoded.push(HEX[(byte & 15) as usize] as char);
        }
    }
    Ok(encoded)
}
fn hex(value: u8) -> Option<u8> {
    match value {
        b'0'..=b'9' => Some(value - b'0'),
        b'a'..=b'f' => Some(value - b'a' + 10),
        b'A'..=b'F' => Some(value - b'A' + 10),
        _ => None,
    }
}
fn valid_url(url: &Url, local: bool) -> bool {
    if url.host().is_none()
        || !url.username().is_empty()
        || url.password().is_some()
        || url.fragment().is_some()
    {
        return false;
    }
    if url.scheme() == "https" {
        return true;
    }
    if !local || url.scheme() != "http" {
        return false;
    }
    match url.host() {
        Some(url::Host::Domain(name)) => name == "localhost",
        Some(url::Host::Ipv4(ip)) => ip.is_loopback(),
        Some(url::Host::Ipv6(ip)) => ip.is_loopback(),
        None => false,
    }
}
impl fmt::Debug for Endpoint {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Endpoint")
            .field("origin", &self.origin)
            .finish_non_exhaustive()
    }
}

/// The purpose of a destination admission decision.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum DestinationPurpose {
    Api,
    OAuthToken,
    Download,
    Realtime,
}

/// A caller's additional admission policy, invoked before authentication/network work.
///
/// Implementations must not include rejected URLs or values in error formatting.
pub trait DestinationPolicy: Send + Sync {
    fn check(&self, destination: &Url, purpose: DestinationPurpose) -> Result<()>;
}

/// An exact origin allowlist. Endpoint admission remains enforced independently.
#[derive(Clone, Debug)]
pub struct OriginPolicy {
    origins: BTreeSet<String>,
}
impl OriginPolicy {
    pub fn new(endpoints: impl IntoIterator<Item = Endpoint>) -> Self {
        Self {
            origins: endpoints.into_iter().map(|e| e.origin).collect(),
        }
    }
}
impl DestinationPolicy for OriginPolicy {
    fn check(&self, url: &Url, purpose: DestinationPurpose) -> Result<()> {
        let mut admitted = url.clone();
        // Realtime uses the same explicitly selected authority with its WS scheme.
        if purpose == DestinationPurpose::Realtime {
            match url.scheme() {
                "wss" => {
                    admitted
                        .set_scheme("https")
                        .map_err(|_| Error::new(ErrorKind::Destination))?;
                }
                "ws" => {
                    admitted
                        .set_scheme("http")
                        .map_err(|_| Error::new(ErrorKind::Destination))?;
                }
                _ => return Err(Error::new(ErrorKind::Destination)),
            }
        }
        if self
            .origins
            .contains(&admitted.origin().ascii_serialization())
        {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Destination))
        }
    }
}

/// Resource bounds; validated by client construction.
///
/// Byte limits apply to bytes consumed after decompression. Limits are SDK defaults,
/// not claims about vendor quotas. Long reports should use a streaming workflow.
#[derive(Clone, Debug)]
#[non_exhaustive]
pub struct Limits {
    pub operation_timeout: Duration,
    pub max_request_bytes: usize,
    pub max_response_bytes: usize,
    pub max_error_bytes: usize,
    pub max_concurrency: usize,
    pub max_retries: u32,
    pub retry_base_delay: Duration,
    pub max_retry_delay: Duration,
    pub stream_handshake_timeout: Duration,
    pub stream_idle_timeout: Duration,
    pub stream_session_timeout: Option<Duration>,
    pub max_event_bytes: usize,
}
impl Default for Limits {
    fn default() -> Self {
        Self {
            operation_timeout: Duration::from_secs(60),
            max_request_bytes: 8 * 1024 * 1024,
            max_response_bytes: 8 * 1024 * 1024,
            max_error_bytes: 64 * 1024,
            max_concurrency: 32,
            max_retries: 2,
            retry_base_delay: Duration::from_millis(250),
            max_retry_delay: Duration::from_secs(5),
            stream_handshake_timeout: Duration::from_secs(30),
            stream_idle_timeout: Duration::from_secs(30),
            stream_session_timeout: None,
            max_event_bytes: 1024 * 1024,
        }
    }
}
impl Limits {
    pub fn validate(&self) -> Result<()> {
        let durations = [
            self.operation_timeout,
            self.retry_base_delay,
            self.max_retry_delay,
            self.stream_handshake_timeout,
            self.stream_idle_timeout,
        ];
        let byte_limits = [
            self.max_request_bytes,
            self.max_response_bytes,
            self.max_error_bytes,
            self.max_event_bytes,
        ];
        if durations
            .iter()
            .any(|v| v.is_zero() || *v > Duration::from_secs(86400))
            || self
                .stream_session_timeout
                .is_some_and(|v| v.is_zero() || v > Duration::from_secs(86400))
            || byte_limits
                .iter()
                .any(|v| *v == 0 || *v > 512 * 1024 * 1024)
            || self.max_error_bytes > 1024 * 1024
            || self.max_event_bytes > 16 * 1024 * 1024
            || !(1..=4096).contains(&self.max_concurrency)
            || self.max_retries > 5
            || self.retry_base_delay > self.max_retry_delay
        {
            return Err(Error::new(ErrorKind::Configuration));
        }
        Ok(())
    }
}

/// Application-owned session cancellation, shared by client clones.
#[derive(Clone, Debug, Default)]
pub struct Session {
    cancellation: CancellationToken,
}
impl Session {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn cancel(&self) {
        self.cancellation.cancel();
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }
    pub async fn cancelled(&self) {
        self.cancellation.cancelled().await;
    }
}

/// One request's total deadline and independent cancellation.
#[derive(Clone, Debug)]
pub struct RequestOptions {
    deadline: Instant,
    cancellation: CancellationToken,
    retry_limit: Option<u32>,
}
impl RequestOptions {
    pub fn with_timeout(timeout: Duration) -> Result<Self> {
        if timeout.is_zero() || timeout > Duration::from_secs(86400) {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let deadline = Instant::now()
            .checked_add(timeout)
            .ok_or_else(|| Error::new(ErrorKind::Configuration))?;
        Ok(Self {
            deadline,
            cancellation: CancellationToken::new(),
            retry_limit: None,
        })
    }
    pub fn deadline(&self) -> Instant {
        self.deadline
    }
    /// Override safe-read retries for this operation. This never enables mutation
    /// or inference replay. Zero guarantees a single API attempt.
    pub fn with_retries(mut self, value: u32) -> Result<Self> {
        if value > 5 {
            return Err(Error::new(ErrorKind::Configuration));
        }
        self.retry_limit = Some(value);
        Ok(self)
    }
    pub fn retry_limit(&self) -> Option<u32> {
        self.retry_limit
    }
    pub fn cancel(&self) {
        self.cancellation.cancel();
    }
    pub fn is_cancelled(&self) -> bool {
        self.cancellation.is_cancelled()
    }
    pub fn cancellation_token(&self) -> CancellationToken {
        self.cancellation.clone()
    }
    /// Bound the complete supplied future; cancellation releases its local resources.
    pub async fn run<T>(
        &self,
        session: &Session,
        future: impl Future<Output = Result<T>>,
    ) -> Result<T> {
        if self.is_cancelled() || session.is_cancelled() {
            return Err(Error::new(ErrorKind::Cancelled));
        }
        if Instant::now() >= self.deadline {
            return Err(Error::new(ErrorKind::Deadline));
        }
        tokio::select! { biased;
            _ = self.cancellation.cancelled() => Err(Error::new(ErrorKind::Cancelled)),
            _ = session.cancelled() => Err(Error::new(ErrorKind::Cancelled)),
            _ = tokio::time::sleep_until(self.deadline) => Err(Error::new(ErrorKind::Deadline)),
            value = future => {
                if self.is_cancelled() || session.is_cancelled() { Err(Error::new(ErrorKind::Cancelled)) }
                else if Instant::now() >= self.deadline { Err(Error::new(ErrorKind::Deadline)) }
                else { value }
            },
        }
    }
}
