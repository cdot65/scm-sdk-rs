#!/usr/bin/env python3
"""Reconcile every frozen exported capability with native public API evidence.

The immutable export classification remains source evidence. This separate ledger
binds concrete public paths, per-member probes, tests and deliberate adaptations.
No missing adapter is silently treated as a generic Error/Client capability.
"""
import argparse
import collections
import hashlib
import json
import pathlib
import re
import sys

import native_operations

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "contracts/native-exports.json"
PROBES = ROOT / "crates/scm-sdk/tests/native_export_probes.rs"
CONSUMER = ROOT / "contracts/native-probes"
MAPPINGS = ROOT / "contracts/native-capability-mappings"
JSON_TYPE = "serde_json::Value"


def sha(path):
    return hashlib.sha256(path.read_bytes()).hexdigest()


def snake(name):
    return re.sub(r"([a-z0-9])([A-Z])", r"\1_\2", name).lower()


def public(path):
    return path.replace("crate::", "scm_sdk::")


def source_type(name, namespace):
    return "scm_sdk::" + namespace + "::models::" + name


def regular(relative):
    name = pathlib.PurePosixPath(relative)
    if name.is_absolute() or ".." in name.parts or str(name) != relative:
        raise ValueError("Evidence path is not canonical and repository-confined: " + relative)
    path = ROOT
    for part in name.parts:
        path = path / part
        if path.is_symlink():
            raise ValueError("Evidence path must not follow symlinks: " + relative)
    if not path.is_file():
        raise ValueError("Evidence file is missing or is a symlink: " + relative)
    return path


def function_exists(reference):
    relative, _, name = reference.partition("::")
    if relative == PROBES.relative_to(ROOT).as_posix() and name == "all_public_export_paths_compile":
        return  # This generator emits the checked external probe below.
    text = regular(relative).read_text()
    if name and not re.search(r"\bfn\s+" + re.escape(name) + r"\s*[(<]", text):
        raise ValueError("Verification function is absent: " + reference)


