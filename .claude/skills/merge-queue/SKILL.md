---
name: merge-queue
description: >-
  MERGE_QUEUE_V2 alias: merge only after ClawMergeVerdict (seven conditions) on
  latest SHA, event-driven CI, machine-readable M-line. Load before squash-merge
  of a session-owned PR. SSOT is merge_verdict.t27, not this page.
---

# Merge queue (MERGE_QUEUE_V2)

**Canonical skill:** `.claude/skills/merge-verdict/SKILL.md`  
**SSOT:** `specs/claw/merge_verdict.t27` (module `ClawMergeVerdict`)

This file exists so briefs and owner orders can say `merge-queue` without duplicating law.

## Quick law

1. Wait on **CI/PR subscription**, not a tight poll (`E_TIMER` is polling in the spec).
2. Classify checks on **latest head SHA** (inherited vs own red — `red_origin.t27`).
3. Emit one line: `M\t<pr>\tMERGE_NOW|WAIT|ESCALATE\t<act>\t<blockers>`.
4. **Squash-merge only** when verdict is `MERGE_NOW`, act is `merge`, and branch policy allows.
5. Otherwise report blockers; do not merge.

## Related specs

| Spec | Role |
|------|------|
| `specs/claw/merge_verdict.t27` | Seven conditions, verdict, act |
| `specs/claw/merge_judge.t27` | Bot sweep (`claw-merge`), not session merge |
| `specs/claw/ledger_union.t27` | Ledger-line / ratchet merges |
| `specs/claw/model_route.t27` | Sonnet/Composer merge pass, not Opus |

## Traps

- Pareto cron ticks **do not** merge; they audit and fix lane-owned `.t27` only.
- Cloud agents merge only when the run explicitly authorizes it and GitHub checks are green.
