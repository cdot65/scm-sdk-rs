#!/usr/bin/env python3
"""Record native operation mappings without changing the immutable TS baseline."""

import argparse
import hashlib
import json
import pathlib
import re
import tomllib

import live_evidence

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


def canonical_namespace(namespace, source):
    """Use one crate-rooted path and bind its module and receiver to source."""
    if not isinstance(namespace, str) or not re.fullmatch(r"[A-Za-z_]\w*(?:::[A-Za-z_]\w*)+", namespace):
        raise ValueError("Native namespace is not a Rust path")
    relative = namespace.removeprefix("scm_sdk::").removeprefix("crate::")
    *module, receiver = relative.split("::")
    source_path = pathlib.PurePosixPath(source)
    if (not source_path.is_relative_to("crates/scm-sdk/src") or ".." in source_path.parts
            or source_path.is_absolute()):
        raise ValueError("Native namespace source must be inside the crate")
    module_path = ROOT.joinpath("crates/scm-sdk/src", *module)
    actual_source = ROOT / source_path
    if module_path.is_dir():
        matches = actual_source.parent == module_path
    else:
        matches = actual_source == module_path.with_suffix(".rs")
    if not matches or not actual_source.is_file():
        raise ValueError("Native namespace module does not match its source")
    if not re.search(r"\b(?:struct|impl)[^\n{};]*\b" + re.escape(receiver) + r"\b", actual_source.read_text()):
        raise ValueError("Native namespace receiver is absent from its source")
    return "scm_sdk::" + relative


def json_boundary_contract():
    """One global, explicit security difference; never a per-operation omission."""
    return {
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
    }


def verify_json_boundary_contract(contract):
    if contract != json_boundary_contract():
        raise ValueError("Native JSON boundary intentional difference is missing or differs")


def live_projection(root=ROOT):
    """Load optional evidence, failing closed when an existing receipt is invalid."""
    annotations, envelope = live_evidence.load(root)
    bound = {"tools/live_evidence.py"}
    if envelope is None:
        return annotations, bound, None
    reference_path = live_evidence.INPUT.as_posix()
    reference = json.loads(live_evidence.regular(root, reference_path).read_text())
    bound.update({reference_path, reference["report"]})
    receipt = {
        "reference": {"path": reference_path, "sha256": sha(root / reference_path)},
        "report": {"path": reference["report"], "sha256": reference["reportSha256"]},
        "verifier": {"path": "tools/live_evidence.py", "sha256": sha(root / "tools/live_evidence.py")},
        "executedAt": envelope["executedAt"],
        "runtimeInputsSha256": envelope["runtimeInputsSha256"],
        "toolchain": envelope["toolchain"],
        "hostTarget": envelope["hostTarget"],
        "cleanupState": envelope["report"]["cleanupState"],
    }
    return annotations, bound, receipt


def verify_live_rows(rows, annotations):
    """Do not trust a persisted ledger's live flags over the native receipt."""
    known = {row["baselineId"] for row in rows}
    if set(annotations) - known:
        raise ValueError("Live evidence references an absent frozen operation")
    for row in rows:
        expected = annotations.get(row["baselineId"], {})
        if (row.get("liveAcceptance") != expected.get("liveAcceptance", "unverified")
                or row.get("liveEvidence") != expected.get("liveEvidence")):
            raise ValueError("Native operation live evidence is stale or unsupported by its receipt")
        if expected and row["implementation"] != "implemented":
            raise ValueError("Live evidence cannot promote an unimplemented native operation")


def generate():
    inventory = json.loads(SOURCE.read_text())
    version = tomllib.loads((ROOT / "crates/scm-sdk/Cargo.toml").read_text())["package"]["version"]
    rows = []
    native = mappings()
    live_annotations, live_bindings, live_receipt = live_projection(ROOT)
    bound = {"crates/scm-sdk/src/engine.rs", "crates/scm-sdk/src/streaming.rs", "crates/scm-sdk/src/transport.rs", "crates/scm-sdk/src/config.rs", "crates/scm-sdk/src/pagination.rs", "crates/scm-sdk/src/pagination/tests.rs", "crates/scm-sdk/src/multipart.rs"}
    bound.update({"crates/scm-sdk/src/json_decode.rs", "crates/scm-sdk/src/json_decode/tests.rs"})
    boundary = json_boundary_contract()
    for documentation in boundary["documentation"]:
        relative = documentation.partition("#")[0]
        if not (ROOT / relative).is_file():
            raise ValueError("JSON boundary documentation is absent")
        bound.add(relative)
    for reference in boundary["tests"]:
        relative, _, test = reference.partition("::")
        if not re.search(r"\bfn\s+" + re.escape(test) + r"\s*[(<]", (ROOT / relative).read_text()):
            raise ValueError("JSON boundary verification function is absent")
        bound.add(relative)
    bound.update(str(path.relative_to(ROOT)) for path in FRAGMENTS.glob("*.json"))
    bound.update(live_bindings)
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
            feature = mapping["feature"]
            namespace = canonical_namespace(mapping["namespace"], mapping["source"])
            expected_headers = ("gateway_realtime" if operation["id"] ==
                "src/ai-gateway/inference-client.ts:AIGatewayInferenceClient.connectRealtime" else "airs_api_defaults")
            if mapping.get("headerContract", "airs_api_defaults") != expected_headers:
                raise ValueError("Native operation has an invalid protocol header contract")
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
                       source=source, tests=tests, headerContract=mapping.get("headerContract", "airs_api_defaults"),
                       optionParity=mapping["optionParity"], deliberateDifferences=mapping["deliberateDifferences"],
                       deliveredInPhase=mapping["deliveredInPhase"], lastContractPhase=mapping["lastContractPhase"])
            row.update(live_annotations.get(operation["id"], {}))
        rows.append(row)
    verify_live_rows(rows, live_annotations)
    return {
        "schemaVersion": 1,
        "baselineCommit": inventory["baseline"]["commit"],
        "baselineOperationsSha256": sha(SOURCE), "generatorSha256": sha(pathlib.Path(__file__)),
        "ownershipOverrides": OWNERSHIP_OVERRIDES,
        "jsonBoundaryContract": boundary,
        "liveReceipt": live_receipt,
        "purpose": "Native implementation and verification status; frozen operations.json remains unchanged. Mapping declarations, method presence and test pointers are checked here; source-bound independent test execution establishes the declared wire behavior. An implemented method with partial option parity remains an explicit owning-phase obligation.",
        "requestHeaderContracts": {
            "gateway_realtime": {
                "purpose": "Explicit runtime-key WebSocket upgrade; bypasses common HTTP API defaults as in frozen openRealtime.",
                "authentication": "x-portkey-api-key",
                "routing": "validated non-authentication x-portkey-* and OpenAI-Beta",
                "nativeUpgrade": ["Host", "Upgrade", "Connection", "Sec-WebSocket-Key", "Sec-WebSocket-Version"],
                "neverInherited": ["SCM OAuth", "x-tsg-id", "service-name", "User-Agent", "HTTP proxy/TLS settings"],
            },
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
                    "pending": len(rows) - len(native),
                    "liveVerified": sum(row["liveAcceptance"] == "verified" for row in rows),
                    "liveRecorded": sum("liveEvidence" in row for row in rows),
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
