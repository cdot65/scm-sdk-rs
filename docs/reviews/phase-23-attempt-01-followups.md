# Phase 23 attempt 1 corrections

The chunk reviews scored 44.5/50 and 45/50; the enclosing phase scored 44.5/50 and did not pass its 46-point gate. All original prompts, CLI output, check snapshots and reviews remain unchanged. Original phase receipts are copied under `docs/checks/phase-23-attempt-01-evidence/` before the new run.

## Corrections for attempt 2

- External frozen consumers now select the forwarded `red-team` feature explicitly. Receipts bind the generated consumer manifest and selected features; negative tests reject the former no-op command. Published consumer source bytes remain unchanged.
- Package receipt verification resolves the local pinned compiler host/version and rejects mismatches. Archive development-path rejection is reachable and tested.
- Every current phase receipt/archive binding is checked again before a passing full report and before review acceptance. A regression simulates a later command replacing an earlier receipt. Historical review snapshots retain their original source and evidence hashes.
- Phase and chunk artifact scopes include the generated public models, graph, union ledger, model tests, management example and notices. Attempt 1 scopes are preserved; attempt 2 has explicit revisions.
- The generated fixture corpus discloses its authored vendor-derived acceptance expectations. Its three Rust test functions are now bound and removal-tested.
- The overlay and generated G25 addendum name both previously-Unknown union shapes that now select appended named variants, retaining legacy branch precedence.
- Removed the unused import, formatted checkpoint tests, and placed generated documentation before attributes.

Fresh cumulative qualification and actual Claude attempt 2 are required; these corrections do not themselves establish acceptance or publication.
