# Design Tokens

SSOT: `specs/t27f/vf_tokens.t27` (VTK_MANIFEST array) + `specs/t27f/vf_theme.t27`.
All values are spec-verified (asserted in test suites). Do not hand-edit — change the spec.

---

## Colors — TH_T27 (primary / site dark)

SSOT: `TH_RGB_T27` in `vf_theme.t27`.

| Const | Hex | Role |
|-------|-----|------|
| `TH_BG` | `#000000` | Background |
| `TH_TEXT` | `#FFFFFF` | Primary text |
| `TH_MUTED` | `#888888` | Secondary / muted text |
| `TH_ACCENT` | `#00FF88` | Interactive green (buttons, links) |
| `TH_ACCENT_DARK` | `#00CC66` | Accent hover / pressed |
| `TH_GOLD` | `#FFD700` | Gold (special calls, awards) |
| `TH_BORDER` | `#141414` | Near-black separator / secondary bg |
| `TH_CARD_BORDER` | `#333333` | Card edge (resting) |
| `TH_CARD_BORDER_HOVER` | `#FFFFFF` | Card edge (hover) |
| `TH_CARD_DESC` | `#A6A6A6` | Card description text |
| `TH_CARD_FOOT` | `#8A8A8A` | Card footer text |

## Colors — TH_PAPER (light / print)

| Const | Hex | Role |
|-------|-----|------|
| `TH_BG` | `#FFFFFF` | Background |
| `TH_TEXT` | `#000000` | Primary text |
| `TH_MUTED` | `#666666` | Secondary text |
| `TH_ACCENT` | `#00A35C` | Interactive green |
| `TH_BORDER` | `#EBEBEB` | Light separator |
| `TH_CARD_BORDER` | `#CCCCCC` | Card edge |

---

## Typography

SSOT: `vf_tokens.t27` (indices 6–11, 24–27), `vf_type.t27`.

**Font family:** Outfit (from `outfit_r_data.t27`, `outfit_b_data.t27`)
**Font weights:** Regular = `VT_REGULAR` (400), Bold = `VT_BOLD` (800)
**Size range:** `VT_MIN_PX` to 80 px

### Type size scale

| Token | Alias | Value (px/lu) | Weight in components |
|-------|-------|--------------|---------------------|
| `type.size.caption` | — | 13 | — |
| `type.size.body` | `type.body`, `type.label` | 16 | Regular |
| `type.size.lead` | — | 20 | — |
| `type.size.heading` | `type.heading` | **26** | **Bold** |
| `type.size.title` | `type.title` | 33 | Bold |
| `type.size.display` | — | 42 | — |

**Heading/body ratio = 26/16 = 1.625 ≈ φ (1.618) — intentional golden ratio.**

### Line heights

Formula: `(px × OF_STRIP_H + OF_EM − 1) / OF_EM` (integer ceiling, no float).

| Size | Line height | Ratio |
|------|------------|-------|
| 16 px body | 24 px | 1.50× |
| 26 px heading | 36 px | 1.38× |

### Font scale ratios (TH_FONT_REM, relative to 16 px root)

```
0.750 · 0.812 · 0.875 · 1.000 · 1.272 · 1.618 · 2.058 · 2.618 · 4.236 · 6.854
```

Steps 3 (1.000) and 5 (1.618 = φ) anchor body and heading.

---

## Space scale

SSOT: `vf_tokens.t27` (indices 0–5, 18–23, 31). Unit: logical units (lu = px at 100% scale).

### Base scale

| Token | Value (lu) | Note |
|-------|-----------|------|
| `space.1` | 4 | minimum gap |
| `space.2` | 6 | control inset, label gap |
| `space.3` | 10 | action gap |
| `space.4` | 16 | card padding, group gap, page gutter |
| `space.5` | 28 | section gap |
| `space.6` | 44 | largest gap |

Sequence: 4 → 6 → 10 → 16 → 28 → 44 (each step ≈ previous × φ, rounded).

### Semantic aliases

| Alias token | Base | Value | Use |
|-------------|------|-------|-----|
| `space.control.inset` | `space.2` | 6 | Horizontal padding inside button/input |
| `space.field.label_gap` | `space.2` | 6 | Gap between label and field |
| `space.card.padding` | `space.4` | 16 | Card inner padding |
| `space.field.group_gap` | `space.4` | 16 | Gap between form groups |
| `space.page.gutter` | `space.4` | 16 | Page horizontal gutter |
| `space.action.gap` | `space.3` | 10 | Gap between action buttons |
| `space.section.gap` | `space.5` | 28 | Gap between sections |

---

## Motion

SSOT: `vf_tokens.t27` (indices 12–15). Unit: milliseconds.

| Token | Value | Use |
|-------|-------|-----|
| `motion.duration.fast` | 100 ms | micro-interactions |
| `motion.duration.normal` | 160 ms | standard transitions |
| `motion.duration.slow` | 260 ms | layout shifts |
| `motion.duration.scene` | 420 ms | page-level transitions |

---

## Composition

SSOT: `vf_tokens.t27` (indices 16–17). Unit: ppm (parts per million = ratio × 10⁶).

| Token | Value (ppm) | Fraction | Note |
|-------|------------|---------|------|
| `comp.primary_share` | 618034 | φ⁻¹ ≈ 0.618 | Golden ratio major share |
| `comp.secondary_share` | 381966 | 1 − φ⁻¹ ≈ 0.382 | Golden ratio minor share |

Used by `vtk_split` to divide a width into the golden pair.

---

## Explicit (non-formula) tokens

| Token | Value | Unit | Rationale |
|-------|-------|------|-----------|
| `layout.card.max_inline` | 560 | lu | Owner-fixed max card width |
| `control.min_target` | 24 | lu | WCAG 2.2 minimum touch target |
| `border.width` | 1 | lu | Base unit for borders |

---

## Total: 32 tokens

Indices in VTK_MANIFEST:
```
[0..5]   space.1–6        base space scale
[6..11]  type.size.*      type size scale
[12..15] motion.duration.*
[16..17] comp.*           golden composition
[18..23] space.card.*  space.field.*  space.action.*  space.section.*  space.page.*
[24..27] type.title  type.heading  type.body  type.label  (aliases)
[28..30] layout.card.max_inline  control.min_target  border.width
[31]     space.control.inset
```

Refs #13214

## Radius, density and DPR (vf_style)

These are not part of the 32 `vf_tokens` above. They live in `specs/t27f/vf_style.t27`, and its
tests are the record of their values; this section only points there.

- Radius: `RADIUS_SM`, `RADIUS_MD` (= `STYLE_RADIUS`), `RADIUS_LG`, plus `radius_full(w, h)` and
  `radius_fit(r, w, h)`. The sm and lg values are style-guide picks, not phi steps.
- Density: `DENSITY_COMPACT`, `DENSITY_DEFAULT`, `DENSITY_COMFORTABLE`. `space_at(step, d)` shifts
  the phi spacing scale one step down or up, so density adds no new number.
- DPR: `dpr_clamp` (1..3), `backing_px`, `backing_rgb_bytes`, `dpr_pct` (the `vf_scale` percent).
  The course HUD raster is still 1x; wiring it is UIKIT Wave A slice A4.
- Shadow: none, because no painter draws one.

Refs #13251
