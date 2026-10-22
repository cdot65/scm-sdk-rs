---
title: Rust API reference
sidebar_position: 1
slug: /reference/api/
---

# Rust API reference

`scm-sdk` exposes native async Rust clients under the crate name `scm_sdk`.
This reference covers **0.2.4**, with Rust **1.89** as the minimum supported compiler.

| Area | Module | Feature |
| --- | --- | --- |
| [Runtime scanning](./scanning.md) | `scm_sdk::airs::scan` | `scan` (default) |
| [AIRS management](./management.md) | `scm_sdk::airs::management` | `management` |
| [Model Security](./model-security.md) | `scm_sdk::airs::model_security` | `model-security` |
| [Red Teaming](./red-teaming.md) | `scm_sdk::airs::red_team` | `red-team` |
| [AI Gateway](./gateway.md) | `scm_sdk::airs::gateway` | `gateway` |
| [Shared services](./shared.md) | `scm_sdk::dlp`, `scm_sdk::shared::iam` | `dlp`, `iam` |
| [Core client](./core.md) | `scm_sdk` | Core types are always available |

Paths here use `scm_sdk::`; with the dependency alias from the [installation guide](/getting-started/installation), use `scm::` instead.

## Choose the reference you need

These pages explain client entry points, feature selection and behavior that
matters when integrating the SDK. The [generated Rust documentation](https://docs.rs/scm-sdk/0.2.4/scm_sdk/)
provides signatures and type definitions for the features built by docs.rs.

The hosted **0.2.4** documentation uses default features. Optional family client
signatures are available by generating documentation from the SDK source checkout:

```sh
cargo doc -p scm-sdk --all-features --no-deps --open
```

Default features enable `scan` and `reqwest-transport`. Enable additional families
explicitly; `models` alone provides model types without those optional clients.
The SDK uses your application's async runtime.

```toml
[dependencies]
scm-sdk = { version = "=0.2.4", features = ["management", "model-security", "red-team", "gateway"] }
```

Availability of an API method does not establish tenant permission or live
acceptance of a provider workflow. See the family notes and [release notes](../../about/release-notes.md).
