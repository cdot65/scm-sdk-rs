---
title: SCM SDK for Rust
slug: /overview
---

# One SDK for Strata Cloud Manager

`scm-sdk` is a community-maintained, native async Rust SDK for Strata Cloud Manager. **Prisma AIRS is the first implemented product family**, alongside shared DLP and IAM clients. NGFW and SASE are planned product areas; this release does not provide those clients.

The current published release is **0.2.4**, with Rust **1.89** as its minimum supported compiler. Applications own their async runtime, credentials, transport configuration, and enforcement decisions. Node.js is used to build this documentation site, not by the Rust crate.

The **0.2.4** patch adds [directional security-profile models](/airs/directional-profiles)
with TypeScript 0.35.0 request validation and preserved unknown response values.

## Choose your starting point

| Area | What is available |
| --- | --- |
| [Prisma AIRS](/products/overview) | Runtime Security, security management, Model Security, AI Red Teaming, AI Gateway, and explicitly experimental clients. |
| [Shared SCM services](/shared/oauth) | OAuth, [DLP](/shared/dlp), and [IAM](/shared/iam). |
| [NGFW](/products/ngfw) | Planned; no client surface is advertised for this release. |
| [SASE](/products/sase) | Planned; no client surface is advertised for this release. |

## Build with Rust

```toml
[dependencies]
scm = { package = "scm-sdk", version = "=0.2.4" }
```

Start with [installation](/getting-started/installation) and the [quick start](/getting-started/quick-start). Select optional product features as needed. Use the [API reference](/reference/api/) for client families and Rust API documentation.

## Compatibility and qualification

The SDK preserves the frozen TypeScript SDK capability surface and adds separately documented vendor API updates. [Response compatibility](/core/response-compatibility), [default handling](/core/response-defaults), and [unknown model fields](/core/models) are explicit parts of the Rust contract.

Implementation and offline contract coverage do not imply live acceptance for every tenant or workflow. Consult [live qualification](/developer/live-qualification) and [release notes](/about/release-notes) before deployment.
