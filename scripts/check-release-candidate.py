#!/usr/bin/env python3
"""Bind the 0.2.0 release to unchanged accepted code and executed clean archives."""
import argparse
import hashlib
import importlib.util
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]
BASELINE = "167a359fb3d51aad37c4dff311060b069c620bdd"
PROTECTED = ("Cargo.toml", "Cargo.lock", "LICENSE", "NOTICE", "crates/scm-sdk/Cargo.toml",
             "crates/scm-sdk/src", "crates/scm-sdk/examples")


def sha(data):
    return hashlib.sha256(data).hexdigest()


def git(root, *args):
    return subprocess.check_output(["git", *args], cwd=root)


def unchanged_runtime(root, baseline=BASELINE):
    names = git(root, "ls-tree", "-r", "--name-only", "-z", baseline, "--", *PROTECTED)
    expected = set(names.decode().split("\0")) - {""}
    actual = set()
    for relative in PROTECTED:
        path = root / relative
        entries = [path, *path.rglob("*")] if path.is_dir() else [path]
        for entry in entries:
            if entry.is_symlink():
                raise ValueError("Protected inputs must not be symlinks")
            if entry.is_file():
                actual.add(entry.relative_to(root).as_posix())
    if not expected or actual != expected:
        raise ValueError("Protected input inventory differs from accepted source")
    rows = []
    for name in sorted(expected):
        data = (root / name).read_bytes()
        if data != git(root, "show", baseline + ":" + name):
            raise ValueError("Protected input changed: " + name)
        rows.append({"path": name, "sha256": sha(data), "bytes": len(data)})
    return rows


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


def verify(root):
    rows = unchanged_runtime(root)
    publication = load_module("release_publication", root / "scripts/verify-publication.py")
    proofs = []
    for compiler in ("1.89.0", "1.99.0"):
        path = root / f"docs/checks/phase-14-release-package-{compiler}.json"
        if path.is_symlink():
            raise ValueError("Proof must be a regular file")
        proof = json.loads(path.read_text())
        publication.require_proof(proof)
        if proof["toolchain"] != compiler:
            raise ValueError("Compiler proof identity differs")
        subprocess.run(["git", "merge-base", "--is-ancestor", proof["releaseCommit"], "HEAD"],
                       cwd=root, check=True, capture_output=True)
        archive = pathlib.Path(proof["retainedArchive"])
        allowed = root / ".local/artifacts"
        if (not archive.is_absolute() or archive.parent != allowed
                or any(p.is_symlink() for p in (archive, *archive.parents))):
            raise ValueError("Archive must be a regular retained repository artifact")
        data = archive.read_bytes()
        if sha(data) != proof["archive_sha256"] or len(data) != proof["archive_bytes"]:
            raise ValueError("Retained archive differs from executed proof")
        files = publication.policy.inspect_archive(data, "scm-sdk-0.2.0/")
        if publication.file_rows(files) != proof["files"]:
            raise ValueError("Archive file inventory differs from proof")
        if publication.policy.validate_vcs(files, proof["releaseCommit"], True) != proof["vcs"]:
            raise ValueError("Archive source identity differs")
        for name, content in files.items():
            if name.startswith(("src/", "examples/")):
                relative = "crates/scm-sdk/" + name
            elif name == "Cargo.toml.orig":
                relative = "crates/scm-sdk/Cargo.toml"
            elif name in {"README.md", "LICENSE", "NOTICE"}:
                relative = name
            else:
                continue
            current = root / relative
            if current.is_symlink() or current.read_bytes() != content:
                raise ValueError("Current packaged input differs: " + relative)
            if git(root, "show", proof["releaseCommit"] + ":" + relative) != content:
                raise ValueError("Packaged input differs from release commit")
        for example in ("response_defaults", "red_team_dataplane"):
            if not any(c["command"][2:6] == ["run", "--example", example, "--all-features"]
                       for c in proof["checks"]):
                raise ValueError("Offline example was not executed: " + example)
        if proof["publicConsumerSourceSha256"] != sha((root / "contracts/native-probes/src/main.rs").read_bytes()):
            raise ValueError("External consumer differs from executed proof")
        proofs.append(proof)
    if any(proofs[0][key] != proofs[1][key] for key in ("releaseCommit", "files", "vcs")):
        raise ValueError("Compiler proofs do not qualify identical source files")
    return {"schemaVersion": 1, "passed": True, "version": "0.2.0", "baselineCommit": BASELINE,
            "releaseCommit": proofs[0]["releaseCommit"], "protectedFiles": rows,
            "readmeSha256": sha((root / "README.md").read_bytes()),
            "archiveSha256": proofs[1]["archive_sha256"], "toolchains": [p["toolchain"] for p in proofs]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=pathlib.Path, required=True)
    args = parser.parse_args()
    result = verify(ROOT)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({k: v for k, v in result.items() if k != "protectedFiles"}))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError, subprocess.SubprocessError) as error:
        sys.exit("Release candidate verification failed: " + str(error))
