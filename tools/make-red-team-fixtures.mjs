#!/usr/bin/env node
// Concrete independent JSON Schema specimens; deliberately does not use the Rust graph compiler.
import fs from 'node:fs';
import {sample as sampleSchema} from './red-team-specimens.mjs';
import {createRequire} from 'node:module';
const require=createRequire(new URL('./package.json',import.meta.url));
const Ajv=require('ajv/dist/2020.js').default;
const formats=require('ajv-formats');
const dir=new URL('../contracts/vendor/red-team-data-plane/0.93.0/',import.meta.url);
const spec=JSON.parse(fs.readFileSync(new URL('openapi.json',dir)));
const id='urn:scm-sdk:red-team-dp:specimens';
const ajv=new Ajv({strict:false,allErrors:true}); formats(ajv);ajv.addSchema({...spec,$id:id});
// Response evolution deliberately preserves unknown fields and enum strings.
// Strict component roots keep required fields; the four route waivers are tested separately.
function responsePolicy(value){
  if(Array.isArray(value))return value.map(responsePolicy);
  if(value===null||typeof value!=='object')return value;
  const result=Object.fromEntries(Object.entries(value).map(([k,v])=>[k,responsePolicy(v)]));
  if(result.type==='string'&&result.enum)delete result.enum;
  if(result.additionalProperties===false)result.additionalProperties=true;
  return result;
}
const responseAjv=new Ajv({strict:false,allErrors:true});formats(responseAjv);
responseAjv.addSchema({...responsePolicy(spec),$id:id});
const sample=s=>sampleSchema(spec,s);
const cases=[];
function add(name,schema,value,valid){
  const actual=ajv.getSchema(id+'#/components/schemas/'+schema)(value);
  if(actual!==valid)throw Error('Invalid independent specimen: '+name);
  const mutationClass=valid?'valid':name.startsWith('missing_')?'missing_required':name.startsWith('closed_')?'closed_object':name.startsWith('rfc3339_')||name.startsWith('duration_')?'format_violation':name.startsWith('runtime_unicode_')||name.startsWith('override_count_')||name.startsWith('asr_')?'bound_violation':'wrong_type_or_structure';
  const responseValid=responseAjv.getSchema(id+'#/components/schemas/'+schema)(value);
  cases.push({name,schema,value:structuredClone(value),valid,mutationClass,responseValid});
}
for(const[name,s]of Object.entries(spec.components.schemas)){
  const base=sample(s),validate=ajv.getSchema(id+'#/components/schemas/'+name);
  add('valid_'+name,name,base,true);
  for(const field of s.required??[]){const value=structuredClone(base);delete value[field];add('missing_'+name+'_'+field,name,value,false);}
  for(const field of Object.keys(s.properties??{})){
    if(!(s.required??[]).includes(field)){
      const omitted=structuredClone(base);delete omitted[field];
      add('optional_omitted_'+name+'_'+field,name,omitted,true);
    }
    for(const bad of [null,false,{},[],2,'invalid']){
      const value={...structuredClone(base),[field]:bad};
      if(!validate(value)){add('wrong_type_'+name+'_'+field,name,value,false);break;}
    }
  }
  if(s.additionalProperties===false)add('closed_'+name,name,{...base,unexpected:'rejected'},false);
}
const runtime='RuntimeProfileUpdateRequestSchema';
for(const n of [0,1,256,257])add('runtime_unicode_'+n,runtime,{runtime_profile_id:'🦀'.repeat(n)},n>0&&n<=256);
const override='ThreatOverrideRequest',output='550e8400-e29b-41d4-a716-446655440000';
for(const v of [true,false,null])add('override_'+v,override,{overrides:[{output_id:output,threat_override:v}]},true);
for(const n of [0,1,100,101])add('override_count_'+n,override,{overrides:Array.from({length:n},()=>({output_id:output,threat_override:null}))},n>=1&&n<=100);
for(const [k,v]of [['output_id','invalid'],['threat_override','false']])add('override_invalid_'+k,override,{overrides:[{output_id:output,threat_override:null,[k]:v}]},false);
for(const n of [-1,0,100,101])add('asr_'+n,'AsrResponseSchema',{job_id:output,job_type:'STATIC',asr:n},n>=0&&n<=100);
for(const [v,valid]of [['2026-10-18T00:00:00Z',true],['2026-10-18t01:00:00+01:00',true],['2016-12-31T23:59:60Z',true],['2026-02-30T00:00:00Z',false],['2026-10-18T00:00Z',false],['2026-10-18T00:00:00+25:00',false]])
  add('rfc3339_'+v,'ThreatDetails',{...sample(spec.components.schemas.ThreatDetails),overridden_at:v},valid);
for(const [v,valid]of [['P0D',true],['PT0S',true],['P1Y2M3DT4H5M6S',true],['P2W',true],['P',false],['PT',false],['P1W2D',false]])
  add('duration_'+v,'JobTimeRecord',{...sample(spec.components.schemas.JobTimeRecord),time_taken:v},valid);
for(const name of ['GoalSchema','GoalResponseSchema'])for(const branch of spec.components.schemas[name].properties.goal_subtype.anyOf){
  if(branch.type==='null')continue;
  const subtype=sample(branch);
  add('subtype_'+name+'_'+subtype,name,{...sample(spec.components.schemas[name]),goal_subtype:subtype},true);
}
const missingTag=sample(spec.components.schemas.JobResponseSchema);
delete missingTag.progress.kind;
add('ambiguous_progress_without_kind','JobResponseSchema',missingTag,false);
missingTag.progress.rows='invalid for standard, unrecognized by agentic';
add('oneof_without_tag_has_one_structural_match','JobResponseSchema',missingTag,true);
const reserved=sample(spec.components.schemas.JobCreateRequestSchema);
reserved.extra_info=JSON.parse('{"__proto__":{"preserved":true},"constructor":"opaque data"}');
add('reserved_keys_are_json_data','JobCreateRequestSchema',reserved,true);
const text=JSON.stringify({schemaVersion:1,source:'Pinned OpenAPI; independent AJV 2020 and concrete boundary specimens',cases},null,2)+'\n';
const target=new URL('model-fixtures.json',dir);
if(process.argv.includes('--check')){if(fs.readFileSync(target,'utf8')!==text)throw Error('Independent model specimen drift');}
else fs.writeFileSync(target,text);
console.log(JSON.stringify({independentModelCases:cases.length}));
