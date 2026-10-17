# Native models

Models are ordinary Rust data, generated from the frozen TypeScript contract inventory. Applications need no JavaScript runtime, Node dependency, or schema compiler. Enable the `models` Cargo feature; `scan` enables it automatically.

## Public namespaces

Typed contracts live under `airs::{scan,management,model_security,red_team,gateway,agentguard}::models`, `dlp::models`, `shared::iam::models`, and `shared::models`. Each family reexports its own contracts and reachable nested types. Structurally identical schema roots share canonical Rust declarations. The `models` feature does not select every product client. All frozen baseline methods are now implemented under their owning product features; the operation and export ledgers record those requirements and separate live status.

The generation ledger, `contracts/native-models.json`, lists all 1,086 public schema exports and 1,157 source model type/interface exports. Every known object field has a concrete Rust type. Additional plain JSON interfaces and exact source aliases cover workspace provisioning records, IAM inputs, Gateway chat/embedding aliases, token information, and secret metadata. Behavioral configuration, client, pagination, and realtime interfaces have concrete native adapters with recorded ownership; they are distinct from generated wire models. `Error` provides the native sanitized error metadata equivalent.

## Construction and validation

Non-exhaustive object models expose `builder()` with typed setters. `build()` checks required field presence. It does not enforce every string constraint, cross-field rule, or remote operation policy. Schema-backed builders also provide `build_validated(Mode::Request)` and `build_validated(Mode::Response)`, which run native validation, apply defaults/transforms, and decode the resulting model. Source-only interfaces have no invented schema identifier and are validated by their owning operation.

```rust
use scm_sdk::airs::scan::models::{AiProfile, Optional, ScanRequest, ScanRequestContentsInner};
use scm_sdk::validation::Mode;

let profile = AiProfile::builder()
    .profile_name(Optional::Value("profile-name".into()))
    .build_validated(Mode::Request)?;
let content = ScanRequestContentsInner::builder()
    .prompt(Optional::Value("Text to scan".into()))
    .build_validated(Mode::Request)?;
let request = ScanRequest::builder()
    .ai_profile(profile)
    .contents(vec![content])
    .build_validated(Mode::Request)?;
# Ok::<(), scm_sdk::Error>(())
```

Schema-backed object types expose their precise graph identifier as `SCHEMA` and provide `parse(serde_json::Value, Mode)`. For root aliases or scalar/union contracts, use the family's generic `parse::<Type>(schema_name, value, mode)` function. SDK request clients validate inputs before authentication or network access. Direct Serde decoding is a representation conversion; use native validation to enforce schema semantics and reject malformed known union variants.

The validator is native Rust and is available even without the `models` feature. Its compact graph retains every frozen schema node. The independently reviewed semantic registry and validation documentation describe all 69 refinement/default/transform bindings. Six output transforms affect model representation: four delete strings become `{ "message": ... }` receipts, an MCP mapping envelope becomes an array, and a realtime event retains its typed `type` field and additional JSON fields.

## Wire fidelity

| Contract state | Native representation |
| --- | --- |
| Optional, non-null field | `Optional<T>`: `Unset` or `Value(T)`; explicit null is rejected |
| Required, nullable field | `Nullable<T>`: `Null` or `Value(T)`; omission is rejected |
| Optional, nullable patch field | `Patch<T>`: `Unset`, `Null`, or `Value(T)` |
| JavaScript number | `FiniteNumber`; no accidental narrowing of integers to `i64` |
| Known object with additional fields | Concrete fields plus redacted `Extensions<T>` |
| String enum or suggested vocabulary | Known variants plus explicit `Unknown(String)` |
| Literal discriminator | Exact literal only; mismatches are rejected during deserialization |
| Arbitrary JSON accepted by the source contract | Redacted `JsonValue` or `JsonObject` |
| Native binary input | Owned `BinaryFile`; JSON encoding and decoding are rejected |

Literal discriminator types enforce exact values when decoded. Numeric literal constructors are checked and accept equivalent finite representations such as `1` and `1.0`. Numeric union branches enforce their number constraints before selecting a native variant, so a sentinel literal cannot be captured by an earlier constrained number branch. Response validation preserves unfamiliar enum strings and additional object fields. Request validation retains the frozen constraints and rejects unfamiliar request enum values. Unknown outcomes remain explicit data and never imply an allow decision. String fields remain strings where that is the actual source contract, including scan `action` and `category`.

`Intersection<A, B>` retains each independently normalized half and merges compatible object fields recursively when serialized. Equal-length arrays merge element by element; overlapping scalars must agree, including equivalent numeric representations. Conflicts are rejected during construction, decoding, and serialization. Both halves and their merged output obey the standalone validator's default byte, depth, node, and work bounds; error messages and formatting omit values.

Extensions cannot override or duplicate known fields: each object serializer checks all extension keys before it begins serializing the map. All generated types, builders, enum unknowns, extensions, and shared model containers redact their contents in `Debug`. Explicit getters, public fields, and serialization intentionally expose values to the caller; applications choose how to handle them.

