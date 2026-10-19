# Prisma AIRS schema reconciliation: management 0.86.0

Authorized 2026-10-19 by the maintainer's “great, proceed” following the accepted
[seven-spec audit](research/2026-10-19-prisma-airs-openapi/README.md).
This plan executes phases 21–25 after completed phase 20. Phase 21 is contracts and
design only. No runtime adoption is claimed by a pinned source or operation map.
`docs/phase-status.json` is authoritative for the current phase and accepted reviews.

## Scope and release decision

Reconcile all seven AIRS sources at pan.dev commit
`582e30543bf2adfd8b0ee457170d22b5c380dfbe`: 160 operations, all already mapped to
native methods. Only Red Team management changes upstream bytes (0.7.67 to 0.86.0,
42 operations). The 25 Runtime, 41 Model Security, 47 Red Team data-plane and five broker
operations have unchanged active pins. There are no new management endpoints to implement.

Candidate release: **0.2.3**, subject to a real packaged old-consumer compatibility
check. `TargetListOptions` and generated public structs/enums are non-exhaustive;
additive optional fields/variants can preserve source compatibility. Preserve every
old model name, field, constructor, method, diagnostic operation name and option.
Do not add required fields to existing types or narrow old accepted payloads. If a
real unavoidable public break remains, choose 0.3.0 with explicit migration guidance
before release preparation; a patch label cannot waive compatibility. MSRV 1.89,
no new runtime dependencies, no `serde_json/preserve_order` feature requirement.

## Source authority and reproducibility

Run `python3 tools/red_team_management.py` to regenerate, or append `--check` to
verify byte-for-byte derived artifacts. It uses committed YAML pins and committed
reviewed native-binding records, never ignored `.local` files. The four audit
inputs have explicit SHA256 pins. Native source reads reject absolute, escaping
or symlinked paths; class-qualified bindings require a matching inherent impl.
The pinned js-yaml 4.3.2 parser is the existing development dependency; production consumers need no Node.

- `contracts/airs-active-sources.json`: seven explicit frozen/active source pairs,
  exact YAML hashes, canonical semantic hashes, actual vendor normalized-JSON
  hashes, adoption state and verifier/overlay ownership. `identityKind=manifest_source`
  resolves six active IDs through the manifest. Data-plane uses
  `identityKind=source_metadata`: its active identity is the existing source.json
  path plus `sourceMetadataSha256`, resolved and checked against active YAML
  path/hash. No unregistered schema namespace is used as a source ID.
- `contracts/airs-operations.json`: all 160 method/path operations with native bindings,
  parameters, body, response, servers, security and recursive reference closure.
- `contracts/vendor/red-team-management/0.86.0/operations.json`: the 42 management operations.
- `compatibility-decisions.json` in that directory: complete structural delta with
  old/new values, categories, dispositions and affected current/legacy closures.
- `contracts/airs-retained-capabilities.json`: 3820 frozen export mappings, 448 native
  methods, 23 default bindings, 14 Model Security query occurrences and SDK extras.
  Its required floor comes from immutable `contracts/compatibility/airs-0.2.2-retention.json`.
  The output is the current inventory; `floorReference` points to the immutable
  seed rather than copying its provenance. Additions change the current artifact
  and are detected by `--check`. The seed is hash-bound in the tool to audited source 0a213fe79c7f0c5758f05f53d04a5c577a4f3f4e.
  Seed `sourceLedgerSha256` records exact hashes from that commit for six native,
  default and post-baseline vendor ledgers. The Phase 21 baseline diff verifies
  those current files still equal audited bytes. Phase 21 additionally runs
  `--verify-retention-origin` to compare every recorded digest with actual ledger
  bytes even in a shallow checkout; later phase commands omit that flag and
  preserve capability subsets while allowing additions.
  Generation never rewrites this seed. Current ledgers must contain every frozen
  export/path/method/default and post-baseline capability; additions are allowed,
  removal/default changes fail even when regeneration is requested. No git history
  or network is needed in shallow CI checkouts.

The frozen TypeScript 0.34.0 commit remains
`4a80dbdb3dc14365e1c25d838ac0bd139485d48c`. Never rewrite its source entries. An
`upstreamPath` can have several revisions: select explicit source IDs, never
`usedForGeneration` or first matching filename. Data-plane 0.93.0 is active despite
the historical 0.7.75 source remaining in the frozen manifest. Management 0.86.0 is
**pinned for adoption**, with runtime still on frozen behavior until phase 22 passes.

