#!/usr/bin/env python3
"""Verify the frozen, credential-free contract corpus without network or dependencies.

The default check proves completeness against the baseline Git tree and checks all
redistributed bytes, fixture specification pins, correction IDs, and notices.
Use --baseline PATH to additionally check an extracted TypeScript src tree, or
--fetch-excluded to audit the external, unlicensed official Gateway pin. Neither
option modifies source repositories or downloads distributable artifacts.
"""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import shutil
import subprocess
import sys
import tempfile
from datetime import datetime, timedelta
from pathlib import Path
from urllib.request import Request, urlopen

ROOT = Path(__file__).resolve().parents[1]
PIN = "4a80dbdb3dc14365e1c25d838ac0bd139485d48c"
VERSION = "0.34.0"
# Explicit ownership: historical vendor trees retain their dedicated verifiers.
VENDOR_ROOTS = ("contracts/vendor/red-team-management/0.86.0",)
EXTRACTION_INPUTS = {"package.json", "tsconfig.json", "scripts/openapi/inventory.ts"}
FIXTURE_CATEGORIES = {
    "independent-openapi-fixture", "fixture-test-reference",
    "public-response-capture", "sanitized-observed-response",
    "sdk-authored-regression-reference",
}


def require(condition: bool, message: str) -> None:
    if not condition:
        raise ValueError(message)


def git_hash(kind: str, value: bytes) -> str:
    return hashlib.sha1(
        kind.encode() + b" " + str(len(value)).encode() + b"\0" + value
    ).hexdigest()


def canonical(value: object) -> bytes:
    return (json.dumps(value, indent=2, ensure_ascii=False) + "\n").encode()


def safe_path(value: str) -> Path:
    path = ROOT / value
    require(not Path(value).is_absolute(), f"Absolute bundled path: {value}")
    require(".." not in Path(value).parts, f"Parent traversal in bundled path: {value}")
    require(path.is_relative_to(ROOT), f"Path escaped repository: {value}")
    for parent in (path, *path.parents):
        if parent == ROOT:
            break
        require(not parent.is_symlink(), f"Symlink in bundled path: {value}")
    return path


def verify_bytes(record: dict, data: bytes, label: str) -> None:
    require(len(data) == record["bytes"], f"Size differs: {label}")
    require(hashlib.sha256(data).hexdigest() == record["sha256"], f"SHA-256 differs: {label}")
    if "gitBlobSha1" in record:
        require(git_hash("blob", data) == record["gitBlobSha1"], f"Git blob differs: {label}")


def verify_file(record: dict) -> bytes:
    path = safe_path(record["path"])
    require(path.is_file(), f"Missing bundled file: {record['path']}")
    data = path.read_bytes()
    verify_bytes(record, data, record["path"])
    return data


def verify_extracted_inputs(records: list[dict], baseline: Path) -> None:
    for record in records:
        relative = Path(record["upstreamPath"])
        require(not relative.is_absolute() and ".." not in relative.parts,
                "Invalid extracted input path")
        path = baseline / relative
        require(path.is_file(), f"Missing extracted baseline input: {relative}")
        for parent in (path, *path.parents):
            if parent == baseline:
                break
            require(not parent.is_symlink(), f"Symlink in extracted baseline input: {relative}")
        verify_bytes(record, path.read_bytes(), relative.as_posix())


def fixture_category(upstream_path: str) -> str:
    if upstream_path.startswith("test/openapi/"):
        return "independent-openapi-fixture" if upstream_path.endswith(".json") else "fixture-test-reference"
    if "/fixtures/" in upstream_path:
        return "public-response-capture" if upstream_path.endswith("model-pricing-live.json") else "sanitized-observed-response"
    return "sdk-authored-regression-reference"


