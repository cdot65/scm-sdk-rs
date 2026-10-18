"""Adversarial guards for the independently versioned vendor adapter."""
import copy
import json
import pathlib
import unittest

import red_team_dataplane as vendor


class VendorAdapterTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.pin, cls.spec, cls.ops, cls.compiler, cls.graph, cls.metadata = vendor.contract()

    def test_all_operations_have_exact_native_binding_and_source(self):
        bindings = vendor.bindings(self.ops)
        self.assertEqual(len(bindings), self.pin["expectedOperations"])
        self.assertEqual(len(vendor.NEW), 11)
        for method, path, op in self.ops:
            self.assertIn(method + " " + path, bindings)
            self.assertEqual(self.metadata[op["operationId"]]["path"], path)
            self.assertEqual(self.metadata[op["operationId"]]["method"], method)
        with self.assertRaises(ValueError):
            vendor.bindings(self.ops[:-1])

    def test_source_and_notice_provenance_are_complete(self):
        self.assertTrue(self.pin["redistributed"])
        self.assertTrue(self.pin["usedForGeneration"])
        notice = vendor.ROOT / self.pin["licensePath"]
        self.assertEqual(vendor.sha(notice.read_bytes()), self.pin["licenseSha256"])
        for name in ["NOTICE", "crates/scm-sdk/NOTICE"]:
            self.assertIn(notice.read_text(), (vendor.ROOT/name).read_text())

    def test_no_frozen_graph_node_or_root_is_changed(self):
        import generate_models
        frozen = generate_models.compact_graph(json.loads(generate_models.SOURCE.read_text()))
        merged = vendor.extend_graph(frozen, self.graph)
        for key, value in frozen["nodes"].items():
            self.assertEqual(merged["nodes"][key], value)
        for key, value in frozen["roots"].items():
            self.assertEqual(merged["roots"][key], value)
        with self.assertRaises(ValueError):
            vendor.extend_graph(frozen, frozen)

    def test_only_four_reviewed_new_response_presence_waivers_exist(self):
        diff = vendor.changed_contracts(self.spec)
        actual = {(row["schema"], row["field"]) for row in diff["responsePresenceWaivers"]}
        self.assertEqual(actual, vendor.WAIVERS)
        changed = copy.deepcopy(self.spec)
        changed["components"]["schemas"]["JobResponseSchema"]["required"].append("runtime_metrics")
        with self.assertRaisesRegex(ValueError, "fresh Claude review"):
            vendor.changed_contracts(changed)

    def test_unknown_keywords_remote_refs_recursive_and_ambiguous_unions_fail(self):
        for schema in [{"type":"string","pattern":"^x$"},
                       {"$ref":"https://untrusted.example/schema.json"},
                       {"$ref":"#/components/schemas/ReportState","maxLength":10},
                       {"oneOf":[{"type":"string"},{"type":"integer"}]}]:
            with self.subTest(schema=schema), self.assertRaises(ValueError):
                self.compiler.compile(schema, "#/negative/" + vendor.sha(vendor.canonical(schema).encode()))
        cycle = {"components":{"schemas":{"Cycle":{"type":"object","properties":{"cycle":{"$ref":"#/components/schemas/Cycle"}}}}}}
        with self.assertRaisesRegex(ValueError,"Recursive"):
            vendor.Compiler(cycle)

    def test_same_pointer_cannot_silently_reuse_different_schema(self):
        self.compiler.compile({"type":"string"}, "#/negative/collision")
        with self.assertRaisesRegex(ValueError,"different input"):
            self.compiler.compile({"type":"integer"}, "#/negative/collision")

    def test_all_unsupported_parameters_are_closed_pre_auth_errors(self):
        table = vendor.outputs()[vendor.ROOT/"contracts/vendor/red-team-data-plane/0.93.0/operations.json"]
        seen = {}
        for row in table["operations"]:
            if not row["rejectedLegacyParameters"]:
                continue
            seen[row["nativeMethod"]] = [p["name"] for p in row["rejectedLegacyParameters"]]
            query = self.graph["nodes"][self.graph["roots"][vendor.PREFIX+row["operationId"]+":query"]]
            self.assertEqual(query["unknownFields"], "strict")
            for item in row["rejectedLegacyParameters"]:
                self.assertNotIn(item["name"], query["properties"])
                self.assertEqual(item["disposition"], "validation_error_before_auth")
        self.assertEqual(seen, vendor.REJECTED)


if __name__ == "__main__":
    unittest.main()
