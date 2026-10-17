use super::*;
use crate::{Limits, RequestOptions, Session, transport::TransportResponse};
use futures_util::Stream;
use http::{HeaderMap, HeaderValue, StatusCode};
use std::{
    collections::VecDeque,
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    task::{Context, Poll},
};
use tokio::sync::Semaphore;

struct ObservedBody {
    chunks: VecDeque<Result<Bytes>>,
    polls: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
    pending_at_end: bool,
}
impl Stream for ObservedBody {
    type Item = Result<Bytes>;
    fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        self.polls.fetch_add(1, Ordering::SeqCst);
        if let Some(value) = self.chunks.pop_front() {
            Poll::Ready(Some(value))
        } else if self.pending_at_end {
            Poll::Pending
        } else {
            Poll::Ready(None)
        }
    }
}
impl Drop for ObservedBody {
    fn drop(&mut self) {
        self.drops.fetch_add(1, Ordering::SeqCst);
    }
}
struct Fixture {
    response: ResponseStream,
    polls: Arc<AtomicUsize>,
    drops: Arc<AtomicUsize>,
    permits: Arc<Semaphore>,
    options: RequestOptions,
    session: Session,
}
async fn response_fixture(
    chunks: Vec<Result<Bytes>>,
    content_type: Option<&str>,
    pending: bool,
    explicit_deadline: bool,
    limits: Limits,
) -> Fixture {
    let polls = Arc::new(AtomicUsize::new(0));
    let drops = Arc::new(AtomicUsize::new(0));
    let permits = Arc::new(Semaphore::new(1));
    let mut headers = HeaderMap::new();
    if let Some(value) = content_type {
        headers.insert(
            http::header::CONTENT_TYPE,
            HeaderValue::from_str(value).unwrap(),
        );
    }
    let response = TransportResponse::new(
        StatusCode::OK,
        headers,
        Box::pin(ObservedBody {
            chunks: chunks.into(),
            polls: polls.clone(),
            drops: drops.clone(),
            pending_at_end: pending,
        }),
    );
    let options = RequestOptions::with_timeout(limits.operation_timeout).unwrap();
    let session = Session::new();
    let response = ResponseStream::new(
        response,
        permits.clone().acquire_owned().await.unwrap(),
        session.clone(),
        &options,
        explicit_deadline,
        &limits,
        "fixture.sse",
    );
    Fixture {
        response,
        polls,
        drops,
        permits,
        options,
        session,
    }
}
async fn from_bytes(bytes: impl Into<Bytes>, policy: TerminalPolicy) -> SseStream {
    let fixture = response_fixture(
        vec![Ok(bytes.into())],
        Some("text/event-stream"),
        false,
        false,
        Limits::default(),
    )
    .await;
    SseStream::new(fixture.response, 1024 * 1024, policy).unwrap()
}

#[tokio::test]
async fn fields_multiline_comments_ids_retry_empty_data_and_eof() {
    let bytes = concat!(
        ": comment\r\nid: fixture-id\r\nretry: 15\r\nignored: fixture\r\nevent: delta\r\n",
        "data: {\"value\":\r\ndata: 1}\r\n\r\ndata: second\rid: ignored\0value\r",
        "retry: not-a-number\r\rid:\ndata:\n\n",
    )
    .as_bytes();
    let mut stream = from_bytes(Bytes::from_static(bytes), TerminalPolicy::Eof).await;
    let first = stream.next().await.unwrap().unwrap();
    assert_eq!(first.event(), "delta");
    assert_eq!(first.data(), "{\"value\":\n1}");
    assert_eq!(
        first.json::<Value>().unwrap(),
        serde_json::json!({"value":1})
    );
    assert_eq!(first.id(), Some("fixture-id"));
    assert_eq!(first.retry(), Some(Duration::from_millis(15)));
    let second = stream.next().await.unwrap().unwrap();
    assert_eq!(second.event(), "message");
    assert_eq!(second.data(), "second");
    assert_eq!(second.id(), Some("fixture-id"));
    assert_eq!(second.retry(), first.retry());
    let third = stream.next().await.unwrap().unwrap();
    assert_eq!(third.data(), "");
    assert_eq!(third.id(), Some(""));
    assert!(stream.next().await.unwrap().is_none());
}

