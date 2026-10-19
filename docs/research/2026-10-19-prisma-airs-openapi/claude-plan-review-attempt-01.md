Note: the `Write` tool is disabled this session, so I can't persist the plan file. Delivering the review inline as requested.

# Prisma AIRS OpenAPI audit — advisory plan review

**Reviewed:** `docs/research/2026-10-19-prisma-airs-openapi/README.md` + 6 evidence JSONs, against `AGENTS.md`, `docs/PHASES.md`, `docs/implementation-plan.md`, actual SDK source, and the pinned tree in `.local/pan-dev-audit-2026-10-19`. Read-only; no edits, tests, or service calls.

## Verdict

**Not accepted. Plan 7/10. Total 33/50** (target was ≥9 and ≥46). Three categories sit below the rubric's per-category floor of 8, so this fails even chunk acceptance (≥42 with all ≥8).

| Category | Score |
|---|---:|
| Implementation feasibility | 7/10 |
| Code quality approach | 7/10 |
| Supportability | 6/10 |
| Architecture and delivery | 6/10 |
| Feature delivery / coverage claims | 7/10 |
| **Total** | **33/50** |

## What verified exactly (credit where due)

- **Provenance is sound.** `inventory.json` frozen baselines match `contracts/manifest.json` byte-for-byte: DP/MP commit `e2e6e63e`, sha256 `667ce2ea`/`4b92495d` (manifest.json:1849-1874); broker `891ec0e6`, `0cf2176d`, 10388 bytes (manifest.json:1877-1890). Versions confirmed in the pinned files (MP 0.86.0 vs frozen 0.7.67). `activeBytesEqualUpstream:false` for MP only, and 6/7 active-source match, are both correct.
- **Counts hold.** 4+21+18+23+47+42+5 = 160. `red-team-operations.json` has 94 rows, zero `implemented:false`. Spot check exact: `scans.rs:50` is `RedTeamScansClient::create`.
- **Management delta is complete at parameter level.** Path sets identical (29 paths / 42 ops both versions). Exhaustive parameter enumeration: 55 new vs 54 old — **exactly one** added parameter repo-wide (`profiling_status`, mp-openapi.yaml:371). Component delta 10 added / 8 renamed-away reproduces.
- **RT-1 is right, including the subtle parts.** `s010504` is a plain `ZodUnion` of two passthrough objects whose `type` is `ZodOptional<ZodLiteral<"stateful"|"stateless">>`; the literal check (validation.rs:775) is mode-independent, and the response-mode string passthrough (validation.rs:664-679) only fires when *every* variant is a string literal. Rejection in both modes holds.
- **RT-2 is right:** `s012061` lacks VERTEX_AI/N8N, `s012067` lacks REALTIME, while `ZodEnum` response mode accepts any string (validation.rs:765-769). The request-blocked / response-tolerant asymmetry is stated precisely.
- **RT-4's "typed gap, not wire rejection" classification is right:** `s010657` and the connection-param objects are `unknownFields:"passthrough"` with `ZodNever` catchall, and validation.rs:316-329 clones unknown keys through in both modes.
- **RT-5's "already covered" list is right:** `TargetMetadata` (generated.rs:174996) has all seven audio/text fields; `TargetMetadataAudioFormat` = WAV/MP3/PCM16+Unknown matches the new `AudioFormat`; `LanguageOption` (generated.rs:146971) already carries `country_code: Patch<String>`.
- Anchors verified: constants.rs:632, targets.rs:17/47, generated.rs:9242/12719/171897/174318/174383, custom_attacks.rs:233, content.rs:74-77, facade.rs:120 (422-only), extras 3+8+2=13. Rubric quoted matches PHASES.md:88-98.

## Blockers and precise repairs

