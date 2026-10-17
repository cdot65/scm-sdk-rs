#!/usr/bin/env python3
"""Project every frozen operation's independent implementation/contract/live status."""

import argparse
import collections
import hashlib
import json
import pathlib

import native_operations

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "contracts/qualification.json"
BASELINE = "4a80dbdb3dc14365e1c25d838ac0bd139485d48c"
REASONS = {
    "verified": "The exact operation succeeded in the source-bound controlled native workflow.",
    "authentication_prerequisite": "The recorded workflow could not satisfy service authentication.",
    "permission_prerequisite": "The recorded identity lacked the required service permission.",
    "not_found_or_unavailable": "The requested service or fixture was absent or unavailable.",
    "rate_limit_prerequisite": "The service rate limit prevented acceptance.",
    "upstream_failure": "The service returned a server failure.",
    "network_prerequisite": "The recorded service connection failed.",
    "service_contract_mismatch": "The native response contract rejected the service response.",
    "unexpected_client_failure": "The controlled native workflow reported a client failure.",
    "not_attempted_missing_prerequisite": "The runner skipped this operation because its explicit prerequisite was missing.",
    "not_attempted_existing_resource": "The runner skipped creation to avoid altering a pre-existing resource.",
    "service_contract_rejection": "The submitted request was rejected; its cause is unestablished and a native request-shape defect is not excluded.",
    "service_reported_scan_failure": "The accepted transport response reported a failed scan workflow.",
    "retained_for_reconciliation": "The resource was retained because its remote state required reconciliation.",
    "archive_not_confirmed": "The service did not confirm workspace archival.",
    "incomplete_or_uncorrelated_receipt": "The HTTP response did not prove completion of the requested, correctly correlated workflow.",
}
NATIVE_LIFECYCLE = {
    "src/management/oauth-client.ts:OAuthClient." + name
    for name in ("clearToken", "getTokenInfo", "isTokenExpired", "isTokenExpiringSoon")
}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def regular(relative):
    path = pathlib.PurePosixPath(relative)
    if path.is_absolute() or ".." in path.parts or path.as_posix() != relative or "\\" in relative:
        raise ValueError("Qualification input path must be canonical and repository-confined")
    value = ROOT
    for part in path.parts:
        value = value / part
        if value.is_symlink():
            raise ValueError("Qualification inputs must not follow symlinks")
    if not value.is_file():
        raise ValueError("Qualification input is missing")
    return value


def summary(rows):
    return {
        "operations": len(rows),
        "implemented": sum(row["implementation"] == "implemented" for row in rows),
        "contractVerified": sum(row["contractVerification"] == "offline_fixture_verified" for row in rows),
        "completeMethodOptionParity": sum(row["methodOptionParity"] == "complete" for row in rows),
        "liveVerified": sum(row["liveAcceptance"] == "verified" for row in rows),
        "liveRecorded": sum("liveEvidence" in row for row in rows),
        "liveNotVerified": sum(row["liveAcceptance"] != "verified" for row in rows),
        "liveByStatus": dict(sorted(collections.Counter(row["liveAcceptance"] for row in rows).items())),
        "unexplainedGaps": sum(not row.get("liveReason") for row in rows),
    }


