#!/usr/bin/env node
// Development-only pin, complete-delta classification and import extraction for
// Portkey-AI/openapi a27feb9..8e5da0e. Fails closed on any unclassified change.
import fs from 'node:fs';
import crypto from 'node:crypto';
import {isDeepStrictEqual as eq} from 'node:util';
import {createRequire} from 'node:module';

const require = createRequire(new URL('./package.json', import.meta.url));
const yaml = require('js-yaml');
const HEAD = '8e5da0e756bc49ee3e82ca13df693b39fa541627';
const BASE = 'a27feb9efaac3653d2d94eca5d7d210944b6e160';
const root = new URL('../', import.meta.url);
const dir = new URL(process.env.PORTKEY_MCP_TAGS_DIR ? `file://${process.env.PORTKEY_MCP_TAGS_DIR}/` : '../contracts/vendor/portkey-mcp-tags/8e5da0e/', import.meta.url);
const fail = message => { throw new Error(message); };
const read = url => fs.readFileSync(url);
const digest = (algorithm, data) => crypto.createHash(algorithm).update(data).digest('hex');
const blob = data => digest('sha1', Buffer.concat([Buffer.from(`blob ${data.length}\0`), data]));

// Reviewed byte identities, independent of the editable source.json manifest.
// The Git blob ids were confirmed against Portkey-AI/openapi contents at HEAD.
const ANCHORS = {
  'openapi.yaml': {sha256: '6013532cdc5f46dd18d57053ec240b3b723e8e60b5042b3ce8a7023d4127f50e', gitBlobSha1: '32d3b82b4e2311f0793967ea5effb418730f3b68', bytes: 1368261},
  LICENSE: {sha256: 'bcba3de214851cce46ed5af42d6698044616eeace887c3231bc7a20474ab639e', gitBlobSha1: '4f14854c3243516f3de1487a60f14882874d7e3a', bytes: 1083},
  base: {sha256: '9c3b5e643c0ecf0a8b63a2fa6fc68c15086b153657185e42a289012c35e4ca92', gitBlobSha1: '3804f18ba715e285d367f60481e6427179fd653f', bytes: 1342901},
};
const BASE_PATH = 'contracts/vendor/portkey-access/a27feb9/openapi.yaml';
const pin = JSON.parse(read(new URL('source.json', dir)));
if (pin.commit !== HEAD || pin.base?.commit !== BASE) fail('Unexpected pin identity');
if (pin.base.path !== BASE_PATH) fail('Base path drift');
if (pin.url !== 'https://github.com/Portkey-AI/openapi') fail('Upstream repository drift');
if (!eq(Object.keys(pin.files).sort(), ['LICENSE', 'openapi.yaml'])) fail('Pinned inventory drift');
const files = {...pin.files, base: pin.base};
for (const [name, info] of Object.entries(files)) {
  const url = name === 'base' ? new URL(info.path, root) : new URL(name, dir);
  const data = read(url);
  const anchor = ANCHORS[name];
  if (!anchor || info.sha256 !== anchor.sha256 || info.bytes !== anchor.bytes || info.gitBlobSha1 !== anchor.gitBlobSha1)
    fail('Pinned manifest differs from reviewed anchors: ' + name);
  if (digest('sha256', data) !== anchor.sha256 || data.length !== anchor.bytes || blob(data) !== anchor.gitBlobSha1)
    fail('Pinned file changed: ' + name);
  const ref = name === 'base' ? BASE : HEAD;
  const sourceName = name === 'base' ? 'openapi.yaml' : name;
  if (info.sourceUrl !== `https://raw.githubusercontent.com/Portkey-AI/openapi/${ref}/${sourceName}`) fail('Source URL drift: ' + name);
}
if (pin.licenseSha256 !== pin.files.LICENSE.sha256) fail('License pin drift');
const pkg = JSON.parse(read(new URL('./package.json', import.meta.url)));
const lock = JSON.parse(read(new URL('./package-lock.json', import.meta.url)));
if (require('js-yaml/package.json').version !== pin.yamlParser.version || pkg.dependencies['js-yaml'] !== pin.yamlParser.version ||
    lock.packages['node_modules/js-yaml'].version !== pin.yamlParser.version) fail('YAML parser drift');

const head = yaml.load(read(new URL('openapi.yaml', dir)).toString());
const base = yaml.load(read(new URL(pin.base.path, root)).toString());

