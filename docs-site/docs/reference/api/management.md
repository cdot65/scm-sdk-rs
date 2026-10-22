---
title: AIRS management
sidebar_position: 3
slug: /reference/api/management
---

# AIRS management

**Module:** `scm_sdk::airs::management` · **Feature:** `management`

`ManagementClient::new` wraps a configured OAuth client. Its resource accessors
keep resource-specific requests and response models together.

| Accessor | Resources |
| --- | --- |
| `profiles()` | AI security profiles |
| `topics()` | Custom topics |
| `api_keys()` | Runtime API keys |
| `customer_apps()` | Customer applications |
| `deployment_profiles()` | Deployment profiles |
| `scan_logs()` | Scan log queries |
| `dashboard()` | Dashboard queries |
| `dlp_profiles()` | AIRS DLP profile operations |
| `oauth()` | OAuth management operations |

Use `with_dashboard_endpoint` when dashboard requests require a separately
selected endpoint. `dlp(endpoint)` creates the canonical DLP client with an
explicit endpoint; the `management` feature also enables `dlp`.

Collection helpers expose pagination and bounded collection outcomes. Do not
assume that a bounded collection is complete without inspecting its completion
status. Mutating operations can report an uncertain remote outcome after a
transport failure; reconcile service state before retrying a write.

## Models and signatures

The [management module](https://docs.rs/scm-sdk/0.2.3/scm_sdk/airs/management/index.html)
contains its model namespace. Hosted 0.2.3 rustdoc omits optional management client
signatures; generate documentation with `--all-features` as described in the
[API reference overview](./index.md).

## Forthcoming 0.2.4 profile models

`ContentTypeConfiguration` and `ContentTypeConfigurations` add typed directional
protections using the existing protection models. Profile create/update keeps
`CreateSecurityProfileRequest`, with current request validation and tolerant
response projections. These additions are not available in 0.2.3. See the
[directional profile guide](/airs/directional-profiles) for exact fields, builders,
versioned schema names, and the request-validation migration.
