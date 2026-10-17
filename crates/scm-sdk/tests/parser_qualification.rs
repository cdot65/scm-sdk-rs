//! Deterministic public-interface SSE qualification. No network, service credentials,
//! random dependencies, or generated parser expectations are used.
// Public non-exhaustive Limits are configured through Default and field updates.
#![allow(clippy::field_reassign_with_default)]

use bytes::Bytes;
use futures_util::{Stream, future::BoxFuture};
use http::{HeaderMap, HeaderValue, Method, StatusCode};
use scm_sdk::{
    Client, Endpoint, Error, ErrorKind, Limits, RawRequest, RemoteOutcome, RequestOptions, Result,
    Secret,
    auth::Authentication,
    streaming::{SseStream, TerminalPolicy},
    transport::{Transport, TransportRequest, TransportResponse},
};
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

struct Body {
    chunks: VecDeque<Result<Bytes>>,
    pending: bool,
    polls: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
}
impl Stream for Body {
    type Item = Result<Bytes>;
    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.polls.fetch_add(1, Ordering::SeqCst);
        match self.chunks.pop_front() {
            Some(chunk) => Poll::Ready(Some(chunk)),
            None if self.pending => Poll::Pending,
            None => Poll::Ready(None),
        }
    }
}
impl Drop for Body {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}
struct FixtureTransport {
    body: Mutex<Option<Body>>,
    requests: AtomicUsize,
    polls: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
}
impl Transport for FixtureTransport {
    fn send<'a>(&'a self, _: TransportRequest) -> BoxFuture<'a, Result<TransportResponse>> {
        Box::pin(async move {
            self.requests.fetch_add(1, Ordering::SeqCst);
            let body = self.body.lock().unwrap().take();
            let mut headers = HeaderMap::new();
            headers.insert(
                http::header::CONTENT_TYPE,
                HeaderValue::from_static("text/event-stream; charset=utf-8"),
            );
            headers.insert("x-request-id", HeaderValue::from_static("parser-fixture"));
            Ok(match body {
                Some(body) => TransportResponse::new(StatusCode::OK, headers, Box::pin(body)),
                None => TransportResponse::from_bytes(StatusCode::OK, headers, Bytes::new()),
            })
        })
    }
}
struct Fixture {
    client: Client,
    transport: Arc<FixtureTransport>,
    options: RequestOptions,
}
impl Fixture {
    fn new(chunks: Vec<Result<Bytes>>, pending: bool, mut limits: Limits) -> Self {
        let polls = Arc::new(AtomicUsize::new(0));
        let drops = Arc::new(AtomicUsize::new(0));
        let transport = Arc::new(FixtureTransport {
            body: Mutex::new(Some(Body {
                chunks: chunks.into(),
                pending,
                polls: polls.clone(),
                drops: drops.clone(),
            })),
            requests: AtomicUsize::new(0),
            polls,
            drops,
        });
        limits.max_concurrency = 1;
        let client = Client::builder(Endpoint::parse("https://parser.invalid").unwrap())
            .authentication(
                Authentication::runtime_key(Secret::new("synthetic-parser-key").unwrap()).unwrap(),
            )
            .transport(transport.clone())
            .limits(limits)
            .build()
            .unwrap();
        Self {
            client,
            transport,
            options: RequestOptions::with_timeout(Duration::from_secs(5)).unwrap(),
        }
    }
    async fn open(&self, limit: usize, policy: TerminalPolicy) -> SseStream {
        let response = self
            .client
            .send_stream(RawRequest::new(Method::POST, "events"), Some(&self.options))
            .await
            .unwrap();
        SseStream::new(response, limit, policy).unwrap()
    }
    async fn released(&self) {
        assert_eq!(self.transport.drops.load(Ordering::SeqCst), 1);
        assert_eq!(self.transport.requests.load(Ordering::SeqCst), 1);
        // A second request proves the sole concurrency permit has been released.
        let options = RequestOptions::with_timeout(Duration::from_millis(100)).unwrap();
        self.client
            .send_bytes(RawRequest::new(Method::GET, "followup"), Some(&options))
            .await
            .unwrap();
        assert_eq!(self.transport.requests.load(Ordering::SeqCst), 2);
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Event {
    name: String,
    data: String,
    id: Option<String>,
    retry: Option<Duration>,
}
#[derive(Debug, PartialEq, Eq)]
struct Outcome {
    events: Vec<Event>,
    error: Option<ErrorKind>,
}
async fn parse(chunks: Vec<Bytes>, limit: usize, policy: TerminalPolicy) -> Outcome {
    let chunk_count = chunks.len();
    let fixture = Fixture::new(
        chunks.into_iter().map(Ok).collect(),
        false,
        Limits::default(),
    );
    let mut stream = fixture.open(limit, policy).await;
    let mut outcome = Outcome {
        events: Vec::new(),
        error: None,
    };
    for step in 0..128 {
        match stream.next().await {
            Ok(Some(event)) => outcome.events.push(Event {
                name: event.event().to_owned(),
                data: event.data().to_owned(),
                id: event.id().map(str::to_owned),
                retry: event.retry(),
            }),
            Ok(None) => break,
            Err(error) => {
                assert_eq!(error.status(), Some(200));
                assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
                assert!(!format!("{error:?} {error}").contains("synthetic-parser-key"));
                outcome.error = Some(error.kind());
                assert_eq!(stream.next().await.unwrap_err().kind(), error.kind());
                break;
            }
        }
        assert!(step < 127, "bounded corpus cannot produce 128 events");
    }
    assert!(fixture.transport.polls.load(Ordering::SeqCst) <= chunk_count + 1);
    fixture.released().await;
    outcome
}

fn random(seed: &mut u32) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 17;
    *seed ^= *seed << 5;
    *seed
}
fn chunked(bytes: &[u8], seed: &mut u32) -> Vec<Bytes> {
    let mut chunks = Vec::new();
    let mut cursor = 0;
    while cursor < bytes.len() {
        let end = (cursor + 1 + random(seed) as usize % 13).min(bytes.len());
        chunks.push(Bytes::copy_from_slice(&bytes[cursor..end]));
        cursor = end;
    }
    chunks
}

#[tokio::test]
async fn exhaustive_short_frame_partitions_preserve_literal_expectations() {
    let mut cases = 0;
    for input in ["data:x\n\n", "data:λ\r\r"] {
        let bytes = input.as_bytes();
        let expected = Outcome {
            events: vec![Event {
                name: "message".into(),
                data: if input.contains('λ') { "λ" } else { "x" }.into(),
                id: None,
                retry: None,
            }],
            error: None,
        };
        for mask in 0..1usize << (bytes.len() - 1) {
            let mut chunks = Vec::new();
            let mut start = 0;
            for end in 1..bytes.len() {
                if mask & (1 << (end - 1)) != 0 {
                    chunks.push(Bytes::copy_from_slice(&bytes[start..end]));
                    start = end;
                }
            }
            chunks.push(Bytes::copy_from_slice(&bytes[start..]));
            assert_eq!(parse(chunks, 128, TerminalPolicy::Eof).await, expected);
            cases += 1;
        }
    }
    assert_eq!(cases, 384);
}

#[tokio::test]
async fn every_two_chunk_split_handles_bom_utf8_crlf_ids_and_multiline_data() {
    let input = concat!(
        "\u{feff}: keepalive\r\nid: λ\r\nretry: 0\r\nevent: delta\r\n",
        "data: café\r\ndata: 👋\r\n\r\nid:\ndata:\n\ndata: [DONE]\r\n\r\nignored",
    );
    let bytes = input.as_bytes();
    let expected = Outcome {
        events: vec![
            Event {
                name: "delta".into(),
                data: "café\n👋".into(),
                id: Some("λ".into()),
                retry: Some(Duration::ZERO),
            },
            Event {
                name: "message".into(),
                data: String::new(),
                id: Some(String::new()),
                retry: Some(Duration::ZERO),
            },
        ],
        error: None,
    };
    for split in 0..=bytes.len() {
        assert_eq!(
            parse(
                vec![
                    Bytes::copy_from_slice(&bytes[..split]),
                    Bytes::copy_from_slice(&bytes[split..]),
                ],
                256,
                TerminalPolicy::DoneMarker,
            )
            .await,
            expected
        );
    }
}

#[tokio::test]
async fn fixed_seed_mutations_are_chunk_invariant_bounded_and_release_resources() {
    let corpus: [&[u8]; 6] = [
        b"data: {\"n\":1}\n\ndata: [DONE]\n\n",
        "\u{feff}data: café 👋\r\n\r\n".as_bytes(),
        b"id: fixture\rretry: 5\rdata: first\r\rdata: second\r\r",
        b"event: error\ndata: synthetic-error\n\n",
        b"data: {\"type\":\"response.completed\"}\n\n",
        b": comment\nunknown: value\ndata:\n\n",
    ];
    let mut cases = 0;
    for initial_seed in [1, 0x1357_9bdf, 0x5eed_cafe, 0xf00d_baad] {
        let mut seed = initial_seed;
        for iteration in 0..512 {
            let mut bytes = corpus[random(&mut seed) as usize % corpus.len()].to_vec();
            let position = random(&mut seed) as usize % (bytes.len() + 1);
            match iteration % 5 {
                0 => bytes.insert(position, random(&mut seed) as u8),
                1 if position < bytes.len() => {
                    bytes.remove(position);
                }
                2 if position < bytes.len() => bytes[position] = random(&mut seed) as u8,
                3 => bytes.truncate(position),
                _ => {
                    let suffix = bytes[position..].to_vec();
                    bytes.extend_from_slice(&suffix);
                }
            }
            assert!(bytes.len() <= 128);
            let policy = match iteration % 3 {
                0 => TerminalPolicy::Eof,
                1 => TerminalPolicy::DoneMarker,
                _ => TerminalPolicy::Responses,
            };
            let limit = 1 + random(&mut seed) as usize % 128;
            let expected = parse(vec![Bytes::copy_from_slice(&bytes)], limit, policy).await;
            let actual = parse(chunked(&bytes, &mut seed), limit, policy).await;
            assert_eq!(actual, expected, "seed {initial_seed:#x}, case {iteration}");
            cases += 1;
        }
    }
    assert_eq!(cases, 2048);
}

#[tokio::test]
async fn malformed_utf8_and_truncation_are_not_hidden_by_chunk_boundaries() {
    let corpus: [&[u8]; 9] = [
        b"data:\xff\n\n",
        b"data:\xc0\xaf\n\n",         // overlong encoding
        b"data:\xed\xa0\x80\n\n",     // UTF-16 surrogate
        b"data:\xf4\x90\x80\x80\n\n", // above U+10FFFF
        b"data:\xe2\x82",             // incomplete scalar
        b":\xff\n\n",                 // invalid UTF-8 in an ignored comment
        b"data: incomplete",
        b"data: incomplete\n",
        b"data:\n",
    ];
    for input in corpus {
        for split in 0..=input.len() {
            let outcome = parse(
                vec![
                    Bytes::copy_from_slice(&input[..split]),
                    Bytes::copy_from_slice(&input[split..]),
                ],
                128,
                TerminalPolicy::Eof,
            )
            .await;
            assert!(outcome.events.is_empty());
            assert_eq!(outcome.error, Some(ErrorKind::Protocol));
        }
    }
}

#[tokio::test]
async fn event_and_wire_bounds_count_comments_utf8_and_ignored_fields() {
    let input = "data: café\r\n\r\n";
    let normalized_bytes = input.replace("\r\n", "\n").len();
    for split in 0..=input.len() {
        let chunks = || {
            vec![
                Bytes::copy_from_slice(&input.as_bytes()[..split]),
                Bytes::copy_from_slice(&input.as_bytes()[split..]),
            ]
        };
        assert_eq!(
            parse(chunks(), normalized_bytes - 1, TerminalPolicy::Eof)
                .await
                .error,
            Some(ErrorKind::Limit)
        );
        assert_eq!(
            parse(chunks(), normalized_bytes, TerminalPolicy::Eof)
                .await
                .events[0]
                .data,
            "café"
        );
    }
    for input in [
        format!(":{}\n\n", "x".repeat(256)),
        format!("ignored:{}", "λ".repeat(128)),
    ] {
        assert_eq!(
            parse(vec![Bytes::from(input)], 128, TerminalPolicy::Eof)
                .await
                .error,
            Some(ErrorKind::Limit)
        );
    }
    let mut limits = Limits::default();
    limits.max_response_bytes = 8;
    let fixture = Fixture::new(
        vec![Ok(Bytes::from_static(b":one\n\n:two\n\n"))],
        false,
        limits,
    );
    let mut stream = fixture.open(128, TerminalPolicy::Eof).await;
    assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Limit);
    fixture.released().await;
}

