"""Independent receipt tampering checks; no compiler/package subprocess executes."""
import copy
import hashlib
import importlib.util
import json
import pathlib
import tempfile
import types
import unittest
from unittest import mock

SPEC = importlib.util.spec_from_file_location('management_package_wrapper', pathlib.Path(__file__).resolve().parents[1] / 'scripts/check-management-package.py')
package = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(package)


def digest(data):
    return hashlib.sha256(data).hexdigest()


class ManagementPackageTests(unittest.TestCase):
    def setUp(self):
        temporary = tempfile.TemporaryDirectory()
        self.addCleanup(temporary.cleanup)
        self.root = pathlib.Path(temporary.name)
        self.source = 'a' * 64
        self.commit = 'b' * 40
        self.host = 'x86_64-unknown-linux-musl'
        self.version = '0.2.2'
        self.files = {}
        self.names = ('response_defaults', 'red_team_dataplane', 'response_compatibility', 'red_team_management')
        for name in self.names:
            data = ('// ' + name + '\nfn main() {}\n').encode()
            member = 'examples/' + name + '.rs'
            self.files[member] = data
            self.write('crates/scm-sdk/' + member, data)
        self.write('crates/scm-sdk/Cargo.toml', b'[package]\nversion="0.2.2"\n')
        for member in ('src/lib.rs', 'LICENSE', 'NOTICE', 'README.md'):
            self.files[member] = ('// original ' + member).encode()
            relative = 'crates/scm-sdk/' + member if member.startswith('src/') else member
            self.write(relative, self.files[member])
        self.files['Cargo.toml.orig'] = (self.root / 'crates/scm-sdk/Cargo.toml').read_bytes()
        self.write('scripts/check-package.py', b'# package\n')
        self.write('scripts/check-management-package.py', b'# wrapper\n')
        self.write(package.management_consumers.MANIFEST, b'{}\n')
        self.archive = self.write('archive.crate', b'synthetic archive bytes')
        self.receipt = self.root / 'receipt.json'
        self.manifest = {'consumers': [
            {'path': 'crates/scm-sdk/examples/' + name + '.rs', 'sha256': digest(self.files['examples/' + name + '.rs'])}
            for name in self.names[:-1]] + [{'path': 'contracts/native-probes/src/main.rs', 'sha256': 'c' * 64}]}
        self.vcs = {'git': {'sha1': self.commit}, 'path_in_vcs': 'crates/scm-sdk'}
        checks = []
        def check(command, cwd='package'):
            checks.append({'command': ['cargo', '+1.99.0', *command], 'exit_code': 0,
                           'cwd': str(self.root / cwd)})
            return len(checks) - 1
        for command in [['test', '--lib', '--no-default-features'], ['test', '--lib', '--all-features'],
                        ['check', '--examples', '--all-features'], ['test', '--doc', '--all-features']]:
            check(command + ['--locked', '--offline', '--target', self.host])
        bindings = {}
        for name in self.names:
            check(['run', '--example', name, '--all-features', '--locked', '--offline', '--target', self.host])
            index = check(['run', '--locked', '--offline', '--target', self.host], name + '/consumer')
            bindings['examples/' + name + '.rs'] = {'sha256': digest(self.files['examples/' + name + '.rs']), 'checkIndex': index}
        probe_index = check(['run', '--locked', '--offline', '--target', self.host], 'probe/consumer')
        self.report = {'schemaVersion': 3, 'sourceSha256': self.source, 'toolchain': '1.99.0',
            'passed': True, 'requireClean': True, 'effectiveDirty': False,
            'packageScriptSha256': package.sha(self.root / 'scripts/check-package.py'),
            'wrapperSha256': package.sha(self.root / 'scripts/check-management-package.py'),
            'oldConsumerManifestSha256': package.sha(self.root / package.management_consumers.MANIFEST),
            'retainedArchive': str(self.archive), 'archive_sha256': package.sha(self.archive),
            'archive_bytes': self.archive.stat().st_size, 'releaseCommit': self.commit, 'vcs': self.vcs,
            'files': [{'path': name, 'sha256': digest(data), 'bytes': len(data)} for name, data in sorted(self.files.items())],
            'oldConsumerSources': {row['path']: row['sha256'] for row in self.manifest['consumers']},
            'checks': checks, 'hostTarget': self.host, 'externalExampleConsumers': bindings,
            'frozenProbeCheckIndex': probe_index,
            'managementConsumerSourceSha256': digest(self.files['examples/red_team_management.rs']),
            'publicConsumerSourceSha256': 'c' * 64}
        policy = types.SimpleNamespace(inspect_archive=lambda data, prefix: copy.deepcopy(self.files),
                                       validate_vcs=lambda files, commit, clean: copy.deepcopy(self.vcs))
        self.addCleanup(mock.patch.stopall)
        mock.patch.object(package, 'fingerprint', return_value=self.source).start()
        mock.patch.object(package, 'package_module', return_value=policy).start()
        mock.patch.object(package.management_consumers, 'verify', return_value=self.manifest).start()

    def write(self, path, data):
        dest = self.root / path
        dest.parent.mkdir(parents=True, exist_ok=True)
        dest.write_bytes(data)
        return dest

    def verify(self, report=None):
        self.receipt.write_text(json.dumps(self.report if report is None else report))
        return package.verify_receipt(self.root, self.receipt, '1.99.0')

    def test_valid_receipt_accepts_exact_artifacts_and_commands(self):
        self.assertEqual(self.verify(), self.report)

    def test_stale_dirty_failed_and_wrong_compiler_receipts_rejected(self):
        for key, value in [('schemaVersion', 2), ('sourceSha256', '0' * 64), ('passed', False),
                           ('requireClean', False), ('effectiveDirty', True), ('toolchain', '1.89.0')]:
            with self.subTest(key=key):
                report = copy.deepcopy(self.report)
                report[key] = value
                with self.assertRaises(ValueError): self.verify(report)

    def test_receipt_input_archive_and_member_tampering_rejected(self):
        for key in ('packageScriptSha256', 'wrapperSha256', 'oldConsumerManifestSha256', 'archive_sha256'):
            with self.subTest(key=key):
                report = copy.deepcopy(self.report)
                report[key] = '0' * 64
                with self.assertRaises(ValueError): self.verify(report)
        for mutation in ('size', 'vcs', 'member'):
            report = copy.deepcopy(self.report)
            if mutation == 'size': report['archive_bytes'] += 1
            elif mutation == 'vcs': report['vcs']['git']['sha1'] = '0' * 40
            else: report['files'][0]['sha256'] = '0' * 64
            with self.assertRaises(ValueError): self.verify(report)

    def test_omitted_source_rejected_even_with_rehashed_member_ledger(self):
        del self.files['src/lib.rs']
        report = copy.deepcopy(self.report)
        report['files'] = [row for row in report['files'] if row['path'] != 'src/lib.rs']
        with self.assertRaisesRegex(ValueError, 'source inventory'):
            self.verify(report)

    def test_archive_or_receipt_symlink_rejected(self):
        link = self.root / 'archive-link.crate'
        link.symlink_to(self.archive)
        report = copy.deepcopy(self.report)
        report['retainedArchive'] = str(link)
        with self.assertRaises(ValueError): self.verify(report)
        self.verify()
        link = self.root / 'receipt-link.json'
        link.symlink_to(self.receipt)
        with self.assertRaises(ValueError): package.verify_receipt(self.root, link, '1.99.0')

    def test_receipt_and_archive_parent_symlinks_rejected(self):
        self.verify()
        original = self.root / 'real'
        original.mkdir()
        receipt = original / 'receipt.json'
        receipt.write_bytes(self.receipt.read_bytes())
        archive = original / 'archive.crate'
        archive.write_bytes(self.archive.read_bytes())
        alias = self.root / 'alias'
        alias.symlink_to(original, target_is_directory=True)
        with self.assertRaises(ValueError):
            package.verify_receipt(self.root, alias / 'receipt.json', '1.99.0')
        report = copy.deepcopy(self.report)
        report['retainedArchive'] = str(alias / 'archive.crate')
        with self.assertRaises(ValueError): self.verify(report)

    def test_non_cargo_and_inconsistent_executable_identity_rejected(self):
        for mutation in ('not-cargo', 'inconsistent'):
            report = copy.deepcopy(self.report)
            if mutation == 'not-cargo':
                for row in report['checks']: row['command'][0] = '/bin/echo'
            else:
                report['checks'][0]['command'][0] = '/other/bin/cargo'
            with self.assertRaises(ValueError): self.verify(report)
        report = copy.deepcopy(self.report)
        for row in report['checks']: row['command'][0] = '/reviewed/bin/cargo'
        self.assertEqual(self.verify(report), report)

    def test_failed_missing_and_wrong_compiler_commands_rejected(self):
        for mutation in ('failed', 'missing', 'compiler'):
            report = copy.deepcopy(self.report)
            if mutation == 'failed': report['checks'][0]['exit_code'] = 1
            elif mutation == 'missing': report['checks'][0]['command'][2] = 'build'
            else: report['checks'][0]['command'][1] = '+1.89.0'
            with self.assertRaises(ValueError): self.verify(report)

    def test_external_consumer_bindings_reject_wrong_index_hash_cwd_and_command(self):
        for mutation in ('index', 'hash', 'cwd', 'command', 'old', 'management', 'public'):
            report = copy.deepcopy(self.report)
            binding = report['externalExampleConsumers']['examples/response_defaults.rs']
            if mutation == 'index': binding['checkIndex'] = True
            elif mutation == 'hash': binding['sha256'] = '0' * 64
            elif mutation == 'cwd': report['checks'][binding['checkIndex']]['cwd'] = '/elsewhere/not-consumer'
            elif mutation == 'command': report['checks'][binding['checkIndex']]['command'][2] = 'check'
            elif mutation == 'old': report['oldConsumerSources'] = {}
            elif mutation == 'management': report['managementConsumerSourceSha256'] = '0' * 64
            else: report['publicConsumerSourceSha256'] = '0' * 64
            with self.assertRaises(ValueError): self.verify(report)

    def test_duplicate_or_missing_consumer_execution_is_rejected(self):
        for mutation in ('duplicate', 'missing-probe', 'probe-reuses-example', 'bad-probe-command'):
            report = copy.deepcopy(self.report)
            first = report['externalExampleConsumers']['examples/response_defaults.rs']['checkIndex']
            if mutation == 'duplicate':
                report['externalExampleConsumers']['examples/red_team_management.rs']['checkIndex'] = first
            elif mutation == 'missing-probe':
                del report['frozenProbeCheckIndex']
            elif mutation == 'probe-reuses-example':
                report['frozenProbeCheckIndex'] = first
            else:
                report['checks'][report['frozenProbeCheckIndex']]['command'][2] = 'check'
            with self.assertRaises(ValueError): self.verify(report)

    def test_archive_runtime_legal_and_manifest_bytes_bind_current_source(self):
        for member in ('src/lib.rs', 'LICENSE', 'NOTICE', 'README.md', 'Cargo.toml.orig'):
            with self.subTest(member=member):
                original = self.files[member]
                self.files[member] = original + b'changed'
                report = copy.deepcopy(self.report)
                # Even a self-consistent archive ledger cannot relabel stale source.
                row = next(row for row in report['files'] if row['path'] == member)
                row.update(sha256=digest(self.files[member]), bytes=len(self.files[member]))
                with self.assertRaisesRegex(ValueError, 'Archive source differs'): self.verify(report)
                self.files[member] = original

    def test_packaged_example_must_equal_current_reviewed_bytes(self):
        self.write('crates/scm-sdk/examples/red_team_management.rs', b'changed source')
        with self.assertRaisesRegex(ValueError, 'reviewed source'): self.verify()

    def test_source_changed_during_clean_execution_is_rejected(self):
        worktree = self.root / '.local/package-worktrees' / self.commit
        worktree.mkdir(parents=True)
        self.verify()
        with mock.patch.object(package, 'ROOT', self.root), \
             mock.patch.object(package, 'fingerprint', side_effect=[self.source, self.source, 'changed']), \
             mock.patch.object(package.subprocess, 'check_output', return_value=self.commit), \
             mock.patch.object(package.subprocess, 'run') as run, \
             mock.patch.object(package.sys, 'argv', ['check-management-package.py', '--toolchain', '1.99.0', '--output', str(self.receipt)]):
            with self.assertRaisesRegex(ValueError, 'Source changed'): package.main()
            self.assertEqual(run.call_count, 1)

    def test_uncommitted_candidate_rejected_before_package_execution(self):
        worktree = self.root / '.local/package-worktrees' / self.commit
        worktree.mkdir(parents=True)
        with mock.patch.object(package, 'ROOT', self.root), \
             mock.patch.object(package, 'fingerprint', side_effect=[self.source, 'different-worktree']), \
             mock.patch.object(package.subprocess, 'check_output', return_value=self.commit), \
             mock.patch.object(package.subprocess, 'run') as run, \
             mock.patch.object(package.sys, 'argv', ['check-management-package.py', '--toolchain', '1.99.0', '--output', str(self.receipt)]):
            with self.assertRaisesRegex(ValueError, 'Commit candidate'): package.main()
            run.assert_not_called()


if __name__ == '__main__':
    unittest.main()
