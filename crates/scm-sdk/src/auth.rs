//! Explicit authentication and isolated, caller-driven OAuth refresh.
//!
//! A provider owns one immutable issuer/client/credential-generation/tenant/scope
//! identity. Refresh has no detached task: concurrent callers share its future and
//! the transport future drops when its last waiter drops. Ordinary formatting
//! exposes no credential, tenant identifier, scope, audience, or issuer URL.

use std::{
    fmt,
    sync::{Arc, Mutex},
    time::Duration,
};

use base64::{Engine as _, engine::general_purpose::STANDARD};
use bytes::Bytes;
use futures_util::{
    FutureExt,
    future::{BoxFuture, Shared, WeakShared},
};
use hmac::{Hmac, Mac};
use http::{
    HeaderMap, HeaderValue, Method,
    header::{AUTHORIZATION, CONTENT_TYPE},
};
use serde::Deserialize;
use sha2::Sha256;
use tokio::time::Instant;
use zeroize::Zeroizing;

use crate::{
    Error, ErrorKind, Result, Secret,
    config::{DestinationPolicy, DestinationPurpose, Endpoint, RequestOptions, Session},
    transport::{Transport, TransportRequest, read_limited},
};

const MAX_TOKEN_RESPONSE: usize = 64 * 1024;
const MAX_REFRESH_TIME: Duration = Duration::from_secs(30);
const MAX_TOKEN_LIFETIME: Duration = Duration::from_secs(31 * 24 * 60 * 60);

/// Immutable identity used to obtain a service token. Formatting is redacted.
#[derive(Clone)]
pub struct AuthContext {
    issuer: Endpoint,
    client_id: String,
    credential_generation: u64,
    tsg_id: String,
    scopes: Vec<String>,
    audience: Option<String>,
}

impl AuthContext {
    /// Default scope is exactly `tsg_id:<tenant>`. Each provider isolates its cache.
    pub fn new(
        issuer: Endpoint,
        client_id: impl Into<String>,
        credential_generation: u64,
        tsg_id: impl Into<String>,
    ) -> Result<Self> {
        let client_id = client_id.into();
        let tsg_id = tsg_id.into();
        if client_id.is_empty()
            || client_id.len() > 1024
            || client_id.chars().any(char::is_control)
            || tsg_id.is_empty()
            || tsg_id.len() > 32
            || !tsg_id.bytes().all(|value| value.is_ascii_digit())
        {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let scopes = vec![format!("tsg_id:{tsg_id}")];
        Ok(Self {
            issuer,
            client_id,
            credential_generation,
            tsg_id,
            scopes,
            audience: None,
        })
    }

    /// Supply explicit scopes including this identity's tenant scope.
    /// Conflicting tenant scopes and duplicate scopes are rejected.
    pub fn with_scopes(mut self, scopes: Vec<String>) -> Result<Self> {
        let tenant_scope = format!("tsg_id:{}", self.tsg_id);
        if scopes.is_empty()
            || scopes.len() > 64
            || !scopes.contains(&tenant_scope)
            || scopes.iter().enumerate().any(|(index, scope)| {
                scope.is_empty()
                    || scope.len() > 512
                    || !scope
                        .bytes()
                        .all(|value| matches!(value, 0x21 | 0x23..=0x5b | 0x5d..=0x7e))
                    || (scope.starts_with("tsg_id:") && scope != &tenant_scope)
                    || scopes[..index].contains(scope)
            })
        {
            return Err(Error::new(ErrorKind::Configuration));
        }
        self.scopes = scopes;
        Ok(self)
    }

    /// Optional service audience, sent as an encoded form field.
    pub fn with_audience(mut self, audience: impl Into<String>) -> Result<Self> {
        let audience = audience.into();
        if audience.trim().is_empty()
            || audience.len() > 2048
            || audience.chars().any(char::is_control)
        {
            return Err(Error::new(ErrorKind::Configuration));
        }
        self.audience = Some(audience);
        Ok(self)
    }

    pub fn issuer(&self) -> &Endpoint {
        &self.issuer
    }
    pub fn client_id(&self) -> &str {
        &self.client_id
    }
    pub fn credential_generation(&self) -> u64 {
        self.credential_generation
    }
    pub fn tsg_id(&self) -> &str {
        &self.tsg_id
    }
    pub fn scopes(&self) -> &[String] {
        &self.scopes
    }
    pub fn audience(&self) -> Option<&str> {
        self.audience.as_deref()
    }
}

impl fmt::Debug for AuthContext {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AuthContext")
            .field("identity", &"[REDACTED]")
            .field("credential_generation", &self.credential_generation)
            .field("scope_count", &self.scopes.len())
            .field("has_audience", &self.audience.is_some())
            .finish()
    }
}

/// A bearer token and the cache generation that supplied it.
/// Explicit secret access is available; formatting redacts the token.
#[derive(Clone)]
pub struct TokenLease {
    token: Secret,
    generation: u64,
    expires_at: Instant,
    refresh_at: Instant,
}

impl TokenLease {
    /// Construct a lease for an application-provided token provider.
    pub fn new(token: Secret, generation: u64, valid_for: Duration) -> Result<Self> {
        Self::issued(token, generation, Instant::now(), valid_for)
    }