#[tokio::test]
async fn lifecycle_json_not_event_name_proves_responses_completion() {
    for terminal in [
        "response.completed",
        "response.failed",
        "response.incomplete",
    ] {
        let fixture = Fixture::new(
            vec![Ok(Bytes::from(format!(
                "event: untrusted\ndata: {{\"type\":\"{terminal}\"}}\n\n"
            )))],
            true,
            Limits::default(),
        );
        let mut stream = fixture.open(128, TerminalPolicy::Responses).await;
        assert_eq!(
            stream
                .next()
                .await
                .unwrap()
                .unwrap()
                .json::<serde_json::Value>()
                .unwrap()["type"],
            terminal
        );
        assert!(stream.next().await.unwrap().is_none());
        fixture.released().await;
    }
    for input in [
        b"event: response.completed\ndata: {}\n\n".as_slice(),
        b"data: [DONE]\n\n".as_slice(),
        b"data: {\"type\":\"response.created\"}\n\n".as_slice(),
    ] {
        let outcome = parse(
            vec![Bytes::copy_from_slice(input)],
            128,
            TerminalPolicy::Responses,
        )
        .await;
        assert!(matches!(
            outcome.error,
            Some(ErrorKind::Protocol | ErrorKind::Decode)
        ));
    }
}

#[tokio::test]
async fn overflow_error_envelopes_fail_closed_with_or_without_arbitrary_precision() {
    for number in ["1e400", "-1e400"] {
        let fixture = Fixture::new(
            vec![Ok(Bytes::from(format!(
                "data: {{\"type\":\"response.created\",\"error\":{number}}}\n\n"
            )))],
            true,
            Limits::default(),
        );
        let mut stream = fixture.open(128, TerminalPolicy::Responses).await;
        let error = stream.next().await.unwrap_err();
        let expected = if serde_json::from_str::<serde_json::Value>(number).is_ok() {
            ErrorKind::Protocol
        } else {
            ErrorKind::Decode
        };
        assert_eq!(error.kind(), expected);
        assert_eq!(stream.next().await.unwrap_err().kind(), expected);
        fixture.released().await;
    }
}

