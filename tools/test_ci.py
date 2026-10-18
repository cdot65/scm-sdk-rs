"""Prevent CI compiler, feature, pinned-action, and credential-policy drift."""

import importlib.util
import json
import pathlib
import re
import os
import shutil
import subprocess
import tempfile
import contextlib
import io
import sys
import time
from unittest import mock
import tomllib
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKFLOW = (ROOT / ".forgejo/workflows/ci.yml").read_text()
NATIVE_WORKFLOW = (ROOT / ".forgejo/workflows/native-platforms.yml").read_text()
WORKFLOW_SOURCES = {"ci.yml": WORKFLOW, "native-platforms.yml": NATIVE_WORKFLOW}
ALL_WORKFLOW_TEXT = "\n".join(WORKFLOW_SOURCES.values())
RUST_ACTION = "https://github.com/dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8"
UV_ACTION = "https://github.com/astral-sh/setup-uv@4959332f0f014c5280e7eac8b70c90cb574c9f9b"


def job_workflow(name):
    matches = [source for source in WORKFLOW_SOURCES.values()
               if re.search(rf"^  {name}:$", source, re.MULTILINE)]
    if len(matches) != 1:
        raise AssertionError(f"Expected one workflow owning {name}")
    return matches[0]


def flow_list(name):
    match = re.search(rf"^\s+{name}: (\[.*\])$", WORKFLOW, re.MULTILINE)
    if match is None:
        raise AssertionError(f"Missing machine-readable {name} matrix")
    return json.loads(match.group(1))


