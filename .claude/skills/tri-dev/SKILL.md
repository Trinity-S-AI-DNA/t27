---
name: tri-dev
description: Bring up the T27F dev environment and SEE the course HUD in the Browser pane after every iteration -- serve the rendered frames, open them in the pane, screenshot, compare against the diff frames, check the free model tiers. The single tri dev command is proposed and not built yet; this skill gives the moves that exist today, measured 2026-11-08, and what each missing piece needs. Use for "tri dev", "dev env", "show the HUD", "visual update", "I want to see the course", "screenshot the course", "start the dev server".
---

# tri dev (proposed, not built yet): see the course HUD

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

The goal (owner, 2026-11-08): one command brings everything up, and after every
iteration the owner SEES the course HUD in the browser. Course epic: #13251.

No HTML page, loader or render script is written for this. The HUD frames are
images; a static file server serves them and the Browser pane shows them.

## What exists (measured 2026-11-08)

| Piece | State | How |
|---|---|---|
| HUD render | exists, heavy | `specs/course/course_hud_*.t27` renders `target/course/hud/hud_NN.ppm` plus `_diff` frames; a render is a heavy run, so it goes to a T27 OS pane, never the Mac shell |
| WASM module | generated | `t27c gen-js` on `course_hud_wasm.t27`; building it is heavy too |
| ppm to png | OK | `ffmpeg -y -loglevel error -i hud_00.ppm hud_00.png`, one literal file per command |
| Dev server | OK on 8027 | `python3 -m http.server 8027 --bind 127.0.0.1 --directory target/dev/hud`, run in the background |
| Browser pane | OK | `preview_start` with `url` `http://localhost:8027/hud_00.png` |
| Playwright MCP | registered, FAIL | needs Google Chrome; none installed |
| GitHub | OK through `gh` | no GitHub MCP registered (it needs a PAT, which an agent may not handle) |
| DeepSeek worker | up through dsh web | `127.0.0.1:3080` answers 401 (auth on); open it with the `dev-monitor` skill; no DeepSeek MCP |
| Model tiers | probe, codes only | see "Health" below |

Port 8080 is held by another project's dev server on this Mac. Do not stop it;
the t27 dev server uses 8027.

## The moves, today

1. Render: hand the HUD render to a free T27 OS pane (`t27os-orchestra` skill).
   Never run it in the Mac shell (the mac-no-heavy hook refuses it).
2. Convert each frame you want to see to png into `target/dev/hud/`
   (gitignored), one ffmpeg command per frame.
3. Serve `target/dev/hud/` on 8027 as a background task, once per session.
4. Open the frame in the Browser pane; take a screenshot with the pane's
   `computer` screenshot action.
5. Before saying what changed, open the `_diff` frame of the same index and
   read the diff counts from the render's `diff.txt`. A screenshot that "looks
   right" is not a measurement.
6. Keep a screenshot the owner should see in `docs/course/hud/`; commit it only
   after the owner has looked at it.

## Iteration loop

change the spec -> render in a pane -> ffmpeg the changed frames -> reload the
pane -> screenshot -> open the `_diff` frame and the diff counts -> say what
changed, with the numbers.

## Health (key names only, never a value)

- **z.ai**: one request per key with `max_tokens` 1 to `glm-4.7-flash`, output
  is the HTTP code only. Pull one `ZAI_KEY_N` line from the gitignored env file
  with grep, turn it into a header with sed and pipe it to `curl -H @-`, so the
  value never reaches the screen. 200 is live, 529 is overload (retry later),
  401 is a dead key. 2026-11-08: keys 1 and 2 gave 200; 3, 5, 6 gave 529.
- **NVIDIA NIM**: `/v1/models` answers 200 without any key, so it proves
  nothing. A chat call with a wrong model answers 404 with and without a key,
  a retired one 410. The model check runs before auth, so a key is only proven
  by a model that answers; none did on 2026-11-08 (`free-tier-probe` memory).
- **OpenCode Zen**: `https://opencode.ai/zen/v1/models` is public (200 without
  a key); no Zen key is in the environment yet.

## Proposed command (not built yet)

None of these subcommands is built; do not assume they exist. They are the
proposed shape, and each needs a spec first (`specs/tri/dev.t27`, planned) and
then the tri CLI wiring, which is hand Rust over the foreign-line caps unless it
is generated:

- `tri dev` (proposed): render in a pane, convert, serve on 8027, open the pane, run the health probes, print one OK/FAIL line per piece.
- `tri visual-update` (proposed): re-render the changed frames and reload the pane.
- `tri visual-show` (proposed): open frame N in the pane.
- `tri visual-diff` (proposed): open the `_diff` frame and print the diff counts.
- `tri visual-open` (proposed): open the served folder in the pane.

An automatic visual update after each commit is planned on top of
`tri visual-update`; until that command exists there is nothing for a
post-commit hook to call, and a hook line is foreign yml under the budget.

## Traps

- The worktree-isolation hook refuses shell for-loops with variables, `.`
  sourcing an env file, `git -C` into another worktree and `$((...))`. Write
  each command with literal names.
- The course HUD frames may live in another session's lane worktree: read them,
  never write there.
- Playwright MCP looks for Chrome at its default path and fails without it.
  Either the owner installs a browser for it, or it is pointed at an installed
  Chromium build with an executable-path option; until then the Browser pane
  takes the screenshots.
- No expanded home path in this file or in any committed file; write `~/...`.
