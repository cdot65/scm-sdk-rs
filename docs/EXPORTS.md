# Frozen exported capability reconciliation

The initial parity baseline is `@cdot65/prisma-airs-sdk` 0.34.0 at
`4a80dbdb3dc14365e1c25d838ac0bd139485d48c`. Its immutable inventory contains
2,630 root exports and 1,190 public members: **3,820 capabilities**. The separate
native ledger is `contracts/native-exports.json`; the frozen classification is
preserved as source evidence.

Every native row records its exact frozen identity/declarations, concrete public
Rust paths, actual Cargo feature requirements, source and test bindings, explicit
language adaptations and an external public compile probe. Missing or deferred
adapters remain errors. Source-proposed features are retained separately from
actual native features: shared validation is available in the core, model contracts
through `models`, and product clients through their named features.

## Three kinds of proof

| Evidence | What it establishes |
| --- | --- |
| Named-schema Zod oracle | All exported validators, normalization, defaults, refinements and typed non-JSON boundaries |
| Native model and operation contracts | Typed wire round trips, unknown values, exact endpoints/headers/bodies/queries and lifecycle behavior |
| Public export probes | Every mapped public type, generic instantiation, field, constructor, accessor, function and constant is reachable outside library privacy |

The generated integration test contains one explicit probe per frozen capability.
`contracts/native-probes` is a separate Cargo package with the same probes. It uses
only public SDK APIs, enables the implemented features, and preserves the reviewed
root dependency versions/checksums. Its execution performs local validation and
literal assertions; lazy constructor/function probes never dispatch service calls.
The wrapper removes PANW/AIRS/SCM environment variables and records the actual
compiler host. This evidence establishes public API availability and literal
contracts; live service acceptance remains a separate denominator.

Manual adapter mappings reside in `contracts/native-capability-mappings`. Concrete
operation mappings and model generation supply the remaining rows. The generator
rejects unknown/duplicate IDs, missing verification functions, unrelated probes,
stale source pins, missing public methods and incomplete adapters. The compiled
consumer detects private, incorrectly qualified or nonexistent public paths.

Run:

```text
python tools/native_exports.py --check
python -m unittest discover -s tools -p test_native_exports.py -v
python scripts/check-native-exports.py --toolchain 1.99.0
```

Regenerate with `python tools/native_exports.py` after reviewed source or adapter
changes. This emits the ledger, integration probes, standalone consumer and its
locked dependency graph together. No consumer requires the development TypeScript
baseline or Node tooling.

## Deliberate native adaptations

Rust builders, immutable client snapshots and explicit environment resolution
replace the JavaScript mutable singleton and constructor option bags. Presence
types preserve omission/null/value states. Generic native pagination reports bounds,
partial results and resumable positions. Caller-driven Stream/Sink polling and drop
replace Node iterator/listener/factory lifecycles; the native SDK has no implicit
reader workers or callback registry. Scope provisioning exposes structured recovery
and retains dependencies after uncertain writes.

Error taxonomy and sanitized problem diagnostics retain explicit access. Ordinary
formatting omits custom messages and upstream bodies. JavaScript inherited stack
hooks become explicit per-error backtrace capture and caller-owned formatting;
`std::error::Error` supplies the native cause interface without retaining arbitrary
transport errors. These changes are individual ledger entries, not blanket claims
that a generic client or error covers unrelated exported members.

The ergonomic provisioning request/result types are the operation's primary native
API. The `models` namespace also retains source-derived wire types for model-only
consumers; their generated structural aliases are not claimed as accepted workflow
inputs. SDK version/User-Agent constants intentionally identify the Rust crate;
frozen vendor vocabulary and protocol constants retain their exact values.

## Gateway source binding

`tools/verify_gateway_bindings.py --check` supplements executed contract suites with
194 exact Phase 6 source bindings. Each row records frozen method/helper hashes,
HTTP method, complete route topology, schema roots and native helper proofs. The
one explicit telemetry decomposition checks identical shared filter node IDs and
bounds refinement against the frozen composed schemas. A pinned reviewed runtime
path helper preserves ordered segments, routing and replay application.

Thirteen adversarial guards reject wrong schema roots, methods, route literals or
segments, changed metrics/refinements, comment-only evidence and unrelated-method
evidence. This is a bounded structural check of reviewed source patterns. Exact wire
fixtures and behavioral tests establish the transport contracts; token matching
does not prove general equivalence between JavaScript and Rust programs.
