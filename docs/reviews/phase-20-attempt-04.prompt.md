Review scope: phase.
Independently review phase 20: G25 0.2.2 qualification and delivery.
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

Reviewed source SHA256: 519219d614183f53ed2d0e4daadfb5a09c1bf69b23d5ae45e08ea8ae7bb1aee3.
Passing check evidence: docs/reviews/phase-20-attempt-04.checks.json, SHA256 b75e9709dca7e70c9b54392d47586cfa67a37931137b3fabb7850a8f10228a82.
Active follow-up plan: docs/g25-response-compatibility-plan.md.
Scope evidence: ["docs/g25-response-compatibility-plan.md", "docs/G25_COMPATIBILITY.md", "CHANGELOG.md", "docs/RELEASE.md", "docs/PACKAGING.md", "docs/CI.md", "docs/LIVE_ACCEPTANCE.md", "tools/check_phase.py", "tools/review_phase.py", "tools/release_delivery.py", "tools/test_release_delivery.py", "scripts/check-package.py", "scripts/check-access-release.py", "tools/test_release_022.py", "crates/scm-sdk/examples/response_compatibility.rs", "crates/scm-sdk/src/validation.rs", "crates/scm-sdk/src/validation/tests.rs", "tools/g25_fixtures.py", "tools/test_g25_generation.py", "tools/g25_report.py", "tools/test_g25_report.py", "crates/scm-sdk/tests/g25_response_compatibility.rs", "contracts/compatibility/g25/projections.json", "docs/SECURITY.md"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.

The maintainer's current five-category rubric supersedes the older /10 threshold.
Review the complete phase; attempt 4/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above40 may proceed with every category >=8
and documented remaining findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: [].

Inspect the latest prior phase findings, phase-status review_findings_disposition,
follow-up scope_evidence, and all declared chunks. Where an accepted fifth chunk
review predates phase-requested corrections, explicitly assess those corrections
and their regressions against the complete current phase. The original chunk
review remains historical evidence; this phase review must cover current source.
