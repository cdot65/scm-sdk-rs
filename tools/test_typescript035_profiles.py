"""Focused mutation and ownership checks for the additive profile overlay."""
import copy
import json
import re
import unittest
from unittest import mock

import generate_models as models
import typescript035_profiles as profiles


def directional_call_bindings(text):
    """Audit direct calls in these four rustfmt-formatted fixture functions.

    This deliberately is not a Rust parser or an execution-coverage claim.
    Comments/string literals cannot satisfy a call; receiver refactors require
    updating this narrow audit alongside the operation ledger.
    """
    methods = {'create': 'create', 'update': 'update', 'get': 'get',
               'list_for_token': 'listForToken', 'list': 'list', 'list_all': 'listAll'}
    result = {method: set() for method in methods.values()}
    # Current fixture functions use ordinary quoted strings, not raw strings.
    code = re.sub(r'"(?:\\.|[^"\\])*"|//[^\n]*|/\*.*?\*/', ' ', text, flags=re.S)
    for name in profiles.WIRE_TESTS:
        bodies = re.findall(r'^async fn ' + re.escape(name) + r'\(\) \{\n(.*?)^\}',
                            code, flags=re.M | re.S)
        if len(bodies) != 1:
            raise AssertionError('expected one directional test body: ' + name)
        calls = re.findall(r'\b(?:client|management\s*\.\s*profiles\s*\(\s*\))'
                           r'\s*\.\s*(' + '|'.join(methods) + r')\s*\(', bodies[0])
        for method in calls:
            result[methods[method]].add(name)
    return result


