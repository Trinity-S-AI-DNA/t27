---
name: cron-batch
description: >-
  CRON_BATCH_V1 + T27L_AUTOMATION_CONTROL_V1: batched event-driven agent turns,
  T27L lifecycle mapping, approved-scope inventory, empty tick = zero model
  calls. No restart, no duplicate schedules. Load for loop-pareto-night and cloud.
---

# CRON_BATCH_V1 + T27L_AUTOMATION_CONTROL_V1

**SSOT:** `specs/tri/trigit/roadmap_dashboard_sync.t27` (`RD_DEBOUNCE_MS`, `RD_CYCLE_MAX`, `rd_debounce_fire`, `rd_cycle_overflow`, event `message_deadlettered` **72**).

**Pareto cron wiring:** `specs/crons/t27-pareto-improvement-loop.t27` (`cron_tick_may_run`, `cron_actions_this_cycle`, `cron_actions_overflow`).

**Wake sources:** `cursor-subscriptions` (CI branch, PR, timer `loop-pareto-night`) — skill `subscribe` under `$0/subscribe/SKILL.md`. Deliveries coalesce; on wake, **one** bounded job, not N poll turns.

## Actionable work only (owner correction)

Wake **only** when the durable queue has pending events **and** debounce fires **and** reconciliation finds `state_changed` or a bounded action to run. If `automation_empty_tick` in `t27-pareto-improvement-loop.t27` is true → **SKIP**: zero model calls, zero status spam, no agent turn.

## Approved-scope inventory (discover before changing)

| Source | Read-only procedure |
|--------|---------------------|
| Cron cards | `specs/crons/*.t27` (`ID`, `HOST`, `SERVICE`, `ENABLED`) |
| This cloud lane | `cursor-subscriptions` `list_subscriptions`; timer `loop-pareto-night` |
| GitHub (repo) | `.github/workflows/*.yml` names only; `gh workflow list` if authorized |
| Mac launchd / DSH | **not** from cloud — mark UNKNOWN; owner handoff via `dev-monitor` / `dsh-startup` |

Per automation record (artifact or table): `automation_id`, `source`, `native_job_id`, `owner`, `trigger`, `status` (`CONFIGURED` \| `SCHEDULED` \| `RUNNING` \| `BLOCKED` \| `COMPLETED` \| `UNKNOWN`), `t27l_connection_status`.

**Rules:** no restart of RUNNING jobs; no duplicate schedule; no foreign cron/workflow/pane edits; unclear ownership → UNKNOWN.

## T27L lifecycle (reuse registered names — never invent action ids)

| Intent | Registered `t27l` action | id |
|--------|--------------------------|-----|
| Batch / cron tick started | `cron_cycle_started` | 79 |
| Batch completed | `cron_cycle_completed` | 80 |
| Empty tick / no work | `schedule_skipped` | 89 |
| Overflow → dead letter | `message_deadlettered` | 72 |
| GH workflow life | `workflow_dispatched` / `workflow_completed` / `workflow_failed` | 113–115 |
| Actor run | `act_run_started` / `act_run_completed` | 119–120 |
| Seal / gate proof | `artifact_verified` | 155 |
| Missing live feed (honest) | `bridge_event_unverified` | 154 |

Names like `automation_registered` have **no** row in `specs/t27os/t27l.t27` yet → **INTEGRATION_GAP** (file issue; do not emit fake ids).

**Writer:** `tl_event_line` + `tl_validate_line` in `specs/t27os/t27l.t27`. **`tri t27l` CLI is NOT BUILT** — cloud agents record INTEGRATION_GAP until a host persists lines.

**Payload:** JSON string only; include `automation_id`, `run_id`, `batch_id`, `source_revision`, `trigger`, `status`, `evidence_locator`; redact secrets (`secret-hygiene`).

**Telemetry ≠ authorization** — green batch does not permit merge, push, or schedule change.

## One turn (when not empty)

1. Collect pending events since last turn (timer + CI + PR notifications).
2. If empty per `automation_empty_tick` → SKIP (no `gh`, no T27L except optional local artifact).
3. If `quiet_ms >= 500` or oldest pending exceeded max wait → **one** bounded batch.
4. Deterministic checks first (`t27c seal`, one compact `gh` JSON if CI event).
5. Cap actions at **100**; overflow → dead-letter / next cycle (no silent drop).
6. Merge only via `merge-queue` / `merge_verdict.t27` (`M\t` line); seven conditions, not ad hoc.
7. Free-first: `.claude/skills/free-first/SKILL.md`; no paid fallback without approval.

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

## Report footer (T27L automation control)

```text
APPROVED_SCOPE:
DISCOVERED_AUTOMATIONS:
RUNNING_AUTOMATIONS:
CONNECTED_TO_T27L:
UNKNOWN_OR_UNCONNECTED:
DUPLICATE_AUTOMATIONS_FOUND:
EMPTY_TICK_MODEL_CALLS:
REAL_T27L_EVENTS_EMITTED:
FREE_ROUTE:
SKILL_PATH: .claude/skills/cron-batch/SKILL.md
TEST_EVIDENCE:
INTEGRATION_GAPS:
BLOCKERS:
NEXT_ACTION:
```

## Traps

- Repeated HOLD turns with identical `gh` output are **policy violations** — use subscriptions.
- Empty ticks: `automation_empty_tick` true → zero model calls; `cron_tick_may_run(0, …)` is false.
- Do not log an event that triggers another log (no feedback loops).
- Cheaper never means skip gates or merge without verdict.
