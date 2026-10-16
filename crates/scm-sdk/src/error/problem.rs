//! Small allowlisted diagnostics. Arbitrary service text and extensions never survive.
use super::{Error, ErrorKind, Result};
use serde::Serialize;
use serde_json::Value;
use std::{fmt, time::Duration};

const WITHHELD: &str = "[withheld]";
const SAFE_TEXT: &[&str] = &[
    "Bad Request",
    "Unauthorized",
    "Forbidden",
    "Not Found",
    "Method Not Allowed",
    "Conflict",
    "Unsupported Media Type",
    "Unprocessable Entity",
    "Too Many Requests",
    "Internal Server Error",
    "Bad Gateway",
    "Service Unavailable",
    "Gateway Timeout",
    "Validation failed",
    "Invalid request payload",
    "Access denied",
    "must not be blank",
    "must not be null",
    "must not be empty",
    "is required",
    "Required request part 'json' is not present",
    "Required request part 'file' is not present",
    "Content-Type 'application/octet-stream' is not supported",
    "Content-Type 'text/plain' is not supported",
];
const SAFE_PROPERTIES: &[&str] = &[
    "name",
    "category",
    "description",
    "type",
    "classification",
    "tags",
    "file",
    "json",
    "originalFileName",
    "original_file_name",
    "regionName",
    "region_name",
    "isCaseSensitive",
    "is_case_sensitive",
    "profileType",
    "profile_type",
    "detectionRules",
    "detection_rules",
    "dataPatterns",
    "data_patterns",
    "supportedConfidenceLevels",
    "supported_confidence_levels",
];

/// Failure origin. A failed response-body read is a network failure even with a status.
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "lowercase")]
#[non_exhaustive]
pub enum FailureKind {
    Http,
    Network,
}

/// The frozen AIRS exception taxonomy, separate from the native failure category.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[non_exhaustive]
pub enum ErrorType {
    ServerSideError,
    ClientSideError,
    UserRequestPayloadError,
    MissingVariable,
    SdkError,
    OAuthError,
    ResponseValidation,
}
impl ErrorType {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::ServerSideError => "AISEC_SERVER_SIDE_ERROR",
            Self::ClientSideError => "AISEC_CLIENT_SIDE_ERROR",
            Self::UserRequestPayloadError => "AISEC_USER_REQUEST_PAYLOAD_ERROR",
            Self::MissingVariable => "AISEC_MISSING_VARIABLE",
            Self::SdkError => "AISEC_SDK_ERROR",
            Self::OAuthError => "AISEC_OAUTH_ERROR",
            Self::ResponseValidation => "AISEC_RESPONSE_VALIDATION",
        }
    }
}

#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct ProblemFieldError {
    property: &'static str,
    reason: &'static str,
}
impl ProblemFieldError {
    pub fn property(&self) -> &'static str {
        self.property
    }
    pub fn reason(&self) -> &'static str {
        self.reason
    }
}
impl fmt::Debug for ProblemFieldError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ProblemFieldError([REDACTED])")
    }
}

