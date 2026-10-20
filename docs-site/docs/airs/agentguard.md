---
title: AgentGuard
---

Enable `agentguard` and construct `airs::agentguard::AgentGuardClient::new(client, data_endpoint, management_endpoint)`. The client must have OAuth authentication. Both endpoints must be admitted by the same destination policy. The constructor shares transport, provider, session, limiter and bounds while deriving `x-tsg-id` exclusively from the provider's immutable tenant. It performs no implicit environment or tenant-file reads. `data_client` and `management_client` expose the scoped clients explicitly.

The frozen experimental browser-derived contracts provide four methods:

| Native method | Plane | Route and query |
| --- | --- | --- |
| `list_scans` | Data | `GET /v1/scans`; optional ordered `skip`, `limit`, `isBackgroundRefresh`, `start_time`, `end_time` |
| `get_scan_stats` | Data | `GET /v1/stats/scans`; `time_period=30_DAYS` by default, the sole supported period |
| `list_scan_vulnerabilities` | Data | `GET /v1/scans/{scan_uuid}/vulnerabilities`; required UUID, no invented filters |
| `list_rules` | Management | `GET /v1/rules`; optional ordered `skip`, `limit` |

Typical PANW bases are `https://api.apps.paloaltonetworks.com/aiag/data` and `https://api.apps.paloaltonetworks.com/aiag/mgmt`; callers select them explicitly. Every method takes `Option<&RequestOptions>` last. Query validation and bounded serialization precede authentication. Pagination counts are safe nonnegative integers, limits are positive, explicit zero and false remain present, timestamps accept explicit offsets, and a start after the end fails locally. Missing pagination fields stay absent. Requests carry exactly OAuth bearer, tenant, service-name and SDK User-Agent headers; GET bodies are empty.

## Result semantics

Typed response models validate required shape, nullable fields and numeric bounds while retaining unknown fields and future vocabulary. A null percentage change means unavailable. Batch scan rows and their children are both returned; aggregating them as independent scans can double count. Findings may contain source code, file paths and other sensitive data. Explicit serialization preserves those fields; ordinary formatting is redacted.

`list_rules` returns one observed catalog page. Its `pagination.total_items` is the current page count, not a catalog total. A catalog default is not proof of effective tenant policy. These four source methods have no exported all-page helper; the SDK does not claim collection completeness from this envelope. Applications may walk explicit bounded offsets until a short/empty page with duplicate protection.

## Qualification and evidence

This surface is implemented and independently tested offline. Browser-derived source evidence is separate from live acceptance; Phase 8 qualification is still required. No AgentGuard mutation or unsupported query filter is invented.

`tests/agentguard_contracts.rs` independently specifies every route, plane, exact ordered query/header set, empty body and typed response. It also exercises omission/default behavior, unknown-field retention, required/null structure, negative counts, source-code formatting redaction, pre-auth validation and resource bounds, destination admission, session cancellation and total deadlines. `contracts/native-operation-mappings/agentguard-oauth.json` records the four exact public owning paths and concrete test bindings.
