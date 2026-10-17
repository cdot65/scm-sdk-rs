use super::from_value;
use crate::data::{FiniteNumber, Nullable, Optional, Patch};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::collections::BTreeMap;

#[test]
fn literal_reserved_protocol_keys_are_rejected_before_wire_value_coercion() {
    for key in [
        "$serde_json::private::Number",
        "$serde_json::private::RawValue",
    ] {
        for encoded in [
            key.to_string(),
            key.bytes().map(|b| format!("\\u{b:04x}")).collect(),
        ] {
            for raw in [
                format!(r#"{{"{encoded}":"false"}}"#),
                format!(r#"{{"ordinary":1,"{encoded}":"0"}}"#),
                format!(r#"{{"nested":[{{"{encoded}":"0"}}]}}"#),
                format!(r#"{{"{encoded}":"0","{encoded}":"1"}}"#),
            ] {
                assert!(super::from_str(&raw).is_err());
                assert!(super::from_slice(raw.as_bytes()).is_err());
            }
        }
        // Check every individual ASCII character's escaped spelling as well.
        for i in 0..key.len() {
            let encoded = format!("{}\\u{:04x}{}", &key[..i], key.as_bytes()[i], &key[i + 1..]);
            assert!(super::from_str(&format!(r#"{{"{encoded}":null}}"#)).is_err());
        }
    }
    // This is the concrete DEFAULT raw_value feature bug the guard prevents.
    let raw = r#"{"$serde_json::private::RawValue":"false"}"#;
    assert_eq!(
        serde_json::from_str::<Value>(raw).unwrap(),
        Value::Bool(false)
    );
    assert!(super::from_str(raw).is_err());
}

#[test]
fn harmless_keys_marker_values_and_json_escapes_remain_accepted() {
    for raw in [
        r#"{"value":"$serde_json::private::RawValue","other":"$serde_json::private::Number"}"#,
        r#"{"$serde_json::private::RawValueX":false,"X$serde_json::private::Number":0}"#,
        r#"{"escaped":"quote\" backslash\\ brace{ colon: end","nested":[null,true,2]}"#,
        r#"{"unicode":"\u00e9\ud83d\ude42","duplicate":1,"duplicate":2}"#,
        r#"["$serde_json::private::RawValue",1,-9223372036854776000,18446744073709552000]"#,
    ] {
        assert_eq!(
            super::from_str(raw).unwrap(),
            serde_json::from_str::<Value>(raw).unwrap()
        );
    }
    for invalid in [
        "",
        "[",
        "{",
        "\"unfinished",
        "[1,]",
        "{\"a\":}",
        "true false",
    ] {
        assert!(super::from_str(invalid).is_err());
    }
}

#[test]
fn programmatic_literal_protocol_objects_and_overdeep_values_fail_safely() {
    for key in [
        "$serde_json::private::RawValue",
        "$serde_json::private::Number",
    ] {
        let literal = json!({"known":"a", "future":[{key:"false"}]});
        assert!(from_value::<Extended>(literal.clone()).is_err());
        assert!(from_value::<Value>(literal).is_err());
    }
    let mut nested = Value::Null;
    for _ in 0..10_000 {
        nested = Value::Array(vec![nested]);
    }
    assert!(from_value::<Value>(nested).is_err());
    let too_deep = format!("{}0{}", "[".repeat(129), "]".repeat(129));
    assert!(super::from_str(&too_deep).is_err());
    let too_many_nodes = format!("[{}0]", "0,".repeat(super::MAX_NODES));
    assert!(super::from_str(&too_many_nodes).is_err());
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
struct Extended {
    known: String,
    #[serde(flatten)]
    extensions: BTreeMap<String, Value>,
}

#[derive(Debug, PartialEq, Serialize, Deserialize)]
#[serde(untagged)]
enum NumberOrOther {
    Known { number: Patch<FiniteNumber> },
    Other(Value),
}

#[test]
fn wide_numbers_preserve_untagged_variant_and_opaque_nested_values() {
    // This is the frozen prompt seed boundary plus a value above u64::MAX.
    // Additional tokens cover the full arbitrary-precision JSON domain.
    for token in [
        "-9223372036854776000",
        "18446744073709552000",
        "340282366920938463463374607431768211456",
        "0.123456789012345678901234567890",
        "-0",
        "-0.0",
    ] {
        let value: Value =
            serde_json::from_str(&format!(r#"{{"number":{token}}}"#)).expect("finite wire number");
        let decoded: NumberOrOther = from_value(value.clone()).unwrap();
        assert!(matches!(decoded, NumberOrOther::Known { .. }));
        assert_eq!(serde_json::to_value(decoded).unwrap(), value);

        let wire =
            format!(r#"{{"known":"visible","future":{token},"nested":[{{"value":{token}}}]}}"#);
        let value: Value = serde_json::from_str(&wire).unwrap();
        let decoded: Extended = from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), value);
    }
}

#[test]
fn opaque_overflow_numbers_remain_exact_when_feature_unification_permits_them() {
    // Without arbitrary_precision the upstream parser rejects overflow, as it
    // should. With it, opaque JSON retains the exact token; FiniteNumber rejects it.
    if let Ok(value) = serde_json::from_str::<Value>(r#"{"future":1e400}"#) {
        let extended = json!({"known":"visible", "data":value});
        let decoded: Extended = from_value(extended.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), extended);
        assert!(from_value::<FiniteNumber>(value["future"].clone()).is_err());
    }
}

#[test]
fn presence_wrappers_reenter_compatible_decoding_from_raw_json() {
    #[derive(Deserialize, Serialize)]
    struct Fields {
        optional: Optional<Extended>,
        nullable: Nullable<Extended>,
        patch: Patch<Extended>,
    }
    let raw = concat!(
        r#"{"optional":{"known":"a","future":-9223372036854776000},"#,
        r#""nullable":{"known":"b","future":18446744073709552000},"#,
        r#""patch":{"known":"c","future":[-9223372036854776000]}}"#,
    );
    let decoded: Fields = serde_json::from_str(raw).unwrap();
    assert_eq!(
        serde_json::to_value(decoded).unwrap(),
        serde_json::from_str::<Value>(raw).unwrap()
    );
    assert!(from_value::<Optional<String>>(Value::Null).is_err());
    assert!(matches!(
        from_value::<Nullable<String>>(Value::Null).unwrap(),
        Nullable::Null
    ));
    assert!(matches!(
        from_value::<Patch<String>>(Value::Null).unwrap(),
        Patch::Null
    ));
}

#[test]
fn ordinary_newtypes_and_external_enum_payloads_use_recursive_adapter() {
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Newtype(Extended);
    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    enum Tagged {
        Unit,
        Newtype(Newtype),
        Tuple(Extended, Value),
        Struct { payload: Extended },
    }
    let node: Value =
        serde_json::from_str(r#"{"known":"a","future":-9223372036854776000}"#).unwrap();
    for value in [
        json!("Unit"),
        json!({"Newtype":node.clone()}),
        json!({"Tuple":[node.clone(),node.clone()]}),
        json!({"Struct":{"payload":node}}),
    ] {
        let decoded: Tagged = from_value(value.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), value);
    }
    assert!(from_value::<Tagged>(json!({"Missing":null})).is_err());
    assert!(from_value::<Tagged>(json!({"Unit":1})).is_err());
    assert!(from_value::<Tagged>(json!({"Tuple":[{"known":"a"}]})).is_err());
}

#[test]
fn json_numeric_boolean_and_enum_map_key_rules_are_preserved() {
    for input in [json!({"1":"one","-2":"two"}), json!({"0":"zero"})] {
        let decoded: BTreeMap<i64, String> = from_value(input.clone()).unwrap();
        assert_eq!(serde_json::to_value(decoded).unwrap(), input);
    }
    for key in ["01", "+1", " 1", "1 ", "1x", "null"] {
        assert!(from_value::<BTreeMap<i64, String>>(json!({key:"invalid"})).is_err());
    }
    let decoded: BTreeMap<bool, u8> = from_value(json!({"false":0,"true":1})).unwrap();
    assert_eq!(decoded, BTreeMap::from([(false, 0), (true, 1)]));
    #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Deserialize)]
    enum Key {
        Alpha,
    }
    assert_eq!(
        from_value::<BTreeMap<Key, u8>>(json!({"Alpha":1})).unwrap(),
        BTreeMap::from([(Key::Alpha, 1)])
    );
}

#[test]
fn ordinary_strings_move_without_a_wire_roundtrip() {
    let string = String::from("ordinary owned value");
    let pointer = string.as_ptr();
    let decoded: String = from_value(Value::String(string)).unwrap();
    assert_eq!(decoded.as_ptr(), pointer);
    assert!(from_value::<Vec<u8>>(json!([1, "wrong"])).is_err());
    assert!(from_value::<(u8, u8)>(json!([1, 2, 3])).is_err());
}

#[test]
fn ordinary_floats_preserve_bits_without_a_number_wire_roundtrip() {
    // The default JSON reader need not have float_roundtrip enabled. Already
    // materialized f64 values must keep their bits through ordinary decoding.
    for bits in [
        1,
        0x0010_0000_0000_0000,
        0x3fef_ffff_ffff_ffff,
        0x3ff0_0000_0000_0001,
        0x4009_21fb_5444_2d18,
        0x7fef_ffff_ffff_ffff,
        0x8000_0000_0000_0000,
    ] {
        let number = f64::from_bits(bits);
        let input = Value::Number(serde_json::Number::from_f64(number).unwrap());
        let decoded: Value = from_value(input.clone()).unwrap();
        assert_eq!(decoded, input);
        assert_eq!(decoded.as_f64().unwrap().to_bits(), number.to_bits());
    }
}

#[test]
fn raw_value_newtype_uses_public_json_deserializer_protocol() {
    let value = json!({"known":"a","future":42});
    let raw: Box<serde_json::value::RawValue> = from_value(value.clone()).unwrap();
    assert_eq!(serde_json::from_str::<Value>(raw.get()).unwrap(), value);
}

#[cfg(feature = "models")]
#[test]
fn intersection_reenters_compatible_decoding_for_both_sides() {
    use crate::models::gateway::Intersection;
    let value: Value = serde_json::from_str(
        r#"{"known":"a","future":-9223372036854776000,"nested":{"number":18446744073709552000}}"#,
    )
    .unwrap();
    let decoded: Intersection<Extended, Extended> = from_value(value.clone()).unwrap();
    assert_eq!(serde_json::to_value(decoded).unwrap(), value);
}

#[cfg(feature = "gateway")]
mod boundaries {
    use super::*;
    use crate::{
        Client, Endpoint, ErrorKind, RawRequest, RemoteOutcome, RequestOptions,
        streaming::TerminalPolicy,
        transport::{Transport, TransportRequest, TransportResponse},
        validation::ValidationLimits,
    };
    use bytes::Bytes;
    use futures_util::future::BoxFuture;
    use http::{HeaderMap, HeaderValue, Method, StatusCode};
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };

    struct Wire {
        body: Bytes,
        sse: bool,
        calls: AtomicUsize,
    }
    impl Transport for Wire {
        fn send<'a>(
            &'a self,
            _: TransportRequest,
        ) -> BoxFuture<'a, crate::Result<TransportResponse>> {
            Box::pin(async move {
                assert_eq!(
                    self.calls.fetch_add(1, Ordering::SeqCst),
                    0,
                    "unexpected replay"
                );
                let mut headers = HeaderMap::new();
                headers.insert(
                    "x-request-id",
                    HeaderValue::from_static("boundary-correlation"),
                );
                if self.sse {
                    headers.insert(
                        "content-type",
                        HeaderValue::from_static("text/event-stream"),
                    );
                }
                Ok(TransportResponse::from_bytes(
                    StatusCode::OK,
                    headers,
                    self.body.clone(),
                ))
            })
        }
    }
    fn setup(body: &str, sse: bool, limits: Option<ValidationLimits>) -> (Client, Arc<Wire>) {
        let wire = Arc::new(Wire {
            body: Bytes::copy_from_slice(body.as_bytes()),
            sse,
            calls: AtomicUsize::new(0),
        });
        let mut builder = Client::builder(Endpoint::parse("https://numbers.example").unwrap())
            .transport(wire.clone());
        if let Some(limits) = limits {
            builder = builder.validation_limits(limits);
        }
        (builder.build().unwrap(), wire)
    }
    fn request() -> RawRequest {
        RawRequest::new(Method::POST, "/number-boundary").named("test.number_boundary")
    }
    const SCHEMA: &str = "GatewayRealtimeEventSchema";
    const VALID: &str = r#"{"type":"delta","known":"a","future":-9223372036854776000,"nested":[18446744073709552000]}"#;

    fn metadata(error: &crate::Error, kind: ErrorKind, wire: &Wire) {
        assert_eq!(error.kind(), kind);
        assert_eq!(error.status(), Some(200));
        assert_eq!(error.request_id(), Some("boundary-correlation"));
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        assert_eq!(wire.calls.load(Ordering::SeqCst), 1);
    }

    #[tokio::test]
    async fn typed_http_preserves_numbers_and_decode_failure_never_replays() {
        let (client, wire) = setup(VALID, false, None);
        let decoded: Extended = client.send_contract(request(), SCHEMA, None).await.unwrap();
        assert_eq!(
            serde_json::to_value(decoded).unwrap(),
            serde_json::from_str::<Value>(VALID).unwrap()
        );
        assert_eq!(wire.calls.load(Ordering::SeqCst), 1);

        // Schema accepts this unknown field, but the caller's declared type does
        // not. Successful remote mutation + local decode failure stays uncertain.
        let (client, wire) = setup(
            r#"{"type":"delta","known":false,"future":-9223372036854776000}"#,
            false,
            None,
        );
        let options = RequestOptions::with_timeout(std::time::Duration::from_secs(5))
            .unwrap()
            .with_retries(5)
            .unwrap();
        let error = client
            .send_contract::<Extended>(request(), SCHEMA, Some(&options))
            .await
            .unwrap_err();
        metadata(&error, ErrorKind::Decode, &wire);
        assert_eq!(error.operation(), Some("test.number_boundary"));
    }

    #[tokio::test]
    async fn typed_http_and_stream_keep_validation_budgets() {
        let limits = ValidationLimits {
            max_nodes: 2,
            ..ValidationLimits::default()
        };
        let (client, wire) = setup(VALID, false, Some(limits.clone()));
        let error = client
            .send_contract::<Extended>(request(), SCHEMA, None)
            .await
            .unwrap_err();
        metadata(&error, ErrorKind::Limit, &wire);

        let (client, wire) = setup(&format!("data: {VALID}\n\n"), true, Some(limits));
        let mut stream = client
            .send_contract_stream::<Extended>(request(), SCHEMA, TerminalPolicy::Eof, None)
            .await
            .unwrap();
        let error = stream.next().await.unwrap_err();
        metadata(&error, ErrorKind::Limit, &wire);
    }

    #[tokio::test]
    async fn typed_stream_preserves_numbers_and_latches_decode_failure() {
        let invalid = r#"{"type":"delta","known":false,"future":18446744073709552000}"#;
        let (client, wire) = setup(&format!("data: {VALID}\n\ndata: {invalid}\n\n"), true, None);
        let mut stream = client
            .send_contract_stream::<Extended>(request(), SCHEMA, TerminalPolicy::Eof, None)
            .await
            .unwrap();
        let event = stream.next().await.unwrap().unwrap();
        assert_eq!(
            serde_json::to_value(event).unwrap(),
            serde_json::from_str::<Value>(VALID).unwrap()
        );
        let error = stream.next().await.unwrap_err();
        metadata(&error, ErrorKind::Decode, &wire);
        let error = stream.next().await.unwrap_err();
        metadata(&error, ErrorKind::Decode, &wire);
    }

    #[tokio::test]
    async fn typed_stream_checks_cancellation_before_buffered_event_decode() {
        let (client, wire) = setup(&format!("data: {VALID}\n\ndata: {VALID}\n\n"), true, None);
        let mut stream = client
            .send_contract_stream::<Extended>(request(), SCHEMA, TerminalPolicy::Eof, None)
            .await
            .unwrap();
        stream.next().await.unwrap().unwrap();
        client.session().cancel();
        let error = stream.next().await.unwrap_err();
        metadata(&error, ErrorKind::Cancelled, &wire);
    }
}

#[cfg(feature = "realtime")]
mod realtime_boundary {
    use super::*;
    use crate::{
        Error, ErrorKind, RemoteOutcome, RequestOptions, Session,
        realtime::{
            GatewayRealtimeConnection, GatewayRealtimeOptions, WebSocketConnector, WebSocketFrame,
            WebSocketRequest, WebSocketUpgrade,
        },
        validation::ValidationLimits,
    };
    use bytes::Bytes;
    use futures_util::{Sink, Stream, future::BoxFuture};
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

    struct Socket(VecDeque<WebSocketFrame>, Arc<AtomicUsize>);
    impl Stream for Socket {
        type Item = crate::Result<WebSocketFrame>;
        fn poll_next(mut self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<Option<Self::Item>> {
            Poll::Ready(self.0.pop_front().map(Ok))
        }
    }
    impl Sink<WebSocketFrame> for Socket {
        type Error = Error;
        fn poll_ready(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<crate::Result<()>> {
            Poll::Ready(Ok(()))
        }
        fn start_send(self: Pin<&mut Self>, _: WebSocketFrame) -> crate::Result<()> {
            self.1.fetch_add(1, Ordering::SeqCst);
            Ok(())
        }
        fn poll_flush(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<crate::Result<()>> {
            Poll::Ready(Ok(()))
        }
        fn poll_close(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<crate::Result<()>> {
            Poll::Ready(Ok(()))
        }
    }
    struct NeverConnect;
    impl WebSocketConnector for NeverConnect {
        fn connect<'a>(
            &'a self,
            _: WebSocketRequest,
        ) -> BoxFuture<'a, crate::Result<WebSocketUpgrade>> {
            Box::pin(async { panic!("an established connection must not reconnect") })
        }
    }

    #[tokio::test]
    async fn realtime_events_keep_exact_numbers_and_latch_safe_decode_metadata() {
        let valid =
            r#"{"type":"delta","future":-9223372036854776000,"nested":[18446744073709552000]}"#;
        let socket = Socket(
            VecDeque::from([
                WebSocketFrame::Text(Bytes::from_static(valid.as_bytes())),
                WebSocketFrame::Text(Bytes::from_static(
                    b"{\"type\":\"\",\"secret\":\"never-echo\"}",
                )),
            ]),
            Arc::new(AtomicUsize::new(0)),
        );
        let mut headers = HeaderMap::new();
        headers.insert(
            "x-request-id",
            HeaderValue::from_static("number-correlation"),
        );
        let upgrade =
            WebSocketUpgrade::new(StatusCode::SWITCHING_PROTOCOLS, headers, Box::new(socket))
                .unwrap();
        let permit = Arc::new(tokio::sync::Semaphore::new(1))
            .acquire_owned()
            .await
            .unwrap();
        let mut connection = GatewayRealtimeConnection::new(
            upgrade,
            permit,
            RequestOptions::with_timeout(std::time::Duration::from_secs(5)).unwrap(),
            Session::new(),
            GatewayRealtimeOptions::new(Arc::new(NeverConnect)),
            ValidationLimits::default(),
        );
        let event = connection.next().await.unwrap().unwrap();
        assert_eq!(
            serde_json::to_value(event).unwrap(),
            serde_json::from_str::<Value>(valid).unwrap()
        );
        for _ in 0..2 {
            let error = connection.next().await.unwrap_err();
            assert_eq!(error.kind(), ErrorKind::Decode);
            assert_eq!(error.status(), Some(101));
            assert_eq!(error.request_id(), Some("number-correlation"));
            assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
            assert!(!format!("{error:?}").contains("never-echo"));
        }
    }

    #[tokio::test]
    async fn realtime_reserved_objects_fail_before_inbound_coercion_or_outbound_write() {
        use crate::{airs::gateway::models::GatewayRealtimeEvent, data::JsonValue};
        for key in [
            "$serde_json::private::RawValue",
            "$serde_json::private::Number",
        ] {
            for incoming in [true, false] {
                let writes = Arc::new(AtomicUsize::new(0));
                let raw = format!(r#"{{"type":"delta","future":{{"{key}":"false"}}}}"#);
                let socket = Socket(
                    VecDeque::from([WebSocketFrame::Text(Bytes::from(raw))]),
                    writes.clone(),
                );
                let upgrade = WebSocketUpgrade::new(
                    StatusCode::SWITCHING_PROTOCOLS,
                    HeaderMap::new(),
                    Box::new(socket),
                )
                .unwrap();
                let permit = Arc::new(tokio::sync::Semaphore::new(1))
                    .acquire_owned()
                    .await
                    .unwrap();
                let mut connection = GatewayRealtimeConnection::new(
                    upgrade,
                    permit,
                    RequestOptions::with_timeout(std::time::Duration::from_secs(5)).unwrap(),
                    Session::new(),
                    GatewayRealtimeOptions::new(Arc::new(NeverConnect)),
                    ValidationLimits::default(),
                );
                let error = if incoming {
                    connection.next().await.unwrap_err()
                } else {
                    let event = GatewayRealtimeEvent::builder()
                        .r#type("session.update".into())
                        .extension("future", JsonValue::new(json!({key:"false"})))
                        .build()
                        .unwrap();
                    connection.send(&event).await.unwrap_err()
                };
                assert_eq!(
                    error.kind(),
                    if incoming {
                        ErrorKind::Decode
                    } else {
                        ErrorKind::Validation
                    }
                );
                assert_eq!(error.status(), Some(101));
                assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
                assert_eq!(writes.load(Ordering::SeqCst), 0);
                assert!(!format!("{error:?}").contains(key));
            }
        }
    }
}
