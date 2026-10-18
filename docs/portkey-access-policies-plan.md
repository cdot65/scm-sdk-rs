# Portkey access policies — scm-sdk 0.2.1

The maintainer authorizes planning, implementation, qualification, publication to
crates.io under cdot65 and ordinary remote delivery. Implementation starts only
after actual Claude Code plan review >=9/10 without blocker/high findings. Each
subsequent phase has the same review gate. This additive release preserves Rust
1.89, dependencies, G10 behavior, all 23 baseline default bindings, 47 Red Team
operations and the frozen 448-method / 3,820-capability baseline.

## Exact scope and source

Pin Portkey commit `a27feb9efaac3653d2d94eca5d7d210944b6e160` and first parent
`8f91369f15e6b6765a13c1af7e1ee3ea336f83ba`. Retain both original YAML files,
their SHA256 values and upstream MIT license under contracts/vendor/portkey-access/a27feb9.
The supplied change adds five operations, nine schemas and one path parameter;
it changes no existing operation. Do not replace the frozen legacy Portkey
baseline or infer full current Portkey/PANW conformance. Release baseline is
`b65f3c98218f32e836bbdcc0bd95614e9836a4e6`, after verified 0.2.0 delivery.

| Method | Route | Public method | Result |
| --- | --- | --- | --- |
| POST | /policies/access | create | AccessPolicyCreated |
| GET | /policies/access | list | AccessPolicyPage |
| GET | /policies/access/{accessPolicyId} | get | AccessPolicy |
| PUT | /policies/access/{accessPolicyId} | update | preserved JSON object |
| DELETE | /policies/access/{accessPolicyId} | delete | preserved JSON object |

List exposes all twelve query parameters: workspace_id, effect, enabled, user_id,
agent_id, server_id, search, created_by, start_time, end_time, page_size,
current_page. Zero is valid for both paging integers; omit absent parameters and
preserve false. No invented defaults or list_all helper: the contract provides
numbered pages with a total, not a stable cursor or snapshot. Callers explicitly
control page traversal. Returned total/data are optional per upstream schema.

## Authentication and interface

Add `airs::gateway::AccessPoliciesClient` under the existing gateway feature.
Construct it from an explicitly configured core Client, requiring Runtime key
authentication (the existing key mechanism sends x-portkey-api-key). This is a
Portkey-compatible **control-plane** client, not an inference client. Do not attach
it to AIGatewayClient's SCM OAuth data/admin clients, add an implicit endpoint,
copy runtime routing headers, or silently rewrite credentials. The caller must
select and admit the control-plane origin. OAuth, scan and anonymous clients are
rejected at construction. Existing endpoint policy, redirects, deadline,
cancellation, response bounds, safe errors and caller transport remain in force.
Use static `airs.gateway.access_policies.*` operation names, including validation
errors. Reads are replay-safe; writes must never be automatically replayed,
including when RequestOptions permits more attempts. A 409 remains an HTTP
conflict with no automatic get/retry, merge or version increment. Ambiguous writes
retain the engine's uncertain-outcome behavior.

## Typed contracts and validation

Handwrite the small isolated model family and validators; do not regenerate or
modify frozen model declarations. Put models in a new access-policy module and
re-export through the client module. Contract extraction is deterministic from
the pinned YAML, and independently evaluated fixtures check handwritten behavior.
Keep response and request types separate. All ordinary Debug output is redacted.

- Request subjects have user/group/agent kinds and a string ID. ANY is permitted
  for users and agents, not groups. Do not impose a UUID on subject/provider/server
  IDs: the schema does not. Path policy IDs must be UUIDs.
- Request targets are tagged MCP-server or provider variants. MCP requires tools;
  provider models may be omitted. A selector is exactly ALL or a nonempty list
  of nonempty strings. Models omitted on the wire stay omitted; backend ALL
  behavior is documented, never injected.
- Create requires a 1..64 Unicode-character name and nonempty subjects/targets.
  Description is omitted/null/string (max 256 characters). enabled and effect
  are optional; do not synthesize enabled=true or infer effect client-side.
  An explicitly allow policy, or a create whose omitted effect implies allow,
  must name a specific subject. Both user ANY and agent ANY imply deny when
  create effect is absent. No client authorization decision is inferred from
  these construction rules.
- Update requires version >=1, nonempty replacement subjects and targets. Name,
  description, effect and enabled preserve omission; null description clears it.
  Omitted effect retains stored state, so its semantics cannot be inferred from
  the replacement subjects. No hidden GET. Explicit allow still requires a
  specific subject. Optimistic concurrency is caller-controlled.
- List validates documented string maximums (36/36/100/64/255), RFC3339 timestamps
  and nonnegative integers, without imposing undocumented timestamp ordering or
  defaults. Reuse bounded serialization and encoded query/path construction.
