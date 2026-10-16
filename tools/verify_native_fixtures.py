#!/usr/bin/env python3
"""Verify packaged independent cases against the frozen fixture provenance ledger."""

import argparse
import hashlib
import json
import os
import pathlib
import re

ROOT = pathlib.Path(__file__).resolve().parents[1]
DIRECTORY = pathlib.Path("crates/scm-sdk/src/fixtures")
REQUIRED = {"red-team-contracts.json", "gateway-contracts.json"}
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


def rust_tokens(text):
    """Read literal macro arguments without counting comments or quoted code."""
    raw_pattern = re.compile(r'(?:br|r)(#{0,255})"')
    literal_pattern = re.compile(r'b?"(?:\\.|[^"\\])*"', re.DOTALL)
    char_pattern = re.compile(r"b?'(?:\\(?:u\{[0-9a-fA-F_]+\}|x[0-9a-fA-F]{2}|.)|[^'\\\n])'")
    identifier_pattern = re.compile(r"[a-zA-Z_][a-zA-Z_0-9]*")
    index = 0
    while index < len(text):
        if text.startswith("//", index):
            end = text.find("\n", index)
            index = len(text) if end < 0 else end + 1
            continue
        if text.startswith("/*", index):
            index += 2
            depth = 1
            while depth and index < len(text):
                if text.startswith("/*", index):
                    depth += 1
                    index += 2
                elif text.startswith("*/", index):
                    depth -= 1
                    index += 2
                else:
                    index += 1
            if depth:
                raise ValueError("Unterminated Rust test comment")
            continue
        raw = raw_pattern.match(text, index)
        if raw:
            start = raw.end()
            end_marker = '"' + raw[1]
            end = text.find(end_marker, start)
            if end < 0:
                raise ValueError("Unterminated Rust test literal")
            yield ("literal", text[start:end])
            index = end + len(end_marker)
            continue
        literal = literal_pattern.match(text, index)
        if literal:
            value = literal[0]
            # Fixture paths deliberately use unescaped, ordinary ASCII literals.
            yield ("literal", value[value.index('"') + 1:-1])
            index = literal.end()
            continue
        char = char_pattern.match(text, index)
        if char:
            index = char.end()
            continue
        identifier = identifier_pattern.match(text, index)
        if identifier:
            yield ("identifier", identifier[0])
            index = identifier.end()
        else:
            if not text[index].isspace():
                yield ("punctuation", text[index])
            index += 1


def verify_test_bindings(root, paths):
    expected = {path.relative_to(root).as_posix() for path in paths}
    bound = set()
    for path in sorted((root / "crates/scm-sdk/tests").rglob("*.rs")):
        source = regular(root, path.relative_to(root).as_posix()).read_text()
        tokens = list(rust_tokens(source))
        for index in range(len(tokens) - 4):
            if (tokens[index:index + 3] != [
                    ("identifier", "include_str"), ("punctuation", "!"), ("punctuation", "(")]
                    or tokens[index + 3][0] != "literal"
                    or tokens[index + 4] != ("punctuation", ")")):
                continue
            name = tokens[index + 3][1]
            if not name.endswith("-contracts.json") or "/fixtures/" not in name:
                continue
            destination = path.parent / name
            # Normalize lexical parent components before regular() checks symlinks.
            relative = pathlib.Path(os.path.normpath(destination)).relative_to(root).as_posix()
            if name != os.path.relpath(root / relative, path.parent):
                raise ValueError("Rust fixture binding must use a canonical relative path")
            regular(root, relative)
            if relative not in expected:
                raise ValueError("Rust test references an unknown native fixture bundle")
            bound.add(relative)
    if bound != expected:
        raise ValueError("Independent native fixture bundle has no literal Rust test binding")
    return len(bound)


def verify(root=ROOT, require_test_bindings=False):
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
    result = {"bundles": len(paths), "exact_frozen_cases": total_cases}
    if require_test_bindings:
        result["test_bound_bundles"] = verify_test_bindings(root, paths)
    return result


if __name__ == "__main__":
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--require-test-bindings", action="store_true")
    args = parser.parse_args()
    print(json.dumps(verify(require_test_bindings=args.require_test_bindings)))
