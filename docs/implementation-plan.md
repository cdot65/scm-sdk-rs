---
schema_version: 1
title: Rust SCM SDK implementation plan
type: spec
created: '2026-10-16'
modified: '2026-10-16'
effort: '[[Efforts/prisma-airs-sdk/index]]'
review_state: pending
summary: Accepted design for scm-sdk, with full Prisma AIRS TypeScript capability parity, product namespaces, security guarantees, and delivery milestones.
---

# Rust SCM SDK implementation plan

Build `scm-sdk`, a public Rust SDK for Strata Cloud Manager, in a dedicated repository named `scm-sdk-rs`. Prisma AIRS is the first complete product family. Future product families include SASE, NGFW, DLP, and IoT, with shared capabilities implemented once when their contracts match.

`cdot65` owns maintenance, compatibility decisions, security exceptions, and releases. Project-owned code uses Apache-2.0 with community-maintained attribution and retained third-party notices. The accepted decisions come from the accepted design interview of October 16, 2026.

## Scope and source authority

Initial parity covers every exported capability of `@cdot65/prisma-airs-sdk` at committed version `0.34.0`, commit `4a80dbdb3dc14365e1c25d838ac0bd139485d48c`. This includes experimental methods, models, validation, pagination, helpers, configuration capabilities, errors, and constants. Each export receives an explicit Rust equivalent or a documented capability-preserving change. For example, per-client configuration replaces the TypeScript global singleton, and Rust validation replaces exported Zod validators.

The existing TypeScript checkout is `/home/cdot/development/cdot65/prisma-airs-sdk`, verified from the imported location in the imported CORE-INFRA source record. Its uncommitted work is outside the frozen baseline. The Rust application checkout is `/home/cdot/development/cdot65/scm-sdk-rs`, cloned from `ssh://git@git-ssh.cdot.io/cdot/scm-sdk-rs.git`. The knowledge vault holds project records.

Freeze the TypeScript commit, export inventory, specification commits and hashes, compatibility corrections, sanitized fixtures, and per-operation verification status together. Later TypeScript or upstream changes become separately tracked additions. After initial parity, Rust evolves against pinned vendor contracts, reviewed service corrections, and shared regression fixtures, including SCM families absent from TypeScript.

Keep three independent measures:

| Measure | Required evidence |
| --- | --- |
| Exported capability parity | Every frozen export maps to an implemented Rust capability, with its tests, documentation, feature requirements, and deliberate differences. |
| Specification conformance | Exact routes, planes, fields, queries, validation, and transport contracts match each separately identified source. |
| Live acceptance | The intended workflow succeeds against the stated tenant, permissions, service deployment, and provider prerequisites. |

Official AIRS, official PANW Gateway, legacy Portkey compatibility, IAM, and observed browser contracts retain their own provenance and coverage denominators. TypeScript parity alone cannot establish full vendor API coverage or successful live workflows. Experimental capabilities can meet implementation and offline contract requirements while retaining explicit live limitations; stable promotion requires successful live acceptance.

## Public modules and packaging

Start with one published crate in a Cargo workspace. Use product modules, a small additive feature set, and private shared transport internals. Split packages when actual dependency or release requirements justify the additional compatibility work.

Consumers may use this dependency alias:

```toml
[dependencies]
scm = { package = "scm-sdk", version = "0.1" }
```

The initial public module structure is:

```text
scm::airs::scan
scm::airs::management
scm::airs::model_security
scm::airs::red_team
scm::airs::gateway
scm::airs::agentguard
scm::dlp
scm::shared::iam
```

`scm::airs::gateway` separates SCM configuration and telemetry from runtime inference, realtime sessions, and public pricing. Their endpoint and credential contracts remain distinct. AgentGuard and other experimental capabilities retain explicit support labels and opt-in exposure.

DLP and IAM have canonical implementations with product convenience access where useful. Future `scm::sase`, `scm::ngfw`, `scm::iot`, and other product modules are introduced with implemented contracts. Shared tenancy and configuration capabilities grow as required. A common resource name or console location is insufficient evidence to share models: verify schema, permissions, and behavior first.