#[tokio::test(start_paused = true)]
async fn cancellation_deadlines_drop_and_close_release_unread_or_buffered_bodies() {
    for state in 0..6 {
        let fixture = Fixture::new(
            vec![Ok(Bytes::from_static(b"data: one\n\ndata: two\n\n"))],
            true,
            Limits::default(),
        );
        let mut stream = fixture.open(128, TerminalPolicy::Eof).await;
        if state >= 3 {
            assert_eq!(stream.next().await.unwrap().unwrap().data(), "one");
        }
        match state % 3 {
            0 => {
                fixture.options.cancel();
                assert_eq!(
                    stream.next().await.unwrap_err().kind(),
                    ErrorKind::Cancelled
                );
            }
            1 => {
                fixture.client.session().cancel();
                assert_eq!(
                    stream.next().await.unwrap_err().kind(),
                    ErrorKind::Cancelled
                );
            }
            _ => {
                tokio::time::advance(Duration::from_secs(6)).await;
                assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Deadline);
            }
        }
        assert_eq!(fixture.transport.drops.load(Ordering::SeqCst), 1);
        // A cancelled session cannot make another request. Resource ownership is
        // checked directly here; noncancelled error paths also prove the permit.
        if state % 3 != 1 {
            fixture.released().await;
        }
    }
    for read_first in [false, true] {
        for explicit_close in [false, true] {
            let fixture = Fixture::new(
                vec![Ok(Bytes::from_static(b"data: one\n\ndata: two\n\n"))],
                true,
                Limits::default(),
            );
            let mut stream = fixture.open(128, TerminalPolicy::Eof).await;
            if read_first {
                stream.next().await.unwrap().unwrap();
            }
            if explicit_close {
                stream.close();
                stream.close();
                assert!(stream.next().await.unwrap().is_none());
            }
            drop(stream);
            fixture.released().await;
        }
    }
}

#[tokio::test(start_paused = true)]
async fn empty_chunk_flood_idle_and_transport_errors_latch_without_replay() {
    for (chunks, pending, expected) in [
        (vec![Ok(Bytes::new()); 129], false, ErrorKind::Protocol),
        (vec![], true, ErrorKind::Deadline),
        (
            vec![Err(Error::new(ErrorKind::Transport))],
            false,
            ErrorKind::Transport,
        ),
    ] {
        let mut limits = Limits::default();
        limits.stream_idle_timeout = Duration::from_millis(10);
        let fixture = Fixture::new(chunks, pending, limits);
        let mut stream = fixture.open(128, TerminalPolicy::Eof).await;
        assert_eq!(stream.next().await.unwrap_err().kind(), expected);
        assert_eq!(stream.next().await.unwrap_err().kind(), expected);
        fixture.released().await;
    }
}
