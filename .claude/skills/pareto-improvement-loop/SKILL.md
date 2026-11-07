---
name: pareto-improvement-loop
description: Fifteen-minute continuous improvement tick: audit swarm state, Pareto-rank weak points, fix the top one in .t27, verify with receipts, land green, beat the hive feed, never break past work. Auto-fix on, self-heal reverts red changes.
---

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

# Pareto improvement loop

The cron card `specs/crons/t27-pareto-improvement-loop.t27` runs every 15 minutes (HOST launchd, ID `launchd/mac/t27-pareto-improvement-loop`). Every tick executes seven steps in order:

1. **AUDIT** — read the swarm ledger, lanes, PRs, seals, receipts; list anomalies.
2. **PARETO** — rank anomalies by impact x frequency; the top one is this tick's target.
3. **FIX** — write the fix as a `.t27` spec (or extend an existing one), with `test`/`invariant`/`bench`.
4. **VERIFY** — `t27c test` + seal on the branch; real corpus receipt compare vs master.
5. **LAND** — commit with an L1 reference (`Closes|Fixes|Resolves|Refs|Updates #N`), push once per batch, open PR, merge when green.
6. **BEAT** — post the tick's verdict (pass/fail/cost/n/repro footer) to the hive feed.
7. **SELF-HEAL** — if this tick's change red-checks past `ROLLBACK_WINDOW` (3 ticks), revert it before the next tick starts.

## Specs this skill stands on

| Spec | Role |
|------|------|
| `specs/crons/t27-pareto-improvement-loop.t27` | Cron card: the seven steps, Pareto floors, rollback window, verdict codes |
| `specs/acceleration/pareto_weak_points.t27` | Anomaly kinds, impacts, scores, outranking, top-anomaly pick |
| `specs/acceleration/competitor_research.t27` | Competitor records, pass@k/cost comparison, LEAD/PARITY/BEHIND verdict |
| `specs/acceleration/decomposition_plan.t27` | Atomic units with `Closes #N` traceability, per-mille coverage, next-unit pick |
| `specs/acceleration/auto_fix_self_heal.t27` | Deterministic fix per failing check kind, rollback decision, keep/watch/rollback verdict |
| `specs/acceleration/coverage_hundred.t27` | Effective coverage: every spec tested, every change traced, no vacuous tests |
| `specs/tri/commands/pareto_commands.t27` | `tri pareto audit|fix|verify|land|beat|self-heal`, `tri experience save` |

## Rules that keep past work intact

- **Never break a past merge.** `fix_may_start` requires the PR gate ran and no own PR is red.
- **Self-heal:** `should_rollback` reverts a change that red-checks past the 3-tick window.
- **Lane hygiene:** only own released lanes are removed, at most 8 per tick (reuses `specs/tri/worktrees/reap.t27`).
- **L1 traceability:** every commit body carries a local `Closes|Fixes|Resolves|Refs|Updates #N`.
- **Foreign-edits budget:** net-negative (added ≤ deleted), ≤40 lines/file, ≤80/PR; no `--no-verify`/`LEFTHOOK=0`.
- **ASCII only** (L3): `bootstrap/build.rs` fails the build on any Cyrillic under `specs/`.

## New `tri` commands

- `tri pareto audit` — list anomalies ranked by impact x frequency
- `tri pareto fix <n>` — write the `.t27` fix for anomaly n, with test/invariant/bench
- `tri pareto verify` — `t27c test` + seal + corpus receipt compare vs master
- `tri pareto land` — commit with L1 ref, push once per batch, open PR, merge when green
- `tri pareto beat` — post the tick verdict to the hive feed
- `tri pareto self-heal` — revert this tick's change if any check red past the window
- `tri experience save` — save a learned lesson into a skill's Traps section (deduplicated)

## Pareto 80/20 acceleration variants

Three variants live in `specs/acceleration/`:
- `pareto_variant_hot_seal.t27` — Variant A: hot-path seal reuse with prefetch (~28% avg cost vs cold baseline)
- `pareto_variant_parallel_tests.t27` — Variant B: parallel test shards by use-graph level (up to 8x throughput)
- `pareto_variant_incremental_gen.t27` — Variant C: incremental gen cache with content-addressed artifacts (~90% gen time saved at 80% hit rate)

## No foreign edits

No `.rs`, `.py`, `.ts`, `.js`, `.sh`, `.zig`, `.c`, `.go`, `.v`, `.yml`, `.toml`, Dockerfile or Makefile is added by this skill. All logic is in `.t27`.