The manifest owns the new management corpus through explicit source and
`vendorArtifacts` entries. Each derived artifact retains repository/commit/license
identity and hashes. Source ownership must reject missing, unowned, altered,
symlinked/escaping and duplicate-ID cases; it must not absorb unrelated historical
vendor trees. Preserve MIT license and notices. Existing data-plane and Portkey
verifiers retain ownership of their separate corpora.

Static native binding checks verify referenced functions exist; they do not prove
request construction or live acceptance. Phase 23 supplies executable fixtures.
Existing runtime/Model Security request leniency, generic error boundaries and
known vendor defects are retained, not silently “corrected.” Upstream deletions
create review work, never automatic SDK removal.

## Runtime and generator design (phase 22)

Use an explicit management overlay after the existing data-plane model augmentation,
not wholesale replacement of the graph by vendor schemas. Extend deterministic
Python generation; do not hand-edit generated Rust. Keep frozen node identities,
old model allocation and union branch prefixes stable. Prefer a dedicated generated
`red_team_management` model namespace for new models where needed to prevent
existing nested-name shifts. New aliases point to stable existing public types.

### Connections and union resolution

- Extend schema-bound `TargetCreateRequestConnectionType` with `VERTEX_AI`, `N8N`
  and `TargetCreateRequestResponseMode` with `REALTIME`. The frozen TypeScript
  parity constants `TargetConnectionType` and `ResponseMode` retain their old
  named variants and represent these values through `Unknown(String)`. REALTIME
  is not a provider connection type.
- Add Vertex fields `project_id`, `location`, `engine_id`, `customer_sa_email`,
  including redacted provider forms. Extend the reviewed request regex allowlist
  for the three Vertex patterns; preserve response regex bypass.
- Add explicit multi-turn `client_session` alongside stateless/stateful. When
  `type` is present use its literal discriminator; when absent preserve legacy
  first-success order: stateful when both ID fields exist, stateless for `{}`.
  The new client_session branch requires an explicit literal tag and never
  inserts its vendor default.
- Append new structural provider branches after old branches; preserve old branch
  order for ambiguous/hybrid objects. Pin prefix identity in generation checks.
  Record observable `Unknown(JsonValue)` to named-variant transitions in release
  notes and match-based consumer fixtures. Unknown strings remain open in responses.
- Add named optional audio (`audio_request_json`, `audio_curl`) and Basic Auth
  (`basic_username`, `basic_password`) fields, including redacted counterparts.
  These were already extension-map fields: use the G25 optional typed projection
  mechanism so null, unexpected types and other formerly accepted raw values
  survive extensions. Do not turn a typed convenience into a new rejection.
- Preserve direct `basic_auth_header` support. Forward masked secret echo without
  local rewriting or derivation; service-side precedence between pair/header is
  not invented by the SDK. Password, raw config and extensions remain redacted
  in Debug/tracing/validation errors.
- Reuse the data-plane audio metadata and `LanguageOption.country_code` projection.
  AudioFormat includes PCM16 for corpus/realtime, but that does not enable PCM16
  REST target selection. Preserve vendor-declared transport distinctions.

No global bypass of object/literal validation. Keep requiredness/types that frozen
TypeScript already enforces, G25 response format tolerance, response open enums,
unknown keys, and G10 no-default omission/null/value behavior. The 23 baseline
default bindings remain identical. New required Vertex fields constrain the new
explicit branch, not old variants or extension payloads.

### Five target response declarations

| Operation | Vendor schema | Compatibility binding and evidence |
|---|---|---|
| POST `/v1/target` | TargetResponseSchema | Preserve existing TargetResponse API; declared new variants |
| GET `/v1/target/{target_uuid}` | TargetRedactSchema | Redacted provider/auth branches and masked echo |
| PUT `/v1/target/{target_uuid}` | TargetSchema | Credential-bearing unredacted branches, secure formatting |
| POST `/v1/target/probe` | TargetResponseSchema | Full configuration variants and sensitive values |
| PUT `/v1/target/{target_uuid}/profile` | TargetSchema | Unredacted branches plus omission/null/value |

