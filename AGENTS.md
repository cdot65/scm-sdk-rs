# SCM SDK implementation contract

Read `docs/PHASES.md` before implementation or review. Read `docs/implementation-plan.md` when changing public contracts, authentication, transport, generation, or acceptance criteria.

Complete phases in order. Parallel work may cover independent milestones within the active phase. Run the phase's specified checks, request a fresh Claude Code review with a score from 1 to 10, address every blocking finding, and repeat until the score is at least 9. Record the review artifact, exact reviewed source state, checks, limitations, and phase outcome in `docs/reviews/` and `docs/phase-status.json` before advancing.

Use the committed TypeScript baseline recorded in `contracts/manifest.json`; preserve export parity and document deliberate Rust differences. Report implementation, contract coverage, and live acceptance separately. Experimental or unavailable service workflows retain explicit status.

Keep credentials and tenant registrations outside git and published packages. Load them only from explicitly selected external paths; print sanitized metadata and outcomes. Live mutations use uniquely owned disposable fixtures, bounded cost, and recorded cleanup. Respect tenant scope and caller destination policy across token exchange and all request transports.

Public Rust is async, uses application-owned runtimes, and has no desktop-specific types. Preserve unknown response values, distinguish patch omission/null/value, and report uncertain remote outcomes. Public formatting and tracing omit sensitive bodies. Project-owned Rust forbids unsafe code.

The authorized push destination is `ssh://git@git-ssh.cdot.io/cdot/scm-sdk-rs.git`. Preserve remote history and push ordinary commits after the corresponding phase gate passes.
