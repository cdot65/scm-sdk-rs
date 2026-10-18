#!/usr/bin/env python3
"""Inventory frozen schema defaults and the additive Rust response policy."""
import argparse
import hashlib
import json
import pathlib
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
PIN = "4a80dbdb3dc14365e1c25d838ac0bd139485d48c"
OUTPUT = "contracts/response-defaults.json"
DOC = "docs/RESPONSE_DEFAULTS.md"
DASHBOARD_TYPES = {
    "appQuerySchema": "DashboardAppQuery",
    "timeRangeSchema": "DashboardTimeRangeQuery",
    "sessionsOverviewQuerySchema": "DashboardSessionsOverviewQuery",
    "sessionQuerySchema": "DashboardSessionQuery",
    "sessionTransactionQuerySchema": "DashboardSessionTransactionQuery",
    "sessionWindowSchema": None,
}


def sha(data):
    return hashlib.sha256(data).hexdigest()


def build(root=ROOT):
    inputs = [
        "contracts/typescript.json", "contracts/native-models.json",
        "crates/scm-sdk/src/validation/schema_graph.json",
        "crates/scm-sdk/src/validation/semantics.json",
    ]
    source, models, graph, semantics = [json.loads((root / p).read_text()) for p in inputs]
    if source["baseline"]["commit"] != PIN or models["baselineCommit"] != PIN:
        raise ValueError("Default inventory requires the frozen TypeScript baseline")
    bindings = []
    for sid, semantic in sorted(semantics.items()):
        if semantic["binding"] != "default":
            continue
        if graph["nodes"][sid]["kind"] != "ZodDefault":
            raise ValueError("Default binding no longer names a default node")
        original = source["schemas"]["definitions"][sid]["semanticSource"]["source"]
        uses = []
        for parent_id, parent in graph["nodes"].items():
            for field, prop in parent.get("properties", {}).items():
                if prop.get("schema", {}).get("$ref") != sid:
                    continue
                for name, ref in sorted(graph["roots"].items()):
                    if ref != parent_id or "#" not in name:
                        continue
                    filename, schema = name.split("#", 1)
                    rust = sorted({
                        "scm_sdk::" + row["namespace"] + "::models::" + row["rustType"]
                        for row in models["schemas"] if row["schema"] == parent_id
                    })
                    if filename == "src/management/dashboard.ts":
                        typename = DASHBOARD_TYPES[schema]
                        rust = ["scm_sdk::airs::management::" + typename] if typename else []
                    uses.append({"schema": schema, "field": field, "rustTypes": rust,
                                 "sourceFile": filename})
        filename = original["file"]
        if "red-team" in filename:
            product = "Red Team"
        elif filename == "src/management/dashboard.ts":
            product = "Management dashboard"
        elif filename == "src/models/model-security.ts":
            product = "Model Security"
        elif filename.startswith("src/models/mgmt-"):
            product = "Management"
        else:
            raise ValueError("Unreviewed default ownership: " + filename)
        if not uses:
            raise ValueError("Default binding has no inventoried property")
        bindings.append({
            "binding": sid, "product": product, "outsideRedTeam": product != "Red Team",
            "default": semantic["default"], "uses": uses,
            "sourceFile": filename, "sourceLine": original["line"],
        })
    if len(bindings) != 23:
        raise ValueError("The frozen default denominator changed")
    return {
        "schemaVersion": 1, "introducedIn": "0.1.1", "baselineCommit": PIN,
        "generatorSha256": sha(pathlib.Path(__file__).read_bytes()),
        "sources": {p: sha((root / p).read_bytes()) for p in inputs},
        "rustOnlyAdditions": [
            "scm_sdk::RequestOptions::without_response_defaults",
            "scm_sdk::RequestOptions::response_defaults",
            "scm_sdk::validation::Mode::ResponseWithoutDefaults",
        ],
        "counts": {"defaultBindings": 23,
                   "redTeamBindings": sum(not b["outsideRedTeam"] for b in bindings),
                   "outsideRedTeamBindings": sum(b["outsideRedTeam"] for b in bindings)},
        "bindings": bindings,
    }


def render(inventory):
    tick = chr(96)
    rows = [
        "# Response defaults", "",
        "The 0.1.1 response policy is opt-in per operation. The default remains",
        "TypeScript-compatible insertion of schema defaults. With",
        tick + "RequestOptions::without_response_defaults()" + tick + ", validation keeps the",
        "same acceptance and resource accounting while omitted fields stay absent.",
        "Explicitly supplied values and valid nulls survive typed serialization.", "",
        "Request validation still inserts defaults. Request/query schema defaults",
        "listed below are unaffected when constructing outgoing requests. These are",
        "23 frozen schema bindings, not every configuration default in the SDK.",
        "Shared schema bindings are listed once with every owning model/schema.",
        "Fields use JSON names; Rust dashboard fields use snake_case.", "",
        "There are 8 Red Team bindings and 15 outside Red Team. No binding originates",
        "in a DLP schema. Reused model contracts keep the same policy in any product.", "",
        "Generated by tools/response_defaults.py from the frozen graph and semantics.",
        "The machine-readable inventory also records the three additive Rust APIs.",
        "See [validation](VALIDATION.md#responses-without-inserted-defaults).", "",
        "| Binding | Product | Model / shared schema | JSON field | Default |",
        "| --- | --- | --- | --- | --- |",
    ]
    for binding in inventory["bindings"]:
        names = sorted({
            name.rsplit("::", 1)[-1]
            for use in binding["uses"]
            for name in (use["rustTypes"] or [use["schema"]])
        })
        fields = sorted({use["field"] for use in binding["uses"]})
        code = lambda s: tick + s + tick
        product = binding["product"] + (" (outside Red Team)" if binding["outsideRedTeam"] else "")
        rows.append("| " + " | ".join([
            code(binding["binding"]), product, ", ".join(map(code, names)),
            ", ".join(map(code, fields)), code(json.dumps(binding["default"])),
        ]) + " |")
    return "\n".join(rows) + "\n"


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    inventory = build()
    outputs = {OUTPUT: json.dumps(inventory, indent=2) + "\n", DOC: render(inventory)}
    for name, content in outputs.items():
        path = ROOT / name
        if args.check:
            if not path.is_file() or path.read_text() != content:
                raise ValueError("Response default inventory is stale: " + name)
        else:
            path.write_text(content)
    print(json.dumps(inventory["counts"]))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError) as error:
        sys.exit(str(error))
