# netlist_card — evidence

Issue #13673, epic #13650. Job nc11, 2026-11-14.

| File | Contents |
|---|---|
| manifest.txt | base commit, overlay sha256, toolchain, kit baseline sha |
| rc.txt | card render exit codes (tests=0, gen=0, build=0, 36 of 36 distinct, 36 of 36 repeat) |
| measure.txt | per-frame, per-node measurements (294 lines, 36 frames, 7 tests pass) |
| sheet-t0.png | 18 cards × light theme contact sheet |
| sheet-t1.png | 18 cards × dark theme contact sheet |
| gallery-t0.png | UI Kit + card sheet, light theme |
| gallery-t1.png | UI Kit + card sheet, dark theme |
| vr-diff.txt | UI Kit baseline diff (empty = all 68 frames match master baseline) |
| t27l_card.json | T27L record |

## Measurements summary (nc11)

- Badges: 0 of 84 ring-spill, padding symmetric (l=5..6, r=5..6)
- Text nodes: 0 of 70 ring-spill
- Overflow (layout estimate): 0 frames
- Repeatability: 36 of 36 repeat renders byte-equal
- UI Kit baseline: 68 of 68 frames equal
- 7 of 7 tests pass; vacuous-pass counter unknown (compiler defect)

## Source state

- Base: origin/master `2c87b30aa2` (3189 master specs)
- Overlay: `specs/analog/netlist_card.t27` sha256 `d3f12b158fc31b34...`
- Toolchain: t27c 0.5.2, zig 0.18.0-dev.1005+75d48dfff, ffmpeg 6.1.1
