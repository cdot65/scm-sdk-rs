#!/usr/bin/env node
/**
 * Frozen baseline inventory. Run with the baseline's pinned tsx installation:
 * .local/typescript/node_modules/.bin/tsx tools/extract-contracts.mts
 *
 * This tool reads source and evaluates schema declarations only. It never invokes
 * clients, validators, refinements, default providers, or application helpers.
 * A temporary instrumented source copy exposes private module-level schemas.
 * Output contains source contracts, never environment or tenant registrations.
 */
import { createHash } from 'node:crypto';
import { readFile, writeFile, mkdir, readdir, rm } from 'node:fs/promises';
import path from 'node:path';
import { pathToFileURL, fileURLToPath } from 'node:url';
import ts from '../.local/typescript/node_modules/typescript/lib/typescript.js';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const baseline = path.join(root, '.local/typescript');
const sourceRoot = path.join(baseline, 'src');
const contractRoot = path.join(root, 'contracts');
const classificationPath = path.join(contractRoot, 'export-classification.json');
const baselineCommit = '4a80dbdb3dc14365e1c25d838ac0bd139485d48c';
const initialize = process.argv.includes('--initialize-classification');
const check = process.argv.includes('--check');
if (process.argv.slice(2).some((argument) => !['--initialize-classification', '--check'].includes(argument)) || initialize && check) {
  throw new Error('Use either --check, --initialize-classification, or no arguments');
}
const hash = (input: string | Uint8Array) => createHash('sha256').update(input).digest('hex');
const relative = (input: string) => path.relative(baseline, input).split(path.sep).join('/');
const sort = <T,>(values: T[], key: (value: T) => string) => values.sort((a, b) => key(a).localeCompare(key(b), 'en'));
const json = (value: unknown) => `${JSON.stringify(value, null, 2)}\n`;
const normalize = (text: string) => text.split(baseline).join('<baseline>');
function validateClassificationHeader(classifications: { schemaVersion?: unknown; baselineCommit?: unknown }) {
  if (classifications.schemaVersion !== 1 || classifications.baselineCommit !== baselineCommit) {
    throw new Error('Classification schema or frozen baseline commit mismatch');
  }
}
if (!initialize) validateClassificationHeader(JSON.parse(await readFile(classificationPath, 'utf8')));

async function filesIn(directory: string): Promise<string[]> {
  const files: string[] = [];
  for (const entry of sort(await readdir(directory, { withFileTypes: true }), (entry) => entry.name)) {
    const file = path.join(directory, entry.name);
    if (entry.isDirectory()) files.push(...await filesIn(file));
    else files.push(file);
  }
  return files;
}

const baselineFiles = (await filesIn(sourceRoot)).filter((file) => file.endsWith('.ts')).sort();
const hashes = await Promise.all(baselineFiles.map(async (file) => ({ file: relative(file), sha256: hash(await readFile(file)) })));
const provenance = JSON.parse(await readFile(path.join(contractRoot, 'manifest.json'), 'utf8'));
if (provenance.baseline.commit !== baselineCommit || json(hashes) !== json(provenance.baseline.srcFiles.map((file: any) => ({ file: file.upstreamPath, sha256: file.sha256 })))) {
  throw new Error('Source tree differs from independently verified frozen provenance; refusing to evaluate baseline code');
}
const ledgerBytes = await readFile(path.join(root, provenance.baseline.gitTreeLedger));
if (hash(ledgerBytes) !== provenance.baseline.gitTreeLedgerSha256) throw new Error('Frozen Git tree ledger hash mismatch');
const ledger = JSON.parse(ledgerBytes.toString('utf8'));
if (ledger.commit !== baselineCommit) throw new Error('Frozen Git tree ledger commit mismatch');
const extractionInputs: { upstreamPath: string; sha256: string; gitBlobSha1: string; bytes: number }[] = provenance.baseline.extractionInputs;
if (!Array.isArray(extractionInputs) || !['package.json', 'tsconfig.json'].every((file) => extractionInputs.some((input) => input.upstreamPath === file))) {
  throw new Error('Frozen compiler/package inputs are absent from provenance');
}
for (const input of extractionInputs) {
  const entry = ledger.files.find((file: { path: string }) => file.path === input.upstreamPath);
  const bytes = await readFile(path.join(baseline, input.upstreamPath));
  const blob = createHash('sha1').update(`blob ${bytes.length}\0`).update(bytes).digest('hex');
  if (!entry || input.gitBlobSha1 !== entry.gitBlobSha1 || input.bytes !== entry.bytes || bytes.length !== entry.bytes ||
    blob !== entry.gitBlobSha1 || hash(bytes) !== input.sha256) {
    throw new Error(`Frozen extraction input mismatch: ${input.upstreamPath}`);
  }
}
const parsed = ts.readConfigFile(path.join(baseline, 'tsconfig.json'), ts.sys.readFile);
if (parsed.error) throw new Error(ts.flattenDiagnosticMessageText(parsed.error.messageText, '\n'));
const config = ts.parseJsonConfigFileContent(parsed.config, ts.sys, baseline);
config.options.typeRoots = [path.join(baseline, 'node_modules/@types')];
config.options.types = ['node'];
const program = ts.createProgram(baselineFiles, config.options);
const diagnostics = ts.getPreEmitDiagnostics(program);
if (diagnostics.length) {
  throw new Error(ts.formatDiagnosticsWithColorAndContext(diagnostics, {
    getCurrentDirectory: () => baseline,
    getCanonicalFileName: (file) => file,
    getNewLine: () => '\n',
  }));
}
const checker = program.getTypeChecker();
const sourceFiles = baselineFiles.map((file) => program.getSourceFile(file)!);
const rootSource = program.getSourceFile(path.join(sourceRoot, 'index.ts'))!;
const rootSymbol = checker.getSymbolAtLocation(rootSource)!;
const publicSymbols = sort(checker.getExportsOfModule(rootSymbol), (symbol) => symbol.name);

function resolveAlias(symbol: ts.Symbol): ts.Symbol {
  return symbol.flags & ts.SymbolFlags.Alias ? checker.getAliasedSymbol(symbol) : symbol;
}

