"""Independent adversarial guards for method-local Gateway source bindings."""
import copy
import json
import pathlib
import unittest

import verify_gateway_bindings as verifier


class GatewayBindingTests(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        root = pathlib.Path(__file__).resolve().parents[1]
        cls.operations = json.loads((root / "contracts/operations.json").read_text())["operations"]
        cls.native = {row["baselineId"]: row for row in json.loads((root / "contracts/native-operations.json").read_text())["operations"]}
        cls.schemas = json.loads((root / "contracts/typescript.json").read_text())
        cls.root = root

    def evidence(self, suffix):
        operation = next(row for row in self.operations if row["id"].endswith(suffix))
        mapping = self.native[operation["id"]]
        source = (self.root / mapping["source"]).read_text()
        return operation, mapping, source

    def rejected(self, suffix, old, new):
        operation, mapping, source = self.evidence(suffix)
        self.assertIn(old, source)
        with self.assertRaises(ValueError):
            verifier.validate_operation(operation, mapping, source.replace(old, new), self.schemas)

    def test_all_194_frozen_methods_have_explicit_source_proofs(self):
        result, failures = verifier.generate()
        self.assertEqual(failures, [])
        self.assertEqual(result["count"], 194)
        self.assertEqual(len({row["baselineId"] for row in result["operations"]}), 194)
        self.assertTrue(all(len(row["nativeRouteTemplates"]) == 1 for row in result["operations"]))

    def test_wrong_response_schema_is_rejected(self):
        self.rejected("AIGatewayRateLimitsClient.get", '"GatewayRateLimitsClientGetResponseSchema"', '"GatewayWriteResponseSchema"')

    def test_comment_cannot_supply_a_missing_schema(self):
        self.rejected("AIGatewayRateLimitsClient.get", '"GatewayRateLimitsClientGetResponseSchema"',
                      '"GatewayWriteResponseSchema" /* "GatewayRateLimitsClientGetResponseSchema" */')

    def test_wrong_request_schema_is_rejected(self):
        self.rejected("AIGatewayConfigsClient.update", '"GatewayConfigUpdateRequestSchema"', '"GatewayConfigCreateRequestSchema"')

    def test_wrong_http_method_is_rejected(self):
        self.rejected("AIGatewayConfigsClient.update", "Method::PUT", "Method::PATCH")

    def test_wrong_literal_route_is_rejected(self):
        self.rejected("AIGatewayConfigsClient.get", '"/configs"', '"/providers"')

    def test_literal_in_unrelated_method_cannot_supply_route_evidence(self):
        operation, mapping, source = self.evidence("AIGatewayConfigsClient.get")
        source = source.replace('"/configs"', '"/providers"')
        source += '\nfn unrelated() { let _ = "/configs"; }\n'
        with self.assertRaises(ValueError):
            verifier.validate_operation(operation, mapping, source, self.schemas)

    def test_parameter_replaced_by_a_literal_is_rejected(self):
        self.rejected("AIGatewayConfigsClient.get", ".segment(config_id)", '.segment("hardcoded-id")')

    def test_extra_segment_is_rejected(self):
        self.rejected("AIGatewayConfigsClient.get", ".segment(config_id)?", '.segment(config_id)?.segment("extra")?')

    def test_runtime_literal_suffix_is_rejected(self):
        self.rejected("AIGatewayRuntimeResourcesClient.downloadFile", '"content"', '"other"')

    def test_chart_metric_cannot_silently_use_another_response(self):
        self.rejected("AIGatewayTelemetryClient.cost", '"/logs/charts/cost"', '"/logs/charts/tokens"')

    def test_composed_filter_field_identity_is_checked(self):
        operation, mapping, source = self.evidence("AIGatewayTelemetryClient.cost")
        schemas = copy.deepcopy(self.schemas)
        roots = {row["name"]: row for row in schemas["schemas"]["roots"] if row["source"]["file"] == "src/ai-gateway/telemetry-client.ts"}
        definitions = schemas["schemas"]["definitions"]
        effect = definitions[roots["filteredChartOptionsSchema"]["schema"]["$ref"]]
        fields = definitions[effect["inner"]["$ref"]]["properties"]
        fields["costMin"]["schema"]["$ref"] = fields["traceId"]["schema"]["$ref"]
        with self.assertRaisesRegex(ValueError, "decomposition changed"):
            verifier.validate_operation(operation, mapping, source, schemas)

    def test_unreviewed_filter_refinement_is_rejected(self):
        operation, mapping, source = self.evidence("AIGatewayTelemetryClient.cost")
        schemas = copy.deepcopy(self.schemas)
        root = next(row for row in schemas["schemas"]["roots"] if row["name"] == "AIGatewayChartFiltersSchema")
        schemas["schemas"]["definitions"][root["schema"]["$ref"]]["semanticSource"]["source"]["text"] = "z.unknown()"
        with self.assertRaisesRegex(ValueError, "refinement decomposition changed"):
            verifier.validate_operation(operation, mapping, source, schemas)


if __name__ == "__main__":
    unittest.main()
