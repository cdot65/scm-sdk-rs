"""G10 inventory, patch compatibility and hostile publication-proof tests."""
import copy
import hashlib
import importlib.util
import io
import json
import pathlib
import tarfile
import tempfile
import tomllib
import unittest

import response_defaults
from check_phase import commands_for

ROOT = pathlib.Path(__file__).resolve().parents[1]
spec = importlib.util.spec_from_file_location("g10_publication", ROOT / "scripts/verify-publication.py")
publication = importlib.util.module_from_spec(spec)
spec.loader.exec_module(publication)
payload_spec = importlib.util.spec_from_file_location("g10_published_payload", ROOT / "scripts/check-published-payload.py")
payload = importlib.util.module_from_spec(payload_spec)
payload_spec.loader.exec_module(payload)


def canonical_sha(value):
    return hashlib.sha256(json.dumps(value, sort_keys=True, separators=(",", ":")).encode()).hexdigest()


class G10ContractTests(unittest.TestCase):
    def test_published_registry_inventory_matches_both_clean_proofs_and_remote_tag(self):
        commit = "7efd1372806bd1969663993de3083d393b20cc62"
        digest = "8339891e9a43582e26573355e65d95605f6865ea434365a649b215c69b01d154"
        receipt = json.loads((ROOT / "docs/checks/release-registry-0.1.1.json").read_text())
        tag = json.loads((ROOT / "docs/checks/release-tag-v0.1.1.json").read_text())
        self.assertEqual(receipt["version"], "0.1.1")
        self.assertEqual(receipt["sourceCommit"], commit)
        self.assertEqual(receipt["archiveSha256"], digest)
        self.assertEqual(tag["sourceCommit"], commit)
        self.assertEqual(tag["remoteRefs"]["refs/tags/v0.1.1^{}"], commit)
        for compiler in ["1.89.0", "1.99.0"]:
            proof = json.loads((ROOT / f"docs/checks/release-source-{commit}/release-package-{compiler}.json").read_text())
            self.assertTrue(proof["passed"])
            self.assertTrue(proof["requireClean"])
            self.assertFalse(proof["effectiveDirty"])
            self.assertEqual(proof["releaseCommit"], commit)
            self.assertEqual(proof["files"], receipt["files"])
            self.assertEqual(proof["vcs"], receipt["vcs"])
            if compiler == "1.99.0":
                self.assertEqual(proof["archive_sha256"], digest)

    def test_accepted_maintenance_command_sets_keep_their_historical_meaning(self):
        for phase, expected in (
            (90, "f7c51a81c3ad5cef4b190ebaebe3e54e4553e0876fc6adbd3ad04770078265b3"),
            (91, "2642061af0f1e9f13a875dddb6a3ea6d79f86e5ceb76614b0cb99e47c051cfcf"),
        ):
            with self.subTest(phase=phase):
                self.assertEqual(canonical_sha(commands_for(phase)), expected)
    def test_published_payload_check_rejects_source_drift_additions_and_unsafe_paths(self):
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            files = {"Cargo.toml.orig": b"fixture manifest", "README.md": b"fixture readme",
                     "LICENSE": b"fixture license", "NOTICE": b"fixture notice",
                     "src/lib.rs": b"// fixture source", "examples/example.rs": b"// fixture example"}
            for name, data in files.items():
                target = (root / "crates/scm-sdk/Cargo.toml" if name == "Cargo.toml.orig" else
                          root / "crates/scm-sdk" / name if name.startswith(("src/", "examples/")) else root / name)
                target.parent.mkdir(parents=True, exist_ok=True)
                target.write_bytes(data)
            receipt = {"schemaVersion": 1, "registry": "crates.io", "package": "scm-sdk",
                       "ownerLogins": ["cdot65"], "version": "0.1.1", "sourceCommit": "a" * 40,
                       "archiveSha256": "b" * 64, "files": publication.file_rows(files)}
            self.assertTrue(payload.verify(root, receipt)["passed"])
            source = root / "crates/scm-sdk/src/lib.rs"
            source.write_bytes(b"// changed fixture")
            with self.assertRaises(ValueError):
                payload.verify(root, receipt)
            source.write_bytes(files["src/lib.rs"])
            source.unlink()
            with self.assertRaisesRegex(ValueError, "missing or unreadable"):
                payload.verify(root, receipt)
            source.write_bytes(files["src/lib.rs"])
            added = root / "crates/scm-sdk/src/added.rs"
            added.write_bytes(b"// added fixture")
            with self.assertRaises(ValueError):
                payload.verify(root, receipt)
            added.unlink()
            added.symlink_to(source)
            with self.assertRaises(ValueError):
                payload.verify(root, receipt)
            added.unlink()
            for bad in [None, [], {}, dict(receipt, version=[]), dict(receipt, files=[{}])]:
                with self.subTest(malformed=type(bad).__name__), self.assertRaises(ValueError):
                    payload.verify(root, bad)
            for name in ["../outside", "/outside", "src\\outside", "src//lib.rs", "src/lib.rs"]:
                changed = copy.deepcopy(receipt)
                changed["files"].append({"path": name, "sha256": "a" * 64, "bytes": 0})
                with self.subTest(path=name), self.assertRaises(ValueError):
                    payload.verify(root, changed)

    def test_rust_additions_resolve_to_declarations_and_the_packaged_example(self):
        self.assertEqual(response_defaults.verify_rust_additions(),
                         json.loads((ROOT / response_defaults.OUTPUT).read_text())["rustOnlyAdditions"])
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            sources = {row[1] for row in response_defaults.RUST_ADDITIONS}
            sources.add("crates/scm-sdk/examples/response_defaults.rs")
            for source in sources:
                path = root / source
                path.parent.mkdir(parents=True, exist_ok=True)
                path.write_text((ROOT / source).read_text())
            for source, old, new in (
                ("crates/scm-sdk/src/config.rs", "pub fn without_response_defaults", "pub fn renamed_policy"),
                ("crates/scm-sdk/src/config.rs", "pub fn response_defaults", "pub fn renamed_getter"),
                ("crates/scm-sdk/src/validation.rs", "    ResponseWithoutDefaults,", "    RenamedVariant,"),
                ("crates/scm-sdk/examples/response_defaults.rs", ".without_response_defaults()", ".renamed_policy()"),
                ("crates/scm-sdk/examples/response_defaults.rs", ".response_defaults()", ".renamed_getter()"),
                ("crates/scm-sdk/examples/response_defaults.rs", "Mode::ResponseWithoutDefaults", "Mode::RenamedVariant"),
            ):
                path = root / source
                original = path.read_text()
                self.assertIn(old, original)
                path.write_text(original.replace(old, new))
                with self.subTest(source=source, declaration=old), self.assertRaises(ValueError):
                    response_defaults.verify_rust_additions(root)
                path.write_text(original)

    def test_supplied_branch_pin_matches_the_retained_plan_scan(self):
        status = json.loads((ROOT / "docs/phase-status.json").read_text())
        phase = next(p for p in status["phases"] if p["id"] == 90)
        scan = json.loads((ROOT / "docs/checks/phase-90-private-material.json").read_text())
        self.assertEqual(phase["scope_evidence"]["source_commit"], scan["headCommit"])
        self.assertEqual(scan["headCommit"], "a6d63e58f067a98504e5b79987639d53e11fc931")

    def test_default_inventory_and_documentation_are_reproducible(self):
        expected = response_defaults.build()
        self.assertEqual(json.loads((ROOT / response_defaults.OUTPUT).read_text()), expected)
        self.assertEqual((ROOT / response_defaults.DOC).read_text(), response_defaults.render(expected))
        self.assertEqual(expected["counts"], {
            "defaultBindings": 23, "redTeamBindings": 8, "outsideRedTeamBindings": 15,
        })
        self.assertEqual(len(expected["rustOnlyAdditions"]), 3)
        self.assertTrue(all(row["uses"] for row in expected["bindings"]))
        self.assertEqual(len({row["binding"] for row in expected["bindings"]}), 23)

    def test_patch_keeps_the_published_dependency_and_feature_contract(self):
        manifest = tomllib.loads((ROOT / "crates/scm-sdk/Cargo.toml").read_text())
        if manifest["package"]["version"] != "0.1.1":
            self.skipTest("This dependency freeze applies to the historical 0.1.1 patch")
        lock = tomllib.loads((ROOT / "Cargo.lock").read_text())
        self.assertEqual(canonical_sha(manifest["dependencies"]),
                         "1de1e4a6ed28c4d49b71cd5585f63f098a42ce5f6d42c3d4317d8b072094f35b")
        self.assertEqual(canonical_sha(manifest["features"]),
                         "ee22fe9d2878d5837c82502c9d716939a4d968fef87aa18a8590e752531300a3")
        self.assertEqual(canonical_sha([p for p in lock["package"] if p.get("source")]),
                         "b65607fe9e4c07fc3c6b5bc7fe7be964d9f789cd5665baeec37a41588fcc6d79")
        workspace = tomllib.loads((ROOT / "Cargo.toml").read_text())
        self.assertEqual(workspace["workspace"]["package"]["rust-version"], "1.89")
        mode = (ROOT / "crates/scm-sdk/src/validation.rs").read_text()
        self.assertRegex(mode, r"#\[non_exhaustive\]\s*pub enum Mode")

    def test_current_version_statements_are_bound_independently_of_publication(self):
        version = tomllib.loads((ROOT / "crates/scm-sdk/Cargo.toml").read_text())["package"]["version"]
        self.assertIn("Crate version: **" + version + "**", (ROOT / "README.md").read_text())
        self.assertIn("workspace version is **" + version + "**", (ROOT / "docs/RELEASE.md").read_text())
        self.assertIn("current Rust crate version is **" + version + "**", (ROOT / "docs/STABILITY.md").read_text())
        headings = [line for line in (ROOT / "CHANGELOG.md").read_text().splitlines()
                    if line.startswith("## ")]
        # Pending additive work may precede the current version without claiming publication.
        if headings and headings[0].startswith("## Unreleased — "):
            headings = headings[1:]
        self.assertTrue(headings)
        self.assertTrue(headings[0].startswith("## " + version + " — "), headings[0])


