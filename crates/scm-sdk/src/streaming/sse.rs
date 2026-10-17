//! Incremental UTF-8 server-sent events with explicit completion policies.
//!
//! Parsing is driven by the consumer. No reader task, reconnect, or replay is
//! started. Frames are bounded before allocation can exceed the selected limit.

use std::{fmt, time::Duration};

use bytes::Bytes;
use serde::{
    Deserialize,
    de::{self, DeserializeOwned, IgnoredAny, MapAccess, Visitor},
};
use serde_json::Value;

use super::ResponseStream;
use crate::{Error, ErrorKind, RemoteOutcome, Result};

/// The remote evidence required before a stream is considered complete.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum TerminalPolicy {
    /// Chat/completion streams end only at an exact `data: [DONE]` frame.
    DoneMarker,
    /// Responses streams deliver a completed/failed/incomplete JSON lifecycle
    /// event and close immediately. An event name alone is not completion proof.
    Responses,
    /// A clean HTTP EOF is sufficient after complete data frames.
    Eof,
}

/// One raw SSE event. Explicit accessors expose server-controlled values;
/// ordinary formatting omits every event, identifier, and payload value.
pub struct SseEvent {
    event: String,
    data: String,
    id: Option<String>,
    retry: Option<Duration>,
    status: u16,
    request_id: Option<String>,
    operation: &'static str,
}

impl SseEvent {
    pub fn event(&self) -> &str {
        &self.event
    }
    pub fn data(&self) -> &str {
        &self.data
    }
    /// The last valid event ID persists across frames. An empty ID clears it.
    pub fn id(&self) -> Option<&str> {
        self.id.as_deref()
    }
    /// Server reconnection advice is metadata; this SDK never reconnects itself.
    pub fn retry(&self) -> Option<Duration> {
        self.retry
    }

    /// Decode explicitly accessed event data without retaining parse errors or
    /// echoing upstream payloads. Gateway error envelopes are rejected safely.
    pub fn json<T: DeserializeOwned>(&self) -> Result<T> {
        let value =
            crate::json_decode::from_str(&self.data).map_err(|_| self.error(ErrorKind::Decode))?;
        if raw_error_truthy(&self.data).map_err(|_| self.error(ErrorKind::Decode))? {
            return Err(self.error(ErrorKind::Protocol));
        }
        crate::json_decode::from_value(value).map_err(|_| self.error(ErrorKind::Decode))
    }

    fn error(&self, kind: ErrorKind) -> Error {
        Error::new(kind)
            .with_response_metadata(self.status, self.request_id.clone())
            .with_operation(self.operation)
            .with_outcome(RemoteOutcome::Unknown)
    }
}

impl fmt::Debug for SseEvent {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SseEvent")
            .field("data_bytes", &self.data.len())
            .field("has_id", &self.id.is_some())
            .field("has_retry", &self.retry.is_some())
            .finish_non_exhaustive()
    }
}

fn raw_error_truthy(data: &str) -> std::result::Result<bool, serde_json::Error> {
    // Inspect the actual error token without changing numeric truthiness under
    // downstream arbitrary_precision unification. The shared JSON boundary
    // rejects literal reserved-protocol objects first. Borrowing RawValue avoids
    // extra payload copies, and unrelated fields are skipped.
    if !data.trim_start().starts_with('{') {
        return Ok(false);
    }
    struct Envelope(bool);
    impl<'de> Deserialize<'de> for Envelope {
        fn deserialize<D: serde::Deserializer<'de>>(
            deserializer: D,
        ) -> std::result::Result<Self, D::Error> {
            struct EnvelopeVisitor;
            impl<'de> Visitor<'de> for EnvelopeVisitor {
                type Value = Envelope;
                fn expecting(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
                    formatter.write_str("a JSON object")
                }
                fn visit_map<A: MapAccess<'de>>(
                    self,
                    mut map: A,
                ) -> std::result::Result<Envelope, A::Error> {
                    let mut truthy = false;
                    while let Some(key) = map.next_key::<String>()? {
                        if key == "error" {
                            let raw: &'de serde_json::value::RawValue = map.next_value()?;
                            let raw = raw.get().trim();
                            truthy = match raw.as_bytes().first() {
                                Some(b'n' | b'f') => false,
                                Some(b't' | b'{' | b'[') => true,
                                Some(b'"') => !serde_json::from_str::<String>(raw)
                                    .map_err(|_| de::Error::custom("invalid JSON error token"))?
                                    .is_empty(),
                                // JSON.parse uses IEEE-754: underflow is zero,
                                // overflow is infinity. Neither path rounds an
                                // opaque API value; this is only truthiness.
                                Some(b'-' | b'0'..=b'9') => {
                                    raw.parse::<f64>().map_err(|_| {
                                        de::Error::custom("invalid JSON error number")
                                    })? != 0.0
                                }
                                _ => return Err(de::Error::custom("invalid JSON error token")),
                            };
                        } else {
                            map.next_value::<IgnoredAny>()?;
                        }
                    }
                    // Duplicate fields follow JSON.parse's last-key-wins rule.
                    Ok(Envelope(truthy))
                }
            }
            deserializer.deserialize_map(EnvelopeVisitor)
        }
    }
    serde_json::from_str::<Envelope>(data).map(|value| value.0)
}

