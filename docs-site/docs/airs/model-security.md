---
title: Model Security
---

Enable `model-security` and construct `airs::model_security::ModelSecurityClient` from an OAuth `Client` plus explicit data and management `Endpoint` values. Typical bases are `https://api.apps.paloaltonetworks.com/aims/data` and `https://api.apps.paloaltonetworks.com/aims/mgmt`. Both destinations must be admitted by the caller's policy. Changing planes preserves the OAuth provider, transport, session, limiter, and limits. Constructors reject other credential kinds locally.

The facade exposes `scans`, `models`, `security_groups`, `security_rules`, `custom_rules`, and `get_pypi_auth`. Standalone resource constructors accept an OAuth `Client` whose endpoint the caller selects explicitly. Model Security clears inherited optional `x-tsg-id` headers to preserve its frozen OAuth-only wire contract. All requests retain `service-name: api`, the Rust `PAN-AIRS/<crate-version>-rust-sdk` User-Agent, and bearer authentication.

## Implemented contracts

This feature implements all 49 methods in the frozen TypeScript 0.34.0 Model Security family: 41 HTTP operations and eight all-page helpers.

| Client | Methods | Plane and routes |
| --- | --- | --- |
| Scans | 14 | Data: scans, evaluations, files, labels and violations under `/v1/scans`, plus individual `/v1/evaluations` and `/v1/violations` |
| Models | 8 | Data: `/v1/models`, nested model versions, and `/v1/model-versions` with nested files |
| Security groups | 10 | Management: `/v1/security-groups`, rule instances, and group-wide rule-instance snapshots |
| Template rules | 4 | Management: `/v1/security-rules` and its global version history |
| Custom rules | 12 | Management: `/v1/custom-rules`, archive state, group assignments and global version history |
| PyPI authentication | 1 | Management: `/v1/pypi/authenticate` |

UUID path arguments are validated before requesting tokens, and path segments are encoded independently. Label keys are opaque strings, not UUIDs. Query construction has a 16 KiB allocation budget and a 256-entry cap before authentication; final URL checks also count the endpoint and percent encoding. Repeated array keys preserve caller order and duplicates. Empty arrays omit their query keys, while explicit `false`, zero offsets, and empty strings remain present.

Direct lists omit unspecified pagination and filters. Their public product option types retain the frozen fields, with Rust snake_case names; `is_background_refresh` sends `isBackgroundRefresh`, and file `type_` sends `type`. A positive `u32` page limit and nonnegative JavaScript-safe `u64` offset/generation replace unvalidated numeric runtime casts. Custom-rule lists and snapshot histories additionally enforce their source limit of 100; custom-rule sort fields and directions remain closed source choices. These checks reject invalid values before authentication.

## Paging and saved positions

Every frozen all-page helper returns a bounded `CollectionResult` with explicit completion, partial data, and resume position. Configure its `CollectOptions`; defaults are 10,000 items, 64 MiB of serialized collection data, and 1,000 pages. Offset helpers start at zero with page size 50 unless supplied. Their `pages`, `model_pages`, `version_pages`, or `file_pages` alternatives expose caller-driven walks and allow an explicit initial position. All requests in a walk share one deadline and cancellation snapshot.

Ordinary offset walks use the reported total when present. A missing/null total instead continues after full pages and stops after short or empty pages. An invalid present total fails safely, including on an empty page; fractions, negative totals, and unsafe integers cannot become offsets. Walks advance by actual returned item count. A collection is not a server snapshot: a resumed page can change while the caller is paused.

Scan `count_capped: true` is evidence that the reported total is incomplete. Once observed, a scan walk ignores reported totals for the remainder of that walk and stops only at a short or empty page. A full final page therefore requires an additional request. This deliberately avoids the baseline helper's possible false completion at a capped total. Raw `list` responses retain the original `count_capped` field.

`ScanCursor` contains the offset and a monotonic count-capped flag. Clones share only that conservative evidence; equality and hashing depend solely on the immutable offset. This lets a mid-page resume retain the flag discovered while its page was fetched. Serialize the cursor, or reconstruct it with `ScanCursor::with_count_capped(skip, flag)`, to save a resume outside the process. Its deserializer validates the offset and does not rely on hidden runtime state. Reconstructing a known capped position with `ScanCursor::new` would discard that evidence.

Custom-rule snapshot history uses opaque `Option<String>` cursors. An empty string is a valid next token and is sent explicitly; only missing/null `next_token` ends the walk. Shared `CursorLimits` bound retained tokens. If a returned token exceeds a configured bound, already accepted rows remain available in the partial collection with a replayable position. `list_all_versions` accepts `SnapshotVersionListAllOptions`; direct `list_versions` uses `SnapshotVersionListOptions`. Group/template version methods retain their distinct group-wide/global routes.

## Requests, responses and remote outcomes

All nine JSON body methods validate against the frozen native schemas before authentication. Group creation inserts the source's missing-description default. Group and custom-rule updates use `PUT` with `application/json`, preserving `Patch::Unset`, `Patch::Null`, and `Patch::Value`; they are not merge-patch requests. Custom-rule bodies remain strict, including nested condition groups, and batch assignments retain their 1–50 group bound.

Successful batch assignment can return HTTP 207 and partial per-group failures. The SDK preserves that result for the caller to inspect; it does not claim every assignment succeeded. Labels use the source's empty object response contract. Void mutations accept any successful response body without pretending it is a typed object. Required list/response structures remain required, including when an empty HTTP success hydrates to `{}`. Unknown response fields and vocabulary remain faithful data.

All 14 mutation methods have no automatic replay. Transport failure or decode failure after a request may retain `RemoteOutcome::Unknown`; cancellation cannot prove that the server did not commit a write. Reconcile remote state before retrying a mutation.

`get_pypi_auth` returns the declared URL and expiry without following the URL. That URL can contain credentials. Ordinary model/client/query/cursor formatting is redacted; applications should handle explicit access to credential-bearing fields accordingly.

## Evidence and limits

`crates/scm-sdk/tests/model_security_contracts.rs` independently authors literal endpoint, method, ordered query, header, JSON-body, and response fixtures from frozen Model Security source and schemas. Its 17 tests invoke every baseline method and cover every mutation's uncertain no-replay result, all query fields, endpoint/header isolation, pre-authentication rejection, capped-total and opaque-token resumes, collection bounds, shared deadlines, and cancellation with buffered rows.

`contracts/native-operation-mappings/model-security.json` binds each baseline method to its concrete native method and named tests. These are offline contract checks. Live acceptance remains unverified and is recorded separately. The frozen Model Security family has no CSV, multipart, or binary transport operations; those Phase 5 contracts belong to Red Team.
