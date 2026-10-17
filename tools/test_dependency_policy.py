"""Reject relaxed advisory policy, graph omissions and unpinned/unapproved sources."""
import copy
import importlib.util
import pathlib
import tomllib
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('policy', ROOT/'scripts/check-dependencies.py')
policy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(policy)


class DependencyPolicyTests(unittest.TestCase):
    def test_real_lock_and_policy_are_accepted(self):
        policy.check_lock(tomllib.loads((ROOT/'Cargo.lock').read_text()))
        policy.check_policy(tomllib.loads((ROOT/'deny.toml').read_text()))

    def test_any_advisory_or_license_exception_is_rejected(self):
        original=tomllib.loads((ROOT/'deny.toml').read_text())
        for table, key, value in [('advisories','ignore',['RUSTSEC-TEST']), ('licenses','exceptions',[{'name':'a'}]),
                                  ('licenses','clarify',[{'name':'a'}])]:
            changed=copy.deepcopy(original)
            changed[table][key]=value
            with self.assertRaisesRegex(ValueError,'exceptions'):
                policy.check_policy(changed)

    def test_graph_omissions_and_relaxed_sources_are_rejected(self):
        original=tomllib.loads((ROOT/'deny.toml').read_text())
        for table,key,value in [('graph','exclude',['a']), ('graph','all-features',False), ('graph','targets',['x86_64-unknown-linux-musl']),
                                ('licenses','allow',['MIT','GPL-3.0']), ('licenses','confidence-threshold',0.5),
                                ('sources','unknown-registry','allow'), ('sources','unknown-git','warn'),
                                ('sources','allow-git',['https://example.test/repo']),
                                ('sources','allow-registry',['https://example.test/index'])]:
            changed=copy.deepcopy(original)
            changed[table][key]=value
            with self.assertRaises(ValueError):
                policy.check_policy(changed)

    def test_missing_checksum_and_git_or_path_sources_are_rejected(self):
        package={'name':'synthetic','version':'1.0.0','source':'registry+https://github.com/rust-lang/crates.io-index','checksum':'a'*64}
        policy.check_lock({'package':[package]})
        for key,value in [('checksum','bad'),('source','git+https://example.test/repo'),('source',None)]:
            with self.assertRaises(ValueError):
                policy.check_lock({'package':[{**package,key:value}]})
