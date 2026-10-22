#!/usr/bin/env python3
"""Bind a selected release to its executed clean archives and exact current payload."""
import argparse
import hashlib
import importlib.util
import json
import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parents[1]



def sha(data):
    return hashlib.sha256(data).hexdigest()


def git(root, *args):
    return subprocess.check_output(["git", *args], cwd=root)


def load_module(name, path):
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


RELEASES = {(17, '0.2.1'), (20, '0.2.2'), (24, '0.2.3'), (28, '0.2.4')}


def verify_offline_delivery(proof, files, version):
    examples = ['response_defaults', 'red_team_dataplane', 'access_policies']
    if version == '0.2.2':
        examples.append('response_compatibility')
    for example in examples:
        if not any(c['command'][2:6] == ['run', '--example', example, '--all-features'] for c in proof['checks']):
            raise ValueError('Offline example was not executed: ' + example)
    for field, example in [('accessPolicyConsumerSourceSha256', 'access_policies')]+(
            [('responseCompatibilityConsumerSourceSha256', 'response_compatibility')] if version == '0.2.2' else []):
        if proof.get(field) != sha(files['examples/' + example + '.rs']):
            raise ValueError('External example consumer differs from qualified source: ' + example)
    expected = 3 if version == '0.2.2' else 2
    if sum(c['command'][2:4] == ['run', '--locked'] for c in proof['checks']) != expected:
        raise ValueError('All packaged external consumers must execute')


def verify(root, phase=17, version='0.2.1'):
    if (phase, version) not in RELEASES:
        raise ValueError('Unreviewed release phase/version pair')
    sys.path.insert(0, str(root / "tools"))
    from release_delivery import commit_fingerprint, retained_package_archive
    from review_phase import fingerprint
    wrapper = None
    if phase in (24, 28):
        wrapper = load_module("release_management_package", root / "scripts/check-management-package.py")
    rows = []
    publication = load_module("release_publication", root / "scripts/verify-publication.py")
    proofs = []
    for compiler in ("1.89.0", "1.99.0"):
        path = root / f"docs/checks/phase-{phase:02d}-release-package-{compiler}.json"
        if path.is_symlink():
            raise ValueError("Proof must be a regular file")
        proof = json.loads(path.read_text())
        if phase in (24, 28):
            publication.require_proof(proof, schema_version=3)
            wrapper.verify_receipt(root, path, compiler, require_directional=True) if phase == 28 else wrapper.verify_receipt(root, path, compiler)
            if commit_fingerprint(root, proof["releaseCommit"]) != fingerprint(root):
                raise ValueError("Package commit differs from current reviewed source")
            subprocess.run(["git", "merge-base", "--is-ancestor", proof["releaseCommit"], "HEAD"], cwd=root, check=True)
        else:
            publication.require_proof(proof)
        if proof["toolchain"] != compiler:
            raise ValueError("Compiler proof identity differs")
        if phase not in (24, 28) and proof["releaseCommit"] != git(root, "rev-parse", "HEAD").decode().strip():
            raise ValueError("Clean proof must qualify the current release commit")
        archive = retained_package_archive(root, proof, phase)
        data = archive.read_bytes()
        if sha(data) != proof["archive_sha256"] or len(data) != proof["archive_bytes"]:
            raise ValueError("Retained archive differs from executed proof")
        files = publication.policy.inspect_archive(data, f"scm-sdk-{version}/")
        if publication.file_rows(files) != proof["files"]:
            raise ValueError("Archive file inventory differs from proof")
        if publication.policy.validate_vcs(files, proof["releaseCommit"], True) != proof["vcs"]:
            raise ValueError("Archive source identity differs")
        actual = {p.relative_to(root / "crates/scm-sdk").as_posix()
                  for directory in ("src", "examples")
                  for p in (root / "crates/scm-sdk" / directory).rglob("*") if p.is_file()}
        expected = {name for name in files if name.startswith(("src/", "examples/"))}
        if actual != expected:
            raise ValueError("Packaged source/example inventory differs from current filesystem")
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
        if phase not in (24, 28):
            verify_offline_delivery(proof, files, version)
        if proof["publicConsumerSourceSha256"] != sha((root / "contracts/native-probes/src/main.rs").read_bytes()):
            raise ValueError("External consumer differs from executed proof")
        for relative in ("Cargo.toml", "Cargo.lock", "contracts/native-probes/Cargo.toml", "contracts/native-probes/Cargo.lock"):
            if (root / relative).read_bytes() != git(root, "show", proof["releaseCommit"] + ":" + relative):
                raise ValueError("Qualification input changed: " + relative)
        if not rows:
            rows = [{"path": name, "sha256": sha(content), "bytes": len(content)}
                    for name, content in sorted(files.items())
                    if name.startswith(("src/", "examples/")) or name in {"README.md", "LICENSE", "NOTICE", "Cargo.toml.orig"}]
        proofs.append(proof)
    if any(proofs[0][key] != proofs[1][key] for key in ("releaseCommit", "files", "vcs")):
        raise ValueError("Compiler proofs do not qualify identical source files")
    return {"schemaVersion": 1, "passed": True, "version": version,
            "releaseCommit": proofs[0]["releaseCommit"], "protectedFiles": rows,
            "readmeSha256": sha((root / "README.md").read_bytes()),
            "archiveSha256": proofs[1]["archive_sha256"], "toolchains": [p["toolchain"] for p in proofs]}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--output", type=pathlib.Path, required=True)
    parser.add_argument("--phase", type=int, default=17)
    parser.add_argument("--version", default="0.2.1")
    args = parser.parse_args()
    result = verify(ROOT, args.phase, args.version)
    args.output.parent.mkdir(parents=True, exist_ok=True)
    args.output.write_text(json.dumps(result, indent=2) + "\n")
    print(json.dumps({k: v for k, v in result.items() if k != "protectedFiles"}))


if __name__ == "__main__":
    try:
        main()
    except (ValueError, OSError, KeyError, subprocess.SubprocessError) as error:
        sys.exit("Release candidate verification failed: " + str(error))