// Complete structural delta. Every changed top-level key, path/method entry and
// component entry is one item; paths/components themselves are covered by items.
const METHODS = new Set(['get', 'put', 'post', 'delete', 'patch', 'options', 'head', 'trace']);
const change = (a, b) => a === undefined ? 'added' : b === undefined ? 'removed' : 'changed';
const items = [];
for (const key of new Set([...Object.keys(base), ...Object.keys(head)]))
  if (!['paths', 'components'].includes(key) && !eq(base[key], head[key])) items.push(`top:${key}:${change(base[key], head[key])}`);
for (const path of new Set([...Object.keys(base.paths), ...Object.keys(head.paths)])) {
  const a = base.paths[path] ?? {}, b = head.paths[path] ?? {};
  for (const key of new Set([...Object.keys(a), ...Object.keys(b)])) {
    if (eq(a[key], b[key])) continue;
    const kind = change(a[key], b[key]);
    const fields = kind === 'changed' && METHODS.has(key)
      ? ':' + [...new Set([...Object.keys(a[key]), ...Object.keys(b[key])])].filter(f => !eq(a[key][f], b[key][f])).sort().join(',')
      : '';
    items.push(`path:${key}:${path}:${kind}${fields}`);
  }
}
for (const group of new Set([...Object.keys(base.components), ...Object.keys(head.components)]))
  for (const name of new Set([...Object.keys(base.components[group] ?? {}), ...Object.keys(head.components[group] ?? {})]))
    if (!eq(base.components[group]?.[name], head.components[group]?.[name]))
      items.push(`component:${group}:${name}:${change(base.components[group]?.[name], head.components[group]?.[name])}`);
items.sort();

const ledger = JSON.parse(read(new URL('delta.json', dir)));
if (ledger.upstreamCommit !== HEAD || ledger.baseCommit !== BASE) fail('Delta ledger identity drift');
const DISPOSITIONS = new Set(['import', 'already_supported', 'not_applicable', 'deferred_new_capability', 'documentation_only']);
const classified = [];
for (const group of ledger.groups) {
  if (!DISPOSITIONS.has(group.disposition) || !group.reason?.trim() || !/^D\d+$/.test(group.id)) fail('Invalid delta group: ' + group.id);
  for (const item of group.items) classified.push(item);
}
const duplicates = classified.filter((item, index) => classified.indexOf(item) !== index);
if (duplicates.length) fail('Delta items classified more than once: ' + duplicates.join(', '));
const missing = items.filter(item => !classified.includes(item));
const stale = classified.filter(item => !items.includes(item));
if (missing.length || stale.length) fail(`Unclassified delta: ${JSON.stringify({missing, stale})}`);

// Imported schema changes must be exactly one added nullable string-map `tags`
// property; every other byte of those schemas is unchanged from the base.
const TAG_SCHEMAS = {
  write: ['CreateMcpIntegration', 'UpdateMcpIntegration', 'CreateMcpServer', 'UpdateMcpServer'],
  read: ['McpIntegration', 'McpIntegrationListItem', 'McpServer', 'McpServerListItem'],
  supported: ['IntegrationList'],
};
const importedSchemas = [...TAG_SCHEMAS.write, ...TAG_SCHEMAS.read];
for (const name of [...importedSchemas, ...TAG_SCHEMAS.supported]) {
  const before = base.components.schemas[name], after = structuredClone(head.components.schemas[name]);
  const tags = after.properties?.tags;
  if (!tags || before.properties?.tags) fail('Expected an added tags property: ' + name);
  const {description, ...shape} = tags;
  if (!eq(shape, {type: 'object', additionalProperties: {type: 'string'}, nullable: true}) || !description) fail('Tags shape drift: ' + name);
  delete after.properties.tags;
  if (!eq(after, before)) fail('Imported schema has an unreviewed change: ' + name);
}
const KEY_RULE = 'Keys may contain only alphanumeric characters, underscores and hyphens.';
for (const name of ['CreateMcpIntegration', 'CreateMcpServer'])
  if (!head.components.schemas[name].properties.tags.description.includes(KEY_RULE)) fail('Write key rule drift: ' + name);
for (const name of ['UpdateMcpIntegration', 'UpdateMcpServer'])
  if (!head.components.schemas[name].properties.tags.description.includes('Replaces existing tags; pass null to clear.')) fail('Update semantics drift: ' + name);

