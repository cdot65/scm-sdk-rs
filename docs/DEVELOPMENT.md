# Development and verification

The SDK is developed in this repository. Consumers build the packaged Rust crate; Node and the TypeScript checkout are development inputs for contract extraction only. Follow `AGENTS.md`, the active phase in `docs/PHASES.md`, and the accepted design in `docs/implementation-plan.md`.

## Prerequisites

Use Python 3.11 or newer, Git, rustup, and a native C/C++ toolchain. The default reqwest/rustls transport builds AWS-LC native code. The pinned non-FIPS configuration needs a C/C++ compiler; it does not require CMake, Go, or bindgen. Windows runners need the MSVC build tools. See [DEPENDENCIES.md](DEPENDENCIES.md) for exact versions, sources, and the transitive native-code boundary.

Install exact toolchains without changing the machine's default:

```sh
rustup toolchain install 1.89.0 --profile minimal --component rustfmt,clippy --no-self-update
rustup toolchain install 1.99.0 --profile minimal --component rustfmt,clippy --no-self-update
```

The workspace now declares Rust 1.89 as its minimum supported Rust version. The optional native WebSocket adapter pins patched `tokio-websockets` 0.13.3, whose declared minimum is 1.89; current ICU and time dependencies retain their 1.88 minimum. The latest pinned verification toolchain is 1.99.0. Phase 7's complete source/feature gate is under verification; a declared dependency minimum is not executable qualification.

Earlier accepted phases retain their actual Rust 1.88.0/1.99.0 reports, including the foundation all-feature build, Clippy, tests and documentation checks on the local musl host on 2026-10-16. Those historical results do not qualify the new WebSocket dependency. Rust 1.95.0 remains the local machine's installed default; commands select their toolchain explicitly. The helper finds Cargo through `PATH` or the rustup proxy in `.cargo/bin`; `--cargo PATH` overrides discovery.

## Native Rust checks

Run each supported feature configuration on each toolchain:

```sh
python3 scripts/check-rust.py --toolchain 1.89.0 --feature-set minimal
python3 scripts/check-rust.py --toolchain 1.89.0 --feature-set default
python3 scripts/check-rust.py --toolchain 1.89.0 --feature-set all
python3 scripts/check-rust.py --toolchain 1.99.0 --feature-set minimal
python3 scripts/check-rust.py --toolchain 1.99.0 --feature-set default
python3 scripts/check-rust.py --toolchain 1.99.0 --feature-set all
```

The helper checks formatting without modifying files, builds all targets, runs Clippy with warnings denied, runs tests including any crate doctests, and builds documentation with warnings denied. Cargo uses the committed lockfile. `minimal` means `--no-default-features`; `all` means `--all-features`.

Product features are `scan`, `management`, `dlp`, `iam`, `model-security`, `red-team`, `gateway`, and experimental `agentguard`. `models` selects shared native contracts and `reqwest-transport` selects the default HTTP adapter. Defaults enable the adapter and scanning. Management includes canonical DLP; Gateway includes canonical IAM for its scope accessor. IAM is independently selectable.

`realtime` includes Gateway and adds a caller-supplied WebSocket connector. `ws-transport` additionally selects the optional native connector. Its transport policy is separate from HTTP: direct TCP/TLS, explicit trust configuration, no inherited reqwest proxy settings, no redirects or extensions, no global logging/crypto changes, and currently 64-bit targets only. Supply a connector for proxies, other network stacks or other target widths. See [REALTIME.md](REALTIME.md) and [DEPENDENCIES.md](DEPENDENCIES.md).

Phases 3–6 implement scanning, management/DLP/IAM, Model Security, Red Team and Gateway HTTP control/telemetry/runtime. Active Phase 7 adds AgentGuard, realtime, provisioning/recovery, explicit configuration/environment and helper/error equivalents, and exhaustive native export reconciliation. Minimal builds require caller-supplied transport and retain validation/data/auth/configuration primitives. There is no empty tracing feature. Keep default/minimal/all and isolated feature matrices as source changes.

Each invocation writes `.local/checks/rust-TOOLCHAIN-FEATURE.json`, with exact commands, outputs, exit codes, durations, compiler/host information, lockfile hash, and a fingerprint of Rust build inputs. Source changes during a run fail the evidence check. The helper targets the compiler's native host explicitly and verifies the packaged license/notice copies. It removes `PANW_`, `AIRS_`, and `SCM_` environment variables from test subprocesses. Tests use synthetic credentials and local fixtures. Live acceptance follows [LIVE-ACCEPTANCE.md](LIVE-ACCEPTANCE.md).