Current shared validation root is `s012107`/TargetResponseSchema. Use a reviewed
compatible shared superset or explicit operation-local response aliases while
retaining public TargetResponse. Every row requires both response modes, raw and
typed round-trip fixtures. Do not assume `Unknown` serde fallback can recover a
payload rejected earlier by the validator. Declared credential shapes are test
requirements, not claims that a live server returns plaintext secrets.

### Target filters

Add optional `profiling_status` to TargetListOptions and propagate it unchanged
through list, pages, items and list_all. Validate the new filter against exactly
NOT_STARTED, IN_PROGRESS, COMPLETED, FAILED. Preserve old `status` strings and wire
behavior without automatic translation, even where upstream may reject them.

| Old input/status | New recommended filter | Preservation decision |
|---|---|---|
| DRAFT, ACTIVE | status | Unchanged |
| FAILED, IN_PROGRESS, COMPLETED | profiling_status | Explicit caller migration |
| QUEUED | None declared | Keep legacy input, no invented mapping |
| PARTIALLY_COMPLETE | None declared | Preserve response value; record filter inconsistency |
| INIT (response only) | None declared | Preserve; do not map to NOT_STARTED |
| NOT_STARTED | profiling_status | New explicit support |

Phase 22 must add rustdoc on `TargetListOptions::status` and
`profiling_status`; Phase 24 must repeat the migration guidance in the changelog.
Explicitly name FAILED, QUEUED, IN_PROGRESS, COMPLETED and PARTIALLY_COMPLETE as
no longer declared upstream for `status` (only DRAFT and ACTIVE remain declared).
Legacy strings continue to be sent unchanged, so the service may reject them.
Use `profiling_status` for FAILED, IN_PROGRESS and COMPLETED; no successor filter
is declared for QUEUED or PARTIALLY_COMPLETE. Do not imply that preserving local
acceptance guarantees upstream acceptance.

Red Team data-plane request UUID guards and target error-log skip/search rejection
remain as documented in 0.2.2; this update is not a vehicle for unrelated policy changes.

## Endpoint profiles and security

Preserve all existing `api.apps.paloaltonetworks.com` defaults. Six specs advertise
`api.sase.paloaltonetworks.com`; equivalent routing is not proved by static source.
Each is a distinct explicit endpoint profile subject to caller destination admission:

| Constant | Service path |
|---|---|
| DEFAULT_MGMT_ENDPOINT | /aisec |
| DEFAULT_MODEL_SEC_DATA_ENDPOINT | /aims/data |
| DEFAULT_MODEL_SEC_MGMT_ENDPOINT | /aims/mgmt |
| DEFAULT_RED_TEAM_DATA_ENDPOINT | /ai-red-teaming/data-plane |
| DEFAULT_RED_TEAM_MGMT_ENDPOINT | /ai-red-teaming/mgmt-plane |
| DEFAULT_RED_TEAM_NETWORK_BROKER_ENDPOINT | /ai-red-teaming/data-plane/network-broker |

Runtime scan regional endpoints are outside this discrepancy. Token issuer, API
origin, tenant/region and scopes remain explicit; do not reuse bearer credentials
across redirects or automatically follow advertised host changes. Qualify selected
profiles read-only, record authorization/backend limitations distinctly, and retain
current defaults unless actual evidence supports a separately reviewed migration.

## Phases, milestones and acceptance

### Phase 21 — evidence and design

1. Register phases 21–25 without reopening completed phases. Wire full/CI command lists,
   source scopes, review/chunk checks, receipts and new-phase allowlists together.
2. Vendor exact 0.86.0 source/normalized JSON/metadata/license with exact ownership.
3. Reproduce all 160 mappings, classify every management delta, freeze retention
   inventory and source precedence, settle unions/filters/defaults/endpoint policy.
4. Run contract baseline/self-tests, management tool `--check`, focused Python tests,
   governance regression tests and `python tools/check_phase.py --phase 21`. Fresh actual Claude plan review
   must meet current rubric before runtime edits.

### Phase 22 — additive target behavior

1. Implement connection union compatibility first, including all five response
   declarations. Independent new/old/hybrid/unknown/missing-discriminator fixtures.
2. Add profiling filter and all-page propagation with unchanged legacy status wire.
   When derived management models land, update NOTICE to name pan.dev management
   0.86.0 revision 582e30543bf2adfd8b0ee457170d22b5c380dfbe alongside the existing
   data-plane revision, before any package publication.
