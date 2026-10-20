---
title: Observability
---

Enable the Cargo feature `tracing` to emit SDK lifecycle events through tracing 0.1.44. It is disabled by default. The SDK installs no subscriber, logger, filter, runtime or worker and does not change global logging or crypto configuration. The application selects a subscriber and level. Subscriber execution is synchronous; the SDK adds no background delivery. SDK events use target `scm_sdk::lifecycle` at `DEBUG`. They have an explicit root parent, so they do not inherit application span fields.

Without the feature, the private lifecycle guard is zero-sized and its methods are no-ops. A transitive dependency may already use tracing independently; disabling SDK tracing does not control another crate's instrumentation. Applications own subscriber configuration for those targets.

Applications using scoped subscribers own thread/task context propagation and subscriber lifetimes. Captured records depend on tracing filters and callsite registration; this optional telemetry is not a guaranteed audit log. The SDK does not repair or replace application subscriber configuration.

## Closed metadata contract

Every SDK lifecycle event has exactly these fields:

| Field | Values |
| --- | --- |
| `operation` | Fixed SDK phase from the table below |
| `stage` | `start`, `dispatch`, `response`, `complete`, `failure`, `closed`, `drop` |
| `kind` | `none` or a known native error-category label |
| `outcome` | `not_sent`, `rejected`, `unknown`, `success` |
| `status` | HTTP status 100–599, or 0 when unavailable/outside that range |
| `has_correlation` | Boolean presence of allowlisted correlation metadata |

The operation vocabulary is private and enum-selected:

| Operation | Meaning |
| --- | --- |
| `sdk.http.exchange` | Buffered HTTP exchange through bounded body acquisition |
| `sdk.http.stream_handshake` | HTTP stream headers and ownership handoff |
| `sdk.http.decode` | Post-response decoding/contract failure |
| `sdk.oauth.refresh` | A shared token exchange and native token validation |
| `sdk.oauth.cache` | Successful token-cache publication |
| `sdk.stream.body` | An owned response body's consumption lifetime |
| `sdk.stream.typed` | A typed SSE stream's consumption/validation lifetime |
| `sdk.realtime.upgrade` | WebSocket upgrade and ownership handoff |
| `sdk.realtime.session` | The owned socket lifetime through clean close or failure |

Product method names and `RawRequest::named` labels are never copied into tracing fields. The latter can be caller-controlled even though their Rust type is `&'static str`. Events contain no URL, host, path, query, header name/value, body, secret, model, message, event payload, correlation ID value or formatted external error. Status, category and outcome are recorded individually without formatting `Error` or transport objects.

## Lifetime and uncertainty

Events begin when the corresponding future is polled. An unpolled future creates no lifecycle. Each started lifecycle attempts one terminal event through the application's subscriber; explicit completion/failure/close prevents a duplicate terminal event on drop. Dropping a future in the concurrency queue records `not_sent`; dropping it after transport dispatch records `unknown`. Cancellation, deadlines, successful-response decoding failures, body release and realtime failure retain their scoped status and correlation-presence metadata. A cancelled partial write is never replayed by instrumentation.

HTTP `complete` means bounded HTTP acquisition or handshake succeeded. It does not claim that a typed decode succeeded, that an inference provider is ready, that remote business work completed, or that billing is settled. A later decoding or typed-stream failure has its own phase. Explicit body `closed` indicates local release with unknown remote outcome. A clean realtime `complete` means peer close 1000 was observed or the bounded close handshake completed.

Retries can emit multiple dispatch/response stages within one exchange. State resets before each dispatch, so an earlier response status is not attributed to a later unresolved attempt. Cached token reuse emits no additional exchange. Shared refresh emits one refresh lifecycle; successful cache publication is recorded after the cache lock is released. No subscriber callback executes while that lock is held. Observer work remains application code.

## Evidence

The private module tests use a locally scoped manual tracing subscriber without extra subscriber dependencies or global installation. They assert the complete field and value allowlists, adversarial URL/query/header/body/model/message/operation/correlation echoes, HTTP rejection and uncertainty, decoding errors, queued/in-flight future drop, deadlines, OAuth deduplication/cache publication, raw-body EOF/close/cancel/drop, typed SSE validation, and realtime upgrade/close/error/drop. Feature-disabled tests assert that the lifecycle has no runtime state. Existing engine/auth/stream/realtime contracts remain the behavioral reference; tracing does not alter authentication, replay, destination admission or ownership.

Qualification reproduced a first-callsite registration race in tracing-core
0.1.36 when an unobserved sibling test first reached the SDK event while a sole
scoped capture was registering. The fixture now retains one inert dispatcher
with an OFF level hint before creating captures; it never installs a global
subscriber. The deterministic regression starts with a subscriberless worker,
checks that worker cannot write into the capture, and then checks the observed
lifecycle. The [pinned callsite implementation](https://docs.rs/tracing-core/0.1.36/src/tracing_core/callsite.rs.html)
explains the single-dispatcher path; production instrumentation remains the
standard tracing macro. Repeated full-library processes separately qualify the
fixture under parallel test scheduling.

The retained [100-process receipt](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/docs/checks/phase-08-tracing-stress-1.99.0.json)
has a committed reproducer. From the repository root, create `.local/checks`
and choose an unused receipt filename:

```sh
python3 scripts/check-tracing-stress.py --toolchain 1.99.0 \
  --processes 100 --test-threads 16 --expected-tests 201 \
  --timeout 60 --build-timeout 900 --total-timeout 900 \
  --output .local/checks/tracing-stress-reproduction.json
```

The runner builds the all-feature library test executable once with Cargo JSON
messages, then executes that exact binary in 100 separate processes. Every
process must report all 201 tests passing with no ignored or filtered tests;
the first failure, timeout or count mismatch stops the run. The explicit
expected count must be reviewed when library tests change. The receipt records
the exact compiler release and host, command, bounded budgets, per-process
counts, source and runtime fingerprints, and binary hash; source, runtime and
binary must remain unchanged. It creates a new private receipt and does not
overwrite earlier evidence or load service registrations.

The phase gate uses `--replace-report` only with the exact repository path
`docs/checks/phase-08-tracing-stress-1.99.0.json`. This explicit mode atomically
replaces that versioned report through a new 0600 file and refuses symlink or
nonregular destinations; arbitrary source or config paths cannot be replaced.
It requires directory-relative filesystem primitives on the gate host. Ordinary
reproduction commands retain the default create-new behavior.

This is bounded native scheduler evidence, not proof of every interleaving or
another platform. Source hashing excludes review/check/status bookkeeping by
the same convention as the phase review; repository writers and the local
compiler remain trusted. The deterministic registration regression also runs
in the ordinary suite. Synthetic runner policy tests are reproducible with
`python3 -m unittest tools/test_auxiliary_evidence.py`.
