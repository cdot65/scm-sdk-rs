# Red Team dataplane 0.93.0 implementation plan

Status: accepted by actual Claude Code at 9/10 in Phase 9 attempt 2
(`docs/reviews/phase-09-attempt-02.json`), after the 8.5/10 attempt was revised.
SDK implementation began only after this plan gate passed. The same
threshold applies to each implementation phase. Historical phases 1–8 and their
reviews remain historical evidence for the frozen TypeScript baseline.

## Scope and source authority

Implement every operation of the vendor's Prisma AIRS Red Teaming Dataplane
OpenAPI 3.1 specification, version 0.93.0, and update the existing dataplane
contracts. The independently counted denominator is 47 method/path operations:
36 have existing native methods and 11 require new methods. Management and
network broker APIs are outside this denominator.

Pin `PaloAltoNetworks/pan.dev` revision
`afef9d08030a56e18a04ea078f589313f1fa2b0c`, path
`openapi-specs/prisma-airs-redteam/data-plane/dp-openapi.yaml`:

- Source: <https://raw.githubusercontent.com/PaloAltoNetworks/pan.dev/afef9d08030a56e18a04ea078f589313f1fa2b0c/openapi-specs/prisma-airs-redteam/data-plane/dp-openapi.yaml>.
- Retrieved 2026-10-18; 225109 bytes; SHA256
  `77f015de78f2dfe9eee944601b94b541c8f111da35f847e40a36084829451ca1`.
- Planning source bytes and machine-readable pin are preserved in
  `contracts/vendor/red-team-data-plane/0.93.0/dp-openapi.yaml` and `source.json`.
  Phase 10 must compare these exact bytes/hash with this reviewed pin and recount
  47 operations from the source; a different count or hash fails the gate.
- Record the source's MIT notice and the repository license/notice provenance.
- Add a separately named vendor source and coverage manifest. Preserve the frozen
  0.7.75 vendor source and TypeScript 0.34.0 inventory, oracle, fixtures and IDs.
- Generate a complete 47-row operation ledger with operationId, HTTP method,
  path, plane, parameters, request and successful response schema/status, native
  method, executable fixture binding, compatibility decision and live status.
  Unknown/unmapped operations fail verification. No counts are inferred from TS
  exports or handwritten completion totals.

## Public interface and all eleven new operations

Methods are async under the existing `red-team` feature and explicit OAuth data
endpoint. Every method accepts `Option<&RequestOptions>`. All path IDs are UUIDs
validated before credentials or transport. Keep the existing client families.

| HTTP method and exact path | Native method | Contract and acceptance |
| --- | --- | --- |
| PUT `/v1/scan/{job_id}/runtime-profile` | `RedTeamScansClient::set_runtime_profile` | Typed `RuntimeProfileUpdateRequest`; required 1–256 character ID; reject extra request fields; returns updated `JobResponse`. Read back using existing scan `get`; no invented GET. |
| GET `/v1/report/static/{job_id}/asr` | `RedTeamReportsClient::get_static_asr` | Typed `AsrResponse`, including category/subcategory counts and ASR in 0–100. |
| GET `/v1/report/dynamic/{job_id}/asr` | `RedTeamReportsClient::get_dynamic_asr` | Same documented response schema, without imposing undocumented cross-field arithmetic or ordering rules. |
| POST `/v1/report/{job_id}/regenerate` | `RedTeamReportsClient::regenerate` | No body; HTTP 202 typed `RegenerateResponse`; preserve background job identity and HTTP 409 conflict. No automatic polling or replay. |
| GET `/v1/report/{job_id}/status` | `RedTeamReportsClient::get_status` | Typed `ReportStatusResponse`; open `ReportState` vocabulary (ACTIVE, STALE, GENERATING, FAILED); unknown state stays explicit. |
| POST `/v1/report/static/{job_id}/attack/{attack_id}/override` | `RedTeamReportsClient::override_attack` | Typed `ThreatOverrideRequest` and `ThreatOverrideResponse`. |
| POST `/v1/report/static/{job_id}/attack-multi-turn/{attack_id}/override` | `RedTeamReportsClient::override_multi_turn_attack` | Same leaf edit schema; preserve separate route and parent IDs. |
| POST `/v1/report/dynamic/{job_id}/stream/{stream_id}/override` | `RedTeamReportsClient::override_stream` | Both job and stream IDs always sent; no stream-only substitute. |
| POST `/v1/custom-attacks/job/{job_id}/attack/{attack_id}/override` | `RedTeamCustomAttackReportsClient::override_attack` | Same typed edits; both job and attack scope required. |
| GET `/v1/error-log/job/{job_id}/download` | `RedTeamClient::download_error_logs` | Raw CSV bytes via existing bounded `send_bytes`; no JSON decoding, filename-based writes, CSV execution or invented content-type requirement. OpenAPI 200 omits a media schema; the description establishes CSV. |
| GET `/v2/report/{job_id}/download` | `RedTeamReportsClient::get_download_url` | Required typed `FileFormat`; typed `ReportDownload` (filename and short-lived download_url). Return the location only; no automatic fetch or filesystem write. |

