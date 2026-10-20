---
title: Pagination
---

`scm_sdk::pagination` provides caller-driven `Pages` and `Items`. Product list
methods retain their native page envelopes; their page/item/all helpers adapt
the service's offset, skip, or Spring page dialect. Every request in a walk uses
one `RequestOptions` snapshot with an absolute deadline and shared cancellation.
There is no background page fetch or item deduplication.

`CollectOptions` defaults to 10,000 items, 64 MiB of serialized JSON array data,
and 1,000 pages. Counts include commas and array brackets. The independent page
limit also bounds the cursor-history entry count. Explicit larger limits are
validated, up to 1,000,000 items, 512 MiB, and 100,000 pages. Use
`with_max_items`, `with_max_bytes`, and `with_max_pages`, or mutate a default
snapshot. Zero does not disable these bounds.

`Collection` contains `items`, `completion`, `resume`, `pages_read`, and
`wire_bytes`. Always inspect `is_complete()` before treating its rows as a full
inventory. `Completion::Truncated` identifies the item, byte, or page limit.
Reaching a limit at the known end of the final page remains complete. A byte
limit leaves the first uncollected item at the saved position.

Failures return `CollectionError` with accepted rows and a resume position in
`partial()`, plus the original structured SDK error in `error()`. `into_parts()`
extracts both. Its standard error source contains only the sanitized SDK error.
An iterator that has failed cannot later claim successful completion. Lookup
helpers reject truncated inventories rather than reporting a false absence.

`Resume<C>` identifies a request cursor and the number of items already consumed
from that page. Construct a fresh product item iterator and call
`resume_from(position)` before fetching. The page is fetched again and the
consumed prefix is skipped. An offset beyond the returned page fails explicitly.
Listings are not snapshots: resource additions, removals, and reordering between
requests can cause misses or repeats. Applications choose reconciliation and
deduplication rules. Ordering and duplicate rows within responses are preserved.

Repeated cursors fail before a duplicate request. Empty non-final pages fail
explicitly. This generalizes the frozen Spring walk's safety rule to every native
walk. Spring collectors choose `last`, then `totalPages`, then a short-page
fallback; their all-page default size is 50. Service page methods retain their
own defaults. Native collections always report truncation; the TypeScript
`collectAll` returns only an array and permits an unbounded `max: 0`.

Generic cursor types implement `Serialize` for resource accounting. Default
`CursorLimits` allow 4 KiB per serialized cursor and 4 MiB of retained history.
`Pages::new_with_cursor_limits` accepts explicitly validated alternatives, capped
at 1 MiB per cursor and 64 MiB total. Product page iterators also accept
`.with_cursor_limits(limits)` before the first fetch; apply this before
`.items().resume_from(position)` when resuming an opaque cursor. Initial, resumed, and returned next cursors
are checked before cloning them into retained state. These bounds count encoded
data; caller-defined serializers, hash functions, and fetch callbacks are trusted
integration code. Ordinary formatting redacts cursor and row data.

Cancellation and deadlines are checked before invoking a fetch callback and
before yielding buffered items. Local failures are latched. The current page's
row data remains subject to the client's response, validator, and per-page
resource limits independently of collection limits.

See the compiled [pagination example](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/pagination.rs)
and [management example](https://github.com/cdot65/scm-sdk-rs/blob/efe9de04d9f60d48e6ba8528d1a07e86376d706d/crates/scm-sdk/examples/management.rs).

If a received next cursor exceeds its bound, the fetched page's rows remain
available. A page walk returns the actual page, then fails on its next advance;
an item walk yields the accepted rows, then fails. Collection errors retain those
rows within the collection bounds, count the page, and resume at the consumed
offset of that page. The rejected next cursor is not cloned into iterator state.
Start a fresh iterator with explicitly larger cursor limits and apply the saved
resume position to continue. Raising limits after a walk has started is rejected.
A known cursor failure never becomes successful completion or a silent cap result.
