#!/usr/bin/env python3
"""Run the active phase checks and preserve actual, credential-free evidence."""

import argparse
import datetime
import json
import os
import pathlib
import signal
import subprocess
import sys
import time

from review_phase import fingerprint

ROOT = pathlib.Path(__file__).resolve().parents[1]
# The longest Rust checker runs six subprocesses with 900-second limits.
# Preserve an outer bound while allowing their full budget and cleanup overhead.
COMMAND_TIMEOUT_SECONDS = 6 * 900 + 60


def run_check(command):
    """Retain output and terminate the checker's process group on timeout."""
    process = subprocess.Popen(command, cwd=ROOT, text=True, stdout=subprocess.PIPE,
                               stderr=subprocess.PIPE, start_new_session=os.name == "posix")
    timed_out = False

    def stop():
        try:
            if os.name == "posix":
                os.killpg(process.pid, signal.SIGKILL)
            else:
                process.kill()
        except ProcessLookupError:
            pass

    try:
        try:
            stdout, stderr = process.communicate(timeout=COMMAND_TIMEOUT_SECONDS)
        except subprocess.TimeoutExpired:
            timed_out = True
            stop()
            stdout, stderr = process.communicate()
    finally:
        if process.poll() is None:
            stop()
            process.wait()
        process.stdout.close()
        process.stderr.close()
    return {"exit_code": None if timed_out else process.returncode,
            "timeout": timed_out, "stdout": stdout, "stderr": stderr}


