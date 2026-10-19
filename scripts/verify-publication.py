#!/usr/bin/env python3
"""Verify a crates.io artifact against executed clean-package qualification."""
import argparse
import datetime
import hashlib
import importlib.util
import json
import os
import pathlib
import re
import sys
import tomllib
import urllib.error
import urllib.request

ROOT = pathlib.Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("publication_package_policy", ROOT / "scripts/check-package.py")
policy = importlib.util.module_from_spec(spec)
spec.loader.exec_module(policy)


def sha(data):
    return hashlib.sha256(data).hexdigest()


def file_rows(files):
    return [{"path": p, "sha256": sha(data), "bytes": len(data)}
            for p, data in sorted(files.items())]


def require_proof(proof, clean=True, schema_version=2):
    checks = proof.get("checks", []) if isinstance(proof, dict) else []
    required = [
        ["package", "-p", "scm-sdk"], ["test", "--lib", "--no-default-features"],
        ["test", "--lib", "--all-features"], ["check", "--examples", "--all-features"],
        ["test", "--doc", "--all-features"], ["run", "--locked", "--offline"],
    ]
    if not (isinstance(proof, dict) and proof.get("schemaVersion") == schema_version
            and proof.get("passed") is True
            and (not clean or (proof.get("requireClean") is True and proof.get("effectiveDirty") is False))
            and isinstance(proof.get("releaseCommit"), str)
            and re.fullmatch("[a-f0-9]{40}", proof.get("releaseCommit", ""))
            and isinstance(proof.get("toolchain"), str)
            and proof["toolchain"] in {"1.89.0", "1.99.0"}
            and isinstance(proof.get("rustcVersion"), str)
            and proof["rustcVersion"].startswith("rustc " + proof["toolchain"] + " ")
            and isinstance(checks, list) and len(checks) >= len(required)
            and all(isinstance(c, dict) and type(c.get("exit_code")) is int
                    and c["exit_code"] == 0 and isinstance(c.get("command"), list)
                    and len(c["command"]) > 2 and c["command"][1] == "+" + proof["toolchain"]
                    for c in checks)
            and all(any(c["command"][2:2 + len(prefix)] == prefix for c in checks) for prefix in required)):
        raise ValueError("Publication requires an executed passing clean-package proof")


def require_release_readiness(version, review_path):
    """Bind reviewed release registry receipt creation to actual current-source Claude approval."""
    if version not in {"0.2.2", "0.2.3"}:
        return None
    if review_path is None:
        raise ValueError(version + " requires --readiness-review before registry verification")
    sys.path.insert(0, str(ROOT / "tools"))
    from review_phase import fingerprint, verify_release_readiness
    path = review_path if review_path.is_absolute() else ROOT / review_path
    if path.is_symlink() or not path.resolve().is_relative_to(ROOT.resolve()):
        raise ValueError("Readiness review must be retained inside the repository")
    source = fingerprint(ROOT)
    if not (verify_release_readiness(ROOT, path, source) if version == "0.2.2" else
            verify_release_readiness(ROOT, path, source, phase=24)):
        raise ValueError("Publication requires an accepted current-source release-readiness review")
    return {"review": path.relative_to(ROOT).as_posix(), "reviewSha256": sha(path.read_bytes()),
            "sourceSha256": source}


def verify_management_proofs(proof_path, msrv_path):
    """Schema 3 cannot be accepted without its complete source/archive verifier."""
    if msrv_path is None:
        raise ValueError('0.2.3 requires both clean compiler package proofs')
    spec = importlib.util.spec_from_file_location('publication_management_package', ROOT / 'scripts/check-management-package.py')
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    module.verify_receipt(ROOT, proof_path, '1.99.0')
    module.verify_receipt(ROOT, msrv_path, '1.89.0')


def verify_response_examples(proof, files):
    for example in ("response_defaults", "response_compatibility"):
        if f"examples/{example}.rs" in files and not any(
            check["command"][2:6] == ["run", "--example", example, "--all-features"]
            for check in proof["checks"]
        ):
            raise ValueError("The response policy example was not executed from the archive: " + example)


