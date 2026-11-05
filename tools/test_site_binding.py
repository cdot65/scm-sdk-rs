"""Docs-site binding: site-only changes, replaced receipts and unmerged releases fail closed."""
import hashlib
import json
import pathlib
import subprocess
import tempfile
import unittest

import site_binding

BINDING = 'docs/reviews/github-pages/release-0.2.5/binding.json'


def git(root, *args):
    return subprocess.check_output(['git', *args], cwd=root, text=True).strip()


class SiteBindingTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        root = self.root = pathlib.Path(self.temporary.name)
        git(root, 'init', '-q', '-b', 'main')
        git(root, 'config', 'user.email', 'test@example.invalid')
        git(root, 'config', 'user.name', 'test')
        (root / 'sdk.txt').write_text('release\n')
        git(root, 'add', '.')
        git(root, 'commit', '-q', '-m', 'release')
        self.release = git(root, 'rev-parse', 'HEAD')
        git(root, 'checkout', '-q', '-b', 'docs/scm-github-pages')
        (root / 'docs-site').mkdir()
        (root / 'docs-site/page.md').write_text('0.2.5\n')
        (root / '.github/workflows').mkdir(parents=True)
        (root / '.github/workflows/docs-deploy.yml').write_text('name: docs\n')
        git(root, 'add', '.')
        git(root, 'commit', '-q', '-m', 'site')
        self.site = git(root, 'rev-parse', 'HEAD')
        git(root, 'checkout', '-q', 'main')
        git(root, 'worktree', 'add', '-q', site_binding.WORKTREE, 'docs/scm-github-pages')
        (root / 'docs/checks').mkdir(parents=True)
        (root / 'docs/checks/release-registry-0.2.5.json').write_text(json.dumps(
            {'published': True, 'sourceCommit': self.release, 'releaseReadiness': {'sourceSha256': 'a' * 64}}))
        self.write_receipts(self.site)
        site_binding.write(root, '0.2.5')

    def tearDown(self):
        self.temporary.cleanup()

    def write_receipts(self, commit):
        receipts = self.root / pathlib.Path(BINDING).parent / commit
        receipts.mkdir(parents=True, exist_ok=True)
        asset = hashlib.sha256(b'0.2.5\n').hexdigest()
        (receipts / 'deployment.json').write_text(json.dumps({
            'version': '0.2.5', 'deploymentCommit': commit, 'artifactZipSha256Verified': True,
            'workflow': {'headSha': commit, 'conclusion': 'success', 'headBranch': 'docs/scm-github-pages', 'url': 'u'},
            'artifact': {'digest': 'sha256:' + 'b' * 64}, 'pages': {'': {'matchesArtifact': True}},
            'protectedImages': {}, 'protectedSourceAssets': {'docs-site/page.md': {'sha256': asset}}}))
        (receipts / 'push.json').write_text(json.dumps(
            {'after': commit, 'ref': 'refs/heads/docs/scm-github-pages', 'exitCode': 0}))

    def site_commit(self, text='changed\n'):
        worktree = self.root / site_binding.WORKTREE
        (worktree / 'docs-site/page.md').write_text(text)
        git(worktree, 'commit', '-q', '-am', 'site-only change')
        return git(worktree, 'rev-parse', 'HEAD')

    def test_recorded_binding_verifies_and_names_the_published_release(self):
        binding, digest = site_binding.check(self.root, '0.2.5')
        self.assertEqual((binding['siteCommit'], binding['releaseSource'], binding['sdkSourceSha256']),
                         (self.site, self.release, 'a' * 64))
        self.assertEqual(digest, hashlib.sha256((self.root / BINDING).read_bytes()).hexdigest())

    def test_site_only_commit_invalidates_the_binding(self):
        self.site_commit()
        with self.assertRaisesRegex(ValueError, 'HEAD differs'):
            site_binding.check(self.root, '0.2.5')

    def test_uncommitted_site_change_invalidates_the_binding(self):
        (self.root / site_binding.WORKTREE / 'docs-site/page.md').write_text('edited\n')
        with self.assertRaisesRegex(ValueError, 'uncommitted'):
            site_binding.check(self.root, '0.2.5')

    def test_replaced_or_mismatched_receipts_fail(self):
        deployment = self.root / pathlib.Path(BINDING).parent / self.site / 'deployment.json'
        original = deployment.read_text()
        deployment.write_text(original + ' ')
        with self.assertRaisesRegex(ValueError, 'differs from the recomputed'):
            site_binding.check(self.root, '0.2.5')
        record = json.loads(original)
        record['workflow']['conclusion'] = 'failure'
        deployment.write_text(json.dumps(record))
        with self.assertRaisesRegex(ValueError, 'do not prove a successful deployment'):
            site_binding.check(self.root, '0.2.5')

    def test_rebinding_to_another_commit_needs_that_commits_own_receipts(self):
        commit = self.site_commit()
        binding = json.loads((self.root / BINDING).read_text())
        binding['siteCommit'] = commit
        (self.root / BINDING).write_text(json.dumps(binding))
        with self.assertRaises((ValueError, FileNotFoundError)):
            site_binding.check(self.root, '0.2.5')

    def test_protected_asset_and_release_identity_are_recomputed(self):
        deployment = self.root / pathlib.Path(BINDING).parent / self.site / 'deployment.json'
        record = json.loads(deployment.read_text())
        record['protectedSourceAssets']['docs-site/page.md']['sha256'] = '0' * 64
        deployment.write_text(json.dumps(record))
        with self.assertRaisesRegex(ValueError, 'Protected site asset'):
            site_binding.check(self.root, '0.2.5')
        self.write_receipts(self.site)
        registry = self.root / 'docs/checks/release-registry-0.2.5.json'
        registry.write_text(json.dumps({'published': True, 'sourceCommit': self.site,
                                        'releaseReadiness': {'sourceSha256': 'a' * 64}}))
        with self.assertRaisesRegex(ValueError, 'differs from the recomputed'):
            site_binding.check(self.root, '0.2.5')

    def test_unmerged_release_source_fails(self):
        git(self.root, 'commit', '-q', '--allow-empty', '-m', 'later release')
        registry = self.root / 'docs/checks/release-registry-0.2.5.json'
        registry.write_text(json.dumps({'published': True, 'sourceCommit': git(self.root, 'rev-parse', 'HEAD'),
                                        'releaseReadiness': {'sourceSha256': 'a' * 64}}))
        with self.assertRaisesRegex(ValueError, 'does not contain the published release'):
            site_binding.check(self.root, '0.2.5')

    def test_existing_binding_is_never_overwritten(self):
        with self.assertRaises(FileExistsError):
            site_binding.write(self.root, '0.2.5')


if __name__ == '__main__':
    unittest.main()
