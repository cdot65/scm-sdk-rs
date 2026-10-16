# Phase 2 review remediation

Actual first review: `phase-02-attempt-01.json`, score 8/10. The phase did not advance.

| Finding | Change |
| --- | --- |
| Encoded path traversal | `config.rs` rejects encoded slash, backslash, and percent before admission, including recursive encoding. Fourteen new engine cases assert zero authentication and transport calls. |
| Missing MSRV feature runs | `commands_for(2)` now runs Rust 1.88.0 and 1.99.0 across default, minimal, and all features. |
| Dependency policy deferral | Phase 2's machine-readable limitation explicitly assigns fresh advisory/license/source policy verification to Phase 8. Selection and native build review remain in `docs/DEPENDENCIES.md`. |
| Public enum/struct evolution | `Authentication`, `TokenInfo`, and `RemoteOutcome` are non-exhaustive. Defensive authentication validation remains. |
| Inert feature flags | Removed product/realtime/tracing flags and direct tracing dependency until their implementations arrive. Only the implemented reqwest adapter is a foundation feature. |
| Deliberate Rust differences | `docs/FOUNDATION.md` explains encoded Basic credentials, bounded numeric tenant IDs, RFC bearer grammar, refresh buffer, operation budgets, and replay restrictions. Configurable refresh buffer is a machine-readable Phase 7 parity item. |
| Documentation compilation | Rustdoc includes the actual packaged foundation example as a checked `no_run` block; the markdown points to that source. The initial follow-up run executes one passing doctest. |

The original check report and native reports are preserved with `review-attempt-01`
suffixes. A new complete passing source-bound check report and fresh actual Claude
review are required before advancement.
