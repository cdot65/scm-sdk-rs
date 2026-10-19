"""Guard the packaged regression subset without mutating source inputs."""
import copy
import hashlib
import json
import pathlib
import unittest
from unittest.mock import patch
import model_fixtures as fixture


class ModelFixturesTests(unittest.TestCase):
    def test_frozen_oracle_tooling_provenance_matches_current_locked_inputs(self):
        corpus=json.loads((fixture.ROOT/fixture.SOURCE).read_text())
        rows=corpus['provenance']['tooling']
        self.assertEqual([r['file'] for r in rows],['tools/package.json','tools/package-lock.json'])
        for row in rows:
            self.assertEqual(row['sha256'],fixture.sha(fixture.ROOT/row['file']))

    def test_g25_format_tolerant_response_policy_and_observations_are_pinned(self):
        corpus=json.loads((fixture.ROOT/fixture.SOURCE).read_text())
        corpus['provenance'].pop('tooling')
        # New source pins rebind the manifest, while frozen oracle observations
        # remain identical. Verify the live binding separately from that pin.
        manifest_sha=corpus['provenance'].pop('manifestSha256')
        self.assertEqual(manifest_sha,fixture.sha(fixture.ROOT/'contracts/manifest.json'))
        digest=hashlib.sha256(json.dumps(corpus,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
        self.assertEqual(digest,'ddc0f2ac51368005823957a1de68c0517ce151b6d94ff4e95e5875553ba5615b')

    def test_original_frozen_request_observations_are_unchanged(self):
        corpus=json.loads((fixture.ROOT/fixture.SOURCE).read_text())
        request=[{**root, 'jsonCases': [
            {key: value for key, value in case.items() if key != 'response'}
            for case in root['jsonCases']]} for root in corpus['roots']]
        digest=hashlib.sha256(json.dumps(request,sort_keys=True,separators=(',',':'),ensure_ascii=False).encode()).hexdigest()
        self.assertEqual(digest,'d9c02bbced1cf94a3ba0c219b62e38816e60518c034a8ee929aa6318fca47f0d')

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
