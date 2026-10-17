//! Closed metadata vocabulary. Caller labels, errors and payloads never enter tracing fields.
use crate::{Error, Result};
#[cfg(feature = "tracing")]
use crate::{ErrorKind, RemoteOutcome};
#[cfg(feature = "tracing")]
use std::sync::atomic::{AtomicBool, AtomicU16, Ordering};

#[derive(Clone, Copy)]
pub(crate) enum Operation {
    Http,
    HttpStream,
    Decode,
    OAuthRefresh,
    OAuthCache,
    BodyStream,
    #[cfg(feature = "gateway")]
    TypedStream,
    #[cfg(feature = "realtime")]
    RealtimeUpgrade,
    #[cfg(feature = "realtime")]
    RealtimeSession,
}
#[cfg(feature = "tracing")]
impl Operation {
    fn name(self) -> &'static str {
        match self {
            Self::Http => "sdk.http.exchange",
            Self::HttpStream => "sdk.http.stream_handshake",
            Self::Decode => "sdk.http.decode",
            Self::OAuthRefresh => "sdk.oauth.refresh",
            Self::OAuthCache => "sdk.oauth.cache",
            Self::BodyStream => "sdk.stream.body",
            #[cfg(feature = "gateway")]
            Self::TypedStream => "sdk.stream.typed",
            #[cfg(feature = "realtime")]
            Self::RealtimeUpgrade => "sdk.realtime.upgrade",
            #[cfg(feature = "realtime")]
            Self::RealtimeSession => "sdk.realtime.session",
        }
    }
}
/// An owned lifecycle tracks dispatch even when its future is dropped during an await.
/// With the feature disabled this is zero-sized and all methods compile to no-ops.
pub(crate) struct Lifecycle {
    #[cfg(feature = "tracing")]
    state: State,
}
#[cfg(feature = "tracing")]
struct State {
    operation: Operation,
    complete: AtomicBool,
    dispatched: AtomicBool,
    status: AtomicU16,
    correlation: AtomicBool,
}
impl Lifecycle {
    pub(crate) fn start(_operation: Operation) -> Self {
        let lifecycle = Self {
            #[cfg(feature = "tracing")]
            state: State {
                operation: _operation,
                complete: AtomicBool::new(false),
                dispatched: AtomicBool::new(false),
                status: AtomicU16::new(0),
                correlation: AtomicBool::new(false),
            },
        };
        #[cfg(feature = "tracing")]
        lifecycle.state.emit("start", "none", "not_sent");
        lifecycle
    }
    pub(crate) fn received(operation: Operation, status: u16, correlation: bool) -> Self {
        let lifecycle = Self::start(operation);
        lifecycle.dispatched();
        lifecycle.response(status, correlation);
        lifecycle
    }
    pub(crate) fn dispatched(&self) {
        #[cfg(feature = "tracing")]
        {
            self.state.dispatched.store(true, Ordering::Relaxed);
            self.state.status.store(0, Ordering::Relaxed);
            self.state.correlation.store(false, Ordering::Relaxed);
            self.state.emit("dispatch", "none", "unknown");
        }
    }
    pub(crate) fn response(&self, _status: u16, _correlation: bool) {
        #[cfg(feature = "tracing")]
        {
            self.state
                .status
                .store(safe_status(Some(_status)), Ordering::Relaxed);
            self.state
                .correlation
                .store(_correlation, Ordering::Relaxed);
            self.state.emit("response", "none", "unknown");
        }
    }
    pub(crate) fn success(&self) {
        #[cfg(feature = "tracing")]
        if !self.state.complete.swap(true, Ordering::Relaxed) {
            self.state.emit("complete", "none", "success");
        }
    }
    pub(crate) fn closed(&self) {
        #[cfg(feature = "tracing")]
        if !self.state.complete.swap(true, Ordering::Relaxed) {
            self.state.emit("closed", "none", "unknown");
        }
    }
    pub(crate) fn failed(&self, _error: &Error) {
        #[cfg(feature = "tracing")]
        if !self.state.complete.swap(true, Ordering::Relaxed) {
            if _error.status().is_some() {
                self.state
                    .status
                    .store(safe_status(_error.status()), Ordering::Relaxed);
            }
            if _error.request_id().is_some() {
                self.state.correlation.store(true, Ordering::Relaxed);
            }
            let remote = if _error.remote_outcome() == RemoteOutcome::NotSent
                && self.state.dispatched.load(Ordering::Relaxed)
            {
                RemoteOutcome::Unknown
            } else {
                _error.remote_outcome()
            };
            self.state
                .emit("failure", kind(_error.kind()), outcome(remote));
        }
    }
    pub(crate) fn finish<T>(&self, result: &Result<T>) {
        match result {
            Ok(_) => self.success(),
            Err(error) => self.failed(error),
        }
    }
}
impl Drop for Lifecycle {
    fn drop(&mut self) {
        #[cfg(feature = "tracing")]
        if !self.state.complete.swap(true, Ordering::Relaxed) {
            self.state.emit(
                "drop",
                "cancelled",
                if self.state.dispatched.load(Ordering::Relaxed) {
                    "unknown"
                } else {
                    "not_sent"
                },
            );
        }
    }
}
pub(crate) fn failure(operation: Operation, error: &Error) {
    let lifecycle = Lifecycle::start(operation);
    lifecycle.failed(error);
}
pub(crate) fn published() {
    let lifecycle = Lifecycle::start(Operation::OAuthCache);
    lifecycle.success();
}
#[cfg(feature = "tracing")]
impl State {
    fn emit(&self, stage: &'static str, kind: &'static str, outcome: &'static str) {
        tracing::event!(target: "scm_sdk::lifecycle", parent: None, tracing::Level::DEBUG,
            operation = self.operation.name(), stage, kind, outcome,
            status = u64::from(self.status.load(Ordering::Relaxed)),
            has_correlation = self.correlation.load(Ordering::Relaxed));
    }
}
#[cfg(feature = "tracing")]
fn safe_status(status: Option<u16>) -> u16 {
    status.filter(|s| (100..=599).contains(s)).unwrap_or(0)
}
#[cfg(feature = "tracing")]
fn kind(value: ErrorKind) -> &'static str {
    match value {
        ErrorKind::Configuration => "configuration",
        ErrorKind::Validation => "validation",
        ErrorKind::Destination => "destination",
        ErrorKind::Authentication => "authentication",
        ErrorKind::Io => "io",
        ErrorKind::Transport => "transport",
        ErrorKind::Http => "http",
        ErrorKind::Decode => "decode",
        ErrorKind::Limit => "limit",
        ErrorKind::Deadline => "deadline",
        ErrorKind::Cancelled => "cancelled",
        ErrorKind::Protocol => "protocol",
        ErrorKind::Pagination => "pagination",
    }
}
#[cfg(feature = "tracing")]
fn outcome(value: RemoteOutcome) -> &'static str {
    match value {
        RemoteOutcome::NotSent => "not_sent",
        RemoteOutcome::Rejected => "rejected",
        RemoteOutcome::Unknown => "unknown",
    }
}
#[cfg(all(test, feature = "tracing"))]
mod tests;
#[cfg(all(test, not(feature = "tracing")))]
mod disabled_tests {
    #[test]
    fn disabled_observability_has_no_runtime_state() {
        assert_eq!(std::mem::size_of::<super::Lifecycle>(), 0);
    }
}
