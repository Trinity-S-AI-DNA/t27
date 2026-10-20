---
id: t27os
name: T27 OS
description: T27 OS is the spaceship instrument panel for working on t27 -- a native macOS window written in .t27 and built by t27b: the 27-petal home, the chat (petal 3), the home tabs t27a/t27b/t27c/t27f/t27o/t27t/t27u + Queen, one console per agent letter with its widgets as instruments, end-to-end analytics per agent domain, and the BENCH of the three FPGA boards. Use when the user says "T27 OS", "t27os", "приборная панель", "дома агентов", "лепесток", "консоль агента", "запусти локально", or asks to change, build, run or screenshot the T27 OS window.
---

# T27 OS

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

## The product (owner, 2026-10-20)

An ergonomic project-management system made 100% of widgets -- our signature. Each letter is a
tool t27<letter> with its own page (t27k kanban, t27c chat + commander, t27i issues, t27p PRs and
CI, t27q Queen, t27s specs, t27w widgets, ...; table in agent_parts.t27 TOOL_NAMES). The work loop:
i (issues) -> k (kanban) -> c (task an agent) -> p (PR, CI) -> v (verdict) -> n (now) -> e
(experience), and q (the Queen in GOD MODE) turns it on her own. A petal opens straight into that
tool's page, the agent's dashboard open; nothing on a page is drawn outside a widget.

## The image (owner, 2026-10-20)

We sit in a spaceship; T27 OS is its **instrument panel** for working on the project.

- **Everything is a widget, and widgets contain widgets.** Screen > home > hero gauges + agent consoles > instruments. One recursive layout packs children into the parent's rect.
- **Many widgets on screen**: 20-40 instruments in a 960 x 540 window, more in full screen. Never a text list, never a nine-cell honeycomb (both were rejected).
- **One console per agent letter** (27 agents, `specs/agents/*.t27`), titled by letter + domain (e.g. `C  COMPILER CORE`); its instruments are the widgets, app tabs and pages assigned to that letter by domain.
- **Look**: the chat screen's sidebar and glass; headings like the website course (giant 800-weight title, segment bar, outlined big numbers, course map); cards like the widgets gallery (card.png, click = the widget live inside the window).
- **End-to-end analytics per domain**: what is happening in each agent's domain (commits 24 h / 7 d, last activity, 7-day pulse, lamp ACTIVE / WARM / QUIET / BLOCKED, default action), every fact labelled CONFIRMED or GUESS, an OPEN count of unowned files (idea from the REA + Opus 5.5 reverse-engineering posts).
- **For hardware engineers**: the BENCH (three AX7203 dice live), provenance of every number, "not measured" instead of 0, keyboard first, idle at no CPU.

## Every widget carries its agents (owner, 2026-10-20, mandatory)

No widget, tab or instrument without its pinned agents: the owner letter plus the letters that feed
it, each with a one-line reason, in `specs/t27os/agent_parts.t27`. A test refuses a widget with no
agent, and the widget factory (`specs/t27f/widget_factory.t27`) refuses a new widget without them.
On screen every instrument shows its agent badges (owner first, bold); the live widget view shows
an `AGENTS:` line. Purpose: make the system's hard-to-follow work transparent to a human.

## Where the law lives (one home each, never restated)

| What | Spec |
|---|---|
| Petals, names, hit test; CHAT is petal index 2 (Cmd+3) | `specs/t27f/petal_menu.t27` (`WORLD_NAMES` is the only copy) |
| Homes t27a..t27u + Queen, letters per home, widget runs, 7 courses x 27 lessons, home petals 19..25 | `specs/t27os/homes.t27` |
| Every gallery widget (195) and app surface (65: 21 HUD views, 10 TRI screens, 34 pages) -> one agent letter, console labels, letters with no instrument | `specs/t27os/agent_parts.t27` |
| What is happening per domain (git pulse, lamp, CONFIRMED/GUESS, OPEN) | `specs/t27os/domain_pulse.t27` (paths -> letters through `use agents::<l>::KEY_FILES`) |
| Boards, provenance of every number, project summary strip | `specs/t27os/bench.t27` (+ `specs/verified/bench_boards.t27`) |
| The window: screens, sidebar helpers shared by chat and home tabs, scrolling, live widget pane | `specs/t27os/glass_window.t27` |
| T27F surfaces T27 OS can host (spec graph, outline, check, particles, dashboard) | `specs/t27f/frontend.t27` |

