#!/usr/bin/env python3
"""Bind the 194 Phase 6 Gateway routes/schema roots to exact frozen method sources.

This supplements executed wire fixtures; it is a structural source check, not a
claim that source-token matching proves complete semantic equivalence.
"""
import argparse
import hashlib
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
OUTPUT = ROOT / "contracts/native-gateway-bindings.json"
# Reviewed RuntimeClient::request preserves ordered segment insertion and shared
# routing/replay application. Any semantic edit requires an explicit pin review.
RUNTIME_REQUEST_TOKENS_SHA256 = "cc1494298b8ebcfa8f841a7d6ac14e1d76dd019608e42f71cfe5907cbb2a228f"
TOKEN = re.compile(r'"(?:\\.|[^"\\])*"|//[^\n]*|/\*[\s\S]*?\*/|[A-Za-z_][A-Za-z_0-9]*|::|[^\s]')


def digest(value):
    return hashlib.sha256(value.encode() if isinstance(value, str) else value).hexdigest()


def tokens(source):
    return [token for token in TOKEN.findall(source) if not token.startswith(("//", "/*"))]


def functions(source):
    values = tokens(source)
    result = {}
    for index, token in enumerate(values):
        if token != "fn" or index + 2 >= len(values):
            continue
        name = values[index + 1]
        start = index + 2
        while start < len(values) and values[start] not in ("{", ";"):
            start += 1
        if start == len(values) or values[start] != "{":
            continue
        end, depth = start + 1, 1
        while depth and end < len(values):
            depth += (values[end] == "{") - (values[end] == "}")
            end += 1
        if depth:
            raise ValueError("Unbalanced Rust method body")
        if name in result:
            if name in ("new", "default", "fmt", "query", "as_str"):
                result[name] = None
                continue
            raise ValueError("Ambiguous Rust method name in contract source: " + name)
        result[name] = values[index:end]
    return {name: body for name, body in result.items() if body is not None}


def expanded(function_map, name):
    if name not in function_map:
        raise ValueError("Native method is absent: " + name)
    used, pending = {}, [name]
    while pending:
        current = pending.pop()
        if current in used:
            continue
        body = function_map[current]
        used[current] = body
        for index in range(len(body) - 3):
            if body[index:index + 2] == ["self", "."] and body[index + 3] in ("(", "<"):
                helper = body[index + 2]
                if helper in function_map:
                    pending.append(helper)
        if len(used) > 16:
            raise ValueError("Native helper expansion exceeds its reviewed bound")
    return used


def source_schemas(operation):
    texts = [operation["implementationSource"]] + [helper["source"] for helper in operation["helpers"]]
    return set(re.findall(r"\b[A-Z][A-Za-z0-9_]*Schema\b", "\n".join(texts)))


def arguments(values, opening):
    pairs = {"(": ")", "[": "]", "{": "}", "<": ">"}
    stack, parts, start = [pairs[values[opening]]], [], opening + 1
    for index in range(start, len(values)):
        token = values[index]
        if token in pairs:
            stack.append(pairs[token])
        elif stack and token == stack[-1]:
            stack.pop()
            if not stack:
                if index > start:
                    parts.append(values[start:index])
                return parts, index + 1
        elif token == "," and len(stack) == 1:
            parts.append(values[start:index])
            start = index + 1
    raise ValueError("Unbalanced native call arguments")


def path_value(value, environment):
    while value and value[0] in ("&", "*"):
        value = value[1:]
    if len(value) == 1 and value[0] in environment:
        return path_value(environment[value[0]], {})
    if len(value) == 1 and value[0].startswith('"'):
        return re.sub(r"\{[^}]+\}", "{}", json.loads(value[0]))
    return "{}"


