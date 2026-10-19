"""Governance regression tests for the additive management follow-up."""
import json
import hashlib
import copy
import os
import subprocess
import pathlib
import tempfile
import unittest
from unittest.mock import patch

from check_phase import CONTRACT_LEDGER_OWNERS, verify_scoped_ledger_owners, MANAGEMENT_BASELINE, PLANNING_FIXTURE, commands_for, ci_commands_for, verify_planning_fixture, verify_planning_gateway_proof, require_only_hash_replacements, MANAGEMENT_PHASES
from review_phase import check_evidence, main, matches_declared_scope, next_phase, verify_record, sha256, gate_decision, verify_attempt_sequence, SCOPE_BOUND_PHASES

ROOT = pathlib.Path(__file__).resolve().parents[1]


class ManagementPhaseTests(unittest.TestCase):
    def test_plan_gate_freezes_sdk_and_checks_provenance(self):
        commands = commands_for(21)
        self.assertEqual(commands[0][:5], ["git", "diff", "--exit-code", MANAGEMENT_BASELINE, "--"])
        self.assertIn("crates/scm-sdk", commands[0])
        self.assertIn("contracts/native-exports.json", commands[0])
        self.assertIn("deny.toml", commands[0])
        self.assertTrue(any("ls-files" in str(c) and "--others" in str(c) for c in commands))
        for suffix in ([], ["--baseline", ".local/typescript"], ["--self-test"]):
            self.assertIn(["python", "tools/verify-contracts.py", *suffix], commands)
        self.assertIn(["python", "tools/red_team_management.py", "--check", "--verify-retention-origin"], commands)
        self.assertFalse(any("scripts/check-rust.py" in c for c in commands))

    def test_only_fixture_source_hash_can_be_rebound(self):
        original = {"schemaVersion": 1, "baselineCommit": "frozen", "specimens": [{"output": "same"}],
                    "provenance": {"source": "corpus.json", "sha256": "0" * 64, "generatorSha256": "same"}}
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            path = root / PLANNING_FIXTURE
            path.parent.mkdir(parents=True)
            (root / "contracts").mkdir()
            (root / "contracts/manifest.json").write_bytes(b"manifest")
            corpus_original = {"provenance": {"manifestSha256": "1" * 64, "generator": "same"}, "cases": [1]}
            corpus = copy.deepcopy(corpus_original)
            corpus["provenance"]["manifestSha256"] = hashlib.sha256(b"manifest").hexdigest()
            (root / "corpus.json").write_text(json.dumps(corpus))
            current = copy.deepcopy(original)
            current["provenance"]["sha256"] = hashlib.sha256((root / "corpus.json").read_bytes()).hexdigest()
            def baseline(command, **kwargs):
                return json.dumps(original if command[-1].endswith(PLANNING_FIXTURE) else corpus_original).encode()
            with patch("check_phase.subprocess.check_output", side_effect=baseline):
                path.write_text(json.dumps(current))
                verify_planning_fixture(root)
                mutations = [dict(current, specimens=[]), dict(current, schemaVersion=2),
                             dict(current, baselineCommit="changed")]
                for key in ("generatorSha256", "source", "sha256"):
                    changed = copy.deepcopy(current)
                    changed["provenance"][key] = "changed"
                    mutations.append(changed)
                for changed in mutations:
                    path.write_text(json.dumps(changed))
                    with self.assertRaises(ValueError):
                        verify_planning_fixture(root)

                for key in ("cases", "provenance"):
                    changed_corpus = copy.deepcopy(corpus)
                    changed_corpus[key] = [] if key == "cases" else dict(corpus["provenance"], generator="changed")
                    (root / "corpus.json").write_text(json.dumps(changed_corpus))
                    changed_fixture = copy.deepcopy(current)
                    changed_fixture["provenance"]["sha256"] = hashlib.sha256((root / "corpus.json").read_bytes()).hexdigest()
                    path.write_text(json.dumps(changed_fixture))
                    with self.assertRaises(ValueError):
                        verify_planning_fixture(root)

    def test_phase_scope_set_covers_every_management_gate(self):
        self.assertTrue(set(MANAGEMENT_PHASES) <= SCOPE_BOUND_PHASES)

    def test_failed_attempts_cannot_unlock_fifth_exception(self):
        review = {"score": 9, "total": 41, "categories": {key: 8.2 for key in
            ("implementation", "code_quality", "supportability", "architecture", "feature_delivery")},
            "findings": [], "limitations": ["Synthetic remaining deficiency"]}
        for completed in range(4):
            self.assertEqual(gate_decision(review, rubric=True, phase=21, attempt=5,
                                          completed_prior=completed), "rejected")
        self.assertEqual(gate_decision(review, rubric=True, phase=21, attempt=5,
                                      completed_prior=4), "accepted_with_deficiencies")

    def test_provenance_guard_rejects_reformatting_and_other_bytes(self):
        old, new = "0" * 64, "1" * 64
        original = json.dumps({"hash": old, "data": [1]}).encode()
        expected = original.replace(old.encode(), new.encode())
        require_only_hash_replacements(original, expected, [(old, new)])
        for changed in (expected + b"\n", json.dumps(json.loads(expected), indent=2).encode(),
                        expected.replace(b"[1]", b"[2]")):
            with self.assertRaises(ValueError):
                require_only_hash_replacements(original, changed, [(old, new)])

    def test_gateway_proof_allows_only_two_rebound_input_hashes(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            inputs = ["tools/verify-contracts.py", "contracts/manifest.json", "other"]
            for name in inputs:
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(name.encode())
            original = {"inputs": [{"file": name, "sha256": str(i) * 64} for i, name in enumerate(inputs)],
                        "results": [{"accepted": True}]}
            current = copy.deepcopy(original)
            for row in current["inputs"][:2]:
                row["sha256"] = hashlib.sha256((root / row["file"]).read_bytes()).hexdigest()
            output = root / "contracts/native-fixtures/gateway-specimens.json"
            output.parent.mkdir(parents=True)
            with patch("check_phase.subprocess.check_output", return_value=json.dumps(original).encode()):
                output.write_text(json.dumps(current))
                verify_planning_gateway_proof(root)
                for changed in (dict(current, results=[]), dict(current, inputs=list(reversed(current["inputs"])))):
                    output.write_text(json.dumps(changed))
                    with self.assertRaises(ValueError):
                        verify_planning_gateway_proof(root)
                current["inputs"][2]["sha256"] = hashlib.sha256(b"other").hexdigest()
                output.write_text(json.dumps(current))
                with self.assertRaises(ValueError):
                    verify_planning_gateway_proof(root)

    def test_vendor_whitespace_is_excluded_from_all_new_hygiene_checks(self):
        for phase in range(21, 26):
            for command in commands_for(phase):
                if command[:2] == ["git", "diff"] and "--check" in command:
                    self.assertIn(":(exclude)contracts/vendor/red-team-management/0.86.0/*.yaml", command)

    def test_plan_gate_covers_manifest_consumers_and_ci(self):
        commands = commands_for(21)
        for tool in ("model_fixtures.py", "generate_constants.py", "response_defaults.py"):
            self.assertIn(["python", "tools/" + tool, "--check"], commands)
        self.assertIn(["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_ci.py", "-v"], commands)
        self.assertTrue(any("tools/schema-oracle.mts" in c for c in commands))
        self.assertTrue(any("tools/verify-gateway-specimens.mts" in c for c in commands))

    def test_plan_retains_static_closure_without_claiming_current_live_proof(self):
        commands = commands_for(21)
        self.assertNotIn(["python", "tools/native_exports.py", "--check"], commands)
        self.assertFalse(any("tools/live_evidence.py" in c for c in commands))
        self.assertIn(["python", "tools/red_team_management.py", "--check", "--verify-retention-origin"], commands)
        frozen = commands[0]
        for path in ("contracts/native-exports.json", "contracts/native-operations.json",
                     "contracts/response-defaults.json", "contracts/vendor/portkey-access/a27feb9/operations.json",
                     "contracts/vendor/red-team-data-plane/0.93.0/operations.json",
                     "contracts/vendor/red-team-data-plane/0.93.0/model-additions.json"):
            self.assertIn(path, frozen)
        for phase in (23, 24):
            self.assertIn(["python", "tools/native_exports.py", "--check"], commands_for(phase))
        self.assertNotIn(["python", "tools/live_evidence.py", "--check"], commands_for(23))
        self.assertIn(["python", "tools/live_evidence.py", "--check"], commands_for(24))

    def test_every_manifest_reader_is_selected_or_explicitly_explained(self):
        # Source-text discovery intentionally overcounts comments: a new literal
        # manifest consumer must receive an execution or a reviewed explanation.
        readers = set()
        for tree in (ROOT / "tools", ROOT / "scripts"):
            for directory, directories, files in os.walk(tree):
                directories[:] = [name for name in directories
                                  if name not in {"node_modules", "__pycache__", "tests"}
                                  and not name.startswith("test_")]
                for name in files:
                    path = pathlib.Path(directory) / name
                    if (path.suffix in {".py", ".mts", ".mjs"} and not name.startswith("test_")
                            and "contracts/manifest.json" in path.read_text()):
                        readers.add(path.relative_to(ROOT).as_posix())
        selected = {argument for command in commands_for(21) for argument in command}
        exemptions = {
            "tools/bootstrap_reference.py": "Executed before the CI gate; restores verified frozen inputs and installed oracle dependencies.",
            "tools/check_phase.py": "This is the executing phase driver; its hash guards bind current manifest provenance.",
            "tools/extract-operations.mts": "Reads frozen baseline, references and fixtures only; verify-contracts checks their unchanged hashes. New vendor sources are not extraction inputs.",
            "tools/g25_audit.py": "Reads only the frozen baseline commit, validated by verify-contracts; no vendor-source consumption.",
        }
        self.assertFalse(readers - selected - exemptions.keys())
        self.assertTrue(exemptions.keys() <= readers)
        self.assertTrue(all(exemptions.values()))

    def test_scoped_contract_ledgers_keep_their_declared_owning_checks(self):
        status = json.loads((ROOT / 'docs/phase-status.json').read_text())
        active = next(row for row in status['phases'] if row['id'] == status['active_phase'])
        scope = active.get('scope_evidence', []) + [path for chunk in active.get('chunks', []) for path in chunk['scope_evidence']]
        verify_scoped_ledger_owners(scope, commands_for(active['id']))
        verify_scoped_ledger_owners(list(CONTRACT_LEDGER_OWNERS), commands_for(23))
        with self.assertRaisesRegex(ValueError, 'Missing ledger owning check'):
            verify_scoped_ledger_owners(['contracts/native-models.json'], [])
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / 'contracts/new').mkdir(parents=True)
            (root / 'contracts/new/unowned.json').write_text('{}')
            with self.assertRaisesRegex(ValueError, 'No declared ledger owner'):
                verify_scoped_ledger_owners(['contracts/new'], commands_for(22), root)

    def test_cumulative_gate_orders_clean_packages_before_final_receipt_checks(self):
        commands = commands_for(23)
        final = ["python", "tools/red_team_management.py", "--check", "--require-through-phase", "23", "--verify-package-receipts"]
        self.assertEqual(commands[-1], final)
        for compiler in ('1.89.0', '1.99.0'):
            package = ["python", "scripts/check-management-package.py", "--toolchain", compiler,
                       "--output", f"docs/checks/phase-23-package-{compiler}.json"]
            self.assertLess(commands.index(package), commands.index(final))
        selected = ci_commands_for(23)
        for phase in range(23, 26):
            self.assertIn(['python', '-m', 'unittest', 'discover', '-s', 'tools', '-p',
                           'test_management_package.py', '-v'], ci_commands_for(phase))
        self.assertNotIn(final, selected)
        self.assertFalse(any('scripts/check-management-package.py' in c for c in selected))
        self.assertFalse(any('tools/live_evidence.py' in c for c in commands))
        self.assertFalse(any('scripts/check-package.py' in c for c in commands))
        for tool in ('tools/management_fixtures.py', 'tools/management_consumers.py'):
            self.assertIn(['python', tool, '--check'], selected)

    def test_future_milestones_require_explicit_complete_artifacts(self):
        for phase in range(22, 26):
            commands = commands_for(phase)
            self.assertIn(["python", "tools/red_team_management.py", "--check",
                           "--require-through-phase", str(phase)], commands)
            self.assertTrue(any("scripts/check-rust.py" in c for c in commands))
            self.assertFalse(any("scripts/check-access-release.py" in c for c in commands))
            self.assertFalse(any("0.2.2" in value for command in commands for value in command))
            for command in commands:
                for value in command:
                    self.assertNotIn("docs/checks/phase-19-", value)
                    self.assertNotIn("docs/checks/phase-20-", value)

    def test_qualification_has_both_packaged_compilers(self):
        for phase in range(23, 26):
            for compiler in ("1.89.0", "1.99.0"):
                self.assertIn(["python", "scripts/check-management-package.py" if phase == 23 else "scripts/check-package.py", "--toolchain", compiler,
                               "--output", f"docs/checks/phase-{phase}-package-{compiler}.json"], commands_for(phase))

    def test_ci_subsets_cannot_replace_full_reports(self):
        for phase in range(21, 26):
            commands = ci_commands_for(phase)
            self.assertTrue(commands)
            self.assertTrue(all(c in commands_for(phase) for c in commands))
            self.assertFalse(any("scripts/check-rust.py" in c for c in commands))
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            for phase in range(21, 26):
                report = root / f"phase-{phase}.json"
                report.write_text(json.dumps({"phase": phase, "passed": True,
                    "source_sha256": "fixed", "source_unchanged": True,
                    "scope": "ci_contracts_only_not_full_phase_qualification",
                    "checks": [{"command": c, "exit_code": 0} for c in ci_commands_for(phase)]}))
                with self.assertRaisesRegex(ValueError, "incomplete"):
                    check_evidence(root, phase, "fixed", report)

    def test_declared_chunk_scope_must_match_exactly(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            (root / "docs").mkdir()
            for phase in range(21, 26):
                (root / "docs/phase-status.json").write_text(json.dumps({"phases": [{"id": phase,
                    "chunks": [{"id": "scope", "scope_evidence": ["a", "b"], "scope_revisions": [{"from_attempt": 1, "scope_evidence": ["a", "b"]}]}]}]}))
                self.assertTrue(matches_declared_scope(root, phase, "scope", ["b", "a"]))
                self.assertFalse(matches_declared_scope(root, phase, "scope", ["a"]))
                self.assertFalse(matches_declared_scope(root, phase, "scope", ["a", "b", "c"]))
                self.assertFalse(matches_declared_scope(root, phase, "scope", ["a", "a"]))

    def test_cli_rejects_scope_mismatch_before_requesting_review(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            (root / "docs").mkdir()
            (root / "declared").write_text("scope")
            (root / "other").write_text("different scope")
            for phase in range(21, 26):
                (root / "docs/phase-status.json").write_text(json.dumps({"active_phase": phase,
                    "phases": [{"id": phase, "name": "test", "chunks": [
                        {"id": "scope", "scope_evidence": ["declared"], "scope_revisions": [{"from_attempt": 1, "scope_evidence": ["declared"]}]}]}]}))
                with patch("review_phase.fingerprint", return_value="fixed"), \
                        patch("review_phase.check_evidence", return_value=root / "checks.json"), \
                        patch("review_phase.subprocess.run") as reviewer:
                    with self.assertRaisesRegex(ValueError, "differs from its declared"):
                        main(["--phase", str(phase), "--chunk", "scope", "--scope-evidence", "other"], root)
                    reviewer.assert_not_called()

    def complete_record(self, root, score=9, chunk=None):
        suffix = "-chunk-scope" if chunk else ""
        path = root / f"docs/reviews/phase-21{suffix}-attempt-01.json"
        path.parent.mkdir(parents=True, exist_ok=True)
        (root / "docs/phase-status.json").write_text(json.dumps({"active_phase": 21, "phases": [
            {"id": 21, "name": "fixture", "chunks": [{"id": "scope", "scope_evidence": ["declared"], "scope_revisions": [{"from_attempt": 1, "scope_evidence": ["declared"]}]}]}]}))
        review = {"score": score, "summary": "Synthetic test only", "findings": [],
                  "evidence": ["Synthetic fixture"], "limitations": [], "total": 50,
                  "categories": {key: 10 for key in ("implementation", "code_quality",
                      "supportability", "architecture", "feature_delivery")}}
        scope = "chunk" if chunk else "phase"
        path.with_suffix(".prompt.md").write_text(f"Review scope: {scope}.\nChunk scope artifacts: [\"declared\"].")
        path.with_suffix(".cli.json").write_text(json.dumps({"is_error": False, "structured_output": review}))
        path.with_suffix(".checks.json").write_text(json.dumps({"phase": 21, "passed": True,
            "source_sha256": "fixed", "source_unchanged": True,
            "checks": [{"command": c, "exit_code": 0} for c in commands_for(21)]}))
        record = {"phase": 21, "attempt": 1, "chunk": chunk, "reviewer": "Claude Code",
            "rubric": "five-category-v1", "decision": "accepted", "source_sha256": "fixed",
            "source_unchanged": True, "gate_passed": True, "review": review,
            "review_scope": scope, "scope_evidence": ["declared"], "scope_revisions": [{"from_attempt": 1, "scope_evidence": ["declared"]}]}
        for key, suffix in (("prompt_sha256", ".prompt.md"), ("cli_output_sha256", ".cli.json"),
                            ("checks_sha256", ".checks.json")):
            record[key] = sha256(path.with_suffix(suffix))
        path.write_text(json.dumps(record))
        return path, record

    def test_plan_score_floor_cannot_be_bypassed_by_category_total(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            path, record = self.complete_record(root)
            self.assertTrue(verify_record(root, path, 21, "fixed"))
            path, record = self.complete_record(root, score=8.5)
            self.assertFalse(verify_record(root, path, 21, "fixed"))
            record["decision"] = "rejected"
            path.write_text(json.dumps(record))
            self.assertFalse(verify_record(root, path, 21, "fixed"))
            self.assertEqual(gate_decision(record["review"], rubric=True, phase=21), "rejected")

    def test_chunk_record_rejects_scope_tampering_even_with_rehashed_prompt(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            path, record = self.complete_record(root, chunk="scope")
            self.assertTrue(verify_record(root, path, 21, "fixed", "scope"))
            for field, value in (("review_scope", "phase"), ("scope_evidence", ["other"])):
                changed = dict(record, **{field: value})
                path.write_text(json.dumps(changed))
                self.assertFalse(verify_record(root, path, 21, "fixed", "scope"))
            path.with_suffix(".prompt.md").write_text('Review scope: chunk.\nChunk scope artifacts: ["other"].')
            record["prompt_sha256"] = sha256(path.with_suffix(".prompt.md"))
            path.write_text(json.dumps(record))
            self.assertFalse(verify_record(root, path, 21, "fixed", "scope"))

    def test_failed_invocation_consumes_attempt_without_dead_end(self):
        for reason in ("timeout", "process_error", "invalid_output", "launch_error"):
            with self.subTest(reason=reason), tempfile.TemporaryDirectory() as temporary:
                root = pathlib.Path(temporary)
                path, record = self.complete_record(root)
                checks = path.with_suffix(".checks.json").read_bytes()
                for item in path.parent.iterdir():
                    item.unlink()
                (root / "checks.json").write_bytes(checks)
                failure = {"timeout": subprocess.TimeoutExpired("claude", 1),
                           "launch_error": OSError("test launch error")}.get(reason)
                outcome = subprocess.CompletedProcess(["claude"], 1 if reason == "process_error" else 0,
                                                      "invalid JSON", "")
                with patch("review_phase.fingerprint", return_value="fixed"), \
                        patch("review_phase.check_evidence", return_value=root / "checks.json"), \
                        patch("review_phase.subprocess.run", side_effect=failure, return_value=outcome):
                    with self.assertRaises((RuntimeError, ValueError, OSError)):
                        main(["--phase", "21"], root)
                self.assertFalse(path.exists())
                failed = json.loads(path.with_suffix(".failed.json").read_text())
                self.assertEqual(failed["reason"], reason)
                self.assertEqual(verify_attempt_sequence(root, 21, 2), 0)
                path.with_suffix(".prompt.md").write_text("tampered")
                with self.assertRaisesRegex(ValueError, "evidence has changed"):
                    self.assertEqual(verify_attempt_sequence(root, 21, 2), 0)

    def test_scope_revision_history_preserves_old_reviews_and_rejects_rewrites(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            path, record = self.complete_record(root, chunk="scope")
            status_path = root / "docs/phase-status.json"
            status = json.loads(status_path.read_text())
            chunk = status["phases"][0]["chunks"][0]
            chunk["scope_evidence"] = ["declared", "added"]
            chunk["scope_revisions"].append({"from_attempt": 2, "scope_evidence": ["declared", "added"]})
            status_path.write_text(json.dumps(status))
            self.assertTrue(verify_record(root, path, 21, "fixed", "scope"))
            self.assertEqual(verify_attempt_sequence(root, 21, 2, "scope"), 1)
            chunk["scope_revisions"][0]["scope_evidence"] = ["added"]
            status_path.write_text(json.dumps(status))
            self.assertFalse(verify_record(root, path, 21, "fixed", "scope"))
            with self.assertRaisesRegex(ValueError, "scope differs"):
                verify_attempt_sequence(root, 21, 2, "scope")
            chunk["scope_evidence"] = ["declared"]
            status_path.write_text(json.dumps(status))
            with self.assertRaisesRegex(ValueError, "scope revisions"):
                matches_declared_scope(root, 21, "scope", ["declared"])

    def test_exhausted_failure_has_no_automatic_budget_reset(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            self.complete_record(root, chunk="scope")
            failure = root / "docs/reviews/phase-21-chunk-scope-attempt-05.failed.json"
            failure.write_text('{"status":"invocation_failed","reason":"timeout"}')
            preserved = failure.read_bytes()
            with self.assertRaisesRegex(ValueError, "at most five attempts"):
                main(["--phase", "21", "--chunk", "scope", "--attempt", "6"], root)
            self.assertEqual(failure.read_bytes(), preserved)
            self.assertFalse(list(failure.parent.glob("*attempt-06*")))
        guidance = (ROOT / "docs/PHASES.md").read_text()
        self.assertIn("explicit maintainer authorization", guidance)
        self.assertIn("No automatic budget reset", guidance)

    def test_missing_prior_artifacts_are_sanitized_gate_errors(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            for suffix in (".json", ".prompt.md", ".cli.json"):
                path, record = self.complete_record(root)
                path.with_suffix(suffix).unlink()
                with self.assertRaisesRegex(ValueError, "Previous review evidence has changed"):
                    verify_attempt_sequence(root, 21, 2)

    def test_current_dependency_audit_and_committed_hygiene_are_required(self):
        commands = commands_for(21)
        self.assertIn(["python", "scripts/check-dependencies.py", "--output", "docs/checks/phase-21-dependencies.json"], commands)
        self.assertTrue(any(c[:3] == ["git", "diff", "--check"] and MANAGEMENT_BASELINE + "..HEAD" in c for c in commands))

    def test_phases_registered_with_nonempty_scopes_and_order(self):
        phases = {p["id"]: p for p in json.loads((ROOT / "docs/phase-status.json").read_text())["phases"]}
        for phase in range(21, 26):
            self.assertTrue(phases[phase]["chunks"])
            for chunk in phases[phase]["chunks"]:
                self.assertTrue(chunk["scope_evidence"])
            if phase < 25:
                pending = {n: dict(p, status="pending") for n, p in phases.items() if 21 <= n <= 25}
                self.assertEqual(next_phase(pending, phase), phase + 1)
        self.assertTrue(phases[25]["final_in_sequence"])


if __name__ == "__main__":
    unittest.main()