Product features control compilation and dependencies. They are additive and do not grant permissions or replace runtime security checks. Establish and test the documented combinations, including default, minimal, and all-feature builds. Experimental labels and feature flags do not override Cargo compatibility rules.

## Interface and generation design

Public clients use typed requests, responses, builders, and `Result` errors. Validate requests before acquiring credentials or performing network work. Constructors accept explicit configuration; environment-loading helpers are explicit and produce a validated configuration snapshot. Support concurrent clients for different tenants without mutable global state.

Use an async public interface with Tokio and reqwest as the default implementation. The application owns runtime creation and shutdown. Client clones share only the appropriate connection pool, limiter, and authentication context. Provide a narrow integration point for caller-controlled transport and destination admission. HTTP, token exchange, downloads, streaming, and realtime connections must preserve the declared transport policy.

Desktop and harness provide integration evidence as their architectures evolve. The SDK introduces no GPUI-specific types or lifecycle requirements. General executor independence or a blocking interface becomes additional scope when a concrete consumer requires it.

Generation uses pinned source specifications and reviewed corrections. Generate repetitive models and contract fixtures where reliable; handwrite public clients, authentication, and transport. Preserve generator version, input hashes, licenses, correction rationale, and deterministic output. Review generated diffs and verify reproducibility in CI. Consumer builds use packaged Rust sources and require no Node SDK or external TypeScript checkout.

## Authentication and destination safety

Applications own interactive login, OS keychains, and persistent credentials. The SDK accepts explicit credentials or token providers, manages in-memory service-token lifecycle, and exposes cancellation for requests and sessions.

Bind token cache state to immutable authentication identity: token issuer, client identity and credential generation, tenant or TSG, requested scopes, and audience where supported. Deduplicate refreshes, refresh before expiry, and invalidate only the token generation rejected by the server. Cancellation by one waiter must not terminate a refresh needed by other callers. Session or credential invalidation must be able to cancel affected in-flight work.

Validate credential-bearing destinations before attaching secrets. Defaults use verified HTTPS or WSS and reject authenticated redirects. Bind each credential kind to its configured origins. Runtime endpoints and keys are supplied explicitly, with separate handling for SCM OAuth, AIRS scan keys or tokens, runtime credentials, and unauthenticated public pricing. Signed download or upload URLs use separate requests without inherited service credentials. An explicit loopback development setting may permit local HTTP.

Caller-controlled transports are trusted integration code; document which guarantees depend on their behavior. The SDK still validates requests and destinations before delegating. OAuth token exchange follows the same supplied policy as API calls. Region headers, endpoint selection, and service initialization remain specific to the service contract.

Validate and serialize HMAC-signed bodies once. Sign the exact immutable bytes transmitted. Retries reuse those bytes, including Unicode and escaping; signing occurs after any body-changing transformation.

## Data and failure guarantees

| Concern | Required behavior |
| --- | --- |
| Response evolution | Preserve additional fields and unfamiliar wire values while validating required structure. Unknown security outcomes remain explicit and never become an implicit allow, benign, or clean result. Application enforcement policy remains with the caller. |
| Request models | Use validated typed inputs and reviewed extension maps where supported. Preserve contractual differences between missing and nullable fields. |
| Merge patch | Represent unchanged, explicit null, and replacement value separately. Verify serialized PATCH bodies against independent fixtures. |
| Errors | Provide structured categories, HTTP status, allowlisted correlation identifiers, service codes where available, retry metadata, and safe diagnostics. Exclude arbitrary upstream bodies and secret values from ordinary errors and formatting. |
| Retries | Repeat operations only when replay safety is established. Honor server-directed delays within bounded deadlines. Default runtime inference to no automatic retries; never silently replay a stream or an ambiguous mutation. |
| Pagination | Offer page and item iteration, retain resumable positions where supported, detect repeated cursors, bound collection, and explicitly report truncation. Document ordering and non-snapshot behavior. |
| Workflows | Return completed steps, known resource IDs, uncertain remote outcomes, and reconciliation information. Establish server state before deleting or recreating resources after an ambiguous failure. |
| Cancellation | Stop local work and release local resources. Preserve uncertainty about committed server writes, running jobs, or billed inference. |

