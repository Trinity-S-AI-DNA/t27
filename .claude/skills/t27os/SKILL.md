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

## The image (owner, 2026-10-20)

We sit in a spaceship; T27 OS is its **instrument panel** for working on the project.

- **Everything is a widget, and widgets contain widgets.** Screen > home > hero gauges + agent consoles > instruments. One recursive layout packs children into the parent's rect.
- **Many widgets on screen**: 20-40 instruments in a 960 x 540 window, more in full screen. Never a text list, never a nine-cell honeycomb (both were rejected).
- **One console per agent letter** (27 agents, `specs/agents/*.t27`), titled by letter + domain (e.g. `C  COMPILER CORE`); its instruments are the widgets, app tabs and pages assigned to that letter by domain.
- **Look**: the chat screen's sidebar and glass; headings like the website course (giant 800-weight title, segment bar, outlined big numbers, course map); cards like the widgets gallery (card.png, click = the widget live inside the window).
- **End-to-end analytics per domain**: what is happening in each agent's domain (commits 24 h / 7 d, last activity, 7-day pulse, lamp ACTIVE / WARM / QUIET / BLOCKED, default action), every fact labelled CONFIRMED or GUESS, an OPEN count of unowned files (idea from the REA + Opus 5.5 reverse-engineering posts).
- **For hardware engineers**: the BENCH (three AX7203 dice live), provenance of every number, "not measured" instead of 0, keyboard first, idle at no CPU.

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
