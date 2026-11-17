# Contributor path: good-first-issue + CONTRIBUTING

Part of `[EPIC] T27_PROMOTION_KERNEL_V5` — Part of #13473.
M0-08: Contributor path (good-first-issue + CONTRIBUTING). Closes #13481.

## Goal

Document the contributor path: good-first-issue template pointers and CONTRIBUTING entry points that convert course/widget interest into PRs.

## Constraints (M0 law)

- Docs/planning only — no X posts, no outbound email, no paid-token usage without owner approval.
- Free models first; on rate limit mark `WAITING_RATE_LIMIT` and stop (no paid fallback).
- Claims stay inside the approved patterns from the epic.
- Boundary: `docs/promotion/contributor-path.md`, `docs/promotion/good-first-issue-template.md`, `docs/promotion/README.md`.

## How a course/widget reader becomes a contributor

1. **Read a lesson** in the course map (`docs/promotion/course-map.md`) — one idea, one widget, one measurable result.
2. **Run the widget** — every widget is a compiled t27 spec with visual output.
3. **Find a gap** — a lesson whose widget has no spec yet, or a spec whose test does not pass.
4. **File a good-first-issue** using `docs/promotion/good-first-issue-template.md`.
5. **Claim it** on the issue (`Closes #N`), work in a lane worktree, verify locally (`t27b test`, seal, gen).
6. **Land** through our own git: golden head compare-and-swap, one batched GitHub push per window.

## Entry points

- `CONTRIBUTING.md` at repo root — the contributing how-to (read first: SOUL.md, Only t27, Issue gate).
- `docs/promotion/good-first-issue-template.md` — the template to copy when filing a first issue.
- `docs/promotion/course-map.md` — 27 lessons, each with a widget to build or verify.
- `docs/promotion/widget-inventory.md` — the 16 widget types to implement as specs.
- `specs/` — every behavior starts as a `.t27` spec with `test` blocks.

## Publishable-asset rule for contributor-facing assets

Every asset linked from this path (and from the promotion kernel) must contain all six:

1. one measurable claim
2. one artifact link
3. one reproducible path
4. one visual proof
5. one CTA
6. one metric to track

If any element is missing, the asset is **not publishable**.

## Linked assets

- `docs/promotion/README.md` — promotion kernel index
- `docs/promotion/good-first-issue-template.md` — the template
- `CONTRIBUTING.md` — repo-level contributing guide
- `SOUL.md` — canonical constitution

// phi^2 + 1/phi^2 = 3 | TRINITY
