---
title: Directional Security Profiles
---

# Directional security profiles

**Available in 0.2.4.** This guide describes the TypeScript SDK 0.35.0
security-profile additions in the [published Rust patch release](https://crates.io/crates/scm-sdk/0.2.4).
Directional live acceptance remains unverified: local model and wire checks do
not establish that a service or tenant applies these settings.

The examples use the `scm` dependency alias from the
[installation guide](/getting-started/installation).

## Types and wire fields

Enable `models` to use `scm::airs::management::models`; enable `management`
for profile clients. `ContentTypeConfiguration` reuses the existing protection
types through these optional fields:

| Rust field | Type | Wire key |
| --- | --- | --- |
| `data_protection` | `Optional<DataProtection>` | `data-protection` |
| `app_protection` | `Optional<PolicyAppProtection>` | `app-protection` |
| `model_protection` | `Optional<Vec<ModelProtectionItem>>` | `model-protection` |
| `agent_protection` | `Optional<Vec<AgentProtectionItem>>` | `agent-protection` |

`ContentTypeConfigurations` has `prompt`, `response`, `tool_call`, and
`tool_response`, each `Optional<ContentTypeConfiguration>`. Their wire keys are
`prompt`, `response`, `tool-call`, and `tool-response`. Each direction is
independently optional. Omitted directions remain absent; unknown future direction
keys and their JSON values survive in extensions. No direction is inferred.

The existing models gain five typed field occurrences:

| Model | Rust field and type | Wire key |
| --- | --- | --- |
| `AiSecurityProfile` | `content_type_mode: Optional<String>` | `content-type-mode` |
| `AiSecurityProfile` | `content_type_configurations: Optional<ContentTypeConfigurations>` | `content-type-configurations` |
| `ModelConfiguration` | `enable_full_conversation_inspection: Optional<bool>` | `enable-full-conversation-inspection` |
| `SecurityProfile` | `dlp_tenant_id: Optional<String>` | `dlp_tenant_id` |
| `CreateSecurityProfileRequest` | `dlp_tenant_id: Optional<String>` | `dlp_tenant_id` |

`per_content_type` is an observed mode string, not an exhaustive enum. Shared
`model_configuration` and legacy protection settings remain available alongside
directional settings. `Optional::Unset` omits a typed field; `Optional::Value(false)`
preserves an explicit false inspection setting. The SDK inserts no inspection or
content-mode default.

Both new types expose builders. The new named validators are
`ContentTypeConfigurationSchema` and `ContentTypeConfigurationsSchema`, usable
with `validation::parse`. Together, the two types and two schemas account for
four added TypeScript exports; generated Rust builders are their native support.

## Construction and current request validation

The 0.2.4 release includes the offline [`directional_profiles` example](https://github.com/cdot65/scm-sdk-rs/blob/6f18529a47cc273feca1000af6e4fc0d96da757e/crates/scm-sdk/examples/directional_profiles.rs). It builds a
prompt configuration, retains false inspection, supplies the placeholder DLP
identifier `100`, validates the request, and demonstrates lossless response
handling. Run it from that release’s source checkout without credentials:

```sh
cargo run -p scm-sdk --example directional_profiles --no-default-features --features management
```

For example, optional setters take the wrapper explicitly:

```rust
use scm::{
    airs::management::models::{ContentTypeConfiguration, ContentTypeConfigurations},
    data::Optional,
    validation::Mode,
};

fn directions() -> scm::Result<ContentTypeConfigurations> {
    let prompt = ContentTypeConfiguration::builder()
        .build_validated(Mode::Request)?;
    ContentTypeConfigurations::builder()
        .prompt(Optional::Value(prompt))
        .build_validated(Mode::Request)
}
```

`CreateSecurityProfileRequest::builder().policy(policy)` takes a required `Policy`
value directly. Both create and update accept this same request type. Their
operation paths validate against
`typescript:0.35.0:CreateSecurityProfileRequestSchema` before authentication or I/O.
No new update request type or endpoint is introduced.

Existing schema names, model `SCHEMA` identities, model `parse` methods and
`build_validated` methods on existing models retain their historical schema
binding. To validate a standalone existing model against the current contract,
call `validation::parse` with `Mode::Request` and the appropriate versioned name:

- `typescript:0.35.0:ModelConfigurationSchema`
- `typescript:0.35.0:AiSecurityProfileSchema`
- `typescript:0.35.0:PolicySchema`
- `typescript:0.35.0:SecurityProfileSchema`
- `typescript:0.35.0:CreateSecurityProfileRequestSchema`
- `typescript:0.35.0:SecurityProfileListResponseSchema`

The two new content-type schemas use their ordinary unprefixed names. Current
request validation rejects explicit null or the wrong type for newly known
fields and known direction entries, including values supplied through raw
extensions. Valid extensions still serialize. Required request policy, integer
request revision, timestamp format and existing protection constraints remain.

## Response compatibility

Upstream TypeScript 0.35.0 response schemas reject newly known fields with invalid
types or explicit null. Rust profile operation responses deliberately retain the
frozen tolerant schema binding. Both `Mode::Response` and
`Mode::ResponseWithoutDefaults` preserve such values through extensions instead
of rejecting them. The new typed projections populate only when they can
represent the value; an unrepresentable nested directional document remains in
the containing `content-type-configurations` extension.

For these five projected fields, a typed `Optional::Value` takes precedence over
a same-key extension during serialization. When the typed field is unset, the
raw extension remains, including explicit null. This preserves omission, null
and value without changing these optional fields into `Patch<T>`. Ordinary
known-field extension collision rules still apply elsewhere, including the two
new content-type types.

The list schema retains its existing `ai_profiles: []` default when the field is
omitted. `ResponseWithoutDefaults` leaves it absent. The versioned list schema
copies that same default; no new default value or response acceptance narrowing
is introduced.

## Evidence and scope

The additive ledger (`contracts/typescript-035-parity.json`) records the new
exports and field occurrences separately from the frozen TypeScript 0.34.0
3,820-capability and 448-method inventory. The
oracle corpus (`contracts/typescript-035-oracle.json`) records actual pinned
Zod inputs and outcomes; Rust request-mode checks compare acceptance and
normalized output. Intentional tolerant response differences have separate tests.

Existing endpoint methods, operation names, shared protections, Rust 1.89,
feature flags and direct dependency declarations remain. The 0.2.4 release
also refreshes two transitive TLS packages in the repository lockfile; see the
[TLS dependency note](/developer/security#024-tls-dependency-patches). The twelve vendor-unsupported
Red Team query bindings retain their documented status; this profile model
addition supplies no evidence that the service applies those options. Publication
and remote delivery require separate evidence. See the
[release notes](/about/release-notes) for publication status and
[live qualification](/developer/live-qualification) for the evidence boundary.
