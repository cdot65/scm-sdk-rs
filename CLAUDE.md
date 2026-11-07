# SCM SDK review context

Follow `AGENTS.md`. Read `docs/PHASES.md` and the active phase scope in `docs/phase-status.json` before reviewing. Assess the actual code, tests, provenance, security, functionality, ergonomics, and maintainability against the accepted plan.

Apply the current maintainer rubric in docs/PHASES.md: five category scores and a total out of 50 for current chunks and phases. Historical reviews retain their original 1-to-10 rubric. Return concrete findings with paths and lines and required fixes. Acceptance requires the active phase criteria and independently verifiable evidence; later phases may remain unimplemented. Do not reward claimed coverage unsupported by implemented methods or tests.

During review, keep the checkout read-only and avoid credential files, tenant registrations, network mutations, commits, and pushes. Distinguish verified test evidence from unexecuted checks and service limitations.

## Agent skills

### Issue tracker

Issues live in Forgejo at `git.cdot.io/cdot/scm-sdk-rs`, driven through the `tea` CLI (login `cdot`); GitHub is a push mirror only. See `docs/agents/issue-tracker.md`.

### Triage labels

The five default triage labels (`needs-triage`, `needs-info`, `ready-for-agent`, `ready-for-human`, `wontfix`) exist in the Forgejo repo. See `docs/agents/triage-labels.md`.

### Domain docs

Single-context: `GLOSSARY.md` and `docs/adr/` at the repo root, created lazily by `/domain-modeling`. See `docs/agents/domain.md`.
