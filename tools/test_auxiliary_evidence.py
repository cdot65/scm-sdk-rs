"""Synthetic-only rejection, leakage, freshness and bounded-runner regressions."""
import contextlib
import copy
import importlib.util
import io
import json
import os
import pathlib
import subprocess
import sys
import tempfile
import unittest
from unittest import mock

ROOT = pathlib.Path(__file__).resolve().parents[1]
sys.path.insert(0, str(ROOT / "scripts"))
import evidence_common as common


def load(name, filename):
    specification = importlib.util.spec_from_file_location(name, ROOT / "scripts" / filename)
    module = importlib.util.module_from_spec(specification)
    specification.loader.exec_module(module)
    return module


private = load("private_material", "check-private-material.py")
stress = load("tracing_stress", "check-tracing-stress.py")
SYNTHETIC = {"mgmtClientId": "synthetic-private-client-identity",
             "mgmtClientSecret": "synthetic-private-secret-selected-for-testing",
             "mgmtTsgId": "98765432101234567"}


class PrivateMaterialTests(unittest.TestCase):
    def setUp(self):
        self.temporary = tempfile.TemporaryDirectory(prefix="scm-private-material-test-")
        self.addCleanup(self.temporary.cleanup)
        self.directory = pathlib.Path(self.temporary.name)
        self.root = self.directory / "repo"
        self.root.mkdir()
        self.config = self.directory / "selected-registration.json"
        self.config.write_text(json.dumps(SYNTHETIC))
        (self.root / ".gitignore").write_text(".local/\n")
        source = self.root / "crates/scm-sdk"
        (source / "src").mkdir(parents=True)
        (source / "examples").mkdir()
        (source / "Cargo.toml").write_text(
            '[package]\nname="scm-sdk"\ninclude=["src/**","examples/**","LICENSE","NOTICE","README.md"]\n')
        (source / "src/lib.rs").write_text("// synthetic source\n")
        (source / "examples/example.rs").write_text("// synthetic example\n")
        self.git("init", "-q")
        self.git("add", ".")
        self.commit()

    def git(self, *arguments):
        return subprocess.check_output(["git", *arguments], cwd=self.root,
                                       env=common.clean_environment(), stderr=subprocess.DEVNULL)

    def commit(self):
        self.git("-c", "user.name=Synthetic Test", "-c", "user.email=synthetic@example.invalid",
                 "commit", "-qm", "Synthetic fixture")

    def receipt(self):
        return private.scan(self.root, self.config)

    def assert_sanitized(self, value):
        output = json.dumps(value)
        for selected in SYNTHETIC.values():
            self.assertNotIn(selected, output)
        self.assertNotIn(str(self.config), output)

    def test_clean_scan_and_receipt_are_sanitized(self):
        report = self.receipt()
        self.assertTrue(report["passed"])
        self.assertEqual(report["selectedExternalClientIdentityOrSecretMatches"], 0)
        self.assertEqual(report["publishedCrateSourceAndExampleCandidatesChecked"], 2)
        self.assert_sanitized(report)

    def test_each_selected_credential_is_detected_in_git_visible_content(self):
        for key in ("mgmtClientId", "mgmtClientSecret"):
            path = self.root / "accidental.txt"
            path.write_text(SYNTHETIC[key])
            report = self.receipt()
            self.assertFalse(report["passed"])
            self.assertEqual(report["selectedExternalClientIdentityOrSecretMatches"], 1)
            self.assert_sanitized(report)
            path.unlink()

    def test_credentials_in_filenames_are_detected_without_exporting_paths(self):
        (self.root / SYNTHETIC["mgmtClientSecret"]).write_text("synthetic")
        report = self.receipt()
        self.assertFalse(report["passed"])
        self.assert_sanitized(report)

    def test_ignored_published_candidate_is_still_scanned(self):
        self.git("config", "core.excludesFile", os.devnull)
        (self.root / ".gitignore").write_text(".local/\nignored.rs\n")
        path = self.root / "crates/scm-sdk/src/ignored.rs"
        path.write_text(SYNTHETIC["mgmtClientSecret"])
        report = self.receipt()
        self.assertFalse(report["passed"])
        self.assertEqual(report["selectedExternalClientIdentityOrSecretMatches"], 0)
        self.assertEqual(report["publishedCrateSourceOrExampleRegistrationValueMatches"], 1)
        self.assert_sanitized(report)

    def test_historical_tenant_only_is_distinguished_from_new_copies(self):
        original = self.root / "historical-fixture.txt"
        original.write_text(SYNTHETIC["mgmtTsgId"])
        self.git("add", ".")
        self.commit()
        historical = self.receipt()
        self.assertTrue(historical["passed"])
        self.assertEqual(historical["existingTenantIdentifier"]["alreadyPresentInHeadFilesCount"], 1)
        original.write_text(SYNTHETIC["mgmtTsgId"] * 2)
        self.assertFalse(self.receipt()["passed"])
        original.write_text(SYNTHETIC["mgmtTsgId"])
        (self.root / "new-copy.txt").write_text(SYNTHETIC["mgmtTsgId"])
        self.assertFalse(self.receipt()["passed"])
        self.assert_sanitized(historical)

    def test_historical_tenant_in_published_source_is_not_exempted(self):
        (self.root / "crates/scm-sdk/src/lib.rs").write_text(SYNTHETIC["mgmtTsgId"])
        self.git("add", ".")
        self.commit()
        report = self.receipt()
        self.assertFalse(report["passed"])
        self.assertEqual(report["publishedCrateSourceOrExampleRegistrationValueMatches"], 1)

    def test_historical_lookup_handles_a_file_larger_than_git_metadata_output_bound(self):
        original = self.root / "large-historical-fixture.txt"
        original.write_bytes(b"x" * (9 * 1024 * 1024) + SYNTHETIC["mgmtTsgId"].encode())
        self.git("add", ".")
        self.commit()
        report = self.receipt()
        self.assertTrue(report["passed"])
        self.assertEqual(report["existingTenantIdentifier"]["alreadyPresentInHeadFilesCount"], 1)
        self.assertEqual(report["existingTenantIdentifier"]["newOrChangedFilesCount"], 0)
        self.assert_sanitized(report)

    def test_frozen_public_corpus_fits_the_standing_file_bound(self):
        for name in ("contracts/native-fixtures/schemas.json", "contracts/typescript.json"):
            self.assertLess((ROOT / name).stat().st_size, common.MAX_FILE_BYTES)
        self.assertEqual(private.LIMITS["bytesPerFile"], common.MAX_FILE_BYTES)

    def test_registration_rejects_relative_internal_missing_and_symlink_paths(self):
        internal = self.root / "selected.json"
        internal.write_text(json.dumps(SYNTHETIC))
        link = self.directory / "selected-link.json"
        link.symlink_to(self.config)
        directory_link = self.directory / "linked-directory"
        directory_link.symlink_to(self.directory, target_is_directory=True)
        for path in (pathlib.Path("selected.json"), internal, link,
                     self.directory / "absent", directory_link / self.config.name,
                     self.directory / ".." / self.directory.name / self.config.name):
            with self.assertRaises((ValueError, OSError)):
                private.registration(path, self.root)

    def test_registration_rejects_malformed_duplicate_unknown_and_oversized_inputs(self):
        cases = [b"{", b"\xff", b"[]", b"null", b"x" * 65537,
                 json.dumps({**SYNTHETIC, "unexpected": "not-supported"}).encode(),
                 b'{"mgmtClientId":"one","mgmtClientId":"two"}']
        for key in SYNTHETIC:
            for value in (None, [], {}, True, "", " ", "x" * 4097, "synthetic\nvalue"):
                cases.append(json.dumps({**SYNTHETIC, key: value}).encode())
        cases += [json.dumps({**SYNTHETIC, "mgmtClientSecret": "x"}).encode(),
                  json.dumps({**SYNTHETIC, "mgmtClientId": "x"}).encode(),
                  json.dumps({**SYNTHETIC, "mgmtTsgId": "1" * 65}).encode(),
                  json.dumps({**SYNTHETIC, "mgmtTsgId": "１２３"}).encode(),
                  json.dumps({**SYNTHETIC, "mgmtClientSecret": SYNTHETIC["mgmtClientId"]}).encode()]
        for content in cases:
            self.config.write_bytes(content)
            with self.assertRaises((ValueError, UnicodeError)):
                private.registration(self.config, self.root)

    def test_published_symlinks_file_bounds_and_include_drift_are_rejected(self):
        source = self.root / "crates/scm-sdk/src"
        link = source / "outside.rs"
        link.symlink_to(self.config)
        with self.assertRaises(ValueError):
            self.receipt()
        link.unlink()
        oversized = source / "large.rs"
        with oversized.open("wb") as output:
            output.truncate(common.MAX_FILE_BYTES + 1)
        with self.assertRaises(ValueError):
            self.receipt()
        oversized.unlink()
        manifest = self.root / "crates/scm-sdk/Cargo.toml"
        manifest.write_text(manifest.read_text().replace('"src/**"', '"**"'))
        with self.assertRaises(ValueError):
            self.receipt()

    def test_receipt_freshness_excludes_later_bookkeeping_but_detects_source_drift(self):
        report = self.receipt()
        destination = self.directory / "receipt.json"
        destination.write_text(json.dumps(report))
        self.assertEqual(private.check_receipt(self.root, destination), report)
        (self.root / "docs/checks").mkdir(parents=True)
        (self.root / "docs/checks/later.json").write_text('{"synthetic":true}')
        private.check_receipt(self.root, destination)
        (self.root / "crates/scm-sdk/src/lib.rs").write_text("// changed source\n")
        with self.assertRaises(ValueError):
            private.check_receipt(self.root, destination)

    def test_public_receipt_accepts_only_bookkeeping_descendants_without_rewriting_history(self):
        report = self.receipt()
        destination = self.root / "docs/checks/historical-receipt.json"
        destination.parent.mkdir(parents=True)
        original = json.dumps(report).encode()
        destination.write_bytes(original)
        self.git("add", "docs/checks/historical-receipt.json")
        self.commit()
        self.assertNotEqual(self.git("rev-parse", "HEAD").decode().strip(), report["headCommit"])
        self.assertEqual(private.check_receipt(self.root, destination), report)
        self.assertEqual(destination.read_bytes(), original)

    def test_public_receipt_rejects_source_edit_and_revert_despite_matching_current_bytes(self):
        report = self.receipt()
        destination = self.directory / "historical-receipt.json"
        original = json.dumps(report).encode()
        destination.write_bytes(original)
        source = self.root / "crates/scm-sdk/src/lib.rs"
        previous = source.read_bytes()
        source.write_text("// changed source\n")
        self.git("add", "crates/scm-sdk/src/lib.rs")
        self.commit()
        source.write_bytes(previous)
        self.git("add", "crates/scm-sdk/src/lib.rs")
        self.commit()
        self.assertEqual(report["sourceSha256"], self.receipt()["sourceSha256"])
        with self.assertRaisesRegex(ValueError, "Receipt head identity mismatch") as caught:
            private.check_receipt(self.root, destination)
        self.assertEqual(private.check_failure_code(caught.exception), "head_changed")
        self.assertEqual(destination.read_bytes(), original)
        fresh = self.receipt()
        destination = self.directory / "fresh-receipt.json"
        destination.write_text(json.dumps(fresh))
        self.assertEqual(private.check_receipt(self.root, destination), fresh)

    def test_public_receipt_rejects_unrelated_history_with_identical_source(self):
        report = self.receipt()
        destination = self.directory / "receipt.json"
        destination.write_text(json.dumps(report))
        self.git("checkout", "--orphan", "unrelated")
        self.git("-c", "user.name=Synthetic Test", "-c", "user.email=synthetic@example.invalid",
                 "commit", "-qm", "Unrelated synthetic history")
        self.assertEqual(report["sourceSha256"], self.receipt()["sourceSha256"])
        with self.assertRaisesRegex(ValueError, "Receipt head identity mismatch"):
            private.check_receipt(self.root, destination)

    def test_public_receipt_does_not_read_excluded_evidence_into_its_source_budget(self):
        report = self.receipt()
        destination = self.directory / "receipt.json"
        destination.write_text(json.dumps(report))
        source_bytes = sum(map(len, common.git_files(self.root).values()))
        for name in ["docs/checks/generated.json", "docs/reviews/generated.json", "docs/phase-status.json"]:
            path = self.root / name
            path.parent.mkdir(parents=True, exist_ok=True)
            path.write_text("x" * 1024)
        with mock.patch.object(common, "MAX_TREE_BYTES", source_bytes + 128):
            self.assertEqual(private.check_receipt(self.root, destination), report)
            with self.assertRaisesRegex(ValueError, "byte-count bound"):
                common.git_files(self.root)

    def test_public_receipt_skips_excluded_symlinks_and_private_scan_rejects_them(self):
        report = self.receipt()
        destination = self.directory / "receipt.json"
        destination.write_text(json.dumps(report))
        path = self.root / "docs/checks/generated.json"
        path.parent.mkdir(parents=True)
        path.symlink_to(self.config)
        self.assertEqual(private.check_receipt(self.root, destination), report)
        with self.assertRaises(ValueError):
            self.receipt()

    def test_private_scan_still_detects_credentials_in_excluded_evidence(self):
        path = self.root / "docs/checks/generated.json"
        path.parent.mkdir(parents=True)
        path.write_text(SYNTHETIC["mgmtClientSecret"])
        report = self.receipt()
        self.assertFalse(report["passed"])
        self.assertEqual(report["selectedExternalClientIdentityOrSecretMatches"], 1)
        self.assert_sanitized(report)

    def test_public_failure_categories_do_not_reflect_values_or_paths(self):
        errors = [(ValueError("Repository byte-count bound exceeded"), "repository_byte_limit"),
            (ValueError("Git evidence acquisition failed"), "git_read_failed"),
            (ValueError("Receipt source fingerprint mismatch"), "source_changed"),
            (ValueError(SYNTHETIC["mgmtClientSecret"]), "invalid_receipt"),
            (OSError(str(self.config)), "filesystem_read_failed")]
        for error, expected in errors:
            result = private.check_failure_code(error)
            self.assertEqual(result, expected)
            self.assert_sanitized(result)

    def test_check_cli_emits_a_closed_category_without_echoing_invalid_receipt(self):
        completed = subprocess.run([sys.executable, str(ROOT / "scripts/check-private-material.py"),
            "--check", str(self.config)], text=True, capture_output=True, timeout=10)
        self.assertEqual(completed.returncode, 1)
        self.assertIn("Public receipt verification failed: invalid_receipt", completed.stderr)
        self.assert_sanitized(completed.stdout + completed.stderr)

    def test_receipt_schema_success_counts_and_scope_are_enforced(self):
        report = self.receipt()
        destination = self.directory / "receipt.json"
        variants = []
        for key, value in (("passed", False), ("sourceUnchanged", False), ("schemaVersion", True),
                           ("sourceSha256", "a" * 64), ("gitVisibleFilesChecked", True),
                           ("selectedExternalClientIdentityOrSecretMatches", 1),
                           ("publishedCrateSourceOrExampleRegistrationValueMatches", 1),
                           ("scope", "unreviewed"), ("limits", {}), ("limitations", []),
                           ("checkedAt", "2026-10-16T00:00:00"), ("unexpected", "unreviewed")):
            variants.append({**report, key: value})
        nested = copy.deepcopy(report)
        nested["existingTenantIdentifier"]["newOrChangedFilesCount"] = 1
        variants.append(nested)
        for variant in variants:
            destination.write_text(json.dumps(variant))
            with self.assertRaises((ValueError, TypeError)):
                private.check_receipt(self.root, destination)

    def test_receipt_freshness_also_binds_ignored_package_candidate_contents(self):
        (self.root / ".gitignore").write_text(".local/\nignored.rs\n")
        path = self.root / "crates/scm-sdk/src/ignored.rs"
        path.write_text("// initially synthetic\n")
        report = self.receipt()
        destination = self.directory / "receipt.json"
        destination.write_text(json.dumps(report))
        private.check_receipt(self.root, destination)
        path.write_text("// changed ignored candidate\n")
        with self.assertRaises(ValueError):
            private.check_receipt(self.root, destination)

    def test_published_tree_byte_and_file_count_bounds_are_fail_closed(self):
        with mock.patch.object(private, "MAX_TREE_BYTES", 1):
            with self.assertRaises(ValueError):
                private.published_candidates(self.root)
        with mock.patch.object(private, "MAX_FILES", 1):
            with self.assertRaises(ValueError):
                private.published_candidates(self.root)

    def test_changed_source_cannot_produce_a_passing_scan(self):
        original = private.git_files
        calls = 0
        def changing(root):
            nonlocal calls
            files = original(root)
            calls += 1
            if calls > 1:
                files["new.txt"] = b"synthetic"
            return files
        with mock.patch.object(private, "git_files", side_effect=changing):
            self.assertFalse(self.receipt()["passed"])

    def test_cli_sanitizes_malformed_input_errors_without_reading_real_credentials(self):
        self.config.write_bytes(b'{"selected":"' + SYNTHETIC["mgmtClientSecret"].encode() + b'", broken}')
        result = subprocess.run([sys.executable, str(ROOT / "scripts/check-private-material.py"),
                                 "--config", str(self.config), "--output", str(self.directory / "output.json")],
                                capture_output=True, timeout=10)
        self.assertNotEqual(result.returncode, 0)
        combined = result.stdout + result.stderr
        for value in SYNTHETIC.values():
            self.assertNotIn(value.encode(), combined)
        self.assertNotIn(str(self.config).encode(), combined)
        self.assertNotIn(b"Traceback", combined)

    def test_check_cli_never_reopens_selected_config(self):
        report = self.receipt()
        destination = self.directory / "receipt.json"
        destination.write_text(json.dumps(report))
        self.config.unlink()
        output = io.StringIO()
        with mock.patch.object(private, "ROOT", self.root), mock.patch.object(sys, "argv", [
                "check-private-material.py", "--check", str(destination)]), contextlib.redirect_stdout(output):
            self.assertEqual(private.main(), 0)
        self.assertEqual(json.loads(output.getvalue())["privateScanReexecuted"], False)


