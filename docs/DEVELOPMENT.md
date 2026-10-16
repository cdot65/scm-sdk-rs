# Development and verification

The SDK is developed in this repository. Consumers build the packaged Rust crate; Node and the TypeScript checkout are development inputs for contract extraction only. Follow `AGENTS.md`, the active phase in `docs/PHASES.md`, and the accepted design in `docs/implementation-plan.md`.

## Prerequisites

Use Python 3.11 or newer, Git, rustup, and a native C/C++ toolchain. The default reqwest/rustls transport builds AWS-LC native code. The pinned non-FIPS configuration needs a C/C++ compiler; it does not require CMake, Go, or bindgen. Windows runners need the MSVC build tools. See [DEPENDENCIES.md](DEPENDENCIES.md) for exact versions, sources, and the transitive native-code boundary.

Install exact toolchains without changing the machine's default:

```sh
rustup toolchain install 1.88.0 --profile minimal --component rustfmt,clippy --no-self-update
rustup toolchain install 1.99.0 --profile minimal --component rustfmt,clippy --no-self-update
```

The workspace declares Rust 1.88 as its minimum supported Rust version. Current locked ICU dependencies also declare 1.88. The latest pinned verification toolchain is 1.99.0. Both passed the foundation all-feature build, Clippy, tests, and documentation checks on the local musl host on 2026-10-16. Rust 1.95.0 remains the local machine's installed default; verification commands select their version explicitly. The helper finds Cargo through `PATH` or the rustup proxy in the user's `.cargo/bin` directory; `--cargo PATH` overrides discovery.

## Native Rust checks

Run each supported feature configuration on each toolchain:

```sh
python3 scripts/check-rust.py --toolchain 1.88.0 --feature-set minimal
python3 scripts/check-rust.py --toolchain 1.88.0 --feature-set default
python3 scripts/check-rust.py --toolchain 1.88.0 --feature-set all
python3 scripts/check-rust.py --toolchain 1.99.0 --feature-set minimal
python3 scripts/check-rust.py --toolchain 1.99.0 --feature-set default
python3 scripts/check-rust.py --toolchain 1.99.0 --feature-set all
```

The helper checks formatting without modifying files, builds all targets, runs Clippy with warnings denied, runs tests including any crate doctests, and builds documentation with warnings denied. Cargo uses the committed lockfile. `minimal` means `--no-default-features`; `all` means `--all-features`.

Product features are `scan`, `management`, `dlp`, `iam`, `model-security`,
`red-team`, and `gateway`; `models` selects the shared native contracts and
`reqwest-transport` selects the default HTTP adapter. Defaults enable the adapter
and scanning. Management includes canonical DLP; Gateway includes canonical IAM
for its scope accessor. IAM is also independently selectable. Phases 3–5 implement
scanning, management/DLP/IAM, Model Security and Red Team. Phase 6 implements
Gateway control, telemetry and HTTP runtime contracts. Minimal builds
require caller-supplied transport and retain validation/data primitives. Realtime
and tracing features arrive with their implementations. Keep the default,
minimal and all-feature matrices as new features enter scope.

Each invocation writes `.local/checks/rust-TOOLCHAIN-FEATURE.json`, with exact commands, outputs, exit codes, durations, compiler/host information, lockfile hash, and a fingerprint of Rust build inputs. Source changes during a run fail the evidence check. The helper targets the compiler's native host explicitly and verifies the packaged license/notice copies. It removes `PANW_`, `AIRS_`, and `SCM_` environment variables from test subprocesses. Tests use synthetic credentials and local fixtures. Live acceptance follows [LIVE-ACCEPTANCE.md](LIVE-ACCEPTANCE.md).

To isolate build caches without altering global Cargo configuration:

```sh
CARGO_TARGET_DIR=.local/target-msrv python3 scripts/check-rust.py --toolchain 1.88.0 --feature-set all
```

## Frozen contract checks

The TypeScript baseline is commit `4a80dbdb3dc14365e1c25d838ac0bd139485d48c`, version 0.34.0. Do not use uncommitted changes in another checkout as generator input. Bootstrap and development package versions are pinned in `tools/package-lock.json` and `.node-version`:

```sh
python3 tools/bootstrap_reference.py
python3 tools/verify-contracts.py --baseline .local/typescript --self-test
.local/typescript/node_modules/.bin/tsx tools/extract-contracts.mts --check
.local/typescript/node_modules/.bin/tsx tools/extract-operations.mts --check
```