## Build and run locally (verified 2026-10-20)

1. `t27b` needs the extern boundary of #8946, not on master: a worktree from master with
   `git cherry-pick --no-commit 865b4ee3f 6113f868b`, then `cargo build --release -p t27b`.
2. Build: `t27b build specs/t27os/pty_server.t27 -o pty_server.o --macho`, the same for
   `glass_window.t27`; `nm -u pty_server.o` must list symbols (an empty list = trapping stubs).
3. Link the window with `-lobjc` and the frameworks Foundation, AppKit, CoreGraphics, CoreText,
   CoreFoundation, QuartzCore, **Security** (keys) and **WebKit** (live widget pane); ad-hoc
   `codesign`; a hand-made Info.plist (`ai.t27.os`). Details: `docs/t27os/run-local.md`.
4. Run from a normal Terminal tab (the Bash sandbox blocks `open`, `kill` and the socket):
   `PATH=$HOME/.local/bin:$PATH <out>/t27os-pty &`, then `open <out>/T27OS.app`.
5. Look before you claim: screenshot the window (computer-use `app_screenshot` on `ai.t27.os`).
   The home logo is drawn at about 0.705 pt per box unit: petal p centre =
   `(480 + (cx - 298) * 0.705, 76 + cy * 0.705)` with cx, cy the vertex mean of `QX/QY` in units;
   take two screenshots after a tap, the first one is often mid cross-fade.

## Traps measured

- Two sessions edit `glass_window.t27` (the window and the keys lanes): `git fetch` and merge master
  before every push; reseal `homes.t27` after master moves (its gen hashes follow its imports).
- `--auto` merge can land a PR before a redesign: do not enable it while the owner is still judging the look.
- `t27b` rejects `;` comment lines and apostrophes inside an array literal.
- `KEY_FILES` of the agent specs drive the analytics; stale paths make every domain look idle.
- The sandbox cannot create worktrees or write `.git/config`: create them from a Terminal tab.
- Use Sonnet subagents for the work, at most three at once on this 8 GB Mac; the orchestrator
  merges, builds, launches and screenshots.

## The 27 petals are the 27 agents (owner, 2026-10-20)

Petal index p = agent ordinal p + 1. The realms match the alphabet layers: RAZUM petals 0-8 =
Archetypal A-I, MATERIYA 9-17 = Spiritual J-R, DUKH 18-26 = Physical S-Ti. A petal opens that
letter's console. **Petal 3 = C: the chat and the commander live here** (driving the agents is
compiling intent into work), together with the compiler core. Homes (t27a..t27u, Queen) are
groups in the console sidebar, not petals.

What each console needs. "Source" = where its live data comes from today; "gap" = what is missing.

