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
        self.assertEqual(result["summary"]["implemented"], len(ledger.mappings()))
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

    def test_representative_gateway_scope_retains_its_real_owner_and_partial_options(self):
        with tempfile.TemporaryDirectory() as directory, mock.patch.object(ledger, "FRAGMENTS", pathlib.Path(directory)):
            rows = {row["baselineId"]: row for row in ledger.generate()["operations"]}
        row = rows["src/ai-gateway/inference-client.ts:AIGatewayInferenceClient.createChatCompletion"]
        self.assertEqual(row["owningPhase"], 6)
        self.assertEqual(row["deliveredInPhase"], 3)
        self.assertEqual(row["optionParity"], "partial")
        self.assertTrue(row["deliberateDifferences"])

    def test_stricter_native_content_invariants_are_machine_recorded(self):
        rows = {row["baselineId"]: row for row in ledger.generate()["operations"]}
        self.assertTrue(rows["src/scan/scanner.ts:Scanner.asyncScan"]["deliberateDifferences"])

    def test_partial_parity_without_a_declared_reason_is_rejected(self):
        native = ledger.mappings()
        row = native["src/ai-gateway/inference-client.ts:AIGatewayInferenceClient.createChatCompletion"]
        row["optionParity"] = "partial"
        row["deliberateDifferences"] = []
        with mock.patch.object(ledger, "mappings", return_value=native):
            with self.assertRaisesRegex(ValueError, "explicit difference"):
                ledger.generate()

    def test_completed_owning_phase_rejects_an_unimplemented_operation(self):
        native = ledger.mappings()
        native.pop("src/management/oauth-client.ts:OAuthClient.getToken")
        with mock.patch.object(ledger, "mappings", return_value=native), mock.patch("sys.argv", ["native_operations.py", "--check", "--require-through-phase", "4"]):
            with self.assertRaisesRegex(ValueError, "owning-phase capability gaps"):
                ledger.main()

    def test_absent_verification_function_is_rejected(self):
        native = ledger.mappings()
        native["src/scan/scanner.ts:Scanner.syncScan"]["tests"] = ["crates/scm-sdk/tests/scan_contracts.rs::invented_verification"]
        with mock.patch.object(ledger, "mappings", return_value=native):
            with self.assertRaisesRegex(ValueError, "verification function"):
                ledger.generate()

    def test_fragment_baseline_and_duplicate_mappings_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            path = pathlib.Path(directory)
            entry = next(iter(ledger.mappings().values()))
            data = {"schemaVersion": 1, "baselineCommit": "wrong", "operations": [entry]}
            (path / "one.json").write_text(json.dumps(data))
            with mock.patch.object(ledger, "FRAGMENTS", path):
                with self.assertRaisesRegex(ValueError, "wrong baseline"):
                    ledger.generate()
            data["baselineCommit"] = json.loads(ledger.SOURCE.read_text())["baseline"]["commit"]
            for name in ("one.json", "two.json"):
                (path / name).write_text(json.dumps(data))
            with mock.patch.object(ledger, "FRAGMENTS", path):
                with self.assertRaisesRegex(ValueError, "Duplicate"):
                    ledger.generate()


if __name__ == "__main__":
    unittest.main()