Both extractors verify source/compiler inputs against the Git provenance ledger before evaluation. Ordinary regeneration requires deliberate classification decisions; it cannot invent missing entries or overwrite existing classifications. The official Gateway specification remains excluded from redistribution and generation while its license status is unresolved.

Native generation and independent schema/model checks are explicit:

```sh
python3 tools/generate_validation.py --check
python3 tools/generate_models.py --check
python3 tools/native_operations.py --check
python3 tools/verify_native_fixtures.py
.local/typescript/node_modules/.bin/tsx tools/schema-oracle.mts --check
python3 tools/generate_models.py --verify-fixtures
```

The oracle executes frozen Zod schemas with network and credential access removed.
Rust integration tests compare both request normalization and response preservation.
The separate native consumer round-trips every successful public JSON schema
specimen; Blob-only roots have typed binary tests. Node is absent from the crate.
`contracts/native-operations.json` records implemented native methods, explicit
pending methods, header contracts, source bindings, and separate live status.
Its mapping generator checks declared methods and reproducibility; independent
integration tests establish the behavior attributed to those mappings. Packaged
fixture subsets are compared directly with manifest-pinned independent source
cases and JSON pointers; tamper tests prevent replacement, type substitution,
unreviewed sources, duplicate coordinates and symlinked inputs.
Phase 3 also runs the minimal suite with dependency feature unification enabling
`serde_json/arbitrary_precision`, so finite-number checks remain effective when
another application dependency selects that feature.
Isolated `scan`, `management`, `dlp`, `iam`, `model-security`, `red-team`,
`gateway`, and `models` configurations are also
checked with default features disabled on Rust 1.99.0, including all targets,
Clippy, tests, and documentation. This catches product-specific conditional code.

## Continuous integration and platform evidence

The primary workflow is [.forgejo/workflows/ci.yml](../.forgejo/workflows/ci.yml). It runs an offline provenance/review-tool suite and Rust 1.88.0/1.99.0 across default, minimal, and all features. Source checkout is pinned to `actions/checkout` v4.3.1 commit `34e114876b0b11c390a56381ad16ebd13914f8d5`, verified against the upstream repository on 2026-10-16. It does not persist checkout credentials. No service credentials, release tokens, publication steps, or external mirror configuration are included.

Automated dependency advisory scanning and machine-enforced license/source policy are deferred to phase 8 and recorded as a phase-2 limitation in `docs/phase-status.json`. Current CI establishes the checks listed above; dependency-policy qualification remains pending until that gate is implemented and executed.

The `ubuntu-latest` Forgejo runner label must provide Git, Python, rustup, Node 20 for the checkout action, and a C/C++ compiler. This is a runner requirement, not evidence that a runner is registered. Remote CI execution is pending until an actual run is retained. Additional native jobs are opt-in through `workflow_dispatch` after their runner labels exist; they assert the compiler host before executing tests. A cross-compilation result cannot substitute for native runtime evidence. [Forgejo runner labels](https://forgejo.org/docs/latest/user/actions/quick-start/) and [remote action URLs](https://forgejo.org/docs/latest/user/actions/actions/) describe this integration.

| Target | Verification configuration | Evidence state |
| --- | --- | --- |
| `x86_64-unknown-linux-musl` | Rust 1.88.0 and 1.99.0, all features | Native foundation checks passed on 2026-10-16: [MSRV](checks/rust-1.88.0-all.json), [current](checks/rust-1.99.0-all.json). Later phase reports supersede these snapshots. |
| `x86_64-unknown-linux-gnu` | Ordinary `ubuntu-latest` jobs | Configured; remote native execution pending. |
| `aarch64-unknown-linux-gnu` | Opt-in `linux-arm64` runner | Runner registration and native execution pending. |
| `aarch64-apple-darwin` | Opt-in `macos-arm64` runner | Runner registration and native execution pending. |
| `x86_64-pc-windows-msvc` | Opt-in `windows-amd64` runner | Runner registration and native execution pending. |

## Phase acceptance

Run `python3 tools/check_phase.py --phase N` for the active phase. That complete check report and an actual Claude Code review are required by the repository gate. Address findings and repeat until the review reaches at least 9/10. Platform qualification, dependency advisories, packaged consumer verification, and successful live acceptance remain separate deliverables; a passing compiler build does not establish them.
