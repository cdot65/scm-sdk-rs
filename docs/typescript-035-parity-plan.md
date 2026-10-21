# TypeScript 0.35.0 directional security-profile parity

## Scope and provenance

This independently authorized follow-up closes the model/schema additions in TypeScript
0.35.0 commit `780c985cac90144ea1884fe6cffa58b5deec9ac4`, compared with the immutable
0.34.0 inventory. It does not reset phase 25's exhausted review budget or claim
that phase 24 remote delivery has completed. Preserve all original SDK capability
ledgers and baseline source identities as historical evidence.

Pin the changed TypeScript model file, export declarations, source fixture and
regression cases with SHA-256 provenance. Never read uncommitted TypeScript files
as authoritative input. Retain Apache attribution. Changes are generated from an
explicit additive overlay, not hand-edited generated Rust.

## Milestone 1: contract and implementation

- Add public `ContentTypeConfiguration` and `ContentTypeConfigurations`, their
  builders, and named validators in the management model namespace.
- Directions are optional `prompt`, `response`, `tool-call`, and `tool-response`;
  unknown future direction keys pass through. No inferred directions or defaults.
- Add typed `content_type_mode` and `content_type_configurations` to
  `AiSecurityProfile`, `enable_full_conversation_inspection` to
  `ModelConfiguration`, and `dlp_tenant_id` to `SecurityProfile` and
  `CreateSecurityProfileRequest`. Use exact upstream wire names.
- Reuse existing data/app/model/agent-protection model types. Preserve canonical
  type names, union order, Rust 1.89, feature flags and production dependencies.
- Use the existing additive projection mechanism: absent fields stay absent,
  explicit null and unsupported response JSON remain representable, and a typed
  value deliberately takes precedence over a same-key extension. Never discard
  unexpected response values or narrow response acceptance.
- Preserve historical frozen schema names. Register versioned 0.35.0 roots for
  updated profile schemas, including dependent policy/request/response wrappers;
  expose the two newly named schemas normally. Current roots enforce upstream
  presence/type rules, while existing tolerant response paths retain their
  frozen validation and lossless typed projections.
- Profile create/update wire paths validate against current request roots, so
  new known request fields receive TypeScript 0.35.0 validation before auth/I/O.
  Explain historical standalone schema validation versus current request roots.
- Existing non-directional configuration and all endpoint methods remain. Red
  Team's 12 vendor-unsupported query bindings stay explicitly documented; this
  model update cannot establish that the service applies those options.

## Milestone 2: compatibility qualification

- Cover prompt/response/tool directions, mixed legacy/shared configuration,
  full-conversation inspection true/false/omitted, DLP tenant placement,
  unknown direction/nested keys, omission/null/unsupported response values,
  typed-over-extension precedence, and current-schema wrong-type rejection.
- Compare accepted/rejected documents with the pinned actual TypeScript schemas;
  do not infer Zod behavior solely from hand-transcribed expectations.
- Exercise actual create/update/get/list and paged response paths with local
  fixtures, including response-defaults disabled. Preserve operation names.
- Regeneration must be deterministic, with a separately counted additive parity
  ledger covering the four new TS exports and every changed field occurrence.
  Existing 3,820-capability claims remain scoped to 0.34.0.
- Run relevant generator/contract/model and management tests, MSRV/current
  compiler checks, package-consumer checks and cumulative regression gates.
  Retain source-bound results and clearly separate live acceptance. Do not reuse
  historical live receipts as evidence for changed source.

## Milestone 3: reviewed delivery

Get actual Claude Code plan review before implementation. Review implementation
and delivery under the five-category rubric: each >=8, chunks >=42 (target 45),
completion >=46, maximum five attempts per review scope. Preserve failures.
Publish an additive patch release after qualification, update CHANGELOG and Rust
site guides, and verify registry bytes and remote delivery. Determine the next
unused version immediately before preparing publication (currently 0.2.4).
Existing Forgejo write/CI limitations remain explicit; do not claim completion
from publication alone or bypass repository protections.

## Registered execution and exact artifact boundary

Phase 26 is planning/provenance (`plan-and-provenance`); phase 27 owns
`models-and-validation` and `wire-and-compatibility`; phase 28 owns
`release-readiness` and `remote-delivery`. Each has five attempts, registered in
phase-status.json. Phase 26's `check_phase.py` command is the source verifier
below; phases 27/28 register their full command sets before implementation review.
This is an independently prioritized maintenance branch. Do not rebind phase 24/25
checks, reviews or historical live receipts. Their `--require-through-phase`
checks continue to fail closed until their own acceptance requirements are met.

