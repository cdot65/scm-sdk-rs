"""Offline regressions for management plan evidence (not runtime conformance)."""
import copy
import subprocess
import sys
import unittest
from unittest.mock import patch

import red_team_management as subject


class ManagementPlanTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.outputs = subject.build()

    def test_seven_sources_and_160_operations(self):
        sources = self.outputs[subject.Path('contracts/airs-active-sources.json')]['sources']
        self.assertEqual({x['upstreamPath']: x['operationCount'] for x in sources}, subject.COUNTS)
        self.assertEqual(len(self.outputs[subject.Path('contracts/airs-operations.json')]['operations']), 160)
        management = next(row for row in sources if row['activeSourceId'] == subject.SOURCE_ID)
        self.assertNotEqual(management['frozenSha256'], management['activeSpecificationSha256'])
        self.assertEqual(management['adoptionState'], 'pinned_for_phase22_not_runtime_adopted')

    def test_every_delta_has_disposition_and_no_removed_operations(self):
        decisions = self.outputs[subject.VENDOR / 'compatibility-decisions.json']
        self.assertEqual(decisions['baselineVersion'], '0.7.67')
        self.assertEqual(decisions['currentVersion'], '0.86.0')
        self.assertGreater(len(decisions['deltas']), 60)
        self.assertTrue(all(x['category'] and x['decision'] for x in decisions['deltas']))
        self.assertFalse(any(x['kind'] == 'removed' and x['path'][0] == 'paths' for x in decisions['deltas']))
        self.assertTrue(any('VertexAI' in '/'.join(x['path']) and x['affectedOperations'] for x in decisions['deltas']))

    def test_structural_diff_preserves_null_removal_and_union_order(self):
        changes = subject.differences({'x': None, 'u': [1, 2]}, {'y': None, 'u': [2, 1]})
        self.assertEqual([x['kind'] for x in changes], ['changed', 'removed', 'added'])
        self.assertEqual(changes[0]['old'], [1, 2])
        self.assertIsNone(changes[1]['old'])
        self.assertIsNone(changes[2]['new'])
        self.assertEqual(len(subject.differences(False, 0)), 1)

    def test_unclassified_delta_rejected(self):
        for path in [['servers', '0'], ['components', 'schemas', 'FutureThing']]:
            with self.assertRaisesRegex(ValueError, 'Unclassified'):
                subject.disposition({'path': path, 'kind': 'added', 'new': {}})

    def test_closure_handles_cycles_escaped_pointers_and_rejects_remote(self):
        spec = {'components': {'schemas': {'a/b': {'$ref': '#/components/schemas/a~1b'}}}}
        self.assertEqual(subject.closure({'$ref': '#/components/schemas/a~1b'}, spec), ['#/components/schemas/a~1b'])
        with self.assertRaisesRegex(ValueError, 'external reference'):
            subject.closure({'$ref': 'https://example.invalid/schema'}, spec)

    def test_pinned_hash_tampering_rejected(self):
        with self.assertRaisesRegex(ValueError, 'Pinned bytes changed'):
            subject.checked_bytes(subject.VENDOR / 'mp-openapi.yaml', '0' * 64)
        with self.assertRaisesRegex(ValueError, 'Unsafe pin path'):
            subject.checked_bytes('../outside.yaml', '0' * 64)

    def test_retention_includes_all_baseline_methods_exports_defaults(self):
        retained = self.outputs[subject.Path('contracts/airs-retained-capabilities.json')]
        self.assertEqual(len(retained['methods']), 448)
        self.assertEqual(len(retained['exports']), 3820)
        self.assertEqual(len(retained['defaultBindings']), 23)
        self.assertEqual(sum(len(x['options']) for x in retained['modelSecurityExtraQueries']), 14)
        self.assertEqual(len(retained['redTeamExtraMethods']), 11)
        self.assertEqual(len(retained['additionalTargetTemplates']), 2)

    def test_structural_annotation_names_removals_and_security_fail_closed(self):
        changes = [
            {'path': ['components', 'schemas', 'TargetSchema', 'properties', 'description'], 'kind': 'added', 'new': {'type': 'number'}},
            {'path': ['components', 'schemas', 'TargetSchema', 'properties', 'name'], 'kind': 'removed', 'old': {'type': 'string'}},
            {'path': ['components', 'schemas', 'TargetSchema', 'required'], 'kind': 'changed', 'new': ['name']},
            {'path': ['paths', '/v1/target', 'get', 'security'], 'kind': 'changed', 'new': []},
        ]
        for change in changes:
            with self.subTest(change=change), self.assertRaisesRegex(ValueError, 'Unclassified'):
                subject.disposition(change)

    def test_parameter_list_removal_cannot_masquerade_as_filter_addition(self):
        delta = next(x for x in self.outputs[subject.VENDOR / 'compatibility-decisions.json']['deltas'] if x['path'] == ['paths', '/v1/target', 'get', 'parameters'])
        changed = copy.deepcopy(delta)
        changed['new'] = [p for p in changed['new'] if p['name'] != 'limit']
        with self.assertRaisesRegex(ValueError, 'Unclassified'):
            subject.disposition(changed)

    def test_all_removed_components_have_real_successor(self):
        spec = subject.load(subject.VENDOR / 'openapi.json')
        decisions = self.outputs[subject.VENDOR / 'compatibility-decisions.json']['deltas']
        removed = [x for x in decisions if x['kind'] == 'removed' and len(x['path']) == 3]
        self.assertEqual(len(removed), 8)
        for delta in removed:
            self.assertEqual(delta['category'], 'component_removed_or_renamed')
            self.assertIn(delta['successorReference'].split('/')[-1], spec['components']['schemas'])

    def test_audit_pins_and_source_paths_are_checked(self):
        original = subject.Path.read_bytes
        audit_path = subject.ROOT / subject.AUDIT / 'inventory.json'
        def changed_read(path):
            data = original(path)
            return data + b' ' if path == audit_path else data
        with patch.object(subject.Path, 'read_bytes', changed_read):
            with self.assertRaisesRegex(ValueError, 'Pinned bytes changed'):
                subject.load(subject.AUDIT / 'inventory.json')
        for path in ('../outside.rs', '/etc/passwd'):
            with self.assertRaisesRegex(ValueError, 'Unsafe pin path'):
                subject.safe_path(path)

    def test_class_binding_does_not_match_other_impl_or_free_function(self):
        source = 'impl Other { pub fn get() {} } fn get() {} impl Wanted { fn other() { fn get() {} } }'
        self.assertFalse(subject.impl_contains(source, 'Wanted', 'get'))
        self.assertTrue(subject.impl_contains(source, 'Other', 'get'))
        self.assertTrue(subject.impl_contains('impl Wanted { pub fn get() { let x = "}"; } }', 'Wanted', 'get'))

    def test_retention_seed_records_six_audited_ledger_hashes(self):
        floor = subject.load(subject.RETENTION_SEED)
        self.assertEqual(set(floor['sourceLedgerSha256']), set(subject.RETENTION_LEDGER_PATHS))
        for path, digest in floor['sourceLedgerSha256'].items():
            self.assertRegex(digest, r'^[0-9a-f]{64}$')

    def test_semantic_type_and_enum_narrowing_fail_closed(self):
        changes = [
            {'path': ['components', 'schemas', 'TargetSchema', 'properties', 'name', 'type'], 'kind': 'changed', 'old': 'string', 'new': 'integer'},
            {'path': ['components', 'schemas', 'TargetConnectionType', 'enum'], 'kind': 'changed', 'old': ['REST', 'N8N'], 'new': ['REST']},
        ]
        for change in changes:
            with self.assertRaisesRegex(ValueError, 'Unclassified semantic'):
                subject.disposition(change)

    def test_retention_origin_checks_actual_bytes_and_rejects_wrong_digest(self):
        floor = subject.load(subject.RETENTION_SEED)
        # Exercise the checker independently of future additive ledger evolution.
        # Phase21's CLI flag separately verifies the actual immutable seed hashes.
        floor['sourceLedgerSha256'] = {path: subject.sha(subject.safe_path(path).read_bytes()) for path in subject.RETENTION_LEDGER_PATHS}
        subject.verify_retention_origin(floor)
        first = next(iter(floor['sourceLedgerSha256']))
        floor['sourceLedgerSha256'][first] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'Pinned bytes changed'):
            subject.verify_retention_origin(floor)

    def test_missing_management_inventory_fails_explicitly(self):
        original_load = subject.load
        def without_management(path):
            value = original_load(path)
            if subject.Path(path) == subject.AUDIT / 'inventory.json':
                value['specs'] = [row for row in value['specs'] if row['path'] != 'openapi-specs/prisma-airs-redteam/management/mp-openapi.yaml']
            return value
        with patch.object(subject, 'load', without_management):
            with self.assertRaisesRegex(ValueError, 'Management source row missing'):
                subject.build()

    def test_every_native_binding_is_type_qualified(self):
        rows = self.outputs[subject.Path('contracts/airs-operations.json')]['operations']
        self.assertTrue(all(binding.get('class') for row in rows for binding in row['nativeBindings']))
        with self.assertRaisesRegex(ValueError, 'Unqualified'):
            subject.verify_native_binding({'source': 'crates/scm-sdk/src/airs/model_security/scans.rs', 'method': 'get'})

    def test_added_and_changed_constraints_require_exact_review(self):
        for leaf, kind, old, new in [('enum', 'added', None, ['one']), ('default', 'changed', 110, 120), ('additionalProperties', 'changed', True, False), ('pattern', 'added', None, '^x$'), ('items', 'changed', {'type': 'string'}, {'type': 'number'})]:
            change = {'path': ['components', 'schemas', 'TargetSchema', 'properties', 'name', leaf], 'kind': kind, 'new': new}
            if kind != 'added':
                change['old'] = old
            with self.subTest(leaf=leaf), self.assertRaisesRegex(ValueError, 'Unclassified.*SHA256'):
                subject.disposition(change)

    def test_source_metadata_every_field_is_verified(self):
        spec = subject.load(subject.VENDOR / 'openapi.json')
        pin = subject.load(subject.VENDOR / 'source.json')
        source = next(row for row in subject.load('contracts/manifest.json')['sources'] if row['id'] == subject.SOURCE_ID)
        subject.verify_management_metadata(pin, source, spec)
        for field in pin:
            damaged = copy.deepcopy(pin)
            damaged[field] = None
            with self.subTest(field=field), self.assertRaisesRegex(ValueError, 'metadata mismatch'):
                subject.verify_management_metadata(damaged, source, spec)

    def test_all_seven_source_identities_resolve(self):
        sources = {row['id']: row for row in subject.load('contracts/manifest.json')['sources']}
        rows = self.outputs[subject.Path('contracts/airs-active-sources.json')]['sources']
        for row in rows:
            actual = subject.resolve_active_identity(row, sources)
            self.assertEqual(actual['path'], row['activeSpecificationPath'])
        dp = next(row for row in rows if row['identityKind'] == 'source_metadata')
        self.assertTrue(dp['activeSourceId'].endswith('/source.json'))
        broken = copy.deepcopy(dp)
        broken['sourceMetadataSha256'] = '0' * 64
        with self.assertRaisesRegex(ValueError, 'Pinned bytes changed'):
            subject.resolve_active_identity(broken, sources)

    def test_single_pass_tokenizer_distinguishes_comments_strings_and_code(self):
        source = '// don"t {{ fn get()\nimpl Wanted { pub fn get() { let value = "// /* }"; } }'
        self.assertTrue(subject.impl_contains(source, 'Wanted', 'get'))
        self.assertFalse(subject.impl_contains('// don"t\nimpl Wanted { fn outer() { fn get() {} } }', 'Wanted', 'get'))
        self.assertTrue(subject.impl_contains('/* nested /* " */ } */ impl Wanted { pub fn get() { let v = r###"} // "###; } }', 'Wanted', 'get'))
        self.assertFalse(subject.impl_contains('impl Wanted { /* fn get() {} */ fn other() {} }', 'Wanted', 'get'))

    def test_retention_output_tracks_current_additions(self):
        original_load = subject.load
        def additional_export(path):
            value = original_load(path)
            if str(path) == 'contracts/native-exports.json':
                value['capabilities'].append({'baselineId': 'NewTestExport', 'rustPaths': ['scm_sdk::NewTestExport']})
            return value
        with patch.object(subject, 'load', additional_export):
            changed = subject.build()[subject.Path('contracts/airs-retained-capabilities.json')]
        self.assertEqual(len(changed['exports']), 3821)
        self.assertNotEqual(subject.encoded(changed), subject.encoded(self.outputs[subject.Path('contracts/airs-retained-capabilities.json')]))

    def test_whole_new_subtrees_and_unknown_prefixed_components_fail_closed(self):
        for path, value in [(['components', 'schemas', 'RestConnectionParamsFuture'], {'type': 'object'}), (['components', 'schemas', 'TargetSchema', 'properties', 'new_field'], {'type': 'string', 'default': 'secret'})]:
            with self.assertRaisesRegex(ValueError, 'Unclassified'):
                subject.disposition({'path': path, 'kind': 'added', 'new': value})

    def test_successor_content_tampering_rejected(self):
        delta = next(x for x in self.outputs[subject.VENDOR / 'compatibility-decisions.json']['deltas'] if x['path'] == ['components', 'schemas', 'NativeConnectionParamsBase-Input'])
        for mutation in ('drop', 'required', 'reorder'):
            spec = subject.load(subject.VENDOR / 'openapi.json')
            successor = spec['components']['schemas']['NativeConnectionParamsBase']
            if mutation == 'drop':
                successor['properties'].pop('multi_turn_config')
            elif mutation == 'required':
                successor['required'] = ['multi_turn_config']
            else:
                successor['properties']['target_connection_config']['anyOf'].reverse()
            with self.subTest(mutation=mutation), self.assertRaisesRegex(ValueError, 'Unreviewed component successor'):
                subject.verify_component_successor(delta, spec)

    def test_duplicate_audit_operation_rejected(self):
        original = subject.load
        def duplicate(path):
            value = original(path)
            if subject.Path(path) == subject.AUDIT / 'red-team-operations.json':
                value['operations'].append(copy.deepcopy(value['operations'][0]))
            return value
        with patch.object(subject, 'load', duplicate):
            with self.assertRaisesRegex(ValueError, 'Duplicate native binding'):
                subject.bindings()

    def test_private_and_crate_visible_functions_are_not_public_bindings(self):
        for visibility in ('', 'pub(crate) ', 'pub(super) '):
            self.assertFalse(subject.impl_contains('impl Wanted { ' + visibility + 'fn get() {} }', 'Wanted', 'get'))
        self.assertTrue(subject.impl_contains('impl Wanted { pub async fn get() {} }', 'Wanted', 'get'))

    def test_stale_review_registry_entries_are_rejected(self):
        with patch.dict(subject.REVIEWED_DELTAS, {('obsolete', 'delta'): '0' * 64}):
            with self.assertRaisesRegex(ValueError, 'Unused reviewed delta'):
                subject.build()
        with patch.dict(subject.REVIEWED_SUCCESSORS, {'ObsoleteComponent': '0' * 64}):
            with self.assertRaisesRegex(ValueError, 'Unused reviewed successor'):
                subject.build()

    def test_union_prefix_ignores_only_trailing_null(self):
        delta = next(x for x in self.outputs[subject.VENDOR / 'compatibility-decisions.json']['deltas'] if x['path'] == ['components', 'schemas', 'NativeConnectionParamsBase-Input'])
        spec = subject.load(subject.VENDOR / 'openapi.json')
        successor = spec['components']['schemas']['NativeConnectionParamsBase']
        branches = successor['properties']['target_connection_config']['anyOf']
        self.assertEqual(branches[-1], {'type': 'null'})
        branches.insert(1, branches.pop())
        # Independently exercise the prefix check after a hypothetical digest review.
        with patch.dict(subject.REVIEWED_SUCCESSORS, {'NativeConnectionParamsBase-Input': subject.sha(subject.encoded(successor).encode())}):
            with self.assertRaisesRegex(ValueError, 'reorders a legacy union prefix'):
                subject.verify_component_successor(delta, spec)

    def test_missing_binding_rejected(self):
        original = subject.bindings()
        original.pop(next(iter(original)))
        with patch.object(subject, 'bindings', return_value=original):
            with self.assertRaisesRegex(ValueError, 'Unmapped operation'):
                subject.build()

    def test_retention_removals_and_changed_defaults_cannot_be_regenerated_away(self):
        original = self.outputs[subject.Path('contracts/airs-retained-capabilities.json')]
        for section in ('exports', 'methods', 'redTeamExtraMethods', 'defaultBindings',
                        'postBaselineRedTeamOperations', 'postBaselineRedTeamModelAdditions',
                        'postBaselinePortkeyOperations', 'modelSecurityExtraQueries', 'additionalTargetTemplates'):
            current = copy.deepcopy(original)
            current[section].pop()
            with self.subTest(section=section), self.assertRaisesRegex(ValueError, 'Retained'):
                subject.retained_floor(current)
        current = copy.deepcopy(original)
        current['exports'][0]['rustPaths'] = []
        with self.assertRaisesRegex(ValueError, 'Retained'):
            subject.retained_floor(current)
        current = copy.deepcopy(original)
        current['defaultBindings'][0]['default'] = 'changed'
        with self.assertRaisesRegex(ValueError, 'Retained'):
            subject.retained_floor(current)

    def test_retention_seed_tampering_is_rejected(self):
        original_read = subject.Path.read_bytes
        seed_path = subject.ROOT / subject.RETENTION_SEED
        def changed_read(path):
            data = original_read(path)
            return data + b' ' if path == seed_path else data
        with patch.object(subject.Path, 'read_bytes', changed_read):
            with self.assertRaisesRegex(ValueError, 'Pinned bytes changed'):
                subject.retained_floor(self.outputs[subject.Path('contracts/airs-retained-capabilities.json')])

    def test_retention_allows_additions_without_rewriting_floor(self):
        current = copy.deepcopy(self.outputs[subject.Path('contracts/airs-retained-capabilities.json')])
        current['exports'].append({'baselineId': 'NewExport', 'rustPaths': ['scm_sdk::NewExport']})
        current['methods'][0]['rustMethods'].append('scm_sdk::new_alias')
        result = subject.retained_floor(current)
        self.assertEqual(len(result['exports']), 3821)
        self.assertIn('scm_sdk::new_alias', result['methods'][0]['rustMethods'])
        self.assertEqual(result['floorReference']['sha256'], subject.RETENTION_SHA256)
        self.assertNotIn('sourceLedgerSha256', result)

    def test_future_phase_cannot_pass_on_plan_artifacts(self):
        for phase in range(22, 26):
            result = subprocess.run([sys.executable, str(subject.ROOT / 'tools/red_team_management.py'), '--check', '--require-through-phase', str(phase)], capture_output=True, text=True)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn('static plan evidence cannot advance', result.stderr)

    def test_all_five_operation_specific_target_response_declarations(self):
        rows = self.outputs[subject.VENDOR / 'operations.json']['operations']
        actual = {(x['method'], x['path']): next((response.get('content', {}).get('application/json', {}).get('schema', {}).get('$ref') for code, response in x['responses'].items() if code.startswith('2')), None) for x in rows}
        expected = {('POST', '/v1/target'): 'TargetResponseSchema', ('GET', '/v1/target/{target_uuid}'): 'TargetRedactSchema', ('PUT', '/v1/target/{target_uuid}'): 'TargetSchema', ('POST', '/v1/target/probe'): 'TargetResponseSchema', ('PUT', '/v1/target/{target_uuid}/profile'): 'TargetSchema'}
        for key, name in expected.items():
            self.assertEqual(actual[key], '#/components/schemas/' + name)


if __name__ == '__main__':
    unittest.main()
