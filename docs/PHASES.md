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
