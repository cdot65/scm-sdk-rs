Review scope: chunk.
Independently review phase 21: AIRS source inventory and management compatibility plan.
Read AGENTS.md, CLAUDE.md, docs/PHASES.md, docs/implementation-plan.md and
docs/phase-status.json, then inspect all actual artifacts relevant to this phase.
Assess functionality, security, contract completeness, maintainability and check evidence.
Give a candid score from 1 to 10. The threshold is 9, but do not inflate the score to
meet it. Report concrete required fixes with paths and lines. Judge this phase's
acceptance criteria, not unimplemented later phases. Verify evidence rather than
trusting completion prose. You have read-only Read/Glob/Grep tools: inspect recorded
checks and distinguish unexecuted checks. Do not read any credential/tenant-registration
files or anything outside this repository. .local/typescript is a frozen nonsecret source
reference and may be inspected. Do not modify files or perform network calls.

Active follow-up plan: docs/red-team-management-0.86-plan.md. Read it and this phase's scope_evidence.

Reviewed source SHA256: e2a407ab7c6f413a9b6651011e00c4b9f765dee0761f0e29b2710eee9d308694.
Passing check evidence: docs/reviews/phase-21-chunk-plan-and-provenance-attempt-03.checks.json, SHA256 1b7cd2b3af86b19bda196360279d4e21e71917972e23acc1937f2c882d1c0dbd.
Scope evidence: ["docs/red-team-management-0.86-plan.md"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.

The maintainer's current five-category rubric supersedes the older /10 threshold.
Review chunk plan-and-provenance; attempt 3/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above40 may proceed with every category >=8
and documented remaining findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: ["docs/red-team-management-0.86-plan.md", "contracts/vendor/red-team-management/0.86.0", "contracts/manifest.json", "tools/red_team_management.py", "tools/verify-contracts.py", "contracts/airs-active-sources.json", "contracts/airs-operations.json", "contracts/airs-retained-capabilities.json", "tools/test_red_team_management.py", "contracts/compatibility/airs-0.2.2-retention.json", "docs/implementation-plan.md", "contracts/native-fixtures/schemas.json", "crates/scm-sdk/src/fixtures/model-regressions.json", "contracts/native-fixtures/gateway-specimens.json"].