| # | Letter, domain | Instruments (widgets inside the console) | Actions | Source / gap |
|---|---|---|---|---|
| 1 | A Architecture / ADR / SOUL | law map L1-L7 with violations, ADR list + status, constitution diff | open ADR, check laws | SOUL.md, architecture/ADR-*; gap: no law-violation counter |
| 2 | B Build / Pipeline | CI lamp per workflow, queue length, build time, red set vs master | rerun, open red log | specs/claw/gates.t27, gh checks; gap: host must poll gh |
| 3 | C Compiler Core + CHAT + COMMANDER | chat panes of all agents (honeycomb), commander line (Cmd+K: tri/t27c verbs), compiler stages t27c-stages, gen backends, seal status | send to agent, run t27c verb | pty server, glass_window chat; widgets t27c-* |
| 4 | D De-Zigfication | foreign lines left (Rust/Zig/TS), port progress, t27o sweep | open port task | specs/tools/t27o, tools/policy; gap: one counter spec |
| 5 | E Experience / XAI | experience log stream, lessons learned, why-cards | save experience | .trinity/experience, why_card.t27 |
| 6 | F Formal Conformance | conformance pass/fail, vacuous passes, test-report | run tri test | conformance/, t27b test; 22 widgets |
| 7 | G Graph / ArchBench | SPEC MAP (T27F spec graph), dependency fan-in/out, archbench | open spec graph | spec_graph*.t27, t27f_graph |
| 8 | H HSLM / NN | model architectures, training runs (t27t), loss curves | open run | specs/t27t, specs/hslm; see t27t study |
| 9 | I ISA / Registers | register file, encodings, t27a assemble/disassemble | assemble a line | specs/isa/t27a.t27; gap: no widget yet |
| 10 | J Jobs / Task Routing | issue queue, claims, lanes | claim, route | specs/queen, gh issues |
| 11 | K Kernel / FPGA MAC | BENCH (3 dice), bitstreams, timing, pins, LUTs (64 widgets) | flash, detect | bench.t27, bench_boards.t27, camera |
| 12 | L Language / Syntax | grammar coverage, unparsed constructs, fmt suggestions | format spec | tri unparsed, fmt_source.t27 |
| 13 | M Metrics / Telemetry | domain_pulse for all letters, CPU/RAM of the Mac, swarm stats | open pulse | domain_pulse.t27 |
| 14 | N Numeric / GoldenFloat | gf16 calc, format table, paper claims 34/8/3 (43 widgets) | recompute claim | gf16.t27, paper_claims.t27 |
| 15 | O Orchestration / Phases | PHI LOOP phase per lane, ring state | advance phase | specs/queen phases; gap |
| 16 | P Physics / SacredPhysics | identity checks phi^2 + phi^-2 = 3, formula catalog | verify identity | specs/physics, specs/sacred |
| 17 | Q Queue / Scheduling | crons (specs/crons), scheduler queue, next runs | pause/resume | specs/crons |
| 18 | R Runtime / Bootstrap | stage0 FROZEN_HASH, bootstrap build, toolchain versions | rebuild t27c | bootstrap/, toolchain.t27 |
| 19 | S Specs / Standards | spec count, seals fresh/stale, duplicates ledger | reseal, dupe scan | .trinity/seals, dupe_scan |
| 20 | T Queen / Lotus | Queen panel, GOD MODE, suggestions, swarm board | confirm/dismiss | queen.t27 |
| 21 | U Universe / Domains | the 27-letter map itself, homes, ownership gaps (OPEN) | reassign path | agent_parts.t27, homes.t27 |
| 22 | V Verdict / Bench | verdicts, bench results, mutation score | run verdict | t27c verdict, tri mutate |
| 23 | W Workflow / tri cell | tri command palette, recent tri runs | run tri verb | specs/tools/tri |
| 24 | X External / Interop | MCP servers, bindings, providers, keys (KEYS tab) | test provider | t27mcp, keys.t27, router.t27 |
| 25 | Y Yield / DePIN | lab nodes, XP earned, hosting | join lab | specs/lab, depin; flags off |
| 26 | Z Zero-Touch UX / Docs | t27u university: 7 course maps, lessons, blog | open lesson | courses.t27 (trinity) |
| 27 | Ti Security / Reserve | secrets scan, key rotation, identity | rotate key | gitleaks, keys.t27 |

## One registry per letter (owner, 2026-10-20)

