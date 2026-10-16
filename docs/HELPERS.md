# Native helpers and constants

The public helper surface preserves the frozen SDK's functionality through ordinary Rust types and functions. It requires no JavaScript runtime. Source helper and constant exports have concrete public probes, implementation bindings and test evidence in `contracts/native-capability-mappings/helpers.json`; isolated configuration and error adapters have their own mappings. Live service acceptance is separate. These pure helpers perform no service calls.

## Constants and isolated initialization

`scm_sdk::constants` exposes 162 static equivalents: endpoint defaults, environment variable names, routes, bounds, retry codes, vocabulary arrays, regional endpoints and directional secret metadata. The 163rd source constant, `globalConfiguration`, maps to caller-owned `configuration::ScanConfiguration`, whose explicit init/reset/getters preserve configuration functionality without a process-wide mutable singleton. `configuration::init(InitOptions)` returns an isolated configuration; environment resolution uses a separately selected `EnvironmentSnapshot`. Existing clients retain their own configuration snapshots. `helpers::init(ClientBuilder)` is an additional convenience for building a validated immutable client.

`SDK_VERSION` follows `CARGO_PKG_VERSION`, and `USER_AGENT` is `PAN-AIRS/<crate-version>-rust-sdk`. `BASELINE_SDK_VERSION` retains `0.34.0` for provenance. Other static values match the frozen exports. `AIRS_ENDPOINTS` is a typed slice of region/URL pairs. Vocabulary arrays remain suggestions where the wire schema accepts future values. Source limits are available as constants; operational defaults are configured through native `Limits` and can be more restrictive.

The optional `iam` feature enables `helpers::generate_workspace_scope_name`, reexported by `shared::iam` and Gateway. It lowercases the display name, collapses non-ASCII-alphanumeric runs to underscores, trims them and uses `workspace` for an empty stem. The result is `ws_<stem>_<six lowercase-alphanumeric characters>`. A bounded OS entropy call with rejection sampling supplies an unbiased suffix. Names over 4,096 UTF-8 bytes and entropy failure return a safe error. This helper neither checks uniqueness remotely nor grants permissions.

## Generic listings and collection

The always-available `helpers` module includes:

| Source capability | Native API |
| --- | --- |
| `serializeListing` | `ListingOptions` and `serialize_listing` |
| `paginate` | `paginate` returning `pagination::Items<T,C>` |
| `collectAll` | `StreamListing<S>` and `collect_all` for any fallible Rust stream |
| `collectSkipPages` | `collect_skip_pages`, `SkipPage<T>`, `WalkAllOptions` |
| `collectSpringPages` | `collect_spring_pages`, `SpringPage<T>`, `SpringWalkOptions` |
| `PaginationPage` | `pagination::Page<T,C>` |
| Collection options | `pagination::CollectOptions` |

Listing serialization emits `skip`, `limit`, then `search`, omitting unset fields and preserving explicit zero and empty strings. Native skips are nonnegative safe integers; search strings have byte and control-character bounds. Product clients retain their own query schemas and defaults.

Cursor iterators receive an initial cursor, a caller fetch function, page bound, session and `RequestOptions`. Fetch callbacks receive a clone of the same absolute budget. The existing native paging engine checks repeated cursors, serialized cursor/history bounds, cancellation between buffered items, page limits and mid-page resume positions.

Skip collectors default to skip zero and limit 50, advance by actual item count, use a supplied nonnegative safe-integer total, and fall back to full-page continuation when total is absent or null. Empty pages terminate. Spring collectors default to page zero and size 50; `last` controls continuation, and an empty non-final page fails. Both return the existing `CollectionResult`, preserving partial rows, truncation and resumable positions.

`collect_all` accepts a mutable `StreamListing<S>` where `S: Stream<Item=Result<T>>`. It stops at an item cap before polling another item. If an item exceeds the remaining byte budget, that item stays buffered in the listing for a later collection call. Successful results report `Complete` or explicit truncation; failures return `StreamCollectionError` with `error()`, `partial()` and `into_parts()`. The listing remains caller-owned. Further advancement after an error is an explicit caller action. One deadline and session cover polling and serialization, including already-buffered data. Arbitrary item streams have no page count; page iterators retain their separate page bound.

Collection defaults remain 10,000 items, 64 MiB of serialized array bytes and 1,000 pages. Zero does not disable a bound. These explicit outcomes replace the source's bare arrays and unlimited `max=0` mode. All collection and iterator formatting conceals item values.

## Native schema composition