#[tokio::test]
async fn done_marker_takes_precedence_over_error_event_name_as_in_frozen_parser() {
    let fixture = response_fixture(
        vec![Ok(Bytes::from_static(b"event: error\ndata: [DONE]\n\n"))],
        Some("text/event-stream"),
        true,
        false,
        Limits::default(),
    )
    .await;
    let mut stream = SseStream::new(fixture.response, 1024, TerminalPolicy::DoneMarker).unwrap();
    assert!(stream.next().await.unwrap().is_none());
    assert_eq!(fixture.drops.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.permits.available_permits(), 1);
    assert!(stream.next().await.unwrap().is_none());
}

#[tokio::test]
async fn bom_crlf_and_utf8_survive_every_two_chunk_split() {
    let input="\u{feff}: comment\r\nevent: delta\r\ndata: {\"text\":\"café 👋\"}\r\n\r\ndata: [DONE]\r\n\r\n".as_bytes();
    for split in 0..=input.len() {
        let fixture = response_fixture(
            vec![
                Ok(Bytes::copy_from_slice(&input[..split])),
                Ok(Bytes::copy_from_slice(&input[split..])),
            ],
            Some("Text/Event-Stream; charset=UTF-8"),
            false,
            false,
            Limits::default(),
        )
        .await;
        let mut stream =
            SseStream::new(fixture.response, 1024, TerminalPolicy::DoneMarker).unwrap();
        let event = stream.next().await.unwrap().unwrap();
        assert_eq!(event.event(), "delta");
        assert_eq!(
            event.json::<Value>().unwrap(),
            serde_json::json!({"text":"café 👋"})
        );
        assert!(stream.next().await.unwrap().is_none());
        assert_eq!(fixture.drops.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.permits.available_permits(), 1);
    }
}

#[tokio::test]
async fn independently_chunked_frames_preserve_event_sequence() {
    let input = concat!(
        "data: {\"text\":\"café 👋\"}\r\n\r\ndata: {\"text\":\"next λ\"}\n\n",
        "data: [DONE]\r\rignored: after-terminal",
    )
    .as_bytes();
    for initial_seed in 1..=32u32 {
        let mut seed = initial_seed;
        let mut cursor = 0;
        let mut chunks = Vec::new();
        while cursor < input.len() {
            seed ^= seed << 13;
            seed ^= seed >> 17;
            seed ^= seed << 5;
            let end = (cursor + (seed as usize % 17) + 1).min(input.len());
            chunks.push(Ok(Bytes::copy_from_slice(&input[cursor..end])));
            cursor = end;
        }
        let fixture = response_fixture(
            chunks,
            Some("text/event-stream"),
            false,
            false,
            Limits::default(),
        )
        .await;
        let mut stream =
            SseStream::new(fixture.response, 1024, TerminalPolicy::DoneMarker).unwrap();
        assert_eq!(
            stream
                .next()
                .await
                .unwrap()
                .unwrap()
                .json::<Value>()
                .unwrap()["text"],
            "café 👋"
        );
        assert_eq!(
            stream
                .next()
                .await
                .unwrap()
                .unwrap()
                .json::<Value>()
                .unwrap()["text"],
            "next λ"
        );
        assert!(stream.next().await.unwrap().is_none());
        assert_eq!(fixture.permits.available_permits(), 1);
    }
}

