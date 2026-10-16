#!/usr/bin/env python3
"""Compile and execute all frozen export probes as a separate offline consumer."""
import argparse
import datetime
import hashlib
import json
import os
import pathlib
import re
import shutil
import subprocess
import sys
import time

ROOT = pathlib.Path(__file__).resolve().parents[1]


def cargo_path(explicit=None):
    candidate = explicit or shutil.which("cargo")
    if candidate:
        return pathlib.Path(candidate).absolute()
    candidate = pathlib.Path.home() / ".cargo/bin" / ("cargo.exe" if os.name == "nt" else "cargo")
    if not candidate.is_file():
        raise ValueError("Install Rust with rustup or supply --cargo PATH")
    return candidate


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--toolchain", default="1.99.0")
    parser.add_argument("--cargo")
    parser.add_argument("--output", type=pathlib.Path)
    args = parser.parse_args()
    if not re.fullmatch(r"\d+\.\d+\.\d+", args.toolchain):
        parser.error("Use an exact stable toolchain release")
    env = os.environ.copy()
    for key in list(env):
        if key.startswith(("PANW_", "AIRS_", "SCM_")):
            env.pop(key)
    env["CARGO_TERM_COLOR"] = "never"
    cargo = cargo_path(args.cargo)
    rustc = cargo.with_name("rustc.exe" if os.name == "nt" else "rustc")
    version = subprocess.check_output([str(rustc), "+" + args.toolchain, "-vV"], cwd=ROOT, env=env, text=True)
    host = next(line.removeprefix("host: ") for line in version.splitlines() if line.startswith("host: "))
    if env.get("CARGO_BUILD_TARGET") and env["CARGO_BUILD_TARGET"] != host:
        raise ValueError("Native export checks require CARGO_BUILD_TARGET to match the actual toolchain host")
    proof = ROOT / "contracts/native-exports.json"
    before = hashlib.sha256(proof.read_bytes()).hexdigest()
    check = subprocess.run([sys.executable, "tools/native_exports.py", "--check"], cwd=ROOT, env=env,
                           capture_output=True, text=True, timeout=120)
    if check.returncode:
        raise ValueError("Export evidence is stale: " + check.stderr.strip())
    command = [str(cargo), "+" + args.toolchain, "run", "--manifest-path", "contracts/native-probes/Cargo.toml",
               "--locked", "--offline", "--target", host]
    started = time.monotonic()
    result = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, text=True, timeout=900)
    recheck = subprocess.run([sys.executable, "tools/native_exports.py", "--check"], cwd=ROOT, env=env,
                             capture_output=True, text=True, timeout=120)
    unchanged = before == hashlib.sha256(proof.read_bytes()).hexdigest() and recheck.returncode == 0
    report = {"schemaVersion": 1, "checkedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "passed": result.returncode == 0 and unchanged, "sourceUnchanged": unchanged,
        "nativeExportsSha256": before, "frozenCapabilities": 3820, "toolchain": args.toolchain,
        "rustcVersion": version, "hostTarget": host, "execution": "separate_public_consumer_compile_and_execution",
        "command": command, "exit_code": result.returncode, "stdout": result.stdout, "stderr": result.stderr,
        "durationSeconds": round(time.monotonic() - started, 3),
        "limitations": ["This proves the recorded native host/compiler public API probes and literal assertions, not successful live service workflows."]}
    output = args.output or ROOT / f".local/checks/native-export-consumer-{args.toolchain}.json"
    output.parent.mkdir(parents=True, exist_ok=True)
    output.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"passed": report["passed"], "hostTarget": host, "frozenCapabilities": 3820, "evidence": str(output)}))
    if not report["passed"]:
        print(result.stderr, file=sys.stderr)
        print(recheck.stderr, file=sys.stderr)
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, FileNotFoundError) as error:
        sys.exit("Native export consumer verification failed: " + str(error))
