//! Experimental, caller-driven realtime connections with explicit WebSocket transport.
//! No worker, reader, refresh, replay, or runtime is created by this module.
use crate::airs::gateway::models::GatewayRealtimeEvent;
use crate::{Error, ErrorKind, RemoteOutcome, RequestOptions, Result, Session};
use bytes::Bytes;
use futures_util::{
    Sink, Stream,
    future::{BoxFuture, poll_fn},
};
use http::{HeaderMap, StatusCode};
use std::{
    fmt,
    pin::Pin,
    sync::{Arc, Mutex},
    task::{Context, Poll},
    time::Duration,
};
use tokio::sync::OwnedSemaphorePermit;
use tokio_util::sync::CancellationToken;
use url::Url;

#[cfg(feature = "ws-transport")]
mod native;
#[cfg(feature = "ws-transport")]
pub use native::NativeWebSocketConnector;

/// Frames exposed to trusted adapters. Payload values are omitted from formatting.
#[derive(Clone)]
#[non_exhaustive]
pub enum WebSocketFrame {
    Text(Bytes),
    Binary(Bytes),
    Close(u16),
}
impl fmt::Debug for WebSocketFrame {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Text(bytes) => f.debug_tuple("Text").field(&bytes.len()).finish(),
            Self::Binary(bytes) => f.debug_tuple("Binary").field(&bytes.len()).finish(),
            Self::Close(code) => f.debug_tuple("Close").field(code).finish(),
        }
    }
}

/// Trusted framed socket. Adapters enforce frame/fragment bounds, strict UTF-8,
/// masking, control replies, bounded writes, and prompt release on drop.
/// Ping/pong control frames are handled internally and never queued as events.
pub trait GatewayWebSocket:
    Stream<Item = Result<WebSocketFrame>> + Sink<WebSocketFrame, Error = Error> + Send + Unpin
{
}
impl<T> GatewayWebSocket for T where
    T: Stream<Item = Result<WebSocketFrame>> + Sink<WebSocketFrame, Error = Error> + Send + Unpin
{
}
pub type BoxWebSocket = Box<dyn GatewayWebSocket>;
pub type GatewayWebSocketFactory = Arc<dyn WebSocketConnector>;

/// Prepared, admitted runtime-key upgrade. Explicit getters expose sensitive data.
pub struct WebSocketRequest {
    pub(crate) url: Url,
    pub(crate) headers: HeaderMap,
    pub(crate) handshake_timeout: Duration,
    pub(crate) max_event_bytes: usize,
}
impl WebSocketRequest {
    pub fn url(&self) -> &Url {
        &self.url
    }
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
    pub fn handshake_timeout(&self) -> Duration {
        self.handshake_timeout
    }
    pub fn max_event_bytes(&self) -> usize {
        self.max_event_bytes
    }
    pub fn operation(&self) -> &'static str {
        OPERATION
    }
}
impl fmt::Debug for WebSocketRequest {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebSocketRequest")
            .field("operation", &OPERATION)
            .field("max_event_bytes", &self.max_event_bytes)
            .finish_non_exhaustive()
    }
}

/// Upgrade result from a trusted connector. The SDK still requires HTTP 101.
pub struct WebSocketUpgrade {
    pub(crate) status: StatusCode,
    pub(crate) headers: HeaderMap,
    pub(crate) socket: BoxWebSocket,
}
impl WebSocketUpgrade {
    pub fn new(status: StatusCode, mut headers: HeaderMap, socket: BoxWebSocket) -> Result<Self> {
        if headers.len() > 128
            || headers
                .iter()
                .try_fold(0usize, |size, (name, value)| {
                    size.checked_add(name.as_str().len())?
                        .checked_add(value.len())
                })
                .is_none_or(|size| size > 64 * 1024)
        {
            return Err(Error::new(ErrorKind::Limit));
        }
        for value in headers.values_mut() {
            value.set_sensitive(true);
        }
        Ok(Self {
            status,
            headers,
            socket,
        })
    }
}
impl fmt::Debug for WebSocketUpgrade {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("WebSocketUpgrade")
            .field("status", &self.status.as_u16())
            .finish_non_exhaustive()
    }
}

/// Explicit transport policy: no redirects, extensions, compression, or subprotocols.
/// Custom connectors own proxy/TLS integration and must return sanitized errors.
pub trait WebSocketConnector: Send + Sync {
    fn connect<'a>(&'a self, request: WebSocketRequest) -> BoxFuture<'a, Result<WebSocketUpgrade>>;
}