Each override body contains 1–100 edits. Each edit requires `output_id` (UUID)
and `threat_override: Nullable<bool>`: true/false set the human verdict, explicit
null clears it, omission is invalid. Closed request objects reject unknown keys,
including extension keys. Preserve AI verdict/provenance, human override, and
nullable effective parent verdict separately. Never turn unknown/null into safe.
Do not invent duplicate-ID semantics absent from the spec.

## Existing 36 operations and model evolution

Reconcile *every* existing operation against the new spec, including path and
query types, requiredness, bounds/defaults, closed request shapes, and success
statuses. Preserve public method names/signatures and old fields unless an
explicit incompatible contract is discovered; document rather than hide one.
Do not send unsupported convenience parameters merely because an older SDK did.
Enumerate every existing parameter absent from the current spec and give it an
explicit disposition. Preserve the source-level option, but reject supplied
filters, scopes, pagination or other data-affecting parameters with a validation
error before credentials/transport. No caller filter is silently dropped. Only
a specifically reviewed cosmetic parameter may be an accepted no-op; none are
assumed here. Require a negative test per parameter and a CHANGELOG entry for
each behavior change. Keep
declared deprecated `threat` filters, correctly encode false/zero, and leave
omitted parameters omitted. Validate present query options against the pinned
schema before credentials; do not require one particular server default on wire.

Cover the full reachable schema closure and produce a machine-readable diff:

- Static/custom/dynamic job metadata: runtime_profile_id; static audio/text
  attack toggles; dynamic multi_turn_enabled; existing language, goals, limits.
- Job responses: runtime_metrics and standard/agentic progress unions, nested
  stage/ETA information, and report freshness. Preserve unfamiliar discriminants
  and additional response fields without classifying them as known/safe.
- Attack/detail/multi-turn/output/stream/custom prompt results: threat_details,
  raw and human verdict provenance, judge explanations/reasoning, audio input and
  output, transcripts/source, audio technique and format. Audio URLs are data,
  redacted in Debug, never fetched automatically. Add AUDIO as known modality.
- Goal subtype, category audio capability, target audio/text capability metadata,
  prompt-set error counts/has_errors, score-trend iso_labels, language country
  codes and updated validation error fields. Dynamic remediation has its own
  response schema with rich category objects: add an optional typed categories
  field to the existing RemediationDetail (currently preserved only as an
  extension), validate that route with DynamicRemediationResponseSchema and keep
  the existing public return type without flattening category metadata. Shared management types receive only
  compatible additions; management request validators retain their own contract.
- Enumerate all other schema and enum changes automatically, including root
  shapes and references, not just added property names. Fail on unclassified
  changes or unsupported OpenAPI validation keywords.

### Generation and compatibility design

Use the existing model/validation machinery. Add a small deterministic OpenAPI
adapter which lowers the schema keywords present in this pinned spec to the
existing validation graph. Consume a pinned canonical JSON conversion of YAML;
development tooling currently resolves `js-yaml` 4.3.2. Assert this exact version
in the plan gate and declare 4.3.2 as a direct development dependency in Phase 10.
Verify canonical JSON against YAML with that pinned parser. Rust consumers need no Node,
Python, YAML reader, schema download or extra runtime dependency.

