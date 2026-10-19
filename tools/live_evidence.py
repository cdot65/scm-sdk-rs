#!/usr/bin/env python3
"""Execute explicitly selected native qualification or verify its sanitized receipt.

Ordinary verification never contacts a service. Digests detect stale/tampered
receipts; they do not authenticate a process against a malicious repository writer.
"""
import argparse
import datetime
import hashlib
import json
import os
import pathlib
import re
import subprocess
import tempfile
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
INPUT = pathlib.Path("contracts/live-evidence.json")
BASELINE = "4a80dbdb3dc14365e1c25d838ac0bd139485d48c"
PROBE_BINDINGS = {
    "oauth.get_token": "src/management/oauth-client.ts:OAuthClient.getToken",
    "management.profiles.list_for_token": "src/management/profiles.ts:ProfilesClient.listForToken",
    "management.profiles.list": "src/management/profiles.ts:ProfilesClient.list",
    "management.api_keys.list_for_token": "src/management/api-keys.ts:ApiKeysClient.listForToken",
    "dlp.data_profiles.list": "src/management/dlp/data-profiles.ts:DataProfilesClient.list",
    "model_security.models.list_models": "src/model-security/models-client.ts:ModelSecurityModelsClient.listModels",
    "model_security.scans.list": "src/model-security/scans-client.ts:ModelSecurityScansClient.list",
    "model_security.rules.list": "src/model-security/security-rules-client.ts:ModelSecurityRulesClient.list",
    "red_team.eula.get_status": "src/red-team/eula-client.ts:RedTeamEulaClient.getStatus",
    "red_team.get_scan_metadata": "src/red-team/client.ts:RedTeamClient.getScanMetadata",
    "red_team.get_management_languages": "src/red-team/client.ts:RedTeamClient.getManagementLanguages",
    "agentguard.list_scans": "src/agentguard/client.ts:AgentGuardClient.listScans",
    "agentguard.get_scan_stats": "src/agentguard/client.ts:AgentGuardClient.getScanStats",
    "agentguard.list_rules": "src/agentguard/client.ts:AgentGuardClient.listRules",
    "gateway.workspaces.list": "src/ai-gateway/workspaces-client.ts:AIGatewayWorkspacesClient.list",
    "gateway.workspaces.provision": "src/ai-gateway/workspaces-client.ts:AIGatewayWorkspacesClient.provision",
    "gateway.workspaces.get": "src/ai-gateway/workspaces-client.ts:AIGatewayWorkspacesClient.get",
    "gateway.workspaces.delete": "src/ai-gateway/workspaces-client.ts:AIGatewayWorkspacesClient.delete",
    "iam.scopes.list": "src/iam/scopes-client.ts:IamScopesClient.list",
    "iam.scopes.get": "src/iam/scopes-client.ts:IamScopesClient.get",
    "iam.scopes.create": "src/iam/scopes-client.ts:IamScopesClient.create",
    "iam.scopes.update": "src/iam/scopes-client.ts:IamScopesClient.update",
    "iam.scopes.delete": "src/iam/scopes-client.ts:IamScopesClient.delete",
    "scan.sync_scan": "src/scan/scanner.ts:Scanner.syncScan",
    "scan.async_scan": "src/scan/scanner.ts:Scanner.asyncScan",
    "scan.query_scan_results": "src/scan/scanner.ts:Scanner.queryByScanIds",
    "scan.query_async_results": "src/scan/scanner.ts:Scanner.queryByScanIds",
    "scan.query_report": "src/scan/scanner.ts:Scanner.queryByReportIds",
}
PROFILING_PROBES = (
    "red_team.targets.list.profiling_not_started",
    "red_team.targets.list.profiling_in_progress",
    "red_team.targets.list.profiling_completed",
    "red_team.targets.list.profiling_failed",
)
PROBE_BINDINGS.update({operation: "src/red-team/targets-client.ts:RedTeamTargetsClient.list"
                       for operation in PROFILING_PROBES})
