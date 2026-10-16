#!/usr/bin/env -S npx tsx
/**
 * Freeze client implementation inputs, without evaluating request builders or emitting Rust.
 * Run: .local/typescript/node_modules/.bin/tsx tools/extract-operations.mts [--check]
 * Baseline source must match contracts/manifest.json. Unsupported syntax is retained as
 * evidence and blocks declarative generation; it never becomes a generic JSON operation.
 */
import { createHash } from 'node:crypto';
import { existsSync, readFileSync, writeFileSync } from 'node:fs';
import { createRequire } from 'node:module';
import { dirname, join, relative, resolve } from 'node:path';
import { fileURLToPath, pathToFileURL } from 'node:url';

const repo = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const baselineRoot = join(repo, '.local/typescript');
type Node = import('../.local/typescript/node_modules/typescript/lib/typescript.js').Node;
type Expression = import('../.local/typescript/node_modules/typescript/lib/typescript.js').Expression;
type Method = import('../.local/typescript/node_modules/typescript/lib/typescript.js').MethodDeclaration;
type SourceFile = import('../.local/typescript/node_modules/typescript/lib/typescript.js').SourceFile;
type IR = Record<string, unknown>;
const digest = (value: string | Buffer) => createHash('sha256').update(value).digest('hex');
const compareText = (left: string, right: string) => left < right ? -1 : left > right ? 1 : 0;
const readJson = (path: string) => JSON.parse(readFileSync(path, 'utf8'));
const manifest = readJson(join(repo, 'contracts/manifest.json'));
if (manifest.schemaVersion !== 1 || manifest.baseline.commit !== '4a80dbdb3dc14365e1c25d838ac0bd139485d48c')
  throw new Error('Frozen provenance schema or baseline commit mismatch.');
const ledgerBytes = readFileSync(join(repo, manifest.baseline.gitTreeLedger));
if (digest(ledgerBytes) !== manifest.baseline.gitTreeLedgerSha256) throw new Error('Frozen Git tree ledger hash mismatch.');
const ledger = JSON.parse(ledgerBytes.toString('utf8'));
if (ledger.commit !== manifest.baseline.commit) throw new Error('Frozen Git tree ledger commit mismatch.');
const extractionInputs: { upstreamPath: string; sha256: string; gitBlobSha1: string; bytes: number }[] = manifest.baseline.extractionInputs;
if (!Array.isArray(extractionInputs) || !['package.json', 'tsconfig.json', 'scripts/openapi/inventory.ts'].every((path) => extractionInputs.some((input) => input.upstreamPath === path)))
  throw new Error('Frozen compiler/package/auditor inputs are absent from provenance.');
for (const input of extractionInputs) {
  const entry = ledger.files.find((file: { path: string }) => file.path === input.upstreamPath);
  const bytes = readFileSync(join(baselineRoot, input.upstreamPath));
  const blob = createHash('sha1').update(`blob ${bytes.length}\0`).update(bytes).digest('hex');
  if (!entry || input.gitBlobSha1 !== entry.gitBlobSha1 || input.bytes !== entry.bytes || bytes.length !== entry.bytes ||
    blob !== entry.gitBlobSha1 || digest(bytes) !== input.sha256) throw new Error(`Frozen extraction input mismatch: ${input.upstreamPath}`);
}
const require = createRequire(join(baselineRoot, 'package.json'));
const ts: typeof import('../.local/typescript/node_modules/typescript/lib/typescript.js') = require('typescript');
const output = join(repo, 'contracts/operations.json');
const sourcePaths: string[] = manifest.baseline.srcFiles.map((entry: { upstreamPath: string }) => entry.upstreamPath);
const bySource = new Map<string, { sha256: string }>(manifest.baseline.srcFiles.map((entry: { upstreamPath: string; sha256: string }) => [entry.upstreamPath, entry]));
for (const file of sourcePaths) {
  const path = join(baselineRoot, file);
  if (!existsSync(path) || digest(readFileSync(path)) !== bySource.get(file)?.sha256)
    throw new Error(`Frozen source mismatch: ${file}. Restore the pinned baseline before extraction.`);
}
const inventoryReference = manifest.references.find((entry: { upstreamPath: string }) => entry.upstreamPath === 'scripts/openapi/inventory.ts');
if (!inventoryReference || digest(readFileSync(join(baselineRoot, inventoryReference.upstreamPath))) !== inventoryReference.sha256)
  throw new Error('Frozen inventory tooling hash does not match the provenance manifest.');
