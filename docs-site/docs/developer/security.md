---
title: Security
---

`scm-sdk` is community maintained by `cdot65`. Project-owned Rust forbids unsafe
code; third-party dependencies and native cryptography have their own boundaries.
The package does not claim FIPS validation. [DEPENDENCIES.md](https://github.com/cdot65/scm-sdk-rs/blob/6f18529a47cc273feca1000af6e4fc0d96da757e/docs/DEPENDENCIES.md)
records exact choices, advisory/license/source checks and remaining platform scope.

## Reporting a vulnerability

Contact `cdot65` privately through an established maintainer contact channel.
If no private channel is available, open a minimal repository issue requesting a
private reporting route. Omit exploit details, credentials, tenant registrations,
source code, signed URLs and raw service payloads from that public request.
No specific response-time commitment or external reporting endpoint is assumed.

A useful private report identifies the affected Rust version/commit, enabled
features and platform; explains the security impact; and provides a minimal
synthetic reproducer with sensitive data removed. The maintainer coordinates
triage, remediation and disclosure. Do not probe another tenant or reuse actual
customer resources to reproduce a report.

## Runtime boundaries

- Credentials and tenant files load only through explicit application inputs.
  Environment snapshots read only selected names. Secret formatting is redacted;
  owned secret buffers are zeroized, without claiming to erase external copies.
- OAuth identity and token caches are isolated. Refreshes use injected transport,
  destination policy and session bounds. A delayed unauthorized response cannot
  evict a newer token generation; dropped waiters do not create detached workers.
- Destinations, path segments, headers, request bodies and typed schemas validate
  before authentication. Verified TLS and redirect rejection protect credential
  routing. Returned URLs and secret references remain data until explicitly used.
- Response modes deliberately tolerate unknown enums and skip the `uuid`,
  `rfc3339`, `datetime`, `date`, `duration`, `email`, `url` and `regex` string-format
  assertions, plus vendor-only string-length assertions. String types, baseline
  presence/type/bounds, finite JSON and resource limits remain enforced. Request
  checks remain strict. Frozen refinements and transforms remain active, including
  AgentGuard timestamp parsing in `time_order`;
  [G25 compatibility](/core/response-compatibility) lists the retained response constraints.
- HTTP, decompression, JSON work, pagination, multipart, SSE and realtime have
  resource bounds. Explicit cancellation stops local work; uncertain remote
  outcomes remain visible, and mutations are not silently replayed.
- Ordinary diagnostics omit bodies, secrets and external error text. Optional
  SDK tracing uses a closed metadata vocabulary and no subscriber installation.
  Explicit getters and serialization may still expose API data to the caller.

Custom transports, token providers, TLS configurations and WebSocket connectors
are trusted application code and own their declared networking policy. Realtime
uses a separate connector; HTTP proxy/root settings are not implicitly inherited.
The application owns its runtime and security enforcement decisions. See
[Transport and Runtime](/core/transport), [OAuth Lifecycle](/shared/oauth), [Realtime](/airs/realtime)
and [Observability](/core/observability).

## Dependency and release policy

Applicable advisories block releases. The enforced default permits no advisory
ignores, license exceptions, unapproved registries or Git dependencies. Only
`cdot65` may approve a documented, scoped, expiring exception after reviewing
exposure and mitigation; no such exception is currently authorized. The policy
and checker must reflect any future approval explicitly.

A fresh audit records its RustSec revision, pinned tool versions and unchanged
lockfile. A clean result describes known findings at that revision. Downstream
applications resolve and audit their own dependency graphs. The published
[0.1.0 line on crates.io](https://crates.io/crates/scm-sdk/0.1.0) is the initial
supported release; the main branch contains ongoing development. Future supported
release lines and migration requirements will be stated in release notes. The
[historical 0.1.0 publication receipt](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/docs/checks/release-registry-0.1.0.json) binds the published
archive and source commit.

[RELEASE.md](https://github.com/cdot65/scm-sdk-rs/blob/6f18529a47cc273feca1000af6e4fc0d96da757e/docs/RELEASE.md) requires source review, exact artifact/commit proof and
explicit publication control. Service qualification is recorded separately in
[Live Qualification](/developer/live-qualification); permissions or missing prerequisites
must not be hidden as SDK successes. Remote CI execution and additional native
platform runtime qualification require retained evidence.

## 0.2.4 TLS dependency patches

The 0.2.4 root and native-probe lockfiles resolve `rustls-webpki` to **0.103.16** and
`webpki-root-certs` to **1.0.10** for RUSTSEC-2026-0382 and RUSTSEC-2026-0383.
These are transitive TLS dependency patches; direct SDK dependency declarations
remain unchanged. Version 0.2.4 is [published on crates.io](https://crates.io/crates/scm-sdk/0.2.4).

Applications retain their own `Cargo.lock`. Installing or updating `scm-sdk`
does not replace that lockfile or establish that these patches are present in
its resolved graph. If either package is in your application's dependency graph,
update it to the patched version or a compatible later version, check the
resulting lockfile, and rerun the application's dependency audit and tests.
The packages selected can depend on enabled features and target platform.

## Private material exclusion evidence

The [retained phase-08 scan receipt](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/docs/checks/phase-08-private-material.json) records
an exact-match check of one explicitly selected external registration. To
reproduce it, create `.local/checks`, select the external nonproduction file
explicitly and choose a new receipt filename:

```sh
python3 scripts/check-private-material.py \
  --config /absolute/path/to/nonproduction-registration.json \
  --output .local/checks/private-material-reproduction.json
```

The registration must contain exactly `mgmtClientId`, `mgmtClientSecret` and
`mgmtTsgId`, without duplicate fields. The file is limited to 64 KiB; string
values are limited to 4096 UTF-8 bytes, with at least 8 bytes for the client
identity and 16 for the secret. The tenant identifier is 1–64 ASCII digits.
The scanner rejects empty, control-containing, duplicate or nonrepresentative
tiny credential values. It requires an absolute path outside the repository,
rejects symlink components and never discovers registrations or selects them
from environment variables.

The scanner checks every Git-visible file and name, including check/review
artifacts, for the selected client identity and secret. It also checks all
regular files and names under the published crate's `src` and `examples`
directories for all three values, including ignored candidates: this is a
conservative superset of the reviewed package include policy. Package archive
membership is independently checked by `scripts/check-package.py`. Files,
total bytes, traversal and Git subprocesses are bounded. Selected values,
their hashes, config contents and the config path are absent from diagnostics
and the receipt. Outputs are new private files; no input is overwritten.

Only a tenant identifier already present at the recorded HEAD, with no increase
in occurrences and outside packaged source/examples, can be classified as a
historical nonsecret identifier. New tenant copies and every selected identity
or secret match fail the check. Historical presence does not establish that a
tenant is public or authorized.

Credential-free checks can verify the retained receipt's exact sanitized
schema and source freshness without opening a registration:

```sh
python3 scripts/check-private-material.py \
  --check docs/checks/phase-08-private-material.json
python3 -m unittest tools/test_auxiliary_evidence.py
```

`--check` validates the recorded successful scope, counts and current source
and package-candidate fingerprints. The recorded `headCommit` identifies the HEAD
used by the private scan for historical-identifier classification; `--check`
deliberately does not require it to equal the current HEAD. Receipt-only commits
may advance HEAD while leaving the checked source unchanged. Release qualification
refreshes the private scan at the clean candidate commit, and final bookkeeping
receives another private scan before delivery completion.

The credential-free check does not repeat secret matching or certify later changes
to review/check/status bookkeeping excluded from that fingerprint; run the
explicit private scan again after final evidence changes when qualifying the
complete tree. Exact-value scanning does not cover encoded, split or unrelated
secrets. Python retains selected values in process memory and cannot guarantee
zeroization. The synthetic tests use temporary repositories and registrations;
they exercise leaks, malformed inputs, bounds, paths, symlinks, safe receipts
and stale evidence without reading real credentials.
