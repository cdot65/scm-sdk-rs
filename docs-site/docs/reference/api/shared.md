---
title: Shared services
sidebar_position: 7
slug: /reference/api/shared
---

# Shared services

Shared SCM resources have canonical namespaces outside individual AIRS clients.

| Module | Feature | Entry point |
| --- | --- | --- |
| `scm_sdk::dlp` | `dlp` | `DlpClient` |
| `scm_sdk::shared::iam` | `iam` | `IamScopesClient` |

## Data loss prevention

`DlpClient::new` accepts a configured client. Its resource accessors are
`data_patterns()`, `data_profiles()`, `dictionaries()` and
`data_filtering_profiles()`. AIRS management also provides a DLP convenience
entry point, while these resources retain their canonical `dlp` namespace.

## IAM scopes

`IamScopesClient` owns SCM IAM scope resources. The module also exposes
`generate_workspace_scope_name` and `validate_scope_name` for workspace scope
names. Gateway configuration can access the same canonical resource through
`AIGatewayClient::iam_scopes()`.

Use the endpoint and authentication appropriate to the resource; enabling a
Cargo feature does not grant service permissions.

## Generated reference

- [DLP module](https://docs.rs/scm-sdk/0.2.4/scm_sdk/dlp/index.html)
- [Shared IAM module](https://docs.rs/scm-sdk/0.2.4/scm_sdk/shared/iam/index.html)

Optional client signatures require local documentation with `--all-features`;
see the [API reference overview](./index.md).
