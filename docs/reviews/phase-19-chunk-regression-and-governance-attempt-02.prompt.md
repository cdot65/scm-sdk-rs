Independently review phase 19: G25 response compatibility implementation.
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

Active follow-up plan: docs/g25-response-compatibility-plan.md. Read it and this phase's scope_evidence.

Reviewed source SHA256: 7edc9306659f009718116c0e170d0c20dffd72eac78a1e88e190f1a401e0c445.
Passing check evidence: docs/reviews/phase-19-chunk-regression-and-governance-attempt-02.checks.json, SHA256 4c02ab14bcdc1450edb6973b88cd0d1e3b11e0e5d7cd60ce27d9616f744f0105.
Active follow-up plan: docs/g25-response-compatibility-plan.md.
Scope evidence: ["docs/g25-response-compatibility-plan.md", "contracts/compatibility/g25/audit.json", "tools/g25_audit.py", "crates/scm-sdk/src/validation.rs", "tools/red_team_dataplane.py", "tools/generate_models.py", "crates/scm-sdk/tests/g25_response_compatibility.rs", "crates/scm-sdk/tests/red_team_contracts.rs", "tools/g25_fixtures.py", "tools/g25-oracle.mts", "docs/G25_COMPATIBILITY.md", "contracts/compatibility/g25/comparison.json", "tools/review_phase.py", "tools/test_review_phase.py", "docs/checks/phase-19-python-preflight.json", "docs/checks/phase-19-rust-1.99.0-json-unification.json", "scripts/check-rust.py", "tools/test_g25_generation.py", "tools/test_model_fixtures.py"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.

The maintainer's current five-category rubric supersedes the older /10 threshold.
Review chunk regression-and-governance; attempt 2/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above40 may proceed with documented remaining
findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: ["tools/g25_fixtures.py", "tools/g25-oracle.mts", "tools/g25_audit.py", "tools/g25_report.py", "docs/G25_COMPATIBILITY.md", "tools/review_phase.py", "tools/test_review_phase.py", "docs/checks/phase-19-python-preflight.json"].
