"""Historical delivery: fingerprint-identical CI, published-archive equality and recorded exceptions."""
import hashlib
import io
import json
import pathlib
import subprocess
import tarfile
import tempfile
import unittest
from unittest import mock

import historical_delivery as hd

VERSION = '0.2.4'


def git(root, *args):
    return subprocess.check_output(['git', *args], cwd=root, text=True).strip()


def crate(files):
    buffer = io.BytesIO()
    with tarfile.open(fileobj=buffer, mode='w:gz') as archive:
        for name, content in files.items():
            info = tarfile.TarInfo(f'scm-sdk-{VERSION}/{name}')
            info.size = len(content)
            archive.addfile(info, io.BytesIO(content))
    return buffer.getvalue()


def rows(files):
    return sorted(({'path': name, 'sha256': hashlib.sha256(data).hexdigest(), 'bytes': len(data)}
                   for name, data in files.items()), key=lambda row: row['path'])


class HistoricalDeliveryTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        root = self.root = pathlib.Path(self.temporary.name)
        git(root, 'init', '-q', '-b', 'main')
        git(root, 'config', 'user.email', 't@example.invalid')
        git(root, 'config', 'user.name', 't')
        (root / 'src.rs').write_text('fn main() {}\n')
        (root / '.forgejo/workflows').mkdir(parents=True)
        (root / '.forgejo/workflows/ci.yml').write_text(
            'name: Test\n  toolchain: ["1.89.0"]\n  features: ["all"]\n')
        git(root, 'add', '.')
        git(root, 'commit', '-q', '-m', 'release')
        self.release = git(root, 'rev-parse', 'HEAD')
        git(root, 'checkout', '-q', '-b', 'delivery/v0.2.4')
        (root / 'docs/checks').mkdir(parents=True)
        (root / 'docs/checks/receipt.json').write_text('{}')  # excluded evidence: fingerprint unchanged
        git(root, 'add', '.')
        git(root, 'commit', '-q', '-m', 'evidence')
        self.ci_commit = git(root, 'rev-parse', 'HEAD')
        self.published = hd.commit_fingerprint(root, self.release)
        self.files = {'.cargo_vcs_info.json': json.dumps({'git': {'sha1': self.release}}).encode(),
                      'src/lib.rs': b'pub fn sdk() {}\n'}
        self.write_proofs(self.files)
        git(root, 'add', '.')
        git(root, 'commit', '-q', '-m', 'proofs')
        self.proof_commit = git(root, 'rev-parse', 'HEAD')
        git(root, 'checkout', '-q', 'main')
        self.spec = {**hd.RELEASES[VERSION], 'release': self.release,
                     'proofs': 'docs/checks/proof-{toolchain}.json', 'registry': 'docs/checks/registry.json'}
        self.write_evidence()

    def tearDown(self):
        self.temporary.cleanup()

    def write_proofs(self, files):
        packaged = {**files, '.cargo_vcs_info.json': json.dumps({'git': {'sha1': self.ci_commit}}).encode()}
        for toolchain in hd.TOOLCHAINS:
            proof = {'passed': True, 'toolchain': toolchain, 'requireClean': True, 'effectiveDirty': False,
                     'sourceSha256': self.published, 'releaseCommit': self.ci_commit,
                     'files': [{'path': f'scm-sdk-{VERSION}/' + row['path'], 'sha256': row['sha256']}
                               for row in rows(packaged)]}
            (self.root / f'docs/checks/proof-{toolchain}.json').write_text(json.dumps(proof))

    def write_evidence(self, failed=(), exceptions=None, published_files=None):
        root, evidence = self.root, self.root / hd.evidence_dir(VERSION)
        evidence.mkdir(parents=True, exist_ok=True)
        data = crate(published_files or self.files)
        (root / '.local/artifacts').mkdir(parents=True, exist_ok=True)
        (root / '.local/artifacts/published.crate').write_bytes(data)
        registry = {'registry': 'crates.io', 'package': 'scm-sdk', 'version': VERSION, 'yanked': False,
                    'ownerLogins': [hd.OWNER], 'sourceCommit': self.release,
                    'retainedArchive': '.local/artifacts/published.crate',
                    'archiveSha256': hashlib.sha256(data).hexdigest(), 'archiveBytes': len(data),
                    'files': rows(published_files or self.files)}
        (root / 'docs/checks').mkdir(parents=True, exist_ok=True)
        (root / 'docs/checks/registry.json').write_text(json.dumps(registry))
        for name, remote in (('tag.json', hd.REMOTE), ('tag-github.json', hd.GITHUB_REMOTE)):
            (evidence / name).write_text(json.dumps({
                'passed': True, 'remote': remote, 'tag': 'v' + VERSION, 'sourceCommit': self.release,
                'tagObject': 'a' * 40, 'remoteRefs': {f'refs/tags/v{VERSION}': 'a' * 40,
                                                      f'refs/tags/v{VERSION}^{{}}': self.release}}))
        (evidence / 'branch.json').write_text(json.dumps({'remote': hd.REMOTE, 'ref': 'refs/heads/delivery/v0.2.4',
                                                          'commit': self.proof_commit}))
        contexts = sorted(hd.expected_contexts(git(root, 'show', f'{self.ci_commit}:.forgejo/workflows/ci.yml')))
        jobs = [{'id': 10 + index, 'context': name, 'status': 'failure' if name in failed else 'success',
                 'target_url': f'/cdot/scm-sdk-rs/actions/runs/7/jobs/{index}'} for index, name in enumerate(contexts)]
        (evidence / 'ci-run-7.json').write_text(json.dumps({
            'version': VERSION, 'sourceCommit': self.ci_commit, 'sourceSha256': self.published,
            'run': {'index_in_repo': 7, 'html_url': hd.RUN_URL + '7', 'commit_sha': self.ci_commit,
                    'status': 'failure' if failed else 'success'},
            'jobs': jobs, 'expectedJobCount': len(jobs)}))
        (root / 'cause.log').write_text('advisory')
        (evidence / 'manifest.json').write_text(json.dumps({
            'version': VERSION, 'phase': 28, 'releaseCommit': self.release, 'proofCommit': self.proof_commit,
            'ciRun': 7, 'ciExceptions': exceptions or []}))
        return contexts

    def verify(self):
        with mock.patch.dict(hd.RELEASES, {VERSION: self.spec}):
            return hd.verify(self.root, VERSION)

    def test_fingerprint_identical_branch_delivery_verifies(self):
        result = self.verify()
        self.assertEqual((result['status'], result['ciCommit'], result['failedContexts']),
                         ('delivered', self.ci_commit, []))

    def test_published_archive_must_equal_the_clean_package_except_vcs_identity(self):
        self.write_evidence(published_files={**self.files, 'src/lib.rs': b'tampered\n'})
        with self.assertRaisesRegex(ValueError, 'differ from the .* clean package'):
            self.verify()

    def test_published_archive_must_name_the_release_commit(self):
        self.write_evidence(published_files={**self.files, '.cargo_vcs_info.json': b'{"git": {"sha1": "%s"}}'
                                             % self.ci_commit.encode()})
        with self.assertRaisesRegex(ValueError, 'VCS identity'):
            self.verify()

    def test_ci_on_a_source_change_is_refused(self):
        git(self.root, 'checkout', '-q', 'delivery/v0.2.4')
        (self.root / 'src.rs').write_text('changed\n')
        git(self.root, 'commit', '-q', '-am', 'source change')
        self.ci_commit = git(self.root, 'rev-parse', 'HEAD')
        git(self.root, 'checkout', '-q', 'main')
        self.write_evidence()
        with self.assertRaisesRegex(ValueError, 'fingerprint differs'):
            self.verify()

    def test_failed_context_needs_an_exact_recorded_exception(self):
        contexts = self.write_evidence(failed=['Test / contracts-and-package (push)'])
        self.assertIn('Test / contracts-and-package (push)', contexts)
        with self.assertRaisesRegex(ValueError, 'recorded exception'):
            self.verify()
        exception = {'context': 'Test / contracts-and-package (push)', 'cause': 'post-release advisory',
                     'evidence': 'cause.log'}
        self.write_evidence(failed=['Test / contracts-and-package (push)'], exceptions=[exception])
        result = self.verify()
        self.assertEqual(result['status'], 'delivered_with_exception')
        self.write_evidence(exceptions=[exception])  # an exception for a passing context is refused
        with self.assertRaisesRegex(ValueError, 'recorded exception'):
            self.verify()

    def test_tag_must_peel_to_the_release_and_mirror_must_match(self):
        evidence = self.root / hd.evidence_dir(VERSION)
        tag = json.loads((evidence / 'tag-github.json').read_text())
        tag['tagObject'] = 'b' * 40
        tag['remoteRefs'][f'refs/tags/v{VERSION}'] = 'b' * 40
        (evidence / 'tag-github.json').write_text(json.dumps(tag))
        with self.assertRaisesRegex(ValueError, 'GitHub tag object differs'):
            self.verify()
        tag['remoteRefs'][f'refs/tags/v{VERSION}^{{}}'] = self.ci_commit
        (evidence / 'tag-github.json').write_text(json.dumps(tag))
        with self.assertRaisesRegex(ValueError, 'does not peel'):
            self.verify()

    def test_ci_commit_must_be_contained_in_the_observed_delivery_branch(self):
        evidence = self.root / hd.evidence_dir(VERSION)
        branch = json.loads((evidence / 'branch.json').read_text())
        branch['commit'] = self.release
        (evidence / 'branch.json').write_text(json.dumps(branch))
        with self.assertRaisesRegex(ValueError, 'ancestry'):
            self.verify()

    def test_proof_commit_must_be_retained_on_main_or_the_delivery_branch(self):
        git(self.root, 'checkout', '-q', '--orphan', 'stray')
        git(self.root, 'commit', '-q', '-m', 'stray', '--allow-empty')
        stray = git(self.root, 'rev-parse', 'HEAD')
        git(self.root, 'checkout', '-q', '-f', 'main')
        manifest_path = self.root / hd.evidence_dir(VERSION) / 'manifest.json'
        manifest = json.loads(manifest_path.read_text())
        manifest['proofCommit'] = stray
        manifest_path.write_text(json.dumps(manifest))
        with self.assertRaisesRegex(ValueError, 'not retained'):
            self.verify()

    def test_proof_must_qualify_a_commit_with_the_published_fingerprint(self):
        git(self.root, 'checkout', '-q', 'delivery/v0.2.4')
        (self.root / 'src.rs').write_text('other source\n')
        git(self.root, 'commit', '-q', '-am', 'different source')
        other = git(self.root, 'rev-parse', 'HEAD')
        for toolchain in hd.TOOLCHAINS:
            path = self.root / f'docs/checks/proof-{toolchain}.json'
            proof = json.loads(path.read_text())
            proof['releaseCommit'] = other
            path.write_text(json.dumps(proof))
        git(self.root, 'commit', '-q', '-am', 'proofs naming another source')
        self.proof_commit = git(self.root, 'rev-parse', 'HEAD')
        git(self.root, 'checkout', '-q', 'main')
        self.write_evidence()
        with self.assertRaisesRegex(ValueError, 'does not qualify the published source'):
            self.verify()

    def test_registry_identity_must_be_unyanked_owned_and_the_release(self):
        registry_path = self.root / 'docs/checks/registry.json'
        original = json.loads(registry_path.read_text())
        for change in ({'yanked': True}, {'ownerLogins': ['someone-else']}, {'version': '9.9.9'},
                       {'sourceCommit': self.ci_commit}):
            registry_path.write_text(json.dumps({**original, **change}))
            with self.assertRaisesRegex(ValueError, 'Registry identity'):
                self.verify()

    def test_manifest_identity_must_match_the_release(self):
        manifest_path = self.root / hd.evidence_dir(VERSION) / 'manifest.json'
        original = json.loads(manifest_path.read_text())
        for change in ({'phase': 24}, {'version': '0.2.3'}, {'releaseCommit': self.ci_commit}):
            manifest_path.write_text(json.dumps({**original, **change}))
            with self.assertRaisesRegex(ValueError, 'manifest identity'):
                self.verify()

    def test_cli_honours_both_root_forms_and_binds_that_trees_rules(self):
        rules = self.root / 'tools/release_delivery.py'
        rules.parent.mkdir()
        rules.write_text(pathlib.Path(hd.RULES).read_text())
        out = io.StringIO()
        bound = pathlib.Path(hd.RULES).resolve()
        self.addCleanup(hd.bind_rules, bound.parents[1])  # restore the rules bound before this test
        with mock.patch.dict(hd.RELEASES, {VERSION: self.spec}), mock.patch('sys.stdout', out):
            for argv in (['verify', '--version', VERSION, '--root', str(self.root)],
                         ['verify', '--version', VERSION, f'--root={self.root}']):
                self.assertEqual(hd.main(argv), 0)
                self.assertEqual(hd.RULES, rules.resolve())
        self.assertEqual([json.loads(line)['status'] for line in out.getvalue().splitlines()], ['delivered'] * 2)

    def test_execute_writes_a_bound_receipt_only_inside_the_evidence_directory(self):
        with mock.patch.dict(hd.RELEASES, {VERSION: self.spec}), \
                mock.patch.object(hd, 'run_suite', return_value=subprocess.CompletedProcess(
                    [], 0, '', 'Ran 3 tests in 0.1s\n\nOK\n')):
            path = hd.execute(self.root, VERSION, f'{hd.evidence_dir(VERSION)}/execution-test.json', 'test')
            receipt = json.loads(path.read_text())
            self.assertEqual((receipt['verify']['result']['status'], receipt['tests']['exitCode'], receipt['label']),
                             ('delivered', 0, 'test'))
            self.assertEqual(receipt['tool']['sha256'], hashlib.sha256(pathlib.Path(hd.__file__).read_bytes()).hexdigest())
            with self.assertRaises(FileExistsError):
                hd.execute(self.root, VERSION, f'{hd.evidence_dir(VERSION)}/execution-test.json', 'test')
            with self.assertRaisesRegex(ValueError, 'evidence directory'):
                hd.execute(self.root, VERSION, 'docs/checks/elsewhere.json', 'test')


if __name__ == '__main__':
    unittest.main()
