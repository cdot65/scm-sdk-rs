# Continuous qualification

[.forgejo/workflows/ci.yml](../.forgejo/workflows/ci.yml) runs credential-free
verification for pushes and pull requests. It supports a manual opt-in for
registered additional native runners. No external mirror, registry publication,
release token, service registration, or live service workflow is configured.

## Required runner tools

The normal ubuntu-latest label must supply a Linux x86_64 runner with Git,
Node 20 for JavaScript actions, Bash, HTTPS/curl support, and a C/C++ toolchain.
Native HTTP/TLS/WebSocket dependencies include AWS-LC C build inputs; the selected
configuration does not require CMake, Go or bindgen. Windows native jobs need MSVC.
Rust and Python need not be preinstalled. The pinned Rust action bootstraps rustup
when absent, exports Cargo on subsequent steps' PATH, and installs Rust 1.89.0
and/or 1.99.0 with rustfmt and Clippy. CI actions set the runner's default Rust toolchain
to the most recently selected version. Project check scripts use explicit
`+version` selection and do not alter the developer machine's default toolchain.

Forgejo labels do not establish the container's distribution: an `ubuntu-latest`
label can select a Debian Node image. The pinned uv setup action installs
**uv 0.13.1**. [The owned bootstrap](../scripts/bootstrap-python.sh) then verifies
that uv version, installs **Python 3.14.7** from `.python-version` into job-local
directories, and exports `python`/`python3` through the runner PATH file. It
disables local uv configuration/cache discovery and Windows registry writes and
does not edit shell profiles. This supports portable Python distributions without
requiring Ubuntu-specific interpreter builds or an existing Python installation.
The action receives an explicitly empty GitHub token and has cache upload disabled.

[Reviewed SHA-256 pins](../scripts/uv-checksums.json) cover both GNU and musl
Linux x86_64/ARM64, macOS ARM64, and Windows x86_64. Before setup-uv runs,
[the integrity helper](../scripts/uv-integrity.cjs) selects the exact host/libc
archive digest and creates a fresh owned tool cache. The workflow passes that
digest to the pinned action's `checksum` input, which checks the archive before
extraction. A fresh cache prevents reuse of an unchecked preexisting executable.
The Python bootstrap independently hashes the exact `uv-path` output before
executing it, then verifies the version and installs Python. Unsupported hosts,
stale pins and mismatched bytes fail closed. Updating uv requires new reviewed
archive and executable digests for every configured host. The pins were obtained
from the official release asset API and independently checked against downloaded
archives; [the execution receipt](checks/phase-08-uv-integrity.json) records both
successful verification and an actual rejected wrong-checksum download.

