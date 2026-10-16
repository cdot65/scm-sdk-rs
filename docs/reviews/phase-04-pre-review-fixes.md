# Phase 4 verification correction

The initial 29-command Phase 4 check sequence passed 28 commands, including both
compiler matrices, 217 tests with all features, schema/typed consumers, packaging, and
isolated DLP. Its final IAM-only Clippy command rejected unused `AuthKind` and
`require_auth` helpers. IAM construction had validated OAuth through
`with_tsg_header` alone; sibling clients also use the explicit credential-kind
guard.

IAM construction now checks `AuthKind::OAuth` before deriving its immutable
tenant header. This preserves IAM's credential plane contract independently of
any future header-scoping extension and exercises the helper in IAM-only builds.
The failed gate and original child reports are retained in
`docs/checks/phase-04-pre-review-iam-fix/`. Fresh full source-bound checks are
required before the first actual Phase 4 Claude review; no review score is
claimed for this check failure.
