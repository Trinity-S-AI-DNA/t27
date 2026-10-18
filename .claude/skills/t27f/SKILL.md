---
id: t27f
name: T27F Visual Spec
description: T27F is the visual spec of a .t27 file -- one spec, one synchronized model, many views (source, architecture, data flow, dependencies, tests and benchmarks), every node labelled exact, derived, suggested or unknown and linked to its source span (gHashTag/trinity#1648, RFC gHashTag/trinity#1649). Use when the user says "t27f", "visual spec", "show / visualize / draw this spec", or opens a .t27 file to see its structure. Also runs the topic-map application built on T27F (gHashTag/trinity#1677) when the user gives a topic or a client brief.
---

# T27F Visual Spec

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

## What T27F is

**This section is the one home of the definition.** `CLAUDE.md`, `specs/OWNERS.md`, the skill card
`specs/skills/t27-t27f.t27` and the spec headers point here; they do not restate it.

T27F is the **visual spec of `.t27`**: every t27 specification has a first-class visual form, an
interactive, inspectable diagram built from reusable widgets. Sources: epic gHashTag/trinity#1648
("native generative UI and visual application engine") and RFC gHashTag/trinity#1649 ("specs as
executable visual diagrams").

- **One spec -> one synchronized source model -> many views**: source, architecture diagram, data
  flow, dependency graph, tests/benchmarks, docs. Every view is derived from the real spec
  (AST/IR plus metadata), never a hand-drawn picture that can drift from the code.
- **Graph IR**: versioned, typed nodes (module, function, step, input/output/parameter/constant,
  type/numeric format, branch/loop, operator, dependency, test/invariant/oracle, benchmark/hardware,
  paper/dataset, opaque region) and typed edges (data flow, control flow, calls/depends on,
  produces/consumes, constrains, tested by, measured by, implemented by, cited by). The full lists
  are in #1649.
- **Every node carries one inference label**: `exact` (present in the AST/IR), `derived` (safe from
  symbol and type resolution), `suggested` (a model's explanation, visibly marked, linked to source
  ranges) or `unknown`. A model's guess is never drawn as a verified property of the program.
- **Bidirectional source mapping**: select a node, see its source span; select a line, see its
  node. Generation is deterministic for a fixed source and toolchain, and two revisions produce a
  graph diff.
- **Read-only viewer first.** Editing comes after the schema survives two structurally different
  specs. GF16 (`specs/numeric/gf16.t27`) is a *candidate* pilot pending the comparison in #1651.
- **The pipeline (#1648)**: intent -> versioned system description -> domain model and computation
  -> renderer-neutral UI plan (trusted widget ids, bindings, layout, actions, theme, a11y,
  provenance) -> T27F renderer -> verification loop. A model never ships code to the renderer: no
  eval, no generated JS, only widgets from an allowlist.
- **In the map** (#1648 comments, 2026-10-16): zooming into a honeycomb cell opens that cell's spec
  as a navigable program, linked to source, tests, issues and real task status; each cell is both a
  spec and a learning entry point.

**What T27F is not:** it is not the topic-to-value engine. gHashTag/trinity#1677 (topic or client
brief -> 27-lesson map -> Bee -> verified artifact) is an *application* built on T27F, and so are
the reels. Describing T27F as #1677 is the error this section exists to stop (owner correction,
2026-10-17).

## What exists and what does not

| Layer | Where | State |
|---|---|---|
| Spec -> graph IR schema (the core) | `specs/t27f/spec_graph.t27` (t27#10020) | spec with tests: node / edge kinds, labels, spans, `validate_graph`, line -> node mapping, node -> ui_plan widget, one hand-read fixture (`specs/tutorial/08_modules_and_arrays.t27`) |
| Graph view: layout of a spec graph | `specs/t27f/spec_graph_view.t27` (t27#10034) | spec with tests: one column per node kind, size / colour / dash / row order from the inference label, UNKNOWN nodes alone in a band, every edge with a source line, node -> widget through `node_widget`; numbers only, laid out on the hand-read fixture; drawn by the MCP tool `t27f_graph` (gHashTag/t27-mcp#9) |
| Graph view for big specs: wrap, fold, expand, hit | `specs/t27f/spec_graph_fold.t27` (t27#10218) | spec with tests: a kind column taller than N = 12 rows wraps into sub-columns so the canvas aspect is near phi (identical to the view when nothing wraps); a fold to one box per kind with the exact count and one arc per pair of kinds with the exact edge count, each labelled by its weakest member; `expand_graph` = one kind plus its direct neighbours, copied verbatim with an origin map; `wrap_hit` / `wrap_hit_line_start` / `wrap_hit_line_end` take a click to a node and its lines; numbers only, the view's colours are not touched |
| Spec -> graph IR exporter | `specs/t27f/spec_graph_export.t27` (t27#10162) | spec with tests: `export_graph(path, src)` is a deterministic byte scanner over the source, `export_verdict` is `validate_graph` of the result. One node per top-level declaration (a leading `pub ` is allowed), spans to the matching closing brace, every node and edge EXACT or (unclassifiable) OPAQUE + UNKNOWN, CALLS / USES / CONSTRAINS edges with the line of the use, refusal past 1024 nodes or 4096 edges. On the tutorial 08 snapshot it equals the hand-read fixture on all 14 declaration nodes and all 9 EXACT edges; it does not emit the fixture's `std.testing` UNKNOWN node, nor its 7 DERIVED edges (4 data flow, 3 tested_by), because a scanner never derives; the derive pass below adds them. Limits: textual matching without scope or type resolution, no block comments, no struct/enum bodies, no cross-spec `use` resolution, capacity 1024 nodes and 4096 edges, u16 indices (all 2225 specs in specs/ fit, largest 775 nodes and 3194 edges after derive; anything past the cap is refused, never truncated), takes a `[]const u8` slice rather than `[65536]u8` plus a length. AST/IR audit is trinity#1650, pilot choice #1651 |
| Graph IR derive pass | `specs/t27f/spec_graph_derive.t27` (t27#10191) | spec with tests: `derive_graph(g, src)` appends what the exporter must not claim and changes nothing it holds. R1 DERIVED `tested_by` = every EXACT CALLS edge from a test, invariant or bench to a fn read backwards (no line). R2 DERIVED `data_flow` = a const or `use` that an EXACT USES edge ties to a fn and that is named inside one of the fn's return statements (from `return` to the next `;`), on the line of the first such name. R3 UNKNOWN opaque node = an undeclared qualified call target of three or more segments (`std.testing.expectEqual(` names `std.testing`), spanning the lines that call through it. Exporter plus derive equal the tutorial 08 fixture as sets: 15 of 15 nodes, 16 of 16 edges, no special case. Limits: names matched as text; call results and flow through locals are not derived; R3 needs 3 or more segments (two cannot be told from a method call); a graph that does not validate, or a source of another line count, comes back unchanged; past 1024 nodes or 4096 edges the result is refused with the too-many verdict. Not wired to a host: the MCP tool does not call it |
| UI plan IR + trusted widget catalog | `specs/t27f/ui_plan.t27` (t27#9962) | spec with tests; no renderer draws a plan yet |
| Honeycomb focus+context geometry | `specs/t27f/reel.t27` | spec; stills drawn by the MCP tool `t27f_view` |
| Renderer in the Trinity UI | gHashTag/trinity | **missing**; no T27F widget is live |
| Application: topic map (#1677) | `specs/t27f/topic_engine.t27`, `topic_reel.t27` | spec; procedure below |
| Application: reels and deck | `specs/t27f/reel.t27`, `deck.t27`, `reel_*.t27` | spec; two reels |
| Application: content factory (epic t27#10560) | `specs/t27f/reel_factory.t27` (t27#10561) | spec with tests: an item's code and title, read keyword by keyword, give 11 features. Those pick 1 of 12 scene archetypes (best pair first, cap 4, no repeat within 2 items, then the least-used generic scene) plus an accent, a motion, a layout and 8 output formats. No renderer draws a plan yet (R3), and `lesson_reel` still draws token_floor (R4). Lessons: read the plan's spread over the whole course before trusting it. A greedy walk in course order gave 7 lessons one scene. Code counts alone handed lesson 8 the tests scene over "A test block". A printing `main` makes t27b reject the spec |
| Related | trinity#1647 dashboards, #1652 MVP research, #1653 three skills, #1655 evolution runtime | open |

When asked "how is T27F going", report against the core rows first. Progress on the applications
is not progress on the core.

## Show a spec (the core: exporter for what it reads, by hand for the rest)

1. Load the spec: `mcp__t27__t27_spec` with `path` and `source: true`, or the file from a
   gHashTag/t27 checkout. Say which one you used.
2. List nodes and edges from the source only: module, imports (`use`), constants, functions,
   `test` / `invariant` / `bench` blocks, calls between them, which tests exercise which function.
   Give every node and edge its line span and its label (`exact` / `derived` / `suggested` /
   `unknown`). The kinds, labels and rules are those of `specs/t27f/spec_graph.t27`; read them
   there, and check the reading against its `validate_graph` rules (its `tutorial_08_graph`
   fixture is a worked example).
3. Show it as a table (node, kind, span, label) and, if useful, a diagram whose every arrow is a
   row of that table. Nothing is drawn that the table does not back.
4. Say plainly that this is a manual reading unless a graph came out of `export_graph`
   (`specs/t27f/spec_graph_export.t27`), with `derive_graph` (`specs/t27f/spec_graph_derive.t27`)
   on top. The exporter reads declarations, spans and exact CALLS / USES / CONSTRAINS edges only,
   within 1024 nodes and 4096 edges; the derive pass adds DERIVED tested_by and data_flow edges
   and UNKNOWN opaque nodes for undeclared qualified call targets, by its three stated rules.
   Anything the rules do not reach (SUGGESTED explanations, call-result flow, anything larger)
   is still a manual reading, and no host runs either pass yet. Name that gap rather than hide it.

## Visual in every output (Claude Code and TRI CLAW)

**The rule is `specs/t27mcp/visual_output.t27`** (its `RULE_CLAUDE_CODE` and `RULE_TRI_CLAW` lines
are what each host reads). **Its particle-still layer is `specs/claw/visual_output.t27`.** This
section is the order of steps; which answer carries which visual and how it reaches each host are
the specs', not repeated here.

1. Decide the answer's kind (`K_*` in the t27mcp spec). A one-sentence reply or a question back
   goes without a visual.
2. Spec, flow or numbers: draw from t27f tool rows (`t27f_outline`, `t27f_graph`, `t27f_view`),
   labelled, with the legend and the source file, as `answer_verdict` checks.
3. A report of a piece of work: a particle still (`takes_still`). Render the reel on the Railway lab
   (never the Mac), take the frame at `STILL_MS` with `ffmpeg -ss 9 -frames:v 1`, and bring it into
   the session folder (`out-reels/`, gitignored, never `git add`).
4. Anything not made: say which and why in the answer. Never show an older file as this turn's.

## Application: topic map (gHashTag/trinity#1677)

Use this part when the user gives a topic or a client brief, not a spec.

**Its rules live in one place: `specs/t27f/topic_engine.t27`.** This section is the order of
operations around it. It never restates a constant, a vocabulary or a gate; it reads them. If
this text and the spec disagree, the spec wins and this text is the bug.

### 0. Load the engine

Read the spec through the t27 MCP server, which is already installed in Claude Code:

- `mcp__t27__t27_spec` with `path: "specs/t27f/topic_engine.t27"` and `source: true`.
- Until the spec is published to t27.ai, the MCP answers "not found": read the file from a
  gHashTag/t27 checkout instead. Say which one you used.

From the source, take: `LENS_NAMES`, `LENS_QUESTIONS`, `STAGE_NAMES`, the four `*_WORDS`
vocabularies and `KEYWORD_MATCH`, the five `*_BEES` lists, `MAX_ACTIVE_BEES`, `GAP_BEE`, and the
functions `intake_verdict`, `map_verdict`, `initial_status`, `ground`, `route`, `dispatch_allowed`,
`bee_target`, `required_evidence`, `evidence_accepted`, `verify`, `counts_as_progress`,
`needs_human_approval`, `action_allowed`, `after_action`. Evaluate them as written; they are small and total.

Everything in `DETERMINISTIC_STEPS` is decided by those functions, never by your judgement.
You write only what `LLM_WRITES` names.

### 1. Intake

Ask for, or take from the message: the topic or brief, and its sources (links, files, notes).
Apply `intake_verdict(has_topic, source_count)`:

- `INTAKE_EMPTY`: ask for the topic and stop.
- `INTAKE_NO_SOURCES`: continue, but every lesson starts as a GAP. Say so up front.
- `INTAKE_OK`: continue.

### 2. Route

Count the brief's hits against each `*_WORDS` list exactly as `KEYWORD_MATCH` says. Show the four
counts and the words that hit, then apply `route(content, engineering, research, sales)`. Name the domain and the Bee list
it allows. Do not override the result; if it looks wrong, the vocabulary in the spec is what needs a
PR.

### 3. Three lenses

Write one short paragraph per lens, answering that lens's question from `LENS_QUESTIONS`, using only
the sources. A claim with no source is not written; it becomes a gap.

### 4. The 27-lesson map

Print a table of 27 rows, lens-major (`step_lens`, `step_stage`): step, lens, stage, lesson title,
source, status.

- For each lesson, `grounded` means at least one source from intake supports it. Sources you find
  yourself are not intake; they arrive through the Bee task in step 5 and `ground`.
- A GAP row keeps its lesson title (titles are in `LLM_WRITES`); its source cell is empty.
- Status from `initial_status(grounded, first_grounded)`; `first_grounded` is the grounded lesson
  with the lowest step. A GAP row's status cell says **GAP: needs a source on <what>** and nothing
  else. Never invent filler to make the map look complete.
- Count grounded lessons per lens and report `map_verdict(...)`.

### 5. One Bee task

Apply `bee_target(open_lessons)`. `TARGET_OPEN_LESSON`: take the first OPEN lesson.
`TARGET_FIRST_GAP` (nothing is OPEN, as when intake was `INTAKE_NO_SOURCES`): take the first GAP, the
capability is `GAP_BEE`, and the task is to find a source for it; once one is attached, apply
`ground`. `active_bees` counts the Bees of this journey still working; at the start it is 0. Check
`dispatch_allowed(active_bees)`. Choose one capability from the
domain's Bee list and write one task for it:

- goal (one sentence), input (the lesson and its sources), output (the artifact),
- the evidence that will verify it: `required_evidence(stage, domain)`.

In this phase the Bee is you, in this session, or a delegated Agent if the user asks for it. Never
start more than `MAX_ACTIVE_BEES`, and never one per lesson.

### 6. Draft, then verify

Produce the artifact as a DRAFT and label it as one. Collect the evidence (a source link, a passing
test, a review, the client's approval). Apply `verify(DRAFT, stage, domain, evidence)`.

- Only VERIFIED or PUBLISHED counts: `counts_as_progress`. Report the progression event as
  `+POINTS_PER_VERIFIED_LESSON` point, and nothing else. `REWARD_IS_MONEY` is false: never describe
  points as money, tokens, payouts or value.
- `opens_next(DRAFT)` is true, so the learner may move on while the draft waits for evidence.

### 7. Approval gate

Before any action, check `needs_human_approval(action)` and `action_allowed(status, action,
approved)`.

- Publish, send to a client, any payment, anything destructive: ask the user in chat and wait for a
  clear yes. A yes covers that one action only.
- Publishing needs a VERIFIED item. A draft is never published. Sending to a client also takes a
  DRAFT, because a client approval is the only evidence that verifies a sales or content DELIVER
  lesson; the draft stays a DRAFT (`after_action`) until that approval comes back.

### 8. Keep the three layers apart

Keep drafts, verified results and published items under separate headings (`layer_of`). If the
journey is saved, save it as a `.t27` data module, not JSON. Saving follows the user's instruction.

## What this skill does not claim

The graph IR is a schema (`specs/t27f/spec_graph.t27`) and `specs/t27f/spec_graph_export.t27` turns source into it, but only exact declarations, spans and edges within the IR's capacity, with `specs/t27f/spec_graph_derive.t27` adding the derived and unknown part by stated rules, and no host calls either yet; no renderer draws a T27F view in the Trinity UI, no
runtime dispatches Bees from a spec, and nothing publishes on its own. If a step needs one of
them, say that it is missing; do not simulate it.