class PublicationProofTests(unittest.TestCase):
    def setUp(self):
        self.files = {
            "Cargo.toml": b'[package]\nname="scm-sdk"\nversion="0.1.1"\n',
            "Cargo.lock": b"version=4\n", "README.md": b"unit fixture",
            "LICENSE": b"unit fixture", "NOTICE": b"unit fixture",
            "src/lib.rs": b"// unit fixture", "src/fixtures/model-regressions.json": b"{}",
            "examples/response_defaults.rs": b"// offline unit fixture",
            ".cargo_vcs_info.json": json.dumps({
                "git": {"sha1": "a" * 40}, "path_in_vcs": "crates/scm-sdk",
            }).encode(),
        }
        self.archive = self.pack(self.files)
        commands = [
            ["package", "-p", "scm-sdk"], ["test", "--lib", "--no-default-features"],
            ["test", "--lib", "--all-features"], ["check", "--examples", "--all-features"],
            ["test", "--doc", "--all-features"],
            ["run", "--example", "response_defaults", "--all-features"],
            ["run", "--locked", "--offline"],
        ]
        self.proof = {
            "schemaVersion": 2, "passed": True, "requireClean": True, "effectiveDirty": False,
            "releaseCommit": "a" * 40, "toolchain": "1.99.0",
            "rustcVersion": "rustc 1.99.0 (synthetic unit fixture)",
            "archive_sha256": publication.sha(self.archive), "archive_bytes": len(self.archive),
            "files": publication.file_rows(self.files),
            "vcs": json.loads(self.files[".cargo_vcs_info.json"]),
            "checks": [{"command": ["cargo", "+1.99.0", *args], "exit_code": 0} for args in commands],
        }
        self.metadata = {"version": {
            "num": "0.1.1", "crate": "scm-sdk", "yanked": False,
            "created_at": "2026-10-18T00:00:00Z", "checksum": self.proof["archive_sha256"],
        }}
        self.owners = {"users": [{"login": "cdot65"}]}

    def pack(self, files, version="0.1.1"):
        output = io.BytesIO()
        with tarfile.open(fileobj=output, mode="w:gz") as archive:
            for name, data in sorted(files.items()):
                member = tarfile.TarInfo("scm-sdk-" + version + "/" + name)
                member.size = len(data)
                archive.addfile(member, io.BytesIO(data))
        return output.getvalue()

    def verify(self, proof=None, metadata=None, owners=None, archive=None, msrv=None):
        return publication.verify("0.1.1", self.proof if proof is None else proof,
                                  self.metadata if metadata is None else metadata,
                                  self.owners if owners is None else owners,
                                  self.archive if archive is None else archive, msrv)

    def test_exact_archive_and_identical_msrv_payload_are_accepted(self):
        msrv = copy.deepcopy(self.proof)
        msrv["toolchain"] = "1.89.0"
        msrv["rustcVersion"] = "rustc 1.89.0 (synthetic unit fixture)"
        for check in msrv["checks"]:
            check["command"][1] = "+1.89.0"
        msrv["archive_sha256"] = "b" * 64  # Cargo archive metadata may differ.
        result = self.verify(msrv=msrv)
        self.assertEqual(result["sourceCommit"], "a" * 40)
        self.assertTrue(result["identicalFileContentsQualifiedOnMsrv"])

    def test_dirty_failed_partial_and_unexecuted_proofs_are_rejected(self):
        for malformed in [None, [], {}, dict(self.proof, toolchain=[]),
                          dict(self.proof, checks=None), dict(self.proof, releaseCommit=None)]:
            with self.subTest(malformed_type=type(malformed).__name__), self.assertRaises(ValueError):
                publication.require_proof(malformed)
        for field, value in [("requireClean", False), ("effectiveDirty", True), ("passed", False)]:
            proof = copy.deepcopy(self.proof)
            proof[field] = value
            with self.subTest(field=field), self.assertRaises(ValueError):
                self.verify(proof=proof)
        for field in ["missing", "failed", "empty", "example"]:
            proof = copy.deepcopy(self.proof)
            if field == "missing":
                proof["checks"].pop()
            elif field == "failed":
                proof["checks"][0]["exit_code"] = 1
            elif field == "empty":
                proof["checks"] = []
            else:
                proof["checks"] = [c for c in proof["checks"] if "--example" not in c["command"]]
            with self.subTest(field=field), self.assertRaises(ValueError):
                self.verify(proof=proof)

    def test_wrong_owner_version_checksum_and_files_are_rejected(self):
        with self.assertRaises(ValueError):
            self.verify(owners={"users": [{"login": "someone-else"}]})
        for key, value in [("num", "0.1.2"), ("crate", "other"), ("checksum", "b" * 64), ("yanked", True)]:
            metadata = copy.deepcopy(self.metadata)
            metadata["version"][key] = value
            with self.subTest(key=key), self.assertRaises(ValueError):
                self.verify(metadata=metadata)
        proof = copy.deepcopy(self.proof)
        proof["files"][0]["sha256"] = "b" * 64
        with self.assertRaises(ValueError):
            self.verify(proof=proof)
        msrv = copy.deepcopy(self.proof)
        msrv["toolchain"] = "1.89.0"
        msrv["rustcVersion"] = "rustc 1.89.0 (synthetic unit fixture)"
        for check in msrv["checks"]:
            check["command"][1] = "+1.89.0"
        msrv["releaseCommit"] = "b" * 40
        with self.assertRaises(ValueError):
            self.verify(msrv=msrv)
        mismatched = copy.deepcopy(self.proof)
        mismatched["toolchain"] = "1.89.0"
        with self.assertRaises(ValueError):
            self.verify(msrv=mismatched)

    def test_dirty_embedded_vcs_is_rejected_even_with_resigned_hashes(self):
        files = copy.deepcopy(self.files)
        vcs = json.loads(files[".cargo_vcs_info.json"])
        vcs["git"]["dirty"] = True
        files[".cargo_vcs_info.json"] = json.dumps(vcs).encode()
        archive = self.pack(files)
        proof, metadata = copy.deepcopy(self.proof), copy.deepcopy(self.metadata)
        proof.update(archive_sha256=publication.sha(archive), archive_bytes=len(archive),
                     files=publication.file_rows(files))
        metadata["version"]["checksum"] = proof["archive_sha256"]
        with self.assertRaises(ValueError):
            self.verify(proof=proof, metadata=metadata, archive=archive)

    def test_package_delta_accounts_for_every_changed_file(self):
        baseline = copy.deepcopy(self.files)
        baseline["Cargo.toml"] = baseline["Cargo.toml"].replace(b"0.1.1", b"0.1.0")
        del baseline["examples/response_defaults.rs"]
        baseline["src/old.rs"] = b"// old unit fixture"
        result = publication.package_delta(self.pack(baseline, "0.1.0"), self.archive, "0.1.1")
        self.assertEqual({(c["path"], c["change"]) for c in result["changes"]}, {
            ("Cargo.toml", "changed"), ("examples/response_defaults.rs", "added"),
            ("src/old.rs", "removed"),
        })
