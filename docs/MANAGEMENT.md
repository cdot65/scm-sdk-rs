# AIRS management

Enable the `management` Cargo feature and supply a `Client` authenticated with SCM OAuth. `ManagementClient::new` and every standalone subclient constructor reject another credential kind before authentication or network work. The immutable OAuth context supplies the tenant for legacy TSG routes. No management constructor reads environment variables or tenant files implicitly.

The facade exposes `profiles`, `topics`, `api_keys`, `customer_apps`, `deployment_profiles`, legacy `dlp_profiles`, legacy `scan_logs`, `dashboard`, and token administration through `oauth`. This milestone implements the 56 non-DLP resource/dashboard methods in the frozen TypeScript 0.34.0 baseline. Token administration and canonical DLP have their own implementation and contract records.

## Endpoints and headers

Pass the AIRS management base explicitly, commonly `https://api.apps.paloaltonetworks.com/aisec`. `with_dashboard_endpoint` changes only dashboard reads. It preserves the provider, transport, session, limiter, and destination policy; an unadmitted destination fails locally. Dashboard always sends `x-tsg-id` taken from its OAuth context, including when the original client's optional tenant header was disabled.

Canonical `ManagementClient::dlp(endpoint)` requires the separate DLP endpoint explicitly. The `management` feature always enables `dlp`; IAM remains independently selected with `iam`. It reuses the caller's admitted destination policy and OAuth identity; it never treats the management base as the DLP service. The legacy `dlp_profiles().list` remains `/v1/mgmt/dlpprofiles` on the management endpoint.

All API requests, including injected transports, carry `service-name: api`, `User-Agent: PAN-AIRS/<crate-version>-rust-sdk`, and OAuth bearer authentication. The Rust User-Agent is a documented language/version difference from the baseline. JSON bodies carry `application/json`. Dashboard adds its tenant header; other management methods retain the caller's OAuth tenant-header setting. Credentials, queries, bodies, cursor values, and response contents are excluded from ordinary formatting.

## Lists and lookups

`list_for_token` uses the OpenAPI token-scoped routes. `list` retains the frozen TSG routes, including the singular customer-app route `/v1/mgmt/customerapp/tsg/<tenant>`. `PaginationOptions` keeps offset 0, limit 100, and optional `latest`; only profile lists send `latest`. Topic `latest_only` is a client-side workflow: it walks all revisions with page size 200, selects the largest revision per name, preserves first-seen name order and first revision ties, then applies the requested local page.

Each inventory provides caller-driven `pages` and `list_all`. The latter returns `CollectionResult<T,u64>` with explicit completion, partial items, and a resumable page/item position. Default collection bounds are 10,000 items, 64 MiB of serialized collection data, and 1,000 pages. Override these through the client's list-all options. The shared pagination module detects repeated cursors and nonterminal empty pages. All requests in a walk share one deadline and cancellation snapshot. Collections are not server snapshots; resumed pages may have changed.

Profile/topic lookups preserve the frozen all-page behavior. Missing resources produce a sanitized validation error only after a complete walk; truncated or interrupted walks produce a limit/pagination/underlying request error. `get_by_name` selects the highest revision and preserves the first result on ties. Topic latest selection also rejects an incomplete walk. Returning partial data as a complete lookup would be incorrect.

A missing or zero `next_offset` ends the source's offset walk. Fractional, negative, or unsafe-integer cursors are rejected instead of issuing an ambiguous next request. Customer-app collections expose the actual `CustomerAppWithKeys` response model, preserving key/deployment associations rather than narrowing to the source's structurally compatible `CustomerApp` annotation.

## Dashboard query contracts

The typed/raw pairs share the same validated request construction and immutable tenant header. They retain the source's query dialects:

| Operation | Identity keys | Default time/page |
| --- | --- | --- |
| Application and violation breakdown | `appid`, `appname` | 30 `days`; allowed windows 7/30/60 |
| Top applications, violation trend, sessions chart | none | 1 `day` |
| Apps list | none | 30 `days` |
| Sessions overview | none | 1 `day`, limit 25, offset 0 |
| Session | `session_id`, `app_id`, `app_name` | 30 `days`, limit 25, offset 0 |
| Session transaction | session/app keys plus `scan_id`, `scan_sub_req_id` | 30 `days` |
| Scan content | `scan_id`, `scan_sub_req_id` | no time window |
| Applications overview | none | 30 `days`, limit 25, offset 0 |

The response's `sub_scan_req_id` intentionally differs from the scan-content query's `scan_sub_req_id`. Unknown response fields and security vocabulary remain faithful data. Typed methods require the declared response structure. The frozen request engine hydrates empty JSON successes as `{}`; roots with only optional fields or explicit defaults accept that object, while missing required arrays/fields remain errors. Raw methods return `Option<JsonValue>`: `None` means an empty body, while `Some(JsonValue::new(Value::Null))` is an explicit JSON null. This preserves availability evidence rather than inventing an empty successful inventory.

All frozen dashboard Zod input schemas are validated natively before authentication. Query serialization is bounded to the shared 16 KiB URL budget before allocating a parsed JSON copy; final URL admission also counts percent encoding and the endpoint. Applications-overview inputs additionally validate the source's declared windows/time units, positive safe-integer limits, and nonnegative safe offsets; the baseline's implementation forwards unvalidated runtime casts for this one route. Query/model formatting is redacted.

## Mutations and legacy logs

Create/update requests use the frozen native schemas before requesting tokens. Profile/topic update and delete paths retain UUID checks; topic force deletion requires a nonblank audit identity. The legacy `force_delete` topic wrapper still fails when its optional identity is absent. Successful string-shaped deletion bodies normalize to the source's `{ "message": ... }` object contract.

Every mutation, including legacy scan-log POST retrieval, has no automatic replay. An ambiguous transport or post-send decode failure retains `RemoteOutcome::Unknown`; local cancellation cannot establish whether the server committed a write. Reconcile remote state before retrying or deleting resources.

Legacy scan logs preserve mixed-case `pageNumber`/`pageSize`, filter/time query keys, and a JSON `{ "page_token": ... }` body only for a nonempty token. The typed Rust options additionally require positive safe-integer interval/page values. This route remains legacy and upstream-unqualified: HTTP 200 with an empty body hydrates an empty optional response model for compatibility; it is unavailable evidence and must not be interpreted as zero activity. Prefer dashboard session APIs for new retrieval workflows.

## Evidence

`crates/scm-sdk/tests/management_contracts.rs` contains independently authored literal routes, queries, headers, request bodies, and response shapes from frozen `src/management/*.ts` and `src/models/mgmt-*.ts`. Every one of the 56 methods has a concrete fixture invocation, including the 22 dashboard typed/raw methods. Further tests cover helpers, pre-authentication rejection, truncation and incomplete lookup handling, endpoint isolation, query redaction, and no-replay uncertainty for all 15 mutation entry points.

`contracts/native-operation-mappings/management.json` maps each baseline method to its native source and named test. Implementation, offline contract checks, and live acceptance are separate records. No live service success is asserted by these offline fixtures.
