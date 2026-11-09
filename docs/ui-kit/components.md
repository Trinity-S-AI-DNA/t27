# Components

SSOT: `specs/t27f/ui_kit.t27`. Six component builders, each returns a `UiKit` (a `VfGraph` plus per-node variant/state arrays).

Architecture: **object graph → meaning → layout → theme → render**. Components carry no colors, no pixel sizes, and no layout logic — those are resolved by the theme and layout layers.

---

## Shared vocabulary

### Variants (KV_*)

| Const | Value | Semantic |
|-------|-------|---------|
| `KV_PRIMARY` | 0 | Primary action / emphasis |
| `KV_SECONDARY` | 1 | Secondary / subdued |
| `KV_GHOST` | 2 | Transparent / no-fill |
| `KV_SUCCESS` | 3 | Positive / green |
| `KV_WARN` | 4 | Warning / destructive |

### States (KS_*)

| Const | Value | Semantic |
|-------|-------|---------|
| `KS_DEFAULT` | 0 | Resting |
| `KS_HOVER` | 1 | Mouse over |
| `KS_PRESSED` | 2 | Active press |
| `KS_DISABLED` | 3 | Non-interactive |
| `KS_ERROR` | 4 | Validation error |
| `KS_FOCUS` | 5 | Keyboard focus |

### Node kinds (VF_* from visual_factory_contract.t27)

| Kind | ID | Children | Interactive |
|------|-----|---------|------------|
| `VF_CONTAINER` | 1 | ≤8 | No |
| `VF_STACK` | 2 | ≤8 | No |
| `VF_TEXT` | 3 | 0 | No |
| `VF_BUTTON` | 4 | 0 | Yes |
| `VF_INPUT` | 5 | 0 | Yes |
| `VF_CARD` | 6 | ≤8 | No |
| `VF_BADGE` | 7 | 0 | No |
| `VF_LIST` | 8 | ≤8 | No |

Maximum 16 nodes per graph (`VF_MAX_NODES`).

---

## Button — `kit_button(variant, state)`

The simplest interactive element.

**Node layout (2 nodes):**
```
0: VF_CONTAINER (root)
1: VF_BUTTON    (parent 0, text "Button")  ← carries variant and state
```

**Variants:** KV_PRIMARY, KV_SECONDARY, KV_GHOST, KV_SUCCESS, KV_WARN
**States:** KS_DEFAULT, KS_HOVER, KS_PRESSED, KS_DISABLED, KS_ERROR, KS_FOCUS

**Token dependencies:**
- `space.control.inset` (6 lu) — horizontal padding left and right, always equal (step 05 law)
- `control.min_target` (24 lu) — minimum height
- `type.body` (16 px), Bold — label text
- `TH_ACCENT` (#00FF88) — PRIMARY fill
- `TH_BORDER` (#141414) — SECONDARY outline

**Step 05 invariant:** `sl + sr + 1 ≈ 2 × button_center ± 4 px` (label horizontally centered; tested in `vct_button_labels_centred_in_token_card`).

---

## Card — `kit_card(variant, state)`

A container with title, body text, and one action button.

**Node layout (5 nodes):**
```
0: VF_CARD   (root, DIR_COLUMN, carries variant/state)
1: VF_TEXT   (parent 0, text "Title",  KV_PRIMARY)
2: VF_TEXT   (parent 0, text "Body",   KV_SECONDARY)
3: VF_STACK  (parent 0, DIR_ROW, KV_PRIMARY)  ← action row
4: VF_BUTTON (parent 3, text "Action", KV_PRIMARY)
```

**States:** KS_DEFAULT, KS_HOVER (card-level)

**Token dependencies:**
- `space.card.padding` (16 lu) — inner padding
- `space.action.gap` (10 lu) — gap between buttons in action row
- `type.heading` (26 px, Bold) — title
- `type.body` (16 px, Regular) — body text
- `layout.card.max_inline` (560 lu) — max card width
- `TH_CARD_BORDER` / `TH_CARD_BORDER_HOVER` — card border

**Token card variant:** `vf_card_tok.t27` — same structure but all sizes/spacing derive from `VctMetrics` (token-driven).

---

## Input — `kit_input(variant, state)`

A labeled text field with optional error message.

**Node layout (3 nodes default; 4 when state = KS_ERROR):**
```
0: VF_STACK (root, DIR_COLUMN)
1: VF_TEXT  (parent 0, text "Label",  KV_PRIMARY)
2: VF_INPUT (parent 0, label_ref=1,  carries variant/state)
3: VF_TEXT  (parent 0, text "Error",  KV_WARN, KS_ERROR)  ← only when KS_ERROR
```

**States:** KS_DEFAULT, KS_HOVER, KS_FOCUS, KS_ERROR, KS_DISABLED

**Token dependencies:**
- `space.field.label_gap` (6 lu) — gap between label and input
- `space.field.group_gap` (16 lu) — gap between input groups
- `space.control.inset` (6 lu) — horizontal padding inside input
- `type.body` (16 px, Regular) — label and input text
- `control.min_target` (24 lu) — minimum input height

**Accessibility:** input node carries `P_LABEL_REF` pointing to its label node (index 1).

---

## Badge — `kit_badge(variant)`

A single-node status indicator.

**Node layout (1 node):**
```
0: VF_BADGE (root, text "Badge", carries variant)
```

**Variants:** KV_PRIMARY, KV_SECONDARY, KV_SUCCESS, KV_WARN

**Token dependencies:**
- `space.2` (6 lu) — horizontal padding
- `type.size.caption` (13 px) — badge text
- `control.min_target` (24 lu) — minimum height

---

## Nav — `kit_nav(active)`

A vertical navigation list with 3 items; `active` (0–2) marks the focused item.

**Node layout (4 nodes):**
```
0: VF_LIST (root, DIR_COLUMN)
1: VF_TEXT (parent 0, "Item", KS_FOCUS if active==0)
2: VF_TEXT (parent 0, "Item", KS_FOCUS if active==1)
3: VF_TEXT (parent 0, "Item", KS_FOCUS if active==2)
```

**Token dependencies:**
- `space.4` (16 lu) — item padding
- `type.body` (16 px, Regular/Bold) — nav labels
- `TH_ACCENT` — active item indicator

---

## Table — `kit_table(rows)`

A list with a header row and 1–2 data rows, each with 3 cells.

**Node layout (9 nodes for rows=1; 13 for rows=2):**
```
0:     VF_LIST  (root, DIR_COLUMN)
1:     VF_STACK (header row, parent 0)
2–4:   VF_TEXT  (header cells, KV_SECONDARY)
5:     VF_STACK (data row 1, parent 0)
6–8:   VF_TEXT  (data cells,  KV_PRIMARY)
9:     VF_STACK (data row 2, parent 0)  ← only when rows=2
10–12: VF_TEXT  (data cells,  KV_PRIMARY)
```

Capacity: 16 nodes → max 2 data rows (3rd row needs 17 nodes).

**Token dependencies:**
- `space.4` (16 lu) — cell padding
- `type.body` (16 px, Regular) — cell text
- `type.size.caption` (13 px, Regular) — header text
- `TH_BORDER` — row separator

---

## Validation

Every `kit_*` builder returns a graph that passes `vf_validate(g) == VF_OK`.
Every builder also passes `kit_parents_first(k) == true` (nodes in depth-first order, parents before children).

Refs #13214
