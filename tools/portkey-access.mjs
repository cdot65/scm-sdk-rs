#!/usr/bin/env node
// Development-only pin and first-parent delta verifier.
import fs from 'node:fs';
import crypto from 'node:crypto';
import {isDeepStrictEqual} from 'node:util';
import {createRequire} from 'node:module';
const require = createRequire(new URL('./package.json', import.meta.url));
const yaml = require('js-yaml');
const dir = new URL('../contracts/vendor/portkey-access/a27feb9/', import.meta.url);
const read = name => fs.readFileSync(new URL(name, dir));
const pin = JSON.parse(read('source.json'));
if (pin.commit !== 'a27feb9efaac3653d2d94eca5d7d210944b6e160' || pin.parent !== '8f91369f15e6b6765a13c1af7e1ee3ea336f83ba') throw Error('Unexpected pin');
for (const [name, info] of Object.entries(pin.files)) {
  const data=read(name);
  if (crypto.createHash('sha256').update(data).digest('hex') !== info.sha256 || data.length !== info.bytes ||
      crypto.createHash('sha1').update(Buffer.concat([Buffer.from(`blob ${data.length}\0`),data])).digest('hex') !== info.gitBlobSha1) throw Error('Pinned file changed: '+name);
}
if(require('js-yaml/package.json').version !== pin.yamlParser.version) throw Error('YAML parser drift');
const spec = yaml.load(read('openapi.yaml').toString());
const parent = yaml.load(read('parent.yaml').toString());
const routes = ['/policies/access','/policies/access/{accessPolicyId}'];
const schemas = ['AccessPolicySubject','AccessPolicySubjectDetails','AccessPolicyTarget','AccessPolicyMcpServerTarget','AccessPolicyProviderTarget','CreateAccessPolicyRequest','UpdateAccessPolicyRequest','AccessPolicy','AccessPolicyListItem'];
const eq = isDeepStrictEqual;
for (const [route,value] of Object.entries(parent.paths)) if (!eq(value,spec.paths[route])) throw Error('Existing route changed: '+route);
if (!eq(Object.keys(spec.paths).filter(x=>!(x in parent.paths)).sort(), [...routes].sort())) throw Error('Unexpected added routes');
for (const [name,value] of Object.entries(parent.components.schemas)) if (!eq(value,spec.components.schemas[name])) throw Error('Existing schema changed: '+name);
if (!eq(Object.keys(spec.components.schemas).filter(x=>!(x in parent.components.schemas)).sort(), [...schemas].sort())) throw Error('Unexpected added schemas');
const parameters = ['AccessPolicyId','WorkspaceIdQuery','PageSize','CurrentPage'];
for (const name of parameters.slice(1)) if (!eq(parent.components.parameters[name],spec.components.parameters[name])) throw Error('Shared parameter changed');
if ('AccessPolicyId' in parent.components.parameters ||
    !eq(Object.keys(spec.components.parameters).filter(x=>!(x in parent.components.parameters)), ['AccessPolicyId'])) throw Error('Parameter delta drift');
const stripped=structuredClone(spec);
for(const route of routes) delete stripped.paths[route];
for(const name of schemas) delete stripped.components.schemas[name];
delete stripped.components.parameters.AccessPolicyId;
if(stripped.tags.filter(x=>x.name==='Access Policies').length!==1) throw Error('Tag delta drift');
stripped.tags=stripped.tags.filter(x=>x.name!=='Access Policies');
// Preserve the entire document except the exact known documentation group.
function removeGroup(value) {
  if(Array.isArray(value)) return value.filter(x=>!(x && x.id==='access-policies')).map(removeGroup);
  if(value && typeof value==='object') return Object.fromEntries(Object.entries(value).map(([k,v])=>[k,removeGroup(v)]));
  return value;
}
stripped['x-code-samples']=removeGroup(stripped['x-code-samples']);
if(!eq(stripped,parent)) throw Error('Unexpected whole-document delta');
if(read('openapi.yaml').toString().split('\n').length-read('parent.yaml').toString().split('\n').length !== pin.additions || pin.deletions!==0) throw Error('Line delta drift');
const contract = {commit:pin.commit, parent:pin.parent, paths:Object.fromEntries(routes.map(x=>[x,spec.paths[x]])), components:{securitySchemes:{'Portkey-Key':spec.components.securitySchemes['Portkey-Key']},schemas:Object.fromEntries(schemas.map(x=>[x,spec.components.schemas[x]])),parameters:Object.fromEntries(parameters.map(x=>[x,spec.components.parameters[x]]))}};
const operations = routes.flatMap(path => Object.entries(spec.paths[path]).filter(([verb])=>['get','post','put','delete','patch'].includes(verb)).map(([method,op])=>({method:method.toUpperCase(),path,operationId:op.operationId,security:op.security})));
if (operations.length !== 5 || operations.some(x=>!eq(x.security,[{'Portkey-Key':[]}])) ) throw Error('Operation/auth delta changed');
const output = JSON.stringify(contract,null,2)+'\n';
const destination = new URL('contract.json',dir);
if (process.argv.includes('--check')) {if(fs.readFileSync(destination,'utf8')!==output) throw Error('Contract drift');}
else fs.writeFileSync(destination,output);
console.log(JSON.stringify({passed:true,operations,schemas:schemas.length}));
