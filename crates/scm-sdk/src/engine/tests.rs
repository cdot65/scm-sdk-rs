use super::*;
use crate::{
    Secret,
    auth::{AuthContext, TokenLease, TokenProvider},
    transport::{BodyStream, TransportResponse},
};
use futures_util::{Stream, future::BoxFuture};
use std::{
    collections::VecDeque,
    pin::Pin,
    sync::{Mutex, atomic::AtomicUsize},
    task::{Context, Poll},
};
use tokio::sync::Notify;

enum Reply {
    Http(StatusCode, HeaderMap, Bytes),
    Failure,
    PendingBody {
        started: Arc<Notify>,
        dropped: Arc<AtomicUsize>,
    },
    BrokenBody,
    PendingMetadata(StatusCode, HeaderMap),
    LocalFailure(ErrorKind),
}
fn ok() -> Reply {
    Reply::Http(StatusCode::OK, HeaderMap::new(), Bytes::from_static(b"ok"))
}
fn status(code: u16) -> Reply {
    Reply::Http(
        StatusCode::from_u16(code).unwrap(),
        HeaderMap::new(),
        Bytes::new(),
    )
}
fn retry_after(seconds: &'static str) -> Reply {
    let mut headers = HeaderMap::new();
    headers.insert(http::header::RETRY_AFTER, HeaderValue::from_static(seconds));
    Reply::Http(StatusCode::TOO_MANY_REQUESTS, headers, Bytes::new())
}

struct Scripted {
    requests: Mutex<Vec<TransportRequest>>,
    replies: Mutex<VecDeque<Reply>>,
}
impl Scripted {
    fn new(replies: Vec<Reply>) -> Arc<Self> {
        Arc::new(Self {
            requests: Mutex::new(Vec::new()),
            replies: Mutex::new(replies.into()),
        })
    }
    fn count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }
}
struct PendingBody {
    started: Arc<Notify>,
    dropped: Arc<AtomicUsize>,
}
impl Stream for PendingBody {
    type Item = Result<Bytes>;
    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.started.notify_one();
        Poll::Pending
    }
}
impl Drop for PendingBody {
    fn drop(&mut self) {
        self.dropped.fetch_add(1, Ordering::SeqCst);
    }
}
impl Transport for Scripted {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.lock().unwrap().push(request);
            match self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected extra network request")
            {
                Reply::Http(status, headers, body) => {
                    Ok(TransportResponse::from_bytes(status, headers, body))
                }
                Reply::Failure => Err(Error::new(ErrorKind::Transport)),
                Reply::LocalFailure(kind) => Err(Error::new(kind)),
                Reply::PendingBody { started, dropped } => Ok(TransportResponse::new(
                    StatusCode::OK,
                    HeaderMap::new(),
                    Box::pin(PendingBody { started, dropped }),
                )),
                Reply::BrokenBody => {
                    let body: BodyStream = Box::pin(futures_util::stream::once(async {
                        Err(Error::new(ErrorKind::Transport))
                    }));
                    Ok(TransportResponse::new(
                        StatusCode::OK,
                        HeaderMap::new(),
                        body,
                    ))
                }
                Reply::PendingMetadata(status, headers) => Ok(TransportResponse::new(
                    status,
                    headers,
                    Box::pin(futures_util::stream::pending()),
                )),
            }
        })
    }
}
fn endpoint() -> Endpoint {
    Endpoint::parse("https://api.example/private-base").unwrap()
}
fn client(transport: Arc<Scripted>) -> Client {
    Client::builder(endpoint())
        .transport(transport)
        .limits(fast_limits())
        .build()
        .unwrap()
}
fn fast_limits() -> Limits {
    Limits {
        retry_base_delay: Duration::from_millis(10),
        max_retry_delay: Duration::from_secs(5),
        ..Limits::default()
    }
}

#[tokio::test]
async fn http_problem_fields_and_body_retry_guidance_are_sanitized_metadata_only() {
    let body = Bytes::from_static(br#"{"title":"Bad Request","detail":"echoed synthetic secret","errors":[{"property":"name","reason":"must not be blank"}],"retry_after":{"interval":2,"unit":"MIN"},"extension":"secret"}"#);
    let transport = Scripted::new(vec![Reply::Http(
        StatusCode::BAD_REQUEST,
        HeaderMap::new(),
        body,
    )]);
    let error = client(transport.clone())
        .send_bytes(RawRequest::read("/problem"), None)
        .await
        .unwrap_err();
    assert_eq!(transport.count(), 1);
    assert_eq!(error.failure_kind(), Some(crate::FailureKind::Http));
    assert_eq!(error.retry_after(), Some(Duration::from_secs(120)));
    let problem = error.problem().unwrap();
    assert_eq!(problem.title(), Some("Bad Request"));
    assert_eq!(problem.detail(), Some("[withheld]"));
    assert!(problem.redacted());
    assert_eq!(problem.errors().unwrap()[0].reason(), "must not be blank");
    assert!(!format!("{error:?} {error}").contains("secret"));
    let mut headers = HeaderMap::new();
    headers.insert(http::header::RETRY_AFTER, HeaderValue::from_static(" 2 "));
    let body = Bytes::from_static(br#"{"retry_after":{"interval":10,"unit":"s"}}"#);
    let transport = Scripted::new(vec![Reply::Http(StatusCode::BAD_REQUEST, headers, body)]);
    let error = client(transport)
        .send_bytes(RawRequest::read("/problem"), None)
        .await
        .unwrap_err();
    assert_eq!(error.retry_after(), Some(Duration::from_secs(2)));
    let mut headers = HeaderMap::new();
    headers.insert(
        http::header::RETRY_AFTER,
        HeaderValue::from_static("Thu, 01 Jan 1970 00:00:00 GMT"),
    );
    assert_eq!(super::retry_after(&headers), Some(Duration::ZERO));
    headers.insert(http::header::RETRY_AFTER, HeaderValue::from_static("+1"));
    assert_eq!(super::retry_after(&headers), None);
    let transport = Scripted::new(vec![Reply::Http(
        StatusCode::BAD_REQUEST,
        HeaderMap::new(),
        Bytes::from_static(
            br#"{"success":false,"data":{"errorCode":"AB01","message":"synthetic secret"}}"#,
        ),
    )]);
    let error = client(transport)
        .send_bytes(RawRequest::read("/problem"), None)
        .await
        .unwrap_err();
    assert_eq!(error.service_code(), Some("AB01"));
    assert!(error.problem().is_none());
    assert!(!format!("{error:?} {error}").contains("secret"));
}

#[tokio::test]
async fn failure_origin_is_preserved_even_after_successful_response_headers() {
    let error = client(Scripted::new(vec![Reply::BrokenBody]))
        .send_bytes(RawRequest::read("/body"), None)
        .await
        .unwrap_err();
    assert_eq!(error.status(), Some(200));
    assert_eq!(error.failure_kind(), Some(crate::FailureKind::Network));
    let error = client(Scripted::new(vec![Reply::Failure]))
        .send_bytes(
            RawRequest::read("/connection"),
            Some(
                &RequestOptions::with_timeout(Duration::from_secs(1))
                    .unwrap()
                    .with_retries(0)
                    .unwrap(),
            ),
        )
        .await
        .unwrap_err();
    assert_eq!(error.failure_kind(), Some(crate::FailureKind::Network));
    assert_eq!(error.status(), None);
}

#[cfg(feature = "gateway")]
#[tokio::test]
async fn typed_sse_standard_stream_adapter_releases_its_owned_permit() {
    use futures_util::StreamExt;
    let mut headers = HeaderMap::new();
    headers.insert(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static("text/event-stream"),
    );
    let transport = Scripted::new(vec![Reply::Http(
        StatusCode::OK,
        headers,
        Bytes::from_static(b"data: {\"type\":\"session.created\"}\n\ndata: [DONE]\n\n"),
    )]);
    let client = client(transport);
    let stream = client
        .send_contract_stream::<serde_json::Value>(
            RawRequest::read("/events"),
            "GatewayRealtimeEventSchema",
            crate::streaming::TerminalPolicy::DoneMarker,
            None,
        )
        .await
        .unwrap();
    assert_eq!(
        client.inner.permits.available_permits(),
        client.limits().max_concurrency - 1
    );
    let mut stream = stream.into_stream().boxed();
    assert_eq!(
        stream.next().await.unwrap().unwrap()["type"],
        "session.created"
    );
    drop(stream);
    assert_eq!(
        client.inner.permits.available_permits(),
        client.limits().max_concurrency
    );
}

#[tokio::test]
async fn successful_decode_failures_preserve_status_and_allowlisted_correlation() {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-correlation-id",
        HeaderValue::from_static("safe-request-123"),
    );
    let transport = Scripted::new(vec![Reply::Http(
        StatusCode::CREATED,
        headers,
        Bytes::from_static(b"upstream-private-secret"),
    )]);
    let error = client(transport)
        .send_json::<serde_json::Value>(RawRequest::new(Method::POST, "create"), None)
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert_eq!(error.status(), Some(201));
    assert_eq!(error.request_id(), Some("safe-request-123"));
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
    let formatted = format!("{error:?} {error}");
    assert!(!formatted.contains("upstream-private-secret"));
    assert!(!formatted.contains("safe-request-123"));
}

#[tokio::test]
async fn successful_body_limits_keep_status_and_correlation() {
    let mut headers = HeaderMap::new();
    headers.insert("x-request-id", HeaderValue::from_static("bounded-123"));
    let transport = Scripted::new(vec![Reply::Http(
        StatusCode::MULTI_STATUS,
        headers,
        Bytes::from_static(b"12345"),
    )]);
    let client = Client::builder(endpoint())
        .transport(transport)
        .limits(Limits {
            max_response_bytes: 3,
            ..fast_limits()
        })
        .build()
        .unwrap();
    let error = client.send_bytes(read(), None).await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Limit);
    assert_eq!(error.status(), Some(207));
    assert_eq!(error.request_id(), Some("bounded-123"));
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
}