    fn issued(
        token: Secret,
        generation: u64,
        issued_at: Instant,
        valid_for: Duration,
    ) -> Result<Self> {
        validate_bearer(token.expose())?;
        if valid_for.is_zero() || valid_for > MAX_TOKEN_LIFETIME {
            return Err(Error::new(ErrorKind::Authentication));
        }
        let expires_at = issued_at
            .checked_add(valid_for)
            .ok_or_else(|| Error::new(ErrorKind::Authentication))?;
        if expires_at <= Instant::now() {
            return Err(Error::new(ErrorKind::Authentication));
        }
        // Short-lived tokens remain usable: refresh buffer is at most 20% of TTL.
        let buffer = std::cmp::min(Duration::from_secs(30), valid_for / 5);
        Ok(Self {
            token,
            generation,
            expires_at,
            refresh_at: expires_at - buffer,
        })
    }

    pub fn token(&self) -> &Secret {
        &self.token
    }
    pub fn secret(&self) -> &Secret {
        &self.token
    }
    pub fn generation(&self) -> u64 {
        self.generation
    }
    pub fn expires_at(&self) -> std::time::Instant {
        self.expires_at.into_std()
    }
    pub fn remaining(&self) -> Duration {
        self.expires_at.saturating_duration_since(Instant::now())
    }
    fn usable(&self) -> bool {
        self.refresh_at > Instant::now()
    }
}

impl fmt::Debug for TokenLease {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TokenLease")
            .field("token", &"[REDACTED]")
            .field("generation", &self.generation)
            .field("remaining", &self.remaining())
            .finish()
    }
}

/// Safe token-cache snapshot, containing no token or identity values.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub struct TokenInfo {
    pub cached: bool,
    pub generation: Option<u64>,
    pub remaining: Option<Duration>,
    pub refresh_in_progress: bool,
}

/// Service-token integration point; provider implementations own credential policy.
pub trait TokenProvider: Send + Sync {
    fn token<'a>(&'a self, options: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>>;
    fn context(&self) -> &AuthContext;
    /// Invalidate only the generation observed by the unauthorized request.
    fn invalidate(&self, generation: u64) -> bool;
}

type Refresh = BoxFuture<'static, Result<TokenLease>>;
struct Pending {
    generation: u64,
    future: WeakShared<Refresh>,
}
#[derive(Default)]
struct Cache {
    token: Option<TokenLease>,
    pending: Option<Pending>,
    clear_epoch: u64,
    next_generation: u64,
}
struct OAuthInner {
    context: AuthContext,
    client_secret: Secret,
    transport: Arc<dyn Transport>,
    policy: Arc<dyn DestinationPolicy>,
    session: Session,
    cache: Mutex<Cache>,
}

/// OAuth2 client-credentials provider with an isolated cache and bounded refresh.
#[derive(Clone)]
pub struct OAuthProvider {
    inner: Arc<OAuthInner>,
}

impl OAuthProvider {
    pub fn new(
        context: AuthContext,
        client_secret: Secret,
        transport: Arc<dyn Transport>,
        policy: Arc<dyn DestinationPolicy>,
        session: Session,
    ) -> Result<Self> {
        if client_secret.expose().chars().any(char::is_control) {
            return Err(Error::new(ErrorKind::Configuration));
        }
        Ok(Self {
            inner: Arc::new(OAuthInner {
                context,
                client_secret,
                transport,
                policy,
                session,
                cache: Mutex::new(Cache::default()),
            }),
        })
    }

    /// Clear the cached token and prevent an older pending refresh from restoring it.
    /// Existing waiters receive an authentication error if that refresh finishes.
    pub fn clear(&self) -> Result<()> {
        let mut cache = self
            .inner
            .cache
            .lock()
            .map_err(|_| Error::new(ErrorKind::Authentication))?;
        cache.clear_epoch = cache
            .clear_epoch
            .checked_add(1)
            .ok_or_else(|| Error::new(ErrorKind::Authentication))?;
        cache.token = None;
        cache.pending = None;
        Ok(())
    }

    pub fn token_info(&self) -> Result<TokenInfo> {
        let cache = self
            .inner
            .cache
            .lock()
            .map_err(|_| Error::new(ErrorKind::Authentication))?;
        Ok(TokenInfo {
            cached: cache.token.is_some(),
            generation: cache.token.as_ref().map(TokenLease::generation),
            remaining: cache.token.as_ref().map(TokenLease::remaining),
            refresh_in_progress: cache
                .pending
                .as_ref()
                .and_then(|pending| pending.future.upgrade())
                .is_some(),
        })
    }

    fn shared_refresh(&self) -> Result<Shared<Refresh>> {
        let mut cache = self
            .inner
            .cache
            .lock()
            .map_err(|_| Error::new(ErrorKind::Authentication))?;
        if self.inner.session.is_cancelled() {
            return Err(Error::new(ErrorKind::Cancelled));
        }
        if let Some(token) = cache.token.as_ref().filter(|token| token.usable()) {
            return Ok(futures_util::future::ready(Ok(token.clone()))
                .boxed()
                .shared());
        }
        if let Some(pending) = cache
            .pending
            .as_ref()
            .and_then(|pending| pending.future.upgrade())
        {
            return Ok(pending);
        }
        cache.next_generation = cache
            .next_generation
            .checked_add(1)
            .ok_or_else(|| Error::new(ErrorKind::Authentication))?;
        let generation = cache.next_generation;
        let epoch = cache.clear_epoch;
        let inner = Arc::clone(&self.inner);
        let future: Shared<Refresh> = async move {
            let result = inner.refresh(generation).await;
            let mut cache = inner
                .cache
                .lock()
                .map_err(|_| Error::new(ErrorKind::Authentication))?;
            if cache.clear_epoch != epoch {
                return Err(Error::new(ErrorKind::Authentication));
            }
            if cache
                .pending
                .as_ref()
                .is_some_and(|pending| pending.generation == generation)
            {
                cache.pending = None;
            }
            if let Ok(token) = &result {
                cache.token = Some(token.clone());
            }
            result
        }
        .boxed()
        .shared();
        cache.pending = Some(Pending {
            generation,
            future: future
                .downgrade()
                .ok_or_else(|| Error::new(ErrorKind::Authentication))?,
        });
        Ok(future)
    }
}

