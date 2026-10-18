# Release and delivery procedure

The workspace version is **0.1.0**, published on
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
receipts do not qualify changed 0.1.1 runtime inputs. The patch updates CI to gate
91 and retains new clean-package, registry and tag proofs at the paths in its plan.

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
Until that configuration exists, the explicitly authorized 0.1.1 release uses
Cargo's token provider with the owner's external mode-0600 credentials file.
CI remains credential-free and never performs registry publication.

New registry publication requires explicit maintainer authorization. No publishing job or
registry credentials are currently installed by this delivery workflow. After an
actual publication, record the registry version/checksum, source commit and
corresponding archive proof. Do not advertise an unpublished dependency as
installable from crates.io.

The previous corrected workflow ran remotely at source `bde0f4e5`: all 46 Rust
matrix jobs succeeded, while `contracts-and-package` failed after setup.
Current remote qualification is recorded in [phase status](phase-status.json). Other target
builds remain compile-only where [PLATFORMS.md](PLATFORMS.md) says so; those facts
are release limitations, not platform runtime or live-provider acceptance.
