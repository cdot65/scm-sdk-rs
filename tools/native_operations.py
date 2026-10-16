#!/usr/bin/env python3
"""Record native operation mappings without changing the immutable TS baseline."""

import argparse
import hashlib
import json
import pathlib
import re
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]
SOURCE = ROOT / "contracts/operations.json"
OUTPUT = ROOT / "contracts/native-operations.json"
MAPPINGS = {
    "src/scan/scanner.ts:Scanner.syncScan": ("scan", "airs::scan::Scanner", ["sync_scan", "scan_request"], "crates/scm-sdk/src/airs/scan/scanner.rs", "crates/scm-sdk/tests/scan_contracts.rs"),
    "src/scan/scanner.ts:Scanner.asyncScan": ("scan", "airs::scan::Scanner", ["async_scan"], "crates/scm-sdk/src/airs/scan/scanner.rs", "crates/scm-sdk/tests/scan_contracts.rs"),
    "src/scan/scanner.ts:Scanner.queryByScanIds": ("scan", "airs::scan::Scanner", ["query_by_scan_ids"], "crates/scm-sdk/src/airs/scan/scanner.rs", "crates/scm-sdk/tests/scan_contracts.rs"),
    "src/scan/scanner.ts:Scanner.queryByReportIds": ("scan", "airs::scan::Scanner", ["query_by_report_ids"], "crates/scm-sdk/src/airs/scan/scanner.rs", "crates/scm-sdk/tests/scan_contracts.rs"),
    "src/management/profiles.ts:ProfilesClient.listForToken": ("management", "airs::management::ProfilesClient", ["list_for_token"], "crates/scm-sdk/src/airs/management/profiles.rs", "crates/scm-sdk/tests/representative_clients.rs"),
    "src/ai-gateway/inference-client.ts:AIGatewayInferenceClient.createChatCompletion": ("gateway", "airs::gateway::RuntimeClient", ["create_chat_completion", "create_chat_completion_stream"], "crates/scm-sdk/src/airs/gateway/runtime.rs", "crates/scm-sdk/tests/representative_clients.rs"),
    "src/ai-gateway/runtime-resources-client.ts:AIGatewayRuntimeResourcesClient.downloadFile": ("gateway", "airs::gateway::RuntimeClient", ["download_file"], "crates/scm-sdk/src/airs/gateway/runtime_resources.rs", "crates/scm-sdk/tests/representative_clients.rs"),
}
PHASES = {"scan": 3, "management": 4, "iam": 4, "model-security": 5,
          "red-team": 5, "ai-gateway": 6, "agentguard": 7}
OWNERSHIP_OVERRIDES = {
    "src/ai-gateway/inference-client.ts:AIGatewayInferenceClient.connectRealtime": {
        "phase": 7, "reason": "Phase 7 owns bounded realtime lifecycle and experimental export closure, as specified in docs/PHASES.md."
    },
    "src/ai-gateway/workspaces-client.ts:AIGatewayWorkspacesClient.provision": {
        "phase": 7, "reason": "Phase 7 owns multi-step provisioning, binding recovery and uncertain remote outcomes, as specified in docs/PHASES.md."
    },
}
FRAGMENTS = ROOT / "contracts/native-operation-mappings"


