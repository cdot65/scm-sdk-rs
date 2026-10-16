# Red Team resources

Enable `red-team` for the native resource clients. Typed models are available under `airs::red_team::models` with `models`. These clients implement the 54 frozen methods for custom attacks, targets, adapters, Network Broker channels, and EULA. Offline verification and live service acceptance are tracked separately; live acceptance is currently unverified.

Use the `RedTeamClient` facade or construct a resource client with `new(Client)`. Each constructor requires OAuth and omits `x-tsg-id`, matching the frozen Red Team authentication contract. The facade selects the explicit management endpoint for custom attacks, targets, adapters and EULA, and the separate broker endpoint for channels. Clones retain the caller's token provider, session, transport, destination admission, limits and shared concurrency bound. Construction performs no API call or EULA acceptance.

## Resource surface

| Client/accessor | Frozen methods mapped to Rust |
| --- | --- |
| `RedTeamCustomAttacksClient` / `custom_attacks()` | `create_prompt_set`, `list_prompt_sets`, `list_all_prompt_sets`, `get_prompt_set`, `update_prompt_set`, `archive_prompt_set`, `get_prompt_set_reference`, `get_prompt_set_version_info`, `list_active_prompt_sets`, `download_template`, `upload_prompts_csv`, `create_prompt`, `list_prompts`, `list_all_prompts`, `get_prompt`, `update_prompt`, `delete_prompt`, `get_property_names`, `create_property_name`, `get_property_values`, `get_property_values_multiple`, `create_property_value` |
| `RedTeamTargetsClient` / `targets()` | `start_profiling`, `get_copilot_auth_url`, `exchange_copilot_token`, `delete_copilot_token`, `create`, `list`, `list_all`, `get`, `update`, `delete`, `probe`, `get_profile`, `update_profile`, `validate_auth`, `get_target_metadata`, `get_target_templates` |
| `RedTeamAdaptersClient` / `adapters()` | `get_config`, `create`, `list`, `list_all`, `get`, `update`, `delete`, `validate` |
| `RedTeamNetworkBrokerClient` / `network_broker()` | `list_channels`, `create_channel`, `get_channel_stats`, `get_channel`, `update_channel` |
| `RedTeamEulaClient` / `eula()` | `get_content`, `get_status`, `accept` |

Every method accepts optional `RequestOptions` for a shared absolute deadline, cancellation and safe-read retry selection. JSON input normalization, UUID validation, filter bounds and multipart checks occur before credential lookup. Additional response fields and unfamiliar statuses remain available; typed-model `Debug` and errors conceal payload values, including Copilot secrets, scripts, variables, tokens, URLs and extension data.

`get_target_metadata` returns `data::JsonObject`, preserving arbitrary metadata while rejecting a non-object response. Its inline TypeScript `z.record(z.unknown())` contract is structurally identical to the frozen module sub-schema `s012087`: `TargetCreateRequestSchema.extra_info`, beneath optional/nullable wrappers. That record has an unconstrained string key and unknown value, without transformations or refinements. The client uses this verified SID through the bounded native validator, rather than assigning a nonexistent public schema name.

## Wire defaults and request states

Direct list methods omit `skip`, `limit`, and `search` when unset. Explicit booleans, including `false`, are transmitted. Query parameters retain frozen order: shared `skip`, `limit`, `search`, followed by the endpoint's filters. Channel `status` and multiple `property_names` values remain repeated query keys in caller order, including duplicates. An empty vector emits no such query key.

Adapter `create` and `update` always send `validate=true` unless `AdapterOperationOptions.validate` explicitly supplies `false`. This follows the frozen implementation; some upstream comments describe a different default. Target `create` and `update` omit `validate` by default, and transmit either explicitly supplied boolean.

Target and adapter updates use PUT. Adapter updates replace the complete definition: required `name`, `script_b64` and `prompt` remain required; an omitted variable key deletes it, while `value: null` preserves an existing stored secret. Prompt-set and prompt updates preserve omission/null/value through the generated `Patch` fields. Channel updates use ordinary `application/json` PATCH, matching their own endpoint contract.

