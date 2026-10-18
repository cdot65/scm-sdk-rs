# SCM SDK for Rust

`scm-sdk` is a community-maintained native Rust SDK for Strata Cloud Manager,
beginning with Prisma AIRS and shared DLP/IAM capabilities. Consumers need Cargo
and an application-owned async runtime; Node is absent from the packaged crate.

The implementation covers **all 448 concrete methods** and **all 3,820 exported
capabilities** in frozen `@cdot65/prisma-airs-sdk` **0.34.0**, commit
`4a80dbdb3dc14365e1c25d838ac0bd139485d48c`. Capabilities include 2,630 root exports
and 1,190 public members; methods are part of that inventory. Native adaptations
and exact public paths are recorded in the [operation matrix](contracts/native-operations.json)
and [export ledger](contracts/native-exports.json).

The baseline implementation phases and maintenance releases require actual Claude
Code review of at least 9/10. [Phase status](docs/phase-status.json) records the
accepted reviews and exact source evidence.

Crate version: **0.1.1**. The [registry release](https://crates.io/crates/scm-sdk/0.1.1)
and [API documentation](https://docs.rs/scm-sdk/0.1.1/scm_sdk/) correspond to this
version. The [publication receipt](docs/checks/release-registry-0.1.1.json), retained
after upload, establishes actual availability, archive checksum and source commit.
The [initial 0.1.0 receipt](docs/checks/release-registry-0.1.0.json) remains historical.
See [response defaults](docs/RESPONSE_DEFAULTS.md), the
[offline example](crates/scm-sdk/examples/response_defaults.rs),
[CI evidence](docs/CI.md) and the [release procedure](docs/RELEASE.md).
The [qualification summary](contracts/qualification.json) consolidates the
source-bound implementation, checks, live classifications and delivery limitations.

## Start here

Add the published crate, optionally using the shorter dependency alias:

```toml
[dependencies]
scm = { package = "scm-sdk", version = "0.1" }
```

The alias exposes `scm::airs::scan`, `scm::airs::management`, and the other product
modules. Add opt-in product names through Cargo's `features` field. For a Git
dependency, use `ssh://git@git-ssh.cdot.io/cdot/scm-sdk-rs.git` and pin `rev` to
the accepted commit you intend to consume. The [compiled scan example](crates/scm-sdk/examples/scan.rs)
shows typed content, profile selection, and async scan submission; the application
owns its Tokio runtime and supplies credentials explicitly.

Build the checkout with Rust **1.89** or newer. Defaults enable scanning and the
reqwest HTTP adapter. The [foundation example](crates/scm-sdk/examples/foundation.rs)
is compiled and included in rustdoc. Use [configuration](docs/CONFIGURATION.md)
for isolated initialization and explicit environment loading, [OAuth](docs/OAUTH.md)
for service credentials, and [foundation](docs/FOUNDATION.md) for transport,
proxy/custom-CA setup, destination admission, cancellation and resource limits.

Product guides cover [scanning](docs/SCANNING.md), [management](docs/MANAGEMENT.md),
[DLP](docs/DLP.md), [IAM](docs/IAM.md), [Model Security](docs/MODEL_SECURITY.md),
[Red Team](docs/RED_TEAM.md), [Red Team resources](docs/RED_TEAM_RESOURCES.md),
[Gateway](docs/GATEWAY.md), [Gateway controls](docs/GATEWAY_CONTROL.md), and
[Gateway runtime](docs/GATEWAY_RUNTIME.md). Shared behavior is documented in
[models](docs/MODELS.md), [validation](docs/VALIDATION.md),
[pagination](docs/PAGINATION.md), [streaming](docs/STREAMING.md),
[provisioning/recovery](docs/PROVISIONING.md), and [errors](docs/ERRORS.md).

## Features and qualification

Other product clients are opt-in through their named Cargo features. Experimental
[AgentGuard](docs/AGENTGUARD.md) uses `agentguard`. [Realtime](docs/REALTIME.md)
uses `realtime` with a caller connector; `ws-transport` adds the native connector
pinned to patched `tokio-websockets` 0.13.3. Optional [tracing](docs/OBSERVABILITY.md)
emits fixed metadata fields without installing a subscriber or logging payloads.
The application owns credentials, runtime, transports and enforcement policy.

Implementation and offline contract coverage do not imply universal live support.
See [live acceptance](docs/LIVE_ACCEPTANCE.md), its current
[receipt pointer](contracts/live-evidence.json), and the per-method operation
matrix for actual successful, blocked and unverified workflows. [Platforms](docs/PLATFORMS.md)
separates Linux musl native execution from other targets' compile-only evidence.
[Dependencies](docs/DEPENDENCIES.md) records advisory/license/source policy and
native cryptography; [packaging](docs/PACKAGING.md) verifies the extracted archive
and external consumers. [Stability](docs/STABILITY.md), [security](docs/SECURITY.md)
and [release](docs/RELEASE.md) describe compatibility and remaining release steps.

## Development

Follow [development](docs/DEVELOPMENT.md), the [accepted design](docs/implementation-plan.md)
and sequential [phase gates](docs/PHASES.md). Development-only frozen TypeScript
and Zod tooling provide independent contract evidence; Cargo consumers build the
packaged Rust sources. Preserve failed review/check attempts and require an actual
Claude Code score of at least 9 before advancing a phase.

Maintained by `cdot65`. Project-owned code is Apache-2.0 with retained notices.
This is not an official Palo Alto Networks SDK. See [CHANGELOG.md](CHANGELOG.md).