**B1 (High) — omitted three-way response-schema divergence, with a secret dimension.** The spec declares *three different* target response schemas: GET `/v1/target/{uuid}` → `TargetRedactSchema` (masked), PUT `/v1/target/{uuid}` and PUT `.../profile` → `TargetSchema` (**unredacted**), POST `/v1/target` and `/v1/target/probe` → `TargetResponseSchema`. The SDK collapses all five onto one frozen node and one type (`targets.rs:177,198,229,266`; `TargetResponse.connection_params` at generated.rs:178620). Unrecorded, so the plan can't cover (a) one node admitting both redacted and unredacted connection-param/auth variants, or (b) PUT responses now returning real credentials that must stay out of Debug/tracing/errors.
*Repair:* per-operation table (vendor response schema → SDK validator node → public type) as an A exit artifact; B fixtures for redacted **and** unredacted variants of all five ops; explicit redaction assertion for `TargetSchema`-shaped PUT responses before B is accepted.

**B2 (High) — union resolution rule unspecified and genuinely ambiguous.** `MultiTurnClientSessionConfig` (mp-openapi.yaml:3257-3265) and `MultiTurnStatelessConfig` (3316-3335) both have **no required properties** and a defaulted `const type`, under `anyOf`. Resolution today is first-match order in validation (validation.rs:680-693) and `#[serde(untagged)]` order in decoding (generated.rs:171897), so `{}` resolves to stateless. Chunk B.1 gives no discriminator rule, no guarantee `{}` keeps resolving to stateless, and no treatment of payloads that previously fell to `ConnectionParams::Unknown(JsonValue)` / `...MultiTurnConfig::Unknown` and will now decode as typed variants — an observable change for callers matching `Unknown`, even with `#[non_exhaustive]`.
*Repair:* discriminator-first resolution on `type` with documented precedence; frozen fixture pinning `{}` → stateless; fixtures + changelog entry for the `Unknown` → typed-variant migration on both unions.

**B3 (High) — cross-product scope error, including a false assertion.** The report scopes the host discrepancy to Red Team. It applies to **six of seven** specs: runtime management (spec `api.sase.../aisec` vs `DEFAULT_MGMT_ENDPOINT` `api.apps.../aisec`, constants.rs:629) and both Model Security planes (constants.rs:630-631). Worse, the Model Security appendix asserts the SDK configures `https://api.sase.../aims/data` and `/aims/mgmt` and that "No implicit base URL migration is required" — contradicting the exported defaults and the executive table's no-action disposition.
*Repair:* correct the appendix; restate as one cross-product item naming all six affected constants; move the endpoint-profile decision into Chunk A with destination-admission review.

**B4 (Medium-High) — evidence not reproducible from the repo, and misdescribed.** Pinned bytes, normalized snapshots and all three scripts live only in `.local/pan-dev-audit-2026-10-19`, excluded by `.gitignore:3`. Yet the Runtime appendix says the "analysis script in this directory" preserves the evidence and Model Security lists `audit-model-security.py` as an artifact — `docs/research/.../` holds only the README and six JSONs. `inventory.json` SHA256s hash bytes that aren't committed.
*Repair:* Chunk A must vendor the 0.86.0 bytes + license/notice under a versioned path parallel to `contracts/vendor/red-team-data-plane/0.93.0/`; put hash/diff tooling under `tools/`, not `.local`; correct both location claims.

**B5 (Medium-High) — no integration with the repo's own gating machinery.** `AGENTS.md`/`PHASES.md` bind every advance to `python tools/check_phase.py --phase N`, a PHASES.md row, a `docs/phase-status.json` entry and a `docs/reviews/` artifact; highest existing phase is 20. The plan defers identifiers and never mentions check_phase.py, phase-status.json, docs/reviews/, a `docs/<name>-plan.md` scope doc, or `contracts/manifest.json` — the ledger keyed by `upstreamPath` (manifest.json:1861-1875) that must gain the 0.86.0 entry. As written A–E cannot be gated.
*Repair:* milestone A.0 registering phases 21–24, authoring the scope doc, extending check_phase.py, and defining the manifest ID convention for two pinned revisions of one `upstreamPath`.