// Imported list operations differ only by one appended inline `tags` string query.
const LISTS = ['/mcp-integrations', '/mcp-servers'];
const TAGS_QUERY = {name: 'tags', in: 'query', schema: {type: 'string'},
  description: 'JSON-encoded object of tag key/value pairs; only resources whose tags match are returned, e.g. {"env":"prod"}'};
for (const path of LISTS) {
  const before = base.paths[path].get, after = structuredClone(head.paths[path].get);
  if (!eq(after.parameters.at(-1), TAGS_QUERY)) fail('Tags query drift: ' + path);
  after.parameters.pop();
  if (!eq(after, before)) fail('Imported list operation has an unreviewed change: ' + path);
}

// Non-imported route changes are also shape-checked so a classification cannot
// hide an unrelated edit to an operation the SDK might bind later.
const ANALYTICS = ledger.groups.find(g => g.id === 'D5').items.filter(item => item.startsWith('path:'));
const WORKSPACE_REFS = [{$ref: '#/components/parameters/IncludeAllWorkspaces'}, {$ref: '#/components/parameters/ExcludeWorkspaceSlugs'}];
for (const item of ANALYTICS) {
  const path = item.split(':')[2];
  const before = base.paths[path].get, after = structuredClone(head.paths[path].get);
  // Upstream inserts the adjacent reference pair exactly once per route.
  const at = after.parameters.findIndex(value => eq(value, WORKSPACE_REFS[0]));
  if (at < 0 || !eq(after.parameters.slice(at, at + 2), WORKSPACE_REFS) ||
      after.parameters.filter(value => WORKSPACE_REFS.some(ref => eq(ref, value))).length !== 2)
    fail('Analytics parameter delta drift: ' + path);
  after.parameters.splice(at, 2);
  if (!eq(after, before)) fail('Analytics route has an unreviewed change: ' + path);
}
const thread = structuredClone(head.paths['/threads/{thread_id}/messages'].post);
if (base.paths['/threads/{thread_id}/messages'].post.operationId !== 'createMessage' || thread.operationId !== 'createThreadMessage') fail('Thread rename drift');
thread.operationId = 'createMessage';
if (!eq(thread, base.paths['/threads/{thread_id}/messages'].post)) fail('Thread route has an unreviewed change');

const contract = {
  commit: HEAD, base: BASE,
  paths: Object.fromEntries(LISTS.map(path => [path, {get: head.paths[path].get}])),
  components: {schemas: Object.fromEntries(importedSchemas.map(name => [name, head.components.schemas[name]]))},
};
const output = JSON.stringify(contract, null, 2) + '\n';
const destination = new URL('contract.json', dir);
if (process.argv.includes('--check')) {
  if (!fs.existsSync(destination) || fs.readFileSync(destination, 'utf8') !== output) fail('Contract drift');
} else fs.writeFileSync(destination, output);

const operationsPath = new URL('operations.json', dir);
let bindings = 0;
if (fs.existsSync(operationsPath)) {
  const operations = JSON.parse(read(operationsPath));
  if (operations.upstreamCommit !== HEAD) fail('Operation ledger identity drift');
  for (const row of operations.bindings) {
    if (!classified.includes(row.delta)) fail('Binding names an unclassified delta item: ' + row.delta);
    const group = ledger.groups.find(g => g.items.includes(row.delta));
    if (group.disposition !== 'import') fail('Binding for a non-imported delta: ' + row.delta);
    const source = fs.readFileSync(new URL(row.source, root), 'utf8');
    const [testFile, testName] = row.test.split('::');
    if (!source.includes(row.evidence) || !fs.readFileSync(new URL(testFile, root), 'utf8').includes(`fn ${testName}(`))
      fail('Implementation/test binding drift: ' + row.delta);
    bindings += 1;
  }
  const imported = ledger.groups.filter(g => g.disposition === 'import').flatMap(g => g.items);
  const bound = new Set(operations.bindings.map(row => row.delta));
  const unbound = imported.filter(item => !bound.has(item));
  if (unbound.length) fail('Imported delta lacks an implementation binding: ' + unbound.join(', '));
}
console.log(JSON.stringify({passed: true, deltaItems: items.length,
  dispositions: Object.fromEntries([...DISPOSITIONS].map(d => [d, ledger.groups.filter(g => g.disposition === d).reduce((n, g) => n + g.items.length, 0)])),
  importedSchemas: importedSchemas.length, importedListOperations: LISTS.length, bindings}));
