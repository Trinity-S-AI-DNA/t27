# Self-improving loop (bounded, no paid fallback)

Part of `[EPIC] T27_PROMOTION_KERNEL_V5` — Part of #13473.
M0-11: Self-improving loop (bounded, no paid fallback). Closes #13484.

## Goal

Specify the bounded self-improving loop: metrics → next action, rate-limit handling `WAITING_RATE_LIMIT`, no paid fallback.

## Constraints (M0 law)

- Docs/planning only — no X posts, no outbound email, no paid-token usage without owner approval.
- Free models first; on rate limit mark `WAITING_RATE_LIMIT` and stop (no paid fallback).
- Claims stay inside the approved patterns from the epic (arXiv:2606.09686 and arXiv:2606.05017 scope).
- Boundary: `docs/promotion/self-improving-loop.md` and `docs/promotion/README.md`.

## The bounded self-improving loop

The loop turns measured metrics into the next action, one cycle at a time. It is **bounded-cycle only**: each cycle reads metrics, picks the next action, executes it, and stops. It does not run forever and it does not fall back to paid models.

### Cycle: metrics → next action

1. **Read metrics** — the metric tracked by the last publishable asset (from the six-field rule: claim/artifact/path/visual/cta/metric).
2. **Decide next action** — the one action that improves the metric most: fix a failing test, complete one widget, write one lesson, or file one good-first-issue.
3. **Execute** — within the M0 boundary: docs/planning/spec only, no publishing, no paid models.
4. **Record** — append the metric and the action taken to the loop log.
5. **Stop** — one cycle ends here. The next cycle starts from the next metric read.

### Rate-limit handling: WAITING_RATE_LIMIT

When a free model hits a rate limit:

- Mark `WAITING_RATE_LIMIT`.
- Save the artifacts already produced.
- **Stop.** Do not switch to a paid model. Do not use paid-token fallback.
- Resume only when the free model is available again, on the same cycle.

### No paid fallback

The loop MUST forbid paid-model fallback. Any cycle that would require a paid token instead marks `WAITING_RATE_LIMIT` and stops. Paid-token usage waits for explicit owner approval and is outside M0.

## Approved claim patterns

The loop's outputs stay inside the epic-approved patterns (arXiv:2606.09686 and arXiv:2606.05017 scope), e.g.:

- "6 bit-exact conformance packs."
- "Every course widget is a compiled t27 spec."
- "No Vivado. Open tools only."

## Publishable-asset rule

Every asset the loop produces or improves must contain all six:

1. one measurable claim
2. one artifact link
3. one reproducible path
4. one visual proof
5. one CTA
6. one metric to track

If any element is missing, the asset is **not publishable**.

## Linked assets

- `docs/promotion/README.md` — promotion kernel index
- `docs/promotion/t27l-promotion-schema.md` — the audit schema that logs loop events
- `specs/promotion/t27l_promotion.t27` — the .t27 module

// phi^2 + 1/phi^2 = 3 | TRINITY
