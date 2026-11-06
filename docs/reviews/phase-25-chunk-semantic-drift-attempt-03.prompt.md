Review scope: chunk.
Independently review phase 25: Prisma AIRS upstream drift detection.
Read AGENTS.md, CLAUDE.md, docs/PHASES.md, docs/implementation-plan.md and
docs/phase-status.json, then inspect all actual artifacts relevant to this phase.
Assess functionality, security, contract completeness, maintainability and check evidence.
Give a candid score from 1 to 10. The threshold is 9, but do not inflate the score to
meet it. Report concrete required fixes with paths and lines. Judge this phase's
acceptance criteria, not unimplemented later phases. Verify evidence rather than
trusting completion prose. You have read-only Read/Glob/Grep tools: inspect recorded
checks and distinguish unexecuted checks. Do not read any credential/tenant-registration
files or anything outside this repository. Use the pinned source artifacts declared
in this phase’s plan; an optional .local/typescript checkout is not required. Do not modify files or perform network calls.

Active follow-up plan: docs/red-team-management-0.86-plan.md. Read it and this phase's scope_evidence.

Reviewed source SHA256: beeec1f77fa53aeffadcbc139943bb36c63673c2f8f233172b96889a160c7a24.
Passing check evidence: docs/reviews/phase-25-chunk-semantic-drift-attempt-03.checks.json, SHA256 c9e007f64939f83276693f47433e29ee321310e1169acb5a0ca90c7c5b5cfd2b.
Scope evidence: [".forgejo/workflows/ci.yml", "contracts/airs-drift.json", "contracts/vendor/red-team-management/0.86.0", "docs/CI.md", "docs/PHASES.md", "docs/RELEASE.md", "docs/red-team-management-0.86-plan.md", "tools/airs_drift.py", "tools/check_phase.py", "tools/historical_delivery.py", "tools/red_team_management.py", "tools/review_phase.py", "tools/test_airs_drift.py", "tools/test_ci.py", "tools/test_historical_delivery.py", "tools/test_management_phases.py", "tools/test_red_team_management.py", "tools/test_review_phase.py"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.
Keep the summary under 150 words and findings concise. For a follow-up, focus on
changed artifacts and disposition of prior findings; avoid repeating resolved
technical explanations. Include enough concrete evidence to justify each score.

The maintainer's current five-category rubric supersedes the older /10 threshold.
Review chunk semantic-drift; attempt 3/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above 40 may proceed with every category >=8
and documented remaining findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: ["tools/airs_drift.py", "tools/test_airs_drift.py", "contracts/airs-drift.json", "tools/red_team_management.py", "tools/test_red_team_management.py", "contracts/vendor/red-team-management/0.86.0", "tools/check_phase.py", "tools/test_management_phases.py", "docs/red-team-management-0.86-plan.md", "docs/PHASES.md"].
