#!/usr/bin/env node
/** Reproduce actual frozen Zod acceptance/normalization for Gateway wire specimens.
 * Inputs are literal source-derived fixtures, never native validator output. */
import { createHash } from 'node:crypto';
import { execFileSync } from 'node:child_process';
import { readFile, writeFile } from 'node:fs/promises';
import path from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';
import { isDeepStrictEqual } from 'node:util';

const repo = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const baseline = path.join(repo, '.local/typescript');
const destination = path.join(repo, 'contracts/native-fixtures/gateway-specimens.json');
const check = process.argv.includes('--check');
if (process.argv.slice(2).some((value) => value !== '--check')) throw new Error('Only --check is supported');
const sha = (bytes: string | Uint8Array) => createHash('sha256').update(bytes).digest('hex');
const inputFiles = [
  'tools/verify-gateway-specimens.mts', 'tools/verify-contracts.py',
  'tools/package.json', 'tools/package-lock.json', 'contracts/manifest.json',
  'contracts/typescript.json', 'crates/scm-sdk/tests/gateway_contracts.rs',
  'crates/scm-sdk/src/fixtures/gateway-contracts.json',
];
const inputs = await Promise.all(inputFiles.map(async (file) => ({
  file, sha256: sha(await readFile(path.join(repo, file))),
})));
// Verify complete frozen source and non-source input bytes before executing them.
execFileSync('python3', ['tools/verify-contracts.py', '--baseline', baseline], { cwd: repo, stdio: 'pipe' });
const inventory = JSON.parse(await readFile(path.join(repo, 'contracts/typescript.json'), 'utf8'));
if (inventory.baseline.commit !== '4a80dbdb3dc14365e1c25d838ac0bd139485d48c') throw new Error('Wrong baseline');
const test = await readFile(path.join(repo, 'crates/scm-sdk/tests/gateway_contracts.rs'), 'utf8');
const matches = [...test.matchAll(/const GOLDEN: &str = r#"([\s\S]*?)"#;/g)];
if (matches.length !== 1) throw new Error('Gateway specimen table must occur exactly once');
const specimens = JSON.parse(matches[0][1]);
if (!Array.isArray(specimens) || specimens.length !== 61) throw new Error('Incomplete Gateway specimen set');
const bundle = JSON.parse(await readFile(path.join(repo, 'crates/scm-sdk/src/fixtures/gateway-contracts.json'), 'utf8'));
const frozenCases = new Map(bundle.cases.map((entry: any) => {
  const family = entry.case.call.className === 'AIGatewayLogExportsClient' ? 'log_exports'
    : entry.case.call.className === 'AIGatewayRateLimitsClient' ? 'rate_limits' : null;
  if (!family) throw new Error('Unexpected independent fixture family');
  return [`${family}.${entry.case.call.member}`, entry];
}));
if (frozenCases.size !== 12) throw new Error('Incomplete independent specimen set');

// Neither credentials nor network services are available to imported callbacks.
for (const key of Object.keys(process.env)) delete process.env[key];
globalThis.fetch = async () => { throw new Error('Network forbidden in Gateway specimen checker'); };
Object.defineProperty(globalThis, 'WebSocket', { configurable: true, value: class {
  constructor() { throw new Error('Network forbidden in Gateway specimen checker'); }
} });
const roots = inventory.schemas.roots.filter((root: any) => root.source.file.startsWith('src/models/'));
const modules = new Map<string, any>();
const usedSources = new Map<string, string>();
const usedNames = new Set<string>();
const usedIndependent = new Set<string>();
let parses = 0;

async function parse(schemaName: string, input: any, operation: string, channel: string) {
  const candidates = roots.filter((root: any) => root.name === schemaName);
  if (candidates.length !== 1) throw new Error(`Ambiguous/missing frozen schema: ${schemaName}`);
  const root = candidates[0];
  const file = root.source.file;
  if (!modules.has(file)) modules.set(file, await import(pathToFileURL(path.join(baseline, file)).href));
  const schema = modules.get(file)[schemaName];
  if (!schema || typeof schema.safeParse !== 'function') throw new Error(`Schema is not publicly executable: ${schemaName}`);
  const actual = schema.safeParse(input);
  if (!actual.success) throw new Error(`Frozen Zod rejects ${operation}/${channel}: ${actual.error.issues.map((x: any) => x.code).join(',')}`);
  // Specimens are already normalized literals. Defaults/stripping/transforms must
  // agree exactly, rather than only accepting the same broad input type.
  if (!isDeepStrictEqual(actual.data, input)) throw new Error(`Frozen Zod normalization differs: ${operation}/${channel}`);
  const source = inventory.baseline.sources.find((entry: any) => entry.file === file);
  if (!source) throw new Error('Schema source lacks frozen provenance');
  usedSources.set(file, source.sha256);
  parses += 1;
  return { channel, schema: schemaName, schemaId: root.schema.$ref,
    source: { file, sha256: source.sha256, declarationSha256: root.source.sha256 },
    input, normalized: actual.data };
}

const results = [];
for (const specimen of specimens) {
  const name = specimen.name;
  if (typeof name !== 'string' || usedNames.has(name)) throw new Error('Duplicate/invalid specimen identity');
  usedNames.add(name);
  const independent: any = frozenCases.get(name);
  let requestSchema = specimen.requestSchema;
  let responseSchema = specimen.responseSchema;
  let querySchema: string | undefined;
  if (independent) {
    const frozen = independent.case;
    usedIndependent.add(name);
    for (const [key, value] of [['body', frozen.body ?? null], ['response', frozen.responseBody]]) {
      if (!isDeepStrictEqual(specimen[key as string], value)) throw new Error(`Independent ${name}/${key} differs`);
    }
    const prefix = frozen.call.className === 'AIGatewayLogExportsClient' ? 'GatewayLogExportsClient' : 'GatewayRateLimitsClient';
    const member = frozen.call.member;
    const capital = member[0].toUpperCase() + member.slice(1);
    if (specimen.body !== null) requestSchema = `${prefix}${capital}RequestSchema`;
    responseSchema = frozen.call.responseSchema;
    if (member === 'list' || (prefix === 'GatewayRateLimitsClient' && member === 'get')) {
      querySchema = `${prefix}${capital}OptionsSchema`;
    }
  }
  const channels = [];
  if (requestSchema) channels.push(await parse(requestSchema, specimen.body, name, 'request'));
  else if (specimen.body !== null) throw new Error(`Unaccounted request schema: ${name}`);
  if (querySchema) channels.push(await parse(querySchema, specimen.queryObject, name, 'query'));
  if (responseSchema) channels.push(await parse(responseSchema, specimen.response, name, 'response'));
  else if (specimen.response !== null) throw new Error(`Unaccounted response schema: ${name}`);
  results.push({ operation: name, fixtureSource: independent ? independent.source : 'literal-source-derived',
    responseContract: responseSchema ? 'json-schema' : 'unit-no-response-schema', channels });
}
if (usedIndependent.size !== frozenCases.size) throw new Error('Unused independent specimen');
const artifact = {
  schemaVersion: 1,
  purpose: 'Actual frozen Zod acceptance and exact normalization of literal Gateway test specimens; no Rust-derived oracle',
  license: 'Apache-2.0; see NOTICE for frozen TypeScript attribution',
  baseline: { commit: inventory.baseline.commit, sourceFingerprint: inventory.baseline.sourceFingerprint },
  inputs, sources: [...usedSources].sort(([a], [b]) => a.localeCompare(b)).map(([file, sha256]) => ({ file, sha256 })),
  counts: { operations: results.length, independentCases: usedIndependent.size, zodParses: parses },
  results,
};
const bytes = `${JSON.stringify(artifact, null, 2)}\n`;
if (check) {
  if (await readFile(destination, 'utf8') !== bytes) throw new Error('Gateway specimen proof is stale; regenerate with tools/verify-gateway-specimens.mts');
} else await writeFile(destination, bytes);
process.stdout.write(`${JSON.stringify({ checked: check, ...artifact.counts })}\n`);
