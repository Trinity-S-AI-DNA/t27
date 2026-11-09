# UI history, step 02: the text inset of a node is the space.control.inset token

Refs gHashTag/t27#13214 (T27F-007). One step of the T27F UI development history: a folder per
step, frames never overwritten, one manifest per step. A film is assembled later from the steps
in folder order, each step as BEFORE, what changed, AFTER, DIFF, what remains.

The source of truth for this step is the typed record `specs/t27f/ui_history_s02.t27` (schema and
rules in `specs/t27f/ui_history.t27`, the ordered log in `specs/t27f/ui_history_log.t27`). It
validates itself in its own tests, and the log is append-only. This file is the human reading copy
of the same fields; where the two differ, the spec wins. Step 01 stays as filed
(`specs/t27f/ui_history_s01.t27`, `../step-01-task-card-tokens/`): this step is a new record.

## Record

```
id:                    ui-2026-11-07-step-02
status:                DRAFT
date:                  2026-11-07
title:                 Painter text inset read from the space.control.inset token
branch:                claude/t27f-007-outfit-type
commit:                the commit that adds this record (see git log for ui_history_s02.t27)
commit_before:         70cf34dc61 (step 01 AFTER frames; BEFORE here are those frames)
pushed:                no, the commits after bd1b9ece89 are local only
theme:                 t0 (t27 black) for the main set; t1 (paper) at 375 and 1280
viewport:              375, 768, 1280 (design width of the card scene)
scale:                 100 percent for the main set; 200 percent at 375 and 1280
visual_review:         PENDING_OWNER
```

`visual_review` stays `PENDING_OWNER` until the owner has looked at the frames and said so in
chat. No frame in this folder has been looked at by a person. Nothing here claims the card looks
better; it claims which number changed, by how much, and which tests pin it.

## Paths (relative to this folder)

```
before_paths:   01-before/before_<w>_p<pct>_t<theme>.png      7 frames
after_paths:    02-after/after_<w>_p<pct>_t<theme>.png        7 frames
                02-after/after_<w>_p<pct>_t<theme>.txt        token and meaning lines of each frame
diff_paths:     03-diff/cmp_<w>_p<pct>_t<theme>.png           7 frames
frames:         375_p100_t0, 768_p100_t0, 1280_p100_t0, 375_p100_t1, 1280_p100_t1,
                375_p200_t0, 1280_p200_t0
```

Each `cmp_*` is three cells side by side at true size: BEFORE, AFTER, DIFF (375 x 230 per cell at
100 percent, so 1125 x 230 at 375 wide). The DIFF cell is the per-channel absolute difference of
BEFORE and AFTER with the contrast stretched to the full range, so it shows where pixels differ,
not by how much: the true largest per-channel difference is 28 of 255. The BEFORE cell equals the
BEFORE raster (difference 0).

BEFORE is the step 01 AFTER set, unchanged: each of the seven BEFORE rasters has zero differing
pixels against the step 01 AFTER raster of the same name.

Film order for this step: 01 (BEFORE), the changed token below, 02 (AFTER), 03 (DIFF), the other
theme (`_t1`), scale 200 (`_p200`), then the remaining defects.

## Changed tokens, BEFORE to AFTER

| Token                    | BEFORE                                                | AFTER                                              |
|--------------------------|-------------------------------------------------------|----------------------------------------------------|
| space.control.inset      | none; the painter read the raw smallest theme step    | 6 px, alias of space.2 (id 31 in vf_tokens)        |
| painter.text_pen_offset  | 6.112 px (6112 milli-px; 1564 q8 at 100 percent)      | 6 px (`VcStyle.pad_x`; 1536 q8 at 100 percent)     |

The theme keeps `TH_CONTROL_INSET` = 6 as the default of the unstyled path, equal to the token and
held equal by a test. `vc_render_s` refuses a pen offset above 64 px with `VC_BAD_METRICS`.

## What changed for a viewer (claims about numbers, not about looks)

Text starts 0.112 px closer to the left edge of its node (6.112 px before, 6 px after). That is a
change in anti-aliasing of glyph edges, not a move of any box: the layout rectangles are the same.
A button centres its label, so Save and Cancel do not change.

Measured on the rasters (magick compare, T27 OS pane 15 renders):

- Every AFTER raster differs from its BEFORE raster by md5.
- Differing pixels (AE): 1565 at 375, 1565 at 768, 1566 at 1280 (100 percent, t0); 1565 at 375 and
  1566 at 1280 in the paper theme (t1); 3272 at 200 percent for both 375 and 1280.
- At 375, 100 percent, t0 the changed pixels sit in one box, 176 x 132 at +22,+16; the largest
  per-channel difference is 28 of 255.
- At 375, 100 percent, t0, differing pixels per node rectangle: title 349, badge 208, label 242,
  input 327, Save 0, Cancel 0.

Whether any of this reads as different is the owner's call.

## Known limitations (not fixed in this step)

- No person has looked at any frame in this step.
- `vf_theme` keeps a second spacing and type scale: `TH_PAD_CARD` is 20 where `space.card.padding`
  is 16, and `TH_TITLE_PX` is 22 where `type.heading` is 26. The choice between them is the owner's.
- The breakpoint is 600 in `vf_theme` and 640 in `vf_layout`; unchanged and pinned in `vf_ring1`.
- `vf_layout` narrows no column and measures no text; the 560 clamp and the centring live in the
  renderer.
- `type.label` and `control.min_target` resolve but do not drive the painter; there is no radius,
  line height or row gap token.
- Error and disabled input states are not drawn; the focus ring on the filled Save button is accent
  on accent; the cap height ratio 700 milli is assumed, not measured.
- No wide layout is claimed for 768 and 1280 beyond the centred 560 wide column.
- PR 13254 is stacked on PR 13250 and carries three inherited master reds (scan,
  duplicate-bodies, documented subcommands) that PR 13225 is meant to clear.

## How the frames were made

```
t27c gen specs/t27os/vf_card_main.t27 > vf_card.zig
zig build-exe vf_card.zig -lc
FRAME_W=375 FRAME_PCT=100 FRAME_THEME=0 FRAME_OUT=after_375_p100_t0.ppm ./vf_card 2> after_375_p100_t0.txt
ffmpeg -y -i after_375_p100_t0.ppm after_375_p100_t0.png
magick compare -metric AE before_375_p100_t0.png after_375_p100_t0.png null:
```

Run in T27 OS pane 15, not on the Mac. The first line of each `.txt` (it names a path on the
machine) was dropped.

## Tests behind the numbers (T27 OS pane 15, working tree before this record)

vf_tokens 21, vf_theme 11, vf_card_tok 15, vf_card 21, vf_storybook 13, ui_history 10,
ui_history_s01 3 (t27c and t27b, 0 failed, 0 vacuous), vf_card_main 6 (t27c, 0 vacuous). With this
record: ui_history_s02 3 and ui_history_log 5 (t27c and t27b, 0 failed, 0 vacuous). The
duplicate-body scan reports only the inherited `bytes_eq` drift (11 to 12).
