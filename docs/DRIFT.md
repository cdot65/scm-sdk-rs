# Upstream drift monitoring

Two tools compare the SDK's pinned upstream specifications with any newer upstream
commit. Both report drift and never adopt it. To adopt a change, pin it in a new
reviewed phase.

| Command | What it does | Network |
|---|---|---|
| `--check` | Verifies the pinned bytes and the ledger, and runs a self-compare | Offline |
| `compare --candidate DIR` | Classifies a fetched tree against the pin | Offline |
| `refresh --commit SHA` | Fetches a tree at an exact commit into `.local/` | Only this command goes online |

`compare` exits with:
- **0** for unchanged, byte-only or annotation-only changes;
- **2** when review is required;
- **1** when a change cannot be classified or tampering is detected.

## pan.dev (Prisma AIRS): `tools/airs_drift.py`

Repository: https://github.com/PaloAltoNetworks/pan.dev. Pinned commit:
`582e30543bf2adfd8b0ee457170d22b5c380dfbe`. Ledger: `contracts/airs-drift.json`.

Whole subtrees are watched and hash-verified, so new, removed or changed files are
detected:

- `openapi-specs/prisma-airs`
- `openapi-specs/prisma-airs-redteam`
- `openapi-specs/prisma-airs-model-security`

These specifications are classified change by change:

| Path | Version |
|---|---|
| `openapi-specs/prisma-airs/management/mgmt-service_latest.yaml` | 0.0.0 |
| `openapi-specs/prisma-airs/scan/scan-service_latest.yaml` | 0.0.0 |
| `openapi-specs/prisma-airs-redteam/management/mp-openapi.yaml` | 0.86.0 |
| `openapi-specs/prisma-airs-redteam/data-plane/dp-openapi.yaml` | 0.93.0 |
| `openapi-specs/prisma-airs-redteam/network-broker/AIRS-Red-Teaming-Network-Broker.yaml` | 1.0.0 |
| `openapi-specs/prisma-airs-model-security/management/mgmt-plane.yml` | 0.1.0 |
| `openapi-specs/prisma-airs-model-security/dataplane/data-plane.yml` | 0.1.0 |

## Portkey AI Gateway: `tools/portkey_drift.py`

Repository: https://github.com/Portkey-AI/openapi. Pinned commit:
`8e5da0e756bc49ee3e82ca13df693b39fa541627`, the SDK's vendored pin in
`contracts/vendor/portkey-mcp-tags/8e5da0e`. Ledger: `contracts/portkey-drift.json`.

The watched paths are all at the repository root:

- `openapi.yaml` is classified with the same fail-closed semantic classifier as
  pan.dev. Operation code samples (`x-code-samples` and `x-codeSamples`) are
  reported as `code_samples_changed` annotations. Any other unclassified keyword
  fails.
- `LICENSE`: any change is reported as `license_changed`.
- The root file listing: a top-level file that is added, removed or changed is
  reported, so a new specification file is never missed.

## Latest checks (2026-11-06)

- pan.dev HEAD `b5ef166` (2026-11-05): **unchanged** for all seven AIRS
  specifications (`docs/checks/airs-drift/`).
- Portkey-AI/openapi HEAD `54aeff6` (2026-11-06): **annotation only**. One
  code-sample change in `/ocr` (Python snippet fix); no API change
  (`docs/checks/portkey-drift/`).
