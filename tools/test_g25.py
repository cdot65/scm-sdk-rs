"""Independent inventory guards for the G25 compatibility work."""
import json, pathlib, unittest
import g25_audit
ROOT=pathlib.Path(__file__).resolve().parents[1]
class G25AuditTests(unittest.TestCase):
    def test_all_operations_are_classified(self):
        a=g25_audit.audit();self.assertEqual(a,json.loads(g25_audit.DEST.read_text()))
        self.assertEqual(a['summary']['operations'],47)
        self.assertEqual(a['summary']['jsonResponseRoots'],46)
        self.assertEqual(a['summary']['matchedTypeScriptOperations'],36)
        self.assertEqual(a['summary']['formatAffectedOperations'],28)
        self.assertEqual(len({r['operationId'] for r in a['operations']}),47)
        self.assertTrue(all(r['baselineExpression'] for r in a['operations'] if r['baselineOperation']))
    def test_target_error_log_contract_is_explicit(self):
        a=g25_audit.audit();r=next(r for r in a['operations'] if r['nativeMethod']=='get_target_profile_error_logs')
        self.assertEqual(r['baselineExpression'],'ErrorLogListResponseSchema')
        self.assertEqual({p['name'] for p in r['rejectedQueryOptions']},{'skip','search'})
        self.assertIn('target_id',r['requestUuidArguments'])
    def test_response_mode_policy_and_release_are_explicit(self):
        plan=(ROOT/'docs/g25-response-compatibility-plan.md').read_text()
        for text in ['0.2.2','Mode::Request','category_display_name','tsg_id','sub_category','47-job']:
            self.assertIn(text,plan)
        self.assertIn('immutable',plan)
    def test_uuid_inventory_and_nested_calls_are_complete(self):
        a=g25_audit.audit()
        self.assertEqual(a['summary']['requestUuidOccurrences'],76)
        self.assertEqual(sum(r['occurrences'] for r in a['requestUuidInventory']),76)
        from verify_gateway_bindings import tokens
        self.assertEqual(g25_audit.uuid_arguments(tokens('common::uuid(value.as_str())?;')),['value.as_str()'])
        for r in a['requestUuidInventory']:
            self.assertEqual(r['occurrences'],sum(map(len,r['methods'].values())))
    def test_reproduction_and_independent_results_are_retained(self):
        cases=json.loads((ROOT/'tools/g25-repro/cases.json').read_text())
        receipt=json.loads((ROOT/'docs/checks/phase-18-reproduction.json').read_text())
        self.assertEqual(cases,receipt['cases'])
        self.assertEqual(len(cases),7)
        self.assertEqual(receipt['confirmedFormatMismatches'],2)
        for case,sdk,ts in zip(cases,receipt['sdkOutcomes'],receipt['typeScriptOutcomes']):
            self.assertEqual(sdk['accepted'],case['expectedSdk'])
            self.assertEqual(ts['accepted'],case['expectedTypeScript'])
    def test_audit_provenance_and_global_scope(self):
        a=g25_audit.audit()
        self.assertGreater(a['summary']['allFrozenRootsWithFormats'],100)
        for source in ['contracts/vendor/red-team-data-plane/0.93.0/operations.json','contracts/manifest.json','contracts/native-operation-mappings/red-team.json']:
            self.assertIn(source,a['inputs'])
        self.assertNotIn('contracts/typescript.json',a['inputs'])
        for path,sha in a['inputs'].items():self.assertEqual(g25_audit.digest(path),sha)
if __name__=='__main__':unittest.main()
