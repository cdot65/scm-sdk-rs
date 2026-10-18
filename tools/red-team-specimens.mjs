// Independent OpenAPI specimens shared by development-only AJV oracles.
export function sample(spec,s){
  if(s.$ref)return sample(spec,spec.components.schemas[s.$ref.split('/').at(-1)]);
  if('const'in s)return s.const;
  if(s.enum)return s.enum[0];
  if(s.anyOf||s.oneOf)return sample(spec,(s.anyOf||s.oneOf).find(x=>x.type!=='null'));
  if(s.type==='null')return null;
  if(s.type==='boolean')return false;
  if(s.type==='integer'||s.type==='number')return Math.max(s.minimum??0,0);
  if(s.type==='string'){
    const formatted={uuid:'550e8400-e29b-41d4-a716-446655440000','date-time':'2026-10-18T00:00:00Z',date:'2026-10-18',duration:'PT1S'};
    return formatted[s.format]??'example'.repeat(Math.ceil((s.minLength??1)/7)).slice(0,s.maxLength??1000);
  }
  if(s.type==='array')return Array.from({length:Math.max(s.minItems??0,1)},()=>sample(spec,s.items));
  if(s.type==='object'){
    const result=Object.fromEntries(Object.entries(s.properties??{}).map(([k,v])=>[k,sample(spec,v)]));
    if(!s.properties&&typeof s.additionalProperties==='object')result[s.propertyNames?sample(spec,s.propertyNames):'example']=sample(spec,s.additionalProperties);
    return result;
  }
  return {preserved:true};
}