#[tokio::test(start_paused = true)]
async fn deadline_after_response_headers_retains_status_and_correlation() {
    let mut headers = HeaderMap::new();
    headers.insert("x-request-id", HeaderValue::from_static("deadline-123"));
    let transport = Scripted::new(vec![Reply::PendingMetadata(StatusCode::CREATED, headers)]);
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    let error = client(transport)
        .send_bytes(RawRequest::new(Method::POST, "write"), Some(&options))
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Deadline);
    assert_eq!(error.status(), Some(201));
    assert_eq!(error.request_id(), Some("deadline-123"));
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
}

#[tokio::test]
async fn empty_json_success_hydrates_an_object_and_optional_json_preserves_absence() {
    let transport = Scripted::new(vec![status(204), status(204)]);
    let client = client(transport);
    assert_eq!(
        client
            .send_json::<serde_json::Value>(read(), None)
            .await
            .unwrap(),
        serde_json::json!({})
    );
    assert!(
        client
            .send_optional_json::<serde_json::Value>(read(), None)
            .await
            .unwrap()
            .is_none()
    );
}

#[cfg(feature = "management")]
#[tokio::test]
async fn typed_empty_response_hydration_and_decode_metadata_follow_the_named_contract() {
    let mut headers = HeaderMap::new();
    headers.insert("x-request-id", HeaderValue::from_static("typed-123"));
    let transport = Scripted::new(vec![
        status(204),
        Reply::Http(
            StatusCode::OK,
            headers,
            Bytes::from_static(b"{\"ai_profiles\":false}"),
        ),
    ]);
    let client = client(transport);
    let value: serde_json::Value = client
        .send_contract(read(), "PolicyLatencySchema", None)
        .await
        .unwrap();
    assert_eq!(value, serde_json::json!({}));
    let error = client
        .send_contract::<serde_json::Value>(read(), "SecurityProfileListResponseSchema", None)
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert_eq!(error.status(), Some(200));
    assert_eq!(error.request_id(), Some("typed-123"));
}

#[tokio::test(start_paused = true)]
async fn rebinding_endpoints_preserves_policy_provider_cancellation_and_shared_capacity() {
    let other = Endpoint::parse("https://other.example/scoped").unwrap();
    let transport = Scripted::new(vec![ok(), ok()]);
    let provider = CountingToken::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), false))
        .destination_policy(Arc::new(OriginPolicy::new([endpoint(), other.clone()])))
        .limits(Limits {
            max_concurrency: 1,
            ..fast_limits()
        })
        .build()
        .unwrap();
    let child = client
        .with_endpoint(other)
        .unwrap()
        .with_tsg_header(true)
        .unwrap();
    assert_eq!(
        child.oauth_context().unwrap().tsg_id(),
        client.oauth_context().unwrap().tsg_id()
    );
    let mut response = child.send_stream(read(), None).await.unwrap();
    let options = RequestOptions::with_timeout(Duration::from_millis(10)).unwrap();
    assert_eq!(
        client
            .send_bytes(read(), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Deadline
    );
    assert_eq!(transport.count(), 1);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(
        transport.requests.lock().unwrap()[0]
            .headers()
            .get("x-tsg-id")
            .unwrap(),
        provider.context().tsg_id()
    );
    response.close();
    client.session().cancel();
    assert_eq!(
        child.send_bytes(read(), None).await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
    let unlisted = Endpoint::parse("https://unlisted.example").unwrap();
    assert_eq!(
        client.with_endpoint(unlisted).unwrap_err().kind(),
        ErrorKind::Destination
    );
    assert_eq!(transport.count(), 1);
}

#[tokio::test]
async fn structured_identifiers_encode_once_and_reject_recursive_traversal() {
    let transport = Scripted::new(vec![ok()]);
    client(transport.clone())
        .send_bytes(
            RawRequest::read("models")
                .segment("provider/model 1%")
                .unwrap(),
            None,
        )
        .await
        .unwrap();
    assert_eq!(
        transport.requests.lock().unwrap()[0].url().path(),
        "/private-base/models/provider%2Fmodel%201%25"
    );
    for value in [
        ".",
        "..",
        "../secret",
        "a/../b",
        "..%2fsecret",
        "%252e%252e%252fsecret",
        "%00",
        "a\\b",
    ] {
        assert_eq!(
            RawRequest::read("models")
                .segment(value)
                .unwrap_err()
                .kind(),
            ErrorKind::Validation
        );
    }
}

#[tokio::test]
async fn assembled_query_and_authenticated_headers_obey_total_bounds() {
    for query in [
        vec![("x".into(), "é".repeat(4000))],
        vec![("x".into(), "v".into()); 257],
    ] {
        let transport = Scripted::new(vec![]);
        let provider = CountingToken::new();
        let c = Client::builder(endpoint())
            .transport(transport.clone())
            .authentication(Authentication::oauth(provider.clone(), true))
            .build()
            .unwrap();
        assert_eq!(
            c.send_bytes(read().query(query), None)
                .await
                .unwrap_err()
                .kind(),
            ErrorKind::Limit
        );
        assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
        assert_eq!(transport.count(), 0);
    }
    for large_value in [false, true] {
        let transport = Scripted::new(vec![]);
        let key = if large_value {
            "k".repeat(16 * 1024)
        } else {
            "key".to_owned()
        };
        let c = Client::builder(endpoint())
            .transport(transport.clone())
            .authentication(Authentication::runtime_key(Secret::new(key).unwrap()).unwrap())
            .build()
            .unwrap();
        let mut request = read();
        if !large_value {
            for i in 0..128 {
                request = request.header(&format!("x-{i}"), "v").unwrap();
            }
        }
        let error = c.send_bytes(request, None).await.unwrap_err();
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Limit, RemoteOutcome::NotSent)
        );
        assert_eq!(transport.count(), 0);
    }
}

#[tokio::test]
async fn forbidden_token_refresh_respects_opa_denial_and_replay_safety() {
    for (code, opa, replay) in [
        (403, None, true),
        (403, Some("true"), true),
        (403, Some("false"), false),
        (401, Some("false"), true),
    ] {
        let mut headers = HeaderMap::new();
        if let Some(value) = opa {
            headers.insert("x-opa-decision", HeaderValue::from_static(value));
        }
        let mut replies = vec![Reply::Http(
            StatusCode::from_u16(code).unwrap(),
            headers,
            Bytes::new(),
        )];
        if replay {
            replies.push(ok());
        }
        let transport = Scripted::new(replies);
        let provider = RotatingToken::new();
        let c = Client::builder(endpoint())
            .transport(transport.clone())
            .authentication(Authentication::oauth(provider.clone(), true))
            .build()
            .unwrap();
        assert_eq!(c.send_bytes(read(), None).await.is_ok(), replay);
        assert_eq!(transport.count(), if replay { 2 } else { 1 });
        assert_eq!(
            provider.invalidations.lock().unwrap().len(),
            usize::from(replay)
        );
    }
    let transport = Scripted::new(vec![status(403)]);
    let provider = RotatingToken::new();
    let c = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .build()
        .unwrap();
    assert!(
        c.send_bytes(RawRequest::new(Method::POST, "write"), None)
            .await
            .is_err()
    );
    assert_eq!(transport.count(), 1);
    assert_eq!(*provider.invalidations.lock().unwrap(), vec![1]);
}

#[tokio::test(start_paused = true)]
async fn per_call_retry_override_keeps_mutations_single_attempt() {
    let transport = Scripted::new(vec![status(503)]);
    let options = RequestOptions::with_timeout(Duration::from_secs(5))
        .unwrap()
        .with_retries(0)
        .unwrap();
    assert!(
        client(transport.clone())
            .send_bytes(read(), Some(&options))
            .await
            .is_err()
    );
    assert_eq!(transport.count(), 1);
    let transport = Scripted::new(vec![status(503), ok()]);
    let c = Client::builder(endpoint())
        .transport(transport.clone())
        .limits(Limits {
            max_retries: 0,
            ..fast_limits()
        })
        .build()
        .unwrap();
    let options = RequestOptions::with_timeout(Duration::from_secs(5))
        .unwrap()
        .with_retries(1)
        .unwrap();
    assert!(c.send_bytes(read(), Some(&options)).await.is_ok());
    assert_eq!(transport.count(), 2);
    let transport = Scripted::new(vec![status(503)]);
    let options = RequestOptions::with_timeout(Duration::from_secs(5))
        .unwrap()
        .with_retries(5)
        .unwrap();
    assert!(
        client(transport.clone())
            .send_bytes(RawRequest::new(Method::POST, "write"), Some(&options))
            .await
            .is_err()
    );
    assert_eq!(transport.count(), 1);
    assert!(
        RequestOptions::with_timeout(Duration::from_secs(5))
            .unwrap()
            .with_retries(6)
            .is_err()
    );
}

