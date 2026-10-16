# SCM SDK for Rust

`scm-sdk` is a community-maintained native Rust SDK for Strata Cloud Manager.
Prisma AIRS is the first product family, with shared DLP and IAM capabilities.

Implementation proceeds through the gated [phases](docs/PHASES.md), following the
[accepted design](docs/implementation-plan.md). [Phase status](docs/phase-status.json)
records actual progress and Claude Code reviews. Implementation, independent
contract verification, and live acceptance are measured separately.

Current state: Phases 1–5 passed Claude Code review at 9/10 and are pushed.
Phase 6 implements Gateway control, telemetry and runtime APIs and is undergoing
verification. Experimental capabilities, export closure and qualification follow
in order. No SDK release is available yet.

The [foundation interface](docs/FOUNDATION.md) describes explicit authentication,
transport injection, cancellation, resource limits, and safe error handling.
See [development](docs/DEVELOPMENT.md) for native checks and CI coverage.
The [model contracts](docs/MODELS.md), [validator](docs/VALIDATION.md), and
[streaming interface](docs/STREAMING.md) describe the native data boundaries.
Product guides cover [management](docs/MANAGEMENT.md), [DLP](docs/DLP.md),
[IAM](docs/IAM.md), [Model Security](docs/MODEL_SECURITY.md),
[Red Team](docs/RED_TEAM.md), [Red Team resources](docs/RED_TEAM_RESOURCES.md),
[Gateway](docs/GATEWAY.md), [Gateway controls](docs/GATEWAY_CONTROL.md),
[Gateway runtime](docs/GATEWAY_RUNTIME.md), and
[pagination and partial collections](docs/PAGINATION.md).

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
