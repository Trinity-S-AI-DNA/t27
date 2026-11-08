# UI history, step 03: cross-axis narrowing in vf_layout and text measurement for the task card

Refs gHashTag/t27#13214 (T27F-007). One step of the T27F UI development history: a folder per
step, frames never overwritten, one manifest per step. A film is assembled later from the steps
in folder order, each step as BEFORE, what changed, AFTER, DIFF, what remains.

The source of truth for this step is the typed record `specs/t27f/ui_history_s03.t27` (schema and
rules in `specs/t27f/ui_history.t27`, the ordered log in `specs/t27f/ui_history_log.t27`). It
validates itself in its own tests, and the log is append-only. This file is the human reading copy
of the same fields; where the two differ, the spec wins. Steps 01 and 02 stay as filed: this step
is a new record.

## Record

```
id:                    ui-2026-11-07-step-03
status:                DRAFT
date:                  2026-11-07
title:                 Cross-axis narrowing in vf_layout and text measurement for the task card
branch:                claude/t27f-007-outfit-type
commit:                the commit that adds this record (see git log for ui_history_s03.t27)
commit_before:         e2b087c5de (BEFORE frames are that commit's own binary)
commit_after:          853db3e75b (the layout rule; AFTER frames are drawn by the binary built from it)
pushed:                no, the commits after bd1b9ece89 are local only
theme:                 t0 (t27 black) for the main set; t1 (paper) at 375
viewport:              375 (design width, unchanged); 233, 201, 191, 160 (narrow widths where the rule acts)
scale:                 100 percent for the main set; 200 percent at 375
visual_review:         PENDING_OWNER
```

`visual_review` stays `PENDING_OWNER` until the owner has looked at the frames and said so in
chat. No frame in this folder has been looked at by a person. Nothing here claims the card looks
better; it claims which rectangle changed, by how much, and which tests pin it.

## Paths (relative to this folder)

```
before_paths:   01-before/before_<w>_p<pct>_t<theme>.png      7 frames
after_paths:    02-after/after_<w>_p<pct>_t<theme>.png        7 frames
                02-after/after_<w>_p<pct>_t<theme>.txt        token and meaning lines of each frame
diff_paths:     03-diff/cmp_<w>_p<pct>_t<theme>.png           7 frames
frames:         375_p100_t0, 375_p100_t1, 375_p200_t0,
                233_p100_t0, 201_p100_t0, 191_p100_t0, 160_p100_t0
```

Each `cmp_*` is three cells side by side at true size: BEFORE, AFTER, DIFF (233 x 230 per cell at
233 wide, so 699 x 230). The DIFF cell is the per-channel absolute difference of BEFORE and AFTER
with the contrast stretched to the full range, so it shows where pixels differ, not by how much.
At 375 the DIFF cell is black because the rasters are identical.

BEFORE is rendered by the binary of commit e2b087c5de (the step 02 AFTER behaviour), AFTER by the
binary built from 853db3e75b; both from the same scene, at the same width, scale and theme.

Film order for this step: 01 (BEFORE), the changed rules below, 02 (AFTER), 03 (DIFF), the other
theme (`_t1`), scale 200 (`_p200`), then the remaining defects.

## Changed rules, BEFORE to AFTER

No token value changed in this step. The two entries below are layout rules, not tokens; they sit
in the record's changed list because the list is where a step names what it changed.

| Rule                        | BEFORE                                                                 | AFTER                                                                    |
|-----------------------------|------------------------------------------------------------------------|--------------------------------------------------------------------------|
| layout.cross_axis_narrowing | none: a column child kept its intrinsic cross size (the actions row stayed 202 wide in a 233 wide card) | a child is held to the inner cross extent of its parent minus its margins; an explicit min_w or min_h still wins |
| layout.leaf_measure         | none: every text node was 80 x 20 whatever it said                     | `vf_layout_m` takes per-leaf width and height overrides; `vf_card_fit` supplies measured text widths (not wired to the painter or the host main) |

## What changed for a viewer (claims about numbers, not about looks)

At 375 nothing changed: the three AFTER rasters (t0 100 percent, t1 100 percent, t0 200 percent)
are byte-identical to their BEFORE rasters (md5 equal, 0 differing pixels). That is a fact about
bytes, not evidence of quality.

At narrow widths the actions row and the fields give way instead of running out of the card.
Differing pixels (magick compare, AE): 383 at 233, 1624 at 201, 1889 at 191, 2809 at 160 (100
percent, t0); the largest per-channel difference is 118 of 255 at 233 and 255 of 255 at the other
three. The rectangles (x,y size, from the `.txt` meaning lines):

