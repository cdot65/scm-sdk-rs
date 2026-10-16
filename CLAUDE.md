# SCM SDK review context

Follow `AGENTS.md`. Read `docs/PHASES.md` and the active phase scope in `docs/phase-status.json` before reviewing. Assess the actual code, tests, provenance, security, functionality, ergonomics, and maintainability against the accepted plan.

Return a candid numeric score from 1 to 10, concrete findings with paths and lines, and required fixes. A score below 9 blocks phase advancement. A score of 9 or 10 requires all active phase acceptance criteria to be met with independently verifiable evidence; later phases may remain unimplemented. Do not reward claimed coverage unsupported by implemented methods or tests.

During review, keep the checkout read-only and avoid credential files, tenant registrations, network mutations, commits, and pushes. Distinguish verified test evidence from unexecuted checks and service limitations.
