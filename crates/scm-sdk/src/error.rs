//! Structured errors with safe ordinary formatting.

use std::{backtrace::Backtrace, fmt, sync::Arc, time::Duration};
use zeroize::Zeroizing;

mod problem;
pub(crate) use problem::body_retry_after;
pub use problem::{ErrorMetadata, ErrorType, FailureKind, ProblemDetails, ProblemFieldError};

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

/// No raw body, credential, URL, transport source error, or upstream message is retained.
/// Allowlisted problem fields and explicit application messages have separate getters.
#[derive(Clone)]
pub struct Error {
    kind: ErrorKind,
    status: Option<u16>,
    operation: Option<&'static str>,
    request_id: Option<String>,
    service_code: Option<String>,
    retry_after: Option<Duration>,
    remote_outcome: RemoteOutcome,
    details: Option<Arc<ErrorDetails>>,
}
#[derive(Clone, Default)]
struct ErrorDetails {
    custom_type: Option<Option<ErrorType>>,
    message: Option<Zeroizing<String>>,
    problem: Option<ProblemDetails>,
    failure_kind: Option<FailureKind>,
    backtrace: Option<Arc<Backtrace>>,
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
            details: None,
        }
    }
    /// Construct an application error with checked metadata and an explicit message.
    /// The message is bounded to 8 KiB and omitted from all ordinary formatting.
    pub fn custom(
        message: impl AsRef<str>,
        error_type: Option<ErrorType>,
        metadata: ErrorMetadata,
    ) -> Result<Self> {
        let message = message.as_ref();
        if message.len() > 8192 {
            return Err(Self::new(ErrorKind::Limit));
        }
        let message = Zeroizing::new(message.to_owned());
        let mut error = Self::new(ErrorKind::Protocol);
        error.status = metadata.status;
        error.retry_after = metadata.retry_after;
        error.details = Some(Arc::new(ErrorDetails {
            message: Some(message),
            custom_type: Some(error_type),
            problem: metadata.problem,
            failure_kind: metadata.failure_kind,
            backtrace: None,
        }));
        Ok(error)
    }
    pub fn name(&self) -> &'static str {
        "ScmSdkError"
    }
    pub fn message(&self) -> Option<&str> {
        self.details
            .as_ref()
            .and_then(|details| details.message.as_ref())
            .map(|value| value.as_str())
    }
    pub fn error_type(&self) -> Option<ErrorType> {
        if let Some(details) = &self.details
            && let Some(value) = details.custom_type
        {
            return value;
        }
        Some(match self.kind {
            ErrorKind::Configuration => ErrorType::MissingVariable,
            ErrorKind::Limit
                if self
                    .status
                    .is_some_and(|value| value == 101 || (200..300).contains(&value)) =>
            {
                ErrorType::ResponseValidation
            }
            ErrorKind::Limit | ErrorKind::Destination
                if self.remote_outcome != RemoteOutcome::NotSent || self.status.is_some() =>
            {
                ErrorType::ClientSideError
            }
            ErrorKind::Protocol
                if self.remote_outcome != RemoteOutcome::NotSent || self.status.is_some() =>
            {
                ErrorType::ClientSideError
            }
            ErrorKind::Validation | ErrorKind::Destination | ErrorKind::Limit => {
                ErrorType::UserRequestPayloadError
            }
            ErrorKind::Authentication => ErrorType::OAuthError,
            ErrorKind::Http if self.status.is_some_and(|value| value >= 500) => {
                ErrorType::ServerSideError
            }
            ErrorKind::Http | ErrorKind::Transport | ErrorKind::Io => ErrorType::ClientSideError,
            ErrorKind::Decode => ErrorType::ResponseValidation,
            _ => ErrorType::SdkError,
        })
    }
    pub fn problem(&self) -> Option<&ProblemDetails> {
        self.details
            .as_ref()
            .and_then(|details| details.problem.as_ref())
    }
    pub fn failure_kind(&self) -> Option<FailureKind> {
        self.details
            .as_ref()
            .and_then(|details| details.failure_kind)
    }
    pub fn metadata(&self) -> ErrorMetadata {
        ErrorMetadata {
            problem: self.problem().cloned(),
            failure_kind: self.failure_kind(),
            status: self.status,
            retry_after: self.retry_after,
        }
    }
    /// Opt-in native stack capture. It is accessible only through the explicit getter.
    pub fn capture_backtrace(mut self) -> Self {
        self.details_mut().backtrace = Some(Arc::new(Backtrace::force_capture()));
        self
    }
    pub fn backtrace(&self) -> Option<&Backtrace> {
        self.details
            .as_ref()
            .and_then(|details| details.backtrace.as_deref())
    }
    fn details_mut(&mut self) -> &mut ErrorDetails {
        Arc::make_mut(
            self.details
                .get_or_insert_with(|| Arc::new(ErrorDetails::default())),
        )
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
    pub(crate) fn with_failure_kind(mut self, value: FailureKind) -> Self {
        self.details_mut().failure_kind = Some(value);
        self
    }
    pub(crate) fn with_http_body(mut self, body: &[u8]) -> Self {
        self.details_mut().problem = ProblemDetails::parse(body).ok().flatten();
        self.details_mut().failure_kind = Some(FailureKind::Http);
        // Body guidance is diagnostic metadata only; transport replay remains separately authorized.
        if self.retry_after.is_none() {
            self.retry_after = body_retry_after(body);
        }
        self
    }
    pub(crate) fn with_response_metadata(
        mut self,
        status: u16,
        request_id: Option<String>,
    ) -> Self {
        self = self.with_status(status);
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
            .field("failure_kind", &self.failure_kind())
            .field("has_problem", &self.problem().is_some())
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn explicit_application_messages_and_backtraces_are_never_formatted() {
        assert!(std::mem::size_of::<Error>() <= 128);
        let metadata = ErrorMetadata::new()
            .with_status(429)
            .unwrap()
            .with_failure_kind(FailureKind::Http)
            .with_retry_after(Duration::from_millis(2));
        let error = Error::custom(
            "synthetic caller secret",
            Some(ErrorType::ClientSideError),
            metadata,
        )
        .unwrap()
        .capture_backtrace();
        assert_eq!(error.message(), Some("synthetic caller secret"));
        assert_eq!(
            error.error_type().unwrap().as_str(),
            "AISEC_CLIENT_SIDE_ERROR"
        );
        assert_eq!(error.failure_kind(), Some(FailureKind::Http));
        assert_eq!(error.metadata().status(), Some(429));
        assert!(error.backtrace().is_some());
        assert!(!format!("{error:?} {error}").contains("secret"));
        assert_eq!(error.name(), "ScmSdkError");
        assert!(Error::custom("x".repeat(8193), None, ErrorMetadata::default()).is_err());
        assert!(ErrorMetadata::new().with_status(0).is_err());
        assert_eq!(
            Error::new(ErrorKind::Http).with_status(503).error_type(),
            Some(ErrorType::ServerSideError)
        );
        assert!(
            Error::custom("message", None, ErrorMetadata::default())
                .unwrap()
                .error_type()
                .is_none()
        );
    }
}