| Width | Node                | BEFORE         | AFTER          |
|-------|---------------------|----------------|----------------|
| 233   | actions stack (6)   | 16,174 202x40  | 16,174 201x40  |
| 233   | Cancel (8)          | 122,174 96x40  | 122,174 95x40  |
| 201   | actions stack (6)   | 16,174 202x40  | 16,174 169x40  |
| 201   | Save (7)            | 16,174 96x40   | 16,174 80x40   |
| 201   | Cancel (8)          | 122,174 96x40  | 106,174 79x40  |
| 191   | field stack (3)     | 16,92 160x66   | 16,92 159x66   |
| 191   | input (5)           | 16,118 160x40  | 16,118 159x40  |
| 191   | actions stack (6)   | 16,174 202x40  | 16,174 159x40  |
| 191   | Save (7)            | 16,174 96x40   | 16,174 75x40   |
| 191   | Cancel (8)          | 122,174 96x40  | 101,174 74x40  |
| 160   | field stack (3)     | 16,92 160x66   | 16,92 128x66   |
| 160   | input (5)           | 16,118 160x40  | 16,118 128x40  |
| 160   | actions stack (6)   | 16,174 202x40  | 16,174 128x40  |
| 160   | Save (7)            | 16,174 96x40   | 16,174 59x40   |
| 160   | Cancel (8)          | 122,174 96x40  | 85,174 59x40   |

In every AFTER row the node ends 16 px inside the card (the card padding), for example 16 + 201 +
16 = 233 and 16 + 128 + 16 = 160. Whether the narrow frames read as better is the owner's call.

## Known limitations (not fixed in this step)

- No person has looked at any frame in this step.
- The painter neither wraps nor clips text, so in the narrow frames a title or a label may run past
  its box. That was not checked by eye and is not claimed either way.
- The host main draws with `vf_layout`, which narrows but measures no text. `vcf_layout` in
  `vf_card_fit` measures text widths and is pinned by its own tests, but is not wired to the painter
  or the host main.
- Buttons, badges, inputs and text height are not measured; text is measured at the painter size
  and weight only.
- `type.label` and `control.min_target` resolve but do not drive the painter; there is no radius,
  line height or row gap token.
- `vf_theme` keeps a second spacing and type scale (`TH_PAD_CARD` 20 against `space.card.padding`
  16, `TH_TITLE_PX` 22 against `type.heading` 26); the breakpoint is 600 in `vf_theme` and 640 in
  `vf_layout`. Both unchanged.
- Error and disabled input states are not drawn; the focus ring on the filled Save button is accent
  on accent; the cap height ratio 700 milli is assumed, not measured.
- PR 13254 is stacked on PR 13250 and carries three inherited master reds (scan,
  duplicate-bodies, documented subcommands) that PR 13225 is meant to clear.

## How the frames were made

```
t27c gen specs/t27os/vf_card_main.t27 > vf_card.zig
zig build-exe vf_card.zig -lc
FRAME_W=233 FRAME_PCT=100 FRAME_THEME=0 FRAME_OUT=after_233_p100_t0.ppm ./vf_card 2> after_233_p100_t0.txt
ffmpeg -y -i after_233_p100_t0.ppm after_233_p100_t0.png
magick compare -metric AE before_233_p100_t0.png after_233_p100_t0.png null:
```

BEFORE frames come from the binary of e2b087c5de built the same way. Run in T27 OS pane 15, not on
the Mac. The first line of each `.txt` (it names a path on the machine) was dropped.

## Tests behind the numbers (T27 OS pane 15, t27c and t27b, 0 failed, 0 vacuous)

vf_layout 19 (5 new), vf_card_fit 5 (new), vf_card_tok 15, vf_card 21, vf_storybook 13, vf_ring1 5,
vf_pixel 7, vf_snapshot 9, vf_meaning 7, vf_render 6, vf_guard 14, vf_extent 4, vf_scale 10,
ui_kit 9, vf_tokens 21, vf_theme 11, ui_history 10, ui_history_s01 3, ui_history_s02 3,
ui_history_log 5; vf_card_main 6 and vf_frame_main 7 (t27c). With this record: ui_history_s03 3
and ui_history_log 6. The duplicate-body scan reports only the inherited `bytes_eq` drift
(11 to 12).