impl TokenProvider for OAuthProvider {
    fn token<'a>(&'a self, options: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        async move {
            let refresh = self.shared_refresh()?;
            // The caller's deadline drops only this waiter. The shared refresh has
            // its own 30-second budget, and has no strong owner inside the cache.
            options.run(&self.inner.session, refresh).await
        }
        .boxed()
    }
    fn context(&self) -> &AuthContext {
        &self.inner.context
    }
    fn invalidate(&self, generation: u64) -> bool {
        let Ok(mut cache) = self.inner.cache.lock() else {
            return false;
        };
        if cache
            .token
            .as_ref()
            .is_some_and(|token| token.generation == generation)
        {
            cache.token = None;
            // Preserve an already-running newer generation.
            true
        } else {
            false
        }
    }
}

impl fmt::Debug for OAuthProvider {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("OAuthProvider")
            .field("context", &self.inner.context)
            .field("credentials", &"[REDACTED]")
            .finish_non_exhaustive()
    }
}

#[derive(Deserialize)]
struct WireToken {
    access_token: String,
    token_type: String,
    expires_in: u64,
}

impl OAuthInner {
    async fn refresh(&self, generation: u64) -> Result<TokenLease> {
        let options = RequestOptions::with_timeout(MAX_REFRESH_TIME)?;
        options
            .run(&self.session, async {
                let destination = self.context.issuer.url().clone();
                self.context.issuer.admit(&destination)?;
                self.policy
                    .check(&destination, DestinationPurpose::OAuthToken)?;
                let form_body = {
                    let mut form = url::form_urlencoded::Serializer::new(String::new());
                    form.append_pair("grant_type", "client_credentials")
                        .append_pair("scope", &self.context.scopes.join(" "));
                    if let Some(audience) = &self.context.audience {
                        form.append_pair("audience", audience);
                    }
                    form.finish()
                };
                let encoded_id: String =
                    url::form_urlencoded::byte_serialize(self.context.client_id.as_bytes())
                        .collect();
                let encoded_secret = Zeroizing::new(
                    url::form_urlencoded::byte_serialize(self.client_secret.expose().as_bytes())
                        .collect::<String>(),
                );
                let pair = Zeroizing::new(format!("{encoded_id}:{}", encoded_secret.as_str()));
                let basic = Zeroizing::new(format!("Basic {}", STANDARD.encode(pair.as_bytes())));
                let mut headers = HeaderMap::new();
                headers.insert(AUTHORIZATION, sensitive_header(basic.as_str())?);
                headers.insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("application/x-www-form-urlencoded"),
                );
                let issued_at = Instant::now();
                let mut response = self
                    .transport
                    .send(TransportRequest::new(
                        Method::POST,
                        destination,
                        headers,
                        Bytes::from(form_body),
                        "oauth.token",
                    ))
                    .await?;
                if !response.status.is_success() {
                    return Err(
                        Error::new(ErrorKind::Authentication).with_status(response.status.as_u16())
                    );
                }
                let body = read_limited(&mut response.body, MAX_TOKEN_RESPONSE).await?;
                let wire: WireToken =
                    serde_json::from_slice(&body).map_err(|_| Error::new(ErrorKind::Decode))?;
                let access_token = Zeroizing::new(wire.access_token);
                if !wire.token_type.eq_ignore_ascii_case("bearer") {
                    return Err(Error::new(ErrorKind::Authentication));
                }
                TokenLease::issued(
                    Secret::new(access_token.as_str())
                        .map_err(|_| Error::new(ErrorKind::Authentication))?,
                    generation,
                    issued_at,
                    Duration::from_secs(wire.expires_in),
                )
            })
            .await
            .map_err(|error| error.with_operation("oauth.token"))
    }
}

/// Per-client authentication. Credentials never enter ordinary formatting.
#[derive(Clone)]
#[non_exhaustive]
pub enum Authentication {
    None,
    Scan {
        api_key: Option<Secret>,
        api_token: Option<Secret>,
    },
    OAuth {
        provider: Arc<dyn TokenProvider>,
        tsg_header: Option<String>,
    },
    RuntimeKey {
        api_key: Secret,
    },
}