Bound queueing, authentication, attempts, retry sleeps, and body reads within the configured operation budget. Streaming and realtime have separately documented handshake, idle, close, and optional session budgets. Polling preserves the job ID on interruption and has cancellation and total bounds. Avoid detached polling or refresh tasks without caller-owned lifetime.

Bound response and error bytes actually consumed, including decompressed bytes. Stream large reports and files. Bound stream event size, realtime queues, parser work, and concurrency; waiting for permits must be cancellable. Explicitly select and validate numerical defaults during the foundation milestone rather than infer service quotas.

Full parity includes JSON, text, binary, multipart, bounded SSE, and experimental realtime WebSockets. Test termination markers, truncated streams, malformed frames, slow consumers, backpressure, cancellation, and resource release on drop. An HTTP upgrade alone does not establish a working realtime provider session.

## Security and observability

Prohibit unsafe code in project-owned Rust. Review network and cryptography dependencies, including their native build requirements and supported targets. Use secret types and safe formatting for credentials and sensitive request or response models, including generated models and unknown extension maps.

Tracing is optional, metadata-only, and uses static operation names plus allowlisted status and correlation information. Ordinary `Debug`, `Display`, tracing, validation errors, and nested errors exclude secrets, prompts, source code, signed URL credentials, and arbitrary server text. Callers retain explicit access to faithful API data and own any diagnostic sink that records sensitive payloads.

Applicable security advisories block releases by default. `cdot65` may authorize a documented, scoped, expiring exception after assessing exposure and mitigations. Check dependency advisories, sources, and licenses across supported feature configurations. A successful dependency audit does not replace transport, parser, or secret-exposure tests.

## Verification and release contract

Ordinary PR verification is deterministic and credential-free. Required checks cover formatting, linting, build, unit and public-interface integration tests, documentation examples, generation reproducibility, frozen contracts, supported features, dependency policy, and source compatibility. Test expected wire behavior independently of the implementation that constructs requests.

Adversarial cases include tenant and token isolation, delayed unauthorized responses during refresh, credential rotation, destination rejection, redirects, HMAC byte identity, unknown security verdicts, omitted/null/value patches, echoed secrets, malformed and oversized bodies, interrupted streams, repeated pagination positions, uncertain mutations, and partial provisioning. Use targeted property tests and parser fuzzing where they add coverage beyond ordinary examples.

Controlled live runs use a dedicated nonproduction tenant, least-privilege credentials, owned synthetic fixtures, bounded costs and resource creation, reviewed mutation cases, and explicit cleanup outcomes. Retain sanitized evidence with source version, tenant scope, operation status, and relevant service prerequisites. Permission failures, provider incompatibility, backend failures, missing fixtures, and missing SDK implementation remain distinct classifications.

Initially target Linux x86_64 and ARM64, macOS Apple Silicon, and Windows x86_64. Record exact target triples and distinguish runtime-tested targets from compilation-only checks. Establish the minimum supported Rust version during the foundation milestone and verify it alongside current stable with supported features. Document compatibility changes.

Publish incremental public `0.x` releases. Keep patch releases source compatible, including experimental interfaces; breaking changes require a new `0.y` and migration notes. Public documentation includes quick starts, product and shared-module navigation, authentication and proxy/custom-CA setup, examples, experimental status, coverage, deliberate TypeScript differences, cancellation and partial outcomes, and vulnerability reporting.

Verify the actual packaged crate contents, notices, version, and release commit before publication. Use owner-controlled release automation with minimal permissions and supported trusted publishing after setup. Until a trusted publisher is configured, explicitly authorized maintainer releases may use Cargo's owner-controlled crates.io token stored outside the repository and vault; CI must not receive that credential. The authorized 0.1.1 maintenance procedure is in docs/g10-0.1.1-plan.md. Align source tags and registry artifacts to the exact qualified release commit.

The authorized 0.2.0 registry delivery follows [the release plan](release-0.2.0-plan.md)
and phases 13–14, after completed vendor phases 9–12. The supplied Gateway update
is a subsequent task.

## Delivery milestones

