"""Adversarial closure guards without modifying frozen files or shared source."""
import copy
import json
import pathlib
import tempfile
import unittest
from unittest.mock import patch

import native_exports as ledger


class NativeExportTests(unittest.TestCase):
    def setUp(self):
        self.live = patch.object(ledger.native_operations, "live_projection",
                                 return_value=({}, {"tools/live_evidence.py"}, None))
        self.live.start()
        self.addCleanup(self.live.stop)
        original = pathlib.Path.read_text
        native_path = ledger.ROOT / "contracts/native-operations.json"

        def without_receipt(path, *args, **kwargs):
            text = original(path, *args, **kwargs)
            if path == native_path:
                value = json.loads(text)
                for row in value["operations"]:
                    row["liveAcceptance"] = "unverified"
                    row.pop("liveEvidence", None)
                return json.dumps(value)
            return text

        self.offline = patch.object(pathlib.Path, "read_text", without_receipt)
        self.offline.start()
        self.addCleanup(self.offline.stop)

    def test_operation_live_evidence_never_promotes_its_constructor_or_sibling(self):
        operation_id = "src/scan/scanner.ts:Scanner.syncScan"
        annotation = {"liveAcceptance": "verified", "liveEvidence": {
            "path": "docs/checks/phase-08-live-attempt-01.json", "sha256": "a" * 64,
            "probe": "scan.sync_scan", "scope": "controlled_nonproduction",
            "runtimeInputsSha256": "b" * 64}}

        def qualify(value):
            row = next(row for row in value["operations"] if row["baselineId"] == operation_id)
            row.update(annotation)

        with self.overlay("contracts/native-operations.json", qualify), \
                patch.object(ledger.native_operations, "live_projection",
                             return_value=({operation_id: annotation}, set(), None)):
            result, _, gaps = ledger.generate()
        self.assertFalse(gaps)
        rows = {row["baselineId"]: row for row in result["capabilities"]}
        self.assertEqual(rows["Scanner.syncScan"]["liveEvidence"], annotation["liveEvidence"])
        self.assertEqual(rows["Scanner.syncScan"]["liveAcceptance"], "verified")
        self.assertEqual(rows["Scanner"]["liveAcceptance"], "unverified")
        self.assertNotIn("liveEvidence", rows["Scanner"])
        self.assertEqual(rows["Scanner.asyncScan"]["liveAcceptance"], "unverified")
        self.assertEqual(result["summary"]["liveVerified"], 1)

    def test_persisted_live_claim_without_a_receipt_is_rejected(self):
        def promote(value):
            value["operations"][0]["liveAcceptance"] = "verified"
        with self.overlay("contracts/native-operations.json", promote):
            with self.assertRaisesRegex(ValueError, "stale or unsupported"):
                ledger.generate()

    def test_stale_receipt_aborts_export_generation(self):
        with patch.object(ledger.native_operations, "live_projection",
                          side_effect=ValueError("stale native receipt")):
            with self.assertRaisesRegex(ValueError, "stale native receipt"):
                ledger.generate()

    def test_global_json_boundary_difference_is_retained_and_cannot_be_omitted(self):
        result, _, gaps = ledger.generate()
        self.assertFalse(gaps)
        self.assertEqual(result["jsonBoundaryContract"], ledger.native_operations.json_boundary_contract())
        self.assertEqual(result["summary"]["frozenCapabilities"], 3820)
        for mutate in (
            lambda value: value.pop("jsonBoundaryContract"),
            lambda value: value["jsonBoundaryContract"]["reservedObjectKeys"].pop(),
            lambda value: value["jsonBoundaryContract"]["errors"].update(receivedJson="success"),
        ):
            with self.overlay("contracts/native-operations.json", mutate):
                with self.assertRaisesRegex(ValueError, "JSON boundary intentional difference"):
                    ledger.generate()

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

    def test_probe_cfg_is_the_exact_required_feature_union_including_agentguard(self):
        result, probes, gaps = ledger.generate()
        self.assertEqual(gaps, [])
        expected = sorted({feature for row in result["capabilities"] for feature in row["requiredFeatures"]})
        self.assertIn("agentguard", expected)
        self.assertEqual(result["probe"]["requiredFeatures"], expected)
        gate = probes.split("#![cfg(all(", 1)[1].split("))]", 1)[0]
        import re
        self.assertEqual(re.findall(r'feature = "([^"]+)"', gate), expected)

    def test_facades_and_content_bind_real_behavior_instead_of_only_compile_probes(self):
        result, _, gaps = ledger.generate()
        self.assertFalse(gaps)
        owners = {"AIGatewayClient", "ManagementClient", "DlpNamespace", "Content"}
        rows = [row for row in result["capabilities"]
                if row["baselineId"].split(".", 1)[0] in owners
                and row["verification"] in ("typed_public_client", "public_accessor")]
        self.assertEqual(len(rows), 45)
        self.assertTrue(all(any(test.startswith("crates/scm-sdk/tests/facade_contracts.rs::")
                               for test in row["tests"]) for row in rows))
        self.assertFalse(any(row["tests"] == ["crates/scm-sdk/tests/native_export_probes.rs::all_public_export_paths_compile"] for row in rows))

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