Prompt, target and adapter deletes return `Option<BaseResponse>` because both a response object and an empty successful body are valid. Property-name creation similarly returns `Option<PropertyNameCreateResponse>` and accepts both the property-name list response and legacy message/status response. Copilot token deletion returns `()`.

Mutations, profiling, auth validation and adapter execution are dispatched once. A transport failure after dispatch returns `RemoteOutcome::Unknown`, including when a retry override is set to five. Applications reconcile state before repeating a mutation. The SDK sends adapter scripts and target connection settings to the configured service; it does not execute scripts or contact a configured target directly.

## CSV upload and download

`upload_prompts_csv(uuid, &BinaryFile, options)` sends exactly one multipart part named `file` with filename `prompts.csv`, regardless of the file's filename hint. The supplied MIME type and all bytes are preserved, including non-UTF-8 bytes; an absent MIME type becomes `application/octet-stream`. The prompt-set UUID is the `prompt_set_uuid` query parameter.

`BinaryFile` owns caller bytes without a construction size ceiling. This operation validates the file against `max_request_bytes`, then bounds the complete multipart envelope, including headers and delimiters. Multipart boundaries are generated locally and checked against part headers and bytes. The operation's single deadline and cancellation token covers validation, encoding, authentication and dispatch. An already-cancelled or expired budget is rejected before encoding or credential lookup.

`download_template` uses the same authenticated transport and bounded response lifecycle, returning raw text without requiring a specific Content-Type. Decoding follows Fetch text behavior: UTF-8 replacement for malformed bytes and removal of a leading UTF-8 BOM. It does not JSON-decode CSV or inherit a separate download transport.

## Bounded collections

The four frozen collection methods use skip/limit pagination with default skip zero and limit 50. Continuation advances by the actual item count. A present total determines whether more records exist; an absent or null total continues only for a full page. An empty page ends the walk even when a stale total says otherwise. Response order and duplicate rows are retained.

`list_all(..., CollectOptions, options)` returns `CollectionResult` with explicit completion, item/byte/page caps, partial failures and a resume position. The defaults cap 10,000 items, 64 MiB of serialized item-array bytes and 1,000 pages. Zero does not disable a bound. A starting skip is a native extension.

Targets and adapters also expose `list_pages` and `list_items`. Custom attacks exposes `prompt_set_pages` / `prompt_set_items` and `prompt_pages` / `prompt_items`. Resume an item iterator with `.resume_from(saved_resume)`, retaining the original filters and page size. One absolute budget covers all pages and buffered items. Listings are not snapshots; an upstream change between reads can move rows, so deduplication policy belongs to the application.

These page/item helpers consistently take `(params, max_pages, request_options)`, with a prompt-set UUID before the parameters when needed. Request lifecycle options are last across the product clients; prerelease review standardized the DLP and resource helpers to this order.

## Deliberate Rust adaptations and evidence

- Explicit clients, endpoints and caller-owned OAuth replace implicit environment/global configuration. The Red Team tenant-header contract remains omission.
- Direct query inputs use nonnegative safe-integer skips, positive `u32` limits, bounded/control-free filters and the shared query-count/URL limits. These reject values that frozen direct lists forwarded without checking. Present totals needed for continuation must be nonnegative safe integers.
- Explicit collection caps and resumable partial outcomes replace a bare array and the TypeScript `max=0` unlimited mode. A caller-selected starting skip is supported.
- Free-form property names use one encoded path segment, with traversal and control checks. Repeated query values retain order.
- Native multipart encoding supplies its own boundary-bearing Content-Type and operational size bounds. Part name, forced filename, MIME and byte semantics match the frozen FormData contract.
- The inline metadata schema uses a structurally verified frozen SID as described above. Request options and uncertain mutation outcomes follow the common Rust lifecycle contract.

`contracts/native-operation-mappings/red-team-resources.json` records every frozen method, native method, implementation file, actual test references and adaptations. `tests/red_team_resources_contracts.rs` checks exact routes, ordered queries, header sets and JSON bytes; all 26 mutation uncertainty paths; request states; CSV envelope and text behavior; pagination, caps, resume and shared budgets; unknown responses; invalid input before credentials; cancellation; bounds and secret-free formatting. Passing offline fixtures establish the native contract implementation, not availability, permissions or successful execution in a tenant.
