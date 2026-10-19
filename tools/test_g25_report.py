"""Regression checks for the G25 comparison report's deliberately local semantics."""
import copy
import pathlib
import json
import hashlib
import re
import unittest

from g25_report import grouped, ledger, residual_strictness, signature, validate_corpus_binding, projection_usage, render, management_projection_section


class ComparisonReportTests(unittest.TestCase):
    def test_corpus_digest_is_required_and_matches_exact_bytes(self):
        data = b'{"cases": []}\n'
        good = hashlib.sha256(data).hexdigest()
        validate_corpus_binding(data, {'casesSha256': good})
        for oracle in ({}, {'casesSha256': None}, {'casesSha256': '0' * 64}):
            with self.subTest(oracle=oracle), self.assertRaisesRegex(ValueError, 'digest mismatch'):
                validate_corpus_binding(data, oracle)
        with self.assertRaisesRegex(ValueError, 'digest mismatch'):
            validate_corpus_binding(data.rstrip(), {'casesSha256': good})

    def field(self, path='$', **changes):
        return {'path': path, 'node': 'one', 'kind': 'ZodString',
                'optional': False, 'nullable': False, **changes}

    def operation(self, **changes):
        return {'operationId': 'op', 'nativeClass': 'Client', 'nativeMethod': 'get',
                'responseRoot': 'response', 'baselineOperation': 'baseline',
                'baselineExpression': 'Schema', 'baselineOpaque': False,
                'vendorFields': [self.field()], 'baselineFields': [self.field()], **changes}

    def test_grouping_keeps_original_union_descriptors(self):
        fields = [self.field('$.value|0'), self.field('$.value|1', kind='ZodNull')]
        original = copy.deepcopy(fields)
        self.assertEqual(grouped(fields), {'$.value': fields})
        self.assertEqual(fields, original)

    def test_presence_nullability_and_bounds_affect_description(self):
        field = self.field()
        for change in ({'optional': True}, {'nullable': True}, {'kind': 'ZodNumber'},
                       {'checks': [{'kind': 'min', 'value': 2}]}):
            with self.subTest(change=change):
                self.assertNotEqual(signature([field]), signature([field | change]))
        self.assertEqual(signature([field]), signature([field | {'node': 'different'}]))

    def test_classification_keeps_opaque_and_new_operations_distinct(self):
        ops = [self.operation(), self.operation(baselineOpaque=True, baselineFields=[]),
               self.operation(baselineOperation=None, baselineFields=[]),
               self.operation(baselineFields=[]), self.operation(vendorFields=[])]
        result = ledger({'operations': ops})
        self.assertEqual([op['fields'][0]['classification'] for op in result['operations']], [
            'same-local-description', 'typescript-opaque-contract',
            'no-typescript-counterpart', 'vendor-only-path', 'typescript-only-path'])

    def test_changed_local_description_retains_both_full_records(self):
        op = self.operation(vendorFields=[self.field(nullable=True)])
        row = ledger({'operations': [op]})['operations'][0]['fields'][0]
        self.assertEqual(row['classification'], 'different-local-description')
        self.assertEqual(row['vendor'], op['vendorFields'])
        self.assertEqual(row['typescript'], op['baselineFields'])

    def test_residual_strictness_counts_mutated_outcomes_and_preserves_case_identity(self):
        corpus = {'roots': [{'root': 'first'}, {'root': 'second'}], 'cases': [
            {'root': 0, 'path': ['value'], 'kind': 'type:number'},
            {'root': 0, 'path': ['value'], 'kind': 'type:null'},
            {'root': 1, 'path': ['id'], 'kind': 'format:uuid'},
            {'root': 1, 'path': [], 'kind': 'base'}]}
        oracle = {'outcomes': [
            {'strict': True, 'typescript': False, 'response': False},
            {'strict': True, 'typescript': False, 'response': False},
            {'strict': False, 'typescript': True, 'response': True},
            {'strict': True, 'typescript': None, 'response': False}]}
        result = residual_strictness(corpus, oracle)
        self.assertEqual(result['caseCount'], 2)
        self.assertEqual(result['operationCount'], 1)
        self.assertEqual(result['mutationCounts'], {'type:null': 1, 'type:number': 1})
        self.assertEqual([row['caseIndex'] for row in result['cases']], [0, 1])
        with self.assertRaises(ValueError):
            residual_strictness(corpus, {'outcomes': oracle['outcomes'][:-1]})

    def test_report_discloses_retained_timestamp_refinement(self):
        report = (pathlib.Path(__file__).resolve().parents[1] / 'docs/G25_COMPATIBILITY.md').read_text()
        for text in ('Frozen refinements and transforms remain active', '`time_order`',
                     '`start_time`', '`end_time`', 'can reject unparseable timestamps',
                     'does not bypass a separate semantic refinement'):
            with self.subTest(text=text):
                self.assertIn(text, report)

    def test_report_request_models_follow_projection_reachability(self):
        root = pathlib.Path(__file__).resolve().parents[1]
        projections = json.loads((root / 'contracts/compatibility/g25/projections.json').read_text())
        audit = json.loads((root / 'contracts/compatibility/g25/audit.json').read_text())
        comparison = json.loads((root / 'contracts/compatibility/g25/comparison.json').read_text())
        report = render(audit, comparison, projections)
        paragraph = report.split('every request-reachable projection model in the data-plane corpus: ', 1)[1].split('. If a caller', 1)[0]
        expected = sorted({r['rustType'] for r in projections if r['usage'] in ('request-only', 'request-and-response')})
        self.assertEqual(paragraph, ', '.join(f'`{name}`' for name in expected))
        self.assertIn('`TargetMetadata`', paragraph)
        self.assertIn('`LanguageOption`', paragraph)
        self.assertNotIn('`ValidationError`', paragraph)
        usage = projection_usage(projections)
        for kind, group in usage.items():
            self.assertIn(f"| `{kind}` | {group['fields']} | " + ', '.join(f'`{name}`' for name in group['models']) + ' |', report)
        # Synthetic new request projections must be reflected without editing prose.
        expanded = render(audit, comparison, projections + [{'usage': 'request-and-response', 'rustType': 'FutureModel'}])
        self.assertIn('`FutureModel`', expanded.split('. If a caller', 1)[0])

    def test_documented_projection_models_cover_both_generators(self):
        root = pathlib.Path(__file__).resolve().parents[1]
        load = lambda path: json.loads((root / path).read_text())
        projections = load('contracts/compatibility/g25/projections.json')
        overlay = load('contracts/red-team-management-model-overlay.json')
        management = overlay['modelOnlyProjections']
        self.assertEqual(len(management), 12)
        expected_management = {'BasicAuthAuthConfig', 'AuthConfigRequestVariant3',
                              'RestConnectionParams', 'StreamingConnectionParams',
                              'WebSocketConnectionParams', 'ConnectionParamsRequestVariant2'}
        self.assertEqual({row['rustType'] for row in management}, expected_management)
        report = render(load('contracts/compatibility/g25/audit.json'),
                        load('contracts/compatibility/g25/comparison.json'), projections, management, overlay['unionDecodeTransitions'])
        for transition in overlay['unionDecodeTransitions']:
            self.assertIn(f"Observable decode transition for `{transition['rustType']}`", report)
            self.assertIn(f"previously `{transition['previousVariant']}`, now `{transition['currentVariant']}`", report)
            self.assertIn(transition['previouslyUnknownShape'], report)
            self.assertIn(transition['precedence'], report)
        table = report.split('| Management projection model |', 1)[1].split('### Rules deliberately retained', 1)[0]
        actual_management = set(re.findall(r'^\| `([^`]+)` \|', table, re.MULTILINE))
        self.assertEqual(actual_management, expected_management)
        for row in management:
            self.assertRegex(table, rf'\| `{row["rustType"]}` \| `{row["legacySchema"]}` \|[^\n]*`{row["field"]}`')
        # Every data-plane model is in its usage table, including standalone types.
        dataplane_table = report.split('| Projection reachability |', 1)[1].split('Baseline decoding errors', 1)[0]
        actual_dataplane = {name for name in re.findall(r'`([^`]+)`', dataplane_table)
                           if name not in projection_usage(projections)}
        self.assertEqual(actual_dataplane | actual_management,
                         {row['rustType'] for row in projections} | expected_management)
        self.assertIn('explicit null, supersedes', report)
        sources = load('contracts/airs-active-sources.json')['sources']
        for suffix, label in [('data-plane/dp-openapi.yaml', 'data-plane vendor contract is Red Team'),
                              ('management/mp-openapi.yaml', 'management projection addendum covers Red Team management')]:
            version = next(row['version'] for row in sources if row['upstreamPath'].endswith('prisma-airs-redteam/' + suffix))
            self.assertIn(f'{label} {version}', report)
        self.assertIn('is removed from extensions', report)
        self.assertIn('historical `projection-bindings.json` owns only that data-plane corpus', report)
        with self.assertRaisesRegex(ValueError, 'Duplicate management projection'):
            management_projection_section(management + [management[0]])

    def test_report_handles_no_residual_strictness_cases(self):
        root = pathlib.Path(__file__).resolve().parents[1]
        audit = json.loads((root / 'contracts/compatibility/g25/audit.json').read_text())
        comparison = json.loads((root / 'contracts/compatibility/g25/comparison.json').read_text())
        comparison['residualStrictness'] = {'caseCount': 0, 'operationCount': 0, 'cases': [], 'mutationCounts': {}}
        report = render(audit, comparison, [])
        self.assertIn('No residual strictness cases were found in the generated corpus', report)
        self.assertNotIn('Examples: .', report)
        self.assertNotIn('Mutation counts: .', report)


if __name__ == '__main__':
    unittest.main()
