# Supplementary G10 pinned-branch review

Fixed point: `840c0906f57548e6c13c1cd98152ad1ac18732a8`.
Reviewed commit: `a6d63e58f067a98504e5b79987639d53e11fc931`.
Diff: `git diff 840c090...a6d63e5` (one additive G10 commit).

These are read-only Codex subagent reports requested by the code-review skill,
not Claude Code scores. The parent reproduced the copy-limit mismatch with an
external public consumer on Rust 1.89. Actual Claude advancement reviews are
retained separately in `phase-90-attempt-*.json` and their CLI/prompt artifacts.
The reproduction command, program, runtime source hash and actual output are
retained in [the pre-fix receipt](../checks/g10-pre-fix-copy-limit-reproduction.json).
That receipt demonstrates a known defect and does not qualify a release.

## Standards

Reviewer: `/root/g10_standards_review`.

No blocking/high runtime, security, maintainability or MSRV violation found.
The default path remains unchanged, response-policy predicates are centralized,
and request validation retains its original mode. No actionable code smell found.

Two medium verification gaps:

- **Later pagination pages are untested:** the new resource test constructs two
  items with `total_items: 2` and collects using the default limit 50. It fetches
  one page despite the every-page claim. `docs/RED_TEAM_RESOURCES.md` promises
  propagation on every page; the implementation plan requires independent wire
  expectations. Use two distinct pages, a smaller explicit limit and asserted
  subsequent skip queries.
- **Typed explicit-null preservation is untested:** the new target/OAuth fixtures
  contain no null values. The schema corpus tests validator output, not typed
  decode/reserialization. AGENTS.md and docs/MODELS.md require omission/null/value
  preservation. Add known nullable fields, nested/extension nulls and an omitted
  counterpart, with exact JSON comparisons across target calls and later pages.

These are missing regression evidence, not demonstrated runtime failures. No
files were changed or tests run by this reviewer.

## Spec

Reviewer: `/root/g10_spec_review`.

**Medium — acceptance differs under identical resource limits.** The user requires
the same documents accepted/rejected as Mode::Response. At validation.rs:766 in
the pinned source, suppressing an absent default also skips its key's copy
accounting. For BasicAuthAuthConfigSchema input `{"basic_auth_header":null}`,
max_input_bytes 26 and max_copied_bytes 30, Response charges 42 bytes and rejects;
ResponseWithoutDefaults charges 23 and accepts. Charge the missing default key
identically before deciding whether to insert it.

**Low — explicit guarantees lack targeted coverage.** The user requires every
list_all page and preservation of server-sent nulls. The new resource test covers
one page without explicit nulls. Add two pages and outer/nested nullable fields.

No scope creep was found. Additive APIs, the default policy, request normalization,
operation names, MSRV and dependencies match. Existing pagination clones the
option; generated presence wrappers preserve absent versus explicit null values.
Static graph traversal found no default descendants under frozen effects,
pipelines, intersections or the discriminated union. Only AuthConfig and
ConnectionParams ordinary unions contain defaults; no additional downstream
refinement mismatch was found. No edits or tests were run by this reviewer.

Summary: Standards has two verification findings, worst medium; Spec has two
findings, worst medium (a demonstrated limit-accounting defect).
