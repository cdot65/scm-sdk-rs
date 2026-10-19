#!/usr/bin/env python3
"""Run the active phase checks and preserve actual, credential-free evidence."""

import argparse
import datetime
import hashlib
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
# The longest Rust checker runs seven subprocesses with 900-second limits.
# Preserve an outer bound while allowing their full budget and cleanup overhead.
COMMAND_TIMEOUT_SECONDS = 7 * 900 + 60



# Every scoped contract JSON requires a declared verifying owner. Unknown artifacts
# fail closed, including new files discovered under a directory scope.
CONTRACT_LEDGER_OWNERS = {
    'contracts/compatibility/management-old-consumers.json': ('tools/management_consumers.py', '--check'),
    'contracts/native-fixtures/red-team-management.json': ('tools/management_fixtures.py', '--check'),
    'contracts/airs-active-sources.json': ('tools/red_team_management.py', '--check'),
    'contracts/airs-operations.json': ('tools/red_team_management.py', '--check'),
    'contracts/airs-retained-capabilities.json': ('tools/red_team_management.py', '--check'),
    'contracts/historical/0.2.2-live-evidence.json': ('tools/red_team_management.py', '--check'),
    'contracts/vendor/red-team-management/0.86.0/operations.json': ('tools/red_team_management.py', '--check'),
    'contracts/vendor/red-team-management/0.86.0/compatibility-decisions.json': ('tools/red_team_management.py', '--check'),
    'contracts/manifest.json': ('tools/verify-contracts.py',),
    'contracts/vendor/red-team-management/0.86.0/openapi.json': ('tools/verify-contracts.py',),
    'contracts/vendor/red-team-management/0.86.0/source.json': ('tools/verify-contracts.py',),
    'contracts/native-models.json': ('tools/generate_models.py', '--check'),
    'contracts/red-team-management-model-overlay.json': ('tools/generate_models.py', '--check'),
    'contracts/red-team-management-union-extensions.json': ('tools/generate_models.py', '--check'),
    'contracts/native-operations.json': ('tools/native_operations.py', '--check'),
    'contracts/native-exports.json': ('tools/native_exports.py', '--check'),
    'contracts/qualification.json': ('tools/qualification.py', '--check'),
    'contracts/response-defaults.json': ('tools/response_defaults.py', '--check'),
    'contracts/native-gateway-bindings.json': ('tools/verify_gateway_bindings.py', '--check'),
    'contracts/native-fixtures/gateway-specimens.json': ('tools/verify-gateway-specimens.mts', '--check'),
    'contracts/native-fixtures/schemas.json': ('tools/schema-oracle.mts', '--check'),
    'contracts/compatibility/g25/audit.json': ('tools/g25_audit.py', '--check'),
    'contracts/compatibility/g25/comparison.json': ('tools/g25_report.py', '--check'),
    'contracts/compatibility/g25/cases.json': ('tools/g25_fixtures.py', '--check'),
    'contracts/compatibility/g25/projection-bindings.json': ('tools/g25_fixtures.py', '--check'),
    'contracts/compatibility/g25/projections.json': ('tools/g25_fixtures.py', '--check'),
    'contracts/compatibility/g25/outcomes.json': ('tools/g25-oracle.mts', '--check'),
}


def verify_scoped_ledger_owners(scope, commands, root=ROOT):
    """Require an executed owning checker for every scoped contract JSON."""
    artifacts = set()
    for relative in scope:
        path = root / relative
        candidates = path.rglob('*.json') if path.is_dir() else (path,)
        for candidate in candidates:
            name = candidate.relative_to(root).as_posix()
            if name.startswith('contracts/') and name.endswith('.json'):
                artifacts.add(name)
    for artifact in sorted(artifacts):
        owner = CONTRACT_LEDGER_OWNERS.get(artifact)
        if owner is None:
            raise ValueError('No declared ledger owner: ' + artifact)
        if not any(any(tuple(command[i:i + len(owner)]) == owner
                       for i in range(len(command) - len(owner) + 1))
                   for command in commands):
            raise ValueError('Missing ledger owning check: ' + artifact)


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


MANAGEMENT_PHASES = (21, 22, 23, 24, 25)
CONTRACT_CI_PHASES = (12, 17, 20, *MANAGEMENT_PHASES)
MANAGEMENT_BASELINE = "0a213fe79c7f0c5758f05f53d04a5c577a4f3f4e"