#[tokio::test(start_paused = true)]
async fn streaming_permit_is_owned_until_close_or_drop() {
    let transport = Scripted::new(vec![ok(), ok()]);
    let provider = CountingToken::new();
    let c = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .limits(Limits {
            max_concurrency: 1,
            ..fast_limits()
        })
        .build()
        .unwrap();
    let mut stream = c.send_stream(read(), None).await.unwrap();
    let options = RequestOptions::with_timeout(Duration::from_millis(10)).unwrap();
    let error = c.send_bytes(read(), Some(&options)).await.unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Deadline, RemoteOutcome::NotSent)
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(transport.count(), 1);
    stream.close();
    assert!(stream.next().await.unwrap().is_none());
    assert!(c.send_bytes(read(), None).await.is_ok());
}

#[tokio::test(start_paused = true)]
async fn streaming_idle_deadline_drops_body_and_releases_capacity() {
    let started = Arc::new(Notify::new());
    let dropped = Arc::new(AtomicUsize::new(0));
    let transport = Scripted::new(vec![
        Reply::PendingBody {
            started,
            dropped: dropped.clone(),
        },
        ok(),
    ]);
    let c = Client::builder(endpoint())
        .transport(transport)
        .limits(Limits {
            max_concurrency: 1,
            stream_idle_timeout: Duration::from_secs(2),
            ..fast_limits()
        })
        .build()
        .unwrap();
    let mut stream = c.send_stream(read(), None).await.unwrap();
    let start = tokio::time::Instant::now();
    let error = stream.next().await.unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Deadline, RemoteOutcome::Unknown)
    );
    assert_eq!(tokio::time::Instant::now() - start, Duration::from_secs(2));
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    assert!(stream.next().await.unwrap().is_none());
    assert!(c.send_bytes(read(), None).await.is_ok());
}

#[tokio::test(start_paused = true)]
async fn stream_caller_and_session_deadlines_include_inter_read_time() {
    for explicit in [false, true] {
        let transport = Scripted::new(vec![ok()]);
        let c = Client::builder(endpoint())
            .transport(transport)
            .limits(Limits {
                stream_session_timeout: (!explicit).then_some(Duration::from_secs(1)),
                ..fast_limits()
            })
            .build()
            .unwrap();
        let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
        let mut stream = c
            .send_stream(read(), explicit.then_some(&options))
            .await
            .unwrap();
        tokio::time::advance(Duration::from_secs(2)).await;
        assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Deadline);
    }
}

#[tokio::test]
async fn stream_cancellation_and_drop_release_body_and_permit() {
    for which in [0, 1, 2] {
        let dropped = Arc::new(AtomicUsize::new(0));
        let transport = Scripted::new(vec![Reply::PendingBody {
            started: Arc::new(Notify::new()),
            dropped: dropped.clone(),
        }]);
        let c = client(transport);
        let options = RequestOptions::with_timeout(Duration::from_secs(5)).unwrap();
        let mut stream = c.send_stream(read(), Some(&options)).await.unwrap();
        if which == 0 {
            options.cancel();
        } else if which == 1 {
            c.session().cancel();
        }
        if which < 2 {
            assert_eq!(
                stream.next().await.unwrap_err().kind(),
                ErrorKind::Cancelled
            );
        }
        drop(stream);
        assert_eq!(dropped.load(Ordering::SeqCst), 1);
    }
}

#[tokio::test]
async fn stream_total_bytes_and_formatting_are_bounded() {
    let transport = Scripted::new(vec![Reply::Http(
        StatusCode::OK,
        HeaderMap::new(),
        Bytes::from_static(b"secret-response-payload"),
    )]);
    let c = Client::builder(endpoint())
        .transport(transport)
        .limits(Limits {
            max_response_bytes: 4,
            ..fast_limits()
        })
        .build()
        .unwrap();
    let mut stream = c.send_stream(read(), None).await.unwrap();
    assert!(!format!("{stream:?}").contains("secret-response"));
    assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Limit);
    assert!(stream.next().await.unwrap().is_none());
}

#[tokio::test]
async fn stream_handshake_never_replays_including_authentication_refresh() {
    for code in [401, 403, 503] {
        let transport = Scripted::new(vec![status(code)]);
        let provider = RotatingToken::new();
        let c = Client::builder(endpoint())
            .transport(transport.clone())
            .authentication(Authentication::oauth(provider.clone(), true))
            .limits(Limits {
                max_retries: 5,
                ..fast_limits()
            })
            .build()
            .unwrap();
        assert_eq!(
            c.send_stream(read(), None).await.unwrap_err().status(),
            Some(code)
        );
        assert_eq!(transport.count(), 1);
        assert_eq!(
            provider.invalidations.lock().unwrap().len(),
            usize::from(code != 503)
        );
    }
}
fn read() -> RawRequest {
    RawRequest::new(Method::GET, "resource").safe_read()
}

struct CountingToken {
    context: AuthContext,
    calls: AtomicUsize,
}
impl CountingToken {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            context: AuthContext::new(
                Endpoint::parse("https://issuer.example/token").unwrap(),
                "synthetic-client",
                1,
                "123",
            )
            .unwrap(),
            calls: AtomicUsize::new(0),
        })
    }
}
impl TokenProvider for CountingToken {
    fn token<'a>(&'a self, _: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            TokenLease::new(
                Secret::new("synthetic-bearer").unwrap(),
                1,
                Duration::from_secs(3600),
            )
        })
    }
    fn context(&self) -> &AuthContext {
        &self.context
    }
    fn invalidate(&self, _: u64) -> bool {
        false
    }
}

struct RotatingToken {
    context: AuthContext,
    calls: AtomicUsize,
    invalidations: Mutex<Vec<u64>>,
}
impl RotatingToken {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            context: factory_context(),
            calls: AtomicUsize::new(0),
            invalidations: Mutex::new(Vec::new()),
        })
    }
}
impl TokenProvider for RotatingToken {
    fn token<'a>(&'a self, _: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            let generation = self.calls.fetch_add(1, Ordering::SeqCst) as u64 + 1;
            TokenLease::new(
                Secret::new(format!("synthetic-token-{generation}")).unwrap(),
                generation,
                Duration::from_secs(3600),
            )
        })
    }
    fn context(&self) -> &AuthContext {
        &self.context
    }
    fn invalidate(&self, generation: u64) -> bool {
        self.invalidations.lock().unwrap().push(generation);
        true
    }
}

fn factory_context() -> AuthContext {
    AuthContext::new(
        Endpoint::parse("https://issuer.example/oauth-token").unwrap(),
        "synthetic-client",
        1,
        "123",
    )
    .unwrap()
}

fn token_response() -> Reply {
    Reply::Http(StatusCode::OK,HeaderMap::new(),Bytes::from_static(
        br#"{"access_token":"synthetic-service-token","token_type":"Bearer","expires_in":3600}"#))
}

struct RecordingPolicy {
    calls: Mutex<Vec<(DestinationPurpose, String)>>,
    reject_token: bool,
}
impl RecordingPolicy {
    fn new(reject_token: bool) -> Arc<Self> {
        Arc::new(Self {
            calls: Mutex::new(Vec::new()),
            reject_token,
        })
    }
}
impl DestinationPolicy for RecordingPolicy {
    fn check(&self, destination: &url::Url, purpose: DestinationPurpose) -> Result<()> {
        self.calls
            .lock()
            .unwrap()
            .push((purpose, destination.origin().ascii_serialization()));
        if self.reject_token && purpose == DestinationPurpose::OAuthToken {
            Err(Error::new(ErrorKind::Destination))
        } else {
            Ok(())
        }
    }
}

#[tokio::test]
async fn oauth_factory_reuses_injected_transport_policy_and_cached_identity() {
    let transport = Scripted::new(vec![token_response(), ok(), ok()]);
    let policy = RecordingPolicy::new(false);
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .destination_policy(policy.clone())
        .oauth_credentials(
            factory_context(),
            Secret::new("synthetic-client-secret").unwrap(),
            true,
        )
        .build()
        .unwrap();
    client.send_bytes(read(), None).await.unwrap();
    client.clone().send_bytes(read(), None).await.unwrap();
    let captured = transport.requests.lock().unwrap();
    assert_eq!(captured.len(), 3);
    let token = &captured[0];
    assert_eq!(token.operation(), "oauth.token");
    assert_eq!(token.method(), Method::POST);
    assert_eq!(token.url().as_str(), "https://issuer.example/oauth-token");
    assert_eq!(
        token.body(),
        b"grant_type=client_credentials&scope=tsg_id%3A123".as_slice()
    );
    assert!(
        token.headers()[http::header::AUTHORIZATION]
            .to_str()
            .unwrap()
            .starts_with("Basic ")
    );
    assert!(!token.headers().contains_key("x-tsg-id"));
    for api in &captured[1..] {
        assert_eq!(
            api.url().as_str(),
            "https://api.example/private-base/resource"
        );
        assert_eq!(
            api.headers()[http::header::AUTHORIZATION],
            "Bearer synthetic-service-token"
        );
        assert_eq!(api.headers()["x-tsg-id"], "123");
        assert!(!api.headers().contains_key("x-portkey-api-key"));
    }
    assert_eq!(
        *policy.calls.lock().unwrap(),
        vec![
            (DestinationPurpose::Api, "https://api.example".into()),
            (
                DestinationPurpose::OAuthToken,
                "https://issuer.example".into()
            ),
            (DestinationPurpose::Api, "https://api.example".into()),
        ]
    );
}

#[tokio::test]
async fn oauth_factory_honors_issuer_denial_before_token_transport() {
    let transport = Scripted::new(vec![]);
    let policy = RecordingPolicy::new(true);
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .destination_policy(policy.clone())
        .oauth_credentials(
            factory_context(),
            Secret::new("synthetic-client-secret").unwrap(),
            true,
        )
        .build()
        .unwrap();
    let error = client.send_bytes(read(), None).await.unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Destination, RemoteOutcome::NotSent)
    );
    assert_eq!(transport.count(), 0);
    assert_eq!(
        *policy.calls.lock().unwrap(),
        vec![
            (DestinationPurpose::Api, "https://api.example".into()),
            (
                DestinationPurpose::OAuthToken,
                "https://issuer.example".into()
            ),
        ]
    );
}

