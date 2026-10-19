"""Guard response-floor selection and exhaustive compatibility corpus coverage."""

import copy
from contextlib import contextmanager
import json
import pathlib
import unittest
from unittest.mock import patch
import g25_fixtures
import red_team_dataplane as vendor


class G25GenerationTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        _, _, cls.ops, _, cls.graph, _ = vendor.contract()

    def test_each_existing_operation_has_an_explicit_mode_switch(self):
        baselines = vendor.response_baselines(self.ops)
        self.assertEqual(len(baselines), 36)
        for operation, baseline in baselines.items():
            prefix = vendor.PREFIX + operation
            wrapper = self.graph["nodes"][self.graph["roots"][prefix + ":response"]]
            self.assertEqual(wrapper["kind"], "ZodResponseCompatibility")
            self.assertEqual(
                wrapper["request"]["$ref"],
                self.graph["roots"][prefix + ":strict-response"],
            )
            if baseline["array"]:
                target = self.graph["nodes"][wrapper["response"]["$ref"]]
                self.assertEqual(
                    target, {"kind": "ZodArray", "element": {"$ref": baseline["root"]}}
                )
            elif baseline["opaque"]:
                target = self.graph["nodes"][wrapper["response"]["$ref"]]
                if baseline["expression"] == "z.unknown()":
                    self.assertEqual(target, {"kind": "ZodUnknown"})
                else:
                    self.assertEqual(
                        baseline["expression"], "z.object({}).passthrough()"
                    )
                    self.assertEqual(
                        set(target), {"kind", "properties", "unknownFields", "catchall"}
                    )
                    self.assertEqual(target["kind"], "ZodObject")
                    self.assertEqual(target["properties"], {})
                    self.assertEqual(target["unknownFields"], "passthrough")
                    self.assertEqual(
                        self.graph["nodes"][target["catchall"]["$ref"]],
                        {"kind": "ZodNever"},
                    )
            else:
                self.assertEqual(wrapper["response"]["$ref"], baseline["root"])

    def test_documented_audit_matches_committed_response_floors(self):
        audit = json.loads(
            (vendor.ROOT / "contracts/compatibility/g25/audit.json").read_text()
        )
        graph = json.loads(
            (
                vendor.ROOT / "crates/scm-sdk/src/validation/schema_graph.json"
            ).read_text()
        )
        paired = [row for row in audit["operations"] if row["baselineOperation"]]
        self.assertEqual(len(paired), 36)
        for row in paired:
            with self.subTest(operation=row["operationId"]):
                root = row["responseRoot"]
                wrapper = graph["nodes"][graph["roots"][root]]
                self.assertEqual(wrapper["kind"], "ZodResponseCompatibility")
                strict_root = root.removesuffix(":response") + ":strict-response"
                self.assertEqual(
                    wrapper["request"]["$ref"], graph["roots"][strict_root]
                )
                response = wrapper["response"]["$ref"]
                if row["baselineArray"]:
                    node = graph["nodes"][response]
                    self.assertEqual(
                        node,
                        {"kind": "ZodArray", "element": {"$ref": row["baselineRoot"]}},
                    )
                elif row["baselineOpaque"]:
                    self.assertIsNone(row["baselineRoot"])
                    node = graph["nodes"][response]
                    if row["baselineExpression"] == "z.unknown()":
                        self.assertEqual(node, {"kind": "ZodUnknown"})
                    else:
                        self.assertEqual(
                            row["baselineExpression"], "z.object({}).passthrough()"
                        )
                        self.assertEqual(node["kind"], "ZodObject")
                        self.assertEqual(node["properties"], {})
                        self.assertEqual(node["unknownFields"], "passthrough")
                        self.assertEqual(
                            graph["nodes"][node["catchall"]["$ref"]],
                            {"kind": "ZodNever"},
                        )
                else:
                    self.assertEqual(response, row["baselineRoot"])

    def test_incomplete_mapping_fails_closed(self):
        with patch.object(vendor, "bindings", return_value={}):
            with self.assertRaises(KeyError):
                vendor.response_baselines(self.ops)

    @contextmanager
    def patched_mapping_inputs(self, transform):
        native_bindings = vendor.bindings(self.ops)
        original = pathlib.Path.read_text

        def read(path, *args, **kwargs):
            text = original(path, *args, **kwargs)
            relative = path.relative_to(vendor.ROOT).as_posix()
            if relative == "contracts/operations.json" or relative.startswith(
                "contracts/native-operation-mappings/red-team"
            ):
                data = json.loads(text)
                transform(relative, data)
                return json.dumps(data)
            return text

        with patch.object(pathlib.Path, "read_text", read), patch.object(
            vendor, "bindings", return_value=native_bindings
        ):
            yield

    def test_ambiguous_mapping_fails_closed(self):
        paired = next(iter(vendor.response_baselines(self.ops).values()))[
            "baselineOperation"
        ]
        duplicated = False

        def transform(relative, data):
            nonlocal duplicated
            if not duplicated and relative.startswith(
                "contracts/native-operation-mappings/red-team"
            ):
                matching = next(
                    (row for row in data["operations"] if row["baselineId"] == paired),
                    None,
                )
                if matching is not None:
                    data["operations"].append(copy.deepcopy(matching))
                    duplicated = True

        with self.patched_mapping_inputs(transform):
            with self.assertRaisesRegex(
                ValueError, "Ambiguous TypeScript response mapping"
            ):
                vendor.response_baselines(self.ops)
        self.assertTrue(duplicated)

    def test_unclassified_response_expression_fails_closed(self):
        paired = next(iter(vendor.response_baselines(self.ops).values()))[
            "baselineOperation"
        ]

        def transform(relative, data):
            if relative == "contracts/operations.json":
                operation = next(
                    row for row in data["operations"] if row["id"] == paired
                )
                operation["directRequest"]["responseSchema"][
                    "expression"
                ] = "z.record(z.string())"

        with self.patched_mapping_inputs(transform):
            with self.assertRaisesRegex(
                ValueError, "Unclassified TypeScript response expression"
            ):
                vendor.response_baselines(self.ops)

    def test_missing_baseline_mapping_fails_coverage_guard(self):
        paired = next(iter(vendor.response_baselines(self.ops).values()))[
            "baselineOperation"
        ]

        def transform(relative, data):
            if relative.startswith("contracts/native-operation-mappings/red-team"):
                data["operations"] = [
                    row for row in data["operations"] if row["baselineId"] != paired
                ]

        with self.patched_mapping_inputs(transform):
            with self.assertRaisesRegex(
                ValueError, "TypeScript response operation coverage changed"
            ):
                vendor.response_baselines(self.ops)

    def test_paired_operation_without_json_response_fails_closed(self):
        pin, specification, operations = copy.deepcopy(vendor.load())
        paired = next(iter(vendor.response_baselines(operations)))
        operation = next(
            row for _, _, row in operations if row["operationId"] == paired
        )
        success = next(
            response
            for status, response in operation["responses"].items()
            if str(status).startswith("2")
        )
        success.pop("content", None)
        with patch.object(
            vendor, "load", return_value=(pin, specification, operations)
        ):
            with self.assertRaisesRegex(
                ValueError, "Paired operation has no JSON response root"
            ):
                vendor.contract()

    def test_all_json_roots_have_a_base_witness_and_independent_outcomes(self):
        corpus = g25_fixtures.generate()
        expected = {
            key for key in self.graph["roots"] if key.endswith(":strict-response")
        }
        self.assertEqual({r["strictRoot"] for r in corpus["roots"]}, expected)
        self.assertEqual(len(expected), 46)
        self.assertEqual(
            {c["root"] for c in corpus["cases"] if c["kind"] == "base"}, set(range(46))
        )
        for kind in (
            "format:uuid",
            "format:rfc3339",
            "format:duration",
            "enum",
            "presence",
            "bound:min",
            "bound:max",
        ):
            self.assertTrue(any(c["kind"] == kind for c in corpus["cases"]), kind)
        outcomes = json.loads(
            (vendor.ROOT / "contracts/compatibility/g25/outcomes.json").read_text()
        )["outcomes"]
        self.assertEqual(len(outcomes), len(corpus["cases"]))
        for case, outcome in zip(corpus["cases"], outcomes):
            if case["kind"] == "base":
                self.assertIs(outcome["strict"], True)
            if case["kind"].startswith("format:") or case["kind"] == "enum":
                self.assertTrue(outcome["response"])
            if outcome["typescript"] is True:
                self.assertTrue(outcome["response"])

    def test_unimplemented_string_witness_formats_fail_closed(self):
        graph_path = vendor.ROOT / "crates/scm-sdk/src/validation/schema_graph.json"
        graph = json.loads(graph_path.read_text())
        corpus = g25_fixtures.generate()
        string_node = next(
            case["node"]
            for case in corpus["cases"]
            if case["kind"].startswith("format:")
        )
        original = pathlib.Path.read_text
        for kind in ("email", "url", "regex", "datetime", "unknown-format"):
            changed = copy.deepcopy(graph)
            changed["nodes"][string_node]["checks"].append({"kind": kind})

            def read(path, *args, **kwargs):
                return (
                    json.dumps(changed)
                    if path == graph_path
                    else original(path, *args, **kwargs)
                )

            with self.subTest(kind=kind), patch.object(pathlib.Path, "read_text", read):
                with self.assertRaisesRegex(ValueError, "No conforming string witness"):
                    g25_fixtures.generate()

    def test_frozen_string_bounds_remain_enforced(self):
        frozen = json.loads((vendor.ROOT / "contracts/typescript.json").read_text())[
            "schemas"
        ]["definitions"]
        graph = json.loads(
            (
                vendor.ROOT / "crates/scm-sdk/src/validation/schema_graph.json"
            ).read_text()
        )
        checked = 0
        for sid in frozen:
            node = graph["nodes"].get(sid, {})
            if node.get("kind") == "ZodString":
                for check in node.get("checks", []):
                    if check["kind"] in ("min", "max"):
                        self.assertNotIn("requestOnly", check)
                        checked += 1
        self.assertEqual(checked, 173)

    def test_disputed_structural_cases_remain_rejected(self):
        corpus = json.loads(
            (vendor.ROOT / "contracts/compatibility/g25/cases.json").read_text()
        )
        outcomes = json.loads(
            (vendor.ROOT / "contracts/compatibility/g25/outcomes.json").read_text()
        )["outcomes"]
        root = (
            vendor.PREFIX
            + "list_attacks_v1_report_static__job_id__list_attacks_get:response"
        )
        for field, kind in [
            ("category_display_name", "presence"),
            ("tsg_id", "type:number"),
            ("sub_category", "type:null"),
        ]:
            matches = [
                i
                for i, case in enumerate(corpus["cases"])
                if corpus["roots"][case["root"]]["root"] == root
                and case["path"] == ["data", 0, field]
                and case["kind"] == kind
            ]
            self.assertTrue(matches, (root, field, kind))
            for index in matches:
                self.assertEqual(
                    outcomes[index],
                    {"strict": False, "typescript": False, "response": False},
                )

    def test_projection_pairing_must_match_explicit_reviewed_bindings(self):
        path = vendor.ROOT / "contracts/compatibility/g25/projection-bindings.json"
        bindings = json.loads(path.read_text())
        self.assertEqual(len(bindings["additions"]), 55)
        self.assertEqual(
            len({(row["legacySchema"], row["field"]) for row in bindings["additions"]}),
            55,
        )
        original = pathlib.Path.read_text
        for change in ("missing", "vendor", "legacy"):
            changed = copy.deepcopy(bindings)
            if change == "missing":
                changed["additions"].pop()
            else:
                changed["additions"][0][change + "Schema"] = "unexpected-schema"

            def read(current, *args, **kwargs):
                return (
                    json.dumps(changed)
                    if current == path
                    else original(current, *args, **kwargs)
                )

            with self.subTest(change=change), patch.object(
                pathlib.Path, "read_text", read
            ):
                with self.assertRaisesRegex(
                    ValueError, "Reviewed legacy/vendor projection bindings changed"
                ):
                    g25_fixtures.projections()

    def test_complete_projection_usage_partition_is_pinned(self):
        # Deliberate request serialization behavior must be visible for every
        # added field, including shared request/response and standalone models.
        expected = {
            "request-and-response": [
                ("LanguageOption", "country_code"),
                ("TargetMetadata", "audio_format"),
                ("TargetMetadata", "audio_multi_turn_probe_error"),
                ("TargetMetadata", "audio_multi_turn_probe_passed"),
                ("TargetMetadata", "audio_probe_error"),
                ("TargetMetadata", "audio_probe_passed"),
                ("TargetMetadata", "supports_audio"),
                ("TargetMetadata", "supports_text"),
            ],
            "request-only": [
                ("JobCreateRequestJobMetadataVariant1", "include_audio_attacks"),
                ("JobCreateRequestJobMetadataVariant1", "include_text_attacks"),
                ("JobCreateRequestJobMetadataVariant1", "runtime_profile_id"),
                ("JobCreateRequestJobMetadataVariant2", "multi_turn_enabled"),
                ("JobCreateRequestJobMetadataVariant2", "runtime_profile_id"),
                ("JobCreateRequestJobMetadataVariant3", "runtime_profile_id"),
            ],
            "response-only": [
                ("AttackDetailResponse", "audio"),
                ("AttackDetailResponse", "audio_technique"),
                ("AttackDetailResponse", "threat_details"),
                ("AttackMultiTurnDetailResponse", "audio"),
                ("AttackMultiTurnDetailResponse", "audio_technique"),
                ("AttackMultiTurnDetailResponse", "threat_details"),
                ("AttackMultiTurnOutput", "judge_reasoning"),
                ("AttackMultiTurnOutput", "response_audio"),
                ("AttackMultiTurnOutput", "threat_details"),
                ("AttackMultiTurnOutput", "transcript"),
                ("AttackMultiTurnOutput", "transcript_source"),
                ("AttackOutput", "judge_reasoning"),
                ("AttackOutput", "response_audio"),
                ("AttackOutput", "threat_details"),
                ("AttackOutput", "transcript"),
                ("AttackOutput", "transcript_source"),
                ("CustomAttackOutput", "judge_reasoning"),
                ("CustomAttackOutput", "threat_details"),
                ("CustomJobMetadata", "runtime_profile_id"),
                ("DynamicJobMetadata", "multi_turn_enabled"),
                ("DynamicJobMetadata", "runtime_profile_id"),
                ("Goal", "goal_english"),
                ("Goal", "goal_subtype"),
                ("JobResponse", "progress"),
                ("JobResponse", "runtime_metrics"),
                ("PromptDetailResponse", "threat_details"),
                ("PromptSetSummary", "errored_attacks"),
                ("PromptSetSummary", "has_errors"),
                ("RemediationDetail", "categories"),
                ("ScoreTrendResponse", "iso_labels"),
                ("StaticJobMetadata", "include_audio_attacks"),
                ("StaticJobMetadata", "include_text_attacks"),
                ("StaticJobMetadata", "runtime_profile_id"),
                ("StreamDetailResponse", "threat_details"),
                ("StreamDetailResponseGoalVariant2", "goal_subtype"),
                ("StreamIterationData", "judge_explanation"),
                ("StreamIterationData", "threat_details"),
                ("SubCategoryModel", "audio_supported"),
                ("SubCategoryStats", "audio_supported"),
            ],
            "standalone-schema": [
                ("ValidationError", "ctx"),
                ("ValidationError", "input"),
            ],
        }
        rows = g25_fixtures.projections()
        actual = {}
        identities = set()
        for row in rows:
            identity = (row["rustType"], row["field"])
            self.assertNotIn(identity, identities)
            identities.add(identity)
            actual.setdefault(row["usage"], []).append(identity)
        actual = {key: sorted(values) for key, values in actual.items()}
        self.assertEqual(len(rows), 55)
        self.assertEqual(len(identities), 55)
        self.assertEqual(actual, expected)

    def test_vendor_string_bounds_are_request_only(self):
        for node in self.graph["nodes"].values():
            if node["kind"] == "ZodString":
                for check in node.get("checks", []):
                    if check["kind"] in ("min", "max"):
                        self.assertIs(check.get("requestOnly"), True)
        rows = g25_fixtures.projections()
        self.assertEqual(len(rows), 55)
        request_models = {r["rustType"] for r in rows if r["usage"] == "request-only"}
        self.assertEqual(
            request_models,
            {
                "JobCreateRequestJobMetadataVariant1",
                "JobCreateRequestJobMetadataVariant2",
                "JobCreateRequestJobMetadataVariant3",
            },
        )
        self.assertTrue(
            all(
                "conforming" in row and isinstance(row["nullable"], bool)
                for row in rows
            )
        )
        corpus = g25_fixtures.generate()
        skipped = corpus["skippedBranches"]
        self.assertTrue(skipped)
        for row in skipped:
            self.assertEqual(set(row), {"root", "path", "node", "choices", "reason"})
            self.assertEqual(row["reason"], "null-branch-covered-by-type:null")
            self.assertTrue(
                any(
                    c["kind"] == "type:null"
                    and corpus["roots"][c["root"]]["root"] == row["root"]
                    and c["path"] == row["path"]
                    and c["choices"] == row["choices"]
                    for c in corpus["cases"]
                ),
                row,
            )


if __name__ == "__main__":
    unittest.main()
