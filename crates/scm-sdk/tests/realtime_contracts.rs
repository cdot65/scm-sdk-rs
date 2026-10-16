#![cfg(feature = "realtime")]
use bytes::Bytes;
use futures_util::{Sink, Stream, StreamExt, future::BoxFuture, task::AtomicWaker};
use http::{HeaderMap, HeaderValue, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, Limits, RemoteOutcome, RequestOptions, Secret,
    airs::gateway::{
        RuntimeClient,
        models::{GatewayRealtimeConnectRequest, GatewayRealtimeEvent},
    },
    auth::Authentication,
    config::{DestinationPolicy, DestinationPurpose},
    realtime::{
        GatewayRealtimeOptions, WebSocketConnector, WebSocketFrame, WebSocketRequest,
        WebSocketUpgrade,
    },
    transport::{Transport, TransportRequest, TransportResponse},
};
use serde_json::json;
use std::{
    collections::VecDeque,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    task::{Context, Poll},
    time::Duration,
};
use url::Url;

struct NoHttp;
impl Transport for NoHttp {
    fn send<'a>(
        &'a self,
        _: TransportRequest,
    ) -> BoxFuture<'a, scm_sdk::Result<TransportResponse>> {
        Box::pin(async { panic!("realtime must never use the HTTP adapter") })
    }
}
#[derive(Default)]
struct Probe {
    incoming: Mutex<VecDeque<scm_sdk::Result<WebSocketFrame>>>,
    writes: Mutex<Vec<WebSocketFrame>>,
    read_waker: AtomicWaker,
    write_waker: AtomicWaker,
    flush_blocked: AtomicBool,
    dropped: AtomicUsize,
    eof: bool,
    echo_on_write: bool,
    auto_close: bool,
}
struct Socket(Arc<Probe>);
impl Drop for Socket {
    fn drop(&mut self) {
        self.0.dropped.fetch_add(1, Ordering::SeqCst);
    }
}
impl Stream for Socket {
    type Item = scm_sdk::Result<WebSocketFrame>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.0.read_waker.register(cx.waker());
        match self.0.incoming.lock().unwrap().pop_front() {
            Some(value) => Poll::Ready(Some(value)),
            None if self.0.eof => Poll::Ready(None),
            None => Poll::Pending,
        }
    }
}
impl Sink<WebSocketFrame> for Socket {
    type Error = Error;
    fn poll_ready(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<scm_sdk::Result<()>> {
        Poll::Ready(Ok(()))
    }
    fn start_send(self: Pin<&mut Self>, frame: WebSocketFrame) -> scm_sdk::Result<()> {
        match &frame {
            WebSocketFrame::Text(_) if self.0.echo_on_write => {
                self.0.incoming.lock().unwrap().push_back(Ok(text(
                    json!({"type":"session.created","provider_unknown":true}),
                )));
                self.0.read_waker.wake();
            }
            WebSocketFrame::Close(1000) if self.0.auto_close => {
                self.0
                    .incoming
                    .lock()
                    .unwrap()
                    .push_back(Ok(WebSocketFrame::Close(1000)));
                self.0.read_waker.wake();
            }
            _ => {}
        }
        self.0.writes.lock().unwrap().push(frame);
        Ok(())
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<scm_sdk::Result<()>> {
        self.0.write_waker.register(cx.waker());
        if self.0.flush_blocked.load(Ordering::SeqCst) {
            Poll::Pending
        } else {
            Poll::Ready(Ok(()))
        }
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<scm_sdk::Result<()>> {
        self.poll_flush(cx)
    }
}
enum Reply {
    Socket(Arc<Probe>),
    Http(u16),
    Failure,
    Pending,
}
struct Connector {
    replies: Mutex<VecDeque<Reply>>,
    requests: Mutex<Vec<(Url, HeaderMap)>>,
}
impl Connector {
    fn new(replies: Vec<Reply>) -> Arc<Self> {
        Arc::new(Self {
            replies: Mutex::new(replies.into()),
            requests: Mutex::new(Vec::new()),
        })
    }
    fn calls(&self) -> usize {
        self.requests.lock().unwrap().len()
    }
}
impl WebSocketConnector for Connector {
    fn connect<'a>(
        &'a self,
        request: WebSocketRequest,
    ) -> BoxFuture<'a, scm_sdk::Result<WebSocketUpgrade>> {
        Box::pin(async move {
            assert!(!format!("{request:?}").contains("synthetic-runtime-key"));
            assert!(request.headers().values().all(HeaderValue::is_sensitive));
            self.requests
                .lock()
                .unwrap()
                .push((request.url().clone(), request.headers().clone()));
            let reply = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected retry");
            match reply {
                Reply::Socket(probe) => {
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        "x-request-id",
                        HeaderValue::from_static("safe-realtime-123"),
                    );
                    WebSocketUpgrade::new(
                        StatusCode::SWITCHING_PROTOCOLS,
                        headers,
                        Box::new(Socket(probe)),
                    )
                }
                Reply::Http(code) => WebSocketUpgrade::new(
                    StatusCode::from_u16(code).unwrap(),
                    HeaderMap::new(),
                    Box::new(Socket(Arc::new(Probe::default()))),
                ),
                Reply::Failure => Err(Error::new(ErrorKind::Transport)),
                Reply::Pending => futures_util::future::pending().await,
            }
        })
    }
}
fn serial_limits() -> Limits {
    let mut limits = Limits::default();
    limits.max_concurrency = 1;
    limits
}
fn runtime(limits: Limits) -> RuntimeClient {
    RuntimeClient::new(
        Client::builder(Endpoint::parse("https://runtime.example/v1").unwrap())
            .authentication(
                Authentication::runtime_key(Secret::new("synthetic-runtime-key").unwrap()).unwrap(),
            )
            .transport(Arc::new(NoHttp))
            .limits(limits)
            .build()
            .unwrap(),
    )
    .unwrap()
}
fn query() -> GatewayRealtimeConnectRequest {
    serde_json::from_value(json!({"model":"@provider/unchanged","extra":"a&b?c"})).unwrap()
}
fn event() -> GatewayRealtimeEvent {
    serde_json::from_value(json!({"type":"response.create","unknown":{"kept":true}})).unwrap()
}
fn text(value: serde_json::Value) -> WebSocketFrame {
    WebSocketFrame::Text(Bytes::from(serde_json::to_vec(&value).unwrap()))
}

