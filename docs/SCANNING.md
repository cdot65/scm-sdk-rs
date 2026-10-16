# Typed AIRS scanning

`airs::scan::Scanner` implements the four frozen scan operations: synchronous
scanning, asynchronous batch submission, result queries by scan ID, and report
queries by report ID. Construct it from a shared `Client` with scan authentication,
or choose an explicit `Region` with `Scanner::for_region`. It never reads an
environment variable, tenant registration, or credential file itself.

`Content::builder` accepts prompt, response, code, context, and tool-event data.
At least one primary field or tool event is required; context alone is insufficient.
Text fields are bounded at 2 MiB each and context at 100 MiB, measured as UTF-8
bytes. The client's request byte bound applies independently. Setters validate a
candidate and preserve the existing content if the change fails.

Content accepts typed JSON, a byte slice, a bounded caller-owned synchronous
reader, or an explicitly selected async file path. File errors omit paths and
operating-system text. The baseline Content helper's undeclared fields are dropped.
Direct typed scan requests and async batches also enforce these content invariants;
this is deliberately stricter than the TypeScript raw async path. Standalone schema
normalization continues to match the frozen Zod contract.

Async submissions contain 1–20 objects. ID queries accept 1–5 values; scan IDs must
be UUIDs. Report identifiers may use opaque values but cannot inject comma-separated
entries or controls. Query encoding preserves supplied order and duplicates.
Result rows remain in server order and retain one-to-many relationships. A receipt
does not imply that a scan completed or produced exactly one row.

Typed request validation and normalization finish before authentication or dispatch.
HMAC signs the final immutable JSON bytes. Every scan request also includes
`service-name: api` and `User-Agent: PAN-AIRS/<crate-version>-rust-sdk`, even with
a caller-supplied transport. The native User-Agent identifies this crate rather
than the frozen TypeScript package; its vendor prefix is retained. Complete
header-name sets are asserted, including absence of body headers on GETs.
Submissions never automatically replay,
including after an authentication failure or an ambiguous transport failure.
Use remote-outcome metadata and application reconciliation when a write may have
reached the server.

Unknown response fields and enum vocabulary survive decoding. Scan action/category
are strings in the frozen source and retain unfamiliar values. Applications must
make an explicit enforcement decision; this SDK never maps an unfamiliar outcome
to an allow decision. Ordinary model and content formatting redacts payload data.

The packaged [scan example](../crates/scm-sdk/examples/scan.rs) compiles with the
scan feature. The independent scanner integration suite exercises all four frozen wire fixtures,
literal HMAC computation, invalid-input rejection, uncertain outcomes, and safe
formatting. Live scan qualification belongs to Phase 8 and is recorded separately.
