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
import unittest

from review_phase import fingerprint

ROOT = pathlib.Path(__file__).resolve().parents[1]
# The longest Rust checker runs six subprocesses with 900-second limits.
# Preserve an outer bound while allowing their full budget and cleanup overhead.
COMMAND_TIMEOUT_SECONDS = 6 * 900 + 60


def require_nonempty_tests(command):
    """unittest discovery otherwise reports success for missing or empty suites."""
    if command[1:4] != ["-m", "unittest", "discover"]:
        return
    start = command[command.index("-s") + 1] if "-s" in command else "."
    pattern = command[command.index("-p") + 1] if "-p" in command else "test*.py"
    suite = unittest.TestLoader().discover(str(ROOT / start), pattern=pattern)
    if suite.countTestCases() == 0:
        raise ValueError("Refusing an empty unittest discovery suite: " + pattern)


def run_check(command):
    """Retain output and terminate the checker's process group on timeout."""
    try:
        require_nonempty_tests(command)
    except (ValueError, ImportError) as error:
        return {"exit_code": 1, "timeout": False, "stdout": "", "stderr": str(error)}
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
    if phase == 11:
        return [
            ["git", "diff", "--check"],
            ["git", "diff", "--cached", "--check"],
            ["node", "tools/red-team-dataplane.mjs", "--check"],
            ["node", "tools/make-red-team-fixtures.mjs", "--check"],
            ["node", "tools/make-red-team-wire-fixtures.mjs", "--check"],
            ["python", "tools/red_team_dataplane.py", "--check", "--require-complete"],
            *[["python", "-m", "unittest", "discover", "-s", "tools", "-p", name, "-v"]
              for name in ("test_vendor_red_team_plan.py", "test_red_team_dataplane.py", "test_review_phase.py")],
            ["python", "tools/verify-contracts.py"],
            ["python", "tools/generate_validation.py", "--check"],
            ["python", "tools/generate_models.py", "--check"],
            ["python", "tools/response_defaults.py", "--check"],
            ["python", "tools/verify_gateway_bindings.py", "--check"],
            ["python", "tools/native_operations.py", "--check", "--require-through-phase", "8"],
            ["python", "tools/native_exports.py", "--check"],
            ["python", "tools/live_evidence.py", "--check"],
            ["python", "tools/qualification.py", "--check"],
            *[["python", "scripts/check-rust.py", "--toolchain", toolchain, "--feature-set", "red-team",
               "--output", f"docs/checks/phase-11-rust-{toolchain}-red-team.json"]
              for toolchain in ("1.89.0", "1.99.0")],
            ["python", "scripts/check-private-material.py", "--check", "docs/checks/phase-11-private-material.json"],
        ]
    if phase == 10:
        return [
            ["git", "diff", "--check"],
            ["git", "diff", "--cached", "--check"],
            ["node", "tools/red-team-dataplane.mjs", "--check"],
            ["node", "tools/make-red-team-fixtures.mjs", "--check"],
            ["python", "tools/red_team_dataplane.py", "--check"],
            *[["python", "-m", "unittest", "discover", "-s", "tools", "-p", name, "-v"]
              for name in ("test_vendor_red_team_plan.py", "test_red_team_dataplane.py", "test_model_generation.py", "test_review_phase.py", "test_ci.py", "test_g10_release.py")],
            ["python", "tools/verify-contracts.py"],
            ["python", "tools/generate_validation.py", "--check"],
            ["python", "tools/generate_models.py", "--check"],
            ["python", "tools/response_defaults.py", "--check"],
            ["python", "tools/verify_gateway_bindings.py", "--check"],
            ["python", "tools/native_operations.py", "--check", "--require-through-phase", "8"],
            ["python", "tools/live_evidence.py", "--check"],
            ["python", "tools/qualification.py", "--check"],
            *[["python", "scripts/check-rust.py", "--toolchain", toolchain, "--feature-set", "red-team",
               "--output", f"docs/checks/phase-10-rust-{toolchain}-red-team.json"]
              for toolchain in ("1.89.0", "1.99.0")],
            ["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", "gateway",
             "--output", "docs/checks/phase-10-rust-1.99.0-gateway.json"],
            ["python", "tools/generate_models.py", "--verify-fixtures"],
            ["python", "scripts/check-native-exports.py", "--toolchain", "1.99.0",
             "--output", "docs/checks/phase-10-native-exports-1.99.0.json"],
            ["python", "scripts/check-private-material.py", "--check", "docs/checks/phase-10-private-material.json"],
        ]
    if phase == 9:
        # Planning/governance only: no SDK implementation before plan review.
        return [["git", "diff", "--check"], ["python", "tools/verify-contracts.py"],
                *[["python", "-m", "unittest", "discover", "-s", "tools", "-p", name, "-v"]
                  for name in ("test_vendor_red_team_plan.py", "test_review_phase.py",
                               "test_ci.py", "test_auxiliary_evidence.py")]]
    if phase == 92:
        return [
            ["git", "diff", "--check", "840c090..HEAD"],
            ["git", "diff", "--check"],
            ["git", "diff", "--cached", "--check"],
            *[["python", "-m", "unittest", "discover", "-s", "tools", "-p", pattern, "-v"]
              for pattern in ("test_review_phase.py", "test_g10_release.py", "test_ci.py",
                              "test_native_operations.py", "test_native_exports.py",
                              "test_live_evidence.py", "test_qualification.py",
                              "test_auxiliary_evidence.py", "test_package_policy.py",
                              "test_dependency_policy.py", "test_model_fixtures.py",
                              "test_model_generation.py", "test_native_fixtures.py",
                              "test_verify_gateway_bindings.py")],
            ["python", "tools/verify-contracts.py"],
            ["python", "tools/verify-contracts.py", "--baseline", ".local/typescript"],
            ["python", "tools/verify-contracts.py", "--self-test"],
            ["python", "tools/response_defaults.py", "--check"],
            ["python", "tools/generate_validation.py", "--check"],
            ["python", "tools/generate_models.py", "--check"],
            ["python", "tools/generate_constants.py", "--check"],
            ["python", "tools/verify_gateway_bindings.py", "--check"],
            ["python", "tools/model_fixtures.py", "--check"],
            ["python", "tools/verify_native_fixtures.py", "--require-test-bindings"],
            ["python", "tools/native_operations.py", "--check", "--require-through-phase", "8"],
            ["python", "tools/native_exports.py", "--check"],
            ["python", "tools/live_evidence.py", "--check"],
            ["python", "tools/qualification.py", "--check"],
            ["python", "scripts/check-dependencies.py", "--output", "docs/checks/phase-92-dependencies.json"],
            ["python", "scripts/check-published-payload.py", "--receipt", "docs/checks/release-registry-0.1.1.json",
             "--output", "docs/checks/phase-92-published-payload.json"],
            ["python", "scripts/check-private-material.py", "--check", "docs/checks/phase-92-private-material.json"],
        ]
    if phase == 90:
        return [
            ["git", "diff", "--check", "840c090..HEAD"],
            ["git", "diff", "--check"],
            ["git", "diff", "--cached", "--check"],
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_review_phase.py", "-v"],
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_ci.py", "-v"],
            ["python", "tools/verify-contracts.py"],
            ["python", "scripts/check-private-material.py", "--check", "docs/checks/phase-90-private-material.json"],
        ]
    if phase == 91:
        return [
            ["git", "diff", "--check"],
            ["git", "diff", "--cached", "--check"],
            *[["python", "-m", "unittest", "discover", "-s", "tools", "-p", pattern, "-v"]
              for pattern in ("test_review_phase.py", "test_g10_release.py", "test_native_operations.py",
                              "test_native_exports.py", "test_live_evidence.py", "test_qualification.py",
                              "test_ci.py", "test_package_policy.py", "test_dependency_policy.py",
                              "test_auxiliary_evidence.py", "test_model_fixtures.py", "test_model_generation.py",
                              "test_native_fixtures.py", "test_verify_gateway_bindings.py")],
            ["python", "tools/response_defaults.py", "--check"],
            ["python", "tools/verify-contracts.py"],
            ["python", "tools/verify-contracts.py", "--baseline", ".local/typescript"],
            ["python", "tools/verify-contracts.py", "--self-test"],
            ["python", "tools/generate_validation.py", "--check"],
            ["python", "tools/generate_models.py", "--check"],
            ["python", "tools/generate_constants.py", "--check"],
            ["python", "tools/verify_gateway_bindings.py", "--check"],
            ["python", "tools/model_fixtures.py", "--check"],
            [".local/typescript/node_modules/.bin/tsx", "tools/schema-oracle.mts", "--check"],
            ["python", "tools/generate_models.py", "--verify-fixtures"],
            ["python", "tools/verify_native_fixtures.py", "--require-test-bindings"],
            ["python", "tools/native_operations.py", "--check", "--require-through-phase", "8"],
            ["python", "tools/native_exports.py", "--check"],
            ["python", "tools/live_evidence.py", "--check"],
            ["python", "tools/qualification.py", "--check"],
            *[["python", "scripts/check-rust.py", "--toolchain", toolchain, "--feature-set", feature,
               "--output", f"docs/checks/phase-91-rust-{toolchain}-{feature}.json"]
              for toolchain, feature in (("1.89.0", "all"), ("1.99.0", "all"),
                                         ("1.99.0", "minimal"), ("1.99.0", "gateway-json-unification"),
                                         ("1.99.0", "default-tracing"))],
            *[["python", "scripts/check-native-exports.py", "--toolchain", toolchain,
               "--output", f"docs/checks/phase-91-native-exports-{toolchain}.json"]
              for toolchain in ("1.89.0", "1.99.0")],
            *[["python", "scripts/check-package.py", "--toolchain", toolchain,
               "--output", f"docs/checks/phase-91-package-{toolchain}.json"]
              for toolchain in ("1.89.0", "1.99.0")],
            ["python", "scripts/verify-publication.py", "--delta", "--version", "0.1.1",
             "--proof", "docs/checks/phase-91-package-1.99.0.json",
             "--output", "docs/checks/phase-91-package-delta.json"],
            ["python", "scripts/check-dependencies.py", "--output", "docs/checks/phase-91-dependencies.json"],
            *[["python", "scripts/check-cross.py", "--target", target,
               "--output", f"docs/checks/phase-91-cross-{target}.json"]
              for target in ("x86_64-unknown-linux-gnu", "aarch64-unknown-linux-gnu", "aarch64-apple-darwin", "x86_64-pc-windows-msvc")],
            ["python", "scripts/check-tracing-stress.py", "--toolchain", "1.99.0",
             "--processes", "100", "--test-threads", "16", "--expected-tests", "203",
             "--output", "docs/checks/phase-91-tracing-stress-1.99.0.json", "--replace-report"],
            ["python", "scripts/check-private-material.py", "--check", "docs/checks/phase-91-private-material.json"],
        ]
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
