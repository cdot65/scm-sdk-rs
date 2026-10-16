# AI Gateway control and public pricing

Enable `gateway` for SCM control clients and the separate public-pricing client. The control facade exposes explicit data/admin endpoints. Standalone `AIGateway*Client::new(Client)` resource constructors use the caller-selected endpoint, require OAuth, and enable `x-tsg-id` from the provider's immutable tenant identity. They retain the transport, pool, session, limiter, destination policy, and limits. No constructor reads credentials or tenant registrations implicitly.

Typical SCM bases are `https://api.apps.paloaltonetworks.com/ai_gw/v2` and `https://api.apps.paloaltonetworks.com/ai_gw/admin/v2`. Explicit additional destinations require admission by the same caller policy. Control requests carry OAuth bearer authentication, `x-tsg-id`, `service-name: api`, and the Rust `PAN-AIRS/<crate-version>-rust-sdk` User-Agent. JSON bodies use `application/json`.

## Implemented control contracts

This partition implements all 79 frozen TypeScript 0.34.0 methods: 78 HTTP operations and the provider-resolution helper. The HTTP operations comprise 34 GET, 13 POST, 20 PUT, and 11 DELETE requests.

| Client | Methods | Plane and contracts |
| --- | --- | --- |
| `AIGatewayApiKeysClient` | 12 | Data: separate `/api-keys/service` and `/api-keys/user` CRUD/rotation |
| `AIGatewayIntegrationsClient` | 12 | Admin: provider integrations, catalog resolution, model/workspace assignments |
| `AIGatewayMcpServersClient` | 12 | Data: MCP servers, connection tests, capabilities, user access and connections |
| `AIGatewayMcpIntegrationsClient` | 10 | Admin: MCP integrations, discovered metadata/capabilities and workspace access |
| `AIGatewayGuardrailsClient` | 9 | Data: workspace guardrails and MCP mappings; capability catalog on explicit admin endpoint |
| `AIGatewayAdminGuardrailsClient` | 8 | Admin: organisation guardrails with distinct input/receipt contracts |
| `AIGatewayUsageLimitsClient` | 7 | Data: usage policies, entities and explicit entity reset |
| `AIGatewaySecretReferencesClient` | 5 | Admin: secret-reference metadata and configuration CRUD |
| `AIGatewayPluginsClient` | 2 | Admin: plugin bindings and credential-bearing creation |
| `AIGatewayAuditLogsClient` | 1 | Admin: audit records over two explicit instants |
| `AIGatewayModelPricingClient` | 1 | Standalone unauthenticated public catalog |

Service/user API-key routes do not fall back to the denied combined collection. Guardrail catalog access requires `with_admin_endpoint` on a standalone data guardrail client; the facade provides that admitted endpoint. Data/admin guardrails never fall back to each other's routes. MCP URLs, integration hosts, secret paths, and returned URLs are data: these clients do not contact those external resources.

## Input and query behavior

Bodies use the frozen native schemas before authentication. PUT requests retain explicit omission/null/value patch states and source strictness. API-key, integration, guardrail and secret-reference updates require at least one source-supported field. MCP-server and usage-policy updates accept `{}` as the source does. API-key rotation accepts `None` for its body and sends validated `{}`; an explicit body preserves the transition interval, whose minimum is 1,800,000 milliseconds. Rotation scheduling, usage-limit reset choices, and secret-reference manager/workspace constraints retain their native cross-field checks.

Data guardrail creation uses numeric SCM organisation identifiers and requires a workspace UUID. Admin creation permits an organisation UUID and optional workspace. Their mapping requests also differ: data mapping phases/capability IDs remain source string fields; admin phases are `input`/`output`, and capability IDs/map keys are UUIDs. Shared response mapping records preserve unfamiliar future phase strings.

Generated query models retain the source field names through serde, while native fields use snake_case (`type_` where necessary). Admin guardrail list input uses the frozen `workspaceId/pageSize/currentPage` schema and sends `workspace_id/page_size/current_page`. Query validation preserves declaration order, explicit false/zero/empty strings, and repeated array values. It omits missing/null values. Serialization, raw query bytes, entries, final encoded URLs, and the operation deadline are bounded before authentication. Unlisted fields on strict query types fail locally.

Lists return their declared page/envelope without adding pagination defaults or silently collecting more pages. This partition has no exported all-page helper in the baseline. Relevant dialects are:

| Resource | Pagination keys | Source bounds |
| --- | --- | --- |
| MCP server list | `current_page`, `page_size` | zero-index page; size 1–100 |
| MCP capabilities/user access | `page`, `page_size` | one-index page; size 1–500 |
| MCP connections | `current_page`, `page_size` | zero-index page; size 1–500 |
| Admin guardrails | `current_page`, `page_size` | zero-index page; size 1–1000 |
| Usage policy list | `current_page`, `page_size` | both nonnegative; **size zero is valid** |
| Usage policy entities | `current_page`, `page_size` | zero-index page; size 1–100 |
| Secret references | `current_page`, `page_size` | zero-index page; size 1–100 |