#[derive(Clone)]
#[non_exhaustive]
pub struct GatewayRealtimeOptions {
    pub(crate) connector: GatewayWebSocketFactory,
    pub handshake_timeout: Duration,
    pub idle_timeout: Duration,
    pub close_timeout: Duration,
    pub max_event_bytes: usize,
    pub max_buffered_events: usize,
    pub max_buffered_bytes: usize,
}
impl GatewayRealtimeOptions {
    pub fn new(connector: GatewayWebSocketFactory) -> Self {
        Self {
            connector,
            handshake_timeout: Duration::from_secs(15),
            idle_timeout: Duration::from_secs(30),
            close_timeout: Duration::from_millis(500),
            max_event_bytes: 1024 * 1024,
            max_buffered_events: 256,
            max_buffered_bytes: 4 * 1024 * 1024,
        }
    }
    pub(crate) fn validate(&self) -> Result<()> {
        if self.handshake_timeout.is_zero()
            || self.handshake_timeout > Duration::from_secs(86400)
            || self.idle_timeout.is_zero()
            || self.idle_timeout > Duration::from_secs(86400)
            || self.close_timeout.is_zero()
            || self.close_timeout > Duration::from_secs(5)
            || !(1..=16 * 1024 * 1024).contains(&self.max_event_bytes)
            || !(1..=4096).contains(&self.max_buffered_events)
            || !(1..=64 * 1024 * 1024).contains(&self.max_buffered_bytes)
        {
            return Err(Error::new(ErrorKind::Configuration));
        }
        Ok(())
    }
}
impl fmt::Debug for GatewayRealtimeOptions {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("GatewayRealtimeOptions([REDACTED])")
    }
}

