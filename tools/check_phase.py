#!/usr/bin/env python3
"""Run the active phase checks and preserve actual, credential-free evidence."""

import argparse
import datetime
import json
import pathlib
import subprocess
import sys
import time

from review_phase import fingerprint

ROOT = pathlib.Path(__file__).resolve().parents[1]


def commands_for(phase):
    if phase == 1:
        return [
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_review_phase.py", "-v"],
            ["python", "tools/verify-contracts.py"],
            ["python", "tools/verify-contracts.py", "--baseline", ".local/typescript"],
            ["python", "tools/verify-contracts.py", "--self-test"],
            [".local/typescript/node_modules/.bin/tsx", "tools/extract-contracts.mts", "--check"],
            [".local/typescript/node_modules/.bin/tsx", "tools/extract-operations.mts", "--check"],
            ["tools/node_modules/.bin/tsc", "--noEmit", "--target", "ES2022", "--module", "NodeNext",
             "--moduleResolution", "NodeNext", "--strict", "--skipLibCheck", "--esModuleInterop",
             "--types", "node", "--typeRoots", ".local/typescript/node_modules/@types",
             "tools/extract-contracts.mts", "tools/extract-operations.mts"],
        ]
    if phase == 2:
        return commands_for(1) + [
            ["python", "scripts/check-rust.py", "--toolchain", "1.88.0", "--feature-set", "default",
             "--output", "docs/checks/rust-1.88.0-default.json"],
            ["python", "scripts/check-rust.py", "--toolchain", "1.88.0", "--feature-set", "minimal",
             "--output", "docs/checks/rust-1.88.0-minimal.json"],
            ["python", "scripts/check-rust.py", "--toolchain", "1.88.0", "--feature-set", "all",
             "--output", "docs/checks/rust-1.88.0-all.json"],
            ["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", "default",
             "--output", "docs/checks/rust-1.99.0-default.json"],
            ["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", "minimal",
             "--output", "docs/checks/rust-1.99.0-minimal.json"],
            ["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", "all",
             "--output", "docs/checks/rust-1.99.0-all.json"],
            ["python", "scripts/check-package.py"],
        ]
    if phase == 3:
        native = [
            ["python", "scripts/check-rust.py", "--toolchain", toolchain, "--feature-set", features,
             "--output", f"docs/checks/phase-03-rust-{toolchain}-{features}.json"]
            for toolchain in ("1.88.0", "1.99.0") for features in ("default", "minimal", "all")
        ]
        return commands_for(1) + [
            ["python", "tools/generate_validation.py", "--check"],
            ["python", "tools/generate_models.py", "--check"],
            ["python", "tools/native_operations.py", "--check"],
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_native_operations.py", "-v"],
            [".local/typescript/node_modules/.bin/tsx", "tools/schema-oracle.mts", "--check"],
            ["tools/node_modules/.bin/tsc", "--noEmit", "--target", "ES2022", "--module", "NodeNext",
             "--moduleResolution", "NodeNext", "--strict", "--skipLibCheck", "--esModuleInterop",
             "--types", "node", "--typeRoots", ".local/typescript/node_modules/@types",
             "tools/schema-oracle.mts"],
            *native,
            ["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", "json-unification",
             "--output", "docs/checks/phase-03-rust-1.99.0-json-unification.json"],
            *[["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", feature,
               "--output", f"docs/checks/phase-03-rust-1.99.0-{feature}.json"]
              for feature in ("scan", "management", "gateway", "models")],
            ["python", "tools/generate_models.py", "--verify-fixtures"],
            ["python", "scripts/check-package.py"],
        ]
    if phase == 4:
        # Preserve historical phase-3 child reports; cumulative regression checks
        # bind their new executions to the active phase's separate report paths.
        previous = [[value.replace("phase-03-rust-", "phase-04-rust-") for value in command]
                    for command in commands_for(3)]
        for command in previous:
            if command == ["python", "tools/native_operations.py", "--check"]:
                command.extend(["--require-through-phase", "4"])
        return previous + [
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_model_generation.py", "-v"],
            *[["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", feature,
               "--output", f"docs/checks/phase-04-rust-1.99.0-{feature}.json"]
              for feature in ("dlp", "iam")],
        ]
    raise ValueError("Check commands must be defined for the active implementation phase")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--phase", required=True, type=int)
    args = parser.parse_args()
    status = json.loads((ROOT / "docs/phase-status.json").read_text())
    if status["active_phase"] != args.phase:
        raise ValueError("Run the active phase's checks")
    before = fingerprint(ROOT)
    commands = commands_for(args.phase)
    records = []
    for command in commands:
        started = time.monotonic()
        invocation = [sys.executable, *command[1:]] if command[0] == "python" else command
        result = subprocess.run(invocation, cwd=ROOT, text=True, capture_output=True, timeout=600)
        records.append({"command": command, "invocation": invocation, "exit_code": result.returncode,
                        "duration_seconds": round(time.monotonic() - started, 3),
                        "stdout": result.stdout, "stderr": result.stderr})
        print(json.dumps({"command": command, "exit_code": result.returncode}), flush=True)
        if result.returncode:
            break
    unchanged = before == fingerprint(ROOT)
    report = {"phase": args.phase, "source_sha256": before, "source_unchanged": unchanged,
              "checked_at": datetime.datetime.now(datetime.timezone.utc).isoformat(),
              "passed": unchanged and len(records) == len(commands)
                        and all(record["exit_code"] == 0 for record in records), "checks": records}
    path = ROOT / f"docs/checks/phase-{args.phase:02d}.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(report, indent=2) + "\n")
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    sys.exit(main())