def mappings():
    result = {}
    for baseline_id, (feature, namespace, methods, source, tests) in MAPPINGS.items():
        differences = []
        option_parity = "complete"
        if "createChatCompletion" in baseline_id or "downloadFile" in baseline_id:
            option_parity = "partial"
            differences.append("Representative Phase 3 slice omits GatewayInferenceRequestOptions routing/OpenAI-Beta headers; full option parity is required in owning Phase 6.")
        if "Scanner.asyncScan" in baseline_id or "Scanner.syncScan" in baseline_id:
            differences.append("Native raw async and scan_request inputs enforce Content primary-field and byte-limit invariants before authentication; the frozen raw async path applies only its request schema. Typed native callers retain explicit validation failures.")
        result[baseline_id] = dict(baselineId=baseline_id, feature=feature, namespace=namespace,
            methods=methods, source=source, tests=[tests], deliberateDifferences=differences,
            optionParity=option_parity, deliveredInPhase=3, lastContractPhase=3, liveAcceptance="unverified")
    replaced = set()
    for fragment in sorted(FRAGMENTS.glob("*.json")):
        data = json.loads(fragment.read_text())
        if data.get("schemaVersion") != 1:
            raise ValueError("Unsupported native operation mapping schema")
        baseline_commit = json.loads(SOURCE.read_text())["baseline"]["commit"]
        if data.get("baselineCommit") != baseline_commit:
            raise ValueError("Native mapping has the wrong baseline")
        for entry in data["operations"]:
            row = dict(entry)
            baseline_id = row["baselineId"]
            verified_phase = row.get("lastContractPhase", row["deliveredInPhase"])
            if baseline_id in replaced:
                raise ValueError("Duplicate native operation mapping")
            replaced.add(baseline_id)
            if baseline_id in result:
                previous = result[baseline_id]
                row["deliveredInPhase"] = min(previous["deliveredInPhase"], row["deliveredInPhase"])
                row["tests"] = sorted(set(previous["tests"] + row["tests"]))
            row["lastContractPhase"] = max(verified_phase, data.get("verifiedInPhase", verified_phase))
            result[baseline_id] = row
    return result


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def generate():
    inventory = json.loads(SOURCE.read_text())
    version = tomllib.loads((ROOT / "crates/scm-sdk/Cargo.toml").read_text())["package"]["version"]
    rows = []
    native = mappings()
    bound = {"crates/scm-sdk/src/engine.rs", "crates/scm-sdk/src/streaming.rs", "crates/scm-sdk/src/transport.rs", "crates/scm-sdk/src/config.rs", "crates/scm-sdk/src/pagination.rs", "crates/scm-sdk/src/pagination/tests.rs", "crates/scm-sdk/src/multipart.rs"}
    bound.update(str(path.relative_to(ROOT)) for path in FRAGMENTS.glob("*.json"))
    unknown = set(native) - {operation["id"] for operation in inventory["operations"]}
    if unknown:
        raise ValueError(f"Native mappings reference absent frozen operations: {sorted(unknown)}")
    unknown_owners = set(OWNERSHIP_OVERRIDES) - {operation["id"] for operation in inventory["operations"]}
    if unknown_owners or any(value.get("phase") != 7 or not value.get("reason")
                             for value in OWNERSHIP_OVERRIDES.values()):
        raise ValueError("Native ownership override is absent from baseline or lacks its explicit Phase 7 reason")
    for operation in inventory["operations"]:
        row = {"baselineId": operation["id"], "family": operation["family"],
               "owningPhase": OWNERSHIP_OVERRIDES.get(operation["id"], {}).get("phase", PHASES[operation["family"]]), "implementation": "pending",
               "contractVerification": "pending", "liveAcceptance": "unverified"}
        if mapping := native.get(operation["id"]):
            required = {"feature", "namespace", "methods", "source", "tests", "deliberateDifferences", "optionParity", "deliveredInPhase"}
            if not required <= mapping.keys() or mapping["optionParity"] not in ("complete", "partial"):
                raise ValueError("Incomplete native mapping contract")
            if mapping["optionParity"] == "partial" and not mapping["deliberateDifferences"]:
                raise ValueError("Partial native parity requires an explicit difference")
            feature, namespace = mapping["feature"], mapping["namespace"]
            methods, source, tests = mapping["methods"], mapping["source"], mapping["tests"]
            if not methods or not tests:
                raise ValueError("Native mappings require concrete methods and verification")
            text = (ROOT / source).read_text()
            for method in methods:
                if not re.search(r"pub\s+(?:async\s+)?fn\s+" + re.escape(method) + r"\s*[(<]", text):
                    raise ValueError(f"Native method absent: {namespace}::{method}")
            for reference in tests:
                path, _, test = reference.partition("::")
                if not (ROOT / path).is_file():
                    raise ValueError("Native operation verification source is absent")
                if test and not re.search(r"\bfn\s+" + re.escape(test) + r"\s*[(<]", (ROOT / path).read_text()):
                    raise ValueError("Native operation verification function is absent")
                bound.add(path)
            bound.add(source)
            row.update(implementation="implemented", contractVerification="offline_fixture_verified",
                       rustMethods=[f"{namespace}::{method}" for method in methods], feature=feature,
                       source=source, tests=tests, headerContract="airs_api_defaults",
                       optionParity=mapping["optionParity"], deliberateDifferences=mapping["deliberateDifferences"],
                       deliveredInPhase=mapping["deliveredInPhase"], lastContractPhase=mapping["lastContractPhase"])
        rows.append(row)
    return {
        "schemaVersion": 1,
        "baselineCommit": inventory["baseline"]["commit"],
        "baselineOperationsSha256": sha(SOURCE), "generatorSha256": sha(pathlib.Path(__file__)),
        "ownershipOverrides": OWNERSHIP_OVERRIDES,
        "purpose": "Native implementation and verification status; frozen operations.json remains unchanged. Mapping declarations, method presence and test pointers are checked here; source-bound independent test execution establishes the declared wire behavior. An implemented method with partial option parity remains an explicit owning-phase obligation.",
        "requestHeaderContracts": {
            "airs_api_defaults": {
                "service-name": "api", "user-agent": f"PAN-AIRS/{version}-rust-sdk",
                "scope": "API requests assembled by Client, including injected transports and streams; OAuth issuer exchange has its own headers.",
                "frozenUserAgent": "PAN-AIRS/0.34.0-typescript-sdk",
                "deliberateDifference": "Native crate version and rust-sdk suffix retain the PAN-AIRS vendor prefix.",
                "regressionSource": "contracts/fixtures/typescript/test/http/request.spec.ts",
                "regressionSourceSha256": sha(ROOT / "contracts/fixtures/typescript/test/http/request.spec.ts"),
                "overridePolicy": "SDK-owned defaults cannot be overridden by raw per-request headers.",
            }
        },
        "sourceBindings": [{"path": name, "sha256": sha(ROOT / name)} for name in sorted(bound)],
        "summary": {"baselineConcreteMethods": len(rows), "implemented": len(native),
                    "pending": len(rows) - len(native), "liveVerified": 0,
                    "completeMethodOptionParity": sum(row.get("optionParity") == "complete" for row in rows),
                    "partialMethodOptionParity": sum(row.get("optionParity") == "partial" for row in rows)},
        "operations": rows,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--require-through-phase", type=int, choices=range(3, 9),
                        help="Fail on missing methods or incomplete method options in completed scope")
    args = parser.parse_args()
    result = generate()
    if args.require_through_phase:
        missing = [row["baselineId"] for row in result["operations"]
                   if row["owningPhase"] <= args.require_through_phase
                   and (row["implementation"] != "implemented" or row.get("optionParity") != "complete")]
        if missing:
            raise ValueError(f"Native owning-phase capability gaps: {missing}")
    content = json.dumps(result, indent=2) + "\n"
    if args.check:
        if not OUTPUT.is_file() or OUTPUT.read_text() != content:
            raise ValueError("Native operation ledger differs; regenerate and review mappings")
    else:
        OUTPUT.write_text(content)
    print(json.dumps({"checked": args.check, **result["summary"]}))


if __name__ == "__main__":
    main()
