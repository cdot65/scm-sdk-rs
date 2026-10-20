---
title: Identity and Access
---

`shared::iam::IamScopesClient` owns the canonical SCM scope API. Enable `iam`
and configure the client's endpoint explicitly, including `/iam/v1`. The frozen
default service is `https://api.apps.paloaltonetworks.com/iam/v1`; the SDK does
not infer another origin from an AIRS endpoint. An endpoint selected with
`Client::with_endpoint` must already be admitted by the caller's policy.

The constructor requires SCM OAuth and derives `x-tsg-id` from the provider's
immutable tenant identity. It retains that provider, the destination policy,
transport, cancellation session, and concurrency limiter. No caller header can
replace tenant or authentication fields. IAM requires a tenant administrator;
workspace-scoped authorization does not establish access to these resources.

| Method | Contract |
| --- | --- |
| `list` | `GET /scopes`; return observed `count` and `items`, including unknown response fields |
| `get` | `GET /scopes/{name}`; the display ID `name:tsg` is not a path key |
| `create` | `POST /scopes`; omitted description/resources become `""`/`[]` |
| `update` | `PUT /scopes/{name}`; full replacement, with the path name repeated in the body |
| `bind_workspace` | Read, preserve existing bindings, append one missing workspace binding, then replace |
| `delete` | Experimental `DELETE /scopes/{name}` inferred by the frozen SDK; live support unverified |

Names used as path keys follow the frozen grammar
`[A-Za-z0-9_][A-Za-z0-9_.-]*`, within the shared identifier bound. Create retains
the frozen request schema's nonempty body-name rule. Resource metadata omitted
on writes becomes `[]`; provided metadata is preserved. Create-input extension
fields are ignored because the frozen helper constructs an explicit write body.
Resource-input extensions are subject to the source's strict write schema.

`update` clears resources when they are omitted. `bind_workspace` preserves
known resource bindings and metadata and does not append an identical workspace
binding twice. As in the frozen helper, it still sends the PUT when the binding
already exists. Unrecognized response resource fields are not copied into the
strict write schema. The workspace slug is validated before the initial GET;
this deliberately prevents a failed-input workflow from starting network work.

Both binding steps share one absolute deadline and cancellation token. Reads
permit bounded safe retries; POST, PUT, and DELETE do not automatically replay,
including a PUT whose intended result is idempotent. Cancellation or failure
after dispatch can leave the remote mutation outcome unknown. Inspect server
state before deciding whether to repeat a write. Binding is a read-modify-write
workflow without an upstream transaction or conditional write; concurrent
writers can overwrite each other's changes. It provides no snapshot guarantee.

The frozen IAM models derive from dated observed traffic, rather than a
published OpenAPI specification. Independent credential-free tests cover all
six methods, exact paths/headers/bodies, defaults, additive binding, deadlines,
cancellation, validation before authentication, sanitized failures, and mutation
replay prevention. Live acceptance has not been performed for this Rust SDK.
DELETE retains the source's explicit experimental status: 404/405 is a service
rejection, and automatic rollback must not assume deletion succeeded.

## Management token administration

`airs::management::OAuthManagementClient` administers application tokens using
the management endpoint and SCM OAuth bearer authentication. It omits `x-tsg-id`
as the frozen management client does, while keeping the same provider identity
and shared resources. These operations do not alter the service-token cache.

`get_access_token` sends the generated `ClientIdAndCustomerApp` body to
`POST /v1/mgmt/oauth/client_credential/accesstoken`. `GetTokenOptions` optionally
adds `tokenTtlInterval` and `tokenTtlUnit` query values. Intervals use finite
numbers with JavaScript-compatible decimal formatting; units and permitted
ranges remain service-defined. The generated `Oauth2Token` result preserves
unknown response fields and redacts ordinary formatting. Its token data is
available through explicit field access.

`invalidate_token` takes `&Secret`, sends the same body to
`POST /v1/mgmt/oauth/invalidateToken`, and encodes the token as query data. The
response is a JSON string confirmation. Arbitrary text or another JSON type
produces a sanitized decode error. Both POST operations prohibit automatic
replay after an uncertain failure. Models, options, requests, clients, and errors
omit token values from ordinary formatting; callers own any explicit API data
they choose to log.
