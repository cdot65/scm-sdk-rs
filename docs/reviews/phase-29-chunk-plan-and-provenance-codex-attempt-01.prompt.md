You are the independent reviewer for chunk `plan-and-provenance` of phase 29 (Portkey MCP tags delta plan and provenance) of the scm-sdk Rust SDK at the current working directory.

Follow AGENTS.md and CLAUDE.md review context. Read docs/portkey-mcp-tags-plan.md (the accepted plan for phases 29-31), docs/PHASES.md and the phase entry in docs/phase-status.json before reviewing.

Keep the checkout read-only: do not edit files, commit, push, contact tenants, read credential files or tenant registrations, or make network requests. You may run read-only inspection commands.

Review attempt 1 of at most 5. Reviewed source fingerprint: 0bb4b535fe5b12a94b12ed69545823232ce9ec2a146383865348fc6645bc16ab. Passing check report for this exact source: docs/checks/phase-29.json (inspect its commands and results).

Scope artifacts: ["docs/portkey-mcp-tags-plan.md", "docs/PHASES.md", "contracts/vendor/portkey-mcp-tags/8e5da0e", "tools/portkey-mcp-tags.mjs", "tools/test_portkey_mcp_tags.py", "tools/codex_review.py", "tools/test_codex_review.py", "tools/check_phase.py", "tools/test_typescript035_governance.py", "tools/test_management_phases.py"].

Acceptance criteria: ["Exact upstream bytes pinned and verified; complete structural delta classified fail-closed with tamper tests", "Plan settles Rust design, phases, chunks, checks and live/release boundaries", "No SDK runtime change", "All phase 29 checks pass for the reviewed source", "Chunk and complete-phase Codex reviews score at least 9/10 with no blocker/high finding"].

Assess the actual code, tests, provenance, security, functionality, ergonomics and maintainability against the plan. Do not reward claimed coverage that is not supported by implemented methods, tests or recorded evidence. Distinguish verified test evidence from unexecuted checks and service limitations. Later phases may remain unimplemented.

Return the structured result: an integer score from 1 to 10 (acceptance requires at least 9 and no blocker/high finding), a summary, concrete findings with repository-relative path, line (0 if not line-specific), severity (blocker|high|medium|low), description and required_fix, the evidence you actually inspected, and limitations.
