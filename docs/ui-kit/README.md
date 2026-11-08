# T27F UI Kit

Single source of truth for the T27OS visual factory: tokens, components, patterns, governance.

**Branch:** `claude/t27f-007-outfit-type` — epic t27#13214.

The spec layer is the authority. These docs are derived from it; when a spec and a doc disagree, trust the spec.

## Architecture

```
visual_factory_contract.t27   ← node kinds, VfGraph, validation
        │
        ├── vf_tokens.t27     ← SSOT for all design tokens (space, type, motion, composition)
        ├── vf_theme.t27      ← SSOT for color palettes (TH_T27 black, TH_PAPER light)
        ├── vf_type.t27       ← Outfit font renderer (Regular 400, Bold 800)
        ├── vf_layout.t27     ← layout engine (flex-like, breakpoints)
        ├── vf_card.t27       ← card renderer
        ├── vf_card_tok.t27   ← token-driven card (VctMetrics)
        └── ui_kit.t27        ← 6 component builders (button, card, input, badge, nav, table)
```

## Canonical themes

| Theme | Const | Background | Text |
|-------|-------|-----------|------|
| Site dark | `TH_T27` | `#000000` | `#FFFFFF` |
| Paper light | `TH_PAPER` | `#FFFFFF` | `#000000` |

**TH_T27 is the primary theme.** All UI history triptychs render on TH_T27 (black background).

## T27OS location

Petal **U — UNIVERSITY** (index 20) in the 27-petal triangle menu.
Spec: [`specs/t27f/petal_university_viz.t27`](../../specs/t27f/petal_university_viz.t27).

## Files in this directory

| File | Contents |
|------|---------|
| `tokens.md` | All 32 design tokens with values, units, and semantic aliases |
| `components.md` | 6 components: variants, states, node graphs, token dependencies |
| `patterns.md` | Recurring layout and interaction patterns |
| `governance.md` | Rules for adding/changing UI elements |
| `changelog.md` | UI history steps (derived from ui_history_log.t27) |

## Quick reference

Font: **Outfit** — Regular (400), Bold (800). Sizes: 13 / 16 / 20 / **26** / 33 / 42 px.
Heading/body ratio = 26/16 = **1.625 ≈ φ** (golden ratio, intentional).
Triptych widget background: always `#000000`.

Refs #13214