def generate():
    inventory = json.loads(regular("contracts/typescript.json").read_text())
    classification = json.loads(regular("contracts/export-classification.json").read_text())
    models = json.loads(regular("contracts/native-models.json").read_text())
    operations = json.loads(regular("contracts/native-operations.json").read_text())
    native_operations.verify_json_boundary_contract(operations.get("jsonBoundaryContract"))
    live_annotations, live_bindings, live_receipt = native_operations.live_projection(ROOT)
    native_operations.verify_live_rows(operations["operations"], live_annotations)
    pin = "4a80dbdb3dc14365e1c25d838ac0bd139485d48c"
    if (inventory["baseline"]["commit"] != pin or classification["baselineCommit"] != pin
            or models["baselineCommit"] != pin or operations["baselineCommit"] != pin
            or models["sourceSha256"] != sha(regular("contracts/typescript.json"))):
        raise ValueError("Native export inputs have stale or mismatched frozen provenance")
    if len(classification["entries"]) != 3820 or len(inventory["exports"]) != 2630:
        raise ValueError("Frozen export denominator differs")
    bound = {"contracts/typescript.json", "contracts/export-classification.json", "contracts/native-models.json",
             "contracts/native-operations.json", "tools/native_exports.py", "tools/test_native_exports.py",
             "tools/verify_gateway_bindings.py",
             "scripts/check-native-exports.py", "Cargo.toml", "Cargo.lock", "crates/scm-sdk/Cargo.toml"}
    bound.update(live_bindings)
    bound.add("tools/native_operations.py")
    manual = {}
    for path in sorted(MAPPINGS.glob("*.json")):
        relative = path.relative_to(ROOT).as_posix()
        bound.add(relative)
        data = json.loads(regular(relative).read_text())
        if data["schemaVersion"] != 1 or data["baselineCommit"] != inventory["baseline"]["commit"]:
            raise ValueError("Manual native capability mapping has the wrong baseline")
        for row in data["capabilities"]:
            key = row["baselineId"]
            if key in manual or key not in classification["entries"]:
                raise ValueError("Duplicate or unknown explicit capability: " + key)
            manual[key] = dict(row)
    all_sources = {p.relative_to(ROOT).as_posix(): regular(p.relative_to(ROOT).as_posix()).read_text()
                   for p in (ROOT / "crates/scm-sdk/src").rglob("*.rs")}
    bound.update(all_sources)
    model_by_name = {row["export"]: row for row in models["modelExports"]}
    schema_by_name = {row["export"]: row for row in models["schemas"]}
    parents = collections.defaultdict(set)
    def references(value):
        if isinstance(value, dict):
            if "$ref" in value:
                yield value["$ref"]
            for child in value.values():
                yield from references(child)
        elif isinstance(value, list):
            for child in value:
                yield from references(child)
    for schema_id, definition in inventory["schemas"]["definitions"].items():
        for target in references(definition):
            parents[target].add(schema_id)
    native_components = collections.defaultdict(list)
    for boundary in models["nativeBoundaries"]:
        pending, seen = [boundary["schema"]], set()
        while pending:
            schema_id = pending.pop()
            if schema_id in seen:
                continue
            seen.add(schema_id)
            native_components[schema_id].append(boundary)
            pending.extend(parents[schema_id])
    operation_by_member = {row["baselineId"].split(":", 1)[1]: row for row in operations["operations"]}
    structs = {}
    aliases = {}
    for relative, text in all_sources.items():
        for match in re.finditer(r"pub\s+struct\s+(\w+)(?:<[^\n{;]+>)?\s*\{([^}]*?)\n\}", text, re.S):
            fields = {m.group(1): m.group(2).strip() for m in re.finditer(r"pub\s+((?:r#)?\w+):\s*([^\n]+),", match[2])}
            structs.setdefault(match[1], []).append((relative, fields))
        for match in re.finditer(r"pub\s+type\s+(\w+)(?:<[^;=]+>)?\s*=\s*([^;]+);", text):
            aliases.setdefault(match[1], []).append(match[2].strip())

    facade_tests = {
        "AIGatewayClient": "crates/scm-sdk/tests/facade_contracts.rs::every_gateway_facade_accessor_dispatches_on_its_exact_credential_plane",
        "ManagementClient": "crates/scm-sdk/tests/facade_contracts.rs::every_management_and_dlp_facade_accessor_preserves_shared_identity_and_explicit_origins",
        "DlpNamespace": "crates/scm-sdk/tests/facade_contracts.rs::every_management_and_dlp_facade_accessor_preserves_shared_identity_and_explicit_origins",
        "Content": "crates/scm-sdk/tests/facade_contracts.rs::every_content_accessor_returns_the_validated_explicit_payload_and_byte_length",
    }

    def evidence(row):
        if not row.get("rustPaths") or not row.get("sources") or not row.get("tests"):
            raise ValueError("Concrete capability evidence is required: " + str(row.get("rustPaths")))
        for relative in row["sources"]:
            regular(relative)
            bound.add(relative)
        for reference in row["tests"]:
            function_exists(reference)
            bound.add(reference.partition("::")[0])
        return row

    def native_model(export):
        row = model_by_name.get(export["name"])
        if not row or row["status"] == "deferred_client_adapter":
            return None
        kind = row["status"]
        if kind.startswith("typed_"):
            name = row["rustType"]
            name = name.replace("<T>", "<serde_json::Value>")
            namespace = export["classification"]["namespace"]
            return source_type(name, namespace)
        if kind == "native_foundation_mapping":
            return {"AISecSDKProblemDetails": "scm_sdk::ProblemDetails", "AISecSDKExceptionMetadata": "scm_sdk::ErrorMetadata"}.get(export["name"])
        name = public(row["rustType"])
        # Private generated support names are never accepted as public evidence.
        if "::models::support::" in name or name == "scm_sdk::Transport":
            return None
        return name

    def feature_for(paths):
        features = set()
        for path in paths:
            if "::models::" in path:
                features.add("models")
            elif path.startswith("scm_sdk::realtime::"):
                features.add("realtime")
            elif path.startswith("scm_sdk::dlp::"):
                features.add("dlp")
            elif path.startswith("scm_sdk::shared::iam::"):
                features.add("iam")
            elif path.startswith("scm_sdk::airs::"):
                features.add(path.split("::")[2].replace("_", "-"))
        return sorted(features)

    def constructor_exists(relative, native, constructor):
        # Match the actual inherent implementation, never another type's `new`
        # in the same source (DictionaryUpload shares a file with its client).
        from verify_gateway_bindings import tokens
        values = tokens(all_sources[relative])
        short = native.rsplit("::", 1)[-1]
        for index in range(len(values) - 3):
            if values[index:index + 3] != ["impl", short, "{"]:
                continue
            depth, end = 1, index + 3
            while depth and end < len(values):
                depth += (values[end] == "{") - (values[end] == "}")
                end += 1
            return "pub fn " + constructor + " (" in " ".join(values[index:end])
        return False

    def fields_for(type_path):
        short = re.sub(r"<.*", "", type_path).rsplit("::", 1)[-1]
        values = structs.get(short, [])
        if len(values) == 1:
            return values[0]
        # Aliases such as request-chart options share the concrete native shape.
        targets = aliases.get(short, [])
        if len(set(targets)) == 1:
            return fields_for(targets[0])
        return None

    def model_field(type_path, name):
        wanted = snake(name)
        found = []
        def visit(current, prefix, depth, seen):
            shape = fields_for(current)
            if not shape or depth > 2 or current in seen:
                return
            relative, fields = shape
            if wanted in fields:
                found.append((relative, prefix + wanted))
            elif "r#" + wanted in fields:
                found.append((relative, prefix + "r#" + wanted))
            for field, target in fields.items():
                if field in ("window", "filters", "chart"):
                    visit(target, prefix + field + ".", depth + 1, seen | {current})
        visit(type_path, "", 0, set())
        return found[0] if len(found) == 1 else None

    def client_option_field(export, member, root):
        model = model_by_name.get(export["name"], {})
        if model.get("status") != "native_client_mapping" or not export["name"].endswith(("ClientOptions", "SubClientOptions", "NamespaceOptions")):
            return None
        name = member["name"]
        table = {
            "auth": ("scm_sdk::ClientBuilder::authentication", "crates/scm-sdk/src/engine.rs"),
            "baseUrl": ("scm_sdk::Client::builder", "crates/scm-sdk/src/engine.rs"),
            "apiEndpoint": ("scm_sdk::Client::builder", "crates/scm-sdk/src/engine.rs"),
            "dataEndpoint": ("scm_sdk::Client::builder", "crates/scm-sdk/src/engine.rs"),
            "endpoint": ("scm_sdk::Client::builder", "crates/scm-sdk/src/engine.rs"),
            "numRetries": ("scm_sdk::RequestOptions::with_retries", "crates/scm-sdk/src/config.rs"),
            "clientSecret": ("scm_sdk::ClientBuilder::oauth_credentials", "crates/scm-sdk/src/engine.rs"),
            "clientId": ("scm_sdk::auth::AuthContext::new", "crates/scm-sdk/src/auth.rs"),
            "tokenEndpoint": ("scm_sdk::auth::AuthContext::new", "crates/scm-sdk/src/auth.rs"),
            "tsgId": ("scm_sdk::auth::AuthContext::new", "crates/scm-sdk/src/auth.rs"),
            "adminEndpoint": ("scm_sdk::Client::with_endpoint", "crates/scm-sdk/src/engine.rs"),
            "adminBaseUrl": ("scm_sdk::Client::with_endpoint", "crates/scm-sdk/src/engine.rs"),
            "mgmtEndpoint": ("scm_sdk::Client::with_endpoint", "crates/scm-sdk/src/engine.rs"),
            "iamEndpoint": ("scm_sdk::Client::with_endpoint", "crates/scm-sdk/src/engine.rs"),
            "networkBrokerEndpoint": ("scm_sdk::Client::with_endpoint", "crates/scm-sdk/src/engine.rs"),
            "dashboardEndpoint": ("scm_sdk::airs::management::ManagementClient::with_dashboard_endpoint", "crates/scm-sdk/src/airs/management/mod.rs"),
            "dlpEndpoint": ("scm_sdk::airs::management::ManagementClient::dlp", "crates/scm-sdk/src/airs/management/mod.rs"),
            "inference": ("scm_sdk::airs::gateway::AIGatewayClient::with_runtime", "crates/scm-sdk/src/airs/gateway/mod.rs"),
            "iamScopes": ("scm_sdk::airs::gateway::AIGatewayWorkspacesClient::with_iam_endpoint", "crates/scm-sdk/src/airs/gateway/workspaces.rs"),
            "apiKey": ("scm_sdk::auth::Authentication::runtime_key", "crates/scm-sdk/src/auth.rs"),
            "fetch": ("scm_sdk::ClientBuilder::transport", "crates/scm-sdk/src/engine.rs"),
        }
        if name not in table:
            return None
        native, source = table[name]
        if native.endswith("AuthContext::new"):
            probe = "let _ = |issuer: scm_sdk::Endpoint| { let _ = scm_sdk::auth::AuthContext::new(issuer, \"probe\", 1, \"1\"); };"
        else:
            probe = "let _ = " + native + ";"
        return dict(rustPaths=[native], sources=[source], tests=root["tests"], verification="explicit_client_configuration_adapter",
            deliberateDifferences=[f"Source {export['name']}.{name} is explicitly supplied through this native configuration capability; endpoints remain admitted, identity-scoped and caller-owned.",
                "Constructor retries use explicit RequestOptions; mutations never gain implicit replay authorization."], compileProbe=probe)

    classes = {}
    for export in inventory["exports"]:
        if export["kind"] != "class":
            continue
        related = [row for key, row in operation_by_member.items() if key.startswith(export["name"] + ".") and row["implementation"] == "implemented"]
        names = set()
        sources = set()
        for row in related:
            sources.add(row["source"])
            text = all_sources[row["source"]]
            if re.search(r"pub\s+struct\s+" + re.escape(export["name"]) + r"\b", text):
                names.add(export["name"])
        namespace = export["classification"]["namespace"]
        special = {"AIGatewayInferenceClient": "scm_sdk::airs::gateway::RuntimeClient",
                   "OAuthClient": "scm_sdk::auth::OAuthProvider", "AISecSDKException": "scm_sdk::Error",
                   "DlpNamespace": "scm_sdk::dlp::DlpClient",
                   "DlpProfilesClient": "scm_sdk::airs::management::DlpProfilesClient"}
        if export["name"] in special:
            classes[export["name"]] = special[export["name"]]
        elif export["name"] in names or any(re.search(r"pub\s+struct\s+" + re.escape(export["name"]) + r"\b", text) for text in all_sources.values()):
            classes[export["name"]] = "scm_sdk::" + namespace + "::" + export["name"]

    rows, gaps = [], []
    probe_lines = []
    public_roots = {}
    for export in inventory["exports"]:
        key = export["name"]
        root = None
        if key in manual:
            root = dict(manual[key])
        elif export["kind"] == "schema":
            schema = schema_by_name.get(key)
            if schema:
                root = dict(rustPaths=["scm_sdk::validation::parse"], sources=["crates/scm-sdk/src/validation.rs", "crates/scm-sdk/src/validation/schema_graph.json"],
                    tests=["crates/scm-sdk/tests/schema_contracts.rs::every_public_schema_and_manual_semantic_has_independent_oracle_evidence",
                           "crates/scm-sdk/tests/schema_contracts.rs::requests_match_original_frozen_zod_acceptance_and_normalized_outputs"],
                    deliberateDifferences=["Exported Zod objects are replaced by native named-schema validation, with typed non-JSON boundaries."],
                    verification="independent_zod_oracle", schemaId=schema["schema"],
                    compileProbe=f'let _ = scm_sdk::validation::parse("{key}", serde_json::Value::Null, scm_sdk::validation::Mode::Request);')
                components = native_components.get(schema["schema"], [])
                if components:
                    root["nativeBoundaryComponents"] = []
                    for boundary in components:
                        native_type = boundary["rustType"]
                        method = "validate_binary" if native_type == "BinaryFile" else "validate_date" if "NativeDate" in native_type \
                            else "validate_cancellation" if "RequestOptions" in native_type else "validate_transport"
                        path = "scm_sdk::validation::" + method
                        if path not in root["rustPaths"]:
                            root["rustPaths"].append(path)
                            root["compileProbe"] += " let _ = " + path + ";"
                        root["nativeBoundaryComponents"].append({**boundary, "rustValidator": path,
                            "domain": "explicit typed Rust component; JSON class spoofs rejected"})
                        if method == "validate_binary":
                            root["feature"] = "models"
                    root["tests"].append("crates/scm-sdk/tests/schema_contracts.rs::native_boundaries_accept_rust_values_and_reject_json_class_spoofs")
                    root["deliberateDifferences"].append("Non-JSON components use explicit native Date/BinaryPart/RequestOptions/Transport seams; JSON parse is not claimed to accept JavaScript classes.")
        elif export["kind"] in ("type_alias", "interface"):
            native = native_model(export)
            if native:
                model = model_by_name[key]
                sources = ["crates/scm-sdk/src/models/generated.rs"] if model["status"].startswith("typed_") else []
                if not sources:
                    short = re.sub(r"<.*", "", native).rsplit("::", 1)[-1]
                    sources = sorted({relative for relative, text in all_sources.items() if re.search(r"pub\s+(?:struct|enum|trait|type)\s+" + re.escape(short) + r"\b", text)})
                related = [row for name, row in operation_by_member.items()
                           if row["implementation"] == "implemented"
                           and row["baselineId"].split(":", 1)[0] == export["source"]["file"]]
                if not sources:
                    sources = sorted({row["source"] for row in related})
                root = dict(rustPaths=[native], sources=sources,
                    tests=sorted({test for row in related for test in row["tests"]}) or ["crates/scm-sdk/tests/schema_contracts.rs::every_public_schema_and_manual_semantic_has_independent_oracle_evidence"],
                    deliberateDifferences=[] if model["status"].startswith("typed_") else [model.get("reason", "Explicit native options adapter")],
                    verification="typed_public_contract", compileProbe=f'let _ = std::marker::PhantomData::<{native}>;')
                if model["status"].startswith("typed_"):
                    root["tests"].extend(["tools/generate_models.py", "crates/scm-sdk/src/models/tests.rs"])
        elif export["kind"] == "class" and key in classes:
            native = classes[key]
            related = [row for name, row in operation_by_member.items() if name.startswith(key + ".") and row["implementation"] == "implemented"]
            sources = sorted({row["source"] for row in related})
            tests = sorted({test for row in related for test in row["tests"]})
            if not sources:
                short = native.rsplit("::", 1)[-1]
                sources = [relative for relative, text in all_sources.items() if re.search(r"pub\s+struct\s+" + short + r"\b", text)]
                tests = ["crates/scm-sdk/tests/native_export_probes.rs::all_public_export_paths_compile"]
            if key in facade_tests:
                tests = [facade_tests[key]]
            root = dict(rustPaths=[native], sources=sources, tests=tests,
                deliberateDifferences=["Explicit Rust constructors and immutable caller-owned configuration replace JavaScript constructor options."],
                verification="typed_public_client", compileProbe=f'let _ = std::marker::PhantomData::<{native}>;')
            constructor = "builder" if key == "Content" else "new"
            if any(constructor_exists(relative, native, constructor) for relative in sources):
                root["rustPaths"].append(native + "::" + constructor)
                root["compileProbe"] += " let _ = " + native + "::" + constructor + ";"
        if root:
            public_roots[key] = root
        candidates = [(key, export, root)]
        for member in export.get("members", []) + export.get("staticMembers", []):
            member_key = member["id"]
            resolution = dict(manual[member_key]) if member_key in manual else None
            operation = operation_by_member.get(key + "." + member["name"])
            if operation is None:
                inherited_sources = {source["file"] for source in member.get("sources", [])}
                inherited = [row for operation_name, row in operation_by_member.items()
                             if operation_name.endswith("." + member["name"])
                             and row["baselineId"].split(":", 1)[0] in inherited_sources]
                if len(inherited) == 1:
                    operation = inherited[0]
            if resolution is None and operation and operation["implementation"] == "implemented" and key in classes:
                native_methods = [path.rsplit("::", 1)[-1] for path in operation["rustMethods"]]
                for method in native_methods:
                    if not re.search(r"pub\s+(?:async\s+)?fn\s+" + re.escape(method) + r"\s*[(<]", all_sources[operation["source"]]):
                        raise ValueError("Mapped native public method is absent: " + member_key)
                paths = [classes[key] + "::" + method for method in native_methods]
                resolution = dict(rustPaths=paths, sources=[operation["source"]], tests=operation["tests"],
                    deliberateDifferences=operation["deliberateDifferences"], verification="independent_operation_contract",
                    operationId=operation["baselineId"], compileProbe=" ".join("let _ = " + path + ";" for path in paths))
            if resolution is None and root and member["kind"] == "property" and export["kind"] == "interface":
                native = root["rustPaths"][0]
                field = model_field(native, member["name"])
                if field:
                    relative, access = field
                    resolution = dict(rustPaths=[native + "." + access], sources=[relative], tests=root["tests"],
                        deliberateDifferences=root["deliberateDifferences"], verification="typed_public_field",
                        compileProbe=f'let _ = |value: &{native}| {{ let _ = &value.{access}; }};')
                elif member["name"] == "max" and "ListAll" in key:
                    resolution = dict(rustPaths=["scm_sdk::pagination::CollectOptions.max_items"], sources=["crates/scm-sdk/src/pagination.rs"],
                        tests=["crates/scm-sdk/src/pagination/tests.rs"], deliberateDifferences=["Source collection max is an explicit bounded CollectOptions.max_items, separate from query filters."],
                        verification="bounded_collection_adapter", compileProbe="let _ = |value: &scm_sdk::pagination::CollectOptions| { let _ = value.max_items; };")
                if resolution is None:
                    resolution = client_option_field(export, member, root)
            if resolution is None and root and export["kind"] == "class" and member["kind"] in ("property", "accessor"):
                method = snake(member["name"])
                native = root["rustPaths"][0]
                source = next((relative for relative in root["sources"] if re.search(r"pub\s+fn\s+" + method + r"\s*\(", all_sources[relative])), None)
                if source:
                    resolution = dict(rustPaths=[native + "::" + method], sources=[source], tests=root["tests"],
                        deliberateDifferences=["JavaScript property/accessor is an explicit Rust accessor method."], verification="public_accessor",
                        compileProbe="let _ = " + native + "::" + method + ";")
            if (resolution and operation and operation["implementation"] == "implemented"
                    and member["kind"] == "method"):
                if resolution.get("operationId", operation["baselineId"]) != operation["baselineId"]:
                    raise ValueError("Explicit capability claims a different canonical operation")
                resolution["operationId"] = operation["baselineId"]
            candidates.append((member_key, member, resolution))
        for capability, frozen, resolution in candidates:
            if resolution is None:
                gaps.append({"baselineId": capability, "kind": frozen["kind"], "owner": key})
                continue
            resolution = evidence(resolution)
            probe = resolution.pop("compileProbe", None)
            if not probe:
                gaps.append({"baselineId": capability, "kind": frozen["kind"], "owner": key, "reason": "missing public compile probe"})
                continue
            for native_path in resolution["rustPaths"]:
                owner, separator, field = native_path.partition(".")
                method_owner, _, method = native_path.rpartition("::")
                instance_call = method_owner in probe and re.search(r"\." + re.escape(method) + r"\b", probe)
                if native_path not in probe and not (separator and owner in probe and "." + field in probe) and not instance_call:
                    raise ValueError("Public probe does not reference its claimed capability: " + capability)
            index = len(probe_lines)
            probe_lines.append(f"    // {capability}\n    {{ {probe} }}")
            row = {"baselineId": capability, "kind": frozen["kind"], "namespace": classification["entries"][capability]["namespace"],
                   "baselineFeature": classification["entries"][capability]["feature"], "implementation": "implemented",
                   "baselineSources": frozen.get("sources", frozen.get("declarations", [frozen.get("source")])),
                   **resolution, "probeIndex": index, "liveAcceptance": "unverified"}
            if operation_id := row.get("operationId"):
                if row["kind"] != "method":
                    raise ValueError("Only an exact public method can inherit operation live evidence: " + capability)
                operation = next((value for value in operations["operations"]
                                  if value["baselineId"] == operation_id), None)
                if operation is None or operation["implementation"] != "implemented":
                    raise ValueError("Capability refers to an absent or unimplemented canonical operation")
                row["liveAcceptance"] = operation["liveAcceptance"]
                if "liveEvidence" in operation:
                    row["liveEvidence"] = operation["liveEvidence"]
            row["requiredFeatures"] = feature_for(resolution["rustPaths"])
            explicit_feature = resolution.get("feature")
            if explicit_feature and explicit_feature not in row["requiredFeatures"]:
                row["requiredFeatures"].append(explicit_feature.replace("_", "-"))
                row["requiredFeatures"].sort()
            rows.append(row)
    expected = set(classification["entries"])
    if len(rows) != len({row["baselineId"] for row in rows}) or set(manual) - expected:
        raise ValueError("Duplicate or extraneous native exported capability")
    required_features = sorted({feature for row in rows for feature in row["requiredFeatures"]})
    probe_gate = "#![cfg(all(\n" + ",\n".join('    feature = "' + feature + '"' for feature in required_features) + "\n))]\n"
    probe = "//! Generated public capability probes; every expression compiles outside the library.\n" + probe_gate + \
            "// Futures are deliberately type-checked without polling or dispatching.\n#![allow(clippy::let_underscore_future)]\n" \
            "#[rustfmt::skip]\n#[test]\nfn all_public_export_paths_compile() {\n" + "\n".join(probe_lines) + "\n}\n"
    result = {"schemaVersion": 1, "baselineCommit": inventory["baseline"]["commit"],
        "policy": "Every frozen root/member has concrete native API evidence and a compiled external public probe; explicit language adaptations never count as missing capabilities.",
        "jsonBoundaryContract": operations["jsonBoundaryContract"],
        "liveReceipt": live_receipt,
        "generatorSha256": sha(pathlib.Path(__file__)), "summary": {"frozenRoots": 2630, "frozenCapabilities": len(expected),
            "implemented": len(rows), "unresolved": len(gaps),
            "liveVerified": sum(row["liveAcceptance"] == "verified" for row in rows),
            "liveVerifiedOperations": len({row["operationId"] for row in rows
                                          if row["liveAcceptance"] == "verified"}),
            "liveRecordedCapabilities": sum("liveEvidence" in row for row in rows)},
        "sourceBindings": [{"path": relative, "sha256": sha(regular(relative))} for relative in sorted(bound) if relative != PROBES.relative_to(ROOT).as_posix()],
        "probe": {"requiredFeatures": required_features, "path": PROBES.relative_to(ROOT).as_posix(), "sha256": hashlib.sha256(probe.encode()).hexdigest()}, "capabilities": rows}
    return result, probe, gaps


