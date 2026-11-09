---
id: cron-batch
name: cron-batch
description: CRON_BATCH_V1 — batched event-driven cron ticks (500 ms debounce, max 100 actions/cycle, overflow to T27M dead letter, T27L emit). Load before any scheduled Queen/cron tick, merge sweep, or roadmap sync that would otherwise poll or spawn per event.
argument-hint: "[tick-name]"
---

# /cron-batch — one tick = one bounded batch job

<!-- only-t27-rule -->
> **Only t27.** Numbers and gates live in `.t27`; this skill is how agents run a tick.

Owner order **CRON_BATCH_V1** (2026-11-10): do **not** wake an agent per event or tight-poll. One cron tick collects pending work, debounces, runs **one** bounded job, emits **T27L**, then stops.

## SSOT map

| Spec | What it pins |
|------|----------------|
| `specs/tri/trigit/roadmap_dashboard_sync.t27` | `RD_DEBOUNCE_MS = 500`, `RD_DEBOUNCE_MAX_MS = 5000`, `rd_debounce_fire`; `RD_CYCLE_MAX = 100`, `rd_cycle_now` / `rd_cycle_overflow`; overflow → `TM_DL_OVERFLOW` |
| `specs/t27os/t27m.t27` | Durable dead letter (`tm_dead_letter`); replay needs approval |
| `specs/t27os/t27l.t27` | `message_deadlettered` = action **72**; emit one line per tick outcome |
| `specs/claw/merge_verdict.t27` | CI reads on `E_CHECK_DONE` / `E_PUSH` / `E_MASTER_MOVED` / `E_ACK` — **not** `E_TIMER` |
| `specs/claw/model_route.t27` + `work-cheap` | Local/deterministic first, then free tier, paid only with scope |
| `.claude/skills/merge-queue/SKILL.md` | Squash-merge only on merge-verdict `MERGE_NOW` + `A_MERGE` |

## Root rules

1. **No tight polling** — no `sleep` loops waiting on `gh pr checks`; use `cursor-subscriptions` (CI, PR, timer as wake-only).
2. **No per-event spawn** — one tick = one batch; do not launch N subagents for N webhooks.
3. **No silent paid fallback** — escalate model only per `model_route.t27`.
4. **No unbounded sync** — at most **100** actions this cycle (`rd_cycle_now`); plan overflow honestly.

## Tick workflow

```text
Cron / notification wake
  → collect pending events into one batch (PR CI done, master moved, labeled, etc.)
  → debounce: fire when quiet_ms >= 500 OR oldest_ms >= 5000 (rd_debounce_fire)
  → classify route: local checks → free model → paid (only if approved)
  → run ONE bounded job (<= 100 actions)
  → merge-queue verdict lines for merge candidates (advise/merge/hold/stop)
  → emit T27L (surface per host card; deadletter 72 if overflow)
  → end turn (subscriptions hold the next wake)
```

## Debounce (500 ms)

Apply to **event accumulation**, not to “ignore work”:

- While events keep arriving, **hold** the batch.
- When the stream is quiet **500 ms**, or the oldest pending item is **5 s**, **fire once**.

Same semantics as `rd_debounce_fire(pending, quiet_ms, oldest_ms)` in `roadmap_dashboard_sync.t27`.

## Bounded cycle (100 actions)

Count **actions**, not API calls or PRs alone. Examples: one `gh pr view` + check classification ≈ 2 actions; one squash-merge ≈ 1 action.

```text
planned = <count after classify>
run_now = min(planned, 100)
overflow = planned - run_now  → dead letter (TM_DL_OVERFLOW), T27L action 72
```

Replay overflow only with owner/policy (`t27m` `dead_letter_replay_needs_approval`).

## Routing (per tick)

| Tier | When |
|------|------|
| **Local / deterministic** | Spec tests, `rd_*` pure functions, `gh` JSON that does not need LLM |
| **Free** | Pane / Composer 2.5 / NIM free row — classify verdicts, short reports |
| **Paid** | Only after failed free attempt or owner word; never for empty ticks |

## Merge sweeps use this skill

A “merge all open PRs” goal run is **one cron tick**:

1. Subscribe (or single wake) — do not re-scan all 55 PRs every 60 s in a loop.
2. Batch PRs whose **CI completed** since last tick (from notification or explicit list).
3. Debounce 500 ms if multiple notifications arrived together.
4. Run merge-queue on **at most 100** PR actions; overflow → dead letter + report.
5. Emit report block below; **no merge** without `merge-queue` verdict.

## End-of-tick report (paste verbatim)

```text
CRON_TICK_STATUS: <batched|empty|overflow>
BATCH_SIZE: <pending collected>
DEBOUNCE_STATUS: <held|fired quiet_ms=… oldest_ms=…>
ROUTE_STATUS: <local|free|paid>
ACTIONS_RUN: <n> (max 100)
OVERFLOW_STATUS: <0|count → dead letter>
BLOCKERS: <comma list>
```

## Traps

- **Timer-only sweeps** without new events waste tokens — prefer `subscribe_github_ci` on branches that just finished.
- **Full open-PR scan (500+)** every turn violates bounded cycle — sample or event-filter first.
- **Pareto / Queen crons** (`specs/crons/*.t27`) are cards; the **batch law** is still `roadmap_dashboard_sync.t27` + this skill when the host runs a tick.
