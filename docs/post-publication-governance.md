# Post-publication delivery qualification for 0.2.4

The published crate is immutable. Version 0.2.4 was uploaded from commit
`6f18529a47cc273feca1000af6e4fc0d96da757e`, qualified with source fingerprint
`f9cebee5703f3d14f79b91dbbbadcbc538a61cf76ce077ce604a1d69e47c2e58`.
The registry receipt records the exact archive and verified ownership. Neither
the release tag nor the published archive is replaced by this maintenance work.

The first remote-delivery chunk review rejected the maintenance evidence at
36/50. Its principal findings were an unregistered CI observation producer and
missing retained execution evidence for the composed delivery guard. The review,
prompt, check snapshot and CLI output remain unchanged. The maintained observer
now selects the maximum `(created_at, id)` over every bounded status page and
retains the sanitized status snapshot. Its verifier rejects a newer failure or
pending status instead of accepting an older success. The maintenance checks
exercise the producer, snapshot validation and release guards. The second review
accepted the chunk at 43/50 and requested stronger observation consistency,
explicit original/fresh CI receipt separation, validation of the retained guard
and failure records, and more readable tests. Its artifacts also remain intact.
The third review rejected the changed governance source at 41/50 because the CI
logging step expected fields absent from the portable report. The producer now
emits both fields, and a regression executes the exact workflow heredoc against
the actual producer with missing-key negative cases. The fourth review accepted
these corrections at 43/50 with no blocker or high findings. Its remaining
findings concern the CI Jobs description, complete review-history bookkeeping,
and machine validation of the readiness scope-correction record. These follow-up
changes require fresh applicable review; the earlier accepted source is retained
as historical acceptance.

## Separate payload and governance identities

`tools/postdelivery.py` compares every tracked and unignored source path, file
mode and byte with the published source. Only its explicit governance file list
may differ. Rust sources, examples, contracts, manifests, dependency lockfiles and
the packaged README must remain identical. The workflow may change only the
contracts verification command and receipt filename; its job matrix remains
identical. This qualification cannot authorize a second registry upload.

The retained original checkout continues to execute its original delivery guard
in a separate process. Its 105-command qualification, readiness review, two
compiler package proofs, archive locations, registry receipt and annotated-tag
receipts retain their original bindings. The current governance checkout runs
its own focused command set and records the original guard's actual result. New
CI and tag observations identify the maintained producer, the current governance
fingerprint and the immutable published source separately.

Original CI evidence remains at `delivery.ci_receipt`. A fresh maintained-producer
observation uses `delivery.ci_observation`, matching the separate original and
fresh tag fields. The retained execution receipt
`docs/checks/phase-28-composed-delivery-F.json` is hash-bound and validated against
the actual original guard execution; the current report separately records its
new execution.

The observer records the start and end of its observation window. It captures
two bounded, normalized status snapshots and run histories, requires them to
agree, and requires the selected successful run to be the latest run for the
same commit. A later failure or pending run cannot be hidden by selecting an
older success. The earlier run-30 failure remains a validated historical input.
These are point-in-time observations: the Forgejo API offers no atomic snapshot
or signature, and a run created after the observation window is outside the
receipt's claim. Two matching captures reduce pagination instability; they do
not make the remote service's state permanently immutable.

Host qualification:

```sh
python3 tools/check_phase.py --phase 28 --post-publication-governance \
  --qualified-release-root /path/to/retained-qualified-checkout
python3 tools/release_delivery.py --phase 28 --version 0.2.4 \
  --post-publication-governance \
  --qualified-release-root /path/to/retained-qualified-checkout
```

The separate `docs/checks/phase-28-postdelivery.json` report records the host
execution. The configured CI route reruns the governance tests, verifies the retained host report and
all its evidence bindings, compares the immutable source inventory, and downloads
the exact public registry archive with a size bound, no redirect, and checksum
comparison. CI writes `phase-28-postdelivery-contracts.json`; it does not claim
to have executed the external retained checkout. This report never replaces the
original 105-command qualification. The governance route currently has host-local
execution evidence only and no terminal remote run. Run 31 executed the
published-source contracts route, not the governance route.

The explicit phase-28 qualification boundary in `docs/PHASES.md` assigns the 49
required CI contexts to the immutable published SDK source. The later governance
workflow variant has local regression evidence only and is excluded from that
published-SDK delivery requirement. It does not have a terminal remote run. The
local governance suite also executes `tools/test_phase_checkpoint.py` because the
maintained phase checker changed; the original 105-command SDK gate is unchanged.

Current delivery receipts are owned by the full phase's `scope_evidence` and the
remote-delivery chunk's `phase_followup_scope_evidence`. The fifth chunk review's
original scope and all five scope revisions remain unchanged. These subsequent
receipts require coverage by a fresh accepted complete-phase review, rather than
a sixth chunk invocation or a retrospective rewrite of the fifth review.

The post-publication readiness chunk's attempt-5 snapshot has this same limited
scope: payload equivalence plus governance tests and executed delivery guards.
It is not a re-execution of the 105-command readiness gate against changed
governance source. The original readiness commands and compiler package proofs
remain executions against the unchanged published-source fingerprint. The
attempt-5 rejection remains preserved and cannot establish chunk acceptance.
Its scope correction is recorded separately in
`docs/checks/phase-28-readiness-05-disposition.json`, preserving the original
review bytes. The current governance guard verifies its referenced hashes,
rejected decision, score, source identities and retained SDK qualification,
then binds the record separately as `readinessDisposition` in its report.
The fifth attempt exhausted that chunk's budget; additional
readiness recovery requires explicit maintainer authorization. The authorized
phase-level final-delivery budget does not reset the chunk budget.

## Review budgets

The maintainer explicitly authorized the final-delivery recovery scope in
`docs/reviews/recovery-authorizations/2026-10-22.json`, answering
“Authorize the final-delivery review budget.” The maintained review tool verifies
the exact grant and pinned original readiness artifacts before using the separate
`phase-28-final-delivery-attempt-NN` namespace. All five original phase readiness
attempts and the rejected remote-delivery attempt remain intact. The new final
phase budget permits at most five actual Claude invocations and requires at least
46/50 for acceptance. Chunk reviews retain their existing budgets and rubric.

Each current review binds the governance fingerprint and immutable published
payload separately. Source changes require refreshed governance checks and a
fresh applicable review. Archived review evidence retains its historical meaning;
it is never presented as an execution against changed governance code.

## Remaining delivery boundaries

Successful release CI belongs to run 31 at the same source commit as the failed
run 30. The failure record and timing reproduction remain visible in `docs/CI.md`.
The passing separate run does not establish a scheduler root cause or fix the test's
short wall-clock deadline. A deterministic test follow-up remains separate from
the immutable published SDK payload.

The documentation site has its own reviewed branch and deployment evidence.
Registry availability does not imply that Pages has deployed the new guide.
Directional-profile live service acceptance remains unverified. Historical phase
24 CI and the separately authorized phase 25 recovery retain their own status;
publication of 0.2.4 does not silently complete those phases.

Remote-delivery attempt 5 was accepted at 43/50 against source `225696bdedc445c1a1ba0c0ecb77df04337bb91874212c36570a79fcb1fdebc6`. Its subsequent corrections retain that historical acceptance and exhausted chunk budget. They require a future authorized full-phase review of at least 46/50; no sixth chunk review is permitted. Readiness recovery authorization remains pending.