def git_tree_hash(files: list[dict]) -> str:
    """Reconstruct Git's root tree from the full ls-tree ledger."""
    root: dict = {}
    for item in files:
        parts = item["path"].split("/")
        require(all(part and part not in (".", "..") for part in parts), "Invalid upstream path")
        node = root
        for part in parts[:-1]:
            node = node.setdefault(part, {})
            require(isinstance(node, dict), "Upstream path collides with a file")
        require(parts[-1] not in node, f"Duplicate upstream tree path: {item['path']}")
        require(item["mode"] in ("100644", "100755", "120000"), "Unsupported tree mode")
        require(bool(re.fullmatch(r"[0-9a-f]{40}", item["gitBlobSha1"])), "Invalid Git blob hash")
        node[parts[-1]] = (item["mode"], item["gitBlobSha1"])

    def tree(node: dict) -> str:
        entries = []
        for name, value in node.items():
            if isinstance(value, dict):
                mode, digest, key = "40000", tree(value), name.encode() + b"/"
            else:
                mode, digest = value
                key = name.encode()
            raw = mode.encode() + b" " + name.encode() + b"\0" + bytes.fromhex(digest)
            entries.append((key, raw))
        return git_hash("tree", b"".join(raw for _, raw in sorted(entries)))

    return tree(root)


