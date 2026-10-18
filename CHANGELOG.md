# Changelog

## 0.2.0 — 2026-10-18

Release of the pinned Prisma AIRS Red Team dataplane OpenAPI 0.93.0 at
`afef9d08030a56e18a04ea078f589313f1fa2b0c`. The
[registry receipt](docs/checks/release-registry-0.2.0.json) identifies the verified
publication artifact and source. The G10 patch remains available as
[scm-sdk 0.1.1](https://crates.io/crates/scm-sdk/0.1.1).

### Added

- All 47 current dataplane operations now have native methods and independent
  executable wire fixtures. Eleven additions cover runtime profile association,
  static/dynamic ASR, report regeneration/status, four verdict override routes,
  raw CSV error-log download and typed V2 report download locations.
- Generate 152 vendor component DTOs under `airs::red_team::models::data_plane`,
  with unique new names also available from `models`. Add 55 optional typed
  fields to existing models, including audio, progress, verdict provenance and
  rich dynamic remediation categories. Existing field types and union identities remain.
- Required nullable override verdicts distinguish true, false and explicit null
  clearing. All six new writes preserve uncertain outcomes and never auto-replay.
- The packaged `red_team_dataplane` example validates an override offline and
  demonstrates application-owned async report calls without automatically
  following signed locations or polling a regeneration job.

### Changed — migration from 0.1.x

Current dataplane calls validate against the pinned vendor contract, while legacy
standalone schema names retain their frozen meaning. The following twelve option
bindings are absent from the current vendor specification. Supplying any now
returns a validation error **before authentication or transport**; they are never
silently discarded. Leave these fields unset and explicitly adapt any application
filtering or presentation pagination to the results the service actually returns.

| Native method | Option rejected when supplied |
| --- | --- |
| `reports().list_attacks` | `search` |
| `reports().list_goal_streams` | `skip` |
| `reports().list_goal_streams` | `limit` |
| `reports().list_goal_streams` | `search` |
| `custom_attack_reports().get_prompt_sets` | `search` |
| `custom_attack_reports().get_prompts_by_set` | `search` |
| `custom_attack_reports().list_custom_attacks` | `search` |
| `get_error_logs` | `search` |
| `get_target_profile_error_logs` | `skip` |
| `get_target_profile_error_logs` | `search` |
| `get_scan_statistics` | `date_range` |
| `get_scan_statistics` | `target_id` |

Other declared filters, including deprecated `threat`, remain supported; false
and zero stay on wire when supplied. Existing V1 `download_report` still returns
JSON. Typed `get_download_url` is a separate V2 call, requires a known `FileFormat`,
and never fetches its returned URL. `download_error_logs` returns `Bytes`, without
UTF-8 replacement or file writes. Regeneration returns its HTTP 202 background
job directly and preserves HTTP 409 conflicts without polling.

Strict vendor response roots are retained. Exactly four response-only presence
waivers permit older deployments to omit `audio_technique` on attack/multi-turn
attack details and `judge_reasoning` on attack/multi-turn outputs. No new endpoint
gets a requiredness waiver. Unknown response states/fields remain explicit.

### Fixed and retained

- Gate the Gateway-only buffered-alternative unit test on `gateway`; the published
  0.1.1 source's isolated Red Team unit-test compile issue does not affect ordinary
  Red Team consumer library builds.
- Rust MSRV remains 1.89; production dependencies and Cargo features are unchanged.
- The G10 default response policy, opt-in omission policy, operation names and
  complete [23-binding default inventory](docs/RESPONSE_DEFAULTS.md) are retained.
- Vendor contract coverage is separate from the frozen 448-method/3,820-capability
  TypeScript denominator and from live acceptance. New Red Team calls remain
  live-unverified; the selected tenant's read probes return permission failures.

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
