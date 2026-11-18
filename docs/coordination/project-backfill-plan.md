# Projects v2 backfill plan (T27_GITHUB_AUTOMATION_V1)

Rule: no bulk backfill. Auto-add is not retroactive; historical items are
added only after triage, in batches of 25, on owner approval.

## Current state (measured 2026-11-10)

- 2020 open issues. The board starts empty; the built-in auto-add workflow
  will cover every newly created or updated issue/PR matching `is:open`.
- 1 open milestone: M0_PROMOTION_KERNEL_SPRINT (10 issues) -- the natural
  first batch: it is already scoped and ordered.

## Batch plan (each batch = 25 items max, one PR-free manual pass)

| Batch | Source | Filter | Est. size | Gate |
|---|---|---|---|---|
| 1 | M0 sprint issues | `is:open milestone:M0_PROMOTION_KERNEL_SPRINT` | 10 | owner approval |
| 2 | Recently active | `is:open sort:updated-desc` | 25 | owner approval |
| 3 | Blocked/hardware | `is:open label:blocked,label:status/blocked-hardware` | <=25 | owner approval |
| 4+ | Ring/epic waves | by `ring` / `epic` label, one wave per batch | <=25 | owner approval |

## Field mapping on backfill

- `Status`: `Triage` default; `Blocked` if `blocked` label present;
  `Done` never set retroactively (only via PR merge automation).
- `Priority`: from `P0`/`P1`/`priority/*` labels, else `P2`.
- `Area`: `area/compiler`|`compiler` -> Compiler; `t27os` -> T27OS;
  `area/rtl`|`numeric`|ring/FPGA work -> FPGA; `docs`|`ux-docs-task` -> Docs;
  `area/infra`|`ci` -> Infra; else `Community`.
- Milestone: linked field, already on the item.

## What is NOT done

- No bulk-close, bulk-reopen, bulk-relabel, no retro auto-add, no mass edit
  of milestone/assignee. Every batch waits for explicit owner approval.
