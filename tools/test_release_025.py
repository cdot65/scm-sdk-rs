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
from check_phase import commands_for, ci_commands_for

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
        commands = commands_for(31)
        readiness = ['python', 'scripts/check-access-release.py', '--phase', '31', '--version', '0.2.5',
                     '--output', 'docs/checks/phase-31-release-readiness.json']
        self.assertEqual(commands[-1], readiness)
        for compiler in ('1.89.0', '1.99.0'):
            package = ['python', 'scripts/check-management-package.py', '--require-directional', '--toolchain', compiler,
                       '--output', f'docs/checks/phase-31-release-package-{compiler}.json']
            self.assertLess(commands.index(package), commands.index(readiness))
        subset = ci_commands_for(31)
        self.assertFalse(any('scripts/check-management-package.py' in c or 'scripts/check-access-release.py' in c for c in subset))
        self.assertIn(['python', 'scripts/check-private-material.py', '--check', 'docs/checks/phase-31-private-material.json'], subset)
        live = json.loads((ROOT / 'contracts/vendor/portkey-mcp-tags/8e5da0e/operations.json').read_text())['liveReceipt']
        self.assertIn(['python', 'tools/mcp_tags_live.py', '--check', live], commands)

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
