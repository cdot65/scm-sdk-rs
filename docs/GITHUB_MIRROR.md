# GitHub mirror and Rust documentation

## Repository roles and current branch split

Forgejo at `ssh://git@git-ssh.cdot.io/cdot/scm-sdk-rs.git` remains the source of
truth for SDK development, review, and release delivery. The intended public
GitHub mirror is [cdot65/scm-sdk-rs](https://github.com/cdot65/scm-sdk-rs).
Creating this workflow does not establish that the mirror or Pages deployment
has been configured or successfully published.

At preparation time, Forgejo `main` points to
`0a213fe79c7f0c5758f05f53d04a5c577a4f3f4e`, the 0.2.2 source state. The published
0.2.3 source and its later work have not yet been merged into that branch. The
initial GitHub `main` mirror must preserve the actual Forgejo `main` commit;
it must not present 0.2.3 as already merged or fully accepted remotely.

The dedicated `docs/scm-github-pages` branch carries the Rust documentation site
and is based on the later 0.2.3 development history. Pages deploys this branch,
not mirrored `main`. This is an explicit documentation branch split, not a
claim that the Forgejo release-delivery and phase-completion gates have passed.
Record the actual remote branch commits and successful Pages run when setup is
complete. After the source branches are merged through the primary repository's
review process, the maintainer can reconcile the documentation branch and its
deployment trigger in a reviewed change.

## 0.2.4 documentation update

The current documentation update integrates accepted SDK release/CI history
`56a88d25f72951b8569193f6252efefae7053ea3`, including published source
`6f18529a47cc273feca1000af6e4fc0d96da757e`. The public main mirror carries that
accepted history. The documentation branch retains its existing Pages workflow,
project base path, user-supplied logo and theme assets.

The registry archive, annotated tags, both package compilers and 49 terminal CI
contexts are verified by the maintained delivery guard in the separate primary
release checkouts. Current phase-28 metadata and supporting reference artifacts
are copied exactly from governance commit
`612864994ee7efbc36d0dd3edbcb60a16fc6e7aa`; its implementation and Git history are
not integrated through this documentation update. The SDK phase remains
`in_progress`, with its readiness recovery authorization and next full-phase
review pending. Documentation review neither completes that phase nor grants
a new SDK review budget.

`docs-site/scripts/review-release-024.py` runs the independent documentation
release-update scope against those primary release proofs, preserving the prior
Pages and logo evidence. It records the actual source manifest, checks, prompt
and Claude Code result, with at most five attempts. A passing readiness review
does not establish that this documentation update has deployed; record its
subsequent Pages run and verify the published URL separately.

## Documentation content

The site lives entirely in `docs-site/` and uses Docusaurus 3.10.2. It follows the
TypeScript documentation's visual style while presenting Rust APIs, Cargo
installation, Rust examples, feature flags, and Rust-specific behavior.
Strata Cloud Manager is the product umbrella. Prisma AIRS is implemented;
NGFW and SASE are planned namespaces, not currently supported SDK functionality.
Product roadmap pages must retain that distinction.

The deployment workflow invokes `npm ci` and `npm run build` inside `docs-site/`.
Its own `package-lock.json` supplies the dependency graph, and `.node-version`
pins Node 24. The artifact is `docs-site/build`. It does not depend on a
repository-root npm package or a TypeScript `docs:api` generation command.
Docusaurus must use the project-site base path `/scm-sdk-rs/` for the intended
URL [cdot65.github.io/scm-sdk-rs/](https://cdot65.github.io/scm-sdk-rs/).
That URL is the deployment target, not evidence of a completed deployment.

## Initial Pages setup

A maintainer configures the public GitHub repository and selects **GitHub
Actions** as the Pages build source. The `github-pages` environment must permit
`docs/scm-github-pages`; do not disable unrelated environment protections.
Publish the reviewed documentation branch, then inspect its workflow run and
actual deployed URL before reporting the site as live. A manual dispatch must
select the same documentation branch.

The workflow runs only on GitHub, only in `cdot65/scm-sdk-rs`, and only for the
selected documentation branch. Push triggers are restricted to site and workflow
changes. A dispatch supports an intentional rebuild without source changes.
The build job has read-only repository permission. Only deployment receives
`pages: write` and `id-token: write` for GitHub's Pages OIDC exchange. Checkout
persists no Git credential. Deployment requires neither a Cargo publishing token
nor a tenant credential. The shared `github-pages` concurrency group serializes
runs without cancelling a deployment already in progress.

Official actions are pinned to immutable commits resolved from their GitHub tag
references on 2026-10-20: checkout v4.3.1, setup-node v4.4.0,
upload-pages-artifact v3.0.1, and deploy-pages v4.0.5. Review action and Node updates
alongside the site lockfile; floating action tags are not used in the workflow.

## Controlled synchronization

Use a separate, explicitly named GitHub remote while keeping the authorized
Forgejo remote intact. Authenticate through the maintainer's existing external
credential mechanism. Never put tokens in remote URLs, repository files, site
configuration, or workflow secrets when GitHub's built-in token suffices.

For each synchronization:

1. Fetch the intended source branch from Forgejo and inspect its exact commit.
2. Fetch the corresponding GitHub branch and compare its history and changes.
3. Push only the explicitly reviewed branch ref with an ordinary fast-forward
   push. If histories diverge, reconcile them through review; do not force an
   overwrite or delete unrelated GitHub refs.
4. Synchronize release tags individually only after checking their exact tag
   object and peeled source commit against the release evidence.
5. Record the source and destination commits. For documentation updates, also
   record the Pages workflow result and verify the published site.

Do not use `git push --mirror`: it can overwrite or delete refs outside the
intended synchronization. The dedicated documentation branch is maintained
explicitly while mirrored `main` follows the actual Forgejo branch. No automatic
bidirectional sync or blanket tag synchronization is configured by this workflow.

## Guide ownership and maintenance

Repository guides under `docs/` are authoritative. `docs-site/DESIGN_SOURCE.json`
lists every derived site page and its source under `derivedFrom` (keys are relative
to `docs-site/`). Update both in the same change: retain the site's front matter,
remove the duplicate first heading where applicable, and translate repository
links to the matching site routes or source permalinks. Review both copies for
semantic agreement. Getting-started, product overview, examples, API reference,
and release-summary pages are authored in the site itself and checked against
Rust source and CHANGELOG.md. The site is not regenerated implicitly by npm.

The repository-root `.node-version` pins contract/oracle tooling; the separate
`docs-site/.node-version` pins the Pages build. The workflow installs clean locked
dependencies, checks site TypeScript, and builds with broken links treated as errors.
The preserved stylesheet loads Inter and JetBrains Mono from Google Fonts; this
inherited external font dependency is retained to match the requested source style.

Actual setup and deployment receipts are retained under
`docs/reviews/github-pages/`; preparation statements above describe the setup
sequence, while those receipts identify completed actions and verified commits.
The existing Forgejo tags through 0.2.2 are mirrored individually. A local 0.2.3
release tag is not advertised as mirrored because it is absent from Forgejo.
