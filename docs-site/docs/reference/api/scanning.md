---
title: Runtime scanning
sidebar_position: 2
slug: /reference/api/scanning
---

# Runtime scanning

**Module:** `scm_sdk::airs::scan` · **Feature:** `scan` (enabled by default)

Use `Scanner` for runtime content inspection. Construct it from a configured
API-key client with `Scanner::new`, or select a regional endpoint with
`Scanner::for_region`. `Content` and `ContentBuilder` construct scan content;
`SyncScanOptions` supplies synchronous scan options.

| Method | Purpose |
| --- | --- |
| `sync_scan` | Scan content synchronously using the scanner's typed convenience interface |
| `scan_request` | Submit a typed synchronous scan request |
| `async_scan` | Submit an asynchronous scan batch |
| `query_by_scan_ids` | Retrieve results for scan identifiers |
| `query_by_report_ids` | Retrieve reports for report identifiers |

Requests remain subject to validation, resource limits and caller-selected
`RequestOptions`. Choose the service region deliberately; a regional scanner
configuration is separate from the endpoints used for OAuth management APIs.

## Generated reference

- [Scanner and its method signatures](https://docs.rs/scm-sdk/0.2.4/scm_sdk/airs/scan/struct.Scanner.html)
- [Scan module and model namespace](https://docs.rs/scm-sdk/0.2.4/scm_sdk/airs/scan/index.html)

See [Core client](./core.md) for request policies, errors and transport injection.
