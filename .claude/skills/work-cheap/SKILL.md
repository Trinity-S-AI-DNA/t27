---
id: work-cheap
name: Work cheap
description: The cost prompt every agent follows -- cheapest model tier that can pass the gate, read only what the brief names, batch tool calls, no polling, short reports, stop when the gate is green. Use at the start of any task, and paste the prompt below into every brief you hand to a pane, a subagent or a side session. Triggers - "work cheaper", "save limits", "cut cost", "cheap prompt", before spawning any agent.
argument-hint: "[none | paste]"
---

# Work cheap (owner request, 2026-10-25)

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

One page, one job: keep an agent from spending tokens it does not need. It carries no table of its own.
The model order is the law in `specs/claw/model_route.t27` (skills `model-routing`, `cheap-model-routing`);
a lane as a pane is `t27os-orchestra`; the whole lane order is `worktree-lane`. This skill is the behaviour
inside the lane, and the prompt to paste into a brief.

## The prompt (paste it at the top of every brief, unchanged)

```
COST RULES (owner, 2026-10-25: work cheaper). These bind you as much as the task does.
1. Cheapest tier that can pass the gate. A free T27 OS pane (z.ai glm-4.5-flash) takes attempts 1-2; a Sonnet
   subagent takes over after 2 failed attempts; Opus after 4 or on the owner's word. Free-model output is judged
   by a machine (compiler, test), never by its own word. Law: specs/claw/model_route.t27.
2. Read only what the brief names. Find a symbol with grep, then Read with offset and limit. Never read a whole
   large spec to find one function. Never re-read a file you just edited; the tool would have failed.
3. Batch independent tool calls in one message. One round trip beats five.
4. Do not poll. A background job or a pane tells you when it ends; do not loop on its output file.
5. A subagent is for independent, bounded work, not for a lookup of a handful of tool calls. At most one
   z.ai call at a time per key: five parallel runs got 429 and 529.
6. Keep thinking off on flash models unless the task needs it (owner, 2026-10-25: thinking-off answers in
   about a quarter of the time with real content; 1305 on glm-4.7-flash is service noise, not a dead key).
7. Report in under 10 lines, as text. No recap of the task, no list of files you did not touch.
8. Stop when the gate is green. No extra refactor, no extra test, no extra doc.
9. No builds, tests or benchmarks on the Mac (hook mac-no-heavy): they run in the lab or a T27 OS pane.
10. Hard rules, always: only .t27 for code; English and ASCII on GitHub; never print, copy or commit a secret.
End your report with one line: model, tool calls, what you chose not to do.
```

## Why each rule is there

- **Rules 1 and 5** come from the 2026-10-18 matrix (t27#10066): Sonnet matched Opus on bounded work at about
  40% of the cost; a free flash model needs a compiler loop; at concurrency 5 most z.ai runs died.
- **Rule 2** is where the money goes: cost lives in cache write, so a prompt that names the exact files and
  tools is cheaper than one that lets the model explore (measured on Sonnet with the t27 MCP tools: half the cost
  at equal correctness).
- **Rule 6** is the owner's measurement of 2026-10-25 and is not yet a law; once it lands in the key-master
  spec, point at that spec here instead of repeating the numbers.
- **Rule 9** is the owner's rule after the watchdog panics of 2026-10-09; `MAC_HEAVY_OK` is the owner's alone.

## Using it

1. Starting a task yourself: apply the ten rules, do not paste them to yourself.
2. Briefing a pane, subagent or side session: the brief starts with the prompt block above, then the issue
   link, the files the agent may touch and the command that proves it done (step 6 of `worktree-lane`).
3. A pane's command line cuts `T27OS_KEYS` at 200 bytes: put the brief in a file and type a one-line pointer to it.
4. Reviewing a report: a report with no cost line or longer than ten lines goes back for a shorter one.

## Traps

- A cheap model forgets a rule it was not told: the brief carries the rules, not a link to this page alone.
- "Cheaper" never means skipping a gate. A cheaper path that fails the gate costs more than the dear one.
- Never save tokens by dropping the secret rule or the language rule from the brief.
