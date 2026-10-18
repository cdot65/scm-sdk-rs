Independently review phase 15: Portkey access-policy plan.
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

Active follow-up plan: docs/portkey-access-policies-plan.md. Read it and this phase's scope_evidence.

Reviewed source SHA256: 75b6b6c1c424961f4038836a176cf720e94afd0a43f8e5d25f668d48bbc53676.
Passing check evidence: docs/checks/phase-15.json, SHA256 df8048f74360c51addc9b7969824c989ab11dac568dbd27cff756175398d4a2b.
Active follow-up plan: docs/portkey-access-policies-plan.md.
Scope evidence: ["contracts/vendor/portkey-access/a27feb9/source.json", "contracts/vendor/portkey-access/a27feb9/contract.json", "docs/portkey-access-policies-plan.md", "tools/portkey-access.mjs", "contracts/vendor/portkey-access/a27feb9/openapi.yaml", "contracts/vendor/portkey-access/a27feb9/parent.yaml", "contracts/vendor/portkey-access/a27feb9/LICENSE", "tools/test_portkey_access.py", "docs/portkey-access-policies-research.md"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.
