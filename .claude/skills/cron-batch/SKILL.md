---
name: cron-batch
description: >-
  CRON_BATCH_V1: one cron/agent turn collects pending events, debounces 500 ms,
  runs one bounded job (max 100 actions), overflow to dead-letter. Event-driven
  via subscriptions; no tight polling. Load for loop-pareto-night and cloud ticks.
---

# CRON_BATCH_V1

**SSOT:** `specs/tri/trigit/roadmap_dashboard_sync.t27` (`RD_DEBOUNCE_MS`, `RD_CYCLE_MAX`, `rd_debounce_fire`, `rd_cycle_overflow`, event `message_deadlettered` **72**).

**Pareto cron wiring:** `specs/crons/t27-pareto-improvement-loop.t27` (`cron_tick_may_run`, `cron_actions_this_cycle`, `cron_actions_overflow`).

**Wake sources:** `cursor-subscriptions` (CI branch, PR, timer `loop-pareto-night`) — skill `subscribe` under `$0/subscribe/SKILL.md`. Deliveries coalesce; on wake, **one** bounded job, not N poll turns.

## One turn

1. Collect pending events since last turn (timer + CI + PR notifications).
2. If `pending == 0` → **do not** call `gh` or wake for status; end turn (SKIP).
3. If `quiet_ms >= 500` or oldest pending exceeded max wait → **one** batch.
4. Run deterministic checks first (`t27c seal`, compact `gh` JSON once if CI event).
5. Cap actions at **100**; overflow → dead-letter / next cycle (no silent drop).
6. Merge only via `merge-queue` / `merge_verdict.t27` (`M\t` line); seven conditions, not ad hoc.

## Report footer (when batch ran)

```text
CRON_TICK_STATUS: PASS|HOLD|SKIP
BATCH_SIZE:
DEBOUNCE_STATUS: fired|waiting
ROUTE_STATUS: local|free|paid
ACTIONS_RUN:
OVERFLOW_STATUS: 0|n
BLOCKERS:
```

## Traps

- Repeated HOLD turns with identical `gh` output are **policy violations** — use subscriptions.
- Empty ticks must not burn tokens: `cron_tick_may_run(0, …)` is false.
- Cheaper never means skip gates or merge without verdict.
