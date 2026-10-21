# Implementation phases and review gates

The completion target is full exported Prisma AIRS TypeScript SDK capability parity at version 0.34.0, commit `4a80dbdb3dc14365e1c25d838ac0bd139485d48c`, with the accepted Rust security and maintainability improvements. Other SCM products have extensible namespace ownership and enter future product scope as implemented contracts.

Every phase ends with relevant checks and a fresh Claude Code review. Preserve the prompt, result, score, reviewed source fingerprint, and fixes. A phase advances only at a score of at least 9/10 with no outstanding blocking findings. A score cannot substitute for test evidence or working implementation. Any source change between review and advancement requires a fresh review. Later phases may change earlier code; they rerun the applicable regression checks and review those cumulative changes. Earlier accepted records retain their historical source snapshots.

Run `python tools/check_phase.py --phase N` before requesting review. The gate binds the complete passing command set, source fingerprint, preserved review prompt, and recorded Claude CLI result. Check reports, review artifacts, and phase bookkeeping are excluded from source hashing to avoid circular hashes; their own hashes bind them to the accepted review. These hashes detect stale or inconsistent artifacts; they do not cryptographically authenticate the CLI process. Repository writers and the invoked reviewer tool are trusted. Reviews must actually be requested through Claude Code. Failed invocations preserve their prompt and private diagnostic output and cannot advance a phase. Claude runs with repository-confined read tools and no command, mutation, or MCP tools.

| Phase | Milestones | Acceptance evidence |
| --- | --- | --- |
| 1. Contracts and repository governance | Freeze all public exports, client methods, models, constants, fixtures, source hashes and licenses; reconcile source provenance; establish review tooling and tracked phase status. | Reproducible credential-free inventory; complete classification; hashes and notices verified; Claude score at least 9. |
| 2. Secure Rust foundation | Native workspace and crate, typed configuration/errors/secrets, caller transport, API-key HMAC, isolated OAuth lifecycle, deadlines, safe retries, limits and cancellation. | Rust checks and adversarial integration tests; independent HTTP fixtures; no credential leakage; Claude score at least 9. |
| 3. Typed models and scanning | Reproducible native models and validation; content builder; sync/async scans and report queries; representative OAuth read, stream, and binary proof. | Every schema and semantic override accounted for; independent valid/invalid fixtures; HMAC scan wire checks; generated output reproducibility; Claude score at least 9. |
| 4. Management and shared modules | Profiles, topics, apps, keys, deployments, logs, dashboard, OAuth management, DLP resources and IAM; paging and reviewed mutation outcomes. | All family methods implemented and tested against independent fixtures; typed patches and explicit collection truncation; Claude score at least 9. |
| 5. Model Security and Red Team | Scans, rules/groups, custom rules/versions, targets, attacks/reports, adapters, network broker, EULA and instance capabilities. | Complete family method mapping, pagination, CSV/binary/multipart and negative validation evidence; Claude score at least 9. |
| 6. AI Gateway | SCM configuration/admin/telemetry, runtime resource families, chat/Responses/completions/embeddings, SSE, multipart/audio/binary and public pricing. | Complete Gateway method and transport contracts, distinct credential/plane policies, bounded cancellable streams, precise live status; Claude score at least 9. |
| 7. Experimental and export closure | AgentGuard, bounded realtime, multi-step provisioning/recovery, exported helpers/constants/configuration/error equivalents, complete parity map. | Every frozen export has concrete native functionality and tests; experimental status explicit; no placeholder methods or unsupported aliases; Claude score at least 9. |
| 8. Qualification and delivery | Full contract suite, adversarial/property/parser tests, API examples, native platform/compiler checks, package/license/dependency audit, controlled live acceptance and cleanup, final documentation. | Complete parity matrix, sanitized live evidence, no unexplained gaps, honest platform labels, actual package checks, Claude score at least 9; push reviewed commits to origin. |
| 9. Vendor dataplane plan | Pin Red Team 0.93.0 source and settle all new/existing contracts and compatibility decisions. | Verifiable source identity, scoped governance checks, actual Claude plan score at least 9 before SDK implementation. |
| 10. Vendor dataplane contracts/models | Namespaced vendor validators, compatible typed model additions, schema diff and complete operation ledger. | Source and generation checks, independent model fixtures, frozen regressions; Claude score at least 9. |
| 11. Vendor dataplane operations | Implement eleven new methods and current-spec fixtures for all 47 operations. | Executable operation bindings, negative input and security/failure checks; Claude score at least 9. |
| 12. Vendor dataplane qualification | Docs/examples/changelog, CI coverage check, compiler/feature/package qualification and remote delivery. | Cumulative relevant checks, explicit live limitations, final Claude score at least 9 and ordinary remote push. |
| 13. SDK 0.2.0 release plan | Fix registry-release scope after completed Red Team source delivery; defer supplied Gateway update. | Actual Claude plan score at least 9 before release preparation. |
| 14. SDK 0.2.0 publication and delivery | Unchanged runtime proof, release guidance, clean archives, reviewed publication, registry/tag verification and remote delivery. | Claude release score at least 9 before publish; verified registry/source tag; terminal CI and recorded receipts. |
| 90. G10 maintenance plan | Review pinned G10 branch and plan compatibility fixes, source qualification and 0.1.1 publication. | Scoped plan checks and actual Claude plan score at least 9 before SDK fixes. |
| 91. G10 maintenance release | Correct response-policy accounting, prove typed omission/null/multiple pages, refresh source-bound ledgers, qualify and publish 0.1.1. | Relevant compiler/package/security/live evidence, actual Claude score at least 9, clean source/tag and verified registry artifact. |
| 92. G10 publication records | Record actual publication provenance and close documentation/inventory guard findings from the accepted source review. | Unchanged published SDK input proof, focused tooling/contract checks, fresh privacy receipt and actual Claude score at least 9. |