class CiContractTests(unittest.TestCase):
    def test_committed_range_gate_has_full_history_and_current_documentation(self):
        from check_phase import commands_for
        phase = int(re.search(r"tools/check_phase\.py --phase (\d+) --verification-only", WORKFLOW).group(1))
        job = re.search(r"^  contracts-and-package:\n(.*?)(?=^  [a-z][a-z-]*:\n|\Z)", WORKFLOW,
                        re.MULTILINE | re.DOTALL).group(1)
        if any(command[:3] == ["git", "diff", "--check"] and any(".." in word for word in command[3:])
               for command in commands_for(phase)):
            checkout = re.search(r"- name: Check out the reviewed source\n(.*?)(?=      - name:)",
                                 job, re.DOTALL).group(1)
            self.assertRegex(checkout, r"fetch-depth:\s*0(?:\s|$)")
        for name in ["docs/CI.md", "docs/RELEASE.md"]:
            self.assertIn(f"--phase {phase}", (ROOT / name).read_text())
        document = (ROOT / "docs/CI.md").read_text()
        section = document.split("## Scripts run by the current contracts gate\n", 1)[1].split("\n## ", 1)[0]
        documented = set(re.findall(r"`(scripts/check-[^`]+\.py)`", section))
        executed = {argument for command in commands_for(phase) for argument in command
                    if argument.startswith("scripts/check-") and argument.endswith(".py")}
        self.assertEqual(documented, executed)
        self.assertIn("scripts/check-dependencies.py", executed)

    def test_committed_range_fails_on_shallow_history_and_passes_after_full_fetch(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory).resolve()
            source, shallow = root / "origin", root / "checkout"
            source.mkdir()
            def git(*args, cwd=source, check=True):
                return subprocess.run(["git", *args], cwd=cwd, check=check, capture_output=True, text=True)
            git("-c", "init.defaultBranch=main", "init")
            def commit(message):
                git("add", "file.txt")
                git("-c", "user.name=SDK fixture", "-c", "user.email=fixture@example.invalid",
                    "-c", "commit.gpgsign=false", "-c", "core.hooksPath=" + str(root / "no-hooks"),
                    "commit", "-m", message)
            (source / "file.txt").write_text("baseline\n")
            commit("fixture baseline")
            base = git("rev-parse", "HEAD").stdout.strip()
            (source / "file.txt").write_text("changed\n")
            commit("fixture current")
            git("clone", "--depth", "1", source.as_uri(), str(shallow), cwd=root)
            self.assertNotEqual(git("diff", "--check", base + "..HEAD", cwd=shallow, check=False).returncode, 0)
            git("fetch", "--unshallow", cwd=shallow)
            self.assertEqual(git("diff", "--check", base + "..HEAD", cwd=shallow).returncode, 0)

    def test_workflows_share_reviewed_tool_versions_and_read_only_permissions(self):
        pins = json.loads((ROOT / "scripts/uv-checksums.json").read_text())
        for name, workflow in WORKFLOW_SOURCES.items():
            with self.subTest(workflow=name):
                version = re.search(r'^  CI_UV_VERSION: "([0-9.]+)"$', workflow, re.MULTILINE)
                self.assertIsNotNone(version)
                self.assertEqual(version.group(1), pins["version"])
                self.assertIn("permissions:\n  contents: read\n", workflow)

    def test_documentation_distinguishes_no_run_examples_and_native_gnu_evidence(self):
        for name in ("docs/PLATFORMS.md", "docs/RELEASE.md"):
            with self.subTest(document=name):
                self.assertIn("no-run doctests", (ROOT / name).read_text())
        self.assertIn("x86_64-unknown-linux-gnu", (ROOT / "CHANGELOG.md").read_text())

    def test_push_workflow_contains_only_required_registered_runner_jobs(self):
        jobs = WORKFLOW.split("\njobs:\n", 1)[1]
        names = set(re.findall(r"^  ([a-z][a-z-]*):$", jobs, re.MULTILINE))
        self.assertEqual(names, {"contracts-and-package", "rust"})
        self.assertNotIn("native_platforms", WORKFLOW)
        self.assertEqual(len(flow_list("toolchain")) * len(flow_list("features")) + 1, 47)

    def test_native_workflow_is_dispatch_only_without_a_job_admission_condition(self):
        path = ROOT / ".forgejo/workflows/native-platforms.yml"
        self.assertTrue(path.is_file())
        workflow = path.read_text()
        triggers = workflow.split("\non:\n", 1)[1].split("\npermissions:", 1)[0]
        self.assertEqual(set(re.findall(r"^  ([a-z_]+):", triggers, re.MULTILINE)),
                         {"workflow_dispatch"})
        jobs = workflow.split("\njobs:\n", 1)[1]
        self.assertNotRegex(jobs, re.compile(r"^    if:", re.MULTILINE),
                            "Native runner admission must follow the dispatch trigger")

    def test_qualification_prepares_complete_locked_graph_before_offline_checks(self):
        from check_phase import commands_for
        qualification = WORKFLOW.split("  contracts-and-package:", 1)[1].split("\n  rust:", 1)[0]
        lines = [line.strip() for line in qualification.splitlines()]
        fetch = "cargo +1.99.0 fetch --locked"
        ready = "cargo +1.99.0 metadata --all-features --locked --offline --format-version 1 > /dev/null"
        phase = "run: python3 tools/check_phase.py --phase 92 --verification-only"
        self.assertIn(fetch, lines)
        self.assertIn(ready, lines)
        self.assertLess(lines.index(fetch), lines.index(ready))
        self.assertLess(lines.index(ready), lines.index(phase))
        current = int(re.search(r"--phase (\d+)", phase).group(1))
        self.assertTrue(any("scripts/check-dependencies.py" in command for command in commands_for(current)))
        if not any("scripts/check-cross.py" in command for command in commands_for(current)):
            self.assertNotIn("rustup target add", qualification)

    def test_nested_cargo_build_outputs_do_not_change_source_or_consume_its_budget(self):
        from review_phase import fingerprint
        spec = importlib.util.spec_from_file_location("ci_build_evidence", ROOT / "scripts/evidence_common.py")
        common = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(common)
        with tempfile.TemporaryDirectory() as temporary:
            directory = pathlib.Path(temporary)
            repository = directory / "repository"
            repository.mkdir()
            subprocess.run(["git", "init", "-q", str(repository)], check=True)
            empty_ignore = directory / "empty-global-ignore"
            empty_ignore.touch()
            subprocess.run(["git", "-C", str(repository), "config", "core.excludesFile",
                            str(empty_ignore)], check=True)
            (repository / ".gitignore").write_text((ROOT / ".gitignore").read_text())
            (repository / "Cargo.toml").write_text("[workspace]\n")
            source = repository / "contracts/native-probes/src/main.rs"
            source.parent.mkdir(parents=True)
            source.write_text("fn main() {}\n")
            subprocess.run(["git", "-C", str(repository), "add", "--", ".gitignore",
                            "Cargo.toml", "contracts/native-probes/src/main.rs"], check=True)
            expected = fingerprint(repository)
            baseline = common.git_files(repository, source_only=True)
            budget = sum(map(len, baseline.values())) + 128
            artifacts = [repository / "target/debug/root.rmeta",
                         repository / "contracts/native-probes/target/debug/deps/consumer.rlib",
                         repository / "crates/future-client/target/debug/future-client"]
            for artifact in artifacts:
                artifact.parent.mkdir(parents=True, exist_ok=True)
                artifact.write_bytes(b"synthetic compiled output\n" * 512)
            self.assertEqual(fingerprint(repository), expected)
            with mock.patch.object(common, "MAX_TREE_BYTES", budget):
                self.assertEqual(common.git_files(repository, source_only=True), baseline)
                self.assertEqual(common.git_files(repository), baseline)
            self.assertTrue(all(path.is_file() for path in artifacts))

    def test_export_checker_owns_default_build_directory_and_preserves_caller_choice(self):
        spec = importlib.util.spec_from_file_location("ci_export_build_directory",
                                                      ROOT / "scripts/check-native-exports.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        with tempfile.TemporaryDirectory() as temporary:
            directory = pathlib.Path(temporary)
            repository = directory / "repository"
            (repository / "contracts").mkdir(parents=True)
            (repository / "contracts/native-exports.json").write_text("{}\n")
            cargo = directory / "tools/cargo"
            supplied = str(directory / "caller-selected-target")
            for index, configured in enumerate((None, "", supplied)):
                environment = {"PATH": os.defpath, "PANW_TEST_SECRET": "synthetic-not-forwarded"}
                if configured is not None:
                    environment["CARGO_TARGET_DIR"] = configured
                report = directory / f"report-{index}.json"
                completed = subprocess.CompletedProcess([], 0, stdout="synthetic check", stderr="")
                with self.subTest(configured=configured), \
                     mock.patch.object(module, "ROOT", repository), \
                     mock.patch.object(module, "cargo_path", return_value=cargo), \
                     mock.patch.object(module.subprocess, "check_output",
                                       return_value="rustc 1.99.0\nhost: x86_64-unknown-linux-gnu\n"), \
                     mock.patch.object(module.subprocess, "run", return_value=completed) as run, \
                     mock.patch.dict(os.environ, environment, clear=True), \
                     mock.patch.object(sys, "argv", ["check-native-exports.py", "--output", str(report)]), \
                     contextlib.redirect_stdout(io.StringIO()):
                    self.assertEqual(module.main(), 0)
                invocation = next(call for call in run.call_args_list if call.args[0][0] == str(cargo))
                expected = configured or str(repository / "target")
                self.assertEqual(invocation.kwargs["env"].get("CARGO_TARGET_DIR"), expected)
                self.assertNotIn("PANW_TEST_SECRET", invocation.kwargs["env"])
                self.assertEqual(json.loads(report.read_text())["cargoTargetDir"], expected)

    def test_phase_timeout_preserves_partial_output_and_a_failing_report(self):
        spec = importlib.util.spec_from_file_location("phase_ci_timeout", ROOT / "tools/check_phase.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        command = ["python", "-c", 'import sys,time; print("START", flush=True); '
            'print("ERROR", file=sys.stderr, flush=True); time.sleep(2)']
        with tempfile.TemporaryDirectory() as temporary:
            directory = pathlib.Path(temporary)
            (directory / "docs").mkdir()
            (directory / "docs/phase-status.json").write_text('{"active_phase": 8}')
            stderr = io.StringIO()
            with mock.patch.object(module, "ROOT", directory), \
                 mock.patch.object(module, "fingerprint", return_value="fixture-source"), \
                 mock.patch.object(module, "commands_for", return_value=[command]), \
                 mock.patch.object(module, "COMMAND_TIMEOUT_SECONDS", 1, create=True), \
                 mock.patch.object(sys, "argv", ["check_phase.py", "--phase", "8"]), \
                 contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(stderr):
                self.assertEqual(module.main(), 1)
            diagnostics = json.loads(stderr.getvalue())
            self.assertIsNone(diagnostics["exit_code"])
            self.assertTrue(diagnostics["timeout"])
            self.assertEqual(diagnostics["stdout_tail"], "START\n")
            self.assertEqual(diagnostics["stderr_tail"], "ERROR\n")
            report = json.loads((directory / "docs/checks/phase-08.json").read_text())
            self.assertFalse(report["passed"])
            self.assertTrue(report["checks"][0]["timeout"])
            self.assertIsNone(report["checks"][0]["exit_code"])
            self.assertEqual(report["checks"][0]["stdout"], "START\n")

    @unittest.skipUnless(os.name == "posix", "POSIX process-group cleanup")
    def test_phase_timeout_stops_a_child_in_the_checker_process_group(self):
        spec = importlib.util.spec_from_file_location("phase_ci_group", ROOT / "tools/check_phase.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        with tempfile.TemporaryDirectory() as temporary:
            directory = pathlib.Path(temporary)
            (directory / "docs").mkdir()
            (directory / "docs/phase-status.json").write_text('{"active_phase": 8}')
            marker = directory / "child-survived"
            child = f'import pathlib,time; time.sleep(1.5); pathlib.Path({str(marker)!r}).touch()'
            command = ["python", "-c", f'import subprocess,sys,time; '
                f'subprocess.Popen([sys.executable, "-c", {child!r}]); '
                'print("CHILD_STARTED", flush=True); time.sleep(2)']
            with mock.patch.object(module, "ROOT", directory), \
                 mock.patch.object(module, "fingerprint", return_value="fixture-source"), \
                 mock.patch.object(module, "commands_for", return_value=[command]), \
                 mock.patch.object(module, "COMMAND_TIMEOUT_SECONDS", 1, create=True), \
                 mock.patch.object(sys, "argv", ["check_phase.py", "--phase", "8"]), \
                 contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(io.StringIO()):
                self.assertEqual(module.main(), 1)
            time.sleep(1.6)
            self.assertFalse(marker.exists(), "Timed-out checker left its child running")

    def test_release_document_guards_run_when_the_development_version_changes(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            fixtures = {
                "docs/phase-status.json": json.dumps({"phases": [{"registry_publication": {
                    "status": "published_and_verified", "version": "0.1.1", "evidence": "not-needed.json",
                }}]}),
                "crates/scm-sdk/Cargo.toml": '[package]\nversion = "0.2.0"\n',
                "README.md": "Development documentation",
                "CHANGELOG.md": "See crates.io",
                "docs/STABILITY.md": "See crates.io",
                "docs/SECURITY.md": "See crates.io",
            }
            for name, content in fixtures.items():
                path = root / name
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text(content)
            def run_guard():
                result = unittest.TestResult()
                with mock.patch.object(sys.modules[__name__], "ROOT", root):
                    CiContractTests("test_verified_publication_is_consistent_in_current_release_documents").run(result)
                return result
            good = run_guard()
            self.assertTrue(good.wasSuccessful(), good.errors)
            self.assertFalse(good.skipped)
            (root / "README.md").write_text("The crate is currently unpublished")
            self.assertFalse(run_guard().wasSuccessful())
            (root / "README.md").write_text(fixtures["README.md"])
            (root / "docs/STABILITY.md").write_text("Registry reference missing")
            self.assertFalse(run_guard().wasSuccessful())

    def test_verified_publication_is_consistent_in_current_release_documents(self):
        status = json.loads((ROOT / "docs/phase-status.json").read_text())
        publications = [phase["registry_publication"] for phase in status["phases"]
                        if "registry_publication" in phase]
        publication = publications[-1] if publications else {}
        self.assertIn(publication.get("status"), {"verified_existing_publication", "published_and_verified"})
        files = [ROOT / "README.md", ROOT / "CHANGELOG.md", *sorted((ROOT / "docs").glob("*.md"))]
        obsolete = re.compile(r"currently unpublished|no registry publication has occurred|"
                              r"registry publication has not occurred|"
                              r"(?:crate|package)\s+is\s+not\s+published|"
                              r"current\s+unpublished workspace|0\.1\.0\s*[—-]\s*Unreleased", re.I)
        for path in files:
            with self.subTest(path=path.relative_to(ROOT)):
                self.assertIsNone(obsolete.search(path.read_text().replace("**", "")))
        for relative in ("CHANGELOG.md", "docs/STABILITY.md", "docs/SECURITY.md"):
            self.assertIn("crates.io", (ROOT / relative).read_text())
        workspace_version = tomllib.loads((ROOT / "crates/scm-sdk/Cargo.toml").read_text())["package"]["version"]
        if workspace_version != publication["version"]:
            return  # Only receipt-specific assertions depend on the development version.
        receipt = json.loads((ROOT / publication["evidence"]).read_text())
        self.assertEqual(publication["version"], receipt["version"])
        changelog = (ROOT / "CHANGELOG.md").read_text()
        for value in (receipt["archiveSha256"], receipt["sourceCommit"], receipt["publishedAt"].split("T")[0]):
            self.assertIn(value, changelog)

    def test_every_job_installs_pinned_node_before_selecting_uv(self):
        action = "https://github.com/actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020"
        version = (ROOT / ".node-version").read_text().strip()
        for job in ("contracts-and-package", "rust", "additional-native-platforms"):
            body = re.search(rf"^  {job}:\n(.*?)(?=^  [a-z][a-z-]*:\n|\Z)",
                             job_workflow(job), re.MULTILINE | re.DOTALL).group(1)
            with self.subTest(job=job):
                self.assertIn(action, body)
                self.assertIn(f'node-version: "{version}"', body)
                self.assertLess(body.index(action), body.index("run: node scripts/uv-integrity.cjs select"))

    def test_live_policy_has_one_unambiguous_filename_and_current_inbound_link(self):
        self.assertTrue((ROOT / "docs/LIVE_ACCEPTANCE_POLICY.md").is_file())
        self.assertFalse((ROOT / "docs/LIVE-ACCEPTANCE.md").exists())
        self.assertIn("(LIVE_ACCEPTANCE_POLICY.md)", (ROOT / "docs/LIVE_ACCEPTANCE.md").read_text())

    def test_failed_phase_check_logs_bounded_diagnostics_and_preserves_full_report(self):
        spec = importlib.util.spec_from_file_location("phase_ci_log", ROOT / "tools/check_phase.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        command = ["python", "-c", 'import sys; print("OUTPUT_START" + "o"*20000 + "OUTPUT_TAIL"); '
            'print("ERROR_START" + "e"*20000 + "ERROR_TAIL", file=sys.stderr); sys.exit(7)']
        with tempfile.TemporaryDirectory() as temporary:
            directory = pathlib.Path(temporary)
            (directory / "docs").mkdir()
            (directory / "docs/phase-status.json").write_text('{"active_phase": 8}')
            stderr = io.StringIO()
            with mock.patch.object(module, "ROOT", directory), \
                 mock.patch.object(module, "fingerprint", return_value="fixture-source"), \
                 mock.patch.object(module, "commands_for", return_value=[command]), \
                 mock.patch.object(sys, "argv", ["check_phase.py", "--phase", "8"]), \
                 contextlib.redirect_stdout(io.StringIO()), contextlib.redirect_stderr(stderr):
                self.assertEqual(module.main(), 1)
            diagnostics = json.loads(stderr.getvalue())
            self.assertEqual(diagnostics["failed_command"], command)
            self.assertEqual(diagnostics["exit_code"], 7)
            for field, tail in [("stdout_tail", "OUTPUT_TAIL\n"), ("stderr_tail", "ERROR_TAIL\n")]:
                self.assertLessEqual(len(diagnostics[field]), 16384)
                self.assertTrue(diagnostics[field].endswith(tail))
            report = json.loads((directory / "docs/checks/phase-08.json").read_text())
            self.assertFalse(report["passed"])
            self.assertTrue(report["checks"][0]["stdout"].startswith("OUTPUT_START"))
            self.assertTrue(report["checks"][0]["stderr"].startswith("ERROR_START"))

    def test_compilers_match_supported_msrv_and_current_pin(self):
        workspace = tomllib.loads((ROOT / "Cargo.toml").read_text())
        msrv = workspace["workspace"]["package"]["rust-version"]
        self.assertEqual(flow_list("toolchain"), [f"{msrv}.0", "1.99.0"])

    def test_every_native_feature_has_an_isolated_job(self):
        crate = tomllib.loads((ROOT / "crates/scm-sdk/Cargo.toml").read_text())
        matrix = flow_list("features")
        self.assertEqual(len(matrix), len(set(matrix)))
        self.assertTrue(set(crate["features"]) <= set(matrix))
        spec = importlib.util.spec_from_file_location("check_rust_ci", ROOT / "scripts/check-rust.py")
        module = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(module)
        self.assertEqual(set(matrix), set(module.FEATURES))
        exports = json.loads((ROOT / "contracts/native-exports.json").read_text())
        expected = sorted({feature for row in exports["capabilities"] for feature in row["requiredFeatures"]})
        flags = module.FEATURES["native-probes"]
        self.assertIn("--no-default-features", flags)
        self.assertEqual(sorted(flags[flags.index("--features") + 1].split(",")), expected)
        phase_spec = importlib.util.spec_from_file_location("phase_ci_budget", ROOT / "tools/check_phase.py")
        phase = importlib.util.module_from_spec(phase_spec)
        phase_spec.loader.exec_module(phase)
        maximum_children = len(module.commands("cargo", "1.99.0", "all", "native-host"))
        self.assertGreaterEqual(phase.COMMAND_TIMEOUT_SECONDS,
                                maximum_children * module.DEFAULT_COMMAND_TIMEOUT_SECONDS)

    def test_node_matches_owned_pin_and_phase_gate_verifies_only(self):
        node = (ROOT / ".node-version").read_text().strip()
        self.assertIn(f'node-version: "{node}"', WORKFLOW)
        self.assertIn("tools/bootstrap_reference.py", WORKFLOW)
        self.assertIn("tools/check_phase.py --phase 92 --verification-only", WORKFLOW)

    def test_actions_and_credentials_are_bounded(self):
        actions = re.findall(r"^\s+uses: (\S+)", ALL_WORKFLOW_TEXT, re.MULTILINE)
        self.assertEqual(len(actions), 13)
        self.assertEqual(set(actions), {
            "https://github.com/actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5",
            "https://github.com/actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020",
            UV_ACTION,
            RUST_ACTION,
        })
        for action in actions:
            self.assertRegex(action, r"^https://github\.com/(?:actions|dtolnay|astral-sh)/[a-z-]+@[0-9a-f]{40}$")
        self.assertEqual(ALL_WORKFLOW_TEXT.count("persist-credentials: false"), 3)
        self.assertIn("permissions:\n  contents: read\n", WORKFLOW)
        for forbidden in ["secrets.", "pull_request_target", "contents: write",
                          "id-token: write", "enable-openid-connect", "cargo publish",
                          "npm publish", "git push"]:
            self.assertNotIn(forbidden, ALL_WORKFLOW_TEXT)

    def test_every_job_bootstraps_rust_before_running_commands(self):
        jobs = {
            "contracts-and-package": ['"1.89.0"', '"1.99.0"'],
            "rust": ["${{ matrix.toolchain }}"],
            "additional-native-platforms": ['"1.99.0"'],
        }
        for job, toolchains in jobs.items():
            with self.subTest(job=job):
                match = re.search(rf"^  {job}:\n(.*?)(?=^  [a-z][a-z-]*:\n|\Z)",
                                  job_workflow(job), re.MULTILINE | re.DOTALL)
                self.assertIsNotNone(match)
                body = match.group(1)
                installations = re.findall(
                    rf"uses: {re.escape(RUST_ACTION)}(?: #[^\n]+)?\n        with:\n"
                    r"          toolchain: (.+)\n          components: rustfmt,clippy\n", body)
                self.assertEqual(installations, toolchains)
                # The runner image may have neither rustup nor Cargo on PATH.
                self.assertLess(body.rindex(RUST_ACTION), body.index("        run:"))
                self.assertIn("uses: " + UV_ACTION, body)
                self.assertIn('version: ${{ env.CI_UV_VERSION }}', body)
                self.assertIn('github-token: ""', body)
                self.assertIn('enable-cache: false', body)
                self.assertIn('UV_PYTHON_INSTALL_DIR: ${{ runner.temp }}/scm-sdk-python', body)
                self.assertIn('UV_PYTHON_BIN_DIR: ${{ runner.temp }}/scm-sdk-python-bin', body)
                selection = body.index("        run: node scripts/uv-integrity.cjs select")
                self.assertEqual(selection, body.index("        run:"))
                self.assertLess(selection, body.index(UV_ACTION))
                self.assertLess(body.index(UV_ACTION), body.index("        run: bash scripts/bootstrap-python.sh"))
                self.assertIn('checksum: ${{ steps.uv-integrity.outputs.checksum }}', body)
                self.assertIn('RUNNER_TOOL_CACHE: ${{ steps.uv-integrity.outputs.tool-cache }}', body)
                self.assertIn('SCM_SDK_UV_EXECUTABLE: ${{ steps.setup-uv.outputs.uv-path }}', body)

    def test_python_pin_supports_tomllib_and_filtered_tar_extraction(self):
        version = (ROOT / ".python-version").read_text().strip()
        self.assertRegex(version, r"^3\.[0-9]+\.[0-9]+$")
        self.assertGreaterEqual(tuple(map(int, version.split("."))), (3, 12, 0))

    def test_python_bootstrap_rejects_unverified_uv_before_executing_it(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = pathlib.Path(temporary)
            executable = directory / "uv"
            executable.write_text('#!/bin/sh\nprintf executed > "$SCM_SDK_TEST_MARKER"\nprintf "uv 0.13.1\\n"\n')
            executable.chmod(0o700)
            marker = directory / "executed"
            path_file = directory / "path"
            path_file.touch()
            environment = {**os.environ,
                "PATH": str(directory) + os.pathsep + os.environ["PATH"],
                "CI_UV_VERSION": "0.13.1",
                "SCM_SDK_UV_EXECUTABLE": str(executable),
                "SCM_SDK_TEST_MARKER": str(marker),
                "UV_PYTHON_INSTALL_DIR": str(directory / "python"),
                "UV_PYTHON_BIN_DIR": str(directory / "bin"),
                "GITHUB_PATH": str(path_file)}
            result = subprocess.run(["bash", "scripts/bootstrap-python.sh"], cwd=ROOT,
                env=environment, text=True, capture_output=True, timeout=10)
            self.assertNotEqual(result.returncode, 0)
            self.assertIn("installed uv executable checksum does not match", result.stderr)
            self.assertFalse(marker.exists(), "An unverified uv executable ran")
            self.assertEqual(path_file.read_text(), "")

    def test_uv_checksums_cover_every_configured_host_and_libc(self):
        pins = json.loads((ROOT / "scripts/uv-checksums.json").read_text())
        version = re.search(r'^  CI_UV_VERSION: "([0-9.]+)"$', WORKFLOW, re.MULTILINE).group(1)
        self.assertEqual(pins["version"], version)
        self.assertEqual(pins["schemaVersion"], 1)
        self.assertEqual(set(pins["targets"]), {
            "x86_64-unknown-linux-gnu", "x86_64-unknown-linux-musl",
            "aarch64-unknown-linux-gnu", "aarch64-unknown-linux-musl",
            "aarch64-apple-darwin", "x86_64-pc-windows-msvc"})
        for pin in pins["targets"].values():
            self.assertEqual(set(pin), {"archiveSha256", "binarySha256"})
            for digest in pin.values():
                self.assertRegex(digest, r"^[a-f0-9]{64}$")

    def test_uv_integrity_checks_reject_unsupported_hosts_stale_pins_and_changed_bytes(self):
        script = r'''
const assert = require("node:assert/strict");
const fs = require("node:fs");
const path = require("node:path");
const crypto = require("node:crypto");
const checks = require(path.join(process.argv[1], "scripts/uv-integrity.cjs"));
const pinfile = path.join(process.argv[1], "scripts/uv-checksums.json");
const pins = checks.loadPins(pinfile, "0.13.1");
for (const [platform, arch, musl, target] of [
  ["linux", "x64", false, "x86_64-unknown-linux-gnu"],
  ["linux", "x64", true, "x86_64-unknown-linux-musl"],
  ["linux", "arm64", false, "aarch64-unknown-linux-gnu"],
  ["linux", "arm64", true, "aarch64-unknown-linux-musl"],
  ["darwin", "arm64", false, "aarch64-apple-darwin"],
  ["win32", "x64", false, "x86_64-pc-windows-msvc"],
]) assert.equal(checks.resolveTarget(platform, arch, musl), target);
for (const host of [["freebsd", "x64"], ["darwin", "x64"], ["win32", "arm64"], ["linux", "ppc64"]])
  assert.throws(() => checks.resolveTarget(...host), /no reviewed checksum/);
assert.throws(() => checks.loadPins(pinfile, "0.13.2"), /not reviewed/);
const filename = path.join(process.argv[2], "uv fixture with spaces");
fs.writeFileSync(filename, "reviewed bytes");
const hash = crypto.createHash("sha256").update("reviewed bytes").digest("hex");
checks.verifyBinary(filename, hash);
fs.appendFileSync(filename, "changed");
assert.throws(() => checks.verifyBinary(filename, hash), /checksum does not match/);
delete pins.targets["aarch64-apple-darwin"];
const incomplete = path.join(process.argv[2], "incomplete.json");
fs.writeFileSync(incomplete, JSON.stringify(pins));
assert.throws(() => checks.loadPins(incomplete, "0.13.1"), /not reviewed/);
'''
        with tempfile.TemporaryDirectory() as temporary:
            result = subprocess.run([shutil.which("node"), "-e", script, str(ROOT), temporary],
                text=True, capture_output=True, timeout=10)
            self.assertEqual(result.returncode, 0, result.stderr)

    def test_uv_selection_creates_distinct_empty_tool_caches(self):
        with tempfile.TemporaryDirectory() as temporary:
            directory = pathlib.Path(temporary)
            output = directory / "output"
            environment = {**os.environ, "CI_UV_VERSION": "0.13.1",
                "RUNNER_TEMP": temporary, "GITHUB_OUTPUT": str(output)}
            for _ in range(2):
                result = subprocess.run([shutil.which("node"), "scripts/uv-integrity.cjs", "select"],
                    cwd=ROOT, env=environment, text=True, capture_output=True, timeout=10)
                self.assertEqual(result.returncode, 0, result.stderr)
            lines = output.read_text().splitlines()
            caches = [pathlib.Path(line.removeprefix("tool-cache=")) for line in lines
                      if line.startswith("tool-cache=")]
            self.assertEqual(len(caches), 2)
            self.assertNotEqual(caches[0], caches[1])
            for cache in caches:
                self.assertEqual(cache.parent, directory)
                self.assertEqual(list(cache.iterdir()), [])

    def test_ci_documentation_matches_action_pins_and_setup_effects(self):
        document = (ROOT / "docs/CI.md").read_text()
        actions = set(re.findall(r"^\s+uses: https://github\.com/(\S+)", ALL_WORKFLOW_TEXT, re.MULTILINE))
        documented = {f"{name}@{commit}" for name, commit in re.findall(
            r"^\| \[([^]]+)\]\(https://github\.com/[^)]+\) \| [^|]+ \| ([0-9a-f]{40}) \|$",
            document, re.MULTILINE)}
        self.assertEqual(documented, actions)
        self.assertEqual(ALL_WORKFLOW_TEXT.count(RUST_ACTION + " # reviewed untagged master commit"), 4)
        self.assertIn((ROOT / ".python-version").read_text().strip(), document)
        self.assertIn("Rust and Python need not be preinstalled", document)
        self.assertIn("CI actions set the runner's default Rust toolchain", document)
        self.assertIn("untagged master commit", document)
        self.assertNotIn("does not change the runner's default toolchain", document)

    def test_portable_python_bootstrap_uses_pinned_job_local_tools(self):
        version = re.search(r'^  CI_UV_VERSION: "([0-9]+\.[0-9]+\.[0-9]+)"$', WORKFLOW, re.MULTILINE)
        self.assertIsNotNone(version)
        self.assertIn(version.group(1), (ROOT / 'docs/CI.md').read_text())
        script = (ROOT / 'scripts/bootstrap-python.sh').read_text()
        for required in ('set -euo pipefail', '${CI_UV_VERSION:?', '${UV_PYTHON_INSTALL_DIR:?',
                         '${UV_PYTHON_BIN_DIR:?', '${GITHUB_PATH:?', '.python-version',
                         '"$SCM_SDK_UV_EXECUTABLE" --no-config --no-cache python install --default --no-registry',
                         'node "$scm_sdk_root/scripts/uv-integrity.cjs" verify "$SCM_SDK_UV_EXECUTABLE"',
                         '"$UV_PYTHON_BIN_DIR" >> "$GITHUB_PATH"'):
            self.assertIn(required, script)
        self.assertLess(script.index('uv-integrity.cjs" verify'), script.index('"$SCM_SDK_UV_EXECUTABLE" --version'))
        self.assertNotIn('update-shell', script)
        self.assertNotIn('--force', script)
        attributes = (ROOT / '.gitattributes').read_text()
        self.assertIn('scripts/bootstrap-python.sh text eol=lf', attributes)
        self.assertIn('.python-version text eol=lf', attributes)

    def test_additional_targets_are_manual_and_assert_actual_host(self):
        self.assertNotIn("additional-native-platforms:", WORKFLOW)
        self.assertIn("workflow_dispatch:", NATIVE_WORKFLOW)
        self.assertIn("'host: ${{ matrix.target }}'", NATIVE_WORKFLOW)
        for target in ["aarch64-unknown-linux-gnu", "aarch64-apple-darwin", "x86_64-pc-windows-msvc"]:
            self.assertIn(target, NATIVE_WORKFLOW)


if __name__ == "__main__":
    unittest.main()
