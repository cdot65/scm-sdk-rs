# Environment variables

Environment loading is explicit. SDK constructors do not read process variables,
search credential files, invoke a credential CLI, or load tenant registrations.
Your application selects the values to capture and passes them into configuration.

## Scan configuration

`SCAN_ENVIRONMENT_NAMES` contains exactly these three names:

| Variable | Meaning |
| --- | --- |
| `PANW_AI_SEC_API_KEY` | AIRS scan API key. |
| `PANW_AI_SEC_API_TOKEN` | AIRS scan bearer token. |
| `PANW_AI_SEC_API_ENDPOINT` | Scan service base URL; defaults to the US service when absent. |

At least one usable key or token is required.

```rust
use scm::{
    airs::scan::Scanner,
    configuration::{InitOptions, init_from_environment},
    environment::{EnvironmentSnapshot, SCAN_ENVIRONMENT_NAMES},
};

fn scanner() -> scm::Result<Scanner> {
    let environment = EnvironmentSnapshot::capture(SCAN_ENVIRONMENT_NAMES)?;
    let configuration =
        init_from_environment(InitOptions::default(), &environment)?;
    Scanner::new(configuration.client_builder()?.build()?)
}
```

`capture` reads only the requested names. It does not enumerate the environment.
For applications that obtain credentials from another source, construct an
`EnvironmentSnapshot::from_pairs` or use explicit
[configuration options](/getting-started/configuration).

### Scan precedence

Each explicit `InitOptions` field takes precedence over its environment fallback.
An explicitly blank credential suppresses the corresponding environment value;
the other credential must then provide authentication. Scan credentials are
trimmed during resolution.

The [quick-start example](/getting-started/quick-start) reads `AIRS_API_KEY` and
`AIRS_PROFILE_NAME` directly in application code. Those example-specific names
are not aliases automatically recognized by the scan environment loader.

## OAuth service credentials

`ResolveOAuthConfigOptions` lets the application choose an environment prefix.
For the example prefix `SCM`, it selects:

| Variable | Meaning |
| --- | --- |
| `SCM_CLIENT_ID` | OAuth service account client ID. |
| `SCM_CLIENT_SECRET` | OAuth service account client secret. |
| `SCM_TSG_ID` | Tenant service group identifier. |
| `SCM_TOKEN_ENDPOINT` | Optional token issuer URL. |

The default issuer is
`https://auth.apps.paloaltonetworks.com/oauth2/access_token`.
The API base URL remains an explicit application input.

```rust
use scm::{
    Client, Endpoint,
    environment::{
        EnvironmentSnapshot, ResolveOAuthConfigOptions, resolve_oauth_config,
    },
};

fn oauth_client() -> scm::Result<Client> {
    let options = ResolveOAuthConfigOptions::new(
        Endpoint::parse("https://api.sase.paloaltonetworks.com/aisec")?,
        "SCM",
    );
    let names = options.environment_names()?;
    let selected: Vec<&str> = names.iter().map(String::as_str).collect();
    let environment = EnvironmentSnapshot::capture(&selected)?;
    let configuration = resolve_oauth_config(options, &environment)?;
    configuration.client_builder(false).build()
}
```

This chooses an AIRS management base and disables the optional tenant header.
Product guides specify the endpoint and tenant-header requirements for other
clients.

### OAuth precedence

Resolution happens independently for each field:

1. Explicit option.
2. Primary environment prefix.
3. Optional fallback prefix.

Set `fallback_env_prefix` before calling `environment_names` to include those
fallback variables in the capture list. An explicit empty value shadows lower
priority sources and fails validation when required. OAuth credentials are not
trimmed. Tenant IDs must contain 1–32 ASCII digits.

## Snapshot behavior

Snapshots preserve the distinction between missing and empty values. Stored
strings are zeroized on drop, and ordinary formatting reports only the number of
variables. Calling `get` returns the raw value, which may contain a credential.

Capture rejects non-UTF-8 values. Bounds are 128 entries, 16 KiB per value, and
256 KiB of aggregate name/value bytes. A snapshot is a point-in-time input:
changing the process environment does not update already-built clients.

See [configuration](/getting-started/configuration) for client isolation,
credential rotation, and destination policies, or the
[API reference](/reference/api/) for resolver types.
