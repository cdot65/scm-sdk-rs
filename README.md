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

Historical phases used actual Claude Code review of at least 9/10. Current phases
use five independent review categories: chunks require at least 42/50 and phases
at least 46/50, with each category at least 8/10. The five-attempt exception and
recorded deficiencies follow the maintainer’s review policy. [Phase status](docs/phase-status.json) records the
accepted reviews and exact source evidence.

Crate version: **0.2.4**, the TypeScript 0.35.0 directional-profile update.
Publication and delivery are established only by verified version-specific
registry, annotated-tag and CI receipts. It retains AIRS management 0.86.0 and the
[G25 response-compatibility correction](docs/G25_COMPATIBILITY.md).
It retains five experimental
[Portkey access-policy APIs](docs/ACCESS_POLICIES.md) and retains full
[Red Team dataplane 0.93.0 coverage](docs/RED_TEAM.md): **47 operations**, including
11 new calls, with a separately pinned vendor ledger. See the
[migration notes](CHANGELOG.md) before adopting the stricter query contract.

Directional profiles add `ContentTypeConfiguration` and
`ContentTypeConfigurations`; see the [directional-profile guide](https://github.com/cdot65/scm-sdk-rs/blob/v0.2.4/docs/DIRECTIONAL_PROFILES.md).
Create/update requests now reject invalid newly modeled settings before I/O;
profile responses retain tolerant parsing and preserve unknown or null values.

A verified earlier release is [0.2.3](https://crates.io/crates/scm-sdk/0.2.3),
with a verified [registry receipt](docs/checks/release-registry-0.2.3.json).

The earlier maintenance release is [0.1.1](https://crates.io/crates/scm-sdk/0.1.1),
with [published API docs](https://docs.rs/scm-sdk/0.1.1/scm_sdk/) and a verified
[publication receipt](docs/checks/release-registry-0.1.1.json). For its G10 behavior,
see [response defaults](docs/RESPONSE_DEFAULTS.md) and the
[offline example](crates/scm-sdk/examples/response_defaults.rs). The
[initial 0.1.0 receipt](docs/checks/release-registry-0.1.0.json) remains historical.
The earlier vendor update [scm-sdk 0.2.0](https://crates.io/crates/scm-sdk/0.2.0) remains historical.
Its [registry receipt](docs/checks/release-registry-0.2.0.json) records the verified
archive and exact release source; its immutable tag is `v0.2.0`.
See [CI](docs/CI.md) and the [release procedure](docs/RELEASE.md).
The [qualification summary](contracts/qualification.json) consolidates the
source-bound implementation, checks, live classifications and delivery limitations.

The 0.2.4 `docs/checks/release-registry-0.2.4.json` and
`docs/checks/release-tag-v0.2.4.json` establish publication only when
present and verified; missing receipts mean that publication is not yet verified.
The [published 0.2.2 receipt](docs/checks/release-registry-0.2.2.json) remains historical.
The access-policy client requires explicit compatible control-plane key credentials
and remains live-unverified. Existing 0.2.0 APIs and G10 behavior are retained.

## Start here

After the 0.2.4 registry receipt above verifies, pin that release using the shorter
dependency alias. If publication is unverified, retain your existing released pin;
the earlier 0.2.3 registry receipt remains available above:

```toml
[dependencies]
scm = { package = "scm-sdk", version = "=0.2.4" }
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
See [live acceptance](docs/LIVE_ACCEPTANCE.md), the archived
[historical 0.2.2 receipt](contracts/historical/0.2.2-live-evidence.json), and the per-method operation
matrix for actual successful, blocked and unverified workflows. The archived receipt
does not qualify the current management changes; fresh live evidence belongs to release qualification. [Platforms](docs/PLATFORMS.md)
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
independent five-category review meeting the current phase threshold before advancing.
Phase 20 CI comprises 49 jobs: 24 feature sets on two compilers and the contract gate.

Maintained by `cdot65`. Project-owned code is Apache-2.0 with retained notices.
This is not an official Palo Alto Networks SDK. See [CHANGELOG.md](CHANGELOG.md).
