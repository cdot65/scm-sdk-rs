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
    "src/ai-gateway/runtime-resources-client.ts:AIGatewayRuntimeResourcesClient.downloadFile": ("gateway", "airs::gateway::RuntimeClient", ["download_file"], "crates/scm-sdk/src/airs/gateway/runtime.rs", "crates/scm-sdk/tests/representative_clients.rs"),
}
PHASES = {"scan": 3, "management": 4, "iam": 4, "model-security": 5,
          "red-team": 5, "ai-gateway": 6, "agentguard": 7}


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def generate():
    inventory = json.loads(SOURCE.read_text())
    version = tomllib.loads((ROOT / "crates/scm-sdk/Cargo.toml").read_text())["package"]["version"]
    rows = []
    bound = {"crates/scm-sdk/src/engine.rs", "crates/scm-sdk/src/streaming.rs", "crates/scm-sdk/src/transport.rs"}
    unknown = set(MAPPINGS) - {operation["id"] for operation in inventory["operations"]}
    if unknown:
        raise ValueError(f"Native mappings reference absent frozen operations: {sorted(unknown)}")
    for operation in inventory["operations"]:
        row = {"baselineId": operation["id"], "family": operation["family"],
               "owningPhase": PHASES[operation["family"]], "implementation": "pending",
               "contractVerification": "pending", "liveAcceptance": "unverified"}
        if mapping := MAPPINGS.get(operation["id"]):
            feature, namespace, methods, source, tests = mapping
            text = (ROOT / source).read_text()
            for method in methods:
                if not re.search(r"pub\s+(?:async\s+)?fn\s+" + re.escape(method) + r"\s*[(<]", text):
                    raise ValueError(f"Native method absent: {namespace}::{method}")
            if not (ROOT / tests).is_file():
                raise ValueError("Native operation verification source is absent")
            bound.update((source, tests))
            row.update(owningPhase=3, implementation="implemented", contractVerification="offline_fixture_verified",
                       rustMethods=[f"{namespace}::{method}" for method in methods],
                       feature=feature, source=source, tests=[tests], headerContract="airs_api_defaults")
        rows.append(row)
    return {
        "schemaVersion": 1,
        "baselineCommit": inventory["baseline"]["commit"],
        "baselineOperationsSha256": sha(SOURCE), "generatorSha256": sha(pathlib.Path(__file__)),
        "purpose": "Native implementation and verification status; frozen operations.json remains unchanged. Mapping presence is checked here; independent native tests establish wire behavior.",
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
        "summary": {"baselineConcreteMethods": len(rows), "implemented": len(MAPPINGS),
                    "pending": len(rows) - len(MAPPINGS), "liveVerified": 0},
        "operations": rows,
    }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    result = generate()
    content = json.dumps(result, indent=2) + "\n"
    if args.check:
        if not OUTPUT.is_file() or OUTPUT.read_text() != content:
            raise ValueError("Native operation ledger differs; regenerate and review mappings")
    else:
        OUTPUT.write_text(content)
    print(json.dumps({"checked": args.check, **result["summary"]}))


if __name__ == "__main__":
    main()
