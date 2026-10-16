# Native validation and wire data

The native validator interprets a compact, pinned schema graph as data. It does not execute JavaScript or require Node in the published crate. The graph contains all 13,018 frozen schema nodes and all 1,125 module declarations, with aliases for 1,086 public schemas. Public schema names, stable `sNNNNNN` IDs, and `source-file#private-name` aliases resolve through the same registry.

`validation::parse(name, value, Mode::Request)` validates a present JSON value and returns its normalized wire representation. `parse_optional` accepts `None` for the source's undefined/omitted state; JSON null remains a separate value. Both have `_with_limits` variants. An unknown schema or invalid payload returns a structured, sanitized SDK error containing no rejected value, field contents, schema source text, or upstream message.

`parse_array` and `parse_array_with_limits` validate anonymous array contracts against a named element schema. The entire array shares one input preflight and one validation/copy budget, including all rows and union alternatives.

## Request and response policy

Requests preserve the source's strict, strip, passthrough, and catchall behavior. Required fields, nullable fields, missing defaults, source ordering of effects, cross-field checks, and six output transforms are implemented natively. Present optional nonnull fields reject null. Patch fields distinguish omission, null, and replacement. Request validation belongs before authentication and network work.

Responses preserve additional object fields and unfamiliar enum values while checking known required structure, nullability, scalar types, and constraints. This widening applies to `ZodEnum`, `ZodNativeEnum` with the correct scalar kind, and unions consisting entirely of string literals. Individual literals and structured-union discriminators remain strict; widening them could select a variant with the wrong required fields. Known catchall values still pass their declared validator. An unfamiliar security verdict remains an unfamiliar value; validation never maps it to allow or benign.

Rust object maps faithfully preserve response extension data, including keys that are ordinary data in Rust. Request object/record normalization retains Zod's omission of the `__proto__` merge key. The explicit Gateway finite-JSON schemas reject `__proto__`, `constructor`, and `prototype` recursively in both modes. These schemas also reject nonfinite and non-JSON runtime values; Rust JSON arrays are inherently dense and cannot contain JavaScript accessors, symbols, cycles, or inherited prototypes.

## Presence and numbers

| Native wrapper | Wire states | Containing field behavior |
| --- | --- | --- |
| `Optional<T>` | omitted or nonnull value | `serde(default)` plus `Optional::is_unset`; explicit null fails |
| `Nullable<T>` | required null or value | no serde default; a missing field fails |
| `Patch<T>` | omitted, null, or value | `serde(default)` plus `Patch::is_unset` |
| `FiniteNumber` | finite JSON number | integer and range constraints come from its schema |
| `JsonValue` / `JsonObject` | faithful arbitrary JSON | explicit access to contents; ordinary formatting is redacted |

Serializing an unset wrapper outside a container that skips it fails rather than silently producing null. The optional/patch defaults have no `T: Default` bound. `FiniteNumber` retains the JSON number representation, supports integer conversions and checked floating-point construction, and rejects NaN/infinity. Schema numeric comparisons follow the frozen JavaScript number checks. Decimal multiple checks mirror Zod's `floatSafeRemainder`, using ECMAScript number and fixed formatting from `ryu-js`, rather than adding an approximate tolerance.

Rust retains exact `i64`/`u64` JSON integers, including values beyond JavaScript's exact-integer range. The source JavaScript runtime can round such inputs when parsing JSON; native validation preserves the original integer while applying the source's floating-point comparisons. This is a deliberate precision-preserving domain difference. It does not relax an explicit schema range. Checked construction and decoding also remain finite if another dependency enables serde_json's `arbitrary_precision` feature.

String length limits count UTF-16 code units, matching Zod rather than UTF-8 bytes or grapheme clusters. The native regex translations preserve ECMAScript ASCII digits and whitespace. Email and UUID checks match the frozen patterns; UUID checking does not invent a version restriction. Date-time checks preserve the pinned Zod calendar/clock/offset pattern, including its acceptance of syntactic offsets that JavaScript `Date.parse` cannot compare. The AgentGuard range refinement separately applies the source's timestamp comparison and rejects an uncomparable pair.

## Native runtime inputs

| Source runtime value | Native contract | JSON behavior |
| --- | --- | --- |
| fetch function | caller-owned `Transport`; `validate_transport` | JSON descriptors rejected |
| `AbortSignal` | `RequestOptions`; `validate_cancellation` | JSON descriptors rejected |
| JavaScript `Date` | bounded `NativeDate` epoch milliseconds; `validate_date` | strings, numbers, and objects rejected as Date substitutes |
| `Blob` / file | canonical `BinaryFile`, reexported as `BinaryPart`; `validate_binary` | JSON serialization/deserialization and spoofed objects rejected |