`BinaryFile` owns bytes and optional filename/MIME metadata. Construction does not impose a byte ceiling; the owning operation calls `validate(max_bytes)` and bounds the complete encoded request before authentication. Filename setters reject controls, path separators and quoted header injection; MIME setters require valid header-safe ASCII. `validate(max_bytes)` applies the operation's byte limit. Native binary validators recognize all six frozen Blob boundaries. Date, caller transport, and cancellation inputs use their explicit native validator adapters instead of JSON lookalikes. Binary and realtime admission, encoding and bounds belong to their owning product clients.

The five native multipart builders (image edit, image variation, transcription, translation and file upload) expose `build()` for required-field construction. Their owning runtime operations perform full mixed JSON/native-file validation under the request budget. Prerelease review removed their always-failing JSON-only `build_validated` method; JSON-capable builders retain that method. JSON parsing and serialization continue to reject fake binary values. A generator regression pins the exact five-builder exception.

The model ledger also records handwritten behavioral adapters separately from JSON declarations. All 54 exported Model Security and Red Team option/configuration interfaces and 18 Gateway client/query/window/runtime/pricing adapters map to implemented native types. Phase 7 closed the remaining 40 behavioral adapters, including management/listing metadata, standalone helpers, isolated configuration, configurable OAuth refresh, realtime, provisioning options and AgentGuard. Concrete `error::ProblemDetails` and `error::ErrorMetadata` preserve bounded diagnostic capabilities. No behavioral adapter remains deferred. `contracts/native-exports.json` binds the complete exported capability surface to concrete public paths and compiled probes.

Provisioning uses the handwritten `airs::gateway::GatewayWorkspaceProvisionRequest` and `GatewayWorkspaceProvisionResult` operation types. The models namespace retains its source-only wire-compatible versions for models-only consumers; those generated versions are not accepted directly by the provisioning method. The ledger records that distinction explicitly. Generic list-all settings retain bounded `CollectOptions`; request lifecycle remains separate `RequestOptions`. Directional secret redaction is an implemented helper with its own rule table and independent oracle, separate from safe model formatting.

## Reserved JSON protocol keys

SDK JSON boundaries reject literal objects containing either exact key
`$serde_json::private::Number` or `$serde_json::private::RawValue`, including
escaped spellings and nested objects. These names collide with Serde JSON's
internal value protocols under dependency feature unification and can otherwise
silently change an object into a scalar before validation. This deliberate Rust
security restriction returns a sanitized decoding or validation error; it does
not drop the field or report a successful response with altered data.

The guard applies to SDK HTTP, stream, realtime and model parsing boundaries,
including programmatically constructed `serde_json::Value` inputs. Byte, depth,
node and work limits still apply. Other unknown fields and unfamiliar values
retain their documented handling. Direct calls to downstream
`serde_json::from_value::<SdkModel>` or `from_slice::<SdkModel>` use that crate's
own behavior; use SDK model `parse` methods for the guarded validation boundary.

## Union compatibility and generator guards

Public unions currently use positional names such as `Variant10`. These names are part of the Rust API. `contracts/native-union-variants.json` pins every source branch index, schema/type identity, native variant name and contained Rust type. Normal generation refuses a reorder, replacement, or removal. Any future pin change needs an explicit API migration and review; the one-time initialization command refuses to overwrite an existing ledger.

Generation also compares every nested string/number check kind and regex source with the implemented native dispatcher. New checks, patterns or regex flags fail generation. Record key schemas are required to contain no effects, defaults, pipelines, catches or coercion, because the current native record adapter preserves the original string keys. Tamper regressions exercise all of these guards.

## Reproduction and evidence

Run from the repository root:

```sh
python3 tools/generate_models.py
python3 tools/generate_models.py --check
python3 tools/generate_models.py --verify-fixtures
```

Generation uses Python's standard library, the committed inventory, and pinned Rustfmt 1.99.0. Consumer builds need none of that tooling. `--graph-only` checks or regenerates the native validator graph separately. `--only scan` is a development slice and must not be accepted as a complete generation ledger.

The separate native consumer compiles a concrete type match for every public schema and roundtrips successful independent Zod/response fixtures. The initial complete run passed 27,216 accepted cases across 1,081 JSON-capable public schemas; five public schema roots require actual native binary input. The independent native boundary tests cover those inputs. The schema oracle also contains rejected and private-root cases, exercised by the native validator suite rather than the model roundtrip check. This is offline contract evidence, not live service acceptance.

Packaged unit regressions use a compact independently bound oracle subset rather
than the full development corpus. See [PACKAGING.md](PACKAGING.md) for actual
extracted-archive tests, [STABILITY.md](STABILITY.md) for public union/API policy,
and [LIVE_ACCEPTANCE.md](LIVE_ACCEPTANCE.md) for service qualification.