def route_templates(function_map, name, environment=None, active=()):
    if name in active or len(active) > 16:
        raise ValueError("Recursive/unbounded native route helper")
    environment = environment or {}
    body = function_map[name]
    routes = set()
    for index in range(len(body) - 4):
        if body[index:index + 2] == ["RawRequest", "::"] and body[index + 2] in ("read", "new") and body[index + 3] == "(":
            args, end = arguments(body, index + 3)
            route = path_value(args[0 if body[index + 2] == "read" else 1], environment)
            while end < len(body):
                if body[end] == "?":
                    end += 1
                    continue
                if end + 3 >= len(body) or body[end] != "." or body[end + 2] != "(":
                    break
                method = body[end + 1]
                chained, end = arguments(body, end + 2)
                if method == "segment":
                    route += "/" + path_value(chained[0], environment)
            routes.add(route)
        if body[index:index + 2] != ["self", "."] or body[index + 3] != "(":
            continue
        helper = body[index + 2]
        args, _ = arguments(body, index + 3)
        if helper == "request" and len(args) == 4:
            # RuntimeClient::request uses a finite ordered segment slice. Its
            # exact implementation is separately pinned in generate().
            route = path_value(args[1], environment)
            segment_tokens = args[2]
            if segment_tokens[:2] != ["&", "["]:
                raise ValueError("Runtime route segments require an explicit ordered slice")
            segments, _ = arguments(segment_tokens, 1)
            for segment in segments:
                route += "/" + path_value(segment, environment)
            routes.add(route)
        elif helper in function_map and helper != "request":
            function = function_map[helper]
            params, _ = arguments(function, function.index("("))
            params = [param for param in params if "self" not in param]
            if len(params) != len(args):
                raise ValueError("Native helper arguments are not completely bound: " + helper)
            bindings = {param[0]: environment.get(arg[0], arg) if len(arg) == 1 else arg for param, arg in zip(params, args)}
            routes.update(route_templates(function_map, helper, bindings, active + (name,)))
    return routes


