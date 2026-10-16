# Rust foundation

The foundation supplies an async HTTP client for the product clients implemented
in subsequent phases. Applications own their Tokio runtime and credential
persistence. Client construction creates no runtime and starts no background task.
Clones share their connection pool, concurrency limit, and session cancellation.

## Explicit configuration

See the [compiled foundation example](../crates/scm-sdk/examples/foundation.rs).
The crate's rustdoc includes that same source as a checked `no_run` example.

`oauth_credentials` uses the same transport, destination policy, and session for
OAuth and API traffic. The default policy admits exactly the configured API and
issuer origins. A supplied `DestinationPolicy` must admit both. A custom
`TokenProvider` supplied through `Authentication::oauth` owns its exchange policy;
the SDK cannot govern networking performed inside arbitrary caller code.

Credentials and tenant registrations are never loaded implicitly. `AuthContext`
fixes the issuer, client, credential generation, tenant, scopes, and audience.
Each provider owns an isolated in-memory cache. Concurrent waiters share a refresh;
one cancelled waiter leaves other waiters running. When the last waiter drops,
the refresh future drops. A late authentication rejection invalidates only its token generation.
`OAuthProvider::clear` prevents an older pending exchange from repopulating a
cleared cache. Rotate credentials by constructing a new context/provider/client;
cancel the old session when its in-flight work must end.

`Authentication::scan` supplies the AIRS API key and/or token. HMAC-SHA256 signs the
exact immutable JSON bytes sent. `Authentication::runtime_key` supplies the
Gateway runtime key. Authenticated redirects are rejected. Separate clients bind
different credential kinds to their configured origins. Signed transfers and
public pricing require distinct unauthenticated handling in their product clients.

Every API request includes SDK-owned `service-name: api` and
`User-Agent: PAN-AIRS/<crate-version>-rust-sdk` headers before transport dispatch,
including caller-supplied adapters. The frozen TypeScript User-Agent uses its
own package version and `-typescript-sdk` suffix; the native value deliberately
identifies Rust while retaining the vendor's `PAN-AIRS/` prefix. These defaults
cannot be overridden through raw per-request headers. They count toward the
combined header budget. The token issuer exchange has its own OAuth headers.

### Deliberate TypeScript differences