def self_test() -> None:
    """Exercise rejection paths against disposable copies of the corpus."""
    cases = ("fixture-change", "fixture-removal", "unmanifested-source", "license-removal",
             "fixture-inventory-omission", "baseline-source-omission", "tree-ledger-omission",
             "excluded-source-generation", "export-source-change", "fixture-category-relabel",
             "fixture-category-unknown", "specification-pin-removal", "observed-specification-pin",
             "extraction-input-omission", "extraction-input-origin-change", "license-evidence-change",
             "unowned-vendor-file", "missing-vendor-file", "modified-vendor-file",
             "vendor-license-commit", "duplicate-source-id", "vendor-artifact-origin",
             "vendor-artifact-omission", "vendor-artifact-escape", "vendor-symlink",
             "unowned-vendor-version")
    with tempfile.TemporaryDirectory(prefix="scm-contract-verifier-") as temporary:
        root = Path(temporary)
        shutil.copytree(ROOT / "contracts", root / "contracts")
        copied_manifest = json.loads((root / "contracts/manifest.json").read_bytes())
        for source in copied_manifest["sources"]:
            if not source["redistributed"]:
                evidence_path = source["licenseEvidence"]["observation"]["path"]
                target = root / evidence_path
                target.parent.mkdir(parents=True, exist_ok=True)
                shutil.copy2(safe_path(evidence_path), target)
        (root / "tools").mkdir()
        shutil.copy2(Path(__file__), root / "tools/verify-contracts.py")
        command = [sys.executable, "-I", str(root / "tools/verify-contracts.py")]
        result = subprocess.run(command, capture_output=True, text=True)
        require(result.returncode == 0, f"Untouched self-test corpus failed: {result.stderr.strip()}")
        manifest_path = root / "contracts/manifest.json"
        original_manifest = manifest_path.read_bytes()
        for case in cases:
            manifest = json.loads(original_manifest)
            changed_path = None
            original_data = None
            if case in ("fixture-change", "fixture-removal"):
                changed_path = root / manifest["fixtures"][0]["path"]
                original_data = changed_path.read_bytes()
                if case == "fixture-change":
                    changed_path.write_bytes(original_data + b"\n")
                else:
                    changed_path.unlink()
            elif case == "unmanifested-source":
                changed_path = root / "contracts/sources/unlicensed.yaml"
                changed_path.write_text("openapi: 3.0.0\n")
            elif case == "license-removal":
                changed_path = root / manifest["licenses"][0]["path"]
                original_data = changed_path.read_bytes()
                changed_path.unlink()
            elif case == "fixture-inventory-omission":
                manifest["fixtures"].pop(0)
            elif case == "baseline-source-omission":
                manifest["baseline"]["srcFiles"].pop(0)
                manifest["baseline"]["srcTreeSha256"] = hashlib.sha256(canonical(manifest["baseline"]["srcFiles"])).hexdigest()
            elif case == "tree-ledger-omission":
                changed_path = root / manifest["baseline"]["gitTreeLedger"]
                original_data = changed_path.read_bytes()
                ledger = json.loads(original_data)
                ledger["files"].pop(0)
                changed_path.write_bytes(canonical(ledger))
                manifest["baseline"]["gitTreeLedgerSha256"] = hashlib.sha256(changed_path.read_bytes()).hexdigest()
            elif case == "excluded-source-generation":
                next(entry for entry in manifest["sources"] if not entry["redistributed"])["usedForGeneration"] = True
            elif case == "export-source-change":
                changed_path = root / "contracts/typescript.json"
                original_data = changed_path.read_bytes()
                inventory = json.loads(original_data)
                inventory["baseline"]["sources"][0]["sha256"] = "0" * 64
                changed_path.write_bytes(canonical(inventory))
            elif case == "fixture-category-relabel":
                next(entry for entry in manifest["fixtures"] if entry["category"] == "sdk-authored-regression-reference")["category"] = "independent-openapi-fixture"
            elif case == "fixture-category-unknown":
                manifest["fixtures"][0]["category"] = "unreviewed"
            elif case == "specification-pin-removal":
                next(entry for entry in manifest["fixtures"] if entry["category"] == "independent-openapi-fixture").pop("specification")
            elif case == "observed-specification-pin":
                observed = next(entry for entry in manifest["fixtures"] if entry["category"] == "sanitized-observed-response")
                observed["specification"] = next(entry for entry in manifest["fixtures"] if "specification" in entry)["specification"]
            elif case == "extraction-input-omission":
                manifest["baseline"]["extractionInputs"].pop(0)
            elif case == "extraction-input-origin-change":
                manifest["baseline"]["extractionInputs"][0]["gitBlobSha1"] = "0" * 40
            elif case == "license-evidence-change":
                official = next(entry for entry in manifest["sources"] if not entry["redistributed"])
                changed_path = root / official["licenseEvidence"]["observation"]["path"]
                original_data = changed_path.read_bytes()
                changed_path.write_bytes(original_data + b"\n")
            elif case == "unowned-vendor-file":
                changed_path = root / VENDOR_ROOTS[0] / "unowned.json"
                changed_path.write_text("{}\n")
            elif case in ("missing-vendor-file", "modified-vendor-file", "vendor-symlink"):
                changed_path = root / manifest["vendorArtifacts"][0]["path"]
                original_data = changed_path.read_bytes()
                if case == "missing-vendor-file":
                    changed_path.unlink()
                elif case == "vendor-symlink":
                    changed_path.unlink()
                    changed_path.symlink_to(root / manifest["licenses"][0]["path"])
                else:
                    changed_path.write_bytes(original_data + b"\n")
            elif case == "vendor-license-commit":
                vendor_license = manifest["vendorArtifacts"][0]["license"]
                next(entry for entry in manifest["licenses"] if entry["id"] == vendor_license)["commit"] = "0" * 40
            elif case == "duplicate-source-id":
                manifest["sources"].append(manifest["sources"][-1].copy())
            elif case == "unowned-vendor-version":
                changed_path = root / Path(VENDOR_ROOTS[0]).parent / "0.87.0" / "unowned.json"
                changed_path.parent.mkdir()
                changed_path.write_text("{}\n")
            elif case == "vendor-artifact-origin":
                manifest["vendorArtifacts"][0]["commit"] = "0" * 40
            elif case == "vendor-artifact-omission":
                manifest["vendorArtifacts"].pop(0)
            elif case == "vendor-artifact-escape":
                manifest["vendorArtifacts"][0]["path"] = VENDOR_ROOTS[0] + "/../escape.json"
            manifest_path.write_bytes(canonical(manifest))
            result = subprocess.run(command, capture_output=True, text=True)
            require(result.returncode != 0 and "Contract verification failed:" in result.stderr,
                    f"Verifier accepted tampered corpus: {case}")
            manifest_path.write_bytes(original_manifest)
            if changed_path is not None:
                if changed_path.is_symlink():
                    changed_path.unlink()
                if original_data is not None:
                    changed_path.write_bytes(original_data)
                else:
                    changed_path.unlink()
        # Exercise the actual extracted non-src byte check without requiring a
        # full external baseline checkout: package.json is already bundled.
        manifest = json.loads(original_manifest)
        package = next(entry for entry in manifest["baseline"]["extractionInputs"] if entry["upstreamPath"] == "package.json")
        extracted = root / "temporary-baseline"
        extracted.mkdir()
        package_bytes = (root / "contracts/sources/typescript/package.json").read_bytes()
        (extracted / "package.json").write_bytes(package_bytes)
        verify_extracted_inputs([package], extracted)
        (extracted / "package.json").write_bytes(package_bytes + b"\n")
        try:
            verify_extracted_inputs([package], extracted)
        except ValueError:
            pass
        else:
            raise ValueError("Verifier accepted changed extracted package.json")
    print(json.dumps({"status": "verified", "rejectedTamperingCases": [*cases, "extracted-non-src-byte-change"]}, indent=2))


