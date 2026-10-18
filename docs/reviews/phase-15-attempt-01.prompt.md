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

Reviewed source SHA256: de5a8957e6093429202febecc72a42752f0f274f13368c0837668b4b50ffa48b.
Passing check evidence: docs/checks/phase-15.json, SHA256 b556dd83b8da3e2df23584bf777cff06408aaee6960729848ecbb11305dd9247.
Active follow-up plan: docs/portkey-access-policies-plan.md.
Scope evidence: ["contracts/vendor/portkey-access/a27feb9/source.json", "contracts/vendor/portkey-access/a27feb9/contract.json", "docs/portkey-access-policies-plan.md", "tools/portkey-access.mjs"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.
