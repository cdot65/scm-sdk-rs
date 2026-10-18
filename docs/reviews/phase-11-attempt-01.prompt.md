Independently review phase 11: Red Team dataplane operation completion.
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

Reviewed source SHA256: 365b7d7a3410e3a20009fe18496f8fd72a5f30a9866ca00ec31dafdaed6a470f.
Passing check evidence: docs/checks/phase-11.json, SHA256 e69a68db09bf4307d793b2cdb974f70571ac63ba168329cdb0d95ad229f31ea1.
Active follow-up plan: docs/red-team-data-plane-0.93.0-plan.md.
Scope evidence: ["docs/red-team-data-plane-0.93.0-plan.md", "docs/RED_TEAM.md", "crates/scm-sdk/src/airs/red_team/", "crates/scm-sdk/tests/red_team_contracts.rs", "crates/scm-sdk/tests/red_team_vendor_models_generated.rs", "tools/make-red-team-wire-fixtures.mjs", "tools/make-red-team-fixtures.mjs", "tools/red-team-specimens.mjs", "tools/red_team_dataplane.py", "tools/test_red_team_dataplane.py", "contracts/vendor/red-team-data-plane/0.93.0/wire-fixtures.json", "contracts/vendor/red-team-data-plane/0.93.0/operations.json"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.