def main() -> None:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--baseline", type=Path, help="Verify an extracted baseline src tree")
    parser.add_argument("--fetch-excluded", action="store_true", help="Verify excluded official Gateway bytes remotely, in memory only")
    parser.add_argument("--self-test", action="store_true", help="Verify that corpus tampering is rejected using temporary copies")
    args = parser.parse_args()
    if args.self_test:
        require(args.baseline is None and not args.fetch_excluded, "Run --self-test separately from optional source audits")
        self_test()
        return
    manifest = json.loads((ROOT / "contracts/manifest.json").read_bytes())
    require(manifest["schemaVersion"] == 1, "Unsupported manifest schema")
    baseline = manifest["baseline"]
    require(baseline["commit"] == PIN and baseline["version"] == VERSION, "Baseline pin differs")
    require(manifest["unresolvedExactSourceHashes"] == [], "Unresolved fixture source hashes")

    owned_files: set[str] = set()

    def tracked(record: dict) -> bytes:
        require(record["path"] not in owned_files, f"Duplicate bundled path: {record['path']}")
        owned_files.add(record["path"])
        return verify_file(record)

    commit = tracked(baseline["commitObject"])
    require(git_hash("commit", commit) == PIN, "Baseline commit object differs")
    require(baseline["commitObject"]["gitObjectSha1"] == PIN, "Commit record differs")
    require(commit.splitlines()[0].decode() == "tree " + baseline["rootTreeSha1"], "Commit tree differs")
    ledger_path = baseline["gitTreeLedger"]
    owned_files.add(ledger_path)
    ledger_bytes = safe_path(ledger_path).read_bytes()
    require(hashlib.sha256(ledger_bytes).hexdigest() == baseline["gitTreeLedgerSha256"], "Baseline Git tree ledger differs")
    ledger = json.loads(ledger_bytes)
    require(ledger["commit"] == PIN, "Git tree ledger pin differs")
    require(git_tree_hash(ledger["files"]) == baseline["rootTreeSha1"], "Incomplete or modified baseline Git tree")
    upstream = {entry["path"]: entry for entry in ledger["files"]}

    source_paths = [entry["upstreamPath"] for entry in baseline["srcFiles"]]
    require(source_paths == sorted(path for path in upstream if path.startswith("src/")), "Baseline src file inventory is incomplete or unordered")
    require(hashlib.sha256(canonical(baseline["srcFiles"])).hexdigest() == baseline["srcTreeSha256"], "Baseline source digest differs")

    def verify_origin(record: dict) -> None:
        original = upstream.get(record["upstreamPath"])
        require(original is not None, f"Unknown baseline file: {record['upstreamPath']}")
        require(original["gitBlobSha1"] == record["gitBlobSha1"] and original["bytes"] == record["bytes"], f"Baseline Git origin differs: {record['upstreamPath']}")

    for entry in baseline["srcFiles"]:
        verify_origin(entry)
        if args.baseline is not None:
            path = args.baseline / entry["upstreamPath"]
            require(path.is_file() and not path.is_symlink(), f"Missing extracted baseline file: {entry['upstreamPath']}")
            verify_bytes(entry, path.read_bytes(), entry["upstreamPath"])
    input_paths = [entry["upstreamPath"] for entry in baseline["extractionInputs"]]
    require(len(input_paths) == len(set(input_paths)) and EXTRACTION_INPUTS <= set(input_paths),
            "Extracted non-src input inventory is incomplete or duplicated")
    require(all(not path.startswith("src/") for path in input_paths), "Extraction inputs must identify non-src baseline files")
    for entry in baseline["extractionInputs"]:
        verify_origin(entry)
    if args.baseline is not None:
        actual = {path.relative_to(args.baseline).as_posix() for path in (args.baseline / "src").rglob("*") if path.is_file()}
        require(actual == set(source_paths), "Extracted baseline has extra or missing src files")
        verify_extracted_inputs(baseline["extractionInputs"], args.baseline.resolve())

    licenses = {entry["id"]: entry for entry in manifest["licenses"]}
    require(len(licenses) == len(manifest["licenses"]), "Duplicate license IDs")
    for entry in licenses.values():
        tracked(entry)
        require(entry["spdx"] in ("MIT", "Apache-2.0"), "Unreviewed redistribution license")
        if entry["id"] == "typescript":
            verify_origin(entry)
    require(baseline["license"] in licenses, "Missing baseline license")

    sources = {}
    excluded = []
    for entry in manifest["sources"]:
        require(entry["id"] not in sources, f"Duplicate specification: {entry['id']}")
        sources[entry["id"]] = entry
        require(bool(re.fullmatch(r"[0-9a-f]{40}", entry["commit"])), "Invalid source commit")
        if entry["redistributed"]:
            tracked(entry)
            require(entry["license"] in licenses, f"Missing source license: {entry['id']}")
            license_record = licenses[entry["license"]]
            require(license_record["repository"] == entry["repository"] and license_record["commit"] == entry["commit"], f"License origin mismatch: {entry['id']}")
        else:
            excluded.append(entry)
            require("path" not in entry and entry["usedForGeneration"] is False, "Excluded source may not be bundled or used for generation")
            require(entry["licenseStatus"] == "unresolved", "Excluded source license status differs")
            observation = entry["licenseEvidence"]["observation"]
            evidence = json.loads(verify_file(observation))
            observed_at = datetime.fromisoformat(evidence["observedAt"])
            require(observed_at.utcoffset() == timedelta(0) and observation["observedAt"] == evidence["observedAt"],
                    "Excluded-source license evidence requires a dated UTC observation")
            require(evidence["repository"] == entry["repository"] and evidence["commit"] == entry["commit"],
                    "Excluded-source license evidence has a different source pin")
            observations = {item["kind"]: item for item in evidence["requests"]}
            tree = observations["pinned-tree"]
            primary_base = entry["repository"].replace("https://github.com/", "https://api.github.com/repos/")
            require(tree["url"] == primary_base + "/git/trees/" + entry["commit"] + "?recursive=1"
                    and observations["repository-license"]["url"] == primary_base + "/license",
                    "License evidence must identify the pinned primary source endpoints")
            require(tree["httpStatus"] == 200 and tree["truncated"] is False and tree["licenseLikePaths"] == [],
                    "Pinned-tree license observation differs")
            require(tree["openapiBlobSha1"] == entry["gitBlobSha1"], "License evidence source blob differs")
            require(observations["repository-license"]["httpStatus"] == 404,
                    "Repository license observation differs")
            audit = observations["excluded-source-byte-audit"]
            require(audit["httpStatus"] == 200 and audit["url"] == entry["sourceUrl"]
                    and audit["responseSha256"] == entry["sha256"] and audit["gitBlobSha1"] == entry["gitBlobSha1"]
                    and audit["responseBytes"] == entry["bytes"] and audit["infoLicenseDeclarationPresent"] is False,
                    "Dated source-byte observation differs from excluded pin")
            if args.fetch_excluded:
                request = Request(entry["sourceUrl"], headers={"User-Agent": "scm-sdk-contract-audit"})
                with urlopen(request, timeout=30) as response:
                    require(response.geturl() == entry["sourceUrl"], "Unexpected excluded-source redirect")
                    data = response.read(entry["bytes"] + 1)
                verify_bytes(entry, data, entry["id"])
    require({entry["id"] for entry in excluded} == {"official-gateway/openapi.yaml"}, "Excluded official Gateway pin is missing")
    require(any(entry["source"] == "official-gateway/openapi.yaml" and entry["status"] == "excluded-pending-license" for entry in manifest["distributionConstraints"]), "Missing official Gateway distribution constraint")

    for entry in manifest["vendorArtifacts"]:
        require(any(Path(entry["path"]).is_relative_to(Path(root)) for root in VENDOR_ROOTS),
                f"Unregistered vendor artifact root: {entry['path']}")
        tracked(entry)
        source = sources.get(entry["source"])
        require(source is not None and source["redistributed"],
                f"Missing redistributed vendor artifact source: {entry['path']}")
        require(any(Path(source["path"]).is_relative_to(Path(root)) for root in VENDOR_ROOTS),
                f"Vendor artifact source is outside registered roots: {entry['path']}")
        for key in ("repository", "commit", "license"):
            require(entry[key] == source[key], f"Vendor artifact {key} origin mismatch: {entry['path']}")
        license_record = licenses[entry["license"]]
        require(license_record["repository"] == entry["repository"]
                and license_record["commit"] == entry["commit"],
                f"Vendor artifact license origin mismatch: {entry['path']}")

    required_prefix = manifest["fixturePolicy"]["requiredTreePrefix"]
    require(required_prefix == "test/openapi/", "Complete OpenAPI fixture tree is required")
    required = {path for path in upstream if path.startswith(required_prefix)}
    required.update(manifest["fixturePolicy"]["additionalUpstreamPaths"])
    actual = [entry["upstreamPath"] for entry in manifest["fixtures"]]
    require(len(set(actual)) == len(actual) and set(actual) == required, "Fixture inventory is incomplete or duplicated")

    corrections = manifest["corrections"]
    correction_ids = {entry["id"] for entry in corrections["catalog"]}
    for entry in manifest["fixtures"]:
        data = tracked(entry)
        verify_origin(entry)
        require(entry["category"] in FIXTURE_CATEGORIES, f"Unreviewed fixture category: {entry['path']}")
        require(entry["category"] == fixture_category(entry["upstreamPath"]),
                f"Fixture category differs from its baseline origin: {entry['path']}")
        require(("specification" in entry) == (entry["category"] == "independent-openapi-fixture"),
                f"Only independent OpenAPI fixtures require a specification pin: {entry['path']}")
        if entry["upstreamPath"].endswith(".json"):
            fixture = json.loads(data)
            if "specification" in entry:
                pin = entry["specification"]
                require(pin == {"source": fixture["source"], "sha256": fixture["sha256"]}, f"Fixture source metadata differs: {entry['path']}")
                source_id = pin["source"].removeprefix("/home/cdot/development/others/")
                require(source_id in sources and sources[source_id]["sha256"] == pin["sha256"], f"Fixture source pin has no exact bundled source: {entry['path']}")
                require(sources[source_id]["redistributed"], "Fixture source is excluded from redistribution")
            for correction in fixture.get("corrections", []):
                require(correction in correction_ids, f"Unknown fixture correction: {correction}")
        # Guard against common accidental credential capture. Dummy literals in
        # regression references remain permitted; this does not replace review.
        require(b"-----BEGIN PRIVATE KEY-----" not in data and b"-----BEGIN RSA PRIVATE KEY-----" not in data, f"Private key in fixture: {entry['path']}")
        require(re.search(rb"eyJ[A-Za-z0-9_-]{8,}\.[A-Za-z0-9_-]{16,}\.[A-Za-z0-9_-]{16,}", data) is None, f"JWT-shaped value in fixture: {entry['path']}")

    for entry in manifest["references"]:
        tracked(entry)
        verify_origin(entry)
        require(entry["license"] in licenses, f"Missing reference license: {entry['path']}")
    compat = safe_path(corrections["primaryReference"]).read_text()
    catalog_text = compat.split("export const compatibilityCorrections = [", 1)[1].split("] as const", 1)[0]
    declared_ids = set(re.findall(r"id: '([^']+)'", catalog_text))
    require(declared_ids == correction_ids, "Correction catalog differs from original source")
    for path in corrections["additionalReferences"]:
        require(path in owned_files, f"Untracked correction reference: {path}")

    for entry in manifest["documentation"]:
        tracked(entry)

    # The export inventory is independently extracted by TypeScript's compiler.
    # Bind its source fingerprint to the same immutable Git baseline.
    inventory_path = ROOT / "contracts/typescript.json"
    require(inventory_path.is_file(), "Missing complete TypeScript export inventory")
    inventory = json.loads(inventory_path.read_bytes())["baseline"]
    require(inventory["commit"] == PIN and inventory["version"] == VERSION and inventory["package"] == baseline["package"], "Export inventory baseline differs")
    expected_sources = [{"file": entry["upstreamPath"], "sha256": entry["sha256"]} for entry in baseline["srcFiles"]]
    require(sorted(inventory["sources"], key=lambda entry: entry["file"]) == expected_sources, "Export inventory source hashes differ")
    # The compiler inventory uses an explicit English filename collation; its
    # fingerprint preserves that ordering while identity checks compare paths.
    expected_fingerprint = hashlib.sha256("".join(f"{entry['sha256']}  {entry['file']}\n" for entry in inventory["sources"]).encode()).hexdigest()
    require(inventory["sourceFingerprint"] == expected_fingerprint, "Export inventory fingerprint differs")

    expected_paths = owned_files
    # Own the product subtree, including unregistered future version directories.
    # Other products retain their existing dedicated verifier ownership.
    vendor_parents = sorted({Path(directory).parent.as_posix() for directory in VENDOR_ROOTS})
    for parent in vendor_parents:
        for path in safe_path(parent).rglob("*"):
            relative = path.relative_to(ROOT)
            safe_path(relative.as_posix())
            if path.is_file():
                require(any(relative.is_relative_to(Path(directory)) for directory in VENDOR_ROOTS),
                        f"Unregistered vendor version file: {relative.as_posix()}")
    corpus_roots = ("contracts/fixtures", "contracts/sources", "contracts/licenses", *VENDOR_ROOTS)
    actual_paths = set()
    for directory in corpus_roots:
        directory_path = safe_path(directory)
        require(directory_path.is_dir(), f"Missing corpus root: {directory}")
        for path in directory_path.rglob("*"):
            # Reject directory symlinks too; rglob does not descend through them.
            safe_path(path.relative_to(ROOT).as_posix())
            if path.is_file():
                actual_paths.add(path.relative_to(ROOT).as_posix())
    require(expected_paths == actual_paths, f"Unmanifested/missing corpus files: {sorted(expected_paths ^ actual_paths)}")
    print(json.dumps({"status": "verified", "baseline": VERSION, "commit": PIN,
                      "fixtures": len(manifest["fixtures"]), "bundledSpecifications": len(sources) - len(excluded),
                      "excludedUnlicensedSpecifications": len(excluded), "licenses": len(licenses),
                      "references": len(manifest["references"]), "baselineSourceFiles": len(source_paths),
                      "baselineNonSourceInputs": len(input_paths),
                      "extractedBaselineVerified": args.baseline is not None,
                      "excludedRemoteBytesVerified": args.fetch_excluded}, indent=2))


if __name__ == "__main__":
    try:
        main()
    except (KeyError, TypeError, ValueError, OSError) as error:
        print(f"Contract verification failed: {error}", file=sys.stderr)
        sys.exit(1)
