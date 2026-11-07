# SCM SDK implementation contract

Read `docs/PHASES.md` before implementation or review. Read `docs/implementation-plan.md` when changing public contracts, authentication, transport, generation, or acceptance criteria.

Complete phases in order. Parallel work may cover independent milestones within the active phase. Run the phase's specified checks, request a fresh Claude Code review with a score from 1 to 10, address every blocking finding, and apply the current maintainer rubric in docs/PHASES.md (historical phases retain their original threshold). Record the review artifact, exact reviewed source state, checks, limitations, and phase outcome in `docs/reviews/` and `docs/phase-status.json` before advancing.

Use the committed TypeScript baseline recorded in `contracts/manifest.json`; preserve export parity and document deliberate Rust differences. Report implementation, contract coverage, and live acceptance separately. Experimental or unavailable service workflows retain explicit status.

Keep credentials and tenant registrations outside git and published packages. Load them only from explicitly selected external paths; print sanitized metadata and outcomes. Live mutations use uniquely owned disposable fixtures, bounded cost, and recorded cleanup. Respect tenant scope and caller destination policy across token exchange and all request transports.

Public Rust is async, uses application-owned runtimes, and has no desktop-specific types. Preserve unknown response values, distinguish patch omission/null/value, and report uncertain remote outcomes. Public formatting and tracing omit sensitive bodies. Project-owned Rust forbids unsafe code.

The authorized push destination is `ssh://git@git-ssh.cdot.io/cdot/scm-sdk-rs.git`. Preserve remote history and push ordinary commits after the corresponding phase gate passes.

## Issue tracking: BonBon

This project tracks work as Cards on its own BonBon Board, **SCM SDK** (`01a1b59d-587f-706b-b318-9c8e9fb450e3`).
Every Card carries the Label `scm-sdk`, one triage Label and at least one kind Label
(see `docs/agents/triage-labels.md`), and its title starts with `[SCM SDK] `. Never put
this project's work on any other Board.

- **Claude Code is the only BonBon interface** (the `bonbon` skill and MCP server; the
  `bonbon-owner-api` skill as a fallback). Other agents never connect to BonBon. They write
  the Card changes they need into their handoff, and a Claude Code session applies them.
- In Claude Code: `whoami` first, then search this Board before creating anything. Read a
  Card fully before working it. Claim before executing, heartbeat at least every 20 minutes,
  and finish with `complete_card` and a Result.
- Only the Owner validates: never mark work DONE or VALIDATED, and preserve other agents'
  Claims.
- Keep secrets, device identifiers and personal or household data out of Cards.
- Board activity alone authorizes no implementation, deploy, commit, push or message.

Details: `docs/agents/issue-tracker.md`.