Maintenance identifiers 90–92 belong to the isolated 0.1.1 patch track. Follow
[the G10 release plan](g10-0.1.1-plan.md) in that order after baseline phases 1–8.
Historical baseline gates and their command sets retain their original meaning.

`docs/phase-status.json` is the progress source of truth. Live prerequisite failures are recorded separately from implementation gaps. Stable features require successful live acceptance; offline-only or upstream-blocked capabilities retain explicit experimental status. Rust dependency and generator versions, target triples, and resource defaults are recorded as foundation outputs.

## Milestone records

Each phase records three milestones: implemented scope, passing independent verification, and accepted Claude review. The detailed checklist below determines implemented scope; each completed item links to code and check evidence in the phase record.

1. **Contracts:** root export/type/schema ledger; complete client method ledger; exact source and notice ledger; deterministic extraction and tampering rejection; review enforcement.
2. **Foundation:** crate and tooling; configuration/secrets/errors; injected transport and destination admission; API-key signing and OAuth lifecycle; deadlines/retry/cancellation/bounds; adversarial tests.
3. **Models and scan:** generated known fields and open values; named semantic validation overrides; patch states; content builder; all four scanner methods; representative read/stream/binary proof.
4. **Management:** each management namespace; canonical DLP resources; canonical IAM; all-page and lookup helpers; dashboard query dialects; uncertain mutation results.
5. **Model and Red Team:** each model-security namespace; each red-team namespace; binary/CSV operations; paging/cursor dialects; all identifier/input constraints.
6. **Gateway:** data/admin namespace families; telemetry adaptation; runtime resource families; multimodal request/response formats; SSE lifecycle; public pricing and configuration helpers.
7. **Closure:** AgentGuard; realtime lifecycle; provisioning and binding recovery; standalone helper equivalents; constants/config/errors; exhaustive exported capability reconciliation.
8. **Delivery:** complete offline gates; controlled live workflows and cleanup; feature/platform/compiler evidence; packaged consumer checks; compatibility/dependency/license/security audit; documentation and reviewed remote push.
9. **Vendor plan:** exact source bytes and machine-readable pin; eleven new routes and all existing contracts; settled validation/compatibility/security policy; accepted Claude review before implementation.
10. **Vendor contracts/models:** exact pin and 47-operation recount; full schema diff/closure; namespaced validators and typed models; compatible shared additions; generation and frozen regressions.
11. **Vendor operations:** all eleven methods; all 47 current-spec executable fixtures; pre-auth validation, safe downloads, override nulls, bounded lifecycle and mutation uncertainty.
12. **Vendor delivery:** scoped CI coverage, native compiler/feature/package checks, examples/docs and explicit live status; final review and remote push. See `docs/red-team-data-plane-0.93.0-plan.md` for the detailed follow-up checklist.

## Gateway ownership

The registry release follows [the 0.2.0 delivery plan](release-0.2.0-plan.md).
Milestone 13 accepts the fixed publication plan. Milestone 14 verifies unchanged
runtime inputs, both clean archives, actual Claude review, registry payload,
immutable tag and terminal remote CI before marking delivery complete.

