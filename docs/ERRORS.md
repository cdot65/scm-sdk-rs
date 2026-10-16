# Errors and sanitized diagnostics

`Error` exposes native `ErrorKind`, HTTP status, static operation, allowlisted
correlation and service codes, retry delay, and `RemoteOutcome`. The frozen AIRS
taxonomy is available separately through `error_type()` and `ErrorType::as_str()`.
Native categories remain more specific than the seven AIRS classifications.

`failure_kind()` distinguishes HTTP rejection from failure at the network
boundary. A broken response body can retain HTTP 200 and still be a network
failure. Validation, cancellation, and deadline categories remain independently
available; a status does not by itself establish the failure's origin.

## Problem details

`problem()` returns optional `ProblemDetails` with `title()`, `detail()`,
`errors()` and `redacted()`. Only exact frozen protocol and validation phrases
and known field names survive. Other text becomes `[withheld]`, unknown
extensions are discarded, and at most 20 field errors are retained. Legacy
message/error envelopes are not reinterpreted as problem documents.
`ProblemDetails::parse` bounds input to 1 MiB; transport uses the smaller
configured error-body limit, default 64 KiB. Malformed JSON is not a problem.

Retry metadata prefers a valid `Retry-After` header and then finite, nonnegative
JSON `retry_after` guidance in milliseconds, seconds, or minutes. Past HTTP
dates yield zero delay. Values outside native `Duration` are omitted. Body
guidance is diagnostic metadata: it does not enable replay or extend the
operation budget. Safe-read and explicit runtime replay policies remain separate.
Allowlisted service codes include the SCM `data.errorCode` envelope.

## Application errors and formatting

`Error::custom(message, error_type, metadata)` accepts an explicit application
message, optional AIRS classification, and checked `ErrorMetadata`. The message
is bounded to 8 KiB before copying and is erased when its last owned buffer drops.
`message()` is explicit access to that application message; ordinary native
errors use their safe `Display` description. No arbitrary service message is
automatically retained. Metadata getters are available on both types.

Ordinary `Debug` and `Display` omit messages, problem text, IDs, codes, URLs,
payloads, backtraces, and upstream error chains. Explicit data getters and
serialization of a sanitized problem are separate diagnostic access points.
Optional diagnostics are shared behind an allocation; the native error remains
small, and a regression test bounds its size to 128 bytes.

Rust `std::error::Error` and `Display` replace JavaScript exception inheritance.
`name()` identifies `ScmSdkError`; `capture_backtrace()` explicitly captures a
native backtrace available through `backtrace()`. No stack is captured by default,
no global stack settings are changed, and arbitrary upstream `cause` chains are
not retained. Applications control their own backtrace and diagnostic sinks.

After a write, stream, or realtime failure, inspect `remote_outcome()`. Local
cancellation releases local resources and cannot establish whether a server
committed work or billed inference. Reconcile remote state before retrying an
uncertain mutation.
