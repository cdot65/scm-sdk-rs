#![cfg(feature = "ws-transport")]
use base64::{Engine as _, engine::general_purpose::STANDARD};
use futures_util::future::BoxFuture;
use scm_sdk::{
    Client, Endpoint, ErrorKind, RemoteOutcome, RequestOptions, Secret,
    airs::gateway::{
        RuntimeClient,
        models::{GatewayRealtimeConnectRequest, GatewayRealtimeEvent},
    },
    auth::Authentication,
    realtime::{GatewayRealtimeOptions, NativeWebSocketConnector},
    transport::{Transport, TransportRequest, TransportResponse},
};
use serde_json::json;
use std::{sync::Arc, time::Duration};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
};
struct NoHttp;
impl Transport for NoHttp {
    fn send<'a>(
        &'a self,
        _: TransportRequest,
    ) -> BoxFuture<'a, scm_sdk::Result<TransportResponse>> {
        Box::pin(async { panic!("realtime must not fall back to HTTP") })
    }
}
fn runtime(port: u16) -> RuntimeClient {
    RuntimeClient::new(
        Client::builder(Endpoint::loopback(&format!("http://127.0.0.1:{port}/v1")).unwrap())
            .authentication(
                Authentication::runtime_key(Secret::new("synthetic-native-key").unwrap()).unwrap(),
            )
            .transport(Arc::new(NoHttp))
            .build()
            .unwrap(),
    )
    .unwrap()
}
fn query() -> GatewayRealtimeConnectRequest {
    serde_json::from_value(json!({"model":"@provider/unchanged"})).unwrap()
}
fn event() -> GatewayRealtimeEvent {
    serde_json::from_value(json!({"type":"response.create","future":true})).unwrap()
}
fn settings() -> GatewayRealtimeOptions {
    GatewayRealtimeOptions::new(Arc::new(NativeWebSocketConnector::new().unwrap()))
}
async fn request(stream: &mut TcpStream) -> Vec<u8> {
    let mut bytes = Vec::new();
    while !bytes.ends_with(b"\r\n\r\n") {
        assert!(bytes.len() < 64 * 1024);
        bytes.push(stream.read_u8().await.unwrap());
    }
    bytes
}
fn handshake(request: &[u8]) -> Vec<u8> {
    let mut headers = [httparse::EMPTY_HEADER; 32];
    let mut parsed = httparse::Request::new(&mut headers);
    assert!(parsed.parse(request).unwrap().is_complete());
    assert_eq!(parsed.method, Some("GET"));
    assert_eq!(
        parsed.path,
        Some("/v1/realtime?model=%40provider%2Funchanged")
    );
    let find = |name: &str| {
        parsed
            .headers
            .iter()
            .find(|entry| entry.name.eq_ignore_ascii_case(name))
            .unwrap()
            .value
    };
    assert_eq!(find("x-portkey-api-key"), b"synthetic-native-key");
    assert_eq!(find("sec-websocket-version"), b"13");
    assert!(!parsed.headers.iter().any(|entry| {
        [
            "authorization",
            "x-tsg-id",
            "sec-websocket-extensions",
            "sec-websocket-protocol",
        ]
        .iter()
        .any(|name| entry.name.eq_ignore_ascii_case(name))
    }));
    let mut input = find("sec-websocket-key").to_vec();
    input.extend_from_slice(b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11");
    let accept = STANDARD.encode(
        aws_lc_rs::digest::digest(&aws_lc_rs::digest::SHA1_FOR_LEGACY_USE_ONLY, &input).as_ref(),
    );
    format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {accept}\r\nx-request-id: safe-native-123\r\n\r\n").into_bytes()
}
fn frame(opcode: u8, fin: bool, payload: &[u8]) -> Vec<u8> {
    let mut bytes = vec![opcode | if fin { 0x80 } else { 0 }];
    match payload.len() {
        0..=125 => bytes.push(payload.len() as u8),
        126..=65535 => {
            bytes.push(126);
            bytes.extend_from_slice(&(payload.len() as u16).to_be_bytes());
        }
        _ => {
            bytes.push(127);
            bytes.extend_from_slice(&(payload.len() as u64).to_be_bytes());
        }
    }
    bytes.extend_from_slice(payload);
    bytes
}
async fn client_frame(stream: &mut TcpStream) -> (u8, Vec<u8>) {
    let opcode = stream.read_u8().await.unwrap();
    assert_eq!(opcode & 0x80, 0x80);
    let length = stream.read_u8().await.unwrap();
    assert_ne!(length & 0x80, 0, "client frames must be masked");
    let length = match length & 0x7f {
        126 => stream.read_u16().await.unwrap() as usize,
        127 => usize::try_from(stream.read_u64().await.unwrap()).unwrap(),
        value => value as usize,
    };
    assert!(length <= 1024 * 1024);
    let mut mask = [0u8; 4];
    stream.read_exact(&mut mask).await.unwrap();
    let mut bytes = vec![0u8; length];
    stream.read_exact(&mut bytes).await.unwrap();
    for (index, byte) in bytes.iter_mut().enumerate() {
        *byte ^= mask[index % 4];
    }
    (opcode & 0xf, bytes)
}

#[tokio::test]
async fn native_socket_pins_masking_ping_pong_fragmented_unicode_tail_and_clean_close() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let request = request(&mut stream).await;
        stream.write_all(&handshake(&request)).await.unwrap();
        let (opcode, bytes) = client_frame(&mut stream).await;
        assert_eq!(opcode, 1);
        assert_eq!(
            serde_json::from_slice::<serde_json::Value>(&bytes).unwrap(),
            serde_json::to_value(event()).unwrap()
        );
        let json = serde_json::to_vec(&json!({"type":"session.created","unknown":"é"})).unwrap();
        let split = json.iter().position(|value| *value == 0xc3).unwrap() + 1;
        let mut wire = frame(9, true, b"probe");
        wire.extend(frame(1, false, &json[..split]));
        wire.extend(frame(0, true, &json[split..]));
        stream.write_all(&wire).await.unwrap();
        let (opcode, pong) = client_frame(&mut stream).await;
        assert_eq!(opcode, 10);
        assert_eq!(pong, b"probe");
        stream
            .write_all(&frame(8, true, &1000u16.to_be_bytes()))
            .await
            .unwrap();
        let (opcode, close) = client_frame(&mut stream).await;
        assert_eq!(opcode, 8);
        assert_eq!(close, 1000u16.to_be_bytes());
    });
    let options = RequestOptions::with_timeout(Duration::from_secs(2)).unwrap();
    let mut connection = runtime(port)
        .connect_realtime(&query(), &settings(), Some(&options))
        .await
        .unwrap();
    connection.send(&event()).await.unwrap();
    assert_eq!(
        serde_json::to_value(connection.next().await.unwrap().unwrap()).unwrap(),
        json!({"type":"session.created","unknown":"é"})
    );
    assert!(connection.next().await.unwrap().is_none());
    tokio::time::timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn native_upgrade_preserves_already_received_frame_bytes() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let req = request(&mut stream).await;
        let mut bytes = handshake(&req);
        bytes.extend(frame(1, true, br#"{"type":"session.created","tail":true}"#));
        stream.write_all(&bytes).await.unwrap();
        let mut discard = [0u8; 1];
        let _ = stream.read(&mut discard).await;
    });
    let mut connection = runtime(port)
        .connect_realtime(
            &query(),
            &settings(),
            Some(&RequestOptions::with_timeout(Duration::from_secs(2)).unwrap()),
        )
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(connection.next().await.unwrap().unwrap()).unwrap()["tail"],
        true
    );
    drop(connection);
    server.await.unwrap();
}

