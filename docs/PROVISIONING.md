# Gateway workspace provisioning and recovery

`AIGatewayWorkspacesClient::provision(request, provision_options, request_options)`
implements the frozen SCM UI sequence: create an IAM scope, create an admin-plane
workspace referencing that scope, then read and update IAM to add its workspace
binding. Existing resources and the scope description survive that final update.
The upstream IAM API supplies no transaction or conditional-write version; concurrent
IAM writers can overwrite one another.

The Gateway facade supplies its shared IAM client automatically. A directly
constructed workspace client requires `with_iam_endpoint(admitted_endpoint)`.
All scoped clients retain the same OAuth identity, destination policy, transport,
cancellation session and concurrency limiter. Complete workspace and scope inputs
are validated before authentication or the first mutation.

`GatewayWorkspaceProvisionRequest` has the workspace creation fields and an optional
`scope_name`. Omission generates `ws_<normalized-name>_<six random characters>`
using fallible operating-system entropy and unbiased alphabet selection. An explicit
name must pass IAM scope-name validation. `existing_scope: true` requires an explicit
name and skips scope creation; this workflow never deletes a reused scope.

One absolute `RequestOptions` deadline and cancellation token cover every step and
any cleanup. Mutations are never automatically replayed, including when retries are
configured. Returned scope/workspace receipts must reference the expected binding
before the workflow advances.

## Recovery facts

Success returns `GatewayWorkspaceProvisionResult { scope, workspace, scope_created }`.
The scope is the result after additive binding; the workspace is the source creation
receipt, including unknown response fields. Read the workspace separately for settings
that are absent from that receipt.

Failure returns `ProvisionError`. `error()` exposes the safe underlying SDK error;
`recovery()` exposes explicit, bounded reconciliation information:

| Fact | Meaning |
| --- | --- |
| `stage` | Preflight, scope creation, workspace creation, or binding |
| `scope_name` / `scope_state` | Name plus existing/created/unknown/deleted certainty |
| `workspace_state` | Not created, uncertain, or confirmed created |
| `workspace` | Creation receipt when one was actually received |
| `cleanup` / `cleanup_error` | Whether owned-scope deletion succeeded or failed |

These facts and model bodies are redacted by ordinary `Debug`/`Display`. Identifiers
remain available through explicit access for application-owned reconciliation.

A definite workspace rejection or a failure before workspace dispatch can permit
deleting a scope created by this workflow. Cleanup uses the remaining original budget;
it cannot extend a cancelled or expired operation. Its failure never replaces the
original error. IAM deletion retains its frozen experimental live status.

An uncertain workspace creation retains its scope: the service might already have
created a workspace that depends on it. This deliberately strengthens the frozen
TypeScript behavior, which attempts rollback after any create failure. Reconcile the
workspace and scope by their explicit identifiers before deciding whether to reuse or
delete either resource. A malformed or mismatched successful receipt is also uncertain.

A binding failure retains the confirmed workspace receipt and its scope. Finish with
the canonical `IamScopesClient::bind_workspace(scope_name, workspace_slug, options)`
after inspecting current state. No rollback of a confirmed workspace is inferred.

## Evidence

`gateway_provision_contracts.rs` independently asserts exact IAM/admin routes,
headers and bodies; success and reuse; preserved bindings; no mutation replay;
unknown creation outcomes; safe cleanup and cleanup failure; pre-authentication
validation; generated-name propagation; mismatched receipts; cancellation and a
shared deadline. The operation mapping preserves all deliberate differences.
Implementation and offline contracts are complete; successful live provisioning
and cleanup remain unverified until controlled qualification.
