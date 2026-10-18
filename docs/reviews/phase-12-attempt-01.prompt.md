Independently review phase 12: Red Team dataplane qualification and delivery.
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

Active follow-up plan: docs/red-team-data-plane-0.93.0-plan.md. Read it and this phase's scope_evidence.

Reviewed source SHA256: de311207659ad73a8a158b79e262714b24e5c158441ee3d40adbe5b4e5e98bce.
Passing check evidence: docs/checks/phase-12.json, SHA256 960f1aeb683b9b1db3a08aec31aa0e9821bee0b8be083c1497a0597298201ffc.
Active follow-up plan: docs/red-team-data-plane-0.93.0-plan.md.
Scope evidence: ["docs/red-team-data-plane-0.93.0-plan.md", "CHANGELOG.md", "README.md", "docs/RED_TEAM.md", "docs/VALIDATION.md", "docs/CI.md", "docs/RELEASE.md", "docs/PACKAGING.md", "docs/PLATFORMS.md", "docs/LIVE_ACCEPTANCE.md", "crates/scm-sdk/examples/red_team_dataplane.rs", "tools/red_team_dataplane.py", "tools/test_red_team_dataplane.py", "crates/scm-sdk/tests/red_team_contracts.rs", "tools/check_phase.py", ".forgejo/workflows/ci.yml", "scripts/check-package.py", "contracts/vendor/red-team-data-plane/0.93.0/operations.json", "tools/test_ci.py", "docs/checks/phase-08-live-attempt-11.json", "tools/test_model_fixtures.py", "docs/checks/phase-12-oracle-provenance-refresh.json", "docs/checks/phase-08-live-attempt-12.json"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.
