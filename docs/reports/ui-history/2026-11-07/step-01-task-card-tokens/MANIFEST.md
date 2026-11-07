# UI history, step 01: task card takes its numbers from design tokens

Refs gHashTag/t27#13214 (T27F-007). One step of the T27F UI development history: a folder
per step, frames never overwritten, one manifest per step. A film is assembled later from the
steps in folder order, each step as BEFORE, what changed, AFTER, DIFF, what remains.

This file is prose on purpose. The owner asked for a `manifest.yaml`; a hand-written YAML file is
a non-t27 file and the project describes itself in t27. The keys below are the owner's field
list, one per line, so a typed `.t27` record (or a YAML file written by a tool) can be generated
from them later. Not done yet.

## Record

```
id:                    ui-2026-11-07-step-01
status:                DRAFT
date:                  2026-11-07, BEFORE rendered 19:49 +07, AFTER rendered 20:57 +07
title:                 Task card: padding, gaps, type sizes, width and outline from tokens
branch:                claude/t27f-007-outfit-type
commit:                70cf34dc61 (AFTER frames; code of this step)
commit_before:         bd1b9ece89 (BEFORE frames; the pushed head of PR 13254)
pushed:                no, the commits after bd1b9ece89 are local only
theme:                 t0 (t27 black) for the main set; t1 (paper) in 03-paper-theme
viewport:              375, 768, 1280 (design width of the card scene)
scale:                 100 percent for the main set; 200 percent in 04-scale-check
visual_review_status:  PENDING_OWNER
visual_review:         PENDING_OWNER
```

`visual_review_status` stays `PENDING_OWNER` until the owner has looked at the frames and said
so in chat. No frame in this folder has been looked at by a person. Nothing here claims the card
looks better; it claims which numbers changed and which tests pin them.

## Paths (relative to this folder)

```
before_paths:
  01-before-token-integration/base_375_p100_t0.png      375 x 192
  01-before-token-integration/base_768_p100_t0.png      768 x 192
  01-before-token-integration/base_1280_p100_t0.png     1280 x 192
after_paths:
  02-after-token-integration/after_375_p100_t0.png      375 x 230
  02-after-token-integration/after_768_p100_t0.png      768 x 230
  02-after-token-integration/after_1280_p100_t0.png     1280 x 230
  02-after-token-integration/after_*_p100_t0.txt        token and meaning lines of each frame
paper_theme_paths:
  03-paper-theme/after_375_p100_t1.png, after_1280_p100_t1.png (+ .txt)
scale_paths:
  04-scale-check/base_375_p200_t0.png, after_375_p200_t0.png        750 wide
  04-scale-check/base_1280_p200_t0.png, after_1280_p200_t0.png      2560 wide
diff_paths:
  05-diff/cmp_375_p100.png, cmp_768_p100.png, cmp_1280_p100.png
  05-diff/cmp_375_p200.png, cmp_1280_p200.png
```

Each `cmp_*` is three cells side by side at true size: BEFORE, AFTER, DIFF. BEFORE is padded with
white to the height of AFTER, so the lower band of the DIFF cell is the 38 design pixels of
added height, not a drawing change. The DIFF is the per-channel absolute difference with the
contrast stretched to the full range (`-auto-level`), so it shows where pixels differ, not by how much.

Film order for this step: 01 (BEFORE), then the changed tokens below, then 02 (AFTER), 05 (DIFF),
03 (other theme), 04 (scale 200), then the remaining defects.

## Changed tokens, BEFORE to AFTER

