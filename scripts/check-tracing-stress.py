#!/usr/bin/env python3
"""Build once and repeat the complete all-feature library suite, fail-fast."""
import argparse
import datetime
import hashlib
import json
import os
import pathlib
import re
import shutil
import sys
import time

from evidence_common import (atomic_replace_private, clean_environment, git_files, read_regular, run_bounded,
                             runtime_fingerprint, source_fingerprint, write_private)

ROOT = pathlib.Path(__file__).resolve().parents[1]
SUMMARY = re.compile(rb"^test result: ok\. (\d+) passed; (\d+) failed; (\d+) ignored; "
                     rb"(\d+) measured; (\d+) filtered out;", re.MULTILINE)


def write_receipt(root, output, report, replace=False):
    content = (json.dumps(report, indent=2) + "\n").encode()
    if replace:
        expected = root / "docs/checks" / ("phase-08-tracing-stress-" + report["compiler"] + ".json")
        if output != expected or ".." in output.parts:
            raise ValueError("Replacement is restricted to the exact versioned tracing stress report")
        atomic_replace_private(output, content)
    else:
        write_private(output, content)


def executable_from_messages(content):
    candidates = []
    for line in content.splitlines():
        message = json.loads(line)
        target = message.get("target", {})
        if (message.get("reason") == "compiler-artifact" and message.get("profile", {}).get("test")
                and target.get("name") == "scm_sdk" and "lib" in target.get("kind", [])
                and message.get("executable")):
            candidates.append(pathlib.Path(message["executable"]))
    if len(set(candidates)) != 1:
        raise ValueError("Expected exactly one library test executable in Cargo JSON")
    return candidates[0]


