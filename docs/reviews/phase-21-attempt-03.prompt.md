Review scope: phase.
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

Reviewed source SHA256: cf84fe545acaee705eef797f3961cf40526f2cd384fd9c057eac7fae0c68e1ec.
Passing check evidence: docs/reviews/phase-21-attempt-03.checks.json, SHA256 3b334c16bbda05c8b5931f758184f09db2b0a16cb99941f3ad80f6f30b43bc32.
Scope evidence: [".forgejo/workflows/ci.yml", "contracts/airs-active-sources.json", "contracts/airs-operations.json", "contracts/airs-retained-capabilities.json", "contracts/compatibility/airs-0.2.2-retention.json", "contracts/manifest.json", "contracts/native-fixtures/gateway-specimens.json", "contracts/native-fixtures/schemas.json", "contracts/vendor/red-team-management/0.86.0", "crates/scm-sdk/src/fixtures/model-regressions.json", "docs/CI.md", "docs/PHASES.md", "docs/RELEASE.md", "docs/implementation-plan.md", "docs/red-team-management-0.86-plan.md", "tools/check_phase.py", "tools/red_team_management.py", "tools/review_phase.py", "tools/test_ci.py", "tools/test_management_phases.py", "tools/test_red_team_management.py", "tools/test_review_phase.py", "tools/verify-contracts.py"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.

The maintainer's current five-category rubric supersedes the older /10 threshold.
Review the complete phase; attempt 3/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above 40 may proceed with every category >=8
and documented remaining findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: [].

This phase is also a plan gate: score must be >=9/10 in addition to the five-category rubric.

Inspect the latest prior phase findings, phase-status review_findings_disposition,
follow-up scope_evidence, and all declared chunks. Where an accepted fifth chunk
review predates phase-requested corrections, explicitly assess those corrections
and their regressions against the complete current phase. The original chunk
review remains historical evidence; this phase review must cover current source.