| Artifacts | Required treatment and owner |
| --- | --- |
| contracts/typescript.json, contracts/operations.json, contracts/export-classification.json, contracts/manifest.json, contracts/native-union-variants.json | Byte-frozen 0.34.0 inputs; compare with parent 578f2e6 |
| contracts/native-fixtures/schemas.json, crates/scm-sdk/src/fixtures/model-regressions.json | Preserve frozen specimens and existing metadata bytes; new directional corpus lives separately |
| contracts/native-models.json | Regenerate via generate_models.py; existing schema IDs/type names retain identity; two new model roots counted explicitly |
| contracts/native-exports.json, contracts/native-operations.json, contracts/native-probes/src/main.rs, crates/scm-sdk/tests/native_export_probes.rs | Regenerate with native_exports.py/native_operations.py; existing 3,820 identities and 448 methods remain; refresh actual source hashes, never claim unchanged-source live success |
| contracts/response-defaults.json | Regenerate its owner only for truthful source binding; all 23 historical default bindings and values remain unchanged; versioned cloned list default recorded separately |
| contracts/sources/typescript-0.35.0/** | Owned by verify_typescript035_source.py; fail on absent/extra/changed files, manifest alteration, wrong version/commit |
| contracts/typescript-035-parity.json and oracle outcomes | New separately counted ledger owned by typescript035_profiles.py --check and typescript035-oracle.mts --check |
| model/validation generated Rust and schema graph | Regenerate from additive overlay; runtime graph retains every historical root/node; no existing type or union identity replaced |

The four added TypeScript exports are two model types plus two named schemas.
The five field occurrences are content-type-mode, content-type-configurations,
enable-full-conversation-inspection, and dlp_tenant_id in each of two models.

### Concrete schema/projection design

The versioned prefix is `typescript:0.35.0:`. Register ModelConfigurationSchema,
AiSecurityProfileSchema, PolicySchema, SecurityProfileSchema,
CreateSecurityProfileRequestSchema and SecurityProfileListResponseSchema. The
ancestor closure is 12 nodes (objects, optional/array wrappers and one default).
Copy the list default's reviewed semantics to its new identity explicitly;
verify [] insertion and ResponseWithoutDefaults omission. No new default value.
Profile create/update both take CreateSecurityProfileRequest: repoint precisely
the two request-schema arguments in profiles.rs; response schema arguments stay
frozen. No invented UpdateSecurityProfileRequest type.

ModelConfiguration and AiSecurityProfile retain s009588/s009599 identities.
The two new types reuse existing protection types, rather than minting duplicates.
Existing .extension(key,value) callers continue to serialize projection keys when
the typed field is unset, including unsupported response values. The actual
existing generic projection serializer (tools/generate_models.py:497–506) excludes
vendorProjection keys from KNOWN and skips a matching extension only when its
typed field is set. This is already typed-wins *serialization*, not just decoding;
the original four unaugmented structs naturally lack that behavior until the
new fields are marked projections. Do not globally weaken baseline collisions.

Current request roots intentionally reject newly modeled wrong-type/null values
that the old passthrough accepted. All preexisting known-field constraints stay
identical; publish a field-by-field delta and this tightening in CHANGELOG and
STABILITY. Existing response acceptance is not narrowed. Extension-based invalid
requests may therefore fail current request validation, while valid raw extension
requests and lossless responses remain supported.

### Reproducible oracle and qualification commands

The pinned mgmt-security-profile.ts imports only `zod`; it does not require a full
TypeScript checkout. Copy this verified file to an isolated ignored oracle
workspace, resolve Zod using the existing committed tools/package-lock.json, and
run the actual file via the locked tsx tool. No uncommitted application source is
imported. The verifier checks the seven pinned files against an independently pinned
manifest digest before the oracle runs. Oracle code emits a committed corpus with
schema name, input, actual success/error and normalized output. SHA-256 bindings
cover the source manifest, lockfile, oracle code and outcomes. Tampering with a
pinned file or manifest must fail before execution. New tooling fixtures exercise
both tamper cases without changing the originals.

Phase 27 commands must include source verification; overlay/oracle --check;
generate_models.py --check; native operation/export checks; the existing frozen
schema oracle and fixture checks; the new directional model/wire tests on 1.89.0
and 1.99.0; and existing management/model/default/security regressions. Bind all
command exit codes and source fingerprints in check_phase reports before review.
Do not rewrite historical receipt paths. Register scoped ledger owners explicitly.

Phase 28 adds version-specific 0.2.4 readiness/delivery guards (never reuse the
0.2.3 guard as proof), both-compiler extracted package consumers, package/license/
dependency checks, registry artifact verification, and CI on the same released
source. Readiness requires cumulative phase 27 checks. Delivery requires the actual
registry checksum, annotated tag, matching remote source and all configured CI
contexts successful. Wire this phase into the workflow and record its exact
context set. The known Forgejo permissions/CI failures are real external blockers;
no new release or completion claim can substitute another source or unreviewed
run. Directional live acceptance stays explicitly unverified without a selected
permitted tenant workflow and current-source receipt.

### Executable maintenance dependency boundary

Phases 26→27→28 are an explicit new sequence: 26 depends on accepted phase 23,
27 on 26, and 28 on 27, plus the immutable published 0.2.3 source/checksum receipt.
The dependency relation is data in phase-status and exact-checked in
validate_predecessors; original phases keep their sequential rule. Phase 25 remains
final in its old sequence and cannot implicitly advance into this maintenance
sequence. The user-selected active phase 26 advances only to 27, then 28. Both old
blocked phases retain exactly their earlier records. No exhausted scope is renamed.
All three phases use normal list-form chunks, scope revisions and SCOPE_BOUND_PHASES.
Phase 26 now executes source verification, tamper/dependency/chunk regression checks,
the review-tool suite and historical management-phase tests before plan review.

The primary contracts/manifest.json still owns the immutable 0.34 baseline required
by AGENTS.md. The supplemental, independently digest-pinned 0.35 manifest is owned
by its verifier; that verifier is itself included in the phase source fingerprint.
It deliberately does not enter airs-active-sources.json, which belongs to the
separate vendor-management/drift work and cannot be rebound by this follow-up.
Phase 27's vendor-management regression scope uses --require-through-phase 23;
phase 24/25 acceptance stays independent and fail-closed. Phase 28 will add an
explicit version 0.2.4 delivery-verification branch in review_phase.py before its
readiness/delivery reviews; absence or mismatch of that guard blocks advancement.

### Provenance and planning gate corrections

The supplemental source includes the raw Git commit and full 699-file tree
ledger. Reuse verify-contracts.py's Git object reconstruction to prove commit,
tree, blob membership, and the complete runtime-source delta. Exactly three
runtime files differ: constants.ts (version), models/index.ts, and
models/mgmt-security-profile.ts. The canonical repository moved to Forgejo;
GitHub is an explicit mirror, matching the upstream package metadata.

The exact public upstream regression fixture is retained as evidence; its upstream
test describes it as sanitized. The SDK does not infer independent sanitization
or tenant ownership from that label. Its four identifier values are explicitly
inventoried and checked against the commit-bound public source; new identifiers
fail. The future generated oracle corpus replaces its DLP identifier with the
obvious placeholder `100`. These source records are outside the published crate
and are never used as tenant registrations or live-call inputs.

Phase 26 also checks all frozen contract bytes and all SDK source against `578f2e6`,
rejects new SDK implementation files before acceptance, runs whitespace hygiene,
and verifies real predecessor evidence plus publication-tamper regressions.
The previously concurrent chunk and phase reviews both terminated with preserved
rejected records; neither was an orphaned invocation. Future interactively interrupted Claude
invocations (KeyboardInterrupt) retain a sanitized failure receipt and consume
their attempt. Uncatchable process termination is not covered by that handler.

Document both validation directions in CHANGELOG and docs/STABILITY.md: upstream
0.35.0 response schemas reject wrong-typed newly modeled fields, while Rust keeps
its frozen tolerant response validation and preserves these values as extensions.
The new request roots reject them, matching TypeScript. This is an explicit,
intentional response-compatibility difference.

The source verifier also checks the pinned upstream Zod range against the locked
oracle dependency. The global active_followup_plan remains historical metadata
for phases 21–25; the explicit plan key on each new phase owns this sequence.
Hygiene covers tracked working/index changes and baseline-to-HEAD history;
immutable upstream source bytes are preserved verbatim.

After phase 28, the maintainer may explicitly reselect active phase 24 to resume
its existing delivery requirements, then phase 25. Preserve all original attempts
and evidence; this maintenance sequence never resets those budgets or silently
marks either older phase complete. The phase-28 release guard is an implementation
deliverable before registering its executable command set; until then the unknown
phase command error prevents obtaining valid check evidence or advancement.

The phase-26 driver now runs unconditional baseline-to-working-tree SDK comparison
and an untracked SDK file guard. They execute even with --verification-only and do
not depend on fingerprint-excluded active_phase metadata. The additional unit-test
freeze is supplementary, not the sole acceptance boundary.
