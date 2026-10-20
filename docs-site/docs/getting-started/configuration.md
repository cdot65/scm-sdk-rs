# Configuration

Each SDK client has explicit credentials, endpoints, and resource limits.
There is no mutable global configuration. Your application owns its runtime,
credential storage, and enforcement decisions.

## Configure runtime scanning

`configuration::init` validates an isolated scan configuration. At least one API
key or bearer token is required. This example reads the key in application code;
`init` itself does not access the environment.

```rust
use scm::{
    Endpoint, Secret,
    airs::scan::Scanner,
    configuration::{InitOptions, init},
};

fn scanner() -> Result<Scanner, Box<dyn std::error::Error>> {
    let mut options = InitOptions::default();
    options.api_key = Some(Secret::new(std::env::var("AIRS_API_KEY")?)?);
    options.api_endpoint = Some(Endpoint::parse(
        "https://service.api.aisecurity.paloaltonetworks.com",
    )?);
    options.num_retries = Some(2);

    let configuration = init(options)?;
    let client = configuration.client_builder()?.build()?;
    Ok(Scanner::new(client)?)
}
```

`InitOptions` also accepts `api_token`. The endpoint defaults to the US scan
service shown above. Scan initialization accepts zero through five additional
retry attempts and defaults to five; operation replay rules still apply.
In particular, scan submissions are not automatically retried.

A client snapshots validated configuration when built. Resetting or
reinitializing the configuration does not change existing clients.

## Choose authentication by service

| Service | Authentication |
| --- | --- |
| AIRS runtime scanning | Scan API key and/or bearer token. |
| OAuth-backed AIRS management, Red Teaming, and Model Security clients | Explicit SCM OAuth service credentials and tenant context. |
| AI Gateway runtime | Runtime key for the selected Gateway endpoint. |

Product constructors validate their expected credential kind. Consult the
specific product's API reference for endpoint selection and tenant-header
behavior; those details vary across services.

For an OAuth-backed client, construct an immutable `AuthContext` and supply the
secret separately:

```rust
use scm::{Client, Endpoint, Secret, auth::AuthContext};

fn oauth_client() -> Result<Client, Box<dyn std::error::Error>> {
    let issuer = Endpoint::parse(
        "https://auth.apps.paloaltonetworks.com/oauth2/access_token",
    )?;
    let context = AuthContext::new(
        issuer,
        std::env::var("SCM_CLIENT_ID")?,
        1,
        std::env::var("SCM_TSG_ID")?,
    )?;
    let secret = Secret::new(std::env::var("SCM_CLIENT_SECRET")?)?;
    let client = Client::builder(Endpoint::parse(
        "https://api.sase.paloaltonetworks.com/aisec",
    )?)
    .oauth_credentials(context, secret, false)
    .build()?;
    Ok(client)
}
```

The third argument to `AuthContext::new` is the credential generation.
Increment it when rotating credentials so identities from different generations
remain isolated; the example uses generation `1`.

This builds the shared HTTP client; it makes no API request. The example chooses
the AIRS management base and disables the optional `x-tsg-id` header. Use the
endpoint and header policy required by the product client you attach.

OAuth refreshes are shared within a provider instance. The default early refresh
buffer is the smaller of 30 seconds and 20% of token lifetime. Credential rotation
uses a new context/provider/client, rather than modifying a global token.

## Endpoints and transport

Normal endpoints require HTTPS. Destination policy admits the configured API
and issuer origins; a custom policy must admit both. Multi-plane clients may
require additional explicit destinations.

The default reqwest transport verifies TLS, rejects authenticated redirects, and
uses a shared connection pool. Explicit proxy and additional PEM trust roots are
available through transport configuration. A caller-supplied transport must
preserve request bytes, verify TLS, enforce bounds, and release resources when
cancelled.

## Timeouts and cancellation

The shared client provides request and response byte limits, concurrency limits,
and operation deadlines. `RequestOptions::with_timeout` sets a per-operation
deadline. A session cancellation affects all clients sharing that session.

Cancellation stops local work; it cannot prove that a server did not commit a
write. Inspect `Error::remote_outcome` before retrying an uncertain mutation.

## Next steps

- [Environment variables and precedence](/getting-started/environment-variables)
- [Scanning workflow](/airs/scanning)
- [Product availability](/products/overview)
- [Client, transport, and option types](/reference/api/)