Keep all frozen graph nodes and root semantics intact for historical oracles.
Add separate namespaced vendor roots for current dataplane preflight/decoding.
Use an explicit, generated compatibility map to add known fields to existing
non-exhaustive Rust structs, including anonymous job metadata branches. Retain
old field types and positional union variant identities. Newly introduced fields
on existing response models remain optional to decode older deployments. Requests
use strict current-spec validation. Responses are also validated before model
decoding, using current schemas with an explicit compatibility overlay only for
these newly required fields: `audio_technique` in AttackDetailResponseSchema and
AttackMultiTurnDetailResponseSchema; `judge_reasoning` in AttackOutputResponseSchema
and AttackMultiTurnOutputResponseSchema. These four fields may be omitted by an
older deployment; absence stays explicit, and no verdict or required baseline
field is waived. Preserve the unmodified strict vendor roots alongside these
response-only compatibility roots. The schema diff is authoritative: any fifth
newly required field stops generation and requires a recorded classification,
plan amendment and fresh Claude plan review; no automatic waiver extension. Test strict rejection and compatibility
acceptance of each omission. New endpoint response schemas have no such waivers.
Legacy standalone schema names continue to mean the frozen contract and are
documented as such.

Generate new native schemas/models (ASR, runtime association, override, report
freshness/download, audio, progress and provenance) in a separate generated module
and re-export them under `airs::red_team::models` without existing-name collisions.
Use optional/nullable/patch distinctions consistently, redacted Debug, extension
collision rejection, and the existing finite number/JSON resource bounds. No
manual edits to generated Rust. Ordinary existing methods use current vendor
request/response roots; a legacy TypeScript fixture is not current-spec evidence.
Unknown response enum strings are accepted explicitly; request enum validation
remains strict where the vendor declares an enum. In particular, reject
`FileFormat::Unknown` in get_download_url before credentials and transport.
Known valid wire values remain strict when represented by any shared open enum.

Preserve existing public return types and the v1 `download_report` method's JSON
return. Introduce typed V2 separately. The existing metadata management fallback
remains narrowly HTTP-422-only and uses the same absolute budget. GET quota
compatibility helpers remain explicitly outside the 47-operation denominator;
POST is the vendor operation. Preserve frozen validator/oracle regression evidence
without pretending stricter current preflight accepts older invalid requests.

## Security and failure behavior

Reuse OAuth, credential destination admission, transport, deadlines, cancellation,
bounded bodies, redacted errors/tracing and safe path segmentation. New writes do
not automatically retry, even if the vendor describes the resulting write as
idempotent. Preserve `RemoteOutcome::Unknown` after ambiguous dispatch failures.
GETs retain the existing bounded retry policy; regeneration conflicts do not
trigger silent replay. No detached polling or service job cancellation on a local
request cancellation. Signed URLs and filenames never become trusted paths or
credential-bearing destinations. Avoid returning them in ordinary formatting.

## Sequential phases and milestones

| Phase | Milestones | Gate and exit criteria |
| --- | --- | --- |
| 9 — Plan | Pin source identity; cover eleven new and all existing operations; settle compatibility, security, verification and release scope. | Review this plan with actual Claude Code. Only docs/governance change before score >=9 and no blocker/high finding. |
| 10 — Vendor contracts and models | Commit pinned source/license/canonical JSON; generate all 47 ledger rows, full schema diff/closure, namespaced validators, new typed models and additive old-model fields; update existing operations' validation/query normalization. | Exact source bytes/hash agree with Phase 9 pin; source recount is 47 or fail; generation reproducibility; unsupported-keyword rejection; independent model fixtures; frozen oracle/union/export regressions; native red-team and shared model compilation. Claude >=9. |
| 11 — Operations | Implement all eleven methods; wire fixtures for every current operation; reject invalid input before auth; cover CSV bytes, V2 URL, 202/409, overrides and failure behavior. | 47/47 executable current-spec operation bindings, exhaustive request/query/response coverage, new-route mutation uncertainty, cancellation/body limits/destination/redaction checks, existing family regression. Claude >=9. |
| 12 — Qualification and delivery | Finish docs/examples/changelog/coverage labels; integrate a small credential-free current-spec coverage check in existing CI; package and consumer qualification; final cumulative review; ordinary remote push. | Relevant generation, focused/adversarial Rust, all-feature/default/minimal and MSRV checks, native export probes and package checks pass; all 47 current operations mapped with contract fixtures. Claude >=9. Report live acceptance separately. |

