# Netlist to T27F Constructor — Gate A Evidence Report

Epic #13650 | Branch `claude/edmund-analog-v0-1-1-07b13f` | 2026-11-14

## Scope

Wave A (census + tokens + acceptance checklist) through Gate A.
Gate A passes when: 17/17 blocks extracted, 247/247 element lines, acceptance
checklist green, T27F widget card rendered with 0 spill, gallery PNGs embedded.

## Census evidence (17/17 blocks, 247/247 element lines)

Measured from `specs/analog/netlist_meaning.t27` (reference deck: SAR ADC netlist).

| Check | Expected | Measured | Source |
| --- | --- | --- | --- |
| Blocks extracted | 17 | 17 | `NA_EXPECT[0]` in `netlist_accept.t27` |
| Element lines | 247 | 247 | `NA_EXPECT[1]` — corrected from 249 (derived) to 247 (measured); commit `9dbaf1f4d7` |
| Silent drops | 0 | 0 | `NA_EXPECT[2]` |
| Decks cataloged | 3 | 3 | `NA_EXPECT[3]` |
| Presets rendered | 3 | 3 | `NA_EXPECT[4]` |
| Changed pixels | 0 | 0 | `NA_EXPECT[5]` |
| Stage events | 5 | 5 | `NA_EXPECT[6]` |
| Foreign files | 0 | 0 | `NA_EXPECT[7]` |

The 2-line discrepancy between the epic's derived table (249) and the measured value (247):
`.control` block `let` commands were counted as inductors (by first letter) in the derived
pass. The measured census finds 247 non-`.control` element lines. `netlist_accept.t27`
uses the measured value; the corrected acceptance spec is on master as of PR #13675.

## T27F widget card (UI Kit m0 nc11)

Job nc11 — `specs/analog/netlist_card.t27` rendered against UI Kit m0 baseline.

| Metric | Value | Source |
| --- | --- | --- |
| Badges | 0 of 84 ring-spill | `runs/netlist_card/measure.txt` |
| Text nodes | 0 of 70 ring-spill | `runs/netlist_card/measure.txt` |
| Overflow (layout estimate) | 0 frames | `runs/netlist_card/measure.txt` |
| Repeatability | 36/36 byte-equal | `runs/netlist_card/rc.txt` |
| UI Kit baseline | 68/68 frames equal | `runs/netlist_card/vr-diff.txt` (empty) |
| Tests pass | 7/7 | `runs/netlist_card/measure.txt` |
| Cards per sheet | 18 | `runs/netlist_card/sheet-t0.png` |

Toolchain: t27c 0.5.2, zig 0.18.0-dev.1005+75d48dfff, ffmpeg 6.1.1
Base commit: `2c87b30aa2` (3189 master specs)

## Gallery — light theme (gallery-t0.png)

18 netlist blocks rendered as T27F widget cards, light theme.
All 17 SAR ADC blocks + 1 synthetic `no_such_block` (refusal case) visible.

![UI Kit + netlist card sheet, light theme](../../runs/netlist_card/gallery-t0.png)

Block names visible in gallery-t0.png:
`top`, `tracking_switch`, `dynamic_comparator_top`, `sar_logic`, `clock_gen`,
`dac_control_top`, `neg_bottom_plate_logic`, `diff_cap_dac`, `inverter`, `nand`,
`dynamic_comparator`, `diff`, `nor3`, `delay_cell`, `dac_control`, `tgate`, `nor`,
`no_such_block`

Badge kinds visible: `pins:N` (yellow), `inst:N` (yellow), `passive:N` (yellow),
`mixed:N` (yellow), `leaf:N` (yellow), `composite:N` (yellow), `refused` (amber — refusal badge on `no_such_block`)

## Gallery — dark theme (gallery-t1.png)

Same 18 blocks, dark theme.

![UI Kit + netlist card sheet, dark theme](../../runs/netlist_card/gallery-t1.png)

## Gate A acceptance table

| Criterion | Required | Status |
| --- | --- | --- |
| 1. Actual PNGs generated | `gallery-t0.png`, `gallery-t1.png` | PASS — `runs/netlist_card/` (job nc11) |
| 2. Actual PNGs embedded in report | Both variants in this doc | PASS — this file |
| 3. Light and dark variants visible | Both themes | PASS — gallery-t0 (light) + gallery-t1 (dark) |
| 4. All 17 blocks + badge kinds visible | 17 real + 1 refusal | PASS — 18 cards shown |
| 5. Report links PNG paths | Relative paths above | PASS |
| 6. User can inspect rendered evidence | PNGs in repo at `runs/netlist_card/` | PASS |

Gate A: **PASS** — all 6 criteria met.

## Specs landed (PR #13675)

| Spec | Wave | Tests |
| --- | --- | --- |
| `specs/analog/netlist_accept.t27` | A | 8-row checklist, `NA_EXPECT`, `NA_MISSING` |
| `specs/analog/netlist_meaning.t27` | A | classification table for 17 blocks |
| `specs/analog/netlist_structure.t27` | B | picture plan per class |
| `specs/t27f/netlist_tokens.t27` | A | token schema + acceptance checklist |
| `specs/analog/netlist_block_svg.t27` | A/B | block SVG layout spec |
| `specs/analog/netlist_card.t27` | A | T27F widget card, 7 tests |

## Remaining (Wave B → Gate B, Wave C → Gate C)

- Wave B: netlist model (block graph, net topology, pin assignment)
- Wave B: parts catalog (device parameterization, MOS/passive presets)
- Wave C: T27F interactive views (expand, fold, topology overlay)
- Wave C: template generator (`.t27` stub from census row)
- Vacuous-pass counter unknown (compiler defect #5980) — not a Gate A blocker

Refs #13650