impl Authentication {
    pub fn scan(api_key: Option<Secret>, api_token: Option<Secret>) -> Result<Self> {
        if api_key.is_none() && api_token.is_none() {
            return Err(Error::new(ErrorKind::Configuration));
        }
        for secret in [&api_key, &api_token].into_iter().flatten() {
            validate_header_secret(secret.expose())?;
        }
        Ok(Self::Scan { api_key, api_token })
    }
    /// Bind the optional tenant header to the provider's immutable context.
    pub fn oauth(provider: Arc<dyn TokenProvider>, with_tsg_header: bool) -> Self {
        let tsg_header = with_tsg_header.then(|| provider.context().tsg_id.clone());
        Self::OAuth {
            provider,
            tsg_header,
        }
    }
    pub fn runtime_key(api_key: Secret) -> Result<Self> {
        validate_header_secret(api_key.expose())?;
        Ok(Self::RuntimeKey { api_key })
    }

    /// Attach protected headers after destination admission. Returns the OAuth
    /// token generation used so a later 401 can invalidate precisely that lease.
    pub async fn prepare(
        &self,
        headers: &mut HeaderMap,
        body: &Bytes,
        has_body: bool,
        options: &RequestOptions,
    ) -> Result<Option<u64>> {
        // Reject caller overrides instead of silently mixing credential identities.
        for name in [
            "authorization",
            "x-pan-token",
            "x-payload-hash",
            "x-portkey-api-key",
            "x-tsg-id",
        ] {
            if headers.contains_key(name) {
                return Err(Error::new(ErrorKind::Configuration));
            }
        }
        match self {
            Self::None => Ok(None),
            Self::Scan { api_key, api_token } => {
                if api_key.is_none() && api_token.is_none() {
                    return Err(Error::new(ErrorKind::Configuration));
                }
                if let Some(token) = api_token {
                    validate_header_secret(token.expose())?;
                    headers.insert(
                        AUTHORIZATION,
                        sensitive_header(&format!("Bearer {}", token.expose()))?,
                    );
                }
                if let Some(key) = api_key {
                    validate_header_secret(key.expose())?;
                    headers.insert("x-pan-token", sensitive_header(key.expose())?);
                    if has_body {
                        let mut hmac = Hmac::<Sha256>::new_from_slice(key.expose().as_bytes())
                            .map_err(|_| Error::new(ErrorKind::Authentication))?;
                        hmac.update(body);
                        let signature: String = hmac
                            .finalize()
                            .into_bytes()
                            .iter()
                            .map(|byte| format!("{byte:02x}"))
                            .collect();
                        headers.insert("x-payload-hash", sensitive_header(&signature)?);
                    }
                }
                Ok(None)
            }
            Self::OAuth {
                provider,
                tsg_header,
            } => {
                if let Some(tenant) = tsg_header {
                    if tenant != provider.context().tsg_id() {
                        return Err(Error::new(ErrorKind::Configuration));
                    }
                    validate_header_secret(tenant)?;
                }
                let lease = provider.token(options).await?;
                validate_bearer(lease.token.expose())?;
                if lease.remaining().is_zero() {
                    return Err(Error::new(ErrorKind::Authentication));
                }
                headers.insert(
                    AUTHORIZATION,
                    sensitive_header(&format!("Bearer {}", lease.token.expose()))?,
                );
                if let Some(tenant) = tsg_header {
                    headers.insert("x-tsg-id", sensitive_header(tenant)?);
                }
                Ok(Some(lease.generation))
            }
            Self::RuntimeKey { api_key } => {
                validate_header_secret(api_key.expose())?;
                headers.insert("x-portkey-api-key", sensitive_header(api_key.expose())?);
                Ok(None)
            }
        }
    }

    pub fn on_unauthorized(&self, generation: u64) -> bool {
        match self {
            Self::OAuth { provider, .. } => provider.invalidate(generation),
            _ => false,
        }
    }
}

impl fmt::Debug for Authentication {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::None => f.write_str("Authentication::None"),
            Self::Scan { api_key, api_token } => f
                .debug_struct("Authentication::Scan")
                .field("has_api_key", &api_key.is_some())
                .field("has_api_token", &api_token.is_some())
                .finish(),
            Self::OAuth { tsg_header, .. } => f
                .debug_struct("Authentication::OAuth")
                .field("has_tenant_header", &tsg_header.is_some())
                .finish_non_exhaustive(),
            Self::RuntimeKey { .. } => f.write_str("Authentication::RuntimeKey([REDACTED])"),
        }
    }
}

