"""Prevent CI compiler, feature, pinned-action, and credential-policy drift."""

import importlib.util
import json
import pathlib
import re
import tomllib
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
WORKFLOW = (ROOT / ".forgejo/workflows/ci.yml").read_text()


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
        self.assertEqual(len(actions), 4)
        self.assertEqual(set(actions), {
            "https://github.com/actions/checkout@34e114876b0b11c390a56381ad16ebd13914f8d5",
            "https://github.com/actions/setup-node@49933ea5288caeca8642d1e84afbd3f7d6820020",
        })
        for action in actions:
            self.assertRegex(action, r"^https://github\.com/actions/[a-z-]+@[0-9a-f]{40}$")
        self.assertEqual(WORKFLOW.count("persist-credentials: false"), 3)
        self.assertIn("permissions:\n  contents: read\n", WORKFLOW)
        for forbidden in ["secrets.", "pull_request_target", "contents: write",
                          "id-token: write", "enable-openid-connect", "cargo publish",
                          "npm publish", "git push"]:
            self.assertNotIn(forbidden, WORKFLOW)

    def test_additional_targets_are_manual_and_assert_actual_host(self):
        self.assertIn("inputs.native_platforms == true", WORKFLOW)
        self.assertIn("'host: ${{ matrix.target }}'", WORKFLOW)
        for target in ["aarch64-unknown-linux-gnu", "aarch64-apple-darwin", "x86_64-pc-windows-msvc"]:
            self.assertIn(target, WORKFLOW)


if __name__ == "__main__":
    unittest.main()
