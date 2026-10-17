# Platform qualification

The SDK's minimum Rust release is **1.89**. Current qualification also uses
**1.99.0**. Toolchain selection is explicit; scripts do not change the installed
default compiler.

| Target | Evidence boundary |
| --- | --- |
| `x86_64-unknown-linux-musl` | Native compiler/test/example/doctest and loopback HTTP/WebSocket execution in retained phase checks. |
| `x86_64-unknown-linux-gnu` | Native all-feature compiler/test/doctest execution on Rust 1.89.0 in the retained CI matrix receipt; portable core/product cross-compilation is also retained. |
| `aarch64-unknown-linux-gnu` | Compile-only minimal and all product/core features with caller transports. |
| `aarch64-apple-darwin` | Compile-only minimal and all product/core features with caller transports. |
| `x86_64-pc-windows-msvc` | Compile-only minimal and all product/core features with caller transports. |

The [GNU matrix receipt](checks/phase-08-ci-run-11/matrix-capture-receipt.json)
records six passing commands at source `72765d5`. Its host/compiler/feature scope
is independent of full workflow completion; current remote delivery outcomes are
recorded in [phase status](phase-status.json). This CI execution supplements the
native musl phase checks without implying other operating-system execution.

Cross-compilation does not link or run a target executable. It establishes Rust
source/type compatibility with the installed target standard library. Reqwest,
native WebSocket framing/TCP/TLS, certificate loading, proxies and native
cryptography are excluded from these cross-target checks. Their Apple,
Windows and ARM runtime behavior remains unqualified. Native GNU and musl
execution remains bounded by the feature/compiler and loopback cases in their
actual reports. Caller-provided
`Transport` and `WebSocketConnector` implementations own their platform behavior.

## Reproduce cross-target checks

Install standard libraries for the exact compiler, then run the checker:

```sh
rustup target add --toolchain 1.99.0 x86_64-unknown-linux-gnu aarch64-unknown-linux-gnu aarch64-apple-darwin x86_64-pc-windows-msvc
python3 scripts/check-cross.py --target aarch64-apple-darwin --output .local/checks/apple-compile.json
```

Omit `--target` to check all four; repeat it to select several. The checker runs
`cargo check --lib --locked --offline --no-default-features` for minimal and for
`models,scan,management,dlp,iam,gateway,agentguard,realtime,model-security,red-team,tracing`.
It verifies the selected dependency trees exclude reqwest, tokio-websockets,
rustls and AWS-LC. Reports preserve installed targets, actual compiler host,
feature sets, commands and outcomes. The repository's full native gate separately
qualifies defaults, transports, isolated features and their combinations.

Wasm and 32-bit native adapters are not qualified. The native WebSocket adapter
requires a 64-bit target; the caller-connector realtime API has a separate feature
boundary. No successful live provider session is implied by a platform build.