- OAuth Basic credentials are form-encoded before Base64, following
  [RFC 6749 section 2.3.1](https://www.rfc-editor.org/rfc/rfc6749#section-2.3.1).
  The frozen TypeScript client directly Base64-encodes the pair;
  reserved characters therefore produce different, standards-compliant bytes.
- TSG IDs accept 1–32 ASCII digits, matching SCM tenant identifiers. The frozen
  TypeScript constructor accepts arbitrary strings; malformed identifiers fail
  locally in Rust instead of reaching the token issuer. The 32-character ceiling
  is an SDK bound, not a claimed vendor maximum.
- Bearer tokens follow [RFC 6750 section 2.1](https://www.rfc-editor.org/rfc/rfc6750#section-2.1)'s `b64token` character set. Arbitrary
  opaque strings outside that grammar are rejected before becoming headers.
- The early-refresh buffer is currently `min(30 seconds, TTL / 5)`, which keeps
  short-lived tokens usable. The TypeScript client exposes `tokenBufferMs`;
  its configurable native equivalent remains a tracked Phase 7 parity item.
- Request deadlines cover the complete operation. Retries require declared read
  semantics, so scan submissions and ambiguous mutations do not inherit the
  TypeScript SDK's permissive retry behavior.

## Transport and destination admission

The default `ReqwestTransport` verifies TLS, disables redirects and automatic
request retries, pools connections, supports gzip, and applies SDK limits to
decompressed bytes. `ReqwestTransport::configured` supports an explicit proxy and
additional trusted PEM roots; it does not expose a certificate-verification bypass.
System proxy settings are supported by the default adapter.

Supply an `Arc<dyn Transport>` to `ClientBuilder::transport` to integrate a caller's
network stack. The SDK validates destinations and input before requesting tokens
and before calling this adapter. A custom adapter is trusted code: it must preserve
the exact body, verify TLS, reject redirects, bound headers/chunks, sanitize its
errors, and release local resources when its future or body is dropped. These
requirements also apply to any network code inside an external token provider.

Endpoints require HTTPS. `Endpoint::loopback` explicitly permits HTTP for literal
loopback addresses and localhost. Paths retain the configured base path and cannot
change the origin or traverse parent segments. Raw path strings reject encoded
separators/percent signs; `RawRequest::segment` accepts a structured identifier and
encodes it once, including literal slash/percent values. Recursive escape and dot
segments are rejected before authentication. Query values are encoded once.
Endpoints and final URLs are bounded at 16 KiB; queries have at most 256 entries.

## Budgets and cancellation

| Default | Value |
| --- | --- |
| Operation budget, including queue/auth/attempts/sleeps/body | 60 seconds |
| OAuth refresh budget | 30 seconds |
| Connection establishment | 10 seconds |
| Request / decompressed response bytes | 8 MiB each |
| Error / token response bytes | 64 KiB each |
| Concurrent API operations | 32 |
| Additional safe-read attempts | 2 |
| Retry delay | 250 ms exponential, capped at 5 seconds |
| Outgoing header count / total bytes | 128 / 16 KiB |
| Default adapter received headers | HTTP/1: 128 headers; HTTP/2: 16 KiB list |
| Stream handshake / idle | 30 seconds each |
| Stream event / future realtime queue | 1 MiB / 64 entries |

`send_bytes` buffers bounded responses. `send_stream` returns a caller-driven
body owning its concurrency permit; dropping or closing it releases the response.
The SSE decoder and typed Gateway stream enforce event and terminal contracts.
See [STREAMING.md](STREAMING.md) for raw and typed stream budget differences.
Limits are configurable within documented validation bounds in
`config::Limits::validate`; they are SDK defaults, not vendor service quotas.

`RequestOptions::with_timeout` establishes one absolute deadline. `cancel` cancels
that operation. `Session::cancel` cancels work for all clients sharing that session.
Queued work is cancellable; dropped body futures and permits are released. Local
cancellation cannot establish whether a server committed a write or charged for
inference. Inspect `Error::remote_outcome` and reconcile server state when unknown.

Only operations with declared read semantics retry transport failures and HTTP
429/502/503/504. `RawRequest::read` makes that declaration. An arbitrary
`RawRequest::new`, including an arbitrary GET, has no automatic replay. Mutations
and inference do not silently retry, including after a 401. `Retry-After` is honored
only within the retry and total budgets; longer delays are returned to the caller.
One safe-read OAuth 401/403 refresh replay is allowed within the same retry count;
a 403 with `x-opa-decision: false` is a policy denial and does not refresh.
Setting `max_retries` to zero disables that replay too; the rejected token generation
is still invalidated for later independent operations.
`RequestOptions::with_retries` overrides the per-call safe-read replay budget
within 0–5; it never makes a mutation or inference replayable.

## Data and error handling

`RawRequest` is an advanced escape hatch. Product clients add typed validation.
Raw callers own service-specific input contracts. JSON syntax, MIME types, request
size, header protection, and destination checks still run before authentication.
The combined outgoing headers are checked again after authentication adds its headers.
`json_value` uses a bounded serializer; it never signs one body and sends another.
Serialization bounds nesting at 128 before recursion reaches the native validator.
`ClientBuilder::validation_limits` supplies explicit JSON node, depth, work, and
copy limits. Defaults scale with byte configuration within fixed safety ceilings;
the request/response byte limits continue to apply. These are SDK resource limits.

Errors expose a category, status, static operation name, retry delay, remote-outcome
classification, and explicit getters for allowlisted correlation IDs and codes.
Ordinary `Debug` and `Display` omit those identifier values, server body text,
credentials, request bodies, query values, and transport error chains. Local
failures after sending and successful-response decoding failures report unknown
remote outcomes. Before dispatch, failures report `NotSent`; definitive HTTP
client rejections report `Rejected`.

Secret formatting is redacted. `Secret::expose` is an explicit access point.
Zeroization erases its final owned buffer; it cannot erase copies in a caller,
HTTP stack, allocator, operating system, or server. No foundation telemetry records
payload data. Product, realtime, and metadata-only tracing features are introduced
with their implementations in later milestones; they are not empty feature flags
in the foundation crate.

## Evidence

The authentication and engine suites use injected synthetic fixtures and real
loopback HTTP fixtures. They exercise token isolation, shared refresh cancellation,
stale 401s, exact HMAC bytes, destination rejection, gzip limits, ambiguous writes,
permit/body release, redirects, invalid JSON, deadlines, and redaction. Exact
executed compiler, feature, command, and outcome evidence lives in
`docs/checks/phase-02.json` and its linked native reports. Live service acceptance
is a separate qualification milestone.

## Phase 4 shared client additions

`Client::with_endpoint` selects an explicit additional API base under the existing
admission policy. It shares the provider, transport, session, limiter, and limits.
Configure an `OriginPolicy` that admits all intended service origins at creation;
rebinding cannot widen it. `with_tsg_header` derives the routing header exclusively
from the immutable OAuth identity. Dashboard and IAM enable this scoped header.
Canonical DLP access uses its own explicitly admitted endpoint.

Buffered decode, validation, response-limit, and body-deadline failures preserve
known HTTP status and allowlisted correlation metadata. Arbitrary upstream text
is not retained. JSON successes with empty bodies hydrate `{}`, following the
frozen engine; explicitly optional response operations return `None` instead.
Cancellation/session/deadline checks after authentication prevent dispatch when a
provider cancels the call synchronously.

`OAuthProvider::get_token`, `clear`, `is_token_expired`,
`is_token_expiring_soon`, and `token_info` provide all five frozen lifecycle
methods. One monotonic instant determines each metadata snapshot; wall-clock
expiry is informational. Custom expiring-soon duration comparisons cannot
overflow timestamps. Provider constructor buffer configuration and metadata
subscriptions remain Phase 7 capabilities.

See [pagination](PAGINATION.md) for collection bounds and partial errors, and
[DLP](DLP.md) for bounded native multipart uploads and merge-patch behavior.

## Bounded text responses

`Client::send_text` uses the same response-byte limits, retry safety, request
budget, cancellation, destination policy, and response metadata as JSON and
binary reads. It matches the frozen Fetch text decoding: UTF-8 replaces malformed
sequences and removes a leading UTF-8 BOM, independent of content type. Byte
limits count received/decompressed bytes; replacement characters can use up to
three output bytes per malformed input byte. No service credentials are attached
to URLs embedded in text or JSON responses; following those URLs is a separate,
explicit operation with its own destination and credential policy.

## Client method conventions

Typed product methods put `Option<&RequestOptions>` last. Native page/item
helpers take identifiers and filters first, then `max_pages`, then request
options. List-all helpers take their typed collection controls before request
options. This keeps cancellation, deadlines and retry selection in a consistent
position across product namespaces. These controls do not change credential kind
or destination admission.
