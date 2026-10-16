# Prisma AIRS Red Teaming

Enable `red-team` for the native clients; the `models` feature alone exposes the
model contracts. The application supplies SCM OAuth credentials, its runtime,
and explicit endpoints. The frozen service defaults are:

| Plane | Endpoint |
| --- | --- |
| Data | `https://api.apps.paloaltonetworks.com/ai-red-teaming/data-plane` |
| Management | `https://api.apps.paloaltonetworks.com/ai-red-teaming/mgmt-plane` |
| Network broker | `https://api.apps.paloaltonetworks.com/ai-red-teaming/data-plane/network-broker` |

Construct `RedTeamClient::new(data_client, management_endpoint, broker_endpoint)`.
The configured destination policy must already admit every selected endpoint.
Endpoint selection retains one immutable OAuth provider, transport, cancellation
session, concurrency limiter, and resource limits. All Red Team clients require
OAuth bearer authentication and omit `x-tsg-id`, matching the frozen source.
They transmit SDK-owned `service-name: api` and
`User-Agent: PAN-AIRS/<crate-version>-rust-sdk` defaults. Each resource client also
has a public constructor accepting an explicit `Client` for narrower integrations.
No credentials or tenant registrations are loaded implicitly.

## Resource families

| Accessor | Plane | Capabilities |
| --- | --- | --- |
| `scans()` | Data | Submit, list, bounded walks, inspect, abort, categories |
| `reports()` | Data | Static/dynamic reports, attacks, multi-turn outputs, goals, report streams, remediation, runtime policies, download descriptors, partial reports |
| `custom_attack_reports()` | Data | Report summary, prompt sets/prompts, attack outputs, property statistics |
| `targets()` | Management | Target lifecycle, authentication/probes, profiling, metadata and templates |
| `custom_attacks()` | Management | Prompt sets, prompts, properties, CSV upload and templates |
| `instances()` | Management | Instance/device licensing and registry credentials |
| `adapters()` | Management | Custom target adapters and validation |
| `eula()` | Management | EULA inspection and acceptance |
| `network_broker()` | Broker | Network channels and statistics |

The facade also exposes scan statistics, score trends, error logs, tenant
languages on each documented plane, sentiments, quota, dashboard overview,
goal categories, and scan metadata. `get_quota` preserves the published POST;
`get_quota_summary` uses the observed GET. There is no automatic method fallback.

`get_scan_metadata` attempts the static data route first. Only an API HTTP 422
triggers the observed management `targets().get_target_metadata()` fallback.
An issuer failure carrying 422, another HTTP status, transport failure, invalid
response, or local validation error does not trigger it. Both reads share one
absolute deadline and cancellation token. Destination policy remains in effect
on the fallback plane.

`reports().download_report` returns JSON, as the frozen SDK does. It may contain
a URL; the SDK does not follow that URL or forward service credentials to it.
Report stream details are retrospective JSON resources, separate from SSE
inference streams. Unknown response fields and unfamiliar response states remain
available; they do not imply a successful scan or a safe target.

## Pagination and request constraints

Direct list calls omit unspecified `skip`, `limit`, and `search` fields. Each
family adds its precise endpoint filters; explicit `false` values are transmitted.
Skip-based all-page walks default to skip 0 and limit 50, and advance by the
number of received items. A provided total ends the walk when that position
reaches it. Missing/null totals fall back to whether the page is full. An empty
page ends a skip walk even with a stale total, matching the source. Listings are
not snapshots; concurrent changes can affect resumed pages.

Walks are lazy and caller-driven. They share one operation budget and retain
partial rows and resume positions on interruption, failure, or truncation.
`CollectOptions` separately bounds items, bytes, and pages. Its default item cap
is 10,000. Rust reports truncation explicitly and requires finite bounds; the
TypeScript `max: 0` unlimited mode is deliberately unavailable. Native walks can
start at an explicitly supplied skip position. Direct pagination uses nonnegative
safe integer skips and positive 32-bit limits; filters are bounded to 4 KiB and
reject control characters. These are SDK resource bounds, not vendor quotas.

Job, attack, goal, stream, and prompt IDs use the frozen UUID grammar where the
source requires it. Free-form tenant IDs and other resource identifiers are
encoded once as path data; empty, recursively encoded traversal, and malformed
identifiers fail before authentication. Serial-number CSV remains one encoded
query value. Typed request schemas validate known fields, defaults and
cross-field rules before dispatch. Models, filters, requests, clients, and errors
redact ordinary formatting; explicit API data access remains the caller's choice.

## Mutation outcomes and acceptance

Scan submission/abort, report generation, sentiment changes, quota POST,
instance/device writes, registry credential issuance, and the other families'
mutations never infer replay safety. Even a configured retry budget cannot
automatically replay these operations. A transport failure, server 5xx, cancelled
post-send operation, or failed success-body decoding can leave an unknown remote
outcome. Inspect service state before retrying or compensating. An observed 4xx
rejection remains distinct from that uncertainty.

Contract evidence includes 38 exact packaged frozen direct fixtures with original
SHA256 and JSON-pointer attribution, literal instance/device cases, every owned
mutation's no-replay behavior, optional query fields, fallback restrictions,
shared deadlines, input rejection, and bounded resumable scan walks. Sibling
client tests cover management resource and CSV/multipart contracts separately.
`tools/verify_native_fixtures.py` deterministically checks the packaged copies
against the manifest-pinned sources; nine tamper cases check rejected changes.
This evidence is credential-free. Rust live acceptance, tenant permissions,
licensing, provider compatibility, and backend availability remain unverified
until the controlled qualification phase; implementation does not establish
live service support.