Phase 6 owns 194 of the 196 frozen Gateway-family methods. Two exact methods
remain visible Phase 7 obligations: `AIGatewayInferenceClient.connectRealtime`
(bounded experimental realtime lifecycle) and `AIGatewayWorkspacesClient.provision`
(multi-step creation, binding and recovery). These follow the original Phase 7
milestones. `tools/native_operations.py` records their IDs and reasons explicitly;
all other Gateway methods remain owned by Phase 6. The cumulative Phase 6 target
is 442 implemented methods, with six Phase 7 methods pending (the two workflows
and four AgentGuard methods). Method ownership does not establish live acceptance.

## Vendor Red Team dataplane follow-up

Phases 9–12 implement the separately pinned vendor dataplane 0.93.0 contract.
Read `docs/red-team-data-plane-0.93.0-plan.md` for the complete active scope,
operation inventory, model compatibility design and acceptance criteria. Phase 9
is a plan review: no SDK implementation starts until its actual Claude Code
review reaches at least 9/10 with no blocker/high findings. Phases 10–12 use
scoped relevant checks and the same review gate. Preserve phases 1–8 as historical
frozen-TypeScript parity evidence; they do not certify the newer vendor contract.

## Portkey access-policy follow-up

The authorized [0.2.1 plan](portkey-access-policies-plan.md) follows completed
0.2.0 delivery. Phase 15 pins the exact upstream delta and requires Claude plan
review >=9/10 before SDK changes. Phase 16 implements all five typed methods and
independent contract/security tests. Phase 17 qualifies both clean archives,
obtains final review >=9/10, publishes and verifies crates.io, tags and pushes,
and waits for terminal 47-job CI before completion. Historical phase 14 remains
a final phase of its completed release; it is not reopened.

## Current review rubric (maintainer instruction, 2026-10-19)

For phase 19 onward, score implementation, code quality, supportability, architecture,
and feature delivery independently from 0 to 10. Chunk acceptance requires every
category >=8 and total >=42/50 (target >=45). Phase acceptance requires >=46/50.
Use actual independent Claude Code reviews and retain each attempt, evidence and
fixes. Limit each chunk/phase loop to five attempts. After the fifth attempt only,
a total strictly above 40 may advance with every category still >=8 and every
remaining deficiency explicitly recorded. Scores do not replace passing checks or publication evidence. Historical
reviews keep their original rubric. This maintainer instruction supersedes the
earlier 9/10 threshold for current work.

### Nested review budgets and phase follow-up corrections

Each chunk and its enclosing phase has its own five-attempt budget. Once a chunk
has an accepted fifth review, corrections requested by the enclosing phase review
do not start a sixth chunk iteration. Advancement may retain that chunk's original
accepted source hash only when a fresh, current-source phase review scores at
least 46/50 (or satisfies the phase's fifth-attempt >40 exception with recorded
deficiencies), every category remains >=8, and no blocker/high finding remains. The
phase review covers the intervening corrections and all declared chunk scope.
The gate verifies the original chunk's preserved review and checks, records its
original source without rewriting it, and separately links the covering phase
review and current source hash. A missing/rejected chunk, a chunk before attempt
five, stale phase coverage, or readiness-only review cannot authorize this
follow-up path. Ordinary unchanged chunks continue
to require their own current-source acceptance.

## G25 response compatibility patch

Follow [the G25 0.2.2 plan](g25-response-compatibility-plan.md): phase 18 accepts the
comparison and plan, phase 19 implements and independently checks compatibility,
and phase 20 qualifies and publishes the immutable patch with terminal CI evidence.
Current phases use the review rubric above. Version 0.2.1 remains the
already published Portkey release. Historical phase 17 is not reopened.


## Prisma AIRS management and source reconciliation

Follow [the management 0.86.0 plan](red-team-management-0.86-plan.md) for
phases 21–25. The seven-spec audit is planning evidence; it does not qualify new
runtime code or live provider workflows. Keep all existing SDK capabilities,
including routes and options absent from upstream specs. Historical phases and
their accepted evidence remain immutable.

