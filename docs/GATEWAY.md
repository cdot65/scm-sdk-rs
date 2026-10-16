# Prisma AIRS AI Gateway

Enable `gateway` for native clients, or `models` for model contracts alone.
Gateway SCM management, runtime inference, and public pricing use separate
credential planes. Applications own the async runtime and select all credentials,
transports, and endpoints explicitly.

| Plane | Frozen default | Authentication |
| --- | --- | --- |
| SCM data | `https://api.apps.paloaltonetworks.com/ai_gw/v2` | OAuth bearer and `x-tsg-id` |
| SCM admin | `https://api.apps.paloaltonetworks.com/ai_gw/admin/v2` | OAuth bearer and `x-tsg-id` |
| IAM | `https://api.apps.paloaltonetworks.com/iam/v1` | Shared OAuth bearer and `x-tsg-id` |
| Runtime | Explicit deployment endpoint | Separately supplied gateway key |
| Public pricing | Explicit catalog endpoint | Public client without credentials |

`AIGatewayClient::new(data_client, admin_endpoint, iam_endpoint)` validates
destination admission before deriving its clients. The scoped clients share one
immutable OAuth provider, transport, cancellation session, limiter, and resource
limits. `GatewayClient` is an alias. `with_runtime(runtime_client)` attaches an
independently constructed `RuntimeClient`; `inference()` and
`runtime_resources()` return that client. Access before attachment returns a
configuration error. Management OAuth is never copied into runtime or pricing.

The facade exposes telemetry, workspaces, canonical IAM scopes, configs,
guardrails/admin guardrails, providers, API keys, integrations/MCP integrations,
MCP servers, deployments, plugins, organisations, audit logs, usage/rate limits,
log exports, and secret references. Each leaf also has a public constructor for
narrower integrations. Public pricing remains a standalone client.

## Telemetry contracts

Telemetry methods preserve the SCM API's specific envelopes, field names, and
units. Spend and savings remain **cents**; latency remains milliseconds. Empty
cohort mean/percentiles may be null. Grouped error buckets and rescued retry
values can contain arrays; feedback model values contain objects. Unknown fields
remain available without implying a successful operation or known vocabulary.

`AIGatewayWindowOptions` carries a workspace slug and optional days/start/end.
An explicit start overrides days; the default rolling window is seven days back
from the selected end, whose default is one sampled current instant. Start must
not follow end. `NativeDate` provides the JavaScript epoch-millisecond range.
Telemetry validates calendar years 0 through 9999 and transmits numeric offsets
such as `+00:00`, with second precision. Its default is explicit UTC. Callers can
set `offset_minutes`; no ambient machine timezone changes the wire contract.
Audit log adapters use UTC ISO strings with three millisecond digits and signed
extended years across the full JavaScript Date range.

`AIGatewayChartOptions` composes a window and generated `AIGatewayChartFilters`.
The verified filters are trace ID, string-valued metadata, nonempty status/API
key/provider-model lists, and inclusive total-token/cost bounds. Four charts
(`cost`, `requests`, `tokens`, `latency`), user grouping, and grouped analytics
accept these filters. Other charts take window options. Group columns/dimensions
are typed source vocabularies; their list order and duplicates are retained.
CSV filters preserve list order, zero values, and fractional cents. Metadata JSON
uses deterministic key order. Unverified upstream analytics filters are rejected.
These adapters implement the frozen SDK's partial SCM contract; they do not claim
the full legacy upstream analytics surface.

`logs()` uses zero-based `currentPage` and `pageSize`, with omitted fields left
to server defaults. Zero values are transmitted. It returns one page with
`total` and `capturedTotal` retained separately. Use a fixed window when paging;
the service does not promise a snapshot. Filter boundaries omit `organisationId`
on their captured `/analytics/filter-boundaries` route. Chart route spelling is
explicit, including `cache-hit-trend` and plural `user-trends`.

## Resource writes, queries, and routing

Workspace reads explicitly choose the data or admin plane. Omitting status
retains the service's active-only behavior; archived rows require the archived
filter. Workspace writes use the admin plane, and deletion archives the record.
No authorization error triggers an automatic plane fallback. Direct workspace
creation requires an existing IAM scope. The Phase 7 `provision` workflow creates
or reuses that scope, creates the workspace and adds its IAM binding. Structured
recovery preserves resources after uncertain writes; see [provisioning](PROVISIONING.md).
This workflow remains excluded from the historical Phase 6 completion denominator.

Config/provider/deployment creation returns its source receipt rather than an
invented detail record. Config detail retains its JSON-encoded config string.
Updates preserve each source request schema, including omitted/null/value states,
full replacements where required, and nonempty partial patches. A successful
empty update body hydrates the source-compatible empty object. Deletes that
return no value remain distinct from object receipts.

Typed query adapters validate the frozen schema before authentication and retain
schema key order, repeated array order, false, zero, and empty string values.
Query construction has the same absolute `RequestOptions` budget as transport.
SDK limits bound initial serialization, cumulative query bytes, entry count, and
the final encoded URL. Source UUID, numeric ID, workspace reference, and opaque
resource identifier grammars apply before dispatch. Opaque resource IDs use the
source 512 UTF-16 unit limit and are encoded once as path data. Recursive traversal
and invalid framing are rejected locally. These are finite SDK resource bounds,
not inferred vendor quotas.

Every SCM mutation is single-dispatch even when the caller selects retries. Transport
failure, unsafe server 5xx, post-send cancellation, or failed success decoding can
leave an unknown remote outcome. A 4xx rejection remains distinct. Reconcile
service state before manually retrying. Ordinary formatting omits secrets, bodies,
metadata, signed URLs, and configuration values.

Log export downloads return a signed URL descriptor; the SDK does not follow that
URL or forward service credentials to it. Export start/cancel/download retain the
frozen source's experimental status because successful live execution was not
established. Rust live acceptance for all methods remains unverified until the
controlled qualification phase.

## Independent evidence

The 61-method control/telemetry suite asserts exact plane, path, query order,
headers, request bodies, and typed responses. Twelve packaged cases are exact
copies of manifest-pinned independent fixtures. The Rust suite includes that
packaged bundle, executes every case against the typed log-export/rate-limit
clients, compares exact routes, queries, bodies and responses, and rejects unused
cases. `tools/verify_native_fixtures.py` checks their exact frozen provenance.
The broader 61-method table contains literal source-derived specimens.
`tools/verify-gateway-specimens.mts --check` independently executes the frozen
Zod schemas for 74 request/query/response parses and requires exact normalized
output. Its reproducible proof in `contracts/native-fixtures/gateway-specimens.json`
binds the baseline, schema declarations, tooling inputs and Rust test source by
SHA256. Unit-returning methods explicitly record the absence of a response schema;
no Rust validator result supplies expected data.
Additional cases cover all 21 mutations under both transport and server ambiguity,
numeric offsets, filter CSV and cents, null/unknown response data, invalid input
before authentication, policy isolation, cancellation/deadlines, and safe errors.
Literal date vectors cover the complete JavaScript Date range. Runtime and sibling
control clients maintain their own independent contract suites.
