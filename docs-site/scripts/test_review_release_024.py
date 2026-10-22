import copy
import importlib.util
import json
from pathlib import Path
import tempfile
import unittest
from unittest import mock

spec = importlib.util.spec_from_file_location('docs_review', Path(__file__).with_name('review-release-024.py'))
r = importlib.util.module_from_spec(spec)
spec.loader.exec_module(r)


class Tests(unittest.TestCase):
    def review(self):
        return {'categories': dict(zip(r.CATEGORIES, (9, 9, 9, 9, 10))), 'total': 46, 'overall': 9,
                'blocking_findings': [], 'nonblocking_findings': [], 'evidence_limitations': ['Read-only inspection.']}

    def test_exact_categories_and_sum(self):
        self.assertTrue(r.validate_review(self.review()))
        bad = self.review(); bad['total'] = 50
        with self.assertRaisesRegex(ValueError, 'sum'): r.validate_review(bad)
        bad = self.review(); bad['categories']['implementation'] = True
        with self.assertRaisesRegex(ValueError, 'scores'): r.validate_review(bad)
        bad = self.review(); bad['categories'].pop('delivery')
        with self.assertRaisesRegex(ValueError, 'categories'): r.validate_review(bad)

    def test_envelope_result_fallback_remains_strict(self):
        expected = self.review()
        for envelope in (
            {'is_error': False, 'result': json.dumps(expected)},
            {'is_error': False, 'structured_output': None, 'result': json.dumps(expected)},
            {'is_error': False, 'structured_output': expected, 'result': 'not JSON'},
        ):
            review, accepted = r.parse_envelope(envelope)
            self.assertEqual(review, expected)
            self.assertTrue(accepted)
        for envelope in (
            {'is_error': True, 'result': json.dumps(expected)},
            {'is_error': False},
            {'is_error': False, 'result': 'not JSON'},
            {'is_error': False, 'structured_output': {}, 'result': json.dumps(expected)},
            {'is_error': False, 'result': json.dumps(dict(expected, total=50))},
        ):
            with self.assertRaises(ValueError): r.parse_envelope(envelope)

    def test_prompt_names_current_source_and_actual_prior_review(self):
        with mock.patch.object(r, 'prior_review_name', side_effect=AssertionError('No prior probe for attempt1')) as prior:
            first = r.render_prompt(1)
            prior.assert_not_called()
        self.assertIn('attempt-01.source.json and attempt-01.checks.json', first)
        self.assertNotIn('For this follow-up', first)
        self.assertNotIn('attempt-00', first)
        for attempt in (2, 3, 5):
            prompt = r.render_prompt(attempt)
            self.assertIn(f'attempt-{attempt:02d}.source.json and attempt-{attempt:02d}.checks.json', prompt)
            self.assertIn(f'attempt-{attempt-1:02d}.json and attempt-{attempt-1:02d}.disposition.json', prompt)
            self.assertNotIn('{current}', prompt)
            self.assertNotIn('{prior}', prompt)

    def test_failed_prior_invocation_retains_actual_terminal_reference(self):
        with tempfile.TemporaryDirectory() as tmp, mock.patch.object(r, 'SCOPE', Path(tmp)):
            (Path(tmp) / 'attempt-01.failed.json').write_text('{}')
            prompt = r.render_prompt(2)
            self.assertIn('attempt-01.failed.json and attempt-01.disposition.json', prompt)
            self.assertIn('docs/reviews/github-pages/release-0.2.4/attempt-01.failed.json', r.prior_review_sources(2))
            self.assertNotIn('docs/reviews/github-pages/release-0.2.4/attempt-01.json', r.prior_review_sources(2))

    def test_snapshot_binds_each_actual_prior_disposition(self):
        state = {'independent_documentation_delivery': {'score': 46}}
        values = {}
        def read(name):
            if name == 'docs/phase-status.json':
                return json.dumps(state).encode()
            if name == 'docs-site/DESIGN_SOURCE.json':
                return b'{}'
            return values.get(name, b'fixed')
        with mock.patch.object(r, 'git', side_effect=lambda *args: b'' if args[0] == 'ls-files' else b'head'), \
             mock.patch.object(r, 'delivery_records', return_value=({'id': 28, 'status': 'pending'}, [])), \
             mock.patch.object(r, 'read', side_effect=read):
            before = r.snapshot(3)
            for prior in (1, 2):
                for suffix in ('.json', '.disposition.json'):
                    self.assertIn(f'docs/reviews/github-pages/release-0.2.4/attempt-{prior:02d}{suffix}', before['files'])
            values['docs/reviews/github-pages/release-0.2.4/attempt-02.disposition.json'] = b'changed'
            self.assertNotEqual(r.snapshot(3)['sourceSha256'], before['sourceSha256'])

    def test_every_release_derived_source_is_bound_in_snapshot(self):
        manifest = json.loads((r.ROOT / 'docs-site/DESIGN_SOURCE.json').read_text())
        state = {'independent_documentation_delivery': {'score': 46}}
        def read(name):
            if name == 'docs/phase-status.json':
                return json.dumps(state).encode()
            if name == 'docs-site/DESIGN_SOURCE.json':
                return json.dumps(manifest).encode()
            return b'fixed'
        with mock.patch.object(r, 'git', side_effect=lambda *args: b'' if args[0] == 'ls-files' else b'head'), \
             mock.patch.object(r, 'delivery_records', return_value=({'id': 28, 'status': 'pending'}, [])), \
             mock.patch.object(r, 'read', side_effect=read):
            files = r.snapshot()['files']
            self.assertTrue(manifest['release024DerivedSources'])
            for entry in manifest['release024DerivedSources'].values():
                self.assertIn(entry['sourcePath'], files)

    def test_missing_or_malformed_phases_is_named(self):
        for status in ({}, {'phases': None}, {'phases': {}}, {'phases': [None]}):
            with mock.patch.object(r, 'read', return_value=json.dumps(status).encode()):
                with self.assertRaisesRegex(ValueError, 'phase-status phases'):
                    r.delivery_records()

    def test_missing_guard_commits_are_named(self):
        phase = {'id': 28, 'status': 'pending'}
        name = 'docs/checks/release-registry-0.2.4.json'
        raw = json.dumps({'sourceCommit': 'published'}).encode()
        valid = {'passed': True, 'version': '0.2.4', 'jobs': 49,
                 'sourceCommit': 'published', 'ciCommit': 'ci'}
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / 'docs/checks').mkdir(parents=True)
            (root / name).write_bytes(raw)
            (root / 'docs/phase-status.json').write_text(json.dumps({'phases': [phase]}))
            with mock.patch.object(r, 'delivery_records', return_value=(phase, [name])), \
                 mock.patch.object(r, 'read', return_value=raw), \
                 mock.patch.object(r.subprocess, 'run') as run:
                for field in ('sourceCommit', 'ciCommit'):
                    for value in (None, '', 31):
                        invalid = dict(valid)
                        if value is None:
                            invalid.pop(field)
                        else:
                            invalid[field] = value
                        run.return_value = mock.Mock(returncode=0, stdout=json.dumps(invalid))
                        with self.assertRaisesRegex(ValueError, 'Primary guard ' + field):
                            r.verify_release_delivery(root)

    def test_missing_registry_source_commit_is_named(self):
        phase = {'id': 28, 'status': 'pending'}
        name = 'docs/checks/release-registry-0.2.4.json'
        raw = b'{}'
        verified = {'passed': True, 'version': '0.2.4', 'jobs': 49,
                    'sourceCommit': 'published', 'ciCommit': 'ci'}
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / 'docs/checks').mkdir(parents=True)
            (root / name).write_bytes(raw)
            (root / 'docs/phase-status.json').write_text(json.dumps({'phases': [phase]}))
            with mock.patch.object(r, 'delivery_records', return_value=(phase, [name])), \
                 mock.patch.object(r, 'read', return_value=raw), \
                 mock.patch.object(r.subprocess, 'run', return_value=mock.Mock(returncode=0, stdout=json.dumps(verified))):
                with self.assertRaisesRegex(ValueError, 'Registry sourceCommit'):
                    r.verify_release_delivery(root)

    def test_threshold_and_findings(self):
        bad = self.review(); bad['categories']['delivery'] = 9; bad['total'] = 45
        self.assertFalse(r.validate_review(bad))
        bad = self.review(); bad['categories'] = dict(zip(r.CATEGORIES, (7, 10, 10, 10, 10))); bad['total'] = 47
        self.assertFalse(r.validate_review(bad))
        finding = {'severity': 'high', 'path': 'docs-site/docs/index.md', 'line': 1,
                   'description': 'Incorrect claim', 'required_fix': 'Correct claim'}
        for field in ('blocking_findings', 'nonblocking_findings'):
            bad = self.review(); bad[field] = [finding]
            self.assertFalse(r.validate_review(bad))

    def test_attempt_limit_and_orphans(self):
        with tempfile.TemporaryDirectory() as tmp, mock.patch.object(r, 'SCOPE', Path(tmp)):
            r.validate_attempt(1)
            for attempt in (0, 6):
                with self.assertRaisesRegex(ValueError, 'five'): r.validate_attempt(attempt)
            with self.assertRaisesRegex(ValueError, 'terminal'): r.validate_attempt(2)
            (Path(tmp)/'attempt-01.prompt.md').write_text('unfinished')
            with self.assertRaisesRegex(ValueError, 'overwrite'): r.validate_attempt(1)
            with self.assertRaisesRegex(ValueError, 'terminal'): r.validate_attempt(2)

    def test_pending_sdk_phase_requires_exact_verified_delivery(self):
        phase = {'id': 28, 'status': 'pending'}
        name = 'docs/checks/release-registry-0.2.4.json'
        registry = json.dumps({'sourceCommit': 'published'}).encode()
        verified = {'passed': True, 'version': '0.2.4', 'jobs': 49,
                    'sourceCommit': 'published', 'ciCommit': 'ci'}
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            (root / 'docs/checks').mkdir(parents=True)
            status = root / 'docs/phase-status.json'
            status.write_text(json.dumps({'phases': [phase]}))
            receipt = root / name
            receipt.write_bytes(registry)
            with mock.patch.object(r, 'delivery_records', return_value=(phase, [name])), \
                 mock.patch.object(r, 'read', return_value=registry), \
                 mock.patch.object(r.subprocess, 'run') as run:
                run.return_value = mock.Mock(returncode=0, stdout=json.dumps(verified))
                result = r.verify_release_delivery(root)
                self.assertEqual(result['sdkPhase28Status'], 'pending')
                self.assertEqual(result['guard'], verified)
                self.assertEqual(run.call_count, 3)
                self.assertIn('tools/release_delivery.py', run.call_args_list[0].args[0])
                for invalid in (dict(verified, jobs=48), dict(verified, passed=False),
                                dict(verified, sourceCommit='wrong')):
                    run.return_value = mock.Mock(returncode=0, stdout=json.dumps(invalid))
                    with self.assertRaisesRegex(ValueError, 'terminal49'):
                        r.verify_release_delivery(root)
                run.return_value = mock.Mock(returncode=1, stdout='')
                with self.assertRaisesRegex(ValueError, 'has not passed'):
                    r.verify_release_delivery(root)
                receipt.write_bytes(b'different')
                with self.assertRaisesRegex(ValueError, 'evidence differs'):
                    r.verify_release_delivery(root)
                status.write_text(json.dumps({'phases': [dict(phase, status='complete')]}))
                with self.assertRaisesRegex(ValueError, 'records differ'):
                    r.verify_release_delivery(root)

    def test_fresh_ci_observation_retains_canonical_ci_receipt(self):
        canonical = 'docs/checks/phase-28-remote-ci-run-31.json'
        observed = 'docs/checks/phase-28-remote-ci-run-31-observation-03.json'
        phase = {'id': 28, 'delivery': {'ci_receipt': canonical, 'ci_observation': observed}}
        inputs = {'docs/phase-status.json': {'phases': [phase]},
                  'docs/checks/release-registry-0.2.4.json': {
                      'releaseReadiness': {'review': 'docs/reviews/phase-28-attempt-05.json'}}}
        with mock.patch.object(r, 'read', side_effect=lambda name: json.dumps(inputs[name]).encode()):
            actual_phase, files = r.delivery_records()
        self.assertEqual(actual_phase, phase)
        self.assertIn(canonical, files)
        self.assertIn(observed, files)

    def test_missing_or_malformed_canonical_ci_receipt_is_named(self):
        for canonical in (None, '', 31, 'docs/checks/phase-28-remote-ci-run-31-observation-03.json',
                          '../phase-28-remote-ci-run-31.json'):
            phase = {'id': 28, 'delivery': {
                'ci_observation': 'docs/checks/phase-28-remote-ci-run-31-observation-03.json'}}
            if canonical is not None:
                phase['delivery']['ci_receipt'] = canonical
            inputs = {'docs/phase-status.json': {'phases': [phase]},
                      'docs/checks/release-registry-0.2.4.json': {
                          'releaseReadiness': {'review': 'docs/reviews/phase-28-attempt-05.json'}}}
            with mock.patch.object(r, 'read', side_effect=lambda name: json.dumps(inputs[name]).encode()):
                with self.assertRaisesRegex(ValueError, 'canonical ci_receipt'):
                    r.delivery_records()

    def test_governance_guard_requires_qualified_binding_and_merged_snapshot(self):
        phase = {'id': 28, 'status': 'pending'}
        name = 'docs/checks/release-registry-0.2.4.json'
        raw = json.dumps({'sourceCommit': 'published', 'archiveSha256': 'archive'}).encode()
        canonical = {'passed': True, 'version': '0.2.4', 'jobs': 49,
                     'sourceCommit': 'published', 'ciCommit': 'ci'}
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp); (root / 'docs/checks').mkdir(parents=True)
            (root / name).write_bytes(raw)
            (root / 'docs/phase-status.json').write_text(json.dumps({'phases': [phase]}))
            qualified = root / 'qualified'
            proof = {'scope': 'post_publication_governance', 'passed': True, 'source_sha256': 'governance', 'qualifiedSourceSha256': 'frozen',
                     'qualifiedReleaseRoot': str(qualified), 'publishedSourceCommit': 'published',
                     'canonicalGuard': {'result': canonical}, 'tagObservations': {name: r.sha(raw)},
                     'currentCiPath': name, 'currentCiSha256': r.sha(raw)}
            with mock.patch.object(r, 'delivery_records', return_value=(phase, [name])), \
                 mock.patch.object(r, 'read', return_value=raw), mock.patch.object(r.subprocess, 'run') as run:
                run.return_value = mock.Mock(returncode=0, stdout=json.dumps(proof))
                result = r.verify_release_delivery(root, qualified)
                self.assertEqual(result['governanceGuard'], proof)
                self.assertEqual(result['releaseRepo'], str(root.resolve()))
                self.assertEqual(result['qualifiedReleaseRepo'], str(qualified.resolve()))
                self.assertIn('--post-publication-governance', run.call_args_list[0].args[0])
                for field in ('currentCiSha256', 'qualifiedReleaseRoot', 'publishedSourceCommit'):
                    bad = dict(proof); bad[field] = 'wrong'
                    run.return_value = mock.Mock(returncode=0, stdout=json.dumps(bad))
                    with self.assertRaises(ValueError): r.verify_release_delivery(root, qualified)
                for changed in (dict(proof, scope='wrong_scope'),
                                dict(proof, source_sha256='frozen'),
                                dict(proof, source_sha256='archive')):
                    run.return_value = mock.Mock(returncode=0, stdout=json.dumps(changed))
                    with self.assertRaisesRegex(ValueError, 'distinct source bindings'):
                        r.verify_release_delivery(root, qualified)

    def test_prior_documentation_metadata_preserved_before_append(self):
        original = {'phases': [{'id': 28, 'status': 'pending'}],
                    'independent_documentation_delivery': {
                        'score': 46, 'logo_update': {'score': 46, 'review': 'original'},
                        'release_updates': [{'version': 'old', 'score': 47}]}}
        expected = r.documentation_metadata_sha(original)
        update = {'version': '0.2.4', 'score': 46}
        mutations = [lambda d: d.update(score=50),
                     lambda d: d['logo_update'].update(score=50),
                     lambda d: d['release_updates'][0].update(score=50),
                     lambda d: d['release_updates'].append({'version': 'unexpected'})]
        for mutate in mutations:
            changed = copy.deepcopy(original)
            mutate(changed['independent_documentation_delivery'])
            before = copy.deepcopy(changed)
            with self.assertRaisesRegex(ValueError, 'metadata changed'):
                r.append_documentation_update(changed, expected, update)
            self.assertEqual(changed, before)
        normal = copy.deepcopy(original)
        r.append_documentation_update(normal, expected, update)
        self.assertEqual(normal['phases'], original['phases'])
        self.assertEqual(normal['independent_documentation_delivery']['release_updates'].pop(), update)
        self.assertEqual(normal, original)

    def test_snapshot_binds_prior_documentation_metadata(self):
        state = {'independent_documentation_delivery': {'score': 46}}
        with mock.patch.object(r, 'git', side_effect=lambda *args: b'' if args[0] == 'ls-files' else b'head'), \
             mock.patch.object(r, 'delivery_records', return_value=({'id': 28, 'status': 'pending'}, [])), \
             mock.patch.object(r, 'read', side_effect=lambda name: json.dumps(state).encode() if name == 'docs/phase-status.json' else b'{}' if name == 'docs-site/DESIGN_SOURCE.json' else b'fixed'):
            before = r.snapshot()
            state['independent_documentation_delivery']['score'] = 50
            after = r.snapshot()
            self.assertNotEqual(before['priorDocumentationMetadataSha256'], after['priorDocumentationMetadataSha256'])
            self.assertNotEqual(before['sourceSha256'], after['sourceSha256'])
            self.assertEqual(before['files'], after['files'])

    def test_fifth_exception_requires_four_intact_substantive_reviews(self):
        review = self.review(); review['categories']['delivery'] = 9; review['total'] = 45
        with tempfile.TemporaryDirectory() as tmp, mock.patch.object(r, 'SCOPE', Path(tmp)):
            self.assertIsNone(r.fifth_attempt_exception(review, 5))
            for number in range(1, 5):
                record = {'reviewer': 'Claude Code', 'scope': 'independent-docs-release-0.2.4',
                          'attempt': number, 'sourceUnchanged': True, 'decision': 'rejected', 'review': review}
                for key, suffix in [('sourceManifestSha256', 'source.json'), ('checksSha256', 'checks.json'),
                                    ('promptSha256', 'prompt.md'), ('cliSha256', 'cli.json')]:
                    path = Path(tmp) / f'attempt-{number:02d}.{suffix}'
                    path.write_text(json.dumps({'exitCode': 0}) if suffix == 'cli.json' else 'retained')
                    record[key] = r.sha(path.read_bytes())
                (Path(tmp) / f'attempt-{number:02d}.json').write_text(json.dumps(record))
            self.assertIsNone(r.fifth_attempt_exception(review, 4))
            self.assertIsNone(r.fifth_attempt_exception(review, 6))
            result = r.fifth_attempt_exception(review, 5)
            self.assertEqual(len(result['priorReviews']), 4)
            self.assertEqual(result['normalThresholdShortfall'], 1)
            for scores in [(8, 8, 8, 8, 8), (7, 9, 9, 10, 10)]:
                bad = copy.deepcopy(review); bad['categories'] = dict(zip(r.CATEGORIES, scores)); bad['total'] = sum(scores)
                self.assertIsNone(r.fifth_attempt_exception(bad, 5))
            (Path(tmp) / 'attempt-02.cli.json').write_text('{}')
            self.assertIsNone(r.fifth_attempt_exception(review, 5))

    def test_reader_rejects_escape_and_symlink(self):
        with self.assertRaises(ValueError): r.read('../not-a-review-file')
        with tempfile.TemporaryDirectory() as tmp, mock.patch.object(r, 'ROOT', Path(tmp)):
            (Path(tmp)/'safe').write_text('text')
            (Path(tmp)/'link').symlink_to(Path(tmp)/'safe')
            self.assertEqual(r.read('safe'), b'text')
            with self.assertRaises(ValueError): r.read('link')


if __name__ == '__main__': unittest.main()
