"""Reject relaxed advisory policy, graph omissions and unpinned/unapproved sources."""
import copy
import contextlib
import importlib.util
import io
import json
import pathlib
import subprocess
import sys
import tempfile
import tomllib
import unittest
from unittest import mock

ROOT = pathlib.Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('policy', ROOT/'scripts/check-dependencies.py')
policy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(policy)


class DependencyPolicyTests(unittest.TestCase):
    def test_probe_audit_is_independent_and_fails_closed(self):
        clean = {'vulnerabilities': {'count': 0}, 'warnings': {}}
        scenarios = ('clean', 'advisory', 'warning', 'exit', 'malformed', 'missing-warnings', 'changed')
        for scenario in scenarios:
            with self.subTest(scenario=scenario), tempfile.TemporaryDirectory() as temporary:
                root = pathlib.Path(temporary)
                probe = root / 'contracts/native-probes/Cargo.lock'
                probe.parent.mkdir(parents=True)
                (root / 'Cargo.lock').write_text('root lock')
                probe.write_text('probe lock')
                commands = []

                def run(command):
                    commands.append(command)
                    is_probe = str(probe) in command
                    document = copy.deepcopy(clean)
                    code = 0
                    if is_probe:
                        if scenario == 'advisory':
                            document['vulnerabilities']['count'] = 1
                        elif scenario == 'warning':
                            document['warnings'] = {'unmaintained': [{'id': 'SYNTHETIC'}]}
                        elif scenario == 'exit':
                            code = 1
                        elif scenario == 'missing-warnings':
                            del document['warnings']
                        elif scenario == 'changed':
                            probe.write_text('changed during audit')
                    stdout = 'not JSON' if is_probe and scenario == 'malformed' else json.dumps(document)
                    return subprocess.CompletedProcess(command, code, stdout, '')

                with mock.patch.object(policy, 'ROOT', root):
                    if scenario == 'malformed':
                        with self.assertRaisesRegex(ValueError, 'machine-readable'):
                            policy.audit_lockfiles('cargo-audit', '/pinned-db', run)
                    else:
                        results = policy.audit_lockfiles('cargo-audit', '/pinned-db', run)
                        self.assertTrue(results[0]['passed'])
                        self.assertEqual(all(row['passed'] for row in results), scenario == 'clean')
                        self.assertEqual(results[1]['path'], 'contracts/native-probes/Cargo.lock')
                        self.assertEqual(results[1]['lockfileSha256'], policy.hashlib.sha256(b'probe lock').hexdigest())
                        self.assertEqual(results[1]['command'], commands[1])
                self.assertEqual(len(commands), 2)
                for command in commands:
                    self.assertEqual(command[command.index('--db') + 1], '/pinned-db')
                    self.assertIn('--no-fetch', command)
                    self.assertEqual(command[-2:], ['--deny', 'warnings'])
        with tempfile.TemporaryDirectory() as temporary, mock.patch.object(policy, 'ROOT', pathlib.Path(temporary)):
            with self.assertRaises(OSError):
                policy.audit_lockfiles('cargo-audit', '/pinned-db', mock.Mock())

    def test_dependency_diagnostics_redact_url_credentials_and_authorization(self):
        message = ('https://synthetic-user:synthetic-password@example.test/repo?access_token=synthetic-token\n'
                   'Authorization: Bearer synthetic-authorization\n'
                   'Proxy-Authorization: Basic synthetic-proxy\n')
        result = policy.output_tail(message)
        for value in ('synthetic-user', 'synthetic-password', 'synthetic-token',
                      'synthetic-authorization', 'synthetic-proxy'):
            self.assertNotIn(value, result)
        self.assertIn('example.test/repo', result)
        self.assertIn('[REDACTED]', result)

    def test_precondition_failure_writes_a_partial_report_without_inventing_child_output(self):
        with tempfile.TemporaryDirectory() as temporary:
            output = pathlib.Path(temporary) / 'failed.json'
            diagnostic = io.StringIO()
            with mock.patch.object(policy, 'check_policy', side_effect=ValueError('Synthetic policy rejected')), \
                 mock.patch.object(policy.subprocess, 'run') as run, \
                 mock.patch.object(sys, 'argv', ['check-dependencies.py', '--output', str(output)]), \
                 contextlib.redirect_stderr(diagnostic):
                self.assertEqual(policy.main(), 1)
            run.assert_not_called()
            report = json.loads(output.read_text())
            self.assertFalse(report['passed'])
            self.assertTrue(report['partial'])
            self.assertEqual(report['checks'], [])
            self.assertIn('Synthetic policy rejected', diagnostic.getvalue())

    def test_early_child_failures_preserve_bounded_diagnostics_and_a_failed_report(self):
        for failure in ('metadata', 'git_fetch', 'tool_version', 'metadata_timeout'):
            with self.subTest(failure=failure), tempfile.TemporaryDirectory() as temporary:
                directory = pathlib.Path(temporary)
                for name in policy.VERSIONS:
                    (directory / name).write_text('synthetic tool fixture\n')
                output = directory / 'failed.json'
                stdout = 'STDOUT_START' + 'o' * 20000 + 'STDOUT_TAIL'
                stderr = 'STDERR_START' + 'e' * 20000 + 'FAILURE_TAIL'

                def fake_run(command, **kwargs):
                    values = list(map(str, command))
                    name = pathlib.Path(values[0]).name
                    failing = ((failure == 'metadata' and 'metadata' in values)
                               or (failure == 'git_fetch' and 'fetch' in values)
                               or (failure == 'tool_version' and '--version' in values))
                    if failure == 'metadata_timeout' and 'metadata' in values:
                        raise subprocess.TimeoutExpired(values, 3, output=stdout.encode(),
                                                        stderr=stderr.encode())
                    if failing:
                        return subprocess.CompletedProcess(values, 101, stdout, stderr)
                    if '--version' in values:
                        text = name + ' ' + policy.VERSIONS[name] + '\n'
                    elif 'get-url' in values:
                        text = policy.DATABASE + '\n'
                    elif 'rev-parse' in values:
                        text = 'a' * 40 + '\n'
                    elif 'show' in values:
                        text = '2026-10-17T00:00:00+00:00\n'
                    elif name == 'cargo-audit':
                        text = json.dumps({'vulnerabilities': {'count': 0}, 'warnings': {}})
                    else:
                        text = ''
                    return subprocess.CompletedProcess(values, 0, text, '')

                def fake_output(command, **kwargs):
                    result = fake_run(command, **kwargs)
                    if result.returncode:
                        raise subprocess.CalledProcessError(result.returncode, command,
                                                            result.stdout, result.stderr)
                    return result.stdout if kwargs.get('text') else result.stdout.encode()

                diagnostic = io.StringIO()
                returned = None
                with mock.patch.object(policy.subprocess, 'run', side_effect=fake_run), \
                     mock.patch.object(policy.subprocess, 'check_output', side_effect=fake_output), \
                     mock.patch.object(sys, 'argv', ['check-dependencies.py', '--tools-dir',
                                                    str(directory), '--output', str(output)]), \
                     contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(diagnostic):
                    try:
                        returned = policy.main()
                    except (ValueError, OSError, subprocess.SubprocessError):
                        pass
                self.assertTrue(output.is_file(), 'Early dependency failure lost its partial report')
                self.assertEqual(returned, 1)
                report = json.loads(output.read_text())
                self.assertFalse(report['passed'])
                failed = [item for item in report['checks'] if item['exit_code'] != 0]
                self.assertEqual(len(failed), 1)
                self.assertLessEqual(len(failed[0]['stdout']), 16384)
                self.assertLessEqual(len(failed[0]['stderr']), 16384)
                self.assertTrue(failed[0]['stdout'].endswith('STDOUT_TAIL'))
                self.assertTrue(failed[0]['stderr'].endswith('FAILURE_TAIL'))
                self.assertIn('FAILURE_TAIL', diagnostic.getvalue())
                self.assertNotIn('STDERR_START', diagnostic.getvalue())
                self.assertEqual(failed[0].get('timeout', False), failure == 'metadata_timeout')

    def test_resolved_feature_graph_records_actual_unified_features(self):
        metadata = {'packages': [
            {'id': 'json-1', 'name': 'serde_json', 'version': '1.0.0', 'source': 'registry'},
            {'id': 'consumer', 'name': 'consumer', 'version': '0.0.0', 'source': None}],
            'resolve': {'nodes': [
                {'id': 'consumer', 'features': ['forwarded-json']},
                {'id': 'json-1', 'features': ['std', 'raw_value', 'default']}]}}
        ledger = policy.check_resolved_features(metadata)
        self.assertEqual(ledger[1]['features'], ['default', 'raw_value', 'std'])
        # Cargo resolve nodes unify direct and transitive feature requests.
        changed = copy.deepcopy(metadata)
        changed['resolve']['nodes'][1]['features'].append('preserve_order')
        with self.assertRaisesRegex(ValueError, 'preserve_order'):
            policy.check_resolved_features(changed)
        changed = copy.deepcopy(metadata)
        changed['packages'].append({'id': 'json-2', 'name': 'serde_json', 'version': '2.0.0', 'source': 'registry'})
        changed['resolve']['nodes'].append({'id': 'json-2', 'features': ['preserve_order']})
        with self.assertRaisesRegex(ValueError, 'preserve_order'):
            policy.check_resolved_features(changed)
        for resolution in (None, {}, {'nodes': []}, {'nodes': [{'id': 'consumer', 'features': []}]},
                           {'nodes': [{'id': 'json-1', 'features': []}, {'id': 'json-1', 'features': []}]},
                           {'nodes': [{'id': 'json-1', 'features': None}]}):
            changed = copy.deepcopy(metadata)
            changed['resolve'] = resolution
            with self.assertRaises(ValueError):
                policy.check_resolved_features(changed)

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
