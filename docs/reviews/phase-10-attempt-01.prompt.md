Independently review phase 10: Red Team vendor contracts and model evolution.
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

Reviewed source SHA256: 854a7f53798a8c8808889b3301a770e714b6d113218dd0826b2bfb59b414fd3a.
Passing check evidence: docs/checks/phase-10.json, SHA256 fc1a35dcc73cd6dc51e7d9b6737f22cdbe669a1e572ce3660ac51dcdb57a46cf.
Active follow-up plan: docs/red-team-data-plane-0.93.0-plan.md.
Scope evidence: ["tools/red_team_dataplane.py", "tools/generate_models.py", "tools/make-red-team-fixtures.mjs", "tools/red-team-dataplane.mjs", "tools/test_red_team_dataplane.py", "contracts/vendor/red-team-data-plane/0.93.0/", "crates/scm-sdk/src/airs/red_team/", "crates/scm-sdk/src/validation.rs", "crates/scm-sdk/src/models/red_team_data_plane.rs", "crates/scm-sdk/tests/red_team_vendor_models_generated.rs", "crates/scm-sdk/tests/red_team_contracts.rs"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.
