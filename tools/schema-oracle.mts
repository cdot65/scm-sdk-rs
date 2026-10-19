#!/usr/bin/env node
/** Independent, credential-free schema fixtures. Expected results come from Zod,
 * never from the Rust validator or generated Rust models. */
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { readFile, writeFile, mkdir, rm, symlink } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { z, type ZodTypeAny } from 'zod';

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const baseline = path.join(repo, '.local/typescript');
const destination = path.join(repo, 'contracts/native-fixtures');
const runtime = path.join(repo, '.local/schema-oracle-runtime');
const check = process.argv.includes('--check');
if (process.argv.slice(2).some((value) => value !== '--check')) throw new Error('Only --check is supported');
const sha = (bytes: string | Uint8Array) => createHash('sha256').update(bytes).digest('hex');
const json = (value: unknown) => `${JSON.stringify(value, null, 2)}\n`;
const compact = (value: unknown) => JSON.stringify(value);
const inventoryBytes = await readFile(path.join(repo, 'contracts/typescript.json'));
const inventory = JSON.parse(inventoryBytes.toString());
const manifestBytes = await readFile(path.join(repo, 'contracts/manifest.json'));
const manifest = JSON.parse(manifestBytes.toString());
// This reconstructs the frozen Git tree and verifies every src and extraction input.
execFileSync('python3', ['tools/verify-contracts.py', '--baseline', baseline], { cwd: repo, stdio: 'pipe' });
if (inventory.baseline.commit !== '4a80dbdb3dc14365e1c25d838ac0bd139485d48c') throw new Error('Unexpected baseline');
const inputHashes = [];
for (const file of ['tools/package.json', 'tools/package-lock.json']) {
  inputHashes.push({ file, sha256: sha(await readFile(path.join(repo, file))) });
}

