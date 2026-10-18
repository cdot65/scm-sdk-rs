#!/usr/bin/env python3
"""Compile the separately pinned vendor dataplane contract; never alter the TS baseline."""
import argparse
import copy
import hashlib
import json
import pathlib
import re
import shutil
import subprocess

ROOT = pathlib.Path(__file__).resolve().parents[1]
VENDOR = ROOT / "contracts/vendor/red-team-data-plane/0.93.0"
PREFIX = "vendor:red-team-dp:0.93.0:"
WAIVERS = {
    ("AttackDetailResponseSchema", "audio_technique"),
    ("AttackMultiTurnDetailResponseSchema", "audio_technique"),
    ("AttackOutputResponseSchema", "judge_reasoning"),
    ("AttackMultiTurnOutputResponseSchema", "judge_reasoning"),
}
WAIVER_POINTERS = {("#/components/schemas/" + name, field) for name, field in WAIVERS}
METHODS = {"get", "post", "put", "patch", "delete", "head", "options", "trace"}
ANNOTATIONS = {"title", "description", "example", "examples", "default", "readOnly"}
KEYWORDS = ANNOTATIONS | {"$ref", "type", "properties", "required", "additionalProperties",
    "propertyNames", "items", "anyOf", "oneOf", "discriminator", "const", "enum", "format",
    "minimum", "maximum", "minLength", "maxLength", "minItems", "maxItems"}

