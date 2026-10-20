---
title: Rust Examples
---

# Rust examples

Examples are included with the crate source. They use an application-owned runtime and explicit credentials. Review each file’s required environment inputs and Cargo features before running it; examples that contact a service can create billable or persistent resources.

- [Access Policies](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/access_policies.rs)
- [Foundation](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/foundation.rs)
- [Gateway](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/gateway.rs)
- [Management](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/management.rs)
- [Model And Red Team](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/model_and_red_team.rs)
- [Pagination](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/pagination.rs)
- [Realtime](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/realtime.rs)
- [Red Team Dataplane](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/red_team_dataplane.rs)
- [Red Team Management](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/red_team_management.rs)
- [Response Compatibility](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/response_compatibility.rs)
- [Response Defaults](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/response_defaults.rs)
- [Scan](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/scan.rs)

The release qualification harness `live_qualification.rs` is intentionally
excluded from this application-example list; its execution and evidence rules
are documented under [Live Qualification](/developer/live-qualification).

Cargo declares these additional feature requirements:

| Example | Required feature |
| --- | --- |
| `access_policies` | `gateway` |
| `response_defaults` | `red-team` |
| `response_compatibility` | `red-team` |
| `red_team_management` | `red-team` |

Other examples have no additional `required-features` declaration in Cargo.toml;
inspect each source for optional feature-gated demonstrations and inputs.
