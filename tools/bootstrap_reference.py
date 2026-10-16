#!/usr/bin/env python3
"""Restore development-only frozen sources and pinned extraction dependencies."""

import argparse
import io
import json
import pathlib
import subprocess
import tarfile
import tempfile

ROOT = pathlib.Path(__file__).resolve().parents[1]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--source-repository", type=pathlib.Path,
                        help="Existing source repository; otherwise fetch the manifest's public remote")
    args = parser.parse_args()
    manifest = json.loads((ROOT / "contracts/manifest.json").read_text())
    baseline = manifest["baseline"]
    destination = ROOT / ".local/typescript"
    if destination.exists():
        subprocess.run(["python", "tools/verify-contracts.py", "--baseline", str(destination)], cwd=ROOT, check=True)
    else:
        with tempfile.TemporaryDirectory(prefix="scm-sdk-reference-") as temporary:
            source = args.source_repository
            if source is None:
                source = pathlib.Path(temporary) / "upstream"
                subprocess.run(["git", "init", "-q", str(source)], check=True)
                subprocess.run(["git", "fetch", "--depth=1", baseline["repository"], baseline["commit"]],
                               cwd=source, check=True)
            version = subprocess.check_output(["git", "show", baseline["commit"] + ":package.json"], cwd=source)
            if json.loads(version)["version"] != baseline["version"]:
                raise ValueError("Unexpected frozen package version")
            archive = subprocess.check_output(
                ["git", "archive", baseline["commit"], "src", "test/openapi", "scripts/openapi",
                 "LICENSE", "NOTICE", "package.json", "tsconfig.json"], cwd=source)
            with tarfile.open(fileobj=io.BytesIO(archive)) as content:
                destination.mkdir(parents=True)
                content.extractall(destination, filter="data")
        subprocess.run(["python", "tools/verify-contracts.py", "--baseline", str(destination)], cwd=ROOT, check=True)
    subprocess.run(["npm", "ci", "--ignore-scripts", "--no-audit", "--no-fund"], cwd=ROOT / "tools", check=True)
    modules = destination / "node_modules"
    pinned_modules = ROOT / "tools/node_modules"
    if modules.is_symlink():
        modules.unlink()
    elif modules.exists():
        raise ValueError("Reference node_modules must be an owned symlink; refusing to replace a directory")
    modules.symlink_to(pinned_modules, target_is_directory=True)
    print("Frozen source and pinned development tooling ready; consumers use packaged Rust sources.")


if __name__ == "__main__":
    main()
