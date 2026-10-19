# Prisma AIRS OpenAPI audit and reconciliation plan

Date: 2026-10-19. Status: research and proposed plan; no implementation authorization inferred from this audit.

## Executive findings

Audited SDK **0.2.2**, source `0a213fe79c7f0c5758f05f53d04a5c577a4f3f4e`, against the complete, non-truncated pan.dev tree at [commit 582e30543bf2adfd8b0ee457170d22b5c380dfbe](https://github.com/PaloAltoNetworks/pan.dev/commit/582e30543bf2adfd8b0ee457170d22b5c380dfbe) (2026-10-16). Snapshot observed 2026-10-19. The frozen TypeScript compatibility baseline remains `4a80dbdb3dc14365e1c25d838ac0bd139485d48c`.

**All 160 published HTTP operations have SDK implementations. This does not mean full schema conformance or live acceptance.** Six of seven current spec files exactly match the SDK's active sources. The outstanding upstream update is **Red Team management 0.7.67 → 0.86.0**, with the same 42 routes but changed target configuration/filtering contracts. That is the priority implementation workstream.

| Product / plane | Current version | Operations mapped | Current source vs SDK active source | Action |
|---|---:|---:|---|---|
| Runtime scan | 0.0.0 | 4/4 | Identical | Preserve; document exceptions |
| Runtime management | 0.0.0 | 21/21 | Identical | Preserve; document exceptions |
| Model Security data | 0.1.0 | 18/18 | Identical | Preserve; automate drift tracking |
| Model Security management | 0.1.0 | 23/23 | Identical | Preserve; automate drift tracking |
| Red Team data | 0.93.0 | 47/47 | Identical to active 0.93.0 vendor pin | Preserve 0.2.2 response compatibility |
| Red Team management | 0.86.0 | 42/42 | Changed from 0.7.67 | Update query, request variants, typed fields |
| Red Team network broker | 1.0.0 | 5/5 | Identical | Preserve |

The catalog contains no separate AIRS Gateway or AgentGuard specification. This does not invalidate those SDK capabilities. The existing Portkey integration, AgentGuard, realtime support, DLP, IAM and other SCM features remain in scope for **preservation**, not removal. SASE/SCM/DLP sibling product specs are outside this AIRS audit's 160-operation denominator.

## Evidence and methodology

[Inventory](inventory.json) records exact source SHA256s, bytes, versions, immutable upstream URLs/commit and active baseline paths. Compared method/path pairs, operation parameters, request/success-response schema references, recursive component changes, native methods, validators and generated projections. Per-family appendices below describe comparison limits. **Fresh-checkout offline limitation:** the current management0.86.0 bytes and local research scripts are not committed in this report. Its hash, byte count and new-side schema findings were derived from the downloaded pinned GitHub snapshot; a fresh checkout must fetch that immutable source (or retain the local snapshot) to reproduce them today. Phase21 vendors the bytes and production tooling before implementation qualification. The current report is not a self-contained offline source archive. Source presence is not evidence of tenant permissions or successful live workflows.

- [Runtime operation mapping](runtime-operation-mapping.json): 25 operations.
- [Model Security operation mapping](model-security-operations.json): 41 operations; [object-field comparison](model-security-schema-comparison.json) covers 320 named properties in 60 mapped object occurrences, plus four generic error envelopes.
- [Red Team operation mapping](red-team-operations.json): 94 operations across three planes.
- [Red Team management raw structural diff](red-team-management-schema-diff.json): includes documentation and component renames as well as meaningful changes; do not count every diff entry as a product gap.

No service calls, runtime edits, package changes, test execution or releases were performed for this audit. The proposed checks below are future acceptance criteria, not claimed results.

## Priorities

### P1 — reconcile Red Team management contracts

**Highest priority:** known nested provider and multi-turn unions can reject new Vertex and `client_session` objects in both response modes as well as requests. Typed `Unknown` fallbacks run after validation and cannot rescue these objects. This is static code-path evidence, not an observed tenant failure. Full-configuration target responses are affected; list responses do not contain those nested objects.

1. Add `profiling_status` to target listing and carry it through pages/list_all. Preserve the old `status` API; upstream now separates lifecycle status from profiling status, so document the legacy behavior rather than silently changing it.
2. Extend request-side connection choices for `VERTEX_AI`, `N8N`, `REALTIME`, and the `client_session` multi-turn variant. Unknown response enum strings already work; that does not make closed request enums accept these values.
3. Add Vertex AI configuration (`project_id`, `location`, `engine_id`, `customer_sa_email`) and the missing named connection fields: `audio_request_json`, `audio_curl`, `basic_username`, `basic_password`, including redacted counterparts. Separate genuinely rejected payloads from fields that already travel in extension maps. Keep direct header-based Basic Auth and all old connection variants.
4. Reconcile new/renamed management components without deleting old public model names or exports. Existing data-plane work already projects TargetMetadata audio fields and LanguageOption country_code: reuse and verify them instead of duplicating types.
5. Preserve request/response asymmetry. Do not regenerate strict vendor response validators over the TypeScript compatibility floor or restore response UUID/date checks.

See the Red Team appendix for exact fields, affected operations and distinctions between request rejection, missing typed access and already-supported shapes.

### P2 — document existing differences; avoid unnecessary rewrites

Six published specifications use `api.sase`, while the corresponding SDK constants use `api.apps`: `DEFAULT_MGMT_ENDPOINT`, `DEFAULT_MODEL_SEC_DATA_ENDPOINT`, `DEFAULT_MODEL_SEC_MGMT_ENDPOINT`, `DEFAULT_RED_TEAM_DATA_ENDPOINT`, `DEFAULT_RED_TEAM_MGMT_ENDPOINT`, and `DEFAULT_RED_TEAM_NETWORK_BROKER_ENDPOINT` (`constants.rs:629–637`). Runtime scan is excluded from this discrepancy. This is a pre-existing discrepancy, not proof of an outage. Verify supported aliases/tenant routing and document endpoint profiles before changing defaults; never follow cross-origin redirects with bearer credentials automatically.

Runtime: retain the flat async scan batch despite a nested-array defect in OpenAPI; retain OAuth where security declarations are omitted; preserve nullable topics and richer SDK models. Record stricter UUID/resource policies separately. `Content::from_json` deliberately discards unknown content keys; the full `scan_request` path is the extension-preserving route.

Model Security: no new endpoint, declared query or useful model-field gap found. Fourteen extra SDK query-option occurrences across ten routes must remain. Existing looser request cardinality/pattern validation is a separate ergonomic opportunity; use opt-in checked builders if wanted, not silent tightening.

Red Team: retain documented data-plane request UUID policy and target-profile error-log query policy. Do not infer support for skip/search merely because other list operations accept them. Likewise, do not remove existing SDK extras simply because this catalog omits them.

## Target response schema and union compatibility matrix

The following mapping is present in the audited SDK today. The declaration distinction predates this management refresh; new credential/configuration shapes make it a necessary qualification seam. A vendor declaration is not proof an actual server returns plaintext credentials.

| Operation | Vendor success schema | Current SDK validator / public type | Required fixture coverage |
|---|---|---|---|
| POST `/v1/target` (create) | `TargetResponseSchema` | `TargetResponseSchema` → `s012107` / `TargetResponse` | Declared connection variants; preserve raw and typed payloads |
| GET `/v1/target/{target_uuid}` | `TargetRedactSchema` | Same `s012107` / `TargetResponse` | Redacted credentials and masked echo |
| PUT `/v1/target/{target_uuid}` | `TargetSchema` | Same `s012107` / `TargetResponse` | Unredacted credential-bearing shape; no secrets in diagnostics |
| POST `/v1/target/probe` | `TargetResponseSchema` | Same `s012107` / `TargetResponse` | Declared variants and sensitive returned values |
| PUT `/v1/target/{target_uuid}/profile` | `TargetSchema` | Same `s012107` / `TargetResponse` | Unredacted shape; omission/null/value preservation |

Phase22 must explicitly reconcile the shared SDK node with these three vendor declarations, including every redacted/unredacted auth/provider branch. Keep the public type and operation names. Both response modes and typed serialization are required for each row. `Unknown` model fallbacks cannot bypass a rejecting validator. Add fixtures for `{}`, explicit stateless/stateful/client_session, unknown discriminators, hybrid provider objects and existing Unknown-to-new-named-variant transitions. This is not permission to globally bypass object/literal validation.

## Target filter migration disposition

Forward existing caller-supplied `status` strings unchanged; no automatic rewriting. These are upstream semantics and may produce service rejection despite local compatibility. Explain the new explicit filter to callers.

| Old value / origin | New supported filter | Disposition |
|---|---|---|
| DRAFT, ACTIVE (old status filter) | `status` with same value | Lifecycle filter retained |
| FAILED, IN_PROGRESS, COMPLETED (old status filter) | `profiling_status` with same value | Explicit caller migration; preserve old wire behavior |
| QUEUED (old status filter and response) | No declared equivalent | Do not invent a mapping |
| PARTIALLY_COMPLETE (old status filter; still a response status) | No declared equivalent | Record vendor inconsistency: returned status absent from new filter |
| INIT (old response status, not old filter) | No declared equivalent | Keep response support; do not silently map to NOT_STARTED |
| NOT_STARTED (new response/filter) | `profiling_status=NOT_STARTED` | Add explicit support |

The new profiling filter contains exactly NOT_STARTED, IN_PROGRESS, COMPLETED, FAILED. Lack of a filter for a response status does not authorize dropping that response value or changing list results client-side.

## Proposed phased delivery

These are proposed phases **21–25**, following completed phase20; they are not registered or completed by this research task. Confirm the ledger has not advanced before registration; never overwrite another active phase. Dependencies: A → B → C → D → E. Runtime/Model Security inventory can proceed independently inside A; do not run several contract migrations concurrently.

### A / Phase21. Freeze evidence and compatibility decisions

**Milestones**

- **A.0 governance wiring:** create `docs/red-team-management-0.86-plan.md`; register proposed phases21–25 and milestones in `docs/PHASES.md` and `docs/phase-status.json`; extend `tools/check_phase.py` with each phase's source scope, artifact/check requirements and exact gates. Record actual reviews under `docs/reviews/`; run `python tools/check_phase.py --phase N` before advancing N. This is future implementation setup, not permission to alter completed phase20 evidence.
- **A.1 source provenance:** vendor exact management0.86.0 YAML, normalized JSON, upstream commit/hash and license/notice under `contracts/vendor/red-team-management/0.86.0/`, parallel to the existing data-plane pin. Add a distinct `contracts/manifest.json` source ID `prisma-airs-redteam/management/mp-openapi.yaml@0.86.0+582e30543bf2`, retaining the original frozen source entry untouched. Retain the already-supported ability for revisions to share `upstreamPath`: the verifier enforces source-ID uniqueness, not upstreamPath uniqueness. Update consumers to select the explicit active source ID rather than assuming one revision per path. Production hash/semantic-diff tooling belongs under `tools/`, not ignored `.local`.
- **A.1 ownership/licensing details:** `tools/verify-contracts.py:416–418` currently equates manifest-owned files with exactly `contracts/{fixtures,sources,licenses}`; merely adding the new vendor source entry would fail. Extend ownership validation to the explicitly registered `contracts/vendor/red-team-management/0.86.0/` root, retaining the existing three-directory checks. Enumerate every file in that root and require exact equality with declared owned entries: source YAML in `sources`, derived JSON/metadata in a new hashed `vendorArtifacts` inventory using the same tracked-file/path/hash checks. Reject missing or unowned files, escaping paths/symlinks and hash mismatches; do not solve the mismatch by dropping vendor paths from verification or globbing unrelated historical vendor trees into ownership. Existing separately verified data-plane/Portkey pins retain their verifier ownership, identified in the precedence ledger. Add license record `pan-dev-582e30543bf2` with exact repository and commit582e30543bf2adfd8b0ee457170d22b5c380dfbe and the hashed upstream license at `contracts/licenses/pan-dev-582e30543bf2.LICENSE`; bind redistributed sources/derived artifacts to it. This preserves the repository-and-commit equality required at `verify-contracts.py:316–320`. Negative cases must cover an unowned vendor file, missing file, modified bytes, mismatched license commit and duplicate source ID.
- **A.0 CI/review wiring detail:** extend `commands_for` for21–25, `ci_commands_for` and the CLI `--contracts-only` allowlists (currently only12/17/20), and generated CI/workflow routing for the applicable phases. Use phase20's reuse of prior complete command sets as the precedent. Preserve separate contracts-only receipts and `review_phase.check_evidence` requiring the full `commands_for` list; a fast CI subset must never qualify a phase review. Add cases proving subset receipts cannot satisfy complete qualification and that the new phase IDs select the expected full/CI lists.
- Promote the seven-source inventory and all 160 operation mappings into supported contract tooling with immutable hashes and a reproducible semantic diff.
- Classify every management delta as new behavior, typed ergonomic gap, already supported, rename/documentation, vendor defect, or compatibility exception. Record baseline and affected operations for each.
- Create a retention manifest for public exports, SDK-only routes/query options, pagination, authentication, response tolerance and all 23 existing default bindings. Preserve G10 omission/null/value semantics and operation names.
- Record the five-operation target response mapping below, including credential-bearing unredacted PUT shapes. Establish per-operation schema binding/overlay decisions while retaining the public `TargetResponse` API.
- Classify all six `api.apps`/`api.sase` discrepancies and establish documented endpoint profiles and destination-admission requirements. No silent default change or bearer redirect forwarding.
- Add an explicit per-upstream-path source-precedence ledger: immutable frozen TypeScript fixture provenance; active vendor source; generator/validator overlays and compatibility-floor owner. For data-plane, the frozen0.7.75 file remains historical even though its manifest says usedForGeneration=true; active upstream drift comparison uses vendor0.93.0. For management, current active source is frozen0.7.67; adoption changes it to vendor0.86.0 without rewriting history. Unchanged families use the frozen source as their active pin. `usedForGeneration` alone must never select precedence. Hash YAML bytes as the canonical pin and separately record normalized JSON derivation/hash.
- Decide the public Rust layout strategy before adding fields: preserve old structs/builders and exports; introduce additive options/types/builders where public struct literals would otherwise break. Audit enum matching and MSRV 1.89. No dependency additions unless justified.
- Obtain actual Claude Code plan review before runtime work; record exact reviewed source and unresolved questions.

**Exit:** every upstream delta has a disposition; no removal/behavior narrowing hidden in generation; reviewed design passes the rubric below.

### B / Phase22. Implement target filters and connection schema support

**Chunks**

1. Connection compatibility (implement first): extend nested request/response unions for Vertex and client_session; prove full target responses survive validation before typed fallback. Cover all five operation-specific redacted/unredacted response declarations below; accept both as required by each operation without exposing returned credentials in Debug, tracing or errors. Multi-turn resolution is discriminator-first when `type` is present; an absent discriminator (including `{}`) retains frozen legacy resolution to stateless, and no new client_session default is injected. Align validator and untagged serde branch behavior. Provider unions without a discriminator retain old branch precedence; append the new Vertex structural branch after existing valid shapes and pin ambiguous/hybrid objects in fixtures. Record observable `Unknown(JsonValue)` to named-variant transitions in the changelog and match-based consumer fixtures, retaining raw values and old variants.
2. Target filtering: add profiling_status with complete list/page/list_all propagation; keep existing status behavior and publish migration guidance. New filter values follow the declared filter enum, independently of response-only statuses.
3. Connection variants: implement Vertex configuration, new enum values and client_session without deleting existing variants. For these declared variants, extend the specific schema branches; do not globally disable object/literal validation. Decide any future-unknown object-union fallback separately, with response-only scope and preservation of wrong-type/required-field checks where the compatibility baseline requires them. Keep SDK endpoint OAuth distinct from credentials being configured for the remote target.
4. Typed fields: Basic Auth pair and audio request/response fields, with extension-preserving conversion, masked-secret echo behavior and no new implicit response defaults. Reuse existing metadata/language projections and retain legacy aliases.

**Exit:** each new declared request shape serializes correctly; each response shape survives both response modes and typed JSON round-trips. No network/remote target code is executed by parsing configuration. Audio/Vertex configuration support is not advertised as successful provider execution without evidence.

### C / Phase23. Compatibility, security and conformance qualification

**Milestones**

- Add generated fixtures for every changed component and every affected operation, including nested unions, null/missing/value, future enum values, unknown keys, query serialization and pagination on later pages.
- Exercise old and new request shapes; verify default behavior unchanged and `without_response_defaults()` leaves absent keys absent while retaining explicit nulls. Test shared schemas through both management and data-plane callers.
- Verify sensitive Basic passwords, authorization/header values, OAuth tokens and Vertex credentials never appear in Debug, errors, tracing, review records or package artifacts. Treat server-provided curl strings as data; never execute them. Cover returned/masked credentials without replacing secrets accidentally.
- Validate REST audio restrictions separately from general AudioFormat: PCM16's presence in the shared enum is not evidence that REST target configuration accepts it. Preserve unknown response values.
- Run the existing compatibility suite, generated-model/contract checks, MSRV/current-toolchain feature matrix and packaged consumer checks, plus affected request/response regressions. Do not add preserve_order or reorder public JSON semantics as part of this work.
- Record vendor defects and optional request-policy decisions for Runtime and Model Security; no feature rewrite is required merely because their unchanged sources were reread.

**Exit:** checks distinguish contract coverage, public compatibility and live acceptance; no critical/high findings; no known regressions on preserved features.

### D / Phase24. Controlled service qualification and release

**Milestones**

- Read-only target listing/filtering and existing reads first, using explicitly selected external tenant configuration and sanitized results.
- Exercise mutation/provider workflows only with available entitlements, uniquely owned disposable fixtures, bounded cost and recorded cleanup. A Vertex/N8N/realtime entitlement gap is recorded as unverified, not implemented-and-live. Never retarget unrelated tenant resources.
- Select patch vs minor from actual public API impact; default expectation is a feature release, not an automatic promise of 0.2.3. Ensure consumer compilation before choosing. No release needed solely for unchanged specs.
- Regenerate source-bound native ledgers and qualification receipts only after source settles. Follow the established ordering: qualify both clean package archives and full checks → final Claude release-readiness review → publish → verify registry version and downloaded archive → create immutable release tag → push ordinary history/tag to `ssh://git@git-ssh.cdot.io/cdot/scm-sdk-rs.git` → await terminal successful remote CI. Record final delivery review/checks against actual artifacts; never mark the phase complete while CI is pending or failed.
- Verify the registry version and downloaded package, publish a changelog covering new support, preserved legacy behavior, intentional differences and live limitations. Consumer adoption remains a separate decision.

**Exit:** accepted review and source-bound checks correspond to the actual published archive; terminal remote CI is successful and recorded; no unsupported live coverage claims.

### E / Phase25. Prevent recurring drift

**Milestones**

- Select the active pin using Phase21's explicit precedence ledger; never report intentional frozen historical sources as current drift. Add a scheduled or manually invoked read-only upstream audit keyed by exact content hashes and normalized semantics, not just info.version (Runtime uses0.0.0).
- Report added/removed routes, parameters, body/response fields, requiredness/nullability, enum values, auth, server and media changes. Resolve aliases and reference closures.
- Upstream removals create review items; they never auto-delete SDK exports or capabilities. Keep compatibility overlays and their rationale explicit.
- Keep source coverage, executable fixtures and tenant acceptance as separate dashboards/ledgers; avoid presenting method counts as complete service qualification.

**Exit:** a synthetic added field, removed endpoint and changed enum are classified correctly without modifying public APIs automatically.

## Review gates and effort

Carry forward the maintainer rubric: implementation, code quality, supportability, architecture and feature delivery, each /10. Chunks require at least8 in every category and total42/50 (target45); phases require46/50. At most five review/remediation rounds per scope; the permitted fifth-attempt exception requires strictly above40, each category at least8, and explicit unresolved limitations. Security/release blockers do not become safe through scoring. Plan review also must meet the previously requested9/10 before implementation. Record real Claude Code output; never substitute a self-score.

Planning estimate, not a delivery promise: **4–8 focused engineering days** for A–D including compatibility fixtures, review and release; **1–2 additional days** for reusable drift automation. Most uncertainty is target-provider semantics, public model compatibility and live entitlements, not endpoint creation. Existing CI duration is elapsed time rather than continuous engineering effort. Finish with an honest evidence boundary if a provider cannot be exercised.

## Detailed family appendices

The following sections contain the source-level findings and preservation inventory. Local source line references apply to the audited commit, not an arbitrary future checkout.


---

# Red Team audit — 2026-10-19

## Scope and evidence

Audited pan.dev commit `582e30543bf2adfd8b0ee457170d22b5c380dfbe` against scm-sdk 0.2.2 actual native source, frozen TypeScript schemas, native operation mappings, and the newer vendored Red Team dataplane pin. Read-only structural analysis; no test execution or live service calls. `red-team-operations.json` contains **94 individual operation rows**, native method/source/line, complete parameter/request/response declarations, schema dependency closure, and status. `red-team-management-schema-diff.json` preserves all 68 exact management JSON-pointer changes (including metadata and schema renames).

Primary sources:

- [Dataplane 0.93.0](https://github.com/PaloAltoNetworks/pan.dev/blob/582e30543bf2adfd8b0ee457170d22b5c380dfbe/openapi-specs/prisma-airs-redteam/data-plane/dp-openapi.yaml).
- [Management 0.86.0](https://github.com/PaloAltoNetworks/pan.dev/blob/582e30543bf2adfd8b0ee457170d22b5c380dfbe/openapi-specs/prisma-airs-redteam/management/mp-openapi.yaml).
- [Network Broker 1.0.0](https://github.com/PaloAltoNetworks/pan.dev/blob/582e30543bf2adfd8b0ee457170d22b5c380dfbe/openapi-specs/prisma-airs-redteam/network-broker/AIRS-Red-Teaming-Network-Broker.yaml).

| Plane | Published operations | Rust methods mapped | Current vs local source |
| --- | ---: | ---: | --- |
| Dataplane | 47 | 47 | Parsed semantics compared against derived `contracts/vendor/red-team-data-plane/0.93.0/openapi.json` are identical; canonical YAML byte equality is separately recorded in inventory.json; no newly missing route/schema |
| Management | 42 | 42 | Old frozen 0.7.67 vs current 0.86.0; no new/removed route; target-list query changed, 10 added/8 renamed-away/21 changed component schemas |
| Network Broker | 5 | 5 | Entire parsed spec identical to frozen source; no new schema delta |

All 94 routes exist, **but route coverage does not imply management payload compatibility**. Nine management operations have non-error-schema changes in their dependency closure; the other 33 have only the common `ValidationError.input/ctx` additions. The nine are create/list/get/update target, update target profile, probe target, validate auth, upload prompts CSV, and management languages. Some deltas in these nine are already covered or documentation-only, detailed below.

## Concrete gaps and already-covered changes

### RT-1 — high: new nested target variants can reject responses and requests

Current management admits `MultiTurnClientSessionConfig {type: "client_session"}` and `VertexAIConnectionParams` / redacted counterpart. The SDK still has only stateful/stateless multi-turn and the four structural provider shapes (OpenAI/HuggingFace share one shape, Databricks, Bedrock, Copilot). These are **known nested fields**, so generic object passthrough does not save them:

- `schema_graph.json` nodes `s010504` -> `s010505/s010511` have literal `stateful` / `stateless`; a `client_session` object matches neither.
- Node `s010462` provider union requires api_key/model_name, auth_type/workspace_url/model_name, access_id/access_secret/region/model_id, or Copilot client/tenant/etc. A valid Vertex-only object with `project_id/location/engine_id/customer_sa_email` matches none.
- `validation.rs:659` requires one object-union branch to succeed; `validation.rs:775` retains literal matching in response mode. Open response enums are not the issue.
- Typed `Unknown(JsonValue)` fallbacks in `models/generated.rs:12719` and `:171897` happen after contract validation and cannot bypass that rejection.

Impact: valid new target configurations may fail create/update/probe before dispatch; target get/create/update/probe/update_profile response documents containing these nested variants may fail reads. This is static code-path evidence, not a reproduced service failure. List responses do not carry full connection_params, so do not claim this specific nested-config issue affects every list.

Implement additive Vertex (including masked service account email handling) and client-session types/unions and validator variants, preserving all older branches. Response union policy must accommodate new supported variants without reintroducing the strict vendor response-floor problem fixed by G25. Add generated and typed wire roundtrip fixtures for both response modes, including omissions/nulls and unexpected future values.

### RT-2 — high: new request enum values rejected

Management `TargetConnectionType` adds **VERTEX_AI, N8N**; `ResponseMode` adds **REALTIME**. Existing management request graph nodes `s012061` and `s012067` omit them. The data-plane projection already has named `VERTEXAI` and `N8N` variants (`models/red_team_data_plane.rs:19480–19494`); reuse its wire-value conventions without renaming existing public variants. `TargetCreateRequestConnectionType` at `models/generated.rs:174318` and response-mode enum at `:174383` expose Unknown(String), but request validator still rejects unlisted enum values. Shared create/update/probe schemas use those exact nodes. Add named variants plus request accepted values without deleting legacy values. Responses already accept these scalar strings and preserve them as Unknown; distinguish request blockage from optional typed response ergonomics.

### RT-3 — medium: missing profiling_status query option

`GET /v1/target` now declares separate `profiling_status` (NOT_STARTED, IN_PROGRESS, COMPLETED, FAILED). SDK `TargetListOptions` (`targets.rs:17`) and `query` (`:47`) have no such field. Add optional filter and carry it through list/pages/items/list_all (cloned options already provide propagation pattern).

Upstream simultaneously changes status semantics: `status` now lifecycle-only DRAFT/ACTIVE, while old spec accepted lifecycle/profiling values together. SDK currently forwards arbitrary bounded status strings, so **do not silently remove legacy values**. Document the upstream split and introduce the new field; retain old caller behavior unless actual endpoint evidence justifies a separately approved migration. Current profiling response enum adds NOT_STARTED and drops INIT/QUEUED: responses already preserve new values; keep old variants.

### RT-4 — medium: typed connection/audio/basic auth completeness

New fields absent from typed management models:

- REST/streaming/WebSocket connection params: `audio_request_json` and `audio_curl`, including redacted counterparts.
- Basic auth/redacted auth: `basic_username`, `basic_password`. Upstream says username/password is preferred, header is derived by service, masked password echoed unchanged preserves stored credential.
- Dedicated Vertex types and client-session model (functional blockage above).

Audio/basic fields reside in passthrough objects today, so callers can carry them through `extensions`; classify these **typed API gaps, not blanket wire rejection**. `BasicAuthAuthConfig` at `models/generated.rs:9242` only declares location/header but flattens extensions. Request-specific basic branch `s010755` is also passthrough. Connection object `s010789` and siblings are passthrough. Add typed fields using omission/null/value semantics and redacted Debug; never decode/rewrite the service's masked secret or default to client-side Basic encoding. Ensure promotion from extensions does not collide or lose unusual response values.

### RT-5 — already covered: target audio metadata and language country_code

Do not file these as missing:

- `TargetMetadata` already has seven audio/text additions (`audio_probe_passed`, `audio_probe_error`, `audio_multi_turn_probe_passed`, `audio_multi_turn_probe_error`, `supports_audio`, `audio_format`, `supports_text`) at `models/generated.rs:174996`. These were added by the dataplane model projection work.
- `LanguageOption.country_code` is already present and losslessly projected (`models/generated.rs:146971`), so management's added LanguageDropdownOptionSchema (the original LanguageOptionSchema remains) is represented by the same SDK type; this alias mapping is covered in the existing shared model (`generated.rs:146971`); `facade.rs:287` is the management-language operation using it.
- Keep current default-insertion policy; do not blindly inject the newer vendor supports_audio=false/supports_text=true defaults into response documents. G10 opt-out and G25 baseline guarantees remain mandatory.

### RT-6 — bookkeeping rather than functionality

The current OpenAPI generator consolidates the eight connection `*-Input`/`*-Output` component names into four base names. Preserve public Rust names/aliases; update source provenance and mapping, not delete types. CSV file schema changes `format: binary` to `contentMediaType: application/octet-stream`; SDK already sends binary multipart with that fallback (`custom_attacks.rs:233`). Keep upload/download behavior and existing CSV media-type correction. Common `ValidationError` adds input/ctx; these are diagnostic model metadata, not a reason to expose sensitive error bodies or claim 42 feature gaps.

## Dataplane and Network Broker conclusions

All current dataplane operations and schemas were already incorporated by phases9–12. Original frozen dataplane0.7.75 differs by11routes plus fields, but comparison against **active** vendor0.93.0 proves those are already delivered. `operations.json` records47implemented; per-operation rows bind actual source methods. Preserve v1/v2 downloads, threat overrides, runtime profile updates, ASR, regeneration/status, error CSV, sentiment, custom attacks, bounded pagination and safe failures.

G25's response floor for36TypeScript counterpart operations, relaxed response string formats, open enums/unknown fields, optional vendor projections, and ResponseWithoutDefaults are intentional compatibility and not upstream gaps. Target profile error logs still only declares limit; skip/search refusal and caller UUID checks remain documented compatibility decisions, not new spec changes.

Network Broker five routes (list/create/stats/get/PATCH channel) and full source unchanged. `network_broker.rs` implements all with OAuth, dedicated endpoint and no x-tsg-id. Do not invent a delete channel API absent from current source; retain any caller-provided raw functionality.

## Shared server/protocol issue

All three published specs advertise `https://api.sase.paloaltonetworks.com/...`, while exported SDK DEFAULT_RED_TEAM_* constants use `https://api.apps.paloaltonetworks.com/...` (`constants.rs:632`). Paths match, OAuth bearer matches, and clients accept explicit caller endpoints. This discrepancy existed in the frozen sources too; it is **not evidence the legacy host is broken**. Verify documented alias/regional routing and tenant acceptance before changing defaults. Prefer additive documented endpoint profiles/aliases; preserve old caller endpoints and destination admission. No redirect-based bearer forwarding should be introduced.

No auth declaration or media-type semantic migration was observed between the frozen and current sources beyond the CSV schema annotation. HTTP errors continue through redacted SDK error policy; do not turn error schema changes into credential exposure.

## Preserve SDK capabilities missing from these three specifications

At least **13 actual route-method capabilities** are outside their published route set and must remain:

- Three EULA operations: GET /v1/eula/content, GET /v1/eula/status, POST /v1/eula/accept (`eula.rs:20,32,41`).
- Eight instance/device/registry operations: create/get/update/delete instance, create/update/delete devices, POST registry credentials (`instances.rs:19` onward).
- GET /v1/template/target-metadata and GET /v1/template/target-templates (`targets.rs:287,300`).

Also retain higher-level pagination, target template discovery, metadata fallback (only the observed HTTP422 condition, `facade.rs:120`), options for no response defaults, bounded lifecycle/uncertain mutation handling, custom transports and security controls. A route disappearing or a schema rename in upstream never authorizes deletion.

## Suggested implementation chunks and acceptance

1. Pin management0.86.0 exact bytes/source/notice alongside existing frozen baseline; record all42operation mappings and the9functional dependency closures plus33error-metadata closures. Keep baseline provenance immutable.
2. Fix new nested target unions and enums (RT1/RT2) first. Add typed Vertex/client-session and current request contracts, retain response compatibility policy and legacy enum branches. Independent fixtures: Vertex redacted/unredacted, N8N client_session, REALTIME; create/update/probe plus typed get/update_profile response roundtrips, no-secret diagnostics.
3. Add profiling_status across every paginator and typed audio/basic fields; tests prove no implicit query rewrite, extension preservation, omission/null/default option semantics, masked credential echo and old methods remain callable.
4. Reconcile schema aliases/CSV/error metadata and server documentation. No default-host switch without evidence; record unsupported live prerequisites honestly.
5. Qualify affected generation/contract/default/union/security/packaged-consumer checks, retain all94route bindings plus13extras, actual independent review under maintainer rubric, release-note changes precisely. Target current patch/minor choice after determining additive API policy; do not republish0.2.2.

No SDK source was altered in this audit and no live service support was claimed solely from OpenAPI presence.


---

# Runtime Security API audit — 2026-10-19

## Scope and result

Compared current `PaloAltoNetworks/pan.dev` commit `582e30543bf2adfd8b0ee457170d22b5c380dfbe` with repository-frozen vendor sources, Rust route/query/body/auth construction, generated model exports and validation graph. Analysis only: no tests, live calls, or application/contract edits performed.

**All 4 scan operations and all 21 management operations have native typed Rust client methods. Both upstream specs are semantically identical to the frozen copies.** This is route/schema-source coverage, not proof every request accepted by the OpenAPI is accepted by the SDK or every operation works live. No genuinely new vendor runtime endpoint or schema update was found.

Upstream sources:

- [Scan specification](https://github.com/PaloAltoNetworks/pan.dev/blob/582e30543bf2adfd8b0ee457170d22b5c380dfbe/openapi-specs/prisma-airs/scan/scan-service_latest.yaml)
- [Management specification](https://github.com/PaloAltoNetworks/pan.dev/blob/582e30543bf2adfd8b0ee457170d22b5c380dfbe/openapi-specs/prisma-airs/management/mgmt-service_latest.yaml)

Both report version `0.0.0`; version alone is insufficient for future drift detection. Scan defines 57 named component schemas, management 31. Normalized snapshots and the research script remain in the ignored `.local/pan-dev-audit-2026-10-19/` directory and are not committed reproducibility tooling. The durable evidence here is the inventory and enriched operation mapping; Phase21 must promote reproducible tooling and the changed vendor pin before implementation claims. They compare schema data, ignoring YAML formatting, not simply version strings or route counts.

## Operation coverage

All listed endpoints use JSON. Scan has four named regional hosts and `x-pan-token` authentication. Rust `Region` covers all four (US, Germany/EU, India, Singapore), `Scanner` requires Scan auth, and the common auth layer adds `x-pan-token`. SDK also retains its bearer-token and HMAC scan features beyond this schema. Management uses bearer OAuth with configurable endpoint; its published server is `https://api.sase.paloaltonetworks.com/aisec`. All Rust management clients require OAuth even where vendor operation security is omitted.

| Method/path | Native method | Contract / important qualifications |
|---|---|---|
| POST `/v1/scan/sync/request` | `Scanner::sync_scan`, `scan_request` | `ScanRequestSchema` → `ScanResponseSchema`; convenience method sends one content item, complete-request method supports full content list. |
| POST `/v1/scan/async/request` | `Scanner::async_scan` | `AsyncScanObjectSchema[]` → `AsyncScanResponseSchema`; flat batch of 1–20. Vendor nested-array defect described below. |
| GET `/v1/scan/results` | `Scanner::query_by_scan_ids` | CSV `scan_ids`, 1–5; SDK requires UUIDs; `ScanIdResultSchema[]`. |
| GET `/v1/scan/reports` | `Scanner::query_by_report_ids` | CSV `report_ids`, 1–5 opaque strings; `ThreatScanReportSchema[]`. |
| POST `/v1/mgmt/apikey` | `ApiKeysClient::create` | `ApiKeyCreateRequestSchema` → `ApiKeySchema`. |
| GET `/v1/mgmt/apikeys` | `ApiKeysClient::list_for_token` | offset, limit; `ApiKeyListResponseSchema`. |
| DELETE `/v1/mgmt/apikey/delete/{api_key_name}` | `ApiKeysClient::delete` | name path + required updated_by query; `ApiKeyDeleteResponseSchema`. |
| POST `/v1/mgmt/apikey/regenerate/{api_key_id}` | `ApiKeysClient::regenerate` | ID path; `ApiKeyRegenerateRequestSchema` → `ApiKeySchema`. |
| POST `/v1/mgmt/profile` | `ProfilesClient::create` | `CreateSecurityProfileRequestSchema` → `SecurityProfileSchema`; preserves optional revision behavior. |
| DELETE `/v1/mgmt/profile/{profile_id}` | `ProfilesClient::delete` | UUID validation before I/O; `DeleteProfileResponseSchema`. |
| PUT `/v1/mgmt/profile/uuid/{profile_id}` | `ProfilesClient::update` | UUID; `CreateSecurityProfileRequestSchema` → `SecurityProfileSchema`. |
| GET `/v1/mgmt/profiles` | `ProfilesClient::list_for_token` | offset, limit, latest; `SecurityProfileListResponseSchema`. |
| DELETE `/v1/mgmt/profile/{profile_id}/force` | `ProfilesClient::force_delete` | UUID + updated_by query; `DeleteProfileResponseSchema`. |
| GET `/v1/mgmt/dlpprofiles` | `DlpProfilesClient::list` | `DlpProfileListResponseSchema`. Preserve alongside canonical `crate::dlp` API. |
| GET `/v1/mgmt/deploymentprofiles` | `DeploymentProfilesClient::list` | unactivated query supported; `DeploymentProfilesResponseSchema`. |
| GET `/v1/mgmt/customerapp` | `CustomerAppsClient::get` | app_name query; `CustomerAppSchema`. |
| DELETE `/v1/mgmt/customerapp` | `CustomerAppsClient::delete` | app_name + updated_by; `CustomerAppDeleteResponseSchema` union includes `CustomerAppSchema`, plus existing message shapes. |
| PUT `/v1/mgmt/customerapp` | `CustomerAppsClient::update` | customer_app_id query; `CustomerAppSchema` request/response. |
| GET `/v1/mgmt/customerapps` | `CustomerAppsClient::list_for_token` | offset, limit; `CustomerAppListResponseSchema`. |
| POST `/v1/mgmt/topic` | `TopicsClient::create` | `CreateCustomTopicRequestSchema` → `CustomTopicSchema`. |
| DELETE `/v1/mgmt/topic/{topic_id}` | `TopicsClient::delete` | UUID; `DeleteTopicResponseSchema`. |
| DELETE `/v1/mgmt/topic/{topic_id}/force` | `TopicsClient::force_delete_with_audit`, `force_delete` | UUID + nonblank updated_by; `DeleteTopicResponseSchema`. |
| PUT `/v1/mgmt/topic/uuid/{topic_id}` | `TopicsClient::update` | UUID; `CreateCustomTopicRequestSchema` → `CustomTopicSchema`. |
| GET `/v1/mgmt/topics` | `TopicsClient::list_for_token` | offset, limit; `CustomTopicListResponseSchema`. |
| POST `/v1/mgmt/oauth/client_credential/accesstoken` | `OAuthManagementClient::get_access_token` | body `ClientIdAndCustomerAppSchema`, tokenTtlInterval + tokenTtlUnit; `Oauth2TokenSchema`. |

SDK route source anchors: `crates/scm-sdk/src/airs/scan/scanner.rs:115,154,175,201,221`; management `api_keys.rs:31,46,121,138`; `profiles.rs:66,81,201,220,236`; `topics.rs:49,64,239,258,274`; `customer_apps.rs:31,46,116,134`; `inventory.rs:19,49`; `oauth_management.rs:57`.

## Field / shape findings

### RUNTIME-01 — no current new multimodal/upload/streaming route

The inspected specs declare only `application/json`, no multipart upload, file upload endpoint, image/audio content property, or SSE/stream response. The scanner already represents prompt/response, extracted code, contextual grounding, tool events, agent metadata, toxic/category/topic reports, masking/offsets, MCP tool detections, malware/command-injection reports and scan timestamps. Therefore this spec audit does **not** justify declaring multimodal/SSE unsupported API gaps. Future product needs may warrant work separately. `Content::from_json_file` reads a local JSON content document; it is not a remote file-scanning upload API.

### RUNTIME-02 — vendor inconsistencies need explicit exceptions, not blind generation

1. Scan `POST /v1/scan/async/request` has request schema array(items=`AsyncScanRequest`), and `AsyncScanRequest` is itself array(items=`AsyncScanObject`). Literal generation would require nested arrays. SDK intentionally sends flat objects, consistent with frozen TypeScript behavior. Preserve flat batch API pending a vendor clarification; don't replace it with nested arrays. Upstream lines 100–108 and 709ff; Rust `scanner.rs:175`.
2. Management `ModelProtectionObject.items.properties.topic-list` lacks its own `items`; a sibling property named `items` references `TopicArray`. `required: [name, action]` is attached to the array rather than its element. This is a malformed authoring layout, not strong evidence a real `items` field is missing. Rust types the useful `topic-list: Vec<TopicArray>` and keeps open extension storage; a literal sibling `items` value can pass through extensions but lacks a named field. Do not promote that typo into a public API without service evidence. Upstream lines 1150–1180; baseline `src/models/mgmt-security-profile.ts:177`.
3. Scan query arrays use `maximum: 5`, a numeric keyword, instead of `maxItems: 5`; Rust explicitly enforces 1–5. Preserve behavior and record normalized interpretation.
4. Seven management operations omit security while root has no security. Rust nevertheless requires OAuth. Treat this as upstream documentation incompleteness; never remove auth to mechanically match the document.

### RUNTIME-03 — legacy SDK request strictness exceeds literal spec

These are pre-existing compatibility/policy differences, not newly introduced upstream API drift:

- `query_by_scan_ids` requires UUID, although query items are plain strings. Report IDs are opaque, but both query methods reject empty strings, controls, commas, >1024-byte IDs and batches outside1–5 (`scanner.rs:238`). Delimiter/size bounds are transport safeguards; a future decision to accept opaque scan IDs can be additive/compatibility-reviewed without weakening them.
- Profile and custom-topic update/delete/force-delete enforce UUID although published path schemas say only string. Preserve existing methods; add documented opaque-ID acceptance only after product evidence and a compatibility decision, as with G25 request-policy disposition.
- scan tr_id/session_id and profile_name are limited to100 characters in the frozen validator even though OpenAPI does not declare these bounds. `contents` requires at least one item; content requires nonempty prompt/response/code or a tool event. Prompt/response/code are capped2MiB each, context100MiB (`content.rs:6,188`). Async batch1–20. These limits need explicit policy documentation rather than automatic removal.
- Pagination uses positive u32 limit and safe-integer-bounded u64 offset, whereas OpenAPI uses unrestricted signed int32 declarations. Existing behavior is deliberate caller safety/TypeScript parity; don't mistake absence of negative paging support for a new API route gap.
- OAuth token TTL schema says int32 but SDK uses `FiniteNumber`; it can send fractional TTL. This is looser SDK validation, not missing capability. Keep existing method behavior and document service-owned validity; don't silently tighten.
- Vendor request body for access-token creation is not marked required, but typed API requires body. An empty-body operation is not proven service-valid from this omission; record for clarification rather than invent a request path.

### RUNTIME-04 — preserve response tolerance and useful schema extras

No missing useful typed property was identified in recursive comparison of mapped scan/management schemas. The sole literal missing property was the likely vendor `items` typo above. Existing SDK is deliberately more permissive/useful:

- PromptDetected.source_code; detection metadata confidence/score.
- Profile csp_id/tsg_id; optional revision/policy in response where vendor requires them.
- Model/agent protection severity, toxicity severity-by-confidence, detector options; URL default/action/severity and malicious-code config.
- Data-protection database-security/source-code-detection; DLP profile description; topic severity.
- `TopicArray.topic` nullable to preserve actual empty-topic server replies; TypeScript source explicitly documents this live-service divergence (`mgmt-security-profile.ts:150`). Never make it non-nullable to match OpenAPI.
- DLP snippet meta/snippets and some member/revision fields remain optional where vendor requires them. Keep these tolerated response shapes.
- Global0.2.2 response policy keeps unknown string enum values/unknown object keys and avoids string-format rejection. Do not replace this with raw vendor response validation.

All generated object models retain extensions, but **`Content::from_json` explicitly clears unknown content extensions** (`content.rs:73–77`) to match its six-field convenience-helper contract. The complete typed `Scanner::scan_request` path is the relevant extension-preserving path. Not a current schema gap because all six declared content fields already exist; document this seam before adding future scan fields. Do not claim every convenience wrapper automatically forwards future fields.

### RUNTIME-05 — retain SDK-only operations

The vendor files are not a superset of useful working SDK capability. Preserve tenant-suffixed `/profiles/tsg`, `/topics/tsg`, `/apikeys/tsg`, `/customerapp/tsg` collection routes, pages/list_all and lookup helpers, latest-topic selection, all Dashboard typed/raw query methods, legacy scanlogs, OAuth invalidateToken, canonical DLP methods, scan HMAC/bearer options, injected transports, cancellation and operation budgets. None should be removed because these two specs omit them. Spec-listed list routes use existing `list_for_token`, not the tenant-suffixed `list`; document this distinction rather than replacing either.

## Proposed path forward (runtime portion)

1. **P1 contract-source inventory:** pin exact upstream commit+SHA bytes and semantic hash; create per-operation mapping rows for all25; record unchanged sources as verified-current rather than mass-regenerating contracts. Add explicit vendor-defect/compatibility exceptions for async double-array, model-protection layout, array maximum keyword, missing security, nullable topics and request policies.
2. **P1 additive regression matrix:** after plan approval, extend evidence around all25 mapped methods covering declared query fields, request serialization, success bodies, unknown-field/enum responses, null/omission preservation, and error-path behavior. Add specimens for both spec-listed token-scoped lists and SDK-only tenant lists. This audit did not run these tests or live calls.
3. **P2 request-policy decision:** audit scan query IDs and six profile/topic mutation UUID guards for real accepted opaque IDs; keep existing behavior unless supported change is intentional. Preserve resource limits and TypeScript compatibility. Verify optional OAuth-body semantics only with documentation/service evidence.
4. **P2 source drift automation:** compare normalized operation/request/response/auth/server/schema shapes across exact commits rather than spec version0.0.0; fail only on unclassified new deltas and never auto-delete SDK capabilities absent from vendor files.
5. **Qualification:** separate contract mapping, executable fixture results, and live tenant permission outcomes. Runtime does not currently require a feature release merely because these upstream files were reread; fold contract-ledger/exception work into the cross-product update plan and prioritize actual gaps from other families.


---

# Model Security API audit — 2026-10-19

## Finding

**No new Model Security upstream schema drift, missing HTTP operation, missing declared query parameter, or missing object-property projection was identified.** Both upstream YAML files are byte-identical to the SDK's committed pan.dev copies. There are **41 operations: 18 data-plane and 23 management-plane**, all matched to executable Rust methods. This is a static source audit, not fresh live acceptance or test execution.

Pinned upstream commit: `582e30543bf2adfd8b0ee457170d22b5c380dfbe`.

- [Upstream data-plane source](https://github.com/PaloAltoNetworks/pan.dev/blob/582e30543bf2adfd8b0ee457170d22b5c380dfbe/openapi-specs/prisma-airs-model-security/dataplane/data-plane.yml)
- [Upstream management source](https://github.com/PaloAltoNetworks/pan.dev/blob/582e30543bf2adfd8b0ee457170d22b5c380dfbe/openapi-specs/prisma-airs-model-security/management/mgmt-plane.yml)

Both specs advertise version `0.1.0`; version alone is insufficient drift detection. Exact SHA256s:

| File | SHA256 |
|---|---|
| dataplane/data-plane.yml | `1a581d23ceb7456b9a5f978b212235e2149dadf917eae7d31e1463667339d6dc` |
| management/mgmt-plane.yml | `92b010dbe1b2ba96984aaaadfca95e4a2ff68a1fa98209765cfa1b0bcffcc808` |

Canonical SDK sources: `contracts/sources/pan-dev/openapi-specs/prisma-airs-model-security/`; frozen TypeScript baseline `4a80dbdb3dc14365e1c25d838ac0bd139485d48c`; actual runtime graph `crates/scm-sdk/src/validation/schema_graph.json` and generated Rust `crates/scm-sdk/src/models/generated.rs`.

## Coverage method and limits

Matched every HTTP method/path after normalizing only path-placeholder names. The specs advertise `https://api.sase.paloaltonetworks.com/aims/data` and `/aims/mgmt`, while SDK constants export the corresponding `api.apps` URLs (`constants.rs:630–631`). Clients accept caller-selected separate endpoints; relative `/v1/...` paths match. Both clients share OAuth and destination policy (`client.rs:32`, `mod.rs:29`). Alias/regional equivalence has not been verified; both planes belong to the six-spec endpoint-profile decision in Phase21. Do not silently migrate defaults.

All 86 component entries were inventoried (40 data-plane, 46 management). All 64 object-component occurrences were classified: **60 map to SDK objects, four are the repeated generic HTTPValidationError/ValidationError envelopes**. Static field comparison covered 320 named property occurrences across mapped objects; no missing known field, vendor-optional/SDK-required mismatch, vendor-nullable/SDK-nonnullable mismatch, or obvious primitive-type narrowing appeared. This structural comparison is not a formal proof of complete JSON Schema equivalence; SDK-specific refinements and resource bounds remain intentional additional acceptance policies.

Aliases were resolved rather than counted as missing models: PaginationSchema→ModelSecurityPaginationSchema; ModelScanIssue/ScanDetails/LabelCondition/RuleResultCondition→their `Schema` roots; ConditionGroup-Input and ConditionGroup-Output→recursive ConditionGroupSchema; group/rule list items→shared corresponding response models with optional count fields; CustomRuleInstanceSummarySchema→nested generated rule-instance custom_rule object. Public model exports are in `crates/scm-sdk/src/models/model_security.rs:2`.

## Complete operation mapping

Each row was checked against the current spec's declared query names and success-response/request-body schema names. Machine-readable rows also contain operation IDs, all declared queries, extra SDK queries, success codes, body references, Rust source locations, and selected runtime schemas.

| Plane | Method / path | Rust implementation | Missing spec query options |
|---|---|---|---|
| dataplane | `GET /v1/evaluations/{uuid}` | `crates/scm-sdk/src/airs/model_security/scans.rs:362` — `get_evaluation` | None |
| dataplane | `GET /v1/model-versions/{uuid}` | `crates/scm-sdk/src/airs/model_security/inventory.rs:165` — `get_model_version` | None |
| dataplane | `GET /v1/model-versions/{uuid}/files` | `crates/scm-sdk/src/airs/model_security/inventory.rs:181` — `list_model_version_files` | None |
| dataplane | `GET /v1/models` | `crates/scm-sdk/src/airs/model_security/inventory.rs:22` — `list_models` | None |
| dataplane | `GET /v1/models/{uuid}` | `crates/scm-sdk/src/airs/model_security/inventory.rs:82` — `get_model` | None |
| dataplane | `GET /v1/models/{uuid}/model-versions` | `crates/scm-sdk/src/airs/model_security/inventory.rs:94` — `list_model_versions` | None |
| dataplane | `GET /v1/scans` | `crates/scm-sdk/src/airs/model_security/scans.rs:123` — `list` | None |
| dataplane | `POST /v1/scans` | `crates/scm-sdk/src/airs/model_security/scans.rs:107` — `create` | None |
| dataplane | `GET /v1/scans/label-keys` | `crates/scm-sdk/src/airs/model_security/scans.rs:329` — `get_label_keys` | None |
| dataplane | `GET /v1/scans/label-keys/{key}/values` | `crates/scm-sdk/src/airs/model_security/scans.rs:344` — `get_label_values` | None |
| dataplane | `GET /v1/scans/{scan_uuid}/evaluations` | `crates/scm-sdk/src/airs/model_security/scans.rs:210` — `get_evaluations` | None |
| dataplane | `GET /v1/scans/{scan_uuid}/files` | `crates/scm-sdk/src/airs/model_security/scans.rs:229` — `get_files` | None |
| dataplane | `DELETE /v1/scans/{scan_uuid}/labels` | `crates/scm-sdk/src/airs/model_security/scans.rs:288` — `delete_labels` | None |
| dataplane | `POST /v1/scans/{scan_uuid}/labels` | `crates/scm-sdk/src/airs/model_security/scans.rs:248` — `add_labels` | None |
| dataplane | `PUT /v1/scans/{scan_uuid}/labels` | `crates/scm-sdk/src/airs/model_security/scans.rs:268` — `set_labels` | None |
| dataplane | `GET /v1/scans/{scan_uuid}/rule-violations` | `crates/scm-sdk/src/airs/model_security/scans.rs:310` — `get_violations` | None |
| dataplane | `GET /v1/scans/{uuid}` | `crates/scm-sdk/src/airs/model_security/scans.rs:194` — `get` | None |
| dataplane | `GET /v1/violations/{uuid}` | `crates/scm-sdk/src/airs/model_security/scans.rs:378` — `get_violation` | None |
| management | `GET /v1/custom-rules` | `crates/scm-sdk/src/airs/model_security/custom_rules.rs:22` — `list` | None |
| management | `POST /v1/custom-rules` | `crates/scm-sdk/src/airs/model_security/custom_rules.rs:82` — `create` | None |
| management | `GET /v1/custom-rules/versions` | `crates/scm-sdk/src/airs/model_security/custom_rules.rs:216` — `list_versions` | None |
| management | `GET /v1/custom-rules/{custom_rule_uuid}` | `crates/scm-sdk/src/airs/model_security/custom_rules.rs:98` — `get` | None |
| management | `PUT /v1/custom-rules/{custom_rule_uuid}` | `crates/scm-sdk/src/airs/model_security/custom_rules.rs:114` — `update` | None |
| management | `POST /v1/custom-rules/{custom_rule_uuid}/archive` | `crates/scm-sdk/src/airs/model_security/custom_rules.rs:133` — `archive` | None |
| management | `GET /v1/custom-rules/{custom_rule_uuid}/security-groups` | `crates/scm-sdk/src/airs/model_security/custom_rules.rs:157` — `list_security_groups` | None |
| management | `POST /v1/custom-rules/{custom_rule_uuid}/security-groups` | `crates/scm-sdk/src/airs/model_security/custom_rules.rs:177` — `assign_security_groups` | None |
| management | `DELETE /v1/custom-rules/{custom_rule_uuid}/security-groups/{sg_uuid}` | `crates/scm-sdk/src/airs/model_security/custom_rules.rs:197` — `remove_assignment` | None |
| management | `POST /v1/custom-rules/{custom_rule_uuid}/unarchive` | `crates/scm-sdk/src/airs/model_security/custom_rules.rs:145` — `unarchive` | None |
| management | `GET /v1/pypi/authenticate` | `crates/scm-sdk/src/airs/model_security/client.rs:74` — `get_pypi_auth` | None |
| management | `GET /v1/security-groups` | `crates/scm-sdk/src/airs/model_security/groups.rs:37` — `list` | None |
| management | `POST /v1/security-groups` | `crates/scm-sdk/src/airs/model_security/groups.rs:21` — `create` | None |
| management | `GET /v1/security-groups/{security_group_uuid}/rule-instances` | `crates/scm-sdk/src/airs/model_security/groups.rs:143` — `list_rule_instances` | None |
| management | `GET /v1/security-groups/{security_group_uuid}/rule-instances/versions` | `crates/scm-sdk/src/airs/model_security/groups.rs:205` — `list_rule_instance_versions` | None |
| management | `GET /v1/security-groups/{security_group_uuid}/rule-instances/{rule_instance_uuid}` | `crates/scm-sdk/src/airs/model_security/groups.rs:162` — `get_rule_instance` | None |
| management | `PUT /v1/security-groups/{security_group_uuid}/rule-instances/{rule_instance_uuid}` | `crates/scm-sdk/src/airs/model_security/groups.rs:182` — `update_rule_instance` | None |
| management | `DELETE /v1/security-groups/{uuid}` | `crates/scm-sdk/src/airs/model_security/groups.rs:132` — `delete` | None |
| management | `GET /v1/security-groups/{uuid}` | `crates/scm-sdk/src/airs/model_security/groups.rs:97` — `get` | None |
| management | `PUT /v1/security-groups/{uuid}` | `crates/scm-sdk/src/airs/model_security/groups.rs:113` — `update` | None |
| management | `GET /v1/security-rules` | `crates/scm-sdk/src/airs/model_security/inventory.rs:263` — `list` | None |
| management | `GET /v1/security-rules/versions` | `crates/scm-sdk/src/airs/model_security/inventory.rs:339` — `list_versions` | None |
| management | `GET /v1/security-rules/{uuid}` | `crates/scm-sdk/src/airs/model_security/inventory.rs:323` — `get` | None |

## Differences that must be retained, not removed

There are **14 extra SDK query-option occurrences across ten routes**. They are frozen TypeScript compatibility, not evidence of API implementation gaps. The server may ignore some; this audit made no tenant calls and does not certify their effect. Keep them and document their provenance.

| Route | SDK extras absent from spec |
|---|---|
| `GET /v1/model-versions/{uuid}/files` | `search` |
| `GET /v1/models` | `isBackgroundRefresh`, `search` |
| `GET /v1/models/{uuid}/model-versions` | `search` |
| `GET /v1/scans` | `isBackgroundRefresh`, `search`, `sort_by` |
| `GET /v1/scans/{scan_uuid}/evaluations` | `search` |
| `GET /v1/scans/{scan_uuid}/files` | `result`, `search` |
| `GET /v1/scans/{scan_uuid}/rule-violations` | `search` |
| `GET /v1/security-groups` | `search` |
| `GET /v1/security-groups/{security_group_uuid}/rule-instances` | `search` |
| `GET /v1/security-rules` | `search` |

Also retain all paginator/collector methods, bounded collection/cursor behavior, shared transport/authentication, caller destination controls, mutation uncertainty, and explicit PyPI credential handling. Those conveniences are not individual OpenAPI operations. Snapshot historical `generation` reads are supported by list options; custom-rule `list_all` deliberately excludes `generation` just like TypeScript's `Omit<..., 'skip' | 'generation'>` (`.local/typescript/src/model-security/custom-rules-client.ts:95`, Rust `query.rs` CustomRuleListAllOptions). Do not label that an unsupported HTTP parameter.

The shared group response model includes optional `enabled_rule_count` and shared security-rule model includes optional `enabled_security_group_count`, even when detail-response schemas omit them. These fields are present on vendor list-item schemas and must stay. No separate public type rename is needed.

Request UUID validation for identifiers remains supported by the vendor and frozen TypeScript. Response enum/unknown-key openness and 0.2.2 response-format tolerance must remain: `crates/scm-sdk/src/validation.rs:1186` and `:883`. Avoid regenerating strict vendor validators over these response contracts. `without_response_defaults()` remains supported; default insertion policy must not change during source refresh.

Generic vendor validation-error schema names are not public typed Model Security response models. HTTP failures go through the shared SDK error machinery. This is an intentional architecture difference, not a missing success operation; richer structured error details would be a separate additive feature with redaction review.

## Existing contract-validation differences worth recording

These were already present in the exact committed spec, so they are **not newly introduced upstream changes** and do not prevent sending a vendor-valid request. Do not silently tighten existing callers while trueing up spec provenance.

1. **Query upper bounds and patterns are not uniformly enforced locally.** The common query builder checks safe nonnegative integer ranges, nonzero limits, URL byte budget, and 256 query entries (`query.rs:14–52`). Most list structs do not enforce the vendor maximum100; scan files vendor maximum1000 is likewise not locally capped. Custom-rule and snapshot options separately enforce100. Date/UUID query filters and most sort/filter strings remain strings. Examples: scans `labels_query` max4096; models `search_query` max256; security-group/rule `search_query` min3/max1000; label-key/value searches carry lexical patterns. Service-side rejection remains possible for invalid requests. Prefer additive checked builders or an explicit strict-request mode over tightening existing defaults.
2. **Label mutations are more permissive than the published spec.** `LabelsCreateRequestSchema` keeps the TypeScript `.min(1)` but not the vendor max50; `LabelSchema` keeps length limits but not the vendor alphanumeric/underscore/hyphen regex. `delete_labels` forwards repeated `keys` without vendor 1–50 cardinality/key-pattern checks (`scans.rs:298`). It still observes global URL bounds. Preserve established capability and optionally add checked convenience APIs.
3. **Scan submission nested arrays are more permissive.** Frozen TypeScript `ScanDetailsSchema` allows `files` without vendor max1000 and `model_formats` without vendor max100; FileScanData `formats` lacks vendor max20. This is a request validation difference, not absent functionality. ScanDetails/LabelsCreateRequest vendor additionalProperties=false is not mirrored by the TypeScript passthrough model, intentionally retaining extension capability (`.local/typescript/src/models/model-security.ts:115–180`).
4. **Response strictness must remain independent.** Vendor lengths, UUIDs, dates, closed enums, and strict-object annotations cannot simply replace the existing response floor. Existing known-field types/requiredness are represented, and additional service keys survive via extensions. Any future new enum/field must prove both raw preservation and typed round-trip behavior.
5. **Live acceptance is a separate ledger.** `contracts/native-operation-mappings/model-security.json` still classifies many operations unverified. Method presence, historical offline fixtures, and the current 41/41 static audit must not be described as successful tenant exercise of all mutations or scan workflows.

### Recorded numeric type widening

The evidence contains **19 property occurrences** (counting the data/management Pagination separately) where vendor integer fields map to SDK integer-or-number acceptance, including nullable forms. These are widenings, not missing functionality; preserve them rather than silently enforcing integers. Exact occurrences: `dataplane:EvalSummarySchema.rules_failed`, `dataplane:EvalSummarySchema.rules_passed`, `dataplane:EvalSummarySchema.total_rules`, `dataplane:FileScanDataSchema.size_bytes`, `dataplane:ModelVersionResponseSchema.file_count`, `dataplane:PaginationSchema.total_items`, `dataplane:RuleEvaluationResponseSchema.violation_count`, `dataplane:ScanBaseResponseSchema.enabled_rule_count_snapshot`, `dataplane:ScanBaseResponseSchema.total_files_scanned`, `dataplane:ScanBaseResponseSchema.total_files_skipped`, `dataplane:ScanDetails.model_size_bytes`, `dataplane:ScanDetails.scan_duration_ms`, `dataplane:ScanDetails.total_files_scanned`, `dataplane:ScanDetails.total_files_skipped`, `management:CustomRuleListItemSchema.enabled_security_group_count`, `management:ModelSecurityGroupListItemSchema.enabled_rule_count`, `management:ModelSecurityRuleListItemSchema.enabled_security_group_count`, `management:PaginationSchema.total_items`, `management:SnapshotVersionSchema.generation`.

## Recommended Model Security workstream

**P0: Preserve and automate the 41-operation baseline, no runtime rewrite.** Pin upstream tree/source hashes in a durable cross-product audit ledger. Preserve the exact previous source files. Capture operation IDs, plane/server, method/path, path/query/body fields, success/status/content types, recursive references, and origin of every deliberate SDK extension. The supplied machine-readable files are a reviewable starting point, not an accepted contract generator.

**P1: Add future drift detection.** Compare normalized semantics and exact source hashes; classify additive operation/property/enum changes separately from breaking requiredness/type/removal changes. Explicitly mark aliases above so renamed component definitions do not produce false missing-type alarms. Require reviewed compatibility decisions before adopting new response schemas. Keep frozen TypeScript behavior as a lower compatibility bound.

**P2: Optional request ergonomics and evidence.** Decide whether opt-in vendor-strict validation or checked builders are worthwhile for the preexisting bounds/pattern gaps. Do not remove extra query options. If approved, add boundary fixtures for limits100/1000, labels50, empty keys, patterns, generation reads, arrays, and null/default behavior; run targeted Model Security tests and both supported compilers. Keep sensitive PyPI credentials out of logs and never follow returned credential URLs automatically.

**P3: Controlled live qualification.** Read-only Model Security inventory/rules/groups first, then separately authorized uniquely owned scan/rule/group fixtures with bounded costs and cleanup. Record service permissions/unavailability separately from implementation gaps. This analysis did not run tests or live calls.

No Model Security endpoint implementation is required solely due to this upstream snapshot. Prioritize actual Runtime Security / Red Team gaps found by the other audit workstreams; fold Model Security's drift ledger and preservation checks into the same phased release gates.

## Audit artifacts

- `model-security-operations.json`: all41 operation rows.
- `model-security-schema-comparison.json`: all64 object-component classifications and320 property comparisons.
- `.local/pan-dev-audit-2026-10-19/audit-model-security.py`: ignored local research helper, not a file in this report directory or production tooling. Phase21 must replace the ephemeral helpers with supported reproducible tooling.
