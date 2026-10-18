"""Guard the explicit release scope; publication remains an executed action."""
import json
import pathlib
import subprocess
import unittest
import importlib.util
import tempfile

from check_phase import commands_for

ROOT = pathlib.Path(__file__).resolve().parents[1]
BASELINE = "167a359fb3d51aad37c4dff311060b069c620bdd"


class ReleasePlanTests(unittest.TestCase):
    def test_plan_has_release_scope_and_separate_gateway_followup(self):
        plan = (ROOT / "docs/release-0.2.0-plan.md").read_text()
        for value in (BASELINE, "**0.2.0**", "**cdot65**", "v0.2.0", "1.89.0", "1.99.0",
                      "a27feb9efaac3653d2d94eca5d7d210944b6e160", ">=9/10",
                      "identical", "403", "completely clean", "downloaded artifact"):
            self.assertIn(value, plan)

    def test_previously_accepted_command_sets_remain_identical(self):
        for phase in (9, 10, 11, 12, 90, 91, 92):
            report = json.loads((ROOT / f"docs/checks/phase-{phase:02d}.json").read_text())
            self.assertEqual(commands_for(phase), [x["command"] for x in report["checks"]])

    def test_planning_has_no_runtime_or_packaged_implementation_changes(self):
        paths = ["Cargo.toml", "Cargo.lock", "LICENSE", "NOTICE",
                 "crates/scm-sdk/Cargo.toml", "crates/scm-sdk/src", "crates/scm-sdk/examples"]
        self.assertEqual(subprocess.check_output(
            ["git", "diff", "--name-only", BASELINE, "--", *paths], cwd=ROOT, text=True), "")

    def test_completed_delivery_does_not_reopen_maintenance_track(self):
        from review_phase import next_phase
        p = json.loads((ROOT / "docs/phase-status.json").read_text())
        phases = {row["id"]: row for row in p["phases"]}
        self.assertEqual(next_phase({13: {}, 14: {"final_in_sequence": True},
                                     90: {"status": "complete"}}, 13), 14)
        self.assertIsNone(next_phase(phases, 14))

    def test_candidate_detects_untracked_ignored_changed_and_symlink_inputs(self):
        path = ROOT / "scripts/check-release-candidate.py"
        if not path.exists():
            self.skipTest("Candidate guard is a Phase 14 deliverable")
        spec = importlib.util.spec_from_file_location("candidate", path)
        candidate = importlib.util.module_from_spec(spec)
        spec.loader.exec_module(candidate)
        with tempfile.TemporaryDirectory() as directory:
            root = pathlib.Path(directory)
            subprocess.run(["git", "init", "-q", str(root)], check=True)
            for relative in candidate.PROTECTED:
                p = root / relative
                if relative.endswith(("/src", "/examples")):
                    p = p / "fixture.rs"
                p.parent.mkdir(parents=True, exist_ok=True)
                p.write_text("accepted input")
            subprocess.run(["git", "add", "."], cwd=root, check=True)
            subprocess.run(["git", "-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid",
                            "commit", "-qm", "baseline"], cwd=root, check=True)
            baseline = candidate.git(root, "rev-parse", "HEAD").decode().strip()
            self.assertTrue(candidate.unchanged_runtime(root, baseline))
            extra = root / "crates/scm-sdk/src/extra.rs"
            extra.write_text("unexpected")
            with self.assertRaises(ValueError):
                candidate.unchanged_runtime(root, baseline)
            (root / ".gitignore").write_text("extra.rs\n")
            with self.assertRaises(ValueError):
                candidate.unchanged_runtime(root, baseline)
            extra.unlink()
            original = root / "crates/scm-sdk/src/fixture.rs"
            original.write_text("changed")
            with self.assertRaises(ValueError):
                candidate.unchanged_runtime(root, baseline)
            original.write_text("accepted input")
            extra.symlink_to(original)
            with self.assertRaises(ValueError):
                candidate.unchanged_runtime(root, baseline)


if __name__ == "__main__":
    unittest.main()