**B6 (Medium) — two-tree source precedence undeclared, which breaks Plan E.** `contracts/sources/pan-dev/.../red-team/data-plane/dp-openapi.yaml` is still **0.7.75** and flagged `usedForGeneration: true` (manifest.json:1854), while the governing contract is `contracts/vendor/red-team-data-plane/0.93.0/`. `inventory.json` silently picks the vendor pin as "active" for DP and the pan-dev copy for MP, with no rule. Hash-keyed drift audit will report permanent drift on an intentionally frozen file, or mask real drift. The appendix also cites `.../0.93.0/openapi.json` where inventory cites `dp-openapi.yaml`.
*Repair:* explicit per-upstream-path precedence manifest (frozen baseline / active pin / which governs validation and generation) as an A exit artifact; key E on the active pin only.

**B7 (Medium) — release sequence inverted and terminal-CI gate omitted.** Section D says "tag/push/publish", inverting the phase 14/17/20 order: qualify both clean archives → final review → publish → verify registry version and downloaded package → immutable tag → push → terminal remote CI. D's exit criteria omit terminal CI entirely, though PHASES.md requires it for every prior release.
*Repair:* restate D in that order, name `ssh://git@git-ssh.cdot.io/cdot/scm-sdk-rs.git` (AGENTS.md:13), make terminal CI an exit condition.

**B8 (Medium) — filter migration incomplete for values with no successor.** `TargetStatusFilter` narrows from {DRAFT, ACTIVE, FAILED, QUEUED, IN_PROGRESS, COMPLETED, PARTIALLY_COMPLETE} (frozen 5881-5890) to {DRAFT, ACTIVE} (5942-5947). `ProfilingStatus` drops INIT/QUEUED, adds NOT_STARTED, and `ProfilingStatusFilter` omits PARTIALLY_COMPLETE even though `ProfilingStatus` still returns it (3630-3647). Callers passing `status=QUEUED|COMPLETED` will likely now 422, with no equivalent filter for INIT/QUEUED, and PARTIALLY_COMPLETE targets are unfilterable.
*Repair:* value-by-value old → new mapping (marking "no equivalent") in A's classification and the changelog; log PARTIALLY_COMPLETE-absent-from-filter as a vendor defect.

**B9 (Low-Medium) — overstated or thin evidence claims.**
- `runtime-operation-mapping.json` is a restatement of the README table plus a bare `"semantic_source_drift": false` — no operationIds, declared-parameter lists, schema refs or Rust source/line, unlike `red-team-operations.json`. The methodology claim that mappings record "operation parameters, request/success-response schema references ... native methods, validators and generated projections" does not hold for Runtime; that appendix's own P1 ("per-operation mapping rows for all 25") is future work, not delivered evidence.
- Model Security's "no obvious primitive-type narrowing" omits the **18** recorded vendor-`integer` → SDK-`integer|number` widenings in `model-security-schema-comparison.json`; they belong in the retained-differences list beside the OAuth-TTL `FiniteNumber` note.
- RT-5 calls LanguageOptionSchema → LanguageDropdownOptionSchema a "ref rename"; it is an **added** component, `LanguageOptionSchema` still exists unchanged (3038-3051). One shared `LanguageOption` now backs two vendor schemas — benign (`country_code` is `Patch`) but it is an alias decision, not a rename.
- RT-5/P1 miss that the data-plane `TargetConnectionType` (red_team_data_plane.rs:19480-19494) **already has named VERTEXAI and N8N variants** — the obvious reuse precedent for RT-2.

## Verification limits of this review

Read-only, no commands: I hashed nothing, so byte-identity/SHA256 claims for Runtime, Model Security and the broker are cross-checked against `contracts/manifest.json` only. I did not recount the 68 JSON-pointer entries or 21 changed component schemas, and verified the 47 data-plane / 5 broker mappings only by spot check.

## Minimum bar to re-review

Close **B1, B2, B3, B4, B5, B7** in the report and plan text, then request a fresh independent review; B6, B8, B9 are correctable in the same pass. No SDK source change is needed to reach a passing plan score — these are audit-accuracy, scope and governance-wiring repairs. The preservation stance the request emphasized (no removal of SDK extras) is already well handled throughout and should be kept verbatim.