def consumer_files(probe):
    # This is a genuinely separate Cargo package with no access to crate-private
    # items. It shares the committed dependency resolution, not a fresh solver.
    source = "//! Generated standalone public SDK capability consumer.\n#![allow(clippy::let_underscore_future)]\n#[rustfmt::skip]\nfn main()" \
        + probe.split("fn all_public_export_paths_compile()", 1)[1]
    manifest = '''[package]
name = "scm-sdk-export-probes"
version = "0.0.0"
edition = "2024"
rust-version = "1.89"
publish = false

[workspace]

[dependencies]
scm-sdk = { path = "../../crates/scm-sdk", default-features = false, features = ["scan", "management", "gateway", "model-security", "red-team", "realtime", "ws-transport"] }
futures-util = "=0.3.34"
serde_json = "=1.0.149"
'''
    # Read exact root package versions rather than guessing independent patches.
    import tomllib
    sdk_manifest = tomllib.loads((ROOT / "crates/scm-sdk/Cargo.toml").read_text())
    features = sorted(set(sdk_manifest["features"]) - {"default"})
    manifest = re.sub(r'features = \[[^\]]+\]', 'features = ' + json.dumps(features), manifest)
    packages = tomllib.loads((ROOT / "Cargo.lock").read_text())["package"]
    serde_version = next(item["version"] for item in packages if item["name"] == "serde_json")
    manifest = manifest.replace('serde_json = "=1.0.149"', 'serde_json = "=' + serde_version + '"')
    lock = (ROOT / "Cargo.lock").read_text()
    # The workspace's root SDK entry includes dev-only direct dependencies.
    # A downstream package excludes them, while preserving every pinned package
    # version/checksum and all transitive entries from the reviewed lock.
    def normal_dependencies(match):
        allowed = set(sdk_manifest["dependencies"])
        lines = [line for line in match[2].splitlines(keepends=True)
                 if line.strip().strip('\",').split(" ", 1)[0] in allowed]
        return match[1] + "".join(lines) + match[3]
    lock, replaced = re.subn(r'(\[\[package\]\]\nname = "scm-sdk"\n[^\[]*dependencies = \[\n)([^\]]*)(\]\n)',
                            normal_dependencies, lock, count=1)
    if replaced != 1:
        raise ValueError("SDK lock entry cannot be mapped into standalone consumer")
    lock += '''
[[package]]
name = "scm-sdk-export-probes"
version = "0.0.0"
dependencies = [
 "futures-util",
 "scm-sdk",
 "serde_json",
]
'''
    return {"Cargo.toml": manifest, "Cargo.lock": lock, "src/main.rs": source}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--report", action="store_true", help="Read-only list of unresolved mappings while implementing")
    args = parser.parse_args()
    result, probe, gaps = generate()
    if args.report:
        print(json.dumps({"summary": result["summary"], "gaps": gaps}, indent=2))
        return
    if gaps:
        raise ValueError("Native export closure is incomplete: " + str(len(gaps)) + " capabilities; use --report")
    consumer = consumer_files(probe)
    result["consumer"] = {"directory": CONSUMER.relative_to(ROOT).as_posix(),
        "files": [{"path": name, "sha256": hashlib.sha256(contents.encode()).hexdigest()} for name, contents in consumer.items()]}
    encoded = json.dumps(result, indent=2, sort_keys=False) + "\n"
    if args.check:
        if OUTPUT.read_text() != encoded or PROBES.read_text() != probe:
            raise ValueError("Native export ledger/public probes are stale")
        if any((CONSUMER / name).read_text() != contents for name, contents in consumer.items()):
            raise ValueError("Standalone native public consumer is stale")
    else:
        OUTPUT.write_text(encoded)
        PROBES.write_text(probe)
        for name, contents in consumer.items():
            path = CONSUMER / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text(contents)
    print(json.dumps({"checked": args.check, **result["summary"]}))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, FileNotFoundError) as error:
        sys.exit("Native export verification failed: " + str(error))
