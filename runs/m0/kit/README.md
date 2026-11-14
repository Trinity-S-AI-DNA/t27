# M0 UI Kit story frames (Trinity-S-AI-DNA/t27#13667, iteration 2: #13672)

Every story of `specs/t27f/vf_storybook.t27` through `specs/t27os/vf_story_main.t27`:
`sb_kit(story)` -> `vsm_graph` (kit variant and state copied into the graph, filtered by the contract masks) ->
`vf_layout` -> `vc_render_s` -> P6 -> PNG. 375 design pixels, 200 percent, theme 0 (dark, TH_T27) and theme 1
(light, TH_PAPER). Rendered 2026-11-14 06:33 UTC on the VPS lane checkout at `dc626f1` plus the two spec diffs of
this PR, with the `t27-m13-final` toolchain; PNG by ffmpeg from the P6 frame.

**This is not a visual-correctness claim.** The owner's verdict is not recorded yet, and the agent that assembled
this folder cannot open images in its session: the sheets below were built and measured, not looked at.

## Gallery

- `gallery/overview.png` (1840 x 3756): every story, dark and light side by side, one row per story with its
  number and id; story 34 is a red row that says it was not rendered.
- `gallery/before_after.png` (1670 x 1067, half scale): the 18 stories whose bytes changed against the previous
  set (`cd7fec5`, PR #13671), columns before dark, after dark, before light, after light.

Both sheets are ffmpeg stacks of the `sNN_tT.png` files in this folder; they hold no pixel the renderer did not make,
except the row labels and the header.

## Result

| Measure | Value | Evidence |
| --- | --- | --- |
| Stories rendered | 34 of 35 | `rc.txt` |
| Frames | 68 of 70 | `index.txt`, `sNN_tT.png` |
| Repeat render byte-equal | 68 of 68 | `rc.txt`, `vr=0` on every rendered line |
| Distinct frames | 44 of 68 (was 20) | `index.txt`, column 5; a diagnostic, not a proof of correctness |
| Stories changed against the previous set | 18 of 34 | byte compare per story, either theme |
| Spec test blocks | not run | see `t27l_kit.json` |

Changed stories: 0..5, 7, 8, 10, 11, 13, 14 (button), 15, 16 (card), 19 (input focus), 23, 24 (badge success, warn),
30 (task card). Stories 0, 15 and 30 changed in their default state because a button with kit variant primary now
reaches the renderer as PRIMARY (accent fill); before, the graph carried no variant and every button was outlined.

## Still the same frame (per theme)

| Stories | Why |
| --- | --- |
| 6, 9, 12 and 7, 10, 13 and 8, 11, 14: ghost, success, warn buttons | the contract's button mask has no ghost, success or warning variant, so the three reach the renderer as the default variant |
| 17 input default, 31 input disabled | disabled dims the ink to the muted colour, and an empty input already shows a muted placeholder |
| 20, 21, 22 badge primary, secondary, ghost | the contract's badge mask has no such variants |
| 25, 26, 27 nav active 0, 1, 2 | the active item is not painted |
| 32, 33 text heading, body | heading versus body is not painted for a lone text node |

Not painted at all: pressed (no contract state), input error colour (the theme has no error token), badge WARNING
uses TH_ACCENT_DARK as a provisional design default.

Story 34 container: `render=1`, no frame in either theme. `kit_container` builds a lone root with no children, the
layout height is 0 and the host refuses the raster. The fix belongs to `specs/t27f/ui_kit.t27` and
`specs/t27f/vf_storybook.t27`, which are held by PR #13513 (another writer); this lane does not write there.

## Known defects, not fixed here

- Badge text can run past its fill (owner's screenshot, 2026-11-14): horizontal text padding is not symmetric.
- No story caption inside the renderer: the row labels of the sheets come from ffmpeg, not from a `.t27` gallery.

## Files

- `sNN_tT.png`: story NN, theme T. 750 pixels wide; height is the lowest node edge of the layout at 200 percent.
- `index.txt`: one line per frame: name, story id, size, first 16 hex of the P6 sha256, sha256 of the PNG.
- `rc.txt`: return codes of the build, each render and each repeat-render byte compare.
- `gallery/`: the two sheets.
- `t27l_kit.json`: the T27L record of this iteration.

## Not done

- Spec tests: not run (the VPS zig cannot compile the generated tests); CI is the first run.
- Visual regression against a stored baseline with `vf_diff`: no host yet (#13662); the byte compare above stands in.
- Accessibility (contrast, focus ring): not run.
- Widths 768 and 1280: not rendered.