| Milestone | Deliverable | Exit criteria |
| --- | --- | --- |
| 1. Freeze contracts and provenance | Full export-to-Rust inventory, source manifest, corrections, feature and stability classification, test oracle, license and notice inventory. | Every frozen capability is assigned; exact historical inputs are identified; official Gateway and legacy Portkey remain separate; generation inputs have reconciled provenance. |
| 2. Establish the crate and core | Workspace, public modules, typed configuration, authentication, transport integration, safe errors, generation tooling, bounds, and baseline CI. | Verified compiler and target matrix; explicit resource defaults; offline security and contract checks pass; packaged Rust builds independently. |
| 3. Prove a representative slice | API-key/HMAC scan, OAuth management read, runtime inference stream, and a representative multipart or binary flow. | Wire and negative cases pass; approved stable workflows pass live; cancellation, redaction, and caller transport are proven. Re-estimate remaining work from measured effort. |
| 4. Complete exported AIRS capabilities | Remaining management and DLP, model security, Red Team, Gateway, IAM, AgentGuard, helpers, and model/validator equivalents. | Every baseline export has an implementation mapping, documentation, independent contract evidence, and explicit live/support status. Deliver coherent incremental releases. |
| 5. Qualify full parity and stable support | Complete matrix, adversarial and compatibility results, supported platform evidence, release artifact, examples, and live acceptance evidence. | No unexplained capability gaps; applicable security findings resolved or scoped exceptions approved; advertised stable capabilities have passing live acceptance. |
| 6. Expand SCM product families | SASE, NGFW, IoT, broader DLP, and other demanded SCM capabilities with shared ownership where verified. | Each addition has a concrete use case, source contracts, maintenance capacity, appropriate scoping, and the same quality gates. |

Quality gates remain fixed while delivery dates or milestone scope adjust. The earlier effort estimate was based on older snapshots; milestone 3 supplies the evidence for a new estimate. Desktop is the first intended adoption test, with its architecture free to change. Harness adoption targets an explicit native workflow; replacing its bundled TypeScript CLI and distribution is separate work.

## Foundation dependencies

The provenance inspection found differences between current local red-team and legacy Gateway specifications and their frozen fixture hashes. Retrieve the exact historical snapshots, reconcile their licenses and notices, and preserve the frozen baseline before vendoring or generation. Browser-observed contracts require their own dated provenance entries.

The verified Rust checkout and remote are recorded above. Confirm registry-name availability at initial publication. Select exact dependency versions, generator implementation, supported target triples, resource bounds, and minimum Rust compiler through the foundation proof. These are implementation outputs with explicit acceptance gates.

## References

- The design interview and imported source records in the prisma-airs-sdk knowledge effort establish accepted scope and repository provenance.
- Frozen TypeScript source: `src/index.ts`, `AGENTS.md`, `NOTICE`, `test/openapi/fixtures/`, and `docs-site/docs/developer/official-gateway-alignment.md` at the baseline commit.
- [SCM API families](https://pan.dev/scm/docs/home/), [service-specific request rules](https://pan.dev/scm/docs/api-call/), [DLP APIs](https://pan.dev/dlp/api/), and [IoT SCM authentication](https://pan.dev/iot/api/iot-public-api-new/).
- [Cargo workspaces](https://doc.rust-lang.org/cargo/reference/workspaces.html), [features](https://doc.rust-lang.org/cargo/reference/features.html), [compiler support](https://doc.rust-lang.org/cargo/reference/rust-version.html), and [compatibility](https://doc.rust-lang.org/cargo/reference/semver.html).
- [RustSec](https://rustsec.org/), [cargo-deny](https://embarkstudios.github.io/cargo-deny/checks/index.html), [crates.io trusted publishing documentation source](https://github.com/rust-lang/crates.io/blob/main/svelte/src/routes/docs/trusted-publishing/%2Bpage.svelte), and [Apache-2.0 terms](https://www.apache.org/licenses/LICENSE-2.0).

## Authorized Portkey access-policy addition

After 0.2.0 delivery, the maintainer authorized the exact Portkey a27feb9 delta
through [phases15–17](portkey-access-policies-plan.md), targeting additive 0.2.1.
Its explicit key-authenticated control plane remains separate from SCM OAuth.