Development scripts require Python 3.12 or newer for TOML loading and filtered tar
extraction. uv and Python are development tools, not Rust consumer dependencies.
See [Forgejo label configuration](https://forgejo.org/docs/latest/admin/actions/configuration/)
and [uv Python installation](https://docs.astral.sh/uv/guides/install-python/).

Development generation uses Node 22.23.2, matching .node-version.
Every job installs that exact version before running the uv integrity helper.
tools/bootstrap_reference.py retrieves the exact public frozen TypeScript commit,
verifies it against the Git/source ledger, installs the owned npm lock with
lifecycle scripts disabled, and creates the development-only dependency symlink.
Packaged Rust consumers do not need Node or TypeScript.

Four distinct third-party actions are used across 13 references, all pinned to
commits. The following upstream Git references were verified with `git ls-remote`
on 2026-10-17. [The recorded checks](checks/phase-08-ci-actions.json) retain the
commands, exact references and results; the two newly introduced action manifests
also have recorded hashes and nonsecret local review copies.

| Action | Reviewed revision | Pinned commit |
| --- | --- | --- |
| [actions/checkout](https://github.com/actions/checkout/tree/v4.3.1) | v4.3.1 | 34e114876b0b11c390a56381ad16ebd13914f8d5 |
| [actions/setup-node](https://github.com/actions/setup-node/tree/v4.4.0) | v4.4.0 | 49933ea5288caeca8642d1e84afbd3f7d6820020 |
| [astral-sh/setup-uv](https://github.com/astral-sh/setup-uv/tree/v6.6.0) | v6.6.0 | 4959332f0f014c5280e7eac8b70c90cb574c9f9b |
| [dtolnay/rust-toolchain](https://github.com/dtolnay/rust-toolchain/tree/683b588c487a69d01f6c6ddf40f036ff682cc2c8) | untagged master commit observed 2026-10-17 | 683b588c487a69d01f6c6ddf40f036ff682cc2c8 |

The Rust action intentionally uses an untagged master commit with explicit
`toolchain` inputs. Its [pinning guidance](https://github.com/dtolnay/rust-toolchain#choice-of-full-length-commit-sha)
requires full commit pins to belong to master history. The reference above was
master's observed head; the workflow always uses that immutable commit, not a
floating branch. On Unix it downloads and executes the official rustup installer
over HTTPS; on Windows it downloads rustup-init.exe. uv downloads Python builds
from Astral's python-build-standalone distributions. Those distribution services
remain trusted download sources; pinning an action does not make them offline
or independently checksum-pin rustup bootstrap.

Checkout disables credential persistence. The permissions block declares content
read only; no secret expressions, write permissions, privileged PR trigger, or
OIDC enablement appears in the workflow. Forgejo ignores some GitHub permission
keys, so this declaration alone does not prove token restrictions; the instance
and repository controls remain authoritative. See the official
[compatibility differences](https://forgejo.org/docs/latest/user/actions/github-actions/)
and [token/runner security](https://forgejo.org/docs/latest/user/actions/security/).
Maintainers should review untrusted contributions before manually starting jobs
on persistent native hosts.

## Jobs

| Job | Qualification |
| --- | --- |
| contracts-and-package | Installs pinned development tools, restores verified frozen inputs, and runs tools/check_phase.py --phase 8 --verification-only. This verifies completed-phase contracts without advancing phase state or invoking live services. It includes generation/oracles, native exports, dependency policy, packaged consumers, cross compilation, and regression guards. |
| rust | Rust 1.89.0 and 1.99.0 each run the complete feature-selection matrix through scripts/check-rust.py: formatting, all-target checks, warning-denied Clippy, tests, and warning-denied documentation. |
| additional-native-platforms | Manual-only native execution on explicitly registered Linux ARM64, Apple Silicon, and Windows x86_64 runners. The actual rustc host is asserted before checks. |

The matrix includes default, minimal, all, every isolated Cargo feature, both JSON
arbitrary-precision combinations, tracing combinations, and the exact native
export probe feature set plus its historical gate regression.
[tools/test_ci.py](../tools/test_ci.py) guards MSRV, Node/Python, feature selections,
action pinning, setup ordering, this document's action inventory and setup effects,
credential configuration, and additional target admission.

Dependency tools are installed with their own lockfiles at exact versions:
cargo-audit 0.22.2 and cargo-deny 0.20.2. The phase gate invokes
scripts/check-dependencies.py; it refreshes and records official RustSec
provenance, audits the complete Cargo lock with warnings denied, and checks
all-feature license/source/ban policy. Public registry, source, npm, and advisory
downloads are allowed; these checks need no service credentials.

The qualification job installs standard libraries and fetches locked inputs for
Linux GNU x86_64/ARM64, Apple Silicon, and Windows MSVC x86_64. The cross checks
compile the transport-independent feature set. They do not link or execute target
binaries or qualify native HTTP/TLS/WebSocket dependencies.

## Evidence status

Committed workflow configuration and local workflow guards are evidence of the
configured checks. They do not establish successful remote execution.
The original remote run reported `rustup: command not found` and exited 127.
The correction reproduces that missing-tool failure and records a successful
isolated local bootstrap plus a workflow guard that rejects the original source.
That local exercise simulates PATH-file application and uses `--no-modify-path`
to preserve shell profiles; it does not execute Forgejo/act or the complete
composite action. The corrected source `bde0f4e5` ran as
[run 9](https://git.cdot.io/cdot/scm-sdk-rs/actions/runs/9): all 46 Rust matrix jobs
succeeded, while `contracts-and-package` failed after 38m35s. A separately
supplied Forgejo token now permits reading run and commit-status metadata.
Forgejo 15.0.6 exposes no job-log API here, and web logs require a signed-in
session. The owner-supplied log identifies
`scripts/check-private-material.py --check` as the failed command after every
preceding check passed. That older runner did not expose its captured stderr. The
[server-status observation](checks/phase-08-ci-run-09.json) preserves these
outcomes; it does not establish a successful complete workflow or native host
qualification without the corresponding reports.
The committed receipt passes in both a clean local clone and the restored
development environment. A focused regression separately reproduces a validator
defect: excluded generated reports can exhaust the read budget even with a
matching source fingerprint. Public freshness checks now read only the source
inputs that their fingerprint covers; full private scans still inspect all
Git-visible files. The same source snapshot rule applies to the tracing stress
runner. Public receipt failures emit closed diagnostic categories without
reflecting paths, values or arbitrary exception text. This fixes the reproduced
defect; the exact cause of run 9 needs source-bound remote execution evidence.
The latest retained outcome is recorded in [phase status](phase-status.json).
Local phase reports identify the executed compiler, native musl host, feature set,
source fingerprint, commands, and outcomes. The portable Python bootstrap has
separate local execution evidence with job-local paths and exact version checks;
other host platforms and runner PATH-file behavior still require remote evidence.
Retain server-reported run and per-job statuses bound to the reviewed commit
before reporting the workflow as successful. Detailed compiler, native host and
feature qualification claims require the corresponding reports or logs. The qualification
job logs its passing source fingerprint, command count, and report hash; complete
report files remain local to the job unless separately retained by the operator.
For a failed credential-free command, the phase runner now prints the final
16,384 characters of each output stream to the job log while preserving the full
streams in its report. A behavioral regression checks the real failed-command
path, its output bounds, and the complete retained report.
The phase runner also records timeouts with a null exit code, retains partial
output, and kills the checker's POSIX process group. Its 5,460-second checker
budget accommodates the Rust checker's six 900-second command bounds plus
cleanup overhead. Real subprocess tests cover timeout reports and child cleanup.

The additional runner labels are linux-arm64, macos-arm64, and windows-amd64;
registration and native execution remain pending until retained runner evidence
exists. Their native environments must provide Git, Node 20, Bash, HTTPS/curl,
and platform C/C++ build tools; pinned actions install Rust/uv and the shared
bootstrap installs portable Python. Shell/pin files retain LF on Windows checkouts.
Enabling the manual input does not create runners.
Publication and retained release-artifact upload require separate owner-controlled
setup; this workflow grants neither permission.