To isolate build caches without altering global Cargo configuration:

```sh
CARGO_TARGET_DIR=.local/target-msrv python3 scripts/check-rust.py --toolchain 1.89.0 --feature-set all
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
python3 tools/native_exports.py --check
python3 tools/generate_constants.py --check
python3 tools/verify_native_fixtures.py
.local/typescript/node_modules/.bin/tsx tools/schema-oracle.mts --check
python3 tools/generate_models.py --verify-fixtures
```

The oracle executes frozen Zod schemas with network and credential access removed.
Rust integration tests compare both request normalization and response preservation.
The separate native consumer round-trips every successful public JSON schema
specimen; Blob-only roots have typed binary tests. Node is absent from the crate.
`contracts/native-exports.json` additionally binds frozen exported roots/members
to public paths, compiled consumer probes, concrete tests and deliberate adapters.
A mapped path without its behavior tests does not establish parity acceptance.
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
`gateway`, and `models` configurations have earlier accepted isolated checks
with default features disabled on Rust 1.99.0. Phase 7 adds isolated `agentguard`,
`realtime`, and `ws-transport` gates. Preserve the exact phase report for the actual
all-target build, Clippy, tests and documentation results; new feature declarations
alone do not establish those outcomes.

## Continuous integration and platform evidence

The primary workflow is [.forgejo/workflows/ci.yml](../.forgejo/workflows/ci.yml). Its **historically configured** matrix still selects Rust 1.88.0/1.99.0 across default/minimal/all features. That file has not yet been updated for the new 1.89 workspace minimum; Phase 8 owns the CI update and actual retained remote execution. The existing 1.88 jobs cannot qualify the current source. The provenance/review-tool job and opt-in additional native runner declarations remain configured.

Source checkout is pinned to `actions/checkout` v4.3.1 commit `34e114876b0b11c390a56381ad16ebd13914f8d5`, verified against upstream on 2026-10-16. Checkout credentials are not persisted. No service credentials, release tokens, publication steps or external mirror configuration are included.

Automated dependency advisory scanning and machine-enforced license/source policy are deferred to phase 8 and recorded as a phase-2 limitation in `docs/phase-status.json`. The current workflow records intended historical checks; no retained remote run establishes execution. Dependency-policy qualification remains pending until the Phase 8 gate is implemented and executed.

The `ubuntu-latest` Forgejo runner label must provide Git, Python, rustup, Node 20 for the checkout action, and a C/C++ compiler. This is a runner requirement, not evidence that a runner is registered. Remote CI execution is pending until an actual run is retained. Additional native jobs are opt-in through `workflow_dispatch` after their runner labels exist; they assert the compiler host before executing tests. A cross-compilation result cannot substitute for native runtime evidence. [Forgejo runner labels](https://forgejo.org/docs/latest/user/actions/quick-start/) and [remote action URLs](https://forgejo.org/docs/latest/user/actions/actions/) describe this integration.

| Target | Verification configuration | Evidence state |
| --- | --- | --- |
| `x86_64-unknown-linux-musl` | Historical Rust 1.88.0/1.99.0; current declared minimum 1.89 | Native foundation reports: [historical MSRV](checks/rust-1.88.0-all.json), [historical current](checks/rust-1.99.0-all.json). Accepted Phase 1–6 reports retain their source state; Phase 7 full qualification is pending its gate. |
| `x86_64-unknown-linux-gnu` | Historical ordinary `ubuntu-latest` jobs | CI/MSRV update and remote native execution pending Phase 8. |
| `aarch64-unknown-linux-gnu` | Opt-in `linux-arm64` runner | Runner registration and native execution pending. |
| `aarch64-apple-darwin` | Opt-in `macos-arm64` runner | Runner registration and native execution pending. |
| `x86_64-pc-windows-msvc` | Opt-in `windows-amd64` runner | Runner registration and native execution pending. |

## Phase acceptance

Phases 1–6 were accepted at actual Claude Code 9/10 and pushed through `41ddab3`. Phase 7 is under verification; Phase 8 qualification is pending. New source and export mappings do not replace the required phase gate.

Run `python3 tools/check_phase.py --phase N` for the active phase. That complete check report and an actual Claude Code review are required by the repository gate. Address findings and repeat until the review reaches at least 9/10. Platform qualification, dependency advisories, packaged consumer verification, and successful live acceptance remain separate deliverables; a passing compiler build does not establish them.
