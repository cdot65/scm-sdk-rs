Independently review phase 17: Portkey access-policy qualification and publication.
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

Reviewed source SHA256: ca1e71973db1bc0aef8aad57466948ef5edd664bc204d7e5e3d518a2efd72be9.
Passing check evidence: docs/checks/phase-17.json, SHA256 f82082f1ed07d6e279ce607d20c8217d1c9d6f2232c279fb4bc402629e80b112.
Active follow-up plan: docs/portkey-access-policies-plan.md.
Scope evidence: ["contracts/vendor/portkey-access/a27feb9/source.json", "contracts/vendor/portkey-access/a27feb9/contract.json", "docs/portkey-access-policies-plan.md", "tools/portkey-access.mjs", "contracts/vendor/portkey-access/a27feb9/openapi.yaml", "contracts/vendor/portkey-access/a27feb9/parent.yaml", "contracts/vendor/portkey-access/a27feb9/LICENSE", "tools/test_portkey_access.py", "docs/portkey-access-policies-research.md", "tools/check_phase.py", "docs/PHASES.md", "docs/implementation-plan.md", "docs/checks/phase-15.json", "crates/scm-sdk/src/airs/gateway/access_policies.rs", "crates/scm-sdk/src/airs/gateway/access_policy_models.rs", "crates/scm-sdk/tests/portkey_access_contracts.rs", "crates/scm-sdk/examples/access_policies.rs", "contracts/vendor/portkey-access/a27feb9/model-fixtures.json", "contracts/vendor/portkey-access/a27feb9/operations.json", "docs/checks/phase-16.json", "docs/checks/phase-16-rust-1.89.0-gateway.json", "docs/checks/phase-16-rust-1.99.0-gateway.json", "docs/ACCESS_POLICIES.md", "CHANGELOG.md", "README.md", "NOTICE", "scripts/check-access-release.py", "scripts/check-package.py", ".forgejo/workflows/ci.yml", "docs/checks/phase-17-release-package-1.89.0.json", "docs/checks/phase-17-release-package-1.99.0.json", "docs/checks/phase-17-access-live.json", "docs/checks/phase-17-candidate.json", "docs/checks/phase-17-dependencies.json", "docs/checks/phase-17-private-material.json", "docs/checks/phase-17-rust-1.89.0-all.json", "docs/checks/phase-17-rust-1.89.0-default.json", "docs/checks/phase-17-rust-1.89.0-minimal.json", "docs/checks/phase-17-rust-1.99.0-all.json", "docs/checks/phase-17-rust-1.99.0-default-tracing.json", "docs/checks/phase-17-rust-1.99.0-default.json", "docs/checks/phase-17-rust-1.99.0-gateway-json-unification.json", "docs/checks/phase-17-rust-1.99.0-gateway.json", "docs/checks/phase-17-rust-1.99.0-minimal.json", "docs/checks/phase-17.json", "crates/scm-sdk/Cargo.toml", "docs/RELEASE.md", "docs/STABILITY.md", "docs/DEVELOPMENT.md", "tools/test_release_020.py", "docs/checks/release-registry-0.2.1.json"].
For a vendor follow-up phase, focus on the active milestones and relevant cumulative
regressions. Generated model correctness is assessed through the compiler, selected
generated declarations and independent fixture/check evidence. Later-phase operations
marked planned are not a missing deliverable of the contracts/models phase. Historical
release evidence does not establish new-source acceptance; avoid re-auditing unrelated
historical infrastructure when evaluating this scoped change.
Return the requested structured review with score, summary, findings, evidence, limitations.
