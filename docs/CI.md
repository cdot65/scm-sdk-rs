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

Development scripts require Python 3.12 or newer for TOML loading and filtered tar
extraction. uv and Python are development tools, not Rust consumer dependencies.
See [Forgejo label configuration](https://forgejo.org/docs/latest/admin/actions/configuration/)
and [uv Python installation](https://docs.astral.sh/uv/guides/install-python/).

Development generation uses Node 22.23.2, matching .node-version.
tools/bootstrap_reference.py retrieves the exact public frozen TypeScript commit,
verifies it against the Git/source ledger, installs the owned npm lock with
lifecycle scripts disabled, and creates the development-only dependency symlink.
Packaged Rust consumers do not need Node or TypeScript.

Four distinct third-party actions are used across 11 references, all pinned to
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
composite action. No successful remote receipt for the corrected workflow is
retained. The Forgejo API returns 403 from this environment.
Local phase reports identify the executed compiler, native musl host, feature set,
source fingerprint, commands, and outcomes. The portable Python bootstrap has
separate local execution evidence with job-local paths and exact version checks;
other host platforms and runner PATH-file behavior still require remote evidence.
Retained Forgejo run logs are
required before reporting this workflow as remotely qualified. The qualification
job logs its passing source fingerprint, command count, and report hash; complete
report files remain local to the job unless separately retained by the operator.

The additional runner labels are linux-arm64, macos-arm64, and windows-amd64;
registration and native execution remain pending until retained runner evidence
exists. Their native environments must provide Git, Node 20, Bash, HTTPS/curl,
and platform C/C++ build tools; pinned actions install Rust/uv and the shared
bootstrap installs portable Python. Shell/pin files retain LF on Windows checkouts.
Enabling the manual input does not create runners.
Publication and retained release-artifact upload require separate owner-controlled
setup; this workflow grants neither permission.
