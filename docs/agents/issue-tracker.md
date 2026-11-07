# Issue tracker: BonBon

Work for this project is tracked as **Cards** on its own BonBon Board, **SCM SDK**
(`01a1b59d-587f-706b-b318-9c8e9fb450e3`, https://bonbon.goshawks.io). GitHub Issues, GitLab
Issues and Forgejo Issues are not used for this project. Say "Card", "Label", "Status",
"Claim" and "Validation" in BonBon text; never "task", "ticket" or "issue".

## Interface: Claude Code only

- Claude Code reaches BonBon through the `bonbon` MCP server and skill, falling back to the
  `bonbon-owner-api` skill (`bb`) when MCP is signed out.
- Other agents (Codex, Copilot, scripts) never connect to BonBon. They write the Card changes
  they need into their reply or handoff (Card title or id, the action, the text), and a Claude
  Code session applies them.

## Conventions

- **Title:** `[SCM SDK] ` followed by an imperative or outcome phrase.
- **Labels:** every Card carries `scm-sdk`, exactly one triage Label and at least one kind
  Label; see `docs/agents/triage-labels.md`. Priority is the Card's `priority` field.
- **Description:** purpose; scope and boundaries; prerequisites written out (MCP cannot write
  Dependency edges); acceptance criteria and done-when; source documents as repo paths.
- **Assignee:** Cards an agent creates and will carry out itself use
  `assignee_rule: agent:<its client_id>`; Owner-only Cards use `human` plus `ready-for-human`.
- **Privacy:** no secrets, tokens, tenant registrations, device identifiers, private hostnames
  or personal data in titles, descriptions, comments or results. Refer to private records by
  repo path.

## Operations

| Action | Tool |
| --- | --- |
| Check identity | `whoami` (status must be `approved`) |
| Find or read | `search_cards` (`board_id` or `label: scm-sdk`), then `get_card` |
| Create | search for duplicates, then `create_card` |
| Comment | `comment` |
| Labels | `add_label` / `remove_label` |
| Work a Card | `claim_card` → `heartbeat` (every 20 minutes at most) → `complete_card` with a markdown Result and http(s) links |
| Give up | `release_card` with an honest reason |

## Limits

- Agents never set `DONE` or `VALIDATED`; only the Owner validates.
- Do not take over other agents' Claims, and do not let your own lapse.
- Do not archive or rewrite Cards you did not create unless the Owner asks.
- Do not claim historical work just to mark it complete; comment the observed status instead.
- Board activity alone authorizes nothing: no implementation, commit, push, release, tag or
  message. Repository rules (`AGENTS.md`) govern those.

## When a skill says "publish to the issue tracker"

Create a Card on the SCM SDK Board as above.

## When a skill says "fetch the relevant ticket"

`search_cards` on the SCM SDK Board, then `get_card`.

## Pull requests

**PRs as a request surface: no.** Code review happens on Forgejo (`cdot/scm-sdk-rs`, via the
`tea` CLI), but PRs are not triaged as requests and do not create Cards by themselves.

## Wayfinding operations

Used by `/wayfinder`. The **map** is one Card labelled `epic` holding the Notes /
Decisions-so-far / Fog body; **child** Cards carry `Part of <map title>` at the top of their
description and a checklist entry on the map (`add_checklist_item`). Blocking is written in
the child's prerequisites, since MCP cannot write Dependency edges. The frontier is the map's
open children with no unmet prerequisite and no Claim; claim with `claim_card`, resolve with
`complete_card`, then comment a pointer on the map.