#[tokio::test]
async fn oauth_factory_default_policy_admits_distinct_api_and_issuer_origins() {
    let transport = Scripted::new(vec![token_response(), ok()]);
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .oauth_credentials(
            factory_context(),
            Secret::new("synthetic-client-secret").unwrap(),
            false,
        )
        .build()
        .unwrap();
    assert_eq!(
        client.send_bytes(read(), None).await.unwrap(),
        b"ok".as_slice()
    );
    let captured = transport.requests.lock().unwrap();
    assert_eq!(captured.len(), 2);
    assert_ne!(captured[0].url().origin(), captured[1].url().origin());
    assert!(!captured[1].headers().contains_key("x-tsg-id"));
}

#[tokio::test]
async fn oauth_factory_session_cancellation_drops_token_exchange_and_blocks_api() {
    let started = Arc::new(Notify::new());
    let dropped = Arc::new(AtomicUsize::new(0));
    let transport = Scripted::new(vec![Reply::PendingBody {
        started: started.clone(),
        dropped: dropped.clone(),
    }]);
    let session = Session::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .session(session.clone())
        .oauth_credentials(
            factory_context(),
            Secret::new("synthetic-client-secret").unwrap(),
            true,
        )
        .build()
        .unwrap();
    let clone = client.clone();
    let request = tokio::spawn(async move { clone.send_bytes(read(), None).await });
    started.notified().await;
    session.cancel();
    let error = request.await.unwrap().unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Cancelled, RemoteOutcome::NotSent)
    );
    assert_eq!(transport.count(), 1);
    assert_eq!(
        transport.requests.lock().unwrap()[0].operation(),
        "oauth.token"
    );
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    let error = client.send_bytes(read(), None).await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Cancelled);
    // Also prove the provider itself carries the factory session, independently
    // of the outer request's cancellation guard.
    let Authentication::OAuth { provider, .. } = &client.inner.auth else {
        panic!("factory did not configure OAuth")
    };
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    assert_eq!(
        provider.token(&options).await.unwrap_err().kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(transport.count(), 1);
}

#[tokio::test]
async fn authentication_and_oauth_factory_setters_use_the_last_explicit_choice() {
    let runtime =
        Authentication::runtime_key(Secret::new("synthetic-runtime-key").unwrap()).unwrap();
    let transport = Scripted::new(vec![ok()]);
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .oauth_credentials(
            factory_context(),
            Secret::new("synthetic-client-secret").unwrap(),
            true,
        )
        .authentication(runtime.clone())
        .build()
        .unwrap();
    client.send_bytes(read(), None).await.unwrap();
    {
        let captured = transport.requests.lock().unwrap();
        assert_eq!(captured.len(), 1);
        assert_eq!(
            captured[0].headers()["x-portkey-api-key"],
            "synthetic-runtime-key"
        );
        assert!(
            !captured[0]
                .headers()
                .contains_key(http::header::AUTHORIZATION)
        );
        assert!(!captured[0].headers().contains_key("x-tsg-id"));
    }
    let transport = Scripted::new(vec![token_response(), ok()]);
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(runtime)
        .oauth_credentials(
            factory_context(),
            Secret::new("synthetic-client-secret").unwrap(),
            true,
        )
        .build()
        .unwrap();
    client.send_bytes(read(), None).await.unwrap();
    let captured = transport.requests.lock().unwrap();
    assert_eq!(captured.len(), 2);
    assert_eq!(captured[0].operation(), "oauth.token");
    assert_eq!(
        captured[1].headers()[http::header::AUTHORIZATION],
        "Bearer synthetic-service-token"
    );
    assert!(
        captured
            .iter()
            .all(|r| !r.headers().contains_key("x-portkey-api-key"))
    );
}

#[tokio::test]
async fn rfc4231_hmac_signs_the_exact_transmitted_body() {
    // RFC 4231 section 4.3, case 2: an independent published HMAC-SHA256 vector.
    let transport = Scripted::new(vec![ok()]);
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::scan(Some(Secret::new("Jefe").unwrap()), None).unwrap())
        .build()
        .unwrap();
    let body = Bytes::from_static(b"what do ya want for nothing?");
    client
        .send_empty(
            RawRequest::new(Method::POST, "scan")
                .body(body.clone(), "text/plain")
                .unwrap(),
            None,
        )
        .await
        .unwrap();
    let captured = transport.requests.lock().unwrap();
    assert_eq!(captured[0].body(), &body);
    assert_eq!(
        captured[0].headers()["x-payload-hash"],
        "5bdcc146bf60754e6a042426089575c75a003f089d2739839dec58b964ec3843"
    );
    assert_eq!(captured[0].headers()["x-pan-token"], "Jefe");
    assert!(
        captured[0]
            .headers()
            .values()
            .all(HeaderValue::is_sensitive)
    );
}

#[tokio::test]
async fn policy_and_invalid_paths_reject_before_authentication_or_transport() {
    struct Reject;
    impl DestinationPolicy for Reject {
        fn check(&self, _: &url::Url, purpose: DestinationPurpose) -> Result<()> {
            assert_eq!(purpose, DestinationPurpose::Api);
            Err(Error::new(ErrorKind::Destination))
        }
    }
    let transport = Scripted::new(vec![]);
    let provider = CountingToken::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .destination_policy(Arc::new(Reject))
        .authentication(Authentication::oauth(provider.clone(), true))
        .build()
        .unwrap();
    let error = client.send_bytes(read(), None).await.unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Destination, RemoteOutcome::NotSent)
    );
    for path in [
        "//attacker.example",
        "https://attacker.example",
        "../secret",
        "%2e%2e/secret",
        "resource?secret=value",
        "resource#secret",
        "\\attacker",
    ] {
        let error = client
            .send_bytes(RawRequest::new(Method::GET, path), None)
            .await
            .unwrap_err();
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Validation, RemoteOutcome::NotSent)
        );
    }
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert_eq!(transport.count(), 0);
}

#[tokio::test]
async fn encoded_separators_and_double_encoded_traversal_reject_before_authentication() {
    let transport = Scripted::new(vec![]);
    let provider = CountingToken::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .build()
        .unwrap();
    // Servers and reverse proxies may decode separators before normalizing
    // paths, or decode a second time. An origin-pinned URL alone does not stop
    // these requests from escaping the configured /private-base path.
    for path in [
        "..%2fsecret",
        "..%2Fsecret",
        "%2e%2e%2fsecret",
        "%2E%2E%2Fsecret",
        "..%5csecret",
        "..%5Csecret",
        "%2e%2E%5Csecret",
        "folder/%2e%2e%2fsecret",
        "folder/..%5csecret",
        "%2f..%2fsecret",
        "%5c%5cattacker.example",
        "%252e%252e%252fsecret",
        "%252E%252E%252Fsecret",
        "folder/%252e%252e%255csecret",
    ] {
        let error = client
            .send_bytes(RawRequest::new(Method::GET, path), None)
            .await
            .unwrap_err();
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Validation, RemoteOutcome::NotSent),
            "encoded path was not rejected before authentication: {path}"
        );
    }
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert_eq!(transport.count(), 0);
}

#[tokio::test]
async fn invalid_json_and_request_limits_reject_before_authentication() {
    let transport = Scripted::new(vec![]);
    let provider = CountingToken::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .limits(Limits {
            max_request_bytes: 32,
            ..fast_limits()
        })
        .build()
        .unwrap();
    for media_type in [
        "application/json",
        "application/problem+json",
        "Application/JSON",
        "application/json ; charset=utf-8",
    ] {
        let request = RawRequest::new(Method::POST, "resource")
            .body(Bytes::from_static(b"{invalid-json"), media_type);
        let error = match request {
            Ok(request) => client.send_bytes(request, None).await.unwrap_err(),
            Err(error) => error,
        };
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Validation, RemoteOutcome::NotSent),
            "{media_type}"
        );
    }
    let error = client
        .send_bytes(
            RawRequest::new(Method::POST, "resource")
                .body(Bytes::from(vec![b'x'; 33]), "text/plain")
                .unwrap(),
            None,
        )
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Limit);
    assert!(
        RawRequest::new(Method::POST, "resource")
            .json(&serde_json::json!({"value":"oversized"}), 4)
            .is_err()
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert_eq!(transport.count(), 0);
}

#[test]
fn framing_credentials_and_header_injection_are_rejected() {
    for header in [
        "Authorization",
        "proxy-authorization",
        "Cookie",
        "Host",
        "Content-Length",
        "transfer-encoding",
        "connection",
        "upgrade",
        "x-pan-token",
        "x-payload-hash",
        "x-portkey-api-key",
        "x-tsg-id",
        "content-type",
        "service-name",
        "User-Agent",
    ] {
        let error = RawRequest::new(Method::GET, "resource")
            .header(header, "synthetic-secret")
            .unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Validation);
        assert!(!format!("{error:?} {error}").contains("synthetic-secret"));
    }
    assert!(
        RawRequest::new(Method::GET, "resource")
            .header("x-safe", "secret\r\nX-Evil: yes")
            .is_err()
    );
    assert!(
        RawRequest::new(Method::GET, "resource")
            .header("bad\r\nname", "value")
            .is_err()
    );
    assert!(
        RawRequest::new(Method::POST, "resource")
            .body(Bytes::new(), "text/plain\r\nX-Evil: yes")
            .is_err()
    );
}

