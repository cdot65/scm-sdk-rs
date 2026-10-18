"""Sanitized native receipts must not promote unexecuted or failed workflows."""
import copy
import json
import pathlib
import tempfile
import unittest
from unittest import mock

import live_evidence as live


def specimen():
    return {"schemaVersion": 1, "tenantScopeMatched": True, "client": "native_scm_sdk",
        "crateVersion": live.crate_version(), "automaticRetries": 0, "maximumTransportAttempts": 80,
        "perOperationTimeoutSeconds": 15, "scanAuthentication": None, "mutationsEnabled": True,
        "cleanupState": "not_needed", "serviceRequests": 1, "tokenExchanges": 1,
        "probes": [{"operation": "iam.scopes.list", "classification": "verified", "status": 200,
            "errorKind": None, "remoteOutcome": None, "workflowAccepted": True}],
        "cleanup": [], "limitations": live.LIMITATIONS}


class LiveEvidenceTests(unittest.TestCase):
    def test_report_version_is_bound_to_the_selected_crate_manifest(self):
        report = specimen()
        live.validate_report(report)
        for wrong in ["0.0.0", "0.1.0" if live.crate_version() != "0.1.0" else "0.1.1", None, True]:
            value = copy.deepcopy(report)
            value["crateVersion"] = wrong
            with self.subTest(version=wrong), self.assertRaises(ValueError):
                live.validate_report(value)
        historical = copy.deepcopy(report)
        historical["crateVersion"] = "0.1.0"
        live.validate_report(historical, expected_version="0.1.0")

    def annotations(self, report, expected_version=None):
        live.validate_report(report, expected_version=expected_version)
        return live.method_annotations(report, {"path": "synthetic-receipt", "sha256": "a" * 64,
            "scope": "controlled_nonproduction", "runtimeInputsSha256": "b" * 64})

    def failure(self, operation="iam.scopes.list", classification="upstream_failure", status=503):
        return {"operation": operation, "classification": classification, "status": status,
            "errorKind": "http", "remoteOutcome": "rejected", "workflowAccepted": False}

    def test_success_cannot_hide_a_failure_in_either_observation_order(self):
        for failure_first in (False, True):
            report = specimen()
            success = report["probes"][0]
            failure = self.failure()
            report["probes"] = [failure, success] if failure_first else [success, failure]
            annotations = self.annotations(report)
            row = annotations[live.PROBE_BINDINGS["iam.scopes.list"]]
            self.assertEqual(row["liveAcceptance"], "upstream_failure")
            self.assertTrue(row["liveEvidence"]["reconciliationRequired"])
            observations = row["liveEvidence"]["observations"]
            self.assertEqual([item["workflowAccepted"] for item in observations],
                             [False, True] if failure_first else [True, False])
            self.assertEqual([item["index"] for item in observations], [0, 1])

    def test_cleanup_failure_or_success_never_hides_its_conflicting_probe(self):
        for cleanup_succeeds in (False, True):
            report = specimen()
            success = report["probes"][0]
            failure = self.failure()
            report["probes"] = [failure if cleanup_succeeds else success]
            report["cleanup"] = [success if cleanup_succeeds else failure]
            row = self.annotations(report)[live.PROBE_BINDINGS["iam.scopes.list"]]
            self.assertEqual(row["liveAcceptance"], "upstream_failure")
            self.assertTrue(row["liveEvidence"]["reconciliationRequired"])
            self.assertEqual([item["section"] for item in row["liveEvidence"]["observations"]],
                             ["probes", "cleanup"])

    def test_conflicting_failures_are_retained_and_summary_is_order_independent(self):
        for reverse in (False, True):
            report = specimen()
            failures = [self.failure(), self.failure(classification="service_contract_mismatch", status=200)]
            report["probes"] = list(reversed(failures)) if reverse else failures
            row = self.annotations(report)[live.PROBE_BINDINGS["iam.scopes.list"]]
            self.assertEqual(row["liveAcceptance"], "service_contract_mismatch")
            self.assertTrue(row["liveEvidence"]["reconciliationRequired"])
            self.assertEqual(len(row["liveEvidence"]["observations"]), 2)
        self.assertEqual(set(live.FAILURE_PRIORITY), live.CLASSIFICATIONS - {"verified", "verified_absent"})

    def test_consistent_alias_observations_are_retained_without_family_promotion(self):
        report = specimen()
        success = report["probes"][0]
        report["probes"] = [dict(success, operation="scan.query_scan_results"),
                            dict(success, operation="scan.query_async_results")]
        report["cleanup"] = [dict(success, operation="iam.scopes.verify_absent",
                                  classification="verified_absent", status=404)]
        annotations = self.annotations(report)
        self.assertEqual(set(annotations), {live.PROBE_BINDINGS["scan.query_scan_results"]})
        row = next(iter(annotations.values()))
        self.assertEqual(row["liveAcceptance"], "verified")
        self.assertFalse(row["liveEvidence"]["reconciliationRequired"])
        self.assertEqual(len(row["liveEvidence"]["observations"]), 2)

    def test_repeated_failures_never_gain_acceptance(self):
        report = specimen()
        report["probes"] = [self.failure(), self.failure()]
        row = self.annotations(report)[live.PROBE_BINDINGS["iam.scopes.list"]]
        self.assertEqual(row["liveAcceptance"], "upstream_failure")
        self.assertFalse(row["liveEvidence"]["reconciliationRequired"])
        self.assertEqual(len(row["liveEvidence"]["observations"]), 2)

    def test_rejection_diagnostics_accept_only_finite_codes_and_known_field_names(self):
        for code in (None, *sorted(live.REJECTION_CODES)):
            report = specimen()
            rejection = self.failure(classification="service_contract_rejection", status=400)
            rejection["rejectionDetails"] = {"code": code, "fields": ["name", "profile_type"]}
            report["probes"] = [rejection]
            row = self.annotations(report)[live.PROBE_BINDINGS["iam.scopes.list"]]
            self.assertEqual(row["liveEvidence"]["observations"][0]["rejectionDetails"],
                             rejection["rejectionDetails"])
        self.assertEqual(len(live.REJECTION_CODES), 13)
        self.assertEqual(len(live.REJECTION_FIELDS), 22)

    def test_rejection_messages_values_unknown_codes_names_and_extra_keys_are_rejected(self):
        for details in (
            {"code": "synthetic-secret-echo", "fields": []},
            {"code": None, "fields": ["synthetic-secret-echo"]},
            {"code": None, "fields": ["[withheld]"]},
            {"code": None, "fields": ["name", "name"]},
            {"code": None, "fields": sorted(live.REJECTION_FIELDS)},
            {"code": None, "fields": [], "message": "synthetic-secret-echo"},
            {"code": None, "fields": [], "values": ["synthetic-secret-echo"]},
            {"code": None, "fields": [{"name": "name", "value": "synthetic-secret-echo"}]},
            {"code": [], "fields": []}, {"code": True, "fields": []},
            {"code": None, "fields": None}, {"code": None}, None,
        ):
            report = specimen()
            rejection = self.failure(classification="service_contract_rejection", status=400)
            rejection["rejectionDetails"] = details
            report["probes"] = [rejection]
            with self.assertRaises(ValueError):
                live.validate_report(report)
        report = specimen()
        report["probes"][0]["rejectionDetails"] = {"code": None, "fields": []}
        with self.assertRaisesRegex(ValueError, "Rejection details require|Accepted workflows"):
            live.validate_report(report)
        for changes in ({"status": 200}, {"status": None}, {"status": 503},
                        {"classification": "unexpected_client_failure"}):
            report = specimen()
            rejection = self.failure(classification="service_contract_rejection", status=400)
            rejection.update(changes)
            rejection["rejectionDetails"] = {"code": None, "fields": []}
            report["probes"] = [rejection]
            with self.assertRaisesRegex(ValueError, "Rejection details require"):
                live.validate_report(report)

    def test_historical_attempts_one_through_five_remain_schema_valid(self):
        for attempt in range(1, 6):
            path = live.ROOT / f"docs/checks/phase-08-live-attempt-{attempt:02d}.json"
            report = json.loads(path.read_text())["report"]
            annotations = self.annotations(report, expected_version="0.1.0")
            self.assertTrue(annotations)
            self.assertFalse(any(row["liveEvidence"]["reconciliationRequired"] for row in annotations.values()))
            scan = annotations[live.PROBE_BINDINGS["scan.query_scan_results"]]
            self.assertEqual(len(scan["liveEvidence"]["observations"]), 2)

    def test_only_allowlisted_metadata_is_exported(self):
        for field in ("apiKey", "tenantRegistration", "body", "headers", "requestId", "prompt"):
            value = specimen()
            value[field] = "synthetic-sensitive-value"
            with self.assertRaisesRegex(ValueError, "Unexpected receipt fields"):
                live.validate_report(value)

    def test_unrecognized_upstream_words_and_probe_names_are_rejected(self):
        for field in ("operation", "classification", "errorKind", "remoteOutcome"):
            value = specimen()
            value["probes"][0][field] = "synthetic-secret-echo"
            with self.assertRaises(ValueError):
                live.validate_report(value)

    def test_failed_workflows_cannot_be_promoted_by_a_flag(self):
        for status in (None, 101, 302, 401, 403, 429, 500):
            value = specimen()
            value["probes"][0]["status"] = status
            with self.assertRaisesRegex(ValueError, "successful native receipt"):
                live.validate_report(value)

    def test_zero_requests_limits_scope_and_numeric_types_are_enforced(self):
        for field, replacement in (("tenantScopeMatched", False), ("automaticRetries", 1),
            ("schemaVersion", True), ("serviceRequests", True), ("serviceRequests", 81), ("maximumTransportAttempts", 81),
            ("perOperationTimeoutSeconds", 0), ("scanAuthentication", "private-token")):
            value = specimen()
            value[field] = replacement
            with self.assertRaises(ValueError):
                live.validate_report(value)

    def test_absence_proofs_only_certify_owned_cleanup_checks(self):
        value = specimen()
        value["probes"][0].update(classification="verified_absent", status=404)
        with self.assertRaisesRegex(ValueError, "unrelated workflow"):
            live.validate_report(value)
        value["probes"][0]["operation"] = "iam.scopes.verify_absent"
        live.validate_report(value)
        value["probes"][0]["status"] = 200
        live.validate_report(value)

    def test_accepted_and_failed_classifications_are_consistent(self):
        for changes in ({"workflowAccepted": False}, {"errorKind": "decode"},
                        {"remoteOutcome": "unknown"}, {"classification": "upstream_failure"}):
            value = specimen()
            value["probes"][0].update(changes)
            with self.assertRaises(ValueError):
                live.validate_report(value)

    def test_request_and_cleanup_consistency_cannot_be_forged(self):
        value = specimen()
        value.update(serviceRequests=0, tokenExchanges=0)
        with self.assertRaisesRegex(ValueError, "transport attempt"):
            live.validate_report(value)
        value = specimen()
        value["cleanupState"] = "confirmed_retired"
        with self.assertRaisesRegex(ValueError, "absence receipt"):
            live.validate_report(value)
        value["cleanupState"] = "not_needed"
        value["cleanup"] = [{"operation": "gateway.provision.recovery",
            "classification": "retained_for_reconciliation", "status": None,
            "errorKind": None, "remoteOutcome": "unknown", "workflowAccepted": False}]
        with self.assertRaisesRegex(ValueError, "must remain explicit"):
            live.validate_report(value)

    def test_stale_source_digest_and_path_tampering_are_rejected(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            (root / "crates/scm-sdk").mkdir(parents=True)
            (root / "crates/scm-sdk/Cargo.toml").write_text(
                '[package]\nversion="' + live.crate_version() + '"\n')
            (root / "contracts").mkdir()
            (root / "docs/checks").mkdir(parents=True)
            (root / "contracts/operations.json").write_text(json.dumps({"operations": [
                {"id": name} for name in sorted(set(live.PROBE_BINDINGS.values()))]}))
            report_path = root / "docs/checks/phase-08-live-attempt-01.json"
            template = ["live_qualification", "--config", "[external-registration]", "--expect-tsg",
                "[verified-external-scope]", "--output", "[private-report]", "--recovery-output", "[private-recovery]", "--mutations"]
            envelope = {"schemaVersion": 1, "baselineCommit": live.BASELINE,
                "executedAt": "2026-10-16T18:00:00+00:00", "runtimeInputsSha256": "a" * 64,
                "binarySha256": "b" * 64, "toolchain": "1.99.0", "hostTarget": "x86_64-unknown-linux-musl",
                "sourceUnchanged": True, "exitCode": 0, "commandTemplate": template, "report": specimen()}

            def save(value):
                report_path.write_text(json.dumps(value))
                ref = {"schemaVersion": 1, "baselineCommit": live.BASELINE,
                    "report": report_path.relative_to(root).as_posix(), "reportSha256": live.digest(report_path)}
                (root / live.INPUT).write_text(json.dumps(ref))

            save(envelope)
            with mock.patch.object(live, "runtime_fingerprint", return_value="a" * 64):
                annotations, _ = live.load(root, required=True)
                self.assertEqual(sum(v["liveAcceptance"] == "verified" for v in annotations.values()), 1)
                for failure_first in (False, True):
                    changed = copy.deepcopy(envelope)
                    success = changed["report"]["probes"][0]
                    failure = self.failure()
                    changed["report"]["probes"] = ([failure, success] if failure_first else [success, failure])
                    save(changed)
                    annotations, _ = live.load(root, required=True)
                    observed = annotations[live.PROBE_BINDINGS["iam.scopes.list"]]
                    self.assertEqual(observed["liveAcceptance"], "upstream_failure")
                    self.assertTrue(observed["liveEvidence"]["reconciliationRequired"])
                    self.assertEqual(len(observed["liveEvidence"]["observations"]), 2)
                for key, value in (("sourceUnchanged", False), ("exitCode", 1),
                                   ("runtimeInputsSha256", "c" * 64), ("binarySha256", "invalid")):
                    changed = copy.deepcopy(envelope)
                    changed[key] = value
                    save(changed)
                    with self.assertRaisesRegex(ValueError, "stale or lacks execution"):
                        live.load(root, required=True)
                save(envelope)
                report_path.write_text(report_path.read_text() + " ")
                with self.assertRaisesRegex(ValueError, "digest mismatch"):
                    live.load(root, required=True)
                save(envelope)
                ref = json.loads((root / live.INPUT).read_text())
                ref["report"] = "../../private.json"
                (root / live.INPUT).write_text(json.dumps(ref))
                with self.assertRaisesRegex(ValueError, "Unexpected controlled report path"):
                    live.load(root, required=True)


if __name__ == "__main__":
    unittest.main()
