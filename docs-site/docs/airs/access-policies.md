---
title: Access Policies
---

Enable `gateway` and construct `airs::gateway::AccessPoliciesClient` from an
explicitly configured core `Client`. Models are in
`airs::gateway::access_policies::models` and are also re-exported by
`access_policies`. This additive API is introduced in scm-sdk 0.2.1.

The client implements all five operations added by Portkey OpenAPI commit
[`a27feb9`](https://github.com/Portkey-AI/openapi/commit/a27feb9efaac3653d2d94eca5d7d210944b6e160):
create/list/get/update/delete under `/policies/access`. All twelve list filters
are exposed, with explicit numbered pages. Zero page size/page number and false
enabled are preserved; missing filters remain absent. No list_all helper or
snapshot consistency is implied.

Supply a compatible **control-plane endpoint** and a key authorized for policy
administration using `Authentication::runtime_key`. That existing credential
mechanism sends `x-portkey-api-key`; its name does not make this an inference
endpoint. There is no default origin, implicit SCM OAuth adapter, tenant-header
copy or inference-routing inheritance. The caller's destination policy applies.
`AIGatewayClient`'s SCM OAuth resource facade does not expose this key client.

Create requires subjects and targets; update replaces them and requires the
version returned by get. Omitted update fields preserve server state, while
`Patch::Null` clears nullable description. Provider models omission lets the
server apply ALL. Create effect/enabled defaults are also server-owned. No new
response defaults are introduced: both RequestOptions response modes preserve
missing keys, explicit nulls and unknown response data identically.

An allow policy requires a specific subject. Omitted create effect means deny
only for both user ANY and agent ANY; update effect omission keeps its stored
value and never triggers a hidden GET. Unknown returned effects, subject kinds
and target variants are data, never an implicit allow. The SDK does not evaluate
policies or decide whether a caller is authorized.

All writes are non-replayable, including when request options specify retries.
409 conflicts return unchanged for caller reconciliation. Deadlines,
cancellation, request/response limits, redacted ordinary formatting, and
uncertain write outcomes use the shared client guarantees.

## Support and evidence

This family is **experimental and live-unverified**. The retained tenant
registration contains SCM OAuth credentials only, not a separately selected
compatible Portkey control-plane endpoint/key. No policy authorization mutations
were performed. Upstream conformance does not establish deployment in PANW SCM.
The five-method [ledger](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/contracts/vendor/portkey-access/a27feb9/operations.json)
is separate from frozen TypeScript parity and the Red Team vendor denominator.

The [offline example](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/access_policies.rs) executes all
five methods with a synthetic transport. Request/response fixtures are evaluated
independently against the pinned OpenAPI, with explicit reasons for semantic
constraints and tolerant response vocabulary. Wire tests cover all operations,
credential isolation, destination rejection, conflicts, bounds and cancellation.