| Phase | Milestones | Required evidence |
| --- | --- | --- |
| 21. Source inventory and compatibility plan | Vendor exact management 0.86.0 bytes and license; reproduce the seven-source/160-operation inventory; classify deltas, retention, endpoint and public-layout decisions; register subsequent gates. | Baseline contracts/self-tests, provenance tampering tests, generation reproducibility, private-material scan, current dependency/advisory audit, baseline-to-HEAD committed hygiene check, unchanged SDK and dependency-policy inputs against `0a213fe79c7f0c5758f05f53d04a5c577a4f3f4e` (only the generated model-regressions fixture source-corpus hash may rebind), accepted current-source chunk and phase reviews. Plan score must also be >=9/10 before runtime changes. |
| 22. Target configuration and filters | Extend provider/multi-turn branches and enums; profiling filters through every page; typed Basic Auth/audio fields with preserved legacy defaults and names. | Changed-component and per-operation response fixtures, both response modes and typed round-trips, redaction and request validation tests, MSRV/current compiler regressions. |
| 23. Compatibility and security qualification | Full changed-schema matrix, old/new consumers, secrets, omission/null/defaults, bounded pagination and unchanged-family retention. | Complete inherited qualification and both compiler package consumers; no new `preserve_order`; explicit implementation/contract/live boundaries. |
| 24. Service qualification and release | Controlled available workflows, explicit unmet prerequisites, source-bound ledgers, compatible release version, reviewed clean archives, publication, tag and remote CI. | Release-specific current-source readiness guard before publication; registry/archive/tag verification; terminal successful CI and final delivery review. Historical 0.2.2 receipts cannot qualify this release. |
| 25. Drift detection | Reproducible semantic comparison against active pins, alias/reference closure, read-only upstream reporting. | Synthetic added-field/removed-route/enum changes correctly classified; no automatic API removal; separate contract, fixture and live coverage. |

Each phase declares named chunks in `docs/phase-status.json`; new chunk reviews
must match those artifact scopes exactly and preserve source-bound checks. Apply
the current five-category rubric to every chunk and enclosing phase. Phase 21 is
scoped planning/provenance work; SDK implementation starts only after its gate.
The `--require-through-phase` verification for later phases must fail closed until
their actual artifacts and checks exist. Registering commands is not completion.

`--contracts-only --verification-only` has a separate receipt for each new phase;
it can never substitute for the complete phase check list. The current workflow
routes to phase 24 contracts. Update routing, tests and documentation together
when moving to phases 22 and 23. Phase 24 must align delivery-context verification
with that workflow, select the actual release version and implement its own
readiness/delivery guards. No existing 0.2.2-specific publication helper is reused
as evidence for a different version.


### Review invocation failures and CI coverage boundary

A timed-out, failed-to-launch, nonzero-exit or invalid-output Claude invocation
writes a sanitized `phase-N[-chunk-ID]-attempt-N.failed.json` receipt alongside
the preserved prompt and check snapshot. Raw diagnostic output stays under
ignored `.local/reviews`. This is a consumed attempt, never an accepted review.
After inspecting the diagnostics and correcting the cause, rerun phase checks
and request the next numbered attempt; the five-attempt ceiling still applies. Invocation failures do not count as
substantive reviews: the fifth-attempt lower-score exception requires four
previous completed, source-bound Claude reviews. Otherwise the normal score
threshold still applies, even on attempt five.
The next attempt verifies the failed receipt's identity and preserved prompt/check
hashes. Do not delete or overwrite prior attempt artifacts. An in-flight prompt
without a completed or failed receipt is not evidence of failure: wait for the
actual invocation to finish. This distinction applies to the concurrent
plan-and-provenance review observed during Phase 21 governance review attempt 1.

If the fifth invocation fails, that chunk cannot advance and no sixth attempt is
permitted. A recovery budget requires explicit maintainer authorization for a
new chunk identity or explicitly revised scope with its own declared budget;
record that authorization and preserve all original attempts and failure receipts.
No automatic budget reset is allowed: agents must not infer this authorization
from a timeout, score, elapsed time or the general instruction to continue work.
Until authorization exists, keep the exhausted scope incomplete and continue
only independent work that does not depend on its acceptance.

The current workflow routes to phase 24 contracts and preserves its independent
48-job Rust matrix. The contracts job checks planning provenance and frozen
runtime inputs, not current live acceptance or release readiness. Local passing
records do not prove the workflow ran remotely: retain an actual terminal run
before claiming remote coverage. Historical phase 20 command sets and receipts
remain unchanged. Repoint the workflow, tests and documentation together at each
later phase; phase 24 additionally binds release delivery to exact CI contexts.


Phase 21's only SDK-file exception is
`crates/scm-sdk/src/fixtures/model-regressions.json`: manifest provenance refresh
changes its corpus `provenance.sha256`. A separate gate compares every other
JSON value, including every specimen and all remaining provenance, to the frozen
baseline and verifies the new hash against the actual corpus bytes. Runtime
Rust, validation/model output, fixture cases and generator identities remain
unchanged. The underlying `contracts/native-fixtures/schemas.json` is likewise
compared to baseline: only `provenance.manifestSha256` may change, and it must
match the current manifest bytes. Six historical capability/default/vendor
ledgers also remain frozen.


