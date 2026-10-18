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
    runtime_retries: Option<u32>,
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
            runtime_retries: None,
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
    pub fn segment(mut self, value: impl AsRef<str>) -> Result<Self> {
        let value = value.as_ref();
        crate::config::encode_segment(value)?;
        self.segments.push(value.to_owned());
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
    #[cfg(feature = "gateway")]
    pub(crate) fn runtime_replay(&mut self, retries: Option<usize>) {
        self.replay_safe = false;
        self.runtime_retries = retries.map(|value| value as u32);
    }
    #[cfg(feature = "gateway")]
    pub(crate) fn routing_headers(&mut self, headers: &HeaderMap) -> Result<()> {
        if headers.keys().any(|name| self.headers.contains_key(name)) {
            return Err(Error::new(ErrorKind::Validation));
        }
        self.headers.extend(headers.clone());
        if headers_exceed(&self.headers) {
            return Err(Error::new(ErrorKind::Limit));
        }
        Ok(())
    }
    #[cfg(feature = "dlp")]
    pub(crate) fn content_type(mut self, value: &str) -> Result<Self> {
        value
            .parse::<mime::Mime>()
            .map_err(|_| Error::new(ErrorKind::Validation))?;
        self.headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_str(value).map_err(|_| Error::new(ErrorKind::Validation))?,
        );
        Ok(self)
    }
    pub(crate) fn json<T: Serialize>(self, value: &T, limit: usize) -> Result<Self> {
        let content_type = self
            .headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|value| value.to_str().ok())
            .unwrap_or("application/json")
            .to_owned();
        self.body(serialize_bounded(value, limit)?, &content_type)
    }
}
fn runtime_retryable(status: StatusCode) -> bool {
    matches!(status.as_u16(), 500 | 502 | 503 | 504)
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
struct BufferedResponse {
    body: Bytes,
    status: u16,
    request_id: Option<String>,
}
impl BufferedResponse {
    fn annotate(&self, error: Error) -> Error {
        let error = error
            .with_status(self.status)
            .with_metadata(self.request_id.clone(), None, None)
            .with_outcome(RemoteOutcome::Unknown);
        crate::observability::failure(crate::observability::Operation::Decode, &error);
        error
    }
}
#[cfg(any(
    feature = "scan",
    feature = "management",
    feature = "gateway",
    feature = "dlp",
    feature = "iam",
    feature = "model-security",
    feature = "red-team",
    feature = "agentguard"
))]
pub(crate) enum AuthKind {
    #[cfg(feature = "scan")]
    Scan,
    #[cfg(any(
        feature = "management",
        feature = "gateway",
        feature = "dlp",
        feature = "iam",
        feature = "model-security",
        feature = "red-team",
        feature = "agentguard"
    ))]
    OAuth,
    #[cfg(feature = "gateway")]
    Runtime,
    #[cfg(feature = "gateway")]
    Public,
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
    oauth_options: crate::auth::OAuthProviderOptions,
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
            oauth_options: crate::auth::OAuthProviderOptions::default(),
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
    /// Configure this client's immutable managed OAuth refresh policy and events.
    pub fn oauth_provider_options(mut self, options: crate::auth::OAuthProviderOptions) -> Self {
        self.oauth_options = options;
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
                let provider = OAuthProvider::new_with_options(
                    context,
                    secret,
                    Arc::clone(&transport),
                    Arc::clone(&policy),
                    self.session.clone(),
                    self.oauth_options,
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
    #[cfg(any(
        feature = "scan",
        feature = "management",
        feature = "gateway",
        feature = "dlp",
        feature = "iam",
        feature = "model-security",
        feature = "red-team",
        feature = "agentguard"
    ))]
    pub(crate) fn contract_limits(
        &self,
        max_bytes: usize,
    ) -> Result<crate::validation::ValidationLimits> {
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
    /// Immutable OAuth identity, when this client uses a service-token provider.
    pub fn oauth_context(&self) -> Option<&AuthContext> {
        match &self.inner.auth {
            Authentication::OAuth { provider, .. } => Some(provider.context()),
            _ => None,
        }
    }
    /// Select an additional explicit endpoint without broadening destination admission.
    /// The new client shares authentication, transport, limits, cancellation, and permits.
    pub fn with_endpoint(&self, endpoint: Endpoint) -> Result<Self> {
        self.inner
            .policy
            .check(endpoint.url(), DestinationPurpose::Api)
            .map_err(|_| Error::new(ErrorKind::Destination))?;
        Ok(self.scoped(endpoint, self.inner.auth.clone()))
    }
    /// Bind or omit the tenant routing header using the provider's immutable identity.
    /// Product clients that require tenant routing enable it in their constructor.
    pub fn with_tsg_header(&self, enabled: bool) -> Result<Self> {
        let Authentication::OAuth { provider, .. } = &self.inner.auth else {
            return Err(Error::new(ErrorKind::Configuration));
        };
        Ok(self.scoped(
            self.inner.endpoint.clone(),
            Authentication::oauth(Arc::clone(provider), enabled),
        ))
    }
    fn scoped(&self, endpoint: Endpoint, auth: Authentication) -> Self {
        Self {
            inner: Arc::new(Inner {
                endpoint,
                auth,
                transport: Arc::clone(&self.inner.transport),
                policy: Arc::clone(&self.inner.policy),
                limits: self.inner.limits.clone(),
                session: self.inner.session.clone(),
                permits: Arc::clone(&self.inner.permits),
                validation: self.inner.validation.clone(),
            }),
        }
    }
    pub(crate) fn operation_options(
        &self,
        options: Option<&RequestOptions>,
    ) -> Result<RequestOptions> {
        options.cloned().map_or_else(
            || RequestOptions::with_timeout(self.inner.limits.operation_timeout),
            Ok,
        )
    }
    #[cfg(any(
        feature = "scan",
        feature = "management",
        feature = "gateway",
        feature = "dlp",
        feature = "iam",
        feature = "model-security",
        feature = "red-team",
        feature = "agentguard"
    ))]
    pub(crate) fn require_auth(&self, kind: AuthKind) -> Result<()> {
        let valid = match kind {
            #[cfg(feature = "scan")]
            AuthKind::Scan => matches!(&self.inner.auth, Authentication::Scan { .. }),
            #[cfg(any(
                feature = "management",
                feature = "gateway",
                feature = "dlp",
                feature = "iam",
                feature = "model-security",
                feature = "red-team",
                feature = "agentguard"
            ))]
            AuthKind::OAuth => matches!(&self.inner.auth, Authentication::OAuth { .. }),
            #[cfg(feature = "gateway")]
            AuthKind::Runtime => matches!(&self.inner.auth, Authentication::RuntimeKey { .. }),
            #[cfg(feature = "gateway")]
            AuthKind::Public => matches!(&self.inner.auth, Authentication::None),
        };
        if valid {
            Ok(())
        } else {
            Err(Error::new(ErrorKind::Configuration))
        }
    }
    #[cfg(any(
        feature = "scan",
        feature = "management",
        feature = "gateway",
        feature = "dlp",
        feature = "iam",
        feature = "model-security",
        feature = "red-team",
        feature = "agentguard"
    ))]
    pub(crate) async fn validated_json<T: Serialize>(
        &self,
        body: &T,
        schema: &str,
        options: &RequestOptions,
    ) -> Result<Bytes> {
        options
            .run(&self.inner.session, async {
                let bytes = serialize_bounded(body, self.inner.limits.max_request_bytes)?;
                let value = crate::json_decode::from_slice(&bytes)
                    .map_err(|_| Error::new(ErrorKind::Validation))?;
                let value = normalize_contract(
                    schema,
                    value,
                    crate::validation::Mode::Request,
                    &self.contract_limits(self.inner.limits.max_request_bytes)?,
                )?;
                serialize_bounded(&value, self.inner.limits.max_request_bytes)
            })
            .await
    }
    #[cfg(any(
        feature = "scan",
        feature = "management",
        feature = "gateway",
        feature = "dlp",
        feature = "iam",
        feature = "model-security",
        feature = "red-team"
    ))]
    async fn prepare_model<T: Serialize>(
        &self,
        request: RawRequest,
        body: &T,
        schema: &str,
        options: &RequestOptions,
    ) -> Result<RawRequest> {
        let name = request.name;
        let bytes = self
            .validated_json(body, schema, options)
            .await
            .map_err(|e| e.with_operation(name))?;
        let content_type = request
            .headers
            .get(http::header::CONTENT_TYPE)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("application/json")
            .to_owned();
        request.body(bytes, &content_type)
    }
    #[cfg(any(
        feature = "scan",
        feature = "management",
        feature = "gateway",
        feature = "dlp",
        feature = "iam",
        feature = "model-security",
        feature = "red-team"
    ))]
    pub(crate) async fn send_model<T: Serialize, R: DeserializeOwned>(
        &self,
        request: RawRequest,
        body: &T,
        request_schema: &str,
        response_schema: &str,
        options: Option<&RequestOptions>,
    ) -> Result<R> {
        let options = self.operation_options(options)?;
        let request = self
            .prepare_model(request, body, request_schema, &options)
            .await?;
        self.send_contract(request, response_schema, Some(&options))
            .await
    }
    #[cfg(any(
        feature = "scan",
        feature = "management",
        feature = "gateway",
        feature = "dlp",
        feature = "iam",
        feature = "model-security",
        feature = "red-team",
        feature = "agentguard"
    ))]
    pub(crate) async fn send_contract<R: DeserializeOwned>(
        &self,
        request: RawRequest,
        schema: &str,
        options: Option<&RequestOptions>,
    ) -> Result<R> {
        let name = request.name;
        let options = self.operation_options(options)?;
        let response = self.send_buffered(request, Some(&options)).await?;
        self.decode_contract(&response, schema, &options)
            .await
            .map_err(|e| response.annotate(e).with_operation(name))
    }
    #[cfg(feature = "gateway")]
    pub(crate) async fn send_contract_one_of<R: DeserializeOwned>(
        &self,
        request: RawRequest,
        schemas: &[&str],
        options: Option<&RequestOptions>,
    ) -> Result<R> {
        let name = request.name;
        let options = self.operation_options(options)?;
        let response = self.send_buffered(request, Some(&options)).await?;
        options
            .run(&self.inner.session, async {
                let body = if response.body.is_empty() {
                    b"{}".as_slice()
                } else {
                    &response.body
                };
                let value = crate::json_decode::from_slice(body)
                    .map_err(|_| Error::new(ErrorKind::Decode))?;
                let value = crate::validation::parse_one_of_with_limits(
                    schemas,
                    value,
                    options.response_mode(),
                    &self.contract_limits(self.inner.limits.max_response_bytes)?,
                )
                .map_err(|error| {
                    Error::new(if error.kind() == ErrorKind::Limit {
                        ErrorKind::Limit
                    } else {
                        ErrorKind::Decode
                    })
                })?;
                crate::json_decode::from_value(value).map_err(|_| Error::new(ErrorKind::Decode))
            })
            .await
            .map_err(|error| response.annotate(error).with_operation(name))
    }
    #[cfg(feature = "red-team")]
    pub(crate) async fn send_model_optional<T: Serialize, R: DeserializeOwned>(
        &self,
        request: RawRequest,
        body: &T,
        request_schema: &str,
        response_schema: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Option<R>> {
        let options = self.operation_options(options)?;
        let request = self
            .prepare_model(request, body, request_schema, &options)
            .await?;
        self.send_optional_contract(request, response_schema, Some(&options))
            .await
    }
    #[cfg(any(feature = "dlp", feature = "red-team"))]
    pub(crate) async fn send_optional_contract<R: DeserializeOwned>(
        &self,
        request: RawRequest,
        schema: &str,
        options: Option<&RequestOptions>,
    ) -> Result<Option<R>> {
        let name = request.name;
        let options = self.operation_options(options)?;
        let response = self.send_buffered(request, Some(&options)).await?;
        if response.body.is_empty() {
            return Ok(None);
        }
        self.decode_contract(&response, schema, &options)
            .await
            .map(Some)
            .map_err(|e| response.annotate(e).with_operation(name))
    }
    #[cfg(any(
        feature = "scan",
        feature = "management",
        feature = "gateway",
        feature = "dlp",
        feature = "iam",
        feature = "model-security",
        feature = "red-team",
        feature = "agentguard"
    ))]
    async fn decode_contract<R: DeserializeOwned>(
        &self,
        response: &BufferedResponse,
        schema: &str,
        options: &RequestOptions,
    ) -> Result<R> {
        options
            .run(&self.inner.session, async {
                // The frozen request engine hydrates empty JSON successes as {}.
                let body = if response.body.is_empty() {
                    b"{}".as_slice()
                } else {
                    response.body.as_ref()
                };
                let value = crate::json_decode::from_slice(body)
                    .map_err(|_| Error::new(ErrorKind::Decode))?;
                let value = normalize_contract(
                    schema,
                    value,
                    options.response_mode(),
                    &self.contract_limits(self.inner.limits.max_response_bytes)?,
                )
                .map_err(|e| {
                    Error::new(if e.kind() == ErrorKind::Limit {
                        ErrorKind::Limit
                    } else {
                        ErrorKind::Decode
                    })
                })?;
                crate::json_decode::from_value(value).map_err(|_| Error::new(ErrorKind::Decode))
            })
            .await
    }
    #[cfg(feature = "gateway")]
    pub(crate) async fn send_contract_stream<R: DeserializeOwned>(
        &self,
        request: RawRequest,
        schema: &'static str,
        terminal: crate::streaming::TerminalPolicy,
        options: Option<&RequestOptions>,
    ) -> Result<crate::streaming::ContractStream<R>> {
        let options = self.operation_options(options)?;
        let name = request.name;
        let response = self.send_stream(request, Some(&options)).await?;
        let status = response.status().as_u16();
        let request_id = response.request_id().map(str::to_owned);
        let stream = crate::streaming::SseStream::new(
            response,
            self.inner.limits.max_event_bytes,
            terminal,
        )?;
        let contract = crate::streaming::EventContract {
            schema,
            operation: name,
            status,
            request_id,
            limits: self.contract_limits(self.inner.limits.max_event_bytes)?,
        };
        Ok(crate::streaming::ContractStream::new(
            stream,
            contract,
            options,
            self.inner.session.clone(),
        ))
    }
    #[cfg(feature = "realtime")]
    pub(crate) async fn open_realtime(
        &self,
        request: RawRequest,
        settings: &crate::realtime::GatewayRealtimeOptions,
        options: &RequestOptions,
    ) -> Result<crate::realtime::GatewayRealtimeConnection> {
        let lifecycle = crate::observability::Lifecycle::start(
            crate::observability::Operation::RealtimeUpgrade,
        );
        let result = self
            .open_realtime_observed(request, settings, options, &lifecycle)
            .await;
        lifecycle.finish(&result);
        result
    }
    #[cfg(feature = "realtime")]
    async fn open_realtime_observed(
        &self,
        mut request: RawRequest,
        settings: &crate::realtime::GatewayRealtimeOptions,
        options: &RequestOptions,
        lifecycle: &crate::observability::Lifecycle,
    ) -> Result<crate::realtime::GatewayRealtimeConnection> {
        use crate::realtime::{GatewayRealtimeConnection, OPERATION, WebSocketRequest};
        self.require_auth(AuthKind::Runtime)?;
        settings.validate()?;
        let mut settings = settings.clone();
        settings.max_event_bytes = settings
            .max_event_bytes
            .min(self.inner.limits.max_event_bytes)
            .min(settings.max_buffered_bytes);
        settings.idle_timeout = settings
            .idle_timeout
            .min(self.inner.limits.stream_idle_timeout);
        let validation = self.contract_limits(settings.max_event_bytes)?;
        // The frozen WebSocket path bypasses common HTTP API headers.
        request.headers.remove(http::header::USER_AGENT);
        request.headers.remove("service-name");
        let (mut url, body) = self.preflight(&request)?;
        if request.method != Method::GET || !body.is_empty() {
            return Err(Error::new(ErrorKind::Validation));
        }
        let scheme = match url.scheme() {
            "https" => "wss",
            "http" => "ws",
            _ => return Err(Error::new(ErrorKind::Destination)),
        };
        url.set_scheme(scheme)
            .map_err(|_| Error::new(ErrorKind::Destination))?;
        self.inner
            .policy
            .check(&url, DestinationPurpose::Realtime)
            .map_err(|_| Error::new(ErrorKind::Destination))?;
        let sent = AtomicBool::new(false);
        let deadline = options.deadline().min(
            tokio::time::Instant::now()
                .checked_add(settings.handshake_timeout)
                .ok_or_else(|| Error::new(ErrorKind::Configuration))?,
        );
        let result = options
            .run(&self.inner.session, async {
                tokio::time::timeout_at(deadline, async {
                    let permit = Arc::clone(&self.inner.permits)
                        .acquire_owned()
                        .await
                        .map_err(|_| Error::new(ErrorKind::Cancelled))?;
                    let mut headers = request.headers;
                    self.inner
                        .auth
                        .prepare(&mut headers, &body, false, options)
                        .await?;
                    if headers_exceed(&headers) {
                        return Err(Error::new(ErrorKind::Limit));
                    }
                    for value in headers.values_mut() {
                        value.set_sensitive(true);
                    }
                    options.run(&self.inner.session, async { Ok(()) }).await?;
                    lifecycle.dispatched();
                    sent.store(true, Ordering::Relaxed);
                    let upgrade = settings
                        .connector
                        .connect(WebSocketRequest {
                            url,
                            headers,
                            handshake_timeout: deadline
                                .saturating_duration_since(tokio::time::Instant::now()),
                            max_event_bytes: settings.max_event_bytes,
                        })
                        .await?;
                    lifecycle.response(
                        upgrade.status.as_u16(),
                        error_metadata(&upgrade.headers, None).0.is_some(),
                    );
                    if upgrade.status != StatusCode::SWITCHING_PROTOCOLS {
                        return Err(Error::new(if upgrade.status.is_redirection() {
                            ErrorKind::Destination
                        } else {
                            ErrorKind::Http
                        })
                        .with_status(upgrade.status.as_u16())
                        .with_metadata(
                            error_metadata(&upgrade.headers, None).0,
                            None,
                            retry_after(&upgrade.headers),
                        )
                        .with_failure_kind(crate::FailureKind::Http)
                        .with_outcome(
                            if upgrade.status.is_client_error() || upgrade.status.is_redirection() {
                                RemoteOutcome::Rejected
                            } else {
                                RemoteOutcome::Unknown
                            },
                        ));
                    }
                    Ok(GatewayRealtimeConnection::new(
                        upgrade,
                        permit,
                        options.clone(),
                        self.inner.session.clone(),
                        settings,
                        validation,
                    ))
                })
                .await
                .unwrap_or_else(|_| Err(Error::new(ErrorKind::Deadline)))
            })
            .await;
        result.map_err(|error| {
            let outcome = if sent.load(Ordering::Relaxed)
                && error.remote_outcome() == RemoteOutcome::NotSent
            {
                RemoteOutcome::Unknown
            } else {
                error.remote_outcome()
            };
            error.with_operation(OPERATION).with_outcome(outcome)
        })
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
        if request.runtime_retries.is_some()
            && !matches!(&self.inner.auth, Authentication::RuntimeKey { .. })
        {
            return Err(Error::new(ErrorKind::Configuration));
        }
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
            && crate::json_decode::from_slice(&body).is_err()
        {
            return Err(Error::new(ErrorKind::Validation).with_operation(request.name));
        }
        Ok((url, body))
    }
    /// Start a caller-driven response body. Handshake includes queueing and OAuth;
    /// idle/session limits apply while reading. Only explicit runtime replay may
    /// retry a failed handshake; successful headers permanently end retry eligibility.
    pub async fn send_stream(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<crate::streaming::ResponseStream> {
        let lifecycle =
            crate::observability::Lifecycle::start(crate::observability::Operation::HttpStream);
        let result = self
            .send_stream_observed(request, options, &lifecycle)
            .await;
        lifecycle.finish(&result);
        result
    }
    async fn send_stream_observed(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
        lifecycle: &crate::observability::Lifecycle,
    ) -> Result<crate::streaming::ResponseStream> {
        let (url, body) = self.preflight(&request)?;
        let explicit_deadline = options.is_some();
        let options = match options {
            Some(v) => v.clone(),
            None => RequestOptions::with_timeout(self.inner.limits.stream_handshake_timeout)?,
        };
        let name = request.name;
        let sent = AtomicBool::new(false);
        let received = std::sync::Mutex::new(None::<(u16, Option<String>)>);
        options
            .run(&self.inner.session, async {
                let permit = Arc::clone(&self.inner.permits)
                    .acquire_owned()
                    .await
                    .map_err(|_| Error::new(ErrorKind::Cancelled))?;
                let max_retries = request.runtime_retries.unwrap_or(0);
                let mut retries = 0;
                loop {
                    if let Ok(mut slot) = received.lock() {
                        *slot = None;
                    }
                    let mut headers = request.headers.clone();
                    let generation = self
                        .inner
                        .auth
                        .prepare(&mut headers, &body, request.body.is_some(), &options)
                        .await?;
                    if headers_exceed(&headers) {
                        return Err(Error::new(ErrorKind::Limit));
                    }
                    options.run(&self.inner.session, async { Ok(()) }).await?;
                    lifecycle.dispatched();
                    sent.store(true, Ordering::Relaxed);
                    let mut response = match self
                        .inner
                        .transport
                        .send(TransportRequest::new(
                            request.method.clone(),
                            url.clone(),
                            headers,
                            body.clone(),
                            name,
                        ))
                        .await
                    {
                        Ok(response) => response,
                        Err(error) => {
                            if error.kind() == ErrorKind::Transport && retries < max_retries {
                                let delay = self.backoff(retries);
                                retries += 1;
                                tokio::time::sleep(delay).await;
                                continue;
                            }
                            return Err(error.with_failure_kind(crate::FailureKind::Network));
                        }
                    };
                    let status = response.status;
                    let request_id = error_metadata(&response.headers, None).0;
                    lifecycle.response(status.as_u16(), request_id.is_some());
                    if let Ok(mut slot) = received.lock() {
                        *slot = Some((status.as_u16(), request_id));
                    }
                    if status.is_redirection() {
                        return Err(Error::new(ErrorKind::Destination)
                            .with_status(status.as_u16())
                            .with_outcome(RemoteOutcome::Rejected));
                    }
                    if !status.is_success() {
                        let bytes =
                            read_limited(&mut response.body, self.inner.limits.max_error_bytes)
                                .await
                                .map_err(body_failure)?;
                        if auth_rejection(status, &response.headers)
                            && let Some(generation) = generation
                        {
                            self.inner.auth.on_unauthorized(generation);
                        }
                        let delay = retry_after(&response.headers);
                        if runtime_retryable(status) && retries < max_retries {
                            let wait = delay.unwrap_or_else(|| self.backoff(retries));
                            if wait <= self.inner.limits.max_retry_delay {
                                retries += 1;
                                tokio::time::sleep(wait).await;
                                continue;
                            }
                        }
                        let (id, code) = error_metadata(&response.headers, Some(&bytes));
                        return Err(Error::new(ErrorKind::Http)
                            .with_status(status.as_u16())
                            .with_metadata(id, code, delay)
                            .with_http_body(&bytes)
                            .with_outcome(if status.is_client_error() {
                                RemoteOutcome::Rejected
                            } else {
                                RemoteOutcome::Unknown
                            }));
                    }
                    // Returning successful headers ends the retry scope permanently.
                    return Ok(crate::streaming::ResponseStream::new(
                        response,
                        permit,
                        self.inner.session.clone(),
                        &options,
                        explicit_deadline,
                        &self.inner.limits,
                        name,
                    ));
                }
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
                let error = if let Some((status, request_id)) =
                    received.lock().ok().and_then(|s| s.clone())
                {
                    error.with_response_metadata(status, request_id)
                } else {
                    error
                };
                error.with_operation(name).with_outcome(outcome)
            })
    }

    pub async fn send_bytes(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<Bytes> {
        self.send_buffered(request, options)
            .await
            .map(|response| response.body)
    }
    async fn send_buffered(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<BufferedResponse> {
        let lifecycle =
            crate::observability::Lifecycle::start(crate::observability::Operation::Http);
        let result = self
            .send_buffered_observed(request, options, &lifecycle)
            .await;
        lifecycle.finish(&result);
        result
    }
    async fn send_buffered_observed(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
        lifecycle: &crate::observability::Lifecycle,
    ) -> Result<BufferedResponse> {
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
        let received = std::sync::Mutex::new(None::<(u16, Option<String>)>);
        let max_retries = request.runtime_retries.unwrap_or_else(|| {
            options
                .retry_limit()
                .unwrap_or(self.inner.limits.max_retries)
        });
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
                if let Ok(mut slot) = received.lock() {
                    *slot = None;
                }
                let mut headers = request.headers.clone();
                let generation = self
                    .inner
                    .auth
                    .prepare(&mut headers, &body, request.body.is_some(), options)
                    .await?;
                if headers_exceed(&headers) {
                    return Err(Error::new(ErrorKind::Limit));
                }
                options.run(&self.inner.session, async { Ok(()) }).await?;
                lifecycle.dispatched();
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
                            && (request.replay_safe || request.runtime_retries.is_some())
                            && retries < max_retries
                        {
                            let delay = self.backoff(retries);
                            retries += 1;
                            tokio::time::sleep(delay).await;
                            continue;
                        }
                        return Err(error.with_failure_kind(crate::FailureKind::Network));
                    }
                };
                let status = response.status;
                let request_id = error_metadata(&response.headers, None).0;
                lifecycle.response(status.as_u16(), request_id.is_some());
                if let Ok(mut slot) = received.lock() {
                    *slot = Some((status.as_u16(), request_id.clone()));
                }
                if status.is_redirection() {
                    return Err(Error::new(ErrorKind::Destination)
                        .with_status(status.as_u16())
                        .with_outcome(RemoteOutcome::Rejected));
                }
                let delay = retry_after(&response.headers);
                if status.is_success() {
                    let body =
                        read_limited(&mut response.body, self.inner.limits.max_response_bytes)
                            .await
                            .map_err(body_failure)?;
                    return Ok(BufferedResponse {
                        body,
                        status: status.as_u16(),
                        request_id,
                    });
                }
                // Read only a bounded amount; never retain or format arbitrary upstream bodies.
                let error_body =
                    read_limited(&mut response.body, self.inner.limits.max_error_bytes)
                        .await
                        .map_err(|e| body_failure(e).with_status(status.as_u16()))?;
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
                if (request.replay_safe && retryable(status)
                    || request.runtime_retries.is_some() && runtime_retryable(status))
                    && retries < max_retries
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
                    .with_http_body(&error_body)
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
            let e = if let Some((status, request_id)) =
                received.lock().ok().and_then(|slot| slot.clone())
            {
                e.with_response_metadata(status, request_id)
            } else {
                e
            };
            e.with_operation(name).with_outcome(outcome)
        })
    }

    /// Read a bounded text response using UTF-8 replacement decoding, as Fetch does.
    /// A leading UTF-8 BOM is omitted; content-type does not select another encoding.
    pub async fn send_text(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<String> {
        let name = request.name;
        let options = self.operation_options(options)?;
        let response = self.send_buffered(request, Some(&options)).await?;
        options
            .run(&self.inner.session, async {
                let body = response
                    .body
                    .strip_prefix(&[0xef, 0xbb, 0xbf])
                    .unwrap_or(&response.body);
                Ok(String::from_utf8_lossy(body).into_owned())
            })
            .await
            .map_err(|e| response.annotate(e).with_operation(name))
    }
    pub async fn send_json<R: DeserializeOwned>(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<R> {
        let name = request.name;
        let options = self.operation_options(options)?;
        let response = self.send_buffered(request, Some(&options)).await?;
        options
            .run(&self.inner.session, async {
                crate::json_decode::from_slice(if response.body.is_empty() {
                    b"{}"
                } else {
                    &response.body
                })
                .and_then(crate::json_decode::from_value)
                .map_err(|_| Error::new(ErrorKind::Decode))
            })
            .await
            .map_err(|e| response.annotate(e).with_operation(name))
    }
    pub async fn send_optional_json<R: DeserializeOwned>(
        &self,
        request: RawRequest,
        options: Option<&RequestOptions>,
    ) -> Result<Option<R>> {
        let name = request.name;
        let options = self.operation_options(options)?;
        let response = self.send_buffered(request, Some(&options)).await?;
        if response.body.is_empty() {
            return Ok(None);
        }
        options
            .run(&self.inner.session, async {
                crate::json_decode::from_slice(&response.body)
                    .and_then(crate::json_decode::from_value)
                    .map(Some)
                    .map_err(|_| Error::new(ErrorKind::Decode))
            })
            .await
            .map_err(|e| response.annotate(e).with_operation(name))
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
#[cfg(any(
    feature = "scan",
    feature = "management",
    feature = "gateway",
    feature = "dlp",
    feature = "iam",
    feature = "model-security",
    feature = "red-team",
    feature = "agentguard"
))]
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
    let value = headers
        .get(http::header::RETRY_AFTER)?
        .to_str()
        .ok()?
        .trim();
    if !value.is_empty()
        && value.bytes().all(|byte| byte.is_ascii_digit())
        && let Ok(seconds) = value.parse::<u64>()
    {
        return Some(Duration::from_secs(seconds));
    }
    Some(
        httpdate::parse_http_date(value)
            .ok()?
            .duration_since(SystemTime::now())
            .unwrap_or(Duration::ZERO),
    )
}
pub(crate) fn body_failure(error: Error) -> Error {
    let kind = if matches!(error.kind(), ErrorKind::Transport | ErrorKind::Io) {
        crate::FailureKind::Network
    } else {
        crate::FailureKind::Http
    };
    error.with_failure_kind(kind)
}
pub(crate) fn error_metadata(
    headers: &HeaderMap,
    body: Option<&[u8]>,
) -> (Option<String>, Option<String>) {
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
        .and_then(|v| crate::json_decode::from_slice(v).ok())
        .and_then(|v| {
            v.get("code")
                .or_else(|| v.get("error").and_then(|v| v.get("code")))
                .or_else(|| v.get("data").and_then(|v| v.get("errorCode")))
                .or_else(|| v.get("errorCode"))
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
