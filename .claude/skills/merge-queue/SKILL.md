---
id: merge-queue
name: merge-queue
description: MERGE_QUEUE_V2 — restore the merge-verdict subagent workflow (six conditions + own_pr, machine-readable verdict, event-driven CI). Load before squash-merging open PRs, running a merge sweep, or spawning a merge judge subagent. Points at specs/claw/merge_verdict.t27 (session verdict) and specs/claw/merge_judge.t27 (claw-merge sweep).
argument-hint: "[pr-number | sweep]"
---

# /merge-queue — merge only when the verdict says MERGE_NOW

<!-- only-t27-rule -->
> **Only t27.** The law is in `.t27`; this skill is the procedure. If they disagree, the spec wins.

Owner order **MERGE_QUEUE_V2** (2026-11-10): do **not** invent a new merger. Restore the pipeline that already shipped as `specs/claw/merge_verdict.t27` (pinned cases #13350, #13351, #13359) and the claw sweep in `specs/claw/merge_judge.t27` (#13357-class checks).

## SSOT map (read these, do not paraphrase)

| Spec | Role |
|------|------|
| `specs/claw/merge_verdict.t27` | **Session merge verdict** — seven conditions, `MERGE_NOW` / `WAIT` / `ESCALATE`, record line `M\tpr\tVERDICT\tact\tblockers` |
| `specs/claw/merge_judge.t27` | **TRI CLAW sweep** — which open PRs `gate-topology` claw-merge may squash (bee PRs → `auto-merge-ready-prs.yml`) |
| `specs/claw/red_origin.t27` | **Ledger-line rule** — whole-line path lists; inherited red only when master fails the same list outside the diff |
| `specs/queen/merger_gate.t27` | **Bee CI-flake rule** — discount a red only when master's decisive run failed the same way (`look_back`, `run_says`) |
| `specs/claw/gates.t27` | Gate inventory; merge verdict rows reference real workflow names |
| `.github/workflows/auto-merge-ready-prs.yml` | Bee merges: `bee-reviewed` + reviewer bee approval (#5547) |
| `specs/claw/model_route.t27` | Merge **subagent** tier: **Composer 2.5** or **Sonnet 5.5**, not Opus, unless escalated |

## The seven conditions (session-owned PR)

Bits in `merge_verdict.t27` when a condition **fails**:

1. **ci_green** — required checks passed; none pending, unscanned, own red, or required inherited red.
2. **no_own_reds** — every red is **inherited** (master already red, same failure list, no touched file in the list).
3. **own_pr** — this session opened the PR (`A_MERGE` vs `A_ADVISE` for others).
4. **scope_match** — diff matches stated scope.
5. **no_conflict** — mergeable with `master`.
6. **no_scope_drift** — no file outside the issue **Boundary** (seal on Boundary counts).
7. **no_regression** — existing tests still pass (coverage regressions are own reds unless inherited).

**Verdict:** all six besides `own_pr` green → `MERGE_NOW`; only `ci_green` failing with a wait end → `WAIT`; else → `ESCALATE` (do not merge).

User-facing **six** map to (1)(2)(4)(5)(7) plus seals/ratchet under (2) and `mv_master_fix` / stale seal → re-save PR (#13359).

## Merge subagent (cheap tier)

1. **Model:** `composer-2.5` or `claude-sonnet-5-5` (`specs/claw/model_route.t27`, `cursor_route.t27`). Opus only after failed attempts or owner word.
2. **Task:** gather facts → compute verdict via spec rules → **squash-merge only if** `mv_act` is `A_MERGE` on this session's PR (`gh pr merge N --squash --delete-branch` with head pinned).
3. **Sweep lane:** another session's open PR → output `M\tN\tMERGE_NOW\tadvise\t…` for the owning session; do not merge unless `own_pr`.

Spawn with the Agent tool; brief must include PR number, issue Boundary paths, and `fix_pr` named when a stale seal needs `F_RESAVE`.

## Event-driven CI (no tight polling)

`merge_verdict.t27` allows reads only on `E_CHECK_DONE`, `E_PUSH`, `E_MASTER_MOVED`, `E_ACK` — **not** `E_TIMER`.

In Cursor Cloud:

- `cursor-subscriptions-subscribe_github_ci` on the PR **head branch** until terminal checks.
- `cursor-subscriptions-subscribe_github_pr` for merge/close.
- End the turn while waiting; re-run this skill on notification.

Forbidden: `sleep` loops scanning `gh pr checks` every few seconds; spawning agents only to poll.

## Machine-readable verdict line

Emit one line per judged PR (spec `mv_line`):

```text
M	<pr>	<MERGE_NOW|WAIT|ESCALATE>	<merge|advise|hold|stop>	<blockers or ->
```

Blocker names: `ci_green`, `no_own_reds`, `own_pr`, `scope_match`, `no_conflict`, `no_scope_drift`, `no_regression` (comma-separated).

## Ledger-line rule (ratchet / inherited red)

For a red check:

1. Fetch failing paths as **whole lines** (same convention as `hits()` in `red_origin.t27`).
2. Compare to master's decisive run of the same check (`merge_judge` B-lines; `merger_gate` look_back).
3. **Inherited** only if lists match exactly, master is red, and **no** named path is in the PR diff.
4. Unexpected new failure line → **own red** → `ESCALATE`, no merge.

## CI-flake rules

- **Stale master red** + same list + outside diff → `WAIT` if a fix PR is named (`mv_can_wait`); else `ESCALATE`.
- **504 / no scan** on secret-scan → `K_NO_SCAN` → `WAIT`, not pass (#13351).
- **Cancelled** master run → **unknown**, never discount (`merger_gate`).

## Workflow (one PR)

1. PR opened; CI runs.
2. Subscribe to CI on head branch (notification).
3. On event, re-read checks + master baseline + diff files + Boundary.
4. Classify each check (`K_PASS`, `K_PENDING`, `K_NO_SCAN`, `K_RED_*`).
5. Compute `mv_blockers` → `mv_verdict` → `mv_act`.
6. If `A_MERGE` and GitHub ruleset allows → squash-merge pinned to head SHA.
7. Else report exact blocker line; no merge.

## Bulk open-PR sweep (merge-all goal)

Two merge paths — do not confuse them:

| Path | When | Law |
|------|------|-----|
| **GitHub auto-merge** (integration) | Required four contexts green + `CLEAN` | Ruleset on `master` |
| **claw-merge job** | Scheduled / workflow_run | `merge_judge.t27` + App token |
| **Bee merge** | After `t27-bees[bot]` review + label | `auto-merge-ready-prs.yml` |
| **Session subagent** | This skill + `merge_verdict.t27` | Seven conditions + record line |

For **55+ open PRs**: arm `gh pr merge N --squash --auto` where mergeable; run **merge subagent** on PRs that are own or when orchestrating; never merge #13420 until base is `master` (owner edit).

## End-of-run report (paste verbatim)

```text
MERGE_SUBAGENT_STATUS: <restored|blocked — reason>
SIX_CONDITIONS_STATUS: <per-PR lines or summary>
LEDGER_LINE_STATUS: <inherited vs own counts>
MERGE_VERDICT_STATUS: <M-lines emitted>
CI_FLAKE_RULES_STATUS: <discounts applied or refused>
EVENT_DRIVEN_STATUS: <subscription ids or N/A>
BLOCKERS: <comma list>
```

## Traps

- **Wrong base** (`main` vs `master`) → `merge_judge` rule 3; fix base before judging (#13420).
- **Zero `bee-reviewed`** → `auto-merge-ready-prs.yml` skips everything; cloud token cannot label — run `tools/bees/reviewer.py` with bee token.
- **Rollup ≠ head SHA** → `WAIT` in `merge_judge`; read check-runs API for head OID only.
- **Merge-all without verdict** → violates owner safety; never `gh pr merge --admin` to bypass reds.
