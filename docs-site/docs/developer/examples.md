---
title: Rust Examples
---

# Rust examples

Examples are included with the crate source. Offline examples validate models without credentials or service calls. Examples that contact a service use an application-owned runtime and explicit credentials; review their environment inputs and Cargo features before running them because they can create billable or persistent resources.

- [Access Policies](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/access_policies.rs)
- [Directional Profiles](https://github.com/cdot65/scm-sdk-rs/blob/6f18529a47cc273feca1000af6e4fc0d96da757e/crates/scm-sdk/examples/directional_profiles.rs) — offline; see the [guide](/airs/directional-profiles).
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
| `directional_profiles` | `management` |
| `response_defaults` | `red-team` |
| `response_compatibility` | `red-team` |
| `red_team_management` | `red-team` |

Other examples have no additional `required-features` declaration in Cargo.toml;
inspect each source for optional feature-gated demonstrations and inputs.
