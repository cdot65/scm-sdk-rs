# SCM SDK for Rust

`scm-sdk` is a community-maintained native Rust SDK for Strata Cloud Manager.
Prisma AIRS is the first product family, with shared DLP and IAM capabilities.

Implementation proceeds through the gated [phases](docs/PHASES.md), following the
[accepted design](docs/implementation-plan.md). [Phase status](docs/phase-status.json)
records actual progress and Claude Code reviews. Implementation, independent
contract verification, and live acceptance are measured separately.

Current state: Phases 1–6 passed Claude Code review at 9/10 and are pushed through
`41ddab3`. Phase 7 implements experimental AgentGuard/realtime, provisioning,
configuration/helpers/error equivalents and export closure, and is under
verification. Phase 8 qualification remains pending. Full parity is not yet
accepted, and no SDK release is available.

The [foundation interface](docs/FOUNDATION.md) describes explicit authentication,
transport injection, cancellation, resource limits, and safe error handling.
[Configuration](docs/CONFIGURATION.md) covers isolated initialization and explicit
environment loading; [OAuth](docs/OAUTH.md) covers refresh buffers and safe
metadata subscriptions.
See [development](docs/DEVELOPMENT.md) for native checks and CI coverage.
The [model contracts](docs/MODELS.md), [validator](docs/VALIDATION.md), and
[streaming interface](docs/STREAMING.md) describe the native data boundaries.
Product guides cover [scanning](docs/SCANNING.md),
[management](docs/MANAGEMENT.md), [DLP](docs/DLP.md),
[IAM](docs/IAM.md), [Model Security](docs/MODEL_SECURITY.md),
[Red Team](docs/RED_TEAM.md), [Red Team resources](docs/RED_TEAM_RESOURCES.md),
[Gateway](docs/GATEWAY.md), [Gateway controls](docs/GATEWAY_CONTROL.md),
[Gateway runtime](docs/GATEWAY_RUNTIME.md), and
[pagination and partial collections](docs/PAGINATION.md). Phase 7 guides cover
[AgentGuard](docs/AGENTGUARD.md), [realtime](docs/REALTIME.md),
[provisioning/recovery](docs/PROVISIONING.md), and [errors](docs/ERRORS.md).

## Native crate and features

The workspace declares Rust 1.89 as its minimum. Default features enable scanning
and the reqwest HTTP adapter. Other product families are opt-in. `realtime` adds
caller-driven experimental connections; `ws-transport` additionally selects the
optional native WebSocket adapter pinned to patched `tokio-websockets` 0.13.3.
The application owns its runtime, transports, destinations and credentials.
Node is absent from the packaged crate. Current and historical platform evidence
and the pending CI/MSRV update are recorded in [development](docs/DEVELOPMENT.md).

## Development contracts

Restore the pinned development reference with `python tools/bootstrap_reference.py`.
An existing source checkout can be supplied with `--source-repository PATH`.
The reference and Node tooling are used to extract contracts during development;
Rust consumers build the packaged crate with Cargo.

Verify frozen provenance with `python tools/verify-contracts.py`, and verify tamper
rejection separately with `python tools/verify-contracts.py --self-test`.
Check extraction using `.local/typescript/node_modules/.bin/tsx tools/extract-contracts.mts --check`
and `.local/typescript/node_modules/.bin/tsx tools/extract-operations.mts --check`.

Run `python tools/check_phase.py --phase N` to record all current phase checks.
Request a Claude review with `python tools/review_phase.py --phase N --attempt M`.
After an accepted review, `python tools/review_phase.py --phase N --verify REVIEW_PATH --advance`
checks its source fingerprint and advances the active phase. Preserve failed attempts.

Maintained by `cdot65`. Project-owned code is Apache-2.0. This project is not an
official Palo Alto Networks SDK.
