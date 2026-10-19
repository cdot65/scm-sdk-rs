Review scope: chunk.
Independently review phase 23: AIRS management compatibility and security qualification.
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

Reviewed source SHA256: 79f03235ed349d674019b834fc9304b110273738f6f213a784b6bcf983b3965e.
Passing check evidence: docs/reviews/phase-23-chunk-package-security-attempt-02.checks.json, SHA256 955bd2a91919980c8a4bc1c22c11f5737a3dbbc56d4bc0a34cd4345bacdf8fc5.
Scope evidence: [".forgejo/workflows/ci.yml", "CHANGELOG.md", "NOTICE", "README.md", "contracts/compatibility/g25/audit.json", "contracts/compatibility/g25/comparison.json", "contracts/compatibility/management-old-consumers.json", "contracts/compatibility/management-old-consumers/0.2.2/native-probes-main.rs", "contracts/compatibility/management-old-consumers/0.2.2/red_team_dataplane.rs", "contracts/compatibility/management-old-consumers/0.2.2/response_compatibility.rs", "contracts/compatibility/management-old-consumers/0.2.2/response_defaults.rs", "contracts/manifest.json", "contracts/native-exports.json", "contracts/native-fixtures/gateway-specimens.json", "contracts/native-fixtures/red-team-management.json", "contracts/native-fixtures/schemas.json", "contracts/native-gateway-bindings.json", "contracts/native-models.json", "contracts/native-operations.json", "contracts/qualification.json", "contracts/red-team-management-model-overlay.json", "contracts/red-team-management-union-extensions.json", "contracts/response-defaults.json", "contracts/vendor/red-team-management/0.86.0/compatibility-decisions.json", "contracts/vendor/red-team-management/0.86.0/operations.json", "crates/scm-sdk/NOTICE", "crates/scm-sdk/examples/red_team_management.rs", "crates/scm-sdk/src/airs/red_team/targets.rs", "crates/scm-sdk/src/fixtures/model-regressions.json", "crates/scm-sdk/src/models/generated.rs", "crates/scm-sdk/src/models/red_team_management.rs", "crates/scm-sdk/src/validation/schema_graph.json", "crates/scm-sdk/tests/red_team_management_contracts.rs", "crates/scm-sdk/tests/red_team_management_fixtures.rs", "crates/scm-sdk/tests/red_team_management_models.rs", "docs/CI.md", "docs/G25_COMPATIBILITY.md", "docs/PHASES.md", "docs/RELEASE.md", "docs/red-team-management-0.86-plan.md", "scripts/check-management-package.py", "scripts/check-package.py", "tools/check_phase.py", "tools/g25_report.py", "tools/generate_models.py", "tools/management_consumers.py", "tools/management_fixtures.py", "tools/red_team_management.py", "tools/red_team_management_models.py", "tools/review_phase.py", "tools/test_ci.py", "tools/test_g25_report.py", "tools/test_management_consumers.py", "tools/test_management_fixtures.py", "tools/test_management_package.py", "tools/test_management_phases.py", "tools/test_phase_checkpoint.py", "tools/test_red_team_management.py", "tools/test_red_team_management_models.py", "tools/test_review_phase.py"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.

The maintainer's current five-category rubric supersedes the older /10 threshold.
Review chunk package-security; attempt 2/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above 40 may proceed with every category >=8
and documented remaining findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: [".forgejo/workflows/ci.yml", "CHANGELOG.md", "README.md", "contracts/compatibility/g25/audit.json", "contracts/compatibility/g25/comparison.json", "contracts/compatibility/management-old-consumers.json", "contracts/compatibility/management-old-consumers/0.2.2/native-probes-main.rs", "contracts/compatibility/management-old-consumers/0.2.2/red_team_dataplane.rs", "contracts/compatibility/management-old-consumers/0.2.2/response_compatibility.rs", "contracts/compatibility/management-old-consumers/0.2.2/response_defaults.rs", "contracts/manifest.json", "contracts/native-exports.json", "contracts/native-fixtures/gateway-specimens.json", "contracts/native-fixtures/schemas.json", "contracts/native-gateway-bindings.json", "contracts/native-models.json", "contracts/native-operations.json", "contracts/qualification.json", "contracts/response-defaults.json", "contracts/vendor/red-team-management/0.86.0/compatibility-decisions.json", "contracts/vendor/red-team-management/0.86.0/operations.json", "docs/CI.md", "docs/PHASES.md", "docs/RELEASE.md", "scripts/check-management-package.py", "scripts/check-package.py", "tools/check_phase.py", "tools/management_consumers.py", "tools/red_team_management.py", "tools/review_phase.py", "tools/test_ci.py", "tools/test_management_consumers.py", "tools/test_management_package.py", "tools/test_management_phases.py", "tools/test_phase_checkpoint.py", "tools/test_red_team_management.py", "tools/test_review_phase.py"].