fn sensitive_header(value: &str) -> Result<HeaderValue> {
    let mut header =
        HeaderValue::from_str(value).map_err(|_| Error::new(ErrorKind::Authentication))?;
    header.set_sensitive(true);
    Ok(header)
}
fn validate_header_secret(value: &str) -> Result<()> {
    if value.is_empty()
        || value.len() > 16 * 1024
        || !value.bytes().all(|byte| byte.is_ascii_graphic())
    {
        Err(Error::new(ErrorKind::Configuration))
    } else {
        Ok(())
    }
}
fn validate_bearer(value: &str) -> Result<()> {
    if value.is_empty() || value.len() > 16 * 1024 {
        return Err(Error::new(ErrorKind::Authentication));
    }
    let stripped = value.trim_end_matches('=');
    if stripped.is_empty()
        || !stripped
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"-._~+/".contains(&byte))
    {
        return Err(Error::new(ErrorKind::Authentication));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{config::OriginPolicy, transport::TransportResponse};
    use http::StatusCode;
    use std::{
        collections::VecDeque,
        sync::atomic::{AtomicUsize, Ordering},
    };
    use tokio::sync::{Notify, Semaphore};

    struct FakeTransport {
        requests: Mutex<Vec<TransportRequest>>,
        responses: Mutex<VecDeque<(StatusCode, Bytes)>>,
        release: Semaphore,
        started: Notify,
        active: AtomicUsize,
        dropped: AtomicUsize,
        blocked: bool,
    }
    impl FakeTransport {
        fn new(blocked: bool, responses: Vec<Bytes>) -> Arc<Self> {
            Arc::new(Self {
                requests: Mutex::new(Vec::new()),
                responses: Mutex::new(
                    responses
                        .into_iter()
                        .map(|body| (StatusCode::OK, body))
                        .collect(),
                ),
                release: Semaphore::new(0),
                started: Notify::new(),
                active: AtomicUsize::new(0),
                dropped: AtomicUsize::new(0),
                blocked,
            })
        }
        fn count(&self) -> usize {
            self.requests.lock().unwrap().len()
        }
    }
    struct InFlight<'a> {
        transport: &'a FakeTransport,
        completed: bool,
    }
    impl Drop for InFlight<'_> {
        fn drop(&mut self) {
            self.transport.active.fetch_sub(1, Ordering::SeqCst);
            if !self.completed {
                self.transport.dropped.fetch_add(1, Ordering::SeqCst);
            }
        }
    }
    impl Transport for FakeTransport {
        fn send<'a>(
            &'a self,
            request: TransportRequest,
        ) -> BoxFuture<'a, Result<TransportResponse>> {
            async move {
                self.requests.lock().unwrap().push(request);
                self.active.fetch_add(1, Ordering::SeqCst);
                let mut guard = InFlight {
                    transport: self,
                    completed: false,
                };
                let (status, body) = self
                    .responses
                    .lock()
                    .unwrap()
                    .pop_front()
                    .unwrap_or((StatusCode::OK, token("fallback", 3600)));
                self.started.notify_one();
                if self.blocked {
                    self.release.acquire().await.unwrap().forget();
                }
                guard.completed = true;
                Ok(TransportResponse::from_bytes(
                    status,
                    HeaderMap::new(),
                    body,
                ))
            }
            .boxed()
        }
    }
    fn token(value: &str, ttl: u64) -> Bytes {
        Bytes::from(
            serde_json::json!({"access_token":value,"token_type":"Bearer","expires_in":ttl})
                .to_string(),
        )
    }
    fn context(tenant: &str) -> AuthContext {
        AuthContext::new(
            Endpoint::parse("https://issuer.example/private-token-path").unwrap(),
            "client-sensitive",
            7,
            tenant,
        )
        .unwrap()
    }
    fn provider_with(
        context: AuthContext,
        transport: Arc<FakeTransport>,
        session: Session,
    ) -> OAuthProvider {
        let policy = Arc::new(OriginPolicy::new([context.issuer.clone()]));
        OAuthProvider::new(
            context,
            Secret::new("secret-sensitive").unwrap(),
            transport,
            policy,
            session,
        )
        .unwrap()
    }
    fn provider(transport: Arc<FakeTransport>) -> OAuthProvider {
        provider_with(context("123"), transport, Session::new())
    }
    fn options() -> RequestOptions {
        RequestOptions::with_timeout(Duration::from_secs(60)).unwrap()
    }

    #[tokio::test]
    async fn rfc4231_signature_uses_exact_bytes_and_protected_headers() {
        let auth = Authentication::scan(Some(Secret::new("Jefe").unwrap()), None).unwrap();
        let mut headers = HeaderMap::new();
        auth.prepare(
            &mut headers,
            &Bytes::from_static(b"what do ya want for nothing?"),
            true,
            &options(),
        )
        .await
        .unwrap();
        assert_eq!(
            headers["x-payload-hash"],
            "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
        );
        assert!(headers["x-pan-token"].is_sensitive());
        assert!(headers["x-payload-hash"].is_sensitive());
        let mut without_body = HeaderMap::new();
        auth.prepare(&mut without_body, &Bytes::new(), false, &options())
            .await
            .unwrap();
        assert!(!without_body.contains_key("x-payload-hash"));
        let mut empty_body = HeaderMap::new();
        auth.prepare(&mut empty_body, &Bytes::new(), true, &options())
            .await
            .unwrap();
        assert!(empty_body.contains_key("x-payload-hash"));
        let mut override_headers = HeaderMap::new();
        override_headers.insert(AUTHORIZATION, HeaderValue::from_static("caller-token"));
        assert_eq!(
            auth.prepare(&mut override_headers, &Bytes::new(), false, &options())
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Configuration
        );
    }

    #[tokio::test]
    async fn header_injection_and_wrong_auth_domains_are_rejected() {
        for invalid in ["key\r\nx-secret: leaked", "key with spaces", ""] {
            let result = Secret::new(invalid).and_then(Authentication::runtime_key);
            assert!(result.is_err());
            assert!(!format!("{:?}", result.err().unwrap()).contains("leaked"));
        }
        for invalid in ["tok en", "tok\nX", "=", "to=ken", "é"] {
            assert!(
                TokenLease::new(Secret::new(invalid).unwrap(), 1, Duration::from_secs(60)).is_err()
            );
        }
        assert!(Authentication::scan(None, None).is_err());
        // Enum-level non_exhaustive still permits constructing known variants;
        // prepare must therefore enforce invariants independently of constructors.
        let raw_empty_scan = Authentication::Scan {
            api_key: None,
            api_token: None,
        };
        assert_eq!(
            raw_empty_scan
                .prepare(&mut HeaderMap::new(), &Bytes::new(), false, &options())
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Configuration
        );
        let transport = FakeTransport::new(false, vec![token("good", 3600)]);
        let auth = Authentication::OAuth {
            provider: Arc::new(provider(transport.clone())),
            tsg_header: Some("456".to_owned()),
        };
        assert!(
            auth.prepare(&mut HeaderMap::new(), &Bytes::new(), false, &options())
                .await
                .is_err()
        );
        assert_eq!(transport.count(), 0);
    }

    #[tokio::test]
    async fn token_exchange_uses_encoded_basic_form_and_no_runtime_credentials() {
        let transport = FakeTransport::new(false, vec![token("token-1", 3600)]);
        let context = AuthContext::new(
            Endpoint::parse("https://issuer.example/token").unwrap(),
            "client: &+",
            1,
            "123",
        )
        .unwrap()
        .with_scopes(vec!["tsg_id:123".into(), "read:api".into()])
        .unwrap()
        .with_audience("service & scope")
        .unwrap();
        let provider = OAuthProvider::new(
            context.clone(),
            Secret::new("sec/ +:").unwrap(),
            transport.clone(),
            Arc::new(OriginPolicy::new([context.issuer.clone()])),
            Session::new(),
        )
        .unwrap();
        let lease = provider.token(&options()).await.unwrap();
        assert_eq!(lease.secret().expose(), "token-1");
        let requests = transport.requests.lock().unwrap();
        let request = &requests[0];
        assert_eq!(request.method(), Method::POST);
        assert_eq!(request.url(), context.issuer.url());
        assert_eq!(
            request.headers()[CONTENT_TYPE],
            "application/x-www-form-urlencoded"
        );
        let basic = request.headers()[AUTHORIZATION]
            .to_str()
            .unwrap()
            .strip_prefix("Basic ")
            .unwrap();
        assert_eq!(
            STANDARD.decode(basic).unwrap(),
            b"client%3A+%26%2B:sec%2F+%2B%3A"
        );
        let form: std::collections::BTreeMap<_, _> = url::form_urlencoded::parse(request.body())
            .into_owned()
            .collect();
        assert_eq!(form["scope"], "tsg_id:123 read:api");
        assert_eq!(form["audience"], "service & scope");
        assert_eq!(form["grant_type"], "client_credentials");
        assert!(request.headers()[AUTHORIZATION].is_sensitive());
        assert!(!request.headers().contains_key("x-portkey-api-key"));
        assert!(!request.headers().contains_key("x-pan-token"));
    }

    #[tokio::test]
    async fn token_destination_policy_runs_before_transport() {
        struct Reject;
        impl DestinationPolicy for Reject {
            fn check(&self, _: &url::Url, purpose: DestinationPurpose) -> Result<()> {
                assert_eq!(purpose, DestinationPurpose::OAuthToken);
                Err(Error::new(ErrorKind::Destination))
            }
        }
        let transport = FakeTransport::new(false, vec![]);
        let provider = OAuthProvider::new(
            context("123"),
            Secret::new("credential").unwrap(),
            transport.clone(),
            Arc::new(Reject),
            Session::new(),
        )
        .unwrap();
        assert_eq!(
            provider.token(&options()).await.unwrap_err().kind(),
            ErrorKind::Destination
        );
        assert_eq!(transport.count(), 0);
    }

    #[tokio::test]
    async fn refresh_is_deduplicated_and_one_cancelled_waiter_preserves_the_other() {
        let transport = FakeTransport::new(true, vec![token("shared-token", 3600)]);
        let provider = provider(transport.clone());
        let first_options = options();
        let first_cancel = first_options.clone();
        let first_provider = provider.clone();
        let first = tokio::spawn(async move { first_provider.token(&first_options).await });
        transport.started.notified().await;
        let second_provider = provider.clone();
        let second = tokio::spawn(async move { second_provider.token(&options()).await });
        tokio::task::yield_now().await;
        assert_eq!(transport.count(), 1);
        first_cancel.cancel();
        assert_eq!(
            first.await.unwrap().unwrap_err().kind(),
            ErrorKind::Cancelled
        );
        assert_eq!(transport.active.load(Ordering::SeqCst), 1);
        transport.release.add_permits(1);
        assert_eq!(
            second.await.unwrap().unwrap().secret().expose(),
            "shared-token"
        );
        assert_eq!(transport.dropped.load(Ordering::SeqCst), 0);
        assert_eq!(provider.token(&options()).await.unwrap().generation(), 1);
        assert_eq!(transport.count(), 1);
    }

    #[tokio::test]
    async fn last_waiter_drop_releases_http_and_next_call_can_refresh() {
        let transport = FakeTransport::new(
            true,
            vec![token("abandoned", 3600), token("replacement", 3600)],
        );
        let provider = provider(transport.clone());
        let task_provider = provider.clone();
        let task = tokio::spawn(async move { task_provider.token(&options()).await });
        transport.started.notified().await;
        task.abort();
        assert!(task.await.is_err());
        assert_eq!(transport.active.load(Ordering::SeqCst), 0);
        assert_eq!(transport.dropped.load(Ordering::SeqCst), 1);
        assert!(!provider.token_info().unwrap().refresh_in_progress);
        transport.release.add_permits(1);
        let lease = provider.token(&options()).await.unwrap();
        assert_eq!(lease.secret().expose(), "replacement");
        assert_eq!(transport.count(), 2);
    }

    #[tokio::test]
    async fn explicit_session_cancellation_interrupts_shared_refresh() {
        let transport = FakeTransport::new(true, vec![token("pending", 3600)]);
        let session = Session::new();
        let provider = provider_with(context("123"), transport.clone(), session.clone());
        let task_provider = provider.clone();
        let task = tokio::spawn(async move { task_provider.token(&options()).await });
        transport.started.notified().await;
        session.cancel();
        assert_eq!(
            task.await.unwrap().unwrap_err().kind(),
            ErrorKind::Cancelled
        );
        assert_eq!(transport.active.load(Ordering::SeqCst), 0);
        assert_eq!(
            provider.token(&options()).await.unwrap_err().kind(),
            ErrorKind::Cancelled
        );
        assert_eq!(transport.count(), 1);
    }

    #[tokio::test]
    async fn already_cancelled_waiter_never_starts_token_exchange() {
        let transport = FakeTransport::new(false, vec![token("unused", 3600)]);
        let provider = provider(transport.clone());
        let options = options();
        options.cancel();
        assert_eq!(
            provider.token(&options).await.unwrap_err().kind(),
            ErrorKind::Cancelled
        );
        assert_eq!(transport.count(), 0);
        assert!(!provider.token_info().unwrap().refresh_in_progress);
    }

    #[tokio::test(start_paused = true)]
    async fn refresh_has_its_own_thirty_second_deadline() {
        let transport = FakeTransport::new(true, vec![token("pending", 3600)]);
        let provider = provider(transport.clone());
        let task = tokio::spawn(async move { provider.token(&options()).await });
        transport.started.notified().await;
        tokio::time::advance(Duration::from_secs(31)).await;
        assert_eq!(task.await.unwrap().unwrap_err().kind(), ErrorKind::Deadline);
        assert_eq!(transport.active.load(Ordering::SeqCst), 0);
        assert_eq!(transport.dropped.load(Ordering::SeqCst), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn one_waiter_deadline_does_not_expire_other_waiters() {
        let transport = FakeTransport::new(true, vec![token("shared-after-deadline", 3600)]);
        let provider = provider(transport.clone());
        let first_provider = provider.clone();
        let first = tokio::spawn(async move {
            first_provider
                .token(&RequestOptions::with_timeout(Duration::from_secs(1)).unwrap())
                .await
        });
        transport.started.notified().await;
        let second_provider = provider.clone();
        let second = tokio::spawn(async move { second_provider.token(&options()).await });
        tokio::task::yield_now().await;
        tokio::time::advance(Duration::from_secs(2)).await;
        assert_eq!(
            first.await.unwrap().unwrap_err().kind(),
            ErrorKind::Deadline
        );
        assert_eq!(transport.active.load(Ordering::SeqCst), 1);
        transport.release.add_permits(1);
        assert_eq!(
            second.await.unwrap().unwrap().secret().expose(),
            "shared-after-deadline"
        );
        assert_eq!(transport.count(), 1);
        assert_eq!(transport.dropped.load(Ordering::SeqCst), 0);
    }

    #[tokio::test(start_paused = true)]
    async fn late_unauthorized_response_cannot_evict_new_generation() {
        let transport = FakeTransport::new(false, vec![token("old", 2), token("new", 3600)]);
        let provider = provider(transport.clone());
        let old = provider.token(&options()).await.unwrap();
        tokio::time::advance(Duration::from_secs(2)).await;
        let new = provider.token(&options()).await.unwrap();
        assert!(new.generation() > old.generation());
        assert!(!provider.invalidate(old.generation()));
        assert_eq!(
            provider.token(&options()).await.unwrap().secret().expose(),
            "new"
        );
        assert_eq!(transport.count(), 2);
        assert!(provider.invalidate(new.generation()));
        assert!(!provider.token_info().unwrap().cached);
    }

    #[tokio::test(start_paused = true)]
    async fn token_lifetime_starts_before_exchange_latency() {
        let transport = FakeTransport::new(true, vec![token("already-expired", 1)]);
        let provider = provider(transport.clone());
        let task_provider = provider.clone();
        let task = tokio::spawn(async move { task_provider.token(&options()).await });
        transport.started.notified().await;
        tokio::time::advance(Duration::from_secs(2)).await;
        transport.release.add_permits(1);
        assert_eq!(
            task.await.unwrap().unwrap_err().kind(),
            ErrorKind::Authentication
        );
        assert!(!provider.token_info().unwrap().cached);
    }

    #[tokio::test(start_paused = true)]
    async fn invalidating_old_cached_token_preserves_new_pending_refresh() {
        let transport = FakeTransport::new(true, vec![token("old", 2), token("new", 3600)]);
        let provider = provider(transport.clone());
        transport.release.add_permits(1);
        let old = provider.token(&options()).await.unwrap();
        tokio::time::advance(Duration::from_secs(2)).await;
        let next_provider = provider.clone();
        let next = tokio::spawn(async move { next_provider.token(&options()).await });
        while transport.count() < 2 {
            tokio::task::yield_now().await;
        }
        assert!(provider.invalidate(old.generation()));
        transport.release.add_permits(1);
        let new = next.await.unwrap().unwrap();
        assert_eq!(
            provider.token(&options()).await.unwrap().generation(),
            new.generation()
        );
        assert_eq!(transport.count(), 2);
    }

    #[tokio::test]
    async fn clear_prevents_pending_old_refresh_from_restoring_cache() {
        let transport = FakeTransport::new(
            true,
            vec![token("old-pending", 3600), token("new-pending", 3600)],
        );
        let provider = provider(transport.clone());
        let old_provider = provider.clone();
        let old = tokio::spawn(async move { old_provider.token(&options()).await });
        transport.started.notified().await;
        provider.clear().unwrap();
        let next_provider = provider.clone();
        let next = tokio::spawn(async move { next_provider.token(&options()).await });
        while transport.count() < 2 {
            tokio::task::yield_now().await;
        }
        transport.release.add_permits(2);
        assert_eq!(
            old.await.unwrap().unwrap_err().kind(),
            ErrorKind::Authentication
        );
        assert_eq!(
            next.await.unwrap().unwrap().secret().expose(),
            "new-pending"
        );
        assert_eq!(
            provider.token(&options()).await.unwrap().secret().expose(),
            "new-pending"
        );
    }

    #[tokio::test]
    async fn identities_caches_and_formatting_are_isolated() {
        let transport = FakeTransport::new(
            false,
            vec![
                token("tenant-one-token", 3600),
                token("tenant-two-token", 3600),
            ],
        );
        let first = provider_with(context("123"), transport.clone(), Session::new());
        let second = provider_with(context("456"), transport.clone(), Session::new());
        let first_token = first.token(&options()).await.unwrap();
        let second_token = second.token(&options()).await.unwrap();
        assert_ne!(
            first_token.secret().expose(),
            second_token.secret().expose()
        );
        assert_eq!(transport.count(), 2);
        assert!(first.invalidate(first_token.generation()));
        assert!(second.token_info().unwrap().cached);
        let auth = Authentication::oauth(Arc::new(second.clone()), true);
        let output = format!(
            "{first:?} {second:?} {auth:?} {first_token:?} {:?}",
            first.token_info().unwrap()
        );
        for secret in [
            "issuer.example",
            "private-token-path",
            "client-sensitive",
            "secret-sensitive",
            "tenant-one-token",
            "tenant-two-token",
            "tsg_id:123",
            "tsg_id:456",
        ] {
            assert!(!output.contains(secret));
        }
        let mut headers = HeaderMap::new();
        assert_eq!(
            auth.prepare(&mut headers, &Bytes::new(), false, &options())
                .await
                .unwrap(),
            Some(second_token.generation())
        );
        assert_eq!(headers["x-tsg-id"], "456");
        assert_eq!(headers[AUTHORIZATION], "Bearer tenant-two-token");
        assert!(headers[AUTHORIZATION].is_sensitive());
        assert!(headers["x-tsg-id"].is_sensitive());
    }

    #[tokio::test]
    async fn malformed_or_unbounded_token_responses_fail_without_values() {
        let cases = [
            (Bytes::from_static(br#"{"access_token":"secret\r\nvalue","token_type":"Bearer","expires_in":3600}"#), ErrorKind::Authentication),
            (Bytes::from_static(br#"{"access_token":"secret","token_type":"MAC","expires_in":3600}"#), ErrorKind::Authentication),
            (Bytes::from_static(br#"{"access_token":"secret","token_type":"Bearer","expires_in":0}"#), ErrorKind::Authentication),
            (Bytes::from_static(br#"{"access_token":"secret","token_type":"Bearer","expires_in":999999999999}"#), ErrorKind::Authentication),
            (Bytes::from_static(br#"{"access_token":"secret","token_type":"Bearer","expires_in":"3600"}"#), ErrorKind::Decode),
            (Bytes::from_static(b"secret malformed upstream response"), ErrorKind::Decode),
            (Bytes::from(vec![b'x'; MAX_TOKEN_RESPONSE + 1]), ErrorKind::Limit),
        ];
        for (body, kind) in cases {
            let transport = FakeTransport::new(false, vec![body]);
            let error = provider(transport).token(&options()).await.unwrap_err();
            assert_eq!(error.kind(), kind);
            assert!(!format!("{error:?} {error}").contains("secret"));
        }
        let transport =
            FakeTransport::new(false, vec![Bytes::from_static(b"secret upstream error")]);
        transport.responses.lock().unwrap()[0].0 = StatusCode::UNAUTHORIZED;
        let error = provider(transport).token(&options()).await.unwrap_err();
        assert_eq!(error.status(), Some(401));
        assert!(!format!("{error:?} {error}").contains("secret"));
    }

    #[test]
    fn immutable_context_rejects_cross_tenant_or_unsafe_scope_configuration() {
        assert!(
            context("123")
                .with_scopes(vec!["tsg_id:456".into()])
                .is_err()
        );
        assert!(
            context("123")
                .with_scopes(vec!["tsg_id:123".into(), "two scopes".into()])
                .is_err()
        );
        assert!(
            context("123")
                .with_scopes(vec!["tsg_id:123".into(), "tsg_id:123".into()])
                .is_err()
        );
        assert!(context("123").with_audience("audience\nInjected").is_err());
        assert!(
            AuthContext::new(
                Endpoint::parse("https://issuer.example/token").unwrap(),
                "client",
                1,
                "123\n"
            )
            .is_err()
        );
    }
}
