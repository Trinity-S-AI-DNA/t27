# Widget inventory: 16 types → lessons

Part of `[EPIC] T27_PROMOTION_KERNEL_V5` — Part of #13473.
M0-03: Widget inventory (16 types → lessons). Closes #13476.

## Goal

Inventory the 16 widget types from the V5 spec and map each to lessons. Every widget MUST be a compiled t27 spec with visual output.

## Constraints (M0 law)

- Docs/planning only — no X posts, no outbound email, no paid-token usage without owner approval.
- Free models first; on rate limit mark `WAITING_RATE_LIMIT` and stop (no paid fallback).
- Claims stay inside the approved patterns from the epic.
- Boundary: `docs/promotion/widget-inventory.md`, `docs/promotion/widget-inventory.json`, `docs/promotion/README.md`.

## The 16 widget types

| # | Widget type | Maps to lesson(s) | Visual output |
|---|-------------|-------------------|---------------|
| 1 | interactive number | 1 | a number the learner adjusts and reads back |
| 2 | bit layout explorer | 2, 3 | sign / exponent / mantissa fields highlighted |
| 3 | encode/decode | 3, 10, 22, 24 | source ↔ generated output, round-trip exact |
| 4 | rounding | 4 | half-to-even tie rounding, digit-by-digit |
| 5 | overflow | 5 | overflow boundary and documented behavior |
| 6 | NaN | 6 | NaN value and the reserved special row |
| 7 | block scale | 7 | one shared exponent across a vector |
| 8 | ternary adder | 8 | balanced-ternary digit addition |
| 9 | ternary multiplier | 9 | balanced-ternary digit multiplication |
| 10 | FPGA blink | 12, 20, 25 | blinking LED on ALINX AX7203, open tools |
| 11 | neural training loop | 13, 26 | one epoch, loss decreasing |
| 12 | loss curve | 14, 23 | loss over epochs, abs_error beside each vector |
| 13 | before/after | 11, 15, 21 | source side-by-side with compiled output |
| 14 | live build | 15, 16, 19 | widget updates on compile without reload |
| 15 | agent swarm view | 17 | 27 agent consoles on one panel |
| 16 | course progress | 18, 27 | progress bar matching completed lessons |

## Rule: every widget is a compiled t27 spec

Each widget above is implemented as a `.t27` spec with `test` blocks, sealed, and generated to visual output. A widget that is not a compiled spec with visual output does not count as inventory.

## Machine-readable inventory

See `docs/promotion/widget-inventory.json` — the same 16 types, one offer each, keyed by widget id.

## Linked assets

- `docs/promotion/course-map.md` — the 27 lessons each widget serves
- `docs/promotion/README.md` — promotion kernel index

// phi^2 + 1/phi^2 = 3 | TRINITY
