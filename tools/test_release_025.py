"""Phase 31 / 0.2.5 release guards: Codex readiness binding, gate order, version pairs."""
import importlib.util
import json
import pathlib
import tempfile
import unittest
from unittest import mock

import codex_review
import release_delivery
import review_phase
import subprocess

import check_phase
from check_phase import commands_for, ci_commands_for, portkey_mcp_post_publication_commands, portkey_mcp_release_commands

ROOT = pathlib.Path(__file__).resolve().parents[1]


def load(name, relative):
    spec = importlib.util.spec_from_file_location(name, ROOT / relative)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def review(score):
    return {'score': score, 'summary': 'ok', 'findings': [], 'evidence': ['x'], 'limitations': []}


class Release025Tests(unittest.TestCase):
    def test_release_pairs_are_registered_together(self):
        self.assertIn((31, '0.2.5'), load('candidate', 'scripts/check-access-release.py').RELEASES)
        self.assertIn((31, '0.2.5'), release_delivery.RELEASES)
        source = (ROOT / 'scripts/check-prepublication.py').read_text()
        self.assertIn('(31, "0.2.5")', source)
        self.assertIn('"0.2.5": 31', (ROOT / 'scripts/verify-publication.py').read_text())

    def test_gate_runs_clean_packages_then_readiness_last_and_ci_subset_excludes_them(self):
        commands = portkey_mcp_release_commands(31)
        readiness = ['python', 'scripts/check-access-release.py', '--phase', '31', '--version', '0.2.5',
                     '--output', 'docs/checks/phase-31-release-readiness.json']
        self.assertEqual(commands[-1], readiness)
        for compiler in ('1.89.0', '1.99.0'):
            package = ['python', 'scripts/check-management-package.py', '--require-directional', '--toolchain', compiler,
                       '--output', f'docs/checks/phase-31-release-package-{compiler}.json']
            self.assertLess(commands.index(package), commands.index(readiness))
        with mock.patch.object(check_phase, 'published', return_value=False):
            self.assertEqual(commands_for(31), commands)
            subset = ci_commands_for(31)
        self.assertFalse(any('scripts/check-management-package.py' in c or 'scripts/check-access-release.py' in c for c in subset))
        self.assertIn(['python', 'scripts/check-private-material.py', '--check', 'docs/checks/phase-31-private-material.json'], subset)
        live = json.loads((ROOT / 'contracts/vendor/portkey-mcp-tags/8e5da0e/operations.json').read_text())['liveReceipt']
        self.assertIn(['python', 'tools/mcp_tags_live.py', '--check', live], commands)

    def test_post_publication_gate_verifies_delivery_and_site_without_regenerating_release_proofs(self):
        commands = portkey_mcp_post_publication_commands(31)
        with mock.patch.object(check_phase, 'published', return_value=True):
            self.assertEqual(commands_for(31), commands)
            subset = ci_commands_for(31)
        self.assertEqual(commands[-2:], [['python', 'tools/release_delivery.py', '--phase', '31', '--version', '0.2.5'],
                                         ['python', 'tools/site_binding.py', '--version', '0.2.5', '--check']])
        self.assertFalse(any('scripts/check-management-package.py' in c or 'scripts/check-access-release.py' in c
                             for c in commands), 'release-bound proofs must never be regenerated after publication')
        self.assertEqual(commands[:-2], [c for c in portkey_mcp_release_commands(31)
                                         if 'scripts/check-management-package.py' not in c
                                         and 'scripts/check-access-release.py' not in c])
        self.assertFalse(any('tools/release_delivery.py' in c or 'tools/site_binding.py' in c for c in subset))

    def test_readiness_requires_the_latest_accepted_codex_record_for_current_source(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / 'docs/checks').mkdir(parents=True)
            report = {'phase': 31, 'passed': True, 'source_sha256': 's1', 'source_unchanged': True,
                      'checks': [{'command': ['python', 'x'], 'exit_code': 0}]}
            (root / 'docs/checks/phase-31.json').write_text(json.dumps(report))
            policy = {'reviewer': 'codex', 'minimum_score': 9}
            chunks = [{'id': name, 'status': 'pending', 'scope_evidence': ['a']} for name in codex_review.REQUIRED_CHUNKS[31]]
            status = {'active_phase': 31, 'phases': [{'id': 31, 'name': 'release', 'status': 'in_progress',
                                                       'review_policy': policy, 'scope_evidence': ['a'], 'chunks': chunks}]}
            (root / 'docs/phase-status.json').write_text(json.dumps(status))

            def run(score, source='s1'):
                def runner(prompt, out, events, schema, _root):
                    events.write_text('{}\n')
                    out.write_text(json.dumps(review(score)))
                    return 0
                with mock.patch.object(codex_review, 'fingerprint', return_value=source), \
                        mock.patch.object(codex_review, 'check_evidence', return_value=root / 'docs/checks/phase-31.json'):
                    codex_review.main(['--phase', '31', '--chunk', 'release-readiness'], root=root, runner=runner)

            first = codex_review.destination(root, 31, 1, 'release-readiness')
            run(9)
            self.assertTrue(review_phase.verify_release_readiness(root, first, 's1', phase=31))
            self.assertFalse(review_phase.verify_release_readiness(root, first, 's2', phase=31))
            run(5)  # a later rejected attempt supersedes the earlier acceptance
            self.assertFalse(review_phase.verify_release_readiness(root, first, 's1', phase=31))
            second = codex_review.destination(root, 31, 2, 'release-readiness')
            self.assertFalse(review_phase.verify_release_readiness(root, second, 's1', phase=31))

    def test_github_mirror_tag_is_required_for_0_2_5(self):
        with self.assertRaises(ValueError):
            release_delivery.verify_github_tag(ROOT, {}, {'tag': 'v0.2.5'}, 31, '0.2.5')
        with self.assertRaises(ValueError):
            release_delivery.verify_github_tag(ROOT, {'github_tag_receipt': 'docs/checks/release-tag-github-v0.2.5.json'},
                                               {}, 31, '0.2.4')

    def test_registry_readiness_accepts_only_the_phase_31_codex_chunk_path(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            registry = {'releaseReadiness': {'review': 'docs/reviews/phase-31-attempt-01.json',
                                             'reviewSha256': 'x', 'sourceSha256': 's1'}}
            with self.assertRaisesRegex(ValueError, 'actual release phase review'):
                release_delivery.verify_registry_readiness(root, registry, 's1', 31)
            path = root / 'docs/reviews/phase-31-chunk-release-readiness-codex-attempt-01.json'
            path.parent.mkdir(parents=True)
            path.write_text('{}')
            registry['releaseReadiness']['review'] = path.relative_to(root).as_posix()
            registry['releaseReadiness']['reviewSha256'] = __import__('hashlib').sha256(b'{}').hexdigest()
            with mock.patch.object(release_delivery, 'verify_release_readiness', return_value=True) as accepted:
                release_delivery.verify_registry_readiness(root, registry, 's1', 31)
            accepted.assert_called_once()
            self.assertEqual(accepted.call_args.kwargs.get('phase'), 31)

    def test_phase_31_completion_requires_verified_delivery(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            row = {'id': 31, 'chunks': []}
            with mock.patch.object(codex_review, 'verify_chunk_acceptance'), \
                    mock.patch('release_delivery.verify', side_effect=ValueError('Phase delivery must identify canonical registry and tag receipts')):
                with self.assertRaisesRegex(ValueError, 'registry and tag receipts'):
                    codex_review._apply_status(root, {'phases': [row]}, row, None, root / 'x.json', review(10), 's1', 'accepted')
            self.assertNotIn('status', row)
            with mock.patch('release_delivery.verify', return_value={}) as delivered:
                codex_review.verify_release_delivery(root, 31, 's1')
                delivered.assert_called_once_with(root, 's1', phase=31, version='0.2.5')
                codex_review.verify_release_delivery(root, 30, 's1')
                delivered.assert_called_once()

    def test_governance_successor_permits_only_reviewed_governance_paths(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            run = lambda *args: subprocess.check_output(['git', *args], cwd=root, text=True).strip()
            run('init', '-q', '-b', 'main')
            run('config', 'user.email', 't@example.invalid')
            run('config', 'user.name', 't')
            for name in ('crates/scm-sdk/src/lib.rs', 'tools/codex_review.py', 'scripts/verify-publication.py'):
                (root / name).parent.mkdir(parents=True, exist_ok=True)
                (root / name).write_text('v1\n')
            run('add', '.')
            run('commit', '-q', '-m', 'release')
            release = run('rev-parse', 'HEAD')
            published = release_delivery.commit_fingerprint(root, release)
            self.assertEqual(release_delivery.verify_governance_successor(root, 31, release, published), [])
            (root / 'tools/codex_review.py').write_text('v2\n')
            (root / 'tools/site_binding.py').write_text('new\n')
            (root / 'docs/reviews').mkdir(parents=True)
            (root / 'docs/reviews/evidence.json').write_text('{}')  # review bookkeeping is not a source input
            self.assertEqual(release_delivery.verify_governance_successor(root, 31, release, published),
                             ['tools/codex_review.py', 'tools/site_binding.py'])
            for name in ('crates/scm-sdk/src/lib.rs', 'scripts/verify-publication.py', 'Cargo.lock'):
                path = root / name
                original = path.read_text() if path.exists() else None
                path.write_text('changed\n')
                with self.assertRaisesRegex(ValueError, 'outside reviewed governance paths'):
                    release_delivery.verify_governance_successor(root, 31, release, published)
                path.write_text(original) if original is not None else path.unlink()
            with self.assertRaisesRegex(ValueError, 'differs from the published source'):
                release_delivery.verify_governance_successor(root, 31, release, '0' * 64)
            with self.assertRaisesRegex(ValueError, 'does not permit'):
                release_delivery.verify_governance_successor(root, 28, release, published)
            run('checkout', '-q', '--orphan', 'unrelated')
            run('commit', '-q', '--allow-empty', '-m', 'unrelated')
            with self.assertRaisesRegex(ValueError, 'does not descend'):
                release_delivery.verify_governance_successor(root, 31, release, published)

    def test_ci_may_run_on_a_committed_governance_successor_only(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            run = lambda *args: subprocess.check_output(['git', *args], cwd=root, text=True).strip()
            run('init', '-q', '-b', 'main')
            run('config', 'user.email', 't@example.invalid')
            run('config', 'user.name', 't')
            for name in ('crates/scm-sdk/src/lib.rs', 'tools/codex_review.py', '.forgejo/workflows/ci.yml'):
                (root / name).parent.mkdir(parents=True, exist_ok=True)
                (root / name).write_text('v1\n')
            run('add', '.')
            run('commit', '-q', '-m', 'release')
            release = run('rev-parse', 'HEAD')
            published = release_delivery.commit_fingerprint(root, release)
            (root / 'tools/codex_review.py').write_text('v2\n')
            run('commit', '-q', '-am', 'governance fix')
            successor = run('rev-parse', 'HEAD')
            self.assertEqual(release_delivery.verify_governance_successor_commit(root, 31, release, published, successor),
                             ['tools/codex_review.py'])
            result = {'sourceCommit': release, 'ciCommit': successor}
            ci_sha = release_delivery.commit_fingerprint(root, successor)
            release_delivery.verify_source_commits(root, result, published, ci_sha)
            with self.assertRaisesRegex(ValueError, 'differs from reviewed inputs'):
                release_delivery.verify_source_commits(root, result, published)  # CI fingerprint is not the published one
            for name in ('crates/scm-sdk/src/lib.rs', '.forgejo/workflows/ci.yml'):
                run('checkout', '-q', successor)
                (root / name).write_text('changed\n')
                run('commit', '-q', '-am', 'outside')
                with self.assertRaisesRegex(ValueError, 'outside reviewed governance paths'):
                    release_delivery.verify_governance_successor_commit(root, 31, release, published,
                                                                        run('rev-parse', 'HEAD'))
            with self.assertRaisesRegex(ValueError, 'does not permit'):
                release_delivery.verify_governance_successor_commit(root, 28, release, published, successor)
            run('checkout', '-q', '--orphan', 'unrelated')
            run('commit', '-q', '-m', 'unrelated')
            with self.assertRaisesRegex(ValueError, 'does not descend'):
                release_delivery.verify_governance_successor_commit(root, 31, release, published, run('rev-parse', 'HEAD'))

    def test_successor_delivery_verifies_package_proofs_against_the_published_source(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / 'scripts').mkdir()
            (root / 'scripts/check-management-package.py').write_text(
                'import json, pathlib\n'
                'def verify_receipt(root, path, toolchain, source_sha=None, *, require_directional=False):\n'
                '    log = pathlib.Path(root) / "calls.jsonl"\n'
                '    log.write_text((log.read_text() if log.exists() else "") + json.dumps(\n'
                '        [pathlib.Path(path).name, toolchain, source_sha, require_directional]) + "\\n")\n')
            release_delivery.verify_management_receipts(root, 31, 'p' * 64)
            calls = [json.loads(line) for line in (root / 'calls.jsonl').read_text().splitlines()]
        self.assertEqual(calls, [['phase-31-release-package-1.99.0.json', '1.99.0', 'p' * 64, True],
                                 ['phase-31-release-package-1.89.0.json', '1.89.0', 'p' * 64, True]])

    def test_delivery_binds_receipts_to_the_published_source_for_a_governance_successor(self):
        with mock.patch.object(release_delivery, 'fingerprint', return_value='c' * 64), \
                mock.patch.object(release_delivery, 'published_source', return_value=('a' * 40, 'p' * 64)), \
                mock.patch.object(release_delivery, 'verify_governance_successor') as successor, \
                mock.patch.object(release_delivery, 'read_record', side_effect=ValueError('stop')):
            with self.assertRaisesRegex(ValueError, 'stop'):
                release_delivery.verify(ROOT, phase=31, version='0.2.5')
            successor.assert_called_once_with(ROOT.resolve(), 31, 'a' * 40, 'p' * 64)
            with self.assertRaisesRegex(ValueError, 'differs from reviewed source'):
                release_delivery.verify(ROOT, 'd' * 64, phase=31, version='0.2.5')
        with mock.patch.object(release_delivery, 'fingerprint', return_value='c' * 64), \
                mock.patch.object(release_delivery, 'published_source', return_value=None):
            with self.assertRaisesRegex(ValueError, 'registry receipt is required'):
                release_delivery.verify(ROOT, phase=31, version='0.2.5')

    def site_status(self, root):
        policy = {'reviewer': 'codex', 'minimum_score': 9}
        chunks = [{'id': name, 'status': 'pending', 'scope_evidence': ['a']} for name in codex_review.REQUIRED_CHUNKS[31]]
        status = {'active_phase': 31, 'phases': [{'id': 31, 'name': 'release', 'status': 'in_progress',
                                                   'review_policy': policy, 'scope_evidence': ['a'], 'chunks': chunks}]}
        (root / 'docs/checks').mkdir(parents=True, exist_ok=True)
        (root / 'docs/phase-status.json').write_text(json.dumps(status))
        report = {'phase': 31, 'passed': True, 'source_sha256': 's1', 'source_unchanged': True,
                  'checks': [{'command': ['python', 'x'], 'exit_code': 0}]}
        (root / 'docs/checks/phase-31.json').write_text(json.dumps(report))
        # Two preserved unbound attempts precede enforcement, exactly as in the repository.
        for attempt in (1, 2):
            path = codex_review.destination(root, 31, attempt, 'docs-site')
            path.parent.mkdir(parents=True, exist_ok=True)
            path.with_suffix('.prompt.md').write_text('p')
            path.with_suffix('.failed.json').write_text(json.dumps(
                {'phase': 31, 'chunk': 'docs-site', 'attempt': attempt, 'reviewer': codex_review.REVIEWER,
                 'status': 'invocation_failed', 'reason': 'interrupted', 'checks_sha256': None,
                 'prompt_sha256': codex_review.sha256(path.with_suffix('.prompt.md'))}))

    def run_site_review(self, root, bindings, score=9):
        def runner(prompt, out, events, schema, _root):
            events.write_text('{}\n')
            out.write_text(json.dumps(review(score)))
            return 0
        states = iter(bindings)
        with mock.patch.object(codex_review, 'fingerprint', return_value='s1'), \
                mock.patch.object(codex_review, 'check_evidence', return_value=root / 'docs/checks/phase-31.json'), \
                mock.patch.object(codex_review, 'verify_site', side_effect=lambda *_: next(states)):
            return codex_review.main(['--phase', '31', '--chunk', 'docs-site'], root=root, runner=runner)

    def test_site_bound_review_records_its_binding_and_rejects_a_site_change_during_review(self):
        bound = ({'version': '0.2.5', 'siteCommit': 'c1', 'docsSiteTree': 't1'}, 'h1')
        moved = ({'version': '0.2.5', 'siteCommit': 'c2', 'docsSiteTree': 't2'}, 'h2')
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            self.site_status(root)
            self.assertEqual(self.run_site_review(root, [bound, moved]), 2)
            third = json.loads(codex_review.destination(root, 31, 3, 'docs-site').read_text())
            self.assertEqual((third['decision'], third['site_unchanged']), ('rejected', False))
            self.assertIn(codex_review.site_prompt(third['site_binding']),
                          codex_review.destination(root, 31, 3, 'docs-site').with_suffix('.prompt.md').read_text())
            self.assertEqual(self.run_site_review(root, [bound, bound]), 0)
            fourth = codex_review.destination(root, 31, 4, 'docs-site')
            record = json.loads(fourth.read_text())
            self.assertEqual((record['decision'], record['site_unchanged']), ('accepted', True))
            self.assertEqual(record['site_binding'], codex_review.site_summary(*bound))
            codex_review.verify_attempt(root, 31, 4, 'docs-site')
            # A preserved record cannot drop or alter its binding.
            altered = dict(record, site_binding=codex_review.site_summary(*moved))
            fourth.write_text(json.dumps(altered))
            with self.assertRaisesRegex(ValueError, 'does not bind'):
                codex_review.verify_attempt(root, 31, 4, 'docs-site')

    def test_site_bound_acceptance_and_phase_advancement_reverify_the_current_site(self):
        bound = ({'version': '0.2.5', 'siteCommit': 'c1', 'docsSiteTree': 't1'}, 'h1')
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            self.site_status(root)
            self.assertEqual(self.run_site_review(root, [bound, bound]), 0)
            status = json.loads((root / 'docs/phase-status.json').read_text())
            row = status['phases'][0]
            chunk = next(c for c in row['chunks'] if c['id'] == 'docs-site')
            self.assertEqual(chunk['status'], 'accepted')
            single = {**row, 'chunks': [chunk]}
            snapshot = codex_review.destination(root, 31, 3, 'docs-site').with_suffix('.checks.json')
            with mock.patch.object(codex_review, 'commands_for', return_value=[c['command'] for c in json.loads(snapshot.read_text())['checks']]), \
                    mock.patch.object(codex_review, 'published_identity', return_value=None):
                with mock.patch.object(codex_review, 'verify_site', return_value=bound), \
                        mock.patch.dict(codex_review.REQUIRED_CHUNKS, {31: ('docs-site',)}):
                    codex_review.verify_chunk_acceptance(root, single, 's1')
                for changed in (({**bound[0], 'siteCommit': 'c9'}, 'h1'), (bound[0], 'h9')):
                    with mock.patch.object(codex_review, 'verify_site', return_value=changed), \
                            mock.patch.dict(codex_review.REQUIRED_CHUNKS, {31: ('docs-site',)}):
                        with self.assertRaisesRegex(ValueError, 'differs from the current site binding'):
                            codex_review.verify_chunk_acceptance(root, single, 's1')
                with mock.patch.object(codex_review, 'verify_site', side_effect=ValueError('HEAD differs')), \
                        mock.patch.dict(codex_review.REQUIRED_CHUNKS, {31: ('docs-site',)}):
                    with self.assertRaisesRegex(ValueError, 'HEAD differs'):
                        codex_review.verify_chunk_acceptance(root, single, 's1')
                    # Completing the phase re-runs chunk acceptance, so a moved site blocks advancement.
                    with self.assertRaisesRegex(ValueError, 'HEAD differs'):
                        codex_review._apply_status(root, {'phases': [single]}, single, None, root / 'docs/reviews/x.json',
                                                   review(10), 's1', 'accepted')
                    self.assertEqual(single['status'], 'in_progress')

    def test_publication_chunk_stays_bound_to_the_published_source(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / 'docs/checks').mkdir(parents=True)
            policy = {'reviewer': 'codex', 'minimum_score': 9}
            gate = [['python', 'scripts/check-management-package.py', '--toolchain', '1.99.0'],
                    ['python', 'scripts/check-access-release.py', '--phase', '31']]
            report = {'phase': 31, 'passed': True, 'source_sha256': 'p1', 'source_unchanged': True,
                      'checks': [{'command': c, 'exit_code': 0} for c in gate]}
            (root / 'docs/checks/phase-31.json').write_text(json.dumps(report))
            chunk = {'id': 'release-readiness', 'status': 'pending', 'scope_evidence': ['a']}
            status = {'active_phase': 31, 'phases': [{'id': 31, 'name': 'release', 'status': 'in_progress',
                                                       'review_policy': policy, 'scope_evidence': ['a'], 'chunks': [chunk]}]}
            (root / 'docs/phase-status.json').write_text(json.dumps(status))

            def runner(prompt, out, events, schema, _root):
                events.write_text('{}\n')
                out.write_text(json.dumps(review(9)))
                return 0
            with mock.patch.object(codex_review, 'fingerprint', return_value='p1'), \
                    mock.patch.object(codex_review, 'check_evidence', return_value=root / 'docs/checks/phase-31.json'):
                codex_review.main(['--phase', '31', '--chunk', 'release-readiness'], root=root, runner=runner)
            row = json.loads((root / 'docs/phase-status.json').read_text())['phases'][0]
            with mock.patch.dict(codex_review.REQUIRED_CHUNKS, {31: ('release-readiness',)}):
                with mock.patch.object(codex_review, 'published_identity', return_value=('r' * 40, 'p1')):
                    codex_review.verify_chunk_acceptance(root, row, 'successor')  # current differs; published binds
                with mock.patch.object(codex_review, 'published_identity', return_value=('r' * 40, 'other')):
                    with self.assertRaisesRegex(ValueError, 'stale, superseded or inconsistent'):
                        codex_review.verify_chunk_acceptance(root, row, 'successor')
                with mock.patch.object(codex_review, 'published_identity', return_value=None), \
                        mock.patch.object(codex_review, 'commands_for', return_value=gate):
                    with self.assertRaisesRegex(ValueError, 'stale, superseded or inconsistent'):
                        codex_review.verify_chunk_acceptance(root, row, 'successor')
                snapshot = codex_review.destination(root, 31, 1, 'release-readiness').with_suffix('.checks.json')
                # Without its pre-publication gate the record cannot stand in for publication readiness.
                data = json.loads(snapshot.read_text())
                data['checks'] = data['checks'][:1]
                snapshot.write_text(json.dumps(data))
                record_path = codex_review.destination(root, 31, 1, 'release-readiness')
                record = json.loads(record_path.read_text())
                record['checks_sha256'] = codex_review.sha256(snapshot)
                record_path.write_text(json.dumps(record))
                with mock.patch.object(codex_review, 'published_identity', return_value=('r' * 40, 'p1')):
                    with self.assertRaisesRegex(ValueError, 'pre-publication gate'):
                        codex_review.verify_chunk_acceptance(root, row, 'successor')

    def test_example_unix_permission_code_is_platform_gated(self):
        text = (ROOT / 'crates/scm-sdk/examples/mcp_tags_live.rs').read_text()
        self.assertIn('#[cfg(not(unix))]\n    fn open_private(', text)
        for index in [i for i in range(len(text)) if text.startswith('std::os::unix', i)]:
            preceding = text[:index]
            function = preceding.rfind('fn open_private(')
            self.assertGreater(function, preceding.rfind('\n    }\n'), 'unix API used outside open_private')
            self.assertTrue(preceding[:function].rstrip().endswith('#[cfg(unix)]\n    fn') or
                            preceding[:function].rstrip().endswith('#[cfg(unix)]'), 'open_private must be cfg(unix)')

    def test_release_documentation_names_the_phase_31_procedure(self):
        release = (ROOT / 'docs/RELEASE.md').read_text()
        for text in ('The workspace version is **0.2.5**', 'scripts/check-prepublication.py --phase 31 --version 0.2.5',
                     'scripts/verify-publication.py --version 0.2.5', 'tools/release_delivery.py --phase 31 --version 0.2.5',
                     'release-tag-github-v0.2.5.json'):
            self.assertIn(text, release)
        self.assertIn('version = "0.2.5"', (ROOT / 'crates/scm-sdk/Cargo.toml').read_text())


if __name__ == '__main__':
    unittest.main()
