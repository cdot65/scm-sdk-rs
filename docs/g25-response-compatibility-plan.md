# G25 response compatibility — scm-sdk 0.2.2

The maintainer requests the G25 compatibility correction and publication. Version
0.2.1 is already published for the Portkey update and is immutable. This fix ships
as the next patch, 0.2.2, retaining all public names/types, dependencies, features,
Rust 1.89 and G10 defaults. Existing releases and CLI pins are not rewritten.
The independently reproduced bug is format validation, not closed response enums.

## Authority and observations

Use the frozen TypeScript 0.34.0 source at 4a80dbdb3dc14365e1c25d838ac0bd139485d48c
and pinned vendor Red Team 0.93.0 schemas. `tools/g25_audit.py` enumerates every
operation, its response root, paired TypeScript schema fields and caller-supplied
UUID checks in `contracts/compatibility/g25/audit.json`. There are 47 operations,
46 JSON response roots and 36 TypeScript counterparts; the CSV response and eleven
new operations have explicit classifications. Twenty-eight operations contain
format checks. Occurrence counts include repeated union branches and are not a
count of unique properties. Do not infer live service acceptance from fixtures. The audit also inventories all
172 frozen roots with reachable format constraints, including non-Red-Team roots
and request-named roots callable with response mode. This shared response relaxation
is a deliberate Rust difference from frozen TypeScript format refinements (including
management, IAM, Model Security and Gateway schemas), not a claim that TypeScript
already ignores those formats. Document the full inventory; requests are unchanged.

The committed `tools/g25-repro` harness, run by `tools/g25_reproduction.py` inside
the phase18 gate, retains exact cases and independent results. A direct Rust reproduction accepts novel sub_category/error_type values but
rejects an opaque UUID string and a timestamp string without RFC3339 formatting.
The actual frozen Zod schemas accept those strings. Zod independently rejects the
three questioned cases: omitted category_display_name, numeric tsg_id and null
sub_category. Keep those structure/type checks. The consumer reports 17 loopback
failures; its checkout is on worf, which this host cannot currently resolve.
Do not claim to have executed those tests. SDK-local equivalent regressions are
required regardless of access to the consumer checkout.

## Response policy

1. In both response modes, bypass only string-format assertions: uuid, rfc3339,
   datetime, date, duration, email, url and regex. Apply this at the shared bounded
   validator so it covers every SDK response root. Preserve original strings
   exactly. Keep string type checks, resource/depth/work/body limits, finite JSON
   rules, unknown enum acceptance, unknown-field passthrough, and the distinction
   between defaults enabled/disabled. Mode::Request retains all existing checks.
2. For each existing Red Team operation, use its actual frozen TypeScript response
   contract as the compatibility floor. Select that contract only in response
   modes, through an explicit mode-aware graph node/adapter; request mode and
   separately retained strict vendor roots still select vendor constraints. Array
   wrappers and unconstrained JSON operations must be explicit, never inferred
   from similar model names. This preserves every baseline presence/type/length
   rule and admits baseline fields the vendor restricts. Vendor-only string length
   checks have no TypeScript justification and are request-only. New operations
   retain vendor structure/types with open responses and relaxed string formats
   and lengths; they must not be labeled TypeScript-parity operations. For these
   unpaired operations, no TypeScript contract establishes broader structural
   cardinality or numeric domains. Retain the vendor numeric/integer and array
   bounds as the accepted structural contract; only the scoped string compatibility
   policy is relaxed. An unexpected out-of-domain service response can still be
   rejected. This retained risk must be disclosed without extending behavior.
3. Baseline passthrough means new vendor fields cannot make an old operation fail.
   Audit the typed decoder as well as validation. Existing public field types must
   remain unchanged. Where an optional vendor-added field cannot decode into its
   typed projection, preserve its original value in the model's existing extension
   storage, leaving the typed projection unset. Generated deserialization and
   serialization must distinguish this from arbitrary known-field collisions:
   baseline-field collisions remain errors, and an explicit typed replacement
   must not produce duplicate JSON keys. Add null/wrong-type/nested-extra roundtrip
   tests for every added field, including anonymous nested model additions.
   Do not silently drop, coerce, default or stringify unsupported service values.
