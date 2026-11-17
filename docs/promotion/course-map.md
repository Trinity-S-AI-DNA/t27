# Course map: 27 lessons (idea + widget + result)

Part of `[EPIC] T27_PROMOTION_KERNEL_V5` — Part of #13473.
M0-02: Course map: 27 lessons (idea + widget + result). Closes #13475.

## Goal

Build the 27-lesson course map. Each lesson lists one core idea, one widget, and one measurable result.

Every widget in this map is a **compiled t27 spec with visual output** (see widget-inventory.md). The map is the planning layer; widgets are built as specs and sealed before they are demonstrated.

## Constraints (M0 law)

- Docs/planning only — no X posts, no outbound email, no paid-token usage without owner approval.
- Free models first; on rate limit mark `WAITING_RATE_LIMIT` and stop (no paid fallback).
- Claims stay inside the approved patterns from the epic (arXiv:2606.09686 and arXiv:2606.05017 scope).
- Boundary: `docs/promotion/course-map.md` and `docs/promotion/README.md`.

## The 27 lessons

| # | Lesson (idea) | Widget | Measurable result |
|---|---------------|--------|-------------------|
| 1 | Why numbers need a new family: the phi rule | interactive number | learner names one phi-derived rule (GoldenFloat E = round((N-1)/phi^2)) |
| 2 | The anchor identity: phi^2 + 1/phi^2 = 3 | bit layout explorer | learner verifies phi^2 + 1/phi^2 = 3 on the widget, bit-exact |
| 3 | Reading a GoldenFloat block: sign, exponent, mantissa | encode/decode | learner encodes GF16 and decodes it back, round-trip exact |
| 4 | Rounding half-to-even at an exact tie | rounding | learner rounds a tie and gets half-to-even, 10/10 ties correct |
| 5 | Overflow: what happens past the widest format | overflow | learner triggers overflow and reads the documented behavior |
| 6 | NaN and the reserved special row | NaN | learner produces NaN and identifies the reserved row |
| 7 | Block scale: one scale for a vector of values | block scale | learner scales a block and reports the shared exponent |
| 8 | Ternary addition: one trit at a time | ternary adder | learner adds two balanced-ternary numbers, digit-exact |
| 9 | Ternary multiplication | ternary multiplier | learner multiplies two ternary numbers, digit-exact |
| 10 | The whole GoldenFloat family GF4–GF1024 in one catalog | encode/decode | learner names all widths GF4, GF8, … GF1024 (27 widths) |
| 11 | Conformance packs: bit-exact vectors with SHA-256 | before/after | learner verifies a pack fingerprint matches SHA-256 |
| 12 | Open tools only: no Vivado | FPGA blink | learner runs the blink demo with open tools, no Vivado |
| 13 | A neural training loop on ternary weights | neural training loop | learner runs one training epoch and sees loss decrease |
| 14 | Reading a loss curve | loss curve | learner reads loss at epoch 1 and epoch N, loss(N) < loss(1) |
| 15 | Before/after: t27 spec vs compiled output | before/after | learner sees source and generated output side by side, identical semantics |
| 16 | Live build: compile a spec and watch it run | live build | learner compiles one spec and the widget updates without reload |
| 17 | Agent swarm view: 27 letters, one panel | agent swarm view | learner counts 27 agent consoles on the panel |
| 18 | Course progress: your place in the 27 lessons | course progress | learner's progress bar matches completed lessons |
| 19 | The matryoshka: each layer compiles itself | live build | learner sees one layer build the next, bit-identical |
| 20 | The golden chain: one .t27 contract to silicon | FPGA blink | learner traces spec → generated Verilog → FPGA evidence |
| 21 | 6 conformance packs, one fingerprint each | before/after | learner counts 6 packs and verifies each SHA-256 |
| 22 | 84 formats, 13 clusters, one catalog | encode/decode | learner reads 84 formats and 13 clusters from the catalog |
| 23 | abs_error published, never hidden | loss curve | learner reads abs_error beside every vector |
| 24 | One catalog, six languages | encode/decode | learner sees Markdown, JSON, Python, Rust, C, TypeScript output |
| 25 | Ternary machine on FPGA: open tools | FPGA blink | learner runs ternary blink on ALINX AX7203 with open tools |
| 26 | Training gradients stay in bf16 | neural training loop | learner confirms gradients are bf16, forward on int8 tensor cores |
| 27 | From widget to contributor: good first issue | course progress | learner files one good-first-issue referencing this course |

That is exactly 27 lessons. Each lesson has one idea, one widget, and one measurable result.

## Linked assets

- `docs/promotion/widget-inventory.md` — the 16 widget types each lesson draws from
- `docs/promotion/README.md` — promotion kernel index
- `specs/numeric/paper_claims.t27` — live catalog: 109 records, 13 clusters (note: "84 formats" and "12 clusters" are refused by the gate, see #13465)
- `specs/numeric/gf16.t27` + `conformance/FORMAT-SPEC-001.json` — numeric SSOT (Law L6)

// phi^2 + 1/phi^2 = 3 | TRINITY
