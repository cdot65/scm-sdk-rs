"""Gate, attempt-budget and evidence-binding tests for the phase 29–31 Codex reviewer."""
import json
import pathlib
import tempfile
import unittest
from unittest import mock

import codex_review

COMMANDS = [['python', 'tools/check.py']]
FINDING = {'severity': 'medium', 'path': 'tools/x.py', 'line': 1, 'description': 'd', 'required_fix': 'f'}


def review(score=9, findings=()):
    return {'score': score, 'summary': 'ok', 'findings': list(findings), 'evidence': ['tools/x.py'], 'limitations': []}


class CodexReviewTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.root = pathlib.Path(self.temporary.name)
        (self.root / 'docs/checks').mkdir(parents=True)
        self.write_checks('s1')
        policy = {'reviewer': 'codex', 'minimum_score': 9}
        self.status = {'active_phase': 29, 'phases': [
            {'id': 29, 'name': 'plan', 'status': 'in_progress', 'review_policy': policy, 'scope_evidence': ['a'],
             'chunks': [{'id': 'plan-and-provenance', 'status': 'pending', 'scope_evidence': ['a']}]},
            {'id': 30, 'name': 'impl', 'status': 'pending', 'review_policy': policy, 'scope_evidence': ['b'], 'chunks': []}]}
        self.write_status()

    def tearDown(self):
        self.temporary.cleanup()

    def write_checks(self, source):
        report = {'phase': 29, 'passed': True, 'source_sha256': source, 'source_unchanged': True,
                  'checks': [{'command': COMMANDS[0], 'exit_code': 0}]}
        (self.root / 'docs/checks/phase-29.json').write_text(json.dumps(report))

    def write_status(self):
        (self.root / 'docs/phase-status.json').write_text(json.dumps(self.status))

    def run_review(self, output, chunk='plan-and-provenance', code=0, source=('s1', 's1')):
        def runner(prompt, out, events, schema, root):
            events.write_text('{}\n')
            if output is not None:
                out.write_text(json.dumps(output))
            return code
        sources = iter(source)
        with mock.patch.object(codex_review, 'fingerprint', side_effect=lambda root: next(sources)), \
                mock.patch.object(codex_review, 'check_evidence', return_value=self.root / 'docs/checks/phase-29.json'), \
                mock.patch.object(codex_review, 'commands_for', return_value=COMMANDS):
            argv = ['--phase', '29'] + (['--chunk', chunk] if chunk else [])
            return codex_review.main(argv, root=self.root, runner=runner)

    def status_now(self):
        return json.loads((self.root / 'docs/phase-status.json').read_text())

    def test_threshold_is_nine_and_blocking_findings_reject(self):
        self.assertEqual(codex_review.decide(review(9)), 'accepted')
        self.assertEqual(codex_review.decide(review(8)), 'rejected')
        self.assertEqual(codex_review.decide(review(10, [{**FINDING, 'severity': 'high'}])), 'rejected')
        self.assertEqual(codex_review.decide(review(10, [FINDING])), 'accepted')

    def test_output_schema_is_closed_and_fully_required(self):
        schema = codex_review.OUTPUT_SCHEMA
        self.assertEqual(set(schema['required']), set(schema['properties']))
        item = schema['properties']['findings']['items']
        self.assertEqual(set(item['required']), set(item['properties']))
        self.assertFalse(schema['additionalProperties'])

    def test_accepted_chunk_records_evidence_and_updates_status(self):
        self.assertEqual(self.run_review(review(9)), 0)
        record = json.loads((self.root / 'docs/reviews/phase-29-chunk-plan-and-provenance-codex-attempt-01.json').read_text())
        self.assertEqual((record['decision'], record['reviewer'], record['source_unchanged']), ('accepted', 'Codex CLI', True))
        chunk = self.status_now()['phases'][0]['chunks'][0]
        self.assertEqual((chunk['status'], chunk['accepted_score'], chunk['accepted_source_sha256']), ('accepted', 9, 's1'))

    def test_rejection_consumes_an_attempt_and_leaves_status_pending(self):
        self.assertEqual(self.run_review(review(8)), 2)
        self.assertEqual(self.status_now()['phases'][0]['chunks'][0]['status'], 'pending')
        self.assertEqual(self.run_review(review(10)), 0)
        self.assertTrue((self.root / 'docs/reviews/phase-29-chunk-plan-and-provenance-codex-attempt-02.json').exists())

    def test_changed_source_during_review_is_rejected(self):
        self.assertEqual(self.run_review(review(10), source=('s1', 's2')), 2)
        record = json.loads((self.root / 'docs/reviews/phase-29-chunk-plan-and-provenance-codex-attempt-01.json').read_text())
        self.assertEqual((record['source_unchanged'], record['decision']), (False, 'rejected'))

    def test_invocation_failures_are_recorded_and_consume_attempts(self):
        self.assertEqual(self.run_review(None, code=3), 1)
        failed = json.loads((self.root / 'docs/reviews/phase-29-chunk-plan-and-provenance-codex-attempt-01.failed.json').read_text())
        self.assertEqual((failed['status'], failed['reason']), ('invocation_failed', 'process_error'))
        self.assertEqual(self.run_review({'score': 'nine'}), 1)
        self.assertTrue((self.root / 'docs/reviews/phase-29-chunk-plan-and-provenance-codex-attempt-02.failed.json').exists())
        self.assertEqual(self.run_review({**review(9.5)}), 1)  # fractional scores are invalid output

    def test_five_attempt_budget_is_enforced(self):
        for _ in range(5):
            self.run_review(review(5))
        with self.assertRaisesRegex(ValueError, 'five-attempt'):
            self.run_review(review(10))

    def test_tampered_prior_evidence_blocks_the_next_attempt(self):
        self.run_review(review(5))
        output = self.root / 'docs/reviews/phase-29-chunk-plan-and-provenance-codex-attempt-01.codex.json'
        output.write_text(json.dumps(review(10)))
        with self.assertRaisesRegex(ValueError, 'Previous Codex output has changed'):
            self.run_review(review(10))

    def test_phase_review_requires_accepted_chunks_then_advances(self):
        with self.assertRaisesRegex(ValueError, 'Every declared chunk'):
            self.run_review(review(9), chunk=None)
        self.assertFalse((self.root / 'docs/reviews/phase-29-codex-attempt-01.prompt.md').exists())
        self.run_review(review(9))
        self.assertEqual(self.run_review(review(9), chunk=None), 0)
        status = self.status_now()
        self.assertEqual((status['phases'][0]['status'], status['active_phase'], status['phases'][1]['status']),
                         ('complete', 30, 'in_progress'))
        self.assertEqual(status['phases'][0]['milestones']['codex_review']['score'], 9)

    def test_phase_review_rejects_stale_missing_or_superseded_chunk_acceptance(self):
        self.run_review(review(9))
        # Stale: chunk accepted against an older source fingerprint.
        with self.assertRaisesRegex(ValueError, 'stale, superseded'):
            self.run_review(review(9), chunk=None, source=('s2', 's2'))
        # Forged: status names a review that does not exist.
        status = self.status_now()
        status['phases'][0]['chunks'][0]['accepted_review'] = 'docs/reviews/missing.json'
        (self.root / 'docs/phase-status.json').write_text(json.dumps(status))
        with self.assertRaisesRegex(ValueError, 'accepted latest review'):
            self.run_review(review(9), chunk=None)

    def test_later_rejection_supersedes_an_earlier_acceptance(self):
        self.run_review(review(9))
        self.assertEqual(self.run_review(review(6)), 2)
        chunk = self.status_now()['phases'][0]['chunks'][0]
        self.assertEqual((chunk['status'], chunk['review_decision']), ('pending', 'rejected'))
        self.assertNotIn('accepted_review', chunk)
        with self.assertRaisesRegex(ValueError, 'accepted latest review'):
            self.run_review(review(9), chunk=None)

    def test_tampered_accepted_decision_is_detected(self):
        self.run_review(review(5))
        path = self.root / 'docs/reviews/phase-29-chunk-plan-and-provenance-codex-attempt-01.json'
        record = json.loads(path.read_text())
        record['decision'] = 'accepted'
        path.write_text(json.dumps(record))
        with self.assertRaisesRegex(ValueError, 'does not follow'):
            self.run_review(review(10))

    def test_orphaned_slot_is_never_reused_and_must_be_marked_interrupted(self):
        reviews = self.root / 'docs/reviews'
        reviews.mkdir(parents=True)
        stem = 'phase-29-chunk-plan-and-provenance-codex-attempt-01'
        (reviews / (stem + '.prompt.md')).write_text('orphan')
        (reviews / (stem + '.checks.json')).write_text((self.root / 'docs/checks/phase-29.json').read_text())
        self.assertEqual(codex_review.next_attempt(self.root, 29, 'plan-and-provenance'), 2)
        with self.assertRaisesRegex(ValueError, 'mark-interrupted 1'):
            self.run_review(review(9))
        self.assertEqual((reviews / (stem + '.prompt.md')).read_text(), 'orphan')
        self.assertEqual(codex_review.main(['--phase', '29', '--chunk', 'plan-and-provenance', '--mark-interrupted', '1'], root=self.root), 1)
        failed = json.loads((reviews / (stem + '.failed.json')).read_text())
        self.assertEqual(failed['reason'], 'interrupted')
        self.assertEqual(failed['checks_sha256'], codex_review.sha256(reviews / (stem + '.checks.json')))
        self.assertEqual(self.run_review(review(9)), 0)
        self.assertTrue((reviews / 'phase-29-chunk-plan-and-provenance-codex-attempt-02.json').exists())

    def test_reservation_is_exclusive_and_live_attempts_cannot_be_marked(self):
        import fcntl
        reviews = self.root / 'docs/reviews'
        reviews.mkdir(parents=True)
        prompt = reviews / 'phase-29-chunk-plan-and-provenance-codex-attempt-01.prompt.md'
        codex_review.reserve(prompt, 'first')
        with self.assertRaises(FileExistsError):
            codex_review.reserve(prompt, 'second')
        self.assertEqual(prompt.read_text(), 'first')
        lock = codex_review.lock_path(self.root, 29, 1, 'plan-and-provenance')
        lock.parent.mkdir(parents=True, exist_ok=True)
        with lock.open('a') as handle:
            fcntl.flock(handle, fcntl.LOCK_EX)
            with self.assertRaisesRegex(ValueError, 'still in flight'):
                codex_review.mark_interrupted(self.root, 29, 1, 'plan-and-provenance')

    def test_interrupt_during_invocation_records_a_consumed_attempt(self):
        def runner(*_):
            raise KeyboardInterrupt
        with mock.patch.object(codex_review, 'fingerprint', return_value='s1'), \
                mock.patch.object(codex_review, 'check_evidence', return_value=self.root / 'docs/checks/phase-29.json'), \
                mock.patch.object(codex_review, 'commands_for', return_value=COMMANDS):
            with self.assertRaises(KeyboardInterrupt):
                codex_review.main(['--phase', '29', '--chunk', 'plan-and-provenance'], root=self.root, runner=runner)
        failed = json.loads((self.root / 'docs/reviews/phase-29-chunk-plan-and-provenance-codex-attempt-01.failed.json').read_text())
        self.assertEqual(failed['reason'], 'interrupted')

    def test_rebinding_record_and_status_metadata_without_evidence_is_rejected(self):
        self.run_review(review(9))
        path = self.root / 'docs/reviews/phase-29-chunk-plan-and-provenance-codex-attempt-01.json'
        record = json.loads(path.read_text())
        record['source_sha256'] = 's2'
        record['scope_evidence'] = ['new-scope']
        path.write_text(json.dumps(record))
        status = self.status_now()
        chunk = status['phases'][0]['chunks'][0]
        chunk.update(accepted_source_sha256='s2', scope_evidence=['new-scope'])
        (self.root / 'docs/phase-status.json').write_text(json.dumps(status))
        with self.assertRaisesRegex(ValueError, 'does not bind the recorded source'):
            self.run_review(review(9), chunk=None, source=('s2', 's2'))
        # Rebinding the prompt too still leaves the check snapshot naming the old source.
        prompt = path.with_suffix('.prompt.md')
        text = prompt.read_text().replace('fingerprint: s1.', 'fingerprint: s2.').replace('["a"]', '["new-scope"]')
        prompt.write_text(text)
        record['prompt_sha256'] = codex_review.sha256(prompt)
        path.write_text(json.dumps(record))
        with self.assertRaisesRegex(ValueError, 'check snapshot does not pass'):
            self.run_review(review(9), chunk=None, source=('s2', 's2'))

    def test_interruption_before_check_snapshot_is_recoverable(self):
        reviews = self.root / 'docs/reviews'
        reviews.mkdir(parents=True)
        stem = 'phase-29-chunk-plan-and-provenance-codex-attempt-01'
        codex_review.reserve(reviews / (stem + '.prompt.md'), 'reserved only')
        self.assertEqual(codex_review.main(['--phase', '29', '--chunk', 'plan-and-provenance', '--mark-interrupted', '1'], root=self.root), 1)
        failed = json.loads((reviews / (stem + '.failed.json')).read_text())
        self.assertIsNone(failed['checks_sha256'])
        self.assertEqual(codex_review.verify_attempt(self.root, 29, 1, 'plan-and-provenance')['reason'], 'interrupted')
        self.assertEqual(self.run_review(review(9)), 0)
        self.assertTrue((reviews / 'phase-29-chunk-plan-and-provenance-codex-attempt-02.json').exists())
        # A null snapshot hash cannot hide check evidence that does exist.
        (reviews / (stem + '.checks.json')).write_text('{}')
        with self.assertRaisesRegex(ValueError, 'omits existing check evidence'):
            codex_review.verify_attempt(self.root, 29, 1, 'plan-and-provenance')

    def test_missing_empty_or_renamed_chunk_declarations_cannot_bypass_chunk_reviews(self):
        for chunks in (None, [], [{'id': 'other', 'status': 'accepted', 'scope_evidence': ['a']}]):
            self.status = json.loads(json.dumps(self.status))
            if chunks is None:
                self.status['phases'][0].pop('chunks', None)
            else:
                self.status['phases'][0]['chunks'] = chunks
            self.write_status()
            with self.assertRaisesRegex(ValueError, 'required chunk set'):
                self.run_review(review(10), chunk=None)
            self.assertEqual(self.status_now()['phases'][0]['status'], 'in_progress')
        self.assertFalse((self.root / 'docs/reviews/phase-29-codex-attempt-01.prompt.md').exists())

    def test_required_chunk_sets_cover_every_track_phase(self):
        self.assertEqual(set(codex_review.REQUIRED_CHUNKS), set(codex_review.PHASES))
        status = json.loads((codex_review.ROOT / 'docs/phase-status.json').read_text())
        for row in status['phases']:
            if row['id'] in codex_review.PHASES:
                self.assertEqual(tuple(c['id'] for c in row['chunks']), codex_review.REQUIRED_CHUNKS[row['id']])

    def test_unregistered_or_inactive_phase_is_refused(self):
        self.status['phases'][0]['review_policy'] = {'reviewer': 'claude', 'minimum_score': 9}
        self.write_status()
        with self.assertRaisesRegex(ValueError, 'not registered for Codex'):
            self.run_review(review(9))
        self.status['active_phase'] = 30
        self.write_status()
        with self.assertRaisesRegex(ValueError, 'active phase'):
            self.run_review(review(9))


if __name__ == '__main__':
    unittest.main()
