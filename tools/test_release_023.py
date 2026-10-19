"""Current management release guard regressions, separate from historical receipts."""
import copy
import contextlib
import io
import subprocess
import hashlib
import importlib.util
import json
import pathlib
import tempfile
import unittest
from unittest.mock import Mock, patch

import release_delivery
import review_phase
import test_release_delivery

ROOT = pathlib.Path(__file__).resolve().parents[1]


def load(name, file):
    spec = importlib.util.spec_from_file_location(name, ROOT / 'scripts' / file)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


publication = load('publication023', 'verify-publication.py')
observer = load('observer023', 'observe-release.py')
prepublish = load('prepublish023', 'check-prepublication.py')


class ManagementDeliveryTests(unittest.TestCase):
    def test_archive_location_binds_exact_commit_even_with_identical_bytes(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            commit = 'a' * 40
            expected = root / '.local/package-worktrees' / commit / '.local/artifacts/package.crate'
            other = root / '.local/package-worktrees' / ('b' * 40) / '.local/artifacts/package.crate'
            historical = root / '.local/artifacts/package.crate'
            for path in (expected, other, historical):
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_bytes(b'identical qualified archive bytes')
            proof = {'releaseCommit': commit, 'retainedArchive': str(expected),
                     'archive_sha256': hashlib.sha256(expected.read_bytes()).hexdigest()}
            self.assertEqual(release_delivery.retained_package_archive(root, proof, 24), expected)
            for path in (other, historical):
                changed = dict(proof, retainedArchive=str(path))
                self.assertEqual(hashlib.sha256(path.read_bytes()).hexdigest(), proof['archive_sha256'])
                with self.assertRaisesRegex(ValueError, 'qualified source location'):
                    release_delivery.retained_package_archive(root, changed, 24)
            old_proof = dict(proof, retainedArchive=str(historical))
            for phase in (17, 20):
                self.assertEqual(release_delivery.retained_package_archive(root, old_proof, phase), historical)
                with self.assertRaises(ValueError):
                    release_delivery.retained_package_archive(root, proof, phase)

    def test_current_context_and_historical_rejection(self):
        fixture = test_release_delivery.DeliveryTests()
        fixture.setUp()
        fixture.registry['version'] = '0.2.3'
        fixture.tag['tag'] = 'v0.2.3'
        fixture.tag['remoteRefs'] = {key.replace('0.2.2', '0.2.3'): value
                                    for key, value in fixture.tag['remoteRefs'].items()}
        args = (fixture.registry, fixture.tag, fixture.ci, fixture.proofs, fixture.contexts, fixture.digest)
        self.assertEqual(release_delivery.validate_receipts(*args, version='0.2.3')['version'], '0.2.3')
        with self.assertRaises(ValueError):
            release_delivery.validate_receipts(*args)
        for mutation in ('wrong-tag', 'pending-ci', 'wrong-source'):
            tag, ci = copy.deepcopy(fixture.tag), copy.deepcopy(fixture.ci)
            if mutation == 'wrong-tag':
                tag['remoteRefs']['refs/tags/v0.2.3^{}'] = 'f' * 40
            elif mutation == 'pending-ci':
                ci['run']['status'] = 'waiting'
            else:
                ci['reviewedSourceSha256'] = 'f' * 64
            with self.subTest(mutation=mutation), self.assertRaises(ValueError):
                release_delivery.validate_receipts(fixture.registry, tag, ci, fixture.proofs,
                                                    fixture.contexts, fixture.digest, '0.2.3')

    def test_readiness_requires_explicit_phase24_and_actual_accepted_record(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            path = root / 'docs/reviews/phase-24-attempt-01.json'
            path.parent.mkdir(parents=True)
            path.write_text(json.dumps({'review_scope': 'release_readiness'}))
            with patch.object(review_phase, 'verify_record', return_value=True) as verifier:
                self.assertTrue(review_phase.verify_release_readiness(root, path, 'a' * 64, phase=24))
                verifier.assert_called_once_with(root, path, 24, 'a' * 64)
                self.assertFalse(review_phase.verify_release_readiness(root, path, 'a' * 64, phase=23))
            with patch.object(publication, 'ROOT', root), patch.object(review_phase, 'fingerprint', return_value='a' * 64), \
                    patch.object(review_phase, 'verify_release_readiness', return_value=True) as verifier:
                result = publication.require_release_readiness('0.2.3', path)
                verifier.assert_called_once_with(root, path, 'a' * 64, phase=24)
                self.assertEqual(result['sourceSha256'], 'a' * 64)
            path.write_text(json.dumps({'review_scope': 'phase'}))
            self.assertFalse(review_phase.verify_release_readiness(root, path, 'a' * 64, phase=24))

    def test_no_023_registry_network_without_readiness_or_msrv(self):
        with tempfile.TemporaryDirectory() as directory:
            output = pathlib.Path(directory) / 'result.json'
            with patch('sys.argv', ['verify-publication.py', '--version', '0.2.3', '--proof', 'missing.json',
                                    '--output', str(output)]), patch.object(publication, 'fetch') as fetch:
                with self.assertRaisesRegex(ValueError, 'requires --readiness-review'):
                    publication.main()
                fetch.assert_not_called()
            with self.assertRaisesRegex(ValueError, 'both clean compiler'):
                publication.verify_management_proofs(pathlib.Path('proof.json'), None)

    def test_retained_registry_is_rehashed(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            archive = root / '.local/artifacts/registry.crate'
            archive.parent.mkdir(parents=True)
            archive.write_bytes(b'archive')
            registry = {'retainedArchive': str(archive.relative_to(root)), 'version': '0.2.3',
                        'archiveSha256': hashlib.sha256(b'archive').hexdigest(), 'archiveBytes': 7, 'files': []}
            policy = Mock()
            policy.file_rows.return_value = []
            release_delivery.verify_retained_registry(root, registry, policy)
            archive.write_bytes(b'changed')
            with self.assertRaisesRegex(ValueError, 'changed'):
                release_delivery.verify_retained_registry(root, registry, policy)
            with self.assertRaises(ValueError):
                release_delivery.verify_retained_registry(root, dict(registry, retainedArchive='../outside'), policy)

    def test_prepublication_runs_readiness_and_candidate_guard_twice_binds_readiness(self):
        publication_mock, candidate = Mock(), Mock()
        publication_mock.require_release_readiness.return_value = {'sourceSha256': 'a' * 64}
        candidate.verify.return_value = {'releaseCommit': 'b' * 40, 'archiveSha256': 'c' * 64}
        with patch.object(prepublish, 'module', side_effect=[publication_mock, candidate]):
            result = prepublish.verify(pathlib.Path('review.json'))
            self.assertFalse(result['publicationPerformed'])
            candidate.verify.assert_called_once_with(prepublish.ROOT, phase=24, version='0.2.3')
            self.assertEqual(publication_mock.require_release_readiness.call_count, 2)
        publication_mock.require_release_readiness.side_effect = [{'sourceSha256': 'a'}, {'sourceSha256': 'b'}]
        with patch.object(prepublish, 'module', side_effect=[publication_mock, candidate]), self.assertRaises(ValueError):
            prepublish.verify(pathlib.Path('review.json'))

    def test_tag_observer_only_accepts_annotated_exact_source_and_preserves_receipt(self):
        source = 'a' * 40
        with tempfile.TemporaryDirectory() as directory:
            output = pathlib.Path(directory) / 'tag.json'
            stdout = f"{'b' * 40}\trefs/tags/v0.2.3\n{source}\trefs/tags/v0.2.3^{{}}\n{source}\trefs/heads/main\n"
            with patch.object(observer.subprocess, 'run', return_value=Mock(stdout=stdout)) as run:
                observer.remote_tag(source, output)
                self.assertEqual(run.call_args.args[0][1], 'ls-remote')
                with self.assertRaises(FileExistsError):
                    observer.remote_tag(source, output)
            with patch.object(observer.subprocess, 'run', return_value=Mock(stdout=stdout.replace(source, 'c' * 40))):
                with self.assertRaises(ValueError):
                    observer.remote_tag(source, pathlib.Path(directory) / 'bad.json')

    def test_phase24_readiness_cannot_advance_and_phase_requires_delivery(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            subprocess.run(['git', 'init', '-q'], cwd=root, check=True)
            (root / 'docs/reviews').mkdir(parents=True)
            status = {'active_phase': 24, 'phases': [
                {'id': 24, 'name': 'Delivery', 'status': 'in_progress', 'reviews': []}]}
            status_path = root / 'docs/phase-status.json'
            status_path.write_text(json.dumps(status))
            review = root / 'docs/reviews/phase-24-attempt-01.json'
            with patch.object(review_phase, 'verify_record', return_value=True), \
                    patch.object(review_phase, 'accepted_chunks', return_value=[]), \
                    patch.object(release_delivery, 'verify', side_effect=ValueError('Delivery incomplete')) as guard, \
                    contextlib.redirect_stdout(io.StringIO()):
                review.write_text(json.dumps({'review_scope': 'release_readiness'}))
                with self.assertRaisesRegex(ValueError, 'does not complete delivery'):
                    review_phase.main(['--phase', '24', '--verify', str(review), '--advance'], root)
                guard.assert_not_called()
                review.write_text(json.dumps({'review_scope': 'phase'}))
                with self.assertRaisesRegex(ValueError, 'Delivery incomplete'):
                    review_phase.main(['--phase', '24', '--verify', str(review), '--advance'], root)
                guard.assert_called_once_with(root, review_phase.fingerprint(root), phase=24, version='0.2.3')
            self.assertEqual(json.loads(status_path.read_text()), status)

    def test_ci_observer_records_only_complete_terminal_matrix(self):
        source = 'c' * 40
        workflow = 'name: Example\n  toolchain: ["1.89.0", "1.99.0"]\n  features: ["all"]\n'
        contexts = release_delivery.expected_contexts(workflow)
        run = {'index_in_repo': 42, 'commit_sha': source, 'status': 'success',
               'html_url': 'https://git.cdot.io/cdot/scm-sdk-rs/actions/runs/42'}
        statuses = [{'context': name, 'status': 'success',
                     'target_url': f'/cdot/scm-sdk-rs/actions/runs/42/jobs/{index}'}
                    for index, name in enumerate(sorted(contexts))]
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / '.forgejo/workflows').mkdir(parents=True)
            (root / '.forgejo/workflows/ci.yml').write_text(workflow)
            for mode in ('success', 'pending', 'missing', 'wrong-run', 'duplicate-job'):
                rows = copy.deepcopy(statuses)
                if mode == 'pending':
                    rows[0]['status'] = 'pending'
                elif mode == 'missing':
                    rows.pop()
                elif mode == 'wrong-run':
                    rows[0]['target_url'] = '/cdot/scm-sdk-rs/actions/runs/41/jobs/0'
                elif mode == 'duplicate-job':
                    rows[0]['target_url'] = rows[1]['target_url']
                opener = Mock()
                def opened(request, timeout):
                    response = Mock()
                    response.__enter__ = Mock(return_value=response)
                    response.__exit__ = Mock(return_value=False)
                    response.read.return_value = json.dumps({'workflow_runs': [run]} if '/actions/runs?' in request.full_url else rows).encode()
                    self.assertTrue(request.full_url.startswith(observer.BASE))
                    return response
                opener.open.side_effect = opened
                with self.subTest(mode=mode), patch.object(observer, 'ROOT', root), \
                        patch.object(observer, 'fingerprint', return_value='a' * 64), \
                        patch.object(observer, 'commit_fingerprint', return_value='a' * 64), \
                        patch.dict(observer.os.environ, {'FORGEJO_TOKEN': 'synthetic-test-token'}), \
                        patch.object(observer.urllib.request, 'build_opener', return_value=opener):
                    output = root / (mode + '.json')
                    if mode == 'success':
                        observer.terminal_ci(source, 42, output)
                        receipt = json.loads(output.read_text())
                        self.assertEqual(receipt['expectedJobCount'], 3)
                        self.assertNotIn('synthetic-test-token', output.read_text())
                    else:
                        with self.assertRaises(ValueError):
                            observer.terminal_ci(source, 42, output)
                        self.assertFalse(output.exists())

    def test_ci_observer_refuses_wrong_source_before_network(self):
        with patch.object(observer, 'fingerprint', return_value='a' * 64), \
                patch.object(observer, 'commit_fingerprint', return_value='b' * 64), \
                patch.object(observer.urllib.request, 'build_opener') as network:
            with self.assertRaises(ValueError):
                observer.terminal_ci('c' * 40, 1, pathlib.Path('never.json'))
            network.assert_not_called()


if __name__ == '__main__':
    unittest.main()
