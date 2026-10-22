# Changelog

## 0.2.4 — TypeScript 0.35.0 directional profile models

Release candidate: not yet published. Publication and remote delivery require
verified 0.2.4 registry, annotated-tag, and CI receipts.
Publication is established only by the verified
`docs/checks/release-registry-0.2.4.json` receipt; the delivery guard also requires
the annotated tag and successful CI evidence.

- Add `ContentTypeConfiguration` and `ContentTypeConfigurations`, their builders,
  and named validators `ContentTypeConfigurationSchema` and
  `ContentTypeConfigurationsSchema`: four added TypeScript exports. Independently
  optional prompt, response, tool-call and tool-response settings reuse existing
  protection types and preserve unknown direction keys.
- Add exactly five field occurrences: `AiSecurityProfile.content_type_mode`
  (`content-type-mode`), `AiSecurityProfile.content_type_configurations`
  (`content-type-configurations`),
  `ModelConfiguration.enable_full_conversation_inspection`
  (`enable-full-conversation-inspection`), and `dlp_tenant_id` on both
  `SecurityProfile` and `CreateSecurityProfileRequest`. All use `Optional<T>`;
  omitted settings stay absent and explicit false stays false.
- Create and update validate the current versioned request root before
  authentication or I/O. Newly modeled null/wrong-type values that previously
  passed as unknown extensions now fail request validation, matching upstream
  TypeScript 0.35.0. Existing standalone schema names remain frozen; select
  `typescript:0.35.0:<SchemaName>` for current validation of the six updated roots.
- Upstream 0.35.0 response schemas also reject invalid newly known values. Rust
  deliberately keeps its frozen tolerant profile response validation in both
  `Mode::Response` and `Mode::ResponseWithoutDefaults`, preserving unsupported
  values and explicit null in extensions. Representable values populate typed
  projections; explicit typed values take precedence over colliding extensions.
  The existing list default remains `[]`; disabling response defaults preserves
  omission without changing acceptance.
- Retain all existing endpoints, operation names, shared/legacy protections,
  capabilities, Rust 1.89, features and production dependency declarations. The 3,820
  capability / 448 method baseline remains scoped to TypeScript 0.34.0; this delta
  has its own additive ledger. The twelve vendor-unsupported Red Team query
  bindings retain their documented status.
- Refresh the lockfile to `rustls-webpki` 0.103.16 and `webpki-root-certs`
  1.0.10 for RUSTSEC-2026-0382 and RUSTSEC-2026-0383. Downstream applications
  retain their own lockfiles: update these transitive packages and audit the
  resolved graph; installing this crate does not replace an application lockfile.
- See the [directional profile guide](docs/DIRECTIONAL_PROFILES.md) and
  [offline example](crates/scm-sdk/examples/directional_profiles.rs). Directional
  live acceptance remains unverified. This entry does not establish a new
  published release or remote delivery.

## 0.2.3 — AIRS management 0.86.0 compatibility

Publication is established only by the verified
`docs/checks/release-registry-0.2.3.json` and
`docs/checks/release-tag-v0.2.3.json`. Until those exist and verify,
this entry describes the release candidate.

### Added and changed

- Reconciled all seven pinned Prisma AIRS specifications and 160 operations.
  Management 0.86.0 updates 42 existing operations; no existing route or SDK-only
  capability is removed. The other six audited specifications are unchanged.
- Schema-bound `TargetCreateRequestConnectionType` names `VERTEX_AI` and `N8N`;
  `TargetCreateRequestResponseMode` names `REALTIME`. Frozen TypeScript parity
  enums `TargetConnectionType` and `ResponseMode` retain `Unknown(String)` for
  these values. Existing names and wire values are unchanged.
- Added `VertexAIConnectionParams`, `VertexAIConnectionRedactParams`, and
  `MultiTurnClientSessionConfig`. The appended connection and multi-turn union
  branches may decode previously `Unknown(JsonValue)` shapes into
  `Variant5(VertexAIConnectionParams)` and `Variant3(MultiTurnClientSessionConfig)`.
  Legacy branch order and precedence remain unchanged; see the precise
  [union transition inventory](contracts/red-team-management-model-overlay.json).