#[tokio::test]
async fn responses_deliver_each_terminal_lifecycle_then_release_immediately() {
    for terminal in [
        "response.completed",
        "response.failed",
        "response.incomplete",
    ] {
        let input = format!(
            "data: {{\"type\":\"response.output_text.delta\",\"delta\":\"fixture\"}}\n\nevent: server-controlled-name\ndata: {{\"type\":\"{terminal}\",\"response\":{{\"error\":{{\"message\":\"synthetic-secret\"}}}},\"future\":true}}\n\n"
        );
        let fixture = response_fixture(
            vec![Ok(Bytes::from(input))],
            Some("text/event-stream"),
            true,
            false,
            Limits::default(),
        )
        .await;
        let mut stream = SseStream::new(fixture.response, 4096, TerminalPolicy::Responses).unwrap();
        assert_eq!(
            stream
                .next()
                .await
                .unwrap()
                .unwrap()
                .json::<Value>()
                .unwrap()["type"],
            "response.output_text.delta"
        );
        let event = stream.next().await.unwrap().unwrap();
        assert_eq!(event.json::<Value>().unwrap()["type"], terminal);
        assert_eq!(event.json::<Value>().unwrap()["future"], true);
        assert_eq!(fixture.drops.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.permits.available_permits(), 1);
        assert!(stream.next().await.unwrap().is_none());
        assert_eq!(fixture.polls.load(Ordering::SeqCst), 1);
        assert!(!format!("{event:?} {stream:?}").contains("synthetic-secret"));
        assert!(!format!("{event:?}").contains("server-controlled-name"));
    }
}

#[tokio::test]
async fn required_terminal_cannot_be_spoofed_by_name_or_done_marker() {
    for input in [
        "data: [DONE]\n\n",
        "event: response.completed\ndata: {\"type\":\"response.created\"}\n\n",
        "event: response.completed\ndata: {}\n\n",
    ] {
        let mut stream = from_bytes(Bytes::from(input), TerminalPolicy::Responses).await;
        let error = loop {
            match stream.next().await {
                Ok(Some(_)) => {}
                Err(error) => break error,
                Ok(None) => panic!("missing typed terminal accepted"),
            }
        };
        assert!(matches!(
            error.kind(),
            ErrorKind::Decode | ErrorKind::Protocol
        ));
        assert_eq!(error.status(), Some(200));
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
    }
}

#[tokio::test]
async fn eof_policy_requires_complete_data_frames_and_required_policies_require_terminal() {
    for policy in [
        TerminalPolicy::DoneMarker,
        TerminalPolicy::Responses,
        TerminalPolicy::Eof,
    ] {
        for input in ["data: incomplete", "data: incomplete\n", "data:\n"] {
            let mut stream = from_bytes(Bytes::from(input), policy).await;
            let error = stream.next().await.unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Protocol);
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        }
    }
    let mut stream = from_bytes(
        Bytes::from_static(b"data: complete\n\n"),
        TerminalPolicy::DoneMarker,
    )
    .await;
    assert_eq!(stream.next().await.unwrap().unwrap().data(), "complete");
    assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Protocol);
    for input in [
        "",
        ": trailing comment",
        "ignored: trailing field",
        "\u{feff}",
    ] {
        assert!(
            from_bytes(Bytes::from(input), TerminalPolicy::Eof)
                .await
                .next()
                .await
                .unwrap()
                .is_none()
        );
    }
}

#[tokio::test]
async fn malformed_utf8_and_error_events_fail_without_sensitive_values() {
    for input in [
        b"data: \xff\n\n".as_slice(),
        b"data: \xe2\x82".as_slice(),
        b"event: error\ndata: synthetic-secret\n\n".as_slice(),
    ] {
        let mut stream = from_bytes(Bytes::copy_from_slice(input), TerminalPolicy::Eof).await;
        let error = stream.next().await.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Protocol);
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        assert!(!format!("{error:?} {error} {stream:?}").contains("synthetic-secret"));
        assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Protocol);
    }
}

