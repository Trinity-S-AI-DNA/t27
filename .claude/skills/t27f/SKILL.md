---
id: t27f
name: T27F Topic Engine
description: Turn any topic or client brief into three lenses, a 27-lesson map, one relevant Bee task and a verified artifact, with gaps shown, evidence gates and human approval for anything that leaves the machine. Use when the user says "t27f", "explain this topic in 27 steps", "turn this brief into a plan", or gives a client brief (villa, car, campaign, product) or a research/engineering topic.
---

# T27F Topic Engine

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

Phase 1 vertical slice of gHashTag/trinity#1677 (parent gHashTag/trinity#1648).

**The rules live in one place: `specs/t27f/topic_engine.t27`.** This skill is the order of
operations around it. It never restates a constant, a vocabulary or a gate; it reads them. If
this text and the spec disagree, the spec wins and this text is the bug.

## 0. Load the engine

Read the spec through the t27 MCP server, which is already installed in Claude Code:

- `mcp__t27__t27_spec` with `path: "specs/t27f/topic_engine.t27"` and `source: true`.
- Until the spec is published to t27.ai, the MCP answers "not found": read the file from a
  gHashTag/t27 checkout instead. Say which one you used.

From the source, take: `LENS_NAMES`, `LENS_QUESTIONS`, `STAGE_NAMES`, the four `*_WORDS`
vocabularies, the five `*_BEES` lists, `MAX_ACTIVE_BEES`, and the functions `intake_verdict`,
`map_verdict`, `initial_status`, `route`, `dispatch_allowed`, `required_evidence`,
`evidence_accepted`, `verify`, `counts_as_progress`, `needs_human_approval`, `action_allowed`,
`after_action`. Evaluate them as written; they are small and total.

Everything in `DETERMINISTIC_STEPS` is decided by those functions, never by your judgement.
You write only what `LLM_WRITES` names.

## 1. Intake

Ask for, or take from the message: the topic or brief, and its sources (links, files, notes).
Apply `intake_verdict(has_topic, source_count)`:

- `INTAKE_EMPTY`: ask for the topic and stop.
- `INTAKE_NO_SOURCES`: continue, but every lesson starts as a GAP. Say so up front.
- `INTAKE_OK`: continue.

## 2. Route

Count whole-word, case-insensitive hits of the brief against each `*_WORDS` list. Show the four
counts, then apply `route(content, engineering, research, sales)`. Name the domain and the Bee list
it allows. Do not override the result; if it looks wrong, the vocabulary in the spec is what needs a
PR.

## 3. Three lenses

Write one short paragraph per lens, answering that lens's question from `LENS_QUESTIONS`, using only
the sources. A claim with no source is not written; it becomes a gap.

## 4. The 27-lesson map

Print a table of 27 rows, lens-major (`step_lens`, `step_stage`): step, lens, stage, lesson title,
source, status.

- For each lesson, `grounded` means at least one source supports it.
- Status from `initial_status(grounded, first_grounded)`. A GAP row says **GAP: needs a source on
  <what>** and nothing else. Never invent filler to make the map look complete.
- Count grounded lessons per lens and report `map_verdict(...)`.

## 5. One Bee task

Pick the first OPEN lesson. Check `dispatch_allowed(active_bees)`. Choose one capability from the
domain's Bee list and write one task for it:

- goal (one sentence), input (the lesson and its sources), output (the artifact),
- the evidence that will verify it: `required_evidence(stage, domain)`.

In this phase the Bee is you, in this session, or a delegated Agent if the user asks for it. Never
start more than `MAX_ACTIVE_BEES`, and never one per lesson.

## 6. Draft, then verify

Produce the artifact as a DRAFT and label it as one. Collect the evidence (a source link, a passing
test, a review, the client's approval). Apply `verify(DRAFT, stage, domain, evidence)`.

- Only VERIFIED or PUBLISHED counts: `counts_as_progress`. Report the progression event as
  `+POINTS_PER_VERIFIED_LESSON` point, and nothing else. `REWARD_IS_MONEY` is false: never describe
  points as money, tokens, payouts or value.
- `opens_next(DRAFT)` is true, so the learner may move on while the draft waits for evidence.

## 7. Approval gate

Before any action, check `needs_human_approval(action)` and `action_allowed(status, action,
approved)`.

- Publish, send to a client, any payment, anything destructive: ask the user in chat and wait for a
  clear yes. A yes covers that one action only.
- Publishing or sending needs a VERIFIED item. A draft is never published.

## 8. Keep the three layers apart

Keep drafts, verified results and published items under separate headings (`layer_of`). If the
journey is saved, save it as a `.t27` data module, not JSON. Saving follows the user's instruction.

## What this skill does not claim

No runtime dispatches Bees from the spec yet, and nothing publishes on its own. Those are later
phases of gHashTag/trinity#1677. If a step needs one of them, say that it is missing; do not
simulate it.
