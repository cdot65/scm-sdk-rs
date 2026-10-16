# Explicit initialization and environment loading

`configuration::init(InitOptions)` returns an isolated `ScanConfiguration`. It validates a complete replacement before committing any state. `ScanConfiguration::init` replaces that instance, `reset` returns it to an uninitialized state, and `client_builder` snapshots validated credentials, endpoint and retry limits. A built client retains its snapshot when the configuration is reset or reinitialized. The SDK has no mutable global configuration and no constructor reads process environment or tenant registrations.

An API key or bearer token is required; both may be supplied. `InitOptions` uses redacted, zeroizing `Secret` values and a validated `Endpoint`. Credential resolution trims the ECMAScript whitespace set, treats a blank value as absent, and checks the source 2,048 UTF-16-unit maximum. HTTP header safety adds an ASCII/header validation requirement. The scan endpoint defaults to `https://service.api.aisecurity.paloaltonetworks.com`; trailing slashes normalize as in the source. HTTPS is required, with a separately constructed `Endpoint::loopback` for explicit local adapters. Retry count is an integer from zero through five, default five. Mutations still require the engine's explicit replay policy.

## Environment snapshots

`EnvironmentSnapshot::from_pairs` accepts application-provided values. `EnvironmentSnapshot::capture(&names)` reads only the explicitly selected variable names from the process; it rejects non-UTF-8 values and never enumerates the environment. Empty and absent values remain distinct. Stored strings are zeroized on drop and ordinary formatting reports only the number of variables. `get` is explicit raw access and its result may contain credentials.

The snapshot is bounded to 128 supplied entries, 16 KiB per value, and 256 KiB aggregate name/value bytes. Names are ASCII letters, digits and underscore, at most 128 bytes. Invalid input errors omit variable names and values. Capturing an empty whitelist performs no reads.

Use `configuration::init_from_environment(options, &snapshot)` or `ScanConfiguration::init_from_environment` to opt into scan fallbacks. `SCAN_ENVIRONMENT_NAMES` contains exactly `PANW_AI_SEC_API_KEY`, `PANW_AI_SEC_API_TOKEN`, and `PANW_AI_SEC_API_ENDPOINT`. Each explicit field takes precedence over its environment value. An explicitly blank credential suppresses its environment fallback; the other credential must then suffice. The ordinary `init` path uses an empty snapshot.

```rust
use scm_sdk::{Secret, configuration::{InitOptions, init_from_environment},
    environment::{EnvironmentSnapshot, SCAN_ENVIRONMENT_NAMES}};

let environment = EnvironmentSnapshot::capture(&SCAN_ENVIRONMENT_NAMES)?;
let mut options = InitOptions::default();
options.api_key = Some(Secret::new("application-supplied-key")?);
let configuration = init_from_environment(options, &environment)?;
let client = configuration.client_builder()?.build()?;
# Ok::<(), scm_sdk::Error>(())
```

## OAuth environment resolution

`resolve_oauth_config(ResolveOAuthConfigOptions, &snapshot)` resolves each credential independently in source order: explicit option, primary prefix, then optional fallback prefix. The suffixes are `_CLIENT_ID`, `_CLIENT_SECRET`, `_TSG_ID`, and `_TOKEN_ENDPOINT`. `environment_names` returns that exact whitelist for optional capture. Explicit empty values shadow lower-priority values and fail validation when required. OAuth credentials are not trimmed. The default issuer is the PANW OAuth token endpoint. Retry input is an integer clamped to zero through five, default five.

The returned `OAuthServiceConfig` exposes the admitted base endpoint and immutable `AuthContext`. Its `client_builder(tsg_header)` carries credentials, retries and provider options into the normal engine. The caller supplies transport, destination policy, TLS and session. `provider(transport, policy, session)` constructs an application-owned `OAuthProvider`, allowing refresh observation before attaching it through `Authentication::oauth`. Both routes preserve the native issuer checks and refresh bounds.

Explicit endpoint arguments on product constructors remain authoritative. Environment resolution does not automatically admit another origin, invoke a credential CLI, read a tenant file, change global state, or own background tasks.

## Evidence

`tests/configuration_contracts.rs` exercises source precedence, blank suppression, exact JavaScript trim, bounds, retry rules, atomic replacement, independent configuration/client lifetimes, exact key/token/HMAC headers, provider option propagation, policy/session isolation, and child-process whitelisted/non-UTF-8 capture. The child tests use synthetic variables without mutating the test process environment.
