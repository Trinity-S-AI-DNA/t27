# Changelog

UI history steps on branch `claude/t27f-007-outfit-type`. SSOT: `specs/t27f/ui_history_log.t27`.

Steps are append-only. A step's `intended_change` is the authoritative description; the entry here is derived from it.

---

## Step 05 — button padding symmetry law

**ID:** `ui-2026-11-07-step-05`
**Spec:** `specs/t27f/ui_history_s05.t27`
**Branch:** `claude/t27f-007-outfit-type`

No rendering value changes. Adds the explicit invariant that button horizontal padding is symmetric (left = right = (button_w − text_w) / 2). The rule was already in the renderer; this step adds a test that pins it on the token card. BEFORE and AFTER are the same behaviour by design — DIFF is black.

**Visual verdict:** PENDING_OWNER

---

## Step 04 — breakpoint seam pinned

**ID:** `ui-2026-11-07-step-04`
**Spec:** `specs/t27f/ui_history_s04.t27`
**Branch:** `claude/t27f-007-outfit-type`

No value changes. The 600-px breakpoint of `vf_theme` and the 640-px breakpoint of `vf_layout` stay pinned. This step records the seam as a history entry with frames at 599, 600, 639, and 640. BEFORE and AFTER are the same behaviour by design — DIFF is black.

**Visual verdict:** PENDING_OWNER

---

## Step 03 — cross-axis narrowing

**ID:** `ui-2026-11-07-step-03`
**Spec:** `specs/t27f/ui_history_s03.t27`
**Branch:** `claude/t27f-007-outfit-type`

No token value changes. A child of a column or row is clamped to the inner cross extent of its parent minus its margins. At 375 px nothing changes; at 233 px and below the action row and fields give way instead of overflowing the card. Frames at 375, 233, 201, 191, and 160 px wide.

**Residual defects:** `vf_theme` keeps a second spacing scale (TH_PAD_CARD 20 vs `space.card.padding` 16) and a stale breakpoint (600 in `vf_theme` vs 640 in `vf_layout`), both unchanged and noted.

**Visual verdict:** PENDING_OWNER

---

## Step 02 — control inset token

**ID:** `ui-2026-11-07-step-02`
**Spec:** `specs/t27f/ui_history_s02.t27`
**Branch:** `claude/t27f-007-outfit-type`

The text inset of a node reads from `space.control.inset` (6 px, alias of `space.2`) through the resolved style instead of a raw renderer constant. The text moves 0.112 px; nothing else changes. All seven AFTER rasters differ from their BEFORE rasters (md5), and each BEFORE raster equals the step 01 AFTER raster of the same name.

**Visual verdict:** PENDING_OWNER

---

## Step 01 — task card tokens

**ID:** `ui-2026-11-07-step-01`
**Spec:** `specs/t27f/ui_history_s01.t27`
**Branch:** `claude/t27f-007-outfit-type`

Establishes the token-driven task card baseline: 16 px padding and group gap, 6 px label gap, 10 px action gap, and a 560 px maximum width centred on wide rasters — all read from design tokens instead of renderer constants. Frames at 375, 768, and 1280 px wide; both themes (t0=TH_T27, t1=TH_PAPER); and 200% scale.

**Visual verdict:** PENDING_OWNER

Refs #13214
