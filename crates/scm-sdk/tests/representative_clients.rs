//! Independent representative wire contracts from the frozen SDK. Responses
//! below are literal fixtures, not output from the native model or validator.
#![cfg(any(feature = "management", feature = "gateway"))]

use bytes::Bytes;
use futures_util::{Stream, future::BoxFuture};
use http::{HeaderMap, HeaderValue, Method, StatusCode};
#[cfg(feature = "management")]
use scm_sdk::auth::{AuthContext, TokenLease, TokenProvider};
use scm_sdk::{
    Client, Endpoint, ErrorKind, RemoteOutcome, RequestOptions, Result, Secret,
    auth::Authentication,
    transport::{Transport, TransportRequest, TransportResponse},
};
#[cfg(feature = "gateway")]
use scm_sdk::{Error, Limits};
use serde_json::{Value, json};
use std::{
    collections::VecDeque,
    pin::Pin,
    sync::{
        Arc, Mutex,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll},
    time::Duration,
};

enum Reply {
    Json(Value),
    #[cfg(feature = "gateway")]
    Binary(Vec<Bytes>),
    #[cfg(feature = "gateway")]
    Http(u16),
    #[cfg(feature = "gateway")]
    Failure,
    #[cfg(feature = "gateway")]
    Sse {
        chunks: Vec<Result<Bytes>>,
        pending: bool,
    },
}
struct TestTransport {
    requests: Mutex<Vec<TransportRequest>>,
    replies: Mutex<VecDeque<Reply>>,
    polls: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
}
impl TestTransport {
    fn new(replies: Vec<Reply>) -> Arc<Self> {
        Arc::new(Self {
            requests: Mutex::new(Vec::new()),
            replies: Mutex::new(replies.into()),
            polls: Arc::new(AtomicUsize::new(0)),
            drops: Arc::new(AtomicUsize::new(0)),
        })
    }
    fn count(&self) -> usize {
        self.requests.lock().unwrap().len()
    }
}
struct TestBody {
    chunks: VecDeque<Result<Bytes>>,
    pending: bool,
    polls: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
}
impl Stream for TestBody {
    type Item = Result<Bytes>;
    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.polls.fetch_add(1, Ordering::SeqCst);
        if let Some(value) = self.chunks.pop_front() {
            Poll::Ready(Some(value))
        } else if self.pending {
            Poll::Pending
        } else {
            Poll::Ready(None)
        }
    }
}
impl Drop for TestBody {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}
impl Transport for TestTransport {
    fn send<'a>(&'a self, request: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.lock().unwrap().push(request);
            let reply = self
                .replies
                .lock()
                .unwrap()
                .pop_front()
                .expect("unexpected extra network request");
            let mut headers = HeaderMap::new();
            let (status, chunks, pending) = match reply {
                Reply::Json(value) => {
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        HeaderValue::from_static("application/json"),
                    );
                    (
                        StatusCode::OK,
                        vec![Ok(Bytes::from(serde_json::to_vec(&value).unwrap()))],
                        false,
                    )
                }
                #[cfg(feature = "gateway")]
                Reply::Binary(chunks) => {
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        HeaderValue::from_static("application/octet-stream"),
                    );
                    (StatusCode::OK, chunks.into_iter().map(Ok).collect(), false)
                }
                #[cfg(feature = "gateway")]
                Reply::Http(code) => (StatusCode::from_u16(code).unwrap(), vec![], false),
                #[cfg(feature = "gateway")]
                Reply::Failure => return Err(Error::new(ErrorKind::Transport)),
                #[cfg(feature = "gateway")]
                Reply::Sse { chunks, pending } => {
                    headers.insert(
                        http::header::CONTENT_TYPE,
                        HeaderValue::from_static("text/event-stream"),
                    );
                    (StatusCode::OK, chunks, pending)
                }
            };
            Ok(TransportResponse::new(
                status,
                headers,
                Box::pin(TestBody {
                    chunks: chunks.into(),
                    pending,
                    polls: self.polls.clone(),
                    drops: self.drops.clone(),
                }),
            ))
        })
    }
}
fn assert_header_contract(headers: &HeaderMap, expected_names: &[&str]) {
    let mut names: Vec<_> = headers.keys().map(http::HeaderName::as_str).collect();
    names.sort_unstable();
    assert_eq!(names, expected_names);
    assert_eq!(
        headers.len(),
        expected_names.len(),
        "duplicate header values"
    );
    assert_eq!(headers["service-name"], "api");
    assert_eq!(
        headers[http::header::USER_AGENT],
        concat!("PAN-AIRS/", env!("CARGO_PKG_VERSION"), "-rust-sdk")
    );
}
#[cfg(feature = "management")]
struct ServiceToken {
    context: AuthContext,
    calls: AtomicUsize,
}
#[cfg(feature = "management")]
impl ServiceToken {
    fn new() -> Arc<Self> {
        Arc::new(Self {
            context: AuthContext::new(
                Endpoint::parse("https://issuer.example/token").unwrap(),
                "synthetic-service-client",
                1,
                "123456789",
            )
            .unwrap(),
            calls: AtomicUsize::new(0),
        })
    }
}
#[cfg(feature = "management")]
impl TokenProvider for ServiceToken {
    fn token<'a>(&'a self, _: &'a RequestOptions) -> BoxFuture<'a, Result<TokenLease>> {
        Box::pin(async move {
            self.calls.fetch_add(1, Ordering::SeqCst);
            TokenLease::new(
                Secret::new("synthetic-scm-bearer").unwrap(),
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

#[cfg(feature = "management")]
mod management {
    use super::*;
    use scm_sdk::airs::management::{ManagementClient, PaginationOptions};

    fn client(transport: Arc<TestTransport>, provider: Arc<ServiceToken>) -> ManagementClient {
        ManagementClient::new(
            Client::builder(Endpoint::parse("https://management.example").unwrap())
                .transport(transport)
                .authentication(Authentication::oauth(provider, true))
                .build()
                .unwrap(),
        )
        .unwrap()
    }

    #[tokio::test]
    async fn token_profile_list_uses_exact_default_route_query_and_oauth_headers() {
        let payload = json!({"ai_profiles":[{"profile_id":"fixture-profile-id","profile_name":"fixture-profile",
            "active":true,"future_profile_field":{"state":"future"}}],"next_offset":100,
            "future_page_field":{"count":1}});
        let transport = TestTransport::new(vec![Reply::Json(payload.clone())]);
        let provider = ServiceToken::new();
        let client = client(transport.clone(), provider.clone());
        let page = client
            .profiles()
            .list_for_token(PaginationOptions::default(), None)
            .await
            .unwrap();
        assert_eq!(serde_json::to_value(&page).unwrap(), payload);
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let request = &requests[0];
        assert_eq!(request.method(), Method::GET);
        assert_eq!(
            request.url().as_str(),
            "https://management.example/v1/mgmt/profiles?offset=0&limit=100"
        );
        assert_eq!(
            request.headers()[http::header::AUTHORIZATION],
            "Bearer synthetic-scm-bearer"
        );
        assert_eq!(request.headers()["x-tsg-id"], "123456789");
        assert_header_contract(
            request.headers(),
            &["authorization", "service-name", "user-agent", "x-tsg-id"],
        );
        assert!(!request.headers().contains_key("x-portkey-api-key"));
        assert!(!request.headers().contains_key("x-pan-token"));
        assert!(request.body().is_empty());
        assert_eq!(provider.calls.load(Ordering::SeqCst), 1);
        assert!(!format!("{page:?} {client:?} {request:?}").contains("synthetic-scm-bearer"));
    }

    #[tokio::test]
    async fn pagination_latest_and_defaulted_empty_profiles_preserve_wire_contract() {
        let transport = TestTransport::new(vec![Reply::Json(json!({"future_page":true}))]);
        let client = client(transport.clone(), ServiceToken::new());
        let mut page = PaginationOptions::default();
        page.offset = 5;
        page.limit = 25;
        page.latest = Some(true);
        let result = client.profiles().list_for_token(page, None).await.unwrap();
        assert_eq!(
            serde_json::to_value(result).unwrap(),
            json!({"ai_profiles":[],"future_page":true})
        );
        assert_eq!(
            transport.requests.lock().unwrap()[0].url().query(),
            Some("offset=5&limit=25&latest=true")
        );
    }

    #[tokio::test]
    async fn invalid_pagination_is_rejected_before_token_or_network_work() {
        let transport = TestTransport::new(vec![]);
        let provider = ServiceToken::new();
        let client = client(transport.clone(), provider.clone());
        for (offset, limit) in [(0, 0), (9_007_199_254_740_992, 100)] {
            let mut page = PaginationOptions::default();
            page.offset = offset;
            page.limit = limit;
            let error = client
                .profiles()
                .list_for_token(page, None)
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
}

#[cfg(feature = "gateway")]
mod gateway {
    use super::*;
    use scm_sdk::{
        airs::gateway::{RuntimeClient, models::GatewayChatCompletionRequest},
        data::Patch,
        validation::Mode,
    };
    const MODEL: &str = "@provider/team/model-α";

    fn request(stream: Patch<bool>) -> GatewayChatCompletionRequest {
        let mut value = GatewayChatCompletionRequest::parse(
            json!({"messages":[{"role":"user","content":"fixture prompt"}],
            "model":MODEL}),
            Mode::Request,
        )
        .unwrap();
        value.stream = stream;
        value
    }
    fn client(transport: Arc<TestTransport>, limits: Limits) -> RuntimeClient {
        RuntimeClient::new(
            Client::builder(Endpoint::parse("https://runtime.example/v1").unwrap())
                .transport(transport)
                .authentication(
                    Authentication::runtime_key(Secret::new("synthetic-runtime-key").unwrap())
                        .unwrap(),
                )
                .limits(limits)
                .build()
                .unwrap(),
        )
        .unwrap()
    }
    fn completion() -> Value {
        json!({"id":"fixture-completion","object":"chat.completion","created":0,"model":MODEL,
        "choices":[{"index":0,"finish_reason":"stop","message":{"role":"assistant","content":"fixture response",
            "future_message":{"retained":true}}}],"future_response":{"retained":true}})
    }
    fn usage_chunk() -> Value {
        json!({"id":"fixture-usage","object":"chat.completion.chunk","created":0,"model":MODEL,
        "choices":[],"usage":{"prompt_tokens":3,"completion_tokens":2,"total_tokens":5},"future_chunk":{"retained":true}})
    }
    fn sse(value: &Value, done: bool) -> Reply {
        Reply::Sse {
            chunks: vec![Ok(Bytes::from(format!(
                "data: {}\n\n{}",
                value,
                if done { "data: [DONE]\n\n" } else { "" }
            )))],
            pending: !done,
        }
    }
    fn assert_runtime_wire(transport: &TestTransport, stream: Option<bool>) {
        let requests = transport.requests.lock().unwrap();
        let request = &requests[0];
        assert_eq!(request.method(), Method::POST);
        assert_eq!(
            request.url().as_str(),
            "https://runtime.example/v1/chat/completions"
        );
        assert_eq!(
            request.headers()["x-portkey-api-key"],
            "synthetic-runtime-key"
        );
        assert_header_contract(
            request.headers(),
            &[
                "content-type",
                "service-name",
                "user-agent",
                "x-portkey-api-key",
            ],
        );
        assert!(!request.headers().contains_key(http::header::AUTHORIZATION));
        assert!(!request.headers().contains_key("x-tsg-id"));
        assert!(!request.headers().contains_key("x-pan-token"));
        let body: Value = serde_json::from_slice(request.body()).unwrap();
        assert_eq!(body["model"], MODEL);
        assert_eq!(body["messages"][0]["content"], "fixture prompt");
        if let Some(value) = stream {
            assert_eq!(body["stream"], value);
        } else {
            assert!(body.get("stream").is_none());
        }
    }

    #[tokio::test]
    async fn download_preserves_frozen_non_utf8_bytes_and_encoded_identifier_contract() {
        // Independent frozen fixtures:
        // contracts/fixtures/typescript/test/ai-gateway/inference.spec.ts uses
        // exactly [0, 255, 10] and the encoded file/a?b#c identifier below.
        // test/openapi/fixtures/gateway-runtime.json pins downloadFile as
        // GET /files/{file_id}/content with no query and a non-JSON response.
        let transport = TestTransport::new(vec![Reply::Binary(vec![
            Bytes::from_static(&[0]),
            Bytes::from_static(&[255, 10]),
        ])]);
        let client = client(transport.clone(), Limits::default());
        let bytes = client.download_file("file/a?b#c", None).await.unwrap();
        assert_eq!(bytes.as_ref(), &[0, 255, 10]);
        assert!(std::str::from_utf8(&bytes).is_err());
        let requests = transport.requests.lock().unwrap();
        assert_eq!(requests.len(), 1);
        let request = &requests[0];
        assert_eq!(request.method(), Method::GET);
        assert_eq!(
            request.url().as_str(),
            "https://runtime.example/v1/files/file%2Fa%3Fb%23c/content"
        );
        assert!(request.url().query().is_none());
        assert_eq!(
            request.headers()["x-portkey-api-key"],
            "synthetic-runtime-key"
        );
        assert_header_contract(
            request.headers(),
            &["service-name", "user-agent", "x-portkey-api-key"],
        );
        assert!(!request.headers().contains_key(http::header::AUTHORIZATION));
        assert!(!request.headers().contains_key("x-tsg-id"));
        assert!(!request.headers().contains_key("x-pan-token"));
        assert!(!request.headers().contains_key(http::header::CONTENT_TYPE));
        assert!(request.body().is_empty());
        assert_eq!(transport.drops.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn oversized_binary_download_is_unknown_and_releases_its_body_and_permit() {
        let transport = TestTransport::new(vec![
            Reply::Binary(vec![
                Bytes::from_static(&[0, 255]),
                Bytes::from_static(&[10]),
            ]),
            Reply::Binary(vec![Bytes::from_static(b"ok")]),
        ]);
        let mut limits = Limits::default();
        limits.max_response_bytes = 2;
        limits.max_concurrency = 1;
        let client = client(transport.clone(), limits);
        let error = client.download_file("oversized", None).await.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Limit);
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        assert_eq!(transport.count(), 1);
        assert_eq!(transport.drops.load(Ordering::SeqCst), 1);
        assert!(!format!("{error:?} {error}").contains("synthetic-runtime-key"));
        let bytes =
            tokio::time::timeout(Duration::from_secs(1), client.download_file("next", None))
                .await
                .unwrap()
                .unwrap();
        assert_eq!(bytes.as_ref(), b"ok");
        assert_eq!(transport.count(), 2);
        assert_eq!(transport.drops.load(Ordering::SeqCst), 2);
    }

    #[tokio::test]
    async fn empty_or_traversing_file_identifiers_are_rejected_before_dispatch() {
        let transport = TestTransport::new(vec![]);
        let client = client(transport.clone(), Limits::default());
        for id in [
            "",
            ".",
            "..",
            "../secret",
            "..\\secret",
            "..%2fsecret",
            "%2E%2E%2Fsecret",
            "%252e%252e%252fsecret",
            "file/../secret",
            "file\0secret",
        ] {
            let error = client.download_file(id, None).await.unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Validation);
            assert_eq!(error.remote_outcome(), RemoteOutcome::NotSent);
        }
        assert_eq!(transport.count(), 0);
        assert_eq!(transport.polls.load(Ordering::SeqCst), 0);
    }

    #[tokio::test]
    async fn chat_json_preserves_model_string_response_fields_and_runtime_credentials() {
        let payload = completion();
        let transport = TestTransport::new(vec![Reply::Json(payload.clone())]);
        let client = client(transport.clone(), Limits::default());
        let value = client
            .create_chat_completion(&request(Patch::Unset), None)
            .await
            .unwrap();
        assert_eq!(serde_json::to_value(&value).unwrap(), payload);
        assert_runtime_wire(&transport, None);
        assert_eq!(transport.count(), 1);
        assert!(!format!("{value:?} {client:?}").contains("synthetic-runtime-key"));
    }

    #[tokio::test]
    async fn streaming_requires_explicit_true_and_json_method_rejects_true() {
        let transport = TestTransport::new(vec![]);
        let client = client(transport.clone(), Limits::default());
        for flag in [Patch::Unset, Patch::Null, Patch::Value(false)] {
            let error = client
                .create_chat_completion_stream(&request(flag), None)
                .await
                .unwrap_err();
            assert_eq!(
                (error.kind(), error.remote_outcome()),
                (ErrorKind::Validation, RemoteOutcome::NotSent)
            );
        }
        let error = client
            .create_chat_completion(&request(Patch::Value(true)), None)
            .await
            .unwrap_err();
        assert_eq!(
            (error.kind(), error.remote_outcome()),
            (ErrorKind::Validation, RemoteOutcome::NotSent)
        );
        assert_eq!(transport.count(), 0);
    }

    #[tokio::test]
    async fn typed_usage_only_sse_chunk_is_valid_and_unknown_fields_survive() {
        let payload = usage_chunk();
        let transport = TestTransport::new(vec![sse(&payload, true)]);
        let client = client(transport.clone(), Limits::default());
        let mut stream = client
            .create_chat_completion_stream(&request(Patch::Value(true)), None)
            .await
            .unwrap();
        assert_eq!(transport.polls.load(Ordering::SeqCst), 0);
        let event = stream.next().await.unwrap().unwrap();
        assert!(event.choices.is_empty());
        assert_eq!(serde_json::to_value(event).unwrap(), payload);
        assert!(stream.next().await.unwrap().is_none());
        assert_eq!(transport.drops.load(Ordering::SeqCst), 1);
        assert_runtime_wire(&transport, Some(true));
        assert_eq!(transport.count(), 1);
    }

    #[tokio::test]
    async fn typed_stream_skips_interleaved_empty_data_keepalives_and_reaches_terminal() {
        // Frozen src/http/event-stream.ts skips an empty joined data payload.
        // The raw SSE codec still exposes it for callers that need raw frames.
        let first = usage_chunk();
        let mut second = usage_chunk();
        second["id"] = json!("fixture-usage-second");
        let frames = format!(
            "data:\n\ndata: {first}\n\ndata: \r\n\r\ndata: {second}\n\ndata:\n\ndata: [DONE]\n\n"
        );
        let transport = TestTransport::new(vec![Reply::Sse {
            chunks: frames
                .as_bytes()
                .chunks(7)
                .map(|bytes| Ok(Bytes::copy_from_slice(bytes)))
                .collect(),
            pending: true,
        }]);
        let client = client(transport.clone(), Limits::default());
        let mut stream = client
            .create_chat_completion_stream(&request(Patch::Value(true)), None)
            .await
            .unwrap();
        assert_eq!(
            serde_json::to_value(stream.next().await.unwrap().unwrap()).unwrap(),
            first
        );
        assert_eq!(
            serde_json::to_value(stream.next().await.unwrap().unwrap()).unwrap(),
            second
        );
        assert!(stream.next().await.unwrap().is_none());
        assert_eq!(transport.drops.load(Ordering::SeqCst), 1);
        assert_eq!(transport.count(), 1);
        assert_runtime_wire(&transport, Some(true));
    }

    #[tokio::test]
    async fn malformed_sse_is_unknown_closes_and_never_replays() {
        for payload in [
            json!({"id":"synthetic-private-body"}),
            json!({"id":"fixture","object":"chat.completion.chunk",
            "created":"synthetic-private-body","model":MODEL,"choices":[]}),
        ] {
            let transport = TestTransport::new(vec![sse(&payload, false)]);
            let client = client(transport.clone(), Limits::default());
            let mut stream = client
                .create_chat_completion_stream(&request(Patch::Value(true)), None)
                .await
                .unwrap();
            let error = stream.next().await.unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Decode);
            assert_eq!(error.status(), Some(200));
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
            assert_eq!(transport.drops.load(Ordering::SeqCst), 1);
            assert_eq!(transport.count(), 1);
            assert!(!format!("{error:?} {error} {stream:?}").contains("synthetic-private-body"));
            assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Decode);
            assert_eq!(transport.count(), 1);
        }
    }

    #[tokio::test]
    async fn stream_drop_before_or_after_iteration_releases_the_concurrency_permit() {
        for read_first in [false, true] {
            let transport =
                TestTransport::new(vec![sse(&usage_chunk(), false), Reply::Json(completion())]);
            let mut limits = Limits::default();
            limits.max_concurrency = 1;
            let client = client(transport.clone(), limits);
            let mut stream = client
                .create_chat_completion_stream(&request(Patch::Value(true)), None)
                .await
                .unwrap();
            if read_first {
                stream.next().await.unwrap().unwrap();
            } else {
                assert_eq!(transport.polls.load(Ordering::SeqCst), 0);
            }
            drop(stream);
            assert_eq!(transport.drops.load(Ordering::SeqCst), 1);
            tokio::time::timeout(
                Duration::from_secs(1),
                client.create_chat_completion(&request(Patch::Unset), None),
            )
            .await
            .unwrap()
            .unwrap();
            assert_eq!(transport.count(), 2);
        }
    }

    #[tokio::test]
    async fn paid_post_and_stream_failures_are_never_replayed() {
        for stream in [false, true] {
            for reply in [Reply::Http(503), Reply::Failure] {
                let transport = TestTransport::new(vec![reply]);
                let client = client(transport.clone(), Limits::default());
                let error = if stream {
                    client
                        .create_chat_completion_stream(&request(Patch::Value(true)), None)
                        .await
                        .unwrap_err()
                } else {
                    client
                        .create_chat_completion(&request(Patch::Unset), None)
                        .await
                        .unwrap_err()
                };
                assert!(matches!(
                    error.kind(),
                    ErrorKind::Http | ErrorKind::Transport
                ));
                assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
                assert_eq!(transport.count(), 1);
            }
        }
        let transport = TestTransport::new(vec![Reply::Sse {
            chunks: vec![Err(Error::new(ErrorKind::Transport))],
            pending: false,
        }]);
        let client = client(transport.clone(), Limits::default());
        let mut stream = client
            .create_chat_completion_stream(&request(Patch::Value(true)), None)
            .await
            .unwrap();
        assert_eq!(
            stream.next().await.unwrap_err().remote_outcome(),
            RemoteOutcome::Unknown
        );
        assert_eq!(transport.count(), 1);
        assert_eq!(transport.drops.load(Ordering::SeqCst), 1);
    }

    #[tokio::test(start_paused = true)]
    async fn typed_stream_total_budget_and_caller_override_cover_buffered_events() {
        for (caller_override, session_cap, should_expire) in [
            (false, None, true),
            (true, None, false),
            (true, Some(Duration::from_secs(1)), true),
        ] {
            let frame = format!(
                "data: {}\n\ndata: {}\n\ndata: [DONE]\n\n",
                usage_chunk(),
                usage_chunk()
            );
            let transport = TestTransport::new(vec![Reply::Sse {
                chunks: vec![Ok(Bytes::from(frame))],
                pending: true,
            }]);
            let mut limits = Limits::default();
            limits.operation_timeout = Duration::from_secs(1);
            limits.stream_session_timeout = session_cap;
            let client = client(transport.clone(), limits);
            let options = caller_override
                .then(|| RequestOptions::with_timeout(Duration::from_secs(10)).unwrap());
            let mut stream = client
                .create_chat_completion_stream(&request(Patch::Value(true)), options.as_ref())
                .await
                .unwrap();
            stream.next().await.unwrap().unwrap();
            tokio::time::advance(Duration::from_secs(2)).await;
            if should_expire {
                let error = stream.next().await.unwrap_err();
                assert_eq!(error.kind(), ErrorKind::Deadline);
                assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
            } else {
                assert!(stream.next().await.unwrap().is_some());
                assert!(stream.next().await.unwrap().is_none());
            }
            assert_eq!(transport.count(), 1);
            assert_eq!(transport.drops.load(Ordering::SeqCst), 1);
        }
    }
}

#[cfg(all(feature = "management", feature = "gateway"))]
#[test]
fn product_clients_reject_credentials_from_the_other_plane_before_network() {
    use scm_sdk::airs::{gateway::RuntimeClient, management::ManagementClient};
    let transport = TestTransport::new(vec![]);
    let scm = Client::builder(Endpoint::parse("https://management.example").unwrap())
        .transport(transport.clone())
        .authentication(Authentication::oauth(ServiceToken::new(), true))
        .build()
        .unwrap();
    let runtime = Client::builder(Endpoint::parse("https://runtime.example").unwrap())
        .transport(transport.clone())
        .authentication(
            Authentication::runtime_key(Secret::new("synthetic-runtime-key").unwrap()).unwrap(),
        )
        .build()
        .unwrap();
    assert_eq!(
        RuntimeClient::new(scm).unwrap_err().kind(),
        ErrorKind::Configuration
    );
    assert_eq!(
        ManagementClient::new(runtime).unwrap_err().kind(),
        ErrorKind::Configuration
    );
    assert_eq!(transport.count(), 0);
}