function valueExport(symbol: ts.Symbol): boolean {
  if (symbol.declarations?.every((node) => ts.isExportSpecifier(node) &&
    (node.isTypeOnly || ts.isExportDeclaration(node.parent.parent) && node.parent.parent.isTypeOnly))) return false;
  return Boolean(resolveAlias(symbol).flags & ts.SymbolFlags.Value);
}

function sourceLocation(node: ts.Node) {
  const source = node.getSourceFile();
  const start = node.getStart(source);
  const location = source.getLineAndCharacterOfPosition(start);
  const text = node.getText(source);
  return {
    file: source.fileName.startsWith(`${sourceRoot}${path.sep}`) ? relative(source.fileName)
      : source.fileName.includes('/node_modules/') ? source.fileName.slice(source.fileName.lastIndexOf('/node_modules/') + 1)
        : path.basename(source.fileName), line: location.line + 1, column: location.character + 1,
    start, end: node.end, sha256: hash(text), text,
  };
}

function docs(symbol: ts.Symbol) {
  return {
    description: ts.displayPartsToString(symbol.getDocumentationComment(checker)),
    tags: symbol.getJsDocTags(checker).map((tag) => ({ name: tag.name, text: ts.displayPartsToString(tag.text) })),
  };
}

function publicMember(symbol: ts.Symbol): boolean {
  return !symbol.getDeclarations()?.some((declaration) => {
    const flags = ts.getCombinedModifierFlags(declaration);
    return flags & (ts.ModifierFlags.Private | ts.ModifierFlags.Protected) ||
      ('name' in declaration && declaration.name && ts.isPrivateIdentifier(declaration.name as ts.Node));
  });
}

function typeText(type: ts.Type) {
  return normalize(checker.typeToString(type, undefined,
    ts.TypeFormatFlags.NoTruncation | ts.TypeFormatFlags.UseAliasDefinedOutsideCurrentScope));
}

const typeIds = new Map<ts.Type, string>();
const typeDefinitions: Record<string, Record<string, unknown>> = {};

function typeRef(type: ts.Type): { $ref: string } {
  const previous = typeIds.get(type);
  if (previous) return { $ref: previous };
  const id = `t${String(typeIds.size + 1).padStart(6, '0')}`;
  typeIds.set(type, id);
  const result: Record<string, unknown> = { display: typeText(type) };
  typeDefinitions[id] = result;
  const flags = type.flags;
  if (flags & ts.TypeFlags.StringLiteral) Object.assign(result, { kind: 'literal', value: (type as ts.StringLiteralType).value });
  else if (flags & ts.TypeFlags.NumberLiteral) Object.assign(result, { kind: 'literal', value: (type as ts.NumberLiteralType).value });
  else if (flags & ts.TypeFlags.BooleanLiteral) Object.assign(result, { kind: 'literal', value: result.display === 'true' });
  else if (flags & ts.TypeFlags.Union) Object.assign(result, { kind: 'union', variants: (type as ts.UnionType).types.map(typeRef) });
  else if (flags & ts.TypeFlags.Intersection) Object.assign(result, { kind: 'intersection', variants: (type as ts.IntersectionType).types.map(typeRef) });
  else if (flags & ts.TypeFlags.IndexedAccess) Object.assign(result, { kind: 'indexed_access',
    object: typeRef((type as ts.IndexedAccessType).objectType), index: typeRef((type as ts.IndexedAccessType).indexType) });
  else if (flags & ts.TypeFlags.TypeParameter) {
    const constraint = checker.getBaseConstraintOfType(type);
    Object.assign(result, { kind: 'type_parameter', ...(constraint ? { constraint: typeRef(constraint) } : {}) });
  } else if (flags & ts.TypeFlags.Object) {
    const arguments_ = (type as ts.ObjectType).objectFlags & ts.ObjectFlags.Reference
      ? checker.getTypeArguments(type as ts.TypeReference) : [];
    const symbol = type.getSymbol();
    const name = symbol?.name;
    if (checker.isTupleType(type)) Object.assign(result, { kind: 'tuple', elements: arguments_.map(typeRef) });
    else if (checker.isArrayType(type) || name === 'ReadonlyArray') Object.assign(result, { kind: 'array', element: typeRef(arguments_[0]) });
    else if (name && ['Promise', 'AsyncIterable', 'AsyncIterableIterator', 'AsyncGenerator', 'Iterable', 'Iterator', 'Map', 'Set', 'WeakMap', 'WeakSet'].includes(name)) {
      Object.assign(result, { kind: 'generic', name, arguments: arguments_.map(typeRef) });
    } else if (name && !['__type', '__object', '__function'].includes(name) && symbol?.declarations?.length && symbol.declarations.every((node) => !node.getSourceFile().fileName.startsWith(`${sourceRoot}${path.sep}`))) {
      Object.assign(result, { kind: 'external', name, declarations: symbol.declarations.map((node) => {
        const file = node.getSourceFile().fileName;
        return file.includes('/node_modules/') ? file.slice(file.lastIndexOf('/node_modules/') + 1) : path.basename(file);
      }) });
    } else {
      Object.assign(result, {
        kind: 'object',
        properties: sort(checker.getPropertiesOfType(type).filter(publicMember), (property) => property.name).map((property) => {
          const declaration = property.valueDeclaration ?? property.declarations?.[0];
          return { name: property.name, optional: Boolean(property.flags & ts.SymbolFlags.Optional),
            type: typeRef(checker.getTypeOfSymbolAtLocation(property, declaration ?? rootSource)),
            documentation: docs(property) };
        }),
        indexSignatures: checker.getIndexInfosOfType(type).map((index) => ({
          key: typeRef(index.keyType), value: typeRef(index.type), readonly: index.isReadonly,
        })),
        calls: checker.getSignaturesOfType(type, ts.SignatureKind.Call).map(signatureInfo),
      });
    }
  } else {
    const primitives: [number, string][] = [
      [ts.TypeFlags.Any, 'any'], [ts.TypeFlags.Unknown, 'unknown'], [ts.TypeFlags.Never, 'never'],
      [ts.TypeFlags.String, 'string'], [ts.TypeFlags.Number, 'number'], [ts.TypeFlags.Boolean, 'boolean'],
      [ts.TypeFlags.BigInt, 'bigint'], [ts.TypeFlags.Null, 'null'], [ts.TypeFlags.Undefined, 'undefined'],
      [ts.TypeFlags.Void, 'void'], [ts.TypeFlags.ESSymbol, 'symbol'], [ts.TypeFlags.NonPrimitive, 'object'],
    ];
    result.kind = primitives.find(([flag]) => flags & flag)?.[1] ?? 'typescript_specific';
    if (result.kind === 'typescript_specific') result.flags = flags;
  }
  return { $ref: id };
}