def commands_for(phase):
    if phase == 9:
        # Planning/governance only: no SDK implementation before plan review.
        return [["git", "diff", "--check"], ["python", "tools/verify-contracts.py"],
                *[["python", "-m", "unittest", "discover", "-s", "tools", "-p", name, "-v"]
                  for name in ("test_vendor_red_team_plan.py", "test_review_phase.py",
                               "test_ci.py", "test_auxiliary_evidence.py")]]
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
    if phase == 5:
        previous = [[value.replace("phase-04-rust-", "phase-05-rust-") for value in command]
                    for command in commands_for(4)]
        for command in previous:
            if command[:3] == ["python", "tools/native_operations.py", "--check"]:
                command[-1] = "5"
        return previous + [
            ["python", "tools/verify_native_fixtures.py"],
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_native_fixtures.py", "-v"],
        ] + [
            ["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", feature,
             "--output", f"docs/checks/phase-05-rust-1.99.0-{feature}.json"]
            for feature in ("model-security", "red-team")
        ]
    if phase == 6:
        previous = [[value.replace("phase-05-rust-", "phase-06-rust-") for value in command]
                    for command in commands_for(5)]
        for command in previous:
            if command[:3] == ["python", "tools/native_operations.py", "--check"]:
                command[-1] = "6"
        return previous + [
            ["python", "tools/verify_native_fixtures.py", "--require-test-bindings"],
            ["tools/node_modules/.bin/tsx", "tools/verify-gateway-specimens.mts", "--check"],
            ["tools/node_modules/.bin/tsc", "--noEmit", "--target", "ES2022", "--module", "NodeNext",
             "--moduleResolution", "NodeNext", "--strict", "--skipLibCheck", "--esModuleInterop",
             "--types", "node", "--typeRoots", ".local/typescript/node_modules/@types",
             "tools/verify-gateway-specimens.mts"],
        ]
    if phase == 7:
        previous = [[value.replace("phase-06-rust-", "phase-07-rust-").replace("1.88.0", "1.89.0")
                     for value in command] for command in commands_for(6)]
        for command in previous:
            if command[:3] == ["python", "tools/native_operations.py", "--check"]:
                command[-1] = "7"
        return previous + [
            ["python", "tools/generate_constants.py", "--check"],
            ["tools/node_modules/.bin/tsx", "tools/helper-oracle.mts", "--check"],
            ["tools/node_modules/.bin/tsc", "--noEmit", "--target", "ES2022", "--module", "NodeNext",
             "--moduleResolution", "NodeNext", "--strict", "--skipLibCheck", "--esModuleInterop",
             "--types", "node", "--typeRoots", ".local/typescript/node_modules/@types",
             "tools/helper-oracle.mts"],
            ["python", "tools/verify_gateway_bindings.py", "--check"],
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_verify_gateway_bindings.py", "-v"],
            ["python", "tools/native_exports.py", "--check"],
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_native_exports.py", "-v"],
            *[["python", "scripts/check-native-exports.py", "--toolchain", toolchain,
               "--output", f"docs/checks/phase-07-native-exports-{toolchain}.json"]
              for toolchain in ("1.89.0", "1.99.0")],
            *[["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", feature,
               "--output", f"docs/checks/phase-07-rust-1.99.0-{feature}.json"]
              for feature in ("agentguard", "realtime", "ws-transport")],
        ]
    if phase == 8:
        previous = [[value.replace("phase-07", "phase-08") for value in command]
                    for command in commands_for(7)]
        for command in previous:
            if command[:3] == ["python", "tools/native_operations.py", "--check"]:
                command[-1] = "8"
        return previous + [
            ["python", "tools/model_fixtures.py", "--check"],
            *[["python", "-m", "unittest", "discover", "-s", "tools", "-p", pattern, "-v"]
              for pattern in ("test_model_fixtures.py", "test_package_policy.py", "test_dependency_policy.py", "test_ci.py", "test_live_evidence.py", "test_qualification.py", "test_auxiliary_evidence.py")],
            ["python", "scripts/check-private-material.py", "--check", "docs/checks/phase-08-private-material.json"],
            ["python", "tools/live_evidence.py", "--check"],
            ["python", "tools/qualification.py", "--check"],
            ["python", "scripts/check-dependencies.py", "--output", "docs/checks/phase-08-dependencies.json"],
            *[["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", feature,
               "--output", f"docs/checks/phase-08-rust-1.99.0-{feature}.json"]
              for feature in ("reqwest-transport", "tracing", "default-tracing", "realtime-tracing", "ws-tracing", "gateway-json-unification", "native-probes", "probe-base")],
            *[["python", "scripts/check-rust.py", "--toolchain", "1.89.0", "--feature-set", feature,
               "--output", f"docs/checks/phase-08-rust-1.89.0-{feature}.json"]
              for feature in ("tracing", "native-probes")],
            *[["python", "scripts/check-package.py", "--toolchain", toolchain,
               "--output", f"docs/checks/phase-08-package-{toolchain}.json"]
              for toolchain in ("1.89.0", "1.99.0")],
            *[["python", "scripts/check-cross.py", "--target", target,
               "--output", f"docs/checks/phase-08-cross-{target}.json"]
              for target in ("x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu", "aarch64-apple-darwin", "x86_64-pc-windows-msvc")],
            ["python", "scripts/check-tracing-stress.py", "--toolchain", "1.99.0",
             "--processes", "100", "--test-threads", "16", "--expected-tests", "201",
             "--output", "docs/checks/phase-08-tracing-stress-1.99.0.json", "--replace-report"],
        ]
    raise ValueError("Check commands must be defined for the active implementation phase")


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--phase", required=True, type=int)
    parser.add_argument("--verification-only", action="store_true",
                        help="Rerun qualification in CI without changing phase status or requesting review")
    args = parser.parse_args()
    status = json.loads((ROOT / "docs/phase-status.json").read_text())
    if status["active_phase"] != args.phase and not args.verification_only:
        raise ValueError("Run the active phase's checks")
    before = fingerprint(ROOT)
    commands = commands_for(args.phase)
    records = []
    for command in commands:
        started = time.monotonic()
        invocation = [sys.executable, *command[1:]] if command[0] == "python" else command
        result = run_check(invocation)
        records.append({"command": command, "invocation": invocation, **result,
                        "duration_seconds": round(time.monotonic() - started, 3)})
        print(json.dumps({"command": command, "exit_code": result["exit_code"],
                          "timeout": result["timeout"]}), flush=True)
        if result["exit_code"] != 0:
            # These fixed verification commands are credential-free. Keep the
            # full streams in the report and bounded failure tails in CI logs.
            print(json.dumps({"failed_command": command, "exit_code": result["exit_code"],
                              "timeout": result["timeout"],
                              "stdout_tail": result["stdout"][-16384:],
                              "stderr_tail": result["stderr"][-16384:]}), file=sys.stderr, flush=True)
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
