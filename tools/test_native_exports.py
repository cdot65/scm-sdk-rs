"""Adversarial closure guards without modifying frozen files or shared source."""
import copy
import json
import pathlib
import tempfile
import unittest
from unittest.mock import patch

import native_exports as ledger


class NativeExportTests(unittest.TestCase):
    def test_evidence_paths_reject_absolute_paths_traversal_and_symlinks(self):
        for relative in ("../outside", "/outside", "contracts//typescript.json"):
            with self.assertRaisesRegex(ValueError, "canonical"):
                ledger.regular(relative)
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / "real").mkdir()
            (root / "real/file").write_text("synthetic")
            (root / "alias").symlink_to(root / "real", target_is_directory=True)
            with patch.object(ledger, "ROOT", root):
                with self.assertRaisesRegex(ValueError, "symlinks"):
                    ledger.regular("alias/file")

    def overlay(self, relative, mutate):
        path = ledger.ROOT / relative
        value = json.loads(path.read_text())
        mutate(value)
        replacement = json.dumps(value)
        original = pathlib.Path.read_text

        def read(current, *args, **kwargs):
            return replacement if current == path else original(current, *args, **kwargs)

        return patch.object(pathlib.Path, "read_text", read)

    def test_every_frozen_root_and_member_has_an_external_public_probe(self):
        result, probes, gaps = ledger.generate()
        self.assertEqual(gaps, [])
        self.assertEqual(result["summary"]["implemented"], 3820)
        self.assertEqual(len({row["baselineId"] for row in result["capabilities"]}), 3820)
        self.assertEqual(len({row["probeIndex"] for row in result["capabilities"]}), 3820)
        self.assertEqual(probes.count("    // "), 3820)
        self.assertTrue(all(row["rustPaths"] and row["sources"] and row["tests"] for row in result["capabilities"]))
        self.assertEqual(result["summary"]["liveVerified"], 0)

    def test_wrong_baseline_is_rejected(self):
        with self.overlay("contracts/native-models.json", lambda v: v.update(baselineCommit="0" * 40)):
            with self.assertRaisesRegex(ValueError, "provenance"):
                ledger.generate()

    def test_stale_model_inventory_input_hash_is_rejected(self):
        with self.overlay("contracts/native-models.json", lambda v: v.update(sourceSha256="0" * 64)):
            with self.assertRaisesRegex(ValueError, "provenance"):
                ledger.generate()

    def test_root_omission_cannot_shrink_the_frozen_denominator(self):
        with self.overlay("contracts/typescript.json", lambda v: v["exports"].pop()):
            with self.assertRaisesRegex(ValueError, "denominator"):
                ledger.generate()

    def test_duplicate_public_member_is_rejected(self):
        def mutate(value):
            root = next(row for row in value["exports"] if row["name"] == "Content")
            root["members"].append(copy.deepcopy(root["members"][0]))
        with self.overlay("contracts/typescript.json", mutate):
            with self.assertRaisesRegex(ValueError, "Duplicate"):
                ledger.generate()

    def test_unknown_manual_capability_is_rejected(self):
        with self.overlay("contracts/native-capability-mappings/errors.json", lambda v: v["capabilities"][0].update(baselineId="InventedExport")):
            with self.assertRaisesRegex(ValueError, "unknown"):
                ledger.generate()

    def test_duplicate_manual_capability_is_rejected(self):
        with self.overlay("contracts/native-capability-mappings/errors.json", lambda v: v["capabilities"].append(copy.deepcopy(v["capabilities"][0]))):
            with self.assertRaisesRegex(ValueError, "Duplicate"):
                ledger.generate()

    def test_missing_test_function_is_rejected(self):
        with self.overlay("contracts/native-capability-mappings/errors.json", lambda v: v["capabilities"][0].update(tests=["crates/scm-sdk/src/error.rs::invented_test"])):
            with self.assertRaisesRegex(ValueError, "function is absent"):
                ledger.generate()

    def test_unrelated_probe_cannot_claim_an_api(self):
        with self.overlay("contracts/native-capability-mappings/errors.json", lambda v: v["capabilities"][0].update(compileProbe="let _ = 1;")):
            with self.assertRaisesRegex(ValueError, "does not reference"):
                ledger.generate()

    def test_removed_concrete_method_is_rejected_before_rust_compilation(self):
        def mutate(value):
            row = next(row for row in value["operations"] if row["baselineId"].endswith("IamScopesClient.get"))
            row["rustMethods"] = ["shared::iam::IamScopesClient::invented_method"]
        with self.overlay("contracts/native-operations.json", mutate):
            with self.assertRaisesRegex(ValueError, "public method is absent"):
                ledger.generate()

    def test_pending_operation_is_reported_as_a_gap(self):
        def mutate(value):
            row = next(row for row in value["operations"] if row["baselineId"].endswith("IamScopesClient.get"))
            row["implementation"] = "pending"
        with self.overlay("contracts/native-operations.json", mutate):
            _, _, gaps = ledger.generate()
            self.assertIn("IamScopesClient.get", {row["baselineId"] for row in gaps})

    def test_deferred_model_is_never_hidden_by_a_generic_type_fallback(self):
        def mutate(value):
            row = next(row for row in value["modelExports"] if row["export"] == "AdapterCreateRequest")
            row["status"] = "deferred_client_adapter"
        with self.overlay("contracts/native-models.json", mutate):
            _, _, gaps = ledger.generate()
            self.assertIn("AdapterCreateRequest", {row["baselineId"] for row in gaps})


if __name__ == "__main__":
    unittest.main()
