#!/usr/bin/env python3
"""Build and verify the consumer archive without development-time Node tooling."""

import hashlib
import io
import json
import os
import pathlib
import subprocess
import sys
import tarfile
import tomllib

ROOT = pathlib.Path(__file__).resolve().parents[1]


def main():
    cargo = pathlib.Path.home() / ".cargo/bin/cargo"
    if os.name == "nt":
        cargo = cargo.with_suffix(".exe")
    env = os.environ.copy()
    for key in list(env):
        if key.startswith(("PANW_", "AIRS_", "SCM_")):
            env.pop(key)
    minimum = tomllib.loads((ROOT / "Cargo.toml").read_text())["workspace"]["package"]["rust-version"]
    toolchain = minimum + ".0" if minimum.count(".") == 1 else minimum
    command = [str(cargo), f"+{toolchain}", "package", "-p", "scm-sdk", "--locked", "--allow-dirty"]
    result = subprocess.run(command, cwd=ROOT, env=env, text=True, capture_output=True, timeout=600)
    if result.returncode:
        print(result.stdout, file=sys.stderr)
        print(result.stderr, file=sys.stderr)
        return result.returncode
    metadata = json.loads(subprocess.check_output(
        [str(cargo), f"+{toolchain}", "metadata", "--no-deps", "--format-version", "1"], cwd=ROOT, env=env
    ))
    package = next(value for value in metadata["packages"] if value["name"] == "scm-sdk")
    archive = pathlib.Path(metadata["target_directory"]) / "package" / f"scm-sdk-{package['version']}.crate"
    content = archive.read_bytes()
    prefix = f"scm-sdk-{package['version']}/"
    with tarfile.open(fileobj=io.BytesIO(content), mode="r:gz") as packed:
        files = {member.name.removeprefix(prefix): member for member in packed.getmembers()}
        required = {"Cargo.toml", "Cargo.lock", "README.md", "LICENSE", "NOTICE", "src/lib.rs"}
        if not required <= files.keys():
            raise ValueError("Consumer package is missing required source, metadata, or notices")
        for name, member in files.items():
            if not member.isfile() or name.startswith(("../", "/", ".local/", "contracts/", "tools/", "node_modules/")):
                raise ValueError("Unexpected member in consumer package")
            if name.endswith((".pem", ".key", ".env", ".ts", ".mts", ".js")):
                raise ValueError("Consumer package includes an unexpected development or credential file")
        for name in ("LICENSE", "NOTICE"):
            if packed.extractfile(files[name]).read() != (ROOT / name).read_bytes():
                raise ValueError("Packaged legal notice differs from repository notice")
    print(json.dumps({"passed": True, "command": command, "archive_sha256": hashlib.sha256(content).hexdigest(),
                      "archive_bytes": len(content), "files": sorted(files),
                      "toolchain": toolchain,
                      "verification": f"cargo package compiled the extracted consumer package with Rust {toolchain}"}))
    return 0


if __name__ == "__main__":
    sys.exit(main())