AUXILIARY = {
    "iam.scopes.owned_fixture_preflight", "iam.scopes.verify_absent", "iam.scopes.reconcile",
    "gateway.provision.owned_fixture_preflight", "gateway.provision.recovery",
    "gateway.workspaces.verify_archived", "iam.provision_scope.clear_owned_binding",
}
CLASSIFICATIONS = {
    "verified", "verified_absent", "authentication_prerequisite", "permission_prerequisite",
    "not_found_or_unavailable", "rate_limit_prerequisite", "upstream_failure",
    "network_prerequisite", "service_contract_mismatch", "unexpected_client_failure",
    "not_attempted_missing_prerequisite", "not_attempted_existing_resource",
    "service_contract_rejection", "service_reported_scan_failure",
    "retained_for_reconciliation", "archive_not_confirmed",
    "incomplete_or_uncorrelated_receipt",
}
ERROR_KINDS = {"configuration", "validation", "destination", "authentication", "transport",
               "http", "decode", "protocol", "limit", "cancelled", "deadline", "io", "pagination", "unknown"}
REJECTION_CODES = {
    "AISEC_SERVER_SIDE_ERROR", "AISEC_CLIENT_SIDE_ERROR", "AISEC_USER_REQUEST_PAYLOAD_ERROR",
    "AISEC_MISSING_VARIABLE", "AISEC_SDK_ERROR", "AISEC_OAUTH_ERROR", "AISEC_RESPONSE_VALIDATION",
    "invalid_request", "invalid_client", "invalid_grant", "unauthorized_client",
    "unsupported_grant_type", "invalid_scope",
}
REJECTION_FIELDS = {
    "name", "category", "description", "type", "classification", "tags", "file", "json",
    "originalFileName", "original_file_name", "regionName", "region_name", "isCaseSensitive",
    "is_case_sensitive", "profileType", "profile_type", "detectionRules", "detection_rules",
    "dataPatterns", "data_patterns", "supportedConfidenceLevels", "supported_confidence_levels",
}
# Select a conservative summary without losing any observation. Uncertain
# cleanup and decoding/workflow failures precede prerequisite and skipped cases.
FAILURE_PRIORITY = (
    "retained_for_reconciliation", "archive_not_confirmed", "incomplete_or_uncorrelated_receipt",
    "unexpected_client_failure", "service_contract_mismatch", "service_reported_scan_failure",
    "network_prerequisite", "upstream_failure", "service_contract_rejection",
    "authentication_prerequisite", "permission_prerequisite", "rate_limit_prerequisite",
    "not_found_or_unavailable", "not_attempted_missing_prerequisite", "not_attempted_existing_resource",
)
LIMITATIONS = [
    "Only the explicitly recorded workflows were attempted; remaining methods are live-unqualified.",
    "Synthetic scans retain server audit records; the frozen scan API has no deletion method.",
    "Workspace deletion is archival; permanent deletion is unavailable in the frozen API.",
]

READ_ONLY_LIMITATIONS = [
    "Read-only qualification attempts OAuth and four single-page target lists with limit 1; all other workflows remain live-unqualified.",
    "Filter acceptance means HTTP success and native typed decoding only; filtering effectiveness is not established, including for empty or ambiguous results.",
    "Vertex AI, client_session and audio workflows remain experimental and live-unverified without provider credentials, fixtures and execution prerequisites.",
]


def command_template(read_only=False):
    return ["live_qualification", "--config", "[external-registration]", "--expect-tsg",
            "[verified-external-scope]", "--output", "[private-report]", "--recovery-output",
            "[private-recovery]", "--read-only" if read_only else "--mutations"]