Resource identifiers follow their actual contracts. Key/integration/guardrail paths require UUIDs, while MCP servers, usage policies/entities, and secret references accept opaque identifiers of 1–512 JavaScript UTF-16 units, encoded once as independent segments. `integrations.delete_models` accepts an integration UUID or slug and a comma-joined `slugs` query; individual model slugs exclude commas and retain source length checks. Native path/query limits also reject traversal and excessive allocations.

`resolve_provider_id` trims JavaScript whitespace, returns a lowercased UUID locally without authentication, otherwise selects the first case-insensitive catalog slug match. Unknown references produce a sanitized error without echoing the reference or provider data. References and deletion queries have explicit allocation bounds.

## Response and mutation behavior

Typed responses validate required structure while preserving unknown fields and open response vocabulary. The source hydrates empty successful JSON bodies as `{}`: all-optional responses accept that value, while missing required structures fail. The six void deletes accept any successful body without JSON decoding. Creation/rotation receipts preserve one-time key material; ordinary model/client/query formatting is redacted.

MCP integration workspace access returns `McpIntegrationWorkspacesResponse`, a typed ordered union of the list and legacy envelopes. Both source branches have overlapping optional fields. The first matching branch wins, exactly as in the frozen Zod union; a legacy-only `workspaces` field can therefore reside in the list variant's preserved extensions. A second-branch-only match remains the `Legacy` variant. Both schemas are checked against one HTTP response within one validation budget; a failed branch never causes another API dispatch. Missing and empty-string `version` options remain distinct.

All 44 mutation methods have no automatic replay. Ambiguous transport and post-send decoding failures retain `RemoteOutcome::Unknown`, including successful HTTP status metadata where available. Reconcile remote state before retrying creation, rotation, replacement, deletion, testing an MCP connection, or resetting usage. Existing traffic counters are not qualification fixtures. Per-method live and experimental labels are tracked separately from implementation and offline contract coverage.

Secret-reference clients manage metadata/authentication configuration and never resolve a secret from AWS, Azure or Vault. Audit responses retain the service's unredacted `request_body` strings faithfully. Applications should deliberately select safe fields when displaying these results; SDK formatting excludes their contents. Required audit `NativeDate` instants serialize to exact JavaScript `toISOString` UTC milliseconds, including extended years across the full supported date range. The SDK does not invent a window or reorder its endpoints.

## Isolated public pricing

`AIGatewayModelPricingClient::new` accepts only a credential-free `Client` with an explicit endpoint and caller-owned destination policy/transport. OAuth and runtime credentials are rejected. Requests send `Accept: application/json`; they never send bearer, runtime-key, or tenant headers, and redirects are rejected. There is no implicit provider, model, or catalog endpoint.

Pricing independently defaults to a 60-second total operation deadline and zero read retries. The budget covers all attempts instead of resetting per attempt as in the baseline. `with_options(AIGatewayModelPricingClientOptions)` supplies explicit defaults; per-call `RequestOptions` overrides the deadline/cancellation and may explicitly override read retries. Retry counts stay within 0–5. The shared native request deadline cap is 24 hours, a deliberate tighter bound than the baseline Node timer maximum. Caller-configured transport/TLS and payload/validation limits remain active.

Returned prices remain upstream catalog units (USD cents where declared), not tenant billing or Prisma cost telemetry. Calculation trees remain bounded, unevaluated data; no formula evaluation, conversion, caching, or price inference occurs. Unknown response fields are retained and known nested structure remains validated.

## Evidence

`crates/scm-sdk/tests/gateway_control_contracts.rs` independently specifies literal wire routes, planes, ordered queries, headers, body defaults, and schema-grounded responses for every method. Thirty control/pricing cases from the frozen OpenAPI corpus are executed with complete exact wire, response, and status assertions. An exhaustive 42-case consumption guard also binds all 12 sibling log-export/rate-limit cases to their separate execution suite. Methods outside that corpus use manually authored source-derived fixtures. Additional sweeps exercise all 44 uncertain transport mutations and all 38 typed mutation decoding failures without replay, strict pre-authentication rejection, UTF-16 identifiers, explicit patch states, overlapping union behavior, secret redaction, and public pricing retry/deadline/cancellation/redirect isolation.

`contracts/native-operation-mappings/gateway-control.json` binds all 79 baseline methods to concrete native source and named tests. These checks do not assert live acceptance. Live qualification remains Phase 8. Gateway runtime/telemetry and the Phase 7 realtime/provisioning/export-closure work have separate ownership records.
