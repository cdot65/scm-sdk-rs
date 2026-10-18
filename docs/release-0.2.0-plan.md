# Deliver scm-sdk 0.2.0

The maintainer's October 18 instruction is to finish SDK delivery before starting
the separately supplied Portkey update. Publish the completed Red Team work as
**0.2.0**, owned by **cdot65**. Phases 9–12 stopped at repository delivery;
this follow-up supplies the registry release step.

## Fixed scope

- Accepted source: `167a359fb3d51aad37c4dff311060b069c620bdd`, delivered with
  metadata in `9ead6e9b1d65b7ce22125407ccfe5a45ff5c8cab`. Phase 12 passed 39
  checks and actual Claude review at 9/10. Forgejo run 19 passed all 47 jobs.
- Publish `scm-sdk` 0.2.0 with immutable annotated tag `v0.2.0`. Preserve earlier
  versions and tags. No SDK implementation, dependency, feature, compiler minimum,
  validator, fixture or example changes. Compare all runtime inputs byte-for-byte
  to the accepted source and fail on drift. MSRV remains Rust 1.89.
- Retain all 47 vendor operations, frozen 448 methods / 3,820 capabilities, G10
  behavior and 23 default bindings, twelve migration rejections and four scoped
  response-presence waivers.
- Eleven new Red Team calls remain live-unverified; selected-tenant reads returned
  403. No paid scans, EULA acceptance, verdict edits or regeneration for release.
- Gateway changes, including supplied Portkey commit
  `a27feb9efaac3653d2d94eca5d7d210944b6e160`, belong to the next task. Do not
  fetch or generate a Gateway baseline in these release phases.

## Sequential milestones

| Phase | Work | Exit evidence |
| --- | --- | --- |
| 13 — Release plan | Pin this scope, retain predecessor gates and establish scoped release checks. No SDK changes. | Actual Claude Code plan review >=9/10, no blocker/high findings. |
| 14 — Release and delivery | Release documentation, unchanged SDK proof, clean archives on both compilers, review, publication, registry/tag verification and remote delivery. | Actual Claude release review >=9/10 before publish; verified registry files and immutable source tag; terminal remote CI and retained receipts. |

Preserve failed checks and reviews. Source changes invalidate current review and
require fresh checks and actual Claude review. Historical command sets retain
their meaning. Phase 14 checks current contract/generation/security guards plus
the new clean archive proofs. Unchanged runtime retains Phase 12's source-bound
integration/compiler/feature evidence; no unexecuted matrix claims. Both archive
checks execute minimal/all-feature unit tests, examples, doctests and 3,820 probes.

## Execution and recovery

The packaged README is a permitted documentation change, so its bytes are expected
to differ from Phase 12. Both clean archive proofs must be regenerated after any
README edit and must include the exact reviewed README. Runtime equality alone
never qualifies a publishable archive. The final candidate guard checks actual
filesystem inventories, including untracked/ignored files and symlinks.

Phase 14 resolves the Phase 12 pagination migration guidance and GNU receipt
pointer advisories. The two low-priority generator advisories (rejecting future
unsupported query kinds at generation time and pinning additional adversarial
test names) remain explicit post-0.2.0 maintenance: the current pinned schema uses
only supported kinds and all existing adversarial tests passed. Their deferral
does not remove any current acceptance requirement. Phase 13's README scope,
untracked input detection, plan discoverability and per-phase plan pointers are
resolved in Phase 14.

1. After Phase 13 acceptance, prepare dated changelog and release guidance locally.
   Push public install guidance after registry verification. Publication receipts
   establish actual release status, separately from candidate preparation.
2. Add a fail-closed candidate guard for every source/example byte, legal notice,
   crate/workspace Cargo manifest and lockfile against the accepted source.
   Only documentation and governance/tooling may change. Keep the existing live
   receipt: unchanged runtime does not need new traffic. Recheck native ledgers.
3. Commit the candidate locally and require a completely clean tree. Execute
   `scripts/check-package.py --require-clean` on Rust 1.89.0 and 1.99.0 into
   ignored `.local/` paths, then retain complete proofs under `docs/checks/`.
   Require the same release commit and identical archive file bytes across
   compilers; archive wrapper metadata may differ.
4. Run the Phase 14 gate and actual Claude review >=9/10 with no blocker/high
   findings before publication. Bind accepted runtime, both clean proofs, both
   executed offline examples and consumer source. Publish only that payload.
5. Create a clean detached publication worktree at the qualified commit.
   Repackage with Rust 1.99.0 and require the complete archive SHA256 to match
   its clean proof before `cargo publish --locked --no-verify`. Verification is
   supplied by the executed clean archive checks. Cargo reads the owner's
   external mode-0600 credentials; never copy or print them. An existing version
   must be verified before another action; never overwrite or yank a release.
6. Create the annotated tag at the exact source commit. Verify registry version,
   cdot65 ownership, non-yanked state, downloaded checksum, clean embedded VCS
   commit and every archive file against both compiler proofs. Retain the actual
   registry receipt. Publication must not expand live or platform claims.
7. Push ordinary commits and the new tag to the authorized remote, preserving
   history. Check the peeled tag target and terminal CI for the exact pushed
   source. Pending is not success. Diagnose any failed job; review source fixes
   before push, without moving an already published tag.
8. Advance Phase 14 only after verified publication and remote delivery. Commit
   receipts and completion bookkeeping with unchanged reviewed source fingerprint.
   Report the published version, registry URL, source/tag, CI and live limitations.
   The supplied Gateway revision is the subsequent task, outside this artifact.

An ambiguous publish result requires reading the registry before retrying.
Registry/artifact mismatch stops further publication and requires investigation;
preserve the evidence. Do not claim completion until the downloaded artifact
has been verified. No local git action can roll back registry publication.
