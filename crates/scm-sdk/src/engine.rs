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

pub(crate) const SDK_USER_AGENT: &str =
    concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk");
fn default_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();
    headers.insert(
        http::header::USER_AGENT,
        HeaderValue::from_static(SDK_USER_AGENT),
    );
    headers.insert("service-name", HeaderValue::from_static("api"));
    headers
}

/// An explicitly raw request for advanced integrations. Product clients validate their
/// typed contracts before reaching this seam. Raw callers own service-specific validation.
pub struct RawRequest {
    name: &'static str,
    method: Method,
    path: String,
    segments: Vec<String>,
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
            segments: Vec::new(),
            query: Vec::new(),
            headers: default_headers(),
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
    /// Append identifier data encoded once as a path segment. Literal slashes and
    /// percent signs are data; recursively encoded traversal is rejected locally.
    pub fn segment(mut self, value: impl Into<String>) -> Result<Self> {
        let value = value.into();
        crate::config::encode_segment(&value)?;
        self.segments.push(value);
        Ok(self)
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
            | "service-name"
            | "user-agent"
    )
}
fn headers_exceed(headers: &HeaderMap) -> bool {
    headers.len() > 128
        || headers
            .iter()
            .try_fold(0usize, |n, (k, v)| {
                n.checked_add(k.as_str().len())?
                    .checked_add(v.as_bytes().len())
            })
            .is_none_or(|n| n > 16 * 1024)
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
#[cfg(any(feature = "scan", feature = "management", feature = "gateway"))]
pub(crate) enum AuthKind {
    #[cfg(feature = "scan")]
    Scan,
    #[cfg(feature = "management")]
    OAuth,
    #[cfg(feature = "gateway")]
    Runtime,
}
struct Inner {
    endpoint: Endpoint,
    auth: Authentication,
    transport: Arc<dyn Transport>,
    policy: Arc<dyn DestinationPolicy>,
    limits: Limits,
    session: Session,
    permits: Arc<Semaphore>,
    validation: Option<crate::validation::ValidationLimits>,
}
pub struct ClientBuilder {
    endpoint: Endpoint,
    auth: Authentication,
    transport: Option<Arc<dyn Transport>>,
    policy: Option<Arc<dyn DestinationPolicy>>,
    limits: Limits,
    session: Session,
    oauth: Option<(AuthContext, crate::Secret, bool)>,
    validation: Option<crate::validation::ValidationLimits>,
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
            validation: None,
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
    /// Set native JSON resource bounds. Request/response byte limits still apply.
    pub fn validation_limits(mut self, value: crate::validation::ValidationLimits) -> Self {
        self.validation = Some(value);
        self
    }
    pub fn session(mut self, value: Session) -> Self {
        self.session = value;
        self
    }
    pub fn build(self) -> Result<Client> {
        self.limits.validate()?;
        if let Some(value) = &self.validation {
            value.validate()?;
        }
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
                validation: self.validation,
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
    pub fn validation_limits(&self) -> Option<&crate::validation::ValidationLimits> {
        self.inner.validation.as_ref()
    }
    #[cfg(any(feature = "scan", feature = "management", feature = "gateway"))]
    fn contract_limits(&self, max_bytes: usize) -> Result<crate::validation::ValidationLimits> {
        let mut limits = self.inner.validation.clone().map_or_else(
            || crate::validation::ValidationLimits::for_bytes(max_bytes),
            Ok,
        )?;
        limits.max_input_bytes = limits.max_input_bytes.min(max_bytes);
        limits.validate()?;
        Ok(limits)
    }
    pub fn session(&self) -> Session {
        self.inner.session.clone()
    }
    pub fn endpoint(&self) -> &Endpoint {
        &self.inner.endpoint
    }
    #[cfg(any(feature = "scan", feature = "management", feature = "gateway"))]
    pub(crate) fn require_auth(&self, kind: AuthKind) -> Result<()> {
        let valid = match kind {
            #[cfg(feature = "scan")]
            AuthKind::Scan => matches!(&self.inner.auth, Authentication::Scan { .. }),
            #[cfg(feature = "management")]
            AuthKind::OAuth => matches!(&self.inner.auth, Authentication::OAuth { .. }),
            #[cfg(feature = "gateway")]
            AuthKind::Runtime => matches!(&self.inner.auth, Authentication::RuntimeKey { .. }),
        };
        if valid {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Configuration))
        }
    }
    #[cfg(any(feature = "scan", feature = "gateway"))]
    pub(crate) async fn send_model<T: Serialize, R: DeserializeOwned>(
        &self,
        request: RawRequest,
        body: &T,
        request_schema: &str,
        response_schema: &str,
        options: Option<&RequestOptions>,
    ) -> Result<R> {
        let options = options.cloned().map_or_else(
            || RequestOptions::with_timeout(self.inner.limits.operation_timeout),
            Ok,
        )?;
        let name = request.name;
        let request = options
            .run(&self.inner.session, async {
                let bytes = serialize_bounded(body, self.inner.limits.max_request_bytes)?;
                let value = serde_json::from_slice(&bytes)
                    .map_err(|_| Error::new(ErrorKind::Validation))?;
                let value = normalize_contract(
                    request_schema,
                    value,
                    crate::validation::Mode::Request,
                    &self.contract_limits(self.inner.limits.max_request_bytes)?,
                )?;
                request.json(&value, self.inner.limits.max_request_bytes)
            })
            .await
            .map_err(|e| e.with_operation(name))?;
        self.send_contract(request, response_schema, Some(&options))
            .await
    }
    #[cfg(any(feature = "scan", feature = "management", feature = "gateway"))]
    pub(crate) async fn send_contract<R: DeserializeOwned>(
        &self,
        request: RawRequest,
        schema: &str,
        options: Option<&RequestOptions>,
    ) -> Result<R> {
        let name = request.name;
        let options = options.cloned().map_or_else(
            || RequestOptions::with_timeout(self.inner.limits.operation_timeout),
            Ok,
        )?;
        let bytes = self.send_bytes(request, Some(&options)).await?;
        options
            .run(&self.inner.session, async {
                let value =
                    serde_json::from_slice(&bytes).map_err(|_| Error::new(ErrorKind::Decode))?;
                let value = normalize_contract(
                    schema,
                    value,
                    crate::validation::Mode::Response,
                    &self.contract_limits(self.inner.limits.max_response_bytes)?,
                )
                .map_err(|e| {
                    Error::new(if e.kind() == ErrorKind::Limit {
                        ErrorKind::Limit
                    } else {
                        ErrorKind::Decode
                    })
                })?;
                serde_json::from_value(value).map_err(|_| Error::new(ErrorKind::Decode))
            })
            .await
            .map_err(|e| e.with_operation(name).with_outcome(RemoteOutcome::Unknown))
    }
    #[cfg(feature = "gateway")]
    pub(crate) async fn send_model_stream<T: Serialize, R: DeserializeOwned>(
        &self,
        request: RawRequest,
        body: &T,
        request_schema: &str,
        stream_contract: (&'static str, crate::streaming::TerminalPolicy),
        options: Option<&RequestOptions>,
    ) -> Result<crate::streaming::ContractStream<R>> {
        let options = options.cloned().map_or_else(
            || RequestOptions::with_timeout(self.inner.limits.operation_timeout),
            Ok,
        )?;
        let name = request.name;
        let request = options
            .run(&self.inner.session, async {
                let bytes = serialize_bounded(body, self.inner.limits.max_request_bytes)?;
                let value = serde_json::from_slice(&bytes)
                    .map_err(|_| Error::new(ErrorKind::Validation))?;
                let value = normalize_contract(
                    request_schema,
                    value,
                    crate::validation::Mode::Request,
                    &self.contract_limits(self.inner.limits.max_request_bytes)?,
                )?;
                request.json(&value, self.inner.limits.max_request_bytes)
            })
            .await
            .map_err(|e| e.with_operation(name))?;
        let response = self.send_stream(request, Some(&options)).await?;
        let status = response.status().as_u16();
        let stream = crate::streaming::SseStream::new(
            response,
            self.inner.limits.max_event_bytes,
            stream_contract.1,
        )?;
        let contract = crate::streaming::EventContract {
            schema: stream_contract.0,
            operation: name,
            status,
            limits: self.contract_limits(self.inner.limits.max_event_bytes)?,
        };
        Ok(crate::streaming::ContractStream::new(
            stream,
            contract,
            options,
            self.inner.session.clone(),
        ))
    }
    fn preflight(&self, request: &RawRequest) -> Result<(url::Url, Bytes)> {
        let url = self
            .inner
            .endpoint
            .resolve(&request.path, &request.segments, &request.query)?;
        self.inner.endpoint.admit(&url)?;
        self.inner
            .policy
            .check(&url, DestinationPurpose::Api)
            .map_err(|_| Error::new(ErrorKind::Destination))?;
        let body = request.body.clone().unwrap_or_default();
        if body.len() > self.inner.limits.max_request_bytes || headers_exceed(&request.headers) {
            return Err(Error::new(ErrorKind::Limit).with_operation(request.name));
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
            return Err(Error::new(ErrorKind::Validation).with_operation(request.name));
        }
        Ok((url, body))
    }
    /// Start a caller-driven response body. Handshake includes queueing and OAuth;
    /// idle/session limits apply while reading. Streams never automatically replay.
    pub async fn send_stream(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<crate::streaming::ResponseStream> {
        let (url, body) = self.preflight(&request)?;
        let explicit_deadline = options.is_some();
        let options = match options {
            Some(v) => v.clone(),
            None => RequestOptions::with_timeout(self.inner.limits.stream_handshake_timeout)?,
        };
        let name = request.name;
        let sent = AtomicBool::new(false);
        options
            .run(&self.inner.session, async {
                let permit = Arc::clone(&self.inner.permits)
                    .acquire_owned()
                    .await
                    .map_err(|_| Error::new(ErrorKind::Cancelled))?;
                let mut headers = request.headers;
                let generation = self
                    .inner
                    .auth
                    .prepare(&mut headers, &body, request.body.is_some(), &options)
                    .await?;
                if headers_exceed(&headers) {
                    return Err(Error::new(ErrorKind::Limit));
                }
                sent.store(true, Ordering::Relaxed);
                let mut response = self
                    .inner
                    .transport
                    .send(TransportRequest::new(
                        request.method,
                        url,
                        headers,
                        body,
                        name,
                    ))
                    .await?;
                let status = response.status;
                if status.is_redirection() {
                    return Err(Error::new(ErrorKind::Destination)
                        .with_status(status.as_u16())
                        .with_outcome(RemoteOutcome::Rejected));
                }
                if !status.is_success() {
                    let bytes = read_limited(&mut response.body, self.inner.limits.max_error_bytes)
                        .await
                        .map_err(|e| e.with_status(status.as_u16()))?;
                    if auth_rejection(status, &response.headers)
                        && let Some(generation) = generation
                    {
                        self.inner.auth.on_unauthorized(generation);
                    }
                    let (id, code) = error_metadata(&response.headers, Some(&bytes));
                    return Err(Error::new(ErrorKind::Http)
                        .with_status(status.as_u16())
                        .with_metadata(id, code, retry_after(&response.headers))
                        .with_outcome(if status.is_client_error() {
                            RemoteOutcome::Rejected
                        } else {
                            RemoteOutcome::Unknown
                        }));
                }
                Ok(crate::streaming::ResponseStream::new(
                    response,
                    permit,
                    self.inner.session.clone(),
                    &options,
                    explicit_deadline,
                    &self.inner.limits,
                    name,
                ))
            })
            .await
            .map_err(|error| {
                let outcome = if sent.load(Ordering::Relaxed)
                    && error.remote_outcome() == RemoteOutcome::NotSent
                {
                    RemoteOutcome::Unknown
                } else {
                    error.remote_outcome()
                };
                error.with_operation(name).with_outcome(outcome)
            })
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
        let (url, body) = self.preflight(&request)?;
        let sent = AtomicBool::new(false);
        let max_retries = options
            .retry_limit()
            .unwrap_or(self.inner.limits.max_retries);
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
                if headers_exceed(&headers) {
                    return Err(Error::new(ErrorKind::Limit));
                }
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
                            && retries < max_retries
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
                if auth_rejection(status, &response.headers)
                    && let Some(generation) = generation
                {
                    let invalidated = self.inner.auth.on_unauthorized(generation);
                    if request.replay_safe && !refreshed && invalidated && retries < max_retries {
                        refreshed = true;
                        retries += 1;
                        continue;
                    }
                }
                if request.replay_safe && retryable(status) && retries < max_retries {
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
fn auth_rejection(status: StatusCode, headers: &HeaderMap) -> bool {
    status == StatusCode::UNAUTHORIZED
        || (status == StatusCode::FORBIDDEN
            && headers.get("x-opa-decision").is_none_or(|v| v != "false"))
}
#[cfg(any(feature = "scan", feature = "management", feature = "gateway"))]
fn normalize_contract(
    schema: &str,
    value: serde_json::Value,
    mode: crate::validation::Mode,
    limits: &crate::validation::ValidationLimits,
) -> Result<serde_json::Value> {
    if let Some(element) = schema.strip_suffix("[]") {
        crate::validation::parse_array_with_limits(element, value, mode, limits)
    } else {
        crate::validation::parse_with_limits(schema, value, mode, limits)
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
    // Stop recursive model/extension serialization before the native validator
    // receives a JSON value. The hard ceiling matches the validator's maximum.
    struct DepthFormatter<'a> {
        depth: usize,
        exceeded: &'a std::cell::Cell<bool>,
    }
    impl serde_json::ser::Formatter for DepthFormatter<'_> {
        fn begin_array<W: Write + ?Sized>(&mut self, writer: &mut W) -> std::io::Result<()> {
            self.depth += 1;
            if self.depth > 128 {
                self.exceeded.set(true);
                return Err(std::io::Error::other("JSON depth limit"));
            }
            writer.write_all(b"[")
        }
        fn end_array<W: Write + ?Sized>(&mut self, writer: &mut W) -> std::io::Result<()> {
            self.depth -= 1;
            writer.write_all(b"]")
        }
        fn begin_object<W: Write + ?Sized>(&mut self, writer: &mut W) -> std::io::Result<()> {
            self.depth += 1;
            if self.depth > 128 {
                self.exceeded.set(true);
                return Err(std::io::Error::other("JSON depth limit"));
            }
            writer.write_all(b"{")
        }
        fn end_object<W: Write + ?Sized>(&mut self, writer: &mut W) -> std::io::Result<()> {
            self.depth -= 1;
            writer.write_all(b"}")
        }
    }
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
    let depth_exceeded = std::cell::Cell::new(false);
    let failed = {
        let mut serializer = serde_json::Serializer::with_formatter(
            &mut writer,
            DepthFormatter {
                depth: 0,
                exceeded: &depth_exceeded,
            },
        );
        value.serialize(&mut serializer).is_err()
    };
    if failed {
        return Err(Error::new(if writer.exceeded || depth_exceeded.get() {
            ErrorKind::Limit
        } else {
            ErrorKind::Validation
        }));
    }
    Ok(Bytes::from(writer.value))
}

#[cfg(test)]
mod tests;
