# Adversarial parser qualification

The SSE qualification target uses the public Client, Transport, ResponseStream,
and SseStream interfaces. Its transport is an in-memory fixture with synthetic
credentials, tracked body polls and drops, and one concurrency permit. It performs
no network requests. Literal expected events establish the basic framing contract;
mutation comparisons establish invariance across chunk boundaries.

## Executed deterministic corpus

[parser_qualification.rs](../crates/scm-sdk/tests/parser_qualification.rs) contains
nine tests:

| Coverage | Exact corpus or bound |
| --- | --- |
| Exhaustive short-frame partitions | Every partition into nonempty byte chunks for data:x followed by two LF delimiters and UTF-8 data:λ followed by two CR delimiters: 128 + 256 = 384 partitions. Expected event values are handwritten literals. |
| Complex two-chunk splits | Every split, including empty first/last chunks, of one BOM/comment/CRLF/Unicode/multiline/ID/retry/empty-data/DONE fixture. Expected events are handwritten literals. |
| Deterministic mutations | Six handwritten base inputs; xorshift32 seeds 0x00000001, 0x13579bdf, 0x5eedcafe, 0xf00dbaad; 512 cases per seed, 2,048 mutations total. Each runs contiguously and with 1–13-byte chunks. |
| Mutation operators | Insert one byte, delete one byte, replace one byte, truncate, or append a suffix. Actual input is bounded to 128 bytes; frame limits vary from 1 through 128; all three terminal policies are exercised. |
| Malformed UTF-8 and truncation | Nine inputs, each at every two-chunk split: invalid byte, overlong encoding, surrogate, scalar above U+10FFFF, incomplete scalar, invalid comment, and three incomplete data-frame forms. |
| Size boundaries | Every split of a Unicode CRLF frame at the exact normalized frame limit and one byte below it; oversized comments/ignored fields; separate total wire-byte limit. |
| Completion | All three Responses lifecycle terminals release immediately even with a pending HTTP body; event-name spoofing, DONE under Responses, and missing lifecycle terminal fail. |
| Numeric error envelopes | Positive/negative overflow fail closed with and without downstream arbitrary-precision unification. |
| Ownership and interruption | Cancellation, session cancellation, absolute deadlines, explicit close, and drop before consumption and with buffered data; 129 empty chunks, idle timeout, and transport failure. |

Each parser run permits at most 128 event iterations, checks consumed body polls
against supplied chunks, and checks immediate body release. Except for an already
cancelled Session, a follow-up request with a 100 ms budget proves the sole
concurrency permit is reusable. Errors latch, retain unknown remote outcome and
HTTP status, omit the synthetic key, and never cause an automatic second dispatch.
Paused Tokio time makes deadline and idle cases deterministic.

The unit target streaming::sse::tests additionally checks media-type admission,
duplicate headers, retry advice, redacted formatting, and the numeric truthiness
matrix. The matrix includes signed zero, decimal zero, zero with an overflowing
exponent, positive/negative underflow, the smallest subnormal, ordinary signed
numbers, finite large numbers, and positive/negative overflow.

## Numeric overflow correction

The Phase 8 regression failed before the correction: with
serde_json/arbitrary_precision, {"error":1e400} was retained as a JSON number,
but Number::as_f64() returned None. The previous check treated that as false and
accepted the upstream error envelope.

The decoder now checks a borrowed raw error token after validating the JSON.
Numeric truthiness uses JavaScript's IEEE-754 behavior: underflow and signed zero
remain false; overflow is true. Strings are decoded before checking emptiness,
objects and arrays are always truthy, and duplicate error fields use the last key.
Without arbitrary precision, the JSON parser itself rejects overflow.
Responses and typed Gateway streams latch the resulting
sanitized failure and release their body and permit. A raw SseEvent JSON decode
returns an error to its caller; raw callers still own stream consumption and close.

An additional regression demonstrated that a literal error object containing
`$serde_json::private::Number` with string value `0` could be reinterpreted as the
number zero by serde_json under arbitrary precision. The raw-token check treats
the original object as truthy, including escaped error keys. The subsequent
shared boundary restriction below rejects reserved protocol objects before
generic decoding. Borrowed `RawValue` avoids copying the payload; unrelated
fields are skipped. Normal objects retain JavaScript truthiness, and ordinary
false/null/empty-string and duplicate-key handling remain unchanged.

## Wide numbers in typed models

Gateway feature qualification exposed another Serde interaction: its intermediate
buffers for untagged enums and flattened fields reject `i128`/`u128` visits from
an arbitrary-precision JSON value. The regression reproduced a failed typed
prompt request and a failed actual HTTP response decode with status 200, using
the unchanged independent frozen fixtures. A new public-client regression covers
numbers outside signed and unsigned 64-bit ranges, nested unknown extensions,
and the public model parsing boundary.

The private JSON value adapter preserves ordinary primitives and moves ordinary
strings/containers. Numeric tokens that can trigger 128-bit visits use the public
JSON reader with a bounded numeric-token writer; zero also uses this path to
preserve signed zero. Other numbers use Serde's ordinary conversion and retain
floating-point bits. It introduces no whole-payload
serialization and never rounds opaque numeric data through `f64`. Presence
wrappers, intersections, model parsing, HTTP decoding, typed SSE and realtime
decoding use the same boundary. The adapter relies on public Serde interfaces;
the upstream interactions are described in [serde_json issue 505](https://github.com/serde-rs/json/issues/505)
and [issue 721](https://github.com/serde-rs/json/issues/721).

## Reserved protocol object rejection

The independent audit then reproduced a related default-feature coercion:
with `serde_json/raw_value`, a literal object containing
`$serde_json::private::RawValue` with string value `false` became boolean false.
A raw-wire correction alone cannot prevent a subsequent generic Serde value
visitor from interpreting these protocol names again.

The SDK therefore rejects both reserved protocol keys in literal objects before
wire parsing and at its owned-value conversion boundary. The guard checks
nested objects and escaped key spellings, accounts for duplicate fields, and
keeps parsing bounded. The allocation-free wire preflight caps bytes at 512 MiB,
container depth at 128, JSON values at 1,000,000, and scan work at four times the
input length; normal operation/validation limits remain tighter. Caller-created
values use iterator frames and iterative disposal on rejection. The guard
returns safe decoding/validation failures and retains
post-dispatch status and uncertainty. Ordinary unknown fields and numeric tokens
remain supported. The exact compatibility restriction and direct downstream
Serde boundary are documented in [MODELS.md](MODELS.md#reserved-json-protocol-keys).

## Reproduce

Run from the repository with the exact installed compiler:

    cargo +1.99.0 test --locked --target x86_64-unknown-linux-musl \
      --no-default-features --test parser_qualification
    cargo +1.99.0 test --locked --target x86_64-unknown-linux-musl \
      --no-default-features --features serde_json/arbitrary_precision \
      --lib streaming::sse::tests
    cargo +1.99.0 test --locked --target x86_64-unknown-linux-musl \
      --no-default-features --features serde_json/arbitrary_precision \
      --test parser_qualification

The initial Phase 8 execution passed all nine integration tests and all twenty SSE
unit tests on Rust 1.99.0, x86_64-unknown-linux-musl. The final phase check report
binds repeated executions to the reviewed source.

This is bounded deterministic property and mutation testing. It is not a
coverage-guided fuzzer, prolonged fuzzing campaign, independent SSE implementation,
or proof against all malformed inputs. It adds no Cargo dependencies. The broader
contract suites separately cover authentication isolation, destination policy,
signing, paging, unknown outcomes, multipart input, realtime, and provisioning.