PLANNING_FIXTURE = "crates/scm-sdk/src/fixtures/model-regressions.json"


def require_only_hash_replacements(original_bytes, current_bytes, replacements):
    expected = original_bytes
    for before, after in replacements:
        if not all(isinstance(value, str) and len(value) == 64
                   and all(c in "0123456789abcdef" for c in value) for value in (before, after)):
            raise ValueError("Provenance replacement must contain SHA256 values")
        before_bytes = ('"' + before + '"').encode()
        if expected.count(before_bytes) != 1:
            raise ValueError("Provenance hash replacement must identify exactly one baseline value")
        expected = expected.replace(before_bytes, ('"' + after + '"').encode(), 1)
    if current_bytes != expected:
        raise ValueError("Planning fixture bytes changed outside allowed provenance hashes")


def verify_planning_fixture(root=ROOT):
    """Permit only the generated fixture's source-corpus hash to be rebound."""
    path = root / PLANNING_FIXTURE
    if path.is_symlink():
        raise ValueError("Planning fixture must be a regular file")
    original_bytes = subprocess.check_output(
        ["git", "show", MANAGEMENT_BASELINE + ":" + PLANNING_FIXTURE], cwd=root)
    original = json.loads(original_bytes)
    current = json.loads(path.read_text())
    original_hash = original["provenance"].pop("sha256")
    source_hash = current["provenance"].pop("sha256")
    require_only_hash_replacements(original_bytes, path.read_bytes(), [(original_hash, source_hash)])
    if current != original:
        raise ValueError("Planning fixture changed outside its source-corpus hash")
    source = root / original["provenance"]["source"]
    if source.is_symlink() or hashlib.sha256(source.read_bytes()).hexdigest() != source_hash:
        raise ValueError("Planning fixture source-corpus hash is stale")
    corpus_bytes = subprocess.check_output(
        ["git", "show", MANAGEMENT_BASELINE + ":" + original["provenance"]["source"]], cwd=root)
    corpus_original = json.loads(corpus_bytes)
    corpus_current = json.loads(source.read_text())
    original_manifest_hash = corpus_original["provenance"].pop("manifestSha256")
    manifest_hash = corpus_current["provenance"].pop("manifestSha256")
    require_only_hash_replacements(corpus_bytes, source.read_bytes(), [(original_manifest_hash, manifest_hash)])
    if corpus_current != corpus_original:
        raise ValueError("Planning oracle corpus changed outside its manifest hash")
    if hashlib.sha256((root / "contracts/manifest.json").read_bytes()).hexdigest() != manifest_hash:
        raise ValueError("Planning oracle corpus manifest hash is stale")


def verify_planning_gateway_proof(root=ROOT):
    relative = "contracts/native-fixtures/gateway-specimens.json"
    original = subprocess.check_output(["git", "show", MANAGEMENT_BASELINE + ":" + relative], cwd=root)
    old = json.loads(original)
    changed_inputs = {"tools/verify-contracts.py", "contracts/manifest.json"}
    replacements = [(row["sha256"], hashlib.sha256((root / row["file"]).read_bytes()).hexdigest())
                    for row in old["inputs"] if row["file"] in changed_inputs]
    if len(replacements) != len(changed_inputs):
        raise ValueError("Gateway provenance input allowlist is incomplete")
    require_only_hash_replacements(original, (root / relative).read_bytes(), replacements)


