"""Guard the explicit release scope; publication remains an executed action."""
import json
import pathlib
import subprocess
import unittest

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


if __name__ == "__main__":
    unittest.main()