class ProfileOverlayTests(unittest.TestCase):
    def test_unmatched_upstream_protection_key_has_explicit_diagnostic(self):
        declarations = {
            'ContentTypeConfigurationSchema': ('data-protection', 'future-protection'),
            'ContentTypeConfigurationsSchema': ('prompt', 'response', 'tool-call', 'tool-response'),
        }
        with mock.patch.object(profiles, 'verify_new_profile_declarations', return_value=declarations):
            with self.assertRaisesRegex(ValueError, 'protection keys absent from frozen ModelConfiguration: future-protection'):
                profiles.Overlay()

    def test_oracle_scope_and_directional_operation_bindings_are_explicit(self):
        scope = self.overlay.ledger()['compatibility']['oracleScope']
        self.assertIn('all eight roots in Mode::Request', scope)
        self.assertIn('Mode::Response and Mode::ResponseWithoutDefaults', scope)
        self.assertIn('six versioned roots do not claim Zod response equivalence', scope)
        mappings = json.loads((profiles.ROOT / 'contracts/native-operation-mappings/management.json').read_text())
        text = (profiles.ROOT / 'crates/scm-sdk/tests/management_contracts.rs').read_text()
        expected = directional_call_bindings(text)
        for method, tests in expected.items():
            matches = [row for row in mappings['operations']
                       if row['baselineId'] == 'src/management/profiles.ts:ProfilesClient.' + method]
            self.assertEqual(len(matches), 1)
            self.assertEqual(set(matches[0]['tests']), {
                'crates/scm-sdk/tests/management_contracts.rs::' + name for name in (
                    'profile_methods_match_literal_routes_schemas_defaults_and_helpers', *tests)
            })

    def test_directional_attribution_tracks_removed_calls_not_comments_or_strings(self):
        text = (profiles.ROOT / 'crates/scm-sdk/tests/management_contracts.rs').read_text()
        original = directional_call_bindings(text)
        rust_methods = {'create': 'create', 'update': 'update', 'get': 'get',
                        'list_for_token': 'listForToken', 'list': 'list', 'list_all': 'listAll'}
        for rust_method, method in rust_methods.items():
            with self.subTest(method=method):
                self.assertTrue(original[method])
                changed = re.sub(r'\.\s*' + rust_method + r'\s*\(', '.removed_call(', text)
                # Apparent calls in comments and ordinary strings are not calls.
                fake = f'// client.{rust_method}();\n        let fake = "client.{rust_method}()";\n'
                for name in profiles.WIRE_TESTS:
                    start = 'async fn ' + name + '() {\n'
                    changed = changed.replace(start, start + fake)
                derived = directional_call_bindings(changed)
                self.assertEqual(derived[method], set())
                for other in original.keys() - {method}:
                    self.assertEqual(derived[other], original[other])

    def test_wire_qualification_names_and_binding_are_explicit(self):
        path = 'crates/scm-sdk/tests/management_contracts.rs'
        text = (profiles.ROOT / path).read_text()
        profiles.verify_wire_tests(text)
        for name in profiles.WIRE_TESTS:
            for changed in (text.replace('async fn ' + name + '(', 'async fn removed_' + name + '('),
                            text + '\n#[tokio::test]\nasync fn ' + name + '() {}\n'):
                with self.subTest(name=name), self.assertRaisesRegex(ValueError, 'wire test'):
                    profiles.verify_wire_tests(changed)
        ledger = self.overlay.ledger()
        self.assertEqual(ledger['sourceBindings'][path], profiles.sha(text.encode()))
        self.assertEqual(ledger['wireTests'], list(profiles.WIRE_TESTS))

    def test_versioned_format_inventory_is_separate_and_complete(self):
        graph = self.overlay.extend_graph(self.graph)
        inventory = self.overlay.format_inventory()
        self.assertEqual(inventory['roots'], 6)
        self.assertEqual(inventory['rootsWithFormats'], 1)
        self.assertEqual(inventory['formatSites'], 1)
        for row in inventory['entries']:
            pending = [graph['roots'][row['root']]]
            seen, sites = set(), []
            while pending:
                sid = pending.pop()
                if sid in seen:
                    continue
                seen.add(sid)
                node = graph['nodes'][sid]
                sites.extend({'node': sid, 'kind': check['kind']} for check in node.get('checks', [])
                             if check['kind'] in profiles.FORMAT_CHECKS)
                pending.extend(profiles.references(node))
            self.assertEqual(row['formatSites'], sorted(sites, key=lambda item: (item['node'], item['kind'])))

    @classmethod
    def setUpClass(cls):
        cls.overlay = profiles.Overlay()
        cls.inventory = json.loads(models.SOURCE.read_text())
        cls.graph = models.compact_graph(cls.inventory)

    def test_pinned_source_failure_stops_construction(self):
        with mock.patch.object(profiles, 'verify', side_effect=ValueError('source changed')):
            with self.assertRaisesRegex(ValueError, 'source changed'):
                profiles.Overlay()

    def test_unreviewed_ancestor_or_root_is_rejected(self):
        with mock.patch.object(profiles, 'CLONES', profiles.CLONES[:-1]):
            with self.assertRaisesRegex(ValueError, 'ancestor closure'):
                profiles.Overlay()
        with mock.patch.dict(profiles.CURRENT_ROOTS, {'PolicySchema': 's009612'}):
            with self.assertRaisesRegex(ValueError, 'root identity'):
                profiles.Overlay()

    def test_graph_is_deterministic_and_does_not_mutate_input(self):
        original = copy.deepcopy(self.graph)
        first = self.overlay.extend_graph(self.graph)
        second = self.overlay.extend_graph(self.graph)
        self.assertEqual(first, second)
        self.assertEqual(self.graph, original)
        self.assertEqual(len(first['nodes']) - len(original['nodes']), 16)
        self.assertEqual(len(first['roots']) - len(original['roots']), 8)
        for key in ('nodes', 'roots'):
            self.assertEqual({sid: first[key][sid] for sid in original[key]}, original[key])
        # Reapplying an additive overlay is misuse, not permission to overwrite.
        with self.assertRaisesRegex(ValueError, 'collision'):
            self.overlay.extend_graph(first)
        self.assertEqual(first, second)

    def test_graph_node_root_and_clone_collisions_leave_input_unchanged(self):
        for kind, key in [('nodes', 's_ts035_content_type_configuration'),
                          ('nodes', 's_ts035_s009653'),
                          ('roots', 'ContentTypeConfigurationsSchema'),
                          ('roots', 'typescript:0.35.0:PolicySchema')]:
            with self.subTest(kind=kind, key=key):
                graph = copy.deepcopy(self.graph)
                graph[kind][key] = {'kind': 'ZodString'} if kind == 'nodes' else 's009612'
                original = copy.deepcopy(graph)
                with self.assertRaisesRegex(ValueError, 'collision'):
                    self.overlay.extend_graph(graph)
                self.assertEqual(graph, original)

    def test_changed_historical_profile_nodes_or_roots_fail_closed(self):
        for sid in profiles.CLONES:
            with self.subTest(sid=sid):
                graph = copy.deepcopy(self.graph)
                graph['nodes'][sid]['kind'] = 'ZodAny'
                with self.assertRaisesRegex(ValueError, 'node identity'):
                    self.overlay.extend_graph(graph)
        graph = copy.deepcopy(self.graph)
        graph['roots']['PolicySchema'] = 's009612'
        with self.assertRaisesRegex(ValueError, 'root identity'):
            self.overlay.extend_graph(graph)

    def test_exact_five_optional_projections_and_frozen_model_rows(self):
        gen = models.Generator(self.inventory)
        original_nodes = copy.deepcopy(gen.nodes)
        rows = copy.deepcopy(gen.schema_models)
        inventory = copy.deepcopy(gen.inventory)
        self.overlay.augment_models(gen)
        self.assertEqual(gen.inventory, inventory)
        self.assertEqual(gen.schema_models, rows)
        projected = {(sid, field) for sid, node in gen.nodes.items()
                     for field, prop in node.get('properties', {}).items() if prop.get('vendorProjection')}
        expected = {('s009588', 'enable-full-conversation-inspection'),
                    ('s009599', 'content-type-mode'), ('s009599', 'content-type-configurations'),
                    ('s009612', 'dlp_tenant_id'), ('s009632', 'dlp_tenant_id')}
        self.assertEqual(projected, expected)
        for sid, field in projected:
            ref = gen.nodes[sid]['properties'][field]['schema']['$ref']
            self.assertEqual(gen.nodes[ref]['kind'], 'ZodOptional')
        for sid, node in original_nodes.items():
            actual = copy.deepcopy(gen.nodes[sid])
            for owner, field in expected:
                if owner == sid:
                    actual['properties'].pop(field)
            self.assertEqual(actual, node)

    def test_model_collisions_are_atomic(self):
        for collision in ('node', 'name', 'declaration', 'alias', 'baseline'):
            with self.subTest(collision=collision):
                gen = models.Generator(self.inventory)
                if collision == 'node':
                    gen.nodes['s_ts035_configurations_optional'] = {'kind': 'ZodString'}
                elif collision == 'name':
                    gen.used.add('ContentTypeConfigurations')
                elif collision == 'declaration':
                    gen.declarations['ContentTypeConfiguration'] = 'existing'
                elif collision == 'alias':
                    gen.aliases['ContentTypeConfigurations'] = 'existing'
                else:
                    gen.nodes['s009588'] = {'kind': 'ZodString'}
                before = copy.deepcopy(gen.__dict__)
                with self.assertRaisesRegex(ValueError, 'identity'):
                    self.overlay.augment_models(gen)
                self.assertEqual(gen.__dict__, before)

    def test_directional_fields_reuse_protections_and_preserve_unknown_keys(self):
        configuration = self.overlay.nodes['s_ts035_content_type_configuration']
        self.assertEqual(configuration['unknownFields'], 'passthrough')
        self.assertEqual(set(configuration['properties']),
                         {'data-protection', 'app-protection', 'model-protection', 'agent-protection'})
        for name, value in configuration['properties'].items():
            self.assertEqual(value, self.inventory['schemas']['definitions']['s009588']['properties'][name])
        directions = self.overlay.nodes['s_ts035_content_type_configurations']
        self.assertEqual(directions['unknownFields'], 'passthrough')
        for node in (configuration, directions):
            self.assertEqual(node['catchall'], {'$ref': 's009589'})
            self.assertEqual(node['catchall'], self.inventory['schemas']['definitions']['s009588']['catchall'])
            self.assertEqual(self.inventory['schemas']['definitions']['s009589']['kind'], 'ZodNever')
        for node in (configuration, directions):
            for prop in node['properties'].values():
                self.assertEqual(prop['inputPresence'], 'accepts')
                self.assertEqual(prop['inputNull'], 'rejects')
        self.assertEqual(set(directions['properties']), {'prompt', 'response', 'tool-call', 'tool-response'})
        self.assertTrue(all(p['schema']['$ref'] == 's_ts035_configuration_optional'
                            for p in directions['properties'].values()))
        self.assertFalse(any(p.get('vendorProjection') for node in self.overlay.nodes.values()
                             for p in node.get('properties', {}).values()))

    def test_only_cloned_list_default_needs_semantics_alias(self):
        graph = self.overlay.extend_graph(self.graph)
        new_defaults = {sid for sid, node in graph['nodes'].items()
                        if sid not in self.graph['nodes'] and node['kind'] == 'ZodDefault'}
        self.assertEqual(new_defaults, {'s_ts035_s009653'})
        aliases = profiles.default_aliases()
        self.assertEqual(aliases, {'s_ts035_s009653': 's009653'})
        aliases.clear()
        self.assertEqual(profiles.default_aliases(), {'s_ts035_s009653': 's009653'})
        default = graph['nodes']['s_ts035_s009653']
        self.assertTrue(default['requiresReviewedSemantics'])
        self.assertEqual(default['inner'], {'$ref': 's_ts035_s009654'})
        self.assertEqual(graph['nodes']['s_ts035_s009654']['element'], {'$ref': 's_ts035_s009612'})
        self.assertEqual(graph['nodes']['s_ts035_s009651']['properties']['ai_profiles']['schema'],
                         {'$ref': 's_ts035_s009653'})

    def test_supplemental_counts_do_not_claim_historical_export_scope(self):
        ledger = self.overlay.ledger()
        self.assertEqual(ledger['modelExportDelta'], self.overlay.export_delta)
        self.assertEqual(ledger['counts']['addedExports'], len(ledger['modelExportDelta']['added']))
        self.assertEqual(ledger['modelExportDelta']['removed'], [])
        self.assertEqual(ledger['counts'], {'addedExports': 4, 'addedModels': 2, 'addedSchemas': 2,
                                          'projectedFields': 5, 'versionedRoots': 6,
                                          'clonedNodes': 12, 'defaultAliases': 1})
        self.assertEqual({row['export'] for row in ledger['schemas']},
                         {'ContentTypeConfigurationSchema', 'ContentTypeConfigurationsSchema'})
        self.assertEqual({row['export'] for row in ledger['modelExports']},
                         {'ContentTypeConfiguration', 'ContentTypeConfigurations'})
        self.assertEqual(len(ledger['versionedRoots']), 6)
        self.assertTrue({
            'crates/scm-sdk/src/airs/management/profiles.rs',
            'crates/scm-sdk/tests/typescript035_models.rs',
            'crates/scm-sdk/examples/directional_profiles.rs',
        } <= ledger['sourceBindings'].keys())


if __name__ == '__main__':
    unittest.main()
