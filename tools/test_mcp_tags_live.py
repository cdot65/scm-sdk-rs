"""Privacy allowlist, accounting and freshness tests for the MCP tags live receipt."""
import copy
import json
import pathlib
import tempfile
import unittest
from unittest import mock

import mcp_tags_live

ROOT = pathlib.Path(__file__).resolve().parents[1]
RECEIPT = ROOT / json.loads((ROOT / 'contracts/vendor/portkey-mcp-tags/8e5da0e/operations.json').read_text())['liveReceipt']


class McpTagsLiveReceiptTests(unittest.TestCase):
    def setUp(self):
        self.envelope = json.loads(RECEIPT.read_text())

    def check_copy(self, envelope, current=False):
        with tempfile.TemporaryDirectory() as directory:
            path = pathlib.Path(directory) / 'receipt.json'
            path.write_text(json.dumps(envelope))
            return mcp_tags_live.check(path, current=current)

    def test_retained_receipt_records_the_complete_verified_workflow(self):
        envelope = self.check_copy(self.envelope)
        self.assertEqual(envelope['summary']['failedSteps'], [])
        self.assertEqual(envelope['summary']['cleanupState'], 'complete')
        self.assertEqual(len(envelope['summary']['verifiedSteps']), 19)
        report = envelope['report']
        self.assertEqual((report['fixturesCreated'], report['fixturesUncertain'], report['fixturesDeleted']), (2, 0, 2))
        for operation in ('mcp_servers.update.tags_replace', 'mcp_servers.get.tags_replaced'):
            self.assertIn(operation, envelope['summary']['verifiedSteps'])

    def test_unexpected_fields_or_values_cannot_be_exported(self):
        for mutate in (lambda e: e['report'].update(fixtureId='550e8400-e29b-41d4-a716-446655440000'),
                       lambda e: e['report']['steps'][0].update(message='tenant text'),
                       lambda e: e['report']['steps'][0].update(operation='mcp_integrations.export_everything'),
                       lambda e: e['report']['steps'][0].update(outcome='error', errorKind='Http', serviceCode='bad code with spaces'),
                       lambda e: e['report'].update(tenantScopeMatched=False),
                       lambda e: e.update(extra=True)):
            envelope = copy.deepcopy(self.envelope)
            mutate(envelope)
            with self.assertRaises(ValueError):
                self.check_copy(envelope)

    def test_cleanup_accounting_and_summary_must_follow_the_steps(self):
        envelope = copy.deepcopy(self.envelope)
        envelope['report']['fixturesDeleted'] = 1
        with self.assertRaisesRegex(ValueError, 'Cleanup state'):
            self.check_copy(envelope)
        envelope = copy.deepcopy(self.envelope)
        envelope['summary']['failedSteps'] = []
        envelope['summary']['verifiedSteps'] = envelope['summary']['verifiedSteps'][:-1]
        with self.assertRaisesRegex(ValueError, 'summary differs'):
            self.check_copy(envelope)
        envelope = copy.deepcopy(self.envelope)
        envelope['report']['steps'][1]['verified'] = False
        with self.assertRaisesRegex(ValueError, 'Verification outcome'):
            self.check_copy(envelope)

    def test_uncertain_fixtures_never_report_complete_cleanup(self):
        envelope = copy.deepcopy(self.envelope)
        envelope['report']['fixturesUncertain'] = 1
        envelope['report']['fixturesCreated'] = 1
        envelope['report']['fixturesDeleted'] = 1
        with self.assertRaisesRegex(ValueError, 'Cleanup state'):
            self.check_copy(envelope)
        envelope['report']['cleanupState'] = 'incomplete'
        envelope['summary']['cleanupState'] = 'incomplete'
        self.check_copy(envelope)

    def test_example_producer_fixture_for_failed_reconciliation_validates(self):
        # The same bytes are asserted equal to the example's offline producer output.
        report = json.loads((ROOT / 'crates/scm-sdk/examples/mcp_tags_live_uncertain_report.json').read_text())
        report['crateVersion'] = mcp_tags_live.crate_version()
        validated = mcp_tags_live.validate_report(report)
        self.assertEqual(validated['cleanupState'], 'incomplete')
        self.assertEqual(mcp_tags_live.summarize(validated)['verifiedSteps'], [])

    def test_stale_runtime_and_unclean_execution_are_rejected(self):
        with mock.patch.object(mcp_tags_live, 'runtime_fingerprint', return_value='0' * 64):
            with self.assertRaisesRegex(ValueError, 'stale for the current runtime'):
                self.check_copy(self.envelope, current=True)
        envelope = copy.deepcopy(self.envelope)
        envelope['sourceUnchanged'] = False
        with self.assertRaisesRegex(ValueError, 'clean execution'):
            self.check_copy(envelope)


if __name__ == '__main__':
    unittest.main()
