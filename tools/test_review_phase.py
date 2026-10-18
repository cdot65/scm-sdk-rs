"""Tests for the enforced review gate; no model calls or credentials."""

import copy
import contextlib
import hashlib
import io
import json
import pathlib
import subprocess
import tempfile
import unittest

from unittest.mock import patch

from check_phase import commands_for, require_nonempty_tests
from review_phase import fingerprint, main, next_phase, sha256, validate_predecessors, validate_review, verify_record


class ReviewGateTests(unittest.TestCase):
    def test_historical_phase_command_contracts_are_unchanged(self):
        historical = {str(i): commands_for(i) for i in range(1, 9)}
        encoded = json.dumps(historical, sort_keys=True, separators=(",", ":")).encode()
        self.assertEqual(hashlib.sha256(encoded).hexdigest(),
                         "54a4c010d73c17187300219a839700d09036b7849b5416f359d4c577d1ac8dfc")

    def test_noncontiguous_maintenance_successors(self):
        phases = {91: {}, 8: {}, 90: {}}
        self.assertEqual(next_phase(phases, 8), 90)
        self.assertEqual(next_phase(phases, 90), 91)
        self.assertIsNone(next_phase(phases, 91))

    def test_successors_do_not_reopen_completed_independent_maintenance(self):
        phases = {10: {}, 11: {}, 12: {"final_in_sequence": True},
                  90: {"status": "complete"}, 91: {"status": "complete"},
                  92: {"status": "in_progress"}}
        self.assertEqual(next_phase(phases, 10), 11)
        self.assertEqual(next_phase(phases, 11), 12)
        self.assertIsNone(next_phase(phases, 12))
        self.assertIsNone(next_phase({8: {}, 90: {"status": "complete"}}, 8))

    def test_discovery_rejects_missing_and_empty_suites(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            with patch("check_phase.ROOT", root):
                for name, content in [("test_missing_suite.py", None), ("test_empty_suite.py", "")]:
                    if content is not None:
                        (root / name).write_text(content)
                    with self.assertRaisesRegex(ValueError, "empty unittest"):
                        require_nonempty_tests(["python", "-m", "unittest", "discover", "-s", ".", "-p", name])

    def test_discovery_accepts_an_executably_populated_suite(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            (root / "test_populated_suite.py").write_text(
                "import unittest\nclass Sample(unittest.TestCase):\n    def test_present(self): pass\n")
            with patch("check_phase.ROOT", root):
                require_nonempty_tests(["python", "-m", "unittest", "discover", "-s", ".", "-p", "test_populated_suite.py"])

    def setUp(self):
        self.review = {"score": 9, "summary": "Verified phase", "findings": [],
                       "evidence": ["Independent contract checks passed"], "limitations": []}

    def test_below_threshold_is_rejected(self):
        self.review["score"] = 8.9
        self.assertFalse(validate_review(self.review))

    def test_high_finding_blocks_even_with_ten(self):
        self.review["score"] = 10
        self.review["findings"] = [{"severity": "high", "path": "src/lib.rs", "line": 1,
                                   "description": "Credential leak", "required_fix": "Redact"}]
        self.assertFalse(validate_review(self.review))

    def test_verified_nine_passes(self):
        self.assertTrue(validate_review(self.review))

    def test_missing_evidence_rejected(self):
        self.review["evidence"] = []
        with self.assertRaises(ValueError):
            validate_review(self.review)

    def test_blank_evidence_rejected(self):
        self.review["evidence"] = ["   "]
        with self.assertRaises(ValueError):
            validate_review(self.review)

    def test_bad_scores_rejected(self):
        for score in (True, None, "9", 0, 11, float('nan')):
            candidate = copy.deepcopy(self.review)
            candidate["score"] = score
            with self.subTest(score=score), self.assertRaises(ValueError):
                validate_review(candidate)

    def test_unknown_fields_rejected(self):
        self.review["claimed_pass"] = True
        with self.assertRaises(ValueError):
            validate_review(self.review)

    def test_source_fingerprint_and_bookkeeping(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            subprocess.run(["git", "init", "-q"], cwd=root, check=True)
            source = root / "source.rs"
            source.write_text("source version one")
            original = fingerprint(root)
            (root / "docs/reviews").mkdir(parents=True)
            (root / "docs/reviews/review.json").write_text("review bookkeeping")
            (root / "docs/phase-status.json").write_text("phase bookkeeping")
            (root / "docs/checks").mkdir()
            (root / "docs/checks/check.json").write_text("check bookkeeping")
            self.assertEqual(original, fingerprint(root))
            source.write_text("source version two")
            self.assertNotEqual(original, fingerprint(root))

    def test_symlink_input_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            subprocess.run(["git", "init", "-q"], cwd=root, check=True)
            (root / "file").write_text("regular")
            (root / "link").symlink_to("file")
            with self.assertRaises(ValueError):
                fingerprint(root)


class ReviewRecordTests(unittest.TestCase):
    """Synthetic records test rejection paths; they are never real phase reviews."""

    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = pathlib.Path(self.temporary.name)
        subprocess.run(["git", "init", "-q"], cwd=self.root, check=True)
        (self.root / ".gitignore").write_text("/.local/\n")
        (self.root / "source.rs").write_text("source")
        (self.root / "docs/reviews").mkdir(parents=True)
        (self.root / "docs/checks").mkdir()
        self.status = {"active_phase": 1, "phases": [
            {"id": 1, "name": "Contracts", "status": "in_progress", "reviews": []},
            {"id": 2, "name": "Foundation", "status": "pending", "reviews": []},
        ]}
        self.status_path = self.root / "docs/phase-status.json"
        self.status_path.write_text(json.dumps(self.status))
        self.source_hash = fingerprint(self.root)
        self.checks_path = self.root / "docs/checks/phase-01.json"
        self.checks = {"phase": 1, "source_sha256": self.source_hash, "source_unchanged": True,
                       "passed": True, "checks": [{"command": command, "exit_code": 0}
                                                   for command in commands_for(1)]}
        self.checks_path.write_text(json.dumps(self.checks))
        self.path = self.root / "docs/reviews/phase-01-attempt-01.json"
        review = {"score": 9, "summary": "Synthetic unit-test review", "findings": [],
                  "evidence": ["Synthetic unit-test evidence"], "limitations": []}
        self.path.with_suffix(".prompt.md").write_text("Synthetic unit-test prompt")
        self.path.with_suffix(".cli.json").write_text(json.dumps({
            "is_error": False, "structured_output": review}))
        self.record = {"reviewer": "Claude Code", "phase": 1, "attempt": 1,
                       "source_sha256": self.source_hash, "source_unchanged": True,
                       "gate_passed": True, "review": review,
                       "checks_sha256": sha256(self.checks_path),
                       "prompt_sha256": sha256(self.path.with_suffix(".prompt.md")),
                       "cli_output_sha256": sha256(self.path.with_suffix(".cli.json"))}
        self.write_record()

    def write_record(self):
        self.path.write_text(json.dumps(self.record))

    def verify(self):
        return verify_record(self.root, self.path, 1, fingerprint(self.root))

    def test_complete_record_verifies(self):
        self.assertTrue(self.verify())

    def test_source_change_rejects_review(self):
        (self.root / "source.rs").write_text("changed")
        self.assertFalse(self.verify())

    def test_cli_output_tampering_rejects_review(self):
        self.path.with_suffix(".cli.json").write_text("{}")
        self.assertFalse(self.verify())

    def test_cli_payload_must_match_score(self):
        envelope = {"is_error": False, "structured_output": dict(self.record["review"], score=8)}
        self.path.with_suffix(".cli.json").write_text(json.dumps(envelope))
        self.record["cli_output_sha256"] = sha256(self.path.with_suffix(".cli.json"))
        self.write_record()
        self.assertFalse(self.verify())

    def test_prompt_tampering_rejects_review(self):
        self.path.with_suffix(".prompt.md").write_text("changed")
        self.assertFalse(self.verify())

    def test_stale_check_source_rejected(self):
        self.checks["source_sha256"] = "stale"
        self.checks_path.write_text(json.dumps(self.checks))
        with self.assertRaises(ValueError):
            self.verify()

    def test_incomplete_check_set_rejected(self):
        self.checks["checks"].pop()
        self.checks_path.write_text(json.dumps(self.checks))
        with self.assertRaises(ValueError):
            self.verify()

    def test_failed_check_rejected_even_with_pass_flag(self):
        self.checks["checks"][0]["exit_code"] = 1
        self.checks_path.write_text(json.dumps(self.checks))
        with self.assertRaises(ValueError):
            self.verify()

    def test_lower_score_cannot_advance(self):
        self.record["review"]["score"] = 8
        self.write_record()
        self.assertFalse(self.verify())

    def test_verify_and_advance_updates_active_phase(self):
        with contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(0, main(["--phase", "1", "--verify", str(self.path), "--advance"], self.root))
        status = json.loads(self.status_path.read_text())
        self.assertEqual(status["active_phase"], 2)
        self.assertEqual(status["phases"][0]["status"], "complete")
        self.assertEqual(status["phases"][0]["accepted_score"], 9)
        self.assertEqual(status["phases"][0]["milestones"]["claude_review"]["status"], "accepted")
        self.assertEqual(status["phases"][0]["milestones"]["independent_verification"]["source_sha256"], self.source_hash)
        self.assertEqual(status["phases"][1]["status"], "in_progress")

    def test_manual_predecessor_completion_is_rejected(self):
        self.status["phases"][0]["status"] = "complete"
        with self.assertRaises(ValueError):
            validate_predecessors(self.root, self.status, 2)

    def test_advance_requires_review_path(self):
        with self.assertRaises(ValueError):
            main(["--phase", "1", "--advance"], self.root)

    def test_moved_review_path_rejected(self):
        other = self.root / "docs/reviews/other.json"
        other.write_text(self.path.read_text())
        self.assertFalse(verify_record(self.root, other, 1, self.source_hash))

    def test_review_request_preserves_prompt_and_real_cli_result(self):
        for p in self.path.parent.iterdir():
            p.unlink()
        envelope = self.path.with_suffix(".cli.json")
        response = json.dumps({"is_error": False, "structured_output": self.record["review"]})
        actual_run = subprocess.run

        def mocked_run(command, **kwargs):
            if command[0] == "claude":
                self.assertIn("--restricted", command)
                self.assertIn("Read,Glob,Grep", command)
                return subprocess.CompletedProcess(command, 0, response, "")
            return actual_run(command, **kwargs)

        with patch("review_phase.subprocess.run", side_effect=mocked_run), contextlib.redirect_stdout(io.StringIO()):
            self.assertEqual(0, main(["--phase", "1"], self.root))
        self.assertTrue(envelope.is_file())
        self.assertTrue(self.path.with_suffix(".prompt.md").is_file())
        self.assertTrue(self.verify())

    def test_authentication_failure_cannot_create_accepted_review(self):
        for p in self.path.parent.iterdir():
            p.unlink()
        actual_run = subprocess.run

        def mocked_run(command, **kwargs):
            if command[0] == "claude":
                return subprocess.CompletedProcess(command, 1, '{"is_error":true}', "")
            return actual_run(command, **kwargs)

        with patch("review_phase.subprocess.run", side_effect=mocked_run), self.assertRaises(RuntimeError):
            main(["--phase", "1"], self.root)
        self.assertFalse(self.path.exists())
        self.assertFalse(self.path.with_suffix(".cli.json").exists())
        self.assertTrue(self.path.with_suffix(".prompt.md").is_file())
        self.assertEqual(json.loads(self.status_path.read_text()), self.status)

    def test_timeout_preserves_private_partial_diagnostics_without_accepted_review(self):
        for p in self.path.parent.iterdir():
            p.unlink()
        actual_run = subprocess.run

        def mocked_run(command, **kwargs):
            if command[0] == "claude":
                raise subprocess.TimeoutExpired(command, 1800, b"partial output", b"private diagnostic")
            return actual_run(command, **kwargs)

        with patch("review_phase.subprocess.run", side_effect=mocked_run):
            with self.assertRaisesRegex(RuntimeError, "private diagnostics preserved"):
                main(["--phase", "1"], self.root)
        self.assertFalse(self.path.exists())
        self.assertFalse(self.path.with_suffix(".cli.json").exists())
        self.assertTrue(self.path.with_suffix(".prompt.md").is_file())
        for suffix, content in (("json", b"partial output"), ("stderr.txt", b"private diagnostic")):
            path = self.root / ".local/reviews" / f"phase-01-attempt-01.{suffix}"
            self.assertEqual(path.read_bytes(), content)
            self.assertEqual(path.stat().st_mode & 0o777, 0o600)
        self.assertEqual(json.loads(self.status_path.read_text()), self.status)


if __name__ == "__main__":
    unittest.main()
