#!/usr/bin/env python3
"""Adversarial development checks for native schema generation and API pins."""
import copy
import importlib.util
import json
import pathlib
import re
import unittest

ROOT=pathlib.Path(__file__).resolve().parents[1]
spec=importlib.util.spec_from_file_location("model_generation",ROOT/"tools/generate_models.py")
generator=importlib.util.module_from_spec(spec)
spec.loader.exec_module(generator)

class GenerationGuards(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.inventory=json.loads(generator.SOURCE.read_text())
        cls.pins=json.loads(generator.UNION_PINS.read_text())

    def changed(self):return copy.deepcopy(self.inventory)

    def test_current_nested_checks_and_record_keys_match_native_implementation(self):
        result=generator.validate_graph_contract(self.inventory)
        self.assertGreater(result["recordKeys"],0)
        self.assertEqual(len(result["regexSources"]),4)

    def test_unknown_nested_string_and_number_checks_fail(self):
        for kind in ["ZodString","ZodNumber"]:
            with self.subTest(kind=kind):
                inventory=self.changed()
                node=next(node for node in inventory["schemas"]["definitions"].values() if node["kind"]==kind)
                node.setdefault("checks",[]).append({"kind":"unimplemented_new_check"})
                with self.assertRaisesRegex(ValueError,"check kinds"):
                    generator.compact_graph(inventory)

    def test_unknown_nested_regex_or_flags_fail(self):
        for field,value in [("$regexp","^new-pattern$"),("flags","i")]:
            with self.subTest(field=field):
                inventory=self.changed()
                check=next(check for node in inventory["schemas"]["definitions"].values() for check in node.get("checks",[]) if check["kind"]=="regex")
                check["regex"][field]=value
                with self.assertRaisesRegex(ValueError,"[Rr]egex"):
                    generator.compact_graph(inventory)

    def test_native_dispatcher_and_pattern_drift_fail(self):
        source=(ROOT/"crates/scm-sdk/src/validation.rs").read_text()
        for changed in [source.replace('"uuid" => {','"new_uuid_kind" => {'),source.replace('r"^\\d+$"','r"^new-pattern$"')]:
            self.assertNotEqual(changed,source)
            with self.assertRaisesRegex(ValueError,"[Nn]ative"):
                generator.validate_graph_contract(self.inventory,changed)

    def test_effect_default_pipeline_and_coercion_in_record_keys_fail(self):
        for kind in ["ZodEffects","ZodDefault","ZodPipeline","coerce"]:
            with self.subTest(kind=kind):
                inventory=self.changed();nodes=inventory["schemas"]["definitions"]
                record=next(node for node in nodes.values() if node["kind"]=="ZodRecord")
                original=record["key"]
                node={"kind":"ZodString","coerce":True} if kind=="coerce" else {"kind":kind,"inner":original,"output":original}
                nodes["tampered_record_key"]=node;record["key"]={"$ref":"tampered_record_key"}
                with self.assertRaisesRegex(ValueError,"Record key normalization"):
                    generator.compact_graph(inventory)

    def test_committed_union_identity_matches_and_reorder_fails(self):
        native=generator.Generator(self.inventory);native.generate()
        generator.verify_union_pins(generator.union_contract(native),self.pins)
        inventory=self.changed()
        event,_=generator.Generator(inventory).normalize("s004321")
        event["variants"][0],event["variants"][1]=event["variants"][1],event["variants"][0]
        changed=generator.Generator(inventory);changed.generate()
        with self.assertRaisesRegex(ValueError,"union identity"):
            generator.verify_union_pins(generator.union_contract(changed),self.pins)

    def test_union_pin_removal_and_header_tampering_fail(self):
        for mutation in ["remove","baseline","version"]:
            pins=copy.deepcopy(self.pins)
            if mutation=="remove":pins["unions"].pop()
            elif mutation=="baseline":pins["baselineCommit"]="tampered"
            else:pins["schemaVersion"]=2
            with self.assertRaisesRegex(ValueError,"union identity"):
                generator.verify_union_pins(self.pins,pins)

    def test_phase_five_behavioral_adapters_resolve_to_implemented_public_types(self):
        native=generator.Generator(self.inventory);native.generate()
        rows=[row for row in native.model_rows if row.get("nativeFeature") in {"red-team","model-security"}]
        self.assertEqual(len(rows),54)
        source="\n".join(path.read_text() for path in (ROOT/"crates/scm-sdk/src").rglob("*.rs"))
        for row in rows:
            with self.subTest(export=row["export"]):
                self.assertEqual(row["status"],"native_client_mapping")
                for name in [row["rustType"],*row.get("additionalTypes",[])]:
                    self.assertRegex(source,r"pub\s+(?:struct|enum|type)\s+"+re.escape(name.split("::")[-1])+r"\b")

if __name__=="__main__":unittest.main()
