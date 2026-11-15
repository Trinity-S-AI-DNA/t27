# Widget inventory

Part of #13473. Issue #13476. Status: DRAFT_UNSENT. Nothing here was posted.

The 16 type names are the list in #13476 FR-003. They are not ids in the live course. The course widget ids are `LESSON_WIDGETS` in the trinity course blob `3b3aa5d21a1dc86876c5b873bf6d6b8cf8d20b21` (see `docs/promotion/course-map.md`).

`test -f specs/widgets/gallery.t27` exited 1. This checkout cannot show that a type is a compiled gallery widget.

`nearest` means a lesson widget or a file in this repo that covers part of the same words. It is not a claim that the V5 type exists under that name.

| type | status | nearest source | visual output checked |
| --- | --- | --- | --- |
| interactive number | nearest | lesson 4 widget `gf16-calc` | screenshot N/A |
| bit layout explorer | nearest | lesson 2 widget `idcode` | screenshot N/A |
| encode/decode | nearest | lesson 2 widget `idcode` | screenshot N/A |
| rounding | nearest | lesson 4 widget `gf16-calc` | screenshot N/A |
| overflow | absent | not in `LESSON_WIDGETS` | N/A |
| NaN | absent | not in `LESSON_WIDGETS` | N/A |
| block scale | absent | not in `LESSON_WIDGETS`; `conformance/vectors/block_fp_conformance_v0.json` is a vector file, not a widget | N/A |
| ternary adder | nearest | lesson 15 spec `specs/ternary/ternary_full_adder.t27` (`test -f` present); widget on that lesson is `lut-explain` | screenshot N/A |
| ternary multiplier | nearest | lesson 8 widget `test-waves` | screenshot N/A |
| FPGA blink | nearest | lesson 24 widget `bit-autopsy`; the lesson task text in the blob names a `blink_j26` sample | screenshot N/A |
| neural training loop | absent | not in `LESSON_WIDGETS` | N/A |
| loss curve | absent | lesson 5 widget `cost-curves` is a LUT-cost chart (`specs/widgets/cost_curves.t27` present), not a training-loss curve | screenshot N/A |
| before/after | partial | `specs/course/lesson_01_diff_main.t27` present; not a `LESSON_WIDGETS` id | screenshot N/A |
| live build | nearest | lesson 23 widget `build-receipt` | screenshot N/A |
| agent swarm view | absent | not a `LESSON_WIDGETS` id; lesson 27 `LESSON_ALSO` includes `race-the-bee` | screenshot N/A |
| course progress | partial | `specs/course/course_hud_state.t27` present; not a `LESSON_WIDGETS` id | screenshot N/A |

## Asset fields

| Field | Value |
| --- | --- |
| claim | N/A for the 16 type names; the approved sentence "Every course widget is a compiled t27 spec." is a content-bank draft, not a result of this inventory |
| artifact | `docs/promotion/widget-inventory.json` |
| reproducible path | `test -f` lines above, plus the course blob fetch in `course-map.md` |
| visual | N/A -- no screenshot |
| CTA | N/A -- not sent |
| metric | 16 types listed; gallery membership not measured |

Machine-readable copy: `docs/promotion/widget-inventory.json`.
