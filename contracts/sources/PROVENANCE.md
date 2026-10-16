# Frozen contract provenance

The parity baseline is TypeScript SDK 0.34.0 at commit
`4a80dbdb3dc14365e1c25d838ac0bd139485d48c`. Files were read directly from
that commit's Git objects, without modifying its repository or trusting a working
checkout. `typescript/commit.txt` and the complete `typescript/git-tree.json`
allow the verifier to reconstruct the root Git tree and bind bundled baseline
files to that immutable commit. The manifest records all 145 `src/` files, their
Git blob identities, byte counts, and SHA-256 hashes.

All files in the baseline's `test/openapi/` tree are preserved. Its JSON fixtures
are generated from specifications independently of SDK validators, subject to
the explicitly recorded corrections. This independence does not imply that the
upstream specifications are complete or that a deployed service accepts a
request. The original generator and correction sources are preserved as
Apache-2.0 reference material; they are not the future Rust generator.

Additional TypeScript tests preserve synthetic request validation, streaming,
multipart, analytics, and related regression fixtures. These are SDK-authored
tests, not independent evidence of the service contract. The four separately
classified response captures retain their original provenance: three use
sanitized or synthetic observed values, while model pricing was captured from a
public endpoint without credentials. Dummy credential literals in tests do not
represent usable credentials. Tenant registrations and credential files are not
included.

## Exact historical specification recovery

The seven AIRS specifications and the legacy Portkey specification are bundled
unchanged. Each selected repository revision is a historical file revision whose
bytes exactly match the SHA-256 recorded in the frozen fixtures. These are
recovered matching revisions, not claims about the precise checkout revision
originally used when those fixtures were generated. Their Git blob identities,
upstream paths, repository URLs, revision commits, and licenses at the selected
revisions are recorded in `contracts/manifest.json`.

In particular, the Red Team fixtures require July 28 revisions, and the legacy
Portkey fixtures require its September 4 revision. Substituting the current
checkout bytes would silently change the parity baseline. The corresponding MIT
notices are retained, including Portkey's original OpenAI copyright notice.

## Official Gateway source is a separate, excluded input

The TypeScript baseline's official alignment document identifies
`PaloAltoNetworks/openapi` commit
`2931d49bc38e30793d5923d22125f5f953d141d0`, `openapi.yaml` SHA-256
`567f53fd7399c4cf87d4fc5e32a8dd5a25765113f698d82de6e538adf4213978`, and
Git blob `e4866136d7394007a2d4bf88799ac30a7cdc0df4`.

The pinned repository tree contains no LICENSE, its repository license endpoint
returned HTTP 404, and the document has no `info.license` declaration. Its YAML
is therefore excluded from redistribution and generation. The manifest preserves
the public URL and verified identities, without assigning the license of
another repository. The legacy Portkey source must never be relabeled as this
official source. Rust parity can use the existing licensed TypeScript models and
fixtures; including the official YAML or deriving distributable artifacts from
it requires an identified license first.

`python3 tools/verify-contracts.py` verifies the bundled corpus offline.
`--baseline .local/typescript` additionally checks an extracted source tree.
`--fetch-excluded` verifies official Gateway bytes directly from the recorded
public URL, in memory, without writing or distributing them. Neither operation
establishes live service acceptance.