- Added optional typed `basic_username`/`basic_password` projections to
  `BasicAuthAuthConfig` and `AuthConfigRequestVariant3`, and
  `audio_request_json`/`audio_curl` to `RestConnectionParams`,
  `StreamingConnectionParams`, `WebSocketConnectionParams`, and
  `ConnectionParamsRequestVariant2`. Representable values now live in typed
  fields instead of `extensions`; incompatible values remain losslessly in
  extensions. Missing, null and value remain distinct, and serialization preserves
  the wire document. An explicitly supplied typed field takes precedence over a
  colliding extension value.
- Added `TargetListOptions::profiling_status`, propagated through list, pages,
  items and list_all. Accepted filters are `NOT_STARTED`, `IN_PROGRESS`,
  `COMPLETED`, and `FAILED`; undeclared values fail before authentication or I/O.

### Migration and deliberately retained behavior

- The new vendor `status` domain is `DRAFT`/`ACTIVE`. The SDK continues to send
  caller-provided legacy `status` values unchanged rather than silently dropping
  or translating them. Migrate `FAILED`, `IN_PROGRESS`, and `COMPLETED` to
  `profiling_status` explicitly. `QUEUED` and `PARTIALLY_COMPLETE` have no declared
  replacement filter and may be rejected by the service. Response values such as
  `INIT`, `QUEUED`, and `PARTIALLY_COMPLETE` remain preserved.
- All 23 G10 default bindings remain unchanged. Default insertion is enabled
  unless `without_response_defaults()` is selected; that option preserves absent
  keys and explicit nulls without changing accepted documents.
- G25 response format tolerance, unknown response enum strings and object keys
  remain. Existing request formats, required fields/types, supported query options,
  path UUID validation, security limits and operation names remain enforced.
  Target error-log queries still accept only `limit`; `skip`/`search` are refused.
- Existing `api.apps` endpoint defaults remain. No automatic `api.sase` migration,
  dependency upgrade, MSRV change (Rust 1.89), default-feature change, or
  `serde_json/preserve_order` enablement is introduced.
- Offline fixtures and packaged consumers qualify implementation compatibility.
  Fresh read-only tenant probes authenticated successfully; all four target-list
  filter probes returned HTTP 403 and remain permission-blocked. These results
  establish neither target-list acceptance nor filtering effectiveness. The probed
  profile was `https://api.sase.paloaltonetworks.com/ai-red-teaming/mgmt`; the SDK
  default `https://api.apps.paloaltonetworks.com/ai-red-teaming/mgmt-plane` was not
  exercised by this receipt. Vertex AI,
  N8N, client-session, audio and realtime provider workflows remain experimental
  and live-unverified where credentials or compatible provider fixtures are absent.
  Historical 0.2.2 live results are not attributed to changed source.

## 0.2.2 — G25 response compatibility

This patch corrects G25 response compatibility. Publication is established only by the verified
[crates.io registry receipt](docs/checks/release-registry-0.2.2.json) and
[remote tag receipt](docs/checks/release-tag-v0.2.2.json); missing receipts mean
publication has not yet been verified. The receipts carry the exact published
source, archive checksum and publication date.

### Changed

- Both response modes now preserve strings without enforcing `uuid`, `rfc3339`,
  `datetime`, `date`, `duration`, `email`, `url`, or `regex` format checks across
  every SDK schema. String type checks remain. Request format checks are unchanged.
- Existing Red Team operations select their actual frozen TypeScript response
  schemas, retaining that baseline's presence, nullability, types and string
  bounds. Vendor-only string min/max bounds are request-only. New operations
  without a TypeScript counterpart retain vendor structure and types with the
  shared relaxed response formats and string bounds.
