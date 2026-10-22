---
id: model-routing
name: model-routing
description: Which model does which job -- a sub-task starts in a T27 OS pane on the free key tier, the orchestrator runs on Sonnet 5.5, Sonnet is the first escalation of a sub-task, and Opus 5.5 is reached only after failed attempts or on the owner's word. Load before spawning any subagent or pane, writing a workflow, or picking a model in TRI CLAW or the Queen.
argument-hint: "[role: orchestrate|plan|review-final|implement|port|search|test|docs|fix]"
---

# /model-routing -- a pane for sub-tasks, Sonnet to orchestrate, Opus only on evidence

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

Owner directive 2026-10-20 (#11317, summarised): run lanes through T27 OS instead of paid subagents -- cheaper, and the owner watches every lane live in the window. It adds the pane tier below Sonnet for sub-tasks. It builds on the directive of 2026-10-19 (translated): "set Sonnet as the default model for all agents -- it is cheaper with the same result" (#10921), which keeps the orchestrator on Sonnet 5.5; that one replaced the 2026-10-18 rule (orchestration on Opus 5.5, #10493). The rule is part of TRI CLAW so development stays economical.

**The law is `specs/claw/model_route.t27`.** This skill is how to follow it in a Claude Code session. If the two disagree, the spec wins; fix the skill.

## The rule

| Role | Kind | Starts at | Escalates to |
|---|---|---|---|
| implement, port, search, test, docs, fix | sub-task | **a T27 OS pane** on the free key tier (`TIER_PANE`, `pane`) | Sonnet 5.5 after 2 failures, Opus 5.5 after 4 |
| orchestrate, plan, review-final | orchestration | **Sonnet 5.5** (`TIER_SONNET`, `sonnet`, `claude-sonnet-5-5`) | Opus 5.5 after 2 failures |

- Escalation is one step per `ESCALATE_AFTER` (2) failed attempts on the same task: pane, then Sonnet, then Opus (`opus`, `claude-opus-5-5`). The owner's word is Opus at any time (`owner_flag`). Nothing else raises a tier.
- Any tier stronger than `route` names is waste (`is_waste`): Opus on a fresh job for every role, and Sonnet on a fresh sub-task that a pane could have run.
- Why: epic #10066 measured Sonnet 5.5 at least as good as Opus 5.5 on ten bounded spec-to-RTL tasks (9 vs 8 direct Verilog, 7 vs 7 in .t27) at about 40% of the cost; a pane costs no tokens, and the gate decides what lands, not the model that wrote the diff.

## How it is wired in Claude Code

- `.claude/settings.json` sets `CLAUDE_CODE_SUBAGENT_MODEL=claude-sonnet-5-5`, so a subagent spawned on escalation, with no model of its own, runs on Sonnet.
- Every agent in `.claude/agents/` carries `model: sonnet`, the orchestrator `trinity.md` (Queen) included.
- The main session is the orchestrator: pick **Sonnet 5.5** for it in the app's model menu; switch it to Opus only for a job that failed twice on Sonnet or on the owner's word.

## How a sub-task runs as a pane

The pane tier is a place, not a Claude model: a Claude Code pane in the T27 OS window, started in the lane's worktree through `T27OS_CTL`, on the free key tier (glm-4.5-flash, then an OpenRouter `:free` model). The skill `t27os-orchestra` is the procedure: the brief as a file, spawn, wait, read the report, verify the diff, land with the gates, kill the pane. Which `T27OS_CTL` verbs exist today and which are pending (#11316) is stated there.

## When you are the orchestrator

1. Plan (on Sonnet): split the task into sub-tasks with a clear input, output and done-check each.
2. Start each sub-task as a pane (`t27os-orchestra`). Give it everything it needs in its brief file: paths, the issue, the boundary, the acceptance check. A brief that needs a stronger model to understand is a planning failure, not a reason to escalate.
3. Verify the result yourself (that is review-final): read the diff or the output, run the check.
4. If the same sub-task fails twice in a pane, re-run it once as a Sonnet subagent (the Agent tool, `model` unset or `"sonnet"`); if it fails twice there too, once more with `model: "opus"`. Say in the result why it escalated.
5. In a Workflow script, set `model: 'sonnet'` on `agent()` calls for escalated sub-tasks and keep the orchestration logic in the script itself.

Parallelism: about 3 working lanes (panes or subagents together) at once on this 8 GB Mac (swap eats the disk); more on the Railway lab.

## Audit

`is_waste(role, tier, failed_attempts, owner_flag)` in the spec answers whether a run spent a stronger tier than `route` names. Use it when TRI CLAW or the Queen records which model ran a job.

## Not covered yet

- TRI CLAW's own chat agent (gHashTag/999-multibots-telegraf) and the Queen (gHashTag/trios) still pick models in their own code; they should read this table. Follow-up issues in those repos.
- Haiku is not in the rule. Add a tier to the spec first, with a measurement, before using it.
