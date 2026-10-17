# Development and verification

Consumers build native packaged Rust sources. Node and the frozen TypeScript
checkout are development-only contract/oracle inputs. Follow `AGENTS.md`,
[PHASES.md](PHASES.md) and the accepted [design](implementation-plan.md).

All **448 methods** and **3,820 exported capabilities** of the frozen TypeScript
0.34.0 baseline are implemented with explicit native adaptations. The
[phase records](phase-status.json) bind each accepted Claude Code review to its
exact source and passing checks. Implementation, executed contracts, live
acceptance and release publication retain separate evidence.

## Prerequisites

Use Python 3.12+ (CI pins 3.14.7 in `.python-version`), Git, rustup and a native
C/C++ toolchain. The default
reqwest/rustls transport uses non-FIPS AWS-LC native code with generated bindings;
the selected configuration does not require CMake, Go or bindgen. Windows native
runners need MSVC tools. See [DEPENDENCIES.md](DEPENDENCIES.md).

Install exact compilers without changing the machine's default:

```sh
rustup toolchain install 1.89.0 --profile minimal --component rustfmt,clippy --no-self-update
rustup toolchain install 1.99.0 --profile minimal --component rustfmt,clippy --no-self-update
```

Rust 1.89 is the workspace minimum, including minimal builds. Patched optional
`tokio-websockets` 0.13.3 declares that minimum; ICU/time retain their 1.88 minimum.
Earlier accepted Rust 1.88.0/1.99.0 reports remain historical evidence of their
original source/dependencies. Current reports select toolchains explicitly.

## Native checks and features

```sh
python3 scripts/check-rust.py --toolchain 1.89.0 --feature-set minimal
python3 scripts/check-rust.py --toolchain 1.89.0 --feature-set default
python3 scripts/check-rust.py --toolchain 1.89.0 --feature-set all
python3 scripts/check-rust.py --toolchain 1.99.0 --feature-set minimal
python3 scripts/check-rust.py --toolchain 1.99.0 --feature-set default
python3 scripts/check-rust.py --toolchain 1.99.0 --feature-set all
```

The checker verifies formatting, all-target builds, strict Clippy, tests/doctests
and rustdoc with warnings denied. Cargo uses the committed lockfile and actual
compiler host. Reports preserve commands, output, compiler/host, hashes and
outcomes; source changes during a run invalidate its evidence. Synthetic tests
have `PANW_`, `AIRS_` and `SCM_` environment values removed. Live execution is a
separately selected workflow.

Cargo build output is ignored at every workspace depth. The standalone export
consumer uses the root `target` directory when `CARGO_TARGET_DIR` is absent or
empty, and preserves a nonempty caller choice. Keep generated build output in
ignored directories so it cannot change source fingerprints or consume the
bounded private-material inventory. See the retained diagnosis in [CI.md](CI.md).

Defaults are `scan` and `reqwest-transport`. Named product features are
`management`, `dlp`, `iam`, `model-security`, `red-team`, `gateway` and `agentguard`;
`models` selects shared contracts. Management includes canonical DLP and Gateway
includes canonical IAM. Minimal builds retain data/validation/auth/configuration
primitives and require an injected transport for requests.

`realtime` adds a caller WebSocket connector and includes Gateway.
`ws-transport` adds the optional native direct TCP/TLS adapter, currently 64-bit
only. It does not inherit HTTP proxy/root configuration. Proxies or other stacks
use a custom connector; explicit native PEM/TLS options configure trust.
Optional `tracing` emits fixed metadata fields without a subscriber or worker.
See [REALTIME.md](REALTIME.md) and [OBSERVABILITY.md](OBSERVABILITY.md).

The matrix also checks each isolated feature, `json-unification` and
`gateway-json-unification` (`serde_json/arbitrary_precision`), tracing combinations
`default-tracing`, `realtime-tracing`, `ws-tracing`, and the exact `native-probes`
and `probe-base` combinations. These checker names are defined in
`scripts/check-rust.py`; they are feature sets, not additional Cargo features.
Preserve actual phase reports for each executed combination.

## Public Rustdoc coverage boundary

