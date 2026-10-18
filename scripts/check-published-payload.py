#!/usr/bin/env python3
"""Qualify documentation/tooling changes only while published SDK inputs match."""
import argparse
import hashlib
import json
import pathlib
import re
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
NORMALIZED_METADATA = {"Cargo.toml", "Cargo.lock", ".cargo_vcs_info.json"}


def verify(root, receipt):
    root = root.resolve()
    if not (isinstance(receipt, dict) and type(receipt.get("schemaVersion")) is int
            and receipt["schemaVersion"] == 1 and receipt.get("registry") == "crates.io"
            and receipt.get("package") == "scm-sdk"
            and isinstance(receipt.get("ownerLogins"), list) and "cdot65" in receipt["ownerLogins"]
            and isinstance(receipt.get("version"), str)
            and re.fullmatch(r"\d+\.\d+\.\d+", receipt["version"])
            and isinstance(receipt.get("sourceCommit"), str)
            and re.fullmatch(r"[a-f0-9]{40}", receipt["sourceCommit"])
            and isinstance(receipt.get("archiveSha256"), str)
            and re.fullmatch(r"[a-f0-9]{64}", receipt["archiveSha256"])):
        raise ValueError("Expected a retained verified scm-sdk publication receipt")
    rows = receipt.get("files")
    if not isinstance(rows, list) or not rows:
        raise ValueError("Published file inventory is absent")
    seen, matched, sdk_paths = set(), [], set()
    for row in rows:
        if not (isinstance(row, dict) and isinstance(row.get("path"), str)
                and isinstance(row.get("sha256"), str) and re.fullmatch(r"[a-f0-9]{64}", row["sha256"])
                and type(row.get("bytes")) is int and row["bytes"] >= 0):
            raise ValueError("Malformed published file record")
        name = row["path"]
        path = pathlib.PurePosixPath(name)
        if (not name or path.is_absolute() or ".." in path.parts or "\\" in name
                or "\0" in name or name in seen or path.as_posix() != name):
            raise ValueError("Unsafe or duplicate published path")
        seen.add(name)
        if name in NORMALIZED_METADATA:
            continue
        if name.startswith(("src/", "examples/")):
            current = root / "crates/scm-sdk" / path
            sdk_paths.add(name)
        elif name == "Cargo.toml.orig":
            current = root / "crates/scm-sdk/Cargo.toml"
        elif name in {"README.md", "LICENSE", "NOTICE"}:
            current = root / path
        else:
            raise ValueError("Unclassified published file")
        if any(p.is_symlink() for p in (current, *current.parents)):
            raise ValueError("Published inputs must remain regular files")
        try:
            data = current.read_bytes()
        except OSError:
            raise ValueError("Published SDK input is missing or unreadable: " + name) from None
        if len(data) != row["bytes"] or hashlib.sha256(data).hexdigest() != row["sha256"]:
            raise ValueError("Published SDK input changed: " + name)
        matched.append(name)
    entries = [p for directory in ("src", "examples")
               for p in (root / "crates/scm-sdk" / directory).rglob("*")]
    if any(p.is_symlink() for p in entries):
        raise ValueError("SDK source trees must not contain symlinks")
    actual = {p.relative_to(root / "crates/scm-sdk").as_posix() for p in entries if p.is_file()}
    if actual != sdk_paths or not {"Cargo.toml.orig", "README.md", "LICENSE", "NOTICE"} <= seen:
        raise ValueError("Published SDK input inventory changed")
    return {"schemaVersion": 1, "passed": True, "version": receipt["version"],
            "publishedSourceCommit": receipt["sourceCommit"],
            "publishedArchiveSha256": receipt["archiveSha256"],
            "matchedSourceFiles": sorted(matched),
            "normalizedMetadataNotCompared": sorted(NORMALIZED_METADATA),
            "limitations": ["Git/package metadata may differ after documentation-only commits.",
                            "The separate current live-receipt check binds root Cargo inputs and runtime source."]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--receipt", type=pathlib.Path, required=True)
    parser.add_argument("--output", type=pathlib.Path, required=True)
    args = parser.parse_args()
    if args.receipt.is_symlink() or args.output.is_symlink():
        raise ValueError("Use regular receipt/output files")
    data = args.receipt.read_bytes()
    result = verify(ROOT, json.loads(data))
    result["receiptSha256"] = hashlib.sha256(data).hexdigest()
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({"publishedPayloadUnchanged": True, "matchedFiles": len(result["matchedSourceFiles"])}))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError, TypeError) as error:
        sys.exit(str(error))
