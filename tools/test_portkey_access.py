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
        description=c['paths']['/policies/access']['post']['description']
        self.assertIn('`allow` policies must name at least one specific user, group, or agent.',description)
        self.assertIn('`deny` if `subjects` contains both a user `ANY` and an agent `ANY` entry',description)
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

    def test_independent_pins_references_and_plan_routes(self):
        import re
        root=ROOT/'contracts/vendor/portkey-access/a27feb9'
        pin=json.loads((root/'source.json').read_text())
        self.assertEqual({k:v['sha256'] for k,v in pin['files'].items()}, {'openapi.yaml': '9c3b5e643c0ecf0a8b63a2fa6fc68c15086b153657185e42a289012c35e4ca92', 'parent.yaml': '2acc089c9c7ac8ac90f264b23b5d394df05abf80784044fd3b14582b867fe3bc', 'LICENSE': 'bcba3de214851cce46ed5af42d6698044616eeace887c3231bc7a20474ab639e'})
        c=json.loads((root/'contract.json').read_text())
        def walk(value):
            if isinstance(value,dict):
                if '$ref' in value:
                    parts=value['$ref'].split('/')
                    self.assertEqual(parts[:2],['#','components'])
                    self.assertIn(parts[3],c['components'][parts[2]])
                for v in value.values():walk(v)
            elif isinstance(value,list):
                for v in value:walk(v)
        walk(c)
        rows=set(re.findall(r'^\| (GET|POST|PUT|DELETE) \| ([^ ]+) \|', (ROOT/'docs/portkey-access-policies-plan.md').read_text(),re.M))
        expected={(verb.upper(),route) for route,ops in c['paths'].items() for verb in ops if verb in ('get','post','put','delete')}
        self.assertEqual(rows,expected)

    def test_authored_whitespace_is_not_dependent_on_staging(self):
        for name in ('docs/portkey-access-policies-plan.md','docs/portkey-access-policies-research.md','tools/portkey-access.mjs','tools/test_portkey_access.py','contracts/vendor/portkey-access/a27feb9/source.json','contracts/vendor/portkey-access/a27feb9/contract.json','contracts/vendor/portkey-access/a27feb9/model-fixtures.json','contracts/vendor/portkey-access/a27feb9/operations.json'):
            text=(ROOT/name).read_text()
            self.assertTrue(text.endswith('\n'),name)
            self.assertFalse(any(line.rstrip()!=line for line in text.splitlines()),name)

    def test_source_and_document_delta_tampering_fails(self):
        import tempfile,shutil,hashlib
        with tempfile.TemporaryDirectory(dir=ROOT/'.local') as tmp:
            root=pathlib.Path(tmp);dest=root/'contracts/vendor/portkey-access/a27feb9'
            shutil.copytree(ROOT/'contracts/vendor/portkey-access/a27feb9',dest)
            (root/'tools').mkdir()
            for name in ('portkey-access.mjs','package.json','package-lock.json'):shutil.copyfile(ROOT/'tools'/name,root/'tools'/name)
            (root/'tools/node_modules').symlink_to(ROOT/'tools/node_modules',target_is_directory=True)
            originals={name:(dest/name).read_bytes() for name in ('source.json','openapi.yaml','parent.yaml')}
            for mode in ('digest','route','parent','tag','additions'):
                for name,content in originals.items():(dest/name).write_bytes(content)
                pin=json.loads((dest/'source.json').read_text())
                if mode=='digest':pin['files']['LICENSE']['sha256']='0'*64
                elif mode=='additions':pin['additions']=701
                else:
                    name='parent.yaml' if mode=='parent' else 'openapi.yaml'
                    data=(dest/name).read_bytes()
                    if mode=='route':data=data.replace(b'/policies/access:',b'/policies/changed:',1)
                    elif mode=='parent':data=data.replace(b'title: Portkey API',b'title: changed API',1)
                    else:data=data.replace(b'- name: Access Policies',b'- name: Changed Policies',1)
                    (dest/name).write_bytes(data)
                    pin['files'][name].update(sha256=hashlib.sha256(data).hexdigest(),bytes=len(data),gitBlobSha1=hashlib.sha1(b'blob '+str(len(data)).encode()+b'\0'+data).hexdigest())
                (dest/'source.json').write_text(json.dumps(pin))
                r=subprocess.run(['node',str(root/'tools/portkey-access.mjs'),'--check'],capture_output=True,text=True)
                self.assertNotEqual(r.returncode,0,mode)

    def test_release_documentation_and_receipt_are_version_bound(self):
        import tomllib
        version=tomllib.loads((ROOT/'crates/scm-sdk/Cargo.toml').read_text())['package']['version']
        if version != '0.2.1':return  # Historical planning/implementation phases retain 0.2.0.
        for name in ('README.md','CHANGELOG.md','docs/STABILITY.md','docs/RELEASE.md'):
            text=(ROOT/name).read_text()
            self.assertIn('release-registry-'+version+'.json',text)
            self.assertIn('publication is not yet verified',text)
        path=ROOT/f'docs/checks/release-registry-{version}.json'
        if path.exists():
            receipt=json.loads(path.read_text())
            self.assertEqual(receipt['version'],version)
            self.assertIn('cdot65',receipt['ownerLogins'])
            self.assertFalse(receipt['yanked'])
            proof=json.loads((ROOT/'docs/checks/phase-17-release-package-1.99.0.json').read_text())
            self.assertEqual(receipt['archiveSha256'],proof['archive_sha256'])
            self.assertEqual(receipt['sourceCommit'],proof['releaseCommit'])

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
