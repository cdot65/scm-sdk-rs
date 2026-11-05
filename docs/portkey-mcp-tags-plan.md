# Portkey MCP tags delta: 0.2.5 plan (phases 29–31)

## Authorization and review policy

The maintainer authorized this maintenance track on 2026-11-05. Its instruction was to import the AI Gateway OpenAPI changes published in `Portkey-AI/openapi` since our pin, implement and test them, validate them end to end, and release a version bump with updated Docusaurus documentation.

The maintainer also set the review policy for this track. At the end of every chunk and every phase, a **Codex CLI** (`codex exec`, read-only sandbox) reviews the scoped work and gives an integer score from 1 to 10. Acceptance requires a score of **at least 9** with no unresolved blocker or high finding.

This track keeps the existing review rules:

- the five-attempt budget per chunk and per phase
- preservation of every prompt, result and failed attempt
- source binding to `review_phase.fingerprint`

This instruction supersedes the Claude-reviewer and 46/50 rubric requirements for phases 29–31 only. Historical phases keep their original reviewers and rubrics.

Phases 24, 25 and 28 keep their incomplete records. They are not rebound or completed by this track. Version 0.2.4 is already published on crates.io (2026-10-22) and tagged `v0.2.4`.

## Pinned source

| Role | Commit | Path | sha256 | Git blob |
| --- | --- | --- | --- | --- |
| Import base (existing pin) | `a27feb9efaac3653d2d94eca5d7d210944b6e160` | `contracts/vendor/portkey-access/a27feb9/openapi.yaml` | `9c3b5e64…4ca92` | `3804f18b…` |
| Upstream head | `8e5da0e756bc49ee3e82ca13df693b39fa541627` (2026-11-05T12:06:27Z) | `contracts/vendor/portkey-mcp-tags/8e5da0e/openapi.yaml` | `6013532c…7f50e` | `32d3b82b…` |

The upstream head is 10 commits ahead of the base and 0 behind. Only `openapi.yaml` changed, with +745/−6 lines. The license bytes are unchanged (MIT; sha256 `bcba3de2…`).

`tools/portkey-mcp-tags.mjs` does three things:

1. It verifies both byte pins, the parser version and the source URLs.
2. It computes the **complete** structural delta: top-level keys, every path/method field, and every component entry.
3. It fails closed unless every delta item appears exactly once in the hand-reviewed classification ledger `contracts/vendor/portkey-mcp-tags/8e5da0e/delta.json`.

It then emits `contract.json`, which holds only the imported upstream fragments.

## Delta classification

| Delta | Upstream | Disposition |
| --- | --- | --- |
| D1 | `tags` added to `CreateMcpIntegration`, `UpdateMcpIntegration`, `CreateMcpServer`, `UpdateMcpServer`: `object<string,string>`, nullable; on update it replaces, and `null` clears | **import**: typed write field |
| D2 | `tags` added to `McpIntegration`, `McpIntegrationListItem`, `McpServer`, `McpServerListItem` | **import**: typed response field |
| D3 | `tags` query parameter (JSON-encoded object string) on `GET /mcp-integrations` and `GET /mcp-servers` | **import**: list filter |
| D4 | `IntegrationList.tags` | **already supported**: `GatewayIntegration.tags` (frozen `s007274`) |
| D5 | `include_all_workspaces` / `exclude_workspace_slugs` on 22 `/analytics/*` routes, plus two components | **not applicable**: the SDK has no analytics client |
| D6 | `POST /messages`, `POST /messages/count_tokens`, 19 `Anthropic*`/`*Message*` schemas, `Messages` tag | **deferred new capability**: outside this delta's import scope; recorded, not implemented |
| D7 | `POST /threads/{thread_id}/messages` operationId renamed to `createThreadMessage` | **not applicable**: the SDK does not implement threads |
| D8 | `x-code-samples` navigation and top-level `tags` | **documentation only** |

The spec has no workspace, scope, IAM, API-key or usage-limit changes. The open IAM scope-bind 400 is a separate defect and is not part of this track.

## Rust design

### Overlay

A generator overlay, `tools/portkey_mcp_tags.py`, follows the `typescript035_profiles.py` pattern:

- **Frozen roots are unchanged.** Frozen roots and their fixtures keep their identities.
- **Clones carry the new field.** The ancestor closure of each projected node is cloned under `portkey:8e5da0e:` roots. A `tags` property is projected onto each clone.
- **Existing Rust structs gain the field.** They become `Optional<Option<TypedMap<String>>>`-style views of what used to be unknown JSON. Every generated struct is already `#[non_exhaustive]` with builders, so this is a semver-compatible addition.
- **The overlay fails closed.** It stops if a frozen node, root or ancestor closure changes, or if an identity collides.

### Request validation

Requests are validated before authentication:

- Tag keys must match `^[A-Za-z0-9_-]+$`, as upstream documents ("Keys may contain only alphanumeric characters, underscores and hyphens").
- Tag values must be strings.
- `null` is accepted, and omission, null and value stay distinct. On update this is the only way to leave tags unchanged, clear them, or replace them.

Responses accept any string keys and string values.

### List queries

- `GatewayMcpServersClientListOptions` gains `tags`.
- `AIGatewayMcpIntegrationsClient::list` is frozen at no query. A new `list_with_options(&query, options)` takes the new strict `GatewayMcpIntegrationsClientListOptions`. That type covers the pinned operation's complete query contract: `organisation_id`, `type`, `workspace_id`, `page_size`, `current_page`, `search` and `tags`.
- `list(options)` keeps byte-identical wire behavior.
- On the wire, `tags` is the upstream string. The public helper `airs::gateway::mcp_tags_filter(&TypedMap<String>) -> Result<String>` builds canonical JSON, with keys sorted.
- Clients reject any `tags` filter that is not a JSON object of string values. This check runs before authentication.

### Ledgers

The following ledgers record these as Portkey-delta capabilities, separate from frozen TypeScript parity:

- `contracts/vendor/portkey-mcp-tags/8e5da0e/operations.json`
- `native-gateway-bindings.json`
- the native operation and export ledgers

Live status is reported per method.

## Phases and chunks

### Phase 29: plan and provenance

- **Chunk `plan-and-provenance`:**
  - vendored bytes and `source.json`
  - delta verifier and classification ledger
  - `contract.json`
  - tamper tests (`tools/test_portkey_mcp_tags.py`)
  - this plan, the `PHASES.md` section, the `phase-status.json` registration and the `check_phase.py` command set
- **Checks:**
  - the delta verifier `--check` and its unit tests
  - `verify-contracts.py`
  - generation `--check` (unchanged)
  - the existing Portkey and governance unit tests
  - `git diff --check`
  - the private-material scan
- **SDK runtime:** no changes in this phase.

### Phase 30: implementation and offline qualification

- **Chunk `models-and-validation`:**
  - the overlay
  - regenerated `generated.rs`, `schema_graph.json`, `semantics.json` and `native-models.json`
  - typed round-trip and validation tests: tags value, null, omitted, invalid key, non-string value, response tolerance, and frozen-root fixtures unchanged
- **Chunk `clients-and-wire`:**
  - client wiring to the new roots
  - `list_with_options`, list `tags` and `mcp_tags_filter`
  - wire tests for exact bodies and queries on all four writes and both lists, pre-auth rejection and unchanged frozen `list`
  - the operations ledger binding
  - updates to `docs/GATEWAY_CONTROL.md` and `CHANGELOG.md`
- **Checks:** the cumulative phase-27 offline command set, including both compilers, every feature set, contracts, generation and exports, plus the phase-29 checks and the new tests.

### Phase 30 end-to-end validation

A bounded live workflow on the designated tenant (goshawks, SCM OAuth) runs through the `mcp_tags_live` example. It creates a uniquely named disposable MCP integration and MCP server with tags, then:

1. GET, verifying the tags
2. list with a `tags` filter, expecting the fixture to be found
3. update the tags to a new value, then to `null`
4. delete the server and the integration

Cleanup is recorded. A sanitized receipt is written to `docs/checks/phase-30-mcp-tags-live-attempt-NN.json`. Service rejections, for example the PANW deployment lacking tags support, are recorded as prerequisites or limitations, never as passes.

### Phase 31: 0.2.5 release and delivery

- **Chunk `release-readiness`:**
  - bump the version to 0.2.5
  - `CHANGELOG`, `README`, `RELEASE` and `LIVE_ACCEPTANCE` documents
  - clean extracted-archive package checks on 1.89.0 and 1.99.0
  - the `check-access-release.py --phase 31 --version 0.2.5` readiness receipt
- **Chunk `remote-delivery`:**
  1. Publish with the owner-controlled Cargo credential.
  2. Verify the registry archive.
  3. Push the annotated tag `v0.2.5` and the commits to `origin`.
  4. Wait for terminal CI.
- **Chunk `docs-site`:** update the SDK's Docusaurus site, `docs-site/` on the `docs/scm-github-pages` branch of the public GitHub mirror (deployed to `https://cdot65.github.io/scm-sdk-rs/` by its `docs-deploy` workflow). Merge the released `main` into that branch, update the release-derived version references and the MCP tags guide content, and build the site locally. Then push the branch so its workflow deploys the site. The work happens in a separate worktree at `.local/docs-site-worktree`.

## Acceptance

Each chunk and each phase needs all of the following:

- passing checks for the reviewed source fingerprint
- a preserved Codex prompt, raw output and parsed record
- a score of at least 9/10
- no open blocker or high finding

Phase 31 additionally requires:

- verified registry bytes for 0.2.5
- the remote tag
- terminal CI
- a successful docs-site build
