use super::*;
use crate::{
    Client, Endpoint, OriginPolicy, RawRequest, RequestOptions, Secret, Session,
    auth::{AuthContext, OAuthProvider},
    transport::{Transport, TransportRequest, TransportResponse},
};
use bytes::Bytes;
use futures_util::future::BoxFuture;
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use std::{
    collections::{BTreeMap, VecDeque},
    sync::{
        Arc, Mutex, OnceLock,
        atomic::{AtomicUsize, Ordering},
    },
    time::Duration,
};
use tracing::{
    Event, Metadata, Subscriber,
    field::{Field, Visit},
    span::{Attributes, Id, Record},
};
#[derive(Clone)]
struct Capture(Arc<Mutex<Vec<BTreeMap<String, String>>>>);
impl Default for Capture {
    fn default() -> Self {
        // A subscriberless sibling can first reach the SDK callsite while a
        // sole scoped subscriber raises the global level. tracing-core 0.1.36
        // then caches that sibling's Never interest through its JustOne path.
        // Register an inert dispatcher first: its OFF hint keeps the callsite
        // disabled until a real capture exists, when both registrars force the
        // context-aware path. It is never installed as a global/default sink.
        static INERT: OnceLock<tracing::Dispatch> = OnceLock::new();
        INERT.get_or_init(|| tracing::Dispatch::new(tracing::subscriber::NoSubscriber::default()));
        Self(Arc::default())
    }
}
impl Subscriber for Capture {
    fn enabled(&self, metadata: &Metadata<'_>) -> bool {
        metadata.target() == "scm_sdk::lifecycle"
    }
    fn new_span(&self, _: &Attributes<'_>) -> Id {
        Id::from_u64(1)
    }
    fn record(&self, _: &Id, _: &Record<'_>) {}
    fn record_follows_from(&self, _: &Id, _: &Id) {}
    fn event(&self, event: &Event<'_>) {
        if self.enabled(event.metadata()) {
            assert!(
                event.is_root(),
                "SDK events do not inherit application span fields"
            );
            let mut fields = Fields(BTreeMap::new());
            event.record(&mut fields);
            self.0.lock().unwrap().push(fields.0);
        }
    }
    fn enter(&self, _: &Id) {}
    fn exit(&self, _: &Id) {}
}

#[test]
fn subscriberless_first_callsite_registration_keeps_local_capture_enabled() {
    let capture = Capture::default();
    tracing::dispatcher::get_default(|current| {
        assert!(current.is::<tracing::subscriber::NoSubscriber>());
    });
    let dispatch = tracing::Dispatch::new(capture.clone());
    let _guard = tracing::dispatcher::set_default(&dispatch);
    std::thread::spawn(|| {
        Lifecycle::start(Operation::BodyStream).success();
    })
    .join()
    .unwrap();
    assert!(capture.0.lock().unwrap().is_empty());
    Lifecycle::received(Operation::BodyStream, 200, false).success();
    assert!(capture.contains("sdk.stream.body", "complete", "none", "success", "200"));
    capture.safe();
}

struct Fields(BTreeMap<String, String>);
impl Visit for Fields {
    fn record_str(&mut self, f: &Field, v: &str) {
        self.0.insert(f.name().into(), v.into());
    }
    fn record_bool(&mut self, f: &Field, v: bool) {
        self.0.insert(f.name().into(), v.to_string());
    }
    fn record_u64(&mut self, f: &Field, v: u64) {
        self.0.insert(f.name().into(), v.to_string());
    }
    fn record_debug(&mut self, f: &Field, v: &dyn std::fmt::Debug) {
        self.0.insert(f.name().into(), format!("{v:?}"));
    }
}
impl Capture {
    fn contains(
        &self,
        operation: &str,
        stage: &str,
        kind: &str,
        outcome: &str,
        status: &str,
    ) -> bool {
        self.0.lock().unwrap().iter().any(|e| {
            e["operation"] == operation
                && e["stage"] == stage
                && e["kind"] == kind
                && e["outcome"] == outcome
                && e["status"] == status
        })
    }
    fn safe(&self) {
        let events = self.0.lock().unwrap();
        assert!(!events.is_empty());
        for e in events.iter() {
            assert_eq!(
                e.keys().map(String::as_str).collect::<Vec<_>>(),
                [
                    "has_correlation",
                    "kind",
                    "operation",
                    "outcome",
                    "stage",
                    "status"
                ]
            );
            assert!(
                [
                    "sdk.http.exchange",
                    "sdk.http.stream_handshake",
                    "sdk.http.decode",
                    "sdk.oauth.refresh",
                    "sdk.oauth.cache",
                    "sdk.stream.body",
                    "sdk.stream.typed",
                    "sdk.realtime.upgrade",
                    "sdk.realtime.session"
                ]
                .contains(&e["operation"].as_str())
            );
            assert!(
                [
                    "start", "dispatch", "response", "complete", "failure", "drop", "closed"
                ]
                .contains(&e["stage"].as_str())
            );
            assert!(
                [
                    "none",
                    "configuration",
                    "validation",
                    "destination",
                    "authentication",
                    "io",
                    "transport",
                    "http",
                    "decode",
                    "limit",
                    "deadline",
                    "cancelled",
                    "protocol",
                    "pagination"
                ]
                .contains(&e["kind"].as_str())
            );
            assert!(
                ["not_sent", "rejected", "unknown", "success"].contains(&e["outcome"].as_str())
            );
            let status = e["status"].parse::<u16>().unwrap();
            assert!(status == 0 || (100..=599).contains(&status));
            assert!(["true", "false"].contains(&e["has_correlation"].as_str()));
            assert!(!format!("{e:?}").contains("secret"));
        }
    }
}
enum Reply {
    Bytes(StatusCode, Bytes),
    Pending,
    Failure(Error),
}
struct Wire {
    replies: Mutex<VecDeque<Reply>>,
    calls: AtomicUsize,
}
impl Wire {
    fn new(replies: Vec<Reply>) -> Arc<Self> {
        Arc::new(Self {
            replies: Mutex::new(replies.into()),
            calls: AtomicUsize::new(0),
        })
    }
}
impl Transport for Wire {
    fn send<'a>(&'a self, _: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            let reply = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected replay");
            match reply {
                Reply::Pending => std::future::pending().await,
                Reply::Failure(e) => Err(e),
                Reply::Bytes(status, bytes) => {
                    let mut headers = HeaderMap::new();
                    headers.insert(
                        "x-request-id",
                        HeaderValue::from_static("correlation-secret"),
                    );
                    headers.insert("x-private", HeaderValue::from_static("header-secret"));
                    if bytes.starts_with(b"data:") {
                        headers.insert(
                            "content-type",
                            HeaderValue::from_static("text/event-stream"),
                        );
                    }
                    Ok(TransportResponse::from_bytes(status, headers, bytes))
                }
            }
        })
    }
}
fn client(wire: Arc<Wire>) -> Client {
    Client::builder(Endpoint::parse("https://secret.example/private-secret").unwrap())
        .transport(wire)
        .build()
        .unwrap()
}
fn secret_request() -> RawRequest {
    RawRequest::new(Method::POST, "/path-secret")
        .named("caller-operation-secret")
        .query(vec![("query-secret".into(), "value-secret".into())])
        .header("x-private", "header-secret")
        .unwrap()
        .json_value(
            &serde_json::json!({"model":"model-secret","body":"payload-secret"}),
            4096,
        )
        .unwrap()
}
#[tokio::test]
async fn http_and_decode_events_capture_status_presence_and_no_echoed_data() {
    let capture = Capture::default();
    let dispatch = tracing::Dispatch::new(capture.clone());
    let _guard = tracing::dispatcher::set_default(&dispatch);
    let w = Wire::new(vec![
        Reply::Bytes(
            StatusCode::SERVICE_UNAVAILABLE,
            Bytes::from_static(
                br#"{"message":"server-message-secret","data":{"errorCode":"code-secret"}}"#,
            ),
        ),
        Reply::Bytes(StatusCode::OK, Bytes::from_static(b"invalid-json-secret")),
        Reply::Bytes(
            StatusCode::BAD_REQUEST,
            Bytes::from_static(b"rejected-secret"),
        ),
    ]);
    let c = client(w.clone());
    assert!(c.send_empty(secret_request(), None).await.is_err());
    assert!(
        c.send_json::<serde_json::Value>(secret_request(), None)
            .await
            .is_err()
    );
    c.send_empty(secret_request(), None).await.unwrap_err();
    assert_eq!(w.calls.load(Ordering::SeqCst), 3);
    assert!(capture.contains("sdk.http.exchange", "failure", "http", "rejected", "400"));
    assert!(capture.contains("sdk.http.exchange", "failure", "http", "unknown", "503"));
    assert!(capture.contains("sdk.http.decode", "failure", "decode", "unknown", "200"));
    assert!(
        capture
            .0
            .lock()
            .unwrap()
            .iter()
            .any(|e| e["status"] == "503" && e["has_correlation"] == "true")
    );
    capture.safe();
}
#[tokio::test]
async fn cancellation_and_dropped_http_futures_distinguish_pre_dispatch_from_uncertainty() {
    let capture = Capture::default();
    let dispatch = tracing::Dispatch::new(capture.clone());
    let _guard = tracing::dispatcher::set_default(&dispatch);
    let w = Wire::new(vec![Reply::Pending]);
    let c = client(w.clone());
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    options.cancel();
    assert_eq!(
        c.send_empty(secret_request(), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert!(capture.contains("sdk.http.exchange", "failure", "cancelled", "not_sent", "0"));
    let mut future = Box::pin(c.send_empty(secret_request(), None));
    assert!(futures_util::poll!(future.as_mut()).is_pending());
    drop(future);
    assert!(capture.contains("sdk.http.exchange", "drop", "cancelled", "unknown", "0"));
    assert_eq!(w.calls.load(Ordering::SeqCst), 1);
    capture.safe();
}
#[tokio::test]
async fn dropping_a_queued_http_future_does_not_claim_dispatch() {
    let capture = Capture::default();
    let dispatch = tracing::Dispatch::new(capture.clone());
    let _guard = tracing::dispatcher::set_default(&dispatch);
    let wire = Wire::new(vec![Reply::Bytes(
        StatusCode::OK,
        Bytes::from_static(b"body-secret"),
    )]);
    let client = Client::builder(Endpoint::parse("https://secret.example").unwrap())
        .transport(wire.clone())
        .limits(crate::Limits {
            max_concurrency: 1,
            ..crate::Limits::default()
        })
        .build()
        .unwrap();
    let body = client.send_stream(secret_request(), None).await.unwrap();
    let mut queued = Box::pin(client.send_empty(secret_request(), None));
    assert!(futures_util::poll!(queued.as_mut()).is_pending());
    drop(queued);
    assert!(capture.contains("sdk.http.exchange", "drop", "cancelled", "not_sent", "0"));
    assert_eq!(wire.calls.load(Ordering::SeqCst), 1);
    drop(body);
    capture.safe();
}
#[tokio::test(start_paused = true)]
async fn deadline_and_adapter_custom_errors_never_format_external_error_values() {
    let capture = Capture::default();
    let dispatch = tracing::Dispatch::new(capture.clone());
    let _guard = tracing::dispatcher::set_default(&dispatch);
    let error = Error::custom("adapter-message-secret", None, crate::ErrorMetadata::new()).unwrap();
    let w = Wire::new(vec![Reply::Pending, Reply::Failure(error)]);
    let c = client(w);
    let options = RequestOptions::with_timeout(Duration::from_millis(1)).unwrap();
    assert_eq!(
        c.send_empty(secret_request(), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Deadline
    );
    c.send_empty(secret_request(), None).await.unwrap_err();
    assert!(capture.contains("sdk.http.exchange", "failure", "deadline", "unknown", "0"));
    capture.safe();
}
#[tokio::test]
async fn oauth_refresh_publication_cache_hits_and_dropped_exchange_are_observable_without_identity()
{
    let capture = Capture::default();
    let dispatch = tracing::Dispatch::new(capture.clone());
    let _guard = tracing::dispatcher::set_default(&dispatch);
    let w = Wire::new(vec![
        Reply::Bytes(
            StatusCode::OK,
            Bytes::from_static(
                br#"{"access_token":"bearer-secret","token_type":"Bearer","expires_in":3600}"#,
            ),
        ),
        Reply::Pending,
    ]);
    let issuer = Endpoint::parse("https://issuer-secret.example/path-secret").unwrap();
    let provider = OAuthProvider::new(
        AuthContext::new(issuer.clone(), "client-secret", 1, "123").unwrap(),
        Secret::new("credential-secret").unwrap(),
        w.clone(),
        Arc::new(OriginPolicy::new([issuer])),
        Session::new(),
    )
    .unwrap();
    provider.get_token(None).await.unwrap();
    provider.get_token(None).await.unwrap();
    assert_eq!(w.calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        capture
            .0
            .lock()
            .unwrap()
            .iter()
            .filter(|e| e["operation"] == "sdk.oauth.refresh" && e["stage"] == "complete")
            .count(),
        1
    );
    assert!(capture.contains("sdk.oauth.cache", "complete", "none", "success", "0"));
    provider.clear().unwrap();
    let mut future = Box::pin(provider.get_token(None));
    assert!(futures_util::poll!(future.as_mut()).is_pending());
    drop(future);
    assert!(capture.contains("sdk.oauth.refresh", "drop", "cancelled", "unknown", "0"));
    capture.safe();
}
#[tokio::test]
async fn body_stream_eof_cancel_close_and_drop_are_separate_owned_lifecycles() {
    let capture = Capture::default();
    let dispatch = tracing::Dispatch::new(capture.clone());
    let _guard = tracing::dispatcher::set_default(&dispatch);
    let w = Wire::new(
        (0..4)
            .map(|_| Reply::Bytes(StatusCode::OK, Bytes::from_static(b"body-secret")))
            .collect(),
    );
    let c = client(w);
    let mut body = c.send_stream(secret_request(), None).await.unwrap();
    assert!(body.next().await.unwrap().is_some());
    assert!(body.next().await.unwrap().is_none());
    let mut body = c.send_stream(secret_request(), None).await.unwrap();
    body.close();
    let body = c.send_stream(secret_request(), None).await.unwrap();
    drop(body);
    let mut body = c.send_stream(secret_request(), None).await.unwrap();
    c.session().cancel();
    body.next().await.unwrap_err();
    assert!(capture.contains("sdk.stream.body", "complete", "none", "success", "200"));
    assert!(capture.contains("sdk.stream.body", "closed", "none", "unknown", "200"));
    assert!(capture.contains("sdk.stream.body", "drop", "cancelled", "unknown", "200"));
    assert!(capture.contains("sdk.stream.body", "failure", "cancelled", "unknown", "200"));
    capture.safe();
}
#[test]
fn metadata_vocabulary_and_status_range_are_closed_even_for_caller_supplied_errors() {
    let capture = Capture::default();
    let dispatch = tracing::Dispatch::new(capture.clone());
    let _guard = tracing::dispatcher::set_default(&dispatch);
    let lifecycle = Lifecycle::start(Operation::Http);
    lifecycle.failed(&Error::new(ErrorKind::Http).with_status(999));
    assert!(capture.contains("sdk.http.exchange", "failure", "http", "not_sent", "0"));
    capture.safe();
}
#[cfg(feature = "gateway")]
#[tokio::test]
async fn typed_stream_validation_failure_is_distinct_from_successful_http_body() {
    let capture = Capture::default();
    let dispatch = tracing::Dispatch::new(capture.clone());
    let _guard = tracing::dispatcher::set_default(&dispatch);
    let w = Wire::new(vec![Reply::Bytes(
        StatusCode::OK,
        Bytes::from_static(b"data: {\"type\":\"\",\"payload\":\"event-secret\"}\n\n"),
    )]);
    let mut body = client(w)
        .send_contract_stream::<serde_json::Value>(
            secret_request(),
            "GatewayRealtimeEventSchema",
            crate::streaming::TerminalPolicy::Eof,
            None,
        )
        .await
        .unwrap();
    body.next().await.unwrap_err();
    assert!(capture.contains("sdk.stream.typed", "failure", "decode", "unknown", "200"));
    capture.safe();
}
#[cfg(feature = "realtime")]
mod realtime {
    use super::*;
    use crate::realtime::{
        BoxWebSocket, GatewayRealtimeOptions, WebSocketConnector, WebSocketFrame, WebSocketRequest,
        WebSocketUpgrade,
    };
    use futures_util::{Sink, Stream};
    use std::{
        pin::Pin,
        task::{Context, Poll},
    };
    struct Socket {
        incoming: VecDeque<Result<WebSocketFrame>>,
    }
    impl Stream for Socket {
        type Item = Result<WebSocketFrame>;
        fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.incoming.pop_front())
        }
    }
    impl Sink<WebSocketFrame> for Socket {
        type Error = Error;
        fn poll_ready(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
            Poll::Ready(Ok(()))
        }
        fn start_send(self: Pin<&mut Self>, _: WebSocketFrame) -> Result<()> {
            Ok(())
        }
        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
            Poll::Ready(Ok(()))
        }
        fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Result<()>> {
            Poll::Ready(Ok(()))
        }
    }
    struct Connector(Mutex<VecDeque<BoxWebSocket>>);
    impl WebSocketConnector for Connector {
        fn connect<'a>(&'a self, _: WebSocketRequest) -> BoxFuture<'a, Result<WebSocketUpgrade>> {
            Box::pin(async move {
                let mut headers = HeaderMap::new();
                headers.insert(
                    "x-request-id",
                    HeaderValue::from_static("correlation-secret"),
                );
                WebSocketUpgrade::new(
                    StatusCode::SWITCHING_PROTOCOLS,
                    headers,
                    self.0.lock().unwrap().pop_front().unwrap(),
                )
            })
        }
    }
    #[tokio::test]
    async fn realtime_upgrade_clean_close_malformed_event_and_drop_use_only_fixed_metadata() {
        let capture = Capture::default();
        let dispatch = tracing::Dispatch::new(capture.clone());
        let _guard = tracing::dispatcher::set_default(&dispatch);
        let c = Client::builder(Endpoint::parse("https://secret.example/private-secret").unwrap())
            .transport(Wire::new(vec![]))
            .authentication(
                crate::auth::Authentication::runtime_key(Secret::new("runtime-secret").unwrap())
                    .unwrap(),
            )
            .build()
            .unwrap();
        let sockets: Vec<BoxWebSocket> = vec![
            Box::new(Socket {
                incoming: vec![Ok(WebSocketFrame::Close(1000))].into(),
            }),
            Box::new(Socket {
                incoming: vec![Ok(WebSocketFrame::Text(Bytes::from_static(
                    b"event-secret-invalid-json",
                )))]
                .into(),
            }),
            Box::new(Socket {
                incoming: vec![].into(),
            }),
        ];
        let settings = GatewayRealtimeOptions::new(Arc::new(Connector(Mutex::new(sockets.into()))));
        let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
        let mut connection = c
            .open_realtime(
                RawRequest::read("/v1/realtime").named("caller-operation-secret"),
                &settings,
                &options,
            )
            .await
            .unwrap();
        assert!(connection.next().await.unwrap().is_none());
        let mut connection = c
            .open_realtime(RawRequest::read("/v1/realtime"), &settings, &options)
            .await
            .unwrap();
        connection.next().await.unwrap_err();
        let connection = c
            .open_realtime(RawRequest::read("/v1/realtime"), &settings, &options)
            .await
            .unwrap();
        drop(connection);
        assert!(capture.contains("sdk.realtime.upgrade", "complete", "none", "success", "101"));
        assert!(capture.contains("sdk.realtime.session", "complete", "none", "success", "101"));
        assert!(capture.contains(
            "sdk.realtime.session",
            "failure",
            "decode",
            "unknown",
            "101"
        ));
        assert!(capture.contains(
            "sdk.realtime.session",
            "failure",
            "cancelled",
            "unknown",
            "101"
        ));
        capture.safe();
    }
}