#[tokio::test]
async fn event_byte_limit_counts_utf8_comments_and_unknown_fields() {
    let input = "data: café\r\n\r\n";
    // CRLF is normalized to one delimiter: the corresponding LF frame is 13 bytes.
    let normalized = input.replace("\r\n", "\n").len();
    for limit in [normalized - 1, normalized] {
        let fixture = response_fixture(
            vec![Ok(Bytes::from(input))],
            Some("text/event-stream"),
            false,
            false,
            Limits::default(),
        )
        .await;
        let mut stream = SseStream::new(fixture.response, limit, TerminalPolicy::Eof).unwrap();
        if limit < normalized {
            assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Limit);
        } else {
            assert_eq!(stream.next().await.unwrap().unwrap().data(), "café");
        }
    }
    for input in [
        format!(":{}", "x".repeat(200)),
        format!("unknown:{}\n\ndata: x\n\n", "x".repeat(200)),
    ] {
        let fixture = response_fixture(
            vec![Ok(Bytes::from(input))],
            Some("text/event-stream"),
            false,
            false,
            Limits::default(),
        )
        .await;
        let mut stream = SseStream::new(fixture.response, 64, TerminalPolicy::Eof).unwrap();
        assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Limit);
        assert_eq!(fixture.drops.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.permits.available_permits(), 1);
    }
}

#[tokio::test]
async fn invalid_media_type_or_limit_releases_body_before_reading() {
    for content_type in [
        None,
        Some("application/json"),
        Some("text/event-stream-extra"),
        Some("text/event-stream; charset=latin1"),
    ] {
        let fixture = response_fixture(
            vec![Ok(Bytes::from_static(b"synthetic-secret"))],
            content_type,
            false,
            false,
            Limits::default(),
        )
        .await;
        let error = SseStream::new(fixture.response, 128, TerminalPolicy::Eof).unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Protocol);
        assert_eq!(error.status(), Some(200));
        assert_eq!(fixture.polls.load(Ordering::SeqCst), 0);
        assert_eq!(fixture.drops.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.permits.available_permits(), 1);
    }
    for limit in [0, 16 * 1024 * 1024 + 1] {
        let fixture = response_fixture(
            vec![],
            Some("text/event-stream"),
            false,
            false,
            Limits::default(),
        )
        .await;
        assert_eq!(
            SseStream::new(fixture.response, limit, TerminalPolicy::Eof)
                .unwrap_err()
                .kind(),
            ErrorKind::Configuration
        );
        assert_eq!(fixture.polls.load(Ordering::SeqCst), 0);
        assert_eq!(fixture.permits.available_permits(), 1);
    }
}

#[tokio::test]
async fn cancellation_checks_buffered_events_and_releases_resources() {
    for session_cancel in [false, true] {
        let fixture = response_fixture(
            vec![Ok(Bytes::from_static(b"data: first\n\ndata: second\n\n"))],
            Some("text/event-stream"),
            true,
            false,
            Limits::default(),
        )
        .await;
        let mut stream = SseStream::new(fixture.response, 128, TerminalPolicy::Eof).unwrap();
        assert_eq!(stream.next().await.unwrap().unwrap().data(), "first");
        if session_cancel {
            fixture.session.cancel();
        } else {
            fixture.options.cancel();
        }
        let error = stream.next().await.unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Cancelled);
        assert_eq!(error.status(), Some(200));
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        assert_eq!(fixture.polls.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.drops.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.permits.available_permits(), 1);
    }
}

#[tokio::test(start_paused = true)]
async fn absolute_deadline_applies_to_buffered_events() {
    let limits = Limits {
        operation_timeout: Duration::from_secs(1),
        ..Limits::default()
    };
    let fixture = response_fixture(
        vec![Ok(Bytes::from_static(b"data: first\n\ndata: second\n\n"))],
        Some("text/event-stream"),
        true,
        true,
        limits,
    )
    .await;
    let mut stream = SseStream::new(fixture.response, 128, TerminalPolicy::Eof).unwrap();
    stream.next().await.unwrap().unwrap();
    tokio::time::advance(Duration::from_secs(2)).await;
    assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Deadline);
    assert_eq!(fixture.polls.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.permits.available_permits(), 1);
}

