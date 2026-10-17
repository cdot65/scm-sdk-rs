# Release and delivery procedure

The workspace version is **0.1.0**. Registry publication has not occurred.
[Phase records](phase-status.json) bind accepted Claude Code reviews and passing
checks to each source snapshot. Clean release-commit package proof follows the
accepted source commit. Its retained receipt, rather than this procedure,
establishes completion and identifies the verified archive. The
[qualification summary](../contracts/qualification.json) consolidates the
source-bound status and limitations.

## Qualify the reviewed source

1. Preserve the frozen TypeScript 0.34.0 baseline and provenance. Check all 448
   operation mappings and 3,820 public capability mappings, independent fixtures,
   deterministic generation and external public probes. Record Rust language
   adaptations and real per-method live status.
2. Run the complete Phase 8 gate with `python3 tools/check_phase.py --phase 8`.
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
minimal/all-feature unit tests, examples, doctests and the independent 3,820-probe
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

## Publication remains a separate action

Before first publication, verify registry-name availability and ownership,
maintainer credentials, notices and the exact reviewed release commit/artifact.
No registry token belongs in the repository, tenant files or knowledge workspace.
The accepted design assigns releases to `cdot65` with minimal-permission,
owner-controlled automation and supported trusted publishing after setup.

Publication requires explicit maintainer authorization. No publishing job or
registry credentials are currently installed by this delivery workflow. After an
actual publication, record the registry version/checksum, source commit and
corresponding archive proof. Do not advertise an unpublished dependency as
installable from crates.io.

Remote CI is configured but has no retained successful execution. Other target
builds remain compile-only where [PLATFORMS.md](PLATFORMS.md) says so; those facts
are release limitations, not platform runtime or live-provider acceptance.
