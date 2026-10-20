---
title: Model Security
sidebar_position: 4
slug: /reference/api/model-security
---

# Model Security

**Module:** `scm_sdk::airs::model_security` · **Feature:** `model-security`

`ModelSecurityClient::new(client, data, management)` accepts explicit data and
management `Endpoint` values. Both planes use the configured OAuth client, with
resource clients routed to their appropriate plane.

| Accessor | Selected operations |
| --- | --- |
| `scans()` | `create`, `list`, `pages`, `list_all`, `get`, `get_evaluations`, `get_files`, `get_violations` |
| `models()` | Model inventory operations |
| `security_groups()` | Group creation, listing, retrieval, update and deletion; rule instance operations |
| `security_rules()` | Security rule inventory |
| `custom_rules()` | Custom rules and their versions |

Scan resources also expose `add_labels`, `set_labels`, `delete_labels`,
`get_label_keys`, `get_label_values`, `get_evaluation` and `get_violation`.
`get_pypi_auth` belongs to the top-level client.

`ScanCursor` retains the scan collection's skip position and count-capped state.
Use the typed query options and collection status rather than assuming that
all service collections have the same cursor or total-count behavior.

## Models and signatures

See the [Model Security module](https://docs.rs/scm-sdk/0.2.3/scm_sdk/airs/model_security/index.html)
for its model namespace. For optional client signatures, generate local rustdoc
with `--all-features` using the [reference instructions](./index.md).

Request-side identifier checks still apply. Relaxed response string formats do
not make an arbitrary caller-supplied path ID valid.