function signatureInfo(signature: ts.Signature): Record<string, unknown> {
  const declaration = signature.getDeclaration();
  return {
    signature: normalize(checker.signatureToString(signature, declaration, ts.TypeFormatFlags.NoTruncation)),
    parameters: signature.getParameters().map((parameter) => {
      const node = parameter.valueDeclaration ?? parameter.declarations?.[0] ?? declaration!;
      const param = ts.isParameter(node) ? node : undefined;
      return {
        name: parameter.name,
        optional: Boolean(parameter.flags & ts.SymbolFlags.Optional) || Boolean(param?.questionToken || param?.initializer),
        rest: Boolean(param?.dotDotDotToken),
        ...(param?.initializer ? { defaultExpression: param.initializer.getText() } : {}),
        type: typeRef(checker.getTypeOfSymbolAtLocation(parameter, node)),
      };
    }),
    returns: typeRef(signature.getReturnType()),
    typeParameters: signature.typeParameters?.map(typeRef) ?? [],
  };
}

function astBehavior(node: ts.Node) {
  const calls: { expression: string; arguments: string[]; line: number }[] = [];
  const strings = new Set<string>();
  const throws: string[] = [];
  function visit(child: ts.Node) {
    if (ts.isCallExpression(child) || ts.isNewExpression(child)) calls.push({
      expression: child.expression.getText(), arguments: child.arguments?.map((argument) => argument.getText()) ?? [],
      line: child.getSourceFile().getLineAndCharacterOfPosition(child.getStart()).line + 1,
    });
    if (ts.isStringLiteralLike(child)) strings.add(child.text);
    if (ts.isThrowStatement(child)) throws.push(child.getText());
    ts.forEachChild(child, visit);
  }
  visit(node);
  return { calls, stringLiterals: [...strings].sort(), throws };
}

