Review scope: chunk.
Independently review phase 27: Directional profile implementation and qualification.
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

Reviewed source SHA256: 9761d2cef77fefd89b27cefca25db51debc503eac5df34e5e2aa4222f397fb7b.
Passing check evidence: docs/reviews/phase-27-chunk-wire-and-compatibility-attempt-01.checks.json, SHA256 3807bc27c784b5b7e50fb0375b5b78db5e58486e120a30fe99040b8dc8401545.
Scope evidence: ["CHANGELOG.md", "contracts/compatibility/g25/audit.json", "contracts/compatibility/g25/comparison.json", "contracts/live-evidence.json", "contracts/native-exports.json", "contracts/native-fixtures/gateway-specimens.json", "contracts/native-models.json", "contracts/native-operations.json", "contracts/native-semantics.json", "contracts/qualification.json", "contracts/red-team-management-model-overlay.json", "contracts/response-defaults.json", "contracts/typescript-035-oracle.json", "contracts/typescript-035-parity.json", "crates/scm-sdk/Cargo.toml", "crates/scm-sdk/examples/directional_profiles.rs", "crates/scm-sdk/src/airs/management/profiles.rs", "crates/scm-sdk/src/models/generated.rs", "crates/scm-sdk/src/models/management.rs", "crates/scm-sdk/src/validation/schema_graph.json", "crates/scm-sdk/src/validation/semantics.json", "crates/scm-sdk/src/validation/tests.rs", "crates/scm-sdk/tests/management_contracts.rs", "crates/scm-sdk/tests/typescript035_models.rs", "docs/DIRECTIONAL_PROFILES.md", "docs/PHASES.md", "docs/RESPONSE_DEFAULTS.md", "docs/STABILITY.md", "docs/typescript-035-parity-plan.md", "scripts/check-rust.py", "tools/check_phase.py", "tools/g25_audit.py", "tools/generate_models.py", "tools/generate_validation.py", "tools/live_evidence.py", "tools/red_team_management.py", "tools/response_defaults.py", "tools/review_phase.py", "tools/test_ci.py", "tools/test_g25.py", "tools/test_live_evidence.py", "tools/test_management_phases.py", "tools/test_red_team_management.py", "tools/test_typescript035_implementation.py", "tools/test_typescript035_oracle.py", "tools/test_typescript035_profiles.py", "tools/test_typescript035_source.py", "tools/typescript035-oracle.mts", "tools/typescript035_profiles.py", "tools/verify-contracts.py", "tools/verify_typescript035_source.py", "contracts/typescript-035-management-compatibility.json", "crates/scm-sdk/tests/schema_contracts.rs", "contracts/native-gateway-bindings.json", "tools/test_g10_release.py", "docs/LIVE_ACCEPTANCE.md", "tools/test_live_acceptance_docs.py", "tools/schema-oracle.mts", "tools/test_schema_oracle.py", "contracts/native-fixtures/schemas.json", "tools/test_model_fixtures.py", "crates/scm-sdk/src/fixtures/model-regressions.json", "Cargo.lock", "tools/test_phase_checkpoint.py", "tools/test_red_team_dataplane.py"].
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
Review chunk wire-and-compatibility; attempt 1/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above 40 may proceed with every category >=8
and documented remaining findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: ["crates/scm-sdk/src/airs/management/profiles.rs", "crates/scm-sdk/tests/typescript035_models.rs", "crates/scm-sdk/tests/management_contracts.rs", "crates/scm-sdk/examples/directional_profiles.rs", "crates/scm-sdk/Cargo.toml", "tools/test_typescript035_implementation.py", "tools/test_management_phases.py", "tools/check_phase.py", "tools/review_phase.py", "scripts/check-rust.py", "CHANGELOG.md", "docs/STABILITY.md", "docs/DIRECTIONAL_PROFILES.md", "docs/RESPONSE_DEFAULTS.md", "contracts/native-operations.json", "contracts/native-exports.json", "contracts/qualification.json", "docs/PHASES.md", "tools/test_ci.py", "tools/g25_audit.py", "tools/test_g25.py", "contracts/compatibility/g25/audit.json", "contracts/compatibility/g25/comparison.json", "tools/live_evidence.py", "tools/test_live_evidence.py", "contracts/live-evidence.json", "tools/red_team_management.py", "tools/test_red_team_management.py", "docs/typescript-035-parity-plan.md", "contracts/typescript-035-management-compatibility.json", "crates/scm-sdk/tests/schema_contracts.rs", "contracts/native-gateway-bindings.json", "tools/test_g10_release.py", "docs/LIVE_ACCEPTANCE.md", "tools/test_live_acceptance_docs.py", "Cargo.lock", "tools/test_phase_checkpoint.py", "tools/test_red_team_dataplane.py"].