`json_nullable(input, inner_parser)` returns `Patch<T>`. Omission and explicit null bypass the inner parser; present nonnull JSON is bounded and passed to it. `Patch<T>` also supports arbitrary native types directly, including types that cannot be represented as JSON. This replaces the exported Zod optional/nullable factory without a runtime schema dependency.

With `models`, `page_schema(input, item_parser)` decodes a generic `dlp::models::Page<T>`, validates each content item with the caller's parser and preserves additional fields. Required content, optional nonnull finite metadata and nested pageable/sort descriptors retain the source envelope. Known metadata nulls and invalid item values are rejected. Applications can compose their own typed parser, including `validation::parse` for a frozen schema. The Rust factory returns parsed typed data rather than a JavaScript Zod object.

## Gateway configuration and JSON paths

Enable `gateway` for the pure helpers under `airs::gateway`:

- `custom_host_configuration(&CustomHostConfigurationOptions)` returns a typed object containing `provider_auth_type: "apiKey"`, the original `custom_host` spelling and `custom_headers`. The URL must be absolute HTTP(S). Header grammar, duplicate case-insensitive names, counts and bytes are checked. Provider Authorization headers are valid configuration data and are concealed in formatting. The SDK does not contact that host.
- `ai_gw_organisations_auth_settings_path` builds the frozen organisation path after checking the tenant's numeric identifier.
- `build_dotted_object(&[GatewayDottedValueEntry])` builds a new `JsonObject`.
- `set_dotted_value(&JsonObject, path, &JsonValue)` returns an immutable updated object.

Dotted paths support object properties, decimal array indexes and escaping of `.`, `[`, `]` and `\`. They begin with an object property. Empty segments, invalid escapes/index syntax, missing separators, `__proto__`, `constructor` and `prototype` path components are rejected. Canonically identical paths are duplicates even if their spelling differs. Build rejects duplicate writes and incompatible object/array shapes; set can replace an existing leaf. Arrays must be dense when the operation finishes, so assigning index one before index zero is valid when both are supplied. Sparse staging avoids allocating memory up to an attacker-supplied index.

Dotted and redaction helpers bound JSON depth to 64, nodes to 100,000 and serialized input/output to 8 MiB. Dotted construction shares a 100,000-step tree/work budget; redaction permits at most 1,000,000 rule traversal steps. Dotted builds accept at most 10,000 entries; paths are at most 4,096 UTF-8 bytes and 64 components. Indexes at or above 100,000 fail before allocation. Combined path/value preparation and final output are bounded. All numeric values must be finite. The native schema parser applies the frozen JSON-value contract. Failed operations leave the caller's data unchanged and return value-free errors.

## Directional secret redaction

`constants::AI_GATEWAY_SECRET_FIELDS` is a typed table of 21 operations and 49 rules: 23 request rules, 26 response rules and eight one-time annotations. Rule metadata remains inspectable and serializable. `AIGatewaySecretOperation` is closed: parsing an unfamiliar operation fails instead of silently leaving secrets visible. `GatewaySecretDirection` defaults to request.

`redact_ai_gateway_secrets(operation, &JsonValue, direction)` clones bounded finite JSON and applies the selected rules. `None` selects request rules. Literal paths select named properties; `*` traverses array entries or object members. Missing paths and scalar shape mismatches are ignored. Matching values or subtrees become `[REDACTED]`. Untouched branches, including arbitrary unrelated keys, remain faithful. Both `value` and `subtree` annotations replace the selected leaf; `oneTime` is descriptive metadata and does not disable redaction. The original input remains unchanged.

This function provides operation-aware redacted data for an explicitly chosen diagnostic sink. Ordinary model, helper, metadata and error formatting already conceals values independently. Explicit serialization and getters intentionally expose caller-owned data.

## Reproduction and evidence

```sh
python3 tools/generate_constants.py --check
tools/node_modules/.bin/tsx tools/helper-oracle.mts --check
python3 tools/generate_models.py --check
python3 tools/test_model_generation.py
cargo +1.99.0 test --locked --target x86_64-unknown-linux-musl \
  --no-default-features --features gateway --test helpers_contracts
```

The independent helper oracle verifies the frozen Git input tree, clears its environment, blocks fetch/WebSocket access and executes the original owned TypeScript functions. It records 157 deterministic cases and all 163 constant expectations. Scope fixtures retain only the expected prefix and suffix validity, avoiding nondeterministic random bytes. Rust tests consume these outputs and add native bounds, unsafe paths/headers, immutable failure, collection partial/resume behavior, shared budgets and formatting checks. Generator guards also pin all 40 behavioral adapter closures, concrete problem-metadata types and the five native binary-builder exceptions. Public export probes compile the actual native paths outside library privacy. These are offline contract checks.
