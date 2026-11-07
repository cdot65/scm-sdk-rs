# SCM SDK review context

Follow `AGENTS.md`. Read `docs/PHASES.md` and the active phase scope in `docs/phase-status.json` before reviewing. Assess the actual code, tests, provenance, security, functionality, ergonomics, and maintainability against the accepted plan.

Apply the current maintainer rubric in docs/PHASES.md: five category scores and a total out of 50 for current chunks and phases. Historical reviews retain their original 1-to-10 rubric. Return concrete findings with paths and lines and required fixes. Acceptance requires the active phase criteria and independently verifiable evidence; later phases may remain unimplemented. Do not reward claimed coverage unsupported by implemented methods or tests.

During review, keep the checkout read-only and avoid credential files, tenant registrations, network mutations, commits, and pushes. Distinguish verified test evidence from unexecuted checks and service limitations.

## Issue tracking: BonBon

This project tracks work as Cards on its own BonBon Board, **SCM SDK** (`01a1b59d-587f-706b-b318-9c8e9fb450e3`), as described in `AGENTS.md` ("Issue tracking: BonBon") and `docs/agents/issue-tracker.md`. Claude Code is the only BonBon interface; other agents hand Card changes to a Claude Code session. Only the Owner validates.

## Agent skills

### Issue tracker

BonBon: Cards on the **SCM SDK** Board, through the `bonbon` MCP server and skill (Claude Code only); Forgejo Issues and GitHub Issues are not used. See `docs/agents/issue-tracker.md`.

### Triage labels

The five default triage roles map to identical BonBon Label strings (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`). See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `GLOSSARY.md` and `docs/adr/` at the repo root, created lazily by `/domain-modeling`. See `docs/agents/domain.md`.