def digest(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def runtime_fingerprint(root=ROOT):
    """Bind executable SDK source/dependencies, excluding evidence and prose."""
    paths = {root / "Cargo.toml", root / "Cargo.lock", root / "LICENSE", root / "NOTICE",
             root / "crates/scm-sdk/Cargo.toml"}
    for name in ("src", "examples"):
        paths.update(p for p in (root / "crates/scm-sdk" / name).rglob("*") if p.is_file())
    sha = hashlib.sha256()
    for path in sorted(paths):
        if path.is_symlink():
            raise ValueError("Runtime inputs must be regular files")
        sha.update(path.relative_to(root).as_posix().encode() + b"\0" + bytes.fromhex(digest(path)))
    return sha.hexdigest()


def regular(root, name):
    path = pathlib.PurePosixPath(name)
    if path.is_absolute() or ".." in path.parts or "\\" in name:
        raise ValueError("Receipt paths must stay in the repository")
    absolute = root / path
    if any(p.is_symlink() for p in (absolute, *absolute.parents)) or not absolute.is_file():
        raise ValueError("Receipt inputs must be regular files")
    return absolute


def require_keys(value, keys, optional=()):
    if not isinstance(value, dict) or not set(keys) <= set(value) <= set(keys) | set(optional):
        raise ValueError("Unexpected receipt fields; unreviewed data must not be exported")


def crate_version(root=ROOT):
    manifest = tomllib.loads(regular(root, "crates/scm-sdk/Cargo.toml").read_text())
    version = manifest["package"]["version"]
    if not isinstance(version, str) or not re.fullmatch(
        r"\d+\.\d+\.\d+(?:-[0-9A-Za-z.-]+)?(?:\+[0-9A-Za-z.-]+)?", version
    ):
        raise ValueError("Qualification requires an explicit SDK package version")
    return version


def validate_report(report, *, expected_version=None):
    expected_version = crate_version() if expected_version is None else expected_version
    require_keys(report, {"schemaVersion", "tenantScopeMatched", "client", "crateVersion",
        "automaticRetries", "maximumTransportAttempts", "perOperationTimeoutSeconds", "scanAuthentication",
        "mutationsEnabled", "cleanupState", "probes", "cleanup", "serviceRequests", "tokenExchanges", "limitations"})
    read_only = report["schemaVersion"] == 2
    budget = 5 if read_only else 80
    if not (type(report["schemaVersion"]) is int and report["schemaVersion"] in (1, 2) and report["tenantScopeMatched"] is True
            and report["client"] == "native_scm_sdk" and report["crateVersion"] == expected_version
            and type(report["automaticRetries"]) is int and report["automaticRetries"] == 0
            and type(report["maximumTransportAttempts"]) is int and report["maximumTransportAttempts"] == budget
            and type(report["perOperationTimeoutSeconds"]) is int and report["perOperationTimeoutSeconds"] == 15
            and type(report["mutationsEnabled"]) is bool
            and report["cleanupState"] in {"not_needed", "confirmed_retired", "requires_reconciliation"}
            and report["scanAuthentication"] in {None, "api_key_hmac", "oauth_bearer"}
            and report["limitations"] == (READ_ONLY_LIMITATIONS if read_only else LIMITATIONS)):
        raise ValueError("Invalid qualification controls or unreviewed report vocabulary")
    for key in ("serviceRequests", "tokenExchanges"):
        if type(report[key]) is not int or not 0 <= report[key] <= budget:
            raise ValueError("Invalid request count")
    if report["serviceRequests"] + report["tokenExchanges"] > budget:
        raise ValueError("Qualification exceeded the transport budget")
    for key in ("probes", "cleanup"):
        if not isinstance(report[key], list) or len(report[key]) > 80:
            raise ValueError("Invalid bounded probe collection")
        for row in report[key]:
            require_keys(row, {"operation", "classification", "status", "errorKind", "remoteOutcome", "workflowAccepted"},
                         {"rejectionDetails"})
            if row["operation"] not in PROBE_BINDINGS.keys() | AUXILIARY:
                raise ValueError("Unknown probe; no arbitrary operation text is exported")
            if row["classification"] not in CLASSIFICATIONS or type(row["workflowAccepted"]) is not bool:
                raise ValueError("Unreviewed probe classification")
            status = row["status"]
            if status is not None and (type(status) is not int or not 100 <= status <= 599):
                raise ValueError("Invalid HTTP status")
            if row["errorKind"] is not None and row["errorKind"] not in ERROR_KINDS:
                raise ValueError("Unreviewed error kind")
            if row["remoteOutcome"] not in {None, "not_sent", "rejected", "unknown"}:
                raise ValueError("Unreviewed remote outcome")
            if "rejectionDetails" in row:
                details = row["rejectionDetails"]
                require_keys(details, {"code", "fields"})
                if (row["classification"] != "service_contract_rejection"
                        or status not in {400, 405, 409, 422}):
                    raise ValueError("Rejection details require a rejected submitted request")
                if (details["code"] is not None
                        and (not isinstance(details["code"], str) or details["code"] not in REJECTION_CODES)):
                    raise ValueError("Unreviewed rejection code")
                fields = details["fields"]
                if (not isinstance(fields, list) or len(fields) > 20
                        or any(not isinstance(field, str) or field not in REJECTION_FIELDS for field in fields)
                        or len(fields) != len(set(fields))):
                    raise ValueError("Unreviewed or unbounded rejection field names")
                if row["workflowAccepted"]:
                    raise ValueError("Accepted workflows cannot carry rejection details")
            if row["workflowAccepted"]:
                if row["classification"] == "verified_absent":
                    if row["operation"] not in AUXILIARY or status not in {200, 404}:
                        raise ValueError("An absence proof cannot certify an unrelated workflow")
                elif not (row["classification"] == "verified" and status is not None and 200 <= status <= 299):
                    raise ValueError("Workflow verification requires a successful native receipt")
                if row["errorKind"] is not None or row["remoteOutcome"] is not None:
                    raise ValueError("Failed workflows cannot be marked accepted")
            elif row["classification"] in {"verified", "verified_absent"}:
                raise ValueError("Successful classifications require acceptance evidence")
    if not report["probes"]:
        raise ValueError("Qualification must actually attempt a workflow")
    if report["serviceRequests"] + report["tokenExchanges"] == 0:
        raise ValueError("Qualification must perform a native transport attempt")
    if report["cleanupState"] == "confirmed_retired" and not any(
            row["operation"] == "iam.scopes.verify_absent" and row["workflowAccepted"]
            for row in report["cleanup"]):
        raise ValueError("Confirmed cleanup requires an owned-resource absence receipt")
    if report["cleanupState"] != "requires_reconciliation" and any(
            row["classification"] in {"retained_for_reconciliation", "archive_not_confirmed"}
            for row in report["cleanup"]):
        raise ValueError("Unresolved resource cleanup must remain explicit")
    if read_only:
        if (report["mutationsEnabled"] is not False or report["scanAuthentication"] is not None
                or report["cleanupState"] != "not_needed" or report["cleanup"]
                or [row["operation"] for row in report["probes"]] != ["oauth.get_token", *PROFILING_PROBES]):
            raise ValueError("Read-only qualification requires exactly OAuth and four filter observations without mutation or cleanup")
        accepted = sum(row["workflowAccepted"] for row in report["probes"][1:])
        if accepted > report["serviceRequests"] or not 1 <= report["tokenExchanges"] <= budget:
            raise ValueError("Read-only accepted probes require corresponding transport attempts")
    elif any(row["operation"] in PROFILING_PROBES for row in report["probes"] + report["cleanup"]):
        raise ValueError("Profiling observations require the bounded read-only qualification schema")
    return report


def method_annotations(report, evidence):
    """Retain all observations; conflicts require a new reviewed reconciliation receipt."""
    grouped = {}
    for section in ("probes", "cleanup"):
        for index, row in enumerate(report[section]):
            baseline_id = PROBE_BINDINGS.get(row["operation"])
            if baseline_id is None:
                continue
            observation = {"section": section, "index": index, **row}
            if "rejectionDetails" in row:
                observation["rejectionDetails"] = {
                    "code": row["rejectionDetails"]["code"],
                    "fields": list(row["rejectionDetails"]["fields"]),
                }
            grouped.setdefault(baseline_id, []).append(observation)
    annotations = {}
    for baseline_id, observations in grouped.items():
        failures = [row for row in observations if not row["workflowAccepted"]]
        selected = min(failures, key=lambda row: FAILURE_PRIORITY.index(row["classification"])) if failures else observations[0]
        conflicting_failures = len({(row["classification"], row["status"], row["errorKind"], row["remoteOutcome"])
                                    for row in failures}) > 1
        mixed = bool(failures) and (len(failures) != len(observations) or conflicting_failures)
        annotations[baseline_id] = {
            "liveAcceptance": selected["classification"] if failures else "verified",
            "liveEvidence": {**evidence, "probe": selected["operation"],
                "observations": observations, "reconciliationRequired": mixed},
        }
    return annotations


def load(root=ROOT, required=False):
    if not (root / INPUT).exists():
        if required:
            raise ValueError("Controlled native qualification has no retained receipt")
        return {}, None
    reference = json.loads(regular(root, INPUT.as_posix()).read_text())
    require_keys(reference, {"schemaVersion", "baselineCommit", "report", "reportSha256"})
    if type(reference["schemaVersion"]) is not int or reference["schemaVersion"] != 1 or reference["baselineCommit"] != BASELINE:
        raise ValueError("Qualification baseline mismatch")
    if not re.fullmatch(r"docs/checks/phase-(?:08|24)-live-attempt-\d{2}\.json", reference["report"]):
        raise ValueError("Unexpected controlled report path")
    path = regular(root, reference["report"])
    if digest(path) != reference["reportSha256"]:
        raise ValueError("Controlled receipt digest mismatch")
    envelope = json.loads(path.read_text())
    require_keys(envelope, {"schemaVersion", "baselineCommit", "executedAt", "runtimeInputsSha256",
        "binarySha256", "toolchain", "hostTarget", "sourceUnchanged", "exitCode", "commandTemplate", "report"})
    if not (type(envelope["schemaVersion"]) is int and envelope["schemaVersion"] == 1 and envelope["baselineCommit"] == BASELINE
            and envelope["sourceUnchanged"] is True and type(envelope["exitCode"]) is int and envelope["exitCode"] == 0
            and envelope["toolchain"] == "1.99.0" and envelope["hostTarget"] == "x86_64-unknown-linux-musl"
            and envelope["runtimeInputsSha256"] == runtime_fingerprint(root)
            and re.fullmatch(r"[0-9a-f]{64}", envelope["binarySha256"])):
        raise ValueError("Native qualification is stale or lacks execution evidence")
    executed_at = datetime.datetime.fromisoformat(envelope["executedAt"])
    if executed_at.tzinfo is None or executed_at.utcoffset() != datetime.timedelta(0):
        raise ValueError("Receipt execution time must be explicit UTC")
    read_only = envelope["report"].get("schemaVersion") == 2
    if reference["report"].startswith("docs/checks/phase-24-") != read_only:
        raise ValueError("Controlled report phase and qualification mode differ")
    if envelope["commandTemplate"] != command_template(read_only):
        raise ValueError("Unreviewed or credential-bearing invocation metadata")
    report = validate_report(envelope["report"], expected_version=crate_version(root))
    frozen_ids = {v["id"] for v in json.loads((root / "contracts/operations.json").read_text())["operations"]}
    if set(PROBE_BINDINGS.values()) - frozen_ids:
        raise ValueError("Controlled probe binding is absent from the frozen inventory")
    annotations = method_annotations(report, {"path": reference["report"], "sha256": reference["reportSha256"],
        "scope": "controlled_nonproduction", "runtimeInputsSha256": envelope["runtimeInputsSha256"]})
    return annotations, envelope


def run(args):
    """Only this explicitly selected mode performs service requests."""
    read_only = getattr(args, "read_only", False)
    if not args.config or not args.expect_tsg or args.mutations == read_only or not 1 <= args.attempt <= 99:
        raise ValueError("Select an external config, expected nonproduction scope, exactly one of --mutations/--read-only and attempt 1–99")
    cargo = pathlib.Path.home() / ".cargo/bin/cargo"
    rustc = cargo.with_name("rustc")
    toolchain = "1.99.0"
    native = subprocess.check_output([str(rustc), f"+{toolchain}", "-vV"], text=True)
    host = next(line[6:] for line in native.splitlines() if line.startswith("host: "))
    if host != "x86_64-unknown-linux-musl":
        raise ValueError("This controlled receipt currently qualifies the recorded musl host only")
    phase = "24" if read_only else "08"
    report_path = ROOT / f"docs/checks/phase-{phase}-live-attempt-{args.attempt:02d}.json"
    if report_path.exists():
        raise ValueError("Preserve previous attempts and select a new attempt number")
    env = {k: v for k, v in os.environ.items() if not k.startswith(("PANW_", "AIRS_", "SCM_"))}
    before = runtime_fingerprint()
    build = [str(cargo), f"+{toolchain}", "build", "--locked", "--all-features", "--target", host,
             "--example", "live_qualification"]
    result = subprocess.run(build, cwd=ROOT, env=env, text=True, capture_output=True, timeout=600)
    if result.returncode:
        raise ValueError("Native qualification application failed to build; no live requests were made")
    if before != runtime_fingerprint():
        raise ValueError("Runtime source changed during build; no live requests were made")
    metadata = json.loads(subprocess.check_output([str(cargo), f"+{toolchain}", "metadata", "--no-deps",
        "--locked", "--offline", "--format-version", "1"], cwd=ROOT, env=env, text=True))
    binary = pathlib.Path(metadata["target_directory"]) / host / "debug/examples/live_qualification"
    private = pathlib.Path(tempfile.mkdtemp(prefix="scm-sdk-native-live-"))
    raw_path = private / "report.json"
    command = [str(binary), "--config", str(args.config.resolve()), "--expect-tsg", args.expect_tsg,
        "--output", str(raw_path), "--recovery-output", str(private / "recovery"), "--read-only" if read_only else "--mutations"]
    try:
        execution = subprocess.run(command, cwd=ROOT, env=env, text=True, capture_output=True, timeout=650)
    except subprocess.TimeoutExpired:
        raise ValueError(f"Native qualification timed out; private recovery retained at {private}") from None
    # Keep diagnostics private even though the application has static formatting.
    for suffix, value in (("stdout.txt", execution.stdout), ("stderr.txt", execution.stderr)):
        path = private / suffix
        path.write_text(value)
        path.chmod(0o600)
    if execution.returncode or not raw_path.is_file():
        raise ValueError(f"Native qualification interrupted; private recovery retained at {private}")
    report = validate_report(json.loads(raw_path.read_text()), expected_version=crate_version())
    envelope = {"schemaVersion": 1, "baselineCommit": BASELINE,
        "executedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "runtimeInputsSha256": before, "binarySha256": digest(binary), "toolchain": toolchain,
        "hostTarget": host, "sourceUnchanged": before == runtime_fingerprint(), "exitCode": execution.returncode,
        "commandTemplate": command_template(read_only),
        "report": report}
    report_path.write_text(json.dumps(envelope, indent=2) + "\n")
    (ROOT / INPUT).write_text(json.dumps({"schemaVersion": 1, "baselineCommit": BASELINE,
        "report": report_path.relative_to(ROOT).as_posix(), "reportSha256": digest(report_path)}, indent=2) + "\n")
    annotations, _ = load(required=True)
    print(json.dumps({"report": report_path.relative_to(ROOT).as_posix(), "privateRecovery": str(private),
        "nativeMethodsVerified": sum(v["liveAcceptance"] == "verified" for v in annotations.values()),
        "cleanupState": report["cleanupState"], "serviceRequests": report["serviceRequests"]}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--run", action="store_true")
    parser.add_argument("--config", type=pathlib.Path)
    parser.add_argument("--expect-tsg")
    mode = parser.add_mutually_exclusive_group()
    mode.add_argument("--mutations", action="store_true")
    mode.add_argument("--read-only", action="store_true")
    parser.add_argument("--attempt", type=int, default=1)
    args = parser.parse_args()
    if args.run and args.check:
        parser.error("Choose execution or credential-free verification")
    if args.run:
        run(args)
    else:
        annotations, envelope = load(required=True)
        print(json.dumps({"checked": True, "nativeMethodsVerified": sum(v["liveAcceptance"] == "verified" for v in annotations.values()),
            "observedMethods": len(annotations), "cleanupState": envelope["report"]["cleanupState"]}))


if __name__ == "__main__":
    main()