#[tokio::test]
async fn native_upgrade_rejects_redirects_bad_accepts_and_oversized_headers_without_replay() {
    for kind in 0..4 {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let req = request(&mut stream).await;
            let bytes=match kind {
                0=>b"HTTP/1.1 302 Found\r\nLocation: https://unadmitted.invalid/secret\r\nx-request-id: safe-redirect-123\r\n\r\n".to_vec(),
                1=>format!("HTTP/1.1 101 Switching Protocols\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Accept: {}\r\n\r\n","A".repeat(1000)).into_bytes(),
                2=>{let good=handshake(&req);String::from_utf8(good).unwrap().replace("Connection: Upgrade\r\n","").into_bytes()},
                _=>format!("HTTP/1.1 101 Switching Protocols\r\nX-Fill: {}\r\n\r\n","x".repeat(64*1024)).into_bytes(),
            };
            let _ = stream.write_all(&bytes).await;
            let mut byte = [0u8; 1];
            let _ = stream.read(&mut byte).await;
        });
        let options = RequestOptions::with_timeout(Duration::from_secs(2))
            .unwrap()
            .with_retries(5)
            .unwrap();
        let error = runtime(port)
            .connect_realtime(&query(), &settings(), Some(&options))
            .await
            .unwrap_err();
        assert_eq!(
            error.kind(),
            match kind {
                0 => ErrorKind::Destination,
                3 => ErrorKind::Limit,
                _ => ErrorKind::Protocol,
            }
        );
        assert_eq!(
            error.remote_outcome(),
            if kind == 0 {
                RemoteOutcome::Rejected
            } else {
                RemoteOutcome::Unknown
            }
        );
        assert!(!format!("{error:?} {error}").contains("secret"));
        tokio::time::timeout(Duration::from_secs(2), server)
            .await
            .unwrap()
            .unwrap();
    }
}

