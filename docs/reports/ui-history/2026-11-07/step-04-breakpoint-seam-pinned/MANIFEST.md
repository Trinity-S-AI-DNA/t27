# UI history, step 04: the 600 / 640 breakpoint seam stays pinned

Refs gHashTag/t27#13214 (T27F-007). One step of the T27F UI development history: a folder per
step, frames never overwritten, one manifest per step. A film is assembled later from the steps
in folder order, each step as BEFORE, what changed, AFTER, DIFF, what remains.

The source of truth for this step is the typed record `specs/t27f/ui_history_s04.t27` (schema and
rules in `specs/t27f/ui_history.t27`, the ordered log in `specs/t27f/ui_history_log.t27`). It
validates itself in its own tests, and the log is append-only. This file is the human reading copy
of the same fields; where the two differ, the spec wins. Steps 01, 02 and 03 stay as filed: this
step is a new record.

## Record

```
id:                    ui-2026-11-07-step-04
status:                DRAFT
date:                  2026-11-07
title:                 The 600 / 640 breakpoint seam stays pinned, recorded with frames at the four seam widths
branch:                claude/t27f-007-outfit-type
commit:                the commit that adds this record
commit_before:         853db3e75b (BEFORE and AFTER are the same behaviour, one binary run twice)
pushed:                no, the commits after bd1b9ece89 are local only
theme:                 t0 (t27 black)
viewport:              599, 600, 639 and 640 (the four widths around the seam)
scale:                 100 percent
visual_review:         PENDING_OWNER
```

`visual_review` stays `PENDING_OWNER` until the owner has looked at the frames and said so in
chat. No frame in this folder has been looked at by a person. BEFORE and AFTER are the same
binary run twice on purpose; the DIFF is empty (black). Whether the seam should be closed, and
at which width, is the owner's call.

## Paths (relative to this folder)

```
before_paths:   01-before/before_<w>_p100_t0.png      4 frames
after_paths:    02-after/after_<w>_p100_t0.png         4 frames
                02-after/after_<w>_p100_t0.txt         token and meaning lines of each frame
diff_paths:     03-diff/cmp_<w>_p100_t0.png            4 frames (black: identical rasters)
frames:         599_p100_t0, 600_p100_t0, 639_p100_t0, 640_p100_t0
```

Each `cmp_*` is three cells side by side at true size: BEFORE, AFTER, DIFF (auto-levelled). At
all four widths the DIFF cell is black because the rasters are identical.

BEFORE and AFTER are the same binary (853db3e75b); the step proves determinism and the absence
of a seam effect in the task card, not a decision about the seam.

## Changed rules, BEFORE to AFTER

No value changed in this step. The two entries name what this step records, not values that moved.

| Rule                         | BEFORE                                                             | AFTER                                                               |
|------------------------------|--------------------------------------------------------------------|---------------------------------------------------------------------|
| breakpoint.seam_600_640      | 600 in vf_theme and 640 in vf_layout, pinned only as numbers      | unchanged and still pinned; now a history step with frames at the four seam widths |
| breakpoint.card_dependency   | not recorded whether the task card reads either width class        | measured: source search and frames both show the card reads neither |

## What changed for a viewer (claims about numbers, not about looks)

Two renders per width by the same binary (built from 853db3e75b) are byte-identical at all four
widths (md5 equal, 0 differing pixels). That is a fact about bytes and about determinism, not
evidence of quality.

Rectangles, equal at all four widths (from the `.txt` meaning lines):

| Node              | Rectangle at all four widths |
|-------------------|------------------------------|
| card (0)          | 0,0 <width>x230              |
| title (1)         | 16,16 80x20                  |
| badge (2)         | 16,52 48x24                  |
| field stack (3)   | 16,92 160x66                 |
| owner label (4)   | 16,92 80x20                  |
| input (5)         | 16,118 160x40                |
| actions stack (6) | 16,174 202x40                |
| Save (7)          | 16,174 96x40                 |
| Cancel (8)        | 122,174 96x40                |

Only the root card width changes (599, 600, 639, 640); every interior node is identical.

The seam as vf_ring1 pins it: 40 disagreements in widths 1 to 2000 — 39 at widths 601 to 639
and 1 at width 1024. None outside those two windows. The three preview widths 375, 768 and 1280
agree between the two layers.

Source search: `direction_md`, `BP_MD` and `DIR_INHERIT` occur in none of `vf_card`,
`vf_card_tok`, `vf_storybook` and `vf_card_main`, and `th_bp` and `vf_breakpoint` occur in none
of those four or of `vf_render` and `vf_scale`. The seam is not read by the task card.

## Known limitations (not fixed in this step)

- No person has looked at any frame in this step.
- BEFORE and AFTER are one binary run twice: the step proves determinism and no change, not a
  decision; whether the breakpoint is 600 or 640 stays the owner's pick and stays pinned.
- The root card is as wide as the viewport at all four widths although `layout.card.max_inline`
  resolves to 560; that was measured and is not changed here.
- The seam is read through the task card and the vf_ring1 five-node sample graph; another graph
  may read the class and may show a difference.
- Widths 768 and 1280 are not expanded before the token scale is reconciled with vf_proportion.

## How the frames were made

```
BIN=target/lane/vc4/vf_card   # binary of 853db3e75b
FRAME_W=599 FRAME_PCT=100 FRAME_THEME=0 FRAME_OUT=before_599_p100_t0.ppm $BIN 2>before_599_p100_t0.txt
FRAME_W=599 FRAME_PCT=100 FRAME_THEME=0 FRAME_OUT=after_599_p100_t0.ppm  $BIN 2>after_599_p100_t0.txt
ffmpeg -y -i before_599_p100_t0.ppm before_599_p100_t0.png
magick compare -metric AE before_599_p100_t0.png after_599_p100_t0.png null:
magick before_599_p100_t0.png after_599_p100_t0.png ( before_599_p100_t0.png after_599_p100_t0.png
    -compose difference -composite -auto-level ) +append cmp_599_p100_t0.png
```

Same pattern for 600, 639, 640. Run in T27 OS pane 15 (target/lane/k-p6.txt), not on the Mac.
The first line of each `.txt` (it names a path on the machine) was dropped.

## Tests behind the numbers (T27 OS pane 15, t27c and t27b, 0 failed, 0 vacuous)

ui_history_s04 3 (new); ui_history_log 7 (was 6; +1 for step-04 test). The duplicate-body scan
reports only the inherited `bytes_eq` drift (11 to 12, same as step 03).