def verify(version, proof, metadata, owners, archive, msrv_proof=None):
    require_proof(proof, schema_version=3 if version == "0.2.3" else 2)
    if version == "0.2.3" and (msrv_proof is None or proof.get("toolchain") != "1.99.0"):
        raise ValueError("0.2.3 registry verification requires development and MSRV proofs")
    if not (isinstance(metadata, dict) and isinstance(metadata.get("version"), dict)
            and isinstance(owners, dict) and isinstance(owners.get("users"), list)
            and all(isinstance(u, dict) and isinstance(u.get("login"), str) for u in owners["users"])):
        raise ValueError("Unexpected registry response structure")
    record = metadata["version"]
    logins = sorted({u["login"] for u in owners["users"]})
    digest = sha(archive)
    if not (record["num"] == version and record["crate"] == "scm-sdk"
            and record["yanked"] is False and "cdot65" in logins
            and record["checksum"] == digest == proof["archive_sha256"]
            and len(archive) == proof["archive_bytes"]):
        raise ValueError("Registry identity, ownership or archive checksum differs from qualification")
    files = policy.inspect_archive(archive, "scm-sdk-" + version + "/")
    vcs = policy.validate_vcs(files, proof["releaseCommit"], True)
    manifest = tomllib.loads(files["Cargo.toml"].decode())
    if manifest["package"]["name"] != "scm-sdk" or manifest["package"]["version"] != version:
        raise ValueError("Registry manifest identity differs")
    rows = file_rows(files)
    if rows != proof["files"] or vcs != proof["vcs"]:
        raise ValueError("Published files or VCS metadata differ from the qualified archive")
    verify_response_examples(proof, files)
    if msrv_proof is not None:
        require_proof(msrv_proof, schema_version=3 if version == "0.2.3" else 2)
        if not (msrv_proof["toolchain"] == "1.89.0"
                and msrv_proof["releaseCommit"] == proof["releaseCommit"]
                and msrv_proof["files"] == rows and msrv_proof["vcs"] == vcs):
            raise ValueError("MSRV proof does not qualify the identical package file contents")
    return {
        "schemaVersion": 1, "checkedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "registry": "crates.io", "package": "scm-sdk", "version": version,
        "publishedAt": record["created_at"], "yanked": False, "ownerLogins": logins,
        "archiveSha256": digest, "archiveBytes": len(archive),
        "sourceCommit": proof["releaseCommit"], "vcs": vcs, "files": rows,
        "qualifiedToolchain": proof["toolchain"],
        "identicalFileContentsQualifiedOnMsrv": msrv_proof is not None,
        "limitations": [
            "Digests bind the artifact to retained executed proofs; repository writers remain trusted.",
            "MSRV and development Cargo may use different archive metadata; compared file bytes must be identical.",
            "Registry publication does not expand native platform or live service acceptance.",
        ],
    }


def package_delta(baseline, candidate, version):
    before = policy.inspect_archive(baseline, "scm-sdk-0.1.0/")
    after = policy.inspect_archive(candidate, "scm-sdk-" + version + "/")
    changes = []
    for name in sorted(before.keys() | after.keys()):
        old, new = before.get(name), after.get(name)
        if old != new:
            changes.append({"path": name,
                            "change": "added" if old is None else "removed" if new is None else "changed",
                            "beforeSha256": sha(old) if old is not None else None,
                            "afterSha256": sha(new) if new is not None else None})
    return {"schemaVersion": 1, "baselineVersion": "0.1.0", "candidateVersion": version,
            "baselineArchiveSha256": sha(baseline), "candidateArchiveSha256": sha(candidate),
            "changes": changes}


def fetch(url, limit):
    request = urllib.request.Request(url, headers={
        "User-Agent": "scm-sdk maintainer artifact verification (https://git.cdot.io/cdot/scm-sdk-rs)",
    })
    with urllib.request.urlopen(request, timeout=60) as response:
        data = response.read(limit + 1)
    if len(data) > limit:
        raise ValueError("Registry response exceeded the verification limit")
    return data


def create_file(path, data):
    path.parent.mkdir(parents=True, exist_ok=True)
    with os.fdopen(os.open(path, os.O_WRONLY | os.O_CREAT | os.O_EXCL, 0o600), "wb") as stream:
        stream.write(data)


