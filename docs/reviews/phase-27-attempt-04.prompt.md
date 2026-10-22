Review scope: phase.
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

Reviewed source SHA256: a04d4f9ae9fdcffa2eb54f7a82f256c5c4cce35f766effebe6899424383123ba.
Passing check evidence: docs/reviews/phase-27-attempt-04.checks.json, SHA256 894c075608cc47ec52b0563c663a6f6805f28d0aa7a491caa8b895d6144ebf6c.
Scope evidence: ["CHANGELOG.md", "contracts/compatibility/g25/audit.json", "contracts/compatibility/g25/comparison.json", "contracts/live-evidence.json", "contracts/native-exports.json", "contracts/native-fixtures/gateway-specimens.json", "contracts/native-models.json", "contracts/native-operations.json", "contracts/native-semantics.json", "contracts/qualification.json", "contracts/red-team-management-model-overlay.json", "contracts/response-defaults.json", "contracts/typescript-035-oracle.json", "contracts/typescript-035-parity.json", "crates/scm-sdk/Cargo.toml", "crates/scm-sdk/examples/directional_profiles.rs", "crates/scm-sdk/src/airs/management/profiles.rs", "crates/scm-sdk/src/models/generated.rs", "crates/scm-sdk/src/models/management.rs", "crates/scm-sdk/src/validation/schema_graph.json", "crates/scm-sdk/src/validation/semantics.json", "crates/scm-sdk/src/validation/tests.rs", "crates/scm-sdk/tests/management_contracts.rs", "crates/scm-sdk/tests/typescript035_models.rs", "docs/DIRECTIONAL_PROFILES.md", "docs/PHASES.md", "docs/RESPONSE_DEFAULTS.md", "docs/STABILITY.md", "docs/typescript-035-parity-plan.md", "scripts/check-rust.py", "tools/check_phase.py", "tools/g25_audit.py", "tools/generate_models.py", "tools/generate_validation.py", "tools/live_evidence.py", "tools/red_team_management.py", "tools/response_defaults.py", "tools/review_phase.py", "tools/test_ci.py", "tools/test_g25.py", "tools/test_live_evidence.py", "tools/test_management_phases.py", "tools/test_red_team_management.py", "tools/test_typescript035_implementation.py", "tools/test_typescript035_oracle.py", "tools/test_typescript035_profiles.py", "tools/test_typescript035_source.py", "tools/typescript035-oracle.mts", "tools/typescript035_profiles.py", "tools/verify-contracts.py", "tools/verify_typescript035_source.py", "contracts/typescript-035-management-compatibility.json", "crates/scm-sdk/tests/schema_contracts.rs", "contracts/native-gateway-bindings.json", "tools/test_g10_release.py", "docs/LIVE_ACCEPTANCE.md", "tools/test_live_acceptance_docs.py", "tools/schema-oracle.mts", "tools/test_schema_oracle.py", "contracts/native-fixtures/schemas.json", "tools/test_model_fixtures.py", "crates/scm-sdk/src/fixtures/model-regressions.json", "Cargo.lock", "tools/test_phase_checkpoint.py", "tools/test_red_team_dataplane.py", "tools/test_typescript035_governance.py", "contracts/native-probes/Cargo.lock", "contracts/native-operation-mappings/management.json", "tools/test_review_phase.py"].
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
Review the complete phase; attempt 4/5.
Score implementation, code_quality, supportability, architecture and feature_delivery
separately from 0 to 10. Return categories and their exact sum as total (/50).
Retain score (/10) as a legacy summary only; it does not decide acceptance.
Each category must be >=8; chunks require >=42/50 (aim >=45); phases require >=46/50.
On attempt five only, a total strictly above 40 may proceed with every category >=8
and documented remaining findings or limitations. Blocker/high findings still block acceptance. Do not inflate scores.
Chunk scope artifacts: ["CHANGELOG.md", "contracts/compatibility/g25/audit.json", "contracts/compatibility/g25/comparison.json", "contracts/live-evidence.json", "contracts/native-exports.json", "contracts/native-fixtures/gateway-specimens.json", "contracts/native-models.json", "contracts/native-operations.json", "contracts/native-semantics.json", "contracts/qualification.json", "contracts/red-team-management-model-overlay.json", "contracts/response-defaults.json", "contracts/typescript-035-oracle.json", "contracts/typescript-035-parity.json", "crates/scm-sdk/Cargo.toml", "crates/scm-sdk/examples/directional_profiles.rs", "crates/scm-sdk/src/airs/management/profiles.rs", "crates/scm-sdk/src/models/generated.rs", "crates/scm-sdk/src/models/management.rs", "crates/scm-sdk/src/validation/schema_graph.json", "crates/scm-sdk/src/validation/semantics.json", "crates/scm-sdk/src/validation/tests.rs", "crates/scm-sdk/tests/management_contracts.rs", "crates/scm-sdk/tests/typescript035_models.rs", "docs/DIRECTIONAL_PROFILES.md", "docs/PHASES.md", "docs/RESPONSE_DEFAULTS.md", "docs/STABILITY.md", "docs/typescript-035-parity-plan.md", "scripts/check-rust.py", "tools/check_phase.py", "tools/g25_audit.py", "tools/generate_models.py", "tools/generate_validation.py", "tools/live_evidence.py", "tools/red_team_management.py", "tools/response_defaults.py", "tools/review_phase.py", "tools/test_ci.py", "tools/test_g25.py", "tools/test_live_evidence.py", "tools/test_management_phases.py", "tools/test_red_team_management.py", "tools/test_typescript035_implementation.py", "tools/test_typescript035_oracle.py", "tools/test_typescript035_profiles.py", "tools/test_typescript035_source.py", "tools/typescript035-oracle.mts", "tools/typescript035_profiles.py", "tools/verify-contracts.py", "tools/verify_typescript035_source.py", "contracts/typescript-035-management-compatibility.json", "crates/scm-sdk/tests/schema_contracts.rs", "contracts/native-gateway-bindings.json", "tools/test_g10_release.py", "docs/LIVE_ACCEPTANCE.md", "tools/test_live_acceptance_docs.py", "tools/schema-oracle.mts", "tools/test_schema_oracle.py", "contracts/native-fixtures/schemas.json", "tools/test_model_fixtures.py", "crates/scm-sdk/src/fixtures/model-regressions.json", "Cargo.lock", "tools/test_phase_checkpoint.py", "tools/test_red_team_dataplane.py", "tools/test_typescript035_governance.py", "contracts/native-probes/Cargo.lock", "contracts/native-operation-mappings/management.json", "tools/test_review_phase.py"].

Inspect the latest prior phase findings, phase-status review_findings_disposition,
follow-up scope_evidence, and all declared chunks. Where an accepted fifth chunk
review predates phase-requested corrections, explicitly assess those corrections
and their regressions against the complete current phase. The original chunk
review remains historical evidence; this phase review must cover current source.