| Token (semantic name)       | BEFORE | AFTER | Where it comes from now                           |
|-----------------------------|-------:|------:|---------------------------------------------------|
| space.card.padding          | 8      | 16    | vf_tokens alias of space.4 (vf_proportion step)   |
| space.field.label_gap       | 8      | 6     | alias of space.2                                  |
| space.field.group_gap       | 8      | 16    | alias of space.4                                  |
| space.action.gap            | 8      | 10    | alias of space.3                                  |
| type.heading                | 26     | 26    | alias of type.size.heading, drives the title size |
| type.body                   | 16     | 16    | alias of type.size.body, drives the label size    |
| border.width                | 1      | 1     | explicit BASE_UNIT, drives the outline width      |
| layout.card.max_inline      | none   | 560   | explicit OWNER_FIXED, bounds the card width       |
| card height (derived)       | 192    | 230   | vf_layout result, 16+20+16+24+16+66+16+40+16      |
| text inset in the painter   | none   | 16    | space.card.padding; BEFORE drew text 6 px right of the node edge, with no minimum distance from the card edge |

Not a token change: heading, body and border are the same numbers as before; they now come from
the token style, which a test proves by changing them and watching the picture change.

Also changed between the two commits, by painting rather than by tokens (commit 4b97fb56df):
the title ink sits at least the inset inside the card, an empty input shows a muted placeholder,
button labels are centred on the cap height, a raster wider than the maximum width centres the
card on the page background (768 and 1280 show it at x offsets 104 and 360), a focus ring exists
for a focused control (the frames are drawn with no focus, so it is not in them).

## What changed for a viewer (claims about numbers, not about looks)

The card is 38 design pixels taller. Content sits 16 px from the edge instead of 8 px.
The label sits 6 px above its field and the field stack is separated from its neighbours by 16 px.
The Save and Cancel buttons are 10 px apart. At 768 and 1280 the card keeps its 375 layout (the
layout rectangles are identical at every width) and is drawn at most 560 wide, centred; BEFORE
the card filled the raster width. Whether this reads as better is the owner's call.

## Known limitations (not fixed in this step)

- Theme defaults (inset 12, width 480) still differ from the tokens (16, 560); the default
  `vc_render_state` path keeps the theme values, and a test pins the gap.
- `type.label` and `control.min_target` resolve but do not drive the painter.
- vf_theme keeps a second spacing and type scale (TH_FONT_REM, TH_SPACE_REM, TH_PAD_CARD); the
  pen offset in the painter still comes from it.
- vf_layout narrows no column (below 234 wide the right padding is lost, below 192 the input overflows)
  and measures no text (the 26 px title and a long label overflow their 80 px nodes).
- The 560 clamp and the centring live in the renderer, not in vf_layout.
- The breakpoint is 600 in vf_theme and 640 in vf_layout; unchanged and pinned in vf_ring1.
- Error and disabled input states are not drawn; the focus ring on the filled Save button is
  accent on accent; the cap height ratio 700 milli is assumed, not measured.
- PR 13254 is stacked on PR 13250 and carries three inherited master reds (scan,
  duplicate-bodies, documented subcommands) that PR 13225 is meant to clear.

## How the frames were made

```
t27c gen specs/t27os/vf_card_main.t27 > vf_card.zig
zig build-exe vf_card.zig -lc
FRAME_W=375 FRAME_PCT=100 FRAME_THEME=0 FRAME_OUT=after_375_p100_t0.ppm ./vf_card 2> after_375_p100_t0.txt
ffmpeg -y -i after_375_p100_t0.ppm after_375_p100_t0.png
```

Run in T27 OS pane 15, not on the Mac. The seven AFTER rasters were rendered again after the
last code change and are byte-identical (md5) to the ones rendered before it. The first line of
each `.txt` (it names a path on the machine) was dropped.

## Tests behind the numbers (T27 OS pane 15, head 70cf34dc61)

vf_card_tok 15, vf_card 20, vf_theme 11, vf_storybook 13, vf_proportion 25, vf_tokens 21
(t27c and t27b, 0 failed, 0 vacuous), vf_card_main 6 (t27c, 0 vacuous). The duplicate-body scan
reports only the inherited `bytes_eq` drift (11 to 12).
