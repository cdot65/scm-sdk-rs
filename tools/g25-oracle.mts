/** Independent AJV and frozen-Zod outcomes for generated G25 mutations. */
import fs from "node:fs";
import { createHash } from "node:crypto";
import path from "node:path";
import { fileURLToPath, pathToFileURL } from "node:url";
import { createRequire } from "node:module";
import { execFileSync } from "node:child_process";
import { z, type ZodTypeAny } from "zod";
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), "..");
execFileSync(
  "python3",
  ["tools/verify-contracts.py", "--baseline", ".local/typescript"],
  { cwd: root, stdio: "pipe" },
);
const read = (p: string) =>
  JSON.parse(fs.readFileSync(path.join(root, p), "utf8"));
const casesBytes = fs.readFileSync(
  path.join(root, "contracts/compatibility/g25/cases.json"),
);
const corpus = JSON.parse(casesBytes.toString("utf8")),
  audit = read("contracts/compatibility/g25/audit.json");
const vendorSpec = read(
  "contracts/vendor/red-team-data-plane/0.93.0/openapi.json",
);
const inventory = read("contracts/typescript.json");
const require = createRequire(import.meta.url),
  Ajv = require("ajv/dist/2020.js").default;
const ajv = new Ajv({ strict: false, allErrors: true });
require("ajv-formats")(ajv);
ajv.addSchema({ ...vendorSpec, $id: "urn:g25:strict" });
function relaxed(value: any): any {
  if (Array.isArray(value)) return value.map(relaxed);
  if (value === null || typeof value !== "object") return value;
  const copy = Object.fromEntries(
    Object.entries(value).map(([k, entry]) => [k, relaxed(entry)]),
  );
  if (copy.type === "string") {
    delete copy.enum;
    delete copy.format;
    delete copy.minLength;
    delete copy.maxLength;
  }
  if (copy.additionalProperties === false) copy.additionalProperties = true;
  if (copy.discriminator) {
    const key = (copy.discriminator as any).propertyName;
    const known = Object.keys((copy.discriminator as any).mapping);
    return {
      anyOf: [
        copy,
        {
          type: "object",
          required: [key],
          properties: { [key]: { type: "string", not: { enum: known } } },
        },
      ],
    };
  }
  return copy;
}
const relaxedAjv = new Ajv({ strict: false, allErrors: true });
require("ajv-formats")(relaxedAjv);
relaxedAjv.addSchema({ ...relaxed(vendorSpec), $id: "urn:g25:response" });
const responseMemo = new Map<any, any>();
function response(schema: ZodTypeAny): ZodTypeAny {
  if (responseMemo.has(schema)) return responseMemo.get(schema);
  const definition: any = { ...schema._def },
    kind = definition.typeName;
  let result: ZodTypeAny;
  responseMemo.set(
    schema,
    z.lazy(() => result),
  );
  if (kind === "ZodEnum") result = z.string();
  else if (kind === "ZodNativeEnum") {
    const values = Object.keys(definition.values)
      .filter(
        (key) => typeof definition.values[definition.values[key]] !== "number",
      )
      .map((key) => definition.values[key]);
    const types = new Set(values.map((value) => typeof value));
    result = schema;
    if (types.has("string")) result = result.or(z.string());
    if (types.has("number")) result = result.or(z.number());
  } else if (
    kind === "ZodUnion" &&
    definition.options.every(
      (option: any) =>
        option._def.typeName === "ZodLiteral" &&
        typeof option._def.value === "string",
    )
  )
    result = z.string();
  else {
    if (kind === "ZodString")
      definition.checks = definition.checks.filter(
        (mutation: any) =>
          ![
            "uuid",
            "datetime",
            "email",
            "url",
            "regex",
            "rfc3339",
            "date",
            "duration",
          ].includes(mutation.kind),
      );
    else if (kind === "ZodObject") {
      const shape = definition.shape;
      definition.shape = () =>
        Object.fromEntries(
          Object.entries(shape()).map(([k, entry]) => [
            k,
            response(entry as ZodTypeAny),
          ]),
        );
      definition.unknownKeys = "passthrough";
      definition.catchall = response(definition.catchall);
    } else if (
      [
        "ZodOptional",
        "ZodNullable",
        "ZodDefault",
        "ZodCatch",
        "ZodReadonly",
      ].includes(kind)
    )
      definition.innerType = response(definition.innerType);
    else if (kind === "ZodEffects")
      definition.schema = response(definition.schema);
    else if (kind === "ZodArray") definition.type = response(definition.type);
    else if (kind === "ZodRecord") {
      definition.keyType = response(definition.keyType);
      definition.valueType = response(definition.valueType);
    } else if (kind === "ZodLazy") {
      const getter = definition.getter;
      definition.getter = () => response(getter());
    } else if (kind === "ZodUnion")
      definition.options = definition.options.map(response);
    else if (kind === "ZodDiscriminatedUnion") {
      // These are frozen Zod discriminators, whose tags are required. Preserve
      // dispatch (including rejection of absent/unknown tags), as schema-oracle
      // does. Vendor optional-tag oneOf rules stay in AJV, which enforces exactly
      // one successful branch; relaxed() only adds the unknown-string-tag case.
      result = z.discriminatedUnion(
        definition.discriminator,
        definition.options.map(response) as any,
      );
      responseMemo.set(schema, result);
      return result;
    } else if (kind === "ZodIntersection") {
      definition.left = response(definition.left);
      definition.right = response(definition.right);
    } else if (kind === "ZodPipeline") {
      definition.in = response(definition.in);
      definition.out = response(definition.out);
    }
    result = new (schema.constructor as any)(definition);
  }
  responseMemo.set(schema, result);
  return result;
}
// Guard adaptation semantics independently of which frozen roots happen to use
// these kinds today. Numeric native enum reverse mappings must not widen to strings.
for (const [schema, input, accepted] of [
  [z.nativeEnum({ A: "A" }), "NEW", true],
  [z.nativeEnum({ A: "A" }), 7, false],
  [z.nativeEnum({ 0: "A", A: 0 }), 7, true],
  [z.nativeEnum({ 0: "A", A: 0 }), "NEW", false],
  [z.union([z.literal("a"), z.literal("b")]), "NEW", true],
  ...[{}, { kind: "NEW" }, { kind: "a" }, { kind: "a", value: 1 }].map(
    (input, index) => [
      z.discriminatedUnion("kind", [
        z.object({ kind: z.literal("a"), value: z.string().optional() }),
        z.object({ kind: z.literal("b"), value: z.number().optional() }),
      ]),
      input,
      index === 2,
    ],
  ),
] as [ZodTypeAny, unknown, boolean][]) {
  if (response(schema).safeParse(input).success !== accepted)
    throw new Error("G25 response adaptation self-check failed");
}
for (const key of Object.keys(process.env)) delete process.env[key];
globalThis.fetch = async () => {
  throw new Error("Network forbidden in G25 oracle");
};
const validators = [];
for (const row of corpus.roots) {
  const operation = audit.operations.find(
    (o: any) => o.responseRoot === row.root,
  );
  if (!operation) throw new Error("Unclassified root");
  const vendor = Object.values(vendorSpec.paths)
    .flatMap((p: any) => Object.values(p))
    .find((o: any) => o.operationId === operation.operationId) as any;
  const success = Object.entries(vendor.responses).find(([code]) =>
    code.startsWith("2"),
  )![1] as any;
  const schema = success.content["application/json"].schema;
  // Local refs resolve through an explicit root vendorSpec merged into the operation schema.
  const strict = ajv.compile({
    ...schema,
    $id: "urn:g25:case:" + operation.operationId,
    components: vendorSpec.components,
  });
  const loose = relaxedAjv.compile({
    ...relaxed(schema),
    $id: "urn:g25:loose:" + operation.operationId,
    components: relaxed(vendorSpec.components),
  });
  let typescriptSchema: ZodTypeAny | undefined;
  if (operation.baselineOpaque)
    typescriptSchema =
      operation.baselineExpression === "z.unknown()"
        ? z.unknown()
        : z.object({}).passthrough();
  else if (operation.baselineOperation) {
    const name = operation.baselineArray
      ? operation.baselineExpression.slice(8, -1)
      : operation.baselineExpression;
    const declaration = inventory.schemas.roots.find(
      (r: any) => r.name === name,
    );
    if (!declaration) throw new Error("Missing TS declaration");
    const module = await import(
      pathToFileURL(
        path.join(root, ".local/typescript", declaration.source.file),
      ).href
    );
    typescriptSchema = module[name];
    if (!typescriptSchema) throw new Error("Unexported schema " + name);
    if (operation.baselineArray) typescriptSchema = z.array(typescriptSchema!);
  }
  validators.push({
    strict,
    loose,
    typescriptSchema,
    response: typescriptSchema ? response(typescriptSchema) : undefined,
  });
}
function set(doc: any, p: any[], value: any, omit = false): any {
  if (!p.length) return omit ? undefined : structuredClone(value);
  let at = doc;
  for (const key of p.slice(0, -1)) at = at[key];
  if (omit) delete at[p.at(-1)];
  else at[p.at(-1)] = structuredClone(value);
  return doc;
}
const outcomes = [];
for (const [index, mutation] of corpus.cases.entries()) {
  let doc = structuredClone(corpus.roots[mutation.root].base);
  for (const [p, entry] of mutation.choices) doc = set(doc, p, entry);
  const entry = validators[mutation.root];
  if (!entry.strict(doc))
    throw new Error(
      "Invalid strict witness " +
        index +
        " " +
        JSON.stringify(entry.strict.errors),
    );
  doc = set(doc, mutation.path, mutation.value, mutation.omit);
  const strict = Boolean(entry.strict(doc)),
    typescript = entry.typescriptSchema
      ? entry.typescriptSchema.safeParse(doc).success
      : null;
  const expected = entry.response
    ? entry.response.safeParse(doc).success
    : Boolean(entry.loose(doc));
  if (
    (mutation.kind === "enum" || mutation.kind.startsWith("format:")) &&
    !expected
  )
    throw new Error("Response mutation was rejected " + index);
  if (typescript && !expected)
    throw new Error("Compatibility oracle narrowed TypeScript " + index);
  outcomes.push({ strict, typescript, response: expected });
}
const policies = {
  request:
    "Strict pinned vendor schema executed by AJV; original frozen TypeScript schema executed by Zod. No Rust implementation is invoked.",
  vendorResponse:
    "Unpaired operations use AJV after removing string enum, format, minLength and maxLength keywords and widening additionalProperties:false to true. Numeric minimum/maximum/integer and array minItems/maxItems constraints remain. Discriminated unions retain exactly-one-match oneOf semantics for absent tags; an additional branch preserves objects with an unknown string discriminator.",
  frozenResponse:
    "Paired operations exclusively use frozen Zod with unknown object fields preserved; enum vocabularies widened only within their original string/number member types and string-literal unions widened to strings; uuid, rfc3339, datetime, date, duration, email, url and regex checks bypassed. Frozen min/max, requiredness, types, nullability, individual literals, catchalls, refinements, defaults and transforms remain. Frozen discriminated unions retain required known-tag dispatch; absent and unknown tags are rejected.",
};
const output =
    JSON.stringify({
      schemaVersion: 1,
      casesSha256: createHash("sha256").update(casesBytes).digest("hex"),
      policies,
      roots: corpus.roots.length,
      outcomes,
    }) + "\n",
  dest = path.join(root, "contracts/compatibility/g25/outcomes.json");
if (process.argv.includes("--check")) {
  if (fs.readFileSync(dest, "utf8") !== output)
    throw new Error("Stale G25 oracle");
} else fs.writeFileSync(dest, output);
console.log(
  JSON.stringify({
    roots: corpus.roots.length,
    independentOutcomes: outcomes.length,
  }),
);