#[tokio::test]
async fn oversized_header_collection_is_rejected_before_authentication() {
    let transport = Scripted::new(vec![]);
    let provider = CountingToken::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .build()
        .unwrap();
    let mut request = RawRequest::new(Method::GET, "resource");
    for n in 0..129 {
        request = request.header(&format!("x-value-{n}"), "value").unwrap();
    }
    assert_eq!(
        client.send_bytes(request, None).await.unwrap_err().kind(),
        ErrorKind::Limit
    );
    let request = RawRequest::new(Method::GET, "resource")
        .header("x-large", &"x".repeat(16 * 1024))
        .unwrap();
    assert_eq!(
        client.send_bytes(request, None).await.unwrap_err().kind(),
        ErrorKind::Limit
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert_eq!(transport.count(), 0);
}

#[tokio::test(start_paused = true)]
async fn only_explicitly_safe_reads_retry_transport_and_server_failures() {
    for reply in [Reply::Failure, status(503)] {
        let transport = Scripted::new(vec![reply, ok()]);
        let result = client(transport.clone())
            .send_bytes(read().query([("value".into(), "a & b%".into())]), None)
            .await
            .unwrap();
        assert_eq!(result, b"ok"[..]);
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests.len(), 2);
        assert_eq!(requests[0].url(), requests[1].url());
        assert_eq!(requests[0].url().query(), Some("value=a+%26+b%25"));
    }
    for (method, reply, kind, outcome) in [
        (
            Method::POST,
            Reply::Failure,
            ErrorKind::Transport,
            RemoteOutcome::Unknown,
        ),
        (
            Method::POST,
            status(503),
            ErrorKind::Http,
            RemoteOutcome::Unknown,
        ),
        (
            Method::GET,
            status(503),
            ErrorKind::Http,
            RemoteOutcome::Unknown,
        ),
        (
            Method::POST,
            status(400),
            ErrorKind::Http,
            RemoteOutcome::Rejected,
        ),
    ] {
        let transport = Scripted::new(vec![reply]);
        let error = client(transport.clone())
            .send_bytes(RawRequest::new(method, "resource"), None)
            .await
            .unwrap_err();
        assert_eq!((error.kind(), error.remote_outcome()), (kind, outcome));
        assert_eq!(transport.count(), 1);
    }
}

#[tokio::test(start_paused = true)]
async fn retry_after_obeys_retry_delay_and_total_budget() {
    let transport = Scripted::new(vec![retry_after("1"), ok()]);
    let start = tokio::time::Instant::now();
    client(transport.clone())
        .send_bytes(read(), None)
        .await
        .unwrap();
    assert_eq!(tokio::time::Instant::now() - start, Duration::from_secs(1));
    assert_eq!(transport.count(), 2);

    let transport = Scripted::new(vec![retry_after("20")]);
    let start = tokio::time::Instant::now();
    let error = client(transport.clone())
        .send_bytes(read(), None)
        .await
        .unwrap_err();
    assert_eq!(error.retry_after(), Some(Duration::from_secs(20)));
    assert_eq!(error.status(), Some(429));
    assert_eq!(error.remote_outcome(), RemoteOutcome::Rejected);
    assert_eq!(tokio::time::Instant::now() - start, Duration::ZERO);
    assert_eq!(transport.count(), 1);

    let transport = Scripted::new(vec![retry_after("2")]);
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    let error = client(transport.clone())
        .send_bytes(read(), Some(&options))
        .await
        .unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Deadline, RemoteOutcome::Unknown)
    );
    assert_eq!(transport.count(), 1);
}

#[tokio::test(start_paused = true)]
async fn retry_cap_and_nonretryable_statuses_are_respected() {
    let transport = Scripted::new(vec![status(502), status(502), status(502)]);
    assert_eq!(
        client(transport.clone())
            .send_bytes(read(), None)
            .await
            .unwrap_err()
            .status(),
        Some(502)
    );
    assert_eq!(transport.count(), 3);
    for code in [400, 401, 403, 404, 500] {
        let transport = Scripted::new(vec![status(code)]);
        assert_eq!(
            client(transport.clone())
                .send_bytes(read(), None)
                .await
                .unwrap_err()
                .status(),
            Some(code)
        );
        assert_eq!(transport.count(), 1);
    }
}

#[tokio::test]
async fn zero_retry_budget_invalidates_unauthorized_token_without_replay() {
    let transport = Scripted::new(vec![status(401)]);
    let provider = RotatingToken::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .limits(Limits {
            max_retries: 0,
            ..fast_limits()
        })
        .build()
        .unwrap();
    let error = client.send_bytes(read(), None).await.unwrap_err();
    assert_eq!(
        (error.status(), error.remote_outcome()),
        (Some(401), RemoteOutcome::Rejected)
    );
    assert_eq!(transport.count(), 1);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(*provider.invalidations.lock().unwrap(), vec![1]);
}

#[tokio::test]
async fn one_retry_budget_permits_exactly_one_fresh_token_replay() {
    let transport = Scripted::new(vec![status(401), ok()]);
    let provider = RotatingToken::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .limits(Limits {
            max_retries: 1,
            ..fast_limits()
        })
        .build()
        .unwrap();
    assert_eq!(
        client.send_bytes(read(), None).await.unwrap(),
        b"ok".as_slice()
    );
    let captured = transport.requests.lock().unwrap();
    assert_eq!(captured.len(), 2);
    assert_eq!(
        captured[0].headers()[http::header::AUTHORIZATION],
        "Bearer synthetic-token-1"
    );
    assert_eq!(
        captured[1].headers()[http::header::AUTHORIZATION],
        "Bearer synthetic-token-2"
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 2);
    assert_eq!(*provider.invalidations.lock().unwrap(), vec![1]);
}

#[tokio::test(start_paused = true)]
async fn unauthorized_refresh_and_server_retries_share_one_total_budget() {
    for (codes, budget, final_status, invalidated) in [
        (vec![401, 503], 1, 503, vec![1]),
        (vec![503, 401], 1, 401, vec![2]),
        (vec![401, 503, 503], 2, 503, vec![1]),
        (vec![401, 401], 1, 401, vec![1, 2]),
    ] {
        let transport = Scripted::new(codes.into_iter().map(status).collect());
        let provider = RotatingToken::new();
        let client = Client::builder(endpoint())
            .transport(transport.clone())
            .authentication(Authentication::oauth(provider.clone(), true))
            .limits(Limits {
                max_retries: budget,
                ..fast_limits()
            })
            .build()
            .unwrap();
        let error = client.send_bytes(read(), None).await.unwrap_err();
        assert_eq!(error.status(), Some(final_status));
        assert_eq!(transport.count(), budget as usize + 1);
        assert_eq!(*provider.invalidations.lock().unwrap(), invalidated);
    }
}

#[tokio::test]
async fn unauthorized_mutation_invalidates_token_without_replaying() {
    let transport = Scripted::new(vec![status(401)]);
    let provider = RotatingToken::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .limits(Limits {
            max_retries: 5,
            ..fast_limits()
        })
        .build()
        .unwrap();
    let error = client
        .send_bytes(RawRequest::new(Method::POST, "write"), None)
        .await
        .unwrap_err();
    assert_eq!(
        (error.status(), error.remote_outcome()),
        (Some(401), RemoteOutcome::Rejected)
    );
    assert_eq!(transport.count(), 1);
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(*provider.invalidations.lock().unwrap(), vec![1]);
}

#[tokio::test]
async fn transport_policy_and_limit_errors_never_trigger_retries() {
    for kind in [
        ErrorKind::Limit,
        ErrorKind::Protocol,
        ErrorKind::Destination,
    ] {
        let transport = Scripted::new(vec![Reply::LocalFailure(kind)]);
        let error = client(transport.clone())
            .send_bytes(read(), None)
            .await
            .unwrap_err();
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (kind, RemoteOutcome::Unknown)
        );
        assert_eq!(transport.count(), 1);
    }
}

#[tokio::test(start_paused = true)]
async fn queue_deadline_and_cancellation_do_not_consume_auth_or_leak_permits() {
    let started = Arc::new(Notify::new());
    let dropped = Arc::new(AtomicUsize::new(0));
    let transport = Scripted::new(vec![
        Reply::PendingBody {
            started: started.clone(),
            dropped: dropped.clone(),
        },
        ok(),
    ]);
    let provider = CountingToken::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .limits(Limits {
            max_concurrency: 1,
            ..fast_limits()
        })
        .build()
        .unwrap();
    let first_client = client.clone();
    let first = tokio::spawn(async move { first_client.send_bytes(read(), None).await });
    started.notified().await;
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    let error = client.send_bytes(read(), Some(&options)).await.unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Deadline, RemoteOutcome::NotSent)
    );
    let queued_options = RequestOptions::with_timeout(Duration::from_secs(10)).unwrap();
    let cancel = queued_options.clone();
    let queued_client = client.clone();
    let queued = tokio::spawn(async move {
        queued_client
            .send_bytes(read(), Some(&queued_options))
            .await
    });
    tokio::task::yield_now().await;
    cancel.cancel();
    let error = queued.await.unwrap().unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Cancelled, RemoteOutcome::NotSent)
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
    assert_eq!(transport.count(), 1);
    first.abort();
    assert!(first.await.is_err());
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    client.send_bytes(read(), None).await.unwrap();
    assert_eq!(provider.calls.load(Ordering::SeqCst), 2);
    assert_eq!(transport.count(), 2);
}

