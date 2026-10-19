import fs from 'node:fs';
import {AttackListResponseSchema,ErrorLogListResponseSchema} from '../../.local/typescript/src/models/red-team.ts';
const schemas={AttackListResponseSchema,ErrorLogListResponseSchema};
for(const c of JSON.parse(fs.readFileSync(new URL('./cases.json',import.meta.url),'utf8'))){
 const accepted=schemas[c.schema as keyof typeof schemas].safeParse(c.value).success;
 console.log(JSON.stringify({schema:c.schema,field:c.field,accepted}));
 if(accepted!==c.expectedTypeScript)throw new Error('Frozen TypeScript outcome differs');
}
