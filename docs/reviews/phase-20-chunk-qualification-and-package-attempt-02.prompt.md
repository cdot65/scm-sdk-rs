Review scope: chunk.
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

Reviewed source SHA256: ee60edd531d651fb2583723f101eddd65d297a8c1dad030828092dbe4f267faa.
Passing check evidence: docs/reviews/phase-20-chunk-qualification-and-package-attempt-02.checks.json, SHA256 769d36334216eeeb1bd461badb3a15251f458ee6f1460bc1ba4bc5efb6bbe867.
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
Review chunk qualification-and-package; attempt 2/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above40 may proceed with every category >=8
and documented remaining findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: ["scripts/check-package.py", "scripts/check-access-release.py", "tools/test_release_022.py", "tools/check_phase.py", "tools/review_phase.py", "tools/test_review_phase.py", "tools/release_delivery.py", "tools/test_release_delivery.py", "crates/scm-sdk/examples/response_compatibility.rs", "CHANGELOG.md", "docs/G25_COMPATIBILITY.md", "docs/RELEASE.md", "docs/PACKAGING.md", "docs/CI.md", "docs/LIVE_ACCEPTANCE.md", "docs/checks/phase-20-release-package-1.89.0.json", "docs/checks/phase-20-release-package-1.99.0.json", "crates/scm-sdk/src/validation.rs", "crates/scm-sdk/src/validation/tests.rs", "tools/g25_fixtures.py", "tools/test_g25_generation.py", "tools/g25_report.py", "tools/test_g25_report.py", "crates/scm-sdk/tests/g25_response_compatibility.rs", "contracts/compatibility/g25/projections.json", "docs/SECURITY.md"].
