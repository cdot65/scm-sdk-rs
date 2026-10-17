"""Prevent CI compiler, feature, pinned-action, and credential-policy drift."""

import importlib.util
import json
import pathlib
import re
import tomllib
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKFLOW = (ROOT / ".forgejo/workflows/ci.yml").read_text()
RUST_ACTION = "https://github.com/dtolnay/rust-toolchain@683b588c487a69d01f6c6ddf40f036ff682cc2c8"
UV_ACTION = "https://github.com/astral-sh/setup-uv@4959332f0f014c5280e7eac8b70c90cb574c9f9b"


def flow_list(name):
    match = re.search(rf"^\s+{name}: (\[.*\])$", WORKFLOW, re.MULTILINE)
    if match is None:
        raise AssertionError(f"Missing machine-readable {name} matrix")
    return json.loads(match.group(1))


class CiContractTests(unittest.TestCase):
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

    def test_node_matches_owned_pin_and_phase_gate_verifies_only(self):
        node = (ROOT / ".node-version").read_text().strip()
        self.assertIn(f'node-version: "{node}"', WORKFLOW)
        self.assertIn("tools/bootstrap_reference.py", WORKFLOW)
        self.assertIn("tools/check_phase.py --phase 8 --verification-only", WORKFLOW)

    def test_actions_and_credentials_are_bounded(self):
        actions = re.findall(r"^\s+uses: (\S+)", WORKFLOW, re.MULTILINE)
        self.assertEqual(len(actions), 11)
        self.assertEqual(set(actions), {
            "https://github.com/actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5",
            "https://github.com/actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020",
            UV_ACTION,
            RUST_ACTION,
        })
        for action in actions:
            self.assertRegex(action, r"^https://github\.com/(?:actions|dtolnay|astral-sh)/[a-z-]+@[0-9a-f]{40}$")
        self.assertEqual(WORKFLOW.count("persist-credentials: false"), 3)
        self.assertIn("permissions:\n  contents: read\n", WORKFLOW)
        for forbidden in ["secrets.", "pull_request_target", "contents: write",
                          "id-token: write", "enable-openid-connect", "cargo publish",
                          "npm publish", "git push"]:
            self.assertNotIn(forbidden, WORKFLOW)

    def test_every_job_bootstraps_rust_before_running_commands(self):
        jobs = {
            "contracts-and-package": ['"1.89.0"', '"1.99.0"'],
            "rust": ["${{ matrix.toolchain }}"],
            "additional-native-platforms": ['"1.99.0"'],
        }
        for job, toolchains in jobs.items():
            with self.subTest(job=job):
                match = re.search(rf"^  {job}:\n(.*?)(?=^  [a-z][a-z-]*:\n|\Z)",
                                  WORKFLOW, re.MULTILINE | re.DOTALL)
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
                self.assertLess(body.index(UV_ACTION), body.index("        run:"))
                self.assertEqual(body.index("        run: bash scripts/bootstrap-python.sh"), body.index("        run:"))

    def test_python_pin_supports_tomllib_and_filtered_tar_extraction(self):
        version = (ROOT / ".python-version").read_text().strip()
        self.assertRegex(version, r"^3\.[0-9]+\.[0-9]+$")
        self.assertGreaterEqual(tuple(map(int, version.split("."))), (3, 12, 0))

    def test_ci_documentation_matches_action_pins_and_setup_effects(self):
        document = (ROOT / "docs/CI.md").read_text()
        actions = set(re.findall(r"^\s+uses: https://github\.com/(\S+)", WORKFLOW, re.MULTILINE))
        documented = {f"{name}@{commit}" for name, commit in re.findall(
            r"^\| \[([^]]+)\]\(https://github\.com/[^)]+\) \| [^|]+ \| ([0-9a-f]{40}) \|$",
            document, re.MULTILINE)}
        self.assertEqual(documented, actions)
        self.assertEqual(WORKFLOW.count(RUST_ACTION + " # reviewed untagged master commit"), 4)
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
                         'uv --no-config --no-cache python install --default --no-registry',
                         '"$UV_PYTHON_BIN_DIR" >> "$GITHUB_PATH"'):
            self.assertIn(required, script)
        self.assertNotIn('update-shell', script)
        self.assertNotIn('--force', script)
        attributes = (ROOT / '.gitattributes').read_text()
        self.assertIn('scripts/bootstrap-python.sh text eol=lf', attributes)
        self.assertIn('.python-version text eol=lf', attributes)

    def test_additional_targets_are_manual_and_assert_actual_host(self):
        self.assertIn("inputs.native_platforms == true", WORKFLOW)
        self.assertIn("'host: ${{ matrix.target }}'", WORKFLOW)
        for target in ["aarch64-unknown-linux-gnu", "aarch64-apple-darwin", "x86_64-pc-windows-msvc"]:
            self.assertIn(target, WORKFLOW)


if __name__ == "__main__":
    unittest.main()
