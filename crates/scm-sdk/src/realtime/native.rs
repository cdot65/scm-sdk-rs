//! Explicit direct TCP/TLS connector. HTTP proxy/TLS configuration is never inherited.
//! The bounded upgrade is ours; framing uses the reviewed patched dependency.
use super::*;
use base64::{Engine as _, engine::general_purpose::STANDARD};
use bytes::Buf;
use http::{HeaderName, HeaderValue, header};
use rustls::{
    ClientConfig, RootCertStore,
    pki_types::{CertificateDer, ServerName, pem::PemObject},
};
use std::{
    io,
    panic::{AssertUnwindSafe, catch_unwind},
    task::ready,
};
use tokio::{
    io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt, ReadBuf},
    net::TcpStream,
};
use tokio_websockets::{ClientBuilder, CloseCode, Config, Limits, Message, WebSocketStream};
use zeroize::Zeroizing;

#[cfg(not(target_pointer_width = "64"))]
compile_error!(
    "The native WebSocket adapter currently supports 64-bit targets; use a caller connector on other targets"
);

const MAX_HEADERS: usize = 64 * 1024;
const GUID: &[u8] = b"258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

/// Default connector uses direct sockets and verified TLS with system trust roots.
/// It never changes global crypto providers or logging, and never follows redirects.
#[derive(Clone)]
pub struct NativeWebSocketConnector {
    tls: Arc<ClientConfig>,
}
impl NativeWebSocketConnector {
    pub fn new() -> Result<Self> {
        Self::with_additional_certificates(&[])
    }
    pub fn with_additional_certificates(certificates: &[&[u8]]) -> Result<Self> {
        if certificates.len() > 64
            || certificates
                .iter()
                .try_fold(0usize, |n, cert| n.checked_add(cert.len()))
                .is_none_or(|size| size > 1024 * 1024)
        {
            return Err(Error::new(ErrorKind::Limit));
        }
        let mut roots = RootCertStore::empty();
        roots.add_parsable_certificates(rustls_native_certs::load_native_certs().certs);
        for pem in certificates {
            let mut found = false;
            for certificate in CertificateDer::pem_slice_iter(pem) {
                found = true;
                roots
                    .add(certificate.map_err(|_| Error::new(ErrorKind::Configuration))?)
                    .map_err(|_| Error::new(ErrorKind::Configuration))?;
            }
            if !found {
                return Err(Error::new(ErrorKind::Configuration));
            }
        }
        if roots.is_empty() {
            return Err(Error::new(ErrorKind::Configuration));
        }
        let provider = rustls::crypto::aws_lc_rs::default_provider();
        let mut tls = ClientConfig::builder_with_provider(Arc::new(provider))
            .with_safe_default_protocol_versions()
            .map_err(|_| Error::new(ErrorKind::Configuration))?
            .with_root_certificates(roots)
            .with_no_client_auth();
        tls.alpn_protocols = vec![b"http/1.1".to_vec()];
        Ok(Self { tls: Arc::new(tls) })
    }
    /// Explicit trusted TLS integration. The caller owns custom verifier policy.
    pub fn with_tls_config(config: Arc<ClientConfig>) -> Self {
        Self { tls: config }
    }
}
impl fmt::Debug for NativeWebSocketConnector {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("NativeWebSocketConnector([REDACTED])")
    }
}
impl WebSocketConnector for NativeWebSocketConnector {
    fn connect<'a>(&'a self, request: WebSocketRequest) -> BoxFuture<'a, Result<WebSocketUpgrade>> {
        Box::pin(async move {
            let host = request
                .url
                .host_str()
                .ok_or_else(|| Error::new(ErrorKind::Destination))?
                .trim_start_matches('[')
                .trim_end_matches(']')
                .to_owned();
            let port = request
                .url
                .port_or_known_default()
                .ok_or_else(|| Error::new(ErrorKind::Destination))?;
            let tcp = TcpStream::connect((host.as_str(), port))
                .await
                .map_err(|_| network())?;
            tcp.set_nodelay(true).map_err(|_| network())?;
            let io: BoxIo = match request.url.scheme() {
                "wss" => {
                    let name = ServerName::try_from(host)
                        .map_err(|_| Error::new(ErrorKind::Destination))?;
                    let tls = tokio_rustls::TlsConnector::from(self.tls.clone())
                        .connect(name, tcp)
                        .await
                        .map_err(|_| network())?;
                    if tls
                        .get_ref()
                        .1
                        .alpn_protocol()
                        .is_some_and(|protocol| protocol != b"http/1.1")
                    {
                        return Err(Error::new(ErrorKind::Protocol));
                    }
                    Box::new(tls)
                }
                "ws" => Box::new(tcp),
                _ => return Err(Error::new(ErrorKind::Destination)),
            };
            upgrade(io, request).await
        })
    }
}
fn network() -> Error {
    Error::new(ErrorKind::Transport).with_failure_kind(crate::FailureKind::Network)
}
trait AsyncIo: AsyncRead + AsyncWrite + Send + Unpin {}
impl<T: AsyncRead + AsyncWrite + Send + Unpin> AsyncIo for T {}
type BoxIo = Box<dyn AsyncIo>;
struct PrefixedIo {
    prefix: Bytes,
    io: BoxIo,
}
impl AsyncRead for PrefixedIo {
    fn poll_read(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        buf: &mut ReadBuf<'_>,
    ) -> Poll<io::Result<()>> {
        if !self.prefix.is_empty() && buf.remaining() > 0 {
            let count = buf.remaining().min(self.prefix.len());
            buf.put_slice(&self.prefix[..count]);
            self.prefix.advance(count);
            Poll::Ready(Ok(()))
        } else {
            Pin::new(&mut self.io).poll_read(cx, buf)
        }
    }
}
impl AsyncWrite for PrefixedIo {
    fn poll_write(
        mut self: Pin<&mut Self>,
        cx: &mut Context<'_>,
        bytes: &[u8],
    ) -> Poll<io::Result<usize>> {
        Pin::new(&mut self.io).poll_write(cx, bytes)
    }
    fn poll_flush(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.io).poll_flush(cx)
    }
    fn poll_shutdown(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<io::Result<()>> {
        Pin::new(&mut self.io).poll_shutdown(cx)
    }
}

