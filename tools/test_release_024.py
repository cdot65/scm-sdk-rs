"""Version-specific 0.2.4 release guards; historical release defaults stay intact."""
import contextlib
import copy
import hashlib
import io
import json
import pathlib
import subprocess
import tempfile
import sys
import unittest
from unittest.mock import Mock, patch

import check_phase
import review_phase
import release_delivery
import test_management_package as package_fixture
from test_release_023 import load
import test_release_delivery

publication = load('publication024', 'verify-publication.py')
prepublish = load('prepublication024', 'check-prepublication.py')
observer = load('observer024', 'observe-release.py')
candidate = load('candidate024', 'check-access-release.py')


class Release024Tests(unittest.TestCase):
    def test_release_documentation_matches_candidate_and_delivery_guards(self):
        root = pathlib.Path(__file__).resolve().parents[1]
        readme = (root / 'README.md').read_text()
        self.assertIn('current verified published version is [0.2.3]', readme)
        self.assertIn('docs/checks/release-registry-0.2.3.json', readme)
        self.assertIn('Crate version: **0.2.4**', readme)
        self.assertIn('candidate, not yet published', readme)
        self.assertIn('Until publication is verified, retain your existing released pin', readme)
        phases = (root / 'docs/PHASES.md').read_text()
        self.assertIn('Phase 28 acceptance requires both named chunks', phases)
        self.assertIn('`release-readiness` and\n`remote-delivery`', phases)
        ci = (root / 'docs/CI.md').read_text()
        self.assertIn('For active phases 23–25 and 27,', ci)
        self.assertIn('Phase 28 does not permit `--resume`', ci)
        release = (root / 'docs/RELEASE.md').read_text().split(
            '### Execute the reviewed 0.2.4 delivery', 1)[1]
        for command in ('cargo +1.99.0 publish -p scm-sdk --locked',
                        'scripts/verify-publication.py --version 0.2.4',
                        '--proof docs/checks/phase-28-release-package-1.99.0.json',
                        '--msrv-proof docs/checks/phase-28-release-package-1.89.0.json',
                        'scripts/observe-release.py tag --version 0.2.4',
                        'scripts/observe-release.py ci --version 0.2.4',
                        'tools/release_delivery.py --phase 28 --version 0.2.4'):
            self.assertIn(command, release)

    def test_explicit_contexts_preserve_old_defaults(self):
        self.assertIn((28, '0.2.4'), candidate.RELEASES)
        self.assertIn((28, '0.2.4'), release_delivery.RELEASES)
        self.assertNotIn((24, '0.2.4'), candidate.RELEASES)
        self.assertEqual(prepublish.verify.__defaults__, (24, '0.2.3'))
        with patch.object(prepublish, 'module') as module:
            with self.assertRaises(ValueError):
                prepublish.verify(pathlib.Path('review'), phase=24, version='0.2.4')
            module.assert_not_called()

    def test_readiness_uses_phase28_and_prevents_network_without_review(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            path = root / 'review.json'
            path.write_text('{}')
            with patch.object(publication, 'ROOT', root), patch.object(review_phase, 'fingerprint', return_value='a' * 64), \
                    patch.object(review_phase, 'verify_release_readiness', return_value=True) as verify:
                publication.require_release_readiness('0.2.4', path)
                verify.assert_called_once_with(root, path, 'a' * 64, phase=28)
            with patch('sys.argv', ['verify-publication', '--version', '0.2.4', '--proof', 'missing', '--output', str(root / 'out')]), \
                    patch.object(publication, 'fetch') as fetch:
                with self.assertRaisesRegex(ValueError, 'readiness-review'):
                    publication.main()
                fetch.assert_not_called()

    def test_prepublication_checks_both_readiness_and_current_candidate(self):
        registry, archive = Mock(), Mock()
        registry.require_release_readiness.return_value = {'sourceSha256': 'a' * 64}
        archive.verify.return_value = {'releaseCommit': 'b' * 40, 'archiveSha256': 'c' * 64}
        with patch.object(prepublish, 'module', side_effect=[registry, archive]):
            result = prepublish.verify(pathlib.Path('review'), phase=28, version='0.2.4')
        archive.verify.assert_called_once_with(prepublish.ROOT, phase=28, version='0.2.4')
        self.assertEqual(registry.require_release_readiness.call_count, 2)
        self.assertEqual(result['version'], '0.2.4')
        self.assertFalse(result['publicationPerformed'])

    def test_contract_subset_diagnostic_lists_all_supported_phases(self):
        stderr = io.StringIO()
        with patch.object(sys, 'argv', ['check_phase.py', '--phase', '28', '--contracts-only']), \
                contextlib.redirect_stderr(stderr):
            with self.assertRaises(SystemExit) as failure:
                check_phase.main()
        self.assertEqual(failure.exception.code, 2)
        self.assertIn(', '.join(map(str, check_phase.CONTRACT_CI_PHASES)), stderr.getvalue())
        self.assertIn('--verification-only', stderr.getvalue())

    def test_phase28_commands_require_both_directional_packages(self):
        commands = check_phase.commands_for(28)
        for compiler in ('1.89.0', '1.99.0'):
            self.assertIn(['python', 'scripts/check-management-package.py', '--require-directional',
                           '--toolchain', compiler, '--output', f'docs/checks/phase-28-release-package-{compiler}.json'], commands)
        self.assertEqual(commands[-1], ['python', 'scripts/check-access-release.py', '--phase', '28', '--version', '0.2.4',
                                       '--output', 'docs/checks/phase-28-release-readiness.json'])
        self.assertFalse(any('scripts/check-management-package.py' in c for c in check_phase.ci_commands_for(28)))
        self.assertTrue(any('tools/typescript035_profiles.py' in c for c in commands))

    def test_phase28_readiness_is_explicit_and_cannot_complete_delivery(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            subprocess.run(['git', 'init', '-q'], cwd=root, check=True)
            (root / 'docs/reviews').mkdir(parents=True)
            status = {'active_phase': 28, 'phases': [
                {'id': 28, 'name': 'Delivery', 'status': 'in_progress', 'reviews': []}]}
            status_path = root / 'docs/phase-status.json'
            status_path.write_text(json.dumps(status))
            review = root / 'docs/reviews/phase-28-attempt-01.json'
            review.write_text(json.dumps({'review_scope': 'release_readiness'}))
            with patch.object(review_phase, 'verify_record', return_value=True) as verify:
                self.assertTrue(review_phase.verify_release_readiness(root, review, 'a' * 64, phase=28))
                verify.assert_called_once_with(root, review, 28, 'a' * 64)
            with patch.object(review_phase, 'validate_predecessors'), \
                    patch.object(review_phase, 'verify_record', return_value=True), \
                    patch.object(review_phase, 'accepted_chunks', return_value=[]), \
                    patch.object(release_delivery, 'verify', side_effect=ValueError('Delivery incomplete')) as guard, \
                    contextlib.redirect_stdout(io.StringIO()):
                with self.assertRaisesRegex(ValueError, 'does not complete delivery'):
                    review_phase.main(['--phase', '28', '--verify', str(review), '--advance'], root)
                guard.assert_not_called()
                review.write_text(json.dumps({'review_scope': 'phase'}))
                with self.assertRaisesRegex(ValueError, 'Delivery incomplete'):
                    review_phase.main(['--phase', '28', '--verify', str(review), '--advance'], root)
                guard.assert_called_once_with(root, review_phase.fingerprint(root), phase=28, version='0.2.4')
            self.assertEqual(json.loads(status_path.read_text()), status)
            with self.assertRaisesRegex(ValueError, 'without chunk, verify, or advance'):
                review_phase.main(['--phase', '28', '--release-readiness', '--advance'], root)

    def test_phase28_archives_require_the_qualified_commit_worktree(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            commit = 'a' * 40
            expected = root / '.local/package-worktrees' / commit / '.local/artifacts/package.crate'
            expected.parent.mkdir(parents=True)
            expected.write_bytes(b'archive')
            proof = {'releaseCommit': commit, 'retainedArchive': str(expected)}
            self.assertEqual(release_delivery.retained_package_archive(root, proof, 28), expected)
            other = root / '.local/package-worktrees' / ('b' * 40) / '.local/artifacts/package.crate'
            other.parent.mkdir(parents=True)
            other.write_bytes(b'archive')
            with self.assertRaises(ValueError):
                release_delivery.retained_package_archive(root, dict(proof, retainedArchive=str(other)), 28)

    def test_publication_requires_both_directional_package_verifications(self):
        with self.assertRaisesRegex(ValueError, 'both clean compiler'):
            publication.verify_management_proofs(pathlib.Path('current'), None, require_directional=True)
        module = Mock()
        with patch.object(publication.importlib.util, 'module_from_spec', return_value=module), \
                patch.object(publication.importlib.util, 'spec_from_file_location') as spec:
            spec.return_value.loader.exec_module.return_value = None
            publication.verify_management_proofs(pathlib.Path('current'), pathlib.Path('msrv'), require_directional=True)
        self.assertEqual(module.verify_receipt.call_count, 2)
        module.verify_receipt.assert_any_call(publication.ROOT, pathlib.Path('current'), '1.99.0', require_directional=True)
        module.verify_receipt.assert_any_call(publication.ROOT, pathlib.Path('msrv'), '1.89.0', require_directional=True)

    def test_remote_observer_requests_only_selected_annotated_tag(self):
        source = 'a' * 40
        stdout = f"{'b' * 40}\trefs/tags/v0.2.4\n{source}\trefs/tags/v0.2.4^{{}}\n{source}\trefs/heads/main\n"
        with tempfile.TemporaryDirectory() as directory, patch.object(observer.subprocess, 'run', return_value=Mock(stdout=stdout)) as run:
            output = pathlib.Path(directory) / 'tag.json'
            with contextlib.redirect_stdout(io.StringIO()):
                observer.remote_tag(source, output, '0.2.4')
            self.assertEqual(json.loads(output.read_text())['tag'], 'v0.2.4')
            self.assertIn('refs/tags/v0.2.4^{}', run.call_args.args[0])
            with self.assertRaises(FileExistsError):
                observer.remote_tag(source, output, '0.2.4')

    def test_ci_observer_handles_fifty_row_pages_and_rejects_exhaustion(self):
        source = 'a' * 40
        workflow = 'name: Test\n  toolchain: ["1.89.0", "1.99.0"]\n  features: ["all"]\n'
        contexts = release_delivery.expected_contexts(workflow)
        run = {'index_in_repo': 42, 'status': 'success', 'commit_sha': source,
               'html_url': 'https://git.cdot.io/cdot/scm-sdk-rs/actions/runs/42'}
        unrelated = [{'context': 'other', 'status': 'success', 'target_url': '/other'}] * 50
        rows = [{'context': name, 'status': 'success',
                 'target_url': f'/cdot/scm-sdk-rs/actions/runs/42/jobs/{i}'}
                for i, name in enumerate(sorted(contexts))]
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / '.forgejo/workflows').mkdir(parents=True)
            (root / '.forgejo/workflows/ci.yml').write_text(workflow)
            for exhausted in (False, True):
                requested = []
                opener = Mock()
                def opened(request, timeout):
                    url = request.full_url
                    requested.append(url)
                    if '/actions/runs?' in url:
                        payload = {'workflow_runs': [run]}
                    else:
                        self.assertIn('limit=50', url)
                        payload = unrelated if exhausted or url.endswith('page=1') else rows
                    response = Mock()
                    response.__enter__ = Mock(return_value=response)
                    response.__exit__ = Mock(return_value=False)
                    response.read.return_value = json.dumps(payload).encode()
                    return response
                opener.open.side_effect = opened
                output = root / ('exhausted.json' if exhausted else 'success.json')
                with patch.object(observer, 'ROOT', root), \
                        patch.object(observer, 'fingerprint', return_value='b' * 64), \
                        patch.object(observer, 'commit_fingerprint', return_value='b' * 64), \
                        patch.dict(observer.os.environ, {'FORGEJO_TOKEN': 'synthetic-test-token'}), \
                        patch.object(observer.urllib.request, 'build_opener', return_value=opener), \
                        contextlib.redirect_stdout(io.StringIO()):
                    if exhausted:
                        with self.assertRaisesRegex(ValueError, 'pagination limit'):
                            observer.terminal_ci(source, 42, output)
                        self.assertFalse(output.exists())
                        self.assertEqual(sum('/statuses/' in url for url in requested), 20)
                    else:
                        observer.terminal_ci(source, 42, output)
                        self.assertEqual(json.loads(output.read_text())['expectedJobCount'], 3)
                        self.assertTrue(any(url.endswith('page=2') for url in requested))

    def test_delivery_rejects_historical_version_and_incomplete_ci(self):
        fixture = test_release_delivery.DeliveryTests()
        fixture.setUp()
        fixture.registry['version'] = '0.2.4'
        fixture.tag['tag'] = 'v0.2.4'
        fixture.tag['remoteRefs'] = {k.replace('0.2.2', '0.2.4'): v for k, v in fixture.tag['remoteRefs'].items()}
        args = (fixture.registry, fixture.tag, fixture.ci, fixture.proofs, fixture.contexts, fixture.digest)
        self.assertTrue(release_delivery.validate_receipts(*args, version='0.2.4')['passed'])
        with self.assertRaises(ValueError):
            release_delivery.validate_receipts(*args, version='0.2.3')
        fixture.ci['jobs'].pop()
        with self.assertRaises(ValueError):
            release_delivery.validate_receipts(*args, version='0.2.4')

    def test_directional_consumer_requires_exact_archive_and_external_execution(self):
        fixture = package_fixture.ManagementPackageTests()
        fixture.setUp()
        self.addCleanup(fixture.doCleanups)
        member = 'examples/directional_profiles.rs'
        content = b'fn main() {}\n'
        fixture.files[member] = content
        fixture.write('crates/scm-sdk/' + member, content)
        fixture.report['files'] = [{'path': name, 'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data)}
                                   for name, data in sorted(fixture.files.items())]
        checks = fixture.report['checks']
        checks.append({'command': ['cargo', '+1.99.0', 'run', '--example', 'directional_profiles', '--all-features', '--locked', '--offline', '--target', fixture.host],
                       'cwd': str(fixture.root / 'archive/scm-sdk'), 'exit_code': 0})
        checks.append({'command': ['cargo', '+1.99.0', 'run', '--locked', '--offline', '--target', fixture.host],
                       'cwd': str(fixture.root / 'archive/consumer'), 'exit_code': 0})
        fixture.report['externalExampleConsumers'][member] = {'sha256': hashlib.sha256(content).hexdigest(), 'checkIndex': len(checks)-1, 'selectedFeatures': []}
        def verify(report):
            fixture.receipt.write_text(json.dumps(report))
            return package_fixture.package.verify_receipt(fixture.root, fixture.receipt, '1.99.0', require_directional=True)
        self.assertTrue(verify(fixture.report)['passed'])
        for field, wrong in [('sha256', '0' * 64), ('checkIndex', fixture.report['frozenProbeCheckIndex']), ('selectedFeatures', ['red-team'])]:
            changed = copy.deepcopy(fixture.report)
            changed['externalExampleConsumers'][member][field] = wrong
            with self.subTest(field=field), self.assertRaises(ValueError):
                verify(changed)
        changed = copy.deepcopy(fixture.report)
        changed['externalExampleConsumers'].pop(member)
        with self.assertRaises(ValueError):
            verify(changed)


if __name__ == '__main__':
    unittest.main()
