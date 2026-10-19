"""Hostile archive and clean-release metadata rejection tests."""
import importlib.util
import io
import json
import pathlib
import tarfile
import unittest
from unittest import mock
import types

spec = importlib.util.spec_from_file_location('package_policy', pathlib.Path(__file__).resolve().parents[1] / 'scripts/check-package.py')
policy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(policy)


class PackagePolicyTests(unittest.TestCase):
    def archive(self, names):
        output = io.BytesIO()
        with tarfile.open(fileobj=output, mode='w:gz') as archive:
            for name, kind in names:
                member = tarfile.TarInfo(name)
                member.type = kind
                member.size = 1 if kind == tarfile.REGTYPE else 0
                archive.addfile(member, io.BytesIO(b'x') if member.size else None)
        return output.getvalue()

    def test_unsafe_members_are_rejected_before_extraction(self):
        for name, kind in [('pkg/../outside',tarfile.REGTYPE), ('/pkg/file',tarfile.REGTYPE),
                           ('other/file',tarfile.REGTYPE), ('pkg/link',tarfile.SYMTYPE),
                           ('pkg/dev',tarfile.CHRTYPE), ('pkg/a\\b',tarfile.REGTYPE),
                           ('pkg/.local/secret',tarfile.REGTYPE), ('pkg/key.pem',tarfile.REGTYPE),
                           ('pkg/.cargo/config.toml',tarfile.REGTYPE), ('pkg/src/external-spec.yaml',tarfile.REGTYPE),
                           ('pkg/src/D:/outside.rs',tarfile.REGTYPE), ('pkg/src/file:stream',tarfile.REGTYPE),
                           ('pkg/src/control\x01.rs',tarfile.REGTYPE), ('pkg/src/control\x7f.rs',tarfile.REGTYPE)]:
            with self.assertRaises(ValueError):
                policy.inspect_archive(self.archive([(name,kind)]), 'pkg/')

    def test_development_prefix_and_credential_suffix_guards_are_reachable(self):
        for name in ('.local/secret', 'contracts/source.json', 'tools/check.py', 'node_modules/a.js',
                     'src/credential.pem', 'examples/credential.key'):
            with self.subTest(name=name):
                with self.assertRaisesRegex(ValueError, 'Development/credential input'):
                    policy.inspect_archive(self.archive([('pkg/' + name, tarfile.REGTYPE)]), 'pkg/')

    def test_external_consumer_manifest_enables_frozen_cfg_without_modifying_source(self):
        original = '[package]\nname = "consumer"\n[dependencies]\nscm-sdk = { path = "../../crates/scm-sdk" }\n'
        manifest = policy.external_consumer_manifest(original)
        parsed = policy.tomllib.loads(manifest)
        self.assertEqual(parsed['features'], {'red-team': ['scm-sdk/red-team']})
        self.assertEqual(parsed['dependencies']['scm-sdk']['path'], '../scm-sdk')
        # Dependencies added for later examples remain in the dependencies table.
        extended = policy.tomllib.loads(manifest + 'bytes = "=1.0.0"\n')
        self.assertEqual(extended['dependencies']['bytes'], '=1.0.0')
        self.assertNotIn('[features]', original)
        for malformed in (original.replace('[dependencies]', ''), original.replace('../../crates/scm-sdk', 'other'),
                          original + '[features]\nred-team = []\n'):
            with self.assertRaises(ValueError):
                policy.external_consumer_manifest(malformed)

    def test_compiler_identity_queries_requested_rustup_toolchain(self):
        compiler = pathlib.Path('/toolchain/bin/cargo')
        helper = types.SimpleNamespace(cargo_path=lambda: compiler)
        loader = mock.Mock()
        version = 'rustc 1.99.0\nhost: x86_64-unknown-linux-gnu\n'
        with mock.patch.object(policy.importlib.util, 'spec_from_file_location', return_value=types.SimpleNamespace(loader=loader)), \
             mock.patch.object(policy.importlib.util, 'module_from_spec', return_value=helper), \
             mock.patch.object(policy.subprocess, 'check_output', return_value=version) as run:
            self.assertEqual(policy.compiler_identity(pathlib.Path('/repo'), '1.99.0'),
                             (compiler, version, 'x86_64-unknown-linux-gnu'))
            command = run.call_args.args[0]
            self.assertEqual(command[1:], ['+1.99.0', '-vV'])
            self.assertIn(pathlib.Path(command[0]).name, ('rustc', 'rustc.exe'))

    def test_duplicate_members_rejected(self):
        with self.assertRaisesRegex(ValueError,'duplicate'):
            policy.inspect_archive(self.archive([('pkg/src/file.rs',tarfile.REGTYPE)] * 2), 'pkg/')

    def test_cargo_omitted_false_dirty_flag_is_valid_clean_metadata(self):
        files = {'.cargo_vcs_info.json': json.dumps({'git': {'sha1': 'a' * 40}, 'path_in_vcs': 'crates/scm-sdk'}).encode()}
        parsed = policy.validate_vcs(files, 'a' * 40, True)
        self.assertNotIn('dirty', parsed['git'])

    def test_wrong_or_dirty_release_vcs_rejected(self):
        def files(commit='a'*40, dirty=False, path='crates/scm-sdk'):
            return {'.cargo_vcs_info.json':json.dumps({'git':{'sha1':commit,'dirty':dirty},'path_in_vcs':path}).encode()}
        policy.validate_vcs(files(), 'a'*40, True)
        for value in (files(commit='b'*40), files(dirty=True), files(path='../elsewhere'), files(dirty=None)):
            with self.assertRaises(ValueError):
                policy.validate_vcs(value,'a'*40,True)
