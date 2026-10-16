//! Structured errors with safe ordinary formatting.

use std::{fmt, time::Duration};

/// SDK result type.
pub type Result<T> = std::result::Result<T, Error>;

/// The failure category, independent of arbitrary upstream messages.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorKind {
    Configuration,
    Validation,
    Destination,
    Authentication,
    Io,
    Transport,
    Http,
    Decode,
    Limit,
    Deadline,
    Cancelled,
    Protocol,
    Pagination,
}

/// Whether the remote operation may have committed before the local failure.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[non_exhaustive]
pub enum RemoteOutcome {
    NotSent,
    Rejected,
    Unknown,
}

/// No body, credential, URL, transport source error, or upstream message is retained.
#[derive(Clone)]
pub struct Error {
    kind: ErrorKind,
    status: Option<u16>,
    operation: Option<&'static str>,
    request_id: Option<String>,
    service_code: Option<String>,
    retry_after: Option<Duration>,
    remote_outcome: RemoteOutcome,
}

impl Error {
    /// Construct an error with no remote request sent.
    pub fn new(kind: ErrorKind) -> Self {
        Self {
            kind,
            status: None,
            operation: None,
            request_id: None,
            service_code: None,
            retry_after: None,
            remote_outcome: RemoteOutcome::NotSent,
        }
    }
    pub fn kind(&self) -> ErrorKind {
        self.kind
    }
    pub fn status(&self) -> Option<u16> {
        self.status
    }
    pub fn operation(&self) -> Option<&'static str> {
        self.operation
    }
    /// Explicit access to an allowlisted correlation field; ordinary formatting redacts it.
    pub fn request_id(&self) -> Option<&str> {
        self.request_id.as_deref()
    }
    /// Explicit access to the service code; ordinary formatting redacts it.
    pub fn service_code(&self) -> Option<&str> {
        self.service_code.as_deref()
    }
    pub fn retry_after(&self) -> Option<Duration> {
        self.retry_after
    }
    pub fn remote_outcome(&self) -> RemoteOutcome {
        self.remote_outcome
    }
    pub(crate) fn with_operation(mut self, name: &'static str) -> Self {
        self.operation = Some(name);
        self
    }
    pub(crate) fn with_outcome(mut self, value: RemoteOutcome) -> Self {
        self.remote_outcome = value;
        self
    }
    pub(crate) fn with_status(mut self, status: u16) -> Self {
        self.status = Some(status);
        self
    }
    pub(crate) fn with_response_metadata(
        mut self,
        status: u16,
        request_id: Option<String>,
    ) -> Self {
        self.status = Some(status);
        if self.request_id.is_none() {
            self.request_id = request_id;
        }
        self
    }
    pub(crate) fn with_metadata(
        mut self,
        request_id: Option<String>,
        code: Option<String>,
        delay: Option<Duration>,
    ) -> Self {
        self.request_id = request_id;
        self.service_code = code;
        self.retry_after = delay;
        self
    }
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Error")
            .field("kind", &self.kind)
            .field("status", &self.status)
            .field("operation", &self.operation)
            .field("remote_outcome", &self.remote_outcome)
            .field("retry_after", &self.retry_after)
            .field("has_request_id", &self.request_id.is_some())
            .field("has_service_code", &self.service_code.is_some())
            .finish()
    }
}
impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "SCM SDK {:?}", self.kind)?;
        if let Some(status) = self.status {
            write!(f, " (HTTP {status})")?;
        }
        Ok(())
    }
}
impl std::error::Error for Error {}
