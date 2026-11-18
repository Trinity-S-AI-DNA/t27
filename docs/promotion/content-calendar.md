# Content calendar (M0-04)

Part of `[EPIC] T27_PROMOTION_KERNEL_V5` — Part of #13473.
M0-04: Content calendar (10-day cycle + weekly cadence). Closes #13477.

## Goal

Define the 10-day content cycle from the V5 spec plus a weekly cadence calendar.

## 10-day content cycle

The 10-day cycle rotates through promotion themes derived from the approved claim patterns in the epic. Each day has one pillar tag, one claim pattern, and one CTA target.

| Day | Pillar | Claim pattern | CTA target |
|-----|--------|---------------|------------|
| 1 | Numeric catalog | "84 formats in one vendor-neutral catalog." | View arXiv paper |
| 2 | Conformance packs | "6 bit-exact conformance packs." | Download pack |
| 3 | SHA-256 fingerprints | "Every pack has a SHA-256 fingerprint." | Verify fingerprint |
| 4 | GoldenFloat | "BF16: 21/21 vectors match ml_dtypes." | Run comparison |
| 5 | FP8 E4M3 | "FP8 E4M3: 15/16 vectors match; the difference is documented." | Read abs_error |
| 6 | Transparency | "We publish abs_error instead of hiding it." | Inspect vectors |
| 7 | Multi-language | "One catalog generates Markdown, JSON, Python, Rust, C, and TypeScript." | Browse codegen |
| 8 | Open tools | "No Vivado. Open tools only." | Build with yosys |
| 9 | Course widgets | "Every course widget is a compiled t27 spec." | Open widget |
| 10 | Course map | 27 lessons, idea + widget + result | Start lesson 1 |

## Weekly cadence

| Day of week | Pillar | Content type | CTA |
|-------------|--------|--------------|-----|
| Monday | Numeric catalog | Thread: one format family | View arXiv |
| Tuesday | Conformance | Thread: one conformance pack | Download pack |
| Wednesday | GoldenFloat | Thread: one rounding rule | Run comparison |
| Thursday | FP8 | Thread: one FP8 vector | Read abs_error |
| Friday | Transparency | Thread: abs_error methodology | Inspect vectors |
| Saturday | Multi-language | Thread: one codegen output | Browse codegen |
| Sunday | Rest / engagement | Reply to comments, DMs | None (listen only) |

## Rules

- One CTA per post (per epic rule)
- No paid-token usage without owner approval
- No outbound email or X post without owner approval
- Claims must stay inside approved patterns from the epic
- On rate limit: mark `WAITING_RATE_LIMIT`, stop, do not switch to paid model

## Linked assets

- `docs/promotion/README.md` — promotion kernel index
- Epic #13473 — T27_PROMOTION_KERNEL_V5
- Approved claim patterns (see README)

// phi^2 + 1/phi^2 = 3 | TRINITY