function executionEvidence(node: ts.Node) {
  const evidence = astBehavior(node);
  const source = node.getText();
  const has = (pattern: RegExp) => evidence.calls.some((call) => pattern.test(call.expression));
  const protocols = [
    ...(has(/stream|Stream/) || /GatewayStream|text\/event-stream/.test(source) ? ['sse'] : []),
    ...(has(/realtime|Realtime|WebSocket/) || /GatewayRealtimeConnection/.test(source) ? ['websocket'] : []),
    ...(has(/multipart|Multipart|FormData/) || /Blob|FormData/.test(source) ? ['multipart_or_binary'] : []),
  ];
  const transportCalls = evidence.calls.filter((call) => /(?:^request(?:Stream|Binary|Raw|Text)?$|\.request$|^fetch$|^executeWithRetry$)/.test(call.expression));
  return { protocols, transportCalls, listOrIterator: /^(?:list|iterate)|(?:iterate|listAll|collect)/.test(('name' in node && node.name ? (node.name as ts.Node).getText() : '')),
    paginationCalls: evidence.calls.filter((call) => /^(?:collectAll|collectSkipPages|collectSpringPages|paginate)$/.test(call.expression)),
    hasLoop: /\b(?:for|while)\s*\(/.test(source), hasAwait: /\bawait\b/.test(source) };
}

function methodInfo(property: ts.Symbol, owner: string, static_: boolean) {
  const declarations = property.declarations ?? [];
  const declaration = property.valueDeclaration ?? declarations[0];
  if (!declaration) return undefined;
  const type = checker.getTypeOfSymbolAtLocation(property, declaration);
  const signatures = checker.getSignaturesOfType(type, ts.SignatureKind.Call);
  const accessor = declarations.some((node) => ts.isGetAccessor(node) || ts.isSetAccessor(node));
  const kind = accessor ? 'accessor' : signatures.length ? 'method' : 'property';
  return {
    id: `${owner}${static_ ? '::' : '.'}${property.name}`,
    name: property.name, kind, static: static_,
    documentation: docs(property),
    sources: declarations.map(sourceLocation),
    ...(signatures.length ? { signatures: signatures.map(signatureInfo) } : { type: typeRef(type) }),
    implementation: declarations.map((node) => ({ source: sourceLocation(node), ast: astBehavior(node) })),
    executionEvidence: declarations.map(executionEvidence),
  };
}

function namespaceFor(file: string, name: string): string {
  if (['src/models/error-response.ts', 'src/models/metadata.ts', 'src/models/oauth-token.ts'].includes(file)) return 'shared';
  if (file === 'src/models/enums.ts' && ['Action', 'ActionType', 'Category', 'CategoryType', 'Verdict', 'VerdictType', 'ErrorStatus', 'ErrorStatusType'].includes(name)) return 'shared';
  if (file === 'src/constants.ts') {
    if (/^(?:DEFAULT_)?AI_GW_|^aiGw/.test(name)) return 'airs::gateway';
    if (/^AGENT_GUARD_/.test(name)) return 'airs::agentguard';
    if (/^(?:DEFAULT_)?DLP_/.test(name)) return 'dlp';
    if (/^(?:DEFAULT_)?IAM_/.test(name)) return 'shared::iam';
    if (/^(?:DEFAULT_)?MODEL_SEC_/.test(name)) return 'airs::model_security';
    if (/^(?:DEFAULT_)?RED_TEAM_/.test(name)) return 'airs::red_team';
    if (/^(?:DEFAULT_)?MGMT_/.test(name)) return 'airs::management';
    if (/^(?:AIRS_|AI_SEC_|ASYNC_SCAN_|SYNC_SCAN_|SCAN_|MAX_CONTENT_|MAX_AI_PROFILE_|MAX_API_KEY_|MAX_NUMBER_OF_BATCH_SCAN_|MAX_NUMBER_OF_REPORT_|MAX_NUMBER_OF_SCAN_|MAX_REPORT_ID_|MAX_SCAN_ID_|MAX_SESSION_ID_|MAX_TOKEN_|MAX_TRANSACTION_ID_|HEADER_API_KEY|HEADER_AUTH_TOKEN|PAYLOAD_HASH|DEFAULT_ENDPOINT)/.test(name)) return 'airs::scan';
    return 'shared';
  }
  if (file === 'src/http/event-stream.ts' || file === 'src/http/realtime.ts') return 'airs::gateway';
  if (file.includes('/dlp') || /^src\/models\/dlp-/.test(file)) return 'dlp';
  if (file.startsWith('src/iam/') || file === 'src/models/iam.ts' || file.includes('/iam-')) return 'shared::iam';
  if (file.startsWith('src/agentguard/') || file.includes('/agentguard')) return 'airs::agentguard';
  if (file.startsWith('src/ai-gateway/') || /\/ai-gateway/.test(file)) return 'airs::gateway';
  if (file.startsWith('src/model-security/') || /\/model-security/.test(file)) return 'airs::model_security';
  if (file.startsWith('src/red-team/') || /\/red-team/.test(file)) return 'airs::red_team';
  if (file.startsWith('src/management/') || /\/mgmt-/.test(file)) return 'airs::management';
  if (file.startsWith('src/scan/') || file.startsWith('src/models/')) return 'airs::scan';
  return 'shared';
}

// The standalone AST traversal is deliberately independent of checker export enumeration.
function astExports(source: ts.SourceFile, seen = new Set<string>()): Set<string> {
  if (seen.has(source.fileName)) return new Set();
  const branch = new Set(seen).add(source.fileName);
  const names = new Set<string>();
  for (const statement of source.statements) {
    if (ts.isExportDeclaration(statement)) {
      if (statement.exportClause && ts.isNamedExports(statement.exportClause)) {
        for (const element of statement.exportClause.elements) names.add(element.name.text);
      } else if (!statement.exportClause && statement.moduleSpecifier && ts.isStringLiteral(statement.moduleSpecifier)) {
        const resolved = ts.resolveModuleName(statement.moduleSpecifier.text, source.fileName, config.options, ts.sys).resolvedModule;
        if (!resolved) throw new Error(`Cannot resolve export ${statement.moduleSpecifier.text}`);
        for (const name of astExports(program.getSourceFile(resolved.resolvedFileName)!, branch)) if (name !== 'default') names.add(name);
      }
    } else if (ts.canHaveModifiers(statement) && ts.getModifiers(statement)?.some((modifier) => modifier.kind === ts.SyntaxKind.ExportKeyword)) {
      if (ts.isVariableStatement(statement)) {
        for (const declaration of statement.declarationList.declarations) {
          if (!ts.isIdentifier(declaration.name)) throw new Error('Destructured exports need explicit AST inventory support');
          names.add(declaration.name.text);
        }
      } else if ('name' in statement && statement.name && ts.isIdentifier(statement.name as ts.Node)) {
        names.add((statement.name as ts.Identifier).text);
      }
    }
  }
  return names;
}

const astNames = [...astExports(rootSource)].sort();
const checkerNames = publicSymbols.map((symbol) => symbol.name).sort();
if (json(astNames) !== json(checkerNames)) {
  throw new Error(`AST/checker disagreement: ${json({ onlyAst: astNames.filter((name) => !checkerNames.includes(name)), onlyChecker: checkerNames.filter((name) => !astNames.includes(name)) })}`);
}

// Instrument module-level schemas in a disposable copy, leaving frozen inputs untouched.
const runtimeRoot = path.join(root, '.local/contract-runtime');
await rm(runtimeRoot, { recursive: true, force: true });
await mkdir(runtimeRoot, { recursive: true });
await writeFile(path.join(runtimeRoot, 'package.json'), '{"type":"module"}\n');
const schemaDeclarations: { file: string; name: string; identifier: string; source: ReturnType<typeof sourceLocation> }[] = [];
const scopedSchemaDeclarations: ReturnType<typeof sourceLocation>[] = [];
const semanticExpressions: Record<string, unknown>[] = [];
const sourceMethods: Record<string, any>[] = [];
const sourceMethodDeclarations: Record<string, any>[] = [];
const supportingFunctions: ReturnType<typeof sourceLocation>[] = [];
for (const source of sourceFiles) {
  const localSchemas: typeof schemaDeclarations = [];
  const semanticInsertions: { position: number; text: string }[] = [];
  function visit(node: ts.Node) {
    if (ts.isFunctionDeclaration(node) && node.body || ts.isVariableDeclaration(node) && node.initializer &&
      (ts.isArrowFunction(node.initializer) || ts.isFunctionExpression(node.initializer))) supportingFunctions.push(sourceLocation(node));
    if (ts.isMethodDeclaration(node) && !(ts.getCombinedModifierFlags(node) & (ts.ModifierFlags.Private | ts.ModifierFlags.Protected))) {
      const parent = node.parent;
      const entry = { owner: ts.isClassDeclaration(parent) ? parent.name?.text ?? '<anonymous>' : '<object>',
        name: node.name.getText(), source: sourceLocation(node) };
      sourceMethodDeclarations.push(entry);
      if (node.body) sourceMethods.push(entry);
    }
    if (ts.isCallExpression(node) && ts.isPropertyAccessExpression(node.expression) &&
      ['refine', 'superRefine', 'transform', 'preprocess', 'default', 'catch', 'custom', 'instanceof', 'pipe'].includes(node.expression.name.text)) {
      semanticExpressions.push({ kind: node.expression.name.text, source: sourceLocation(node),
        referencedDeclarations: node.arguments.filter(ts.isIdentifier).flatMap((argument) => {
          const symbol = checker.getSymbolAtLocation(argument);
          return symbol ? (resolveAlias(symbol).declarations ?? []).map(sourceLocation) : [];
        }) });
      semanticInsertions.push({ position: node.getStart(source), text: '__scmContractTagSchema(' });
      semanticInsertions.push({ position: node.end, text: `, ${JSON.stringify({ kind: node.expression.name.text, source: sourceLocation(node) })})` });
    }
    if (ts.isVariableDeclaration(node) && ts.isIdentifier(node.name) && node.initializer) {
      const type = checker.getTypeAtLocation(node.name);
      if (type.getProperty('_def') && type.getProperty('safeParse')) {
        if (ts.isVariableDeclarationList(node.parent) && ts.isVariableStatement(node.parent.parent) && ts.isSourceFile(node.parent.parent.parent)) {
          const entry = { file: relative(source.fileName), name: node.name.text,
            identifier: `__scmContractSchema${localSchemas.length}`, source: sourceLocation(node) };
          localSchemas.push(entry);
          schemaDeclarations.push(entry);
        } else scopedSchemaDeclarations.push(sourceLocation(node));
      }
    }
    ts.forEachChild(node, visit);
  }
  visit(source);
  const copy = path.join(runtimeRoot, relative(source.fileName));
  await mkdir(path.dirname(copy), { recursive: true });
  const appended = localSchemas.map((entry) => `export { ${entry.name} as ${entry.identifier} };`).join('\n');
  let instrumented = source.text;
  for (const insertion of semanticInsertions.sort((a, b) => b.position - a.position)) instrumented = instrumented.slice(0, insertion.position) + insertion.text + instrumented.slice(insertion.position);
  const tagHelper = 'function __scmContractTagSchema<T>(value: T, origin: unknown): T { if (value && typeof value === "object") Object.defineProperty(value, Symbol.for("scm.contract.origin"), { value: origin, configurable: true }); return value; }';
  await writeFile(copy, `${instrumented}\n${tagHelper}\n${appended}\n`);
}
// Resolve zod from the frozen dependencies, not an unpinned independent install.
await mkdir(path.join(runtimeRoot, 'node_modules'), { recursive: true });
const { symlink } = await import('node:fs/promises');
await symlink(path.join(baseline, 'node_modules/zod'), path.join(runtimeRoot, 'node_modules/zod'));
for (const name of Object.keys(process.env)) delete process.env[name];
globalThis.fetch = async () => { throw new Error('Network is forbidden during contract extraction'); };
if ('WebSocket' in globalThis) Object.defineProperty(globalThis, 'WebSocket', {
  configurable: true, value: class { constructor() { throw new Error('Network is forbidden during contract extraction'); } },
});
const runtimeModules = new Map<string, Record<string, unknown>>();
for (const source of sourceFiles) runtimeModules.set(relative(source.fileName), await import(pathToFileURL(path.join(runtimeRoot, relative(source.fileName))).href));
const runtimeExports = runtimeModules.get('src/index.ts')!;
const runtimeNames = Object.keys(runtimeExports).filter((name) => !name.startsWith('__scmContractSchema')).sort();
const checkerValueNames = publicSymbols.filter(valueExport).map((symbol) => symbol.name).sort();
if (json(runtimeNames) !== json(checkerValueNames)) throw new Error(`Runtime/checker value export disagreement: ${json({ onlyRuntime: runtimeNames.filter((name) => !checkerValueNames.includes(name)), onlyChecker: checkerValueNames.filter((name) => !runtimeNames.includes(name)) })}`);

function portable(value: unknown): unknown {
  if (value === undefined) return { $undefined: true };
  if (typeof value === 'number' && !Number.isFinite(value)) return { $number: String(value) };
  if (typeof value === 'bigint') return { $bigint: String(value) };
  if (typeof value === 'function') return { $function: value.toString(), portability: 'requires_reviewed_rust_implementation' };
  if (value instanceof RegExp) return { $regexp: value.source, flags: value.flags };
  if (value instanceof Date) return { $date: value.toISOString() };
  if (Array.isArray(value)) return value.map(portable);
  if (value && typeof value === 'object') return Object.fromEntries(Object.entries(value).map(([key, child]) => [key, portable(child)]));
  return value;
}

type Schema = { _def: Record<string, any>; description?: string };
const schemaIds = new Map<Schema, string>();
const schemaDefinitions: Record<string, Record<string, unknown>> = {};
const manualSemantics: { schema: string; kind: string; source?: unknown; originalExpression?: unknown }[] = [];
const unsupported: { schema: string; kind: string }[] = [];
function schemaRef(schema: Schema): { $ref: string } {
  const previous = schemaIds.get(schema);
  if (previous) return { $ref: previous };
  const id = `s${String(schemaIds.size + 1).padStart(6, '0')}`;
  schemaIds.set(schema, id);
  const definition = schema._def;
  const kind = String(definition.typeName);
  const result: Record<string, unknown> = { kind, ...(schema.description ? { description: schema.description } : {}) };
  const origin = (schema as any)[Symbol.for('scm.contract.origin')];
  if (origin) result.semanticSource = origin;
  schemaDefinitions[id] = result;
  switch (kind) {
    case 'ZodObject': {
      result.unknownFields = definition.unknownKeys;
      result.catchall = schemaRef(definition.catchall);
      result.properties = Object.fromEntries(Object.entries(definition.shape()).map(([name, child]) => {
        const typed = child as Schema;
        // isOptional()/isNullable() execute parsers, and therefore arbitrary refinements.
        // Structural acceptance is derived separately without invoking validation code.
        return [name, { schema: schemaRef(typed), inputPresence: presence(typed, 'undefined'), inputNull: presence(typed, 'null') }];
      }));
      break;
    }
    case 'ZodString': case 'ZodNumber': case 'ZodBigInt': case 'ZodDate':
      result.checks = portable(definition.checks ?? []);
      result.coerce = definition.coerce ?? false;
      break;
    case 'ZodArray':
      result.element = schemaRef(definition.type);
      result.minLength = portable(definition.minLength);
      result.maxLength = portable(definition.maxLength);
      result.exactLength = portable(definition.exactLength);
      break;
    case 'ZodTuple':
      result.elements = definition.items.map(schemaRef);
      result.rest = definition.rest ? schemaRef(definition.rest) : null;
      break;
    case 'ZodEnum': result.values = portable(definition.values); break;
    case 'ZodNativeEnum': {
      result.enumObject = portable(definition.values);
      result.values = Object.keys(definition.values).filter((key) => typeof definition.values[definition.values[key]] !== 'number').map((key) => definition.values[key]);
      break;
    }
    case 'ZodLiteral': result.value = portable(definition.value); break;
    case 'ZodUnion': result.variants = definition.options.map(schemaRef); break;
    case 'ZodDiscriminatedUnion':
      result.discriminator = definition.discriminator;
      result.variants = definition.options.map(schemaRef);
      result.discriminatorMapping = [...definition.optionsMap].map(([value, child]) => ({ value: portable(value), schema: schemaRef(child) }));
      break;
    case 'ZodIntersection': result.left = schemaRef(definition.left); result.right = schemaRef(definition.right); break;
    case 'ZodRecord': case 'ZodMap': result.key = schemaRef(definition.keyType); result.value = schemaRef(definition.valueType); break;
    case 'ZodSet': result.element = schemaRef(definition.valueType); result.minSize = portable(definition.minSize); result.maxSize = portable(definition.maxSize); break;
    case 'ZodOptional': case 'ZodNullable': case 'ZodReadonly': result.inner = schemaRef(definition.innerType); break;
    case 'ZodBranded': result.inner = schemaRef(definition.type); break;
    case 'ZodDefault': case 'ZodCatch': {
      result.inner = schemaRef(definition.innerType);
      const provider = definition.defaultValue ?? definition.catchValue;
      result.provider = portable(provider);
      manualSemantics.push({ schema: id, kind, source: portable(provider), ...(origin ? { originalExpression: origin } : {}) });
      break;
    }
    case 'ZodEffects':
      result.inner = schemaRef(definition.schema);
      result.effect = portable(definition.effect);
      result.semanticPortability = 'requires_reviewed_rust_implementation';
      manualSemantics.push({ schema: id, kind: `effect:${definition.effect.type}`, source: portable(definition.effect), ...(origin ? { originalExpression: origin } : {}) });
      break;
    case 'ZodPipeline': result.input = schemaRef(definition.in); result.output = schemaRef(definition.out); break;
    case 'ZodLazy': result.target = schemaRef(definition.getter()); break;
    case 'ZodPromise': result.inner = schemaRef(definition.type); break;
    case 'ZodFunction': result.arguments = schemaRef(definition.args); result.returns = schemaRef(definition.returns); break;
    case 'ZodBoolean': result.coerce = definition.coerce ?? false; break;
    case 'ZodAny': case 'ZodUnknown': case 'ZodNever': case 'ZodNull': case 'ZodUndefined': case 'ZodVoid': case 'ZodNaN': case 'ZodSymbol': break;
    default:
      unsupported.push({ schema: id, kind });
      result.semanticPortability = 'unsupported_requires_manual_schema';
  }
  return { $ref: id };
}

type Presence = 'accepts' | 'rejects' | 'requires_semantic_review';
function presence(schema: Schema, value: 'null' | 'undefined', seen = new Set<Schema>()): Presence {
  if (seen.has(schema)) return 'requires_semantic_review';
  const branch = new Set(seen).add(schema);
  const def = schema._def;
  const kind = def.typeName;
  if (['ZodAny', 'ZodUnknown'].includes(kind)) return 'accepts';
  if (kind === 'ZodNull') return value === 'null' ? 'accepts' : 'rejects';
  if (kind === 'ZodUndefined' || kind === 'ZodVoid') return value === 'undefined' ? 'accepts' : 'rejects';
  if (kind === 'ZodLiteral') return def.value === (value === 'null' ? null : undefined) ? 'accepts' : 'rejects';
  if (kind === 'ZodOptional' && value === 'undefined' || kind === 'ZodNullable' && value === 'null') return 'accepts';
  if (kind === 'ZodEffects' || kind === 'ZodCatch' || kind === 'ZodDefault' || def.coerce) return 'requires_semantic_review';
  if (['ZodOptional', 'ZodNullable', 'ZodReadonly', 'ZodBranded'].includes(kind)) return presence(def.innerType ?? def.type, value, branch);
  if (kind === 'ZodLazy') return presence(def.getter(), value, branch);
  if (kind === 'ZodPipeline') return 'requires_semantic_review';
  if (kind === 'ZodUnion' || kind === 'ZodDiscriminatedUnion') {
    const variants = def.options.map((child: Schema) => presence(child, value, branch));
    return variants.includes('accepts') ? 'accepts' : variants.includes('requires_semantic_review') ? 'requires_semantic_review' : 'rejects';
  }
  if (kind === 'ZodIntersection') {
    const variants = [presence(def.left, value, branch), presence(def.right, value, branch)];
    return variants.includes('rejects') ? 'rejects' : variants.includes('requires_semantic_review') ? 'requires_semantic_review' : 'accepts';
  }
  return 'rejects';
}

const schemaRoots = schemaDeclarations.map((entry) => {
  const schema = runtimeModules.get(entry.file)![entry.identifier] as Schema;
  if (!schema?._def) throw new Error(`Instrumented schema unavailable: ${entry.file}:${entry.name}`);
  return { name: entry.name, source: entry.source, proposedNamespace: namespaceFor(entry.source.file, entry.name), schema: schemaRef(schema) };
});

const exports = publicSymbols.map((exportSymbol) => {
  const symbol = resolveAlias(exportSymbol);
  const declarations = symbol.declarations ?? [];
  const declaration = symbol.valueDeclaration ?? declarations[0];
  if (!declaration) throw new Error(`Missing declaration for export ${exportSymbol.name}`);
  const source = sourceLocation(declaration);
  const runtime = runtimeExports[exportSymbol.name];
  const schema = runtime && typeof runtime === 'object' && '_def' in runtime && 'safeParse' in runtime;
  const kind = schema ? 'schema' : symbol.flags & ts.SymbolFlags.Class ? 'class'
    : symbol.flags & ts.SymbolFlags.Interface ? 'interface' : symbol.flags & ts.SymbolFlags.TypeAlias ? 'type_alias'
      : symbol.flags & ts.SymbolFlags.Enum ? 'enum' : symbol.flags & ts.SymbolFlags.Function ? 'function' : 'constant';
  const type = kind === 'class' || kind === 'interface' || kind === 'type_alias'
    ? checker.getDeclaredTypeOfSymbol(symbol) : checker.getTypeOfSymbolAtLocation(symbol, declaration);
  const members = (kind === 'class' || kind === 'interface' || exportSymbol.name === 'globalConfiguration')
    ? sort(checker.getPropertiesOfType(type).filter(publicMember), (property) => property.name).map((property) => methodInfo(property, exportSymbol.name, false)).filter(Boolean)
    : [];
  const staticType = kind === 'class' ? checker.getTypeOfSymbolAtLocation(symbol, declaration) : undefined;
  const staticMembers = staticType ? sort(checker.getPropertiesOfType(staticType).filter((property) => property.name !== 'prototype' && publicMember(property)), (property) => property.name)
    .map((property) => methodInfo(property, exportSymbol.name, true)).filter(Boolean) : [];
  const entry: Record<string, any> = {
    name: exportSymbol.name, kind, valueExport: valueExport(exportSymbol),
    source, declarations: declarations.map(sourceLocation), documentation: docs(symbol),
    proposedNamespace: namespaceFor(source.file, exportSymbol.name), members, staticMembers,
  };
  if (schema) entry.schema = schemaRef(runtime as Schema);
  else if (kind === 'function') entry.signatures = checker.getSignaturesOfType(type, ts.SignatureKind.Call).map(signatureInfo);
  else if (kind === 'class') entry.constructors = checker.getSignaturesOfType(staticType!, ts.SignatureKind.Construct).map(signatureInfo);
  else entry.type = typeRef(type);
  if (kind === 'constant' || kind === 'enum' || kind === 'type_alias' && valueExport(exportSymbol)) {
    if (typeof runtime !== 'function' && exportSymbol.name !== 'globalConfiguration') entry.value = portable(runtime);
  }
  return entry;
});

const classifiedEntries = exports.flatMap((entry) => [entry, ...entry.members, ...entry.staticMembers]);
const internalClassOwners = new Set(['<object>', 'Configuration', 'Content', 'ApiKeyAuth', 'OAuthAuth', 'TsgHeaderAuth']);
const clientSourceMethods = sourceMethods.filter((entry) => !internalClassOwners.has(entry.owner));
const clientMethodSourceKeys = clientSourceMethods.map((entry) => `${entry.source.file}:${entry.source.start}`).sort();
const sourceBodyKeys = new Set(sourceMethods.map((entry) => `${entry.source.file}:${entry.source.start}`));
const exportedClientMethodSourceKeys = [...new Set(exports.filter((entry) => entry.kind === 'class' && entry.name !== 'Content')
  .flatMap((entry) => [...entry.members, ...entry.staticMembers])
  .filter((entry) => entry.kind === 'method')
  .flatMap((entry) => entry.sources.map((source: any) => `${source.file}:${source.start}`))
  .filter((key) => sourceBodyKeys.has(key)))].sort();
if (json(clientMethodSourceKeys) !== json(exportedClientMethodSourceKeys)) throw new Error('AST/checker client method-body disagreement');
await mkdir(contractRoot, { recursive: true });
if (initialize) {
  try {
    await readFile(classificationPath);
    throw new Error('Classification already exists. Edit decisions explicitly; initialization never overwrites them.');
  } catch (error) {
    if (!(error && typeof error === 'object' && 'code' in error && error.code === 'ENOENT')) throw error;
  }
  const entries = Object.fromEntries(classifiedEntries.map((entry) => {
    const id = entry.id ?? entry.name;
    const owner = exports.find((export_) => export_.name === (entry.id?.split(/[.:]/)[0] ?? entry.name))!;
    const tags = entry.documentation.tags.map((tag: { name: string }) => tag.name);
    const namespace = owner.proposedNamespace;
    const agentguard = namespace === 'airs::agentguard';
    const deprecated = tags.includes('deprecated');
    const experimental = agentguard || tags.includes('experimental');
    return [id, {
      namespace,
      feature: namespace === 'airs::agentguard' ? 'agentguard' : namespace.split('::').at(-1),
      support: deprecated ? 'baseline_deprecated' : experimental ? 'baseline_experimental' : tags.includes('internal') ? 'internal_export' : 'baseline_supported',
      rustImplementation: 'unimplemented', liveAcceptance: 'unverified',
      rationale: agentguard ? 'Observed AgentGuard service contracts are experimental as a product family.'
        : deprecated ? 'Frozen declaration documents a deprecated or broken baseline capability; preserve capability with explicit limitations.'
          : experimental ? 'Frozen implementation documents unresolved service or provider acceptance; Rust must retain experimental classification.'
            : tags.includes('internal') ? 'Exported despite an internal TypeScript annotation; full parity includes a documented Rust equivalent.'
              : 'Included in the frozen public export contract. Baseline support is not a claim of Rust implementation or live acceptance.',
      ...(entry.kind === 'method' ? {
        execution: ['provision', 'forceDelete', 'forceDeleteWithAudit', 'bindWorkspace'].includes(entry.name) ? 'workflow'
          : entry.executionEvidence.some((item: any) => item.protocols.includes('websocket')) ? 'realtime'
            : entry.executionEvidence.some((item: any) => item.protocols.includes('sse')) ? 'stream'
              : entry.executionEvidence.some((item: any) => item.paginationCalls.length || item.listOrIterator && item.hasLoop) ? 'pagination_workflow'
                : entry.executionEvidence.some((item: any) => item.transportCalls.length) ? 'direct_transport'
                  : 'helper_or_delegated',
      } : {}),
    }];
  }));
  await writeFile(classificationPath, json({ schemaVersion: 1, baselineCommit, policy: 'Every exported capability and public member requires an explicit classification. Tags are evidence, not the source of truth. Edit classifications deliberately; ordinary extraction never invents missing entries.', entries }));
}
const classifications = JSON.parse(await readFile(classificationPath, 'utf8'));
validateClassificationHeader(classifications);
const ids = classifiedEntries.map((entry) => entry.id ?? entry.name).sort();
const policyIds = Object.keys(classifications.entries).sort();
if (json(ids) !== json(policyIds)) throw new Error('Classification entries are missing or stale; review every changed export/member explicitly');
const supportValues = ['baseline_supported', 'baseline_experimental', 'baseline_deprecated', 'internal_export'];
for (const entry of classifiedEntries) {
  const id = entry.id ?? entry.name;
  const policy = classifications.entries[id];
  if (!supportValues.includes(policy.support) || !policy.namespace || !policy.feature || !policy.rationale || policy.liveAcceptance !== 'unverified' || policy.rustImplementation !== 'unimplemented') {
    throw new Error(`Incomplete or misleading frozen classification: ${id}`);
  }
  if (entry.kind === 'method' && !['workflow', 'realtime', 'stream', 'pagination_workflow', 'direct_transport', 'helper_or_delegated'].includes(policy.execution)) throw new Error(`Missing explicit execution classification: ${id}`);
  entry.classification = policy;
}

const sourceFingerprint = hash(hashes.map((entry) => `${entry.sha256}  ${entry.file}\n`).join(''));
const counts = {
  exports: exports.length,
  runtimeExports: runtimeNames.length,
  typeOnlyExports: exports.length - runtimeNames.length,
  kinds: Object.fromEntries([...new Set(exports.map((entry) => entry.kind))].sort().map((kind) => [kind, exports.filter((entry) => entry.kind === kind).length])),
  publicMembers: classifiedEntries.length - exports.length,
  publicMethods: classifiedEntries.filter((entry) => entry.kind === 'method').length,
  baselinePublicClassMethodBodies: sourceMethods.filter((entry) => entry.owner !== '<object>').length,
  baselinePublicClassMethodDeclarations: sourceMethodDeclarations.filter((entry) => entry.owner !== '<object>').length,
  clientMethodBodies: clientSourceMethods.length,
  objectLiteralMethodBodies: sourceMethods.filter((entry) => entry.owner === '<object>').length,
  publicInheritedExternalClassMethods: exports.filter((entry) => entry.kind === 'class').flatMap((entry) => [...entry.members, ...entry.staticMembers]).filter((entry) => entry.kind === 'method' && entry.sources.every((source: any) => !source.file.startsWith('src/'))).length,
  sourceFiles: hashes.length,
  moduleLevelSchemas: schemaRoots.length,
  publicSchemas: exports.filter((entry) => entry.kind === 'schema').length,
  schemaNodes: schemaIds.size,
  typeNodes: typeIds.size,
  manualSemanticNodes: manualSemantics.length,
  unsupportedSchemaKinds: unsupported.length,
  localSchemaDeclarations: scopedSchemaDeclarations.length,
};
const inventory = {
  schemaVersion: 1,
  baseline: { package: '@cdot65/prisma-airs-sdk', version: '0.34.0', commit: baselineCommit, sourceFingerprint, sources: hashes, extractionInputs },
  tools: { typescript: ts.version, zod: JSON.parse(await readFile(path.join(baseline, 'node_modules/zod/package.json'), 'utf8')).version,
    node: process.versions.node, generatorSha256: hash(await readFile(fileURLToPath(import.meta.url))) },
  counts,
  independentChecks: { checkerRootExports: checkerNames.length, astRootExports: astNames.length, runtimeValueExports: runtimeNames.length,
    rootExportNamesSha256: hash(checkerNames.join('\n')), equalNameSets: true,
    astClientMethodBodies: clientSourceMethods.length, checkerClientMethodBodies: exportedClientMethodSourceKeys.length,
    clientMethodBodiesSha256: hash(clientMethodSourceKeys.join('\n')), equalClientMethodBodySets: true },
  guarantees: [
    'All root exports and public inherited members are inventoried with original source, documentation and typed signatures.',
    'Every module-level schema, including private request validators, is evaluated in an instrumented temporary copy.',
    'Schema references preserve shared identity, recursion, object unknown-field policy, input omission/null states, and constraints.',
    'Refinements, transforms, catches and defaults require reviewed semantic ports; extraction never executes their validation callbacks.',
    'No implementation or live-acceptance claim follows from this inventory.',
  ],
  limitations: [
    'Function-local schema declarations and inline schemas remain exact AST/source contracts; arbitrary application functions are never invoked to obtain them.',
    'Effect function text may close over baseline helpers. Original declaration and implementation source, not function text alone, is the semantic authority.',
    'TypeScript-specific types and external library types retain exact type text and source ownership rather than pretending to be portable runtime validators.',
  ],
  exports,
  methods: { baselinePublicMethodBodies: sourceMethods, baselinePublicMethodDeclarations: sourceMethodDeclarations, supportingFunctions },
  schemas: { roots: schemaRoots, definitions: schemaDefinitions, manualSemantics, semanticExpressions, unsupported, functionLocalDeclarations: scopedSchemaDeclarations },
  types: { definitions: typeDefinitions },
};
const output = json(inventory);
const destination = path.join(contractRoot, 'typescript.json');
if (check) {
  if (await readFile(destination, 'utf8') !== output) throw new Error('Frozen inventory differs from deterministic extraction');
} else await writeFile(destination, output);
await rm(runtimeRoot, { recursive: true, force: true });
console.log(json({ mode: check ? 'check' : 'write', sourceFingerprint, counts, independentChecks: inventory.independentChecks }));
if (unsupported.length) process.exitCode = 1;