async fn upgrade(mut io: BoxIo, request: WebSocketRequest) -> Result<WebSocketUpgrade> {
    let mut nonce = [0u8; 16];
    getrandom::fill(&mut nonce).map_err(|_| Error::new(ErrorKind::Io))?;
    let key = STANDARD.encode(nonce);
    let outgoing = build_request(&request, &key)?;
    io.write_all(&outgoing).await.map_err(|_| network())?;
    io.flush().await.map_err(|_| network())?;
    let mut received = Zeroizing::new(Vec::with_capacity(4096));
    let end = loop {
        if let Some(start) = received.windows(4).position(|value| value == b"\r\n\r\n") {
            let end = start + 4;
            if end > MAX_HEADERS {
                return Err(Error::new(ErrorKind::Limit));
            }
            break end;
        }
        if received.len() >= MAX_HEADERS {
            return Err(Error::new(ErrorKind::Limit));
        }
        let mut chunk = [0u8; 4096];
        let count = io.read(&mut chunk).await.map_err(|_| network())?;
        if count == 0 {
            return Err(network());
        }
        received.extend_from_slice(&chunk[..count]);
    };
    let (status, headers) = validate_upgrade(&received[..end], &key)?;
    let prefix = Bytes::copy_from_slice(&received[end..]);
    let socket = ClientBuilder::new()
        .config(Config::default().flush_threshold(1).frame_size(16 * 1024))
        .limits(Limits::default().max_payload_len(Some(request.max_event_bytes)))
        .take_over(PrefixedIo { prefix, io });
    WebSocketUpgrade::new(
        status,
        headers,
        Box::new(NativeSocket {
            socket: Some(socket),
            pending_close: None,
            flush_before_read: false,
            max_event_bytes: request.max_event_bytes,
        }),
    )
}
fn build_request(request: &WebSocketRequest, key: &str) -> Result<Zeroizing<Vec<u8>>> {
    let mut bytes = Zeroizing::new(Vec::new());
    let path = &request.url[url::Position::BeforePath..url::Position::AfterQuery];
    let host = &request.url[url::Position::BeforeHost..url::Position::AfterPort];
    bytes.extend_from_slice(format!("GET {path} HTTP/1.1\r\nHost: {host}\r\nUpgrade: websocket\r\nConnection: Upgrade\r\nSec-WebSocket-Key: {key}\r\nSec-WebSocket-Version: 13\r\n").as_bytes());
    for (name, value) in &request.headers {
        if [
            "host",
            "upgrade",
            "connection",
            "sec-websocket-key",
            "sec-websocket-version",
            "sec-websocket-extensions",
            "sec-websocket-protocol",
            "content-length",
            "transfer-encoding",
        ]
        .contains(&name.as_str())
        {
            return Err(Error::new(ErrorKind::Validation));
        }
        if bytes
            .len()
            .checked_add(name.as_str().len())
            .and_then(|n| n.checked_add(value.len()))
            .and_then(|n| n.checked_add(6))
            .is_none_or(|n| n > MAX_HEADERS)
        {
            return Err(Error::new(ErrorKind::Limit));
        }
        bytes.extend_from_slice(name.as_str().as_bytes());
        bytes.extend_from_slice(b": ");
        bytes.extend_from_slice(value.as_bytes());
        bytes.extend_from_slice(b"\r\n");
    }
    bytes.extend_from_slice(b"\r\n");
    if bytes.len() > MAX_HEADERS {
        return Err(Error::new(ErrorKind::Limit));
    }
    Ok(bytes)
}
fn validate_upgrade(bytes: &[u8], key: &str) -> Result<(StatusCode, HeaderMap)> {
    if bytes.len() > MAX_HEADERS {
        return Err(Error::new(ErrorKind::Limit));
    }
    let mut entries = [httparse::EMPTY_HEADER; 128];
    let mut parsed = httparse::Response::new(&mut entries);
    if !matches!(parsed.parse(bytes), Ok(httparse::Status::Complete(count)) if count == bytes.len())
    {
        return Err(Error::new(ErrorKind::Protocol));
    }
    let status = StatusCode::from_u16(parsed.code.ok_or_else(|| Error::new(ErrorKind::Protocol))?)
        .map_err(|_| Error::new(ErrorKind::Protocol))?;
    let mut headers = HeaderMap::new();
    for entry in parsed.headers.iter() {
        let name = HeaderName::from_bytes(entry.name.as_bytes())
            .map_err(|_| Error::new(ErrorKind::Protocol))?;
        let mut value =
            HeaderValue::from_bytes(entry.value).map_err(|_| Error::new(ErrorKind::Protocol))?;
        value.set_sensitive(true);
        headers.append(name, value);
    }
    let request_id = crate::engine::error_metadata(&headers, None).0;
    let protocol = || {
        Error::new(ErrorKind::Protocol)
            .with_response_metadata(status.as_u16(), request_id.clone())
            .with_failure_kind(crate::FailureKind::Http)
    };
    if status != StatusCode::SWITCHING_PROTOCOLS {
        return Err(Error::new(if status.is_redirection() {
            ErrorKind::Destination
        } else {
            ErrorKind::Http
        })
        .with_status(status.as_u16())
        .with_metadata(request_id, None, None)
        .with_failure_kind(crate::FailureKind::Http)
        .with_outcome(if status.is_client_error() || status.is_redirection() {
            RemoteOutcome::Rejected
        } else {
            RemoteOutcome::Unknown
        }));
    }
    if parsed.version != Some(1) {
        return Err(protocol());
    }
    for name in [
        header::UPGRADE,
        header::CONNECTION,
        header::SEC_WEBSOCKET_ACCEPT,
    ] {
        if headers.get_all(name).iter().count() != 1 {
            return Err(protocol());
        }
    }
    if headers
        .get(header::UPGRADE)
        .is_none_or(|value| !value.as_bytes().eq_ignore_ascii_case(b"websocket"))
        || headers.get(header::CONNECTION).is_none_or(|value| {
            !value
                .as_bytes()
                .split(|byte| *byte == b',')
                .any(|token| token.trim_ascii().eq_ignore_ascii_case(b"upgrade"))
        })
        || [
            header::SEC_WEBSOCKET_EXTENSIONS,
            header::SEC_WEBSOCKET_PROTOCOL,
            header::CONTENT_LENGTH,
            header::TRANSFER_ENCODING,
        ]
        .iter()
        .any(|name| headers.contains_key(name))
    {
        return Err(protocol());
    }
    let mut input = key.as_bytes().to_vec();
    input.extend_from_slice(GUID);
    let expected = STANDARD.encode(
        aws_lc_rs::digest::digest(&aws_lc_rs::digest::SHA1_FOR_LEGACY_USE_ONLY, &input).as_ref(),
    );
    // Exact canonical comparison avoids the dependency's unchecked accept decoding.
    if headers
        .get(header::SEC_WEBSOCKET_ACCEPT)
        .is_none_or(|value| value.len() != 28 || value.as_bytes() != expected.as_bytes())
    {
        return Err(protocol());
    }
    Ok((status, headers))
}

