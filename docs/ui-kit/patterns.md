# Patterns

Recurring compositions of components. Each pattern is a named arrangement of `kit_*` builders; none defines new components. SSOT: `specs/t27f/ui_kit.t27` + `specs/t27f/vf_layout.t27`.

---

## Form layout

A vertical stack of labeled fields with an action row at the bottom.

**Structure:**
```
VF_STACK (DIR_COLUMN, space.section.gap=28 between groups)
  VF_STACK (input group A, DIR_COLUMN, space.field.group_gap=16 between fields)
    kit_input(KV_PRIMARY, KS_DEFAULT)
    kit_input(KV_PRIMARY, KS_DEFAULT)
  VF_STACK (action row, DIR_ROW, space.action.gap=10 between buttons)
    kit_button(KV_PRIMARY, KS_DEFAULT)
    kit_button(KV_SECONDARY, KS_DEFAULT)
```

**Token chain:** `space.section.gap` (28) → `space.field.group_gap` (16) → `space.field.label_gap` (6) → `space.control.inset` (6).

**When to use:** any data-entry flow with 1-5 fields and one or two submit/cancel buttons.

**Anti-pattern:** nesting an action row inside a card and also having a top-level submit button — pick one submission point per form.

---

## Action row

A horizontal row of buttons, leftmost is primary.

**Structure:**
```
VF_STACK (DIR_ROW, space.action.gap=10)
  kit_button(KV_PRIMARY, state)
  kit_button(KV_SECONDARY, state)   ← cancel / secondary action
```

**Rule:** at most 3 buttons in an action row. Beyond 3, group secondary actions with a ghost menu button.

**Token:** `space.action.gap` (10) — alias of `space.3`.

**When to use:** inside `kit_card` (already built-in at node 3), at the bottom of a form, or in a toolbar.

**Anti-pattern:** an action row of KV_GHOST buttons only — use KV_PRIMARY for the leading action.

---

## Card grid

A row of cards at equal width, filling the viewport with a maximum per-card width.

**Structure:**
```
VF_CONTAINER (DIR_ROW, wrap, space.action.gap=10 between cards)
  kit_card(KV_PRIMARY, KS_DEFAULT)
  kit_card(KV_PRIMARY, KS_DEFAULT)
  ...
```

**Max card width:** `layout.card.max_inline` = 560 lu — set on the VF_CARD node.

**When to use:** dashboards, catalog pages, search results.

**Anti-pattern:** a card grid inside another card — keep nesting to one level.

---

## Empty state

A centered text message with an optional action button.

**Structure:**
```
VF_CONTAINER (DIR_COLUMN, centered)
  VF_TEXT (KV_SECONDARY, text "No items yet")  ← type.lead (20 px)
  kit_button(KV_PRIMARY, KS_DEFAULT)            ← optional CTA
```

**When to use:** lists with zero items, search results with no matches, first-use flows.

**Anti-pattern:** showing an empty state before a data fetch completes — use the loading state instead.

---

## Loading state

A skeleton or muted indicator that occupies the same space as the loaded content.

**Structure:** replace a `kit_card` or `kit_input` root with a VF_BADGE at `KV_SECONDARY` + `KS_DISABLED`.

**Rules:**
- Loading skeleton must match the bounding box of the loaded component.
- Use `KS_DISABLED` on the placeholder node so the renderer draws it in muted fill.
- Never show a spinner where a skeleton fits.

**When to use:** any async fetch that takes >100 ms (i.e., one `motion.duration.fast` tick).

---

## Token chain summary

| Pattern | Key tokens (innermost → outermost) |
|---------|-----------------------------------|
| Form layout | inset(6) → label_gap(6) → group_gap(16) → section_gap(28) |
| Action row | action_gap(10) |
| Card grid | action_gap(10) between cards, card.padding(16) inside |
| Empty state | — |
| Loading state | min_target(24) |

Refs #13214
