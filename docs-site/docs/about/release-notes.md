---
title: Release notes
sidebar_position: 1
slug: /about/release-notes
---

# Release notes

These notes summarize consumer-facing changes in published `scm-sdk` releases
and explicitly marked forthcoming additions.
The [crate registry](https://crates.io/crates/scm-sdk) provides published versions.

## Forthcoming 0.2.4 — Directional security profiles

This release is not yet published. Installation instructions continue to use 0.2.3.

- Adds `ContentTypeConfiguration` and `ContentTypeConfigurations`, their builders,
  and two named validators for independently optional prompt, response, tool-call,
  and tool-response protections.
- Adds optional content mode and directional configurations to `AiSecurityProfile`,
  full-conversation inspection to `ModelConfiguration`, and `dlp_tenant_id` to
  both profile and create/update request models.
- Validates profile create/update against the current TypeScript 0.35.0 request
  schema before authentication or I/O. Newly known fields with null or wrong types
  now fail request validation, including values supplied through extensions.
- Retains tolerant Rust profile responses, preserving invalid newly known values
  and explicit null in extensions. Upstream TypeScript 0.35.0 responses reject
  those values. Explicit typed values take precedence over same-key extensions.
- Refreshes the repository lockfile to `rustls-webpki` 0.103.16 and
  `webpki-root-certs` 1.0.10 for RUSTSEC-2026-0382 and RUSTSEC-2026-0383.
  Applications retain their own lockfiles: update these transitive packages where
  present and audit the resolved graph. Installing the SDK does not replace an
  application lockfile. See the [TLS dependency note](/developer/security#forthcoming-024-tls-dependency-patches).

Existing standalone schema names retain historical validation; use versioned
`typescript:0.35.0:<SchemaName>` roots for current standalone validation. Shared
protections, list defaults, endpoint methods, Rust 1.89, features, and direct
dependency declarations remain. Directional live acceptance remains unverified. See the
[directional profile guide](/airs/directional-profiles) for fields and migration.

## 0.2.3 — AIRS management compatibility

[Published crate](https://crates.io/crates/scm-sdk/0.2.3)

- Aligns Red Team management contracts with version 0.86.0 while retaining existing
  routes and SDK-only capabilities.
- Adds schema-bound connection names `VERTEX_AI` and `N8N`, response mode
  `REALTIME`, Vertex AI connection models and client-session multi-turn config.
- Adds optional basic-auth and audio projections. Representable values move from
  extension storage into typed fields; serialization retains omission and null.
- Adds `TargetListOptions::profiling_status`, including across pagination.

**Migration:** move `FAILED`, `IN_PROGRESS` and `COMPLETED` profiling filters to
`profiling_status`. Legacy `status` values are still sent unchanged and may be
rejected by the service. `QUEUED` and `PARTIALLY_COMPLETE` have no declared
replacement. Some previously unknown union shapes now decode into new named
branches; frozen compatibility enums still retain unknown values.

Rust 1.89, default features, endpoint defaults, G10 behavior and G25 tolerance
remain unchanged. Tenant filter probes returned HTTP 403; provider workflows
without credentials or compatible fixtures remain experimental/live-unverified.
Publication does not imply successful live acceptance of every feature.

## 0.2.2 — G25 response compatibility

[Published crate](https://crates.io/crates/scm-sdk/0.2.2)

Both response modes stop enforcing string formats such as UUID and RFC 3339.
Existing Red Team operations retain the compatibility baseline's response
presence, nullability, type and string-bound rules. Unknown response enum strings
and object keys were already accepted and remain accepted.

Request validation is unchanged. Target error-log requests still accept only
`limit`; `skip` and `search` are rejected. Missing `category_display_name`, numeric
`tsg_id` and null `sub_category` remain invalid where the baseline schema rejects
them. This is a compatibility correction, not an unrestricted response parser.

## 0.2.1 — Gateway access policies

[Published crate](https://crates.io/crates/scm-sdk/0.2.1)

Adds typed create/list/get/update/delete access-policy calls for the separate
key-authenticated control plane. Omission/null, optimistic concurrency and
non-replayable writes are preserved. Existing interfaces remain unchanged;
these new policy calls are experimental and live-unverified.

## 0.2.0 — Red Team data-plane coverage

[Published crate](https://crates.io/crates/scm-sdk/0.2.0)

Adds native methods and wire fixtures for all 47 operations in the pinned Red
Team data-plane OpenAPI 0.93.0, including eleven new operations and 152 vendor
component models. Existing routes and SDK-only capabilities remain available.
The release also introduces stricter vendor query-option validation; consult the
[complete migration notes](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/CHANGELOG.md#changed--migration-from-01x).
Response format compatibility was subsequently corrected in 0.2.2.

## 0.1.1 — G10 response defaults

[Published crate](https://crates.io/crates/scm-sdk/0.1.1)

Adds `RequestOptions::without_response_defaults()`, `response_defaults()` and
`Mode::ResponseWithoutDefaults`. Consumers can retain missing server fields
without inserting defaults such as `ws_response_timeout: 110`, while explicit
null survives. Default behavior and the set of accepted documents are unchanged.

See [Core client and policies](../reference/api/core.md) for the current response
policy and [Red Teaming](../reference/api/red-teaming.md) for target behavior.

## 0.1.0 — Initial native Rust release

[Published crate](https://crates.io/crates/scm-sdk/0.1.0)

Implements the frozen TypeScript SDK 0.34.0 surface through 448 concrete methods
and 3,820 mapped capabilities: scanning, management, shared DLP/IAM, Model
Security, Red Team, Gateway, experimental AgentGuard/realtime, and provisioning.
Includes native typed models, omission/null/value handling, bounded transports,
explicit credentials, and application-owned async runtimes. Implementation
coverage is separate from tenant and provider live qualification.
