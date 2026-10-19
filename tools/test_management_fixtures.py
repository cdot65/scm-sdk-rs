"""Independent coverage and source integrity checks for management specimens."""
import copy
import unittest
from unittest import mock
import management_fixtures as fixtures


class ManagementFixtures(unittest.TestCase):
    def test_complete_projection_and_mode_witnesses(self):
        corpus = fixtures.expected()
        occurrences = corpus['projectionOccurrences']
        self.assertEqual(len({(row['schema'], row['field']) for row in occurrences}), 12)
        self.assertEqual(len({row['rustType'] for row in occurrences}), 6)
        cases = corpus['projectionCases']
        for occurrence in occurrences:
            selected = [row for row in cases if (row['schema'], row['field']) == (occurrence['schema'], occurrence['field'])]
            self.assertEqual({row['case'] for row in selected}, {'omitted','null','string','number','boolean','array','object'})
        self.assertEqual(len(corpus['unionTails']), 2)
        self.assertEqual({row['value'] for row in corpus['enumAdditions']}, {'VERTEX_AI','N8N','REALTIME'})
        self.assertIn('no service acceptance', corpus['evidenceBoundary'])

    def test_pinned_vendor_tamper_fails(self):
        with mock.patch.object(fixtures.Path, 'read_bytes', return_value=b'changed'):
            with self.assertRaisesRegex(ValueError, 'pin'):
                fixtures.expected()

    def test_missing_model_or_projection_requires_review(self):
        with mock.patch.object(fixtures, 'MODELS', fixtures.MODELS[:-1]):
            with self.assertRaisesRegex(ValueError, 'coverage'):
                fixtures.expected()

    def test_expected_values_are_independent_of_generated_graph(self):
        corpus = fixtures.expected()
        paths = {row['path'] for row in corpus['sourceBindings']}
        self.assertNotIn('crates/scm-sdk/src/validation/schema_graph.json', paths)
        self.assertNotIn('contracts/red-team-management-model-overlay.json', paths)
        defaults = next(row for row in corpus['projectionCases'] if row['rustType'] == 'WebSocketConnectionParams' and row['case'] == 'omitted')
        self.assertEqual(defaults['expected']['ws_response_timeout'], 110)
        self.assertNotIn('ws_response_timeout', defaults['withoutDefaults'])


if __name__ == '__main__':
    unittest.main()
