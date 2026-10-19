"""Keep the current service narrative tied to its actual receipt."""
import json
import pathlib
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]


class LiveAcceptanceDocsTests(unittest.TestCase):
    def test_current_read_only_summary_matches_selected_receipt(self):
        reference = json.loads((ROOT / "contracts/live-evidence.json").read_text())
        report = json.loads((ROOT / reference["report"]).read_text())["report"]
        text = (ROOT / "docs/LIVE_ACCEPTANCE.md").read_text()
        current = text.split("## Current 0.2.3 read-only qualification", 1)[1].split("## Historical qualification", 1)[0]
        self.assertIn(reference["report"].removeprefix("docs/"), current)
        accepted = {row["operation"] for row in report["probes"] if row["workflowAccepted"]}
        self.assertIn(f"**{len(accepted)} exact native method**", current)
        target_probes = [row for row in report["probes"] if row["operation"].startswith("red_team.targets.list.")]
        self.assertEqual(len(target_probes), 4)
        self.assertTrue(all(row["classification"] == "permission_prerequisite" and row["status"] == 403 for row in target_probes))
        self.assertIn("HTTP 403 permission prerequisites", current)
        self.assertIn("cleanupState: " + report["cleanupState"], current)
        self.assertIn("neither\nfilter acceptance nor effectiveness", current)

    def test_current_profile_and_historical_boundary_are_explicit(self):
        text = (ROOT / "docs/LIVE_ACCEPTANCE.md").read_text()
        current, historical = text.split("## Historical qualification", 1)
        for profile in ("https://api.sase.paloaltonetworks.com/ai-red-teaming/mgmt",
                        "https://api.apps.paloaltonetworks.com/ai-red-teaming/mgmt-plane"):
            self.assertIn(profile, current)
        self.assertIn("was not exercised", current)
        self.assertIn("../contracts/historical/0.2.2-live-evidence.json", historical)
        self.assertIn("verified **17 exact native methods**", historical)
        self.assertNotIn("current **0.2.2**", text)


if __name__ == "__main__":
    unittest.main()
