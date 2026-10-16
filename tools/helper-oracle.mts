#!/usr/bin/env node
/** Expected pure-helper behavior comes from the exact owned TypeScript baseline. */
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {readFile,writeFile} from 'node:fs/promises';
import path from 'node:path';
import {fileURLToPath,pathToFileURL} from 'node:url';
import {z} from 'zod';
const root=path.resolve(path.dirname(fileURLToPath(import.meta.url)),'..');
const baseline=path.join(root,'.local/typescript');
const output=path.join(root,'contracts/native-fixtures/helpers.json');
const check=process.argv.includes('--check');
if(process.argv.slice(2).some(value=>value!=='--check'))throw new Error('Only --check is supported');
execFileSync('python3',['tools/verify-contracts.py','--baseline',baseline],{cwd:root,stdio:'pipe'});
const inventory=JSON.parse(await readFile(path.join(root,'contracts/typescript.json'),'utf8'));
const hash=(bytes:Uint8Array|string)=>createHash('sha256').update(bytes).digest('hex');
const inputs=[];
for(const file of ['contracts/typescript.json','tools/package.json','tools/package-lock.json'])inputs.push({file,sha256:hash(await readFile(path.join(root,file)))});
for(const key of Object.keys(process.env))delete process.env[key];
globalThis.fetch=async()=>{throw new Error('Network forbidden in helper oracle');};
Object.defineProperty(globalThis,'WebSocket',{configurable:true,value:class{constructor(){throw new Error('Network forbidden in helper oracle');}}});
const load=(file:string)=>import(pathToFileURL(path.join(baseline,'src',file)).href);
const dotted=await load('ai-gateway/nested-values.ts');
const secret=await load('ai-gateway/secret-fields.ts');
const host=await load('ai-gateway/integrations-client.ts');
const listing=await load('listing.ts');
const scope=await load('iam/scope-name.ts');
const nullable=await load('models/dlp-json-nullable.ts');
const page=await load('models/dlp-page.ts');
const constants=await load('constants.ts');
type Case={helper:string,label:string,input:unknown,expected:{success:boolean,output?:unknown}};
const cases:Case[]=[];
async function sample(helper:string,label:string,input:unknown,invoke:()=>unknown|Promise<unknown>){
 try{cases.push({helper,label,input,expected:{success:true,output:await invoke()}});}catch{cases.push({helper,label,input,expected:{success:false}});}
}
for(const input of [[],[{path:'retry.attempts',value:3},{path:'targets[0].provider',value:'@synthetic'}],[{path:'a[1]',value:2},{path:'a[0]',value:1}],[{path:'a[0][1]',value:'b'},{path:'a[0][0]',value:'a'}],[{path:'a\\.b.c\\[d\\]',value:null}],[{path:'a',value:{b:1}},{path:'a.c',value:2}]])await sample('buildDottedObject','valid',input,()=>dotted.buildDottedObject(input));
for(const path of ['', '.a','a.','a..b','[0]','a[01]','a[-1]','a[1.0]','a[]','a[','a]','a[0]b','a.[0]','a\\q','a\\','__proto__.x','a.constructor','a.prototype','a[1]']){const input=[{path,value:1}];await sample('buildDottedObject',path,input,()=>dotted.buildDottedObject(input));}
for(const input of [[{path:'a',value:1},{path:'a',value:2}],[{path:'a',value:1},{path:'a.b',value:2}],[{path:'a.b',value:1},{path:'a',value:2}],[{path:'a[0]',value:1},{path:'a.b',value:2}],[{path:'a',value:{b:1}},{path:'a.b',value:2}]])await sample('buildDottedObject','conflict',input,()=>dotted.buildDottedObject(input));
for(const input of [{object:{a:{b:1}},path:'a.b',value:2},{object:{a:[1,2]},path:'a[0]',value:null},{object:{},path:'a[0].b',value:false},{object:{a:[1]},path:'a[1]',value:2},{object:{a:[1]},path:'a[2]',value:2},{object:{a:1},path:'a.b',value:2},{object:{a:[]},path:'a.b',value:2},{object:{},path:'__proto__.x',value:2}])await sample('setDottedValue',input.path,input,()=>dotted.setDottedValue(input.object,input.path,input.value));
for(const input of [{host:'http://localhost:8000/v1'},{host:'https://example.test/v1/',headers:{Authorization:'synthetic-provider-key','X-Test':'value'}},{host:'https://example.test/v1?query=value#fragment'},{host:'http://user:synthetic@example.test/v1'},{host:'not-a-url'},{host:'ftp://example.test/v1'},{host:'file:///tmp/example'}])await sample('customHostConfiguration',input.host,input,()=>host.customHostConfiguration(input));
for(const name of ['Truffles','Recipe Gen (EU)','','___','Helloİ K','Production 2'])await sample('generateWorkspaceScopeName','stem',{name},()=>{const result=scope.generateWorkspaceScopeName(name);return {prefix:result.slice(0,-6),validSuffix:/[a-z0-9]{6}$/.test(result)};});
for(const input of [null,{}, {skip:0,limit:0,search:''},{skip:2,limit:50,search:'synthetic filter'}])await sample('serializeListing','canonical',input,()=>listing.serializeListing(input??undefined));
await sample('aiGwOrganisationsAuthSettingsPath','numeric',{tsgId:'1234567890'},()=>constants.aiGwOrganisationsAuthSettingsPath('1234567890'));
for(const array of [false,true]){
 const wildcard=array?['synthetic-secret','synthetic-secret']:{first:'synthetic-secret',second:'synthetic-secret'};
 const payload={safe:'retained',key:'synthetic-secret',data:[{auth_config:{password:'synthetic-secret'}}],settings:{password:'synthetic-secret'},auth_config:{password:'synthetic-secret'},client_auth:'synthetic-secret',credentials:{password:'synthetic-secret',key:'synthetic-secret'},configurations:{azure_entra_client_secret:'synthetic-secret',aws_secret_access_key:'synthetic-secret',vertex_service_account_json:{private_key:'synthetic-secret'},custom_headers:wildcard,client_secret:'synthetic-secret',oauth_client_secret:'synthetic-secret',oauth_metadata:{token:'synthetic-secret'}},scim_token:'synthetic-secret',auth_settings:{client_secret:'synthetic-secret'},model_config:{token:'synthetic-secret'}};
 for(const operation of Object.keys(secret.AI_GATEWAY_SECRET_FIELDS))for(const direction of ['request','response']){const scopedPayload=JSON.parse(JSON.stringify(payload));if(operation==='organisations.getAuthSettings'||operation==='organisations.updateAuthSettings')scopedPayload.data={scim_token:'synthetic-secret'};const input={operation,direction,payload:scopedPayload};await sample('redactAIGatewaySecrets',array?'array':'object',input,()=>secret.redactAIGatewaySecrets(operation,scopedPayload,direction));}
}
for(const input of [{presence:'unset'},{presence:'null'},{presence:'value',value:'valid'},{presence:'value',value:''},{presence:'value',value:3}])await sample('jsonNullable','composed',input,()=>{const result=nullable.jsonNullable(z.string().min(1)).parse(input.presence==='unset'?undefined:input.presence==='null'?null:input.value);return result===undefined?{presence:'unset'}:result===null?{presence:'null'}:{presence:'value',value:result};});
for(const input of [{content:[{id:'one',extra:1}],last:false,totalElements:1,future:{retained:true}},{content:[],empty:true,first:true,last:true,number:0,numberOfElements:0,pageable:{offset:0,pageNumber:0,pageSize:50,paged:true,unpaged:false,sort:{sorted:true}},size:50,sort:{empty:false},totalElements:0,totalPages:0},{content:[],last:null},{content:[{id:3}]},{content:[]},{last:true}])await sample('pageSchema','composed',input,()=>page.pageSchema(z.object({id:z.string()}).passthrough()).parse(input));
await sample('collectAll','cap',{items:[1,2,3,4],max:3},async()=>{async function* items(){yield* [1,2,3,4];}return listing.collectAll(items(),{max:3});});
for(const total of [null,3]){const pages=[{items:[1,2],total},{items:[3],total}];await sample('collectSkipPages','actual-count',{pages,limit:2},()=>{let i=0;return listing.collectSkipPages(async()=>pages[i++]??{items:[],total},{limit:2});});}
for(const pages of [[{items:[1,2],last:false},{items:[3],last:true}],[{items:[],last:false}]])await sample('collectSpringPages','last',{pages,size:2},()=>{let i=0;return listing.collectSpringPages(async()=>pages[i++],{size:2});});
await sample('paginate','cursor',{pages:[{items:[1,2],next:1},{items:[3]}]},async()=>{const values=[];for await(const value of listing.paginate(async(cursor:number)=>cursor===0?{items:[1,2],next:1}:{items:[3]},0))values.push(value);return values;});
const frozenConstants=inventory.exports.filter((row:any)=>row.kind==='constant').map((row:any)=>({name:row.name,...('value' in row?{value:row.value}:{nativeAdaptation:'isolated caller-owned configuration'})}));
const fixture={schemaVersion:1,baselineCommit:inventory.baseline.commit,inputs,sourceFiles:inventory.baseline.sources,constants:frozenConstants,cases};
const bytes=`${JSON.stringify(fixture,null,2)}\n`;
if(check){if(await readFile(output,'utf8')!==bytes)throw new Error('Helper oracle differs');}else await writeFile(output,bytes);
process.stdout.write(`${JSON.stringify({checked:check,cases:cases.length,constants:frozenConstants.length})}\n`);
