#!/usr/bin/env python3
"""Run reproducible, credential-free native Rust checks and record actual evidence."""

import argparse
import datetime
import hashlib
import json
import os
import pathlib
import platform
import re
import shutil
import subprocess
import sys
import time


ROOT = pathlib.Path(__file__).resolve().parents[1]
FEATURES = {"default": [], "minimal": ["--no-default-features"], "all": ["--all-features"],
            "json-unification": ["--no-default-features", "--features", "serde_json/arbitrary_precision"],
            **{name: ["--no-default-features", "--features", name] for name in ("scan", "management", "gateway", "models")}}


def rust_fingerprint():
    paths = subprocess.check_output(
        ["git", "ls-files", "--cached", "--others", "--exclude-standard", "-z"], cwd=ROOT
    ).decode().split("\0")
    digest = hashlib.sha256()
    for name in sorted(set(paths) - {""}):
        if not (name in {"Cargo.toml", "Cargo.lock", "rust-toolchain", "rust-toolchain.toml", "scripts/check-rust.py", "LICENSE", "NOTICE"}
                or name.startswith(("crates/", ".cargo/"))):
            continue
        path = ROOT / name
        if path.is_symlink() or not path.is_file():
            raise ValueError(f"Rust check inputs must be regular files: {name}")
        digest.update(name.encode() + b"\0" + hashlib.sha256(path.read_bytes()).digest())
    return digest.hexdigest()


def cargo_path(explicit):
    candidate = explicit or shutil.which("cargo")
    if candidate:
        return str(pathlib.Path(candidate).absolute())
    candidate = pathlib.Path.home() / ".cargo/bin" / ("cargo.exe" if os.name == "nt" else "cargo")
    if not candidate.is_file():
        raise ValueError("Install Rust with rustup or supply --cargo PATH")
    return str(candidate)


def commands(cargo, toolchain, feature_set, host):
    base = [cargo, f"+{toolchain}"]
    flags = FEATURES[feature_set]
    return [
        [*base, "fmt", "--all", "--", "--check"],
        [*base, "check", "--workspace", "--all-targets", "--locked", "--target", host, *flags],
        [*base, "clippy", "--workspace", "--all-targets", "--locked", "--target", host, *flags, "--", "-D", "warnings"],
        [*base, "test", "--workspace", "--locked", "--target", host, *flags],
        [*base, "doc", "--workspace", "--no-deps", "--locked", "--target", host, *flags],
    ]


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--toolchain", required=True, help="Exact Rust release, for example 1.88.0")
    parser.add_argument("--feature-set", choices=FEATURES, required=True)
    parser.add_argument("--cargo", help="Path to the rustup cargo proxy")
    parser.add_argument("--output", type=pathlib.Path, help="Evidence JSON; default .local/checks/rust-TOOLCHAIN-FEATURE.json")
    parser.add_argument("--timeout", type=int, default=900, help="Seconds allowed per command")
    args = parser.parse_args()
    if not re.fullmatch(r"\d+\.\d+\.\d+", args.toolchain) or args.timeout <= 0:
        parser.error("Use an exact stable release and a positive command timeout")
    before = rust_fingerprint()
    cargo = cargo_path(args.cargo)
    env = os.environ.copy()
    # Tests use injected synthetic credentials/local fixtures. Service registrations
    # and application credentials must not influence the ordinary offline suite.
    for key in list(env):
        if key.startswith(("PANW_", "AIRS_", "SCM_")):
            env.pop(key)
    env["CARGO_TERM_COLOR"] = "never"
    env["RUSTDOCFLAGS"] = "-D warnings"
    cargo_version = subprocess.check_output([cargo, f"+{args.toolchain}", "--version", "--verbose"], cwd=ROOT, env=env, text=True)
    rustc = pathlib.Path(cargo).with_name("rustc.exe" if os.name == "nt" else "rustc")
    rustc_version = subprocess.check_output([str(rustc), f"+{args.toolchain}", "--version", "--verbose"], cwd=ROOT, env=env, text=True)
    host = next(line.removeprefix("host: ") for line in rustc_version.splitlines() if line.startswith("host: "))
    # This command set executes tests on the toolchain host. Do not silently turn
    # an ambient Cargo cross target into purported native acceptance evidence.
    if env.get("CARGO_BUILD_TARGET") and env["CARGO_BUILD_TARGET"] != host:
        raise ValueError("Native checks require CARGO_BUILD_TARGET to match the toolchain host")
    records = []
    for command in commands(cargo, args.toolchain, args.feature_set, host):
        started = time.monotonic()
        try:
            result = subprocess.run(command, cwd=ROOT, env=env, capture_output=True, text=True, timeout=args.timeout)
            record = {"command": command, "exit_code": result.returncode, "stdout": result.stdout, "stderr": result.stderr}
        except subprocess.TimeoutExpired as error:
            def decoded(value):
                return value.decode(errors="replace") if isinstance(value, bytes) else value or ""
            record = {"command": command, "exit_code": None, "timeout": True, "stdout": decoded(error.stdout), "stderr": decoded(error.stderr)}
        record["duration_seconds"] = round(time.monotonic() - started, 3)
        records.append(record)
        print(json.dumps({"command": command, "exit_code": record["exit_code"]}), flush=True)
        if record["exit_code"] != 0:
            break
    unchanged = before == rust_fingerprint()
    notices_match = all((ROOT / name).read_bytes() == (ROOT / "crates/scm-sdk" / name).read_bytes() for name in ("LICENSE", "NOTICE"))
    passed = unchanged and notices_match and len(records) == 5 and all(record["exit_code"] == 0 for record in records)
    report = {
        "schemaVersion": 1, "passed": passed, "checkedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(),
        "rustInputsSha256": before, "sourceUnchanged": unchanged,
        "cargoLockSha256": hashlib.sha256((ROOT / "Cargo.lock").read_bytes()).hexdigest(),
        "toolchain": args.toolchain, "featureSet": args.feature_set, "hostTarget": host,
        "execution": "native_checks_and_tests", "os": platform.system(), "machine": platform.machine(),
        "rustcVersion": rustc_version, "cargoVersion": cargo_version,
        "packagedNoticesMatch": notices_match,
        "limitations": ["This report covers the recorded native host and feature set. It makes no claim about other targets or live service acceptance."],
        "checks": records,
    }
    destination = args.output or ROOT / f".local/checks/rust-{args.toolchain}-{args.feature_set}.json"
    destination.parent.mkdir(parents=True, exist_ok=True)
    destination.write_text(json.dumps(report, indent=2) + "\n")
    print(json.dumps({"passed": passed, "hostTarget": host, "featureSet": args.feature_set, "evidence": str(destination)}), flush=True)
    if not passed:
        for record in records:
            if record["exit_code"] != 0:
                print(record["stdout"], file=sys.stderr)
                print(record["stderr"], file=sys.stderr)
    return 0 if passed else 1


if __name__ == "__main__":
    sys.exit(main())