def management_commands(phase):
    """New phase gates retain historical checks and fail closed on later scope."""
    unit = lambda name: ["python", "-m", "unittest", "discover", "-s", "tools", "-p", name, "-v"]
    vendor_scope = ["--", ".", ":(exclude)contracts/vendor/red-team-management/0.86.0/*.yaml"]
    common = [["git", "diff", "--check", *vendor_scope],
              ["git", "diff", "--cached", "--check", *vendor_scope],
              ["python", "tools/verify-contracts.py"],
              ["python", "tools/verify-contracts.py", "--baseline", ".local/typescript"],
              ["python", "tools/verify-contracts.py", "--self-test"],
              ["python", "tools/red_team_management.py", "--check", *(["--verify-retention-origin"] if phase == 21 else [])],
              *[unit(name) for name in ("test_red_team_management.py", "test_management_phases.py", "test_review_phase.py")],
              ["python", "scripts/check-private-material.py", "--check", f"docs/checks/phase-{phase}-private-material.json"]]
    if phase == 21:
        paths = ["crates/scm-sdk", "Cargo.toml", "Cargo.lock", "LICENSE", "NOTICE", "deny.toml",
                 "contracts/native-exports.json", "contracts/native-operations.json",
                 "contracts/response-defaults.json",
                 "contracts/vendor/portkey-access/a27feb9/operations.json",
                 "contracts/vendor/red-team-data-plane/0.93.0/operations.json",
                 "contracts/vendor/red-team-data-plane/0.93.0/model-additions.json"]
        return [
            ["git", "diff", "--exit-code", MANAGEMENT_BASELINE, "--", *paths,
             ":(exclude)" + PLANNING_FIXTURE],
            ["python", "-c", "import sys; sys.path.insert(0, 'tools'); from check_phase import verify_planning_fixture, verify_planning_gateway_proof; verify_planning_fixture(); verify_planning_gateway_proof()"],
            ["python", "-c", "import subprocess; assert not subprocess.check_output("
             + repr(["git", "ls-files", "--others", "--exclude-standard", "--", *paths])
             + "), 'Unexpected SDK implementation file before plan acceptance'"],
            ["git", "diff", "--check", MANAGEMENT_BASELINE + "..HEAD", "--", ".",
             ":(exclude)contracts/vendor/red-team-management/0.86.0/*.yaml",
             ":(exclude)contracts/vendor/red-team-data-plane/0.93.0/*.yaml",
             ":(exclude)contracts/vendor/portkey-access/a27feb9/*.yaml"],
            *common,
            ["python", "scripts/check-dependencies.py", "--output", "docs/checks/phase-21-dependencies.json"],
            unit("test_ci.py"),
            *[["python", "tools/" + tool, "--check"] for tool in
              ("model_fixtures.py", "generate_constants.py", "response_defaults.py")],
            ["python", "tools/verify_native_fixtures.py", "--require-test-bindings"],
            ["python", "tools/red_team_dataplane.py", "--check", "--require-complete"],
            [".local/typescript/node_modules/.bin/tsx", "tools/schema-oracle.mts", "--check"],
            ["tools/node_modules/.bin/tsx", "tools/verify-gateway-specimens.mts", "--check"],
            ["python", "tools/generate_validation.py", "--check"],
            ["python", "tools/generate_models.py", "--check"]]
    # The verifier must reject unimplemented later milestones. Listing a future
    # gate is not a claim that its fixtures, qualification or delivery exist.
    commands = [*common, ["python", "tools/red_team_management.py", "--check", "--require-through-phase", str(phase)]]
    commands += [
        ["tools/node_modules/.bin/tsx", "tools/verify-gateway-specimens.mts", "--check"],
        ["python", "tools/verify_gateway_bindings.py", "--check"],
    ]
    if phase == 22:
        commands += [
            ["git", "diff", "--check", MANAGEMENT_BASELINE + "..HEAD", "--", ".",
             ":(exclude)contracts/vendor/red-team-management/0.86.0/*.yaml",
             ":(exclude)contracts/vendor/red-team-data-plane/0.93.0/*.yaml",
             ":(exclude)contracts/vendor/portkey-access/a27feb9/*.yaml"],
            unit("test_red_team_management_models.py"),
            ["python", "tools/native_operations.py", "--check", "--require-through-phase", "8"],
            ["python", "tools/native_exports.py", "--check"],
            ["python", "tools/qualification.py", "--check"],
            ["python", "scripts/check-dependencies.py", "--output", "docs/checks/phase-22-dependencies.json"],
        ]
        inherited = commands_for(19)
    else:
        inherited = commands_for(20)
        if phase == 23:
            commands += [
                ["python", "tools/management_fixtures.py", "--check"],
                ["python", "tools/management_consumers.py", "--check"],
                unit("test_management_consumers.py"),
                unit("test_management_fixtures.py"),
                unit("test_red_team_management_models.py"),
            ]
    for original in inherited:
        # Historical release proofs are immutable and version-specific. New
        # publication readiness/delivery belongs to the phase 24 milestone gate.
        if "scripts/check-access-release.py" in original:
            continue
        if phase == 23 and "scripts/check-package.py" in original:
            continue  # The source-matched clean wrapper owns package execution.
        if phase == 23 and original == ["python", "tools/live_evidence.py", "--check"]:
            continue  # Cumulative offline qualification; fresh live acceptance belongs to phase 24.
        command = [value.replace("phase-19-", f"phase-{phase}-").replace("phase-20-", f"phase-{phase}-")
                   for value in original]
        if command[:2] == ["git", "diff"] and "--check" in command:
            if "--" not in command:
                command += ["--", "."]
            if vendor_scope[-1] not in command:
                command.append(vendor_scope[-1])
        if command not in commands:
            commands.append(command)
    if phase >= 23:
        commands.append(unit("test_management_package.py"))
        for compiler in ("1.89.0", "1.99.0"):
            commands.append(["python", "scripts/check-management-package.py" if phase == 23 else "scripts/check-package.py",
                             "--toolchain", compiler, "--output", f"docs/checks/phase-{phase}-package-{compiler}.json"])
    if phase == 23:
        commands.append(["python", "tools/red_team_management.py", "--check", "--require-through-phase", "23", "--verify-package-receipts"])
    return commands


