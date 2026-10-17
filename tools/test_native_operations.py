"""The native ledger must not silently accept missing or invented mappings."""
import json
import pathlib
import tempfile
import unittest
from unittest import mock

import native_operations as ledger


class NativeOperationLedgerTests(unittest.TestCase):
    def setUp(self):
        # Ordinary ledger guards must stay independent of an in-progress live run.
        self.live_projection = mock.patch.object(
            ledger, "live_projection", return_value=({}, {"tools/live_evidence.py"}, None)
        )
        self.live_projection.start()
        self.addCleanup(self.live_projection.stop)

    def test_missing_receipt_cannot_be_promoted_by_a_mapping_declaration(self):
        native = ledger.mappings()
        native["src/scan/scanner.ts:Scanner.syncScan"]["liveAcceptance"] = "verified"
        with mock.patch.object(ledger, "mappings", return_value=native):
            result = ledger.generate()
        self.assertEqual(result["summary"]["liveVerified"], 0)
        self.assertTrue(all(row["liveAcceptance"] == "unverified" for row in result["operations"]))

    def test_native_json_decoder_and_its_regressions_are_bound_core_inputs(self):
        result = ledger.generate()
        bindings = {row["path"]: row["sha256"] for row in result["sourceBindings"]}
        for name in ("crates/scm-sdk/src/json_decode.rs", "crates/scm-sdk/src/json_decode/tests.rs"):
            self.assertEqual(bindings[name], ledger.sha(ledger.ROOT / name))

    def test_reserved_json_restriction_is_an_exact_global_intentional_difference(self):
        result = ledger.generate()
        contract = result["jsonBoundaryContract"]
        self.assertEqual(contract, {
            "status": "intentional_rust_security_difference",
            "scope": "SDK JSON decoding and validated input boundaries: HTTP, SSE, realtime and model parsing, including programmatically constructed Value inputs.",
            "reservedObjectKeys": ["$serde_json::private::Number", "$serde_json::private::RawValue"],
            "matching": "Exact decoded object keys at any depth, including escaped spellings, independent of dependency features.",
            "action": "Reject before Serde protocol coercion; never silently alter or drop the object.",
            "errors": {"receivedJson": "Decode", "requestOrCallerValue": "Validation", "diagnostics": "sanitized"},
            "acceptedControls": "Near-match keys and reserved names used as ordinary string values retain normal handling.",
            "excludedScope": "Direct downstream serde_json deserialization bypasses SDK validation boundaries.",
            "documentation": ["docs/MODELS.md#reserved-json-protocol-keys", "docs/ADVERSARIAL.md#reserved-protocol-object-rejection"],
            "tests": [
                "crates/scm-sdk/src/json_decode/tests.rs::literal_reserved_protocol_keys_are_rejected_before_wire_value_coercion",
                "crates/scm-sdk/src/json_decode/tests.rs::programmatic_literal_protocol_objects_and_overdeep_values_fail_safely",
                "crates/scm-sdk/src/json_decode/tests.rs::realtime_reserved_objects_fail_before_inbound_coercion_or_outbound_write",
                "crates/scm-sdk/src/streaming/sse/tests.rs::protocol_named_literal_objects_fail_closed_in_event_data",
                "crates/scm-sdk/src/streaming/sse/tests.rs::reserved_number_objects_cannot_hide_a_truthy_error_envelope",
                "crates/scm-sdk/tests/gateway_runtime_contracts.rs::reserved_serde_protocol_objects_fail_closed_at_http_and_model_boundaries",
            ],
        })
        bindings = {row["path"] for row in result["sourceBindings"]}
        paths = {name.partition("#")[0] for name in contract["documentation"]}
        paths.update(name.partition("::")[0] for name in contract["tests"])
        self.assertTrue(paths <= bindings)
        self.assertTrue(all("jsonBoundaryContract" not in row for row in result["operations"]))
        self.assertEqual(result["summary"]["baselineConcreteMethods"], 448)

    def test_only_the_exact_receipted_method_is_live_verified(self):
        key = "src/scan/scanner.ts:Scanner.syncScan"
        annotation = {"liveAcceptance": "verified", "liveEvidence": {
            "path": "docs/checks/phase-08-live-attempt-01.json", "sha256": "a" * 64,
            "probe": "scan.sync_scan", "scope": "controlled_nonproduction",
            "runtimeInputsSha256": "b" * 64}}
        with mock.patch.object(ledger, "live_projection", return_value=({key: annotation}, set(), None)):
            result = ledger.generate()
        self.assertEqual(result["summary"]["liveVerified"], 1)
        self.assertEqual(result["summary"]["liveRecorded"], 1)
        row = next(row for row in result["operations"] if row["baselineId"] == key)
        self.assertEqual(row["liveEvidence"], annotation["liveEvidence"])
        self.assertEqual(sum("liveEvidence" in row for row in result["operations"]), 1)

    def test_invalid_or_stale_live_receipt_stops_generation(self):
        for reason in ("stale source", "receipt digest mismatch", "failed execution"):
            with mock.patch.object(ledger, "live_projection", side_effect=ValueError(reason)):
                with self.assertRaisesRegex(ValueError, reason):
                    ledger.generate()

    def test_live_annotation_cannot_certify_an_unimplemented_or_unknown_operation(self):
        annotation = {"liveAcceptance": "verified", "liveEvidence": {}}
        native = ledger.mappings()
        key = "src/scan/scanner.ts:Scanner.syncScan"
        native.pop(key)
        with mock.patch.object(ledger, "mappings", return_value=native), \
                mock.patch.object(ledger, "live_projection", return_value=({key: annotation}, set(), None)):
            with self.assertRaisesRegex(ValueError, "stale or unsupported"):
                ledger.generate()
        with mock.patch.object(ledger, "live_projection", return_value=({"invented": annotation}, set(), None)):
            with self.assertRaisesRegex(ValueError, "absent frozen operation"):
                ledger.generate()

    def test_only_explicit_realtime_and_provision_workflows_are_owned_in_phase_seven(self):
        rows = {row["baselineId"]: row for row in ledger.generate()["operations"]}
        gateway = [row for row in rows.values() if row["family"] == "ai-gateway"]
        self.assertEqual(len(gateway), 196)
        self.assertEqual(sum(row["owningPhase"] == 6 for row in gateway), 194)
        self.assertEqual({row["baselineId"] for row in gateway if row["owningPhase"] == 7},
                         set(ledger.OWNERSHIP_OVERRIDES))
        self.assertTrue(all(row["reason"] for row in ledger.OWNERSHIP_OVERRIDES.values()))

    def test_invented_or_unexplained_ownership_overrides_are_rejected(self):
        for overrides in ({"invented": {"phase": 7, "reason": "x"}},
                          {next(iter(ledger.OWNERSHIP_OVERRIDES)): {"phase": 7, "reason": ""}}):
            with mock.patch.object(ledger, "OWNERSHIP_OVERRIDES", overrides):
                with self.assertRaisesRegex(ValueError, "ownership override"):
                    ledger.generate()

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

    def test_realtime_header_contract_is_distinct_and_cannot_be_forged(self):
        native = ledger.mappings()
        key = "src/ai-gateway/inference-client.ts:AIGatewayInferenceClient.connectRealtime"
        row = next(row for row in ledger.generate()["operations"] if row["baselineId"] == key)
        self.assertEqual(row["headerContract"], "gateway_realtime")
        native[key]["headerContract"] = "airs_api_defaults"
        with mock.patch.object(ledger, "mappings", return_value=native):
            with self.assertRaisesRegex(ValueError, "protocol header contract"):
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
