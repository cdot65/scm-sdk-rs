# Triage Labels

The engineering skills speak in terms of five canonical triage roles. BonBon Labels are
global and use the same strings, so the mapping is the identity. At most one triage Label per
Card.

| Label in mattpocock/skills | Label in BonBon  | Meaning                                                            |
| -------------------------- | ---------------- | ------------------------------------------------------------------ |
| `needs-triage`             | `needs-triage`   | The Owner or a maintainer needs to evaluate it                      |
| `needs-info`               | `needs-info`     | Waiting for information                                             |
| `ready-for-agent`          | `ready-for-agent`| Fully specified; an agent may claim and do it                       |
| `ready-for-human`          | `ready-for-human`| Needs the Owner or a human (also set `assignee_rule: human`)        |
| `wontfix`                  | `wontfix`        | Will not be actioned                                                |

Every Card also carries the project Label `scm-sdk` and at least one kind Label. Reuse these
exact strings; never invent near-duplicates such as `bugs`, `Bug` or `doc`.

## Kind Labels (at least one per Card)

| Label         | Use for                                        |
| ------------- | ---------------------------------------------- |
| `bug`         | Something is broken                            |
| `feature`     | New capability                                 |
| `maintenance` | Upkeep, refactors, dependency bumps            |
| `docs`        | Documentation                                  |
| `testing`     | Tests, qualification, verification             |
| `research`    | Investigation or comparison; no implementation |
| `tooling`     | Build, scripts, developer tooling              |
| `security`    | Security-relevant work                         |
| `ops`         | Deploys, infrastructure, operations            |

Project-specific kind Labels already in use on the Board: `ci`.

## Modifier Labels (optional, any number)

| Label          | Use for                                                        |
| -------------- | -------------------------------------------------------------- |
| `epic`         | A parent Card split into child Cards                           |
| `decision`     | Records or needs an Owner decision                             |
| `owner-action` | Only the Owner can do it (purchases, accounts, physical access)|
| `upstream`     | Tracks or reports an issue in a third-party project            |
| `idea`         | Optional; authorizes nothing                                   |
| `superseded`   | Replaced by another Card (say which in a comment)              |

Priority uses the Card's `priority` field (`low|medium|high|urgent`), not Labels.
