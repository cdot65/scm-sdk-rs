# Native OAuth lifecycle and observation

`auth::AuthContext` holds immutable issuer, client ID, credential generation, tenant, scopes and optional audience. `OAuthProvider` uses the caller's transport, destination policy and session. `ClientBuilder::oauth_provider_options` supplies immutable `OAuthProviderOptions`; standalone callers use `OAuthProvider::new_with_options`.

Refreshes are deduplicated by provider instance. One cancelled or dropped waiter does not cancel other waiters. When all waiters disappear, the pending HTTP future is released; the SDK owns no detached task. One refresh has at most 30 seconds and a 64 KiB response budget. Authentication validates the response token type, bearer syntax, positive bounded lifetime and header safety. Form-encoded Basic client credentials follow RFC 6749. Tenant IDs use the native numeric contract. Token generations prevent a delayed unauthorized response from evicting a newer token. A session cancellation interrupts pending refreshes and API requests.

## Refresh buffer

`OAuthProviderOptions::refresh_buffer` selects:

- `RefreshBuffer::Adaptive(maximum)`: the smaller of `maximum` and 20% of the token lifetime. The default maximum is 30 seconds.
- `RefreshBuffer::Fixed(duration)`: the exact source fixed-buffer behavior. Set `Duration::from_millis(30_000)` for the frozen default; zero disables early refresh. A duration at least as long as the token lifetime makes every subsequent `get_token` call refresh.

Buffers use monotonic instants and saturating duration arithmetic, including `Duration::MAX`. `is_token_expiring_soon(Some(duration))` compares an explicit buffer directly against remaining lifetime; `None` uses the configured buffer. `is_token_expired`, `token_info`, and `clear` retain their separate meanings. Clearing invalidates older pending publication so an old response cannot restore the cleared cache.

The adaptive default is a documented safety difference for short-lived tokens. Caller-selected fixed buffers retain source configurability without unvalidated JavaScript numbers or timestamp overflow.

## Refresh observation

`OAuthProvider::subscribe_refresh()` returns a bounded `tokio::sync::broadcast::Receiver<TokenInfo>`. Subscribe before fetching if the first refresh matters. `OAuthProviderOptions::event_capacity` is 1–1024, default 16. Application code owns and polls the receiver; no observer callback executes inside the provider and no SDK worker consumes events.

A successful refresh emits one metadata snapshot after publishing its generation and releasing the cache lock. Cache hits, failures, cancelled refreshes and rejected stale publication emit no successful event. Snapshots are historical: a subsequent clear or refresh may already have changed the current cache. Dropped or absent observers cannot block token delivery. Slow observers receive `RecvError::Lagged` and can query `token_info` to reconcile current state.

`TokenInfo` contains cache presence, generation, optional remaining duration, refresh-in-progress, validity/expiry flags and optional wall-clock expiry. It contains no token, client ID, issuer path, scopes, audience or tenant. Optional `Duration`/`SystemTime` values replace the source millisecond and zero sentinels; cache validity uses the monotonic clock. `TokenLease::secret` is explicit secret access and ordinary formatting is redacted.

## Evidence

The auth module's independent fake-transport tests cover deduplication, waiter drop/cancellation, refresh budgets, TLS/policy seams, isolation, delayed unauthorized generations, HMAC/header safety and formatting. Additional buffer/event tests cover fixed/adaptive/zero/oversized windows, successful generation publication, reentrant observation, failed/cleared publication, lagged/dropped observers and capacity validation. `tests/configuration_contracts.rs` checks provider options, metadata observation, destination policy, explicit environment precedence and cancellation through the public construction paths.
