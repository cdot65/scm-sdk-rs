---
title: Core client and policies
sidebar_position: 8
slug: /reference/api/core
---

# Core client and policies

`Client` and `ClientBuilder` provide shared authentication, transport and request
policies. Product clients wrap this foundation. The SDK is async and uses the
application's runtime.

| Module or type | Responsibility |
| --- | --- |
| `auth` | `Authentication`, OAuth providers and caller-supplied `TokenProvider` implementations |
| `config` | `Endpoint`, `OriginPolicy`, `Limits`, `RequestOptions` and `Session` |
| `transport` | `Transport` abstraction, request/response types and optional `ReqwestTransport` |
| `error` | Typed errors and error classifications |
| `data` | JSON values, finite numbers, omission/null/value representations |
| `pagination` | Pages, item streams, bounded collections, resume and truncation status |
| `validation` | Request and response validation modes |
| `secret` | Secret-bearing values with redacted formatting |

## Response policy

`RequestOptions::without_response_defaults()` disables insertion of missing
schema defaults for that operation. `response_defaults()` reports the policy.
Direct validation offers `Mode::Request`, `Mode::Response` and
`Mode::ResponseWithoutDefaults`.

Both response modes preserve unknown enum strings and object keys and tolerate
response string formats. Required fields and types still depend on the selected
schema. Request validation continues to enforce its contract.

`Patch` distinguishes unset, null and value. A bounded collection can report
truncation; inspect its completion status before treating it as a full result.
Errors and transport failures on writes may leave the remote outcome uncertain.

## Generated reference

- [Client](https://docs.rs/scm-sdk/0.2.4/scm_sdk/struct.Client.html)
- [Configuration](https://docs.rs/scm-sdk/0.2.4/scm_sdk/config/index.html)
- [Authentication](https://docs.rs/scm-sdk/0.2.4/scm_sdk/auth/index.html)
- [Transport](https://docs.rs/scm-sdk/0.2.4/scm_sdk/transport/index.html)
- [Pagination](https://docs.rs/scm-sdk/0.2.4/scm_sdk/pagination/index.html)
