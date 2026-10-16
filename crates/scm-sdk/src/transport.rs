//! A narrow caller-owned HTTP transport seam, shared by APIs and OAuth exchange.

use crate::{Error, ErrorKind, Result};
use bytes::{Bytes, BytesMut};
use futures_util::{Stream, StreamExt, future::BoxFuture};
use http::{HeaderMap, Method, StatusCode};
use std::{fmt, pin::Pin};
use url::Url;

pub type BodyStream = Pin<Box<dyn Stream<Item = Result<Bytes>> + Send>>;

/// A prepared request. Inspecting values explicitly transfers diagnostic responsibility.
pub struct TransportRequest {
    pub(crate) method: Method,
    pub(crate) url: Url,
    pub(crate) headers: HeaderMap,
    pub(crate) body: Bytes,
    pub(crate) operation: &'static str,
}
impl TransportRequest {
    pub(crate) fn new(
        method: Method,
        url: Url,
        mut headers: HeaderMap,
        body: Bytes,
        operation: &'static str,
    ) -> Self {
        for value in headers.values_mut() {
            value.set_sensitive(true);
        }
        Self {
            method,
            url,
            headers,
            body,
            operation,
        }
    }
    pub fn method(&self) -> &Method {
        &self.method
    }
    pub fn url(&self) -> &Url {
        &self.url
    }
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
    pub fn body(&self) -> &Bytes {
        &self.body
    }
    pub fn operation(&self) -> &'static str {
        self.operation
    }
}
impl fmt::Debug for TransportRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportRequest")
            .field("method", &self.method)
            .field("operation", &self.operation)
            .field("body_bytes", &self.body.len())
            .finish_non_exhaustive()
    }
}

/// Streaming response, whose ordinary formatting excludes headers and body values.
pub struct TransportResponse {
    pub(crate) status: StatusCode,
    pub(crate) headers: HeaderMap,
    pub(crate) body: BodyStream,
}
impl TransportResponse {
    pub fn new(status: StatusCode, headers: HeaderMap, body: BodyStream) -> Self {
        Self {
            status,
            headers,
            body,
        }
    }
    pub fn from_bytes(status: StatusCode, headers: HeaderMap, bytes: Bytes) -> Self {
        Self::new(
            status,
            headers,
            Box::pin(futures_util::stream::once(async move { Ok(bytes) })),
        )
    }
    pub fn status(&self) -> StatusCode {
        self.status
    }
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
}
impl fmt::Debug for TransportResponse {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("TransportResponse")
            .field("status", &self.status.as_u16())
            .finish_non_exhaustive()
    }
}

/// Trusted integration code that sends the SDK's admitted, authenticated request.
///
/// Implementations must preserve its exact body, enforce TLS, reject redirects,
/// bound received headers/chunks, and release resources when the future/body drops.
/// Return sanitized SDK errors. The SDK enforces admission, deadlines and consumed
/// byte limits, but cannot prevent arbitrary behavior inside a caller's implementation.
pub trait Transport: Send + Sync {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>>;
}

pub(crate) async fn read_limited(body: &mut BodyStream, limit: usize) -> Result<Bytes> {
    let mut collected = BytesMut::new();
    while let Some(chunk) = body.next().await {
        let chunk = chunk?;
        let size = collected
            .len()
            .checked_add(chunk.len())
            .ok_or_else(|| Error::new(ErrorKind::Limit))?;
        if size > limit {
            return Err(Error::new(ErrorKind::Limit));
        }
        collected.extend_from_slice(&chunk);
    }
    Ok(collected.freeze())
}

/// Default pooled Tokio HTTP adapter, with TLS, redirects and retries fixed safely.
#[cfg(feature = "reqwest-transport")]
#[derive(Clone)]
pub struct ReqwestTransport {
    client: reqwest::Client,
}
#[cfg(feature = "reqwest-transport")]
impl ReqwestTransport {
    pub fn new() -> Result<Self> {
        Self::configured(None, &[])
    }
    /// Configure an explicit proxy and additional trusted PEM certificates.
    /// Environment proxy settings remain supported by reqwest when no proxy is supplied.
    pub fn configured(proxy: Option<&str>, certificates: &[&[u8]]) -> Result<Self> {
        let mut builder = reqwest::Client::builder()
            .redirect(reqwest::redirect::Policy::none())
            .retry(reqwest::retry::never())
            .connect_timeout(std::time::Duration::from_secs(10))
            .pool_max_idle_per_host(32)
            .pool_idle_timeout(std::time::Duration::from_secs(90))
            .http1_max_headers(128)
            .http2_max_header_list_size(16 * 1024)
            .no_brotli()
            .no_deflate()
            .no_zstd()
            .gzip(true)
            .referer(false)
            .user_agent(concat!("scm-sdk/", env!("CARGO_PKG_VERSION")));
        if let Some(proxy) = proxy {
            builder = builder.proxy(
                reqwest::Proxy::all(proxy).map_err(|_| Error::new(ErrorKind::Configuration))?,
            );
        }
        for pem in certificates {
            let certificate = reqwest::Certificate::from_pem(pem)
                .map_err(|_| Error::new(ErrorKind::Configuration))?;
            builder = builder.add_root_certificate(certificate);
        }
        Ok(Self {
            client: builder
                .build()
                .map_err(|_| Error::new(ErrorKind::Configuration))?,
        })
    }
}
#[cfg(feature = "reqwest-transport")]
impl fmt::Debug for ReqwestTransport {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ReqwestTransport(..)")
    }
}
#[cfg(feature = "reqwest-transport")]
impl Transport for ReqwestTransport {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            let response = self
                .client
                .request(request.method, request.url)
                .headers(request.headers)
                .body(request.body)
                .send()
                .await
                .map_err(|_| Error::new(ErrorKind::Transport))?;
            let status = response.status();
            let headers = response.headers().clone();
            let stream = response
                .bytes_stream()
                .map(|v| v.map_err(|_| Error::new(ErrorKind::Transport)));
            Ok(TransportResponse::new(status, headers, Box::pin(stream)))
        })
    }
}