#[tokio::test]
async fn native_socket_bounds_assembled_fragments_and_rejects_invalid_utf8_and_binary() {
    for kind in 0..3 {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let req = request(&mut stream).await;
            let mut bytes = handshake(&req);
            match kind {
                0 => {
                    bytes.extend(frame(1, false, &[b'a'; 60]));
                    bytes.extend(frame(0, true, &[b'b'; 10]));
                }
                1 => bytes.extend(frame(1, true, &[0xff])),
                _ => bytes.extend(frame(2, true, b"synthetic binary")),
            };
            stream.write_all(&bytes).await.unwrap();
            let mut discard = [0u8; 1];
            let _ = stream.read(&mut discard).await;
        });
        let mut settings = settings();
        settings.max_event_bytes = 64;
        let mut connection = runtime(port)
            .connect_realtime(
                &query(),
                &settings,
                Some(&RequestOptions::with_timeout(Duration::from_secs(2)).unwrap()),
            )
            .await
            .unwrap();
        let error = connection.next().await.unwrap_err();
        assert_eq!(error.status(), Some(101));
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        assert_eq!(
            error.kind(),
            if kind == 0 {
                ErrorKind::Limit
            } else {
                ErrorKind::Protocol
            }
        );
        tokio::time::timeout(Duration::from_secs(2), server)
            .await
            .unwrap()
            .unwrap();
    }
}

#[tokio::test]
async fn native_ping_flood_is_flushed_before_more_reads_and_does_not_queue_events() {
    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let port = listener.local_addr().unwrap().port();
    let server = tokio::spawn(async move {
        let (mut stream, _) = listener.accept().await.unwrap();
        let req = request(&mut stream).await;
        let mut bytes = handshake(&req);
        for index in 0u8..128 {
            bytes.extend(frame(9, true, &[index]));
        }
        bytes.extend(frame(1, true, br#"{"type":"session.created"}"#));
        stream.write_all(&bytes).await.unwrap();
        for index in 0u8..128 {
            let (opcode, payload) = client_frame(&mut stream).await;
            assert_eq!(opcode, 10);
            assert_eq!(payload, vec![index]);
        }
        let mut discard = [0u8; 1];
        let _ = stream.read(&mut discard).await;
    });
    let mut connection = runtime(port)
        .connect_realtime(
            &query(),
            &settings(),
            Some(&RequestOptions::with_timeout(Duration::from_secs(2)).unwrap()),
        )
        .await
        .unwrap();
    assert_eq!(
        serde_json::to_value(connection.next().await.unwrap().unwrap()).unwrap()["type"],
        "session.created"
    );
    drop(connection);
    tokio::time::timeout(Duration::from_secs(2), server)
        .await
        .unwrap()
        .unwrap();
}

#[tokio::test]
async fn native_eof_and_unconfirmed_close_never_become_success() {
    for eof in [true, false] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let port = listener.local_addr().unwrap().port();
        let server = tokio::spawn(async move {
            let (mut stream, _) = listener.accept().await.unwrap();
            let req = request(&mut stream).await;
            stream.write_all(&handshake(&req)).await.unwrap();
            if eof {
                return;
            }
            let (opcode, _) = client_frame(&mut stream).await;
            assert_eq!(opcode, 8);
            let mut discard = [0u8; 1];
            let _ = stream.read(&mut discard).await;
        });
        let mut settings = settings();
        settings.close_timeout = Duration::from_millis(10);
        let mut connection = runtime(port)
            .connect_realtime(
                &query(),
                &settings,
                Some(&RequestOptions::with_timeout(Duration::from_secs(2)).unwrap()),
            )
            .await
            .unwrap();
        let error = if eof {
            connection.next().await.unwrap_err()
        } else {
            connection.cancel().await.unwrap_err()
        };
        assert_eq!(error.status(), Some(101));
        assert_eq!(error.remote_outcome(), RemoteOutcome::Unknown);
        tokio::time::timeout(Duration::from_secs(2), server)
            .await
            .unwrap()
            .unwrap();
    }
}
