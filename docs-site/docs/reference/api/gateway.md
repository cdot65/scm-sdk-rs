---
title: AI Gateway
sidebar_position: 6
slug: /reference/api/gateway
---

# AI Gateway

**Module:** `scm_sdk::airs::gateway` · **Feature:** `gateway`

`AIGatewayClient` exposes SCM configuration and administration. `RuntimeClient`
handles runtime inference and resources using the runtime client's authentication
and endpoint policy. Configure runtime access explicitly with `with_runtime`;
`inference()` and `runtime_resources()` retrieve that configured client.

| Surface | Selected entry points |
| --- | --- |
| SCM resources | `workspaces()`, `guardrails()`, `telemetry()`, `configs()`, `deployments()` |
| Administration | `organisations()`, `providers()`, `rate_limits()`, `api_keys()`, `integrations()`, `audit_logs()` |
| Additional resources | `mcp_servers()`, `mcp_integrations()`, `usage_limits()`, `secret_references()`, `plugins()` |
| Runtime inference | `create_completion`, `create_chat_completion`, `create_embedding`, `create_response` |
| Streaming | `create_completion_stream`, `create_chat_completion_stream`, `create_response_stream` |

Runtime prompt rendering and prompt-completion methods are also available.
`GatewayRouting` and `RuntimeReplay` make routing and replay decisions explicit;
request authorization for one plane does not imply authorization for another.

## Access policies and realtime

The `access_policies` module adds typed create, list, get, update and delete
operations. Its key-authenticated control plane is distinct from SCM OAuth.
These calls remain experimental and live-unverified; the SDK preserves omitted
fields, explicit nulls and non-replayable writes.

Enable `realtime` for realtime support and `ws-transport` for the optional default
WebSocket transport. Streaming consumers should handle cancellation and bounded
resource limits instead of assuming an unlimited connection lifetime.

## Models and signatures

See the [Gateway module](https://docs.rs/scm-sdk/0.2.3/scm_sdk/airs/gateway/index.html).
Hosted 0.2.3 rustdoc is built with default features; generate `--all-features`
documentation for optional client and access-policy signatures using the
[reference instructions](./index.md).
