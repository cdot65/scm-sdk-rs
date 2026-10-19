"""Interrupted execution cannot turn a passing prefix into a full gate."""
import datetime
import json
from pathlib import Path
import tempfile
import unittest
from unittest.mock import patch
import check_phase as gate

class CheckpointTests(unittest.TestCase):
    def setUp(self):
        directory = tempfile.TemporaryDirectory()
        self.addCleanup(directory.cleanup)
        self.root = Path(directory.name)
        self.path = self.root / 'docs/checks/phase-23-partial.json'
        self.commands = [['python', 'fixture.py', '--output', 'docs/checks/fixture.json'], ['python', 'second.py']]
        self.receipt = self.root / 'docs/checks/fixture.json'
        self.receipt.parent.mkdir(parents=True)
        self.receipt.write_text('{}')
        self.started = datetime.datetime.now(datetime.timezone.utc).isoformat()
        self.patch = patch.object(gate, 'ROOT', self.root)
        self.patch.start(); self.addCleanup(self.patch.stop)
        self.record = {'command': self.commands[0], 'exit_code': 0, 'timeout': False,
                       'source_unchanged': True, 'checkpoint_bindings': gate.checkpoint_bindings(self.commands[0])}
        gate.write_checkpoint(self.path, 23, 'source', self.commands, [self.record], self.started)

    def test_only_completed_prefix_restored(self):
        records, started = gate.read_checkpoint(self.path, 23, 'source', self.commands)
        self.assertEqual(records, [self.record]); self.assertEqual(started, self.started)
        self.assertFalse(json.loads(self.path.read_text())['complete'])

    def test_source_phase_command_changes_rejected(self):
        for phase, source, commands in [(24,'source',self.commands), (23,'changed',self.commands), (23,'source',list(reversed(self.commands)))]:
            with self.assertRaises(ValueError): gate.read_checkpoint(self.path, phase, source, commands)

    def test_failed_timed_out_incomplete_or_reordered_records_rejected(self):
        for key, value in [('exit_code',1), ('timeout',True), ('source_unchanged',False), ('command',self.commands[1])]:
            changed = {**self.record, key:value}
            gate.write_checkpoint(self.path,23,'source',self.commands,[changed],self.started)
            with self.assertRaises(ValueError): gate.read_checkpoint(self.path,23,'source',self.commands)

    def test_modified_or_missing_receipt_rejected(self):
        self.receipt.write_text('{"changed":true}')
        with self.assertRaises(ValueError): gate.read_checkpoint(self.path,23,'source',self.commands)
        self.receipt.unlink()
        with self.assertRaises(ValueError): gate.read_checkpoint(self.path,23,'source',self.commands)

    def test_archive_bytes_also_bound(self):
        archive=self.root/'archive.crate';archive.write_bytes(b'archive')
        self.receipt.write_text(json.dumps({'retainedArchive':str(archive)}))
        self.record['checkpoint_bindings']=gate.checkpoint_bindings(self.commands[0])
        gate.write_checkpoint(self.path,23,'source',self.commands,[self.record],self.started)
        archive.write_bytes(b'changed')
        with self.assertRaises(ValueError): gate.read_checkpoint(self.path,23,'source',self.commands)

    def test_stale_and_future_checkpoints_rejected(self):
        for hours in [-9,1]:
            date=(datetime.datetime.now(datetime.timezone.utc)+datetime.timedelta(hours=hours)).isoformat()
            gate.write_checkpoint(self.path,23,'source',self.commands,[self.record],date)
            with self.assertRaises(ValueError): gate.read_checkpoint(self.path,23,'source',self.commands)

    def test_no_checkpoint_starts_empty_and_symlink_rejected(self):
        self.path.unlink(); self.assertEqual(gate.read_checkpoint(self.path,23,'source',self.commands)[0],[])
        self.path.symlink_to(self.receipt)
        with self.assertRaises(ValueError): gate.read_checkpoint(self.path,23,'source',self.commands)

if __name__ == '__main__': unittest.main()
