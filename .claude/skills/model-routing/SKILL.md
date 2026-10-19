---
id: model-routing
name: model-routing
description: Which Claude model does which job -- every agent runs on Sonnet 5.5 by default, the orchestrator included, and any job escalates to Opus 5.5 only after two failed attempts or on the owner's word. Load before spawning any subagent, writing a workflow, or picking a model in TRI CLAW or the Queen.
argument-hint: "[role: orchestrate|plan|review-final|implement|port|search|test|docs|fix]"
---

# /model-routing -- Sonnet by default for every agent, Opus only on evidence

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

Owner directive 2026-10-19 (translated): "set Sonnet as the default model for all agents -- it is cheaper with the same result" (#10921). It replaces the 2026-10-18 rule (orchestration on Opus 5.5, sub-tasks on Sonnet 5.5, #10493). The rule is part of TRI CLAW so development stays economical.

**The law is `specs/claw/model_route.t27`.** This skill is how to follow it in a Claude Code session. If the two disagree, the spec wins; fix the skill.

## The rule

| Role | Kind | Model |
|---|---|---|
| orchestrate, plan, review-final | orchestration | **Sonnet 5.5** (`sonnet`, `claude-sonnet-5-5`) |
| implement, port, search, test, docs, fix | sub-task | **Sonnet 5.5** (`sonnet`, `claude-sonnet-5-5`) |

- Any job goes to Opus 5.5 (`opus`, `claude-opus-5-5`) only on evidence: **2 failed Sonnet attempts on the same task** (`ESCALATE_AFTER`), or the owner says so. Nothing else raises a tier.
- Opus without that evidence is waste for every role (`is_waste`), the orchestrator included.
- Why: epic #10066 measured Sonnet 5.5 at least as good as Opus 5.5 on ten bounded spec-to-RTL tasks (9 vs 8 direct Verilog, 7 vs 7 in .t27) at about 40% of the cost.

## How it is wired in Claude Code

- `.claude/settings.json` sets `CLAUDE_CODE_SUBAGENT_MODEL=claude-sonnet-5-5`, so any subagent with no model of its own runs on Sonnet.
- Every agent in `.claude/agents/` carries `model: sonnet`, the orchestrator `trinity.md` (Queen) included.
- The main session is the orchestrator: pick **Sonnet 5.5** for it in the app's model menu; switch it to Opus only for a job that failed twice on Sonnet or on the owner's word.

## When you are the orchestrator

1. Plan (on Sonnet): split the task into sub-tasks with a clear input, output and done-check each.
2. Spawn each sub-task with the Agent tool. Leave `model` unset (the default is Sonnet) or pass `model: "sonnet"`. Give the subagent everything it needs in the prompt: file paths, the issue, the acceptance check. A sub-task prompt that needs Opus to understand is a planning failure, not a reason to escalate.
3. Verify the result yourself (that is review-final): read the diff or the output, run the check.
4. If the same sub-task fails twice on Sonnet, re-spawn it once with `model: "opus"`, and say in the result why it escalated.
5. In a Workflow script, set `model: 'sonnet'` on sub-task `agent()` calls and keep the orchestration logic in the script itself.

Parallelism: at most 3 subagents at once on this 8 GB Mac (swap eats the disk); more on the Railway lab.

## Audit

`is_waste(role, tier, failed_attempts, owner_flag)` in the spec answers whether a run spent Opus on any role without evidence. Use it when TRI CLAW or the Queen records which model ran a job.

## Not covered yet

- TRI CLAW's own chat agent (gHashTag/999-multibots-telegraf) and the Queen (gHashTag/trios) still pick models in their own code; they should read this table. Follow-up issues in those repos.
- Haiku is not in the rule. Add a tier to the spec first, with a measurement, before using it.
