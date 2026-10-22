# Installation

`scm-sdk` is a native async Rust SDK for Strata Cloud Manager, starting with
Prisma AIRS and shared DLP and IAM services. Use Cargo to install it; your
application does not need Node.js or the TypeScript SDK.

## Requirements

- Rust **1.89 or newer**.
- An application-owned Tokio runtime for async API calls.
- Credentials and access to the service you intend to call.

The SDK creates no runtime or background credential-loading process. Product
constructors do not read environment variables or tenant registration files.

## Add the dependency

Use the `scm` dependency alias so imports begin with `scm::`:

```toml
[dependencies]
scm = { package = "scm-sdk", version = "=0.2.4" }
tokio = { version = "1", features = ["macros", "rt"] }
```

The exact version pin makes upgrades explicit. The package name remains
`scm-sdk`; without the alias, its Rust import name is `scm_sdk`.

Default features enable AIRS scanning and the reqwest HTTP transport. This is
enough for the [quick start](/getting-started/quick-start).

## Select product features

Add the features your application uses:

```toml
[dependencies]
scm = { package = "scm-sdk", version = "=0.2.4", features = ["management", "red-team", "model-security"] }
tokio = { version = "1", features = ["macros", "rt"] }
```

Cargo features are additive. That configuration retains default scanning and
HTTP transport alongside the three selected product clients.

| Feature | Enables |
| --- | --- |
| `scan` | AIRS runtime scanning; enabled by default. |
| `reqwest-transport` | Default HTTP adapter with TLS verification; enabled by default. |
| `management` | AIRS management clients; also enables `dlp`. |
| `red-team` | AIRS Red Teaming clients. |
| `model-security` | AIRS Model Security clients. |
| `gateway` | AI Gateway clients; also enables `iam`. |
| `dlp` | Shared DLP clients. |
| `iam` | Shared IAM clients. |
| `models` | Model contracts without enabling every product client. |
| `agentguard` | Experimental AgentGuard integration. |
| `realtime` | Experimental realtime interfaces with a caller-supplied connector; also enables `gateway`. |
| `ws-transport` | Native realtime WebSocket connector; also enables `realtime`. |
| `tracing` | Optional metadata-only tracing; your application installs the subscriber. |

For a Red Teaming application that does not need scanning:

```toml
scm = { package = "scm-sdk", version = "=0.2.4", default-features = false, features = ["reqwest-transport", "red-team"] }
```

Disabling `reqwest-transport` means supplying your own HTTP transport. HTTP and
WebSocket connectors have separate proxy and TLS configuration.

## Next steps

- [Make a first scan request](/getting-started/quick-start).
- [Choose credentials and endpoints](/getting-started/configuration).
- [Load selected environment variables](/getting-started/environment-variables).
- [Explore the product scope](/products/overview).
- [Browse the Rust API reference](/reference/api/).
