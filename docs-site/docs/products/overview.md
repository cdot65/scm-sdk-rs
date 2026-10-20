# Strata Cloud Manager products

`scm-sdk` is organized around Strata Cloud Manager, with Prisma AIRS as the first
implemented product umbrella. Shared DLP and IAM clients sit alongside AIRS so
applications can use those services directly.

The crate is community-maintained and is not an official Palo Alto Networks SDK.

## Available in 0.2.3

| Area | Rust module | Cargo feature | Start here |
| --- | --- | --- | --- |
| AIRS runtime security | `scm::airs::scan` | `scan` — default | [Scanning](/airs/scanning) |
| AIRS management | `scm::airs::management` | `management` | [Management](/airs/management) |
| AIRS Red Teaming | `scm::airs::red_team` | `red-team` | [Red Teaming](/airs/red-team) |
| AIRS Model Security | `scm::airs::model_security` | `model-security` | [Model Security](/airs/model-security) |
| AI Gateway | `scm::airs::gateway` | `gateway` | [Gateway](/airs/gateway) |
| Shared DLP | `scm::dlp` | `dlp` | [DLP](/shared/dlp) |
| Shared IAM | `scm::shared::iam` | `iam` | [IAM](/shared/iam) |

Runtime scanning supports synchronous scans, asynchronous batches, and result
and report queries. Management, Red Teaming, Model Security, and Gateway each
have their own resources and authentication requirements. Consult their guides
before sharing a client or selecting a service endpoint.

AgentGuard and realtime capabilities are experimental and feature-gated.
Experimental Gateway access-policy APIs also have separate qualification
limitations. Enabling a feature does not establish that a tenant has access to
every service workflow.

## How coverage is described

The SDK has a frozen TypeScript compatibility baseline plus separately pinned
vendor OpenAPI updates. Native implementation, offline contract coverage, and
successful live service calls are distinct claims.

OpenAPI documents are used to identify additions and changed contracts. Existing
SDK functionality is retained when it is absent from a newer vendor document.
Per-product documentation records relevant compatibility differences and live
qualification limitations.

## Planned product families

| Product family | SDK status |
| --- | --- |
| [NGFW](/products/ngfw) | Planned; no dedicated NGFW client or Cargo feature in 0.2.3. |
| [SASE](/products/sase) | Planned; no dedicated SASE client or Cargo feature in 0.2.3. |

These are sibling product families under the SCM scope. They are not nested
under AIRS. Future client names, resource boundaries, and delivery dates remain
undecided.

An AIRS API hosted on an `api.sase.paloaltonetworks.com` hostname does not imply
that the SDK implements SASE product management.

## Start building

Install only the product features your application needs. All implemented
clients use the native Rust foundation, with application-owned async execution
and explicit credentials.

- [Installation and feature selection](/getting-started/installation)
- [First runtime scan](/getting-started/quick-start)
- [Configuration and authentication](/getting-started/configuration)
- [Rust API reference](/reference/api/)
