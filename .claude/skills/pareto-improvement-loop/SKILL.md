---
name: pareto-improvement-loop
description: Fifteen-minute continuous improvement tick: audit swarm state, Pareto-rank weak points, fix the top one in .t27, verify with receipts, land green, beat the hive feed, never break past work. A red typecheck rolls that lane back to its pre-tick HEAD and leaves older commits in place. Where this page and the cron spec disagree, the spec wins.
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
| `specs/tri/commands/pareto_commands.t27` | `tri pareto audit|fix|verify|land|beat|self-heal` (proposed, not built yet), `tri experience save` |

## Rules that keep past work intact

The law is `self_recovery_action` and `prior_commits_untouched` in `specs/crons/t27-pareto-improvement-loop.t27`. Where this skill and that spec disagree, the spec wins.

- **Own files only.** A tick commits only the task files it named. `tick_commit_allowed` calls `oc_commit_ok`; the body is not copied.
- **Red typecheck.** Roll this lane back to the HEAD recorded before the tick wrote (`SR_ROLLBACK_LANE`). If that HEAD was not recorded, move no ref (`SR_REFUSE`). Do not guess an older commit.
- **Prior commits stay.** The rollback target is that pre-tick HEAD. Commits older than it stay. This is not a reset of master and not a force-push.
- **Window.** `oc_window_open` stays false. A tick that asks to open the window is refused.
- **Never break a past merge.** `fix_may_start` calls `pick_may_run`: the PR gate ran and no own PR is red.
- **Later window.** `should_rollback` in `specs/acceleration/auto_fix_self_heal.t27` still reverts a landed change that red-checks past `ROLLBACK_WINDOW` (3 ticks). That is a second gate. It does not replace the pre-tick rollback above.
- **Lane hygiene:** only own released lanes are removed, at most 8 per tick (reuses `specs/tri/worktrees/reap.t27`).
- **L1 traceability:** every commit body carries a local `Closes|Fixes|Resolves|Refs|Updates #N`.
- **Foreign-edits budget:** net-negative (added ≤ deleted), ≤40 lines/file, ≤80/PR; no `--no-verify`/`LEFTHOOK=0`.
- **ASCII only** (L3): `bootstrap/build.rs` fails the build on any Cyrillic under `specs/`.

Lesson, 2026-11-10: a failed tick that resets the lane to an ancestor older than the pre-tick HEAD, or that resets master, drops green commits the tick did not make. Record the lane HEAD before writing. On a red typecheck, `git reset --hard` that recorded commit on this lane only. Do not force-push. Do not open the window.

## Free-first routing (FREEFIRSTSUBAGENTMODE)

The name is a bundle, not a second law. Load at tick start: `.claude/skills/work-cheap/SKILL.md`, `model-routing`, `swarm-economy`. The checked rules are `specs/claw/cost_policy.t27` and `specs/claw/model_route.t27`. Claude tiers stay in `model_route.t27`; do not copy them here. `cursor_route.t27` is not in the tree yet; on Cursor Cloud follow `work-cheap` (cheapest tier that passes the machine gate; Fast and 500k are not defaults).

Order each tick: (1) local script — `gh`, `git`, `t27c check`, `t27c seal`, `python3 tools/*` with no LLM; (2) free bounded helper only when a file must change and the gate is not script-readable; (3) paid model only after two failures on the same bounded task or on owner approval for P0 security, complex integration, or final acceptance. Max two helpers at once; one task, one artifact, eight turns; no nested delegation. Merge, push to master, publish, window shell, and paid API stay gated (`specs/t27os/orchestrator.t27`).

End each tick report with one routing ledger row (plain text): `task | cost_route | data_route | capability_route | model | turns | artifact | gate_proof`. Example: `reseal cron seal | local | repo | lane commit ok | none | 0 | t27c seal exit 0 | tools/check_seal_coverage.py later on CI`.

Lesson, 2026-11-10 (Asia/Bangkok): resealing `crons_CronParetoImprovementLoop.json` with `t27c seal` plus `tri seals sync-twins` beat spawning a subagent to re-read CI logs.

## New `tri` commands (proposed, not built yet)

The `tri pareto` contract is `specs/tri/commands/pareto_commands.t27`; no `tri` subcommand runs it yet.

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

## Traps

- 2026-11-10 (Asia/Bangkok): PR #13541 stayed green on its cron seal while CI failed on `queen_dash` (#13506) and sieve dupes (#13505). Do not reseal or dedupe those paths from the Pareto lane. Land the sibling PRs on master, then `git merge origin/master` on the Pareto branch and re-run `gh pr checks` — no cherry-pick of their files.
- Before editing for a red check, use `pareto_fix_may_start` in `specs/crons/t27-pareto-improvement-loop.t27` (wraps `gate_patch_allowed` + `fix_may_start`); if it is false, stop and wait for the owning lane.
- At STEP AUDIT, `inherited_repo_gate_red(duplicate_bodies_red, seal_coverage_red)` names the two master-debt checks this lane does not patch; pass flags from `gh pr checks`, not from memory.
- Mass "required check pending" on PRs is a gate class, not per-PR whack-a-mole. Law: `specs/ci/pr_ready_aggregate.t27` (aggregate `pr-ready`, always terminal on latest SHA). Host: `.github/workflows/pr-ready.yml` after owner labels foreign workflow work; until then fix master debt (#13505 dupes, #13506 queen_dash) per sibling lanes.
- Before `git push`, call `batch_push_may_run(skill_only_tick, inherited_repo_gate_red, own_task_path)` in the same spec; when it is false, verify locally and update the PR body only.
- Before a second `git merge origin/master` in the same CI window, call `merge_master_may_run(required_checks_pending, master_ahead)`; when required checks are still pending after the last push, wait.

## No foreign edits

No `.rs`, `.py`, `.ts`, `.js`, `.sh`, `.zig`, `.c`, `.go`, `.v`, `.yml`, `.toml`, Dockerfile or Makefile is added by this skill. All logic is in `.t27`.