def project(frozen, native, exports, annotations, receipt):
    if annotations and receipt is None:
        raise ValueError("Live annotations require their validated native receipt")
    if any(value != BASELINE for value in (
        frozen["baseline"]["commit"], native["baselineCommit"], exports["baselineCommit"]
    )):
        raise ValueError("Qualification baseline differs")
    for contract in (native.get("jsonBoundaryContract"), exports.get("jsonBoundaryContract")):
        native_operations.verify_json_boundary_contract(contract)
    expected = {row["id"] for row in frozen["operations"]}
    operations = {row["baselineId"]: row for row in native["operations"]}
    if (len(expected) != 448 or len(frozen["operations"]) != 448
            or len(native["operations"]) != 448 or set(operations) != expected):
        raise ValueError("Qualification must account for all 448 unique frozen operations")
    if (len(exports["capabilities"]) != 3820
            or len({row["baselineId"] for row in exports["capabilities"]}) != 3820
            or any(row["implementation"] != "implemented" for row in exports["capabilities"])):
        raise ValueError("Qualification requires complete native export closure")
    native_operations.verify_live_rows(native["operations"], annotations)
    contract_bindings = {binding["path"]: binding["sha256"] for binding in native["sourceBindings"]}
    public = collections.defaultdict(list)
    for capability in exports["capabilities"]:
        operation_id = capability.get("operationId")
        if operation_id is None:
            if capability["liveAcceptance"] != "unverified" or "liveEvidence" in capability:
                raise ValueError("Non-operation exports cannot inherit server qualification")
            continue
        if capability["kind"] != "method":
            raise ValueError("Constructors and non-method exports cannot claim server operation evidence")
        if operation_id not in expected:
            raise ValueError("Native export refers to an unknown frozen operation")
        operation = operations[operation_id]
        if (capability["liveAcceptance"] != operation["liveAcceptance"]
                or capability.get("liveEvidence") != operation.get("liveEvidence")):
            raise ValueError("Native export live status differs from its canonical operation")
        public[operation_id].append(capability["baselineId"])
    if set(public) != expected:
        raise ValueError("Every frozen operation requires an explicit native export binding")
    rows = []
    for frozen_operation in frozen["operations"]:
        operation = operations[frozen_operation["id"]]
        if (operation["implementation"] != "implemented"
                or operation["contractVerification"] != "offline_fixture_verified"
                or operation.get("optionParity") != "complete"):
            raise ValueError("Qualification has an implementation, contract, or option-parity gap")
        status = operation["liveAcceptance"]
        source_paths = {operation["source"]} | {test.partition("::")[0] for test in operation["tests"]}
        if not source_paths <= contract_bindings.keys():
            raise ValueError("Qualification implementation/contract evidence lacks its source binding")
        if status == "unverified":
            if receipt is None:
                reason = "No retained controlled native receipt exists."
            elif operation["baselineId"] in NATIVE_LIFECYCLE:
                reason = "This local OAuth lifecycle method has offline evidence; the controlled runner did not qualify it independently."
            else:
                reason = "This exact operation was not executed by the controlled native probe set."
        else:
            reason = REASONS.get(status)
            if reason is None or "liveEvidence" not in operation:
                raise ValueError("Qualification live status lacks an explained native receipt")
        if operation.get("liveEvidence", {}).get("reconciliationRequired"):
            if status == "verified":
                raise ValueError("Conflicting live observations require reviewed reconciliation")
            reason += " Mixed or conflicting observations require a new reviewed reconciliation receipt; earlier successes do not certify this method."
        row = {
            "baselineId": operation["baselineId"],
            "family": operation["family"],
            "namespace": operation["rustMethods"][0].rsplit("::", 2)[0],
            "feature": operation["feature"],
            "owningPhase": operation["owningPhase"],
            "baselineSource": frozen_operation["source"],
            "implementation": operation["implementation"],
            "rustMethods": operation["rustMethods"],
            "source": operation["source"],
            "sourceSha256": contract_bindings[operation["source"]],
            "contractVerification": operation["contractVerification"],
            "contractTests": operation["tests"],
            "contractEvidence": [{"reference": test, "path": test.partition("::")[0],
                                 "sha256": contract_bindings[test.partition("::")[0]]}
                                for test in operation["tests"]],
            "contractExecutionAuthority": "Source-bound phase checks record execution separately; generation checks concrete mappings and evidence pointers.",
            "methodOptionParity": operation["optionParity"],
            "deliberateDifferences": operation["deliberateDifferences"],
            "nativeExportCapabilities": sorted(public[operation["baselineId"]]),
            "liveAcceptance": status,
            "liveReason": reason,
        }
        if "liveEvidence" in operation:
            row["liveEvidence"] = operation["liveEvidence"]
        rows.append(row)
    result = {
        "schemaVersion": 1,
        "baselineCommit": BASELINE,
        "purpose": "Exhaustive frozen operation qualification; implementation, offline contracts, and controlled live acceptance are independent dimensions. No family-wide or constructor promotion is inferred.",
        "jsonBoundaryContract": native["jsonBoundaryContract"],
        "liveReceipt": receipt,
        "summary": summary(rows),
        "families": {family: summary([row for row in rows if row["family"] == family])
                     for family in sorted({row["family"] for row in rows})},
        "operations": rows,
    }
    if result["summary"]["unexplainedGaps"]:
        raise ValueError("Qualification contains unexplained gaps")
    return result


def generate():
    frozen = json.loads(regular("contracts/operations.json").read_text())
    native = json.loads(regular("contracts/native-operations.json").read_text())
    if native != native_operations.generate():
        raise ValueError("Native operation qualification inputs are stale; regenerate first")
    exports = json.loads(regular("contracts/native-exports.json").read_text())
    annotations, bindings, receipt = native_operations.live_projection(ROOT)
    for binding in exports["sourceBindings"]:
        if sha(regular(binding["path"])) != binding["sha256"]:
            raise ValueError("Native export source binding is stale; regenerate first")
    if exports["generatorSha256"] != sha(regular("tools/native_exports.py")):
        raise ValueError("Native export generator binding is stale")
    result = project(frozen, native, exports, annotations, receipt)
    bindings.update({
        "contracts/operations.json", "contracts/native-operations.json", "contracts/native-exports.json",
        "tools/native_operations.py", "tools/native_exports.py", "tools/qualification.py",
        "tools/test_qualification.py",
        "crates/scm-sdk/src/json_decode.rs", "crates/scm-sdk/src/json_decode/tests.rs",
    })
    result["generatorSha256"] = sha(pathlib.Path(__file__))
    result["sourceBindings"] = [{"path": name, "sha256": sha(regular(name))} for name in sorted(bindings)]
    return result


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    result = generate()
    content = json.dumps(result, indent=2) + "\n"
    if args.check:
        if not OUTPUT.is_file() or OUTPUT.read_text() != content:
            raise ValueError("Qualification projection is stale; regenerate and review")
    else:
        OUTPUT.write_text(content)
    print(json.dumps({"checked": args.check, **result["summary"]}))


if __name__ == "__main__":
    main()
