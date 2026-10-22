"""Retained contracts and bounded TypeScript 0.35.0 implementation invariants."""
import json
import hashlib
from pathlib import Path
import re
import subprocess
import tomllib
import unittest

from typescript035_profiles import Overlay, PROJECTIONS, default_aliases
from verify_typescript035_source import SDK_MAINTENANCE_BASELINE

ROOT = Path(__file__).resolve().parents[1]
BASELINE = SDK_MAINTENANCE_BASELINE


def old(path):
    return subprocess.check_output(['git', 'show', BASELINE + ':' + path], cwd=ROOT, timeout=30)


def load(path):
    return json.loads((ROOT / path).read_bytes())


class TypeScript035ImplementationTests(unittest.TestCase):
    def test_frozen_contracts_and_fixtures_preserved(self):
        paths = ['contracts/typescript.json', 'contracts/operations.json',
                 'contracts/export-classification.json', 'contracts/manifest.json',
                 'contracts/native-union-variants.json',
                 'contracts/sources/typescript/git-tree.json', 'contracts/sources/typescript/commit.txt']
        paths += subprocess.check_output(['git', 'ls-tree', '-r', '--name-only', BASELINE, '--',
                                         'contracts/fixtures', 'crates/scm-sdk/src/fixtures'], cwd=ROOT).decode().splitlines()
        paths.remove('crates/scm-sdk/src/fixtures/model-regressions.json')
        for path in paths:
            with self.subTest(path=path):
                self.assertEqual((ROOT / path).read_bytes(), old(path))

    def test_historical_oracle_only_refreshes_generator_provenance(self):
        path = 'contracts/native-fixtures/schemas.json'
        before, after = json.loads(old(path)), load(path)
        self.assertEqual(after['provenance']['generatorSha256'],
                         hashlib.sha256((ROOT / 'tools/schema-oracle.mts').read_bytes()).hexdigest())
        after['provenance']['generatorSha256'] = before['provenance']['generatorSha256']
        self.assertEqual(after, before)

    def test_packaged_specimens_only_refresh_oracle_provenance(self):
        path = 'crates/scm-sdk/src/fixtures/model-regressions.json'
        before, after = json.loads(old(path)), load(path)
        for key, source in [('sha256', 'contracts/native-fixtures/schemas.json'),
                            ('oracleSha256', 'tools/schema-oracle.mts')]:
            self.assertEqual(after['provenance'][key],
                             hashlib.sha256((ROOT / source).read_bytes()).hexdigest())
            after['provenance'][key] = before['provenance'][key]
        self.assertEqual(after, before)

    def test_maintenance_scopes_are_nonempty(self):
        status = load('docs/phase-status.json')
        for phase_id, expected in ((27, ['models-and-validation', 'wire-and-compatibility']),
                                   (28, ['release-readiness', 'remote-delivery'])):
            phase = next(p for p in status['phases'] if p['id'] == phase_id)
            self.assertEqual([c['id'] for c in phase['chunks']], expected)
            self.assertTrue(phase['scope_evidence'])
            for chunk in phase['chunks']:
                self.assertTrue(chunk['scope_evidence'])
                self.assertTrue(chunk['scope_revisions'])
                self.assertEqual(chunk['scope_revisions'][-1]['scope_evidence'], chunk['scope_evidence'])

    def test_profile_client_only_changes_two_request_roots(self):
        path = 'crates/scm-sdk/src/airs/management/profiles.rs'
        before, after = old(path).decode(), (ROOT / path).read_text()
        previous = '"CreateSecurityProfileRequestSchema"'
        current = '"typescript:0.35.0:CreateSecurityProfileRequestSchema"'
        self.assertEqual(before.count(previous), 2)
        self.assertEqual(after.count(current), 2)
        self.assertEqual(after.replace(current, previous), before)
        self.assertEqual(re.findall(r'\.named\("([^"]+)"\)', after),
                         re.findall(r'\.named\("([^"]+)"\)', before))

    def test_dependency_feature_and_msrv_contracts_unchanged(self):
        for path in ('Cargo.toml', 'crates/scm-sdk/Cargo.toml'):
            before = tomllib.loads(old(path).decode())
            after = tomllib.loads((ROOT / path).read_text())
            # New offline examples and the eventual patch version are additive delivery metadata.
            for document in (before, after):
                document.pop('example', None)
                if 'package' in document:
                    document['package'].pop('version', None)
            self.assertEqual(after, before, path)
        self.assertEqual(tomllib.loads((ROOT / 'Cargo.toml').read_text())['workspace']['package']['rust-version'], '1.89')

    def test_lockfile_changes_only_the_two_tls_security_patches(self):
        before = tomllib.loads(old('Cargo.lock').decode())
        after = tomllib.loads((ROOT / 'Cargo.lock').read_text())
        patched = {'rustls-webpki': ('0.103.15', '0.103.16'),
                   'webpki-root-certs': ('1.0.9', '1.0.10')}
        self.assertEqual(len(before['package']), len(after['package']))
        for previous, current in zip(before['package'], after['package']):
            self.assertEqual(previous['name'], current['name'])
            name = previous['name']
            if name in patched:
                self.assertEqual((previous.pop('version'), current.pop('version')), patched[name])
                self.assertNotEqual(previous.pop('checksum'), current.pop('checksum'))
            elif name == 'scm-sdk':
                previous.pop('version')
                current.pop('version')
            self.assertEqual(previous, current, name)
        from check_phase import commands_for
        commands = commands_for(27)
        audit = next(i for i, c in enumerate(commands) if 'scripts/check-dependencies.py' in c)
        first_rust = next(i for i, c in enumerate(commands) if 'scripts/check-rust.py' in c)
        self.assertLess(audit, first_rust)

    def test_graph_preserves_historical_nodes_and_roots(self):
        path = 'crates/scm-sdk/src/validation/schema_graph.json'
        before, after = json.loads(old(path)), load(path)
        for section in ('nodes', 'roots'):
            for key, value in before[section].items():
                self.assertEqual(after[section].get(key), value, section + ':' + key)
        overlay = Overlay()
        clones = {'s009588', 's009599', 's009605', 's009606', 's009608', 's009609',
                  's009612', 's009625', 's009632', 's009651', 's009653', 's009654'}
        self.assertEqual(set(overlay.mapping), clones)
        self.assertEqual(set(after['nodes']) - set(before['nodes']),
                         {'s_ts035_' + sid for sid in clones} | set(overlay.nodes))
        self.assertEqual(len(overlay.nodes), 4)
        self.assertEqual(set(after['roots']) - set(before['roots']), set(overlay.roots))
        self.assertEqual(len(overlay.roots), 8)
        self.assertEqual(after, overlay.extend_graph(before))

    def test_exact_additive_fields_and_default_alias(self):
        self.assertEqual({(schema, field) for schema, field, _ in PROJECTIONS}, {
            ('s009588', 'enable-full-conversation-inspection'),
            ('s009599', 'content-type-mode'),
            ('s009599', 'content-type-configurations'),
            ('s009612', 'dlp_tenant_id'),
            ('s009632', 'dlp_tenant_id'),
        })
        self.assertEqual(len(PROJECTIONS), 5)
        self.assertEqual(default_aliases(), {'s_ts035_s009653': 's009653'})

    def test_frozen_model_identity_and_union_inventory(self):
        path = 'contracts/native-models.json'
        before, after = json.loads(old(path)), load(path)
        for field in ('baselineCommit', 'schemas', 'modelExports', 'unionVariantPins', 'unionVariantPinsSha256', 'aliases'):
            self.assertEqual(after[field], before[field], field)
        self.assertEqual(len(after['typescript035Schemas']), 2)
        self.assertEqual(len(after['typescript035ModelExports']), 2)
        self.assertEqual({r['rustType'] for r in after['typescript035ModelExports']},
                         {'ContentTypeConfiguration', 'ContentTypeConfigurations'})


if __name__ == '__main__':
    unittest.main()
