---
name: tri-dev
description: Bring up the full T27F dev environment for the course HUD in one pass and report OK/FAIL per item -- what the owner calls "tri dev" (2026-11-08). Renders the three targets (native frames, web gen-js, FPGA gen-verilog), serves the frames on localhost, opens them in the Browser pane, checks the MCP and GitHub access, reads provider key health by name only, and shows the first screenshot. Load when the owner says "tri dev", "bring up the dev environment", "dev server", "open the HUD in the browser", "WASM build", or at the start of a HUD work session. Companion skills - visual-update (re-render + diff), access-map (providers), only-t27.
---

# tri-dev

<!-- only-t27-rule -->
> **Only t27.** Every item below is a `.t27` program built by `t27c`, or an existing tool driven by one short command
> (`python3 -m http.server`, `sips`, `gh`, `curl` on a keyless URL). A hand-written JS shim, `loader.js`, an HTML
> `<script>` body or a Rust/wasm-pack crate is foreign code and is refused here, scratch included (skill `only-t27`).

`tri dev` **does not exist** as a tri subcommand (it answers "unrecognized subcommand", 2026-11-08). Neither do
`tri visual-update`, `tri visual-show`, `tri visual-diff` and `tri visual-open`: they are proposed names for the steps
below, and this skill is the procedure until they are built. When they are, each must run exactly its steps and this
file shrinks to a pointer.

## Proposed subcommands (not built)

| name | steps here |
|---|---|
| `tri visual-update` | skill `visual-update` (render + diff + PNG) |
| `tri visual-open` | step 3 (server + Browser pane) |
| `tri visual-show` | step 6 (screenshot in the chat, copy to `docs/course/hud/`) |
| `tri visual-diff` | skill `visual-update` step 4, then open `*_diff.png` |
| `tri dev` | all of steps 1-7 |

## Steps

Run from the HUD worktree root. Each command is one short line (skill `short-terminal-commands`). `t27c gen*` and
`zig` go through a Terminal tab (`run_in_terminal`), because the Bash tool's `mac-no-heavy` hook denies them; close
each tab after reading it.

1. **Native target.** Skill `visual-update` steps 1-4: `t27c gen` the HUD, `zig build-exe`, run it, diff against
   `base/`. OK = build rc 0, run rc 0, 13 frames in `target/course/hud/`.
2. **Web and FPGA targets.**
   - `t27c gen-js specs/course/course_hud_wasm.t27 > target/course/hud/hud_wasm.mjs; echo rc $?`
   - `t27c gen-verilog specs/course/course_hud_state.t27 > target/course/hud/hud_state.v; echo rc $?`
   OK = rc 0 for both. This is the state machine and the frame logic for the browser, **not** a running browser HUD:
   see "What is missing".
3. **Serve and open.** `.claude/launch.json` names `course-hud`: `python3 -m http.server 8127` bound to 127.0.0.1,
   serving `target/course/hud/png`. Start it with `preview_start` name `course-hud` (port 8080 is held by another
   project's `trunk` server on this Mac; leave it alone). Navigate the Browser pane to
   `http://localhost:8127/hud_init.png`. OK = the tab title reads `hud_init.png (375x812)`.
4. **MCP and GitHub.**
   - `claude mcp list | grep -i playwright` -- OK when it says Connected. The built-in Browser pane already
     screenshots, so Playwright is a second route, not a requirement.
   - `gh auth status` -- OK when logged in. `gh issue/pr create, merge, comment` is the GitHub route; a GitHub MCP
     server adds nothing `gh` lacks, so none is registered.
   - DeepSeek: FAIL by design until it has a key-master profile (skill `access-map`: provider row only, account
     unfunded per t27#13034). Never name it available.
5. **Provider health, names only.** Never print, echo, copy or move a key value (skill `secret-hygiene`).
   - z.ai: `grep -o '^ZAI_KEY_[0-9]*' ~/.claude/.env | sort -u` counts the key names.
   - NVIDIA: `grep -o 'NVIDIA_API_KEY_[0-9]*' ~/.zshrc | sort -u` counts the key names.
   - OpenCode Zen: `curl -s -m 15 -o /dev/null -w '%{http_code}' https://opencode.ai/zen/v1/models` (keyless
     catalog). 200 means the catalog answers, not that the free pool has quota (it answered 429 FreeUsageLimitError on
     2026-10-31).
   - Liveness per key is the T27 OS KEYS tab check button (`specs/t27os/keys_probe.t27`), or the last recorded probe.
     Report the probe's time with its result; a probe older than a day is "stale", not "OK".
6. **First screenshot.** `computer` screenshot of the Browser pane tab; copy the frames you showed to
   `docs/course/hud/` (`hud_init.png`, the menu frame `hud_07.png`, `hud_init_diff.png`). Never open a PNG with the
   Read tool.
7. **Report.** One table, OK/FAIL per item: native, web gen-js, fpga gen-verilog, server, Browser pane, Playwright,
   GitHub, DeepSeek, z.ai, NVIDIA, OpenCode Zen, screenshot. A FAIL row names its cause and who holds the fix.

## What is missing (say so, do not paper over it)

- **No browser host.** `t27c` has no `wasm32` target and `t27b build --target wasm32` does not exist. `gen-js` emits
  the module, but nothing generated draws it on a canvas or feeds it pointer events, and `course_hud_wasm.t27` says
  the canvas renderer belongs to the browser. An interactive HUD in the browser needs a generated host (canvas
  imports `fill_rect`, `draw_text`, `put_pixels`, `clear`; an event pump into `hud_step_packed`). That is a compiler
  gap to file against t27c with the issue shape (skill `issue-shape`), not a shim to write.
- **Per-commit re-render.** A git or lefthook hook is a `.yml`/shell entry (foreign, owner-gated). Until a t27
  trigger exists, the 4-hour scheduled tick of `visual-update` (with its skip-if-unchanged step) is the automation.

## Traps

- `python3 -m http.server` serves a directory listing, so no `index.html` is needed; writing one with a script body
  is the shim this skill refuses.
- `preview_start` refuses a port that another process holds; check `lsof -nP -iTCP:<port> -sTCP:LISTEN` and pick a
  free port in `launch.json` rather than stopping someone else's server.
- `sips ... a/*.ppm b/*.ppm --out png/` overwrites same-named files; convert `base/` into `png/base/`.