/// A bounded, single-consumer SSE decoder owning its HTTP response and permit.
pub struct SseStream {
    response: ResponseStream,
    policy: TerminalPolicy,
    max_event_bytes: usize,
    pending: Bytes,
    offset: usize,
    line: Vec<u8>,
    first_line: bool,
    skip_lf: bool,
    frame_bytes: usize,
    event_name: String,
    data: String,
    has_data: bool,
    last_id: Option<String>,
    retry: Option<Duration>,
    finished: bool,
    failure: Option<Error>,
}

impl SseStream {
    /// Validate stream media type before any body is consumed. The frame limit
    /// counts UTF-8 bytes and one byte per normalized CR/LF/CRLF line delimiter,
    /// including comments and ignored fields; total wire bytes have their own
    /// ResponseStream limit. The limit must be 1 byte through 16 MiB.
    pub fn new(
        mut response: ResponseStream,
        max_event_bytes: usize,
        policy: TerminalPolicy,
    ) -> Result<Self> {
        let fail = |kind| {
            Error::new(kind)
                .with_response_metadata(
                    response.status().as_u16(),
                    response.request_id().map(str::to_owned),
                )
                .with_operation(response.operation)
                .with_outcome(RemoteOutcome::Unknown)
        };
        if !(1..=16 * 1024 * 1024).contains(&max_event_bytes) {
            let error = fail(ErrorKind::Configuration);
            response.close();
            return Err(error);
        }
        let mut values = response
            .headers()
            .get_all(http::header::CONTENT_TYPE)
            .iter();
        let content_type = values
            .next()
            .and_then(|value| value.to_str().ok())
            .and_then(|value| value.parse::<mime::Mime>().ok());
        let valid = values.next().is_none()
            && content_type.is_some_and(|value| {
                value.type_() == mime::TEXT
                    && value
                        .subtype()
                        .as_str()
                        .eq_ignore_ascii_case("event-stream")
                    && value
                        .get_param(mime::CHARSET)
                        .is_none_or(|charset| charset.as_str().eq_ignore_ascii_case("utf-8"))
            });
        if !valid {
            let error = fail(ErrorKind::Protocol);
            response.close();
            return Err(error);
        }
        Ok(Self {
            response,
            policy,
            max_event_bytes,
            pending: Bytes::new(),
            offset: 0,
            line: Vec::new(),
            first_line: true,
            skip_lf: false,
            frame_bytes: 0,
            event_name: String::new(),
            data: String::new(),
            has_data: false,
            last_id: None,
            retry: None,
            finished: false,
            failure: None,
        })
    }

    /// End local consumption immediately and release the response and permit.
    pub fn close(&mut self) {
        self.response.close();
        self.pending = Bytes::new();
        self.offset = 0;
        self.line = Vec::new();
        self.data = String::new();
        self.event_name = String::new();
        self.last_id = None;
        self.retry = None;
        self.finished = true;
    }

    fn error(&self, kind: ErrorKind) -> Error {
        Error::new(kind)
            .with_response_metadata(
                self.response.status().as_u16(),
                self.response.request_id().map(str::to_owned),
            )
            .with_operation(self.response.operation)
            .with_outcome(RemoteOutcome::Unknown)
    }
    fn fail(&mut self, error: Error) -> Error {
        let error = error
            .with_response_metadata(
                self.response.status().as_u16(),
                self.response.request_id().map(str::to_owned),
            )
            .with_outcome(RemoteOutcome::Unknown);
        self.close();
        self.failure = Some(error.clone());
        error
    }
    fn active(&mut self) -> Result<()> {
        self.response
            .check_active()
            .map_err(|error| self.fail(error))
    }