Phase 21 does not claim current-source native-export/live qualification. The
metadata-only fixture refresh changes the existing live-evidence runtime
fingerprint, so `native_exports.py --check` correctly considers the old live
qualification stale. Phase 21 retains exact baseline native/export/default/vendor
ledger bytes and verifies the immutable capability-retention floor for static
closure. It neither rebinds historical live receipts nor reruns mutating service
qualification during planning. Current-source native-export checks remain required in phases 23–24; fresh
live-evidence checks belong to Phase 24 after implementation settles. This is an
evidence-scope boundary, not a discovered runtime capability gap.

The planning metadata exception is byte-exact: only the specified 64-character
hash values may change; formatting, key order and every other byte remain frozen.
The Gateway specimen proof likewise allows only the manifest and
`tools/verify-contracts.py` input hashes to rebind to their current bytes.


New management chunks retain `scope_revisions`, ordered by `from_attempt`, with
an immutable scope for each prior attempt. The current scope equals the latest
revision. Historical record verification uses its attempt's revision and retained
prompt; phase acceptance requires the latest chunk review to cover the current
scope. Expanding scope requires a new revision and review, never rewriting prior
artifacts. Missing prior artifacts fail with a sanitized gate error. Failure to
supply substantive-review history never enables the fifth-attempt exception.

Phase 21 requires network access to refresh the RustSec advisory database and
full Git history for the baseline-to-HEAD whitespace check. `deny.toml` remains
byte-identical to the audited baseline alongside Cargo inputs; a passing audit
cannot silently authorize a relaxed dependency policy.


### Phase 22 offline qualification boundary

Archive the original current-live reference byte-for-byte as
`contracts/historical/0.2.2-live-evidence.json`; its recorded execution report is
unchanged. The 17 historically verified methods remain evidence for their old
runtime fingerprint, never acceptance of the changed runtime. Regenerate current
native operations, exports and qualification with no current live reference:
every live annotation must be unverified and current verified counts must be zero.
Phase 22's verifier rejects a current-live reference, altered history or inherited
live annotations. Fresh service qualification belongs to Phase 24. Phase 23
cumulative offline qualification must preserve this separation when activated.

### Phase 23 cumulative offline qualification

The active gate derives the management fixture corpus from the pinned vendor
contracts and verifies immutable consumer sources extracted from published 0.2.2.
Both supported compilers run the source-matched clean package wrapper; a final
receipt check follows execution and rejects stale or incomplete package evidence.
CI runs only the contracts subset alongside the independent Rust matrix.
Historical live acceptance remains archived and is not attributed to this source.
Fresh tenant acceptance and publication belong to Phase 24.

### Phase 24 publication boundary

The local gate reuses the full phase-23 compatibility matrix with fresh phase-24
receipts, native read-only tenant qualification, both schema-3 clean package
proofs, and a version-specific 0.2.3 candidate check. The release-readiness chunk
and a full release-readiness Claude review must pass before publication. A
readiness review cannot complete the phase. The delivery chunk and final phase
review additionally require verified registry bytes, remote annotated tag and
terminal successful CI on the matching source. Ordinary CI verifies sanitized
service receipts without loading credentials or making tenant requests.


## Independent TypeScript 0.35.0 maintenance follow-up (phases 26–28)

The user's explicit parity-fix instruction prioritizes this isolated maintenance
branch. Phase 26 gates its plan/provenance, phase 27 its model/wire qualification,
and phase 28 its release and delivery. It neither completes nor resets phases
24–25. Their historical fingerprints, failed attempts and fail-closed acceptance
requirements remain unchanged. See docs/typescript-035-parity-plan.md for exact
artifact boundaries, command sets and named chunks in phase-status.json. Every
chunk/phase retains the five-category rubric and five-attempt budget.

Phase 26 acceptance requires origin-verified pinned inputs, frozen baseline/SDK
bytes, hygiene and governance regressions, accepted plan-and-provenance chunk,
and a complete-phase Claude review of at least 46/50 (every category at least 8).
The per-phase plan and milestone entries are explicit in phase-status.json.

After phase 28, explicitly reselect phase 24 to resume its existing delivery gate,
then phase 25, preserving their original review attempts and acceptance rules.
This maintenance sequence does not implicitly reactivate or complete them.
