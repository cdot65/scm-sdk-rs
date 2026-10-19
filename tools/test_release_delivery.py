"""Offline negative tests for the final registry/tag/CI delivery gate."""
import contextlib
import copy
import io
import json
import pathlib
import subprocess
import tempfile
import unittest
from unittest.mock import patch

from release_delivery import REMOTE, commit_fingerprint, expected_contexts, validate_receipts, verify_source_commits
from review_phase import fingerprint, main, verify_release_readiness, verify_record


class DeliveryTests(unittest.TestCase):
    def setUp(self):
        self.source = 'a' * 40
        self.ci_source = 'b' * 40
        self.digest = 'c' * 64
        self.contexts = expected_contexts('name: Example\n  toolchain: ["1.89.0", "1.99.0"]\n  features: ["all"]\n')
        self.proofs = [{'toolchain': compiler, 'releaseCommit': self.source,
                        'files': [{'path': 'src/lib.rs', 'bytes': 1, 'sha256': self.digest}],
                        'vcs': {'git': {'sha1': self.source}, 'path_in_vcs': 'crates/scm-sdk'},
                        'archive_sha256': self.digest, 'archive_bytes': 123}
                       for compiler in ('1.89.0', '1.99.0')]
        self.registry = {'registry': 'crates.io', 'package': 'scm-sdk', 'version': '0.2.2',
                         'yanked': False, 'ownerLogins': ['cdot65'], 'sourceCommit': self.source,
                         'archiveSha256': self.digest, 'archiveBytes': 123,
                         'files': self.proofs[1]['files'], 'vcs': self.proofs[1]['vcs'],
                         'qualifiedToolchain': '1.99.0', 'identicalFileContentsQualifiedOnMsrv': True}
        self.tag = {'passed': True, 'remote': REMOTE, 'tag': 'v0.2.2',
                    'sourceCommit': self.source, 'tagObject': 'd' * 40,
                    'remoteRefs': {'refs/tags/v0.2.2': 'd' * 40,
                                   'refs/tags/v0.2.2^{}': self.source,
                                   'refs/heads/main': self.ci_source}}
        self.ci = {'reviewedSourceSha256': self.digest, 'sourceCommit': self.ci_source,
                   'run': {'status': 'success', 'commit_sha': self.ci_source, 'index_in_repo': 22,
                           'html_url': 'https://git.cdot.io/cdot/scm-sdk-rs/actions/runs/22'},
                   'expectedJobCount': 3, 'counts': {'success': 3}, 'contextsMatchConfiguredMatrix': True,
                   'jobs': [{'context': context, 'status': 'success',
                             'target_url': f'/cdot/scm-sdk-rs/actions/runs/22/jobs/{i}'}
                            for i, context in enumerate(sorted(self.contexts))]}

    def check(self):
        return validate_receipts(self.registry, self.tag, self.ci, self.proofs, self.contexts, self.digest)

    def test_matching_observations_pass(self):
        self.assertEqual(self.check()['jobs'], 3)

    def test_latest_workflow_requires_exact_contexts(self):
        for mutation in ('missing', 'extra', 'duplicate', 'failed', 'pending', 'wrong-run', 'duplicate-job'):
            with self.subTest(mutation=mutation):
                self.setUp()
                if mutation == 'missing':
                    self.ci['jobs'].pop()
                elif mutation == 'extra':
                    self.ci['jobs'].append(dict(self.ci['jobs'][0], context='unexpected'))
                elif mutation == 'duplicate':
                    self.ci['jobs'][0]['context'] = self.ci['jobs'][1]['context']
                elif mutation in ('failed', 'pending'):
                    self.ci['jobs'][0]['status'] = mutation
                elif mutation == 'wrong-run':
                    self.ci['jobs'][0]['target_url'] = '/cdot/scm-sdk-rs/actions/runs/21/jobs/0'
                else:
                    self.ci['jobs'][0]['target_url'] = self.ci['jobs'][1]['target_url']
                with self.assertRaises(ValueError):
                    self.check()

    def test_stale_sources_and_false_summary_fail(self):
        for field, wrong in [('reviewedSourceSha256', 'stale'), ('sourceCommit', 'e' * 40),
                             ('counts', {'success': 49}), ('expectedJobCount', 49),
                             ('contextsMatchConfiguredMatrix', False)]:
            with self.subTest(field=field):
                self.setUp()
                self.ci[field] = wrong
                with self.assertRaises(ValueError):
                    self.check()
        self.setUp()
        self.ci['run']['status'] = 'running'
        with self.assertRaises(ValueError):
            self.check()

    def test_registry_and_tag_must_bind_qualified_package(self):
        for field, wrong in [('version', '0.2.1'), ('yanked', True), ('ownerLogins', []),
                             ('archiveSha256', 'changed'), ('files', []), ('sourceCommit', 'e' * 40),
                             ('identicalFileContentsQualifiedOnMsrv', False)]:
            with self.subTest(field=field):
                self.setUp()
                self.registry[field] = wrong
                with self.assertRaises(ValueError):
                    self.check()
        for ref in ('refs/tags/v0.2.2', 'refs/tags/v0.2.2^{}', 'refs/heads/main'):
            with self.subTest(ref=ref):
                self.setUp()
                self.tag['remoteRefs'][ref] = 'e' * 40
                with self.assertRaises(ValueError):
                    self.check()

    def test_compiler_proofs_require_identical_source_files(self):
        self.proofs = copy.deepcopy(self.proofs)
        self.proofs[0]['files'] = []
        with self.assertRaises(ValueError):
            self.check()

    def test_workflow_matrix_is_derived_and_fail_closed(self):
        root = pathlib.Path(__file__).resolve().parents[1]
        contexts = expected_contexts((root / '.forgejo/workflows/ci.yml').read_text())
        self.assertEqual(len(contexts), 49)
        self.assertTrue(any('red-team-json-unification' in context for context in contexts))
        for workflow in ('name: Example\n',
                         'name: Example\n toolchain: ["x", "x"]\n features: ["all"]\n'):
            with self.assertRaises(ValueError):
                expected_contexts(workflow)

    def test_git_commit_fingerprint_matches_review_bookkeeping_exclusions(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            subprocess.run(['git', 'init', '-q'], cwd=root, check=True)
            (root / 'source').write_text('source')
            (root / 'docs/checks').mkdir(parents=True)
            (root / 'docs/checks/evidence.json').write_text('{}')
            (root / 'docs/phase-status.json').write_text('{}')
            subprocess.run(['git', 'add', '.'], cwd=root, check=True)
            subprocess.run(['git', '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid',
                            'commit', '-qm', 'Synthetic fixture'], cwd=root, check=True)
            commit = subprocess.check_output(['git', 'rev-parse', 'HEAD'], cwd=root).decode().strip()
            self.assertEqual(commit_fingerprint(root, commit), fingerprint(root))
            verify_source_commits(root, {'sourceCommit': commit, 'ciCommit': commit}, fingerprint(root))
            tree = subprocess.check_output(['git', 'rev-parse', 'HEAD^{tree}'], cwd=root).decode().strip()
            unrelated = subprocess.check_output([
                'git', '-c', 'user.name=Test', '-c', 'user.email=test@example.invalid',
                'commit-tree', tree, '-m', 'Unrelated synthetic source'], cwd=root).decode().strip()
            self.assertEqual(commit_fingerprint(root, unrelated), fingerprint(root))
            with self.assertRaises(subprocess.CalledProcessError):
                verify_source_commits(root, {'sourceCommit': unrelated, 'ciCommit': commit}, fingerprint(root))
            (root / 'docs/checks/evidence.json').write_text('{"updated":true}')
            self.assertEqual(commit_fingerprint(root, commit), fingerprint(root))
            (root / 'source').write_text('different')
            self.assertNotEqual(commit_fingerprint(root, commit), fingerprint(root))
            with self.assertRaisesRegex(ValueError, 'differs from reviewed inputs'):
                verify_source_commits(root, {'sourceCommit': commit, 'ciCommit': commit}, fingerprint(root))

    def test_phase20_cannot_advance_without_delivery_verification(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            subprocess.run(['git', 'init', '-q'], cwd=root, check=True)
            (root / 'docs/reviews').mkdir(parents=True)
            status_path = root / 'docs/phase-status.json'
            status = {'active_phase': 20, 'phases': [
                {'id': 20, 'name': 'Delivery', 'status': 'in_progress', 'reviews': []}]}
            status_path.write_text(json.dumps(status))
            review_path = root / 'docs/reviews/phase-20-attempt-01.json'
            review_path.write_text(json.dumps({'review_scope': 'phase'}))
            with patch('review_phase.verify_record', return_value=True), \
                    patch('review_phase.accepted_chunks', return_value=[]), \
                    patch('release_delivery.verify', side_effect=ValueError('Delivery incomplete')) as guard, \
                    contextlib.redirect_stdout(io.StringIO()):
                with self.assertRaisesRegex(ValueError, 'Delivery incomplete'):
                    main(['--phase', '20', '--verify', str(review_path), '--advance'], root)
                guard.assert_called_once_with(root, fingerprint(root))
            self.assertEqual(json.loads(status_path.read_text()), status)


    def test_readiness_can_be_reviewed_before_delivery_but_cannot_advance(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            subprocess.run(['git', 'init', '-q'], cwd=root, check=True)
            (root / '.gitignore').write_text('/.local/\n')
            (root / 'docs/checks').mkdir(parents=True)
            status_path = root / 'docs/phase-status.json'
            status = {'active_phase': 20, 'phases': [
                {'id': 20, 'name': 'Delivery', 'status': 'in_progress', 'reviews': [],
                 'chunks': [{'id': 'qualification-and-package'}, {'id': 'registry-and-remote-delivery'}]}]}
            status_path.write_text(json.dumps(status))
            checks = root / 'docs/checks/phase-20.json'
            checks.write_text('Synthetic check evidence')
            review = {'score': 9.2, 'summary': 'Synthetic release readiness',
                      'findings': [], 'evidence': ['Synthetic qualification'],
                      'limitations': ['Publication and terminal remote CI are pending'],
                      'categories': {'implementation': 10, 'code_quality': 9, 'supportability': 9,
                                     'architecture': 9, 'feature_delivery': 9}, 'total': 46}
            actual_run = subprocess.run
            def mock_run(command, **kwargs):
                if command[0] == 'claude':
                    self.assertIn('RELEASE READINESS assessment, not phase completion', kwargs['input'])
                    return subprocess.CompletedProcess(command, 0,
                        json.dumps({'is_error': False, 'structured_output': review}), '')
                return actual_run(command, **kwargs)
            with patch('review_phase.check_evidence', return_value=checks), \
                    patch('review_phase.subprocess.run', side_effect=mock_run), \
                    contextlib.redirect_stdout(io.StringIO()):
                self.assertEqual(main(['--phase', '20', '--release-readiness'], root), 0)
                path = root / 'docs/reviews/phase-20-attempt-01.json'
                self.assertTrue(verify_release_readiness(root, path, fingerprint(root)))
                self.assertEqual(json.loads(path.read_text())['review_scope'], 'release_readiness')
                with self.assertRaisesRegex(ValueError, 'does not complete delivery'):
                    main(['--phase', '20', '--verify', str(path), '--advance'], root)
                with self.assertRaisesRegex(ValueError, 'already exists'):
                    main(['--phase', '20'], root)
                # A source-bound phase record cannot masquerade as publish authorization.
                record = json.loads(path.read_text())
                record['review_scope'] = 'phase'
                path.write_text(json.dumps(record))
                self.assertFalse(verify_release_readiness(root, path, fingerprint(root)))
                self.assertFalse(verify_record(root, path, 20, fingerprint(root)))
            self.assertEqual(json.loads(status_path.read_text()), status)

    def test_phase20_chunk_scope_is_declared_recorded_and_prompt_bound(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            subprocess.run(['git', 'init', '-q'], cwd=root, check=True)
            (root / '.gitignore').write_text('/.local/\n')
            (root / 'source.rs').write_text('source')
            (root / 'other.rs').write_text('other')
            (root / 'docs/checks').mkdir(parents=True)
            status_path = root / 'docs/phase-status.json'
            status = {'active_phase': 20, 'phases': [
                {'id': 20, 'name': 'Delivery', 'status': 'in_progress', 'reviews': [],
                 'chunks': [{'id': 'qualification', 'scope_evidence': ['source.rs', 'other.rs']}]}]}
            status_path.write_text(json.dumps(status))
            checks = root / 'docs/checks/phase-20.json'
            checks.write_text('Synthetic check evidence')
            review = {'score': 9.2, 'summary': 'Synthetic qualification review',
                      'findings': [], 'evidence': ['Synthetic qualification'], 'limitations': [],
                      'categories': {'implementation': 10, 'code_quality': 9, 'supportability': 9,
                                     'architecture': 9, 'feature_delivery': 9}, 'total': 46}
            actual_run = subprocess.run
            def mock_run(command, **kwargs):
                if command[0] == 'claude':
                    return subprocess.CompletedProcess(command, 0,
                        json.dumps({'is_error': False, 'structured_output': review}), '')
                return actual_run(command, **kwargs)
            with patch('review_phase.check_evidence', return_value=checks), \
                    patch('review_phase.subprocess.run', side_effect=mock_run), \
                    contextlib.redirect_stdout(io.StringIO()):
                with self.assertRaisesRegex(ValueError, 'differs from its declared scope'):
                    main(['--phase', '20', '--chunk', 'qualification', '--scope-evidence', 'source.rs'], root)
                self.assertFalse(list((root / 'docs').glob('reviews/*.checks.json')))
                self.assertEqual(main(['--phase', '20', '--chunk', 'qualification',
                                       '--scope-evidence', 'source.rs', 'other.rs'], root), 0)
                path = root / 'docs/reviews/phase-20-chunk-qualification-attempt-01.json'
                record = json.loads(path.read_text())
                self.assertEqual(record['scope_evidence'], ['source.rs', 'other.rs'])
                self.assertTrue(verify_record(root, path, 20, fingerprint(root), 'qualification'))
                # Bookkeeping cannot silently broaden or narrow the accepted scope.
                status['phases'][0]['chunks'][0]['scope_evidence'] = ['source.rs']
                status_path.write_text(json.dumps(status))
                self.assertFalse(verify_record(root, path, 20, fingerprint(root), 'qualification'))
                # Editing both bookkeeping and the record still cannot change the
                # preserved prompt that actually instructed the reviewer.
                record['scope_evidence'] = ['source.rs']
                path.write_text(json.dumps(record))
                self.assertFalse(verify_record(root, path, 20, fingerprint(root), 'qualification'))