    /// Yield one event. A required terminal missing at EOF is an explicit
    /// protocol failure. Failed streams retain their sanitized failure.
    pub async fn next(&mut self) -> Result<Option<SseEvent>> {
        if let Some(error) = &self.failure {
            return Err(error.clone());
        }
        if self.finished {
            return Ok(None);
        }
        self.active()?;
        let mut work = 0usize;
        let mut empty_chunks = 0usize;
        loop {
            while self.offset < self.pending.len() {
                let byte = self.pending[self.offset];
                self.offset += 1;
                work += 1;
                if self.skip_lf {
                    self.skip_lf = false;
                    if byte == b'\n' {
                        continue;
                    }
                }
                self.frame_bytes = self
                    .frame_bytes
                    .checked_add(1)
                    .ok_or_else(|| self.error(ErrorKind::Limit))
                    .map_err(|error| self.fail(error))?;
                if self.frame_bytes > self.max_event_bytes {
                    let error = self.error(ErrorKind::Limit);
                    return Err(self.fail(error));
                }
                if matches!(byte, b'\r' | b'\n') {
                    self.skip_lf = byte == b'\r';
                    match self.finish_line() {
                        Ok(Some(event)) => {
                            self.active()?;
                            // Frozen parser recognizes the chat sentinel before
                            // interpreting an untrusted event name.
                            if self.policy == TerminalPolicy::DoneMarker && event.data == "[DONE]" {
                                self.close();
                                return Ok(None);
                            }
                            if event.event == "error" {
                                let error = self.error(ErrorKind::Protocol);
                                return Err(self.fail(error));
                            }
                            if self.policy == TerminalPolicy::Responses {
                                let value: Value = match event.json() {
                                    Ok(value) => value,
                                    Err(error) => return Err(self.fail(error)),
                                };
                                let Some(kind) = value
                                    .get("type")
                                    .and_then(Value::as_str)
                                    .filter(|value| !value.is_empty())
                                else {
                                    let error = self.error(ErrorKind::Protocol);
                                    return Err(self.fail(error));
                                };
                                if matches!(
                                    kind,
                                    "response.completed"
                                        | "response.failed"
                                        | "response.incomplete"
                                ) {
                                    self.close();
                                }
                            }
                            return Ok(Some(event));
                        }
                        Ok(None) => {}
                        Err(error) => return Err(self.fail(error)),
                    }
                } else {
                    self.line.push(byte);
                }
                if work >= 4096 {
                    tokio::task::yield_now().await;
                    self.active()?;
                    work = 0;
                }
            }
            self.pending = Bytes::new();
            self.offset = 0;
            match self.response.next().await {
                Ok(Some(bytes)) => {
                    if bytes.is_empty() {
                        empty_chunks += 1;
                        if empty_chunks > 128 {
                            let error = self.error(ErrorKind::Protocol);
                            return Err(self.fail(error));
                        }
                        tokio::task::yield_now().await;
                        self.active()?;
                    } else {
                        empty_chunks = 0;
                        self.pending = bytes;
                    }
                }
                Ok(None) => {
                    // A trailing non-data comment/field may be ignored, but data
                    // without its blank frame delimiter must not disappear.
                    if !self.line.is_empty()
                        && let Err(error) = self.finish_line()
                    {
                        return Err(self.fail(error));
                    }
                    if self.has_data || self.policy != TerminalPolicy::Eof {
                        let error = self.error(ErrorKind::Protocol);
                        return Err(self.fail(error));
                    }
                    self.close();
                    return Ok(None);
                }
                Err(error) => return Err(self.fail(error)),
            }
        }
    }

    fn finish_line(&mut self) -> Result<Option<SseEvent>> {
        let bytes = std::mem::take(&mut self.line);
        let mut line = String::from_utf8(bytes).map_err(|_| self.error(ErrorKind::Protocol))?;
        if self.first_line {
            self.first_line = false;
            if line.starts_with('\u{feff}') {
                line.drain(..'\u{feff}'.len_utf8());
            }
        }
        if line.is_empty() {
            self.frame_bytes = 0;
            let event_name = std::mem::take(&mut self.event_name);
            if !self.has_data {
                return Ok(None);
            }
            self.has_data = false;
            return Ok(Some(SseEvent {
                event: if event_name.is_empty() {
                    "message".into()
                } else {
                    event_name
                },
                data: std::mem::take(&mut self.data),
                id: self.last_id.clone(),
                retry: self.retry,
                status: self.response.status().as_u16(),
                request_id: self.response.request_id().map(str::to_owned),
                operation: self.response.operation,
            }));
        }
        if line.starts_with(':') {
            return Ok(None);
        }
        let (field, value) = line.split_once(':').unwrap_or((&line, ""));
        let value = value.strip_prefix(' ').unwrap_or(value);
        match field {
            "data" => {
                if self.has_data {
                    self.data.push('\n');
                }
                self.data.push_str(value);
                self.has_data = true;
            }
            "event" => self.event_name = value.to_owned(),
            "id" if !value.contains('\0') => self.last_id = Some(value.to_owned()),
            "retry" if !value.is_empty() && value.bytes().all(|byte| byte.is_ascii_digit()) => {
                if let Ok(milliseconds) = value.parse::<u64>() {
                    self.retry = Some(Duration::from_millis(milliseconds));
                }
            }
            _ => {}
        }
        Ok(None)
    }
}

impl fmt::Debug for SseStream {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter
            .debug_struct("SseStream")
            .field("policy", &self.policy)
            .field("max_event_bytes", &self.max_event_bytes)
            .field("finished", &self.finished)
            .field("failed", &self.failure.is_some())
            .finish_non_exhaustive()
    }
}

impl Drop for SseStream {
    fn drop(&mut self) {
        self.close();
    }
}

#[cfg(test)]
mod tests;
