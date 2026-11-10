# Projects v2 backfill plan (ORDER T27_GITHUB_AUTOMATION_V1, Wave 7)

This is the PLAN for backfilling the Projects v2 board "T27 Delivery".
It is NOT an execution order: no bulk moves of historical issues happen
without owner approval. Measured 2026-11-10 with `gh api`.

## Source data (measured 2026-11-10)

- 2021 open issues (issues only, PRs excluded).
- Milestones (`gh api repos/Trinity-S-AI-DNA/t27/milestones?state=all`):
  - #1 EPOCH-01-HARDEN -- closed (20 closed, 0 open)
  - #2 Phase 3 -- Science Tests -- closed (3 closed)
  - #3 T27_MCP_RUNTIME_M0 -- closed (5 closed)
  - #4 M0_PROMOTION_KERNEL_SPRINT -- the only OPEN milestone, 10 open issues
- Label counts over open issues (top): queen-shaped 454, enhancement 64,
  area/specs 60, spec 50, epic 48, needs-boundary 46, phi-loop 24, P1 24,
  roadmap 23, documentation 14, priority/high 13, P0 8.
- Area labels that exist today: `area/compiler` (6), `area/specs` (60).
- Labels that do NOT exist yet: `area/host`, `area/infra`, `area/rtl`,
  `area/t27os`, `area/fpga`, `area/docs`, `area/community`.

## Regenerating the groups (measured commands)

```sh
# group open issues by milestone
gh api 'repos/Trinity-S-AI-DNA/t27/issues?state=open&per_page=100' --paginate \
  | jq -r '.[] | select(.pull_request == null) | .milestone.title // "none"' \
  | sort | uniq -c | sort -rn

# group open issues by label
gh api 'repos/Trinity-S-AI-DNA/t27/issues?state=open&per_page=100' --paginate \
  | jq -r '.[] | select(.pull_request == null) | .labels[]?.name' \
  | sort | uniq -c | sort -rn

# members of the open milestone
gh api 'repos/Trinity-S-AI-DNA/t27/issues?state=open&milestone=4&per_page=100' \
  | jq -r '.[] | select(.pull_request == null) | "#\(.number) \(.title)"'
```

## Area mapping for the Projects v2 board (future additions)

| Label on issue | Board field "Area" |
|---|---|
| `area/compiler`, `area/host`, `area/specs` | Compiler |
| `area/rtl` | FPGA |
| `area/infra` | Infra |
| (labels `area/t27os`, `area/fpga`, `area/docs`, `area/community` do not exist yet) | -- |

Note: `area/host`, `area/infra` and `area/rtl` do not exist yet either;
until they are created, those Area cells are set from the issue's content
during triage, not from a label.

## Backfill policy

- NO bulk add of historical issues to the "T27 Delivery" board.
- The built-in auto-add workflow covers newly created/updated open issues
  and PRs going forward; it is not retroactive.
- Historical additions happen only after owner approval, in batches of 25
  items maximum, ordered:
  1. members of the current milestone M0_PROMOTION_KERNEL_SPRINT (milestone 4) first;
  2. issues labeled `epic` or `initiative` next;
  3. the rest by open date, descending (newest first).
- Per batch: set Area from the mapping above, Status default `Triage`;
  never set `Done` retroactively (only PR-merge automation sets it).

## Batches

### Batch 1 (measured 2026-11-10): milestone 4 M0_PROMOTION_KERNEL_SPRINT, 10 issues

- #13473 [EPIC] T27_PROMOTION_KERNEL_V5
- #13484 [M0-11] Self-improving loop (bounded, no paid fallback)
- #13483 [M0-10] T27L audit event schema for promotion_kernel
- #13482 [M0-09] Monetization ladder (Free/Low/Mid/High/B2B)
- #13481 [M0-08] Contributor path (good-first-issue + CONTRIBUTING)
- #13476 [M0-03] Widget inventory (16 types -> lessons)
- #13475 [M0-02] Course map: 27 lessons (idea + widget + result)
- #13465 T27_PROMOTION_KERNEL_V5
- #13462 grok crm: T27L audit projection
- #13428 t27os: M0 first screenshot artifact via vf_card_main pipeline

Batch 1 waits for owner approval before any item is added to the board.

### Batch 2+ (planned, not yet executed)

- Batch 2: open issues labeled `epic` (48) / `initiative` (2), first 25 by open date desc.
- Batch 3+: remaining open issues, 25 per batch, open date desc.

## What is NOT done (and stays undone)

- No bulk-close, bulk-reopen, bulk-relabel, no retro auto-add, no mass edit
  of milestone/assignee. Every batch waits for explicit owner approval.