NEW = {
    "PUT /v1/scan/{job_id}/runtime-profile": ("scans", "RedTeamScansClient", "set_runtime_profile"),
    "GET /v1/report/static/{job_id}/asr": ("reports", "RedTeamReportsClient", "get_static_asr"),
    "GET /v1/report/dynamic/{job_id}/asr": ("reports", "RedTeamReportsClient", "get_dynamic_asr"),
    "POST /v1/report/{job_id}/regenerate": ("reports", "RedTeamReportsClient", "regenerate"),
    "GET /v1/report/{job_id}/status": ("reports", "RedTeamReportsClient", "get_status"),
    "POST /v1/report/static/{job_id}/attack/{attack_id}/override": ("reports", "RedTeamReportsClient", "override_attack"),
    "POST /v1/report/static/{job_id}/attack-multi-turn/{attack_id}/override": ("reports", "RedTeamReportsClient", "override_multi_turn_attack"),
    "POST /v1/report/dynamic/{job_id}/stream/{stream_id}/override": ("reports", "RedTeamReportsClient", "override_stream"),
    "POST /v1/custom-attacks/job/{job_id}/attack/{attack_id}/override": ("custom_attack_reports", "RedTeamCustomAttackReportsClient", "override_attack"),
    "GET /v1/error-log/job/{job_id}/download": ("facade", "RedTeamClient", "download_error_logs"),
    "GET /v2/report/{job_id}/download": ("reports", "RedTeamReportsClient", "get_download_url"),
}
# These options remain source compatible, but accepting them would discard caller intent.
REJECTED = {
    "list_attacks": ["search"], "list_goal_streams": ["skip", "limit", "search"],
    "get_prompt_sets": ["search"], "get_prompts_by_set": ["search"],
    "list_custom_attacks": ["search"], "get_error_logs": ["search"],
    "get_target_profile_error_logs": ["skip", "search"],
    "get_scan_statistics": ["date_range", "target_id"],
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def read_yaml(path):
    script = "const fs=require('fs'),y=require('./tools/node_modules/js-yaml');if(require('./tools/node_modules/js-yaml/package.json').version!=='4.3.2')throw Error('Unreviewed parser');process.stdout.write(JSON.stringify(y.load(fs.readFileSync(process.argv[1],'utf8'))));"
    return json.loads(subprocess.check_output(["node", "-e", script, str(path)], cwd=ROOT, text=True))


def load():
    pin = json.loads((VENDOR / "source.json").read_text())
    source = ROOT / pin["path"]
    if source.is_symlink() or sha(source.read_bytes()) != pin["sha256"] or source.stat().st_size != pin["bytes"]:
        raise ValueError("Vendor source differs from reviewed Phase 9 pin")
    notice = ROOT / pin["licensePath"]
    if notice.is_symlink() or sha(notice.read_bytes()) != pin["licenseSha256"]:
        raise ValueError("Vendor MIT notice missing or changed")
    spec = json.loads((VENDOR / "openapi.json").read_text())
    if spec != read_yaml(source):
        raise ValueError("Canonical vendor JSON differs from pinned YAML")
    if spec["info"]["version"] != "0.93.0" or spec["openapi"] != "3.1.0":
        raise ValueError("Unexpected vendor version")
    ops = [(method.upper(), path, op) for path, item in spec["paths"].items()
           for method, op in item.items() if method in METHODS]
    if len(ops) != pin["expectedOperations"] or pin["expectedOperations"] != 47:
        raise ValueError("Operation recount differs from reviewed plan")
    return pin, spec, ops


class Compiler:
    """Lower only the schema vocabulary actually reviewed for this vendor source."""
    def __init__(self, spec, response=False):
        self.spec, self.response = spec, response
        self.nodes, self.roots = {}, {}
        self.active = set()
        self.inputs = {}
        for name, schema in spec["components"]["schemas"].items():
            self.roots[name] = self.compile(schema, "#/components/schemas/" + name)

    def ident(self, pointer):
        return "sdp_" + ("r_" if self.response else "q_") + sha(pointer.encode())[:24]

    def node(self, pointer, value):
        sid = self.ident(pointer)
        if sid in self.nodes and self.nodes[sid] != value:
            raise ValueError("Schema pointer collision: " + pointer)
        self.nodes[sid] = value
        return sid

    def compile(self, s, pointer):
        if pointer in self.inputs and self.inputs[pointer] != s:
            raise ValueError("Schema pointer reused with different input: " + pointer)
        self.inputs[pointer] = copy.deepcopy(s)
        if pointer in self.active:
            raise ValueError("Recursive schema requires an explicit adapter: " + pointer)
        self.active.add(pointer)
        try:
            return self._compile(s, pointer)
        finally:
            self.active.remove(pointer)

    def _compile(self, s, pointer):
        if not isinstance(s, dict) or set(s) - KEYWORDS:
            raise ValueError(f"Unsupported schema keyword/shape at {pointer}: {set(s) - KEYWORDS if isinstance(s,dict) else s}")
        sid = self.ident(pointer)
        if sid in self.nodes:
            return sid
        if "$ref" in s:
            if set(s) - ANNOTATIONS - {"$ref"}:
                raise ValueError("Unimplemented ref siblings: " + pointer)
            ref = s["$ref"]
            if not ref.startswith("#/components/schemas/"):
                raise ValueError("Only local component references are supported")
            name = ref.removeprefix("#/components/schemas/")
            return self.compile(self.spec["components"]["schemas"][name], ref)
        if "const" in s:
            if set(s) - ANNOTATIONS - {"const", "type"}:
                raise ValueError("Unimplemented const siblings: " + pointer)
            return self.node(pointer, {"kind": "ZodLiteral", "value": s["const"]})
        if "enum" in s:
            if set(s) - ANNOTATIONS - {"enum", "type"}:
                raise ValueError("Unimplemented enum siblings: " + pointer)
            if not all(isinstance(v, str) for v in s["enum"]):
                raise ValueError("Non-string enum needs an explicit adapter")
            return self.node(pointer, {"kind": "ZodEnum", "values": s["enum"]})
        branches = s.get("anyOf", s.get("oneOf"))
        if branches is not None:
            if set(s) - ANNOTATIONS - {"anyOf", "oneOf", "discriminator"}:
                raise ValueError("Unimplemented union siblings: " + pointer)
            key = "oneOf" if "oneOf" in s else "anyOf"
            variants = [{"$ref": self.compile(v, pointer + f"/{key}/{i}")} for i, v in enumerate(branches)]
            # A union of string vocabularies has one unambiguous wire type. Flatten
            # it so an open enum in the first serde branch cannot swallow a known
            # value belonging to another enum branch.
            vocabulary, nulls = [], False
            for variant in variants:
                branch = self.nodes[variant["$ref"]]
                if branch["kind"] == "ZodNull":
                    nulls = True
                elif branch["kind"] == "ZodEnum":
                    vocabulary.extend(branch["values"])
                elif branch["kind"] == "ZodLiteral" and isinstance(branch["value"], str):
                    vocabulary.append(branch["value"])
                else:
                    break
            else:
                if vocabulary and key == "anyOf":
                    enum = {"kind": "ZodEnum", "values": list(dict.fromkeys(vocabulary))}
                    if not nulls:
                        return self.node(pointer, enum)
                    inner = self.node(pointer + "/vocabulary", enum)
                    return self.node(pointer, {"kind": "ZodNullable", "inner": {"$ref": inner}})
            node = {"kind": "ZodUnion", "variants": variants}
            if "discriminator" in s:
                d = s["discriminator"]
                if set(d) - {"propertyName", "mapping"}:
                    raise ValueError("Unsupported discriminator")
                mapping = []
                for value, ref in d.get("mapping", {}).items():
                    target = self.compile({"$ref": ref}, pointer + "/discriminator/" + value)
                    mapping.append({"value": value, "schema": {"$ref": target}})
                if not mapping:
                    raise ValueError("Explicit discriminator mapping required")
                if {m["schema"]["$ref"] for m in mapping} != {v["$ref"] for v in variants} or len(mapping) != len(variants):
                    raise ValueError("Discriminator must cover every oneOf variant exactly once")
                for item in mapping:
                    target = self.nodes[item["schema"]["$ref"]]
                    if target["kind"] != "ZodObject" or d["propertyName"] not in target["properties"]:
                        raise ValueError("Discriminator target must declare its property")
                    tag = self.nodes[target["properties"][d["propertyName"]]["schema"]["$ref"]]
                    if tag["kind"] == "ZodOptional":
                        tag = self.nodes[tag["inner"]["$ref"]]
                    if tag.get("kind") != "ZodLiteral" or tag.get("value") != item["value"]:
                        raise ValueError("Discriminator mapping and literal differ")
                node.update(kind="ZodDiscriminatedUnion", discriminator=d["propertyName"],
                            discriminatorMapping=mapping, preserveUnknownResponse=True)
            elif key == "oneOf":
                raise ValueError("oneOf without a discriminator needs an exclusivity adapter")
            return self.node(pointer, node)
        typ = s.get("type")
        allowed = {
            "object": {"properties", "required", "additionalProperties", "propertyNames"},
            "array": {"items", "minItems", "maxItems"},
            "string": {"format", "minLength", "maxLength"},
            "integer": {"minimum", "maximum"}, "number": {"minimum", "maximum"},
            "boolean": set(), "null": set(), None: set(),
        }
        if typ in allowed and set(s) - ANNOTATIONS - {"type"} - allowed[typ]:
            raise ValueError("Unimplemented schema siblings at " + pointer)
        if typ == "object":
            props = s.get("properties", {})
            extra = s.get("additionalProperties", True)
            if not props and isinstance(extra, dict):
                key = self.compile(s.get("propertyNames", {"type": "string"}), pointer + "/propertyNames")
                value = self.compile(extra, pointer + "/additionalProperties")
                return self.node(pointer, {"kind": "ZodRecord", "key": {"$ref": key}, "value": {"$ref": value}, "preserveReservedKeys": True})
            if "propertyNames" in s:
                raise ValueError("Property name checks on mixed objects need an adapter")
            required = set(s.get("required", []))
            if not required.issubset(props):
                raise ValueError("Required property is not declared")
            fields = {}
            for key, field in props.items():
                child = self.compile(field, pointer + "/properties/" + key)
                if key not in required or (self.response and (pointer, key) in WAIVER_POINTERS):
                    child = self.node(pointer + "/optional/" + key, {"kind": "ZodOptional", "inner": {"$ref": child},
                        "responseOnly": key in required})
                fields[key] = {"schema": {"$ref": child}}
            catchall = self.compile(extra, pointer + "/additionalProperties") if isinstance(extra, dict) else self.node(pointer + "/catchall", {"kind": "ZodNever"})
            return self.node(pointer, {"kind": "ZodObject", "properties": fields,
                "unknownFields": "strict" if extra is False else "passthrough", "catchall": {"$ref": catchall}, "preserveReservedKeys": True})
        if typ == "array":
            if "items" not in s:
                raise ValueError("Array needs items")
            node = {"kind": "ZodArray", "element": {"$ref": self.compile(s["items"], pointer + "/items")}}
            for a, b in [("minItems", "minLength"), ("maxItems", "maxLength")]:
                if a in s:
                    node[b] = {"value": s[a]}
            return self.node(pointer, node)
        if typ in {"integer", "number"}:
            checks = [{"kind": "int"}] if typ == "integer" else []
            for key, kind in [("minimum", "min"), ("maximum", "max")]:
                if key in s:
                    checks.append({"kind": kind, "value": s[key], "inclusive": True})
            return self.node(pointer, {"kind": "ZodNumber", "checks": checks, "coerce": False})
        if typ == "string":
            checks = [{"kind": kind, "value": s[key], "unit": "code_points"}
                      for key, kind in [("minLength", "min"), ("maxLength", "max")] if key in s]
            if "format" in s:
                fmt = {"uuid": "uuid", "date-time": "rfc3339", "duration": "duration", "date": "date"}.get(s["format"])
                if not fmt:
                    raise ValueError("Unsupported string format " + s["format"])
                checks.append({"kind": fmt})
            return self.node(pointer, {"kind": "ZodString", "checks": checks, "coerce": False})
        if typ in {"boolean", "null"}:
            return self.node(pointer, {"kind": "ZodBoolean" if typ == "boolean" else "ZodNull"})
        if typ is None and not (set(s) - ANNOTATIONS):
            return self.node(pointer, {"kind": "ZodUnknown"})
        raise ValueError(f"Unimplemented schema type {typ!r} at {pointer}")


def contract():
    pin, spec, ops = load()
    request, response = Compiler(spec), Compiler(spec, response=True)
    metadata = {}
    for method, path, op in ops:
        query = {p["name"]: p["schema"] for p in op.get("parameters", []) if p["in"] == "query"}
        required = [p["name"] for p in op.get("parameters", []) if p["in"] == "query" and p.get("required")]
        qsid = request.compile({"type": "object", "properties": query, "required": required,
                               "additionalProperties": False}, "#/operations/" + op["operationId"] + "/query")
        root = PREFIX + op["operationId"]
        request.roots[root + ":query"] = qsid
        kinds = {}
        for name, s in query.items():
            while "$ref" in s:
                s = spec["components"]["schemas"][s["$ref"].split("/")[-1]]
            branch = next((b for b in s.get("anyOf", []) if b.get("type") != "null"), s)
            kinds[name] = branch.get("type", "string")
        success = {k: v for k, v in op["responses"].items() if str(k).startswith("2")}
        if len(success) != 1:
            raise ValueError("Multiple success shapes need an explicit operation adapter")
        status, reply = next(iter(success.items()))
        schema = reply.get("content", {}).get("application/json", {}).get("schema")
        if schema is not None:
            pointer = "#/operations/" + op["operationId"] + "/response"
            request.roots[root + ":strict-response"] = request.compile(schema, pointer)
            response.roots[root + ":response"] = response.compile(schema, pointer)
        body = op.get("requestBody", {}).get("content", {}).get("application/json", {}).get("schema")
        if body is not None:
            request.roots[root + ":body"] = request.compile(body, "#/operations/" + op["operationId"] + "/body")
        metadata[op["operationId"]] = {"method": method, "path": path, "queryKinds": kinds,
            "querySchema": root + ":query", "requestSchema": root + ":body" if body else None,
            "responseSchema": root + ":response" if schema is not None else None, "successStatus": int(status)}
    nodes = request.nodes | response.nodes
    roots = {PREFIX + name: sid for name, sid in request.roots.items() if not name.startswith(PREFIX)}
    roots.update({PREFIX + name + ":compatible": sid for name, sid in response.roots.items() if not name.startswith(PREFIX)})
    roots.update({key: sid for c in (request, response) for key, sid in c.roots.items() if key.startswith(PREFIX)})
    return pin, spec, ops, request, {"nodes": nodes, "roots": roots}, metadata


def extend_graph(graph, compiled):
    result = copy.deepcopy(graph)
    if set(result["nodes"]) & set(compiled["nodes"]) or set(result["roots"]) & set(compiled["roots"]):
        raise ValueError("Vendor and frozen graph namespaces collide")
    result["nodes"].update(compiled["nodes"])
    result["roots"].update(compiled["roots"])
    return result


def model_outputs(generator_class, compiler):
    inventory = {"schemas": {"definitions": compiler.nodes, "roots": []}, "exports": [
        {"name": name, "kind": "schema", "schema": {"$ref": sid},
         "classification": {"namespace": "airs::red_team"}} for name, sid in compiler.roots.items()
        if not name.startswith(PREFIX)]}
    gen = generator_class(inventory)
    outputs = gen.generate("red_team")
    source = outputs[ROOT / "crates/scm-sdk/src/models/generated.rs"]
    source = source.replace(",JsonObject", "")
    source = source.replace("frozen schema", "vendor schema")
    source = source.replace("from tools/generate_models.py", "from the pinned vendor dataplane")
    source = source.replace("//! Generated native models. Regenerate with tools/generate_models.py.",
        "//! Generated from the pinned Red Team dataplane 0.93.0 MIT specification.\n//! Regenerate with tools/generate_models.py; see package NOTICE.")
    return source, gen


def augment_models(gen, compiler, vendor_gen):
    """Add typed fields to non-exhaustive models without changing frozen validation."""
    gen.nodes = copy.deepcopy(gen.nodes)
    gen.nodes.update(compiler.nodes)
    gen.external_types = {sid: "super::red_team_data_plane::" + name for sid, name in vendor_gen.names.items()}
    pairs, additions = set(), []

    def unwrap(sid):
        while gen.nodes[sid]["kind"] in {"ZodOptional", "ZodNullable", "ZodDefault"}:
            sid = gen.nodes[sid]["inner"]["$ref"]
        node = gen.nodes[sid]
        if node["kind"] == "ZodUnion":
            nonnull = [v["$ref"] for v in node["variants"] if gen.nodes[v["$ref"]]["kind"] != "ZodNull"]
            if len(nonnull) == 1:
                return unwrap(nonnull[0])
        return sid

    def pair(old, current):
        old, current = unwrap(old), unwrap(current)
        if (old, current) in pairs:
            return
        pairs.add((old, current))
        a, b = gen.nodes[old], gen.nodes[current]
        if a["kind"] == b["kind"] == "ZodObject":
            for name, field in b["properties"].items():
                if name in a["properties"]:
                    pair(a["properties"][name]["schema"]["$ref"], field["schema"]["$ref"])
                else:
                    child = field["schema"]["$ref"]
                    if gen.nodes[child]["kind"] != "ZodOptional":
                        optional = "sdp_model_" + sha((old + name).encode())[:24]
                        gen.nodes[optional] = {"kind": "ZodOptional", "inner": {"$ref": child}}
                        child = optional
                    a["properties"][name] = {"schema": {"$ref": child}}
                    additions.append({"legacySchema": old, "field": name, "vendorSchema": current})
        elif a["kind"] == b["kind"] == "ZodArray":
            pair(a["element"]["$ref"], b["element"]["$ref"])
        elif a["kind"] == b["kind"] == "ZodRecord":
            pair(a["key"]["$ref"], b["key"]["$ref"])
            pair(a["value"]["$ref"], b["value"]["$ref"])
        elif a["kind"] in {"ZodEnum", "ZodNativeEnum"} and b["kind"] in {"ZodEnum", "ZodNativeEnum"}:
            a["values"] = list(dict.fromkeys(a["values"] + b["values"]))
        elif a["kind"] in {"ZodUnion", "ZodDiscriminatedUnion"} and b["kind"] in {"ZodUnion", "ZodDiscriminatedUnion"}:
            for left in a["variants"]:
                old_node = gen.nodes[unwrap(left["$ref"])]
                candidates = [(ref["$ref"], gen.nodes[unwrap(ref["$ref"])]) for ref in b["variants"]]
                if old_node["kind"] == "ZodObject":
                    candidates = [(sid, n) for sid, n in candidates if n["kind"] == "ZodObject"]
                    if candidates:
                        best = max(candidates, key=lambda x: len(set(old_node["properties"]) & set(x[1]["properties"])))
                        pair(left["$ref"], best[0])

    aliases = {"AttackOutputSchema": "AttackOutputResponseSchema", "AttackMultiTurnOutputSchema": "AttackMultiTurnOutputResponseSchema",
               "AttackListItemSchema": "AttackListSchema", "CustomAttackOutputSchema": "CustomAttackOutputResponseSchema"}
    for name, sid in gen.roots.items():
        candidate = next((n for n in [aliases.get(name), name, name.removesuffix("Schema")] if n in compiler.roots), None)
        if candidate:
            pair(sid, compiler.roots[candidate])
    pair(gen.roots["RemediationDetailSchema"], compiler.roots["DynamicRemediationDetailSchema"])
    return additions


def bindings(ops):
    fixtures = json.loads((ROOT / "crates/scm-sdk/src/fixtures/red-team-contracts.json").read_text())["cases"]
    mapping = json.loads((ROOT / "contracts/native-operation-mappings/red-team.json").read_text())["operations"]
    native = {row["baselineId"]: row for row in mapping}
    old = {}
    for wrapper in fixtures:
        c = wrapper["case"]
        if c["call"]["plane"] == "redteam-data":
            call = c["call"]
            row = native[call["source"] + ":" + call["className"] + "." + call["member"]]
            old[c["method"] + " " + c["path"]] = (pathlib.Path(row["source"]).stem, call["className"], row["methods"][0])
    added = {method + " " + path for method, path, _ in ops} - set(old)
    if added != set(NEW) or {method + " " + path for method, path, _ in ops} != set(old) | set(NEW):
        raise ValueError("Added method/path identities differ from the reviewed eleven")
    return old | NEW


def required_changes(before, after, pointer):
    """Find newly required fields throughout shared component/inline schemas."""
    result = set()
    if isinstance(before, dict) and isinstance(after, dict):
        if isinstance(after.get("required"), list):
            earlier = before.get("required", [])
            if not isinstance(earlier, list):
                raise ValueError("Required schema fields must be an array")
            result.update((pointer, field) for field in set(after["required"]) - set(earlier))
        for key in before.keys() & after.keys():
            escaped = key.replace("~", "~0").replace("/", "~1")
            result.update(required_changes(before[key], after[key], pointer + "/" + escaped))
    elif isinstance(before, list) and isinstance(after, list):
        for index, (a, b) in enumerate(zip(before, after)):
            result.update(required_changes(a, b, pointer + "/" + str(index)))
    return result


def schema_change_classes(before, after):
    if before is None:
        return {"added_schema"}
    if after is None:
        return {"removed_schema"}
    classes = set()
    for key in before.keys() | after.keys():
        a, b = before.get(key), after.get(key)
        if a == b:
            continue
        if key == "properties":
            a, b = a or {}, b or {}
            classes.update("added_required_property" if name in after.get("required", []) else "added_optional_property" for name in b.keys() - a.keys())
            if a.keys() - b.keys():
                classes.add("removed_property")
            for name in a.keys() & b.keys():
                classes.update(schema_change_classes(a[name], b[name]))
        elif key == "enum":
            if set(b or []) - set(a or []): classes.add("enum_values_added")
            if set(a or []) - set(b or []): classes.add("enum_values_removed")
        elif key == "required":
            if set(b or []) - set(a or []): classes.add("required_field_added")
            if set(a or []) - set(b or []): classes.add("required_field_removed")
        elif key in {"anyOf", "oneOf"}: classes.add("union_changed")
        elif key in {"minimum", "maximum", "minLength", "maxLength", "minItems", "maxItems"}: classes.add("bound_changed")
        elif key in {"type", "$ref", "format", "items", "additionalProperties", "propertyNames", "const", "discriminator", "default"}:
            classes.add({"type":"type_changed", "$ref":"reference_changed", "format":"format_changed", "items":"array_items_changed", "additionalProperties":"unknown_fields_changed", "propertyNames":"record_keys_changed", "const":"literal_changed", "discriminator":"discriminator_changed", "default":"default_changed"}[key])
        else:
            raise ValueError("Unclassified schema change keyword: " + key)
    return classes


def changed_contracts(spec):
    # The canonical frozen vendor source is parsed by the pinned Node checker.
    previous = read_yaml(ROOT / "contracts/sources/pan-dev/openapi-specs/prisma-airs-redteam/data-plane/dp-openapi.yaml")
    changes, required = [], set()
    def structural(value):
        if isinstance(value, list):
            return [structural(x) for x in value]
        if isinstance(value, dict):
            return {k: structural(v) for k, v in value.items() if k not in ANNOTATIONS - {"default"}}
        return value
    for name, value in spec["components"]["schemas"].items():
        before = previous["components"]["schemas"].get(name)
        if before is not None:
            required.update(required_changes(before, value, "#/components/schemas/" + name))
        if structural(before) != structural(value):
            changes.append({"schema": name, "before": structural(before), "after": structural(value),
                            "changeClasses": sorted(schema_change_classes(structural(before), structural(value))),
                            "disposition": "current_vendor_validation_and_typed_models"})
    for name, before in previous["components"]["schemas"].items():
        if name not in spec["components"]["schemas"]:
            changes.append({"schema": name, "before": structural(before), "after": None,
                            "changeClasses": ["removed_schema"],
                            "disposition": "retain_legacy_native_type_and_use_current_vendor_replacement"})
    operation_changes = []
    for path, methods in spec["paths"].items():
        for method, op in methods.items():
            if method not in METHODS:
                continue
            before = previous["paths"].get(path, {}).get(method)
            if before is not None:
                required.update(required_changes(before, op, "#/paths/" + path.replace("/", "~1") + "/" + method))
            if structural(before) != structural(op):
                operation_classes = {"added_operation"} if before is None else {
                    {"parameters":"operation_parameters_changed", "responses":"operation_responses_changed", "requestBody":"operation_body_changed", "operationId":"operation_identity_changed"}.get(key, "unclassified_operation_" + key)
                    for key in structural(before).keys() | structural(op).keys()
                    if structural(before).get(key) != structural(op).get(key)}
                operation_changes.append({"method": method.upper(), "path": path,
                    "before": structural(before), "after": structural(op),
                    "changeClasses": sorted(operation_classes),
                    "disposition": "current_vendor_client_contract"})
    if required != WAIVER_POINTERS:
        raise ValueError("New required response fields differ from four reviewed waivers; amend plan and obtain fresh Claude review")
    allowed = {"added_schema", "removed_schema", "added_optional_property", "added_required_property", "enum_values_added", "enum_values_removed", "required_field_added", "reference_changed", "union_changed", "array_items_changed", "added_operation", "operation_parameters_changed", "operation_responses_changed"}
    actual = {cls for row in changes + operation_changes for cls in row["changeClasses"]}
    if actual - allowed:
        raise ValueError("Unreviewed vendor change classes require plan review: " + str(sorted(actual - allowed)))
    return {"schemaVersion": 1, "responsePresenceWaivers": [dict(schema=n, field=f) for n, f in sorted(WAIVERS)],
            "schemaChanges": changes, "operationChanges": operation_changes}


def client_impl(text, cls):
    """Rustfmt keeps the client impl's closing brace at column zero."""
    match = re.search(r"(?ms)^impl " + re.escape(cls) + r"\s*\{(.*?)^\}", text)
    return match.group(1) if match else ""


def option_fields(text, cls, member):
    match = re.search(r"(?ms)^    pub async fn " + re.escape(member) + r"\s*\((.*?)\)\s*->", client_impl(text, cls))
    if not match:
        return set()
    option = re.search(r"\bparams:\s*&([A-Za-z0-9_]+)", match.group(1))
    if not option:
        return set()
    definitions = []
    for source in (ROOT / "crates/scm-sdk/src/airs/red_team").glob("*.rs"):
        definitions.extend(re.findall(r"(?ms)^pub struct " + re.escape(option.group(1)) + r"\s*\{(.*?)^\}", source.read_text()))
    if len(definitions) != 1:
        raise ValueError("Public query option type must have one definition: " + option.group(1))
    return set(re.findall(r"(?m)^    pub (\w+):", definitions[0]))


def native_method(text, cls, member):
    match = re.search(r"(?ms)^    pub async fn " + re.escape(member) +
                      r"\b.*?(?=^    (?:pub (?:async )?fn |///|#\[)|\Z)", client_impl(text, cls))
    return match.group() if match else ""


def verify_schema_binding(text, cls, member, metadata, op):
    method = native_method(text, cls, member)
    expected = {value for key, value in metadata.items() if key in {"requestSchema", "responseSchema"} and value}
    # The two older JSON methods have intentionally unconstrained response schemas.
    unconstrained = op["responses"][str(metadata["successStatus"])].get("content", {}).get("application/json", {}).get("schema") == {}
    if unconstrained:
        expected.discard(metadata["responseSchema"])
        if ".send_json(" not in method:
            raise ValueError("Opaque JSON operation must use bounded JSON decoding")
    elif metadata["responseSchema"] is None and ".send_bytes(" not in method:
        raise ValueError("Schemaless CSV operation must use bounded bytes")
    actual = set(re.findall(r'"(vendor:red-team-dp:0\.93\.0:[^"\n]+)"', method))
    if actual != expected:
        raise ValueError("Native method schema roots differ from its exact operation: " + cls + "." + member)
    return {"requestSchema": metadata["requestSchema"], "responseSchema": metadata["responseSchema"],
            "successStatus": metadata["successStatus"], "responseHandling": "opaque_json" if unconstrained else "csv_bytes" if metadata["responseSchema"] is None else "typed_contract"}


WIRE_FIXTURES = "contracts/vendor/red-team-data-plane/0.93.0/wire-fixtures.json"
WIRE_TEST = "crates/scm-sdk/tests/red_team_contracts.rs"
WIRE_TEST_NAME = "all_47_vendor_operations_match_independent_wire_contracts_under_both_response_policies"


def wire_bindings(ops, mapping, corpus=None, test_source=None):
    path = ROOT / WIRE_FIXTURES
    if corpus is None:
        if not path.is_file():
            return {}
        corpus = json.loads(path.read_text())
    expected = {op["operationId"]: (method, route) for method, route, op in ops}
    rows = corpus["cases"]
    if {r["operationId"] for r in rows} != set(expected) or len({r["name"] for r in rows}) != len(rows):
        raise ValueError("Wire operation ID set differs from pinned OpenAPI or cases are duplicated")
    pin = json.loads((VENDOR / "source.json").read_text())
    if corpus["sourceSha256"] != pin["sha256"]:
        raise ValueError("Wire fixture source pin is stale")
    test_source = (ROOT / WIRE_TEST).read_text() if test_source is None else test_source
    test = re.search(r"(?s)async fn " + WIRE_TEST_NAME + r"\s*\(\s*\).*", test_source)
    helper = re.search(r"(?s)async fn call_vendor\b(.*?)#\[tokio::test\]", test_source)
    if not test or not helper or "call_vendor(" not in test.group() or "VENDOR_WIRE" not in test_source:
        raise ValueError("Vendor wire test and real dispatch helper are required")
    result = {}
    for row in rows:
        method, route = expected[row["operationId"]]
        _, cls, member = mapping[method + " " + route]
        concrete = route
        for key, value in row["pathParameters"].items():
            concrete = concrete.replace("{" + key + "}", value)
        if (row["method"], row["path"]) != (method, concrete) or row["call"] != {"className": cls, "member": member}:
            raise ValueError("Wire binding does not match its exact native operation")
        arm = re.search(r'\("' + re.escape(cls) + r'", "' + re.escape(member) + r'"\)\s*=>\s*(.*?)(?=\n\s*\("|\n\s*key =>)', helper.group(1), re.S)
        if not arm or not re.search(r"\." + re.escape(member) + r"\s*\(", arm.group(1)):
            raise ValueError("Wire dispatch arm does not call its recorded native method")
        result.setdefault(row["operationId"], []).append(row["name"])
    return result


def outputs():
    pin, spec, ops, request, graph, meta = contract()
    mapping = bindings(ops)
    wires = wire_bindings(ops, mapping)
    rejected = {}
    rows = []
    for method, path, op in ops:
        module, cls, member = mapping[method + " " + path]
        source = f"crates/scm-sdk/src/airs/red_team/{module}.rs"
        text = (ROOT / source).read_text()
        implemented = re.search(r"(?m)^    pub async fn " + re.escape(member) + r"\b", client_impl(text, cls)) is not None
        schemas = verify_schema_binding(text, cls, member, meta[op["operationId"]], op) if implemented else None
        declared = {p["name"] for p in op.get("parameters", []) if p["in"] == "query"}
        unsupported = option_fields(text, cls, member) - declared
        if unsupported:
            rejected[member] = unsupported
        rows.append({"operationId": op["operationId"], "method": method, "path": path, "plane": "red_team_data",
            "nativeClass": cls, "nativeMethod": member, "source": source,
            "sourceSha256": sha((ROOT / source).read_bytes()), "implemented": implemented,
            "parameters": op.get("parameters", []), "requestBody": op.get("requestBody"),
            "responses": op["responses"], "liveAcceptance": "unverified",
            "schemaBinding": schemas,
            "rejectedLegacyParameters": [{"name": name, "disposition": "validation_error_before_auth"} for name in sorted(unsupported)],
            "contractVerification": "executable_fixture" if op["operationId"] in wires and implemented else "planned",
            "fixtureBinding": {"path": WIRE_FIXTURES, "sha256": sha((ROOT / WIRE_FIXTURES).read_bytes()), "cases": wires[op["operationId"]],
                "test": WIRE_TEST + "::" + WIRE_TEST_NAME, "testSha256": sha((ROOT / WIRE_TEST).read_bytes())} if op["operationId"] in wires else None})
    if rejected != {name: set(fields) for name, fields in REJECTED.items()}:
        raise ValueError("Derived rejected legacy parameters differ from reviewed dispositions")
    ledger = {"schemaVersion": 1, "sourceSha256": pin["sha256"], "sourcePin": str((VENDOR / "source.json").relative_to(ROOT)),
              "generator": {"path": "tools/red_team_dataplane.py", "sha256": sha(pathlib.Path(__file__).read_bytes())},
              "operationCount": len(rows), "implementedCount": sum(row["implemented"] for row in rows), "operations": rows}
    return {ROOT / "contracts/vendor/red-team-data-plane/0.93.0/operations.json": ledger,
            ROOT / "contracts/vendor/red-team-data-plane/0.93.0/schema-diff.json": changed_contracts(spec),
            ROOT / "crates/scm-sdk/src/airs/red_team/contract.json": {
                name: {key: value for key, value in row.items() if key in {"queryKinds", "querySchema"}}
                for name, row in meta.items()}}


def model_probe():
    from generate_models import Generator
    _, _, _, compiler, _, _ = contract()
    _, gen = model_outputs(Generator, compiler)
    arms = []
    for name, sid in compiler.roots.items():
        if name.startswith(PREFIX):
            continue
        typed = gen.names[sid]
        arms.append(f'"{name}" => {{let model: vendor::{typed}=serde_json::from_value(value.clone()).expect("typed vendor decode"); assert_eq!(serde_json::to_value(&model).unwrap(),*value,"typed model preserves wire values"); assert!(!format!("{{model:?}}").contains("example"));}},')
    return '''// Generated vendor model probes; inputs are independently checked by AJV.
#![cfg(feature="red-team")]
use scm_sdk::{airs::red_team::models::data_plane as vendor, validation::{self,Mode}};
use serde_json::Value;
const CORPUS:&str=include_str!("../../../contracts/vendor/red-team-data-plane/0.93.0/model-fixtures.json");
#[test]
fn every_independent_vendor_model_fixture_matches_native_validation_and_typed_decode(){
let corpus:Value=serde_json::from_str(CORPUS).unwrap();
for row in corpus["cases"].as_array().unwrap(){
let value=&row["value"];let schema=row["schema"].as_str().unwrap();
let valid=row["valid"].as_bool().unwrap();
let result=validation::parse(&format!("vendor:red-team-dp:0.93.0:{schema}"),value.clone(),Mode::Request);
assert_eq!(result.is_ok(),valid,"fixture {}",row["name"]);
for mode in [Mode::Response,Mode::ResponseWithoutDefaults]{
let result=validation::parse(&format!("vendor:red-team-dp:0.93.0:{schema}"),value.clone(),mode);
assert_eq!(result.is_ok(),row["responseValid"].as_bool().unwrap(),"response fixture {} class {}",row["name"],row["mutationClass"]);
}
if !valid{continue;}
assert_eq!(result.unwrap(),*value,"vendor normalization preserves caller JSON");
for mode in [Mode::Request,Mode::Response,Mode::ResponseWithoutDefaults]{
assert_eq!(validation::parse(&format!("vendor:red-team-dp:0.93.0:{schema}"),value.clone(),mode).unwrap(),*value,"all response policies preserve valid vendor specimens");
match schema{''' + "\n".join(arms) + '''_=>panic!("unbound fixture schema")}
}}}
'''


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    parser.add_argument("--require-complete", action="store_true")
    args = parser.parse_args()
    result = outputs()
    probe = ROOT / "crates/scm-sdk/tests/red_team_vendor_models_generated.rs"
    content = model_probe()
    if args.check:
        # Rustfmt is the canonical formatter for generated probes too.
        formatted = subprocess.check_output([shutil.which("rustfmt") or str(pathlib.Path.home()/".cargo/bin/rustfmt"), "+1.99.0", "--edition", "2024"], input=content, text=True)
        if not probe.is_file() or probe.read_text() != formatted:
            raise ValueError("Generated vendor model probes differ")
    else:
        formatted = subprocess.check_output([shutil.which("rustfmt") or str(pathlib.Path.home()/".cargo/bin/rustfmt"), "+1.99.0", "--edition", "2024"], input=content, text=True)
        probe.write_text(formatted)
    for path, value in result.items():
        text = json.dumps(value, indent=2, ensure_ascii=False) + "\n"
        if args.check:
            if not path.is_file() or path.read_text() != text:
                raise ValueError("Generated vendor contract differs: " + str(path.relative_to(ROOT)))
        else:
            path.write_text(text)
    ledger = next(v for p, v in result.items() if p.name == "operations.json")
    if args.require_complete and (ledger["implementedCount"] != ledger["operationCount"] or
            any(row["contractVerification"] != "executable_fixture" for row in ledger["operations"])):
        raise ValueError("Vendor operations require implemented methods and executable wire fixtures")
    print(json.dumps({"vendorOperations": ledger["operationCount"], "implemented": ledger["implementedCount"], "checked": args.check}))


if __name__ == "__main__":
    main()
