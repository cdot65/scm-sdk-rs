#!/usr/bin/env python3
"""Verify packaged independent cases against the frozen fixture provenance ledger."""

import hashlib
import json
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
DIRECTORY = pathlib.Path("crates/scm-sdk/src/fixtures")
REQUIRED = {"red-team-contracts.json"}
MAX_BYTES = 64 * 1024 * 1024


def regular(root, relative):
    name = pathlib.PurePosixPath(relative)
    if name.is_absolute() or ".." in name.parts or str(name) != relative:
        raise ValueError("Fixture provenance must use a canonical repository path")
    path = root
    for part in name.parts:
        path = path / part
        if path.is_symlink():
            raise ValueError("Fixture provenance must not follow symlinks")
    if not path.is_file() or path.stat().st_size > MAX_BYTES:
        raise ValueError("Fixture input is absent or exceeds its bound")
    return path


def strict_json(path):
    def invalid(_):
        raise ValueError("Fixture input contains a nonfinite JSON constant")
    return json.loads(path.read_text(), parse_constant=invalid)


def canonical(value):
    # JSON identity distinguishes true from 1 and integers from decimal literals.
    return json.dumps(value, sort_keys=True, separators=(",", ":"), allow_nan=False)


def verify(root=ROOT):
    manifest = strict_json(regular(root, "contracts/manifest.json"))
    frozen = {record["path"]: record for record in manifest["fixtures"]}
    paths = sorted((root / DIRECTORY).glob("*-contracts.json"))
    if not REQUIRED <= {path.name for path in paths}:
        raise ValueError("Required independent native fixture bundle is absent")
    total_cases = 0
    for path in paths:
        bundle = strict_json(regular(root, path.relative_to(root).as_posix()))
        if bundle["schemaVersion"] != 1 or bundle["baselineCommit"] != manifest["baseline"]["commit"]:
            raise ValueError("Native fixture bundle has the wrong schema or baseline")
        if not bundle["license"].startswith("Apache-2.0"):
            raise ValueError("Native fixture attribution is absent")
        sources = {}
        for source in bundle["sources"]:
            name = source["path"]
            record = frozen.get(name)
            if name in sources or record is None or record["category"] != "independent-openapi-fixture":
                raise ValueError("Native fixture source lacks independent frozen provenance")
            source_path = regular(root, name)
            if (source["sha256"] != record["sha256"]
                    or hashlib.sha256(source_path.read_bytes()).hexdigest() != record["sha256"]
                    or source["license"] != "Apache-2.0"):
                raise ValueError("Native fixture source hash or attribution differs")
            sources[name] = strict_json(source_path)
        cases = bundle["cases"]
        if not 0 < len(cases) <= 2048:
            raise ValueError("Native fixture bundle has an invalid case count")
        seen = set()
        used = set()
        for entry in cases:
            source = entry["source"]
            name, pointer = source["path"], source["pointer"]
            match = re.fullmatch(r"/cases/(0|[1-9][0-9]*)", pointer)
            if (name not in sources or match is None or (name, pointer) in seen
                    or source["sha256"] != frozen[name]["sha256"]):
                raise ValueError("Native fixture case has invalid source coordinates")
            index = int(match[1])
            originals = sources[name]["cases"]
            if index >= len(originals) or canonical(entry["case"]) != canonical(originals[index]):
                raise ValueError("Native fixture case differs from its frozen source")
            seen.add((name, pointer))
            used.add(name)
        if used != set(sources):
            raise ValueError("Native fixture bundle declares an unused source")
        total_cases += len(cases)
    return {"bundles": len(paths), "exact_frozen_cases": total_cases}


if __name__ == "__main__":
    print(json.dumps(verify()))