/// Sanitized RFC 7807 fields; no response body or unknown extension is retained.
#[derive(Clone, PartialEq, Eq, Serialize)]
pub struct ProblemDetails {
    #[serde(skip_serializing_if = "Option::is_none")]
    title: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    detail: Option<&'static str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    errors: Option<Vec<ProblemFieldError>>,
    redacted: bool,
}
impl ProblemDetails {
    /// Recognize a problem envelope and preserve only exact frozen protocol vocabulary.
    pub fn from_value(value: &Value) -> Option<Self> {
        let record = value.as_object()?;
        let title_or_detail = record.contains_key("title") || record.contains_key("detail");
        if !title_or_detail
            && (!record.contains_key("errors")
                || ["message", "error_message", "msg", "data", "error"]
                    .iter()
                    .any(|key| record.contains_key(*key)))
        {
            return None;
        }
        let mut result = Self {
            title: None,
            detail: None,
            errors: None,
            redacted: false,
        };
        if let Some(value) = record.get("title").filter(|value| !value.is_null()) {
            result.title = Some(sanitize(value, SAFE_TEXT, &mut result.redacted));
        }
        if let Some(value) = record.get("detail").filter(|value| !value.is_null()) {
            result.detail = Some(sanitize(value, SAFE_TEXT, &mut result.redacted));
        }
        if let Some(errors) = record.get("errors").and_then(Value::as_array) {
            result.errors = Some(
                errors
                    .iter()
                    .take(20)
                    .map(|entry| ProblemFieldError {
                        property: sanitize(
                            entry.get("property").unwrap_or(&Value::Null),
                            SAFE_PROPERTIES,
                            &mut result.redacted,
                        ),
                        reason: sanitize(
                            entry.get("reason").unwrap_or(&Value::Null),
                            SAFE_TEXT,
                            &mut result.redacted,
                        ),
                    })
                    .collect(),
            );
            result.redacted |= errors.len() > 20;
        } else if record.get("errors").is_some_and(|value| !value.is_null()) {
            result.redacted = true;
        }
        Some(result)
    }
    /// Parse at most 1 MiB; malformed or unrelated JSON is not a problem envelope.
    pub fn parse(bytes: &[u8]) -> Result<Option<Self>> {
        if bytes.len() > 1024 * 1024 {
            return Err(Error::new(ErrorKind::Limit));
        }
        Ok(serde_json::from_slice::<Value>(bytes)
            .ok()
            .as_ref()
            .and_then(Self::from_value))
    }
    pub fn title(&self) -> Option<&'static str> {
        self.title
    }
    pub fn detail(&self) -> Option<&'static str> {
        self.detail
    }
    pub fn errors(&self) -> Option<&[ProblemFieldError]> {
        self.errors.as_deref()
    }
    pub fn redacted(&self) -> bool {
        self.redacted
    }
}
impl fmt::Debug for ProblemDetails {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ProblemDetails")
            .field("redacted", &self.redacted)
            .field("field_count", &self.errors.as_ref().map_or(0, Vec::len))
            .finish_non_exhaustive()
    }
}
fn sanitize(value: &Value, allowed: &'static [&'static str], redacted: &mut bool) -> &'static str {
    if let Some(value) = value.as_str()
        && let Some(known) = allowed.iter().copied().find(|known| *known == value)
    {
        return known;
    }
    *redacted = true;
    WITHHELD
}

/// Checked public exception metadata, also used for application-created SDK errors.
#[derive(Clone, Default)]
pub struct ErrorMetadata {
    pub(super) problem: Option<ProblemDetails>,
    pub(super) failure_kind: Option<FailureKind>,
    pub(super) status: Option<u16>,
    pub(super) retry_after: Option<Duration>,
}
impl ErrorMetadata {
    pub fn new() -> Self {
        Self::default()
    }
    pub fn with_problem(mut self, value: ProblemDetails) -> Self {
        self.problem = Some(value);
        self
    }
    pub fn with_failure_kind(mut self, value: FailureKind) -> Self {
        self.failure_kind = Some(value);
        self
    }
    pub fn with_status(mut self, value: u16) -> Result<Self> {
        if !(100..=999).contains(&value) {
            return Err(Error::new(ErrorKind::Validation));
        }
        self.status = Some(value);
        Ok(self)
    }
    pub fn with_retry_after(mut self, value: Duration) -> Self {
        self.retry_after = Some(value);
        self
    }
    pub fn problem(&self) -> Option<&ProblemDetails> {
        self.problem.as_ref()
    }
    pub fn failure_kind(&self) -> Option<FailureKind> {
        self.failure_kind
    }
    pub fn status(&self) -> Option<u16> {
        self.status
    }
    pub fn retry_after(&self) -> Option<Duration> {
        self.retry_after
    }
}
impl fmt::Debug for ErrorMetadata {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ErrorMetadata")
            .field("status", &self.status)
            .field("failure_kind", &self.failure_kind)
            .field("retry_after", &self.retry_after)
            .field("has_problem", &self.problem.is_some())
            .finish()
    }
}