def check_delta(version, proof, output):
    require_proof(proof, clean=False)
    candidate_path = pathlib.Path(proof["retainedArchive"])
    if candidate_path.is_symlink():
        raise ValueError("Candidate archive must be a regular owned file")
    candidate = candidate_path.read_bytes()
    if sha(candidate) != proof["archive_sha256"]:
        raise ValueError("Qualified candidate archive changed")
    files = policy.inspect_archive(candidate, "scm-sdk-" + version + "/")
    if file_rows(files) != proof["files"]:
        raise ValueError("Candidate file hashes differ from its proof")
    for name, data in files.items():
        if name.startswith(("src/", "examples/")):
            current = ROOT / "crates/scm-sdk" / name
        elif name in {"README.md", "LICENSE", "NOTICE"}:
            current = ROOT / name
        elif name == "Cargo.toml.orig":
            current = ROOT / "crates/scm-sdk/Cargo.toml"
        else:
            continue
        if current.is_symlink() or current.read_bytes() != data:
            raise ValueError("Candidate source differs from current input: " + name)
    baseline_receipt = json.loads((ROOT / "docs/checks/release-registry-0.1.0.json").read_text())
    cached = ROOT / ".local/artifacts/scm-sdk-0.1.0-registry.crate"
    baseline = cached.read_bytes() if cached.is_file() and not cached.is_symlink() else fetch(
        "https://crates.io/api/v1/crates/scm-sdk/0.1.0/download", 64 * 1024 * 1024)
    if sha(baseline) != baseline_receipt["archiveSha256"]:
        raise ValueError("Published baseline archive checksum differs")
    if not cached.exists():
        create_file(cached, baseline)
    delta = package_delta(baseline, candidate, version)
    allowed = {
        ".cargo_vcs_info.json", "Cargo.lock", "Cargo.toml", "Cargo.toml.orig", "README.md",
        "src/config.rs", "src/engine.rs", "src/engine/tests.rs", "src/streaming.rs",
        "src/validation.rs", "examples/response_defaults.rs",
    }
    if any(c["path"] not in allowed or c["change"] == "removed" for c in delta["changes"]):
        raise ValueError("Unexpected packaged change outside the reviewed G10 delta")
    delta.update({"passed": True, "baselineSourceCommit": baseline_receipt["sourceCommit"],
                  "candidateSourceCommit": proof["releaseCommit"],
                  "candidateEffectiveDirty": proof["effectiveDirty"],
                  "candidateProofSha256": sha(json.dumps(proof, sort_keys=True).encode())})
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(delta, indent=2) + "\n")
    print(json.dumps({"packageDeltaPassed": True, "changedFiles": [c["path"] for c in delta["changes"]]}))


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--version", required=True)
    parser.add_argument("--proof", type=pathlib.Path, required=True)
    parser.add_argument("--msrv-proof", type=pathlib.Path)
    parser.add_argument("--readiness-review", type=pathlib.Path,
                        help="Accepted current-source phase20/24 readiness review; required for 0.2.2/0.2.3")
    parser.add_argument("--output", type=pathlib.Path, required=True)
    parser.add_argument("--delta", action="store_true", help="Compare the qualified candidate with published 0.1.0")
    args = parser.parse_args()
    if not re.fullmatch(r"\d+\.\d+\.\d+", args.version):
        raise ValueError("Use an exact numeric release version")
    if args.output.exists() and not args.delta:
        raise ValueError("Preserve the existing publication receipt")
    readiness = require_release_readiness(args.version, args.readiness_review)
    proof = json.loads(args.proof.read_text())
    if args.delta:
        return check_delta(args.version, proof, args.output)
    require_proof(proof, schema_version=3 if args.version == "0.2.3" else 2)
    msrv = json.loads(args.msrv_proof.read_text()) if args.msrv_proof else None
    if args.version == "0.2.3":
        verify_management_proofs(args.proof, args.msrv_proof)
    base = "https://crates.io/api/v1/crates/scm-sdk"
    version_url, owners_url = base + "/" + args.version, base + "/owners"
    download = base + "/" + args.version + "/download"
    metadata = json.loads(fetch(version_url, 2 * 1024 * 1024))
    owners = json.loads(fetch(owners_url, 2 * 1024 * 1024))
    archive = fetch(download, 64 * 1024 * 1024)
    report = verify(args.version, proof, metadata, owners, archive, msrv)
    retained = ROOT / ".local/artifacts" / (
        "scm-sdk-" + args.version + "-registry-" + report["archiveSha256"][:16] + ".crate")
    if retained.exists():
        if retained.is_symlink() or retained.read_bytes() != archive:
            raise ValueError("Retained registry archive differs")
    else:
        create_file(retained, archive)
    report.update({
        "proofSha256": sha(args.proof.read_bytes()),
        "msrvProofSha256": sha(args.msrv_proof.read_bytes()) if args.msrv_proof else None,
        "verifierSha256": sha(pathlib.Path(__file__).read_bytes()),
        "retainedArchive": retained.relative_to(ROOT).as_posix(),
        "sources": [version_url, owners_url, download],
    })
    if args.version in {"0.2.2", "0.2.3"}:
        if require_release_readiness(args.version, args.readiness_review) != readiness:
            raise ValueError("Readiness or reviewed source changed during registry verification")
        report["releaseReadiness"] = readiness
    create_file(args.output, (json.dumps(report, indent=2) + "\n").encode())
    print(json.dumps({"version": args.version, "published": True,
                      "sourceCommit": report["sourceCommit"], "archiveSha256": report["archiveSha256"]}))


if __name__ == "__main__":
    try:
        main()
    except urllib.error.URLError:
        sys.exit("Registry verification request failed; no publication receipt was written")
    except (ValueError, OSError, KeyError) as error:
        sys.exit("Publication verification failed: " + str(error))
