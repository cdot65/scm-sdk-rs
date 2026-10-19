"""Independent structural checks for the additive management overlay."""
import copy
import json
import unittest
from unittest import mock

import generate_models as models
import red_team_management_models as management


class ManagementModelTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.overlay = management.Overlay()
        cls.inventory = json.loads(models.SOURCE.read_text())

    def test_runtime_changes_are_only_reviewed_nodes_and_two_unions(self):
        graph = models.compact_graph(self.inventory)
        changed = self.overlay.extend_graph(graph)
        touched = {sid for sid in graph['nodes'] if graph['nodes'][sid] != changed['nodes'][sid]}
        self.assertEqual(touched, {'s012061', 's012067', 's010462', 's010504'})
        self.assertFalse(any(node['kind'] == 'ZodDefault' for node in self.overlay.nodes.values()))
        for sid in management.UNION_BRANCHES:
            self.assertEqual(changed['nodes'][sid]['variants'][:-1], graph['nodes'][sid]['variants'])

    def test_projection_fields_do_not_narrow_runtime_validation(self):
        graph = self.overlay.extend_graph(models.compact_graph(self.inventory))
        for projection in self.overlay.projections:
            self.assertNotIn(projection['field'], graph['nodes'][projection['legacySchema']]['properties'])
        gen = models.Generator(self.inventory)
        _, standalone = self.overlay.model_outputs(models.Generator)
        self.overlay.augment_models(gen, standalone)
        for projection in self.overlay.projections:
            field = gen.nodes[projection['legacySchema']]['properties'][projection['field']]
            self.assertTrue(field['vendorProjection'])
            self.assertEqual(gen.nodes[field['schema']['$ref']]['kind'], 'ZodOptional')

    def test_source_fragment_unknown_keyword_or_pattern_fails_closed(self):
        for mutation in ('pattern', 'keyword'):
            spec = copy.deepcopy(self.overlay.spec)
            field = spec['components']['schemas']['VertexAIConnectionParams']['properties']['project_id']
            if mutation == 'pattern':
                field['pattern'] = '.*'
            else:
                field['maxLength'] = 4
            with self.assertRaisesRegex(ValueError, 'Unreviewed'):
                management.Overlay(spec)

    def test_baseline_identity_and_projection_completeness_fail_closed(self):
        baseline = models.compact_graph(self.inventory)['nodes']
        for sid in ('s010721', 's010462'):
            damaged = copy.deepcopy(baseline)
            damaged[sid] = damaged['s010504']
            with self.assertRaisesRegex(ValueError, 'identity'):
                self.overlay.extend_nodes(damaged)
        missing = dict(management.PROJECTIONS)
        missing.pop('s010755')
        with self.assertRaisesRegex(ValueError, 'coverage'):
            management.check_projection_completeness(baseline, missing)
        extra = copy.deepcopy(baseline)
        extra['new_node'] = copy.deepcopy(extra['s010721'])
        with self.assertRaisesRegex(ValueError, 'coverage'):
            management.check_projection_completeness(extra, management.PROJECTIONS)
        gen = models.Generator(self.inventory)
        gen.names.update(management.BASELINE_NAMES)
        self.overlay.verify_model_names(gen)
        gen.names['s010755'] = 'WrongModel'
        with self.assertRaisesRegex(ValueError, 'identity'):
            self.overlay.verify_model_names(gen)

    def test_added_defaults_require_explicit_review(self):
        for component, field in [('VertexAIConnectionParams', 'project_id'),
                                 ('BasicAuthAuthConfig', 'basic_password'),
                                 ('RestConnectionParamsBase', 'audio_curl')]:
            with self.subTest(component=component, field=field):
                spec = copy.deepcopy(self.overlay.spec)
                spec['components']['schemas'][component]['properties'][field]['default'] = 'new-default'
                # Keep counterpart parity to exercise the default guard itself.
                masked = management.PROJECTION_COUNTERPARTS.get(component)
                if masked:
                    spec['components']['schemas'][masked]['properties'][field]['default'] = 'new-default'
                with self.assertRaisesRegex(ValueError, 'Unreviewed management default'):
                    management.Overlay(spec)

    def test_vendor_projection_discovery_includes_masked_and_unknown_components(self):
        for plain, masked in management.PROJECTION_COUNTERPARTS.items():
            for field in management.PROJECTED_KEYS & self.overlay.spec['components']['schemas'][masked]['properties'].keys():
                for mutation in ('field', 'required'):
                    with self.subTest(masked=masked, field=field, mutation=mutation):
                        spec = copy.deepcopy(self.overlay.spec)
                        schema = spec['components']['schemas'][masked]
                        if mutation == 'field':
                            schema['properties'][field] = {'type': 'integer'}
                        else:
                            schema['required'] = [*schema.get('required', []), field]
                        with self.assertRaisesRegex(ValueError, 'divergent masked projection'):
                            management.Overlay(spec)
        for field in management.PROJECTED_KEYS:
            spec = copy.deepcopy(self.overlay.spec)
            spec['components']['schemas']['NewAuthVariant'] = {
                'type': 'object', 'properties': {field: {'type': 'string'}}}
            with self.assertRaisesRegex(ValueError, 'component/field coverage'):
                management.Overlay(spec)
        spec = copy.deepcopy(self.overlay.spec)
        del spec['components']['schemas']['BasicAuthAuthConfigRedact']['properties']['basic_password']
        with self.assertRaisesRegex(ValueError, 'component/field coverage'):
            management.Overlay(spec)

    def test_enum_inventory_is_derived_from_applied_values(self):
        spec = copy.deepcopy(self.overlay.spec)
        spec['components']['schemas']['ResponseMode']['enum'].extend(['FUTURE_MODE', 'FUTURE_MODE'])
        overlay = management.Overlay(spec)
        changed = overlay.extend_graph(models.compact_graph(self.inventory))
        before = management.baseline_nodes()['s012067']['values']
        self.assertEqual(overlay.ledger()['runtimeEnumExtensions']['s012067'],
            [v for v in changed['nodes']['s012067']['values'] if v not in before])
        self.assertEqual(overlay.ledger()['runtimeEnumExtensions']['s012067'], ['REALTIME', 'FUTURE_MODE'])

    def test_production_source_rejects_added_vendor_enum_value(self):
        source = (management.VENDOR / 'mp-openapi.yaml').read_bytes()
        self.assertIn(b'- REALTIME', source)
        changed = source.replace(b'- REALTIME', b'- REALTIME\n      - FUTURE_MODE', 1)
        with mock.patch.object(management.Path, 'read_bytes', return_value=changed):
            with self.assertRaisesRegex(ValueError, 'reviewed pin'):
                management.load_spec()

    def test_generated_rewrite_requires_exactly_one_match(self):
        for source in ('missing', 'match match'):
            with self.assertRaisesRegex(ValueError, 'identity'):
                management.replace_once(source, 'match', 'new')
        self.assertEqual(management.replace_once('a match b', 'match', 'new'), 'a new b')

    def test_default_is_not_injected_into_client_session(self):
        sid = self.overlay.roots['MultiTurnClientSessionConfig']
        field = self.overlay.nodes[sid]['properties']['type']['schema']['$ref']
        self.assertEqual(self.overlay.nodes[field], {'kind': 'ZodLiteral', 'value': 'client_session'})

    def test_union_extension_verifier_preserves_exact_legacy_prefix(self):
        baseline = json.loads(models.UNION_PINS.read_text())
        actual = copy.deepcopy(baseline)
        expected = {row['schema']: row for row in self.overlay.extension_ledger()['extensions']}
        for row in actual['unions']:
            if row.get('schema') in expected:
                extension = expected[row['schema']]
                row['branches'].append({'sourceIndex': extension['legacyBranches'] + 1,
                    'schema': extension['appendedSchema'], 'rustVariant': extension['rustVariant'], 'rustType': extension['rustType']})
        self.overlay.verify_union_pins(actual, baseline, models.verify_union_pins)
        for mutation in ('prefix', 'tail', 'missing'):
            damaged = copy.deepcopy(actual)
            row = next(row for row in damaged['unions'] if row.get('schema') == 's010462')
            if mutation == 'prefix':
                row['branches'][0], row['branches'][1] = row['branches'][1], row['branches'][0]
            elif mutation == 'tail':
                row['branches'][-1]['rustType'] = 'OtherType'
            else:
                row['branches'].pop()
            with self.assertRaises(ValueError):
                self.overlay.verify_union_pins(damaged, baseline, models.verify_union_pins)


if __name__ == '__main__':
    unittest.main()
