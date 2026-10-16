//! Bounded, caller-driven response bodies. Dropping a response releases its socket
//! and concurrency permit; no reader or background task outlives the caller.
pub mod sse;
pub use sse::{SseEvent, SseStream, TerminalPolicy};

/// A native schema-validated event stream. Unknown response fields and values survive.
#[cfg(feature = "gateway")]
pub struct ContractStream<T> {
    stream: SseStream,
    contract: EventContract,
    options: RequestOptions,
    session: Session,
    failure: Option<Error>,
    marker: std::marker::PhantomData<T>,
}
#[cfg(feature = "gateway")]
pub(crate) struct EventContract {
    pub schema: &'static str,
    pub operation: &'static str,
    pub status: u16,
    pub limits: crate::validation::ValidationLimits,
}
#[cfg(feature = "gateway")]
impl<T: serde::de::DeserializeOwned> ContractStream<T> {
    pub(crate) fn new(
        stream: SseStream,
        contract: EventContract,
        options: RequestOptions,
        session: Session,
    ) -> Self {
        Self {
            stream,
            contract,
            options,
            session,
            failure: None,
            marker: std::marker::PhantomData,
        }
    }
    pub fn close(&mut self) {
        self.stream.close();
    }
    pub async fn next(&mut self) -> Result<Option<T>> {
        if let Some(error) = &self.failure {
            return Err(error.clone());
        }
        let event = loop {
            match self.stream.next().await {
                Ok(Some(event)) if event.data().is_empty() => {
                    // Match the frozen typed Gateway parser's empty-data keepalives.
                    // Raw SSE consumers still receive these framing events.
                    let active = self
                        .options
                        .run(&self.session, async {
                            tokio::task::yield_now().await;
                            Ok(())
                        })
                        .await;
                    if let Err(error) = active {
                        let error = error
                            .with_status(self.contract.status)
                            .with_operation(self.contract.operation)
                            .with_outcome(RemoteOutcome::Unknown);
                        self.stream.close();
                        self.failure = Some(error.clone());
                        return Err(error);
                    }
                }
                Ok(Some(event)) => break event,
                Ok(None) => return Ok(None),
                Err(error) => {
                    self.failure = Some(error.clone());
                    return Err(error);
                }
            }
        };
        let result = self
            .options
            .run(&self.session, async {
                let value = event.json::<serde_json::Value>()?;
                let value = crate::validation::parse_with_limits(
                    self.contract.schema,
                    value,
                    crate::validation::Mode::Response,
                    &self.contract.limits,
                )
                .map_err(|error| {
                    Error::new(if error.kind() == ErrorKind::Limit {
                        ErrorKind::Limit
                    } else {
                        ErrorKind::Decode
                    })
                })?;
                serde_json::from_value(value).map_err(|_| Error::new(ErrorKind::Decode))
            })
            .await;
        match result {
            Ok(value) => Ok(Some(value)),
            Err(error) => {
                let error = error
                    .with_status(self.contract.status)
                    .with_operation(self.contract.operation)
                    .with_outcome(RemoteOutcome::Unknown);
                self.stream.close();
                self.failure = Some(error.clone());
                Err(error)
            }
        }
    }
}
#[cfg(feature = "gateway")]
impl<T> fmt::Debug for ContractStream<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ContractStream")
            .field("operation", &self.contract.operation)
            .field("stream", &self.stream)
            .finish_non_exhaustive()
    }
}

use crate::{
    Error, ErrorKind, Limits, RemoteOutcome, RequestOptions, Result, Session, transport::BodyStream,
};
use bytes::Bytes;
use futures_util::StreamExt;
use http::{HeaderMap, StatusCode};
use std::{fmt, time::Duration};
use tokio::{sync::OwnedSemaphorePermit, time::Instant};
use tokio_util::sync::CancellationToken;

pub struct ResponseStream {
    status: StatusCode,
    headers: HeaderMap,
    body: Option<BodyStream>,
    permit: Option<OwnedSemaphorePermit>,
    session: Session,
    cancellation: CancellationToken,
    deadline: Option<Instant>,
    idle: Duration,
    operation: &'static str,
    max_bytes: usize,
    consumed: usize,
}

