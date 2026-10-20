---
title: Data Loss Prevention
---

Enable the `dlp` feature for canonical DLP operations. `management` also enables DLP. Models remain available under `dlp::models` whenever `models` is enabled. The implementation matches the frozen TypeScript SDK's four `/v2/api/*` resource clients; live acceptance remains unverified until Phase 8.

Construct `DlpClient::new(client)` with an explicit DLP endpoint and SCM OAuth authentication. It rejects scan/runtime credentials. The client retains the caller's transport, destination admission, tenant identity, request bounds and session. Management convenience access uses the same implementation with an explicitly admitted DLP destination.

Tenant routing remains caller-selected: `Authentication::oauth(provider, true)` sends `x-tsg-id` from the immutable provider identity; `false` omits it. `DlpClient` preserves that selection. The frozen TypeScript DLP client sends no tenant header, so select `false` for that exact header contract. Both choices have native wire fixtures.

| Accessor | Operations |
| --- | --- |
| `data_patterns()` | `list`, `create`, `get`, `replace`, `patch`, `delete`, `list_all` |
| `dictionaries()` | `list`, `create`, `get`, `replace`, `patch`, `delete`, `list_all` |
| `data_profiles()` | `list`, `create`, `get`, `replace`, `patch`, `list_all` |
| `data_filtering_profiles()` | `list`, `get`, `replace`, `list_all` |

Every client also exposes `list_pages` and `list_items`. There is no invented create/delete endpoint for filtering profiles or delete endpoint for data profiles. The legacy AIRS `/v1/mgmt/dlpprofiles` resource remains a separate management contract.

Page and item helpers take `(params, max_pages, request_options)`, with request lifecycle options last. Their argument order was standardized during prerelease review to match the other product clients.

## Requests and patches

Operations accept generated typed models and optional `RequestOptions`. Inputs, path identifiers, and multipart metadata are checked before credentials are requested. Sort criteria remain separate `sort=` parameters, in caller order, including duplicate entries. Dictionary `keywords=false` is transmitted explicitly when supplied. Unknown response fields, nullable values and unfamiliar enum strings remain available in typed responses; ordinary formatting redacts values.

PATCH sends `application/merge-patch+json`. `Patch::Unset` omits a field, `Patch::Null` clears it, and `Patch::Value` replaces it. Required fields remain required: a data-pattern patch needs `name`, `type`, and `detection_config`; a data-profile patch needs `name` and `profile_type`; a dictionary patch needs `category`, `name`, and `original_file_name`.

DELETE accepts an empty successful response. Dictionary PUT returns `Option<DictionaryResponse>` because both a resource response and HTTP 204 are valid. Mutations are dispatched once even when a retry override is supplied. Transport failures after dispatch preserve `RemoteOutcome::Unknown`; callers reconcile server state before repeating a mutation.

## Dictionary multipart uploads

`DictionaryUpload` holds typed metadata, an owned `BinaryFile`, and optional `include_keywords`. `DictionaryUpload::text` supplies UTF-8 text with `text/plain`; binary inputs preserve their bytes and declared MIME type, defaulting to `application/octet-stream` when none is supplied.

The upload has exactly two parts: `json` uses filename `metadata.json` and MIME `application/json`; `file` uses the metadata's `original_file_name`. The file's own filename hint is replaced by that required metadata name. Metadata is normalized through `DictionaryRequestSchema`, then the encoder applies filename/header checks and bounds the entire multipart envelope. Boundaries are generated locally and checked against all part bytes and headers. The same caller-owned transport sends the final bytes.

```rust
use scm_sdk::dlp::{DlpClient, DictionaryUpload, models::{DictionaryCategory, DictionaryRequest}};

async fn upload(dlp: &DlpClient) -> scm_sdk::Result<()> {
    let metadata = DictionaryRequest::builder()
        .category(DictionaryCategory::Confidential)
        .name("Synthetic keywords".into())
        .original_file_name("keywords.txt".into())
        .region_name("us".into())
        .build()?;
    let input = DictionaryUpload::text(metadata, "alpha\nbravo\n")?
        .include_keywords(true);
    let _resource = dlp.dictionaries().create(&input, None).await?;
    Ok(())
}
```

`BinaryFile` construction owns caller bytes without imposing a size ceiling. The owning upload applies `validate(max_request_bytes)` and the encoder checks the complete encoded size, including metadata and delimiters. CR/LF, controls, traversal filenames, quoted header injection and invalid MIME metadata are rejected locally.

## Bounded page walks

`list` returns the complete Spring envelope. A walk uses page zero and size 50 by default. It determines completion in the frozen order: explicit `last`, otherwise `page + 1 >= totalPages`, otherwise a short page. An empty non-final page is an error. Responses retain server order and duplicates.

`list_all(params, collect_options, request_options)` returns a `CollectionResult`, with explicit completion, accepted items, page counts, byte counts and a resume position. Defaults cap collection at 10,000 items, 64 MiB of serialized item-array bytes, and 1,000 pages. `max_items=0` does not disable bounds. Truncation states identify the item, byte or page cap; failed walks retain accepted rows and a separate safe error.

Use `list_items(...).resume_from(saved_resume)` to refetch a saved page and skip its already-consumed prefix. Preserve the original filters and page size when resuming. Listings are not snapshots; a changed server page can move rows, so applications decide whether resource IDs need deduplication. One absolute request deadline and cancellation token covers an entire walk, including buffered items and all credential requests.

## Deliberate Rust adaptations

- Canonical product namespace and explicit clients replace the TypeScript namespace's internal constructor options; `RequestOptions` controls each call's lifecycle.
- Direct list queries use typed nonnegative pages and positive sizes, bounded/control-free sort and name filters, and the shared URL bound. These reject inputs the frozen direct-list code forwarded to the server.
- Collection caps are explicit and never silently report a truncated list as complete. A starting page is supported as a native extension. Invalid or fractional/negative `totalPages` is reported as a pagination error when that metadata is needed.
- Dictionary metadata is validated before encoding. The frozen create/replace implementation built FormData directly without invoking its metadata schema. Filenames and MIME values also receive native header-safety checks.
- Native multipart bytes include an explicit boundary-bearing Content-Type. TypeScript delegated that header and encoding to FormData. The part names, metadata, filename, MIME semantics and file bytes match.
- Mutations are never automatically replayed; uncertain outcomes are explicit.

The per-operation mappings in `contracts/native-operation-mappings/dlp.json` record these adaptations. Hand-authored tests cover all 24 frozen methods, exact headers/routes/queries, multipart bytes, patch states, Spring stopping rules and caps/resume, invalid input before credentials, encoded identifiers, empty deletes, nullable replacements and uncertain mutation failures. Offline contract verification is separate from live acceptance.
