# Experimental Gateway realtime

Enable `realtime` for the explicit caller-owned WebSocket seam. Add `ws-transport`
for the native direct TCP/TLS connector. Realtime requires a separately constructed
runtime-key client; SCM OAuth, tenant headers, HTTP retries and HTTP transport
configuration are never copied into the connection. No HTTP fallback is attempted.
The crate minimum is Rust 1.89, required by the patched framing dependency.

`RuntimeClient::connect_realtime(query, realtime_options, request_options)`
validates the typed query, routing headers, limits, and both HTTP/WSS destination
policies before credential dispatch. Model names and additional string query
parameters are preserved. The factory receives a prepared request with sensitive
headers. The WebSocket upgrade follows the frozen separate protocol path and
does not add common HTTP API `service-name` or User-Agent defaults.

## Ownership and bounds

| Boundary | Default | Native ceiling |
| --- | --- | --- |
| Complete connection lifetime, including queue/auth/upgrade/events | 60 seconds | 1 day via `RequestOptions` |
| Upgrade handshake, including queue and authentication | 15 seconds | Remaining connection budget |
| Individual read/write idle wait | 30 seconds | Client stream idle policy |
| Explicit graceful close cleanup | 500 milliseconds | 5 seconds |
| Event bytes | 1 MiB | 16 MiB and client event limit |
| Buffered bytes | 4 MiB | 64 MiB, intersected with event bound |
| Requested buffered event bound | 256 | 4,096 |

These are SDK resource bounds, not vendor quotas. Native iteration pulls from the
socket and keeps no SDK event queue; the event-count option validates a bound
that the zero-queue implementation satisfies. Event and byte limits intersect
before framing and serialization. Backpressure is explicit, with one pending
send and no worker or background reader. Unknown provider events and fields remain
faithful data. Provider `error` events do not become an invented ready or successful
state. HTTP 101 alone establishes only an upgraded socket.

The connection exposes `next`, `send`, `cancel`, and consuming `into_stream`.
`split` permits simultaneous reads and writes without a hidden task. Dropping
either half immediately drops the socket and concurrency permit and wakes the
remaining owner. A failed or cancelled partial send latches uncertainty and
terminates the connection; it is never replayed. Errors remain latched through
later direct reads and sends. Only a peer close with code 1000 is clean; raw EOF,
binary events, malformed JSON/UTF-8, and oversized frames or fragments fail.

`cancel` sends close 1000 and waits for confirmation within its separate close
budget, then releases the socket and permit on either success or failure. This
cleanup budget is independent of an expired operation or cancelled session.
Dropping a connection or a cancelled close future forces immediate local release.
Local cleanup gives no provider-delivery or billing guarantee.

## Native connector and custom network integration

`NativeWebSocketConnector::new` uses direct sockets and verified TLS with system
roots. `with_additional_certificates` accepts bounded additional PEM roots;
`with_tls_config` is an explicit trusted TLS seam. A custom `WebSocketConnector`
is the integration point for proxies or another network stack. The HTTP client's
proxy and CA settings are not implicitly inherited. The SDK passes the actual
WSS destination to caller admission policy. Its default origin policy recognizes
only the WS counterpart of an explicitly admitted HTTP origin.

The native adapter manually bounds the upgrade to 64 KiB and 128 headers, checks
HTTP/1.1, status 101, critical headers, and the exact canonical accept digest, and
rejects redirects, duplicate critical headers, unsolicited extensions and
subprotocols. Bytes received after the header terminator are retained for framing.
Masking, fragmented payload bounds, and UTF-8 validation use patched
`tokio-websockets` 0.13.3. Control replies flush before more reads, and control-heavy
polls yield. The SDK neither installs a global crypto provider nor changes logging.
Dependency error or frame formatting is never forwarded to ordinary diagnostics.

The native adapter is restricted to 64-bit targets. Current native loopback
execution covers Linux x86_64 musl; other declared platforms require their own
qualification. Caller connectors remain available separately. Realtime is
experimental and has no successful live provider qualification yet.

See the [compiled example](../crates/scm-sdk/examples/realtime.rs),
[runtime contracts](GATEWAY_RUNTIME.md), [errors](ERRORS.md), and
[dependency review](DEPENDENCIES.md).
