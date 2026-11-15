---
name: free-first
description: >-
  Free-first model and tool routing for lanes, panes, and cloud ticks. Paste
  work-cheap rules into briefs; escalate per model_route.t27. Load when the
  owner says economize, free models, or FREEFIRSTSUBAGENTMODE.
---

# Free-first routing

**Paste block:** `.claude/skills/work-cheap/SKILL.md` (cost rules 1–11)  
**Swarm economy:** `.claude/skills/swarm-economy/SKILL.md`  
**Laws:** `specs/claw/model_route.t27`, `specs/claw/cost_policy.t27`, `specs/claw/cursor_route.t27`

## Order of execution

1. **Local shell** — `git`, `gh`, `t27c seal --verify`, grep/read named paths; no LLM.
2. **Free pane** — T27 OS pane on z.ai flash tier when the gate is machine-judgeable (`t27os-orchestra`).
3. **Sonnet** — first paid escalation for bounded edit/test (after two failures on same task).
4. **Opus** — owner word or fourth failure; never for routine merge-eligibility reads.

## Cursor Cloud Agent

- Default merge/audit pass: **Composer 2.5** or **Sonnet** per `cursor_route.t27`.
- **No tight `gh pr checks` loops** — `cursor-subscriptions` + 15m `loop-pareto-night` timer.
- DSH-backed free calls from cloud: `.claude/skills/dsh-startup/CLOUD.md`.

## Pareto lane

Load `.claude/skills/pareto-improvement-loop/SKILL.md` section **Free-first routing (FREEFIRSTSUBAGENTMODE)** each tick.

End reports with one routing ledger row:

```text
task | cost_route | data_route | capability_route | model | turns | artifact | gate_proof
```

## Traps

- Cheaper never means skip gates, leak secrets, or merge without `merge-queue` / `merge-verdict`.
- Do not spawn subagents to poll CI; subscribe instead.
