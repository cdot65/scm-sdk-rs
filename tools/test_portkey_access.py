"""Source and planning guards for the separately pinned Portkey delta."""
import json
import pathlib
import subprocess
import unittest

ROOT = pathlib.Path(__file__).resolve().parents[1]
BASELINE = 'b65f3c98218f32e836bbdcc0bd95614e9836a4e6'

class PortkeyPlanTests(unittest.TestCase):
    def test_exact_operation_and_parameter_inventory(self):
        c=json.loads((ROOT/'contracts/vendor/portkey-access/a27feb9/contract.json').read_text())
        ops=[op['operationId'] for path in c['paths'].values() for verb,op in path.items() if verb in ('get','post','put','delete')]
        self.assertEqual(set(ops),{'createAccessPolicy','listAccessPolicies','getAccessPolicy','updateAccessPolicy','deleteAccessPolicy'})
        self.assertEqual(len(c['components']['schemas']),9)
        self.assertEqual(len(c['paths']['/policies/access']['get']['parameters']),12)

    def test_full_contract_bounds_and_auth(self):
        c=json.loads((ROOT/'contracts/vendor/portkey-access/a27feb9/contract.json').read_text())
        scheme=c['components']['securitySchemes']['Portkey-Key']
        self.assertEqual((scheme['type'],scheme['in'],scheme['name']),('apiKey','header','x-portkey-api-key'))
        expected={('post','/policies/access'):'createAccessPolicy',('get','/policies/access'):'listAccessPolicies',('get','/policies/access/{accessPolicyId}'):'getAccessPolicy',('put','/policies/access/{accessPolicyId}'):'updateAccessPolicy',('delete','/policies/access/{accessPolicyId}'):'deleteAccessPolicy'}
        for (method,path),name in expected.items():
            op=c['paths'][path][method]
            self.assertEqual(op['operationId'],name)
            self.assertEqual(op['security'],[{'Portkey-Key':[]}])
        params={}
        for row in c['paths']['/policies/access']['get']['parameters']:
            if '$ref' in row:row=c['components']['parameters'][row['$ref'].split('/')[-1]]
            params[row['name']]=row['schema']
        self.assertEqual(set(params),set('workspace_id effect enabled user_id agent_id server_id search created_by start_time end_time page_size current_page'.split()))
        for name,maximum in [('user_id',36),('agent_id',36),('server_id',100),('search',64),('created_by',255)]:self.assertEqual(params[name]['maxLength'],maximum)
        for name in ('page_size','current_page'):self.assertEqual(params[name],{'type':'integer','minimum':0})
        schemas=c['components']['schemas']
        self.assertEqual(set(schemas['CreateAccessPolicyRequest']['required']),{'name','subjects','targets'})
        self.assertEqual(set(schemas['UpdateAccessPolicyRequest']['required']),{'version','subjects','targets'})

    def test_exact_additions_without_deletions(self):
        import difflib
        root=ROOT/'contracts/vendor/portkey-access/a27feb9'
        before=(root/'parent.yaml').read_text().splitlines()
        after=(root/'openapi.yaml').read_text().splitlines()
        added=removed=0
        for op,a,b,c,d in difflib.SequenceMatcher(None,before,after,autojunk=False).get_opcodes():
            if op!='equal':added+=d-c;removed+=b-a
        self.assertEqual((added,removed),(700,0))

    def test_plan_contains_release_and_security_requirements(self):
        p=(ROOT/'docs/portkey-access-policies-plan.md').read_text()
        for value in ('0.2.1','>=9/10','x-portkey-api-key','409','23 baseline','47 Red Team','1.89','all 47 jobs','/home/cdot/.cargo/credentials.toml','No live access-policy create/update/delete'):
            self.assertIn(value,p)

    def test_no_sdk_implementation_before_plan_acceptance(self):
        status=json.loads((ROOT/'docs/phase-status.json').read_text())
        if status['active_phase'] != 15:
            return  # Only this plan phase freezes SDK inputs; subsequent phases implement it.
        paths=['Cargo.toml','Cargo.lock','crates/scm-sdk','LICENSE','NOTICE']
        self.assertEqual(subprocess.check_output(['git','diff','--name-only',BASELINE,'--',*paths],cwd=ROOT,text=True),'')
        self.assertEqual(subprocess.check_output(['git','ls-files','--others','--exclude-standard','--',*paths],cwd=ROOT,text=True),'')

if __name__ == '__main__':unittest.main()
