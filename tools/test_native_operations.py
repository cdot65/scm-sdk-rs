"""The native ledger must not silently accept missing or invented mappings."""
import json
import pathlib
import tempfile
import unittest
from unittest import mock

import native_operations as ledger


class NativeOperationLedgerTests(unittest.TestCase):
    def test_every_frozen_operation_is_accounted_for_without_baseline_mutation(self):
        before = ledger.SOURCE.read_bytes()
        result = ledger.generate()
        ids = [row["baselineId"] for row in result["operations"]]
        expected = {row["id"] for row in json.loads(before)["operations"]}
        self.assertEqual(set(ids), expected)
        self.assertEqual(len(ids), len(expected))
        self.assertEqual(result["summary"]["implemented"], len(ledger.MAPPINGS))
        self.assertEqual(ledger.SOURCE.read_bytes(), before)
        self.assertEqual(result["requestHeaderContracts"]["airs_api_defaults"]["service-name"], "api")
        self.assertRegex(result["requestHeaderContracts"]["airs_api_defaults"]["user-agent"], r"^PAN-AIRS/.+-rust-sdk$")

    def test_invented_frozen_operation_is_rejected(self):
        with mock.patch.dict(ledger.MAPPINGS, {"invented:Client.method": next(iter(ledger.MAPPINGS.values()))}):
            with self.assertRaisesRegex(ValueError, "absent frozen operations"):
                ledger.generate()

    def test_removed_native_method_is_rejected(self):
        original = pathlib.Path.read_text
        source = ledger.ROOT / "crates/scm-sdk/src/airs/scan/scanner.rs"

        def read(path, *args, **kwargs):
            value = original(path, *args, **kwargs)
            return value.replace("fn sync_scan(", "fn removed_scan(") if path == source else value

        with mock.patch.object(pathlib.Path, "read_text", read):
            with self.assertRaisesRegex(ValueError, "Native method absent"):
                ledger.generate()

    def test_stale_ledger_is_rejected_without_rewriting_it(self):
        with tempfile.TemporaryDirectory() as directory:
            output = pathlib.Path(directory) / "ledger.json"
            output.write_text("{}\n")
            with mock.patch.object(ledger, "OUTPUT", output), mock.patch("sys.argv", ["native_operations.py", "--check"]):
                with self.assertRaisesRegex(ValueError, "ledger differs"):
                    ledger.main()
            self.assertEqual(output.read_text(), "{}\n")


if __name__ == "__main__":
    unittest.main()