type Root = { name: string; source: { file: string }; schema: { $ref: string } };
const declarations: Root[] = inventory.schemas.roots;
const byFile = new Map<string, Root[]>();
for (const root of declarations) byFile.set(root.source.file, [...(byFile.get(root.source.file) ?? []), root]);
await rm(runtime, { recursive: true, force: true });
try {
await mkdir(runtime, { recursive: true });
await writeFile(path.join(runtime, 'package.json'), '{"type":"module"}\n');
for (const entry of manifest.baseline.srcFiles) {
  const file = entry.upstreamPath;
  const contents = await readFile(path.join(baseline, file), 'utf8');
  const exports = (byFile.get(file) ?? []).map((root, index) => `export { ${root.name} as __oracle${index} };`).join('\n');
  const output = path.join(runtime, file);
  await mkdir(path.dirname(output), { recursive: true });
  await writeFile(output, `${contents}\n${exports}\n`);
}
await mkdir(path.join(runtime, 'node_modules'), { recursive: true });
await symlink(path.join(baseline, 'node_modules/zod'), path.join(runtime, 'node_modules/zod'));

// Frozen schema callbacks can inspect no credentials or make network requests.
for (const key of Object.keys(process.env)) delete process.env[key];
globalThis.fetch = async () => { throw new Error('Network forbidden in schema oracle'); };
Object.defineProperty(globalThis, 'WebSocket', { configurable: true, value: class {
  constructor() { throw new Error('Network forbidden in schema oracle'); }
} });
const schemas = new Map<string, ZodTypeAny>();
for (const [file, roots] of byFile) {
  const module = await import(pathToFileURL(path.join(runtime, file)).href);
  for (const [index, root] of roots.entries()) {
    const schema = module[`__oracle${index}`] as ZodTypeAny;
    if (!schema || typeof schema.safeParse !== 'function') throw new Error(`Missing frozen schema: ${file}:${root.name}`);
    const previous = schemas.get(root.schema.$ref);
    if (previous && previous !== schema) throw new Error(`Schema identity changed: ${root.schema.$ref}`);
    schemas.set(root.schema.$ref, schema);
  }
}

// Bind every recorded schema node to the corresponding actual runtime object.
// This gives the 69 semantic nodes their own oracle samples, including private
// callbacks/defaults nested below a public root.
const runtimeNodes = new Map<string, ZodTypeAny>();
function bindRuntime(schema: ZodTypeAny, sid: string): void {
  const previous = runtimeNodes.get(sid);
  if (previous) { if (previous !== schema) throw new Error(`Node identity mismatch: ${sid}`); return; }
  const graph = inventory.schemas.definitions[sid]; const def: any = schema._def;
  if (!graph || graph.kind !== def.typeName) throw new Error(`Node kind mismatch: ${sid}`);
  runtimeNodes.set(sid, schema);
  const bind = (value: ZodTypeAny, ref: { $ref: string }) => bindRuntime(value, ref.$ref);
  switch (def.typeName) {
    case 'ZodObject':
      bind(def.catchall, graph.catchall);
      for (const [key, value] of Object.entries(def.shape())) bind(value as ZodTypeAny, graph.properties[key].schema);
      break;
    case 'ZodArray': bind(def.type, graph.element); break;
    case 'ZodOptional': case 'ZodNullable': case 'ZodReadonly': case 'ZodDefault': case 'ZodCatch': bind(def.innerType, graph.inner); break;
    case 'ZodBranded': bind(def.type, graph.inner); break;
    case 'ZodEffects': bind(def.schema, graph.inner); break;
    case 'ZodLazy': bind(def.getter(), graph.target); break;
    case 'ZodPipeline': bind(def.in, graph.input); bind(def.out, graph.output); break;
    case 'ZodRecord': bind(def.keyType, graph.key); bind(def.valueType, graph.value); break;
    case 'ZodIntersection': bind(def.left, graph.left); bind(def.right, graph.right); break;
    case 'ZodUnion': case 'ZodDiscriminatedUnion':
      for (const [index, value] of def.options.entries()) bind(value, graph.variants[index]);
      break;
  }
}
for (const [sid, schema] of schemas) bindRuntime(schema, sid);
if (runtimeNodes.size !== inventory.counts.schemaNodes) throw new Error('Incomplete runtime node binding');
const moduleSids = new Set(schemas.keys());
const nativeSids = Object.entries(inventory.schemas.definitions).filter(([, node]: any) =>
  node.kind === 'ZodDate' || /(?:instanceof\(Blob\)|AbortSignal|globalThis\.fetch)/.test(node.semanticSource?.source?.text ?? '')).map(([sid]) => sid);
for (const sid of [...inventory.schemas.manualSemantics.map((entry: any) => entry.schema), ...nativeSids]) {
  const actual = runtimeNodes.get(sid); if (!actual) throw new Error(`Missing semantic runtime node ${sid}`);
  schemas.set(sid, actual);
}

function isJson(value: unknown, seen = new Set<object>()): boolean {
  if (value === null || typeof value === 'string' || typeof value === 'boolean') return true;
  if (typeof value === 'number') return Number.isFinite(value);
  if (!value || typeof value !== 'object' || seen.has(value)) return false;
  if (!Array.isArray(value) && ![Object.prototype, null].includes(Object.getPrototypeOf(value))) return false;
  const branch = new Set(seen).add(value);
  return Object.values(value).every((child) => isJson(child, branch));
}
function describeNative(value: any): any {
  if (value === undefined) return { nativeType: 'undefined' };
  if (typeof value === 'function') return { nativeType: 'callback', invoked: false };
  if (value instanceof Blob) return { nativeType: 'Blob', bytes: value.size, contentType: value.type };
  if (value instanceof Date) return { nativeType: 'Date', iso: value.toISOString() };
  if (value instanceof AbortSignal) return { nativeType: 'AbortSignal', aborted: value.aborted };
  if (Array.isArray(value)) return value.map(describeNative);
  if (value && typeof value === 'object') return Object.fromEntries(Object.entries(value).map(([key, child]) => [key, describeNative(child)]));
  if (typeof value === 'number' && !Number.isFinite(value)) return { nativeType: 'nonfinite-number', value: String(value) };
  return value;
}
function parsed(schema: ZodTypeAny, input: any): { success: boolean; output?: any; issues?: any[] } {
  const result = schema.safeParse(input);
  if (result.success) return { success: true, output: result.data };
  return { success: false, issues: result.error.issues.map((issue: any) => ({ code: issue.code, path: issue.path })) };
}
const primitiveSeeds: any[] = ['fixture', '1', 'fixture__model', '550e8400-e29b-41d4-a716-446655440000',
  '2026-01-02T03:04:05Z', 'https://example.invalid/resource', 'fixture@example.invalid', '', 0, 1, -1,
  false, true, null, {}, [], { type: 'fixture.event' }, { key: 'fixture' }];
const nativeSeeds: any[] = [new Blob(['fixture'], { type: 'application/octet-stream' }),
  new Date('2026-01-02T03:04:05Z'), new AbortController().signal, () => { throw new Error('Fixture callback must not be invoked'); }];

function seed(schema: ZodTypeAny, full: boolean, depth = 0, active = new Set<ZodTypeAny>()): any {
  const def: any = schema._def; const kind = def.typeName;
  if (depth > 20 || active.has(schema)) {
    for (const value of [null, [], {}, 'fixture']) if (schema.safeParse(value).success) return value;
    return undefined;
  }
  const next = new Set(active).add(schema);
  const child = (value: ZodTypeAny) => seed(value, full, depth + 1, next);
  let result: any;
  switch (kind) {
    case 'ZodString': {
      const min = Math.max(0, ...(def.checks ?? []).filter((v: any) => v.kind === 'min').map((v: any) => v.value));
      const max = Math.min(4096, ...(def.checks ?? []).filter((v: any) => v.kind === 'max').map((v: any) => v.value));
      for (const value of [...primitiveSeeds.filter((v) => typeof v === 'string'), 'x'.repeat(Math.min(max, Math.max(min, 1)))]) {
        if (schema.safeParse(value).success) return value;
      }
      return 'fixture';
    }
    case 'ZodNumber': {
      const checks = def.checks ?? [];
      const candidates = [0, 1, -1, 0.5, ...checks.flatMap((v: any) => typeof v.value === 'number' ? [v.value, v.value + 1, v.value - 1] : [])];
      for (const value of candidates) if (schema.safeParse(value).success) return value;
      return 1;
    }
    case 'ZodBoolean': return false;
    case 'ZodNull': return null;
    case 'ZodDate': return nativeSeeds[1];
    case 'ZodLiteral': return def.value;
    case 'ZodEnum': return def.values[0];
    case 'ZodNativeEnum': return Object.keys(def.values).filter((key) => typeof def.values[def.values[key]] !== 'number').map((key) => def.values[key])[0];
    case 'ZodAny': case 'ZodUnknown': return full ? { fixture_key: 'fixture' } : null;
    case 'ZodNever': return undefined;
    case 'ZodOptional': return !full && depth > 0 ? undefined : child(def.innerType);
    case 'ZodNullable': return !full ? null : child(def.innerType);
    case 'ZodDefault': return !full && depth > 0 ? undefined : child(def.innerType);
    case 'ZodReadonly': return child(def.innerType);
    case 'ZodBranded': return child(def.type);
    case 'ZodLazy': return child(def.getter());
    case 'ZodPipeline': return child(def.in);
    case 'ZodEffects': {
      result = child(def.schema);
      if (schema.safeParse(result).success) return result;
      for (const value of [...primitiveSeeds, ...nativeSeeds]) if (schema.safeParse(value).success) return value;
      return result;
    }
    case 'ZodArray': {
      const count = def.exactLength?.value ?? Math.max(def.minLength?.value ?? 0, full ? 1 : 0);
      return Array.from({ length: count }, () => child(def.type));
    }
    case 'ZodRecord': return full ? { fixture: child(def.valueType) } : {};
    case 'ZodObject': {
      result = {};
      for (const [key, value] of Object.entries(def.shape())) {
        const candidate = child(value as ZodTypeAny);
        if (candidate !== undefined) result[key] = candidate;
      }
      return result;
    }
    case 'ZodUnion': case 'ZodDiscriminatedUnion': {
      for (const variant of def.options) {
        const candidate = child(variant);
        if (schema.safeParse(candidate).success) return candidate;
      }
      return child(def.options[0]);
    }
    case 'ZodIntersection': {
      const left = child(def.left); const right = child(def.right);
      return left && right && typeof left === 'object' && typeof right === 'object' && !Array.isArray(left) && !Array.isArray(right)
        ? { ...left, ...right } : left;
    }
    default: throw new Error(`Unhandled schema kind: ${kind}`);
  }
}

// Response expectations use real Zod execution with only the explicitly accepted
// Rust policy adaptations. Frozen request semantics above remain untouched.
const responseMemo = new Map<ZodTypeAny, ZodTypeAny>();
function responseSchema(schema: ZodTypeAny): ZodTypeAny {
  const existing = responseMemo.get(schema); if (existing) return existing;
  const def: any = schema._def; const kind = def.typeName;
  let value: ZodTypeAny;
  if (kind === 'ZodEnum') value = z.union([schema, z.string()]);
  else if (kind === 'ZodNativeEnum') {
    const values = Object.keys(def.values).filter((key) => typeof def.values[def.values[key]] !== 'number').map((key) => def.values[key]);
    const types = [...new Set(values.map((v) => typeof v))];
    value = schema;
    if (types.includes('string')) value = value.or(z.string());
    if (types.includes('number')) value = value.or(z.number());
  } else if (kind === 'ZodUnion' && def.options.every((v: any) => v._def.typeName === 'ZodLiteral' && typeof v._def.value === 'string')) value = z.string();
  else {
    const cloned = { ...def };
    // Lazy indirection breaks recursive identity before any children are visited.
    const placeholder = z.lazy(() => value); responseMemo.set(schema, placeholder);
    if (kind === 'ZodString') cloned.checks = def.checks.filter((check: any) => !['uuid','datetime','email','url','regex','date','duration','rfc3339'].includes(check.kind));
    else if (kind === 'ZodObject') {
      cloned.unknownKeys = 'passthrough';
      cloned.shape = () => Object.fromEntries(Object.entries(def.shape()).map(([key, child]) => [key, responseSchema(child as ZodTypeAny)]));
      cloned.catchall = responseSchema(def.catchall);
    } else if (['ZodOptional', 'ZodNullable', 'ZodDefault', 'ZodReadonly', 'ZodCatch'].includes(kind)) cloned.innerType = responseSchema(def.innerType);
    else if (kind === 'ZodEffects') cloned.schema = responseSchema(def.schema);
    else if (kind === 'ZodArray') cloned.type = responseSchema(def.type);
    else if (kind === 'ZodRecord') { cloned.keyType = responseSchema(def.keyType); cloned.valueType = responseSchema(def.valueType); }
    else if (kind === 'ZodLazy') cloned.getter = () => responseSchema(def.getter());
    else if (kind === 'ZodUnion') cloned.options = def.options.map(responseSchema);
    else if (kind === 'ZodDiscriminatedUnion') {
      value = z.discriminatedUnion(def.discriminator, def.options.map(responseSchema) as any);
      responseMemo.set(schema, value); return value;
    }
    else if (kind === 'ZodIntersection') { cloned.left = responseSchema(def.left); cloned.right = responseSchema(def.right); }
    else if (kind === 'ZodPipeline') { cloned.in = responseSchema(def.in); cloned.out = responseSchema(def.out); }
    value = new (schema.constructor as any)(cloned);
  }
  responseMemo.set(schema, value); return value;
}

function underlying(schema: ZodTypeAny): ZodTypeAny {
  const def: any = schema._def;
  if (['ZodOptional', 'ZodNullable', 'ZodDefault', 'ZodReadonly'].includes(def.typeName)) return underlying(def.innerType);
  if (def.typeName === 'ZodEffects') return underlying(def.schema);
  if (def.typeName === 'ZodLazy') return underlying(def.getter());
  return schema;
}
const externalSeeds = new Map<string, any[]>();
for (const fixture of manifest.fixtures) {
  if (!fixture.upstreamPath.startsWith('test/openapi/fixtures/')) continue;
  const data = JSON.parse(await readFile(path.join(repo, fixture.path), 'utf8'));
  for (const entry of data.cases ?? []) {
    for (const [name, value] of [[entry.call?.requestSchema, entry.body], [entry.call?.responseSchema, entry.responseBody]]) {
      if (name && value !== undefined) externalSeeds.set(name, [...(externalSeeds.get(name) ?? []), value]);
    }
  }
}

const roots = [];
const nativeBoundaries = [];
for (const [sid, schema] of [...schemas].sort(([a], [b]) => a.localeCompare(b))) {
  const names = inventory.exports.filter((entry: any) => entry.kind === 'schema' && entry.schema.$ref === sid).map((entry: any) => entry.name);
  const modules = declarations.filter((entry) => entry.schema.$ref === sid).map((entry) => ({ file: entry.source.file, name: entry.name }));
  const candidates: { label: string; input: any }[] = [];
  const add = (label: string, input: any) => { candidates.push({ label, input }); };
  add('minimal', seed(schema, false)); add('populated', seed(schema, true));
  const object = underlying(schema);
  if (object._def.typeName === 'ZodObject' && !schema.safeParse(candidates[0].input).success) {
    for (const [key, child] of Object.entries((object._def as any).shape())) {
      add(`single-field:${key}`, { [key]: seed(child as ZodTypeAny, false) });
    }
  }
  for (const name of names) for (const input of externalSeeds.get(name) ?? []) add('pinned-openapi-specimen', input);
  for (const input of [null, {}, [], '', 'fixture', 0, 1, -1, true, false]) add('root-type-or-null', input);
  if (inventory.schemas.definitions[sid]?.semanticSource?.source?.text?.includes('isFiniteJsonValue')) {
    for (const key of ['__proto__', 'constructor', 'prototype']) add(`prototype-unsafe:${key}`, JSON.parse(`{"${key}":{"fixture":"rejected"}}`));
  }
  const baseCandidates = candidates.filter((candidate) => schema.safeParse(candidate.input).success);
  let valid = baseCandidates.find((candidate) => isJson(candidate.input))?.input;
  if (valid === undefined) valid = baseCandidates[0]?.input;
  const baseSchema = underlying(schema); const def: any = baseSchema._def;
  if (def.typeName === 'ZodObject' && valid && typeof valid === 'object' && !Array.isArray(valid)) {
    for (const [key, childValue] of Object.entries(def.shape())) {
      const child = childValue as ZodTypeAny; const childDef: any = underlying(child)._def;
      const missing = { ...valid }; delete missing[key]; add(`omit:${key}`, missing);
      for (const value of [null, {}, [], '', ' ', '__future_enum__', -1, 0.5, true]) add(`field:${key}:type-null-boundary`, { ...valid, [key]: value });
      for (const check of childDef.checks ?? []) {
        if (typeof check.value === 'number' && ['min', 'max'].includes(check.kind)) {
          if (childDef.typeName === 'ZodString' && check.value < 8192) add(`field:${key}:string-${check.kind}`, { ...valid, [key]: 'x'.repeat(Math.max(0, check.value + (check.kind === 'min' ? -1 : 1))) });
          if (childDef.typeName === 'ZodNumber') add(`field:${key}:number-${check.kind}`, { ...valid, [key]: check.value + (check.kind === 'min' ? -1 : 1) });
        }
      }
      if (childDef.typeName === 'ZodArray') add(`field:${key}:empty-array`, { ...valid, [key]: [] });
    }
    add('unknown-object-field', { ...valid, future_field: { nested: 'preserve-me' } });
    for (const [lower, upper] of [['start_time', 'end_time'], ['start', 'end'], ['totalUnitsMin', 'totalUnitsMax'], ['costMin', 'costMax']]) {
      if (lower in def.shape() && upper in def.shape()) {
        add(`cross-field:${lower}>${upper}`, { ...valid, [lower]: lower.includes('time') ? '2026-01-03T00:00:00Z' : 10,
          [upper]: upper.includes('time') ? '2026-01-01T00:00:00Z' : 1 });
      }
    }
    for (const [first, second, firstValue, secondValue] of [
      ['periodic_reset', 'periodic_reset_days', 'monthly', 1],
      ['rotation_period', 'next_rotation_at', 'monthly', '2026-01-01T00:00:00Z'],
      ['allow_all_workspaces', 'allowed_workspaces', true, ['fixture-workspace']],
    ] as const) {
      if (first in def.shape() && second in def.shape()) add(`cross-field:mutually-exclusive:${first}:${second}`,
        { ...valid, [first]: firstValue, [second]: secondValue });
    }
    if ('manager_type' in def.shape() && 'auth_config' in def.shape()) {
      add('cross-field:manager-auth-mismatch', { ...valid, manager_type: 'azure_kv' });
    }
  }
  if (def.typeName === 'ZodUnion' || def.typeName === 'ZodDiscriminatedUnion') {
    for (const [index, variant] of def.options.entries()) { add(`variant:${index}:minimal`, seed(variant, false)); add(`variant:${index}:populated`, seed(variant, true)); }
  }
  const unique = new Set<string>(); const jsonCases = []; const nativeCases = [];
  const adapted = responseSchema(schema);
  for (const candidate of candidates) {
    if (!isJson(candidate.input)) {
      const result = parsed(schema, candidate.input);
      nativeCases.push({ label: candidate.label, input: describeNative(candidate.input), zod: result.success
        ? { success: true, output: describeNative(result.output) } : result });
      continue;
    }
    const key = compact(candidate.input); if (unique.has(key)) continue; unique.add(key);
    const request = parsed(schema, candidate.input); const response = parsed(adapted, candidate.input);
    if (request.success && !isJson(request.output) || response.success && !isJson(response.output)) {
      nativeCases.push({ label: candidate.label, input: candidate.input, zod: request.success
        ? { success: true, output: describeNative(request.output) } : request });
      continue;
    }
    jsonCases.push({ label: candidate.label, input: candidate.input, zod: request, response });
  }
  // Missing root values and real JS classes remain explicitly outside Value's domain.
  for (const input of [undefined, ...nativeSeeds]) {
    const result = parsed(schema, input);
    if (result.success || input === undefined) nativeCases.push({ label: 'native-boundary', input: describeNative(input), zod: result.success
      ? { success: true, output: describeNative(result.output) } : result });
  }
  const accepted = jsonCases.filter((entry) => entry.zod.success).length;
  const rejected = jsonCases.length - accepted;
  const entry = { sid, names, scope: moduleSids.has(sid) ? 'module-root' : 'semantic-or-native-node', moduleRoots: modules, jsonCases, nativeCases,
    coverage: { acceptedJsonCases: accepted, rejectedJsonCases: rejected,
      status: accepted ? 'json-covered' : nativeCases.some((entry) => entry.zod.success) ? 'requires-native-input' : 'unresolved-valid-specimen' } };
  roots.push(entry);
  if (nativeCases.length) nativeBoundaries.push({ sid, names, cases: nativeCases });
}

const counts = { publicSchemaExports: inventory.counts.publicSchemas, moduleRoots: declarations.length,
  uniqueRoots: moduleSids.size, oracleEntries: roots.length, manualSemanticNodes: inventory.schemas.manualSemantics.length,
  nativeBoundaryNodes: nativeSids.length,
  jsonCoveredRoots: roots.filter((entry) => entry.scope === 'module-root' && entry.coverage.status === 'json-covered').length,
  requiresNativeInput: roots.filter((entry) => entry.scope === 'module-root' && entry.coverage.status === 'requires-native-input').map((entry) => ({ sid: entry.sid, names: entry.names })),
  unresolvedValidSpecimens: roots.filter((entry) => entry.coverage.status === 'unresolved-valid-specimen').map((entry) => ({ sid: entry.sid, names: entry.names })),
  jsonCases: roots.reduce((sum, entry) => sum + entry.jsonCases.length, 0),
  acceptedJsonCases: roots.reduce((sum, entry) => sum + entry.coverage.acceptedJsonCases, 0),
  rejectedJsonCases: roots.reduce((sum, entry) => sum + entry.coverage.rejectedJsonCases, 0),
  rootsWithoutJsonRejection: roots.filter((entry) => entry.coverage.rejectedJsonCases === 0).map((entry) => ({ sid: entry.sid, names: entry.names })) };
const output = { schemaVersion: 1, baseline: inventory.baseline,
  provenance: { inventorySha256: sha(inventoryBytes), manifestSha256: sha(manifestBytes),
    generatorSha256: sha(await readFile(fileURLToPath(import.meta.url))), tooling: inputHashes,
    zod: JSON.parse(await readFile(path.join(baseline, 'node_modules/zod/package.json'), 'utf8')).version },
  policies: { request: 'Original frozen Zod safeParse; no Rust implementation is invoked.',
    response: 'Actual Zod execution with explicit Rust adaptations: preserve unknown object fields; widen enum/string-literal-union vocabularies; bypass string-format checks uuid, rfc3339, datetime, date, duration, email, url and regex while retaining frozen min/max bounds; preserve required types, individual literals, catchalls, refinements, defaults and transforms.',
    native: 'Native descriptors report real JS inputs and safeParse observations. They are not JSON conformance passes; Rust typed boundary equivalents must be tested separately.' }, counts,
  semanticBindings: inventory.schemas.manualSemantics.map((entry: any) => ({ sid: entry.schema, kind: entry.kind,
    expression: entry.originalExpression?.source ?? null, oracleSid: entry.schema })),
  nativeBindings: nativeSids.map((sid) => ({ sid, kind: inventory.schemas.definitions[sid].kind,
    expression: inventory.schemas.definitions[sid].semanticSource?.source ?? null,
    mapping: inventory.schemas.definitions[sid].kind === 'ZodDate' ? 'Rust native date/time input; JSON strings are not JS Date instances.'
      : /Blob/.test(inventory.schemas.definitions[sid].semanticSource?.source?.text ?? '') ? 'Rust native binary/multipart part; JSON objects are not Blob instances.'
        : /AbortSignal/.test(inventory.schemas.definitions[sid].semanticSource?.source?.text ?? '') ? 'Rust RequestOptions cancellation; JSON objects are not AbortSignal instances.'
          : 'Rust caller-owned Transport integration; JSON objects are not callable fetch functions.' })), roots };
await mkdir(destination, { recursive: true });
const file = path.join(destination, 'schemas.json'); const bytes = json(output);
if (check) { if (await readFile(file, 'utf8') !== bytes) throw new Error('Schema oracle fixtures differ from frozen Zod execution'); }
else await writeFile(file, bytes);
console.log(json(counts));
} finally {
  await rm(runtime, { recursive: true, force: true });
}
