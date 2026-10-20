# Phase 24 delivery handoff — 2026-10-20

scm-sdk 0.2.3 is published and registry-verified. This is not phase completion.

- Published source: `14a67541f5a57d411c4c4a64c91ced5ba170dafd`.
- Archive SHA256: `80a58b7b287d24261c869cf0fda7d77b177f2442db3a173634c527d6fa73e809`.
- Registry evidence: `docs/checks/release-registry-0.2.3.json`.
- Current delivery commit: `c7999e979f11eb5007dc227583252e78b4c3630b`.
- PR: https://git.cdot.io/cdot/scm-sdk-rs/pulls/1.
- Run: https://git.cdot.io/cdot/scm-sdk-rs/actions/runs/26.
- Actual readiness chunk attempt 3 and full readiness attempt 1: both 46/50.
- Post-publication documentation guard: 37 tests passed; receipt in
  `docs/checks/phase-24-published-document-guard.json`.

## Server restriction

Forgejo rejected an ordinary atomic main/tag push because airs-bot cannot push
main, and rejected a separate ordinary tag push because airs-bot cannot push the
repository. The supported AGit PR push succeeded. No force push or alternative
credential was attempted. Run 26 has 49 pending jobs and requires maintainer
approval. Earlier runs are superseded and cannot qualify this source.

A maintainer must either give airs-bot the required repository write access and
approve the workflow, or deliver the history-preserving merge and annotated tag.
The tag must target the published source above, not the later evidence commit.
The existing local annotated tag is v0.2.3; its source is independently checked by
the registry receipt. Do not recreate it at another source or rewrite history.

## Remaining acceptance

Observe the actual remote annotated tag and main ancestry, retain a terminal
successful configured 49-job workflow on matching source, obtain actual delivery
chunk review and final full-phase review, then advance phase 24. Phase 25 remains
planning-only until those obligations are satisfied.

## Remaining review limitations

The full readiness review retains one medium documentation limitation and four
low follow-ups. The live probes used /ai-red-teaming/mgmt, outside the declared
endpoint profiles. Their HTTP 403 results do not establish permissions as the
sole blocker; all four target reads remain live-unqualified. The other findings
concern README receipt discoverability, per-method references to newer management
tests, historical helper naming, and generic future receipt-policy strictness.
The independent management fixture ledger and complete gate bind the executed
tests; the current release-specific guard requires endpoint disclosure. Preserve
the immutable published source and original review records when following up.
