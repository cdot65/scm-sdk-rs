"""Hostile archive and clean-release metadata rejection tests."""
import importlib.util
import io
import json
import pathlib
import tarfile
import unittest

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
