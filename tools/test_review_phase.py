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

    def test_phase19_gate_retains_privacy_ci_and_all_rust_combinations(self):
        commands = commands_for(19)
        self.assertIn(["python", "scripts/check-private-material.py", "--check",
                       "docs/checks/phase-19-private-material.json"], commands)
        self.assertIn(["python", "-m", "unittest", "discover", "-s", "tools", "-p",
                       "test_ci.py", "-v"], commands)
        expected = {("1.89.0", "red-team"), ("1.99.0", "red-team"),
                    ("1.99.0", "json-unification"),
                    ("1.89.0", "red-team-json-unification"),
                    ("1.99.0", "red-team-json-unification")}
        rust_commands = [command for command in commands if "scripts/check-rust.py" in command]
        self.assertEqual(len(rust_commands), len(expected))
        self.assertEqual({(command[command.index("--toolchain") + 1],
                           command[command.index("--feature-set") + 1])
                          for command in rust_commands}, expected)
        for compiler, features in expected:
            self.assertIn(["python", "scripts/check-rust.py", "--toolchain", compiler,
                           "--feature-set", features, "--output",
                           f"docs/checks/phase-19-rust-{compiler}-{features}.json"], commands)

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


class FiveCategoryReviewTests(unittest.TestCase):
    def review(self, values=(10, 9, 9, 9, 9)):
        from review_phase import CATEGORIES
        return {"score": 9, "summary": "Synthetic rubric review", "findings": [],
                "evidence": ["Synthetic fixture evidence"], "limitations": [],
                "categories": dict(zip(CATEGORIES, values)), "total": sum(values)}

    def test_phase_and_chunk_thresholds(self):
        self.assertTrue(validate_review(self.review(), rubric=True))
        review = self.review((9, 9, 9, 9, 9))
        self.assertFalse(validate_review(review, rubric=True))
        self.assertTrue(validate_review(review, rubric=True, chunk=True))
        self.assertTrue(validate_review(self.review((9, 9, 8, 8, 8)), rubric=True, chunk=True))
        self.assertFalse(validate_review(self.review((9, 8, 8, 8, 8)), rubric=True, chunk=True))
        self.assertFalse(validate_review(self.review((10, 10, 10, 10, 7)), rubric=True, chunk=True))
        self.assertFalse(validate_review(self.review((10, 10, 10, 10, 7)), rubric=True))

    def test_fifth_exception_is_explicit_and_requires_deficiency(self):
        from review_phase import gate_decision
        review = self.review((9, 8, 8, 8, 8))
        self.assertFalse(validate_review(review, rubric=True, attempt=5))
        review['limitations'] = ['Delivery documentation needs improvement']
        for attempt in range(1, 5):
            self.assertFalse(validate_review(review, rubric=True, attempt=attempt))
        self.assertTrue(validate_review(review, rubric=True, attempt=5, completed_prior=4))
        self.assertEqual(gate_decision(review, rubric=True, attempt=5, completed_prior=4), 'accepted_with_deficiencies')
        catastrophic = self.review((10, 10, 10, 10, 1))
        catastrophic['limitations'] = ['One category is unacceptable']
        self.assertFalse(validate_review(catastrophic, rubric=True, attempt=5))
        self.assertFalse(validate_review(catastrophic, rubric=True, chunk=True, attempt=5))
        review = self.review((8, 8, 8, 8, 8))
        review['limitations'] = ['Below the exception threshold']
        self.assertFalse(validate_review(review, rubric=True, attempt=5))
        for attempt in (0, 6, True):
            with self.assertRaises(ValueError):
                validate_review(review, rubric=True, attempt=attempt)

    def test_categories_cannot_be_fabricated(self):
        for value in (True, '10', -1, 11, float('nan'), float('inf')):
            review = self.review()
            review['categories']['implementation'] = value
            with self.subTest(value=value), self.assertRaises(ValueError):
                validate_review(review, rubric=True)
        for total in (True, 50, float('nan')):
            review = self.review()
            review['total'] = total
            with self.subTest(total=total), self.assertRaises(ValueError):
                validate_review(review, rubric=True)
        review = self.review()
        del review['categories']['architecture']
        with self.assertRaises(ValueError):
            validate_review(review, rubric=True)

    def test_blockers_still_block_fifth_attempt(self):
        review = self.review((10, 10, 10, 10, 10))
        review['findings'] = [{'severity': 'high', 'path': 'file.rs', 'line': 1,
                               'description': 'Credential exposure', 'required_fix': 'Redact'}]
        self.assertFalse(validate_review(review, rubric=True, attempt=5))

    def test_historical_policy_and_chunk_names(self):
        from review_phase import uses_rubric, review_destination
        for phase in (*range(1, 19), 90, 91, 92):
            self.assertFalse(uses_rubric(phase))
        for phase in (19, 20, 93):
            self.assertTrue(uses_rubric(phase))
        self.assertEqual(review_destination(pathlib.Path('/repo'), 19, 1, 'typed-models').name,
                         'phase-19-chunk-typed-models-attempt-01.json')
        for name in ('../escape', '', 'UPPER', 'a/b'):
            with self.assertRaises(ValueError):
                review_destination(pathlib.Path('/repo'), 19, 1, name)

    def test_live_new_rubric_record_and_snapshot(self):
        # Exercise actual orchestration without invoking a model; fixture output is
        # never written into the real repository's evidence directories.
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            subprocess.run(['git', 'init', '-q'], cwd=root, check=True)
            (root / '.gitignore').write_text('/.local/\n')
            (root / 'source.rs').write_text('source')
            (root / 'docs/checks').mkdir(parents=True)
            status = {'active_phase': 19, 'phases': [
                {'id': 19, 'name': 'Compatibility', 'status': 'in_progress', 'reviews': [],
                 'chunks': [{'id': 'example', 'status': 'pending'}]}]}
            (root / 'docs/phase-status.json').write_text(json.dumps(status))
            source = fingerprint(root)
            checks = root / 'docs/checks/phase-19.json'
            checks.write_text(json.dumps({'phase': 19, 'source_sha256': source,
                'source_unchanged': True, 'passed': True,
                'checks': [{'command': ['synthetic-check'], 'exit_code': 0}]}))
            review = self.review()
            actual_run = subprocess.run

            def mock_run(command, **kwargs):
                if command[0] == 'claude':
                    self.assertIn('categories', json.loads(command[-1])['required'])
                    self.assertIn('five-category rubric', kwargs['input'])
                    return subprocess.CompletedProcess(command, 0,
                        json.dumps({'is_error': False, 'structured_output': review}), '')
                return actual_run(command, **kwargs)

            with patch('check_phase.commands_for', return_value=[['synthetic-check']]), \
                    patch('review_phase.subprocess.run', side_effect=mock_run), \
                    contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(main(['--phase', '19', '--chunk', 'example',
                                       '--scope-evidence', 'source.rs'], root), 0)
                self.assertEqual(main(['--phase', '19'], root), 0)
                path = root / 'docs/reviews/phase-19-attempt-01.json'
                self.assertTrue(verify_record(root, path, 19, source))
                # Current source must satisfy today's full command policy.
                with patch('check_phase.commands_for', return_value=[['new-current-check']]):
                    with self.assertRaises(ValueError):
                        verify_record(root, path, 19, source)
                # Historical chunk/follow-up and predecessor evidence retain
                # their actual command snapshot across deduplication/additions.
                source_file = root / 'source.rs'
                source_file.write_text('next phase source')
                chunk_path = root / 'docs/reviews/phase-19-chunk-example-attempt-01.json'
                for historical_path, chunk in ((path, None), (chunk_path, 'example')):
                    for current_commands in ([['deduplicated-check']],
                                             [['synthetic-check'], ['new-release-check']]):
                        with patch('check_phase.commands_for', return_value=current_commands):
                            self.assertTrue(verify_record(root, historical_path, 19, source, chunk))
                    snapshot = historical_path.with_suffix('.checks.json')
                    original = snapshot.read_bytes()
                    tampered = json.loads(original)
                    tampered['checks'] = []
                    snapshot.write_text(json.dumps(tampered))
                    self.assertFalse(verify_record(root, historical_path, 19, source, chunk))
                    snapshot.write_bytes(original)
                # Direct qualification never gains historical-snapshot trust.
                from review_phase import check_evidence
                with patch('check_phase.commands_for', return_value=[['new-current-check']]):
                    with self.assertRaises(ValueError):
                        check_evidence(root, 19, source, path.with_suffix('.checks.json'))
                source_file.write_text('source')
                # A later qualification run cannot erase this attempt's checks.
                checks.write_text('{}')
                self.assertTrue(verify_record(root, path, 19, source))
                self.assertEqual(main(['--phase', '19', '--verify', str(path), '--advance'], root), 0)
                final = json.loads((root / 'docs/phase-status.json').read_text())
                self.assertEqual(final['phases'][0]['accepted_total'], 46)
                self.assertEqual(final['phases'][0]['review_decision'], 'accepted')
                self.assertEqual(final['phases'][0]['milestones']['independent_verification']['evidence'],
                                 [str(path.with_suffix('.checks.json').relative_to(root))])

    @contextlib.contextmanager
    def chunk_repository(self):
        """Isolated CLI fixture: synthetic evidence never reaches project records."""
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary) / 'repository'
            root.mkdir()
            subprocess.run(['git', 'init', '-q'], cwd=root, check=True)
            (root / '.gitignore').write_text('/.local/\n')
            (root / 'source.rs').write_text('source')
            (root / 'docs/checks').mkdir(parents=True)
            status = {'active_phase': 19, 'phases': [
                {'id': 19, 'name': 'Compatibility', 'status': 'in_progress', 'reviews': []}]}
            (root / 'docs/phase-status.json').write_text(json.dumps(status))
            source = fingerprint(root)
            (root / 'docs/checks/phase-19.json').write_text(json.dumps({
                'phase': 19, 'source_sha256': source, 'source_unchanged': True,
                'passed': True, 'checks': [{'command': ['synthetic-check'], 'exit_code': 0}]}))
            with patch('check_phase.commands_for', return_value=[['synthetic-check']]), \
                    contextlib.redirect_stdout(io.StringIO()):
                yield root, source, status

    def test_chunk_orchestration_accumulates_scope_and_binds_record(self):
        with self.chunk_repository() as (root, source, status):
            review = self.review((9, 9, 9, 8, 8))
            actual_run = subprocess.run
            calls = []

            def mock_run(command, **kwargs):
                if command[0] == 'claude':
                    calls.append(command)
                    self.assertIn('chunk regression-and-governance', kwargs['input'])
                    self.assertIn('Chunk scope artifacts: ["source.rs", ".gitignore"]', kwargs['input'])
                    return subprocess.CompletedProcess(command, 0,
                        json.dumps({'is_error': False, 'structured_output': review}), '')
                return actual_run(command, **kwargs)

            with patch('review_phase.subprocess.run', side_effect=mock_run):
                self.assertEqual(main(['--phase', '19', '--chunk', 'regression-and-governance',
                                       '--scope-evidence', 'source.rs', '--scope-evidence', '.gitignore'], root), 0)
            self.assertEqual(len(calls), 1)
            path = root / 'docs/reviews/phase-19-chunk-regression-and-governance-attempt-01.json'
            record = json.loads(path.read_text())
            self.assertEqual(record['chunk'], 'regression-and-governance')
            self.assertEqual(record['rubric'], 'five-category-v1')
            self.assertEqual(record['decision'], 'accepted')
            self.assertEqual(record['review']['total'], 43)
            self.assertTrue(verify_record(root, path, 19, source, 'regression-and-governance'))
            self.assertFalse(verify_record(root, path, 19, source))
            self.assertFalse(verify_record(root, path, 19, source, 'another-chunk'))
            self.assertEqual(main(['--phase', '19', '--chunk', 'regression-and-governance',
                                   '--verify', str(path)], root), 0)
            self.assertEqual(json.loads((root / 'docs/phase-status.json').read_text()), status)
            for key, wrong in (('chunk', 'another-chunk'), ('rubric', 'unknown'),
                               ('decision', 'accepted_with_deficiencies')):
                with self.subTest(key=key):
                    path.write_text(json.dumps(dict(record, **{key: wrong})))
                    self.assertFalse(verify_record(root, path, 19, source, 'regression-and-governance'))
            path.write_text(json.dumps(record))

    def test_chunk_orchestration_rejects_advance_and_missing_scope(self):
        with self.chunk_repository() as (root, _, status):
            for suffix, message in ((['--advance'], 'cannot advance'),
                                    ([], 'require --scope-evidence')):
                with self.subTest(suffix=suffix), self.assertRaisesRegex(ValueError, message):
                    main(['--phase', '19', '--chunk', 'example', *suffix], root)
            self.assertEqual(json.loads((root / 'docs/phase-status.json').read_text()), status)
            self.assertFalse(list((root / 'docs').glob('reviews/*.cli.json')))

    def test_chunk_orchestration_rejects_historical_rubric(self):
        with self.chunk_repository() as (root, _, status):
            status['active_phase'] = 18
            status['phases'][0]['id'] = 18
            (root / 'docs/phase-status.json').write_text(json.dumps(status))
            with self.assertRaisesRegex(ValueError, 'require the new rubric'):
                main(['--phase', '18', '--chunk', 'example', '--scope-evidence', 'source.rs'], root)

    def test_chunk_scope_must_resolve_to_existing_repository_path(self):
        with self.chunk_repository() as (root, _, status):
            external = root.parent / 'external.rs'
            external.write_text('outside repository')
            # Ignored link avoids the separate source-fingerprint symlink guard,
            # allowing this test to exercise resolved scope-path validation itself.
            (root / '.local').mkdir()
            (root / '.local/external.rs').symlink_to(external)
            for scoped_path in ('../external.rs', str(external), '.local/external.rs', 'missing.rs'):
                with self.subTest(path=scoped_path), self.assertRaisesRegex(
                        ValueError, 'must exist inside the repository'):
                    main(['--phase', '19', '--chunk', 'example',
                          '--scope-evidence', scoped_path], root)
            self.assertEqual(json.loads((root / 'docs/phase-status.json').read_text()), status)
            self.assertFalse(list((root / 'docs/reviews').glob('*.checks.json')))
            self.assertFalse(list((root / 'docs/reviews').glob('*.prompt.md')))
            self.assertFalse(list((root / 'docs/reviews').glob('*.cli.json')))

    def run_synthetic_review(self, root, *, chunk=None, attempt=1, values=(10, 9, 9, 9, 9), limitations=None):
        """Invoke the real CLI orchestration with an explicitly synthetic reviewer."""
        actual_run = subprocess.run
        review = self.review(values)
        if limitations is not None:
            review["limitations"] = limitations

        def mock_run(command, **kwargs):
            if command[0] == 'claude':
                return subprocess.CompletedProcess(command, 0,
                    json.dumps({'is_error': False, 'structured_output': review}), '')
            return actual_run(command, **kwargs)

        arguments = ['--phase', '19', '--attempt', str(attempt)]
        if chunk:
            arguments += ['--chunk', chunk, '--scope-evidence', 'source.rs']
        with patch('review_phase.subprocess.run', side_effect=mock_run):
            outcome = main(arguments, root)
        from review_phase import review_destination
        return outcome, review_destination(root, 19, attempt, chunk)

    def declare_chunks(self, root, *names):
        status_path = root / 'docs/phase-status.json'
        status = json.loads(status_path.read_text())
        status['phases'][0]['chunks'] = [{'id': name, 'status': 'pending'} for name in names]
        status_path.write_text(json.dumps(status))
        return status_path.read_bytes()

    def test_phase_advancement_requires_every_declared_chunk(self):
        with self.chunk_repository() as (root, _, _):
            original = self.declare_chunks(root, 'first', 'missing')
            self.run_synthetic_review(root, chunk='first')
            _, phase_path = self.run_synthetic_review(root)
            with self.assertRaisesRegex(ValueError, 'Missing accepted chunk review: missing'):
                main(['--phase', '19', '--verify', str(phase_path), '--advance'], root)
            self.assertEqual((root / 'docs/phase-status.json').read_bytes(), original)

    def test_phase_advancement_records_all_verified_chunk_scores(self):
        with self.chunk_repository() as (root, source, _):
            original = self.declare_chunks(root, 'first', 'second')
            _, first_path = self.run_synthetic_review(root, chunk='first', values=(9, 9, 8, 8, 8))
            self.run_synthetic_review(root, chunk='second')
            # Concurrent chunk workers leave phase bookkeeping untouched.
            self.assertEqual((root / 'docs/phase-status.json').read_bytes(), original)
            _, phase_path = self.run_synthetic_review(root)
            self.assertEqual(main(['--phase', '19', '--verify', str(phase_path), '--advance'], root), 0)
            phase = json.loads((root / 'docs/phase-status.json').read_text())['phases'][0]
            self.assertEqual(phase['status'], 'complete')
            first, second = phase['chunks']
            self.assertEqual(first['status'], 'accepted')
            self.assertEqual(first['accepted_total'], 42)
            self.assertEqual(first['accepted_categories'], self.review((9, 9, 8, 8, 8))['categories'])
            self.assertEqual(first['accepted_review'], str(first_path.relative_to(root)))
            self.assertEqual(first['accepted_source_sha256'], source)
            self.assertEqual(second['accepted_total'], 46)

    def test_phase_advancement_never_selects_earlier_passing_chunk(self):
        with self.chunk_repository() as (root, _, _):
            original = self.declare_chunks(root, 'example')
            self.run_synthetic_review(root, chunk='example')
            outcome, _ = self.run_synthetic_review(root, chunk='example', attempt=2,
                                                   values=(8, 8, 8, 8, 8))
            self.assertEqual(outcome, 1)
            _, phase_path = self.run_synthetic_review(root)
            with self.assertRaisesRegex(ValueError, 'Latest chunk review is rejected, stale, or invalid'):
                main(['--phase', '19', '--verify', str(phase_path), '--advance'], root)
            self.assertEqual((root / 'docs/phase-status.json').read_bytes(), original)

    def test_phase_advancement_rejects_stale_or_misbound_chunk(self):
        for field, wrong in (('source_sha256', 'stale'), ('chunk', 'wrong-chunk')):
            with self.subTest(field=field), self.chunk_repository() as (root, _, _):
                original = self.declare_chunks(root, 'example')
                _, chunk_path = self.run_synthetic_review(root, chunk='example')
                _, phase_path = self.run_synthetic_review(root)
                record = json.loads(chunk_path.read_text())
                record[field] = wrong
                chunk_path.write_text(json.dumps(record))
                with self.assertRaisesRegex(ValueError, 'Latest chunk review is rejected, stale, or invalid'):
                    main(['--phase', '19', '--verify', str(phase_path), '--advance'], root)
                self.assertEqual((root / 'docs/phase-status.json').read_bytes(), original)

    def test_phase_advancement_rejects_missing_or_empty_chunk_declarations(self):
        for declaration in (None, []):
            with self.subTest(declaration=declaration), self.chunk_repository() as (root, _, _):
                self.declare_chunks(root, 'example')
                self.run_synthetic_review(root, chunk='example')
                _, phase_path = self.run_synthetic_review(root)
                status_path = root / 'docs/phase-status.json'
                status = json.loads(status_path.read_text())
                if declaration is None:
                    del status['phases'][0]['chunks']
                else:
                    status['phases'][0]['chunks'] = declaration
                status_path.write_text(json.dumps(status))
                before = status_path.read_bytes()
                with self.assertRaisesRegex(ValueError, 'nonempty chunks list'):
                    main(['--phase', '19', '--verify', str(phase_path), '--advance'], root)
                self.assertEqual(status_path.read_bytes(), before)

    def test_phase_advancement_rejects_removed_or_undeclared_reviewed_chunk(self):
        for originally_declared in (False, True):
            with self.subTest(originally_declared=originally_declared), self.chunk_repository() as (root, _, _):
                self.declare_chunks(root, 'retained', *(['removed'] if originally_declared else []))
                self.run_synthetic_review(root, chunk='retained')
                self.run_synthetic_review(root, chunk='removed')
                _, phase_path = self.run_synthetic_review(root)
                before = self.declare_chunks(root, 'retained')
                with self.assertRaisesRegex(ValueError, 'Reviewed chunks are missing declarations: removed'):
                    main(['--phase', '19', '--verify', str(phase_path), '--advance'], root)
                self.assertEqual((root / 'docs/phase-status.json').read_bytes(), before)

    def refresh_synthetic_checks(self, root):
        path = root / 'docs/checks/phase-19.json'
        checks = json.loads(path.read_text())
        checks['source_sha256'] = fingerprint(root)
        path.write_text(json.dumps(checks))

    def test_accepted_fifth_chunk_can_retain_original_source_with_current_phase_coverage(self):
        with self.chunk_repository() as (root, original_source, _):
            self.declare_chunks(root, 'example')
            for attempt in range(1, 6):
                _, chunk_path = self.run_synthetic_review(root, chunk='example', attempt=attempt)
            (root / 'source.rs').write_text('Phase-requested correction')
            self.refresh_synthetic_checks(root)
            _, phase_path = self.run_synthetic_review(root)
            self.assertEqual(main(['--phase', '19', '--verify', str(phase_path), '--advance'], root), 0)
            chunk = json.loads((root / 'docs/phase-status.json').read_text())['phases'][0]['chunks'][0]
            self.assertEqual(chunk['accepted_source_sha256'], original_source)
            self.assertEqual(chunk['accepted_review'], str(chunk_path.relative_to(root)))
            self.assertEqual(chunk['phase_followup_review'], str(phase_path.relative_to(root)))
            self.assertEqual(chunk['phase_followup_current_source_sha256'], fingerprint(root))

    def test_fifth_followup_and_predecessor_survive_command_policy_changes(self):
        with self.chunk_repository() as (root, original_source, _):
            self.declare_chunks(root, 'example')
            checks_path = root / 'docs/checks/phase-19.json'
            checks = json.loads(checks_path.read_text())
            checks['checks'] *= 2
            checks_path.write_text(json.dumps(checks))
            with patch('check_phase.commands_for', return_value=[['synthetic-check']] * 2):
                for attempt in range(1, 6):
                    self.run_synthetic_review(root, chunk='example', attempt=attempt)
            (root / 'source.rs').write_text('Reviewed deduplication correction')
            checks['checks'] = checks['checks'][:1]
            checks_path.write_text(json.dumps(checks))
            self.refresh_synthetic_checks(root)
            _, phase_path = self.run_synthetic_review(root)
            self.assertEqual(main(['--phase', '19', '--verify', str(phase_path), '--advance'], root), 0)
            status = json.loads((root / 'docs/phase-status.json').read_text())
            self.assertEqual(status['phases'][0]['chunks'][0]['accepted_source_sha256'], original_source)
            # A later release adds a test to the cumulative predecessor policy.
            (root / 'source.rs').write_text('Later release tooling adds checks')
            with patch('check_phase.commands_for', return_value=[['synthetic-check'], ['release-check']]):
                validate_predecessors(root, status, 20)

    def test_chunk_followup_rejects_missing_rejected_stale_or_early_coverage(self):
        from review_phase import accepted_chunks
        for failure in ('missing-phase', 'rejected-phase', 'stale-phase', 'early-chunk', 'rejected-chunk'):
            with self.subTest(failure=failure), self.chunk_repository() as (root, _, _):
                self.declare_chunks(root, 'example')
                for attempt in range(1, 5 if failure == 'early-chunk' else 6):
                    values = (8, 8, 8, 8, 8) if failure == 'rejected-chunk' else (10, 9, 9, 9, 9)
                    self.run_synthetic_review(root, chunk='example', attempt=attempt, values=values)
                (root / 'source.rs').write_text('Phase-requested correction')
                self.refresh_synthetic_checks(root)
                phase_path = None
                if failure != 'missing-phase':
                    values = (9, 9, 9, 9, 9) if failure == 'rejected-phase' else (10, 9, 9, 9, 9)
                    _, phase_path = self.run_synthetic_review(root, values=values)
                if failure == 'stale-phase':
                    (root / 'source.rs').write_text('Another unreviewed correction')
                with self.assertRaises(ValueError):
                    accepted_chunks(root, 19, [{'id': 'example'}], fingerprint(root), phase_review=phase_path)

    def test_chunk_followup_accepts_authorized_fifth_phase_exception(self):
        from review_phase import accepted_chunks
        with self.chunk_repository() as (root, original_source, _):
            self.declare_chunks(root, 'example')
            for attempt in range(1, 6):
                self.run_synthetic_review(root, chunk='example', attempt=attempt)
            (root / 'source.rs').write_text('Phase-requested correction')
            self.refresh_synthetic_checks(root)
            for attempt in range(1, 6):
                outcome, phase_path = self.run_synthetic_review(
                    root, attempt=attempt, values=(9, 9, 9, 9, 9), limitations=['Recorded residual documentation issue'])
            self.assertEqual(outcome, 0)
            chunks = accepted_chunks(root, 19, [{'id': 'example'}], fingerprint(root), phase_review=phase_path)
            self.assertEqual(chunks[0]['accepted_source_sha256'], original_source)
            self.assertEqual(chunks[0]['phase_followup_current_source_sha256'], fingerprint(root))

    def test_attempt_five_cannot_skip_previous_reviews(self):
        from review_phase import verify_attempt_sequence
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaisesRegex(ValueError, "Previous review evidence has changed"):
                verify_attempt_sequence(pathlib.Path(directory), 19, 5)


if __name__ == "__main__":
    unittest.main()
