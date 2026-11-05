"""Provenance, complete-delta classification and tamper guards for the Portkey 8e5da0e pin."""
import difflib
import json
import os
import pathlib
import shutil
import subprocess
import tempfile
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
PIN = ROOT / 'contracts/vendor/portkey-mcp-tags/8e5da0e'
SCRIPT = ROOT / 'tools/portkey-mcp-tags.mjs'


def run(directory=None, check=True):
    env = dict(os.environ)
    if directory is not None:
        env['PORTKEY_MCP_TAGS_DIR'] = str(directory)
    return subprocess.run(['node', str(SCRIPT), *(['--check'] if check else [])], cwd=ROOT, env=env,
                          text=True, capture_output=True, timeout=300)


class PortkeyMcpTagsProvenanceTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.copy = pathlib.Path(self.temporary.name) / 'pin'
        shutil.copytree(PIN, self.copy)

    def tearDown(self):
        self.temporary.cleanup()

    def assert_rejected(self, message):
        result = run(self.copy)
        self.assertNotEqual(result.returncode, 0, result.stdout)
        self.assertIn(message, result.stderr)

    def test_pinned_delta_is_reproducible_and_fully_classified(self):
        result = run()
        self.assertEqual(result.returncode, 0, result.stderr)
        report = json.loads(result.stdout.strip().splitlines()[-1])
        self.assertEqual(report['deltaItems'], 61)
        self.assertEqual(report['dispositions'], {'import': 10, 'already_supported': 1, 'not_applicable': 25,
                                                  'deferred_new_capability': 23, 'documentation_only': 2})
        self.assertEqual((report['importedSchemas'], report['importedListOperations']), (8, 2))

    def test_exact_line_delta_against_the_existing_pin(self):
        source = json.loads((PIN / 'source.json').read_text())
        before = (ROOT / source['base']['path']).read_text().splitlines()
        after = (PIN / 'openapi.yaml').read_text().splitlines()
        added = removed = 0
        for op, a, b, c, d in difflib.SequenceMatcher(None, before, after, autojunk=False).get_opcodes():
            if op != 'equal':
                added += d - c
                removed += b - a
        self.assertEqual((added, removed), (745, 6))

    def test_pins_identify_the_upstream_objects(self):
        source = json.loads((PIN / 'source.json').read_text())
        self.assertEqual(source['commit'], '8e5da0e756bc49ee3e82ca13df693b39fa541627')
        self.assertEqual(source['base']['commit'], 'a27feb9efaac3653d2d94eca5d7d210944b6e160')
        access = json.loads((ROOT / 'contracts/vendor/portkey-access/a27feb9/source.json').read_text())
        self.assertEqual(source['base']['sha256'], access['files']['openapi.yaml']['sha256'])
        self.assertEqual(source['licenseSha256'], access['licenseSha256'])
        self.assertEqual(len(source['upstreamCommits']), 10)

    def test_modified_pinned_bytes_are_rejected(self):
        path = self.copy / 'openapi.yaml'
        path.write_bytes(path.read_bytes().replace(b'createThreadMessage', b'createMessage', 1))
        self.assert_rejected('Pinned file changed: openapi.yaml')

    def test_coordinated_byte_and_manifest_rewrite_is_rejected(self):
        import hashlib
        path = self.copy / 'openapi.yaml'
        data = path.read_bytes()
        marker = b'AnthropicTextContent:'
        self.assertIn(marker, data)
        changed = data.replace(b'createThreadMessage', b'createThreadMessagX', 1)
        path.write_bytes(changed)
        source_path = self.copy / 'source.json'
        source = json.loads(source_path.read_text())
        source['files']['openapi.yaml'].update(
            sha256=hashlib.sha256(changed).hexdigest(), bytes=len(changed),
            gitBlobSha1=hashlib.sha1(b'blob %d\0' % len(changed) + changed).hexdigest())
        source_path.write_text(json.dumps(source))
        self.assert_rejected('differs from reviewed anchors: openapi.yaml')

    def test_unclassified_and_duplicate_delta_items_are_rejected(self):
        ledger_path = self.copy / 'delta.json'
        ledger = json.loads(ledger_path.read_text())
        removed = ledger['groups'][4]['items'].pop()
        ledger_path.write_text(json.dumps(ledger))
        self.assert_rejected('Unclassified delta')
        ledger['groups'][4]['items'].append(removed)
        ledger['groups'][7]['items'].append(removed)
        ledger_path.write_text(json.dumps(ledger))
        self.assert_rejected('classified more than once')

    def test_stale_classification_and_unknown_disposition_are_rejected(self):
        ledger_path = self.copy / 'delta.json'
        ledger = json.loads(ledger_path.read_text())
        ledger['groups'][7]['items'].append('top:info:changed')
        ledger_path.write_text(json.dumps(ledger))
        self.assert_rejected('Unclassified delta')
        ledger['groups'][7]['items'].pop()
        ledger['groups'][0]['disposition'] = 'maybe'
        ledger_path.write_text(json.dumps(ledger))
        self.assert_rejected('Invalid delta group')

    def test_extracted_contract_drift_is_rejected(self):
        path = self.copy / 'contract.json'
        contract = json.loads(path.read_text())
        contract['components']['schemas']['McpServer']['properties']['tags']['nullable'] = False
        path.write_text(json.dumps(contract, indent=2) + '\n')
        self.assert_rejected('Contract drift')

    def test_imported_fragments_are_exactly_the_reviewed_tags_contract(self):
        contract = json.loads((PIN / 'contract.json').read_text())
        schemas = contract['components']['schemas']
        self.assertEqual(set(schemas), {'CreateMcpIntegration', 'UpdateMcpIntegration', 'CreateMcpServer', 'UpdateMcpServer',
                                        'McpIntegration', 'McpIntegrationListItem', 'McpServer', 'McpServerListItem'})
        for schema in schemas.values():
            tags = schema['properties']['tags']
            self.assertEqual((tags['type'], tags['additionalProperties'], tags['nullable']), ('object', {'type': 'string'}, True))
        names = {path: [row.get('name') for row in contract['paths'][path]['get']['parameters']] for path in contract['paths']}
        self.assertEqual(names['/mcp-integrations'], ['organisation_id', 'type', 'workspace_id', 'page_size', 'current_page', 'search', 'tags'])
        self.assertEqual(names['/mcp-servers'], ['workspace_id', 'current_page', 'page_size', 'id', 'search', 'tags'])

    def test_already_supported_integration_tags_are_frozen_typed_fields(self):
        definitions = json.loads((ROOT / 'contracts/typescript.json').read_text())['schemas']['definitions']
        data = definitions['s007334']['properties']['data']['schema']['$ref']
        self.assertEqual(definitions[data]['element'], {'$ref': 's007274'})
        self.assertIn('tags', definitions['s007274']['properties'])

    def test_plan_and_phase_registration_record_the_codex_review_policy(self):
        plan = (ROOT / 'docs/portkey-mcp-tags-plan.md').read_text()
        for text in ('8e5da0e756bc49ee3e82ca13df693b39fa541627', 'Codex CLI', 'at least 9', 'five-attempt budget'):
            self.assertIn(text, plan)
        status = json.loads((ROOT / 'docs/phase-status.json').read_text())
        rows = {row['id']: row for row in status['phases']}
        self.assertTrue({29, 30, 31} <= set(rows))
        for phase in (29, 30, 31):
            self.assertEqual(rows[phase]['plan'], 'docs/portkey-mcp-tags-plan.md')
            self.assertEqual(rows[phase]['review_policy']['reviewer'], 'codex')
            self.assertEqual(rows[phase]['review_policy']['minimum_score'], 9)


if __name__ == '__main__':
    unittest.main()