#[tokio::test]
async fn per_request_cancellation_drops_response_stream_and_releases_permit() {
    let started = Arc::new(Notify::new());
    let dropped = Arc::new(AtomicUsize::new(0));
    let transport = Scripted::new(vec![
        Reply::PendingBody {
            started: started.clone(),
            dropped: dropped.clone(),
        },
        ok(),
    ]);
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .limits(Limits {
            max_concurrency: 1,
            ..fast_limits()
        })
        .build()
        .unwrap();
    let options = RequestOptions::with_timeout(Duration::from_secs(60)).unwrap();
    let cancel = options.clone();
    let first_client = client.clone();
    let first = tokio::spawn(async move {
        first_client
            .send_bytes(RawRequest::new(Method::POST, "write"), Some(&options))
            .await
    });
    started.notified().await;
    cancel.cancel();
    let error = first.await.unwrap().unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Cancelled, RemoteOutcome::Unknown)
    );
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
    tokio::time::timeout(Duration::from_secs(1), client.send_bytes(read(), None))
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn session_cancellation_reaches_clones_and_blocks_future_requests() {
    let started = Arc::new(Notify::new());
    let dropped = Arc::new(AtomicUsize::new(0));
    let transport = Scripted::new(vec![Reply::PendingBody {
        started: started.clone(),
        dropped: dropped.clone(),
    }]);
    let client = client(transport.clone());
    let cloned = client.clone();
    let task = tokio::spawn(async move { cloned.send_bytes(read(), None).await });
    started.notified().await;
    client.session().cancel();
    let error = task.await.unwrap().unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Cancelled, RemoteOutcome::Unknown)
    );
    let error = client.send_bytes(read(), None).await.unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Cancelled, RemoteOutcome::NotSent)
    );
    assert_eq!(transport.count(), 1);
    assert_eq!(dropped.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn already_cancelled_request_never_calls_authentication() {
    let transport = Scripted::new(vec![]);
    let provider = CountingToken::new();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(Authentication::oauth(provider.clone(), true))
        .build()
        .unwrap();
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    options.cancel();
    assert_eq!(
        client
            .send_bytes(read(), Some(&options))
            .await
            .unwrap_err()
            .remote_outcome(),
        RemoteOutcome::NotSent
    );
    assert_eq!(provider.calls.load(Ordering::SeqCst), 0);
    assert_eq!(transport.count(), 0);
}

#[tokio::test]
async fn post_send_body_and_decode_failures_report_uncertain_outcomes() {
    let transport = Scripted::new(vec![Reply::BrokenBody]);
    let error = client(transport)
        .send_bytes(read(), None)
        .await
        .unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Transport, RemoteOutcome::Unknown)
    );
    let transport = Scripted::new(vec![Reply::Http(
        StatusCode::OK,
        HeaderMap::new(),
        Bytes::from_static(b"synthetic-secret-invalid-json"),
    )]);
    let error = client(transport)
        .send_json::<serde_json::Value>(read(), None)
        .await
        .unwrap_err();
    assert_eq!(
        (error.kind(), error.remote_outcome()),
        (ErrorKind::Decode, RemoteOutcome::Unknown)
    );
    assert!(!format!("{error:?} {error}").contains("synthetic-secret"));
    let transport = Scripted::new(vec![Reply::Http(
        StatusCode::NO_CONTENT,
        HeaderMap::new(),
        Bytes::new(),
    )]);
    assert_eq!(
        client(transport)
            .send_optional_json::<serde_json::Value>(read(), None)
            .await
            .unwrap(),
        None
    );
}

#[tokio::test]
async fn success_and_error_bodies_are_bounded_with_uncertain_local_failures() {
    for (code, body_limit) in [(200, 16), (403, 8)] {
        let transport = Scripted::new(vec![Reply::Http(
            StatusCode::from_u16(code).unwrap(),
            HeaderMap::new(),
            Bytes::from_static(b"synthetic-secret-overlong-response"),
        )]);
        let client = Client::builder(endpoint())
            .transport(transport.clone())
            .limits(Limits {
                max_response_bytes: body_limit,
                max_error_bytes: body_limit,
                ..fast_limits()
            })
            .build()
            .unwrap();
        let error = client.send_bytes(read(), None).await.unwrap_err();
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Limit, RemoteOutcome::Unknown)
        );
        if code == 403 {
            assert_eq!(error.status(), Some(403));
        }
        assert!(!format!("{error:?} {error}").contains("synthetic-secret"));
        assert_eq!(transport.count(), 1);
    }
}

