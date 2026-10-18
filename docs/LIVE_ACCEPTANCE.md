# Native service qualification

The [live acceptance policy](LIVE_ACCEPTANCE_POLICY.md) governs tenant selection,
owned fixtures, cost limits, promotion, and cleanup.

The source of truth is [the exhaustive qualification matrix](../contracts/qualification.json),
with one row for each of the 448 frozen methods. Implementation, independent
contract verification, and successful service execution have separate fields.
Each unqualified method has a reason. The [current receipt reference](../contracts/live-evidence.json)
binds an allowlisted native execution report to the SDK source, dependencies,
compiled executable, Rust version, host target, and execution time.

The current vendor-development run (attempt 10) verified 17 exact native methods
from 24 service requests and one token exchange. Its scan-ID lookups completed
with correlated results. Three Red Team reads returned 403 permission rejections;
IAM scope creation and Gateway provisioning returned 400 rejecting the submitted
requests. The cause of these two 400s remains unestablished, including whether a
native request-shape defect contributed. Both creations were rejected before
ownership, so cleanup was not needed. Those methods remain live-unqualified.
The receipt binds this candidate's full runtime fingerprint, including its
current vendor validators and typed model additions.

The published 0.1.1 source had a separate run (attempt 08) verifying 16 methods.
Its synchronous scan-ID lookup was incomplete or uncorrelated while the async
lookup succeeded, leaving that canonical method unqualified in that receipt.
Attempts 09 and 10 are new executions; their results do not rewrite the publication's
historical observations. Earlier 0.1.0 receipts also verified 17 methods.

The first 0.1.1 execution (attempt 07) completed 24 service requests with cleanup
not needed, but the wrapper rejected its version because it was hardcoded to
0.1.0. The [parser-attempt record](checks/phase-91-live-refresh-attempt-07.json)
preserves that failure. The manifest-bound parser now rejects receipts for a
different SDK version. No earlier failure was promoted.

## What the controlled run covers

The qualification application uses the native crate for OAuth, AIRS scan HMAC,
management, DLP, Model Security, Red Team, AgentGuard, Gateway configuration, and
shared IAM. It selects one explicitly supplied nonproduction registration and
checks its TSG against a separately supplied expected scope before authentication.
It uses a returned scan key only when that key explicitly belongs to the selected
tenant; otherwise it explicitly selects the obtained OAuth token for scanning.

Read probes use bounded lists. Synthetic scanning submits one short prompt and
one single-item async batch. Result acceptance requires correlated scan/report
identities and completed non-error scan data; an empty, pending, or unrelated
typed response does not establish completion. Unknown verdict values stay unknown.
The run does not accept a Red Team EULA or start a model, agent, or attack job.

Mutation probes create a fresh IAM scope, read/update it, delete it, and verify
absence. A second fresh scope and workspace exercise the native provisioning and
binding workflow. All fixture names include 128 bits of locally generated entropy.
The preflight uses the unpaginated IAM listing only when its count accounts for
every returned item. This is a collision check; the upstream API offers no atomic
reservation. Known creation rejections grant no cleanup ownership. Subsequent
scope mutations require the confirmed identity, retain unrelated bindings and
extension data, and delete only an empty owned scope.

Workspace DELETE is archival. Cleanup requires the exact owned workspace to
appear in the archived listing before its owned binding is removed. Other
bindings are retained. Ambiguous initial writes retain dependencies for
reconciliation; an immediate missing listing cannot prove that an uncertain
create will never complete. Local cancellation does not undo server writes.

The application has no automatic retries, at most 80 transport attempts, a
15-second budget per operation, bounded bodies, and a ten-minute overall limit.
Private recovery snapshots record intent before each creation and preserve known
resource IDs and uncertainty. Snapshots and diagnostics remain in a newly owned
private directory outside git and published packages. Synthetic scan audit records
remain on the server because the frozen scan API has no deletion method.

## Reproduce a controlled run

Only this explicit mode performs external requests:

```sh
python tools/live_evidence.py --run \
  --config /absolute/path/to/nonproduction-registration.json \
  --expect-tsg EXPECTED_NONPRODUCTION_TSG \
  --mutations --attempt NEXT_UNUSED_NUMBER
```

The registration has exactly `mgmtClientId`, `mgmtClientSecret`, and `mgmtTsgId`.
Keep it outside the checkout. Each attempt preserves its earlier public receipt
and private recovery snapshots. An interruption reports the private recovery
location. Reconcile any uncertain writes before rerunning. Never select production
or repurpose an existing resource for these disposable-fixture probes.

Ordinary verification has no service access:

```sh
python tools/live_evidence.py --check
python tools/native_operations.py --check --require-through-phase 8
python tools/native_exports.py --check
python tools/qualification.py --check
```

The verifier rejects stale runtime inputs, tampered receipt hashes, failed native
execution, arbitrary output fields, unreviewed classifications, failed operations
marked successful, and hidden cleanup uncertainty. Digests detect inconsistency;
repository writers and the actual qualification process remain trusted.

Rejected-request rows may include `rejectionDetails` with a closed-vocabulary
service code and up to 20 distinct, already-allowlisted problem field names.
Titles, reasons, messages, values and unknown codes are withheld. A null code or
empty field list supplies no additional cause evidence; it does not establish a
service defect or exclude an SDK defect. Historical receipts without this field
retain their original observations.

Every observation of a method is retained, including cleanup and reconciliation.
A success combined with a failure cannot qualify the method: the receipt marks
the conflict for reviewed reconciliation and the matrix conservatively keeps it
live-unqualified. A new source-bound receipt after reconciliation is required to
establish success; an earlier successful probe cannot mask a later failure.

## Scope of the evidence

Only workflows recorded as successful in the current receipt are live-verified.
Neither a list call nor a constructor certifies other operations in its family.
403 responses retain a permission prerequisite; unavailable routes, rate limits,
network failures, service contract rejections, and SDK validation/decoding failures
have distinct classifications. A failed native operation is never promoted by
offline parity evidence. Previous attempts remain historical evidence and do not
certify later changed runtime sources.

Gateway runtime inference, SSE provider execution, and realtime provider readiness
need an explicitly selected runtime origin, runtime credential, and compatible
provider fixture. Those prerequisites are not established by SCM OAuth or a
WebSocket upgrade. The current run therefore leaves them live-unqualified; their
independent transport/parser/contract checks remain recorded. Red Team permissions
and DLP service deployment are similarly assessed from actual receipts without
assuming that management-console access grants product permissions.

Native execution currently covers Linux x86_64 musl. See [platform evidence](PLATFORMS.md)
for the separately labeled compile checks and [phase status](phase-status.json)
for retained remote Forgejo qualification. The [registry publication receipt](checks/release-registry-0.1.0.json)
separately records the published artifact and its exact source; publication does
not expand live or native platform acceptance.
