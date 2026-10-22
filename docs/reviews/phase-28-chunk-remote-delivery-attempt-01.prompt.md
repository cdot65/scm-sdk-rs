Review scope: chunk.
Independently review phase 28: Directional profile release and delivery.
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

Active follow-up plan: docs/typescript-035-parity-plan.md. Read it and this phase's scope_evidence.

Reviewed source SHA256: f9cebee5703f3d14f79b91dbbbadcbc538a61cf76ce077ce604a1d69e47c2e58.
Passing check evidence: docs/reviews/phase-28-chunk-remote-delivery-attempt-01.checks.json, SHA256 6f2172ba51014573854f57c104d7f9de4cff81636280441b4eb7ab3692eb2f0e.
Scope evidence: [".forgejo/workflows/ci.yml", "CHANGELOG.md", "Cargo.lock", "Cargo.toml", "README.md", "contracts/compatibility/g25/audit.json", "contracts/compatibility/g25/comparison.json", "contracts/live-evidence.json", "contracts/native-exports.json", "contracts/native-fixtures/gateway-specimens.json", "contracts/native-fixtures/schemas.json", "contracts/native-gateway-bindings.json", "contracts/native-models.json", "contracts/native-operation-mappings/management.json", "contracts/native-operations.json", "contracts/native-probes/Cargo.lock", "contracts/native-probes/Cargo.toml", "contracts/native-semantics.json", "contracts/qualification.json", "contracts/red-team-management-model-overlay.json", "contracts/response-defaults.json", "contracts/typescript-035-management-compatibility.json", "contracts/typescript-035-oracle.json", "contracts/typescript-035-parity.json", "crates/scm-sdk/Cargo.toml", "crates/scm-sdk/examples/directional_profiles.rs", "crates/scm-sdk/src/airs/management/profiles.rs", "crates/scm-sdk/src/fixtures/model-regressions.json", "crates/scm-sdk/src/models/generated.rs", "crates/scm-sdk/src/models/management.rs", "crates/scm-sdk/src/validation/schema_graph.json", "crates/scm-sdk/src/validation/semantics.json", "crates/scm-sdk/src/validation/tests.rs", "crates/scm-sdk/tests/management_contracts.rs", "crates/scm-sdk/tests/schema_contracts.rs", "crates/scm-sdk/tests/typescript035_models.rs", "docs/CI.md", "docs/DEPENDENCIES.md", "docs/DIRECTIONAL_PROFILES.md", "docs/LIVE_ACCEPTANCE.md", "docs/PHASES.md", "docs/RELEASE.md", "docs/RESPONSE_DEFAULTS.md", "docs/STABILITY.md", "docs/phase-status.json", "docs/typescript-035-parity-plan.md", "scripts/check-access-release.py", "scripts/check-dependencies.py", "scripts/check-management-package.py", "scripts/check-package.py", "scripts/check-prepublication.py", "scripts/check-private-material.py", "scripts/check-rust.py", "scripts/observe-release.py", "scripts/verify-publication.py", "tools/check_phase.py", "tools/g25_audit.py", "tools/generate_models.py", "tools/generate_validation.py", "tools/live_evidence.py", "tools/red_team_management.py", "tools/release_delivery.py", "tools/response_defaults.py", "tools/review_phase.py", "tools/schema-oracle.mts", "tools/test_ci.py", "tools/test_dependency_policy.py", "tools/test_g10_release.py", "tools/test_g25.py", "tools/test_live_acceptance_docs.py", "tools/test_live_evidence.py", "tools/test_management_package.py", "tools/test_management_phases.py", "tools/test_model_fixtures.py", "tools/test_package_policy.py", "tools/test_phase_checkpoint.py", "tools/test_red_team_dataplane.py", "tools/test_red_team_management.py", "tools/test_release_022.py", "tools/test_release_023.py", "tools/test_release_024.py", "tools/test_release_delivery.py", "tools/test_review_phase.py", "tools/test_schema_oracle.py", "tools/test_typescript035_governance.py", "tools/test_typescript035_implementation.py", "tools/test_typescript035_oracle.py", "tools/test_typescript035_profiles.py", "tools/test_typescript035_source.py", "tools/typescript035-oracle.mts", "tools/typescript035_profiles.py", "tools/verify-contracts.py", "tools/verify_typescript035_source.py"].
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
Review chunk remote-delivery; attempt 1/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above 40 may proceed with every category >=8
and documented remaining findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: [".forgejo/workflows/ci.yml", "CHANGELOG.md", "README.md", "docs/CI.md", "docs/DIRECTIONAL_PROFILES.md", "docs/PHASES.md", "docs/RELEASE.md", "docs/STABILITY.md", "docs/phase-status.json", "docs/typescript-035-parity-plan.md", "scripts/check-access-release.py", "scripts/check-management-package.py", "scripts/check-package.py", "scripts/check-prepublication.py", "scripts/observe-release.py", "scripts/verify-publication.py", "tools/check_phase.py", "tools/release_delivery.py", "tools/review_phase.py", "tools/test_ci.py", "tools/test_management_package.py", "tools/test_package_policy.py", "tools/test_release_023.py", "tools/test_release_024.py", "tools/test_release_delivery.py", "docs/reviews/recovery-tools/ci-observer/v1/observer.py", "docs/reviews/recovery-tools/ci-observer/v1/test_observer.py", "docs/reviews/recovery-tools/ci-observer/v1/unit-checks.json"].
