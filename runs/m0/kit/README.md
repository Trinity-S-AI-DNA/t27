# M0 UI Kit story frames (Trinity-S-AI-DNA/t27#13667)

First render of every story of `specs/t27f/vf_storybook.t27` through `specs/t27os/vf_story_main.t27`:
`sb_graph(story)` -> `vf_layout` -> `vc_render_s` -> P6 -> PNG. 375 design pixels, 200 percent, themes 0 and 1.
Rendered on the VPS lane checkout at `b339f8a` with the `t27-m13-final` toolchain; PNG by ffmpeg from the P6 frame.

**This is not a visual-correctness claim.** It is the first time the kit is visible. The owner's verdict is not recorded yet.

## Result

| Measure | Value | Evidence |
| --- | --- | --- |
| Stories rendered | 34 of 35 | `rc.txt` |
| Frames | 68 of 70 | `index.txt`, `sNN_tT.png` |
| Repeat render byte-equal | 68 of 68 | `rc.txt`, `vr=0` on every rendered line |
| Theme 0 differs from theme 1 | 34 of 34 stories | `index.txt`, column 4 (negative control for the byte compare) |
| Distinct frames | 20 of 68 | `index.txt`, column 4 |
| Spec test blocks | see `t27l_kit.json` | not part of this table |

## What the frames show is not painted

The byte compare finds groups of stories that render to the same frame in a theme. The renderer does not paint the
variant or the state of these stories; only the kind and the text reach the pixels.

| Stories | Frames per theme | Not painted |
| --- | --- | --- |
| 0..14 button (5 variants x default, hover, disabled) | 1 | variant, hover, disabled |
| 15, 16 card default, hover | 1 | hover |
| 17, 19, 31 input default, focus, disabled | 1 | focus, disabled |
| 18 input error | 1 (own frame, taller) | error colour not checked by eye |
| 20..24 badge (5 variants) | 1 | variant |
| 25..27 nav active 0, 1, 2 | 1 | active item |
| 32, 33 text heading, body | 1 | heading versus body |
| 34 container | 0 | refused by the renderer with code 3 (VC_BAD_RASTER) in both themes |

Stories with their own frame: 18 input-error, 28 table-1-row, 29 table-2-rows, 30 task-card.

## Files

- `sNN_tT.png`: story NN, theme T. 750 pixels wide; height is the lowest node edge of the layout at 200 percent.
- `index.txt`: one line per frame: name, story id, size, first 16 hex of the P6 sha256, sha256 of the PNG.
- `rc.txt`: return codes of the build, each render and each repeat-render byte compare.
- `t27l_kit.json`: the T27L record of this iteration.

## Not done

- Visual regression against a stored baseline: these frames are the first baseline. `vf_diff` has no host yet (#13662).
- Accessibility (contrast, focus ring): not run.
- Widths 768 and 1280: not rendered.
