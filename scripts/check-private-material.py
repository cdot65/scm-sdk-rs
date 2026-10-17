#!/usr/bin/env python3
"""Privately scan exact values from one explicitly selected external registration."""
import argparse
import datetime
import json
import pathlib
import re
import sys
import time
import tomllib

from evidence_common import (MAX_FILE_BYTES, MAX_FILES, MAX_TREE_BYTES, git_files, git_output, read_regular,
                             safe_parts, source_fingerprint, write_private)

ROOT = pathlib.Path(__file__).resolve().parents[1]
KEYS = {"mgmtClientId", "mgmtClientSecret", "mgmtTsgId"}
LIMITS = {"registrationBytes": 65536, "selectedValueBytes": 4096,
          "filesPerTree": MAX_FILES, "bytesPerTree": MAX_TREE_BYTES,
          "bytesPerFile": MAX_FILE_BYTES, "historicalLookupBudgetSeconds": 120}
SCOPE = "Exact UTF-8 selected values checked in file contents and names; no selected values or config path retained."
CLASSIFICATION = "Only unchanged preexisting HEAD occurrences outside package source may be historical nonsecret tenant identifiers."
LIMITATIONS = [
    "Exact-value scanning does not detect encodings, split/transformed secrets or unrelated credentials.",
    "All regular src/examples files are scanned as a conservative superset; actual archive membership is separately qualified by check-package.py.",
    "HEAD presence is historical evidence, not proof a tenant is public or authorized; no new tenant occurrences are accepted.",
    "Python holds selected strings in process memory and cannot guarantee zeroization; no raw values, registration content or secret hashes are persisted.",
    "Credential-free --check validates source freshness and structure only; it does not re-execute private matching or certify later changes in excluded review/check/status bookkeeping."]


def registration(path, root):
    path = safe_parts(path)
    if path.is_relative_to(root):
        raise ValueError("Registration must be outside the repository")
    def unique_object(pairs):
        result = {}
        for name, value in pairs:
            if name in result:
                raise ValueError("Duplicate registration field")
            result[name] = value
        return result
    value = json.loads(read_regular(path, 65536), object_pairs_hook=unique_object)
    if not isinstance(value, dict) or set(value) != KEYS:
        raise ValueError("Registration must contain exactly the three supported fields")
    for name in KEYS:
        field = value[name]
        minimum = {"mgmtClientId": 8, "mgmtClientSecret": 16, "mgmtTsgId": 1}[name]
        if (not isinstance(field, str) or not minimum <= len(field.encode("utf-8")) <= 4096
                or not field.strip() or field != field.strip()
                or any(ord(character) < 32 or ord(character) == 127 for character in field)):
            raise ValueError("Registration fields must be bounded nonempty strings")
    if not re.fullmatch(r"[0-9]{1,64}", value["mgmtTsgId"]):
        raise ValueError("Tenant identifier must be bounded ASCII digits")
    if len(set(value.values())) != 3:
        raise ValueError("Registration values must be distinct")
    return {name: field.encode("utf-8") for name, field in value.items()}


def published_candidates(root):
    """Scan a conservative superset of the package's explicitly included source."""
    manifest = tomllib.loads(read_regular(root / "crates/scm-sdk/Cargo.toml").decode("utf-8"))
    includes = manifest.get("package", {}).get("include")
    if includes != ["src/**", "examples/**", "LICENSE", "NOTICE", "README.md"]:
        raise ValueError("Published include policy changed; scanner coverage must be reviewed")
    files, total = {}, 0
    for directory in ("src", "examples"):
        base = root / "crates/scm-sdk" / directory
        safe_parts(base)
        if not base.is_dir():
            raise ValueError("Published source directory is missing")
        for index, path in enumerate(base.rglob("*")):
            if index >= MAX_FILES:
                raise ValueError("Published source traversal bound exceeded")
            if path.is_symlink():
                raise ValueError("Published source contains a symlink")
            if path.is_dir():
                continue
            content = read_regular(path)
            total += len(content)
            if len(files) >= MAX_FILES or total > MAX_TREE_BYTES:
                raise ValueError("Published source scan bound exceeded")
            files[path.relative_to(root).as_posix()] = content
    return files


def matching_files(files, selected):
    return {name for name, content in files.items()
            if any(value in content or value in name.encode("utf-8") for value in selected)}


def scan(root, config):
    selected = registration(config, root)
    head = git_output(root, "rev-parse", "--verify", "HEAD").decode().strip()
    if not re.fullmatch(r"[0-9a-f]{40,64}", head):
        raise ValueError("Cannot establish repository history identity")
    before_files, package_files = git_files(root), published_candidates(root)
    before = source_fingerprint(before_files)
    credentials = (selected["mgmtClientId"], selected["mgmtClientSecret"])
    credential_matches = matching_files(before_files, credentials)
    package_matches = matching_files(package_files, selected.values())
    tenant = selected["mgmtTsgId"]
    tenant_matches = matching_files(before_files, [tenant])
    historical, introduced = 0, 0
    deadline = time.monotonic() + LIMITS["historicalLookupBudgetSeconds"]
    # No value, path, hash of a secret, or registration document is retained.
    for name in sorted(tenant_matches):
        remaining = deadline - time.monotonic()
        if remaining <= 0:
            raise ValueError("Historical evidence lookup exceeded its budget")
        try:
            previous = git_output(root, "show", head + ":" + name, timeout=min(30, remaining),
                                  output_limit=MAX_FILE_BYTES)
        except ValueError:
            previous = b""
        current = before_files[name]
        if tenant in previous and current.count(tenant) <= previous.count(tenant) and tenant not in name.encode():
            historical += 1
        else:
            introduced += 1
    unchanged = (before_files == git_files(root) and package_files == published_candidates(root)
                 and head == git_output(root, "rev-parse", "--verify", "HEAD").decode().strip())
    passed = unchanged and not credential_matches and not package_matches and introduced == 0
    return {"schemaVersion": 2, "checkedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(),
            "sourceSha256": before, "headCommit": head, "sourceUnchanged": unchanged, "passed": passed,
            "gitVisibleFilesChecked": len(before_files),
            "selectedExternalClientIdentityOrSecretMatches": len(credential_matches),
            "publishedCrateSourceAndExampleCandidatesChecked": len(package_files),
            "publishedCrateSourceAndExamplesSha256": source_fingerprint(package_files),
            "publishedCrateSourceOrExampleRegistrationValueMatches": len(package_matches),
            "existingTenantIdentifier": {"matchingFilesCount": len(tenant_matches),
                "alreadyPresentInHeadFilesCount": historical, "newOrChangedFilesCount": introduced,
                "classification": CLASSIFICATION},
            "scope": SCOPE, "limits": LIMITS, "limitations": LIMITATIONS}


