# Caller-driven response and SSE streams

`Client::send_stream` returns a `ResponseStream` after destination admission,
authentication, concurrency admission, and response-header checks. The caller
owns the Tokio runtime and every read. No background reader, reconnect, or
automatic replay runs after the stream starts.

Raw `send_stream` defaults to the configured handshake timeout, followed by idle
and optional session bounds. Typed Gateway inference retains the frozen SDK's
total operation budget through consumption: the default is 60 seconds, and
explicit `RequestOptions` can override it. A configured stream session limit can
shorten that total. The typed deadline also covers validation and buffered
events; it never silently becomes an unlimited session.

The shared `SseStream` accepts that response, an explicit event byte limit, and
one completion policy:

| Policy | Completion evidence |
| --- | --- |
| `DoneMarker` | An exact `[DONE]` data frame, consumed without yielding a payload |
| `Responses` | JSON `type` equals `response.completed`, `response.failed`, or `response.incomplete`; yield that event and release the connection immediately |
| `Eof` | HTTP EOF after complete data frames |

An SSE event name alone cannot establish a Responses terminal. Required terminal
policies fail explicitly when HTTP ends early. An unfinished data frame fails
under every policy, including `Eof`. A failed stream retains its sanitized error.
The chat `[DONE]` sentinel takes precedence over a named `error` event, matching
the frozen parser; a Responses stream still requires its typed terminal.
Full product event-schema validation belongs to the corresponding typed client;
recognizing a framing terminator does not establish successful service execution.

The decoder verifies a single `text/event-stream` media type before reading the
payload and accepts UTF-8 with an optional UTF-8 charset parameter. It handles a
leading BOM, CR/LF/CRLF boundaries, split UTF-8 characters, comments, multiline
data, event names, persistent IDs, and numeric retry advice. Unknown SSE fields
are ignored. Retry advice is exposed as metadata and never triggers reconnection.
An ID containing NUL is ignored; an empty ID resets the stored value.

Recorded Rust adaptations include explicit EOF policy, standard ID/retry
metadata, raw empty-data events, exact media-type checks, and UTF-8 charset
validation. The shared decoder yields raw events; typed Gateway streams skip
empty-data keepalive frames, matching the frozen SDK, before applying their
concrete response schema after JSON decoding.

The event limit includes UTF-8 bytes in comments and ignored fields and one byte
per normalized line ending; CRLF counts once. It is enforced before adding bytes
to the frame. The initial BOM counts toward the bound. Decompressed HTTP bytes
have their own total response limit. Empty data lines produce a raw empty event;
explicitly requesting JSON decoding on that raw event reports a safe decode
error. Typed Gateway streams skip those frames and continue through the next
payload or terminal. Repeated empty transport chunks are bounded independently.

`SseEvent` offers explicit `event`, `data`, `id`, and `retry` accessors plus typed
`json` decoding. Ordinary event and stream formatting omits server names, IDs,
payloads, and arbitrary decoding errors. Gateway error envelopes and named error
events produce sanitized errors with the response status and an unknown remote
outcome. Returned JSON retains unknown fields through ordinary `Value` decoding;
typed response normalization follows the SDK's separate schema contract.

Cancellation and absolute session deadlines are checked before yielding buffered
events as well as while reading HTTP. Idle deadlines cover a pending body read;
the application controls time between reads. A slow consumer causes no prefetch
or event queue growth. The decoder keeps one transport chunk and one bounded
frame, yields cooperatively during long frames, and releases its response and
concurrency permit on terminal, failure, explicit close, or drop.

Independent tests exercise every two-chunk boundary in a UTF-8/BOM/CRLF fixture,
deterministic varied chunking, multiline and metadata behavior, malformed input,
event bounds, truncation, cancellation, deadlines, and resource ownership. A
loopback HTTP fixture verifies the default reqwest adapter, including release at
`[DONE]` before HTTP EOF and admission of the next request with concurrency one.
These are credential-free protocol tests, not live Gateway acceptance evidence.

## Binary file responses

`RuntimeClient::download_file` returns bounded `Bytes` from
`GET /files/{file_id}/content` without UTF-8 or JSON decoding. Structured file
identifiers are encoded as one path segment; empty identifiers and traversal are
rejected before dispatch. The shared response byte limit applies across chunks,
and failure releases the response body and concurrency permit. Independent
tests preserve the frozen SDK fixture's NUL and non-UTF-8 bytes exactly.

## Runtime replay and response metadata

Gateway runtime calls default to `RuntimeReplay::Never`, including resource GETs.
`RequestOptions::with_retries` controls safe reads outside that runtime policy;
it cannot opt a billable call into replay. Applications may explicitly configure
`RuntimeReplay::bounded(0..=5)` on a cloned runtime handle. This can duplicate
remote computation or charges after a lost response. Only transport failure or
HTTP 500/502/503/504 may retry, within the original absolute budget and bounded
backoff. HTTP 429 and authentication rejection are excluded. Successful response
headers permanently end retry eligibility, even if decoding, SSE framing or a
later body read fails. Buffered requests reuse the same immutable request bytes;
streams never reconnect or replay consumed events.

Body, handshake-error-body, framing and typed-event errors retain the known HTTP
status and an allowlisted bounded request/correlation ID. Arbitrary response
headers and payloads remain excluded from ordinary errors and formatting.
