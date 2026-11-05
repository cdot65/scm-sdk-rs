You are the independent reviewer for chunk `models-and-validation` of phase 30 (Portkey MCP tags implementation and qualification) of the scm-sdk Rust SDK at the current working directory.

Follow AGENTS.md and CLAUDE.md review context. Read docs/portkey-mcp-tags-plan.md (the accepted plan for phases 29-31), docs/PHASES.md and the phase entry in docs/phase-status.json before reviewing.

Keep the checkout read-only: do not edit files, commit, push, contact tenants, read credential files or tenant registrations, or make network requests. You may run read-only inspection commands.

Review attempt 1 of at most 5. Reviewed source fingerprint: 6dc6e14ede94cff85e7946afc99ae60555caf8d6f1ea063d97ce97832b528e83. Passing check report for this exact source: docs/checks/phase-30.json (inspect its commands and results).

Scope artifacts: ["tools/portkey_mcp_tags.py", "tools/generate_models.py", "tools/generate_validation.py", "tools/codex_review.py", "tools/test_codex_review.py", "tools/test_portkey_mcp_tags.py", "contracts/portkey-mcp-tags-overlay.json", "contracts/native-models.json", "contracts/native-semantics.json", "contracts/typescript-035-parity.json", "contracts/red-team-management-model-overlay.json", "contracts/response-defaults.json", "contracts/compatibility/g25/audit.json", "crates/scm-sdk/src/models/generated.rs", "crates/scm-sdk/src/models/gateway.rs", "crates/scm-sdk/src/models/mod.rs", "crates/scm-sdk/src/models/support.rs", "crates/scm-sdk/src/data.rs", "crates/scm-sdk/src/validation/schema_graph.json", "crates/scm-sdk/src/validation/semantics.json", "crates/scm-sdk/tests/portkey_mcp_tags_models.rs"].

Acceptance criteria: ["Typed tags on every imported write/read model with omission/null/value distinctions and pre-auth key/value validation", "List tags filters and list_with_options with exact wire tests; frozen methods byte-identical", "Cumulative offline gate on both compilers and all feature sets passes", "Bounded disposable live MCP tags workflow recorded with cleanup or explicit service limitation", "Chunk and complete-phase Codex reviews score at least 9/10 with no blocker/high finding"].

Assess the actual code, tests, provenance, security, functionality, ergonomics and maintainability against the plan. Do not reward claimed coverage that is not supported by implemented methods, tests or recorded evidence. Distinguish verified test evidence from unexecuted checks and service limitations. Later phases may remain unimplemented.

Return the structured result: an integer score from 1 to 10 (acceptance requires at least 9 and no blocker/high finding), a summary, concrete findings with repository-relative path, line (0 if not line-specific), severity (blocker|high|medium|low), description and required_fix, the evidence you actually inspected, and limitations.
