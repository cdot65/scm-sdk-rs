#!/usr/bin/env node
/** Execute the verified upstream 0.35.0 schemas with locked Zod, independently of Rust. */
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { mkdir, mkdtemp, readFile, rm, symlink, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import type { ZodTypeAny } from 'zod';

type Json = null | boolean | number | string | Json[] | { [key: string]: Json };
type ObjectJson = { [key: string]: Json };
const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const source = path.join(repo, 'contracts/sources/typescript-0.35.0');
const destination = path.join(repo, 'contracts/typescript-035-oracle.json');
const check = process.argv.includes('--check');
if (process.argv.slice(2).some(argument => argument !== '--check')) throw new Error('Only --check is supported');
const sha = (bytes: string | Uint8Array) => createHash('sha256').update(bytes).digest('hex');
// Fail closed before copying or executing any upstream source.
execFileSync('python3', ['tools/verify_typescript035_source.py', '--check'], { cwd: repo, stdio: 'pipe' });
const manifestBytes = await readFile(path.join(source, 'manifest.json'));
const manifest = JSON.parse(manifestBytes.toString());
const lockBytes = await readFile(path.join(repo, 'tools/package-lock.json'));
const locked = JSON.parse(lockBytes.toString()).packages['node_modules/zod'].version;
const installed = JSON.parse(await readFile(path.join(repo, 'tools/node_modules/zod/package.json'), 'utf8')).version;
if (installed !== locked) throw new Error('Installed Zod differs from the committed lockfile; run npm ci in tools');
const roots = [
  'ContentTypeConfigurationSchema', 'ContentTypeConfigurationsSchema',
  'ModelConfigurationSchema', 'AiSecurityProfileSchema', 'PolicySchema',
  'SecurityProfileSchema', 'CreateSecurityProfileRequestSchema', 'SecurityProfileListResponseSchema',
];
const specimens: { id: string; schema: string; input: Json }[] = [];
const add = (schema: string, label: string, input: Json) => specimens.push({ id: `${schema}:${label}`, schema, input: structuredClone(input) });
const fixture = JSON.parse(await readFile(path.join(source, 'test/models/fixtures/directional-security-profile.json'), 'utf8')) as ObjectJson;
fixture.dlp_tenant_id = '100';
const policy = fixture.policy as ObjectJson;
const ai = (policy['ai-security-profiles'] as ObjectJson[])[0];
const directions = ai['content-type-configurations'] as ObjectJson;
const shared = ai['model-configuration'] as ObjectJson;
const protection = directions.prompt as ObjectJson;
const bases: Record<string, Json> = {
  ContentTypeConfigurationSchema: protection,
  ContentTypeConfigurationsSchema: directions,
  ModelConfigurationSchema: { ...protection, ...shared },
  AiSecurityProfileSchema: ai,
  PolicySchema: policy,
  SecurityProfileSchema: fixture,
  CreateSecurityProfileRequestSchema: fixture,
  SecurityProfileListResponseSchema: { ai_profiles: [fixture] },
};
for (const schema of roots) {
  add(schema, 'public-fixture', bases[schema]);
  add(schema, 'empty', {});
  for (const [label, input] of Object.entries<Json>({ null: null, string: 'invalid', number: 7, boolean: false, array: [] })) add(schema, `root-${label}`, input);
  add(schema, 'unknown-field', { ...(bases[schema] as ObjectJson), 'future-field': { preserved: [null, false, 7] } });
}
function fieldCases(schema: string, field: string, base: ObjectJson, valid: Json[]) {
  const omitted = structuredClone(base);
  delete omitted[field];
  add(schema, `${field}-omitted`, omitted);
  for (const [index, value] of [...valid, null, 'invalid', 7, false, [], {}].entries()) {
    add(schema, `${field}-value-${index}`, { ...base, [field]: value });
  }
}
for (const direction of ['prompt', 'response', 'tool-call', 'tool-response']) {
  fieldCases('ContentTypeConfigurationsSchema', direction, {}, [protection]);
  add('ContentTypeConfigurationsSchema', `${direction}-nested-unknown`, { [direction]: { 'future-protection': { nested: null } } });
  for (const field of ['data-protection', 'app-protection', 'model-protection', 'agent-protection']) {
    add('ContentTypeConfigurationsSchema', `${direction}-${field}-invalid`, { [direction]: { [field]: 7 } });
  }
}
for (const [index, value] of [null, false, 7, 'future', [], { retained: true }].entries()) add('ContentTypeConfigurationsSchema', `future-direction-${index}`, { 'future-direction': value });
for (const schema of ['ContentTypeConfigurationSchema', 'ModelConfigurationSchema']) {
  for (const field of ['data-protection', 'app-protection', 'model-protection', 'agent-protection']) fieldCases(schema, field, {}, [protection[field]]);
  for (const [label, input] of Object.entries<Json>({
    'dlp-missing-action': { 'data-protection': { 'data-leak-detection': { member: [] } } },
    'dlp-null-member': { 'data-protection': { 'data-leak-detection': { action: 'block', member: null } } },
    'dlp-member-missing-text': { 'data-protection': { 'data-leak-detection': { action: 'block', member: [{}] } } },
    'database-null': { 'data-protection': { 'database-security': null } },
    'agent-missing-name': { 'agent-protection': [{ action: 'block' }] },
    'model-topic-null': { 'model-protection': [{ 'topic-list': [{ action: 'block', topic: null }] }] },
    'model-topic-missing': { 'model-protection': [{ 'topic-list': [{ action: 'block' }] }] },
    'malicious-missing-action': { 'app-protection': { 'malicious-code-protection': { name: 'code' } } },
  })) add(schema, label, input);
}
// Exercise nested optional fields independently of structural schema reuse.
// Each mutation starts from a valid object with all required sibling fields.
const nestedBase: ObjectJson = {
  'data-protection': {
    'data-leak-detection': { action: 'block', member: [{ text: 'fixture', id: '100', version: '1' }], 'mask-data-inline': false },
    'database-security': [{ name: 'fixture', action: 'block', severity: 'high' }],
    'source-code-detection': { action: 'block', severity: 'high' },
  },
  'app-protection': {
    'alert-url-category': { member: ['fixture'] }, 'block-url-category': { member: ['fixture'] },
    'allow-url-category': { member: ['fixture'] }, 'default-url-category': { member: ['fixture'] },
    'url-detected-action': 'block', 'url-detected-severity': 'high',
    'malicious-code-protection': { name: 'fixture', action: 'block', severity: 'high' },
  },
  'model-protection': [{
    name: 'fixture', action: 'block', severity: 'high', options: [],
    'severity-by-confidence': { high: 'high', moderate: 'moderate' },
    'toxic-category-list': [{ category: 'fixture', action: 'block', 'severity-by-confidence': { high: 'high', moderate: 'moderate' } }],
    'topic-list': [{ action: 'block', topic: [{ topic_name: 'fixture', topic_id: '100', revision: 1, severity: 'high' }] }],
  }],
  'agent-protection': [{ name: 'fixture', action: 'block', severity: 'high' }],
};
function nestedCases(schema: string, base: ObjectJson, keys: (string | number)[]) {
  for (const [label, value] of Object.entries<Json>({ null: null, number: 7, boolean: false, array: [], object: {}, string: 'invalid' })) {
    const input = structuredClone(base);
    let parent: Json = input;
    for (const key of keys.slice(0, -1)) {
      parent = typeof key === 'number' ? (parent as Json[])[key] : (parent as ObjectJson)[key];
    }
    const last = keys[keys.length - 1];
    if (typeof last === 'number') (parent as Json[])[last] = value;
    else (parent as ObjectJson)[last] = value;
    add(schema, `nested-${keys.join('.')}-${label}`, input);
  }
}
const nestedPaths: (string | number)[][] = [
  ['data-protection', 'data-leak-detection'],
  ['data-protection', 'data-leak-detection', 'member'],
  ['data-protection', 'data-leak-detection', 'member', 0, 'id'],
  ['data-protection', 'data-leak-detection', 'member', 0, 'version'],
  ['data-protection', 'data-leak-detection', 'mask-data-inline'],
  ['data-protection', 'database-security', 0],
  ['data-protection', 'database-security', 0, 'severity'],
  ['data-protection', 'source-code-detection'],
  ['data-protection', 'source-code-detection', 'action'],
  ['data-protection', 'source-code-detection', 'severity'],
  ['app-protection', 'url-detected-action'],
  ['app-protection', 'url-detected-severity'],
  ['app-protection', 'malicious-code-protection'],
  ['app-protection', 'malicious-code-protection', 'severity'],
  ['model-protection', 0, 'name'], ['model-protection', 0, 'action'],
  ['model-protection', 0, 'severity'], ['model-protection', 0, 'options'],
  ['model-protection', 0, 'severity-by-confidence'],
  ['model-protection', 0, 'severity-by-confidence', 'high'],
  ['model-protection', 0, 'severity-by-confidence', 'moderate'],
  ['model-protection', 0, 'toxic-category-list'],
  ['model-protection', 0, 'toxic-category-list', 0],
  ['model-protection', 0, 'toxic-category-list', 0, 'category'],
  ['model-protection', 0, 'toxic-category-list', 0, 'action'],
  ['model-protection', 0, 'toxic-category-list', 0, 'severity-by-confidence'],
  ['model-protection', 0, 'toxic-category-list', 0, 'severity-by-confidence', 'high'],
  ['model-protection', 0, 'toxic-category-list', 0, 'severity-by-confidence', 'moderate'],
  ['model-protection', 0, 'topic-list'],
  ['model-protection', 0, 'topic-list', 0, 'topic', 0, 'severity'],
  ['agent-protection', 0, 'severity'],
];
for (const category of ['alert-url-category', 'block-url-category', 'allow-url-category', 'default-url-category']) {
  nestedPaths.push(['app-protection', category], ['app-protection', category, 'member'], ['app-protection', category, 'member', 0]);
}
for (const schema of ['ContentTypeConfigurationSchema', 'ModelConfigurationSchema']) {
  add(schema, 'nested-valid-base', nestedBase);
  for (const keys of nestedPaths) nestedCases(schema, nestedBase, keys);
}
const latencyBase = { latency: { 'inline-timeout-action': 'block', 'max-inline-latency': 5 } };
for (const field of ['inline-timeout-action', 'max-inline-latency']) nestedCases('ModelConfigurationSchema', latencyBase, ['latency', field]);
fieldCases('ModelConfigurationSchema', 'enable-full-conversation-inspection', {}, [true, false]);
fieldCases('ModelConfigurationSchema', 'mask-data-in-storage', {}, [true, false]);
fieldCases('ModelConfigurationSchema', 'latency', {}, [{ 'max-inline-latency': 5 }, { 'max-inline-latency': 1.5 }]);
fieldCases('AiSecurityProfileSchema', 'content-type-mode', {}, ['per_content_type', 'future-mode', '']);
fieldCases('AiSecurityProfileSchema', 'content-type-configurations', {}, [directions]);
fieldCases('AiSecurityProfileSchema', 'model-configuration', {}, [shared]);
add('AiSecurityProfileSchema', 'mixed-legacy-directional', { ...ai, 'model-configuration': { ...protection, ...shared } });
fieldCases('PolicySchema', 'ai-security-profiles', {}, [[ai], []]);
for (const schema of ['SecurityProfileSchema', 'CreateSecurityProfileRequestSchema']) {
  const base = { profile_name: 'fixture', policy: {} };
  fieldCases(schema, 'dlp_tenant_id', base, ['100', '']);
  fieldCases(schema, 'policy', base, [policy]);
  fieldCases(schema, 'profile_name', base, ['fixture']);
  fieldCases(schema, 'revision', base, [1, 1.5]);
  fieldCases(schema, 'last_modified_ts', base, ['2026-10-09T11:50:51Z', '2026-10-09T13:50:51+02:00']);
  add(schema, 'dlp-nested-unknown-location', { ...base, policy: { dlp_tenant_id: 7 } });
  for (const [label, invalidAi] of Object.entries<Json>({
    'direction-null': { 'content-type-configurations': { prompt: null } },
    'mode-number': { 'content-type-mode': 7 },
    'inspection-string': { 'model-configuration': { 'enable-full-conversation-inspection': 'false' } },
  })) {
    const nested = { 'ai-security-profiles': [invalidAi] };
    add(schema, label, { ...base, policy: nested });
    if (schema === 'SecurityProfileSchema') {
      add('PolicySchema', label, nested);
      add('SecurityProfileListResponseSchema', label, { ai_profiles: [{ ...base, policy: nested }] });
    }
  }
}
fieldCases('SecurityProfileListResponseSchema', 'ai_profiles', {}, [[fixture], []]);
add('SecurityProfileListResponseSchema', 'missing-list-unknown-preserved', { future: false });
add('SecurityProfileListResponseSchema', 'nested-dlp-type', { ai_profiles: [{ profile_name: 'fixture', dlp_tenant_id: 100 }] });
if (new Set(specimens.map(row => row.id)).size !== specimens.length) throw new Error('Duplicate oracle case ID');

await mkdir(path.join(repo, '.local'), { recursive: true });
const runtime = await mkdtemp(path.join(repo, '.local/typescript035-oracle-'));
try {
  await writeFile(path.join(runtime, 'package.json'), '{"type":"module"}\n');
  await symlink(path.join(repo, 'tools/node_modules'), path.join(runtime, 'node_modules'));
  const model = path.join(runtime, 'mgmt-security-profile.ts');
  await writeFile(model, await readFile(path.join(source, 'src/models/mgmt-security-profile.ts')));
  const schemas = await import(pathToFileURL(model).href) as Record<string, ZodTypeAny>;
  const cases = specimens.map(row => {
    if (!schemas[row.schema]?.safeParse) throw new Error(`Missing actual upstream schema ${row.schema}`);
    const result = schemas[row.schema].safeParse(row.input);
    const zod = result.success
      ? { success: true, output: result.data }
      : { success: false, issues: result.error.issues };
    return { ...row, zod };
  });
  const output = JSON.stringify({
    schemaVersion: 1,
    provenance: {
      sourceManifestSha256: sha(manifestBytes), lockfileSha256: sha(lockBytes),
      oracleSha256: sha(await readFile(fileURLToPath(import.meta.url))),
      sourceCommit: manifest.commit, zodVersion: locked,
    },
    roots, cases, outcomesSha256: sha(JSON.stringify(cases)),
  }, null, 2) + '\n';
  if (check) {
    if (await readFile(destination, 'utf8') !== output) throw new Error('Stale TypeScript 0.35.0 oracle corpus');
  } else await writeFile(destination, output);
  console.log(JSON.stringify({ roots: roots.length, cases: cases.length, accepted: cases.filter(row => row.zod.success).length }));
} finally {
  await rm(runtime, { recursive: true, force: true });
}
