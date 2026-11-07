"""Portkey-AI/openapi drift: classification, code-sample annotations, repository files and tampering."""
import json
import socket
import tempfile
import unittest
from pathlib import Path
from unittest import mock

import airs_drift as drift
import portkey_drift as pk

COMMIT = 'c' * 40
OTHER = 'd' * 40
BASE_SPEC = {
    'openapi': '3.0.0', 'info': {'title': 'Portkey', 'version': '1.0.0'},
    'paths': {'/mcp-servers': {'get': {
        'operationId': 'listMcpServers',
        'parameters': [{'in': 'query', 'name': 'tags', 'schema': {'type': 'string'}}],
        'responses': {'200': {'description': 'ok'}},
        'x-code-samples': [{'lang': 'python', 'source': 'client.get("/mcp-servers")'}]}}},
}
LICENSE_TEXT = b'MIT License\n'


def dumps(value):
    return (json.dumps(value, indent=2) + '\n').encode()


class PortkeyDriftTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        pin_dir = self.root / pk.PIN_DIR
        pin_dir.mkdir(parents=True)
        spec_bytes = dumps(BASE_SPEC)
        (pin_dir / pk.SPEC).write_bytes(spec_bytes)
        (pin_dir / pk.LICENSE).write_bytes(LICENSE_TEXT)
        files = {name: {'sha256': drift.sha(data), 'gitBlobSha1': drift.git_blob_sha1(data), 'bytes': len(data)}
                 for name, data in ((pk.SPEC, spec_bytes), (pk.LICENSE, LICENSE_TEXT))}
        (pin_dir / 'source.json').write_text(json.dumps({'commit': COMMIT, 'url': pk.REPOSITORY, 'files': files}))
        baseline = self.candidate(BASE_SPEC, commit=COMMIT, name='pin')
        tree = json.loads((baseline / 'tree.json').read_text())
        document = pk.ledger({'commit': COMMIT, 'rootTreeSha': tree['rootTreeSha'], 'rootEntries': tree['rootEntries']},
                             self.root)
        (self.root / pk.LEDGER).parent.mkdir(parents=True, exist_ok=True)
        (self.root / pk.LEDGER).write_text(pk.ledger_text(document))

    def candidate(self, spec, *, commit=OTHER, name='candidate', license_text=LICENSE_TEXT, extra_root=()):
        directory = self.root / 'candidates' / name
        (directory / 'files').mkdir(parents=True)
        spec_bytes = spec if isinstance(spec, bytes) else dumps(spec)
        (directory / 'files' / pk.SPEC).write_bytes(spec_bytes)
        (directory / 'files' / pk.LICENSE).write_bytes(license_text)
        entries = [{'path': pk.SPEC, 'type': 'blob', 'sha': drift.git_blob_sha1(spec_bytes)},
                   {'path': pk.LICENSE, 'type': 'blob', 'sha': drift.git_blob_sha1(license_text)},
                   {'path': 'README.md', 'type': 'blob', 'sha': 'a' * 40}, *extra_root]
        tree = {'schemaVersion': 1, 'repository': pk.REPOSITORY, 'commit': commit, 'rootTreeSha': 'b' * 40,
                'rootEntries': sorted(entries, key=lambda e: e['path'])}
        (directory / 'tree.json').write_text(json.dumps(tree))
        return directory

    def compare(self, spec, **options):
        return pk.compare_candidate(self.candidate(spec, **options), self.root)

    def mutated(self, change):
        spec = json.loads(json.dumps(BASE_SPEC))
        change(spec)
        return spec

    def test_check_and_self_compare_are_unchanged(self):
        pk.check(self.root)
        self.assertEqual(self.compare(BASE_SPEC, commit=COMMIT, name='same')['status'], 'unchanged')

    def test_code_sample_only_change_is_an_annotation(self):
        spec = self.mutated(lambda s: s['paths']['/mcp-servers']['get']['x-code-samples'][0].update(source='new'))
        report = self.compare(spec)
        self.assertEqual(report['status'], 'annotation_only')
        self.assertEqual([c['pointer'] for c in report['codeSamples']], ['/paths/~1mcp-servers/get/x-code-samples'])

    def test_api_changes_are_classified_and_require_review(self):
        def add(spec):
            spec['paths']['/mcp-servers/{id}'] = {'get': {'operationId': 'getMcpServer',
                                                          'responses': {'200': {'description': 'ok'}}}}
            spec['paths']['/mcp-servers']['get']['parameters'][0]['required'] = True
        report = self.compare(self.mutated(add))
        self.assertEqual(report['status'], 'review_required')
        classes = {c['class'] for c in report['specification']['changes']}
        self.assertTrue({'operation_added', 'requiredness_changed'} <= classes)

    def test_unknown_keywords_still_fail_closed(self):
        spec = self.mutated(lambda s: s['paths']['/mcp-servers']['get'].update({'x-internal': True}))
        with self.assertRaisesRegex(ValueError, 'no receipt written'):
            self.compare(spec)

    def test_license_and_root_files_are_reported(self):
        report = self.compare(BASE_SPEC, license_text=b'Other License\n',
                              extra_root=[{'path': 'openapi-v2.yaml', 'type': 'blob', 'sha': 'e' * 40}])
        self.assertEqual(report['status'], 'review_required')
        self.assertEqual({c['class'] for c in report['repositoryFiles']}, {'license_changed', 'repository_file_added'})

    def test_tampering_is_refused(self):
        directory = self.candidate(BASE_SPEC, name='tampered')
        (directory / 'files' / pk.SPEC).write_bytes(b'{}\n')
        with self.assertRaisesRegex(ValueError, 'does not match its listed blob'):
            pk.compare_candidate(directory, self.root)
        same = self.candidate(self.mutated(lambda s: s['info'].update(title='Changed')), commit=COMMIT, name='pin-bytes')
        with self.assertRaisesRegex(ValueError, 'source tampering'):
            pk.compare_candidate(same, self.root)
        (self.root / pk.PIN_DIR / pk.LICENSE).write_bytes(b'tampered\n')
        with self.assertRaisesRegex(ValueError, 'Pinned Portkey bytes changed'):
            pk.check(self.root)

    def test_stale_ledger_fails_check(self):
        path = self.root / pk.LEDGER
        document = json.loads(path.read_text())
        document['watched'] = [pk.SPEC]
        path.write_text(pk.ledger_text(document))
        with self.assertRaisesRegex(ValueError, 'Stale or tampered'):
            pk.check(self.root)

    def test_check_and_compare_never_use_the_network(self):
        with mock.patch.object(socket, 'socket', side_effect=AssertionError('network used')):
            pk.check(self.root)
            self.compare(BASE_SPEC, name='offline')

    def test_refresh_verifies_blobs_with_an_injected_fetcher(self):
        spec_bytes = dumps(BASE_SPEC)
        blobs = {pk.SPEC: spec_bytes, pk.LICENSE: LICENSE_TEXT}
        listing = {'sha': 't' * 40, 'truncated': False, 'tree': [
            {'path': name, 'type': 'blob', 'sha': drift.git_blob_sha1(data)} for name, data in blobs.items()]}

        def fetch(url, headers, limit):
            if url == f'{pk.GITHUB_API}/commits/{OTHER}':
                body = json.dumps({'sha': OTHER, 'commit': {'tree': {'sha': 't' * 40}, 'committer': {'date': 'x'}}})
            elif url == f'{pk.GITHUB_API}/git/trees/{"t" * 40}':
                body = json.dumps(listing)
            else:
                body = blobs[url.rsplit('/', 1)[1]]
            return 200, url, body.encode() if isinstance(body, str) else body
        output = pk.ROOT / '.local/portkey-drift' / ('test-' + self.id().rsplit('.', 1)[1])
        self.addCleanup(lambda: __import__('shutil').rmtree(output, ignore_errors=True))
        target = pk.refresh(OTHER, output, fetch=fetch, token='secret-token')
        self.assertTrue((target / 'files' / pk.SPEC).is_file())
        self.assertNotIn('secret-token', (target / 'tree.json').read_text())
        blobs[pk.SPEC] = b'changed\n'
        with self.assertRaisesRegex(ValueError, 'does not match its blob'):
            pk.refresh(OTHER, output.with_name(output.name + '-bad'), fetch=fetch)
        with self.assertRaisesRegex(ValueError, 'beneath'):
            pk.refresh(OTHER, pk.ROOT / 'contracts/portkey-escape', fetch=fetch)


if __name__ == '__main__':
    unittest.main()