3. Add safe typed projections, aliases, generator checks, secret-safe diagnostics;
   prove retained defaults and no-default round-trips. Update graph/model/operation
   ledgers and package-independent consumer examples.
4. Review each chunk, fix blockers, then phase review; no phase 23 advancement on a
   mere plan inventory. Extend `--require-through-phase 22` with real implementation
   artifacts/test-binding checks before allowing it to pass.

### Phase 23 — cumulative qualification

Run independent request/response/security fixtures (both modes, omission/null,
unknown enums, wrong-type extension values, raw typed round-trip, union order,
all pagination pages, pre-auth invalid request failure and credential redaction).
Regenerate graph/models/constants/native ledgers and retained defaults; compare
frozen regressions. Verify default/minimal/all/relevant features on MSRV 1.89 and
current pinned compiler, existing privacy/dependency/package policies, and real
old-consumer source compatibility. Both clean packaged consumers must build.
Keep full qualification separate from smaller CI contracts receipts. Add stronger
phase 23 artifact gates; phase 21 static evidence never qualifies this phase.

### Phase 24 — reviewed delivery

Choose patch 0.2.3 only after compatibility proof. Update changelog with every new
variant/filter/projection, schema-bound versus frozen-parity enum names, Unknown
transition, typed-field relocation out of extensions, retained validation rule and live
limitation. Explicitly document the narrowed upstream `status` domain and all five
affected legacy values listed above, with `profiling_status` migration guidance
where supported and the unchanged-wire/upstream-rejection caveat. Run controlled read-only tenant qualification with explicit credential
paths; mutations require uniquely owned fixtures, bounded cost and cleanup evidence.
Provider prerequisites unavailable remain explicit experimental/unverified status,
never implied acceptance. Before publishing: current-source full checks, complete
phase/chunk reviews, immutable release commit, clean archive and source hash proof.
Publish with owner credential outside repo, verify crates.io bytes/version, tag the
actual published source, push ordinary history and wait for terminal remote CI.
Record publication/readiness separately; readiness is not delivery completion.
Extend the release gate to the actual selected version before phase 24 may pass.

### Phase 25 — maintained drift detection

Provide offline deterministic hash/semantic comparison and an explicitly invoked
online inventory refresh against a newly pinned complete pan.dev tree. Classify
additions/removals/type/requiredness/union order/auth/server changes by source,
not version strings. Unknown changes fail classification; they never mutate the
SDK automatically. Preserve source revisions, compatibility floor and all retained
capabilities; test simulated additions, deletions, renamed components, same-version
changes and source tampering. Finish with independent review and remote CI.

## Review contract and evidence limits

Use actual Claude Code, read-only repository tools, preserved prompt/result and
exact source fingerprint. Each chunk: every category >= 8, total >= 42/50 (aim 45).
Each phase: total >= 46/50. Five attempts maximum per loop; only on attempt 5 may
strictly > 40 with every category >= 8 advance, recording every deficiency and no
blocker/high finding. Scores do not replace tests, immutable publication or CI.

Phase 21 checks concern source inventory and planning tools only. They cannot
certify runtime behavior or tenant permissions. The `--require-through-phase` values 22 through 25 intentionally fail until implementation extends their evidence requirements.

### Retention ordering decision

Export/default/operation inventory rows are identity sets; changing their listing
order does not change SDK behavior and is intentionally accepted by the retention
subset check. Union branch order is a separate runtime contract, protected by
`contracts/native-union-variants.json` and its strict generator comparison. Phase 22
adds only explicitly reviewed union tails with exact old-prefix checks. Do not
reject a reordered default inventory as if it reordered runtime union branches.

### Reviewing a new semantic delta

Every delta, including whole added or removed subtrees and annotation changes,
requires an explicit reviewed identity binding. For a delta, select its `path`, `kind`, `old`, `new` keys, omitting absent
keys. Compute `sha256(encoded(identity).encode("utf-8"))`; `encoded()` writes
sorted-key JSON with indent 2, ensure_ascii=False and one final newline. The tool
prints the old/new identity and candidate hash on rejection. Review the semantic
change and compatibility decision before adding its exact path/hash to
REVIEWED_DELTAS. Never automatically accept new delta digests. Renamed components
also bind the entire successor object to a reviewed hash and verify each old union
reference prefix, ignoring a trailing nullable branch.