Current required item documentation covers all public items in the handwritten
scanner entry module (`Region`, `SyncScanOptions`, `Scanner` and their members),
plus the primary constructor of each product facade: `ManagementClient`,
`DlpClient`, `IamScopesClient`, `ModelSecurityClient`, `RedTeamClient`,
`AIGatewayClient`, `RuntimeClient` and `AgentGuardClient`. Each product constructor
has a short compiled `no_run` example with its Cargo feature, credential plane
and endpoint requirements explained. Examples accept caller-configured clients
and use async functions polled by the application; documentation checks perform
no service requests or credential lookups. Adding a product entry point requires
the same item documentation and compiled example during review.

`scanner.rs` enables a scoped `warn(missing_docs)` lint; the normal warnings-denied
checks enforce that complete module boundary. The other facade constructor
examples are required by code review and compiled doctests.

Workspace-wide `missing_docs` warnings are deliberately disabled. The generated
schema/model namespaces contain thousands of mechanical fields, builders,
variants and aliases whose wire contracts come from the frozen schema graph;
blanket warnings would require repetitive generated prose without documenting
handwritten behavior. Their schema identifiers, validation APIs and
[model guide](MODELS.md) provide the contract reference. Generator changes must
retain that provenance and the existing structural/round-trip checks.

The entry-point requirement is the present Rustdoc coverage boundary, not a claim
that every resource method has individual item documentation. Product guides and
the operation ledger cover those resources. As handwritten methods are added or
changed, document behavior that affects the caller's decisions: authentication,
plane selection, validation, pagination completeness, deadlines/cancellation,
replay and uncertain remote outcomes. Review those descriptions against the
corresponding independent contract tests.

Rustdoc warnings (including broken links) remain errors in the checker. Doctests
compile only the entry points enabled by their enclosing product feature; the
default/minimal/all matrix checks the boundary without enabling product features
inside examples. To run focused documentation checks directly:

```sh
cargo +1.99.0 test --locked --no-default-features --doc
cargo +1.99.0 test --locked --doc
cargo +1.99.0 test --locked --all-features --doc
RUSTDOCFLAGS="-D warnings" cargo +1.99.0 doc --locked --all-features --no-deps
```

To isolate caches without changing global Cargo configuration:

```sh
CARGO_TARGET_DIR=.local/target-msrv python3 scripts/check-rust.py --toolchain 1.89.0 --feature-set all
```

## Frozen contracts and native probes

The baseline is TypeScript **0.34.0** commit
`4a80dbdb3dc14365e1c25d838ac0bd139485d48c`. Use the pinned development Node/tooling
versions in `.node-version` and `tools/package-lock.json`; do not regenerate from
uncommitted changes in another checkout.

```sh
python3 tools/bootstrap_reference.py
python3 tools/verify-contracts.py --baseline .local/typescript --self-test
.local/typescript/node_modules/.bin/tsx tools/extract-contracts.mts --check
.local/typescript/node_modules/.bin/tsx tools/extract-operations.mts --check
python3 tools/generate_validation.py --check
python3 tools/generate_models.py --check
python3 tools/native_operations.py --check --require-through-phase 8
python3 tools/native_exports.py --check
python3 tools/generate_constants.py --check
python3 tools/verify_native_fixtures.py
.local/typescript/node_modules/.bin/tsx tools/schema-oracle.mts --check
python3 tools/generate_models.py --verify-fixtures
python3 scripts/check-native-exports.py --toolchain 1.89.0
python3 scripts/check-native-exports.py --toolchain 1.99.0
```

Extractors verify source/compiler provenance before evaluation. The independent
oracle executes frozen Zod with network/credential access removed. Rust tests
compare request normalization, response evolution and typed round trips;
non-JSON boundaries have concrete native adapters. The source-bound
[operation matrix](../contracts/native-operations.json) and
[export ledger](../contracts/native-exports.json) reject missing adapters,
unrelated probes and denominator drift. The independent Cargo consumer proves
public reachability; behavior and live service acceptance have their own tests.

The official Gateway specification stays excluded from redistribution/generation
while its license status is unresolved. Frozen permitted source, hashes, licenses,
fixture pointers and deliberate corrections remain in the provenance manifest.

