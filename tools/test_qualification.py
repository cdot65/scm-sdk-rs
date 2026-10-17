"""Qualification cannot turn declarations, stale receipts, or failed probes into acceptance."""

import copy
import json
import pathlib
import tempfile
import unittest
from unittest import mock

import live_evidence
import native_operations
import qualification
from test_live_evidence import specimen


class QualificationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        root = qualification.ROOT
        cls.frozen = json.loads((root / "contracts/operations.json").read_text())
        cls.native = json.loads((root / "contracts/native-operations.json").read_text())
        cls.exports = json.loads((root / "contracts/native-exports.json").read_text())
        canonical = {row["baselineId"].split(":", 1)[1]: row["baselineId"]
                     for row in cls.native["operations"]}
        for row in cls.native["operations"] + cls.exports["capabilities"]:
            row["liveAcceptance"] = "unverified"
            row.pop("liveEvidence", None)
        # Five manually adapted OAuth methods also have exact canonical bindings.
        # This permits isolated tests before the final receipt-driven regeneration.
        for row in cls.exports["capabilities"]:
            if row["kind"] == "method" and row["baselineId"] in canonical:
                row["operationId"] = canonical[row["baselineId"]]

    def inputs(self):
        return copy.deepcopy(self.frozen), copy.deepcopy(self.native), copy.deepcopy(self.exports)

    def annotate(self, native, exports, status):
        key = "src/scan/scanner.ts:Scanner.syncScan"
        annotation = {"liveAcceptance": status, "liveEvidence": {
            "path": "docs/checks/phase-08-live-attempt-01.json", "sha256": "a" * 64,
            "probe": "scan.sync_scan", "scope": "controlled_nonproduction",
            "runtimeInputsSha256": "b" * 64}}
        next(row for row in native["operations"] if row["baselineId"] == key).update(annotation)
        for row in exports["capabilities"]:
            if row.get("operationId") == key:
                row.update(annotation)
        receipt = {"report": {"path": annotation["liveEvidence"]["path"], "sha256": "a" * 64}}
        return {key: annotation}, receipt

    def test_exhaustive_denominator_and_unexecuted_methods_stay_unqualified(self):
        result = qualification.project(*self.inputs(), {}, None)
        self.assertEqual(result["summary"]["operations"], 448)
        self.assertEqual(result["summary"]["implemented"], 448)
        self.assertEqual(result["summary"]["contractVerified"], 448)
        self.assertEqual(result["summary"]["liveVerified"], 0)
        self.assertEqual(result["summary"]["liveRecorded"], 0)
        self.assertEqual(result["summary"]["unexplainedGaps"], 0)
        self.assertEqual(sum(value["operations"] for value in result["families"].values()), 448)
        self.assertTrue(all(row["liveReason"] for row in result["operations"]))

    def test_global_security_difference_is_required_even_without_qualification_gaps(self):
        result = qualification.project(*self.inputs(), {}, None)
        self.assertEqual(result["jsonBoundaryContract"], native_operations.json_boundary_contract())
        self.assertEqual(result["summary"]["unexplainedGaps"], 0)
        for index in (1, 2):
            for mutation in ("missing", "different_key", "unsanitized_error"):
                inputs = self.inputs()
                if mutation == "missing":
                    inputs[index].pop("jsonBoundaryContract")
                elif mutation == "different_key":
                    inputs[index]["jsonBoundaryContract"]["reservedObjectKeys"] = ["other"]
                else:
                    inputs[index]["jsonBoundaryContract"]["errors"]["diagnostics"] = "raw body"
                with self.assertRaisesRegex(ValueError, "JSON boundary intentional difference"):
                    qualification.project(*inputs, {}, None)

    def test_one_verified_operation_never_promotes_its_sibling_or_family(self):
        frozen, native, exports = self.inputs()
        annotations, receipt = self.annotate(native, exports, "verified")
        result = qualification.project(frozen, native, exports, annotations, receipt)
        self.assertEqual(result["summary"]["liveVerified"], 1)
        self.assertEqual(result["families"]["scan"]["liveVerified"], 1)
        rows = {row["baselineId"]: row for row in result["operations"]}
        self.assertEqual(rows["src/scan/scanner.ts:Scanner.asyncScan"]["liveAcceptance"], "unverified")
        self.assertNotIn("liveEvidence", rows["src/scan/scanner.ts:Scanner.asyncScan"])

    def test_failed_receipted_probe_is_observed_but_never_verified(self):
        for status in ("service_contract_mismatch", "permission_prerequisite",
                       "service_reported_scan_failure", "not_attempted_missing_prerequisite",
                       "incomplete_or_uncorrelated_receipt"):
            frozen, native, exports = self.inputs()
            annotations, receipt = self.annotate(native, exports, status)
            result = qualification.project(frozen, native, exports, annotations, receipt)
            self.assertEqual(result["summary"]["liveVerified"], 0)
            self.assertEqual(result["summary"]["liveRecorded"], 1)
            self.assertEqual(result["summary"]["liveByStatus"][status], 1)

    def test_every_controlled_failure_class_has_an_explicit_explanation(self):
        # verified_absent is restricted to auxiliary cleanup proofs; it cannot
        # certify a frozen operation and therefore has no operation status.
        self.assertEqual(set(qualification.REASONS), live_evidence.CLASSIFICATIONS - {"verified_absent"})

    def test_rejected_request_reason_does_not_rule_out_a_native_defect(self):
        frozen, native, exports = self.inputs()
        annotations, receipt = self.annotate(native, exports, "service_contract_rejection")
        result = qualification.project(frozen, native, exports, annotations, receipt)
        row = next(row for row in result["operations"] if row["baselineId"] in annotations)
        self.assertIn("cause is unestablished", row["liveReason"])
        self.assertIn("native request-shape defect is not excluded", row["liveReason"])
        self.assertEqual(result["summary"]["liveVerified"], 0)

    def test_mixed_observations_require_reconciliation_and_cannot_be_verified(self):
        for status in ("verified", "permission_prerequisite"):
            frozen, native, exports = self.inputs()
            annotations, receipt = self.annotate(native, exports, status)
            next(iter(annotations.values()))["liveEvidence"]["reconciliationRequired"] = True
            if status == "verified":
                with self.assertRaisesRegex(ValueError, "require reviewed reconciliation"):
                    qualification.project(frozen, native, exports, annotations, receipt)
            else:
                result = qualification.project(frozen, native, exports, annotations, receipt)
                row = next(row for row in result["operations"] if row["baselineId"] in annotations)
                self.assertIn("new reviewed reconciliation receipt", row["liveReason"])
                self.assertEqual(result["summary"]["liveVerified"], 0)

    def test_constructor_and_non_server_export_cannot_claim_operation_receipt(self):
        frozen, native, exports = self.inputs()
        annotations, receipt = self.annotate(native, exports, "verified")
        root = next(row for row in exports["capabilities"] if row["baselineId"] == "Scanner")
        root.update(next(iter(annotations.values())))
        with self.assertRaisesRegex(ValueError, "Non-operation exports"):
            qualification.project(frozen, native, exports, annotations, receipt)
        root["operationId"] = next(iter(annotations))
        with self.assertRaisesRegex(ValueError, "Constructors and non-method"):
            qualification.project(frozen, native, exports, annotations, receipt)

    def test_unsupported_live_claim_and_missing_receipt_are_rejected(self):
        frozen, native, exports = self.inputs()
        annotations, receipt = self.annotate(native, exports, "verified")
        with self.assertRaisesRegex(ValueError, "require their validated native receipt"):
            qualification.project(frozen, native, exports, annotations, None)
        with self.assertRaisesRegex(ValueError, "stale or unsupported"):
            qualification.project(frozen, native, exports, {}, receipt)

    def test_method_omission_duplicate_and_option_gap_are_rejected(self):
        for mutation in ("omit", "duplicate", "partial"):
            frozen, native, exports = self.inputs()
            if mutation == "omit":
                native["operations"].pop()
            elif mutation == "duplicate":
                native["operations"][-1] = copy.deepcopy(native["operations"][0])
            else:
                native["operations"][0]["optionParity"] = "partial"
            with self.assertRaisesRegex(ValueError, "448 unique|option-parity gap"):
                qualification.project(frozen, native, exports, {}, None)

    def test_exports_must_bind_every_operation_and_match_its_live_result(self):
        frozen, native, exports = self.inputs()
        annotations, receipt = self.annotate(native, exports, "verified")
        row = next(row for row in exports["capabilities"]
                   if row.get("operationId") == next(iter(annotations)))
        row["liveAcceptance"] = "unverified"
        with self.assertRaisesRegex(ValueError, "differs from its canonical"):
            qualification.project(frozen, native, exports, annotations, receipt)
        frozen, native, exports = self.inputs()
        missing = next(row["operationId"] for row in exports["capabilities"] if "operationId" in row)
        for row in exports["capabilities"]:
            if row.get("operationId") == missing:
                row.pop("operationId")
        with self.assertRaisesRegex(ValueError, "explicit native export binding"):
            qualification.project(frozen, native, exports, {}, None)

    def test_unknown_live_classification_never_becomes_an_unexplained_gap(self):
        frozen, native, exports = self.inputs()
        annotations, receipt = self.annotate(native, exports, "invented_acceptance")
        with self.assertRaisesRegex(ValueError, "explained native receipt"):
            qualification.project(frozen, native, exports, annotations, receipt)

    def test_stale_or_failed_receipt_stops_projection_before_generation(self):
        for reason in ("stale native receipt", "failed native execution"):
            with mock.patch.object(native_operations, "generate", side_effect=ValueError(reason)):
                with self.assertRaisesRegex(ValueError, reason):
                    qualification.generate()

    def test_real_receipt_loader_binds_projection_and_rejects_stale_or_failed_execution(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / "contracts").mkdir()
            (root / "docs/checks").mkdir(parents=True)
            (root / "tools").mkdir()
            (root / "tools/live_evidence.py").write_text("synthetic-verifier-input")
            (root / "contracts/operations.json").write_text(json.dumps({"operations": [
                {"id": name} for name in sorted(set(live_evidence.PROBE_BINDINGS.values()))]}))
            path = root / "docs/checks/phase-08-live-attempt-01.json"
            envelope = {
                "schemaVersion": 1, "baselineCommit": live_evidence.BASELINE,
                "executedAt": "2026-10-16T18:00:00+00:00", "runtimeInputsSha256": "a" * 64,
                "binarySha256": "b" * 64, "toolchain": "1.99.0",
                "hostTarget": "x86_64-unknown-linux-musl", "sourceUnchanged": True,
                "exitCode": 0, "commandTemplate": [
                    "live_qualification", "--config", "[external-registration]", "--expect-tsg",
                    "[verified-external-scope]", "--output", "[private-report]",
                    "--recovery-output", "[private-recovery]", "--mutations"],
                "report": specimen(),
            }

            def save():
                path.write_text(json.dumps(envelope))
                reference = {"schemaVersion": 1, "baselineCommit": live_evidence.BASELINE,
                             "report": path.relative_to(root).as_posix(),
                             "reportSha256": live_evidence.digest(path)}
                (root / live_evidence.INPUT).write_text(json.dumps(reference))

            save()
            with mock.patch.object(live_evidence, "runtime_fingerprint", return_value="a" * 64):
                annotations, bound, receipt = native_operations.live_projection(root)
                self.assertEqual(sum(row["liveAcceptance"] == "verified" for row in annotations.values()), 1)
                self.assertEqual(bound, {"contracts/live-evidence.json", "tools/live_evidence.py",
                                        "docs/checks/phase-08-live-attempt-01.json"})
                self.assertEqual(receipt["report"]["sha256"], live_evidence.digest(path))
                for key, value in (("sourceUnchanged", False), ("exitCode", 1),
                                   ("runtimeInputsSha256", "c" * 64)):
                    previous = envelope[key]
                    envelope[key] = value
                    save()
                    with self.assertRaisesRegex(ValueError, "stale or lacks execution"):
                        native_operations.live_projection(root)
                    envelope[key] = previous
                save()
                path.write_text(path.read_text() + " ")
                with self.assertRaisesRegex(ValueError, "digest mismatch"):
                    native_operations.live_projection(root)

    def test_stale_projection_check_does_not_rewrite_the_artifact(self):
        with tempfile.TemporaryDirectory() as directory:
            output = pathlib.Path(directory) / "qualification.json"
            output.write_text("{}\n")
            value = qualification.project(*self.inputs(), {}, None)
            with mock.patch.object(qualification, "OUTPUT", output), \
                    mock.patch.object(qualification, "generate", return_value=value), \
                    mock.patch("sys.argv", ["qualification.py", "--check"]):
                with self.assertRaisesRegex(ValueError, "projection is stale"):
                    qualification.main()
            self.assertEqual(output.read_text(), "{}\n")


if __name__ == "__main__":
    unittest.main()
