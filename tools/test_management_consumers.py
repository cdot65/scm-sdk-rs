"""Tampering checks for the immutable published consumer source corpus."""
import json
import shutil
import tempfile
import unittest
from pathlib import Path

import management_consumers as consumers


class PublishedConsumerTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = Path(self.temporary.name)
        manifest = consumers.verify()
        paths = [consumers.MANIFEST, manifest['receipt']['path']]
        paths += [row[key] for row in manifest['consumers'] for key in ('path', 'frozenPath')]
        for path in paths:
            dest = self.root / path
            dest.parent.mkdir(parents=True, exist_ok=True)
            shutil.copyfile(consumers.ROOT / path, dest)
        self.manifest = manifest

    def test_exact_original_sources_verify_without_archive_or_network(self):
        self.assertEqual(consumers.verify(self.root), self.manifest)

    def test_manifest_changes_cannot_redefine_original(self):
        path = self.root / consumers.MANIFEST
        changed = json.loads(path.read_text())
        changed['consumers'][0]['sha256'] = '0' * 64
        path.write_text(json.dumps(changed))
        with self.assertRaisesRegex(ValueError, 'manifest pin'):
            consumers.verify(self.root)

    def test_changed_current_or_frozen_bytes_are_rejected(self):
        for row in self.manifest['consumers']:
            for key in ('path', 'frozenPath'):
                with self.subTest(consumer=row['name'], key=key):
                    path = self.root / row[key]
                    original = path.read_bytes()
                    path.write_bytes(original + b'\n')
                    with self.assertRaises(ValueError):
                        consumers.verify(self.root)
                    path.write_bytes(original)

    def test_changed_release_receipt_is_rejected(self):
        path = self.root / self.manifest['receipt']['path']
        path.write_bytes(path.read_bytes() + b'\n')
        with self.assertRaisesRegex(ValueError, 'receipt pin'):
            consumers.verify(self.root)

    def test_unowned_frozen_file_is_rejected(self):
        (self.root / consumers.FROZEN_ROOT / 'extra.rs').write_text('// unowned\n')
        with self.assertRaisesRegex(ValueError, 'Unowned'):
            consumers.verify(self.root)

    def test_missing_frozen_source_is_rejected(self):
        (self.root / self.manifest['consumers'][0]['frozenPath']).unlink()
        with self.assertRaises(OSError):
            consumers.verify(self.root)

    def test_symlink_and_parent_escape_are_rejected(self):
        row = self.manifest['consumers'][0]
        path = self.root / row['frozenPath']
        path.unlink()
        path.symlink_to(self.root / row['path'])
        with self.assertRaisesRegex(ValueError, 'Symlink'):
            consumers.verify(self.root)
        with self.assertRaisesRegex(ValueError, 'Unsafe'):
            consumers.read(self.root, '../outside')


if __name__ == '__main__':
    unittest.main()
