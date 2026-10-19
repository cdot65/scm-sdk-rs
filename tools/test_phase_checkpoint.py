"""Interrupted execution cannot turn a passing prefix into a full gate."""
import copy
import contextlib
import datetime
import io
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import check_phase as gate
import review_phase


class CheckpointTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        self.path = self.root / 'docs/checks/phase-23-partial.json'
        self.commands = [['python', 'fixture.py', '--output', 'docs/checks/fixture.json'],
                         ['python', 'second.py']]
        self.receipt = self.root / 'docs/checks/fixture.json'
        self.receipt.parent.mkdir(parents=True)
        self.receipt.write_text('{}')
        self.started = datetime.datetime.now(datetime.timezone.utc).isoformat()
        self.patch = patch.object(gate, 'ROOT', self.root)
        self.patch.start()
        self.addCleanup(self.patch.stop)
        self.record = {'command': self.commands[0], 'exit_code': 0, 'timeout': False,
                       'source_unchanged': True,
                       'checkpoint_bindings': gate.checkpoint_bindings(self.commands[0])}
        gate.write_checkpoint(self.path, 23, 'source', self.commands, [self.record], self.started)

    def test_only_completed_prefix_restored(self):
        records, started = gate.read_checkpoint(self.path, 23, 'source', self.commands)
        self.assertEqual(records, [self.record])
        self.assertEqual(started, self.started)
        self.assertFalse(json.loads(self.path.read_text())['complete'])

    def test_source_phase_command_changes_rejected(self):
        for phase, source, commands in [(24, 'source', self.commands), (23, 'changed', self.commands),
                                       (23, 'source', list(reversed(self.commands)))]:
            with self.assertRaises(ValueError):
                gate.read_checkpoint(self.path, phase, source, commands)

    def test_failed_timed_out_incomplete_or_reordered_records_rejected(self):
        for key, value in [('exit_code', 1), ('timeout', True),
                           ('source_unchanged', False), ('command', self.commands[1])]:
            changed = {**self.record, key: value}
            gate.write_checkpoint(self.path, 23, 'source', self.commands, [changed], self.started)
            with self.assertRaises(ValueError):
                gate.read_checkpoint(self.path, 23, 'source', self.commands)

    def test_modified_or_missing_receipt_rejected(self):
        self.receipt.write_text('{"changed":true}')
        with self.assertRaises(ValueError):
            gate.read_checkpoint(self.path, 23, 'source', self.commands)
        self.receipt.unlink()
        with self.assertRaises(ValueError):
            gate.read_checkpoint(self.path, 23, 'source', self.commands)

    def test_archive_bytes_also_bound(self):
        archive = self.root / 'archive.crate'
        archive.write_bytes(b'archive')
        self.receipt.write_text(json.dumps({'retainedArchive': str(archive)}))
        self.record['checkpoint_bindings'] = gate.checkpoint_bindings(self.commands[0])
        gate.write_checkpoint(self.path, 23, 'source', self.commands, [self.record], self.started)
        archive.write_bytes(b'changed')
        with self.assertRaises(ValueError):
            gate.read_checkpoint(self.path, 23, 'source', self.commands)

    def test_stale_and_future_checkpoints_rejected(self):
        for hours in [-9, 1]:
            date = (datetime.datetime.now(datetime.timezone.utc) + datetime.timedelta(hours=hours)).isoformat()
            gate.write_checkpoint(self.path, 23, 'source', self.commands, [self.record], date)
            with self.assertRaises(ValueError):
                gate.read_checkpoint(self.path, 23, 'source', self.commands)

    def test_no_checkpoint_starts_empty_and_symlink_rejected(self):
        self.path.unlink()
        self.assertEqual(gate.read_checkpoint(self.path, 23, 'source', self.commands)[0], [])
        self.path.symlink_to(self.receipt)
        with self.assertRaises(ValueError):
            gate.read_checkpoint(self.path, 23, 'source', self.commands)

    def test_changed_earlier_receipt_rejected_before_final_report(self):
        records, _ = gate.read_checkpoint(self.path, 23, 'source', self.commands)
        records.append({'command': self.commands[1], 'exit_code': 0,
                        'checkpoint_bindings': {}})
        self.receipt.write_text('{"changedByLaterCommand": true}')
        with self.assertRaisesRegex(ValueError, 'Checkpoint evidence changed'):
            gate.verify_checkpoint_bindings(records)

    def test_review_rechecks_current_receipts_but_preserves_history(self):
        report = {'phase': 23, 'passed': True, 'source_sha256': 'source',
                  'source_unchanged': True, 'checks': [copy.deepcopy(self.record)]}
        final = self.root / 'docs/checks/phase-23.json'
        final.write_text(json.dumps(report))
        with patch.object(gate, 'commands_for', return_value=self.commands[:1]), \
                patch.object(review_phase, 'fingerprint', return_value='source'):
            self.assertEqual(review_phase.check_evidence(self.root, 23, 'source'), final)
            self.receipt.write_text('{"changedAfterGate": true}')
            with self.assertRaisesRegex(ValueError, 'Checkpoint evidence changed'):
                review_phase.check_evidence(self.root, 23, 'source')
        with patch.object(gate, 'commands_for', return_value=self.commands[:1]), \
                patch.object(review_phase, 'fingerprint', return_value='later-source'):
            self.assertEqual(review_phase.check_evidence(self.root, 23, 'source'), final)

    def test_missing_binding_rejected(self):
        row = {key: value for key, value in self.record.items()
               if key != 'checkpoint_bindings'}
        with self.assertRaisesRegex(ValueError, 'Checkpoint evidence changed'):
            gate.verify_checkpoint_bindings([row])

    def test_later_command_mutation_cannot_write_passing_report(self):
        (self.root / 'docs/phase-status.json').write_text('{"active_phase": 23}')
        self.path.unlink()

        def execute(command):
            if command[-1] == 'second.py':
                self.receipt.write_text('{"replacedDuringRun": true}')
            return {'exit_code': 0, 'timeout': False, 'stdout': '', 'stderr': ''}

        with patch('sys.argv', ['check_phase.py', '--phase', '23', '--resume']), \
                patch.object(gate, 'commands_for', return_value=self.commands), \
                patch.object(gate, 'fingerprint', return_value='source'), \
                patch.object(gate, 'run_check', side_effect=execute), \
                contextlib.redirect_stdout(io.StringIO()):
            with self.assertRaisesRegex(ValueError, 'Checkpoint evidence changed'):
                gate.main()
        self.assertFalse((self.root / 'docs/checks/phase-23.json').exists())


if __name__ == '__main__':
    unittest.main()