#[tokio::test(start_paused = true)]
async fn slow_consumer_has_no_prefetch_or_hidden_reader() {
    let fixture = response_fixture(
        vec![
            Ok(Bytes::from_static(b"data: first\n\n")),
            Ok(Bytes::from_static(b"data: second\n\n")),
        ],
        Some("text/event-stream"),
        false,
        false,
        Limits::default(),
    )
    .await;
    let mut stream = SseStream::new(fixture.response, 128, TerminalPolicy::Eof).unwrap();
    assert_eq!(fixture.polls.load(Ordering::SeqCst), 0);
    assert_eq!(stream.next().await.unwrap().unwrap().data(), "first");
    tokio::time::advance(Duration::from_secs(3600)).await;
    assert_eq!(fixture.polls.load(Ordering::SeqCst), 1);
    assert_eq!(stream.next().await.unwrap().unwrap().data(), "second");
    drop(stream);
    assert_eq!(fixture.drops.load(Ordering::SeqCst), 1);
    assert_eq!(fixture.permits.available_permits(), 1);
}

#[tokio::test(start_paused = true)]
async fn idle_body_and_transport_failures_are_bounded_and_sanitized() {
    let fixture = response_fixture(
        vec![],
        Some("text/event-stream"),
        true,
        false,
        Limits {
            stream_idle_timeout: Duration::from_millis(10),
            ..Limits::default()
        },
    )
    .await;
    let mut stream = SseStream::new(fixture.response, 128, TerminalPolicy::Eof).unwrap();
    assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Deadline);
    assert_eq!(fixture.permits.available_permits(), 1);
    let fixture = response_fixture(
        vec![Err(Error::new(ErrorKind::Transport))],
        Some("text/event-stream"),
        false,
        false,
        Limits::default(),
    )
    .await;
    let mut stream = SseStream::new(fixture.response, 128, TerminalPolicy::Eof).unwrap();
    let error = stream.next().await.unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Transport);
    assert_eq!(error.status(), Some(200));
    assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
}

#[tokio::test]
async fn typed_json_errors_and_formatting_never_echo_data_ids_or_event_names() {
    let mut stream = from_bytes(
        Bytes::from_static(
            concat!(
                "event: synthetic-event-secret\nid: synthetic-id-secret\n",
                "data: synthetic-data-secret\n\n",
            )
            .as_bytes(),
        ),
        TerminalPolicy::Eof,
    )
    .await;
    let event = stream.next().await.unwrap().unwrap();
    let error = event.json::<Value>().unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Decode);
    let output = format!("{event:?} {stream:?} {error:?} {error}");
    for secret in [
        "synthetic-event-secret",
        "synthetic-id-secret",
        "synthetic-data-secret",
    ] {
        assert!(!output.contains(secret));
    }
    let mut stream = from_bytes(
        Bytes::from_static(b"data: {\"error\":{\"message\":\"synthetic-secret\"}}\n\n"),
        TerminalPolicy::Eof,
    )
    .await;
    let error = stream
        .next()
        .await
        .unwrap()
        .unwrap()
        .json::<Value>()
        .unwrap_err();
    assert_eq!(error.kind(), ErrorKind::Protocol);
    assert!(!format!("{error:?} {error}").contains("synthetic-secret"));
}