Binary input is available with the `models` feature. Its size, basename, and MIME metadata are checked by the canonical native type; explicit getters expose data while formatting remains redacted. File-containing requests require their multipart adapter to validate metadata and binary fields and encode the actual bytes. The later multipart milestone supplies those request encoders; a successful typed binary predicate is not evidence that a whole multipart API operation has already shipped.

## Resource bounds

Default `ValidationLimits` allow 64 levels of actual JSON nesting, 100,000 input nodes, 8 MiB of input strings/keys, 1,000,000 validation/copy steps, and 32 MiB of copied strings/keys. `ValidationLimits::for_bytes` derives byte bounds from a configured request/response budget up to 512 MiB. Its node/work budgets scale monotonically in 8 MiB increments from those defaults, capped at 1,000,000 nodes and 16,000,000 steps; copied-data bounds are four times the byte budget, capped at 512 MiB. Explicit limits can select up to 128 JSON levels, 1,000,000 input nodes, 16,000,000 steps, and 2 GiB of copied data. Limits are validated before use; these caps are SDK resource policy rather than PANW service quotas. Wire-size limits remain enforced separately by serialization and HTTP body reading.

Optional, nullable, default, lazy, and effect wrappers are traversed iteratively and do not consume JSON nesting depth. Static wrapper chains have their own 64-step bound. Union backtracking shares the operation's work/copy budget instead of resetting it for each branch. Input preflight is iterative. Rejected, overdeep caller-created values are drained iteratively so cleanup cannot overflow the native call stack; releasing already-owned input still necessarily visits its contents.

An application allowing a large context or response must pass matching validation limits through `_with_limits`; the default 8 MiB policy is not a claim that larger source contracts are unsupported. `NativeDate` follows JavaScript Date's finite timestamp range. JSON input uses Rust/serde's valid UTF-8 representation; unpaired UTF-16 surrogate strings and arbitrary JavaScript object identities are outside that JSON domain.

## Reproduction and evidence

`python tools/generate_models.py --graph-only --check` verifies the compact schema graph. `python tools/generate_validation.py --check` verifies all 69 explicit semantic bindings and their compact runtime registry. Every binding pins its reviewed source-expression hash, verifies the actual expression text, and requires the frozen source fingerprint covering referenced helpers. Changed semantics require a new native review; they never become accept-all validators.

The bindings cover 40 refinements, 23 missing defaults, and six transforms. The public ledger is `contracts/native-semantics.json`; `validation::tests::all_semantic_bindings_are_exercised` checks each binding's concrete native behavior and exact registry coverage against graph nodes requiring reviewed semantics. Both finite-JSON bindings use only the bounded validator context; a dedicated test admits their complete inner parse, then proves their recursive refinement consumes additional shared work budget in both modes. Dedicated tests cover omission/null/value states, required nullable fields, redaction, UTF-16 lengths, ECMAScript whitespace, calendar and decimal edge cases, response evolution, configured limits, wrapper depth, and safe cleanup of a 10,000-level rejected value.

Independent fixtures in `contracts/native-fixtures/schemas.json` are produced by executing the frozen Zod schemas and a separately defined response-widening policy. The corpus contains 60,271 JSON cases and omitted/default probes, with direct source/runtime bindings for all 69 semantic nodes and typed examples for non-JSON boundaries. The integration suite compares both success/failure and normalized output for both modes. Fixture generation, native tests, review artifacts, and live service acceptance remain separate evidence; these offline checks make no live acceptance claim.

## Native multipart and ordered alternatives

`parse_one_of_with_limits` preserves ordered first-match schema behavior with one
shared preflight, work counter and copied-byte budget across alternatives.

Gateway multipart uses `parse_multipart_with_limits` for the five frozen image,
audio and file upload objects. Native `BinaryFile` references travel separately
from JSON metadata. The validator resolves the pinned lazy object contracts,
validates each actual file against its known Blob slot, checks required fields,
normalizes all metadata with the source strict-object policy, rejects duplicate
or colliding file keys, and bounds file bytes and metadata work. Ordinary JSON
validation continues to reject fabricated Blob objects. The multipart encoder
adds part/header/envelope limits before authentication. Binary builders offer
`build` for required-field presence; their owning operation performs complete
native validation. A JSON-only `build_validated` is intentionally absent from
these five builders because native files cannot be serialized as JSON.