- Vendor-added optional typed projections preserve values they cannot represent
  in extension storage, including explicit null and unknown nested data. Omitted
  fields remain omitted. An explicit typed projection takes precedence over a raw
  extension with the same key, including every request-reachable projection model
  derived from the projection inventory in the
  [compatibility report](docs/G25_COMPATIBILITY.md). This includes `TargetMetadata`,
  `LanguageOption` and all three `JobCreateRequestJobMetadataVariant` models;
  standalone schema projections are documented separately.

### Deliberately retained

- Unknown response enum strings and object keys were already accepted; they stay
  accepted. This correction addresses formats and structural compatibility, not
  a closed response enum bug.
- The TypeScript baseline still rejects missing `category_display_name`, numeric
  `tsg_id`, and null `sub_category`. Its schema is selected exclusively rather
  than unioned with the vendor schema; vendor-permitted but TypeScript-invalid
  documents can still fail. The generated report quantifies these cases.
- Default insertion remains enabled unless callers opt into
  `RequestOptions::without_response_defaults()`. Both response modes accept the
  same documents. Required baseline fields/types/bounds, finite JSON and resource
  limits, request validation, authentication, destination admission and safe path
  encoding remain enforced. Vendor-only operations retain vendor requiredness,
  nullability, numeric/integer checks and array bounds. Frozen individual literals,
  discriminator dispatch, refinements and transforms remain as defined by the
  selected contract; only the listed string formats and vendor-only string bounds
  are relaxed. For operations without a TypeScript counterpart, no baseline
  establishes broader accepted structures or numeric domains. Structural
  cardinality and numeric domain constraints therefore remain as specified in
  the accepted plan; only the scoped string compatibility policy is relaxed.
  An unexpected out-of-domain response can still be rejected.
- All [76 caller-supplied UUID checks](docs/G25_COMPATIBILITY.md#retained-caller-uuid-checks)
  remain strict. The report lists every method and argument; accepting opaque IDs
  in responses does not make opaque request path IDs valid.
- `get_target_profile_error_logs` continues to allow only `limit`. Supplied `skip`
  and `search` are rejected, never silently dropped. CLI consumers must treat
  `--offset` and `--search` as usage errors for that call. No populated live error
  logs establish service support for these options. Other rejected query options
  are unchanged and listed in the report.
- Public API names/types, dependencies, feature defaults, Rust 1.89 minimum and
  G10 behavior remain. `serde_json/preserve_order` is not enabled. Generic model
  decode errors remain redacted to avoid exposing response data.

See [the operation and field comparison](docs/G25_COMPATIBILITY.md),
[qualification status](docs/phase-status.json), and [live evidence](contracts/historical/0.2.2-live-evidence.json).
The reported airs-cli-rs loopback suite on worf has not been run here, and consumer
adoption is not claimed. Offline compatibility coverage does not imply additional
live privileges or service acceptance.

## 0.2.1 — 2026-10-18

Published 2026-10-18 from `80baf80f0f6d90f9b382e8895083ca30fa5e8997`.
Verified crates.io archive SHA256: `624f97bbc3fba604e0317d68c04a8ef88b35c104d504b44e1a235f38bd01d0d4`.

Additive Portkey access-policy update at `a27feb9efaac3653d2d94eca5d7d210944b6e160`.
The [crates.io release](https://crates.io/crates/scm-sdk/0.2.1) is verified only by
the [registry receipt](docs/checks/release-registry-0.2.1.json); a missing receipt
means publication is not yet verified.

- Add create/list/get/update/delete with typed inputs and faithful open responses.
- Keep the explicit key-authenticated control plane separate from SCM OAuth.
- Preserve omission/null, optimistic concurrency and non-replayable writes.
- Add an executed offline example, independent schema fixtures and wire/security tests.
- Retain all existing SDK interfaces, G10 defaults, Red Team coverage, dependencies
  and Rust 1.89 minimum. No existing method contract changes.
- New calls remain experimental/live-unverified; no live policy writes were made.

See [access-policy documentation](docs/ACCESS_POLICIES.md) for scope and usage.

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
Historical 0.2.2 live acceptance uses its archived [receipt](contracts/historical/0.2.2-live-evidence.json) and
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