class BoundedEvidenceTests(unittest.TestCase):
    @unittest.skipUnless(os.open in os.supports_dir_fd and hasattr(os, "O_DIRECTORY"),
                         "Atomic replacement requires directory-relative OS primitives")
    def test_maintenance_stress_receipt_preserves_the_exact_path_boundary(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            reports = root / "docs/checks"
            reports.mkdir(parents=True)
            path = reports / "phase-91-tracing-stress-1.99.0.json"
            report = {"compiler": "1.99.0", "passed": True}
            stress.write_receipt(root, path, report, replace=True)
            self.assertEqual(json.loads(path.read_text()), report)
            self.assertEqual(path.stat().st_mode & 0o777, 0o600)
            for unsafe in (reports / "phase-90-tracing-stress-1.99.0.json",
                           reports / "phase-91-tracing-stress-1.89.0.json",
                           root / "Cargo.toml"):
                with self.assertRaises(ValueError):
                    stress.write_receipt(root, unsafe, report, replace=True)

    @unittest.skipUnless(os.open in os.supports_dir_fd and hasattr(os, "O_DIRECTORY"),
                         "Atomic replacement requires directory-relative OS primitives")
    def test_stress_report_replacement_is_restricted_atomic_and_private(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            reports = root / "docs/checks"
            reports.mkdir(parents=True)
            destination = reports / "phase-08-tracing-stress-1.99.0.json"
            destination.write_text("old-report")
            report = {"compiler": "1.99.0", "passed": True}
            stress.write_receipt(root, destination, report, replace=True)
            self.assertEqual(json.loads(destination.read_text()), report)
            self.assertEqual(destination.stat().st_mode & 0o777, 0o600)
            self.assertEqual(list(reports.iterdir()), [destination])
            with self.assertRaises(OSError):
                stress.write_receipt(root, destination, report)
            for unsafe in (root / "Cargo.toml", reports / "unrelated.json",
                           reports / "phase-08-tracing-stress-1.89.0.json",
                           reports / ".." / "checks" / destination.name):
                with self.assertRaises(ValueError):
                    stress.write_receipt(root, unsafe, report, replace=True)
            destination.unlink()
            target = root / "outside.json"
            target.write_text("untouched")
            destination.symlink_to(target)
            with self.assertRaises(ValueError):
                stress.write_receipt(root, destination, report, replace=True)
            self.assertEqual(target.read_text(), "untouched")
            destination.unlink()
            destination.mkdir()
            with self.assertRaises(ValueError):
                stress.write_receipt(root, destination, report, replace=True)

    def run_synthetic_stress(self, *, source_drift=False, runtime_drift=False,
                             binary_drift=False, build_fails=False):
        with tempfile.TemporaryDirectory(prefix="scm-stress-test-") as temporary:
            root = pathlib.Path(temporary)
            binary = root / "synthetic-library-tests"
            binary.write_bytes(b"synthetic-binary")
            artifact = {"reason": "compiler-artifact", "target": {"name": "scm_sdk", "kind": ["lib"]},
                        "profile": {"test": True}, "executable": str(binary)}
            compiler = b"rustc 1.99.0 (synthetic)\nhost: x86_64-unknown-linux-musl\nrelease: 1.99.0\n"
            good = {"exitCode": 0, "failure": None, "durationSeconds": 0.1,
                    "stdout": b"test result: ok. 201 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n", "stderr": b""}
            builds = []
            def run(command, cwd, environment, timeout, **kwargs):
                self.assertNotIn("SCM_TEST_SECRET", environment)
                if command[-1] == "-vV":
                    return {**good, "stdout": compiler}
                if "--no-run" in command:
                    builds.append(command)
                    return {**good, "exitCode": 1 if build_fails else 0,
                            "stdout": json.dumps(artifact).encode()}
                if binary_drift:
                    binary.write_bytes(b"changed-synthetic-binary")
                return good
            output = root / "receipt.json"
            files = [{"synthetic.rs": b"before"}, {"synthetic.rs": b"after" if source_drift else b"before"}]
            runtime = ["a" * 64, ("b" if runtime_drift else "a") * 64]
            with mock.patch.object(stress, "ROOT", root), mock.patch.object(stress, "run_bounded", side_effect=run), \
                    mock.patch.object(stress, "git_files", side_effect=files), \
                    mock.patch.object(stress, "runtime_fingerprint", side_effect=runtime), \
                    mock.patch.dict(os.environ, {"SCM_TEST_SECRET": "synthetic-ignored-environment"}), \
                    mock.patch.object(sys, "argv", ["check-tracing-stress.py", "--toolchain", "1.99.0", \
                                                  "--processes", "2", "--output", str(output)]), \
                    contextlib.redirect_stdout(io.StringIO()):
                exit_code = stress.main()
            return exit_code, json.loads(output.read_text()), builds

    def test_stress_receipt_binds_one_build_exact_compiler_source_runtime_and_binary(self):
        exit_code, report, builds = self.run_synthetic_stress()
        self.assertEqual(exit_code, 0)
        self.assertTrue(report["passed"])
        self.assertEqual(len(builds), 1)
        for flag in ("--lib", "--all-features", "--offline", "--locked", "--no-run", "--message-format=json"):
            self.assertIn(flag, builds[0])
        self.assertEqual(report["hostTarget"], "x86_64-unknown-linux-musl")
        self.assertEqual(len(report["runs"]), 2)
        for key in ("sourceSha256", "runtimeInputsSha256", "binarySha256"):
            self.assertRegex(report[key], r"^[0-9a-f]{64}$")

    def test_stress_source_runtime_binary_drift_or_build_failure_cannot_pass(self):
        for options in ({"source_drift": True}, {"runtime_drift": True},
                        {"binary_drift": True}, {"build_fails": True}):
            exit_code, report, builds = self.run_synthetic_stress(**options)
            self.assertEqual(exit_code, 1)
            self.assertFalse(report["passed"])
            self.assertEqual(len(builds), 1)

    def test_sensitive_environment_inputs_are_removed(self):
        with mock.patch.dict(os.environ, {"SCM_TEST_SECRET": "synthetic", "AIRS_TEST_SECRET": "synthetic",
                                         "PANW_TEST_SECRET": "synthetic", "GIT_DIR": "/synthetic", "RUST_LOG": "debug"}):
            environment = common.clean_environment()
        for key in ("SCM_TEST_SECRET", "AIRS_TEST_SECRET", "PANW_TEST_SECRET", "GIT_DIR", "RUST_LOG"):
            self.assertNotIn(key, environment)

    def test_private_receipt_creation_does_not_overwrite_or_follow_links(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            output = root / "receipt.json"
            common.write_private(output, b"{}")
            if os.name == "posix":
                self.assertEqual(output.stat().st_mode & 0o777, 0o600)
            with self.assertRaises(OSError):
                common.write_private(output, b"overwritten")
            link = root / "link"
            link.symlink_to(output)
            with self.assertRaises(ValueError):
                common.write_private(link, b"overwritten")
            self.assertEqual(output.read_bytes(), b"{}")

    def test_subprocess_timeout_and_output_bounds(self):
        with tempfile.TemporaryDirectory() as temporary:
            root = pathlib.Path(temporary)
            result = common.run_bounded([sys.executable, "-c", "import time; time.sleep(5)"],
                                         root, common.clean_environment(), 0.05)
            self.assertEqual(result["failure"], "timeout")
            result = common.run_bounded([sys.executable, "-c", "print('x' * 10000)"],
                                         root, common.clean_environment(), 5, output_limit=100)
            self.assertEqual(result["failure"], "output_limit")

    def test_cargo_json_selects_only_the_library_test_executable(self):
        artifact = {"reason": "compiler-artifact", "target": {"name": "scm_sdk", "kind": ["lib"]},
                    "profile": {"test": True}, "executable": "/synthetic/library-tests"}
        other = {**artifact, "target": {"name": "example", "kind": ["example"]}}
        lines = b"\n".join(json.dumps(value).encode() for value in (other, artifact, {"reason": "build-finished"}))
        self.assertEqual(stress.executable_from_messages(lines), pathlib.Path(artifact["executable"]))
        for content in (b"", json.dumps(other).encode(), b"not-json",
                        b"\n".join(json.dumps(value).encode() for value in (artifact, {**artifact, "executable": "/different"}))):
            with self.assertRaises(ValueError):
                stress.executable_from_messages(content)

    def test_repetition_is_exact_fail_fast_and_checks_every_summary(self):
        good = {"exitCode": 0, "failure": None, "durationSeconds": 0.1,
                "stdout": b"test result: ok. 201 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out;\n", "stderr": b""}
        with mock.patch.object(stress, "run_bounded", return_value=good) as run:
            rows = stress.repeat_suite(pathlib.Path("/synthetic/tests"), ROOT, {}, 100, 16, 201, 60, 900)
            self.assertEqual(len(rows), 100)
            self.assertTrue(all(row["passed"] for row in rows))
            self.assertEqual(run.call_count, 100)
            self.assertEqual(run.call_args.args[0], ["/synthetic/tests", "--test-threads=16"])
        for bad in ({**good, "exitCode": 1}, {**good, "failure": "timeout"},
                    {**good, "stdout": good["stdout"].replace(b"201 passed", b"200 passed")},
                    {**good, "stdout": good["stdout"].replace(b"0 ignored", b"1 ignored")},
                    {**good, "stdout": good["stdout"] * 2}):
            with mock.patch.object(stress, "run_bounded", side_effect=[good, bad]) as run:
                rows = stress.repeat_suite(pathlib.Path("/synthetic/tests"), ROOT, {}, 100, 16, 201, 60, 900)
                self.assertEqual(len(rows), 2)
                self.assertFalse(rows[-1]["passed"])
                self.assertEqual(run.call_count, 2)

    def test_total_repetition_deadline_is_fail_closed(self):
        with mock.patch.object(stress.time, "monotonic", side_effect=[0, 2]), mock.patch.object(stress, "run_bounded") as run:
            rows = stress.repeat_suite(pathlib.Path("/synthetic/tests"), ROOT, {}, 100, 16, 201, 60, 1)
            self.assertEqual(rows, [{"run": 1, "exitCode": None, "failure": "total_timeout"}])
            run.assert_not_called()


if __name__ == "__main__":
    unittest.main()