pub(crate) const OPERATION: &str = "airs.gateway.inference.connect_realtime";
struct Owned {
    socket: Option<BoxWebSocket>,
    permit: Option<OwnedSemaphorePermit>,
    failure: Option<Error>,
    closed: bool,
    closing: bool,
}
struct Shared {
    owned: Mutex<Owned>,
    options: RequestOptions,
    session: Session,
    cancellation: CancellationToken,
    settings: GatewayRealtimeOptions,
    validation: crate::validation::ValidationLimits,
    request_id: Option<String>,
}
impl Shared {
    fn lock(&self) -> std::sync::MutexGuard<'_, Owned> {
        // A caller-owned adapter may unwind. Recover only to close its owned socket.
        match self.owned.lock() {
            Ok(value) => value,
            Err(poison) => poison.into_inner(),
        }
    }
    fn terminal(&self) -> Result<bool> {
        let state = self.lock();
        if let Some(error) = &state.failure {
            return Err(error.clone());
        }
        Ok(state.closed)
    }
    fn finish(&self, failure: Option<Error>) {
        let (socket, permit) = {
            let mut state = self.lock();
            if state.closed {
                return;
            }
            state.closed = true;
            state.failure = failure;
            (state.socket.take(), state.permit.take())
        };
        self.cancellation.cancel();
        drop(socket);
        drop(permit);
    }
    fn fail(&self, error: Error) -> Error {
        let error = error
            .with_response_metadata(101, self.request_id.clone())
            .with_operation(OPERATION)
            .with_outcome(RemoteOutcome::Unknown);
        let (socket, permit) = {
            let mut state = self.lock();
            if let Some(previous) = &state.failure {
                return previous.clone();
            }
            if state.closed {
                return error;
            }
            state.closed = true;
            state.failure = Some(error.clone());
            (state.socket.take(), state.permit.take())
        };
        self.cancellation.cancel();
        drop(socket);
        drop(permit);
        error
    }
    async fn run<T>(&self, future: impl std::future::Future<Output = Result<T>>) -> Result<T> {
        let result = self
            .options
            .run(&self.session, async {
                tokio::select! { biased;
                    _ = self.cancellation.cancelled() => Err(Error::new(ErrorKind::Cancelled)),
                    result = tokio::time::timeout(self.settings.idle_timeout, future) =>
                        result.unwrap_or_else(|_| Err(Error::new(ErrorKind::Deadline))),
                }
            })
            .await;
        result.map_err(|error| self.fail(error))
    }
    fn poll_frame(&self, cx: &mut Context<'_>) -> Poll<Result<WebSocketFrame>> {
        let mut state = self.lock();
        if let Some(error) = &state.failure {
            return Poll::Ready(Err(error.clone()));
        }
        let Some(socket) = state.socket.as_mut() else {
            return Poll::Ready(Err(Error::new(ErrorKind::Protocol)));
        };
        match Pin::new(socket.as_mut()).poll_next(cx) {
            Poll::Ready(Some(result)) => Poll::Ready(result),
            Poll::Ready(None) => Poll::Ready(Err(Error::new(ErrorKind::Protocol))),
            Poll::Pending => Poll::Pending,
        }
    }
    async fn write(&self, frame: WebSocketFrame) -> Result<()> {
        let mut frame = Some(frame);
        poll_fn(|cx| {
            let mut state = self.lock();
            if let Some(error) = &state.failure {
                return Poll::Ready(Err(error.clone()));
            }
            let Some(socket) = state.socket.as_mut() else {
                return Poll::Ready(Err(Error::new(ErrorKind::Protocol)));
            };
            if frame.is_some() {
                match Pin::new(socket.as_mut()).poll_ready(cx) {
                    Poll::Ready(Ok(())) => {}
                    Poll::Ready(Err(error)) => return Poll::Ready(Err(error)),
                    Poll::Pending => return Poll::Pending,
                }
                if let Some(frame) = frame.take()
                    && let Err(error) = Pin::new(socket.as_mut()).start_send(frame)
                {
                    return Poll::Ready(Err(error));
                }
            }
            Pin::new(socket.as_mut()).poll_flush(cx)
        })
        .await
    }
    async fn next(&self) -> Result<Option<GatewayRealtimeEvent>> {
        if self.terminal()? {
            return Ok(None);
        }
        let frame = self.run(poll_fn(|cx| self.poll_frame(cx))).await?;
        match frame {
            WebSocketFrame::Close(1000) => {
                self.finish(None);
                Ok(None)
            }
            WebSocketFrame::Close(_) | WebSocketFrame::Binary(_) => {
                Err(self.fail(Error::new(ErrorKind::Protocol)))
            }
            WebSocketFrame::Text(bytes) => {
                let result = self
                    .run(async {
                        if bytes.len() > self.settings.max_event_bytes {
                            return Err(Error::new(ErrorKind::Limit));
                        }
                        let value = serde_json::from_slice(&bytes)
                            .map_err(|_| Error::new(ErrorKind::Decode))?;
                        let value = crate::validation::parse_with_limits(
                            "GatewayRealtimeEventSchema",
                            value,
                            crate::validation::Mode::Response,
                            &self.validation,
                        )
                        .map_err(|error| {
                            Error::new(if error.kind() == ErrorKind::Limit {
                                ErrorKind::Limit
                            } else {
                                ErrorKind::Decode
                            })
                        })?;
                        serde_json::from_value(value)
                            .map(Some)
                            .map_err(|_| Error::new(ErrorKind::Decode))
                    })
                    .await?;
                Ok(result)
            }
        }
    }
    async fn send(self: &Arc<Self>, event: &GatewayRealtimeEvent) -> Result<()> {
        if self.terminal()? || self.lock().closing {
            return Err(Error::new(ErrorKind::Protocol));
        }
        let bytes = self
            .run(async {
                let limit = self
                    .settings
                    .max_event_bytes
                    .min(self.settings.max_buffered_bytes);
                let bytes = crate::engine::serialize_bounded(event, limit)?;
                let value = serde_json::from_slice(&bytes)
                    .map_err(|_| Error::new(ErrorKind::Validation))?;
                let value = crate::validation::parse_with_limits(
                    "GatewayRealtimeEventSchema",
                    value,
                    crate::validation::Mode::Request,
                    &self.validation,
                )?;
                crate::engine::serialize_bounded(&value, limit)
            })
            .await?;
        let mut guard = SendGuard {
            shared: self.clone(),
            complete: false,
        };
        let result = self.run(self.write(WebSocketFrame::Text(bytes))).await;
        guard.complete = true;
        result
    }
    async fn cancel(self: &Arc<Self>) -> Result<()> {
        if self.terminal()? {
            return Ok(());
        }
        self.lock().closing = true;
        let mut guard = SendGuard {
            shared: self.clone(),
            complete: false,
        };
        let result = tokio::time::timeout(self.settings.close_timeout, async {
            self.write(WebSocketFrame::Close(1000)).await?;
            loop {
                if self.terminal()? {
                    return Ok(());
                }
                match poll_fn(|cx| self.poll_frame(cx)).await? {
                    WebSocketFrame::Close(1000) => return Ok(()),
                    WebSocketFrame::Close(_) => return Err(Error::new(ErrorKind::Protocol)),
                    // Discard in-flight frames only during the bounded close handshake.
                    WebSocketFrame::Text(_) | WebSocketFrame::Binary(_) => {}
                }
                tokio::task::yield_now().await;
            }
        })
        .await
        .unwrap_or_else(|_| Err(Error::new(ErrorKind::Deadline)));
        guard.complete = true;
        match result {
            Ok(()) => {
                self.finish(None);
                Ok(())
            }
            Err(_) if self.terminal().is_ok_and(|closed| closed) => Ok(()),
            Err(error) => Err(self.fail(error)),
        }
    }
}
struct SendGuard {
    shared: Arc<Shared>,
    complete: bool,
}
impl Drop for SendGuard {
    fn drop(&mut self) {
        if !self.complete {
            self.shared.fail(Error::new(ErrorKind::Cancelled));
        }
    }
}

