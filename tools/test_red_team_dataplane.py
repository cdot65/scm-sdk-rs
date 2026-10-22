"""Adversarial guards for the independently versioned vendor adapter."""
import copy
import json
import pathlib
import tomllib
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

    def test_vendor_update_preserves_msrv_dependencies_features_and_migration_inventory(self):
        manifest=tomllib.loads((vendor.ROOT/'crates/scm-sdk/Cargo.toml').read_text())
        lock=tomllib.loads((vendor.ROOT/'Cargo.lock').read_text())
        if manifest['package']['version'].startswith('0.2.'):
            self.assertEqual(vendor.sha(vendor.canonical(manifest['dependencies']).encode()),'1de1e4a6ed28c4d49b71cd5585f63f098a42ce5f6d42c3d4317d8b072094f35b')
            self.assertEqual(vendor.sha(vendor.canonical(manifest['features']).encode()),'ee22fe9d2878d5837c82502c9d716939a4d968fef87aa18a8590e752531300a3')
            # Reviewed TLS security patches only; the directional invariant test
            # separately compares every package against the historical lockfile.
            self.assertEqual(vendor.sha(vendor.canonical([p for p in lock['package'] if p.get('source')]).encode()),'d96eaa5b52d666c9028773aa3091424525afdcb7fb43ddfa95713569cf479de6')
        workspace=tomllib.loads((vendor.ROOT/'Cargo.toml').read_text())
        self.assertEqual(workspace['workspace']['package']['rust-version'],'1.89')
        changelog=(vendor.ROOT/'CHANGELOG.md').read_text().split('## 0.1.1',1)[0]
        for member,fields in vendor.REJECTED.items():
            for field in fields:
                self.assertTrue(any(member in row and '`'+field+'`' in row for row in changelog.splitlines()),(member,field))

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
            seen[row["nativeMethod"]] = {p["name"] for p in row["rejectedLegacyParameters"]}
            query = self.graph["nodes"][self.graph["roots"][vendor.PREFIX+row["operationId"]+":query"]]
            self.assertEqual(query["unknownFields"], "strict")
            for item in row["rejectedLegacyParameters"]:
                self.assertNotIn(item["name"], query["properties"])
                self.assertEqual(item["disposition"], "validation_error_before_auth")
        self.assertEqual(seen, {name:set(fields) for name,fields in vendor.REJECTED.items()})

    def test_impl_probe_does_not_find_a_method_on_an_unrelated_client(self):
        source = 'impl OtherClient {\n    pub async fn get() {}\n}\nimpl ActualClient {\n}\n'
        self.assertNotIn('pub async fn get', vendor.client_impl(source, 'ActualClient'))
        self.assertIn('pub async fn get', vendor.client_impl(source, 'OtherClient'))

    def test_option_fields_are_derived_from_actual_public_struct(self):
        source = 'impl FixtureClient {\n    pub async fn list(&self, params: &RedTeamListOptions) -> Result<()> {}\n}\n'
        self.assertEqual(vendor.option_fields(source, 'FixtureClient', 'list'), {'skip','limit','search'})
        self.assertEqual(vendor.option_fields(source, 'OtherClient', 'list'), set())

    def test_nested_and_inline_new_required_fields_fail_closed(self):
        before={'type':'object','properties':{'nested':{'type':'object','properties':{'x':{'type':'string'}}}}}
        after=copy.deepcopy(before);after['properties']['nested']['required']=['x']
        self.assertEqual(vendor.required_changes(before,after,'#/inline'),{('#/inline/properties/nested','x')})
        changed=copy.deepcopy(self.spec)
        changed['paths']['/v1/scan']['post']['requestBody']['content']['application/json']['schema']['required']=['new_inline_field']
        with self.assertRaisesRegex(ValueError,'fresh Claude review'):
            vendor.changed_contracts(changed)

    def test_structural_changes_have_discriminated_reviewed_classes(self):
        changes=vendor.changed_contracts(self.spec)
        for row in changes['schemaChanges']+changes['operationChanges']:
            self.assertTrue(row['changeClasses'])
        changed=copy.deepcopy(self.spec)
        changed['components']['schemas']['JobCreateRequestSchema']['properties']['name']['maxLength']=254
        with self.assertRaisesRegex(ValueError,'Unreviewed vendor change classes'):
            vendor.changed_contracts(changed)

    def test_wire_bindings_fail_on_missing_extra_tampered_or_unexecutable_operations(self):
        mapping = vendor.bindings(self.ops)
        corpus = json.loads((vendor.ROOT/vendor.WIRE_FIXTURES).read_text())
        source = (vendor.ROOT/vendor.WIRE_TEST).read_text()
        bindings = vendor.wire_bindings(self.ops, mapping, corpus, source)
        self.assertEqual(set(bindings), {op['operationId'] for _,_,op in self.ops})
        for mutation in ('missing','extra','route','call','source','test'):
            changed=copy.deepcopy(corpus); test=source
            if mutation=='missing':changed['cases']=[r for r in changed['cases'] if r['operationId']!=changed['cases'][0]['operationId']]
            elif mutation=='extra':changed['cases'].append(dict(changed['cases'][0],operationId='invented_operation',name='extra'))
            elif mutation=='route':changed['cases'][0]['path']='/unrelated'
            elif mutation=='call':changed['cases'][0]['call']['member']='invented_method'
            elif mutation=='source':changed['sourceSha256']='0'*64
            else:test=source.replace('async fn call_vendor(', 'async fn missing_dispatch(')
            with self.subTest(mutation=mutation),self.assertRaises(ValueError):
                vendor.wire_bindings(self.ops,mapping,changed,test)

    def test_schema_roots_bind_to_the_exact_method_and_detect_transposed_overrides(self):
        op=next(op for _,_,op in self.ops if op['operationId'].startswith('threat_override_static_attack_'))
        cls,member='RedTeamReportsClient','override_attack'
        source=(vendor.ROOT/'crates/scm-sdk/src/airs/red_team/reports.rs').read_text()
        meta=self.metadata[op['operationId']]
        vendor.verify_schema_binding(source,cls,member,meta,op)
        for suffix in (':body',':response'):
            changed=source.replace(op['operationId']+suffix,'threat_override_static_multi_turn_v1_report_static__job_id__attack_multi_turn__attack_id__override_post'+suffix)
            with self.subTest(suffix=suffix),self.assertRaisesRegex(ValueError,'schema roots differ'):
                vendor.verify_schema_binding(changed,cls,member,meta,op)
        embedded=vendor.outputs()[vendor.ROOT/'crates/scm-sdk/src/airs/red_team/contract.json']
        self.assertTrue(all(set(row)=={'queryKinds','querySchema'} for row in embedded.values()))


if __name__ == "__main__":
    unittest.main()