## Dependency, package and platform qualification

```sh
python3 scripts/check-dependencies.py --output .local/checks/dependencies.json
python3 scripts/check-cross.py --output .local/checks/cross.json
python3 scripts/check-package.py --toolchain 1.89.0 --output .local/checks/package-1.89.0.json
python3 scripts/check-package.py --toolchain 1.99.0 --output .local/checks/package-1.99.0.json
```

The dependency checker requires cargo-audit 0.22.2 and cargo-deny 0.20.2, fetches
fresh authoritative RustSec HEAD, and enforces complete resolved
license/source/checksum policy. Install the exact tools on PATH or use its
`--tools-dir` option. [DEPENDENCIES.md](DEPENDENCIES.md) describes report scope.

Package checks validate the actual archive and execute minimal/all-feature units,
examples/doctests and the external public consumer against its extracted source.
The repository's integration corpus remains separate. [PACKAGING.md](PACKAGING.md)
records actual contents and archive proof; [RELEASE.md](RELEASE.md) explains the
still-pending clean-commit check after final source commit. An allow-dirty smoke
check does not close that release obligation.

[PLATFORMS.md](PLATFORMS.md) distinguishes Linux x86_64 musl native execution from
GNU, ARM Linux, Apple and Windows compile-only checks with caller transports.
Cross checks exclude reqwest/native WS/TLS/cryptography and do not run a target
binary. Other runtime targets, Wasm and 32-bit native adapters remain unqualified.

## Continuous integration

The configured [Forgejo workflow](../.forgejo/workflows/ci.yml) now selects Rust
1.89.0/1.99.0, default/minimal/all and the complete isolated/combination matrix.
It configures pinned development tooling, source/oracle checks, dependency policy,
compile-only targets and archive/consumer verification. The first reported remote
run failed during setup because `rustup` was absent. Current execution evidence is
recorded in [phase status](phase-status.json); configuration alone is not CI evidence.

Checkout/setup actions are pinned to exact commits and do not retain checkout
credentials. No service credentials, registry tokens, publication step or mirror
configuration is installed. Every job uses the pinned `dtolnay/rust-toolchain`
action to bootstrap rustup when absent, expose Cargo on subsequent steps' PATH,
and install its exact Rust compiler with rustfmt and Clippy. Pinned setup-uv
installs uv 0.13.1 with an explicit archive checksum and a fresh owned tool cache.
`scripts/uv-integrity.cjs` selects the host/libc pins from
`scripts/uv-checksums.json`; `scripts/bootstrap-python.sh` hashes the exact
installed executable before invoking it, then installs the portable
interpreter selected by `.python-version` into job-local directories and exports
its executables through the runner PATH file. It requires no preinstalled Python
or Ubuntu-specific interpreter distribution. Updating uv requires reviewed
archive and executable digests for every configured host. Every job uses
setup-node to select the pinned development version before the integrity helper.
The runner must provide Git, Node 20 for
JavaScript actions, Bash, HTTPS/curl support, and a C/C++ compiler. Rust and Python
need not be preinstalled. uv is also installed explicitly. The Rust CI action
sets the runner's default compiler;
project scripts select explicit versions and preserve the developer's default.
Additional native platform jobs require
registered runner labels and explicit dispatch. Their declarations are not runtime
qualification. Local native and cross reports remain independently retained.

## Phase acceptance and live evidence

Run `python3 tools/check_phase.py --phase 8`, request a fresh actual Claude Code
review, improve findings and repeat until at least 9/10. CI can rerun checks with
`--verification-only` without advancing status or requesting review. The source
fingerprint and exact reports bind each accepted phase; preserve prior attempts.

Ordinary checks do not contact service APIs. Live qualification selects external
registration and expected scope explicitly, bounds operations/cost, and records
cleanup/recovery. Follow [LIVE_ACCEPTANCE.md](LIVE_ACCEPTANCE.md) and the current
[receipt pointer](../contracts/live-evidence.json). Service availability,
permissions and unverified methods remain separate from implemented parity.
Registry publication, a clean final commit/artifact and retained remote CI
execution require their own actual proof.