#[tokio::test]
async fn echoed_error_values_never_enter_debug_or_display() {
    let mut headers = HeaderMap::new();
    headers.insert("x-request-id", HeaderValue::from_static("synthetic-secret"));
    let transport = Scripted::new(vec![Reply::Http(
        StatusCode::BAD_REQUEST,
        headers,
        Bytes::from_static(br#"{"code":"synthetic-secret","message":"Bearer synthetic-secret"}"#),
    )]);
    let auth = Authentication::runtime_key(Secret::new("synthetic-secret").unwrap()).unwrap();
    let client = Client::builder(endpoint())
        .transport(transport.clone())
        .authentication(auth.clone())
        .build()
        .unwrap();
    let raw = RawRequest::new(Method::POST, "synthetic-secret")
        .query([("secret".into(), "synthetic-secret".into())])
        .header("x-test", "synthetic-secret")
        .unwrap()
        .body(Bytes::from_static(b"synthetic-secret"), "text/plain")
        .unwrap();
    let raw_debug = format!("{raw:?}");
    let error = client.send_bytes(raw, None).await.unwrap_err();
    assert_eq!(error.request_id(), Some("synthetic-secret"));
    assert_eq!(error.service_code(), Some("synthetic-secret"));
    let captured = transport.requests.lock().unwrap();
    let output = format!(
        "{error:?} {error} {auth:?} {client:?} {raw_debug} {:?}",
        captured[0]
    );
    assert!(!output.contains("synthetic-secret"));
    assert!(!output.contains("private-base"));
}

#[test]
fn invalid_limits_fail_and_injected_client_constructs_without_runtime() {
    let invalid = [
        Limits {
            max_concurrency: 0,
            ..fast_limits()
        },
        Limits {
            max_concurrency: 4097,
            ..fast_limits()
        },
        Limits {
            max_retries: 6,
            ..fast_limits()
        },
        Limits {
            max_response_bytes: 0,
            ..fast_limits()
        },
        Limits {
            operation_timeout: Duration::ZERO,
            ..fast_limits()
        },
        Limits {
            max_error_bytes: 1024 * 1024 + 1,
            ..fast_limits()
        },
        Limits {
            retry_base_delay: Duration::from_secs(6),
            max_retry_delay: Duration::from_secs(5),
            ..fast_limits()
        },
    ];
    for limits in invalid {
        assert_eq!(
            Client::builder(endpoint())
                .transport(Scripted::new(vec![]))
                .limits(limits)
                .build()
                .unwrap_err()
                .kind(),
            ErrorKind::Configuration
        );
    }
    assert!(
        Client::builder(endpoint())
            .transport(Scripted::new(vec![]))
            .build()
            .is_ok()
    );
    assert!(RequestOptions::with_timeout(Duration::ZERO).is_err());
    assert!(RequestOptions::with_timeout(Duration::from_secs(86401)).is_err());
    assert!(RequestOptions::with_timeout(Duration::from_secs(1)).is_ok());
}

#[cfg(feature = "reqwest-transport")]
mod default_adapter {
    use super::*;
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
        sync::oneshot,
    };

    struct WireRequest {
        headers: String,
        body: Vec<u8>,
    }
    async fn receive(stream: &mut TcpStream) -> WireRequest {
        let mut bytes = Vec::new();
        let header_end;
        loop {
            let mut chunk = [0u8; 2048];
            let count = stream.read(&mut chunk).await.unwrap();
            assert!(count > 0, "request ended before headers");
            bytes.extend_from_slice(&chunk[..count]);
            assert!(bytes.len() < 64 * 1024, "unbounded test request");
            if let Some(end) = bytes.windows(4).position(|v| v == b"\r\n\r\n") {
                header_end = end + 4;
                break;
            }
        }
        let headers = String::from_utf8(bytes[..header_end].to_vec()).unwrap();
        let length = headers
            .lines()
            .find_map(|line| {
                let (key, value) = line.split_once(':')?;
                key.eq_ignore_ascii_case("content-length")
                    .then(|| value.trim().parse::<usize>().unwrap())
            })
            .unwrap_or(0);
        assert!(length < 64 * 1024);
        while bytes.len() - header_end < length {
            let mut chunk = [0u8; 2048];
            let count = stream.read(&mut chunk).await.unwrap();
            assert!(count > 0);
            bytes.extend_from_slice(&chunk[..count]);
        }
        WireRequest {
            headers,
            body: bytes[header_end..header_end + length].to_vec(),
        }
    }
    async fn fixture(
        response: Vec<u8>,
    ) -> (
        Endpoint,
        oneshot::Receiver<WireRequest>,
        tokio::task::JoinHandle<()>,
    ) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint =
            Endpoint::loopback(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
        let (tx, rx) = oneshot::channel();
        let task = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let captured = receive(&mut stream).await;
            stream.write_all(&response).await.unwrap();
            stream.shutdown().await.unwrap();
            let _ = tx.send(captured);
        });
        (endpoint, rx, task)
    }

    #[tokio::test]
    async fn exact_json_hmac_and_bytes_survive_the_default_http_adapter() {
        let (endpoint, wire, server) = fixture(
            b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok".to_vec(),
        )
        .await;
        let client = Client::builder(endpoint)
            .authentication(
                Authentication::scan(Some(Secret::new("synthetic-hmac-key").unwrap()), None)
                    .unwrap(),
            )
            .build()
            .unwrap();
        let body = Bytes::from_static(b"{ \"message\": \"wire\", \"n\": 2 }\n");
        tokio::time::timeout(
            Duration::from_secs(2),
            client.send_bytes(
                RawRequest::new(Method::POST, "scan")
                    .body(body.clone(), "application/json")
                    .unwrap(),
                None,
            ),
        )
        .await
        .unwrap()
        .unwrap();
        let captured = wire.await.unwrap();
        server.await.unwrap();
        assert_eq!(captured.body, body);
        // Independently computed with Python stdlib hmac.new(key, literal_bytes, sha256).
        assert!(captured.headers.to_ascii_lowercase().contains(
            "x-payload-hash: 6caffe1ce32f42862177f9118424efb03d4b5aa675659cf8d187680033641cc2\r\n"
        ));
        assert!(
            captured
                .headers
                .contains("x-pan-token: synthetic-hmac-key\r\n")
        );
    }

    #[tokio::test]
    async fn redirect_never_transmits_credentials_to_the_location_target() {
        let target = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let location = format!("http://{}/credential-sink", target.local_addr().unwrap());
        let response = format!(
            concat!(
                "HTTP/1.1 307 Temporary Redirect\r\nLocation: {}\r\n",
                "Content-Length: 0\r\nConnection: close\r\n\r\n",
            ),
            location
        )
        .into_bytes();
        let (endpoint, wire, server) = fixture(response).await;
        let client = Client::builder(endpoint)
            .authentication(
                Authentication::runtime_key(Secret::new("synthetic-runtime-secret").unwrap())
                    .unwrap(),
            )
            .build()
            .unwrap();
        let error = tokio::time::timeout(
            Duration::from_secs(2),
            client.send_bytes(RawRequest::new(Method::POST, "original"), None),
        )
        .await
        .unwrap()
        .unwrap_err();
        assert_eq!(
            (error.kind(), error.status(), error.remote_outcome()),
            (ErrorKind::Destination, Some(307), RemoteOutcome::Rejected)
        );
        assert!(
            wire.await
                .unwrap()
                .headers
                .contains("x-portkey-api-key: synthetic-runtime-secret")
        );
        server.await.unwrap();
        assert!(
            tokio::time::timeout(Duration::from_millis(80), target.accept())
                .await
                .is_err()
        );
        assert!(!format!("{error:?} {error}").contains("synthetic-runtime-secret"));
    }

    #[tokio::test]
    async fn gzip_limit_applies_to_decompressed_bytes() {
        let plain = vec![b'x'; 8192];
        let mut gzip = flate2::write::GzEncoder::new(Vec::new(), flate2::Compression::fast());
        gzip.write_all(&plain).unwrap();
        let compressed = gzip.finish().unwrap();
        assert!(compressed.len() < 128);
        let mut response = format!(
            concat!(
                "HTTP/1.1 200 OK\r\nContent-Encoding: gzip\r\n",
                "Content-Length: {}\r\nConnection: close\r\n\r\n",
            ),
            compressed.len()
        )
        .into_bytes();
        response.extend_from_slice(&compressed);
        let (endpoint, wire, server) = fixture(response).await;
        let client = Client::builder(endpoint)
            .limits(Limits {
                max_response_bytes: 128,
                ..fast_limits()
            })
            .build()
            .unwrap();
        let error = tokio::time::timeout(Duration::from_secs(2), client.send_bytes(read(), None))
            .await
            .unwrap()
            .unwrap_err();
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Limit, RemoteOutcome::Unknown)
        );
        wire.await.unwrap();
        server.await.unwrap();
    }

    #[tokio::test]
    async fn failed_post_is_not_replayed_by_the_default_http_adapter() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint =
            Endpoint::loopback(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let request = receive(&mut stream).await;
            assert!(request.headers.starts_with("POST /write HTTP/1.1\r\n"));
            assert_eq!(request.body, b"{\"write\":true}");
            drop(stream); // Request reached the server; its response is now uncertain.
            assert!(
                tokio::time::timeout(Duration::from_millis(80), listener.accept())
                    .await
                    .is_err()
            );
        });
        let client = Client::builder(endpoint).build().unwrap();
        let error = tokio::time::timeout(
            Duration::from_secs(2),
            client.send_bytes(
                RawRequest::new(Method::POST, "write")
                    .body(Bytes::from_static(b"{\"write\":true}"), "application/json")
                    .unwrap(),
                None,
            ),
        )
        .await
        .unwrap()
        .unwrap_err();
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Transport, RemoteOutcome::Unknown)
        );
        server.await.unwrap();
    }

    #[tokio::test]
    async fn cancellation_closes_an_incomplete_default_http_response() {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let endpoint =
            Endpoint::loopback(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
        let (ready_tx, ready_rx) = oneshot::channel();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            receive(&mut stream).await;
            stream
                .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 1000\r\n\r\nx")
                .await
                .unwrap();
            ready_tx.send(()).unwrap();
            let mut byte = [0u8; 1];
            let closed = tokio::time::timeout(Duration::from_secs(1), stream.read(&mut byte))
                .await
                .unwrap();
            assert!(
                matches!(closed, Ok(0))
                    || matches!(closed, Err(ref error) if error.kind() == std::io::ErrorKind::ConnectionReset),
                "dropped response must close its HTTP connection: {closed:?}"
            );
        });
        let client = Client::builder(endpoint)
            .limits(Limits {
                max_concurrency: 1,
                ..fast_limits()
            })
            .build()
            .unwrap();
        let options = RequestOptions::with_timeout(Duration::from_secs(10)).unwrap();
        let cancel = options.clone();
        let request = tokio::spawn(async move { client.send_bytes(read(), Some(&options)).await });
        ready_rx.await.unwrap();
        cancel.cancel();
        let error = request.await.unwrap().unwrap_err();
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Cancelled, RemoteOutcome::Unknown)
        );
        server.await.unwrap();
    }

    #[test]
    fn default_adapter_constructs_without_a_tokio_runtime() {
        assert!(Client::builder(endpoint()).build().is_ok());
    }
}

#[test]
fn json_serialization_bounds_depth_before_validation() {
    let mut value = serde_json::json!({"secret":"fixture-secret"});
    for _ in 0..140 {
        value = serde_json::Value::Array(vec![value]);
    }
    let error = serialize_bounded(&value, 1024 * 1024).unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Limit);
    assert!(!format!("{error:?}").contains("fixture-secret"));
    let bytes =
        serialize_bounded(&serde_json::json!({"a":[1,"snowman ☃"],"b":true}), 1024).unwrap();
    assert_eq!(
        serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
        serde_json::json!({"a":[1,"snowman ☃"],"b":true})
    );
}

#[test]
fn client_validation_limits_are_explicit_and_validated() {
    let transport = Scripted::new(vec![]);
    let invalid = crate::validation::ValidationLimits {
        max_nodes: 0,
        ..Default::default()
    };
    let result = Client::builder(Endpoint::parse("https://example.invalid").unwrap())
        .transport(transport.clone())
        .validation_limits(invalid)
        .build();
    assert_eq!(result.unwrap_err().kind(), ErrorKind::Configuration);
    assert_eq!(transport.count(), 0);
    let custom = crate::validation::ValidationLimits {
        max_nodes: 200_000,
        ..Default::default()
    };
    let client = Client::builder(Endpoint::parse("https://example.invalid").unwrap())
        .transport(transport)
        .validation_limits(custom)
        .build()
        .unwrap();
    assert_eq!(client.validation_limits().unwrap().max_nodes, 200_000);
}

#[tokio::test]
async fn text_responses_match_fetch_decoding_and_preserve_body_bounds() {
    let transport = Scripted::new(vec![
        Reply::Http(
            StatusCode::OK,
            HeaderMap::new(),
            Bytes::from_static(b"\xef\xbb\xbfheader\n\xfftail\n"),
        ),
        Reply::Http(StatusCode::NO_CONTENT, HeaderMap::new(), Bytes::new()),
    ]);
    let client = client(transport);
    assert_eq!(
        client.send_text(read(), None).await.unwrap(),
        "header\n\u{fffd}tail\n"
    );
    assert_eq!(client.send_text(read(), None).await.unwrap(), "");
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-request-id",
        HeaderValue::from_static("safe-text-request"),
    );
    let transport = Scripted::new(vec![Reply::Http(
        StatusCode::OK,
        headers,
        Bytes::from_static(b"too-long"),
    )]);
    let client = Client::builder(endpoint())
        .transport(transport)
        .limits(Limits {
            max_response_bytes: 4,
            ..fast_limits()
        })
        .build()
        .unwrap();
    let error = client.send_text(read(), None).await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Limit);
    assert_eq!(error.status(), Some(200));
    assert_eq!(error.request_id(), Some("safe-text-request"));
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
}