def check_receipt(root, path):
    """Validate a successful receipt against current public inputs, without secrets."""
    report = json.loads(read_regular(path, 128 * 1024))
    keys = {"schemaVersion", "checkedAt", "sourceSha256", "headCommit", "sourceUnchanged", "passed",
            "gitVisibleFilesChecked", "selectedExternalClientIdentityOrSecretMatches",
            "publishedCrateSourceAndExampleCandidatesChecked",
            "publishedCrateSourceAndExamplesSha256",
            "publishedCrateSourceOrExampleRegistrationValueMatches", "existingTenantIdentifier",
            "scope", "limits", "limitations"}
    if not isinstance(report, dict) or set(report) != keys:
        raise ValueError("Unexpected sanitized receipt fields")
    tenant = report["existingTenantIdentifier"]
    tenant_keys = {"matchingFilesCount", "alreadyPresentInHeadFilesCount", "newOrChangedFilesCount", "classification"}
    if not isinstance(tenant, dict) or set(tenant) != tenant_keys:
        raise ValueError("Unexpected historical receipt fields")
    counts = [report[name] for name in ("gitVisibleFilesChecked", "selectedExternalClientIdentityOrSecretMatches",
              "publishedCrateSourceAndExampleCandidatesChecked", "publishedCrateSourceOrExampleRegistrationValueMatches")]
    counts += [tenant[name] for name in ("matchingFilesCount", "alreadyPresentInHeadFilesCount", "newOrChangedFilesCount")]
    if not all(type(value) is int and 0 <= value <= MAX_FILES for value in counts):
        raise ValueError("Invalid receipt counts")
    checked_at = datetime.datetime.fromisoformat(report["checkedAt"])
    if (checked_at.utcoffset() != datetime.timedelta(0) or type(report["schemaVersion"]) is not int
            or report["schemaVersion"] != 2 or report["sourceUnchanged"] is not True
            or report["passed"] is not True or report["scope"] != SCOPE or report["limits"] != LIMITS
            or report["limitations"] != LIMITATIONS or tenant["classification"] != CLASSIFICATION
            or report["selectedExternalClientIdentityOrSecretMatches"] != 0
            or report["publishedCrateSourceOrExampleRegistrationValueMatches"] != 0
            or tenant["newOrChangedFilesCount"] != 0
            or tenant["matchingFilesCount"] != tenant["alreadyPresentInHeadFilesCount"]):
        raise ValueError("Receipt is not a successful sanitized scan")
    files, package_files = git_files(root), published_candidates(root)
    if (report["sourceSha256"] != source_fingerprint(files)
            or report["publishedCrateSourceAndExampleCandidatesChecked"] != len(package_files)
            or report["publishedCrateSourceAndExamplesSha256"] != source_fingerprint(package_files)
            or not isinstance(report["headCommit"], str)
            or not re.fullmatch(r"[0-9a-f]{40,64}", report["headCommit"])):
        raise ValueError("Receipt is stale or not bound to this source")
    return report


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    mode = parser.add_mutually_exclusive_group(required=True)
    mode.add_argument("--config", type=pathlib.Path,
                      help="Explicit absolute external registration path; no discovery or environment fallback")
    mode.add_argument("--check", type=pathlib.Path, help="Credential-free receipt freshness check, not a private scan")
    parser.add_argument("--output", type=pathlib.Path, help="New sanitized receipt path (scan mode only)")
    args = parser.parse_args()
    if args.check:
        if args.output:
            parser.error("--check does not write receipts")
        report = check_receipt(ROOT, args.check.absolute())
        print(json.dumps({"receiptFresh": True, "privateScanReexecuted": False,
                          "gitVisibleFilesChecked": report["gitVisibleFilesChecked"]}))
        return 0
    if args.output is None:
        parser.error("--config requires --output")
    report = scan(ROOT, args.config)
    write_private(args.output.absolute(), (json.dumps(report, indent=2) + "\n").encode())
    print(json.dumps({"passed": report["passed"],
                      "gitVisibleFilesChecked": report["gitVisibleFilesChecked"],
                      "credentialMatchingFiles": report["selectedExternalClientIdentityOrSecretMatches"],
                      "publishedMatchingFiles": report["publishedCrateSourceOrExampleRegistrationValueMatches"]}))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, OSError, UnicodeError, KeyError, TypeError, RecursionError):
        sys.exit("Private material verification failed; selected input or repository could not be validated.")
