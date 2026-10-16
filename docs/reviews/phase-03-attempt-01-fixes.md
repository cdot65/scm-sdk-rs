# Phase 3 review 1 fixes

The actual Claude Code review scored 8/10. Its original CLI result, prompt, score,
and source binding remain in `phase-03-attempt-01.*`. The exact check reports for
that reviewed source are retained under `docs/checks/phase-03-attempt-01/`.

- API requests now include `service-name: api` and
  `User-Agent: PAN-AIRS/<crate-version>-rust-sdk` before either transport. The
  Rust version/suffix is documented; raw callers cannot override these defaults.
  Complete header name/value sets are checked for scan, OAuth management,
  runtime JSON/SSE, and binary operations.
- Typed Gateway streams skip empty-data keepalives while raw SSE retains them.
  A chunked interleaved regression reaches both typed payloads and `[DONE]`.
- Compatible intersection outputs now merge recursively, including arrays and
  numerically equal scalar representations. Conflicts, depth, node/work and
  combined output bounds are checked; direct regression tests cover stripped fields.
- Scan sync/async transport-failure tests explicitly assert unknown outcomes
  and no replay despite a configured replay budget of five.
- Unreachable unbudgeted JSON refinement code was removed; both finite-JSON
  bindings now have recursive work-budget proofs.
- Isolated scan, management, Gateway, and models configurations enter the
  source-bound Rust check matrix. Tests conditionally compile their own helpers.
- Public documentation spacing and scanner evidence attribution were corrected.

These fixes require a new complete check run and a fresh Claude review. This
record does not claim that the second review has passed.
