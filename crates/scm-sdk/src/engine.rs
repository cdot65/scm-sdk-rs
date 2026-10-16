use crate::{
    Endpoint, Error, ErrorKind, Limits, OriginPolicy, RemoteOutcome, RequestOptions, Result,
    Session,
    auth::{AuthContext, Authentication, OAuthProvider},
    config::{DestinationPolicy, DestinationPurpose},
    transport::{Transport, TransportRequest, read_limited},
};
use bytes::Bytes;
use http::{HeaderMap, HeaderName, HeaderValue, Method, StatusCode};
use serde::{Serialize, de::DeserializeOwned};
use std::{
    fmt,
    io::Write,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, SystemTime},
};
use tokio::sync::Semaphore;

/// An explicitly raw request for advanced integrations. Product clients validate their
/// typed contracts before reaching this seam. Raw callers own service-specific validation.
pub struct RawRequest {
    name: &'static str,
    method: Method,
    path: String,
    query: Vec<(String, String)>,
    headers: HeaderMap,
    body: Option<Bytes>,
    replay_safe: bool,
}
impl RawRequest {
    /// No replay is assumed for an arbitrary operation, including an arbitrary GET.
    pub fn new(method: Method, path: impl Into<String>) -> Self {
        Self {
            name: "custom.request",
            method,
            path: path.into(),
            query: Vec::new(),
            headers: HeaderMap::new(),
            body: None,
            replay_safe: false,
        }
    }
    /// Declare a GET with read semantics, allowing bounded replay-safe retries.
    pub fn read(path: impl Into<String>) -> Self {
        Self::new(Method::GET, path)
            .named("custom.read")
            .safe_read()
    }
    pub fn query(mut self, values: impl IntoIterator<Item = (String, String)>) -> Self {
        self.query.extend(values);
        self
    }
    /// Authentication, routing authority and framing headers belong to the SDK.
    pub fn header(mut self, name: &str, value: &str) -> Result<Self> {
        let name = HeaderName::from_bytes(name.as_bytes())
            .map_err(|_| Error::new(ErrorKind::Validation))?;
        if forbidden_header(name.as_str()) {
            return Err(Error::new(ErrorKind::Validation));
        }
        let mut value =
            HeaderValue::from_str(value).map_err(|_| Error::new(ErrorKind::Validation))?;
        value.set_sensitive(true);
        self.headers.append(name, value);
        Ok(self)
    }
    /// Send bytes with an explicit content type. Bytes remain exact across signing/retries.
    pub fn body(mut self, bytes: Bytes, content_type: &str) -> Result<Self> {
        content_type
            .parse::<mime::Mime>()
            .map_err(|_| Error::new(ErrorKind::Validation))?;
        let value =
            HeaderValue::from_str(content_type).map_err(|_| Error::new(ErrorKind::Validation))?;
        self.headers.insert(http::header::CONTENT_TYPE, value);
        self.body = Some(bytes);
        Ok(self)
    }
    /// Encode an already constructed finite JSON value within an explicit byte bound.
    pub fn json_value(self, value: &serde_json::Value, limit: usize) -> Result<Self> {
        self.json(value, limit)
    }
    pub(crate) fn named(mut self, name: &'static str) -> Self {
        self.name = name;
        self
    }
    pub(crate) fn safe_read(mut self) -> Self {
        self.replay_safe = true;
        self
    }
    pub(crate) fn json<T: Serialize>(self, value: &T, limit: usize) -> Result<Self> {
        self.body(serialize_bounded(value, limit)?, "application/json")
    }
}
fn forbidden_header(name: &str) -> bool {
    matches!(
        name,
        "authorization"
            | "proxy-authorization"
            | "cookie"
            | "host"
            | "content-length"
            | "content-type"
            | "transfer-encoding"
            | "connection"
            | "upgrade"
            | "x-pan-token"
            | "x-payload-hash"
            | "x-portkey-api-key"
            | "x-tsg-id"
    )
}
impl fmt::Debug for RawRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RawRequest")
            .field("operation", &self.name)
            .field("method", &self.method)
            .field("has_body", &self.body.is_some())
            .finish_non_exhaustive()
    }
}

