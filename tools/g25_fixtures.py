#!/usr/bin/env python3
"""Generate one-site mutations from every strict vendor response graph root."""

import argparse
import json
import pathlib
import re
import shutil
import subprocess
import generate_models as models
import red_team_dataplane as vendor

ROOT = pathlib.Path(__file__).resolve().parents[1]
DEST = ROOT / "contracts/compatibility/g25/cases.json"
FORMATS = {"uuid", "rfc3339", "datetime", "date", "duration", "email", "url", "regex"}


def generate():
    graph = json.loads(
        (ROOT / "crates/scm-sdk/src/validation/schema_graph.json").read_text()
    )
    nodes = graph["nodes"]
    roots = []
    cases = []
    skipped = []

    def sample(schema_id):
        schema_node = nodes[schema_id]
        kind = schema_node["kind"]
        if kind in ("ZodOptional", "ZodNullable", "ZodDefault"):
            return sample(schema_node["inner"]["$ref"])
        if kind == "ZodNull":
            return None
        if kind == "ZodBoolean":
            return False
        if kind == "ZodLiteral":
            return schema_node["value"]
        if kind in ("ZodEnum", "ZodNativeEnum"):
            return schema_node["values"][0]
        if kind == "ZodNumber":
            return max(
                [0]
                + [
                    entry["value"]
                    for entry in schema_node.get("checks", [])
                    if entry["kind"] == "min"
                ]
            )
        if kind == "ZodString":
            formats = {entry["kind"] for entry in schema_node["checks"]}
            unsupported = formats - {
                "uuid",
                "rfc3339",
                "duration",
                "date",
                "min",
                "max",
            }
            if unsupported:
                raise ValueError(
                    "No conforming string witness for checks: "
                    + ", ".join(sorted(unsupported))
                )
            for format_kind, format_value in [
                ("uuid", "550e8400-e29b-41d4-a716-446655440000"),
                ("rfc3339", "2026-10-18T00:00:00Z"),
                ("duration", "PT1S"),
                ("date", "2026-10-18"),
            ]:
                if format_kind in formats:
                    return format_value
            minimum = max(
                [1]
                + [
                    entry["value"]
                    for entry in schema_node["checks"]
                    if entry["kind"] == "min"
                ]
            )
            maximum = min(
                [1000]
                + [
                    entry["value"]
                    for entry in schema_node["checks"]
                    if entry["kind"] == "max"
                ]
            )
            return ("example" * ((minimum + 6) // 7))[:maximum]
        if kind == "ZodObject":
            return {
                key: sample(entry["schema"]["$ref"])
                for key, entry in schema_node["properties"].items()
            }
        if kind == "ZodArray":
            return [
                sample(schema_node["element"]["$ref"])
                for _ in range(
                    max(1, (schema_node.get("minLength") or {}).get("value", 0))
                )
            ]
        if kind == "ZodRecord":
            return {
                str(sample(schema_node["key"]["$ref"])): sample(
                    schema_node["value"]["$ref"]
                )
            }
        if kind in ("ZodUnion", "ZodDiscriminatedUnion"):
            return sample(
                next(
                    entry["$ref"]
                    for entry in schema_node["variants"]
                    if nodes[entry["$ref"]]["kind"] != "ZodNull"
                )
            )
        if kind in ("ZodUnknown", "ZodAny"):
            return {"future": True}
        raise ValueError("Uncovered graph specimen kind: " + kind)

    def walk(schema_id, path=(), choices=()):
        schema_node = nodes[schema_id]
        kind = schema_node["kind"]
        yield schema_id, list(path), list(choices)
        if kind in ("ZodOptional", "ZodNullable", "ZodDefault"):
            yield from walk(schema_node["inner"]["$ref"], path, choices)
        elif kind == "ZodObject":
            for key, entry in schema_node["properties"].items():
                yield from walk(entry["schema"]["$ref"], (*path, key), choices)
        elif kind == "ZodArray":
            yield from walk(schema_node["element"]["$ref"], (*path, 0), choices)
        elif kind == "ZodRecord":
            yield from walk(
                schema_node["value"]["$ref"],
                (*path, str(sample(schema_node["key"]["$ref"]))),
                choices,
            )
        elif kind in ("ZodUnion", "ZodDiscriminatedUnion"):
            for branch in schema_node["variants"]:
                if nodes[branch["$ref"]]["kind"] == "ZodNull":
                    skipped.append(
                        {
                            "root": root,
                            "path": list(path),
                            "node": branch["$ref"],
                            "choices": list(choices),
                            "reason": "null-branch-covered-by-type:null",
                        }
                    )
                    continue
                yield from walk(
                    branch["$ref"],
                    path,
                    (*choices, [list(path), sample(branch["$ref"])]),
                )

    for name, schema_id in sorted(graph["roots"].items()):
        if not (
            name.startswith("vendor:red-team-dp:0.93.0:")
            and name.endswith(":strict-response")
        ):
            continue
        root = name.removesuffix(":strict-response") + ":response"
        index = len(roots)
        roots.append({"root": root, "strictRoot": name, "base": sample(schema_id)})
        cases.append(
            {
                "root": index,
                "node": schema_id,
                "path": [],
                "choices": [],
                "kind": "base",
                "value": roots[-1]["base"],
                "omit": False,
            }
        )
        for kind, value in [
            ("null", None),
            ("string", "root-value"),
            ("number", 1),
            ("boolean", True),
            ("object", {}),
            ("array", []),
        ]:
            cases.append(
                {
                    "root": index,
                    "node": schema_id,
                    "path": [],
                    "choices": [],
                    "kind": "root:" + kind,
                    "value": value,
                    "omit": False,
                }
            )
        seen = set()
        for node, path, choices in walk(schema_id):
            schema_node = nodes[node]
            kind = schema_node["kind"]

            def add(kind, value=None, omit=False):
                case = {
                    "root": index,
                    "node": node,
                    "path": path,
                    "choices": choices,
                    "kind": kind,
                    "value": value,
                    "omit": omit,
                }
                key = json.dumps(case, sort_keys=True)
                if key not in seen:
                    cases.append(case)
                    seen.add(key)

            if kind == "ZodString":
                for check in schema_node["checks"]:
                    if check["kind"] in FORMATS:
                        add("format:" + check["kind"], "service-opaque-value")
                    elif check["kind"] == "min" and check["value"] > 0:
                        add("bound:min", "x" * (check["value"] - 1))
                    elif check["kind"] == "max":
                        add("bound:max", "x" * (check["value"] + 1))
            if kind in ("ZodEnum", "ZodNativeEnum") and all(
                isinstance(entry, str) for entry in schema_node["values"]
            ):
                add("enum", "FUTURE_SERVICE_VALUE")
            if (
                path
                and isinstance(path[-1], int)
                and kind in ("ZodUnion", "ZodDiscriminatedUnion")
                and any(
                    nodes[entry["$ref"]]["kind"] == "ZodNull"
                    for entry in schema_node["variants"]
                )
            ):
                add("type:null", None)
            # One set per property/branch, including optionality and all JSON types.
            if (
                path
                and isinstance(path[-1], str)
                and kind not in ("ZodOptional", "ZodNullable", "ZodDefault")
            ):
                for mutation_kind, mutation_value in [
                    ("null", None),
                    ("string", "service-value"),
                    ("number", 1),
                    ("boolean", True),
                    ("object", {}),
                    ("array", []),
                ]:
                    add("type:" + mutation_kind, mutation_value)
                add("presence", omit=True)
    return {
        "schemaVersion": 1,
        "roots": roots,
        "cases": cases,
        "skippedBranches": skipped,
        "summary": {
            "roots": len(roots),
            "mutations": len(cases),
            "formats": sum(entry["kind"].startswith("format:") for entry in cases),
            "enums": sum(entry["kind"] == "enum" for entry in cases),
        },
    }


def projections():
    inventory = json.loads(models.SOURCE.read_text())
    frozen = models.compact_graph(inventory)["nodes"]
    generator = models.Generator(inventory)
    _, _, _, compiler, _, _ = vendor.contract()
    _, vendor_graph = vendor.model_outputs(models.Generator, compiler)
    additions = vendor.augment_models(generator, compiler, vendor_graph)
    bindings = json.loads(
        (ROOT / "contracts/compatibility/g25/projection-bindings.json").read_text()
    )
    if bindings.get("schemaVersion") != 1 or additions != bindings.get("additions"):
        raise ValueError("Reviewed legacy/vendor projection bindings changed")
    generator.generate("red_team")

    def optional(schema_id, graph=frozen):
        schema_node = graph[schema_id]
        if schema_node["kind"] in ("ZodOptional", "ZodDefault", "ZodAny", "ZodUnknown"):
            return True
        if schema_node["kind"] in (
            "ZodNullable",
            "ZodEffects",
            "ZodBranded",
            "ZodReadonly",
        ):
            return optional(schema_node["inner"]["$ref"], graph)
        return False

    def sample(schema_id, graph=frozen):
        schema_node = graph[schema_id]
        kind = schema_node["kind"]
        if kind in (
            "ZodOptional",
            "ZodNullable",
            "ZodDefault",
            "ZodEffects",
            "ZodBranded",
            "ZodReadonly",
            "ZodCatch",
        ):
            return sample(schema_node["inner"]["$ref"], graph)
        if kind == "ZodLazy":
            return sample(schema_node["target"]["$ref"], graph)
        if kind == "ZodPipeline":
            return sample(schema_node["output"]["$ref"], graph)
        if kind == "ZodObject":
            return {
                key: sample(entry["schema"]["$ref"], graph)
                for key, entry in schema_node["properties"].items()
                if not optional(entry["schema"]["$ref"], graph)
            }
        if kind == "ZodArray":
            return [
                sample(schema_node["element"]["$ref"], graph)
                for _ in range(
                    max(1, (schema_node.get("minLength") or {}).get("value", 0))
                )
            ]
        if kind == "ZodRecord":
            return {}
        if kind == "ZodString":
            return "example"
        if kind == "ZodNumber":
            return max(
                [0]
                + [
                    entry["value"]
                    for entry in schema_node.get("checks", [])
                    if entry["kind"] == "min"
                ]
            )
        if kind == "ZodBoolean":
            return False
        if kind == "ZodLiteral":
            return schema_node["value"]
        if kind == "ZodNull":
            return None
        if kind in ("ZodAny", "ZodUnknown"):
            return {"future": True}
        if kind in ("ZodEnum", "ZodNativeEnum"):
            return schema_node["values"][0]
        if kind in ("ZodUnion", "ZodDiscriminatedUnion"):
            return sample(
                next(
                    entry["$ref"]
                    for entry in schema_node["variants"]
                    if graph[entry["$ref"]]["kind"] != "ZodNull"
                ),
                graph,
            )
        if kind == "ZodIntersection":
            return {
                **sample(schema_node["left"]["$ref"], graph),
                **sample(schema_node["right"]["$ref"], graph),
            }
        raise ValueError("Unclassified projection witness " + kind)

    def reachable(starts):
        seen = set()
        pending = list(starts)
        while pending:
            schema_id = pending.pop()
            if schema_id in seen:
                continue
            seen.add(schema_id)

            def refs(value):
                if isinstance(value, dict):
                    if "$ref" in value:
                        pending.append(value["$ref"])
                    else:
                        for child in value.values():
                            refs(child)
                elif isinstance(value, list):
                    for child in value:
                        refs(child)

            refs(frozen[schema_id])
        return seen

    named = {r["name"]: r["schema"]["$ref"] for r in inventory["schemas"]["roots"]}
    request_names = set()
    for operation in json.loads((ROOT / "contracts/operations.json").read_text())[
        "operations"
    ]:
        if operation["family"] != "red-team":
            continue
        schema = (operation.get("directRequest") or {}).get("requestSchema")
        if schema:
            for name in schema["names"]:
                if name not in named:
                    raise ValueError("Unclassified request schema " + name)
                request_names.add(name)
    request = reachable([named[name] for name in sorted(request_names)])
    response = reachable(
        [
            b["root"]
            for b in vendor.response_baselines(vendor.load()[2]).values()
            if b["root"]
        ]
    )
    rows = []
    for addition in additions:
        schema_id = addition["legacySchema"]
        field = addition["field"]
        child = generator.deref(
            generator.nodes[schema_id]["properties"][field]["schema"]
        )
        rows.append(
            {
                "schema": schema_id,
                "field": field,
                "rustType": generator.names[schema_id],
                "base": sample(schema_id),
                "conforming": sample(child, generator.nodes),
                "nullable": generator.presence(child)[1],
                "usage": (
                    (
                        "request-and-response"
                        if schema_id in response
                        else "request-only"
                    )
                    if schema_id in request
                    else (
                        "response-only"
                        if schema_id in response
                        else "standalone-schema"
                    )
                ),
            }
        )
    return rows


def rust_bindings(projection_rows):
    path = ROOT / "crates/scm-sdk/tests/g25_response_compatibility.rs"
    rust_source = path.read_text()
    arms = []
    model_schema_exemptions = []
    for operation in json.loads(
        (
            ROOT / "contracts/vendor/red-team-data-plane/0.93.0/operations.json"
        ).read_text()
    )["operations"]:
        root = operation["schemaBinding"]["responseSchema"]
        if not root:
            continue
        body = vendor.native_method(
            (ROOT / operation["source"]).read_text(),
            operation["nativeClass"],
            operation["nativeMethod"],
        )
        rust_type = (
            re.search(r"->\s*Result<([\s\S]+)>", body.split("{", 1)[0]).group(1).strip()
        )
        if (
            operation["operationId"]
            == "set_runtime_profile_v1_scan__job_id__runtime_profile_put"
        ):
            model_schema_exemptions.append(operation["operationId"])
            # The new operation returns the legacy model without imposing its
            # frozen schema. Public model parse would incorrectly validate twice.
            decoder = (
                "{ /* Decoder-exempt: operation vendor contract differs from JobResponse::SCHEMA; "
                "raw serde only checks the legacy output shape here. */ "
                "serde_json::to_value(serde_json::from_value::<JobResponse>(value).unwrap()).unwrap() }"
            )
        elif rust_type in ("JsonObject", "JsonValue"):
            # These operation results have no model decoder or public parse API.
            # Their object/JSON contract is validated before this identity arm.
            decoder = "{ /* Decoder-exempt: validated unconstrained JSON has no model parse API. */ value }"
        elif rust_type.startswith("Vec<") and rust_type.endswith(">"):
            element_type = rust_type[4:-1]
            decoder = (
                "Value::Array(value.as_array().unwrap().iter().cloned().map(|item|"
                "serde_json::to_value("
                + element_type
                + "::parse(item,mode).unwrap()).unwrap()"
                ").collect())"
            )
        else:
            decoder = (
                "serde_json::to_value("
                + rust_type
                + "::parse(value,mode).unwrap()).unwrap()"
            )
        arms.append(json.dumps(root) + "=>" + decoder + ",")
    if model_schema_exemptions != [
        "set_runtime_profile_v1_scan__job_id__runtime_profile_put"
    ]:
        raise ValueError("Operation decoder exemption inventory changed")
    start = rust_source.index("// BEGIN GENERATED OPERATION TYPES")
    end = rust_source.index("// END GENERATED OPERATION TYPES", start)
    rust_source = (
        rust_source[:start]
        + "// BEGIN GENERATED OPERATION TYPES\nfn roundtrip(schema:&str,value:Value,mode:Mode)->Value{match schema{\n"
        + "\n".join(arms)
        + "\n"
        + rust_source[end:]
    )
    start = rust_source.index("// BEGIN GENERATED PROJECTION TYPES")
    end = rust_source.index("// END GENERATED PROJECTION TYPES", start)
    arms = [
        json.dumps(name)
        + "=>serde_json::to_value("
        + name
        + "::parse(value,mode).unwrap()).unwrap(),"
        for name in sorted({r["rustType"] for r in projection_rows})
    ]
    rust_source = (
        rust_source[:start]
        + "// BEGIN GENERATED PROJECTION TYPES\nfn projection_roundtrip(name:&str,value:Value,mode:Mode)->Value{match name{\n"
        + "\n".join(arms)
        + "\n"
        + rust_source[end:]
    )
    # Assert actual projection state as well as JSON identity for every added field.
    start = rust_source.index("// BEGIN GENERATED PROJECTION STATES")
    end = rust_source.index("// END GENERATED PROJECTION STATES", start)
    arms = []
    for row in projection_rows:
        name, field = row["rustType"], row["field"]
        member = models.field_name(field)
        enum = "Patch" if row["nullable"] else "Optional"
        states = f'scm_sdk::data::{enum}::Unset=>"unset",scm_sdk::data::{enum}::Value(_)=>"value",'
        if row["nullable"]:
            states += 'scm_sdk::data::Patch::Null=>"null",'
        arms.append(
            "("
            + json.dumps(name)
            + ","
            + json.dumps(field)
            + ")=>{let model:"
            + name
            + "="
            + name
            + "::parse(value,mode).unwrap();let state=match model."
            + member
            + "{"
            + states
            + "};(state,model.extensions.get("
            + json.dumps(field)
            + ").is_some())},"
        )
    rust_source = (
        rust_source[:start]
        + "// BEGIN GENERATED PROJECTION STATES\nfn projection_state(name:&str,field:&str,value:Value,mode:Mode)->(&'static str,bool){match(name,field){\n"
        + "\n".join(arms)
        + "\n"
        + rust_source[end:]
    )
    rustfmt = shutil.which("rustfmt") or str(pathlib.Path.home() / ".cargo/bin/rustfmt")
    return path, subprocess.check_output(
        [rustfmt, "+1.99.0", "--edition", "2024"], input=rust_source, text=True
    )


def main():
    parser = argparse.ArgumentParser()
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    result = generate()
    projection_rows = projections()
    outputs = {
        DEST: json.dumps(result, separators=(",", ":")) + "\n",
        ROOT
        / "contracts/compatibility/g25/projections.json": json.dumps(
            projection_rows, indent=2
        )
        + "\n",
    }
    path, rust = rust_bindings(projection_rows)
    outputs[path] = rust
    for path, text in outputs.items():
        if args.check:
            if path.read_text() != text:
                raise SystemExit("G25 generated fixture/binding is stale: " + str(path))
        else:
            path.write_text(text)
    print(
        json.dumps({**result["summary"], "typedProjectionFields": len(projection_rows)})
    )


if __name__ == "__main__":
    main()
