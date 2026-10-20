# AGENTS — Trinity S³AI / t27

This file is the **repository entry point** for humans and coding agents. It summarizes **where law lives** and **how to work safely** in this tree.

---

## 1. Read first (constitutional stack)

| Order | File | Role |
|------:|------|------|
| 1 | [`SOUL.md`](SOUL.md) | **Canonical** constitution (language policy, TDD mandate, validation). |
| 2 | [`docs/nona-03-manifest/SOUL.md`](docs/nona-03-manifest/SOUL.md) | Expanded reference; if it conflicts with root `SOUL.md`, **root wins**. |
| 3 | [`docs/T27-CONSTITUTION.md`](docs/T27-CONSTITUTION.md) | **SSOT-MATH**, **LANG-EN**, **DOCS-TREE** (where `docs/` files may live). |
| 4 | [`TASK.md`](TASK.md) + [`docs/coordination/TASK_PROTOCOL.md`](docs/coordination/TASK_PROTOCOL.md) | Multi-agent coordination, locks, anchor issue. |
| 5 | [`OWNERS.md`](OWNERS.md) | Domain ownership; each major directory may have its own `OWNERS.md`. |

Supporting: [`CONTRIBUTING.md`](CONTRIBUTING.md), [`SECURITY.md`](SECURITY.md), [`architecture/ADR-004-language-policy.md`](architecture/ADR-004-language-policy.md).

---

## 2. Agent model (27-letter alphabet)

- **Documentation map:** [`docs/README.md`](docs/README.md) — where first-party docs live (`agents/`, `coordination/`, `nona-01..03/`).
- **Full alphabet and roles:** [`docs/agents/AGENTS_ALPHABET.md`](docs/agents/AGENTS_ALPHABET.md) — canon for T, N, P, C, B, etc.
- **Operational agent specs (e.g. watchdogs, schemas):** [`docs/agents/AGENTS.md`](docs/agents/AGENTS.md) — complements the alphabet; not a second constitution.

Use **domain directories**, not “one folder per agent.” Primary contact for a path is the **Primary** listed in the nearest `OWNERS.md`.

- **Every path, spec, widget and app surface has its agents** (owner rule, 2026-10-20). A path maps
  to one letter through the agents' `KEY_FILES` (`specs/t27os/domain_pulse.t27`); a widget or app
  surface carries its owner letter plus the letters that feed it (`specs/t27os/agent_parts.t27`),
  and the widget factory refuses a widget without them. Keep `KEY_FILES` pointing at the
  directories a domain really changes: stale entries make the domain look idle.

---

## 3. Non-negotiables for changes

