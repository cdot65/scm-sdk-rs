---
title: Red Teaming
sidebar_position: 5
slug: /reference/api/red-teaming
---

# Red Teaming

**Module:** `scm_sdk::airs::red_team` · **Feature:** `red-team`

`RedTeamClient` separates data, management and network-broker endpoints.
Its resource accessors include `scans()`, `reports()`, `targets()`,
`custom_attacks()`, `custom_attack_reports()`, `adapters()`, `instances()`,
`eula()` and `network_broker()`.

| Resource | Selected methods |
| --- | --- |
| Scans | `create`, `list`, `get`, `abort`, `set_runtime_profile`, `list_pages`, `list_items`, `list_all` |
| Targets | `create`, `list`, `get`, `update`, `delete`, `probe`, `get_profile`, `update_profile`, `start_profiling` |
| Reports | `list_attacks`, `get_attack_detail`, `get_static_report`, `get_dynamic_report`, `get_static_asr`, `get_dynamic_asr` |
| Report workflows | `regenerate`, `get_status`, `override_attack`, `override_multi_turn_attack`, `override_stream`, `get_download_url` |

A returned download location is API data. Applications decide whether to follow
it; receiving a signed URL does not automatically authorize a storage request.

## Target filtering in 0.2.3

`TargetListOptions::profiling_status` accepts `NOT_STARTED`, `IN_PROGRESS`,
`COMPLETED` and `FAILED`, including across pagination helpers. Unsupported values
fail before authentication or transport. Legacy `status` values are still sent
unchanged. Explicitly move supported profiling states to `profiling_status`;
`QUEUED` and `PARTIALLY_COMPLETE` have no declared replacement filter.

`get_target_profile_error_logs` accepts only `limit`. `skip` and `search` are
rejected rather than silently dropped.

## Preserve server omission and null

Set `RequestOptions::without_response_defaults()` when a typed response must
retain absent fields without inserting schema defaults. Explicit null remains
null. Without this option, default insertion remains enabled. Unknown response
enum strings and object keys are retained; request validation remains strict.

Vertex AI, N8N, client-session, audio and realtime provider workflows have
implementation and offline coverage without universal live qualification. The
0.2.3 tenant filter probes returned HTTP 403; they do not establish filtering
effectiveness for your tenant.

## Models and signatures

See the [Red Team module](https://docs.rs/scm-sdk/0.2.3/scm_sdk/airs/red_team/index.html).
`models::data_plane` provides current vendor dataplane DTOs alongside compatible
existing models. Optional client signatures require local rustdoc with
`--all-features`; see the [reference overview](./index.md).
