"""Pin historical release selection and required G25 archive/consumer execution."""
import copy
import importlib.util
import pathlib
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location('release_candidate', ROOT / 'scripts/check-access-release.py')
candidate = importlib.util.module_from_spec(spec)
spec.loader.exec_module(candidate)
spec = importlib.util.spec_from_file_location('registry_verifier_022', ROOT / 'scripts/verify-publication.py')
publication = importlib.util.module_from_spec(spec)
spec.loader.exec_module(publication)


class Release022Tests(unittest.TestCase):
    def fixture(self, version='0.2.2'):
        examples = ['response_defaults', 'red_team_dataplane', 'access_policies']
        if version == '0.2.2':
            examples.append('response_compatibility')
        files = {'examples/' + name + '.rs': name.encode() for name in examples}
        checks = [{'command': ['cargo', '+1.89.0', 'run', '--example', name, '--all-features']} for name in examples]
        checks += [{'command': ['cargo', '+1.89.0', 'run', '--locked', '--offline']} for _ in range(3 if version == '0.2.2' else 2)]
        proof = {'checks': checks, 'accessPolicyConsumerSourceSha256': candidate.sha(files['examples/access_policies.rs'])}
        if version == '0.2.2':
            proof['responseCompatibilityConsumerSourceSha256'] = candidate.sha(files['examples/response_compatibility.rs'])
        return proof, files

    def test_historical_defaults_and_reviewed_release_pairs(self):
        self.assertEqual(candidate.verify.__defaults__, (17, '0.2.1'))
        self.assertEqual(set(candidate.RELEASES), {(17, '0.2.1'), (20, '0.2.2')})
        for phase, version in [(17, '0.2.2'), (20, '0.2.1'), (21, '0.2.3')]:
            with self.assertRaisesRegex(ValueError, 'Unreviewed release'):
                candidate.verify(ROOT, phase, version)

    def test_old_and_new_executed_delivery_profiles(self):
        for version in ('0.2.1', '0.2.2'):
            proof, files = self.fixture(version)
            candidate.verify_offline_delivery(proof, files, version)

    def test_every_archive_example_is_required(self):
        proof, files = self.fixture()
        for example in ('response_defaults', 'red_team_dataplane', 'access_policies', 'response_compatibility'):
            changed = copy.deepcopy(proof)
            changed['checks'] = [row for row in changed['checks'] if example not in row['command']]
            with self.assertRaisesRegex(ValueError, 'Offline example'):
                candidate.verify_offline_delivery(changed, files, '0.2.2')

    def test_consumer_source_and_execution_are_required(self):
        proof, files = self.fixture()
        for field in ('accessPolicyConsumerSourceSha256', 'responseCompatibilityConsumerSourceSha256'):
            for value in (None, '0' * 64):
                changed = copy.deepcopy(proof)
                changed[field] = value
                with self.assertRaisesRegex(ValueError, 'consumer differs'):
                    candidate.verify_offline_delivery(changed, files, '0.2.2')
        proof['checks'].pop()
        with self.assertRaisesRegex(ValueError, 'consumers must execute'):
            candidate.verify_offline_delivery(proof, files, '0.2.2')

    def test_registry_verifier_requires_each_present_response_example(self):
        proof, files = self.fixture()
        publication.verify_response_examples(proof, files)
        for example in ('response_defaults', 'response_compatibility'):
            changed = copy.deepcopy(proof)
            changed['checks'] = [row for row in changed['checks'] if example not in row['command']]
            with self.subTest(example=example), self.assertRaisesRegex(ValueError, 'not executed'):
                publication.verify_response_examples(changed, files)
        publication.verify_response_examples({'checks': []}, {})


if __name__ == '__main__':
    unittest.main()