impl ResponseStream {
    pub(crate) fn new(
        response: crate::transport::TransportResponse,
        permit: OwnedSemaphorePermit,
        session: Session,
        options: &RequestOptions,
        explicit_deadline: bool,
        limits: &Limits,
        operation: &'static str,
    ) -> Self {
        let deadline = match (
            explicit_deadline.then_some(options.deadline()),
            limits
                .stream_session_timeout
                .and_then(|v| Instant::now().checked_add(v)),
        ) {
            (Some(a), Some(b)) => Some(a.min(b)),
            (a, b) => a.or(b),
        };
        let mut headers = response.headers;
        for value in headers.values_mut() {
            value.set_sensitive(true);
        }
        Self {
            status: response.status,
            headers,
            body: Some(response.body),
            permit: Some(permit),
            session,
            cancellation: options.cancellation_token(),
            deadline,
            idle: limits.stream_idle_timeout,
            operation,
            max_bytes: limits.max_response_bytes,
            consumed: 0,
        }
    }
    pub fn status(&self) -> StatusCode {
        self.status
    }
    /// Explicit access to response headers; values are marked sensitive for formatting.
    pub fn headers(&self) -> &HeaderMap {
        &self.headers
    }
    pub fn bytes_consumed(&self) -> usize {
        self.consumed
    }
    pub fn close(&mut self) {
        self.body = None;
        self.permit = None;
    }
    pub(crate) fn check_active(&self) -> Result<()> {
        let kind = if self.cancellation.is_cancelled() || self.session.is_cancelled() {
            Some(ErrorKind::Cancelled)
        } else if self.deadline.is_some_and(|value| Instant::now() >= value) {
            Some(ErrorKind::Deadline)
        } else {
            None
        };
        match kind {
            Some(kind) => Err(Error::new(kind)
                .with_operation(self.operation)
                .with_outcome(RemoteOutcome::Unknown)),
            None => Ok(()),
        }
    }
    /// Read the next decompressed chunk within idle, caller, and session budgets.
    /// No stream is automatically replayed, including before its first chunk.
    pub async fn next(&mut self) -> Result<Option<Bytes>> {
        if self.body.is_none() {
            return Ok(None);
        }
        let result = async {
            self.check_active()?;
            let now = Instant::now();
            let deadline = now
                .checked_add(self.idle)
                .ok_or_else(|| Error::new(ErrorKind::Configuration))?;
            let deadline = self.deadline.map_or(deadline, |value| value.min(deadline));
            if now >= deadline {
                return Err(Error::new(ErrorKind::Deadline));
            }
            let body = self
                .body
                .as_mut()
                .ok_or_else(|| Error::new(ErrorKind::Protocol))?;
            let value = tokio::select! {biased;
                _=self.cancellation.cancelled()=>Err(Error::new(ErrorKind::Cancelled)),
                _=self.session.cancelled()=>Err(Error::new(ErrorKind::Cancelled)),
                _=tokio::time::sleep_until(deadline)=>Err(Error::new(ErrorKind::Deadline)),
                value=body.next()=>value.transpose(),
            }?;
            if self.cancellation.is_cancelled() || self.session.is_cancelled() {
                return Err(Error::new(ErrorKind::Cancelled));
            }
            if Instant::now() >= deadline {
                return Err(Error::new(ErrorKind::Deadline));
            }
            if let Some(bytes) = &value {
                self.consumed = self
                    .consumed
                    .checked_add(bytes.len())
                    .ok_or_else(|| Error::new(ErrorKind::Limit))?;
                if self.consumed > self.max_bytes {
                    return Err(Error::new(ErrorKind::Limit));
                }
            }
            Ok(value)
        }
        .await;
        match result {
            Ok(None) => {
                self.close();
                Ok(None)
            }
            Ok(value) => Ok(value),
            Err(error) => {
                self.close();
                Err(error
                    .with_operation(self.operation)
                    .with_outcome(RemoteOutcome::Unknown))
            }
        }
    }
}
impl fmt::Debug for ResponseStream {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ResponseStream")
            .field("operation", &self.operation)
            .field("status", &self.status)
            .field("bytes_consumed", &self.consumed)
            .field("closed", &self.body.is_none())
            .finish_non_exhaustive()
    }
}