/// Single consumer connection. An HTTP upgrade does not establish provider readiness.
pub struct GatewayRealtimeConnection {
    shared: Option<Arc<Shared>>,
}
pub struct GatewayRealtimeSender {
    shared: Arc<Shared>,
}
pub struct GatewayRealtimeReceiver {
    shared: Arc<Shared>,
}
impl GatewayRealtimeConnection {
    pub(crate) fn new(
        upgrade: WebSocketUpgrade,
        permit: OwnedSemaphorePermit,
        options: RequestOptions,
        session: Session,
        settings: GatewayRealtimeOptions,
        validation: crate::validation::ValidationLimits,
    ) -> Self {
        let request_id = crate::engine::error_metadata(&upgrade.headers, None).0;
        Self {
            shared: Some(Arc::new(Shared {
                owned: Mutex::new(Owned {
                    socket: Some(upgrade.socket),
                    permit: Some(permit),
                    failure: None,
                    closed: false,
                    closing: false,
                }),
                options,
                session,
                settings,
                validation,
                request_id,
                cancellation: CancellationToken::new(),
            })),
        }
    }
    fn inner(&self) -> Result<&Arc<Shared>> {
        self.shared
            .as_ref()
            .ok_or_else(|| Error::new(ErrorKind::Protocol))
    }
    pub async fn next(&mut self) -> Result<Option<GatewayRealtimeEvent>> {
        self.inner()?.next().await
    }
    pub async fn send(&mut self, event: &GatewayRealtimeEvent) -> Result<()> {
        self.inner()?.send(event).await
    }
    pub async fn cancel(&mut self) -> Result<()> {
        self.inner()?.cancel().await
    }
    pub fn into_stream(self) -> impl Stream<Item = Result<GatewayRealtimeEvent>> + Send {
        futures_util::stream::try_unfold(self, |mut connection| async move {
            connection
                .next()
                .await
                .map(|value| value.map(|event| (event, connection)))
        })
    }
    /// Both halves share one session. Dropping either half terminates the socket immediately.
    pub fn split(mut self) -> Result<(GatewayRealtimeSender, GatewayRealtimeReceiver)> {
        let shared = self
            .shared
            .take()
            .ok_or_else(|| Error::new(ErrorKind::Protocol))?;
        Ok((
            GatewayRealtimeSender {
                shared: shared.clone(),
            },
            GatewayRealtimeReceiver { shared },
        ))
    }
}
impl GatewayRealtimeSender {
    pub async fn send(&mut self, event: &GatewayRealtimeEvent) -> Result<()> {
        self.shared.send(event).await
    }
    pub async fn cancel(&mut self) -> Result<()> {
        self.shared.cancel().await
    }
}
impl GatewayRealtimeReceiver {
    pub async fn next(&mut self) -> Result<Option<GatewayRealtimeEvent>> {
        self.shared.next().await
    }
    pub async fn cancel(&mut self) -> Result<()> {
        self.shared.cancel().await
    }
}
impl Drop for GatewayRealtimeConnection {
    fn drop(&mut self) {
        if let Some(shared) = &self.shared
            && !shared.terminal().is_ok_and(|closed| closed)
        {
            shared.fail(Error::new(ErrorKind::Cancelled));
        }
    }
}
impl Drop for GatewayRealtimeSender {
    fn drop(&mut self) {
        if !self.shared.terminal().is_ok_and(|closed| closed) {
            self.shared.fail(Error::new(ErrorKind::Cancelled));
        }
    }
}
impl Drop for GatewayRealtimeReceiver {
    fn drop(&mut self) {
        if !self.shared.terminal().is_ok_and(|closed| closed) {
            self.shared.fail(Error::new(ErrorKind::Cancelled));
        }
    }
}
crate::redaction::redacted_debug!(
    GatewayRealtimeConnection,
    GatewayRealtimeSender,
    GatewayRealtimeReceiver
);