def commands_for(phase):
    if phase in MANAGEMENT_PHASES:
        return management_commands(phase)
    if phase == 20:
        # Reuse complete prior qualification and G25 regressions without changing
        # their historical command lists or overwriting historical check reports.
        combined = commands_for(17) + commands_for(19)
        commands = []
        for original in combined:
            command = [value.replace("phase-17-", "phase-20-").replace("phase-19-", "phase-20-")
                       for value in original]
            if "scripts/check-access-release.py" in command:
                command += ["--phase", "20", "--version", "0.2.2"]
            if command not in commands:
                commands.append(command)
        commands.append(["python", "tools/generate_models.py", "--verify-fixtures"])
        commands += [["python", "scripts/check-native-exports.py", "--toolchain", compiler,
                      "--output", f"docs/checks/phase-20-native-exports-{compiler}.json"]
                     for compiler in ("1.89.0", "1.99.0")]
        return commands
    if phase == 19:
        return [["git", "diff", "--check"],
                ["python", "scripts/check-private-material.py", "--check", "docs/checks/phase-19-private-material.json"],
                *[["python", "tools/" + tool, "--check"] for tool in
                  ("g25_audit.py", "g25_fixtures.py", "g25_report.py", "generate_validation.py", "generate_models.py", "model_fixtures.py", "response_defaults.py")],
                ["python", "tools/red_team_dataplane.py", "--check", "--require-complete"],
                ["python", "tools/verify-contracts.py", "--baseline", ".local/typescript"],
                ["node", "tools/make-red-team-fixtures.mjs", "--check"],
                ["tools/node_modules/.bin/tsx", "tools/g25-oracle.mts", "--check"],
                [".local/typescript/node_modules/.bin/tsx", "tools/schema-oracle.mts", "--check"],
                ["tools/node_modules/.bin/tsc", "--noEmit", "--target", "ES2022", "--module", "NodeNext",
                 "--moduleResolution", "NodeNext", "--strict", "--skipLibCheck", "--esModuleInterop",
                 "--types", "node", "--typeRoots", ".local/typescript/node_modules/@types", "tools/g25-oracle.mts"],
                *[["python", "-m", "unittest", "discover", "-s", "tools", "-p", name, "-v"] for name in
                  ("test_g25*.py", "test_review_phase.py", "test_ci.py", "test_model_generation.py", "test_red_team_dataplane.py", "test_model_fixtures.py", "test_g10_release.py")],
                *[["python", "scripts/check-rust.py", "--toolchain", version, "--feature-set", "red-team",
                   "--output", f"docs/checks/phase-19-rust-{version}-red-team.json"] for version in ("1.89.0", "1.99.0")],
                ["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", "json-unification",
                 "--output", "docs/checks/phase-19-rust-1.99.0-json-unification.json"],
                *[["python", "scripts/check-rust.py", "--toolchain", version, "--feature-set", "red-team-json-unification",
                   "--output", f"docs/checks/phase-19-rust-{version}-red-team-json-unification.json"] for version in ("1.89.0", "1.99.0")]]
    if phase == 18:
        return [["git", "diff", "--check"],
                ["git", "diff", "--exit-code", "7f4a062", "--", "crates/scm-sdk", "Cargo.toml", "Cargo.lock", "LICENSE", "NOTICE"],
                ["python", "-c", "import subprocess; assert not subprocess.check_output(['git','ls-files','--others','--exclude-standard','--','crates/scm-sdk','Cargo.toml','Cargo.lock','LICENSE','NOTICE']), 'Unexpected SDK implementation file before plan acceptance'"],
                ["python", "tools/g25_reproduction.py"],
                ["python", "tools/g25_audit.py", "--check"],
                ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_g25.py", "-v"],
                ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_review_phase.py", "-v"]]
    if phase == 17:
        return [
            ["git", "diff", "--check", "b65f3c9..HEAD", "--", ".", ":(exclude)contracts/vendor/portkey-access/a27feb9/*.yaml"],
            *commands_for(15)[:3],
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_*.py", "-v"],
            *commands_for(12)[4:25],
            *[["python", "scripts/check-rust.py", "--toolchain", version, "--feature-set", feature,
               "--output", f"docs/checks/phase-17-rust-{version}-{feature}.json"]
              for version in ("1.89.0", "1.99.0") for feature in ("minimal", "default", "all")],
            *[["python", "scripts/check-rust.py", "--toolchain", "1.99.0", "--feature-set", feature,
               "--output", f"docs/checks/phase-17-rust-1.99.0-{feature}.json"]
              for feature in ("gateway", "gateway-json-unification", "default-tracing")],
            ["python", "scripts/check-access-release.py", "--output", "docs/checks/phase-17-candidate.json"],
            ["python", "scripts/check-dependencies.py", "--output", "docs/checks/phase-17-dependencies.json"],
            ["python", "scripts/check-private-material.py", "--check", "docs/checks/phase-17-private-material.json"],
        ]
    if phase == 16:
        return [*commands_for(15)[:4],
                *[["python", "-m", "unittest", "discover", "-s", "tools", "-p", name, "-v"]
                  for name in ("test_review_phase.py", "test_portkey_access.py", "test_ci.py")],
                *[["python", "scripts/check-rust.py", "--toolchain", version, "--feature-set", "gateway",
                   "--output", f"docs/checks/phase-16-rust-{version}-gateway.json"] for version in ("1.89.0", "1.99.0")]]
    if phase == 15:
        return [["git", "diff", "--check", "--", ".", ":(exclude)contracts/vendor/portkey-access/a27feb9/*.yaml"],
                ["git", "diff", "--cached", "--check", "--", ".", ":(exclude)contracts/vendor/portkey-access/a27feb9/*.yaml"],
                ["node", "tools/portkey-access.mjs", "--check"],
                ["python", "tools/verify-contracts.py"],
                ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_review_phase.py", "-v"],
                ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_portkey_access.py", "-v"]]
    if phase == 14:
        return [
            ["git", "diff", "--check"],
            ["git", "diff", "--cached", "--check"],
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_*.py", "-v"],
            *[c for c in commands_for(12)[4:25]],
            ["python", "scripts/check-release-candidate.py", "--output", "docs/checks/phase-14-candidate.json"],
            ["python", "scripts/check-dependencies.py", "--output", "docs/checks/phase-14-dependencies.json"],
            ["python", "scripts/check-private-material.py", "--check", "docs/checks/phase-14-private-material.json"],
        ]
    if phase == 13:
        return [["git", "diff", "--check"], ["python", "tools/verify-contracts.py"],
                *[["python", "-m", "unittest", "discover", "-s", "tools", "-p", name, "-v"]
                  for name in ("test_release_020.py", "test_review_phase.py", "test_ci.py")]]
    if phase == 12:
        return [
            ["git", "diff", "--check", "840c090..HEAD", "--", ".",
             ":(exclude)contracts/vendor/red-team-data-plane/0.93.0/dp-openapi.yaml"],
            ["git", "diff", "--check"],
            ["git", "diff", "--cached", "--check"],
            ["python", "-m", "unittest", "discover", "-s", "tools", "-p", "test_*.py", "-v"],
            ["node", "tools/red-team-dataplane.mjs", "--check"],
            ["node", "tools/make-red-team-fixtures.mjs", "--check"],
            ["node", "tools/make-red-team-wire-fixtures.mjs", "--check"],
            ["python", "tools/red_team_dataplane.py", "--check", "--require-complete"],
            ["python", "tools/verify-contracts.py"],
            ["python", "tools/verify-contracts.py", "--baseline", ".local/typescript"],
            ["python", "tools/verify-contracts.py", "--self-test"],
            ["python", "tools/generate_validation.py", "--check"],
            ["python", "tools/generate_models.py", "--check"],
            ["python", "tools/generate_constants.py", "--check"],
            ["python", "tools/response_defaults.py", "--check"],
            ["python", "tools/verify_gateway_bindings.py", "--check"],
            ["python", "tools/model_fixtures.py", "--check"],
            ["python", "tools/verify_native_fixtures.py", "--require-test-bindings"],
            ["python", "tools/native_operations.py", "--check", "--require-through-phase", "8"],
            ["python", "tools/native_exports.py", "--check"],
            ["python", "tools/live_evidence.py", "--check"],
            ["python", "tools/qualification.py", "--check"],
            [".local/typescript/node_modules/.bin/tsx", "tools/schema-oracle.mts", "--check"],
            ["tools/node_modules/.bin/tsx", "tools/helper-oracle.mts", "--check"],
            ["tools/node_modules/.bin/tsx", "tools/verify-gateway-specimens.mts", "--check"],
            *[["python", "scripts/check-rust.py", "--toolchain", toolchain, "--feature-set", features,
               "--output", f"docs/checks/phase-12-rust-{toolchain}-{features}.json"]
              for toolchain in ("1.89.0", "1.99.0") for features in ("default", "minimal", "all")],
            ["python", "scripts/check-rust.py", "--toolchain", "1.89.0", "--feature-set", "red-team",
             "--output", "docs/checks/phase-12-rust-1.89.0-red-team.json"],
            ["python", "tools/generate_models.py", "--verify-fixtures"],
            *[["python", "scripts/check-native-exports.py", "--toolchain", toolchain,
               "--output", f"docs/checks/phase-12-native-exports-{toolchain}.json"]
              for toolchain in ("1.89.0", "1.99.0")],
            *[["python", "scripts/check-package.py", "--toolchain", toolchain,
               "--output", f"docs/checks/phase-12-package-{toolchain}.json"]
              for toolchain in ("1.89.0", "1.99.0")],
            ["python", "scripts/check-dependencies.py", "--output", "docs/checks/phase-12-dependencies.json"],
            ["python", "scripts/check-private-material.py", "--check", "docs/checks/phase-12-private-material.json"],
        ]
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


def ci_commands_for(phase):
    """Fast current contracts complement the independent Rust matrix.

    This subset cannot qualify a phase review: it writes a separate report and
    review_phase.check_evidence still requires the complete commands_for list.
    """
    if phase not in CONTRACT_CI_PHASES:
        raise ValueError("The contracts-only scope is not defined for this phase")
    native_scripts = {"scripts/check-management-package.py", "scripts/check-rust.py", "scripts/check-native-exports.py", "scripts/check-package.py", "scripts/check-access-release.py"}
    return [command for command in commands_for(phase)
            if not native_scripts.intersection(command)
            and "--verify-package-receipts" not in command
            and command != ["python", "tools/generate_models.py", "--verify-fixtures"]]


def checkpoint_bindings(command, root=None):
    root = ROOT if root is None else root
    bindings = {}
    for argument in command:
        if argument.startswith('docs/checks/') and argument.endswith('.json'):
            path = root / argument
            if path.is_symlink() or not path.is_file():
                raise ValueError('Missing or nonregular checkpoint evidence: ' + argument)
            bindings[str(path)] = hashlib.sha256(path.read_bytes()).hexdigest()
            report = json.loads(path.read_text())
            if report.get('retainedArchive'):
                archive = pathlib.Path(report['retainedArchive'])
                if not archive.is_absolute():
                    archive = root / archive
                if archive.is_symlink() or not archive.is_file():
                    raise ValueError('Checkpoint archive is missing or nonregular')
                bindings[str(archive)] = hashlib.sha256(archive.read_bytes()).hexdigest()
    return bindings


def read_checkpoint(path, phase, source, commands):
    if not path.exists():
        return [], datetime.datetime.now(datetime.timezone.utc).isoformat()
    if path.is_symlink():
        raise ValueError('Checkpoint must be a regular file')
    data = json.loads(path.read_text())
    age = datetime.datetime.now(datetime.timezone.utc) - datetime.datetime.fromisoformat(data['started_at'])
    if (data.get('schema_version') != 1 or data.get('phase') != phase
            or data.get('source_sha256') != source or data.get('commands') != commands
            or not datetime.timedelta(0) <= age <= datetime.timedelta(hours=8)):
        raise ValueError('Checkpoint source, command plan or eight-hour freshness differs')
    records = data['checks']
    if (not isinstance(records, list) or len(records) > len(commands)
            or [row.get('command') for row in records] != commands[:len(records)]
            or any(row.get('exit_code') != 0 or row.get('timeout') is not False
                   or row.get('source_unchanged') is not True for row in records)):
        raise ValueError('Checkpoint is not an executed passing command prefix')
    for row in records:
        if row.get('checkpoint_bindings') != checkpoint_bindings(row['command']):
            raise ValueError('Checkpoint evidence changed')
    return records, data['started_at']


def write_checkpoint(path, phase, source, commands, records, started_at):
    path.parent.mkdir(parents=True, exist_ok=True)
    temporary = path.with_suffix('.tmp')
    temporary.write_text(json.dumps({'schema_version': 1, 'phase': phase,
        'source_sha256': source, 'commands': commands, 'checks': records,
        'started_at': started_at, 'complete': len(records) == len(commands)}, indent=2) + '\n')
    temporary.replace(path)


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--phase", required=True, type=int)
    parser.add_argument("--verification-only", action="store_true",
                        help="Rerun qualification in CI without changing phase status or requesting review")
    parser.add_argument("--contracts-only", action="store_true",
                        help="Selected phase CI contract subset; requires --verification-only and never qualifies a phase review")
    parser.add_argument('--resume', action='store_true',
                        help='Resume a source-bound passing prefix within eight hours (phases 23–25 only)')
    args = parser.parse_args()
    if args.resume and (args.phase not in (23, 24, 25) or args.contracts_only or args.verification_only):
        parser.error('--resume requires an active full phase 23–25 qualification')
    if args.contracts_only and (not args.verification_only or args.phase not in CONTRACT_CI_PHASES):
        parser.error("--contracts-only requires --phase 12 --verification-only (or another supported phase: 17, 20, 21–25)")
    status = json.loads((ROOT / "docs/phase-status.json").read_text())
    if status["active_phase"] != args.phase and not args.verification_only:
        raise ValueError("Run the active phase's checks")
    before = fingerprint(ROOT)
    commands = ci_commands_for(args.phase) if args.contracts_only else commands_for(args.phase)
    records = []
    checkpoint = ROOT / f'docs/checks/phase-{args.phase:02d}-partial.json'
    if args.resume:
        records, started_at = read_checkpoint(checkpoint, args.phase, before, commands)
        print(json.dumps({'resumed_checks': len(records), 'source_sha256': before}), flush=True)
    for command in commands[len(records):]:
        started = time.monotonic()
        invocation = [sys.executable, *command[1:]] if command[0] == "python" else command
        result = run_check(invocation)
        records.append({"command": command, "invocation": invocation, **result,
                        "duration_seconds": round(time.monotonic() - started, 3)})
        if args.resume and result['exit_code'] == 0:
            if fingerprint(ROOT) != before:
                raise ValueError('Source changed during resumable qualification')
            records[-1].update(source_unchanged=True,
                               executed_at=datetime.datetime.now(datetime.timezone.utc).isoformat(),
                               checkpoint_bindings=checkpoint_bindings(command))
            write_checkpoint(checkpoint, args.phase, before, commands, records, started_at)
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
    if args.contracts_only:
        report["scope"] = "ci_contracts_only_not_full_phase_qualification"
    suffix = "-contracts" if args.contracts_only else ""
    path = ROOT / f"docs/checks/phase-{args.phase:02d}{suffix}.json"
    path.parent.mkdir(parents=True, exist_ok=True)
    path.write_text(json.dumps(report, indent=2) + "\n")
    return 0 if report["passed"] else 1


if __name__ == "__main__":
    sys.exit(main())
