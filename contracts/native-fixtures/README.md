# Independent frozen schema oracle

`schemas.json` contains expected results from executing the frozen TypeScript
SDK's actual Zod schemas. The Rust implementation is never imported or queried
by the generator. It preserves the original source checkout and evaluates a
disposable instrumented copy with the environment cleared and network globals
disabled. The source commit, source hashes, extractor inputs, inventory hash,
generator hash, pinned tooling files, and Zod version are recorded in the output.

Regenerate or verify with the pinned development dependencies:

```sh
.local/typescript/node_modules/.bin/tsx tools/schema-oracle.mts
.local/typescript/node_modules/.bin/tsx tools/schema-oracle.mts --check
```

The corpus accounts for all 1,086 public schema exports and 1,125 module-level
declarations. Shared identities produce 1,085 unique roots. Additional direct
probes cover all 69 manual semantic nodes and ten native boundary nodes, yielding
1,127 oracle entries. Every entry has actual rejected JSON specimens; no rejected
case is inferred from a label or from the Rust validator.

There are 60,271 concrete JSON cases: 12,956 accepted and 47,315 rejected by the
original Zod parser. Cases cover minimal and populated objects, pinned OpenAPI
specimens, union variants, omitted fields, null and incorrect types, enum values,
string and number boundaries, arrays, additional properties, cross-field
conditions, defaults, transforms, and prototype-unsafe JSON. These are concrete
conformance checks, not an exhaustive proof over every possible value.

Each JSON case records two independent expectations:

- `zod` executes the original frozen schema. Request validation matches its
  acceptance and normalized output, subject to separately documented resource
  limits and typed native boundaries.
- `response` executes actual Zod with the accepted Rust response policy applied:
  object additions are preserved, enum vocabularies remain open, and individual
  literals and structured discriminators remain constrained. Required fields,
  scalar types, catchall contracts, refinements, defaults, and transforms remain
  validated. This policy is explicit and is not attributed to the frozen SDK.

All roots have a real `undefined` probe. Rust tests use `parse_optional` to check
omission and default behavior without collapsing absence into JSON null.

## Native inputs

Five public multipart request roots require a real Blob and cannot have a valid
JSON-only instance: image editing, image variation, transcription, translation,
and file creation. Their successful native specimens are recorded as descriptors
with original Zod observations, and their JSON inputs are verified as rejected.
They are not counted as JSON acceptance passes or complete multipart workflows.

The ten native boundary nodes map to concrete Rust seams:

| JavaScript input | Rust equivalent checked independently |
| --- | --- |
| Blob | Validated binary part with explicit bytes, filename, and media type |
| Date | Validated native epoch-millisecond date |
| AbortSignal | Caller-owned request cancellation |
| fetch callback | Caller-owned transport |

The native tests validate these concrete values, reject JSON objects that imitate
the JavaScript classes, and prove that transport validation does not send a
request. Product-specific multipart serialization remains a separate workflow
milestone. No credentials, live requests, or service acceptance claims are part
of this corpus.

Run the native comparison with:

```sh
cargo test -p scm-sdk --test schema_contracts --no-default-features
cargo test -p scm-sdk --test schema_contracts --all-features
```