#[tokio::test]
async fn numeric_error_envelopes_follow_javascript_truthiness_including_overflow() {
    for (number, truthy) in [
        ("0", false),
        ("-0", false),
        ("0.0", false),
        ("-0.0", false),
        ("0e400", false),
        ("-0e400", false),
        ("1e-400", false),
        ("-1e-400", false),
        ("5e-324", true),
        ("-5e-324", true),
        ("1", true),
        ("-1", true),
        ("0.000001", true),
        ("1e308", true),
        ("1e400", true),
        ("-1e400", true),
    ] {
        let data = format!("{{\"error\":{number},\"synthetic-secret\":true}}");
        let expected = if serde_json::from_str::<Value>(&data).is_err() {
            // Without arbitrary_precision, serde_json rejects overflow at parse.
            Some(ErrorKind::Decode)
        } else if truthy {
            Some(ErrorKind::Protocol)
        } else {
            None
        };
        let mut stream = from_bytes(
            Bytes::from(format!("data: {data}\n\n")),
            TerminalPolicy::Eof,
        )
        .await;
        let event = stream.next().await.unwrap().unwrap();
        match (event.json::<Value>(), expected) {
            (Ok(_), None) => {}
            (Err(error), Some(kind)) => {
                assert_eq!(error.kind(), kind, "numeric error {number}");
                assert_eq!(error.status(), Some(200));
                assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
                assert!(!format!("{error:?} {error}").contains("synthetic-secret"));
            }
            _ => panic!("wrong numeric error truthiness for {number}"),
        }
    }
}

#[tokio::test]
async fn overflow_response_error_fails_closed_and_releases_resources() {
    for number in ["1e400", "-1e400", r#"{"$serde_json::private::Number":"0"}"#] {
        let fixture = response_fixture(
            vec![Ok(Bytes::from(format!(
                "data: {{\"type\":\"response.created\",\"error\":{number}}}\n\n"
            )))],
            Some("text/event-stream"),
            true,
            false,
            Limits::default(),
        )
        .await;
        let mut stream = SseStream::new(fixture.response, 128, TerminalPolicy::Responses).unwrap();
        let error = stream.next().await.unwrap_err();
        assert!(matches!(
            error.kind(),
            ErrorKind::Protocol | ErrorKind::Decode
        ));
        assert_eq!(stream.next().await.unwrap_err().kind(), error.kind());
        assert_eq!(fixture.polls.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.drops.load(Ordering::SeqCst), 1);
        assert_eq!(fixture.permits.available_permits(), 1);
    }
}

#[tokio::test]
async fn protocol_named_literal_objects_fail_closed_in_event_data() {
    for data in [
        r#"{"future":{"$serde_json::private::RawValue":"false"}}"#,
        r#"{"future":{"$serde_json::private::RawValue":"{"}}"#,
        r#"{"future":{"$serde_json::private::Number":"0"}}"#,
        r#"{"future":[{"$serde_json::private::RawValue":"null"}]}"#,
        r#"{"future":{"\u0024serde_json::private::RawValue":"false"}}"#,
        r#"{"future":{"$serde_json\u003a\u003aprivate::Number":"0"}}"#,
        r#"{"future":{"ordinary":1,"$serde_json::private::RawValue":"false"}}"#,
        r#"{"future":{"$serde_json::private::RawValue":"false"},"future":null}"#,
    ] {
        let bytes = format!("data: {data}\n\n");
        let mut stream = from_bytes(bytes, TerminalPolicy::Eof).await;
        let event = stream.next().await.unwrap().unwrap();
        let error = event.json::<Value>().unwrap_err();
        assert_eq!(error.kind(), ErrorKind::Decode);
        assert_eq!(error.status(), Some(200));
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
    }
}

#[tokio::test]
async fn reserved_number_objects_cannot_hide_a_truthy_error_envelope() {
    for data in [
        r#"{"error":{"$serde_json::private::Number":"0"}}"#,
        r#"{"error":{"$serde_json::private::Number":"-0"}}"#,
        r#"{"err\u006fr":{"$serde_json::private::Number":"0"}}"#,
    ] {
        let mut stream = from_bytes(
            Bytes::from(format!("data: {data}\n\n")),
            TerminalPolicy::Eof,
        )
        .await;
        let event = stream.next().await.unwrap().unwrap();
        assert_eq!(event.json::<Value>().unwrap_err().kind(), ErrorKind::Decode);
    }
}

