"""Guard the packaged regression subset without mutating source inputs."""
import copy
import json
import pathlib
import unittest
from unittest.mock import patch
import model_fixtures as fixture


class ModelFixturesTests(unittest.TestCase):
    def test_all_eighteen_exact_zod_outputs_are_present(self):
        result = fixture.generate()
        self.assertEqual(len(result['specimens']), 18)
        self.assertEqual([x['name'] for x in result['specimens']], fixture.NAMES)
        self.assertLess(len(json.dumps(result)), 30_000)
        corpus = json.loads((fixture.ROOT / fixture.SOURCE).read_text())
        for item in result['specimens']:
            current = corpus
            for token in item['sourcePointer'].split('/')[1:]:
                current = current[int(token)] if isinstance(current, list) else current[token]
            self.assertEqual(item['output'], current)

    def mutated(self, mutate):
        source = fixture.ROOT / fixture.SOURCE
        data = json.loads(source.read_text())
        mutate(data)
        original = pathlib.Path.read_text
        return patch.object(pathlib.Path, 'read_text', lambda p, *a, **k: json.dumps(data) if p == source else original(p, *a, **k))

    def test_modified_packaged_output_is_rejected(self):
        expected = json.dumps(fixture.generate(), indent=2, ensure_ascii=False, allow_nan=False) + '\n'
        path = fixture.ROOT / fixture.OUTPUT
        changed = json.loads(expected)
        changed['specimens'][0]['output'] = {'tampered': True}
        original = pathlib.Path.read_text
        with patch.object(pathlib.Path, 'read_text', lambda p, *a, **k: json.dumps(changed) if p == path else original(p, *a, **k)):
            with self.assertRaisesRegex(ValueError, 'differs'):
                fixture.verify_output(expected)

    def test_wrong_baseline_rejected(self):
        with self.mutated(lambda data: data['baseline'].update(commit='0' * 40)):
            with self.assertRaisesRegex(ValueError, 'baseline'):
                fixture.generate()

    def test_stale_oracle_hash_rejected(self):
        with self.mutated(lambda data: data['provenance'].update(generatorSha256='0' * 64)):
            with self.assertRaisesRegex(ValueError, 'provenance'):
                fixture.generate()

    def test_missing_or_duplicate_root_rejected(self):
        name = fixture.NAMES[0]
        for duplicate in (False, True):
            def mutate(data):
                root = next(x for x in data['roots'] if name in x['names'])
                if duplicate:
                    data['roots'].append(copy.deepcopy(root))
                else:
                    root['names'].remove(name)
            with self.mutated(mutate):
                with self.assertRaisesRegex(ValueError, 'Missing or duplicate'):
                    fixture.generate()

    def test_unaccepted_specimen_rejected(self):
        def mutate(data):
            root = next(x for x in data['roots'] if fixture.NAMES[0] in x['names'])
            for case in root['jsonCases']:
                case['zod']['success'] = False
        with self.mutated(mutate):
            with self.assertRaisesRegex(ValueError, 'No independent accepted'):
                fixture.generate()
