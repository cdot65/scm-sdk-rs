# Issue tracker: Forgejo

Issues and specs for this repo live in the Forgejo repository `cdot/scm-sdk-rs` at
`https://git.cdot.io`. GitHub (`cdot65/scm-sdk-rs`) is a push mirror only; do not file
issues there. Use the `tea` CLI (login `cdot`, the default) for all operations; skills
written for `gh` must use the equivalents below. The global `forgejo-tea` skill has the
full command reference.

Inside this clone `tea` resolves the repo from `origin`. Elsewhere pass `--repo cdot/scm-sdk-rs`.

## Conventions

- **Create an issue**: `tea issues create -t "..." -d "..."`. For multi-line bodies write the
  body to a file and pass `--description-file body.md` (`-` reads stdin).
- **Read an issue**: `tea issue <number>`; comments: `tea comments ls <number> -o json`.
- **List issues**: `tea issues ls --state open -o json --fields index,title,state,labels,assignees,body,comments`,
  filtered with `-L <label>` and `--state`.
- **Make an issue a sub-issue of a parent**: Forgejo has no sub-issues. Put `Part of #<parent>`
  at the top of the child body and add `- [ ] #<child>` to a task list in the parent body.
- **Comment on an issue**: `tea comments add <number> "..."`.
- **Apply / remove labels**: `tea issues edit <number> --add-labels "..."` / `--remove-labels "..."`
  (label names, comma-separated).
- **Close**: `tea comments add <number> "..."` then `tea issues close <number>`.

## Pull requests as a triage surface

**PRs as a request surface: no.** _(Set to `yes` if this repo treats external PRs as feature
requests; `/triage` reads this flag.)_

Forgejo shares one number space across issues and PRs: `tea issue <n>` on a PR number returns
the PR; `tea issues ls -K all` lists both. PR operations: `tea pull <n>`, `tea pulls checkout <n>`,
`tea pulls review-comments <n>`, `tea pulls edit <n> --add-labels ...`, `tea pulls close <n>`.

## When a skill says "publish to the issue tracker"

Create a Forgejo issue as above.

## When a skill says "fetch the relevant ticket"

Read it as in **Read an issue** above.

## Wayfinding operations

Used by `/wayfinder`. The **map** is a single issue with **child** issues as tickets.

- **Map**: one issue labelled `wayfinder:map` holding the Notes / Decisions-so-far / Fog body:
  `tea issues create -L wayfinder:map ...` (create the label first if absent).
- **Child ticket**: an issue with `Part of #<map>` at the top of its body and a task-list entry
  in the map. Labels: `wayfinder:<type>` (`research`/`prototype`/`grilling`/`task`). Once
  claimed, the ticket is assigned to the driving dev.
- **Blocking**: Forgejo's native issue dependencies, via the API:
  `tea api -X POST 'repos/{owner}/{repo}/issues/<child>/dependencies' -F index=<blocker-number>`;
  read with `tea api 'repos/{owner}/{repo}/issues/<child>/dependencies'`. A ticket is unblocked
  when every blocker is closed.
- **Frontier query**: list the map's open children (`tea issues ls --state open -o json`, scoped
  to the map's task list), drop any with an open blocker or an assignee; first in map order wins.
- **Claim**: `tea issues edit <n> --add-assignees cdot`, the session's first write.
- **Resolve**: `tea comments add <n> "<answer>"`, then `tea issues close <n>`, then append a
  context pointer to the map's Decisions-so-far.
