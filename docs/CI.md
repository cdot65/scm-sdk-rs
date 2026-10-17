# Continuous qualification

[.forgejo/workflows/ci.yml](../.forgejo/workflows/ci.yml) runs credential-free
verification for pushes and pull requests. It supports a manual opt-in for
registered additional native runners. No external mirror, registry publication,
release token, service registration, or live service workflow is configured.

## Required runner tools

The normal ubuntu-latest label must supply a Linux x86_64 runner with Git,
Python 3.11 or newer, rustup, a C/C++ compiler, CMake, and the Node runtime needed
to execute checkout/setup actions. Native HTTP/TLS/WebSocket dependencies include
aws-lc C build inputs. The workflow installs Rust 1.89.0 and 1.99.0 as needed,
with rustfmt and Clippy; it does not change the runner's default toolchain.

Development generation uses Node 22.23.2, matching .node-version.
tools/bootstrap_reference.py retrieves the exact public frozen TypeScript commit,
verifies it against the Git/source ledger, installs the owned npm lock with
lifecycle scripts disabled, and creates the development-only dependency symlink.
Packaged Rust consumers do not need Node or TypeScript.

Only two third-party actions are used, pinned to commits. Their tag-to-commit
bindings were rechecked against the official repository Git-reference API on
2026-10-16:

| Action | Reviewed tag | Pinned commit |
| --- | --- | --- |
| [actions/checkout](https://github.com/actions/checkout/tree/v4.3.1) | v4.3.1 | 34e114876b0b11c390a56381ad16ebd13914f8d5 |
| [actions/setup-node](https://github.com/actions/setup-node/tree/v4.4.0) | v4.4.0 | 49933ea5288caeca8642d1e84afbd3f7d6820020 |

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
[tools/test_ci.py](../tools/test_ci.py) guards MSRV, Node, feature selections,
action pinning, credential configuration, and additional target admission.

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
Local phase reports identify the executed compiler, native musl host, feature set,
source fingerprint, commands, and outcomes. Retained Forgejo run logs are
required before reporting this workflow as remotely qualified. The qualification
job logs its passing source fingerprint, command count, and report hash; complete
report files remain local to the job unless separately retained by the operator.

The additional runner labels are linux-arm64, macos-arm64, and windows-amd64;
registration and native execution remain pending until retained runner evidence
exists. Their native environments must provide Python, Git, rustup, CMake, and
the platform C/C++ build tools. Enabling the manual input does not create runners.
Publication and retained release-artifact upload require separate owner-controlled
setup; this workflow grants neither permission.