Each of the 27 letters gathers, with no copy of any list: its TOOL t27<letter> (t27a assembler,
t27b native compiler, t27c compiler, t27f FRONTEND, t27o converter, t27t trainer, t27u university;
other letters: inventory pending), its agent (specs/agents), its specs (domain_pulse letter_of_path),
its widgets and app surfaces with owner + feeders (agent_parts), its tri commands and MCP tools
(the agent's TOOLS field; cards in specs/tools/tri (53), specs/tools/mcp (11), specs/t27mcp
tool_schema (20 t27_/t27f_ tools)), and its skills (the agent's SKILLS field; specs/skills,
.claude/skills). The registry is an extension of agent_parts.t27, not a new spec.
On screen: a console's command strip runs its tri commands and MCP tools in a PTY pane inside the
window (output kept in the console log, numbers flow into instruments labelled measured + command);
skills are playbook buttons that start an agent in the C chat with that skill; the commander on C
(Cmd+K) is a palette of every command and tool with its agent badge; empty letters show their gap.
Open question for the owner: where a tool letter and an alphabet agent disagree (A assembler vs
Architecture, B native compiler vs Build, F frontend vs Formal Conformance, T trainer vs Queen,
U university vs Universe), the default is that the tool decides the letter.

## The t27<letter> tools that exist (inventory 2026-10-20, origin/master + all local repos)

| Tool | What | Law / path | Front door | Status |
|---|---|---|---|---|
| t27a | assembler / disassembler | specs/isa/t27a.t27 | t27c asm, t27c disasm | live |
| t27b | native compiler (AArch64 JIT, Mach-O, Verilog) | cli/t27b | t27b test/build/verilog | live |
| t27c | bootstrap compiler + sweep | bootstrap/src/compiler.rs | t27c, tri test | live |
| t27f | FRONTEND (spec graph, outline, map stills, petal home) | specs/t27f/frontend.t27 | MCP t27f_*, claw/t27f, t27.ai/t27f/graph | live |
| t27o | Ouroboros: foreign code into .t27 | specs/tools/t27o/rust_convert.t27 | tri t27o | live (Rust first) |
| t27t | trainer (law; runners igla-coder-gpu train.py, specs/ml/train_hslm.t27) | specs/t27t/t27t.t27 | tri t27t | live spec; F_MEASURED=false, RUNNER_USES_INT_MM=false |
| t27u | university, 7 courses x 27 lessons | trinity apps/website/specs/course/courses.t27 | t27.ai/learn | live (law in trinity) |
| t27os / t27mcp | the OS window / MCP service | specs/t27os, specs/t27mcp | T27 OS app, t27_* tools | live |

No tool exists for d, e, g, h, i, j, k, l, m, n, p, q, r, s, v, w, x, y, z (hits for t27x/t27w/t27s/t27z
are false: a refused extension, logs, temp names). specs/compiler/theory/toolchain.t27:643 refuses
inventing a t27d..t27z series without an artifact: do not name a tool before it exists.
t27t gaps (live console for H/N): no live run feed (runner private), no model registry (own_model.t27
is a map), HTOK data and checkpoints host-local in /tmp/hslm; real today: GEMM bench, saving with its
estimate label (bind to F_MEASURED), paper_claims, dataset/igla-coder MANIFEST.

## Plan (open items, 2026-10-20)

1. Cockpit dashboard in the home tabs: nested widget tree, agent consoles, card.png instruments,
   live widget pane (WKWebView), t27u course maps (issue #11275).
2. Wire `agent_parts.t27` and `domain_pulse.t27` into the consoles; fix `KEY_FILES` to real paths.
3. T27F instruments in the consoles: SPEC MAP (spec_graph_view drawn natively), CHECK lamp, particles
   from the pulse; register T27 OS as a T27F host in `frontend.t27`.
4. BENCH tab drawn from `bench.t27`; project summary strip on T27B/T27C.
5. Idle CPU to zero (#11025): redraw on events only.
6. Promotion, only after the owner's Telegram yes: an X Article "T27 OS + Opus 5.5, full setup
   included", a 30-45 s video petals -> console -> live widget -> board blinking; no claim
   without its command.
