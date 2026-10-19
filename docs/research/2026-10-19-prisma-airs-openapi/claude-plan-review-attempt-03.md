Write is disabled this session, so the review can't be persisted to the plan file. Delivering inline.

# Prisma AIRS audit — final targeted advisory review (attempt 03)

**Scope.** Only the attempt-02 residuals and regressions introduced by the repairs, against README.md at `review-input-03.json` hashes, with line-level spot checks in `tools/verify-contracts.py`, `tools/check_phase.py`, `tools/test_ci.py`, `contracts/manifest.json`, `contracts/vendor/`, SDK source. Read-only; no commands, tests, service calls, or edits. Advisory plan acceptance only.

## Verdict

**Plan 9/10 (target ≥9: met). Total 47/50, every category ≥8 (targets met).** The medium ownership/license invariant that held attempt 02 at 45 is closed in exact, convention-consistent terms. B1–B9 remain closed; no regression found.

| Category | Score |
|---|---:|
| Implementation feasibility | 10/10 |
| Code-quality approach | 9/10 |
| Supportability | 9/10 |
| Architecture | 10/10 |
| Feature delivery / coverage claims | 9/10 |
| **Total** | **47/50** |

## Residual closure

**1 (medium) — A.1 corpus-ownership + license binding: Closed.** README:97 names `verify-contracts.py:416–418` exact owned-files equality (confirmed: `owned_files` accumulated by `tracked()` at `:257–259`; glob restricted to `contracts/{fixtures,sources,licenses}` at `:417`), extends ownership to the explicitly registered `contracts/vendor/red-team-management/0.86.0/` root while retaining the three existing directories, requires full enumeration with exact equality and a hashed `vendorArtifacts` inventory, and forbids both dropping vendor paths and globbing historical vendor trees. That restriction is correct: `red-team-data-plane/0.93.0/` (10 files incl. `LICENSE`, `source.json`) and `portkey-access/a27feb9/` are verified by `tools/red_team_dataplane.py --check` / `tools/portkey-access.mjs --check` and keep separate ownership. License side: `:316–320` requires record `repository` **and** `commit` equality, and `pan-dev-582e30543bf2` + `contracts/licenses/pan-dev-582e30543bf2.LICENSE` matches the established convention exactly (`manifest.json:1974–2012`: `pan-dev-8bf86246fa53`, `-e2e6e63e6acd`, `-891ec0e6c220`, `-2021f974ab64`, all MIT). Feasible: license records are only `tracked()` (`:304`), `verify_origin` applies solely to `id == "typescript"` (`:306`), so a non-baseline license file is admissible. Uniqueness correction is accurate — `:313` enforces `id`. The `…@0.86.0+582e30543bf2` ID form breaks no consumer; nothing derives a path from a source `id` (`:379–381` strips an explicit prefix for fixture pins only).

**2 (low-med) — check_phase wiring: Closed.** README:98 extends `commands_for` 21–25, `ci_commands_for`, and the **two** `--contracts-only` allowlists — confirmed as separate hardcoded tuples at `check_phase.py:510` and `:526` — plus CI routing, using phase 20's reuse construction (`:70–87`) as precedent. It preserves the right invariant: `review_phase.check_evidence` compares the full `commands_for` list (`review_phase.py:187`), with cases proving a subset receipt cannot qualify (precedent `test_ci.py:495–522`).

**3a — DP comparison basis: Closed.** README:184 states parsed-semantic comparison against derived `…/0.93.0/openapi.json`, canonical YAML byte equality recorded separately; consistent with `inventory.json:39–41` pinning `dp-openapi.yaml`, `activeBytesEqualUpstream: true`. Both artifacts exist.

**3b — facade anchor: Closed.** `generated.rs:146971` is the shared model (`country_code: Patch<String>` at `:146974`); `facade.rs:287` is `get_management_languages` in `airs/red_team/facade.rs`. README:230 now says exactly that.

**B4 residual — MP hash reproducibility: Closed.** README:25 states the offline limitation explicitly: 0.86.0 bytes and scripts uncommitted, findings from the downloaded pinned snapshot, fresh checkout must fetch or retain it, Phase21 vendors bytes and production tooling, "not a self-contained offline source archive."

**Regression scan: none.** Every added line reference verified. No pinned count breaks — no test asserts `bundledSpecifications`/`licenses` totals (only historical `docs/checks/*` stdout; `test_release_020.py:26` and `test_g10_release.py:51–57` compare command lists/hashes). Preservation stance unchanged.

## Optional precision notes (non-blocking)

1. **CI repointing lockstep unnamed.** "Generated CI/workflow routing" is slightly off — `.forgejo/workflows/ci.yml` is committed and test-validated. If CI leaves phase 20, these move together: the workflow phase string and report path; `test_ci.py:504–506`, which literally asserts `--phase 20 --verification-only --contracts-only`, `docs/checks/phase-20-contracts.json`, and absence of `docs/checks/phase-20.json`; `docs/CI.md` ("CI now executes the Phase N contracts gate", full/subset/job command counts, and a `scripts/check-*.py` list equal to the executed subset — `test_ci.py:44–65`, `:130–145`); `docs/RELEASE.md`'s `--phase N`. Cleanest: declare CI stays on the phase-20 contracts gate during 21–25 so only `commands_for` grows. Keep the substring `requires --phase 12 --verification-only` in `check_phase.py:527` (asserted at `test_ci.py:522`).
2. **Name the negative-case harness.** The five A.1 negative cases belong in `verify-contracts.py` `self_test` (`cases` at `:132–136`; `copytree(ROOT / "contracts")` at `:139` already covers the new vendor root), and phase 21's `commands_for` must include `verify-contracts.py --self-test` — absent from the phase-19/20 lists (`:88–108`), reaching 17/20 only via the phase-12 slice — or the cases never gate.
3. **SPDX field.** The new license record needs `spdx: "MIT"`; `:305` rejects anything outside MIT/Apache-2.0.
4. **Naming precedent.** `red_team_data_plane.rs:21253` already exposes `LanguageDropdownOption` while management's model uses `LanguageOption` (`generated.rs:179777`). README:230 is correct about management; citing the DP type would strengthen the alias decision.

## Unproven static assertions (acceptable; keep labeled)

- All 0.86.0-side findings remain unverifiable from this repo until Phase21 vendors the bytes — README:25 now says so.
- RT findings are static code-path evidence, stated as such; no tenant evidence claimed.
- Not re-verified per request: 47 DP / 5 broker rows, 94 RT rows, 320 property comparisons, 68 pointers, all SHA256/byte-identity claims. No hashing performed.

## Score rationale

Feasibility and architecture reach 10: the single hard-failure mechanism is named at the exact line, scoped to an explicitly registered root that cannot over-glob historical pins, bound to a license record satisfying repository-and-commit equality under the repo's existing naming scheme, and backed by an existing tampering harness. Remaining −1s are depth, not correctness — the CI-doc/test lockstep set and the self-test/SPDX mechanics are unnamed (code quality), and offline reproduction still depends on retaining the snapshot until Phase21 (supportability, delivery claims). Plan holds at 9 for that same intrinsic evidence boundary. No security or release blocker remains; targets plan ≥9, total ≥46, each ≥8 are met.