# Phase 22 first-review corrections

All three first chunk reviews were accepted without blocker/high findings. The second review cycle covers the following corrections on a fresh checked source; these notes do not substitute for executed checks or acceptance.

## Target filtering (44/50)

- Generate the internal profiling allowlist from `ProfilingStatusFilter` in the pinned management spec; the client uses that generated constant.
- Document response-only `INIT` and `PARTIALLY_COMPLETE` values and the absence of automatic mappings.
- Exercise all four accepted filter values across list/pages/items/list_all, with search, adapter, target-type and legacy status filters; compare full decoded query maps and counts to detect duplicates.

## Connection compatibility (43.5/50)

- Generate a separate management projection table in the G25 report, qualify the historical data-plane corpus, and assert the documented model set covers both projection mechanisms.
- Check exact frozen node identities, expected generated model names, and independently discover the complete affected projection object set; reject missing/incorrect mappings.
- Derive enum-extension ledger values from the applied source delta, including duplicate handling; reject modified production source bytes through the pin.
- Add negative tests for missing wire-test names and disagreements between union ledgers.
- Match schema-bound enum values to named variants while proving unrelated values remain Unknown and frozen constant enums retain their old behavior.
- Record the five operation response declarations, retained shared root, field coverage/requiredness/nullability, fixture binding and explicit limits of the comparison.
- Remove a dead ledger assignment; make generated header/import/declaration rewrites require exactly one match.
- Correct human-readable phase spacing; retain machine identifiers with an explicit comment.

## Typed connections (44/50)

- Include both Gateway derived-ledger owner checks in phases 22–25, with governance regression coverage and updated CI counts.
- Explain frozen TypeScript parity enums versus current schema-bound enums in generated rustdoc and the overlay ledger.
- Explain the declaration-only redacted Vertex view and the shared branch selected by target GET responses.
- Bind the full wire-test file bytes in the generated overlay source bindings and require that binding in the adoption verifier.
- Retain separate implementation, executable contract, and live acceptance evidence. No current live claim is inferred from historical receipts.