4. Produce an operation and field comparison ledger, with baseline/vendored
   requiredness, nullability, types, string bounds, additions and deliberate kept
   constraints. Validate it against independent Zod outcomes. Shared schemas may
   be deduplicated, but every operation must link its actual response contract.

## Request/query behavior

Keep strict request validation in this patch. Inventory every common::uuid call on
caller-supplied Red Team path IDs, compare with TypeScript's string parameters and
list the retained checks explicitly in the changelog/compatibility report. The
maintainer allows either relaxing them or documenting the behavior change; choose
explicit documentation to keep this response correction narrowly scoped. Preserve
safe segment encoding, traversal rejection, destination admission and auth.

Target-profile error logs accept only limit under the pinned vendor contract.
Keep rejecting supplied skip/search; do not silently drop them or speculate that
the service honors them. No populated tenant error logs establish semantics.
Document that CLI --offset/--search must be usage errors when adopting this release.
Record other previously rejected options as unchanged. No live mutation is needed
to establish response compatibility or query behavior.

## Sequential milestones and review gates

**18 — Plan and comparison.** Commit this plan and reproducible paired schema/
request-check inventory. Retain direct reproduction and independent Zod outcomes.
Run audit guards and review-tool tests; obtain actual Claude Code >=9/10 with no
blocker/high finding before any SDK implementation. Preserve every attempt and
disposition advisory findings.

**19 — Implement and regress.** Change the shared mode-aware string validation,
operation compatibility selection and generated typed fallback. Generate regression
cases by traversing every vendor response root, not a hand-picked operation list.
For every format node and enum occurrence, mutate one field to a nonconforming or
novel string, exercise both response modes, and assert original-value preservation.
Exercise strict vendor roots in Request mode to prove rejection where constraints
exist. Union branches and nullable values need branch-valid witnesses; record
unreachable branches rather than silently skipping them. Independent Zod-derived
presence/null/type/bounds cases check every paired operation. Test the three kept
strict cases and job-1/p-1/map-1 plus both reported timestamp forms. Test every
vendor-added typed projection and raw roundtrip. Execute typed public clients for
attack lists and target error logs through caller transport, including operation
names, modes and query/path rejections. Cover non-Red-Team format validation too.
Update independent fixture expectations only with the explicit response-policy
change; retain request oracle outcomes. Run generator/audit/oracle regressions and
native Red Team checks on both compilers; review >=9 before advancing.

**20 — Qualify and deliver.** Bump to 0.2.2 and document every loosened and retained
rule plus live/consumer limitations. Run the complete existing Python/contract/
generation suites, minimal/default/all on both compilers, targeted Red Team and
JSON/tracing combinations, frozen export probes, dependency/privacy scans and
both clean package consumers. Re-run the selected bounded nonproduction live
qualification without broadening cost or policy mutations; report failures and
missing privileges honestly. Rebuild source-bound ledgers from actual evidence.
Wire the new regressions into the expanded 49-job CI matrix, preserving historical
gate command lists. Commit a clean release source, create both executed clean
archive proofs and verify current packaged bytes against that source. Obtain final
actual Claude review >=9 before publishing. Publish only the exact qualified
archive via the existing external Cargo credential, verify the downloaded registry
artifact, owner, non-yanked state, checksum, VCS and file inventory. Tag v0.2.2 and
ordinary-push to the authorized Forgejo remote. Require actual registry, tag and
terminal exact 49-job CI receipts before completion. Post-publication provenance
edits require current review; excluded evidence-only bookkeeping does not alter SDK
source. Generalize release documentation tests to verify linked receipts rather
than require self-referential archive hashes in packaged files. Never republish a
version or claim consumer adoption without its own evidence.

## Maintainer review update — 2026-10-19

Phase18 was accepted before the new instruction. Remaining chunks and phases use
the five-category rubric in docs/PHASES.md. Phase19 chunks are shared validation
and operation selection; typed projection preservation; and independent regression
coverage plus comparison documentation. Phase20 chunks are qualification/package
proofs and registry/remote delivery. Each chunk and phase receives a fresh actual
Claude Code review with all five scores and retained deficiencies.

Review round two refined CI coverage: retain the historical bare-core
`json-unification` combination and add distinct `red-team-json-unification`
checks on both compilers. This expands the delivery matrix from47 to49 jobs.
It preserves prior coverage while executing the new projection-number regression;
the phase19 gate checks both combinations before the next review.
