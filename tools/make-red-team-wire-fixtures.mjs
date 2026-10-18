#!/usr/bin/env node
// Independent wire values come from the pinned OpenAPI and AJV, never Rust models.
import fs from 'node:fs';
import {createRequire} from 'node:module';
import {sample} from './red-team-specimens.mjs';
const require=createRequire(new URL('./package.json',import.meta.url));
const Ajv=require('ajv/dist/2020.js').default;
const formats=require('ajv-formats');
const dir=new URL('../contracts/vendor/red-team-data-plane/0.93.0/',import.meta.url);
const spec=JSON.parse(fs.readFileSync(new URL('openapi.json',dir)));
const ledger=JSON.parse(fs.readFileSync(new URL('operations.json',dir)));
const pin=JSON.parse(fs.readFileSync(new URL('source.json',dir)));
const id='urn:scm-sdk:red-team-dp:wire';
const ajv=new Ajv({strict:false,allErrors:true});formats(ajv);ajv.addSchema({...spec,$id:id});
const uuid=n=>'550e8400-e29b-41d4-a716-'+String(n).padStart(12,'0');
const ids=Object.fromEntries(['job_id','attack_id','stream_id','goal_id','prompt_set_id','prompt_id','target_id'].map((key,i)=>[key,uuid(i+1)]));
function validate(schema,value,label){
  const test=ajv.compile({...schema,$id:undefined,components:spec.components});
  if(!test(value))throw Error(label+': '+JSON.stringify(test.errors));
}
const cases=[];
for(const [path,methods] of Object.entries(spec.paths))for(const[method,op]of Object.entries(methods)){
  if(!['get','post','put','patch','delete'].includes(method))continue;
  const binding=ledger.operations.find(row=>row.operationId===op.operationId);
  if(!binding)throw Error('Missing native call identity: '+op.operationId);
  const parameters={},query={};let concrete=path;
  for(const p of op.parameters??[]){
    let value=p.in==='path'?(ids[p.name]??sample(spec,p.schema)):sample(spec,p.schema);
    if(p.name==='target_type')value='APPLICATION';
    if(p.name==='target_id')value=ids.target_id;
    validate(p.schema,value,op.operationId+' '+p.name);
    if(p.in==='path'){parameters[p.name]=value;concrete=concrete.replace('{'+p.name+'}',value);}
    else if(p.in==='query')query[p.name]=value;
    else throw Error('Unimplemented parameter location');
  }
  const bodySchema=op.requestBody?.content?.['application/json']?.schema;
  const body=bodySchema?sample(spec,bodySchema):undefined;
  if(binding.nativeMethod==='set_runtime_profile')body.runtime_profile_id='fixture-runtime-profile';
  if(binding.nativeMethod.startsWith('override_'))body.overrides=[true,false,null].map((value,i)=>({output_id:uuid(11+i),threat_override:value}));
  if(bodySchema)validate(bodySchema,body,op.operationId+' body');
  const successes=Object.entries(op.responses).filter(([status])=>/^2\d\d$/.test(status));
  if(successes.length!==1)throw Error('Explicit success selection required');
  const[status,response]=successes[0],responseSchema=response.content?.['application/json']?.schema;
  let responseBody=responseSchema?sample(spec,responseSchema):undefined;
  if(binding.nativeMethod==='get_dynamic_asr')responseBody.job_type='DYNAMIC';
  if(binding.nativeMethod==='set_runtime_profile')responseBody.job_metadata.runtime_profile_id=body.runtime_profile_id;
  if(binding.nativeMethod==='regenerate')responseBody.background_job_id=uuid(101);
  if(binding.nativeMethod==='get_download_url'){
    responseBody.filename='../../opaque-report.csv';
    responseBody.download_url='https://unadmitted-storage.example/report.csv?signature=synthetic-private-download';
  }
  if(responseSchema)validate(responseSchema,responseBody,op.operationId+' response');
  else if(binding.nativeMethod!=='download_error_logs')throw Error('Unclassified binary response');
  const operation='airs.red_team.'+(binding.nativeClass==='RedTeamClient'?'':binding.nativeClass==='RedTeamScansClient'?'scans.':binding.nativeClass==='RedTeamReportsClient'?'reports.':'custom_attack_reports.')+binding.nativeMethod;
  const base={operationId:op.operationId,call:{className:binding.nativeClass,member:binding.nativeMethod},operation,method:method.toUpperCase(),path:concrete,pathParameters:parameters,query,
    queryWire:Object.fromEntries(Object.entries(query).map(([k,v])=>[k,[String(v)]])),
    ...(body===undefined?{}:{body}),status:Number(status),
    ...(responseBody===undefined?{rawResponseHex:'6a6f625f69642c6572726f720a31ff2c666978747572650a'}:{responseBody})};
  cases.push({name:op.operationId+'__all_parameters',...base});
  const optional=new Set((op.parameters??[]).filter(p=>p.in==='query'&&!p.required).map(p=>p.name));
  if(optional.size){const row=structuredClone(base);for(const name of optional){delete row.query[name];delete row.queryWire[name];}cases.push({name:op.operationId+'__omitted_optional_parameters',...row});}
}
const expected=new Set(ledger.operations.map(row=>row.operationId)),actual=new Set(cases.map(row=>row.operationId));
if(expected.size!==actual.size||[...expected].some(id=>!actual.has(id)))throw Error('Operation fixture set differs from spec');
const text=JSON.stringify({schemaVersion:1,sourceSha256:pin.sha256,source:'Pinned OpenAPI 0.93.0 and independent AJV 2020; ledger supplies native call names only',cases},null,2)+'\n';
const target=new URL('wire-fixtures.json',dir);
if(process.argv.includes('--check')){if(fs.readFileSync(target,'utf8')!==text)throw Error('Independent wire fixture drift');}
else fs.writeFileSync(target,text);
console.log(JSON.stringify({independentWireCases:cases.length,operations:actual.size}));
