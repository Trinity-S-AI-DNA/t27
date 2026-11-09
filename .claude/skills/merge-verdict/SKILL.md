---
name: merge-verdict
description: >-
  MERGE_QUEUE_V2: wait for CI on subscriptions, evaluate ClawMergeVerdict (seven
  conditions), emit one machine-readable M-line, squash-merge only on MERGE_NOW
  when this session owns the PR. Sonnet or Composer 2.5 for the merge pass; no
  tight polling. Load before merging a session's own PR or restoring the merge
  subagent workflow.
---

# Merge verdict (MERGE_QUEUE_V2)

Restore the merge subagent workflow by following the spec. Do not restate the
rule in prose here; when this skill and the spec disagree, the spec wins.

## SSOT

| Artifact | Role |
|----------|------|
| `specs/claw/merge_verdict.t27` | **ClawMergeVerdict** -- seven conditions, WAIT / ESCALATE / MERGE_NOW, `mv_line` record |
| `specs/claw/red_origin.t27` | Inherited red: whole-line list match (`hits`) |
| `specs/claw/ledger_union.t27` | Ratchet / ledger-line merge rule (three-way ledger) |
| `specs/claw/merge_judge.t27` | Repo-wide claw-merge sweep (bot); not the session merge subagent |
| `specs/claw/model_route.t27` | Merge pass model: orchestration tier (Sonnet 5.5), not Opus |
| `~/.cursor/skills-cursor/subscribe/SKILL.md` | Event-driven CI: subscribe, do not poll |

Pinned cases in the spec: **#13350** (all hold, merge), **#13351** (NO_SCAN then inherited), **#13359** (stale seal re-save on master).

## Seven conditions (owner order)

Bit names when a condition **fails** (`B_*` in the spec):

1. **ci_green** -- required checks pass; nothing pending or NO_SCAN; no own red; no required inherited red without a named fix PR on master.
2. **no_own_reds** -- every red is inherited (same failure list as master, no failing path in the PR diff).
3. **own_pr** -- this session opened the PR (else **advise**, not merge).
4. **scope_match** -- diff matches stated scope.
5. **no_conflict** -- mergeable with master.
6. **no_scope_drift** -- diff stays inside the issue Boundary (Boundary spec seal counts).
7. **no_regression** -- existing tests still pass (coverage / regression class).

**MERGE_NOW** when all seven hold. **WAIT** only when the sole blocker is **ci_green** and wait can end (pending, NO_SCAN, or inherited required red with fix PR named). Otherwise **ESCALATE** / **stop**.

User-facing "six conditions" maps to the six verdict bits besides **own_pr**; **own_pr** still gates **who** may call `gh pr merge`.

## Machine-readable verdict line

Emit exactly one line per evaluation (spec `mv_line`):

```text
M\t<pr>\t<MERGE_NOW|WAIT|ESCALATE>\t<merge|advise|hold|stop>\t<blockers>\n
```

Blockers: `-` when none; comma-separated condition names when blocked; `?` when mask is invalid.

## Ledger-line rule (ratchet-like checks)

For checks that compare a ledger file to a baseline (native ratchet, ledger union):

- Read **base / theirs / ours** via `specs/claw/ledger_union.t27`.
- Allow only expected line differences; any unexpected failure is **own red**, not inherited.
- Do not patch ratchet debt inside an unrelated lane PR; fix on master or the owning lane.

## CI: event-driven, not polling

Spec events (`E_CHECK_DONE`, `E_PUSH`, `E_MASTER_MOVED`, `E_ACK` only; **`E_TIMER` is polling** and must not drive reads):

1. Subscribe: `cursor-subscriptions` **github:ci:branch** and/or **github:pull_request:pr** (see subscribe skill).
2. End the turn; wake on notification.
3. Re-read **latest SHA** checks; never merge on stale rollup.
4. API budget: `API_LIMIT_PER_HOUR` in the spec -- shared token; batch reads.

For recurring lane ticks use a **named timer** (e.g. 900s), not a tight `gh pr checks` loop.

## Merge subagent workflow

1. PR open; CI running on head SHA.
2. Session subscribes to CI/PR events.
3. On wake: classify each required check (`K_PASS`, `K_PENDING`, `K_NO_SCAN`, `K_RED_OWN`, `K_RED_INHERITED`, `K_RED_INHERITED_REQ`) using master baseline and diff paths.
4. Set the seven booleans; compute mask, verdict, act.
5. If **MERGE_NOW** and **A_MERGE** and policy allows: **squash-merge pinned to the evaluated head SHA** (`gh pr merge --squash` only when owner rules and branch protection allow; cloud agents do not merge unless the task explicitly grants it).
6. Else: report **exact blockers**; no merge.

**Model route:** cheapest tier that passes the machine gate -- **Sonnet 5.5** or **Composer 2.5** for the merge eligibility pass (`model_route.t27` orchestration). Opus only on owner escalation.

## NON-NEGOTIABLE safety

- No merge without green required checks on **latest SHA**.
- No merge on a **new** own red introduced by this PR.
- No merge on scope drift, conflict, regression, or invalid seals.
- No merge without owner approval where policy requires it (e.g. `no-auto-merge` label, bee PR rules in `merge_judge.t27`).
- No unauthorized merge from a lane that does not own the PR files (sibling lanes: do not merge for them).

## MERGE_SUBAGENT_STATUS report (end of merge pass)

```text
MERGE_SUBAGENT_STATUS: <restored|evaluated|refused>
SIX_CONDITIONS_STATUS: <mask bits or MERGE_NOW>
LEDGER_LINE_STATUS: <ok|n/a|blocked + check name>
MERGE_VERDICT_STATUS: <full M-line>
CI_FLAKE_RULES_STATUS: <inherited vs own per check>
EVENT_DRIVEN_STATUS: <subscription ids or none>
BLOCKERS: <comma list or none>
```

## Traps

- **Stale master red** (e.g. stale seal): inherited only with matching failure lists; fix via **F_RESAVE** on master, not inside an unrelated PR (#13359 pattern).
- **Secret scan HTTP 504**: `mv_scan_class` -> **NO_SCAN** -> **WAIT**, not pass (#13351).
- **t27b-native-ratchet** fail with diff touching ratchet paths: treat as **own red** until ledger rule passes.
- Pareto lane: still **HOLD** push/merge_master while `required_checks_pending` on cron spec; merge subagent is separate from pareto tick commits.