#[tokio::test]
async fn raw_error_envelope_uses_last_duplicate_key_and_decoded_strings() {
    for (data, truthy) in [
        (r#"{"error":{"ordinary":0},"error":false}"#, false),
        (r#"{"error":false,"error":{"ordinary":0}}"#, true),
        (r#"{"error":""}"#, false),
        (r#"{"error":"\u0000"}"#, true),
        (r#"{"error":null}"#, false),
        (r#"{"error":[]}"#, true),
        (r#"{"error":{}}"#, true),
    ] {
        let mut stream = from_bytes(
            Bytes::from(format!("data: {data}\n\n")),
            TerminalPolicy::Eof,
        )
        .await;
        let event = stream.next().await.unwrap().unwrap();
        let result = event.json::<Value>();
        if truthy {
            assert_eq!(result.unwrap_err().kind(), ErrorKind::Protocol);
        } else {
            assert!(result.is_ok());
        }
    }
}

#[tokio::test]
async fn empty_chunk_flood_is_rejected_and_explicit_close_is_idempotent() {
    let fixture = response_fixture(
        vec![Ok(Bytes::new()); 129],
        Some("text/event-stream"),
        false,
        false,
        Limits::default(),
    )
    .await;
    let mut stream = SseStream::new(fixture.response, 128, TerminalPolicy::Eof).unwrap();
    assert_eq!(stream.next().await.unwrap_err().kind(), ErrorKind::Protocol);
    assert_eq!(fixture.permits.available_permits(), 1);
    let fixture = response_fixture(
        vec![Ok(Bytes::from_static(b"data: unread\n\n"))],
        Some("text/event-stream"),
        true,
        false,
        Limits::default(),
    )
    .await;
    let mut stream = SseStream::new(fixture.response, 128, TerminalPolicy::Eof).unwrap();
    stream.close();
    stream.close();
    assert!(stream.next().await.unwrap().is_none());
    assert_eq!(fixture.polls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.drops.load(Ordering::SeqCst), 1);
}

#[tokio::test]
async fn typed_decode_failure_and_duplicate_media_headers_are_sanitized() {
    #[derive(serde::Deserialize)]
    struct TypedEvent {
        count: u32,
    }
    let mut stream = from_bytes(
        Bytes::from_static(b"data: {\"count\":\"synthetic-typed-secret\"}\n\n"),
        TerminalPolicy::Eof,
    )
    .await;
    let event = stream.next().await.unwrap().unwrap();
    let error = match event.json::<TypedEvent>() {
        Ok(value) => panic!("wrong scalar accepted as {}", value.count),
        Err(error) => error,
    };
    assert_eq!(error.kind(), ErrorKind::Decode);
    assert!(!format!("{error:?} {error}").contains("synthetic-typed-secret"));

    let mut fixture = response_fixture(
        vec![],
        Some("text/event-stream"),
        false,
        false,
        Limits::default(),
    )
    .await;
    fixture.response.headers.append(
        http::header::CONTENT_TYPE,
        HeaderValue::from_static("application/json"),
    );
    assert_eq!(
        SseStream::new(fixture.response, 128, TerminalPolicy::Eof)
            .unwrap_err()
            .kind(),
        ErrorKind::Protocol
    );
    assert_eq!(fixture.polls.load(Ordering::SeqCst), 0);
    assert_eq!(fixture.permits.available_permits(), 1);
}

#[tokio::test]
async fn retry_advice_ignores_invalid_numbers_and_preserves_valid_zero() {
    let input = concat!(
        "retry: 7\ndata: first\n\nretry: -1\nretry: 1.5\nretry:  4\n",
        "retry: 18446744073709551616\ndata: second\n\nretry: 000\ndata: third\n\n",
    )
    .as_bytes();
    let mut stream = from_bytes(Bytes::from_static(input), TerminalPolicy::Eof).await;
    assert_eq!(
        stream.next().await.unwrap().unwrap().retry(),
        Some(Duration::from_millis(7))
    );
    assert_eq!(
        stream.next().await.unwrap().unwrap().retry(),
        Some(Duration::from_millis(7))
    );
    assert_eq!(
        stream.next().await.unwrap().unwrap().retry(),
        Some(Duration::ZERO)
    );
    assert!(stream.next().await.unwrap().is_none());
}

#[cfg(feature = "reqwest-transport")]
#[tokio::test]
async fn default_http_adapter_decodes_sse_and_terminal_releases_socket_and_permit() {
    use crate::{Client, Endpoint, RawRequest, auth::Authentication};
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::{TcpListener, TcpStream},
    };
    async fn headers(stream: &mut TcpStream) -> String {
        let mut bytes = Vec::new();
        loop {
            let mut block = [0u8; 1024];
            let count = stream.read(&mut block).await.unwrap();
            assert!(count > 0);
            bytes.extend_from_slice(&block[..count]);
            assert!(bytes.len() < 8192);
            if bytes.windows(4).any(|value| value == b"\r\n\r\n") {
                return String::from_utf8(bytes).unwrap();
            }
        }
    }
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let endpoint =
        Endpoint::loopback(&format!("http://{}", listener.local_addr().unwrap())).unwrap();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let request = headers(&mut stream).await;
        assert!(request.starts_with("POST /events HTTP/1.1\r\n"));
        assert!(request.contains("x-portkey-api-key: synthetic-stream-key\r\n"));
        stream
            .write_all(
                concat!(
                    "HTTP/1.1 200 OK\r\nContent-Type: text/event-stream; charset=utf-8\r\n",
                    "Transfer-Encoding: chunked\r\n\r\n",
                )
                .as_bytes(),
            )
            .await
            .unwrap();
        let frame = "\u{feff}data: {\"text\":\"café 👋\"}\r\n\r\ndata: [DONE]\r\n\r\n".as_bytes();
        for chunk in frame.chunks(3) {
            stream
                .write_all(format!("{:x}\r\n", chunk.len()).as_bytes())
                .await
                .unwrap();
            stream.write_all(chunk).await.unwrap();
            stream.write_all(b"\r\n").await.unwrap();
        }
        // HTTP is deliberately incomplete: the SSE terminal must release the
        // connection without waiting for an HTTP EOF or an extra data event.
        let mut byte = [0u8; 1];
        assert_eq!(
            tokio::time::timeout(Duration::from_secs(1), stream.read(&mut byte))
                .await
                .unwrap()
                .unwrap(),
            0
        );
        let (mut followup, _) = listener.accept().await.unwrap();
        assert!(
            headers(&mut followup)
                .await
                .starts_with("GET /followup HTTP/1.1\r\n")
        );
        followup
            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 2\r\nConnection: close\r\n\r\nok")
            .await
            .unwrap();
    });
    let client = Client::builder(endpoint)
        .authentication(
            Authentication::runtime_key(crate::Secret::new("synthetic-stream-key").unwrap())
                .unwrap(),
        )
        .limits(Limits {
            max_concurrency: 1,
            ..Limits::default()
        })
        .build()
        .unwrap();
    tokio::time::timeout(Duration::from_secs(2), async {
        let response = client
            .send_stream(RawRequest::new(http::Method::POST, "events"), None)
            .await
            .unwrap();
        let mut stream = SseStream::new(response, 512, TerminalPolicy::DoneMarker).unwrap();
        assert_eq!(
            stream
                .next()
                .await
                .unwrap()
                .unwrap()
                .json::<Value>()
                .unwrap()["text"],
            "café 👋"
        );
        assert!(stream.next().await.unwrap().is_none());
        assert_eq!(
            client
                .send_bytes(RawRequest::new(http::Method::GET, "followup"), None)
                .await
                .unwrap(),
            b"ok".as_slice()
        );
    })
    .await
    .unwrap();
    server.await.unwrap();
}