/// Parse source-compatible body guidance for metadata. It does not authorize replay.
pub(crate) fn body_retry_after(bytes: &[u8]) -> Option<Duration> {
    let body: Value = serde_json::from_slice(bytes).ok()?;
    let retry = body.get("retry_after")?.as_object()?;
    let interval = retry.get("interval")?.as_f64()?;
    if !interval.is_finite() || interval < 0.0 {
        return None;
    }
    let unit = retry.get("unit")?.as_str()?;
    if unit.len() > 16 {
        return None;
    }
    let seconds = match unit.to_ascii_lowercase().as_str() {
        "ms" | "msec" | "msecs" | "millisecond" | "milliseconds" => interval / 1000.0,
        "s" | "sec" | "secs" | "second" | "seconds" => interval,
        "m" | "min" | "mins" | "minute" | "minutes" => interval * 60.0,
        _ => return None,
    };
    Duration::try_from_secs_f64(seconds).ok()
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn allowlists_are_exact_and_unknown_text_never_survives() {
        for text in SAFE_TEXT {
            let problem = ProblemDetails::from_value(&json!({"title":text})).unwrap();
            assert_eq!(problem.title(), Some(*text));
            assert!(!problem.redacted());
        }
        let problem = ProblemDetails::from_value(&json!({"title":"echoed secret", "detail":17,
            "errors":[{"property":"signed-secret-url","reason":"prompt"}], "extension":"secret"}))
        .unwrap();
        assert_eq!(problem.title(), Some(WITHHELD));
        assert_eq!(problem.detail(), Some(WITHHELD));
        assert!(problem.redacted());
        assert_eq!(problem.errors().unwrap()[0].property(), WITHHELD);
        let encoded = serde_json::to_string(&problem).unwrap();
        assert!(!encoded.contains("secret"));
        assert!(!format!("{problem:?}").contains(WITHHELD));
        for property in SAFE_PROPERTIES {
            let problem = ProblemDetails::from_value(
                &json!({"errors":[{"property":property,"reason":"is required"}]}),
            )
            .unwrap();
            assert_eq!(problem.errors().unwrap()[0].property(), *property);
            assert!(!problem.redacted());
        }
    }
    #[test]
    fn unrelated_errors_null_and_malformed_entries_match_source() {
        for value in [
            json!(null),
            json!([]),
            json!({}),
            json!({"message":"legacy","errors":[]}),
        ] {
            assert!(ProblemDetails::from_value(&value).is_none());
        }
        let value = json!({"title":null,"detail":null,"errors":null});
        let result = ProblemDetails::from_value(&value).unwrap();
        assert_eq!(
            serde_json::to_value(&result).unwrap(),
            json!({"redacted":false})
        );
        let problem = ProblemDetails::from_value(&json!({"errors":[null, {}, 1]})).unwrap();
        assert!(problem.redacted());
        assert_eq!(problem.errors().unwrap().len(), 3);
        let problem = ProblemDetails::from_value(
            &json!({"errors":vec![json!({"property":"name","reason":"is required"}); 21]}),
        )
        .unwrap();
        assert_eq!(problem.errors().unwrap().len(), 20);
        assert!(problem.redacted());
        assert!(ProblemDetails::parse(&vec![b' '; 1024 * 1024 + 1]).is_err());
    }
    #[test]
    fn body_retry_metadata_is_bounded_and_does_not_authorize_replay() {
        for (unit, expected) in [("MS", 0.002), ("seconds", 2.0), ("MIN", 120.0)] {
            let bytes =
                serde_json::to_vec(&json!({"retry_after":{"interval":2,"unit":unit}})).unwrap();
            assert_eq!(
                body_retry_after(&bytes),
                Some(Duration::from_secs_f64(expected))
            );
        }
        for value in [
            json!({"interval":-1,"unit":"s"}),
            json!({"interval":"2","unit":"s"}),
            json!({"interval":1,"unit":"days"}),
        ] {
            assert!(
                body_retry_after(&serde_json::to_vec(&json!({"retry_after":value})).unwrap())
                    .is_none()
            );
        }
    }
}