#[tokio::test(start_paused = true)]
async fn text_body_deadlines_keep_metadata_and_cancel_before_dispatch() {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-request-id",
        HeaderValue::from_static("safe-text-deadline"),
    );
    let transport = Scripted::new(vec![Reply::PendingMetadata(StatusCode::OK, headers)]);
    let client = client(transport.clone());
    let options = RequestOptions::with_timeout(Duration::from_millis(1)).unwrap();
    let error = client.send_text(read(), Some(&options)).await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Deadline);
    assert_eq!(error.status(), Some(200));
    assert_eq!(error.request_id(), Some("safe-text-deadline"));
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
    let options = RequestOptions::with_timeout(Duration::from_secs(1)).unwrap();
    options.cancel();
    assert_eq!(
        client
            .send_text(read(), Some(&options))
            .await
            .unwrap_err()
            .kind(),
        ErrorKind::Cancelled
    );
    assert_eq!(transport.count(), 1);
}

#[cfg(feature = "gateway")]
fn runtime_client(transport: Arc<Scripted>) -> Client {
    Client::builder(endpoint())
        .authentication(Authentication::runtime_key(Secret::new("runtime-key").unwrap()).unwrap())
        .transport(transport)
        .limits(fast_limits())
        .build()
        .unwrap()
}

#[cfg(feature = "gateway")]
#[tokio::test(start_paused = true)]
async fn runtime_replay_requires_explicit_permit_and_uses_exact_status_set() {
    for streaming in [false, true] {
        for code in [429, 500, 502, 503, 504] {
            for explicit in [false, true] {
                let eligible = explicit && code != 429;
                let transport = Scripted::new(if eligible {
                    vec![status(code), ok()]
                } else {
                    vec![status(code)]
                });
                let client = runtime_client(transport.clone());
                let mut request = RawRequest::new(Method::POST, "inference")
                    .body(
                        Bytes::from_static(b"{\"prompt\":\"private\"}"),
                        "application/json",
                    )
                    .unwrap();
                let replay = if explicit {
                    crate::RuntimeReplay::bounded(1).unwrap()
                } else {
                    crate::RuntimeReplay::Never
                };
                replay.apply(&mut request).unwrap();
                let options = RequestOptions::with_timeout(Duration::from_secs(2))
                    .unwrap()
                    .with_retries(5)
                    .unwrap();
                let successful = if streaming {
                    client.send_stream(request, Some(&options)).await.is_ok()
                } else {
                    client.send_bytes(request, Some(&options)).await.is_ok()
                };
                assert_eq!(
                    successful, eligible,
                    "stream={streaming}, status={code}, explicit={explicit}"
                );
                assert_eq!(transport.count(), if eligible { 2 } else { 1 });
                let requests = transport.requests.lock().unwrap();
                assert!(requests.iter().all(|r| r.body() == requests[0].body()));
            }
        }
    }
    let transport = Scripted::new(vec![]);
    let client = client(transport.clone());
    let mut request = RawRequest::new(Method::POST, "mutation");
    crate::RuntimeReplay::bounded(1)
        .unwrap()
        .apply(&mut request)
        .unwrap();
    assert_eq!(
        client.send_bytes(request, None).await.unwrap_err().kind(),
        ErrorKind::Configuration
    );
    assert_eq!(transport.count(), 0);
}

#[cfg(feature = "gateway")]
#[tokio::test]
async fn runtime_replay_stops_at_success_headers_and_body_failures_keep_metadata() {
    for streaming in [false, true] {
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-request-id",
            HeaderValue::from_static("safe-runtime-body"),
        );
        let transport = Scripted::new(vec![Reply::Http(
            StatusCode::OK,
            headers,
            Bytes::from_static(b"oversized"),
        )]);
        let client = Client::builder(endpoint())
            .authentication(
                Authentication::runtime_key(Secret::new("runtime-key").unwrap()).unwrap(),
            )
            .transport(transport.clone())
            .limits(Limits {
                max_response_bytes: 1,
                ..fast_limits()
            })
            .build()
            .unwrap();
        let mut request = RawRequest::new(Method::POST, "inference");
        crate::RuntimeReplay::bounded(5)
            .unwrap()
            .apply(&mut request)
            .unwrap();
        let error = if streaming {
            client
                .send_stream(request, None)
                .await
                .unwrap()
                .next()
                .await
                .unwrap_err()
        } else {
            client.send_bytes(request, None).await.unwrap_err()
        };
        assert_eq!(error.kind(), ErrorKind::Limit);
        assert_eq!(error.status(), Some(200));
        assert_eq!(error.request_id(), Some("safe-runtime-body"));
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        assert_eq!(transport.count(), 1);
    }
}

#[tokio::test(start_paused = true)]
async fn streaming_error_body_deadlines_retain_known_status_and_correlation() {
    let mut headers = HeaderMap::new();
    headers.insert(
        "x-request-id",
        HeaderValue::from_static("safe-stream-handshake"),
    );
    let transport = Scripted::new(vec![Reply::PendingMetadata(
        StatusCode::SERVICE_UNAVAILABLE,
        headers,
    )]);
    let client = client(transport.clone());
    let options = RequestOptions::with_timeout(Duration::from_millis(1)).unwrap();
    let error = client
        .send_stream(read(), Some(&options))
        .await
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Deadline);
    assert_eq!(error.status(), Some(503));
    assert_eq!(error.request_id(), Some("safe-stream-handshake"));
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
    assert_eq!(transport.count(), 1);
}

#[test]
fn segment_checks_borrowed_length_before_owning_identifier() {
    let identifier = "x".repeat(1024 * 1024);
    assert_eq!(
        RawRequest::new(Method::GET, "files")
            .segment(&identifier)
            .unwrap_err()
            .kind(),
        ErrorKind::Validation
    );
}

#[cfg(feature = "gateway")]
#[tokio::test]
async fn typed_stream_and_sse_parser_errors_keep_allowlisted_correlation_without_payloads() {
    for payload in [
        b"data: private-invalid-json\n\n".as_slice(),
        b"data: {}\n\n".as_slice(),
    ] {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/event-stream"),
        );
        headers.insert("x-request-id", HeaderValue::from_static("safe-event-id"));
        let transport = Scripted::new(vec![Reply::Http(
            StatusCode::OK,
            headers,
            Bytes::copy_from_slice(payload),
        )]);
        let client = runtime_client(transport.clone());
        let mut response = client
            .send_contract_stream::<serde_json::Value>(
                RawRequest::new(Method::POST, "inference"),
                "GatewayInferenceCreateCompletionResponseSchema",
                crate::streaming::TerminalPolicy::DoneMarker,
                None,
            )
            .await
            .unwrap();
        let error = response.next().await.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Decode);
        assert_eq!(error.status(), Some(200));
        assert_eq!(error.request_id(), Some("safe-event-id"));
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        assert!(!format!("{error:?} {error} {response:?}").contains("private-invalid-json"));
        assert_eq!(
            response.next().await.unwrap_err().request_id(),
            Some("safe-event-id")
        );
        assert_eq!(transport.count(), 1);
    }
}

#[cfg(feature = "gateway")]
#[tokio::test]
async fn contract_sse_events_use_the_operation_response_default_policy() {
    for omit in [false, true] {
        let mut headers = HeaderMap::new();
        headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_static("text/event-stream"),
        );
        let transport = Scripted::new(vec![Reply::Http(
            StatusCode::OK,
            headers,
            Bytes::from_static(
                b"data: {\"basic_auth_header\":null,\"future\":null}\n\ndata: [DONE]\n\n",
            ),
        )]);
        let client = client(transport);
        let mut options = RequestOptions::with_timeout(Duration::from_secs(10)).unwrap();
        if omit {
            options = options.without_response_defaults();
        }
        // Gateway's current event schemas have no defaults. Use the same stream
        // adapter with a frozen default-bearing contract to prove propagation.
        let mut events = client
            .send_contract_stream::<serde_json::Value>(
                RawRequest::read("/events").named("gateway.events.response_defaults"),
                "BasicAuthAuthConfigSchema",
                crate::streaming::TerminalPolicy::DoneMarker,
                Some(&options),
            )
            .await
            .unwrap();
        let mut expected = serde_json::json!({"basic_auth_header":null,"future":null});
        if !omit {
            expected["basic_auth_location"] = serde_json::json!("HEADER");
        }
        assert_eq!(events.next().await.unwrap().unwrap(), expected);
        assert!(events.next().await.unwrap().is_none());
        assert_eq!(
            client.inner.permits.available_permits(),
            client.limits().max_concurrency
        );
    }
}

#[cfg(feature = "gateway")]
#[tokio::test]
async fn buffered_alternative_contracts_use_the_operation_response_default_policy() {
    for omit in [false, true] {
        let input = serde_json::json!({"basic_auth_header":null,"future":null});
        let transport = Scripted::new(vec![Reply::Http(
            StatusCode::OK,
            HeaderMap::new(),
            Bytes::from(serde_json::to_vec(&input).unwrap()),
        )]);
        let client = client(transport);
        let mut options = RequestOptions::with_timeout(Duration::from_secs(10)).unwrap();
        if omit {
            options = options.without_response_defaults();
        }
        let actual: serde_json::Value = client
            .send_contract_one_of(
                RawRequest::read("/target").named("red_team.targets.get"),
                &["HeadersAuthConfigSchema", "BasicAuthAuthConfigSchema"],
                Some(&options),
            )
            .await
            .unwrap();
        let mut expected = input;
        if !omit {
            expected["basic_auth_location"] = serde_json::json!("HEADER");
        }
        assert_eq!(actual, expected);
    }
}