def repeat_suite(binary, root, environment, processes, test_threads, expected_tests,
                 timeout, total_timeout):
    records, start = [], time.monotonic()
    for index in range(processes):
        remaining = total_timeout - (time.monotonic() - start)
        if remaining <= 0:
            records.append({"run": index + 1, "exitCode": None, "failure": "total_timeout"})
            break
        result = run_bounded([str(binary), "--test-threads=" + str(test_threads)], root,
                             environment, min(timeout, remaining))
        matches = SUMMARY.findall(result["stdout"])
        counts = [int(value) for value in matches[0]] if len(matches) == 1 else None
        passed = (result["exitCode"] == 0 and result["failure"] is None
                  and counts == [expected_tests, 0, 0, 0, 0])
        records.append({"run": index + 1, "exitCode": result["exitCode"],
                        "failure": result["failure"], "passed": passed,
                        "passedTests": counts[0] if counts else None,
                        "durationSeconds": result["durationSeconds"]})
        if not passed:
            break
    return records


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--toolchain", required=True)
    parser.add_argument("--processes", type=int, default=100)
    parser.add_argument("--test-threads", type=int, default=16)
    parser.add_argument("--expected-tests", type=int, default=201)
    parser.add_argument("--timeout", type=int, default=60, help="Seconds per test process (1–300)")
    parser.add_argument("--build-timeout", type=int, default=900)
    parser.add_argument("--total-timeout", type=int, default=900, help="Total test-process budget")
    parser.add_argument("--cargo", type=pathlib.Path, help="Explicit rustup Cargo proxy")
    parser.add_argument("--output", type=pathlib.Path, required=True, help="New receipt path")
    parser.add_argument("--replace-report", action="store_true",
                        help="Atomically replace only docs/checks/phase-08-tracing-stress-TOOLCHAIN.json")
    args = parser.parse_args()
    if (not re.fullmatch(r"\d+\.\d+\.\d+", args.toolchain)
            or not 1 <= args.processes <= 1000 or not 1 <= args.test_threads <= 64
            or not 1 <= args.expected_tests <= 100000 or not 1 <= args.timeout <= 300
            or not 1 <= args.build_timeout <= 3600 or not 1 <= args.total_timeout <= 7200):
        parser.error("Use an exact stable toolchain and bounded positive counts/timeouts")
    environment = clean_environment()
    cargo = args.cargo or shutil.which("cargo") or pathlib.Path.home() / ".cargo/bin/cargo"
    cargo = pathlib.Path(cargo).absolute()
    rustc = cargo.with_name("rustc.exe" if os.name == "nt" else "rustc")
    version_result = run_bounded([str(rustc), "+" + args.toolchain, "-vV"], ROOT,
                                 environment, 30)
    if version_result["exitCode"] or version_result["failure"]:
        raise ValueError("Cannot establish exact compiler identity")
    version = version_result["stdout"].decode("utf-8")
    release = re.search(r"^release: (.+)$", version, re.MULTILINE)
    host = re.search(r"^host: ([a-zA-Z0-9_-]+)$", version, re.MULTILINE)
    if release is None or release[1] != args.toolchain or host is None:
        raise ValueError("Compiler release or host does not match the requested stable toolchain")
    host = host[1]
    if environment.get("CARGO_BUILD_TARGET", host) != host:
        raise ValueError("Stress tests must execute on the exact compiler host")
    before, runtime_before = source_fingerprint(git_files(ROOT)), runtime_fingerprint(ROOT)
    build_command = [str(cargo), "+" + args.toolchain, "test", "-p", "scm-sdk", "--lib",
                     "--all-features", "--locked", "--offline", "--target", host,
                     "--no-run", "--message-format=json"]
    start = time.monotonic()
    build = run_bounded(build_command, ROOT, environment, args.build_timeout,
                        output_limit=64 * 1024 * 1024)
    report = {"schemaVersion": 2, "checkedAt": datetime.datetime.now(datetime.timezone.utc).isoformat(),
              "sourceSha256": before, "runtimeInputsSha256": runtime_before,
              "compiler": args.toolchain, "rustcVersion": version, "hostTarget": host,
              "buildCommand": build_command, "buildExitCode": build["exitCode"],
              "buildFailure": build["failure"], "processes": args.processes,
              "expectedTestsPerProcess": args.expected_tests, "testThreads": args.test_threads,
              "processTimeoutSeconds": args.timeout, "totalTestTimeoutSeconds": args.total_timeout,
              "runs": [], "binarySha256": None, "binaryUnchanged": False,
              "limitations": [
                  "Repeated native processes are bounded scheduler evidence, not proof of every interleaving.",
                  "Only synthetic fixtures and loopback transports execute; no service registration is loaded.",
                  "The receipt binds source/runtime/binary hashes; repository writers and the local toolchain remain trusted."]}
    if build["exitCode"] == 0 and build["failure"] is None:
        binary = executable_from_messages(build["stdout"])
        binary_before = hashlib.sha256(read_regular(binary, 512 * 1024 * 1024)).hexdigest()
        report["binarySha256"] = binary_before
        report["command"] = [str(binary), "--test-threads=" + str(args.test_threads)]
        report["runs"] = repeat_suite(binary, ROOT, environment, args.processes, args.test_threads,
                                       args.expected_tests, args.timeout, args.total_timeout)
        report["binaryUnchanged"] = binary_before == hashlib.sha256(
            read_regular(binary, 512 * 1024 * 1024)).hexdigest()
    report["sourceUnchanged"] = before == source_fingerprint(git_files(ROOT))
    report["runtimeUnchanged"] = runtime_before == runtime_fingerprint(ROOT)
    report["passed"] = (report["sourceUnchanged"] and report["runtimeUnchanged"]
                        and report["binaryUnchanged"] and len(report["runs"]) == args.processes
                        and all(row.get("passed") for row in report["runs"]))
    report["durationSeconds"] = round(time.monotonic() - start, 3)
    write_receipt(ROOT, args.output.absolute(), report, args.replace_report)
    print(json.dumps({"passed": report["passed"], "processesCompleted": len(report["runs"]),
                      "expectedTestsPerProcess": args.expected_tests, "hostTarget": host}))
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    try:
        sys.exit(main())
    except (ValueError, OSError, UnicodeError, KeyError, TypeError):
        sys.exit("Tracing stress verification failed; inputs, build or receipt could not be validated.")
