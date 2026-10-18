# Release and delivery procedure

The workspace version is **0.2.1**. [Phases15–17](portkey-access-policies-plan.md)
qualify the additive Portkey access-policy delta after completed 0.2.0 delivery.
The [registry receipt](checks/release-registry-0.2.1.json) and
[tag receipt](checks/release-tag-v0.2.1.json) establish actual publication and
remote source identity. A missing receipt means publication is not yet verified.

CI runs `--phase 17 --verification-only --contracts-only` with the unchanged
46-job Rust feature/compiler matrix. The complete local phase17 gate additionally
requires both clean archive proofs, the new offline example and external consumer,
all relevant compiler checks, current live-ledger freshness, dependency/privacy
checks and actual Claude review >=9/10 before publication. Historical gates retain
their accepted command sets. Publication uses the owner-controlled external
Cargo credential; CI receives no publishing credential. Repackage at the exact
qualified clean source, compare archive digest, publish, download and verify every
file, create/push the immutable source tag, then wait for terminal 47-job CI.

The prior [0.2.0 receipt](checks/release-registry-0.2.0.json) and
[release plan](release-0.2.0-plan.md) remain historical.

[The 0.1.1 publication receipt](checks/release-registry-0.1.1.json) records the
published G10 patch, with unchanged default behavior, MSRV and dependencies.
[Its maintenance plan](g10-0.1.1-plan.md) and phases 90–92 remain historical:
gate 91 qualified the published source; gate 92 guarded that immutable payload
before the vendor update. Source delivery and package checks do not establish
registry publication.

## Initial 0.1.0 publication

The initial **0.1.0** release was published on
[crates.io](https://crates.io/crates/scm-sdk/0.1.0) on 2026-10-17 at 01:14:40 UTC.
The owner is `cdot65`. The
[publication receipt](checks/release-registry-0.1.0.json) verifies the downloaded
archive SHA-256 `6d555d955c3734dd5ee78f7173569bdecd4426095b2ffedc4b77d8f0f2c2e8d9`
and embedded source commit `4f65dc48686a3b701ed0585812fcca46859f7e5a`.
All archive file bytes match the earlier qualified clean archive except
`.cargo_vcs_info.json`, which identifies that subsequent evidence-only commit.
This is an exact payload comparison, not a claim that the publication receipt
reran the archive tests. The matching tag is `v0.1.0`; its required target is the
published source commit. Remote tag confirmation is recorded after its push.

[Phase records](phase-status.json) bind accepted Claude Code reviews and passing
checks to each source snapshot. Clean release-commit package proof follows the
accepted source commit. Its retained receipt, rather than this procedure,
establishes completion and identifies the verified archive. The
[qualification summary](../contracts/qualification.json) consolidates the
source-bound status and limitations.

## Qualify the reviewed source

For the authorized G10 0.1.1 maintenance track, follow
[the patch plan](g10-0.1.1-plan.md): plan gate 90, then current source gate 91.
The Phase 8 procedure below describes the initial 0.1.0 delivery. Its historical
receipts do not qualify changed 0.1.1 runtime inputs. The patch qualified its
published source with gate 91. That track ran the post-publication gate using
`python3 tools/check_phase.py --phase 92 --verification-only`; clean-package,
registry and tag proofs remain at the paths in the plan.

The actual 0.1.1 remote CI observation is linked from Phase 91's `remote_ci` record
in [phase status](phase-status.json) and the
[retained observation](checks/release-ci-0.1.1-observation-01.json). Runs 16 and 17
were running and waiting at that observation; a terminal success was not recorded.
Phase 92 completed after its updated contracts job passed in
[run 18](checks/phase-92-remote-contracts-run-18.json). Its historical source scope
is separate from current vendor qualification.

1. Preserve the frozen TypeScript 0.34.0 baseline and provenance. Check all 448
   operation mappings and 3,820 public capability mappings, independent fixtures,
   deterministic generation and external public probes. Record Rust language
   adaptations and real per-method live status.
2. For 0.1.1, run the maintenance gate with `python3 tools/check_phase.py --phase 91`.
   The complete Phase 8 command set retains its initial-delivery meaning.
   Retain exact source fingerprints and commands, feature/compiler results,
   packaged consumers, dependency policy, platform scope and live receipts.
   Request a fresh actual Claude Code review, address findings and repeat until
   the score is at least 9. Follow [PHASES.md](PHASES.md).
3. Verify the latest controlled live receipt and its cleanup/recovery state using
   `python3 tools/live_evidence.py --check`. Keep unavailable prerequisites and
   unverified methods visible. See [LIVE_ACCEPTANCE.md](LIVE_ACCEPTANCE.md).
4. Review compatibility and migration notes, legal notices, version and package
   metadata. Applicable dependency findings block release unless an explicitly
   approved scoped exception is reflected in the enforced policy.

## Produce the clean commit proof

Commit only the reviewed source and evidence through the authorized repository
workflow. Keep the complete tree clean, including untracked files. Then run both
native archive checks with separate ignored output paths:

```sh
python3 scripts/check-package.py --toolchain 1.89.0 --require-clean --output .local/checks/release-package-1.89.0.json
python3 scripts/check-package.py --toolchain 1.99.0 --require-clean --output .local/checks/release-package-1.99.0.json
```

The checker binds `.cargo_vcs_info.json` to the current commit, requires effective
`dirty=false` before and after verification, and preserves the archive SHA-256,
file hashes, compiler/host and exact command results. Cargo may omit a false dirty
field; that defined omission is accepted and recorded separately. It runs locked
minimal/all-feature unit tests, compiles examples and no-run doctests, and runs the independent 3,820-probe
consumer against extracted archive contents. A prior `--allow-dirty` smoke check
does not satisfy this clean-commit requirement. See [PACKAGING.md](PACKAGING.md).

Writing a tracked or untracked report changes clean-tree state. Ignored `.local/`
outputs avoid that during proof. Retaining the proof in a later evidence-only
commit must preserve the recorded release source commit and artifact digest;
do not silently retarget an already verified archive. A source change requires
its own appropriate review and artifact verification.

Push ordinary commits to the authorized remote
`ssh://git@git-ssh.cdot.io/cdot/scm-sdk-rs.git` while preserving history. Record the
actual pushed commit and retained clean artifact. Source delivery does not imply
registry publication or a successful remote CI run.

## Publication authorization and provenance

Before a new publication, verify registry-name ownership,
maintainer credentials, notices and the exact reviewed release commit/artifact.
No registry token belongs in the repository, tenant files or knowledge workspace.
The accepted design assigns releases to `cdot65` with minimal-permission,
owner-controlled automation and supported trusted publishing after setup.
Until that configuration exists, the explicitly authorized maintainer releases use
Cargo's token provider with the owner's external mode-0600 credentials file.
CI remains credential-free and never performs registry publication.

Maintainer authorization covers 0.1.1 and the subsequent 0.2.0 delivery recorded
in the release plan. CI has no publishing job or registry credentials; Cargo reads the maintainer
credential from the external owner-controlled file. After an
actual publication, record the registry version/checksum, source commit and
corresponding archive proof. Do not advertise an unpublished dependency as
installable from crates.io.

The previous corrected workflow ran remotely at source `bde0f4e5`: all 46 Rust
matrix jobs succeeded, while `contracts-and-package` failed after setup.
Current remote qualification is recorded in [phase status](phase-status.json). Other target
builds remain compile-only where [PLATFORMS.md](PLATFORMS.md) says so; those facts
are release limitations, not platform runtime or live-provider acceptance.
