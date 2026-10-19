Independently review phase 18: G25 response compatibility plan and audit.
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

Reviewed source SHA256: 8db994cfb5f688546d691a7daef37ce3fe7b40d8cf7fe7fb5b9542f1c43bc99f.
Passing check evidence: docs/checks/phase-18.json, SHA256 85885b9a59ae9c351ae9d8798c96ca5d4f6c319f530e1573c2d0aaf42b696cc1.
Active follow-up plan: docs/g25-response-compatibility-plan.md.
Scope evidence: ["docs/g25-response-compatibility-plan.md", "contracts/compatibility/g25/audit.json", "tools/g25_audit.py"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.