/// Shared configuration for product clients. Clones share the pool, session and limiter.
#[derive(Clone)]
pub struct Client {
    inner: Arc<Inner>,
}
struct Inner {
    endpoint: Endpoint,
    auth: Authentication,
    transport: Arc<dyn Transport>,
    policy: Arc<dyn DestinationPolicy>,
    limits: Limits,
    session: Session,
    permits: Arc<Semaphore>,
}
pub struct ClientBuilder {
    endpoint: Endpoint,
    auth: Authentication,
    transport: Option<Arc<dyn Transport>>,
    policy: Option<Arc<dyn DestinationPolicy>>,
    limits: Limits,
    session: Session,
    oauth: Option<(AuthContext, crate::Secret, bool)>,
}
impl ClientBuilder {
    pub fn new(endpoint: Endpoint) -> Self {
        Self {
            endpoint,
            auth: Authentication::None,
            transport: None,
            policy: None,
            limits: Limits::default(),
            session: Session::new(),
            oauth: None,
        }
    }
    /// A caller-supplied provider owns its token-exchange transport and policy.
    pub fn authentication(mut self, value: Authentication) -> Self {
        self.auth = value;
        self.oauth = None;
        self
    }
    /// Manage service credentials using this client's transport, destination policy,
    /// and session for both token exchange and API requests. The default policy admits
    /// only the API and issuer origins; a supplied policy must admit both explicitly.
    pub fn oauth_credentials(
        mut self,
        context: AuthContext,
        secret: crate::Secret,
        tsg_header: bool,
    ) -> Self {
        self.auth = Authentication::None;
        self.oauth = Some((context, secret, tsg_header));
        self
    }
    pub fn transport(mut self, value: Arc<dyn Transport>) -> Self {
        self.transport = Some(value);
        self
    }
    pub fn destination_policy(mut self, value: Arc<dyn DestinationPolicy>) -> Self {
        self.policy = Some(value);
        self
    }
    pub fn limits(mut self, value: Limits) -> Self {
        self.limits = value;
        self
    }
    pub fn session(mut self, value: Session) -> Self {
        self.session = value;
        self
    }
    pub fn build(self) -> Result<Client> {
        self.limits.validate()?;
        let policy = self.policy.unwrap_or_else(|| {
            let mut endpoints = vec![self.endpoint.clone()];
            if let Some((context, _, _)) = &self.oauth {
                endpoints.push(context.issuer().clone());
            }
            Arc::new(OriginPolicy::new(endpoints))
        });
        let transport = match self.transport {
            Some(value) => value,
            None => {
                #[cfg(feature = "reqwest-transport")]
                {
                    Arc::new(crate::transport::ReqwestTransport::new()?) as Arc<dyn Transport>
                }
                #[cfg(not(feature = "reqwest-transport"))]
                {
                    return Err(Error::new(ErrorKind::Configuration));
                }
            }
        };
        let auth = match self.oauth {
            Some((context, secret, tsg_header)) => {
                let provider = OAuthProvider::new(
                    context,
                    secret,
                    Arc::clone(&transport),
                    Arc::clone(&policy),
                    self.session.clone(),
                )?;
                Authentication::oauth(Arc::new(provider), tsg_header)
            }
            None => self.auth,
        };
        let permits = Arc::new(Semaphore::new(self.limits.max_concurrency));
        Ok(Client {
            inner: Arc::new(Inner {
                endpoint: self.endpoint,
                auth,
                transport,
                policy,
                limits: self.limits,
                session: self.session,
                permits,
            }),
        })
    }
}
impl fmt::Debug for ClientBuilder {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ClientBuilder(..)")
    }
}
impl fmt::Debug for Client {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Client")
            .field("endpoint", &self.inner.endpoint)
            .field("limits", &self.inner.limits)
            .finish_non_exhaustive()
    }
}
impl Client {
    pub fn builder(endpoint: Endpoint) -> ClientBuilder {
        ClientBuilder::new(endpoint)
    }
    pub fn limits(&self) -> &Limits {
        &self.inner.limits
    }
    pub fn session(&self) -> Session {
        self.inner.session.clone()
    }
    pub fn endpoint(&self) -> &Endpoint {
        &self.inner.endpoint
    }
    pub async fn send_bytes(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<Bytes> {
        let default_options;
        let options = match options {
            Some(v) => v,
            None => {
                default_options =
                    RequestOptions::with_timeout(self.inner.limits.operation_timeout)?;
                &default_options
            }
        };
        let name = request.name;
        let mut url = self.inner.endpoint.resolve(&request.path, &request.query)?;
        // Empty fragments are rejected by Endpoint as well; URL path assembly is origin-pinned.
        self.inner.endpoint.admit(&url)?;
        self.inner
            .policy
            .check(&url, DestinationPurpose::Api)
            .map_err(|_| Error::new(ErrorKind::Destination))?;
        let body = request.body.clone().unwrap_or_default();
        if body.len() > self.inner.limits.max_request_bytes
            || request.headers.len() > 128
            || request
                .headers
                .iter()
                .map(|(k, v)| k.as_str().len() + v.as_bytes().len())
                .sum::<usize>()
                > 16 * 1024
        {
            return Err(Error::new(ErrorKind::Limit).with_operation(name));
        }
        if request
            .headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|h| h.to_str().ok())
            .and_then(|v| v.parse::<mime::Mime>().ok())
            .is_some_and(|v| {
                v.type_() == mime::APPLICATION
                    && (v.subtype() == mime::JSON || v.suffix() == Some(mime::JSON))
            })
            && request.body.is_some()
            && serde_json::from_slice::<serde_json::Value>(&body).is_err()
        {
            return Err(Error::new(ErrorKind::Validation).with_operation(name));
        }
        // The query is already encoded once; each retry uses an immutable clone.
        url.set_fragment(None);
        let sent = AtomicBool::new(false);
        let task = async {
            let _permit = self
                .inner
                .permits
                .acquire()
                .await
                .map_err(|_| Error::new(ErrorKind::Cancelled))?;
            let mut retries = 0;
            let mut refreshed = false;
            loop {
                let mut headers = request.headers.clone();
                let generation = self
                    .inner
                    .auth
                    .prepare(&mut headers, &body, request.body.is_some(), options)
                    .await?;
                sent.store(true, Ordering::Relaxed);
                let prepared = TransportRequest::new(
                    request.method.clone(),
                    url.clone(),
                    headers,
                    body.clone(),
                    name,
                );
                let mut response = match self.inner.transport.send(prepared).await {
                    Ok(value) => value,
                    Err(error) => {
                        if error.kind() == ErrorKind::Transport
                            && request.replay_safe
                            && retries < self.inner.limits.max_retries
                        {
                            let delay = self.backoff(retries);
                            retries += 1;
                            tokio::time::sleep(delay).await;
                            continue;
                        }
                        return Err(error);
                    }
                };
                let status = response.status;
                if status.is_redirection() {
                    return Err(Error::new(ErrorKind::Destination)
                        .with_status(status.as_u16())
                        .with_outcome(RemoteOutcome::Rejected));
                }
                let delay = retry_after(&response.headers);
                if status.is_success() {
                    return read_limited(&mut response.body, self.inner.limits.max_response_bytes)
                        .await;
                }
                // Read only a bounded amount; never retain or format arbitrary upstream bodies.
                let error_body =
                    read_limited(&mut response.body, self.inner.limits.max_error_bytes)
                        .await
                        .map_err(|e| e.with_status(status.as_u16()))?;
                let (request_id, code) = error_metadata(&response.headers, Some(&error_body));
                if status == StatusCode::UNAUTHORIZED
                    && let Some(generation) = generation
                {
                    let invalidated = self.inner.auth.on_unauthorized(generation);
                    if request.replay_safe
                        && !refreshed
                        && invalidated
                        && retries < self.inner.limits.max_retries
                    {
                        refreshed = true;
                        retries += 1;
                        continue;
                    }
                }
                if request.replay_safe
                    && retryable(status)
                    && retries < self.inner.limits.max_retries
                {
                    let wait = delay.unwrap_or_else(|| self.backoff(retries));
                    if wait <= self.inner.limits.max_retry_delay {
                        retries += 1;
                        tokio::time::sleep(wait).await;
                        continue;
                    }
                }
                let outcome = if status.is_client_error() {
                    RemoteOutcome::Rejected
                } else {
                    RemoteOutcome::Unknown
                };
                return Err(Error::new(ErrorKind::Http)
                    .with_status(status.as_u16())
                    .with_metadata(request_id, code, delay)
                    .with_outcome(outcome));
            }
        };
        options.run(&self.inner.session, task).await.map_err(|e| {
            let outcome =
                if sent.load(Ordering::Relaxed) && e.remote_outcome() == RemoteOutcome::NotSent {
                    RemoteOutcome::Unknown
                } else {
                    e.remote_outcome()
                };
            e.with_operation(name).with_outcome(outcome)
        })
    }
    pub async fn send_json<R: DeserializeOwned>(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<R> {
        let name = request.name;
        let options = match options {
            Some(v) => v.clone(),
            None => RequestOptions::with_timeout(self.inner.limits.operation_timeout)?,
        };
        let body = self.send_bytes(request, Some(&options)).await?;
        options
            .run(&self.inner.session, async {
                serde_json::from_slice(&body).map_err(|_| Error::new(ErrorKind::Decode))
            })
            .await
            .map_err(|e| e.with_operation(name).with_outcome(RemoteOutcome::Unknown))
    }
    pub async fn send_optional_json<R: DeserializeOwned>(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<Option<R>> {
        let name = request.name;
        let options = match options {
            Some(v) => v.clone(),
            None => RequestOptions::with_timeout(self.inner.limits.operation_timeout)?,
        };
        let body = self.send_bytes(request, Some(&options)).await?;
        if body.is_empty() {
            return Ok(None);
        }
        options
            .run(&self.inner.session, async {
                serde_json::from_slice(&body)
                    .map(Some)
                    .map_err(|_| Error::new(ErrorKind::Decode))
            })
            .await
            .map_err(|e| e.with_operation(name).with_outcome(RemoteOutcome::Unknown))
    }
    pub async fn send_empty(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<()> {
        self.send_bytes(request, options).await.map(|_| ())
    }
    fn backoff(&self, retry: u32) -> Duration {
        self.inner
            .limits
            .retry_base_delay
            .saturating_mul(1u32 << retry)
            .min(self.inner.limits.max_retry_delay)
    }
}
fn retryable(status: StatusCode) -> bool {
    matches!(status.as_u16(), 429 | 502 | 503 | 504)
}
fn retry_after(headers: &HeaderMap) -> Option<Duration> {
    let value = headers.get(http::header::RETRY_AFTER)?.to_str().ok()?;
    if let Ok(seconds) = value.parse::<u64>() {
        return Some(Duration::from_secs(seconds));
    }
    httpdate::parse_http_date(value)
        .ok()?
        .duration_since(SystemTime::now())
        .ok()
}
fn error_metadata(headers: &HeaderMap, body: Option<&[u8]>) -> (Option<String>, Option<String>) {
    let request_id = ["x-request-id", "x-correlation-id", "request-id"]
        .iter()
        .find_map(|k| {
            headers
                .get(*k)
                .and_then(|v| v.to_str().ok())
                .filter(|v| valid_metadata(v))
                .map(str::to_owned)
        });
    let code = body
        .and_then(|v| serde_json::from_slice::<serde_json::Value>(v).ok())
        .and_then(|v| {
            v.get("code")
                .or_else(|| v.get("error").and_then(|v| v.get("code")))
                .and_then(|v| v.as_str())
                .filter(|v| valid_metadata(v))
                .map(str::to_owned)
        });
    (request_id, code)
}
fn valid_metadata(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 128
        && value
            .bytes()
            .all(|v| v.is_ascii_alphanumeric() || matches!(v, b'-' | b'_' | b'.' | b':'))
}

pub(crate) fn serialize_bounded<T: Serialize>(value: &T, limit: usize) -> Result<Bytes> {
    struct Writer {
        value: Vec<u8>,
        limit: usize,
        exceeded: bool,
    }
    impl Write for Writer {
        fn write(&mut self, bytes: &[u8]) -> std::io::Result<usize> {
            if self
                .value
                .len()
                .checked_add(bytes.len())
                .is_none_or(|n| n > self.limit)
            {
                self.exceeded = true;
                return Err(std::io::Error::other("request byte limit"));
            }
            self.value.extend_from_slice(bytes);
            Ok(bytes.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }
    let mut writer = Writer {
        value: Vec::new(),
        limit,
        exceeded: false,
    };
    if serde_json::to_writer(&mut writer, value).is_err() {
        return Err(Error::new(if writer.exceeded {
            ErrorKind::Limit
        } else {
            ErrorKind::Validation
        }));
    }
    Ok(Bytes::from(writer.value))
}

#[cfg(test)]
mod tests;