def validate_operation(operation, mapping, source, schema_inventory=None):
    functions_map = functions(source)
    native_methods = [path.rsplit("::", 1)[-1] for path in mapping["rustMethods"]]
    used = {}
    for method in native_methods:
        used.update(expanded(functions_map, method))
    native_tokens = [token for body in used.values() for token in body]
    native_strings = [json.loads(token) for token in native_tokens if token.startswith('"')]
    calls = operation["callSiteEvidence"]
    if len(calls) != 1:
        raise ValueError("Gateway operation needs explicit single-call evidence: " + operation["id"])
    call = calls[0]
    if call.get("method") not in {"GET", "POST", "PUT", "PATCH", "DELETE"} or not call.get("path"):
        raise ValueError("Incomplete frozen Gateway request evidence")
    http_methods = set(re.findall(r"Method :: (GET|POST|PUT|PATCH|DELETE)\b", " ".join(native_tokens)))
    if "RawRequest :: read" in " ".join(native_tokens):
        http_methods.add("GET")
    if http_methods != {call["method"]}:
        raise ValueError("Gateway HTTP method binding differs: " + operation["id"])
    route = re.sub(r"\{[^}]+\}", "{}", call["path"])
    helper_proofs = []
    if call["path"] == "{aiGwOrganisationsAuthSettingsPath(tsgId)}":
        candidates = [helper for helper in operation["namedFunctionEvidence"]
                      if helper["name"] == "aiGwOrganisationsAuthSettingsPath"]
        if len(candidates) != 1:
            raise ValueError("Auth-settings helper lacks exact source evidence")
        helper = candidates[0]
        returned = re.search(r"return\s+`([^`]+)`;", helper["source"])
        if not returned or returned[1] != "/organisations/${tsgId}/auth-settings":
            raise ValueError("Unreviewed auth-settings helper expression")
        route = returned[1].replace("${tsgId}", "{}")
        helper_proofs.append({"name": helper["name"], "sourcePath": helper["sourcePath"],
                              "sourceFileSha256": helper["sourceFileSha256"], "expressionSha256": digest(helper["source"])})
    static = route.split("/{", 1)[0]
    route_literals = [value for value in native_strings if value.startswith("/")]
    # The frozen path's initial literal prefix must be supplied by this exact
    # native method/helper closure. Telemetry chart helpers append one reviewed
    # literal metric argument to /logs/charts.
    if static not in route_literals:
        if operation["className"] == "AIGatewayTelemetryClient" and static.startswith(("/logs/charts/", "/logs/groups/")):
            prefix, _, metric = static.rpartition("/")
            root_strings = [json.loads(token) for name in native_methods for token in functions_map[name] if token.startswith('"')]
            if prefix not in route_literals or metric not in root_strings:
                raise ValueError("Telemetry metric route differs: " + operation["id"])
        else:
            raise ValueError("Gateway literal route prefix differs: " + operation["id"] + " " + static)
    for suffix in re.split(r"\{\}", route)[1:]:
        for part in suffix.strip("/").split("/"):
            if part and part not in native_strings:
                raise ValueError("Gateway literal route suffix differs: " + operation["id"])
    actual_routes = set()
    for method in native_methods:
        actual_routes.update(route_templates(functions_map, method))
    if actual_routes != {route}:
        raise ValueError("Gateway complete route topology differs: " + operation["id"] + " " + str(sorted(actual_routes)))
    frozen_schemas = source_schemas(operation)
    native_schemas = {value.removesuffix("[]") for value in native_strings if re.fullmatch(r"[A-Z]\w*Schema(?:\[\])?", value)}
    missing = frozen_schemas - native_schemas
    extra = native_schemas - frozen_schemas
    decompositions = []
    filtered = {"cost", "requests", "latency", "tokens", "byUser", "groupBy", "byStatusCode"}
    if (operation["className"] == "AIGatewayTelemetryClient" and operation["method"] in filtered
            and extra == {"AIGatewayChartFiltersSchema"} and schema_inventory is not None):
        # This is the sole reviewed extra native schema: window/group options
        # compose the public filter object with typed window/column adapters.
        # Prove actual frozen node identity for every shared filter field and
        # the same validChartBounds refinement, rather than trusting a name.
        roots = {root["name"]: root for root in schema_inventory["schemas"]["roots"]
                 if root["source"]["file"] == "src/ai-gateway/telemetry-client.ts"}
        source_root = "groupOptionsSchema" if operation["method"] in {"groupBy", "byStatusCode"} else "filteredChartOptionsSchema"
        definitions = schema_inventory["schemas"]["definitions"]
        public_root, composed_root = roots["AIGatewayChartFiltersSchema"], roots[source_root]
        public_effect, composed_effect = definitions[public_root["schema"]["$ref"]], definitions[composed_root["schema"]["$ref"]]
        if not all(node["kind"] == "ZodEffects" and node["semanticSource"]["source"]["text"].endswith(".refine(validChartBounds)")
                   for node in (public_effect, composed_effect)):
            raise ValueError("Telemetry filter refinement decomposition changed")
        public_fields = definitions[public_effect["inner"]["$ref"]]["properties"]
        composed_fields = definitions[composed_effect["inner"]["$ref"]]["properties"]
        if not public_fields or any(composed_fields.get(key) != value for key, value in public_fields.items()):
            raise ValueError("Telemetry filter schema decomposition changed")
        decompositions.append({"publicRoot": public_root["schema"]["$ref"], "composedRoot": composed_root["schema"]["$ref"],
            "publicDeclarationSha256": public_root["source"]["sha256"], "composedDeclarationSha256": composed_root["source"]["sha256"],
            "sharedFieldSchemaIds": {key: value["schema"]["$ref"] for key, value in public_fields.items()},
            "difference": "Native window/column options compose the frozen public filter root; all shared fields and the bounds refinement are identical."})
        extra = set()
    if missing or extra:
        raise ValueError("Gateway schema roots differ: " + operation["id"] + " missing=" + ",".join(sorted(missing)) + " extra=" + ",".join(sorted(extra)))
    return {"baselineId": operation["id"], "method": call["method"], "route": route,
        "plane": operation["endpoint"]["plane"], "frozenSource": operation["source"],
        "implementationSha256": digest(operation["implementationSource"]),
        "namedHelperProofs": helper_proofs,
        "schemaDecompositions": decompositions,
        "frozenSchemaRoots": sorted(frozen_schemas), "nativeSchemaRoots": sorted(native_schemas),
        "nativeMethods": native_methods, "nativeSource": mapping["source"], "nativeSourceSha256": digest(source),
        "nativeRouteTemplates": sorted(actual_routes),
        "nativeHelperProofs": [{"method": name, "tokensSha256": digest(json.dumps(body, separators=(",", ":")))} for name, body in sorted(used.items())],
        "tests": mapping["tests"]}


