# Phase 28 historical delivery — 2026-11-06

scm-sdk 0.2.4 is on crates.io, and phase 28 never recorded its delivery. This branch
(`delivery/v0.2.4`) finishes that work without changing the published source. It is
the release commit `6f18529` plus evidence-only commits: `docs/checks`, `docs/reviews`
and `docs/phase-status.json` are excluded from the source fingerprint. Every commit on
the branch therefore has exactly the published fingerprint, and tools in this tree are
the 0.2.4 release's own tools.

## What was wrong and is disclosed

- **Publication bypassed the readiness gate.**
  - 0.2.4 was published on 2026-10-22 at 17:49Z by `cdot65`, from `6f18529`.
  - At that time all three phase-level readiness reviews had been rejected:
    - `docs/reviews/phase-28-attempt-01.json` (43/50)
    - `docs/reviews/phase-28-attempt-02.json` (44/50)
    - `docs/reviews/phase-28-attempt-03.json` (44/50)
  - `docs/RELEASE.md` requires an accepted phase-level readiness review before
    `cargo publish`. That rule was not followed.
- **The retained clean package proofs qualify a different commit.**
  - `docs/checks/phase-28-release-package-{1.99.0,1.89.0}.json`, as originally
    committed, qualify `69eb33e`.
  - The published archive's `.cargo_vcs_info.json` names `6f18529`.
- **No terminal CI run was recorded.** Run 32 on `6f18529` failed.
- **Readiness chunk binding.** Release-readiness chunk attempts 1–3 (accepted at 44, 45 and 45)
  were bound to earlier candidate sources: `5156a408…`, `63009ca5…` and `a61ee873…`.
  Attempt 4 (accepted 44/50, `docs/reviews/phase-28-chunk-release-readiness-attempt-04.json`)
  is the one bound to the published fingerprint `f9cebee5…`. It was recorded on `main`
  after `6f18529`, and this branch retains a byte-identical copy so the chunk's latest
  review is on the published source. Its status in `docs/phase-status.json` stays `pending`
  until phase completion records the accepted chunks.
- **Regenerated proofs.** The 105-command gate run on this branch regenerated
  `docs/checks/phase-28-release-package-*.json` and `phase-28-release-readiness.json` for the
  branch source. Those are post-publication gate outputs, not pre-publication readiness. The
  pre-publication versions remain in git history at `6f18529`.
- **The published-source CI cannot pass as committed.** At `6f18529` the committed
  `phase-28-private-material.json` receipt is stale for that source (`source_changed`).

## What this branch establishes

1. **Post-publication qualification of the exact published bytes:**
   - `docs/checks/phase-28-published-package-{1.99.0,1.89.0}.json`: clean extracted-package
     proofs (`--require-directional`) on this fingerprint-identical source.
   - `docs/checks/phase-28-published-archive-comparison.json`: all 142 files of the
     crates.io archive (sha256 `533294fe…`) are byte-identical to both proofs, except
     `.cargo_vcs_info.json`, which records the commit identity.
2. **A refreshed private-material receipt** (`424407a`), so the CI contracts gate can
   pass on the published source.
3. **The full phase 28 gate:** 105/105 commands, recorded in `docs/checks/phase-28.json`
   on this branch.
4. **Remote CI on this branch:** Forgejo run 44 on `7dd0fff`. Run 39 (`424407a`) passed
   48/49; its one failure was a 41-second Rust job, and a local rerun of the same feature
   set passed. Run 41 (`b40409e`) was cancelled by Forgejo when a newer commit was pushed
   to the branch. Run 44's terminal receipt is under `docs/checks/historical-delivery/0.2.4/`.
5. **Registry and tag receipts:** under `docs/checks/historical-delivery/0.2.4/` and
   `docs/checks/release-registry-0.2.4.json`, copied byte-for-byte from `main`, where
   `tools/historical_delivery.py` created them.

## Why delivery is verified on `main`, not by this tree's `tools/release_delivery.py`

This tree's `release_delivery.verify` requires two things:

- the current fingerprint must equal the published source, which holds here;
- the observed remote `refs/heads/main` must equal the CI commit, which can never hold
  again. Remote `main` has advanced past 0.2.4, and history is not rewritten to force it.

`main` therefore carries `tools/historical_delivery.py`, an evidence-only copy of which
is in `docs/reviews/historical-delivery/`. Its `verify --version 0.2.4` recomputes the
following offline:

- the published archive's identity, checksum, retained bytes, VCS commit and inventory;
- both clean proofs against the release fingerprint, and their byte equality to the
  published files except VCS identity;
- the canonical and GitHub `v0.2.4` annotated tags peeling to `6f18529`;
- the CI run's newest status for every configured context, on a branch commit whose
  fingerprint equals the release and which the observed remote delivery branch contains.

Its execution is recorded by the tool itself: `historical_delivery.py execute` runs `verify`
and the tool's own regression suite, then writes a receipt. The receipt records the HEAD
commit, a clean-tree flag, the bound `release_delivery.py` rules (path and sha256), the tool
and test sha256, the test summary and the verify result. It writes only under the evidence
directory, in create-only mode.

- `docs/checks/historical-delivery/0.2.4/execution-main.json`: run on `main`.
- `docs/checks/historical-delivery/0.2.4/execution-release-tree.json`: the evidence copy in
  `docs/reviews/historical-delivery/`, run *inside this tree* with `--root .`. Its `rules`
  entry is this release's own `tools/release_delivery.py`.

The tool sha256 in both receipts equals the in-tree copy. Earlier hand-assembled receipts and
the plain `verify.json` transcription were removed; they remain in git history.

These checks cannot be part of this tree's recorded 105-command gate.
`tools/check_phase.py` is fingerprinted release source and cannot change. The receipts above
are the execution evidence for them.

The retained registry archive (`.local/artifacts`) is gitignored. Archive equality can still
be checked in-tree: the published inventory in `docs/checks/release-registry-0.2.4.json`
(`files`) and both proofs' `files` are committed receipts.

Phase 28 completion is recorded on `main` after the remote-delivery chunk review and
the final phase review are accepted. The `--advance` path in this tree cannot be used:
it calls the original verifier.

## Repository-level note

The top-level `active_followup_plan` in `docs/phase-status.json` names the phase 21–25 plan
(`docs/red-team-management-0.86-plan.md`). It is the repository-wide legacy follow-up pointer,
not phase 28's plan. Phase 28's own `plan` field is `docs/typescript-035-parity-plan.md`.

## Related decision

The maintainer closed phase 24 (0.2.3) as **superseded by 0.2.4**. 0.2.3 exact-source
CI cannot pass, because RUSTSEC-2026-0382/0383 were published after release and affect
its locked `webpki-root-certs 1.0.9`. 0.2.4 uses `webpki-root-certs 1.0.10`.
