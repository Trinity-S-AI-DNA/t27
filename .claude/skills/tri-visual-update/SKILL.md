---
name: tri-visual-update
description: How the course HUD (and any t27 UI) is shown to the owner after every iteration -- WASM running in the browser, screenshot in chat and saved under docs/course/hud/, before/after diff. Load when asked to render, preview, screenshot or "show" a t27 UI, when a cron tick has a visual step, or when someone reaches for a PNG as the preview (owner, 2026-11-08: "PNG is a crutch").
---

# tri-visual-update

## 1. The rule

Owner order, 2026-11-08: the preview of a t27 UI is the WASM module running in a
real browser, not a PNG converted from a PPM. After every iteration: rebuild,
re-screenshot, show the screenshot in chat, save it under `docs/course/hud/`, and
diff it against the previous one.

## 2. The route

a. The spec is `specs/course/course_hud_main.t27`. The state machine is in
   `course_hud_state.t27`, hit testing in `course_hud_nav.t27`, data in
   `course_hud_data.t27`; the browser contract is documented in `course_hud_wasm.t27`.
b. `t27c gen specs/course/course_hud_main.t27 > target/course/hud/hm.zig` runs in a
   T27 OS pane, never in the Mac shell (hook mac-no-heavy).
c. The export glue `docs/course/hud/export.zig` is the one hand-written input, as
   `specs/t27f/build.t27` already names export.zig for t27f.wasm. It holds one global
   HudState, one Learner and one 913500-byte RGB raster, and wraps the generated pub
   fns as `export fn`. Copy it next to `hm.zig` before linking:
   `cp docs/course/hud/export.zig target/course/hud/`.
d. Link in the pane: `zig build-exe export.zig -target wasm32-freestanding -fno-entry
   -rdynamic -O ReleaseSmall -femit-bin=course_hud.wasm`. `-rdynamic` is required: a
   `-fno-entry` wasm exports nothing without it.
e. Copy `course_hud.wasm` to `docs/course/hud/`. A .wasm is data for the own-language
   gate and may be committed.

## 3. The export contract

| export | meaning |
|---|---|
| `hud_reset()` | state and learner to init |
| `hud_event(ev: u8, x: i32, y: i32)` | one event: EV_UP 1, EV_DOWN 2, EV_ENTER 3, EV_BACK 4, EV_TAP 5 (canvas x, y), EV_MENU 6, EV_TICK 7; keys pass -1, -1 |
| `hud_answer(a: u32)` | answer for the current scene |
| `hud_paint()` | renders the raster |
| `fb_ptr() -> usize` | offset in `memory` of the 375x812x3 RGB raster (913500 bytes) |
| `hud_screen() -> u8` | SCR_OVERVIEW 0, SCENE 1, MENU 2, PROGRESS 3 |

The host (`docs/course/hud/index.html` + `shim.js`) chooses nothing: events in, one
putImageData out per frame. `fill_rect`, `draw_text` and `clear` happen inside the
wasm, in the t27 painters.

## 4. The gate on the glue and the shim

`export.zig`, `index.html` and `shim.js` are hand-written foreign files.
`specs/policy/own_language.t27` FOREIGN_EXTS covers zig, js, mjs, html. A new
foreign file is DENY unless listed in `tools/policy/foreign-exceptions.txt`, which
only the owner's `owner-approved-foreign` label unlocks. Budget: 40 added hand
lines per file, 80 per PR, added <= deleted.

The three files are listed there since 2026-11-08 (owner: "Yes, add them", chat on
PR #13304; entry PR #13342), so they are tracked under `docs/course/hud/`. Every
further hand line in them is still budget: pay it with a deletion of foreign code in
the same diff. A fourth hand-written file needs its own entry and the owner's word.
The .wasm, the .png screenshots, `.claude/launch.json` and the .t27 specs land
normally with `Refs #13251`. Never `--no-verify`.

## 5. Serve and see

`.claude/launch.json` has the configuration `course-hud`:
`python3 -m http.server 8137 -d docs/course/hud --bind 127.0.0.1` (Python's
http.server sends application/wasm and text/javascript). Start it with the app's
preview_start (never a Bash server), then navigate the Browser pane to
http://127.0.0.1:8137/ and take a screenshot; it is shown inline in chat.
Keyboard: arrows, Enter, Escape, m, 1-3. A click is a tap.

## 6. Save the screenshot

Playwright MCP was registered 2026-11-08 with
`claude mcp add playwright -- npx -y @playwright/mcp@latest` (local scope, this
worktree path). It is callable only in a session started after the registration:
`browser_navigate` http://127.0.0.1:8137/ then `browser_take_screenshot` with
filename `docs/course/hud/hud_<YYYYMMDD>_<n>.png`.

Fallback in a session without it, the Playwright CLI:
`npx -y playwright screenshot --viewport-size=375,812 --wait-for-timeout=1500
http://127.0.0.1:8137/ docs/course/hud/hud_<YYYYMMDD>_<n>.png`. It needs a browser
Playwright can launch. Whether one is installed on this Mac is not measured; Google
Chrome was not found in /Applications on 2026-11-08.

## 7. Diff before and after

Keep the previous PNG. Convert both: `sips -s format bmp a.png --out a.bmp` (BMP is
uncompressed). `cmp -l a.bmp b.bmp | wc -l` is the count of differing bytes. Show
both PNGs in chat with the count. A count of 0 after a visual change means the
build did not pick the change up.

## 8. Cron integration

A scheduled tick's visual step is: rebuild in the pane, copy the wasm, reload the
Browser pane, screenshot, save, diff, then a one-line report with the differing-byte
count and the two file names. No polling; a pane or the preview tells you when it is
ready.

## 9. Never

- A PPM->PNG as the preview.
- Hand JS beyond the shim.
- A canvas import design with `@extern` (t27c cannot compile @extern specs).
- A hand-written file beyond the three listed in `tools/policy/foreign-exceptions.txt`.
- A bare `~/.t27os/t27os-pty`.
- `--no-verify`.

## 10. Companions

only-t27, work-cheap, t27os-orchestra, foreign-budget-discipline,
short-terminal-commands.
