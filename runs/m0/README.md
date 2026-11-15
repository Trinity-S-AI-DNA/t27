# M0 first screenshot and visual regression record

Issue: Trinity-S-AI-DNA/t27#13428 (epic #13426, milestone M0). Date: 2026-11-14.
Where it ran: the VPS lane `/root/lanes/t27f-uikit-m0`, master `b339f8ae91`. No new code was written.

## How the frame was made

1. `t27c gen specs/t27os/vf_card_main.t27` wrote the Zig source.
2. `zig build-exe vf_card.zig -lc -O ReleaseSafe` made the program (exit 0).
3. The program ran with `FRAME_W=375 FRAME_PCT=200 FRAME_THEME=0|1`, twice per theme
   (baseline and after), and wrote 750x480 P6 frames.
4. `ffmpeg` turned the frames into PNG and made the difference pictures.

## Files

| File | What it is | sha256 (first 12) |
|---|---|---|
| `m0_card_v1.ppm` | the frame, theme 0, as the renderer wrote it | 3c2a3f45d9eb |
| `card_375_t0.png` | the same frame as PNG | 4b099d1f1f08 |
| `card_375_t1.png` | theme 1 | 696e7b6fca41 |
| `card_t0_after.txt` | the tokens and the laid-out nodes the frame was drawn from | 1f5fadc5bd47 |
| `diff_t0_base_after.png` | baseline against after, theme 0 | 4532e45aba57 |
| `diff_t0_vs_t1.png` | theme 0 against theme 1 (negative control) | 0efb9a6b1d75 |
| `diff_t0_vs_blank.png` | theme 0 against a black frame | 4b099d1f1f08 |
| `t27l_m0.json` | the T27L record | |

## Visual regression result

- Baseline against after: 0 bytes differ in both themes. The render is deterministic. Accepted.
- Negative control, theme 0 against theme 1: 1,060,042 of 1,080,000 pixel bytes differ, so the
  comparison does see a change.
- Theme 0 against a black frame gives the frame itself, byte for byte (same sha256 as
  `card_375_t0.png`).

## Spec sweep on the VPS (`t27c test-report`)

| Spec | Tests | Pass | Fail |
|---|---|---|---|
| ui_kit | 12 | 12 | 0 |
| visual_factory_contract | 11 | 11 | 0 |
| vf_tokens | 21 | 21 | 0 |
| vf_theme | 15 | 15 | 0 |
| vf_layout | 22 | 22 | 0 |
| vf_storybook | 13 | 13 | 0 |
| vf_snapshot | 9 | 9 | 0 |
| vf_diff, vf_card, vf_card_tok | not run | | |

The count of vacuous passes is unknown: the counting build did not compile on that toolchain.
The last three specs did not compile for the report there (t27c 0.5.2 with zig 0.18.0-dev:
"type 'u8' does not support array initialization syntax"); the renderer itself compiled and ran
through `t27c gen`.

## What this does not show

- Nobody has opened the picture and compared it with `card_t0_after.txt`. The bytes are
  compared; the look is not judged.
- The difference pictures were made with ffmpeg, not with `specs/t27f/vf_diff.t27`: that spec
  has no host program yet.
- No accessibility check was run on the frame.
