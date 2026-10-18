# Changelog

## 0.1.1 — 2026-10-18

Published on [crates.io](https://crates.io/crates/scm-sdk/0.1.1) from reviewed source
`7efd1372806bd1969663993de3083d393b20cc62`; archive SHA-256:
`8339891e9a43582e26573355e65d95605f6865ea434365a649b215c69b01d154`.
The [registry receipt](docs/checks/release-registry-0.1.1.json) records the downloaded
artifact and both clean compiler qualifications. Post-publication documentation
and development guardrails preserve the source tag and published crate contents.

### Added

- Packaged offline example `examples/response_defaults.rs`, exercising both
  response policies and the new public methods without credentials or network IO.
- `RequestOptions::without_response_defaults()` and
  `RequestOptions::response_defaults()`: per-operation opt-in that validates typed
  JSON responses (buffered contracts and Gateway SSE contract events) without
  inserting frozen schema defaults, so consumers can distinguish a server value
  from a default such as a Red Team target's `ws_response_timeout: 110` or
  `basic_auth_location: "HEADER"`. Acceptance is identical to the default policy;
  operation names, retries and pagination are unchanged.
- `validation::Mode::ResponseWithoutDefaults` (on the non-exhaustive `Mode`) for
  direct validation and generated `parse`/`build_validated` calls.

### Fixed

- Bind live-qualification receipt versions to the crate manifest so new patch
  versions are verified without accepting stale SDK receipts.
- Preserve identical validator copy-budget accounting when omitted response
  defaults are suppressed, including rejection at the same resource limits.
- Accept EOF or a connection reset as transport closure in the cancellation
  regression test; unexpected bytes, other errors and timeouts still fail.

### Documentation

- Update the packaged README from the published 0.1.0 snapshot and document the
  23 schema-default bindings, response policy, current release and provenance.
  Production dependencies, feature defaults and Rust 1.89 support are unchanged.

Default behaviour is unchanged: responses still receive the source SDK defaults.
See [VALIDATION.md](docs/VALIDATION.md#responses-without-inserted-defaults).

## 0.1.0 — 2026-10-17

Initial native Rust implementation of the frozen Prisma AIRS TypeScript SDK
0.34.0 (`4a80dbdb3dc14365e1c25d838ac0bd139485d48c`), covering all **448 concrete
methods** and **3,820 exported capabilities** through explicit native interfaces
and documented language adaptations. Published on
[crates.io](https://crates.io/crates/scm-sdk/0.1.0) from source commit
`4f65dc48686a3b701ed0585812fcca46859f7e5a`; archive SHA-256:
`6d555d955c3734dd5ee78f7173569bdecd4426095b2ffedc4b77d8f0f2c2e8d9`.
The [publication receipt](docs/checks/release-registry-0.1.0.json) records provenance.

### Added

- Async scanning, management, canonical DLP/IAM, Model Security, Red Team, Gateway
  control/telemetry/runtime, experimental AgentGuard and realtime, and structured
  provisioning/recovery workflows.
- Native typed models, bounded validation/normalization, unknown response data
  preservation, omission/null/value types, finite numbers, binary inputs,
  multipart, SSE and bounded pagination with partial/resumable collections.
- Isolated explicit configuration/environment loading, immutable OAuth identity,
  deduplicated cancellable token refresh, configurable buffers and bounded
  metadata subscriptions, exact-byte scan HMAC and distinct runtime credentials.
- Injected transports and destination policy, TLS/redirect checks, bounded
  resources/deadlines, safe read retries, and explicit uncertain mutation outcomes.
- Sanitized native errors/problem metadata and optional metadata-only tracing.
  Consumers own runtimes and subscribers; the packaged crate needs no Node.
- Complete source-bound operation/export ledgers, independent frozen Zod/wire
  oracles, compiled external public probes, extracted-archive verification and
  enforced dependency advisory/license/source checks.

### Qualification and compatibility

Rust minimum: **1.89**. Current pinned verification also uses **1.99.0**.
[Phase records](docs/phase-status.json) retain actual Claude Code scores, accepted
source fingerprints, and passing checks. [Release evidence](docs/RELEASE.md)
separately identifies the clean-commit archive proof.
Live acceptance uses the current [receipt](contracts/live-evidence.json) and
[per-method matrix](contracts/native-operations.json); full implementation is
separate from actual service qualification.

Linux x86_64 musl and `x86_64-unknown-linux-gnu` have retained native execution
evidence; GNU scope follows the recorded compiler/feature reports. The remaining
three declared targets have compile-only boundaries. Current remote CI outcomes are recorded in
[phase status](docs/phase-status.json).
AgentGuard and realtime remain experimental. See [stability](docs/STABILITY.md),
[platforms](docs/PLATFORMS.md), [security](docs/SECURITY.md),
[release](docs/RELEASE.md), and [live acceptance](docs/LIVE_ACCEPTANCE.md).

### JSON boundary hardening

- Preserve exact wide numbers in typed model, HTTP, SSE and realtime decoding under arbitrary-precision dependency feature unification.
- Reserved Serde protocol object keys fail with sanitized errors before coercion or outgoing writes; the exact Rust security restriction is documented in [MODELS.md](docs/MODELS.md#reserved-json-protocol-keys).