def generate():
    inventory = json.loads((ROOT / "contracts/operations.json").read_text())
    schemas = json.loads((ROOT / "contracts/typescript.json").read_text())
    native = json.loads((ROOT / "contracts/native-operations.json").read_text())
    if inventory["baseline"]["commit"] != "4a80dbdb3dc14365e1c25d838ac0bd139485d48c" or native["baselineCommit"] != inventory["baseline"]["commit"]:
        raise ValueError("Gateway binding baseline differs")
    mappings = {row["baselineId"]: row for row in native["operations"]}
    runtime_source = (ROOT / "crates/scm-sdk/src/airs/gateway/runtime.rs").read_text()
    runtime_helper = functions(runtime_source)["request"]
    if digest(json.dumps(runtime_helper, separators=(",", ":"))) != RUNTIME_REQUEST_TOKENS_SHA256:
        raise ValueError("Runtime ordered-path helper changed without an explicit source review")
    rows, failures = [], []
    for operation in inventory["operations"]:
        mapping = mappings[operation["id"]]
        if operation["family"] != "ai-gateway" or mapping["owningPhase"] != 6:
            continue
        if mapping["implementation"] != "implemented":
            raise ValueError("Phase 6 Gateway method lacks implementation")
        source = (ROOT / mapping["source"]).read_text()
        try:
            rows.append(validate_operation(operation, mapping, source, schemas))
        except ValueError as error:
            failures.append(str(error))
    if len(rows) + len(failures) != 194:
        raise ValueError("Phase 6 Gateway binding denominator differs")
    result = {"schemaVersion": 1, "baselineCommit": inventory["baseline"]["commit"],
        "policy": "Exact frozen method/helper source-bound schema roots and literal route/method checks supplement independent executed wire contracts; no general JavaScript/Rust semantic equivalence claim.",
        "inputs": [{"path": file, "sha256": digest((ROOT / file).read_bytes())} for file in ["tools/verify_gateway_bindings.py", "contracts/operations.json", "contracts/typescript.json", "contracts/native-operations.json"]],
        "count": len(rows), "operations": rows}
    result["runtimeRequestHelper"] = {"path": "crates/scm-sdk/src/airs/gateway/runtime.rs", "tokensSha256": RUNTIME_REQUEST_TOKENS_SHA256}
    return result, failures


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--report", action="store_true")
    args = parser.parse_args()
    result, failures = generate()
    if args.report:
        print(json.dumps({"checked": len(result["operations"]), "failures": failures}, indent=2))
        return
    if failures:
        raise ValueError("\n".join(failures))
    expected = json.dumps(result, indent=2) + "\n"
    if args.check:
        if OUTPUT.read_text() != expected:
            raise ValueError("Gateway source binding proof is stale")
    else:
        OUTPUT.write_text(expected)
    print(json.dumps({"checked": args.check, "gatewayPhaseSixMethods": result["count"]}))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, KeyError, FileNotFoundError) as error:
        sys.exit("Gateway source verification failed: " + str(error))
