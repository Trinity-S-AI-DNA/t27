# Monetization ladder: Free / Low / Mid / High / B2B

Part of `[EPIC] T27_PROMOTION_KERNEL_V5` — Part of #13473.
M0-09: Monetization ladder (Free/Low/Mid/High/B2B). Closes #13482.

## Goal

Write the monetization ladder doc: Free / Low / Mid / High / B2B, without selling AGI access — only verifiable engineering offers.

## Constraints (M0 law)

- Docs/planning only — no X posts, no outbound email, no paid-token usage without owner approval.
- Free models first; on rate limit mark `WAITING_RATE_LIMIT` and stop (no paid fallback).
- Claims stay inside the approved patterns from the epic.
- Boundary: `docs/promotion/monetization-ladder.md` and `docs/promotion/README.md`.

## What we sell: verifiable engineering, not AGI access

Sell the ability to **build, verify, and run a ternary machine — from open tools to FPGA**. Every tier names one concrete, reproducible offer. No tier sells "AGI access", vague benchmarks, or hidden failures.

## The ladder — one offer per tier

| Tier | Offer | What the customer gets | Measurable result |
|------|-------|------------------------|-------------------|
| Free | Open course + widgets | 27 lessons, 16 widget types, all compiled t27 specs, open tools | learner builds and runs a widget, result measured |
| Low | Conformance pack + report | 6 bit-exact conformance packs with SHA-256 fingerprints, abs_error published | 6 packs verified, fingerprints match |
| Mid | FPGA demonstration | ternary blink and training loop on ALINX AX7203, open tools, no Vivado | board blinks / trains, part + tool versions named |
| High | Private course instance | dedicated 27-lesson course with learner progress and widget engine | 27 lessons run, progress tracked per learner |
| B2B | Ternary compute license | verifiable ternary machine IP, generation from .t27 to Verilog, settlement receipts | generated RTL, one receipt per completed job |

## Funnel position (one CTA per step)

Free → Low → Mid → High → B2B maps to the promotion kernel funnel:

X → arXiv/GitHub → t27.ai course → widget → good first issue → contributor → paid product → B2B

## Publishable-asset rule

Every offer above is a publishable asset only if it contains all six:

1. one measurable claim
2. one artifact link
3. one reproducible path
4. one visual proof
5. one CTA
6. one metric to track

If any element is missing, the offer is **not publishable**.

## Approved claim patterns (stay inside these)

- "6 bit-exact conformance packs."
- "Every pack has a SHA-256 fingerprint."
- "We publish abs_error instead of hiding it."
- "No Vivado. Open tools only."
- "Every course widget is a compiled t27 spec."

## Linked assets

- `docs/promotion/README.md` — promotion kernel index
- `docs/promotion/course-map.md` — 27 lessons
- `docs/promotion/widget-inventory.md` — 16 widget types
- `docs/promotion/contributor-path.md` — how contributors enter

// phi^2 + 1/phi^2 = 3 | TRINITY