type Socket = WebSocketStream<PrefixedIo>;
struct NativeSocket {
    socket: Option<Socket>,
    pending_close: Option<WebSocketFrame>,
    flush_before_read: bool,
    max_event_bytes: usize,
}
impl NativeSocket {
    fn call<T>(&mut self, call: impl FnOnce(Pin<&mut Socket>) -> T) -> Result<T> {
        let socket = self.socket.as_mut().ok_or_else(network)?;
        match catch_unwind(AssertUnwindSafe(|| call(Pin::new(socket)))) {
            Ok(value) => Ok(value),
            Err(_) => {
                self.socket = None;
                Err(network())
            }
        }
    }
    fn error(&mut self, error: tokio_websockets::Error) -> Error {
        self.socket = None;
        match error {
            tokio_websockets::Error::Io(_) => network(),
            tokio_websockets::Error::PayloadTooLong { .. } => Error::new(ErrorKind::Limit),
            _ => Error::new(ErrorKind::Protocol),
        }
    }
    fn flush(&mut self, cx: &mut Context<'_>) -> Poll<Result<()>> {
        match self.call(|socket| socket.poll_flush(cx)) {
            Ok(Poll::Ready(Ok(()))) => Poll::Ready(Ok(())),
            Ok(Poll::Ready(Err(error))) => Poll::Ready(Err(self.error(error))),
            Ok(Poll::Pending) => Poll::Pending,
            Err(error) => Poll::Ready(Err(error)),
        }
    }
}
impl Stream for NativeSocket {
    type Item = Result<WebSocketFrame>;
    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let this = self.get_mut();
        for _ in 0..32 {
            if this.flush_before_read {
                match ready!(this.flush(cx)) {
                    Ok(()) => {}
                    Err(error) => return Poll::Ready(Some(Err(error))),
                }
                this.flush_before_read = false;
                if let Some(close) = this.pending_close.take() {
                    return Poll::Ready(Some(Ok(close)));
                }
            }
            let message = match this.call(|socket| socket.poll_next(cx)) {
                Ok(Poll::Ready(Some(Ok(value)))) => value,
                Ok(Poll::Ready(Some(Err(error)))) => {
                    return Poll::Ready(Some(Err(this.error(error))));
                }
                Ok(Poll::Ready(None)) => return Poll::Ready(None),
                Ok(Poll::Pending) => return Poll::Pending,
                Err(error) => return Poll::Ready(Some(Err(error))),
            };
            if message.is_close() {
                let code = message.as_close().map_or(1005, |(code, _)| u16::from(code));
                this.pending_close = Some(WebSocketFrame::Close(code));
                this.flush_before_read = true;
                continue;
            }
            if message.is_ping() || message.is_pong() {
                this.flush_before_read = true;
                continue;
            }
            let bytes = Bytes::copy_from_slice(message.as_payload());
            let frame = if message.is_text() {
                WebSocketFrame::Text(bytes)
            } else {
                WebSocketFrame::Binary(bytes)
            };
            return Poll::Ready(Some(Ok(frame)));
        }
        cx.waker().wake_by_ref();
        Poll::Pending
    }
}
impl Sink<WebSocketFrame> for NativeSocket {
    type Error = Error;
    fn poll_ready(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();
        match this.call(|socket| socket.poll_ready(cx)) {
            Ok(Poll::Ready(Ok(()))) => Poll::Ready(Ok(())),
            Ok(Poll::Ready(Err(error))) => Poll::Ready(Err(this.error(error))),
            Ok(Poll::Pending) => Poll::Pending,
            Err(error) => Poll::Ready(Err(error)),
        }
    }
    fn start_send(self: Pin<&mut Self>, frame: WebSocketFrame) -> Result<()> {
        let this = self.get_mut();
        let message = match frame {
            WebSocketFrame::Text(bytes)
                if bytes.len() <= this.max_event_bytes && std::str::from_utf8(&bytes).is_ok() =>
            {
                Message::text(bytes)
            }
            WebSocketFrame::Close(1000) => Message::close(Some(CloseCode::NORMAL_CLOSURE), ""),
            _ => return Err(Error::new(ErrorKind::Validation)),
        };
        match this.call(|socket| socket.start_send(message))? {
            Ok(()) => Ok(()),
            Err(error) => Err(this.error(error)),
        }
    }
    fn poll_flush(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        self.get_mut().flush(cx)
    }
    fn poll_close(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Result<()>> {
        let this = self.get_mut();
        match this.call(|socket| socket.poll_close(cx)) {
            Ok(Poll::Ready(Ok(()))) => Poll::Ready(Ok(())),
            Ok(Poll::Ready(Err(error))) => Poll::Ready(Err(this.error(error))),
            Ok(Poll::Pending) => Poll::Pending,
            Err(error) => Poll::Ready(Err(error)),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    const KEY: &str = "dGhlIHNhbXBsZSBub25jZQ==";
    const ACCEPT: &str = "s3pPLMBiTxaQ9kYGzzhZRbK+xOo=";
    fn response(
        version: &str,
        upgrade: &str,
        connection: &str,
        accept: &str,
        extra: &str,
    ) -> Vec<u8> {
        format!("HTTP/{version} 101 Switching Protocols\r\nUpgrade: {upgrade}\r\nConnection: {connection}\r\nSec-WebSocket-Accept: {accept}\r\n{extra}\r\n").into_bytes()
    }
    #[test]
    fn rfc6455_known_accept_vector_and_connection_tokens_are_exact() {
        let bytes = response("1.1", "websocket", "keep-alive, UpGrAdE", ACCEPT, "");
        let (status, headers) = validate_upgrade(&bytes, KEY).unwrap();
        assert_eq!(status, StatusCode::SWITCHING_PROTOCOLS);
        assert!(headers.values().all(HeaderValue::is_sensitive));
    }
    #[test]
    fn untrusted_accepts_duplicates_protocol_extensions_and_versions_never_panic() {
        let mut invalid = vec![
            response("1.0", "websocket", "Upgrade", ACCEPT, ""),
            response("1.1", "h2", "Upgrade", ACCEPT, ""),
            response("1.1", "websocket", "keep-alive", ACCEPT, ""),
            response("1.1", "websocket", "Upgrade", "AAAA", ""),
        ];
        for extra in [
            "Upgrade: websocket\r\n",
            "Connection: Upgrade\r\n",
            "Sec-WebSocket-Accept: s3pPLMBiTxaQ9kYGzzhZRbK+xOo=\r\n",
            "Sec-WebSocket-Extensions: permessage-deflate\r\n",
            "Sec-WebSocket-Protocol: unrequested\r\n",
            "Content-Length: 1\r\n",
            "Transfer-Encoding: chunked\r\n",
        ] {
            invalid.push(response("1.1", "websocket", "Upgrade", ACCEPT, extra));
        }
        invalid.push(response(
            "1.1",
            "websocket",
            "Upgrade",
            &"A".repeat(60_000),
            "",
        ));
        invalid.push(vec![b'A'; MAX_HEADERS + 1]);
        for bytes in invalid {
            assert!(
                catch_unwind(|| validate_upgrade(&bytes, KEY))
                    .unwrap()
                    .is_err()
            );
        }
    }
    #[test]
    fn optional_tls_setup_preserves_global_crypto_provider_and_rejects_bad_certificates() {
        let before = rustls::crypto::CryptoProvider::get_default().map(Arc::as_ptr);
        NativeWebSocketConnector::new().unwrap();
        assert_eq!(
            rustls::crypto::CryptoProvider::get_default().map(Arc::as_ptr),
            before
        );
        assert!(
            NativeWebSocketConnector::with_additional_certificates(&[b"not a certificate"])
                .is_err()
        );
    }
}
