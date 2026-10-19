# Compatibility and support status

The current Rust crate version is **0.2.2**, the [G25 response compatibility patch](G25_COMPATIBILITY.md).
The [registry receipt](checks/release-registry-0.2.2.json) establishes publication
only when present and verified. A missing receipt means publication is not yet verified.
The [access-policy family](ACCESS_POLICIES.md) remains experimental/live-unverified;
its Portkey key/control-plane contract is separate from SCM OAuth.

The [0.1.1 publication receipt](checks/release-registry-0.1.1.json) establishes the
available G10 patch and its immutable source/archive identity. The opt-in response
policy and default behavior are retained. The 0.2.0 minor increment
is required because twelve legacy query options now fail before credentials;
[CHANGELOG](../CHANGELOG.md) lists every migration binding.

The vendor dataplane has a separate 47-operation ledger and current contract
fixtures. It does not change the frozen TypeScript denominator or imply successful
live execution. New methods retain explicit live-unverified status.

The initial Rust version **0.1.0** was published on
[crates.io](https://crates.io/crates/scm-sdk/0.1.0). The
[publication receipt](checks/release-registry-0.1.0.json) records its exact source
and archive checksum. The frozen parity
baseline is TypeScript SDK **0.34.0** at
`4a80dbdb3dc14365e1c25d838ac0bd139485d48c`. All 448 concrete methods and 3,820
exported capabilities have implemented native mappings. [EXPORTS.md](EXPORTS.md)
explains the independent validation, behavior and public-consumer evidence.

## Separate claims

| Claim | Evidence |
| --- | --- |
| Implemented | Concrete native public API, complete options, source/test bindings |
| Offline contracts verified | Executed independent request/response, validation and lifecycle checks |
| Live accepted | Successful native workflow in the selected service deployment and tenant scope |
| Platform qualified | Actual compiler/runtime evidence for the documented target and feature set |
| Released | Accepted source, clean commit, verified archive and recorded publication |

A denied permission, unavailable provider, missing fixture or blocked service does
not become a passing live result. Experimental AgentGuard and realtime remain
explicitly labeled. Other operations retain their actual live classifications;
implementation parity alone does not advertise every service workflow as stable.
Consult [LIVE_ACCEPTANCE.md](LIVE_ACCEPTANCE.md), the
[operation matrix](../contracts/native-operations.json), and
[PLATFORMS.md](PLATFORMS.md).

## Version policy

The accepted design uses incremental public `0.x` releases. Patch releases remain
source compatible, including experimental interfaces. Breaking changes require a
new `0.y` minor version and migration notes. An experimental feature label does
not waive that policy. Additive Cargo features select code and dependencies;
they do not grant service permissions or weaken runtime security checks.

The declared minimum Rust version is 1.89 and applies to minimal builds too.
Compiler minimum, public API, feature/dependency and documented behavior changes
receive compatibility review and release notes. Supported feature combinations
are tested alongside the minimum and current pinned compiler. Platform execution
and downstream dependency resolution remain separate qualification boundaries.

Public extensible types use non-exhaustive representations where appropriate.
Responses preserve unknown fields and future vocabulary while validating known
required structure; literal discriminators stay strict. Unknown security outcomes
remain explicit. Applications own enforcement decisions and must not turn an
unfamiliar outcome into an implicit allow.

## Language adaptations

Native parity preserves capabilities through Rust interfaces: isolated client
configuration replaces a global singleton, native validation replaces exported
Zod objects, traits replace fetch/factory inputs, owned streams replace listeners,
and bounded collections retain partial results and resumable positions. The
[export ledger](../contracts/native-exports.json) records each adaptation.

Literal objects using either reserved Serde JSON protocol key are rejected at
SDK JSON boundaries to prevent object-to-scalar coercion under feature
unification. See [the exact restriction](MODELS.md#reserved-json-protocol-keys).
This security adaptation does not silently discard unknown fields.

Cancellation releases local resources and can leave remote writes or billed
inference uncertain. Mutations do not inherit permissive read retries. Explicit
secret/data access and serialization preserve API data; ordinary formatting and
SDK tracing omit sensitive contents. See [FOUNDATION.md](FOUNDATION.md),
[SECURITY.md](SECURITY.md) and [OBSERVABILITY.md](OBSERVABILITY.md).

`cdot65` owns compatibility decisions, security exceptions and releases. Future
SCM product modules enter scope with their own contracts and qualification;
this AIRS baseline does not claim coverage of every SCM product API.
