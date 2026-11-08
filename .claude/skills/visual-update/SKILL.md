---
name: visual-update
description: Re-render the course HUD (lesson 1, 375x812, 13 frames) after a change to its specs, diff every frame against the previous render, convert to PNG and show the result -- what the owner calls "tri visual-update" (2026-11-08). Load after any commit or push that touches specs/course/course_hud_*.t27, vf_style, vf_uikit, vf_type, petal_menu or the Outfit data; on every 4-hour scheduled tick; and when the owner says "show the visual", "visual update", "re-render the HUD", "before/after".
---

# visual-update

<!-- only-t27-rule -->
> **Only t27.** Every step below is a `.t27` program built by `t27c` or a one-line terminal command. The render is
> `specs/course/course_hud_main.t27`, the before/after diff is `specs/course/course_hud_diff_main.t27` (on
> `t27f::vf_diff`), and PNG conversion is macOS `sips`. Writing a JS, Python or shell script to do any of this is
> forbidden (`only-t27`), scratch included.

`tri visual-update` **does not exist** as a tri subcommand (a proposed name; the built tri has none). This skill is the
procedure until it is built; when it does, the subcommand must run exactly these steps and this file shrinks to a pointer.

## What one run produces

All under the worktree's `target/course/hud/` (gitignored, never `/tmp`):

| file | what |
|---|---|
| `hud_init.ppm`, `hud_00.ppm` .. `hud_11.ppm` | the 13 frames of the 12-step event script (native target) |
| `base/*.ppm` | the previous run's frames, the "before" |
| `*_diff.ppm` | per frame: changed pixels red, unchanged ones grey |
| `diff.txt` | one line per frame, `<name>: <n> pixels differ, bbox ...`, and a total |
| `trace.txt` | the event trace: step, event, screen, selection, scene, score, digest |
| `png/*.png`, `png/base/*.png` | every frame and diff frame as PNG, and the before frames, for the Browser pane |
| `hud_wasm.mjs`, `hud_state.v` | the web and FPGA targets, regenerated from the same specs |
| `last.txt` | the commit the run rendered |

## Steps

Each command is one short line for a Terminal tab (`run_in_terminal`, no `tab_id`; close the tab after). The Bash
tool refuses `t27c gen` and `zig` on this Mac (hook `mac-no-heavy`). Run from the worktree root, because the HUD
program writes its frames relative to the working directory.

1. **Skip if nothing moved.** Compare the newest commit touching the HUD closure with `last.txt`:
   `git log -1 --format=%H -- specs/course specs/t27f/vf_style.t27 specs/t27f/vf_uikit.t27`
   If it equals `target/course/hud/last.txt` and the 13 frames exist, report "unchanged" and stop. Most ticks end here.
2. **Rotate the before frames.** `mkdir -p target/course/hud/base` then
   `cp target/course/hud/hud_*[0-9t].ppm target/course/hud/base/` (frames only, not the `_diff` ones). The first run
   seeds `base/` from the last frames anyone rendered.
3. **Render (native).**
   - `export H=target/course/hud`
   - `t27c gen specs/course/course_hud_main.t27 > $H/hud.zig`
   - `zig build-exe $H/hud.zig -lc -femit-bin=$H/hud`
   - `$H/hud 2> $H/trace.txt; echo rc $?`
4. **Diff.**
   - `t27c gen specs/course/course_hud_diff_main.t27 > $H/hud_diff.zig`
   - `zig build-exe $H/hud_diff.zig -lc -femit-bin=$H/hud_diff`
   - `$H/hud_diff 2> $H/diff.txt; echo rc $?` (exit 1 is a diff refusal, exit 2 a missing or malformed frame)
5. **Web and FPGA targets.**
   - `t27c gen-js specs/course/course_hud_wasm.t27 > $H/hud_wasm.mjs; echo rc $?`
   - `t27c gen-verilog specs/course/course_hud_state.t27 > $H/hud_state.v; echo rc $?`
6. **PNG.** `mkdir -p $H/png/base`, then two calls, because the before and after frames share names and one call
   would let `base/` overwrite the new frames:
   - `sips -s format png $H/*.ppm --out $H/png/` (new frames and `_diff` frames)
   - `sips -s format png $H/base/*.ppm --out $H/png/base/` (before frames)
7. **Show.** In the Browser pane: `preview_start` name `course-hud` (`.claude/launch.json`, port 8127, serves
   `$H/png`), then navigate to `http://localhost:8127/<frame>.png` and take a screenshot -- the changed frame, its
   `_diff`, and `base/<frame>.png` for the before. Never open a PNG with the Read tool. Without a Browser pane, give
   the paths as links. Skill `tri-dev` is the full bring-up around this step.
8. **Record.** Write the commit from step 1 into `last.txt`. Report in under 10 lines: frames changed of 13, the
   largest bbox, rc of each target, the trace's final digest.

## Scheduled run (every 4 hours)

The owner asked for this run on a 4-hour schedule (2026-11-08). The scheduled task's prompt is one line: "Load
skill visual-update and run one tick in the course-hud-style worktree" (`.claude/worktrees/course-hud-style` under the
main clone). Step 1 makes an idle
tick cost a single `git log`. A tick never commits, pushes or opens a PR; it only renders and reports.

## What this is not

- **Not a visual-correctness claim.** A pixel count says how many pixels moved between two runs, not that either frame
  looks right. That starts when somebody opens the frames next to the trace.
- **Not a test.** The HUD's tests are the `test` blocks in the five `course_hud_*` specs; run them with `t27b test`.
- **Not a second renderer.** It drives the existing specs; a new screen or style value goes into those specs.
- **Not the web HUD running in a browser.** The web target is `gen-js` output (the state machine and frame logic as a
  module). Nothing generated draws it on a canvas yet: `t27c` has no `wasm32` target and no canvas host, which is a
  compiler gap to file, not a JS shim or wasm-pack crate to write (`only-t27`). Until then the browser shows the
  native frames as PNGs.

## Traps

- The frames are byte-identical across runs of the same commit (the render is deterministic); a diff on an unchanged
  commit means the build used different inputs, usually an untracked spec in the worktree. Check `git status`.
- `course_hud_main` needs `t27f::vf_type` and `t27f::outfit_r_data` from #13254; on a branch without them `t27c gen`
  fails at resolve, which is a missing dependency, not a render failure.
- zsh: brace `${H}` before a colon.