// This existing auditor evaluates only its constants module, not request builders. Its
// call-site output is secondary evidence: it omits spreads and can flatten workflows.
for (const name of Object.keys(process.env)) delete process.env[name];
globalThis.fetch = async () => { throw new Error('Network is forbidden during operation extraction'); };
if ('WebSocket' in globalThis) Object.defineProperty(globalThis, 'WebSocket', {
  configurable: true, value: class { constructor() { throw new Error('Network is forbidden during operation extraction'); } },
});
const { collectCallSites } = await import(pathToFileURL(join(baselineRoot, 'scripts/openapi/inventory.ts')).href);
const existingCalls: Record<string, unknown>[] = collectCallSites();
const configFile = ts.readConfigFile(join(baselineRoot, 'tsconfig.json'), ts.sys.readFile);
if (configFile.error) throw new Error('Cannot read frozen TypeScript compiler configuration.');
const config = ts.parseJsonConfigFileContent(configFile.config, ts.sys, baselineRoot);
const program = ts.createProgram(sourcePaths.map((path) => join(baselineRoot, path)), config.options);
const checker = program.getTypeChecker();
const sources = sourcePaths.map((path) => program.getSourceFile(join(baselineRoot, path))!);
if (sources.some((source) => !source)) throw new Error('A frozen source file was not loaded by the compiler.');
const sourceName = (source: SourceFile) => relative(baselineRoot, source.fileName).replaceAll('\\', '/');
const code = (node: Node) => node.getText(node.getSourceFile());
const normalizedType = (value: string) => value.replaceAll(baselineRoot.replaceAll('\\', '/'), '<baseline>').replace(/import\("[^"]*\/node_modules\/[^"]*\/([^/"]+)"\)/g, 'import("<dependency>/$1")');
const typeText = (node: Node) => normalizedType(checker.typeToString(checker.getTypeAtLocation(node), node, ts.TypeFormatFlags.NoTruncation | ts.TypeFormatFlags.UseAliasDefinedOutsideCurrentScope));
function typeReferences(node: Node | undefined): IR[] {
  if (!node) return [];
  const result: IR[] = [];
  const visit = (child: Node) => {
    if (ts.isTypeReferenceNode(child)) {
      let symbol = checker.getSymbolAtLocation(child.typeName);
      if (symbol && (symbol.flags & ts.SymbolFlags.Alias)) symbol = checker.getAliasedSymbol(symbol);
      result.push({ name: code(child.typeName), resolvedSymbol: symbol?.getName() ?? null, declarations: (symbol?.declarations ?? []).map((declaration) => {
        const file = declaration.getSourceFile().fileName.replaceAll('\\', '/');
        const src = file.startsWith(baselineRoot + '/') ? relative(baselineRoot, file).replaceAll('\\', '/') : file.includes('/node_modules/') ? '<dependency>/' + file.split('/node_modules/').at(-1) : '<typescript-library>/' + file.split('/').at(-1);
        return { source: src, line: declaration.getSourceFile().getLineAndCharacterOfPosition(declaration.getStart()).line + 1 };
      }) });
    }
    ts.forEachChild(child, visit);
  };
  visit(node);
  return result;
}
const privateMember = (node: Method) => node.modifiers?.some((modifier) => [ts.SyntaxKind.PrivateKeyword, ts.SyntaxKind.ProtectedKeyword].includes(modifier.kind)) ?? false;
const classes = new Map<string, { node: import('../.local/typescript/node_modules/typescript/lib/typescript.js').ClassDeclaration; source: SourceFile; methods: Map<string, Method> }>();
const functions = new Map<string, import('../.local/typescript/node_modules/typescript/lib/typescript.js').FunctionDeclaration>();
const constants = new Map<string, Expression>();
for (const source of sources) {
  for (const statement of source.statements) {
    if (ts.isClassDeclaration(statement) && statement.name) classes.set(statement.name.text, {
      node: statement, source,
      methods: new Map(statement.members.filter(ts.isMethodDeclaration).filter((method) => method.body).map((method) => [code(method.name), method])),
    });
    if (ts.isFunctionDeclaration(statement) && statement.name) functions.set(`${sourceName(source)}:${statement.name.text}`, statement);
    if (sourceName(source) === 'src/constants.ts' && ts.isVariableStatement(statement))
      for (const declaration of statement.declarationList.declarations)
        if (ts.isIdentifier(declaration.name) && declaration.initializer) constants.set(declaration.name.text, declaration.initializer);
  }
}
const clientSource = (path: string) => /^src\/(agentguard|ai-gateway|iam|management|model-security|red-team)\//.test(path) || path === 'src/scan/scanner.ts';
const memberCalls = (body: Node) => {
  const calls: import('../.local/typescript/node_modules/typescript/lib/typescript.js').CallExpression[] = [];
  const visit = (node: Node) => { if (ts.isCallExpression(node)) calls.push(node); ts.forEachChild(node, visit); };
  visit(body);
  return calls;
};
const unsupported = (node: Node, reason: string): IR => ({ kind: 'unsupported', syntax: ts.SyntaxKind[node.kind], source: code(node), reason });
const wrappers = (node: Expression): Expression => {
  while (ts.isParenthesizedExpression(node) || ts.isAsExpression(node) || ts.isTypeAssertionExpression(node) || ts.isNonNullExpression(node)) node = node.expression;
  return node;
};
/** Restricted data-transform IR. Nothing here executes source expressions. */
function expression(raw: Expression, resolving = new Set<string>()): IR {
  const node = wrappers(raw);
  if (ts.isStringLiteral(node) || ts.isNoSubstitutionTemplateLiteral(node)) return { kind: 'literal', value: node.text };
  if (ts.isNumericLiteral(node)) return { kind: 'literal', value: Number(node.text) };
  if (node.kind === ts.SyntaxKind.TrueKeyword || node.kind === ts.SyntaxKind.FalseKeyword) return { kind: 'literal', value: node.kind === ts.SyntaxKind.TrueKeyword };
  if (node.kind === ts.SyntaxKind.NullKeyword) return { kind: 'null' };
  if (node.kind === ts.SyntaxKind.ThisKeyword) return { kind: 'client' };
  if (ts.isIdentifier(node)) {
    if (node.text === 'undefined') return { kind: 'missing' };
    const constant = constants.get(node.text);
    if (constant && !resolving.has(node.text)) return { kind: 'constant', name: node.text, value: expression(constant, new Set(resolving).add(node.text)) };
    return { kind: 'reference', name: node.text };
  }
  if (ts.isPropertyAccessExpression(node)) return { kind: node.questionDotToken ? 'optional_field' : 'field', target: expression(node.expression, resolving), name: node.name.text };
  if (ts.isElementAccessExpression(node)) return { kind: node.questionDotToken ? 'optional_index' : 'index', target: expression(node.expression, resolving), index: expression(node.argumentExpression, resolving) };
  if (ts.isObjectLiteralExpression(node)) return { kind: 'object', entries: node.properties.map((property) => {
    if (ts.isSpreadAssignment(property)) return { kind: 'spread', value: expression(property.expression, resolving) };
    if (ts.isShorthandPropertyAssignment(property)) return { kind: 'entry', name: property.name.text, value: expression(property.name, resolving) };
    if (ts.isPropertyAssignment(property) && !ts.isComputedPropertyName(property.name)) return { kind: 'entry', name: ts.isStringLiteral(property.name) ? property.name.text : code(property.name), value: expression(property.initializer, resolving) };
    return unsupported(property, 'Computed properties, methods, and accessors require a reviewed native adapter.');
  }) };
  if (ts.isArrayLiteralExpression(node)) return { kind: 'array', items: node.elements.map((element) => ts.isSpreadElement(element) ? { kind: 'spread', value: expression(element.expression, resolving) } : expression(element as Expression, resolving)) };
  if (ts.isTemplateExpression(node)) return { kind: 'template', parts: [{ kind: 'literal', value: node.head.text }, ...node.templateSpans.flatMap((span) => [expression(span.expression, resolving), { kind: 'literal', value: span.literal.text }])] };
  if (ts.isConditionalExpression(node)) return { kind: 'conditional', condition: expression(node.condition, resolving), yes: expression(node.whenTrue, resolving), no: expression(node.whenFalse, resolving) };
  if (ts.isBinaryExpression(node)) {
    const allowed = new Set(['??', '||', '&&', '===', '!==', '==', '!=', '<', '<=', '>', '>=', '+', '-', '*', '/', '%']);
    if (!allowed.has(code(node.operatorToken))) return unsupported(node, 'Mutation or an unapproved binary operation cannot be treated as a data expression.');
    return { kind: 'binary', operator: code(node.operatorToken), left: expression(node.left, resolving), right: expression(node.right, resolving) };
  }
  if (ts.isPrefixUnaryExpression(node) && [ts.SyntaxKind.ExclamationToken, ts.SyntaxKind.MinusToken, ts.SyntaxKind.PlusToken].includes(node.operator)) return { kind: 'unary', operator: ts.tokenToString(node.operator), value: expression(node.operand, resolving) };
  if (ts.isTypeOfExpression(node)) return { kind: 'type_test_input', value: expression(node.expression, resolving) };
  if (ts.isCallExpression(node)) {
    const callee = code(node.expression);
    const args = node.arguments.map((argument) => expression(argument, resolving));
    if (['String', 'encodeURIComponent', 'gatewayPathSegment'].includes(callee)) return { kind: 'intrinsic', name: callee, arguments: args };
    if (ts.isPropertyAccessExpression(node.expression) && ['join', 'toISOString', 'trim', 'includes'].includes(node.expression.name.text)) return { kind: 'intrinsic_method', name: node.expression.name.text, target: expression(node.expression.expression, resolving), arguments: args };
    if (callee === 'Object.keys') return { kind: 'intrinsic', name: 'object_keys', arguments: args };
    return unsupported(node, 'Function calls need named native semantics; arbitrary JavaScript is not an executable generator input.');
  }
  return unsupported(node, 'This expression is outside the restricted native data-transform IR.');
}
function unsupportedNodes(value: unknown): IR[] {
  if (!value || typeof value !== 'object') return [];
  if (Array.isArray(value)) return value.flatMap(unsupportedNodes);
  const record = value as IR;
  return [...(record.kind === 'unsupported' ? [record] : []), ...Object.values(record).flatMap(unsupportedNodes)];
}
function assignmentRoots(value: unknown): string[] {
  if (!value || typeof value !== 'object') return [];
  if (Array.isArray(value)) return value.flatMap(assignmentRoots);
  const record = value as IR;
  const roots: string[] = [];
  if (record.kind === 'assign') {
    let target = record.target as IR;
    while (target.kind === 'field' || target.kind === 'index') target = target.target as IR;
    roots.push(target.kind === 'reference' ? String(target.name) : '<nonlocal>');
  }
  return [...roots, ...Object.values(record).flatMap(assignmentRoots)];
}
function statementIR(statement: import('../.local/typescript/node_modules/typescript/lib/typescript.js').Statement): IR {
  if (ts.isBlock(statement)) return { kind: 'block', statements: statement.statements.map(statementIR) };
  if (ts.isVariableStatement(statement)) return { kind: 'declare', immutable: (statement.declarationList.flags & ts.NodeFlags.Const) !== 0, bindings: statement.declarationList.declarations.map((declaration) => ({ name: code(declaration.name), type: declaration.type ? code(declaration.type) : typeText(declaration.name), value: declaration.initializer ? expression(declaration.initializer) : { kind: 'missing' } })) };
  if (ts.isIfStatement(statement)) return { kind: 'if', condition: expression(statement.expression), yes: statementIR(statement.thenStatement), ...(statement.elseStatement ? { no: statementIR(statement.elseStatement) } : {}) };
  if (ts.isExpressionStatement(statement) && ts.isBinaryExpression(statement.expression) && statement.expression.operatorToken.kind === ts.SyntaxKind.EqualsToken) return { kind: 'assign', target: expression(statement.expression.left), value: expression(statement.expression.right) };
  if (ts.isExpressionStatement(statement) && ts.isCallExpression(statement.expression) && /^assert[A-Z]/.test(code(statement.expression.expression))) return { kind: 'validation', name: code(statement.expression.expression), arguments: statement.expression.arguments.map((argument) => expression(argument)), requiresNativeValidator: true };
  if (ts.isReturnStatement(statement) && statement.expression) return { kind: 'return', value: expression(statement.expression) };
  if (ts.isThrowStatement(statement)) return unsupported(statement, 'Explicit error construction and control flow need reviewed validation semantics.');
  return unsupported(statement, 'This statement requires a reviewed native adapter.');
}
const requestProperties = (call: import('../.local/typescript/node_modules/typescript/lib/typescript.js').CallExpression) => {
  const first = call.arguments[0];
  if (!first || !ts.isObjectLiteralExpression(first)) return { fields: {} as Record<string, Expression>, spreads: [] as Expression[], unsupported: true };
  const fields: Record<string, Expression> = {};
  const spreads: Expression[] = [];
  for (const property of first.properties) {
    if (ts.isPropertyAssignment(property)) fields[code(property.name)] = property.initializer;
    else if (ts.isShorthandPropertyAssignment(property)) fields[property.name.text] = property.name;
    else if (ts.isSpreadAssignment(property)) spreads.push(property.expression);
    else return { fields, spreads, unsupported: true };
  }
  return { fields, spreads, unsupported: false };
};
function schemaMetadata(node: Expression | undefined): unknown {
  if (!node) return null;
  const names = [...new Set(code(node).match(/\b[A-Za-z_][A-Za-z0-9_]*Schema\b/g) ?? [])].sort();
  return { expression: code(node), names, conditional: ts.isConditionalExpression(wrappers(node)), allowsMissing: /\.optional\(\)/.test(code(node)) };
}
const gatewayAdmin = new Set(['AIGatewayAdminGuardrailsClient', 'AIGatewayIntegrationsClient', 'AIGatewayMcpIntegrationsClient', 'AIGatewayDeploymentsClient', 'AIGatewayPluginsClient', 'AIGatewayOrganisationsClient', 'AIGatewayAuditLogsClient', 'AIGatewaySecretReferencesClient']);
function endpoint(path: string, className: string, field?: Expression): IR {
  const sourceExpression = field ? code(field) : null;
  if (className === 'OAuthClient') return { plane: 'oauth_token', sourceExpression, auth: 'client_credentials', defaultConstant: 'DEFAULT_TOKEN_ENDPOINT' };
  if (className === 'AIGatewayModelPricingClient') return { plane: 'public_pricing', sourceExpression, auth: 'none', defaultConstant: null, requirement: 'explicit_public_endpoint_no_implicit_default' };
  if (path.startsWith('src/scan/')) return { plane: 'scan', sourceExpression, auth: 'airs_api_key_hmac_or_bearer', defaultConstant: 'DEFAULT_ENDPOINT' };
  if (path.startsWith('src/iam/')) return { plane: 'shared_iam', sourceExpression, auth: 'oauth_bearer_and_x_tsg_id', defaultConstant: 'DEFAULT_IAM_ENDPOINT' };
  if (path.startsWith('src/agentguard/')) return { plane: sourceExpression?.includes('mgmt') ? 'agentguard_management' : 'agentguard_data', sourceExpression, auth: 'oauth_bearer_and_x_tsg_id', defaultConstant: sourceExpression?.includes('mgmt') ? 'AGENT_GUARD_MGMT_ENDPOINT' : 'AGENT_GUARD_DATA_ENDPOINT', determination: sourceExpression ? 'request_expression' : 'per_method_private_getter_arguments' };
  if (path.startsWith('src/management/dlp/')) return { plane: 'shared_dlp', sourceExpression, auth: 'oauth_bearer', defaultConstant: 'DEFAULT_DLP_ENDPOINT' };
  if (className === 'DashboardClient') return { plane: 'airs_dashboard', sourceExpression, auth: 'oauth_bearer_with_optional_x_tsg_id', defaultConstant: 'DEFAULT_MGMT_ENDPOINT', callerOverride: 'dashboardEndpoint' };
  if (path.startsWith('src/management/')) return { plane: 'airs_management', sourceExpression, auth: 'oauth_bearer', defaultConstant: 'DEFAULT_MGMT_ENDPOINT' };
  if (path.startsWith('src/model-security/')) {
    const mgmt = /security-(groups|rules)|custom-rules/.test(path) || className === 'ModelSecurityClient';
    return { plane: mgmt ? 'model_security_management' : 'model_security_data', sourceExpression, auth: 'oauth_bearer', defaultConstant: mgmt ? 'DEFAULT_MODEL_SEC_MGMT_ENDPOINT' : 'DEFAULT_MODEL_SEC_DATA_ENDPOINT' };
  }
  if (path.startsWith('src/red-team/')) {
    const network = path.includes('network-broker');
    const mgmt = /targets-client|adapters-client|custom-attacks-client|instances-client|eula-client/.test(path) || sourceExpression?.includes('mgmt');
    return { plane: network ? 'red_team_network_broker' : mgmt ? 'red_team_management' : 'red_team_data', sourceExpression, auth: 'oauth_bearer', defaultConstant: network ? 'DEFAULT_RED_TEAM_NETWORK_BROKER_ENDPOINT' : mgmt ? 'DEFAULT_RED_TEAM_MGMT_ENDPOINT' : 'DEFAULT_RED_TEAM_DATA_ENDPOINT' };
  }
  if (/\/(inference|runtime-resources)-client\.ts$/.test(path)) return { plane: 'gateway_runtime', sourceExpression, auth: 'gateway_runtime_api_key', defaultConstant: null, requirement: 'explicit_runtime_endpoint_and_credential' };
  if (className === 'AIGatewayWorkspacesClient' && sourceExpression?.startsWith('this.urlFor')) return { plane: 'gateway_data_or_admin', sourceExpression, auth: 'oauth_bearer_and_x_tsg_id', selection: 'options.plane === admin ? admin : data', defaultConstant: 'DEFAULT_AI_GW_DATA_ENDPOINT', adminDefaultConstant: 'DEFAULT_AI_GW_ADMIN_ENDPOINT' };
  const admin = sourceExpression?.includes('adminBaseUrl') || gatewayAdmin.has(className);
  return { plane: admin ? 'gateway_admin' : 'gateway_data', sourceExpression, auth: 'oauth_bearer_and_x_tsg_id', defaultConstant: admin ? 'DEFAULT_AI_GW_ADMIN_ENDPOINT' : 'DEFAULT_AI_GW_DATA_ENDPOINT' };
}
function manualReasons(path: string, className: string, method: Method): string[] {
  const name = code(method.name);
  const reasons: string[] = [];
  if (className === 'DashboardClient') reasons.push('Dashboard uses private request builders, strict/defaulted query schemas, tenant headers, and typed/raw response variants; port explicitly.');
  if (className === 'AgentGuardClient') reasons.push('AgentGuard private getter hides route/query construction from existing inventory; port named query validation and both endpoints explicitly.');
  if (className === 'OAuthClient') reasons.push('Token lifecycle/cache methods require independent secure OAuth implementation, not a request-method wrapper.');
  if (name === 'provision') reasons.push('Multi-step workspace/IAM workflow must preserve partial outcomes; inventory helper expansion must never flatten it to POST /workspaces.');
  if (className === 'IamScopesClient' && name === 'bindWorkspace') reasons.push('Read/modify/write binding workflow preserves existing resources and requires explicit concurrent-update policy.');
  if (/^listAll/.test(name)) reasons.push('Bounded multi-page workflow needs dialect-specific continuation and explicit truncated outcomes.');
  if (['ProfilesClient', 'TopicsClient'].includes(className) && ['get', 'getByName', 'forceDelete'].includes(name)) reasons.push('Lookup or forced-delete convenience behavior requires an explicit native workflow.');
  if (className === 'Scanner') reasons.push('Scan body assembly, request limits, identifier validation, and global-to-per-client configuration corrections require a native adapter.');
  if (className === 'AIGatewayInferenceClient') reasons.push('Inference JSON/SSE/realtime operation and caller runtime policy require typed native protocol adapters.');
  if (className === 'AIGatewayModelPricingClient') reasons.push('Public unauthenticated pricing client must prevent management/runtime credential attachment.');
  if (className === 'IamScopesClient' && ['create', 'update'].includes(name)) reasons.push('IAM input normalization defaults description/resources/metadata and repeats the path name in update body.');
  if (className === 'AIGatewayTelemetryClient') reasons.push('Telemetry window/filter helpers contain cross-field checks and wire-name conversion; review native query adapters.');
  if (path.endsWith('/organisations-client.ts') && /AuthSettings/.test(name)) reasons.push('Organisation auth-settings route is produced by a helper unresolved by existing call-site inventory.');
  return reasons;
}
const fixtureLinks = new Map<string, string[]>();
const fixtureCounts: Record<string, number> = {};
for (const fixtureEntry of [...manifest.fixtures].sort((left, right) => compareText(left.path, right.path))) {
  const file = fixtureEntry.path;
  const path = join(repo, file);
  if (!file.endsWith('.json')) continue;
  if (digest(readFileSync(path)) !== fixtureEntry.sha256) throw new Error(`Frozen fixture mismatch: ${file}`);
  const value = readJson(path);
  if (!Array.isArray(value.cases)) continue;
  fixtureCounts[file] = value.cases.length;
  value.cases.forEach((fixture: { call?: { source: string; className: string; member: string } }, index: number) => {
    if (!fixture.call) return;
    const key = `${fixture.call.source}:${fixture.call.className}.${fixture.call.member}`;
    fixtureLinks.set(key, [...(fixtureLinks.get(key) ?? []), `${file}#/cases/${index}`]);
  });
}
const operations: IR[] = [];
const classRecords: IR[] = [];
for (const [className, entry] of [...classes.entries()].sort(([left], [right]) => compareText(left, right))) {
  const path = sourceName(entry.source);
  if (!clientSource(path)) continue;
  const concrete = [...entry.methods.entries()].filter(([, method]) => !privateMember(method));
  const extendsNames = entry.node.heritageClauses?.filter((clause) => clause.token === ts.SyntaxKind.ExtendsKeyword).flatMap((clause) => clause.types.map((type) => code(type.expression))) ?? [];
  classRecords.push({ name: className, source: path, extends: extendsNames, ownConcretePublicMethodCount: concrete.length, inheritedMethodsCountedAtDeclaringClass: true,
    constructorEvidence: entry.node.members.filter(ts.isConstructorDeclaration).map((constructor) => code(constructor)),
    nonMethodPublicSurface: entry.node.members.filter((member) => !ts.isMethodDeclaration(member) && !ts.isConstructorDeclaration(member) && !(ts.canHaveModifiers(member) ? ts.getModifiers(member) : undefined)?.some((modifier) => [ts.SyntaxKind.PrivateKeyword, ts.SyntaxKind.ProtectedKeyword].includes(modifier.kind))).map((member) => ({ syntax: ts.SyntaxKind[member.kind], source: code(member) })),
  });
  for (const [name, method] of concrete.sort(([left], [right]) => compareText(left, right))) {
    const id = `${path}:${className}.${name}`;
    const calls = memberCalls(method.body!);
    const requestCalls = calls.filter((call) => ['request', 'openRealtime'].includes(code(call.expression)));
    const helperCalls = calls.filter((call) => ts.isPropertyAccessExpression(call.expression) && call.expression.expression.kind === ts.SyntaxKind.ThisKeyword && entry.methods.has(call.expression.name.text));
    const direct = requestCalls.length === 1 ? requestProperties(requestCalls[0]) : null;
    const reasons = manualReasons(path, className, method);
    const parameters = method.parameters.map((parameter) => ({ name: code(parameter.name), declaration: code(parameter), declaredType: parameter.type ? code(parameter.type) : null, resolvedType: parameter.type ? code(parameter.type) : typeText(parameter.name), typeReferences: typeReferences(parameter.type), optional: !!parameter.questionToken || !!parameter.initializer, default: parameter.initializer ? expression(parameter.initializer) : null }));
    const overloads = entry.node.members.filter(ts.isMethodDeclaration).filter((candidate) => !candidate.body && code(candidate.name) === name).map((candidate) => code(candidate));
    const prefixStatements = method.body!.statements.filter((statement) => !(ts.isReturnStatement(statement) && statement.expression && ts.isCallExpression(statement.expression) && ['request', 'openRealtime'].includes(code(statement.expression.expression))));
    const prefixIR = prefixStatements.map(statementIR);
    const requestIR = direct ? Object.fromEntries(Object.entries(direct.fields).filter(([key]) => !['responseSchema', 'requestSchema', 'auth'].includes(key)).map(([key, value]) => [key, expression(value)])) : null;
    if (requestCalls.length !== 1) reasons.push(`Expected one direct request expression; found ${requestCalls.length}. The method is a workflow, helper delegation, or lifecycle operation.`);
    if (direct?.unsupported) reasons.push('Request argument has unsupported properties or is not an object literal.');
    if (direct?.spreads.length) reasons.push('Request option spreads need a reviewed native transport/query builder; spread semantics are not automatically flattened.');
    const localBindings = new Set(prefixStatements.filter(ts.isVariableStatement).flatMap((statement) => statement.declarationList.declarations.filter((declaration) => ts.isIdentifier(declaration.name)).map((declaration) => code(declaration.name))));
    if (assignmentRoots(prefixIR).some((name) => !localBindings.has(name))) reasons.push('Assignment targets state outside local request-builder bindings; review native mutation semantics.');
    const rejected = unsupportedNodes({ prefixIR, requestIR, parameters });
    if (rejected.length) reasons.push(...[...new Set(rejected.map((node) => `${node.syntax}: ${node.reason} Source: ${node.source}`))]);
    if (direct?.fields.formData || direct?.fields.encodeFormData) reasons.push('Multipart boundary ownership, exact part encoding, and file bytes require a typed native upload adapter.');
    if (direct?.fields.responseType || direct?.fields.streamResponse || requestCalls.some((call) => code(call.expression) === 'openRealtime')) reasons.push('Non-JSON, streaming, or request-format-dependent output requires a typed native response adapter.');
    if (direct?.fields.responseSchema && ts.isConditionalExpression(wrappers(direct.fields.responseSchema))) reasons.push('Conditional response validation requires an explicit typed native output adapter.');
    const signature = checker.getSignatureFromDeclaration(method);
    const returnType = method.type ? code(method.type) : signature ? normalizedType(checker.typeToString(checker.getReturnTypeOfSignature(signature), method, ts.TypeFormatFlags.NoTruncation | ts.TypeFormatFlags.UseAliasDefinedOutsideCurrentScope)) : null;
    const sourcePosition = entry.source.getLineAndCharacterOfPosition(method.getStart(entry.source));
    const helperEvidence = [...new Set(helperCalls.map((call) => (call.expression as import('../.local/typescript/node_modules/typescript/lib/typescript.js').PropertyAccessExpression).name.text))].sort().map((helperName) => {
      const helper = entry.methods.get(helperName)!;
      return { name: helperName, visibility: privateMember(helper) ? 'private_or_protected' : 'public', source: code(helper), expansionPermitted: false };
    });
    const functionEvidence = new Map<string, IR>();
    for (const call of calls.filter((call) => ts.isIdentifier(call.expression))) {
      if (['request', 'openRealtime'].includes(code(call.expression))) continue;
      let symbol = checker.getSymbolAtLocation(call.expression);
      if (symbol && (symbol.flags & ts.SymbolFlags.Alias)) symbol = checker.getAliasedSymbol(symbol);
      const declaration = symbol?.declarations?.find(ts.isFunctionDeclaration) ?? functions.get(`${path}:${code(call.expression)}`);
      if (!declaration?.body || !bySource.has(sourceName(declaration.getSourceFile()))) continue;
      const helperSource = sourceName(declaration.getSourceFile());
      functionEvidence.set(`${helperSource}:${code(call.expression)}`, { name: code(call.expression), sourcePath: helperSource, sourceFileSha256: bySource.get(helperSource)!.sha256, source: code(declaration), expansionPermitted: false });
    }
    const endpointMetadata = endpoint(path, className, direct?.fields.baseUrl);
    if (className === 'AIGatewayWorkspacesClient' && name === 'provision') Object.assign(endpointMetadata, {
      plane: 'multi_endpoint_workflow', planes: ['shared_iam', 'gateway_admin'], sourceExpression: null,
      defaultConstant: null, endpointDefaults: ['DEFAULT_IAM_ENDPOINT', 'DEFAULT_AI_GW_ADMIN_ENDPOINT'],
      determination: 'Explicit IAM create, workspace create, binding, and compensation evidence; never infer one route from call-site expansion.',
    });
    if (className === 'AgentGuardClient') {
      const getter = helperCalls.find((call) => code(call.expression) === 'this.get');
      if (getter?.arguments[0]) Object.assign(endpointMetadata, endpoint(path, className, getter.arguments[0]));
    }
    operations.push({
      id, className, method: name, family: path.split('/')[1], source: { path, line: sourcePosition.line + 1, sha256: bySource.get(path)!.sha256, methodSha256: digest(code(method)) },
      implementationSource: code(method), parameters, declaredReturnType: method.type ? code(method.type) : null, resolvedReturnType: returnType, returnTypeReferences: typeReferences(method.type), returnTypeResolution: method.type ? 'declared_nominal_contract_with_compiler_symbol_links' : 'compiler_inferred_structure', overloads,
      methodKind: method.asteriskToken ? 'generator' : method.modifiers?.some((modifier) => modifier.kind === ts.SyntaxKind.AsyncKeyword) ? 'async' : 'sync',
      classification: reasons.length ? 'handwritten_adapter_required' : 'native_declarative', blockers: [...new Set(reasons)],
      implementationStatus: 'not_implemented', contractVerificationStatus: 'baseline_evidence_only', liveAcceptanceStatus: 'not_run',
      endpoint: endpointMetadata,
      directRequest: direct ? { executor: code(requestCalls[0].expression), fields: requestIR, rawFields: Object.fromEntries(Object.entries(direct.fields).map(([key, value]) => [key, code(value)])), spreads: direct.spreads.map((value) => ({ source: code(value), ir: expression(value) })), requestSchema: schemaMetadata(direct.fields.requestSchema), responseSchema: schemaMetadata(direct.fields.responseSchema), bodyType: direct.fields.body ? parameters.find((parameter) => parameter.name === code(direct.fields.body!))?.declaredType ?? typeText(direct.fields.body) : null, authExpression: direct.fields.auth ? code(direct.fields.auth) : null } : null,
      inputTransform: { kind: 'statement_sequence', statements: prefixIR, executable: false },
      helpers: helperEvidence,
      namedFunctionEvidence: [...functionEvidence.values()],
      callSiteEvidence: existingCalls.filter((call) => call.source === path && call.className === className && call.member === name),
      callSiteEvidenceIsCompleteWorkflow: false,
      fixtureReferences: fixtureLinks.get(id) ?? [],
      generationPolicy: reasons.length ? 'Reject generation until a reviewed typed native adapter is supplied.' : 'Emit a concrete typed wrapper only after native endpoint, validator, type, and request-executor semantics are implemented; never interpret this metadata as JavaScript.',
    });
  }
}
operations.sort((left, right) => compareText(String(left.id), String(right.id)));
const counts = operations.reduce((result: Record<string, number>, operation) => { const family = String(operation.family); result[family] = (result[family] ?? 0) + 1; return result; }, {});
const expected = { agentguard: 4, 'ai-gateway': 196, iam: 6, management: 87, 'model-security': 49, 'red-team': 102, scan: 4 };
if (operations.length !== 448 || Object.entries(expected).some(([family, count]) => counts[family] !== count)) throw new Error(`Client inventory count changed: ${JSON.stringify(counts)}. Review the baseline rather than adjusting the gate silently.`);
if (new Set(operations.map((operation) => operation.id)).size !== operations.length) throw new Error('Duplicate operation IDs.');
const nativeOperations = operations.filter((operation) => operation.classification === 'native_declarative');
if (nativeOperations.some((operation) => unsupportedNodes(operation.directRequest).length || unsupportedNodes(operation.inputTransform).length || (operation.blockers as string[]).length)) throw new Error('An unsupported expression escaped the declarative generation gate.');
const runtime = classRecords.find((entry) => entry.name === 'AIGatewayRuntimeResourcesClient');
const inference = classRecords.find((entry) => entry.name === 'AIGatewayInferenceClient');
if (runtime?.ownConcretePublicMethodCount !== 48 || !(inference?.extends as string[]).includes('AIGatewayRuntimeResourcesClient')) throw new Error('Runtime inheritance parity changed.');
for (const operation of operations) {
  const route = operation.endpoint as IR;
  const constantNames = [route.defaultConstant, route.adminDefaultConstant, ...(route.endpointDefaults as string[] ?? [])].filter((name): name is string => typeof name === 'string');
  if (constantNames.some((name) => !constants.has(name))) throw new Error(`Unknown endpoint constant for ${operation.id}.`);
}
// Negative extraction checks: accepted metadata cannot smuggle source evaluation into
// native generation. These checks run on every extraction and never send requests.
for (const input of ['runArbitraryCode(value)', 'Object.fromEntries(values)', '(value) => value', 'await request(value)']) {
  const parsed = ts.createSourceFile('rejected-expression.ts', input, ts.ScriptTarget.Latest, true);
  const first = parsed.statements[0];
  if (!ts.isExpressionStatement(first) || unsupportedNodes(expression(first.expression)).length === 0) throw new Error(`Unsafe expression unexpectedly accepted: ${input}`);
}
const payload = {
  schemaVersion: 1, purpose: 'Reviewed declarative operation inputs and explicit native-adapter backlog; this artifact is not implemented SDK functionality.',
  baseline: { repository: manifest.baseline.repository, version: manifest.baseline.version, commit: manifest.baseline.commit, sourceTreeLedgerSha256: manifest.baseline.gitTreeLedgerSha256, inventoryToolSha256: inventoryReference.sha256, extractionInputs },
  generator: { path: 'tools/extract-operations.mts', sha256: digest(readFileSync(fileURLToPath(import.meta.url))), typescriptVersion: ts.version, tsxVersion: require('tsx/package.json').version },
  extractionPolicy: { sourceHashVerification: 'all frozen src files and ledger-bound compiler/package/auditor inputs', evaluationIsolation: 'process.env cleared and fetch/WebSocket disabled before importing verified upstream tooling', publicClientMethodScope: 'Concrete public bodies in client families; inherited runtime methods counted only at declaring class. Constructors, accessors, and public properties retained as class evidence; helper/functions/constants tracked separately by export inventory.', expressionEvaluation: 'none', unsupportedExpressions: 'explicit blockers, never generic JSON stubs', helperExpansion: 'never flatten workflows; retain exact helper source', nativeDeclarativeMeaning: 'Syntax accepted for future typed native generation; Rust type/validator/executor implementation and review are still required.', securityPolicy: 'Baseline retries, global state, logging, and error behavior are evidence only and require the approved Rust corrections.' },
  summary: { concretePublicClientMethods: operations.length, overloadDeclarations: operations.reduce((sum, operation) => sum + (operation.overloads as unknown[]).length, 0), methodsByFamily: counts, nativeDeclarative: nativeOperations.length, handwrittenAdapterRequired: operations.length - nativeOperations.length, existingInventoryCallSites: existingCalls.length, committedFixtureCases: Object.values(fixtureCounts).reduce((sum, count) => sum + count, 0), openApiTransportFixtureCases: Object.entries(fixtureCounts).filter(([path]) => path.includes('/openapi/fixtures/')).reduce((sum, [, count]) => sum + count, 0), fixtureCasesByFile: fixtureCounts },
  classes: classRecords, operations,
};
const serialized = `${JSON.stringify(payload, null, 2)}\n`;
if (process.argv.includes('--check')) {
  if (!existsSync(output) || readFileSync(output, 'utf8') !== serialized) throw new Error('contracts/operations.json is stale; run tools/extract-operations.mts.');
  process.stdout.write(`Operation metadata verified: ${operations.length} methods, ${nativeOperations.length} declarative, ${operations.length - nativeOperations.length} explicit native adapters.\n`);
} else {
  writeFileSync(output, serialized);
  process.stdout.write(`Wrote contracts/operations.json: ${operations.length} methods, ${nativeOperations.length} declarative, ${operations.length - nativeOperations.length} explicit native adapters.\n`);
}