#[tokio::test]
async fn standard_stream_adapter_owns_and_releases_connection() {
    let probe = Arc::new(Probe {
        incoming: Mutex::new(vec![Ok(text(json!({"type":"session.created"})))].into()),
        ..Probe::default()
    });
    let connection = runtime(Limits::default())
        .connect_realtime(
            &query(),
            &GatewayRealtimeOptions::new(Connector::new(vec![Reply::Socket(probe.clone())])),
            None,
        )
        .await
        .unwrap();
    let mut stream = connection.into_stream().boxed();
    assert!(stream.next().await.unwrap().is_ok());
    drop(stream);
    assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn prepared_upgrade_pins_runtime_key_query_and_routing_without_oauth_or_http_defaults() {
    let probe = Arc::new(Probe {
        incoming: Mutex::new(
            vec![
                Ok(text(
                    json!({"type":"session.created","unknown":[1,true,null]}),
                )),
                Ok(WebSocketFrame::Close(1000)),
            ]
            .into(),
        ),
        ..Probe::default()
    });
    let connector = Connector::new(vec![Reply::Socket(probe.clone())]);
    let runtime = runtime(Limits::default())
        .with_routing(
            scm_sdk::GatewayRouting::new()
                .header("OpenAI-Beta", "realtime=v1")
                .unwrap()
                .header("x-portkey-provider", "test")
                .unwrap(),
        )
        .unwrap();
    let mut connection = runtime
        .connect_realtime(
            &query(),
            &GatewayRealtimeOptions::new(connector.clone()),
            None,
        )
        .await
        .unwrap();
    {
        let requests = connector.requests.lock().unwrap();
        let (url, headers) = &requests[0];
        assert_eq!(url.scheme(), "wss");
        assert_eq!(url.host_str(), Some("runtime.example"));
        assert_eq!(url.path(), "/v1/realtime");
        assert_eq!(
            url.query_pairs().collect::<Vec<_>>(),
            vec![
                ("extra".into(), "a&b?c".into()),
                ("model".into(), "@provider/unchanged".into())
            ]
        );
        assert_eq!(headers.len(), 3);
        assert_eq!(headers["x-portkey-api-key"], "synthetic-runtime-key");
        assert_eq!(headers["openai-beta"], "realtime=v1");
        assert!(!headers.contains_key("authorization"));
        assert!(!headers.contains_key("x-tsg-id"));
        assert!(!headers.contains_key("user-agent"));
        assert!(!headers.contains_key("service-name"));
    }
    assert_eq!(
        serde_json::to_value(connection.next().await.unwrap().unwrap()).unwrap(),
        json!({"type":"session.created","unknown":[1,true,null]})
    );
    assert!(connection.next().await.unwrap().is_none());
    assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
    assert!(connection.next().await.unwrap().is_none());
}

#[tokio::test]
async fn unknown_provider_error_events_are_faithful_and_do_not_imply_provider_readiness() {
    let expected = json!({"type":"error","error":{"code":"future_provider_value","message":"synthetic sensitive provider text"}});
    let probe = Arc::new(Probe {
        incoming: Mutex::new(vec![Ok(text(expected.clone()))].into()),
        ..Probe::default()
    });
    let connector = Connector::new(vec![Reply::Socket(probe.clone())]);
    let mut connection = runtime(Limits::default())
        .connect_realtime(&query(), &GatewayRealtimeOptions::new(connector), None)
        .await
        .unwrap();
    let received = connection.next().await.unwrap().unwrap();
    assert_eq!(serde_json::to_value(&received).unwrap(), expected);
    assert!(!format!("{connection:?} {received:?}").contains("sensitive"));
    drop(connection);
    assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn malformed_binary_oversized_and_abnormal_endings_latch_errors_and_drop_transport() {
    for frame in [
        WebSocketFrame::Text(Bytes::from_static(b"not json secret")),
        text(json!({"type":""})),
        WebSocketFrame::Binary(Bytes::from_static(b"binary secret")),
        WebSocketFrame::Close(1005),
        WebSocketFrame::Close(1011),
        WebSocketFrame::Text(Bytes::from(vec![b'x'; 257])),
    ] {
        let probe = Arc::new(Probe {
            incoming: Mutex::new(vec![Ok(frame)].into()),
            ..Probe::default()
        });
        let connector = Connector::new(vec![Reply::Socket(probe.clone())]);
        let mut settings = GatewayRealtimeOptions::new(connector);
        settings.max_event_bytes = 256;
        let mut connection = runtime(Limits::default())
            .connect_realtime(&query(), &settings, None)
            .await
            .unwrap();
        let error = connection.next().await.unwrap_err();
        assert_eq!(error.status(), Some(101));
        assert_eq!(error.request_id(), Some("safe-realtime-123"));
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        assert!(!format!("{error:?} {error}").contains("secret"));
        assert_eq!(connection.next().await.unwrap_err().kind(), error.kind());
        assert_eq!(
            connection.send(&event()).await.unwrap_err().kind(),
            error.kind()
        );
        assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
    }
    let probe = Arc::new(Probe {
        eof: true,
        ..Probe::default()
    });
    let mut connection = runtime(Limits::default())
        .connect_realtime(
            &query(),
            &GatewayRealtimeOptions::new(Connector::new(vec![Reply::Socket(probe)])),
            None,
        )
        .await
        .unwrap();
    assert_eq!(
        connection.next().await.unwrap_err().kind(),
        ErrorKind::Protocol
    );
}

#[tokio::test]
async fn sends_are_validated_bounded_and_never_replayed() {
    let probe = Arc::new(Probe::default());
    let mut connection = runtime(Limits::default())
        .connect_realtime(
            &query(),
            &GatewayRealtimeOptions::new(Connector::new(vec![Reply::Socket(probe.clone())])),
            None,
        )
        .await
        .unwrap();
    connection.send(&event()).await.unwrap();
    {
        let writes = probe.writes.lock().unwrap();
        let WebSocketFrame::Text(bytes) = &writes[0] else {
            panic!("expected text")
        };
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(bytes).unwrap(),
            serde_json::to_value(event()).unwrap()
        );
    }
    let invalid: GatewayRealtimeEvent = serde_json::from_value(json!({"type":""})).unwrap();
    assert_eq!(
        connection.send(&invalid).await.unwrap_err().kind(),
        ErrorKind::Validation
    );
    assert_eq!(probe.writes.lock().unwrap().len(), 1);
    assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
}

#[tokio::test(start_paused = true)]
async fn cancelled_partial_send_releases_socket_and_retains_uncertainty() {
    let probe = Arc::new(Probe::default());
    probe.flush_blocked.store(true, Ordering::SeqCst);
    let mut connection = runtime(Limits::default())
        .connect_realtime(
            &query(),
            &GatewayRealtimeOptions::new(Connector::new(vec![Reply::Socket(probe.clone())])),
            None,
        )
        .await
        .unwrap();
    assert!(
        tokio::time::timeout(Duration::from_millis(5), connection.send(&event()))
            .await
            .is_err()
    );
    assert_eq!(probe.writes.lock().unwrap().len(), 1);
    assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
    let error = connection.next().await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
}

#[tokio::test]
async fn split_allows_simultaneous_read_write_and_either_half_drop_terminates_every_owner() {
    let probe = Arc::new(Probe {
        echo_on_write: true,
        ..Probe::default()
    });
    let connection = runtime(Limits::default())
        .connect_realtime(
            &query(),
            &GatewayRealtimeOptions::new(Connector::new(vec![Reply::Socket(probe.clone())])),
            None,
        )
        .await
        .unwrap();
    let (mut sender, mut receiver) = connection.split().unwrap();
    let reading = tokio::spawn(async move {
        let event = receiver.next().await.unwrap().unwrap();
        (receiver, event)
    });
    tokio::task::yield_now().await;
    sender.send(&event()).await.unwrap();
    let (mut receiver, received) = reading.await.unwrap();
    assert_eq!(
        serde_json::to_value(received).unwrap()["type"],
        "session.created"
    );
    drop(sender);
    assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
    assert_eq!(
        receiver.next().await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
    let probe = Arc::new(Probe::default());
    let connection = runtime(Limits::default())
        .connect_realtime(
            &query(),
            &GatewayRealtimeOptions::new(Connector::new(vec![Reply::Socket(probe.clone())])),
            None,
        )
        .await
        .unwrap();
    let (mut sender, receiver) = connection.split().unwrap();
    drop(receiver);
    assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
    assert_eq!(
        sender.send(&event()).await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
}

#[tokio::test(start_paused = true)]
async fn explicit_close_is_bounded_and_drop_closes_even_without_iteration() {
    for auto_close in [true, false] {
        let probe = Arc::new(Probe {
            auto_close,
            ..Probe::default()
        });
        let mut settings =
            GatewayRealtimeOptions::new(Connector::new(vec![Reply::Socket(probe.clone())]));
        settings.close_timeout = Duration::from_millis(5);
        let mut connection = runtime(Limits::default())
            .connect_realtime(&query(), &settings, None)
            .await
            .unwrap();
        let result = connection.cancel().await;
        assert_eq!(result.is_ok(), auto_close);
        assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
        assert!(matches!(
            probe.writes.lock().unwrap()[0],
            WebSocketFrame::Close(1000)
        ));
    }
    let probe = Arc::new(Probe::default());
    let connection = runtime(Limits::default())
        .connect_realtime(
            &query(),
            &GatewayRealtimeOptions::new(Connector::new(vec![Reply::Socket(probe.clone())])),
            None,
        )
        .await
        .unwrap();
    drop(connection);
    assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
}

#[tokio::test(start_paused = true)]
async fn lifecycle_cancellation_idle_and_absolute_deadlines_release_every_resource() {
    for cancel in [false, true] {
        let probe = Arc::new(Probe::default());
        let options = RequestOptions::with_timeout(Duration::from_millis(20)).unwrap();
        let mut settings =
            GatewayRealtimeOptions::new(Connector::new(vec![Reply::Socket(probe.clone())]));
        settings.idle_timeout = Duration::from_millis(5);
        let mut connection = runtime(Limits::default())
            .connect_realtime(&query(), &settings, Some(&options))
            .await
            .unwrap();
        if cancel {
            options.cancel();
        }
        let error = connection.next().await.unwrap_err();
        assert_eq!(
            error.kind(),
            if cancel {
                ErrorKind::Cancelled
            } else {
                ErrorKind::Deadline
            }
        );
        assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
    }
    let probe = Arc::new(Probe::default());
    let options = RequestOptions::with_timeout(Duration::from_millis(20)).unwrap();
    let mut settings =
        GatewayRealtimeOptions::new(Connector::new(vec![Reply::Socket(probe.clone())]));
    settings.idle_timeout = Duration::from_secs(1);
    let mut connection = runtime(Limits::default())
        .connect_realtime(&query(), &settings, Some(&options))
        .await
        .unwrap();
    tokio::time::advance(Duration::from_millis(21)).await;
    assert_eq!(
        connection.next().await.unwrap_err().kind(),
        ErrorKind::Deadline
    );
    assert_eq!(probe.dropped.load(Ordering::SeqCst), 1);
}

#[tokio::test(start_paused = true)]
async fn queue_and_handshake_share_original_budget_and_release_permits_without_replay() {
    let first = Arc::new(Probe::default());
    let second = Arc::new(Probe::default());
    let connector = Connector::new(vec![Reply::Socket(first.clone()), Reply::Socket(second)]);
    let client = runtime(serial_limits());
    let settings = GatewayRealtimeOptions::new(connector.clone());
    let connection = client
        .connect_realtime(&query(), &settings, None)
        .await
        .unwrap();
    let options = RequestOptions::with_timeout(Duration::from_millis(5)).unwrap();
    let error = client
        .connect_realtime(&query(), &settings, Some(&options))
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Deadline);
    assert_eq!(error.remote_outcome(), RemoteOutcome::NotSent);
    assert_eq!(connector.calls(), 1);
    drop(connection);
    let connection = client
        .connect_realtime(&query(), &settings, None)
        .await
        .unwrap();
    drop(connection);
    assert_eq!(connector.calls(), 2);
    for reply in [Reply::Failure, Reply::Http(401), Reply::Http(503)] {
        let connector = Connector::new(vec![reply]);
        let options = RequestOptions::with_timeout(Duration::from_secs(1))
            .unwrap()
            .with_retries(5)
            .unwrap();
        let error = runtime(Limits::default())
            .connect_realtime(
                &query(),
                &GatewayRealtimeOptions::new(connector.clone()),
                Some(&options),
            )
            .await
            .unwrap_err();
        assert_eq!(connector.calls(), 1);
        assert_ne!(error.remote_outcome(), RemoteOutcome::NotSent);
    }
    let connector = Connector::new(vec![
        Reply::Pending,
        Reply::Socket(Arc::new(Probe::default())),
    ]);
    let mut settings = GatewayRealtimeOptions::new(connector.clone());
    settings.handshake_timeout = Duration::from_millis(5);
    let client = runtime(serial_limits());
    assert_eq!(
        client
            .connect_realtime(&query(), &settings, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Deadline
    );
    drop(
        client
            .connect_realtime(&query(), &settings, None)
            .await
            .unwrap(),
    );
    assert_eq!(connector.calls(), 2);
}

struct RejectRealtime(AtomicUsize);
impl DestinationPolicy for RejectRealtime {
    fn check(&self, url: &Url, purpose: DestinationPurpose) -> scm_sdk::Result<()> {
        if purpose == DestinationPurpose::Realtime {
            assert_eq!(url.scheme(), "wss");
            self.0.fetch_add(1, Ordering::SeqCst);
            Err(Error::new(ErrorKind::Destination))
        } else {
            Ok(())
        }
    }
}
#[tokio::test]
async fn destination_invalid_bounds_and_precancellation_fail_before_connector_or_secret_dispatch() {
    let connector = Connector::new(vec![]);
    let mut settings = GatewayRealtimeOptions::new(connector.clone());
    settings.max_buffered_events = 0;
    assert_eq!(
        runtime(Limits::default())
            .connect_realtime(&query(), &settings, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Configuration
    );
    settings.max_buffered_events = 256;
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    options.cancel();
    assert_eq!(
        runtime(Limits::default())
            .connect_realtime(&query(), &settings, Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    let policy = Arc::new(RejectRealtime(AtomicUsize::new(0)));
    let client = RuntimeClient::new(
        Client::builder(Endpoint::parse("https://runtime.example/v1").unwrap())
            .destination_policy(policy.clone())
            .authentication(
                Authentication::runtime_key(Secret::new("synthetic-runtime-key").unwrap()).unwrap(),
            )
            .transport(Arc::new(NoHttp))
            .build()
            .unwrap(),
    )
    .unwrap();
    assert_eq!(
        client
            .connect_realtime(&query(), &settings, None)
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Destination
    );
    assert_eq!(policy.0.load(Ordering::SeqCst), 1);
    assert_eq!(connector.calls(), 0);
}