Phase status and actual review/check artifacts are recorded in the existing
`docs/phase-status.json`, `docs/checks/phase-NN.json` and `docs/reviews/` mechanisms.
Extend scoped command sets for 9–12; do not rerun unrelated historical multi-hour
stress qualification at each phase. Final regression includes affected frozen
contracts, all-feature suite, feature isolation and packaged consumers at MSRV
1.89.0 and development Rust 1.99.0. Never substitute an unexecuted matrix claim.

## Verification details

- Independent fixtures originate from the pinned vendor contract and hand-reviewed
  concrete examples. Roundtrip-only fixtures are insufficient. Assert exact
  method, data-plane base, segments, queries, headers, body and returned values.
- All 47 operation bindings reference actual source methods and actual Rust test
  cases. Coverage verifier checks IDs/sets, not a hard-coded successful count, and
  rejects stale hashes, extra/missing operations and unresolved schema changes.
- Model fixtures cover every changed/new field, optional/null/required handling,
  closed objects, string/array/numeric bounds, Unicode lengths (OpenAPI code points, preserving frozen UTF-16 semantics separately), UUID/date-time,
  unions/discriminants and unknown response enums/fields. Exercise mutation of
  fixtures to prove rejection paths. Preserve older response decoding explicitly.
- For all eleven new methods, exercise invalid IDs, appropriate invalid bodies or
  query inputs, deadline/cancellation and redacted formatting. For all six new
  mutation operations, prove no replay and unknown outcome after dispatch failure.
  HTTP PUT runtime + POST regenerate + four override routes = six mutations.
- Verify override counts 0/1/100/101 and true/false/null/missing; runtime ID empty,
  Unicode/max length/too long; ASR bounds; V2 format validation; 202 success and
  409 conflict; raw CSV non-UTF8/empty/oversized content with exact byte preservation;
  FileFormat::Unknown is rejected before auth; signed URL output causes no second request and contains no inherited credentials.
- Current fixtures must satisfy vendor requiredness independently; frozen fixture
  compatibility uses the frozen contract and is labeled separately where current
  validators intentionally tighten behavior. Keep the four named response-presence waivers explicit and separate; no other
  schema relaxation is permitted to pass an old fixture.
- Live calls are optional additional evidence, never a substitute for offline
  contracts. Previously observed Red Team 403 means service access is not assumed.
  No paid scan, verdict edit, runtime association or regeneration against existing
  tenant jobs. If an owned, bounded disposable workflow is unavailable, keep these
  methods live-unverified and report that limitation without stalling delivery.

## Delivery and non-goals

Update API docs, an async example, changelog and endpoint coverage report. Preserve
the existing 448-method TypeScript parity denominator; vendor additions have their
own ledger and totals. Do not relabel baseline parity as live acceptance.

Push reviewed ordinary commits to the already authorized remote. Prepare version **0.2.0**: the per-parameter disposition ledger identifies twelve
previously accepted options which now fail before transport. These are deliberate
runtime behavior breaks even though Rust signatures remain compatible. Record
each affected method/parameter and the migration in CHANGELOG; type-level probes
alone cannot justify a patch release. Existing field types,
method signatures and union variant indices are compatibility acceptance criteria.
This request authorizes implementing and delivering the repository update;
registry publication is a separate release action and is not a completion gate.
Do not modify historical tags, publish an unreviewed archive, or expand to other
SCM product APIs during this update.