- Response fields follow upstream optional/nullable declarations; use Optional
  and Patch as appropriate, with flattened extra fields preserved. Unknown effect,
  subject kind, target variant and status stay explicit open values; never map
  unknown values to allow. Preserve missing versus null and unknown nested values
  after serialization. Both default response modes produce the same data here
  because this response family declares no defaults. Do not manufacture targets
  or normalize server JSON. Wrong known-field types are decode errors.
- Bound all request work with the core operation options and serializer limits;
  no extra network, unbounded recursion or arbitrary-body diagnostics. Request
  schema additionalProperties is permissive; preserve reviewed extension maps
  without allowing collisions to override typed fields.

## Sequential phases and acceptance

**15 — Source and plan.** Pin sources/license and verify the exact first-parent
delta, extract a normalized contract with all references resolved for this family,
record rationale and this plan, register phase gates. No SDK implementation.
Run pin/delta checks, whitespace and review-tool tests. Obtain actual Claude plan
review >=9/10. Resolve all blocker/high findings and disposition every advisory.

**16 — Implement and verify.** Implement the five methods, typed models and
request validation; add an offline consumer example. Maintain a five-operation
ledger separate from the frozen baseline. Independent literal wire fixtures
exercise every verb/path/query/body and response. Schema fixtures are checked
against the pinned OpenAPI with a separate development-only schema validator,
including documented semantic constraints not expressible in OpenAPI. Cases:
missing/null/value, unknown vocabularies and nested extras, ALL versus lists,
empty and Unicode boundaries, create effect inference versus update omission,
version 0/1, invalid UUID/date/query bounds, zero paging and false enabled,
wrong auth, forbidden destination, redacted output, 409 and no mutation replay,
cancel/deadline and response limits. Run all Gateway tests and native Rust
checks on 1.89.0 and 1.99.0; default/minimal/all combinations follow in phase17.
Review >=9/10 before advancing. Preserve failed attempts.

**17 — Qualify, release and deliver.** Bump only scm-sdk to 0.2.1; no new runtime
dependencies or MSRV increase. Update changelog, feature docs, README, NOTICE and
package example coverage. Re-run frozen generation/contract/native ledgers and
dependency/security checks. Preserve historical command lists. Scope the old
0.2.0 immutable-runtime test to its accepted source rather than applying that
old release freeze forever to new features. Add the new contract verifier to CI
without changing the existing 46 Rust matrix jobs or weakening old checks.
Refresh source-bound ledgers and live evidence; classify new endpoints separately.
Run minimal/default/all on both compilers and targeted gateway/tracing/JSON
unification checks. Commit a clean candidate; execute both clean archive proofs,
including the new offline example and a packaged external consumer of all five
new methods. Obtain final actual Claude review >=9/10 before publication.

Live testing is restricted to read-only list/get against a separately selected
compatible control-plane registration. Inspect configuration privately; never
guess that an inference URL/key is a policy administrator. If no appropriate
registration exists, record not-attempted with the prerequisite; do not block
shipping the explicitly experimental, offline-qualified API on unavailable
service access. No live access-policy create/update/delete: these can alter
authorization for real users. Do not expand paid/provider traffic. Existing live
qualification may be rerun using the prior selected nonproduction registration;
old results remain historical, never rebound to new source without execution.

Publication uses /home/cdot/.cargo/credentials.toml outside repository and vault,
owner cdot65, mode0600. Never pass the token in environment or command arguments,
capture it in diagnostics/receipts, or supply it to CI. Repackage a clean detached
worktree at the qualified source and require the complete archive digest to match
the 1.99.0 proof before publishing. Check registry first after ambiguous results.
Verify downloaded crates.io version, ownership, non-yanked state, archive digest,
every file and clean embedded VCS against both compiler proofs. Create immutable
annotated v0.2.1 at that source. Ordinary push to the authorized Forgejo remote;
wait for all 47 jobs and the exact run to finish successfully. Retain registry,
tag and terminal-CI receipts, then advance17 and push evidence-only bookkeeping.
Do not claim publication until the downloaded artifact is verified.

## Scope boundaries and review follow-up

Portkey source compatibility is distinct from deployed PANW feature support. New
methods remain experimental/live-unverified unless explicit read evidence exists;
even read evidence cannot promote mutation acceptance. Unrelated current Gateway
endpoints, SCM OAuth adaptations, automatic policy evaluation and broad Portkey
baseline migration are outside this commit's scope. Any newly discovered required
contract correction must enter the plan and receive review before implementation.

Phase14's low-priority generic release-verifier negative tests remain recorded
maintenance; this release must still enforce real archive identity and record all
review findings. Phase17 explicitly replaces future-version documentation guards
with version-derived checks so release claims require actual matching receipts
before phase advancement. Historical receipts are preserved, never overwritten.

Source rationale and primary citations: [research findings](portkey-access-policies-research.md).
