import copy
import hashlib
import json
import pathlib
import tempfile
import unittest

from verify_native_fixtures import DIRECTORY, REQUIRED, verify


class NativeFixtureTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory()
        self.addCleanup(self.temporary.cleanup)
        self.root = pathlib.Path(self.temporary.name)
        self.source = "contracts/fixtures/typescript/test/openapi/fixtures/example.json"
        self.case = {"method": "POST", "body": {"enabled": True, "count": 1}}
        self.write(self.source, {"cases": [self.case]})
        self.sha = hashlib.sha256((self.root / self.source).read_bytes()).hexdigest()
        self.manifest = {"baseline": {"commit": "frozen"}, "fixtures": [
            {"path": self.source, "sha256": self.sha, "category": "independent-openapi-fixture"}
        ]}
        self.write("contracts/manifest.json", self.manifest)
        self.bundle = {"schemaVersion": 1, "baselineCommit": "frozen", "license": "Apache-2.0",
                       "sources": [{"path": self.source, "sha256": self.sha, "license": "Apache-2.0"}],
                       "cases": [{"source": {"path": self.source, "sha256": self.sha,
                                             "pointer": "/cases/0"}, "case": copy.deepcopy(self.case)}]}
        self.destination = (DIRECTORY / "red-team-contracts.json").as_posix()
        self.save()

    def write(self, name, value):
        path = self.root / name
        path.parent.mkdir(parents=True, exist_ok=True)
        path.write_text(json.dumps(value) + "\n")

    def save(self):
        for name in REQUIRED:
            self.write((DIRECTORY / name).as_posix(), self.bundle)

    def rejected(self):
        self.save()
        with self.assertRaises(ValueError):
            verify(self.root)

    def test_exact_copy_passes(self):
        self.assertEqual(verify(self.root), {"bundles": len(REQUIRED), "exact_frozen_cases": len(REQUIRED)})

    def test_case_edits_and_json_type_substitution_rejected(self):
        for key, value in (("enabled", 1), ("count", 1.0), ("count", 2)):
            self.bundle["cases"][0]["case"] = copy.deepcopy(self.case)
            self.bundle["cases"][0]["case"]["body"][key] = value
            self.rejected()

    def test_source_changes_cannot_be_blessed_by_bundle_hash(self):
        self.write(self.source, {"cases": [{"changed": True}]})
        changed = hashlib.sha256((self.root / self.source).read_bytes()).hexdigest()
        self.bundle["sources"][0]["sha256"] = changed
        self.bundle["cases"][0]["source"]["sha256"] = changed
        self.rejected()

    def test_wrong_baseline_and_missing_attribution_rejected(self):
        self.bundle["baselineCommit"] = "changed"
        self.rejected()
        self.bundle["baselineCommit"] = "frozen"
        self.bundle["license"] = ""
        self.rejected()

    def test_unknown_sources_and_regressions_cannot_claim_independence(self):
        original = self.bundle["sources"][0]["path"]
        self.bundle["sources"][0]["path"] = "contracts/unreviewed.json"
        self.rejected()
        self.bundle["sources"][0]["path"] = original
        self.manifest["fixtures"][0]["category"] = "sdk-authored-regression-reference"
        self.write("contracts/manifest.json", self.manifest)
        self.rejected()

    def test_invalid_duplicate_or_missing_coordinates_rejected(self):
        for pointer in ("/cases/01", "/cases/1", "/cases/-1", "/cases/0/body"):
            self.bundle["cases"][0]["source"]["pointer"] = pointer
            self.rejected()
        self.bundle["cases"][0]["source"]["pointer"] = "/cases/0"
        self.bundle["cases"].append(copy.deepcopy(self.bundle["cases"][0]))
        self.rejected()

    def test_duplicate_source_and_empty_cases_rejected(self):
        self.bundle["sources"].append(copy.deepcopy(self.bundle["sources"][0]))
        self.rejected()
        self.bundle["sources"].pop()
        self.bundle["cases"] = []
        self.rejected()

    def test_missing_bundle_and_symlink_rejected(self):
        path = self.root / self.destination
        original = path.read_bytes()
        path.unlink()
        with self.assertRaises(ValueError):
            verify(self.root)
        sibling = path.with_name("elsewhere.json")
        sibling.write_bytes(original)
        path.symlink_to(sibling)
        with self.assertRaises(ValueError):
            verify(self.root)

    def test_nonfinite_json_rejected(self):
        self.bundle["cases"][0]["case"]["body"]["count"] = float("nan")
        self.rejected()

    def bindings(self, wrapper="{}", raw=False):
        directory = self.root / "crates/scm-sdk/tests"
        directory.mkdir(parents=True, exist_ok=True)
        lines = []
        for name in sorted(REQUIRED):
            argument = f'r#"../src/fixtures/{name}"#' if raw else f'"../src/fixtures/{name}"'
            lines.append(wrapper.format(f'const CASES: &str = include_str!({argument});'))
        (directory / "fixture_tests.rs").write_text("\n".join(lines))

    def test_literal_test_bindings_pass(self):
        for raw in (False, True):
            self.bindings(raw=raw)
            self.assertEqual(verify(self.root, require_test_bindings=True)["test_bound_bundles"],
                             len(REQUIRED))

    def test_orphaned_bundle_rejected(self):
        with self.assertRaisesRegex(ValueError, "no literal Rust test binding"):
            verify(self.root, require_test_bindings=True)

    def test_commented_or_quoted_bindings_do_not_count(self):
        for wrapper in ("// {}", "/* outer /* nested */ {} */", 'const TEXT: &str = r###"{}"###;'):
            self.bindings(wrapper)
            with self.assertRaisesRegex(ValueError, "no literal Rust test binding"):
                verify(self.root, require_test_bindings=True)

    def test_unknown_bundle_binding_rejected(self):
        self.bindings()
        self.write((DIRECTORY / "unknown-contracts.json").as_posix(), self.bundle)
        # This new bundle also needs its own literal binding.
        with self.assertRaisesRegex(ValueError, "no literal Rust test binding"):
            verify(self.root, require_test_bindings=True)

    def test_symlinked_test_source_rejected(self):
        self.bindings()
        path = self.root / "crates/scm-sdk/tests/fixture_tests.rs"
        replacement = self.root / "fixture-source.rs"
        path.rename(replacement)
        path.symlink_to(replacement)
        with self.assertRaisesRegex(ValueError, "must not follow symlinks"):
            verify(self.root, require_test_bindings=True)


if __name__ == "__main__":
    unittest.main()
