# Directional security profiles

Publication statements below record the source qualification snapshot and remain
in the immutable archive after release. Check [scm-sdk 0.2.4 on crates.io](https://crates.io/crates/scm-sdk/0.2.4) and the repository's version-specific
registry receipt for current availability.

The scm-sdk 0.2.4 release candidate, not yet published, implements the security-profile model changes in TypeScript
SDK 0.35.0 at commit `780c985cac90144ea1884fe6cffa58b5deec9ac4`. It is separate
from the published 0.2.3 release. Offline model and wire checks do not establish
that a service or tenant applies these settings; directional live acceptance
remains unverified.

## Types and wire fields

Enable `models` to use `scm_sdk::airs::management::models`; enable `management`
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

The [offline example](../crates/scm-sdk/examples/directional_profiles.rs) builds a
prompt configuration, retains false inspection, supplies the placeholder DLP
identifier `100`, validates the request, and demonstrates lossless response
handling. Run it from the repository without credentials:

```sh
cargo run -p scm-sdk --example directional_profiles --no-default-features --features management
```

For example, optional setters take the wrapper explicitly:

```rust
use scm_sdk::{
    airs::management::models::{ContentTypeConfiguration, ContentTypeConfigurations},
    data::Optional,
    validation::Mode,
};

let prompt = ContentTypeConfiguration::builder()
    .build_validated(Mode::Request)?;
let directions = ContentTypeConfigurations::builder()
    .prompt(Optional::Value(prompt))
    .build_validated(Mode::Request)?;
```

For the complete compiled and executed program, see
[directional_profiles.rs](../crates/scm-sdk/examples/directional_profiles.rs).

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

The [additive ledger](../contracts/typescript-035-parity.json) records the new
exports and field occurrences separately from the frozen TypeScript 0.34.0
3,820-capability and 448-method inventory. The
[oracle corpus](../contracts/typescript-035-oracle.json) records actual pinned
Zod inputs and outcomes; Rust request-mode checks compare acceptance and
normalized output. Intentional tolerant response differences have separate tests.

Existing endpoint methods, operation names, shared protections, Rust 1.89,
feature flags and production dependencies remain. The twelve vendor-unsupported
Red Team query bindings retain their documented status; this profile model
addition supplies no evidence that the service applies those options. Publication
and remote delivery require their own phase 28 evidence.