1. **Specs are source of truth** — behavior belongs in `.t27` / `.tri`; generated `gen/` output is not hand-edited (except documented exceptions).
2. **TDD inside specs** — new or changed specs need `test`, `invariant`, and/or `bench` where SOUL requires it.
3. **English + ASCII** — first-party Markdown and source comments per **LANG-EN** and **ADR-004**; grandfathered paths only in [`docs/.legacy-non-english-docs`](docs/.legacy-non-english-docs).
4. **No new Python on the verification critical path** — see **SSOT-MATH** and [`docs/nona-02-organism/TZ-T27-001-NO-PYTHON-CRITICAL-PATH.md`](docs/nona-02-organism/TZ-T27-001-NO-PYTHON-CRITICAL-PATH.md).
5. **Issue gate** — PRs should link issues (`Closes #N`) where project policy requires it.
   **Issue shape** -- a new issue stays open only if [`specs/policy/issue_shape.t27`](specs/policy/issue_shape.t27) (#7498) passes; otherwise `needs-boundary.yml` closes it seconds after filing, labelled `shape-refused`. That spec is the rule. Before `gh issue create`, load the skill [`.claude/skills/issue-shape/SKILL.md`](.claude/skills/issue-shape/SKILL.md): it explains the sections, the Boundary path trap, checking a body on the Railway lab, and recovery with `gh issue edit`.
6. **Ring / gold work** — follow [`docs/nona-01-foundation/GOLDEN-RINGS-CANON.md`](docs/nona-01-foundation/GOLDEN-RINGS-CANON.md) for parser/compiler/spec changes; compiler seal path: `bootstrap/stage0/FROZEN_HASH`.

---

## 4. Layout reminders (after repo hygiene refactors)

- **Core:** `specs/`, `compiler/`, `bootstrap/`, `gen/`, `conformance/`, `tests/`.
- **Non-core services & tooling:** `contrib/backend/`, `contrib/portable-claude-setup/`.
- **Vendored / datasets / upstream:** `external/` (e.g. OpenCode submodule, `external/kaggle/`).

---

## 5. Law Reference

The seven **Invariant Laws (L1–L8)** are defined in [`docs/T27-CONSTITUTION.md`](docs/T27-CONSTITUTION.md#2--invariant-laws-never-change-without-constitutional-amendment):

| Law | Name | Legacy alias | Summary |
|-----|------|-------------|---------|
| **L1** | TRACEABILITY | ISSUE-GATE | No code merged without `Closes #N` |
| **L2** | GENERATION | NO-HAND-EDIT-GEN | Files under `gen/` are generated; edit specs instead |
| **L3** | PURITY | SOUL-ASCII | Source files must be ASCII-only with English identifiers |
| **L4** | TESTABILITY | TDD-MANDATE | Every `.t27` spec must contain `test`/`invariant`/`bench` |
| **L5** | IDENTITY | PHI-IDENTITY | φ² = φ + 1; φ² + φ⁻² = 3; IEEE f64 checks use tolerance |
| **L6** | CEILING | TRINITY-SACRED | `FORMAT-SPEC-001.json` + `gf16.t27` are numeric SSOT |
| **L7** | UNITY | NO-NEW-SHELL | No new `*.sh` on critical path; use `tri`/`t27c` |

**Law Priority:** L1 > L2 > L3 > L4 > L5 > L6 > L7 > L8 (Asimov-style hierarchy)

---

## 6. Cursor / automation

- Rule file: [`.cursor/rules/t27-ssot-math.mdc`](.cursor/rules/t27-ssot-math.mdc) — keep in sync with **SSOT-MATH** and this entry point.

---

## 7. Crons and skills are specs

A scheduled job is a `.t27` card before it is anything else. Cron cards live in
[`specs/crons/`](specs/crons/README.md) (the README holds the schema and the
rules the generator enforces), skill cards in [`specs/skills/`](specs/skills/),
and the scheduler contract is
[`specs/automation/inngest-queen-scheduler.t27`](specs/automation/inngest-queen-scheduler.t27).
This repository is the only place to edit them.

Before you add, move, list or "migrate" a cron -- a workflow `schedule:`, an
Inngest function, a Railway cron service, a `setInterval`, a launchd plist or a
crontab line on the owner's Mac -- find its card here, and write one if it is
missing. Do not start a second registry (a database table, a JSON list, a
profile tab with its own store). A page that shows crons reads the cards.

Who reads the cards, and what each reader can and cannot tell you:

| Reader | Reads | Tells you | Does not tell you |
|---|---|---|---|
| `app.t27.ai/game/crons` (gHashTag/trinity `CronExplorer`) | a vendored copy, compiled at site build time into `/queen/crons/spec-crons.json` | the card typechecks and the code it names exists | whether the job ran: `health: ok` describes the card, and the page loads no run history |
| The Queen: Inngest app `t27-queen` in gHashTag/BrowserOS `trios/agent-server` | its own vendored copy, pinned to a t27 commit (`GET /queen/scheduler`, field `pin`) | how it serves each card: `HOST github-actions` gets `workflow-dispatch`, every other host gets `tick-only` | anything about a `tick-only` card, whose tick event has no consumer |
| The host (GitHub Actions, the 999 Inngest app, Railway, the process) | its own schedule | whether the job ran and how it ended | -- |

So "is this cron working?" is answered by the host, never by the card or the
page: `gh run list -R <repo> --workflow <file> --event schedule` for
`github-actions`, the 999 Inngest dashboard for `inngest`, the service logs for
`timer` and `railway-cron`. Put what you measured into the card's `NOTE` with
its date, and re-measure a `NOTE` before you repeat it.

Gaps that are open until a PR closes them. Do not assume otherwise:

- **The owner's Mac has no `HOST`.** The vocabulary is `github-actions`,
  `inngest`, `railway-cron`, `timer`. launchd jobs, crontab lines and Claude
  Code scheduled tasks therefore have no card, and neither the page nor the
  Queen sees them. A new host is a schema change in `specs/crons/README.md`
  first, then in the trinity generator (`apps/website/scripts/agents-from-specs.mjs`),
  then in the Queen's `src/inngest/plan.ts`.
- **Dispatched workflows fire twice.** A `github-actions` card the Queen
  dispatches still has its `schedule:` block until a PR removes it, in the
  order fixed by
  [`docs/now/2026-09-13-the-queen-holds-the-scheduler-crons-and-skills-as-one-inngest-app.md`](docs/now/2026-09-13-the-queen-holds-the-scheduler-crons-and-skills-as-one-inngest-app.md).
- **There are three copies.** The trinity site and the Queen each vendor the
  cards. Edit them here, then refresh the copies: trinity
  `apps/website/scripts/sync-t27-specs.mjs`, the Queen
  `trios/agent-server/scripts/sync-t27-specs.sh`. Check the Queen's `pin` after
  a squash merge: a pin to a deleted branch cannot be reproduced.

---

## 8. Developer toolkit: what we built and where it lives

Start here before you write a script or a new tool. Most of what a developer
needs already exists as a `.t27` spec with a front door. This section is a map,
and every row links to the rule's one home. Measured numbers carry their date.

### TRI CLAW: every CI gate behind one front door

TRI CLAW (TRI-27 -- Trinity S3AI) packages this repository's CI gates as a
product. The gate logic is `.t27`, and the delivery is a GitHub App plus a
reusable workflow. Epic #9271.

| Piece | Where |
|---|---|
| Gate inventory (SSOT): every workflow as data, with its class and events | [`specs/claw/gates.t27`](specs/claw/gates.t27) |
| Drift law `claw_status`: workflows directory vs inventory | same spec, lowered to `gen/c/claw/gates.c`, run by the claw-drift step of `gate-topology.yml` |
| Service contract: modes, exit codes, release tag `claw-v*`, check prefix `claw/` | [`specs/claw/service.t27`](specs/claw/service.t27) |
| Auto-fix recipes and the fix engine: which red check gets which deterministic fix | [`specs/claw/recipes.t27`](specs/claw/recipes.t27), [`specs/claw/engine.t27`](specs/claw/engine.t27) |
| Docs, release checklist, competitors | [`docs/claw/`](docs/claw/README.md) |
| The GitHub App | [github.com/apps/tri-claw](https://github.com/apps/tri-claw), id 5166765, installed on all repositories. It is the reviewer-bee identity ([`tools/bees/README.md`](tools/bees/README.md)): bees approve under its name, so a review is never the author approving itself. It also posts the `claw/t27f` check below |
| T27F check `claw/t27f` (#10268): changed specs and their direct importers, judged by their seals | law [`specs/claw/t27f_gate.t27`](specs/claw/t27f_gate.t27), lowered to `gen/c/claw/t27f_gate.c`; run every 10 minutes by the `claw-t27f` job of `.github/workflows/gate-topology.yml` |
| SARIF alerts `claw/sarif` (#10356): a changed spec that does not parse or is unwritten, as a code-scanning alert | adapter [`specs/claw/sarif_feed.t27`](specs/claw/sarif_feed.t27) over [`specs/claw/sarif.t27`](specs/claw/sarif.t27), lowered to `gen/c/claw/sarif_feed.c`; run on every pull request by the `claw-sarif` job of `.github/workflows/gate-topology.yml` |
| Suggestion review (#10365): a changed spec that `t27c fmt` would reformat, as a "Commit suggestion" review | driver [`specs/claw/suggest_feed.t27`](specs/claw/suggest_feed.t27) over [`specs/claw/suggest.t27`](specs/claw/suggest.t27) and the lossless formatter [`specs/tri/fmt_source.t27`](specs/tri/fmt_source.t27), lowered to `gen/c/claw/suggest_feed.c`; run on every pull request by the `claw-suggest` job of `.github/workflows/gate-topology.yml` |

The `claw/t27f` check: one scheduled job in this repository posts it on every
open pull request that touches `.t27`, in each repository the App is installed
on. No consumer repository carries a workflow file. An unsealed spec is neutral
and a stale seal is failure; each summary row links to the spec's T27F graph.
It posts nothing until the owner finishes the four steps in
[`docs/claw/README.md`](docs/claw/README.md#the-t27f-check-in-every-t27-repository-10268):
App permission checks: write, accepting that permission, secret
`CLAW_APP_PRIVATE_KEY`, variable `CLAW_APP_ID`. That section has the status.

Adding a workflow file means adding its row to `gates.t27` in the same commit.
The drift gate refuses anything else.

### T27F: t27's frontend

T27F is how t27 shows anything visual, through several tools: spec diagrams,
map stills, particle reels and stills, the hot-reload stream and the
`claw/t27f` check-run. TRI CLAW serves its MCP tools. Its core is the visual
spec of every `.t27`: one graph model, many views, every node labelled exact,
derived, suggested or unknown. The surfaces and their status are
[`specs/t27f/frontend.t27`](specs/t27f/frontend.t27); the definition lives in
[`.claude/skills/t27f/SKILL.md`](.claude/skills/t27f/SKILL.md), "What T27F is".

- **Specs:** [`specs/t27f/`](specs/t27f/). The IR is `spec_graph.t27`. The
  exporter (`.t27` source to graph) is `spec_graph_export.t27`, and its derived
  edges are `spec_graph_derive.t27`. Fold, expand and hit-test for big specs are
  `spec_graph_fold.t27`. Layout is `spec_graph_view.t27`, and the agent-sized
  text view is `spec_graph_outline.t27`.
- **See it:** [t27.ai/t27f/graph/](https://t27.ai/t27f/graph/), and the graph
  layer of the Spec Explorer at app.t27.ai/game/specs.
- **Read a spec cheaply:** the MCP tool `t27f_outline` returns one line per
  node and edge instead of the whole file. Use it before reading a big spec in
  full.

### t27t: the t27 trainer

t27t is the t27 training product. Its law is
[`specs/t27t/t27t.t27`](specs/t27t/t27t.t27), run with `tri t27t`. Its runner
today is the PyTorch trainer gHashTag/igla-coder-gpu (`train.py`, tern-tc
models). The spec says what the runner must do; it does not rewrite the runner.

- **What makes training cheaper** (measured on an RTX 4090 with CUTLASS 3.5.1,
  2026-10-18, #10360): the forward GEMMs run on int8 tensor cores, at 2.9x to
  3.5x bf16, and the gradients stay in bf16. PyTorch reaches those cores through
  `torch._int_mm`, so no t27c CUDA backend is needed for the saving.
- **What makes it more expensive:** int8 dp4a on CUDA cores runs at 0.91x to
  0.94x of bf16. `pick_path` never chooses it.
- **The saving law:** f x (1 - 1/s), which is 16% to 24%, or $16 to $24 on a
  $100 run. f is the share of a step spent in forward GEMMs, and it is **an
  estimate, not a measurement** (`F_MEASURED = false`). Do not quote the saving
  without saying so.
- **When a generated kernel takes over:** only on a shape where it beats tuned
  CUTLASS s8 by 10% (`GEN_CUDA_BAR_E2`).
- **Open:** the runner is not wired to `torch._int_mm` yet
  (`RUNNER_USES_INT_MM = false`). The spec's `NOT_MEASURED` lists what else was
  not measured.
- **Numbers about our formats** go through the papers check,
  [`specs/numeric/paper_claims.t27`](specs/numeric/paper_claims.t27) (`tri papers`).
  Of 45 numbers printed in three papers, 34 recompute, 8 do not, 3 cannot be
  checked.

### T27 OS: the instrument panel, and GOD MODE evolution

T27 OS is the native window through which the project is worked on and watched: a spaceship
instrument panel, written in `.t27` and built by `t27b` (`specs/t27os/`, run recipe
`docs/t27os/run-local.md`). The image, the per-letter registry, the build traps and the plan live
in one place, the skill [`.claude/skills/t27os/SKILL.md`](.claude/skills/t27os/SKILL.md); this
section only states the rules.

- **27 petals = 27 letters.** Petal index p is letter p + 1; the rings are the alphabet layers
  (A-I, J-R, S-Ti). Petal 3 (C) holds the chat and the commander. A letter's tool t27<letter>
  leads where one exists (t27a, t27b, t27c, t27f frontend, t27o, t27t, t27u); never name a tool
  before its artifact exists (`specs/compiler/theory/toolchain.t27`).
- **T27 OS is an ergonomic project-management system, 100% widgets** (owner, 2026-10-20: "our
  signature"). Each letter is a tool t27<letter> with its own page -- t27k the kanban board, t27c
  the chat and commander, t27i issues, t27p pull requests and CI, t27q the Queen; the table is
  `TOOL_NAMES` in `specs/t27os/agent_parts.t27`. Every pixel of every page belongs to a widget.
- **Everything is a widget; widgets contain widgets.** A letter's console holds its instruments:
  its widgets and app surfaces, its specs, its tri commands, MCP tools and skills, its live
  domain pulse. No text-only screens.
- **Every number shows where it came from**: measured (with the command), derived, or not
  measured; a missing value prints "not measured", never 0 (`specs/t27os/bench.t27`). Every
  fact about a domain is labelled CONFIRMED or GUESS (`domain_pulse.t27`).
- **GOD MODE evolves the system, inside the same laws.** With GOD MODE on, the Queen
  (`specs/t27os/queen.t27`) picks the next gap from the panel itself -- a letter with no
  instrument, a stale seal, a red check on a domain's path, an unowned path, a spec no widget
  shows -- turns it into an issue with the 4-heading shape, starts one agent in a PTY pane on a
  fresh lane worktree (Sonnet; at most three at once on an 8 GB Mac), and lands the result only
  through the normal gates (`Closes #N`, tests, seals, Only t27, DRY). The decision rule is
  `specs/t27os/evolve.t27`. GOD MODE never does what CLAUDE.md "Proactivity" forbids
  (passwords, payments, secrets, public claims without a measurement, owner-only gate files);
  those become requests to the owner. Cmd+J, Escape or the switch stops it; every action is in
  the RECENT log on screen.
- **Two lanes edit `glass_window.t27`.** Merge master before every push to it; reseal
  `specs/t27os/homes.t27` after master moves.

### Hot reload and incremental work: only what changed is redone

The rule: key every result on the content hash of its inputs and reuse it while
the key matches. Seals are those keys.

| Tool | What it does | Measured |
|---|---|---|
| `t27c seal <spec> --save / --verify` | Seal v2 in `.trinity/seals/*.json`: spec hash, the four gen hashes, the `use` closure and the toolchain | -- |
| `t27c frontier [--list] [--reseal] [--audit] [--watch]` | Decides which specs must be rebuilt. The seven decisions are [`specs/verified/seal_identity.t27`](specs/verified/seal_identity.t27). `--watch` polls with a stat gate and reruns the importers | the gen-hash cache (`~/.cache/t27/genhash`) cut a full run from 67.7 s to 3.0 s; a warm run takes about 1 s |
| `t27c use-edges <specs_dir>` | The import graph. [`specs/ci/affected.t27`](specs/ci/affected.t27) turns it into "which specs does this diff touch" | -- |
| T27F hot loop | [`specs/t27f/hot.t27`](specs/t27f/hot.t27). The reuse key is sha256(spec) plus sha256(exporter wasm). Debounce is 20 ms quiet and 60 ms at most, and the budget is 100 ms per save | -- |
| `node server.mjs --watch <dir> --port 2727` in [gHashTag/t27-mcp](https://github.com/gHashTag/t27-mcp) | Watches any directory of `.t27` files, in any repository. `GET /events` streams one `t27f` event per changed spec; `GET /graph?path=` returns the graph or a refusal | save to event: p50 24 ms, max 59 ms (2026-10-18) |

Measured cost, 2227 specs (2026-10-18, #10232): exporting a graph takes
milliseconds (p50 0.09 ms, worst 18.8 ms). The slow parts were the network and
manual rebuilds, so fix those before you optimise the exporter.

### The t27 MCP server: reuse a validated spec before writing one

[gHashTag/t27-mcp](https://github.com/gHashTag/t27-mcp) serves the published
corpus to agents. Its README lists every tool. The ones that save the most work:

- `t27_find` and `t27_specs` find an existing spec by intent or filter.
- `t27_spec` returns one spec's record, its source and its seal.
- `t27_check` parses and typechecks with the real front end and gives hints for
  foreign syntax.
- `t27_examples` returns short worked idioms, and `t27f_outline` /
  `t27f_graph` show a spec's structure.

DRY (above) says: search before adding. This server is the fastest way to do it.

### Gates you will meet on a pull request

Each gate is a spec. Read the spec, not a summary of it.

| Gate | Rule | What trips it |
|---|---|---|
| Only t27 / SHRINK | `specs/policy/own_language.t27` (section "Only t27" above) | a new foreign file, or more foreign lines added than deleted |
| Issue shape | [`specs/policy/issue_shape.t27`](specs/policy/issue_shape.t27) | an issue without Boundary / Scenarios / Requirements / Criteria; it is closed as `shape-refused` |
| PR budget | [`specs/policy/pr_budget.t27`](specs/policy/pr_budget.t27) | more than 20 open PRs per author; batch port waves per ring or crate |
| Compiler preflight | [`specs/tri/pr/preflight.t27`](specs/tri/pr/preflight.t27) | a bee PR whose specs do not pass the compiler |
| Duplicate bodies | `tools/duplicate_bodies_baseline.txt` (DRY above) | a copied function body |
| Gate drift | `specs/claw/gates.t27` | a workflow file without its inventory row |

### Traps we paid for

- **gen-c and strings.** `.len` on `str` reaches C verbatim, so it cannot appear
  anywhere in a spec that is lowered by gen-c, test bodies included. `==` on
  `str` is a byte compare in Zig but a pointer compare in C. The cross-backend
  idiom is explicit length parameters, a `str_eq` loop function, and length
  pins in a Zig-only companion module (`specs/claw/gates_pins.t27`). Filed on
  #5980.
- **A parse is not a proof.** A typecheck stays green for `assert 1 > 2`. Only
  the spec's own `test` blocks, run to completion with 0 vacuous, show it is
  true.
- **Heavy runs do not go on a laptop.** Builds, corpus sweeps and mutation runs
  go to the Railway lab. A local Mac run has panicked the machine
  (2026-10-09).
- **A merge is not a deploy.** A trinity merge is live on t27.ai only after the
  `gHashTag/ghashtag.github.io` publish workflow runs and its Pages build is
  `built`.

---

**φ² + 1/φ² = 3 | TRINITY**

---

## Only t27 (owner hard rule, 2026-10-05)

Owner's rule, 2026-10-05: no new commit or pull request may contain code in
another language; the hooks must stop it. It binds people, bees and agents alike.

- **Allowed:** `.t27` specs; files `t27c` generated under a generated root
  (`gen/`, `bootstrap/gen/`, `bootstrap/src/memory/generated/`); prose and data
  (`.md`, `.txt`, `.json`); and **deletions**.
- **Denied:** adding a new hand-written file in any other language --
  `.rs .py .ts .js .sh .zig .c .go .v .lean .yml .toml`, Dockerfile, Makefile and
  the rest of the list in the spec.
- **Allowed without the label since 2026-10-18:** editing a foreign file that
  already exists (`bootstrap/src/compiler.rs`, `cli/t27b`, a workflow `.yml`).
  Owner rule, 2026-10-18 (translated): the label is not for every pull request,
  only for the most important files. The edit is still held by `check_budget()`:
  at most 40 added foreign lines per file and 80 per pull request, and never more
  added than deleted. No label lifts that.
- **The rule is** `specs/policy/own_language.t27`; its `t27c gen-c` output
  `gen/c/policy/own_language.c` runs in `lefthook.yml` (pre-commit, pre-push)
  and in `.github/workflows/own-language.yml` on every pull request. Read the
  spec for the exact list; do not copy it here.
- **The only override** is the label `owner-approved-foreign`, and it is needed
  only for the most important files: a new foreign file, and the gate itself
  (the spec, its generated C, `lefthook.yml`, `.github/workflows/own-language.yml`,
  `tools/policy/foreign-exceptions.txt`). It is applied only on the owner's
  explicit approval, and the pull request body quotes that approval (in English,
  with its date and where it was given).
- **Owner-approved exceptions** live in `tools/policy/foreign-exceptions.txt`:
  data, one path prefix per line, each under a comment naming the approval. On
  an approved branch, add the entry in the same branch; the local hooks read the
  working-tree list, so the commit goes through. CI reads the list from the pull
  request's base, and any change to the list file needs the owner's label, so
  every entry on master was approved by the owner. There is no env-var bypass.
- **Existing foreign code is debt** that only shrinks: by deletion, or by
  replacing it with a spec and its generated output.
- **`--no-verify` is forbidden**, and so is `LEFTHOOK=0`. Install the hooks
  once per clone with `lefthook install`.
- What gen cannot express yet is a compiler defect: file it on the self-host
  epic (#5980) instead of writing the code by hand.

## t27b is written in t27

Owner's rule, 2026-10-05: t27b -- the native backend -- and the tools that
measure it are written in **t27**, not in Rust or Python by hand.

- New t27b logic starts as a `.t27` spec with `test` blocks and reaches Rust
  or Python only through `t27c gen-rust` / `gen-c` / `gen-js`. The generated
  file is never hand-edited (L2).
- What gen cannot express yet is a defect of the self-host work (#5980): file
  it there. Do not work around it in a hand-written file.
- A new hand-written file is denied by the gate in "Only t27" above unless the
  owner labels the pull request `owner-approved-foreign`; an edit to an existing
  one passes within the budget. Either way the PR names the spec that will
  replace it and links the port epic #6198.
- The debt only shrinks, and the gate enforces it: since #8236 `check_budget()` refuses
  any pull request that adds more hand-written foreign lines than it deletes. On
  2026-10-10 (master 1a7c8b939, `wc -l`) it is `cli/t27b/src/**/*.rs` 15627 lines and
  `cli/t27b/tests/*.rs` 8349, against 8004 generated in `gen/rust/tri/t27b`. Measure it
  again with `wc -l` on master; its history is `git log`. A pull request does not append
  its counts here any more: that hand-kept log made every parallel t27b lane conflict on
  this paragraph (#8587).

This is the "Own language first" rule below, applied to code: a project whose
claim is "here is a language worth writing" writes its own backend in it.

**A lane's evidence (#7672).** A t27b coverage PR proves "nothing regressed"
with two signed corpus receipts, not a pasted table of lab numbers.

- Base: `git merge-base origin/master HEAD`. Head: the PR's head, pushed to a
  branch on origin. Take both receipts from the SAME lab: per-file timeouts differ
  between the two machines, so a cross-lab pair shows changes no commit made.
  Use t27b-lab-2 (`https://t27b-lab-2-production.up.railway.app`): it runs an
  admitted request before a moved master, up to 4 in a row (#9026), where
  t27b-lab runs master first. Use `<lab>/runs/<sha>.receipt.json` when it exists;
  otherwise request it with a fresh challenge you keep, and request the base too
  when that lab has no receipt for it:
  `c=$(openssl rand -hex 32); ssh -o BatchMode=yes 805f6f82-71d1-40d2-88bd-004e271be5ef@ssh.railway.com "mkdir -p /srv/requests && printf %s $c > /srv/requests/<sha>"`
  (the user is t27b-lab-2's Railway service instance; `ssh t27b-lab` reaches lab 1).
  On a host-key mismatch, retry up to 10 times; never edit known_hosts. At most 8
  requests wait, dropped after 6 hours; a run takes about 15 minutes and publishes
  `/runs/<sha>.json` and the receipt.
- From a master checkout's root: `t27c corpus-receipt compare BASE.json HEAD.json
  --challenge <base's> --challenge-head <head's>` (omit a challenge you did not
  write). Paste its output and exit code: `lane` lines name each changed file.
- Exit 3 IMPROVED_ONLY, 0 EQUIVALENT or 4 NEUTRAL may merge. Exit 1 REGRESSED
  blocks the merge; exit 2 REFUSED means the evidence did not authenticate, so
  it proves nothing. The rules are `specs/verified/corpus_receipt.t27`.

## The matryoshka: every layer is generated from t27 and compiles itself (owner rule, 2026-10-11)

The stack is a set of nested layers. Each layer meets three conditions:
1. **Source in t27.** Its logic is written as `.t27` specs with `test` blocks.
2. **Generated by the layer below.** Its code comes out of the previous layer. Hand-written glue only shrinks (#8236).
3. **Proves itself.** It compiles its own `.t27` source and passes its own tests. Building it twice, once with the layer below and once with itself, gives bit-identical output.

The layers today:
- **L0, seed.** `t27c` is hand-written Rust (`bootstrap/src`). It exists only to start the chain, and #5980 P5 retires it.
- **L1, t27core.** The compiler written in t27 (`specs/compiler/core/t27core.t27`), built by `t27c gen-c`. It must write the same bytes `t27c` writes: P0-P3 are done, P4 covers the other backends, P5 replaces the seed.
- **L2, t27b.** The native AArch64 backend. Its decisions live in `specs/tri/t27b/*.t27`, reach Rust through `t27c gen-rust`, and are mounted with `#[path]` (#6198).
- **L3, t27b compiles itself.** t27b runs the tests of the specs it is generated from: `type_text`, `source_text`, `eval_arith`, `i128_plan`, `a64` and `libm` pass under `t27b test --check`.
  - Next, t27b emits the objects for its own generated modules instead of `gen-rust`. Then no Rust stands between a `.t27` spec and the AArch64 that runs t27b.
  - The last step is for t27b to build t27b.
- **L4, hardware.** The Verilog path is decided in `.t27` plans (t27b verilog, t27c silicon), so the die is generated from t27 as well.

How each layer is checked:
- **Fixed point.** Stage N, built by stage N-1, builds stage N+1. Stages 2 and 3 must be bit-identical.
- **DDC.** Diverse double compiling with an independent front end shows the seed hides nothing. See #7706 and #6488 C4.
- **Receipts.** A signed corpus receipt is made for every master commit (#7576). A layer's claim is a receipt, not a table pasted into a PR.

For agents:
- A new layer, tool or service follows the same shape: spec first, generated, self-compiling, with receipts.
- When a layer cannot yet compile its own source, file the gap as an issue under the layer's epic (#5980 for the compiler, #6198 for t27b). Never paper it over with hand-written code.
- A layer is done when its own `.t27` passes under itself and two builds of it are bit-identical (#8920).
- **t27o (Ouroboros)** converts code in other languages into `.t27` (`specs/tools/t27o/rust_convert.t27`, Rust first; the functions behind its Rust std table, each with its I/O proven against Rust, in `rust_std.t27`; run it as `tri t27o <file.rs> [--oracle] [--check]`), so foreign layers are eaten into the matryoshka instead of rewritten by hand.

## DRY: never add what already exists (owner rule, 2026-10-11)

Every spec is a running cost: a seal, a ledger row, CI minutes on every pull request, and tokens for every agent that reads it. So:
- **Search before adding a spec or a function.** Use `t27c dupes --name <fn>` / `--bodies`, `python3 tools/dupe_scan.py --name/--like <spec>` and `specs/tools/catalog.t27`. If a match exists, `use` it or extend it. Never copy it.
- **Extend before creating.** Prefer extending an existing plan or conformance spec over a new file. Keep one conformance spec per area, not one per feature.
- **t27o (the converter) reuses.** It reuses an existing t27 function with the same input/output contract instead of emitting a new body, and reports how many it reused.
- **Duplicates only go down.** `tools/duplicate_bodies_baseline.txt` only moves down. A pull request that adds a duplicate body is wrong even where the check is not required (#8963).

## Actors, not threads (owner rule, 2026-10-08)

Owner's rule, 2026-10-08: the Queen and the swarm move to the actor model.
Tracking: #7851.

- **Everything that runs at the same time is an actor.** That covers a round,
  a reviewer, a poller and a runner lane. Each one has a pid and a bounded
  mailbox, and a supervisor restarts it. The rules are one spec,
  `specs/queen/actors.t27`, built on BEAM/OTP: links, monitors, a control lane,
  one_for_one / one_for_all / rest_for_one, restart intensity and backoff. A
  runtime reaches those rules only through the spec's generated card.
- **No new loops.** Queen or swarm work gets no new `setInterval` timer, thread
  pool, worker counter or ad-hoc promise fan-out. A new concurrent piece is a
  new actor under a supervisor.
- **Replace in this order, never another:**
  1. An MVP of the actor runtime runs the same workload.
  2. Its tests pass.
  3. A benchmark runs the current loop and the actor runtime on the same input,
     measuring throughput, latency, recovery from a crashed worker, and work
     lost.
  4. The numbers are posted on #7851.
  5. Only then does the swap happen, and only if the actors are at least as good
     on every measure.

  Never swap first and measure later.
- **Where the code lives.** The Queen server lives in `gHashTag/trios`, on
  branches `queen` and `queen-runners`. `gHashTag/BrowserOS` is a fork of
  someone else's project. It holds the browser extension and nothing else: no
  Queen code, no server code, no specs, no plans.

This section is the rule's one full home. The AGENTS.md of our other
repositories point here; they do not copy it.

## Own language first

When this project publishes something about itself, it publishes in **this
project's own language and format** -- not translated into somebody else's.

Owner's rule, 2026-09-20: stop writing in other people's languages, we have our
own.

This bites on any file whose only reason to exist is that an outside tool
expects that shape: `llms.txt`, `agents.json`, `ai.txt`, `.well-known/*.json`,
A2A agent cards, `ai-plugin` manifests, OpenAPI stubs, JSON-LD blocks, a README
that restates a spec. The reflex is to write four of them in four foreign
formats, and the reflex is wrong: a project whose claim is "here is a language
worth writing" and which then describes itself in three of other people's
formats has published three documents that are not true of it.

**The move:** find the address the outside world already fetches, then serve our
own language at it. `/llms.txt` at t27.ai **is** a t27 module -- `llms.txt`
requires nothing but text, and every prose line of a `.t27` file is a `;`
comment, so it stays readable to anything that cannot compile it.

**Three qualifications, so the rule stays honest:**

- A format a resolver genuinely parses -- a sitemap, `package.json`, a lockfile
  -- is machinery, not a description. **Generate** it from our own source; never
  hand-write it into a second home for the truth.
- Code against someone else's API uses their types. Prose for a human who has
  never heard of the project uses that human's language.
- If a format demands a claim we cannot back, **publish nothing**. An A2A card
  with no A2A server behind it is a false claim, and a missing file is more
  honest than a lying one.

The test: *is this file the project speaking about itself?* If yes, it speaks
our language. If it is plumbing, it speaks the plumbing's.

**Worked example, compiler-checked rather than asserted:** in `gHashTag/trinity`,
`apps/website/public/t27/files/specs/catalog/onboarding.t27` generates
`/llms.txt` and `/agents.t27` byte-identically, gated in CI as
`check:onboarding`. The generator evaluates the spec's own `test` blocks --
`typecheck.ok` stays true for `assert 1 > 2`, so a compiler saying "this parses"
is not a compiler saying "this is true" -- and re-compiles the rendered document
before writing it.

**The full rule lives in exactly one place: the `own-language-first` skill**
(`~/.claude/skills/own-language-first/SKILL.md`). It carries the consent gate for
documents addressed to other people's agents, the six negative controls, and the
`;`-alone-on-a-line trap that silently discards a `module` declaration. This
section is a pointer, not a copy -- the recorded defect in this codebase family
is the hand-copied rule that only two of its three homes knew about.

## New functionality is written in .t27 only

**Owner, 2026-10-11:** new functionality is added on `t27c` / `t27b`, and the
outdated logic that lives outside `.t27` is abandoned, not extended.

1. **Spec first, `.t27` only.** A new rule, number, threshold, limit, price,
   access decision or order of steps is written as a `.t27` spec and executed
   by `t27c` / `t27b`. It is not written in TypeScript, Rust, Python or
   hand-written Zig.
2. **Existing logic outside `.t27` is legacy.** It is not extended. When it is
   touched for a fix, the rule moves into a spec first and the host keeps only
   plumbing.
3. **Plumbing may stay in the host language**: transport, HTTP, database
   drivers, rendering, wiring. Plumbing never decides a rule. A host function
   that only looks like a rule is a defect, not plumbing.
4. **Concurrency uses the actor model of `specs/queen/actors.t27`** (pids,
   mailboxes, links, monitors, supervisors), not hand-written threads or
   queues.
5. **A claim that `t27b` can host a runtime is verified by running it.** Where
   it cannot yet, the existing host stays as a thin transitional host and the
   gap is recorded as an issue, not hidden.
