# Content bank drafts (M0-05)

Part of `[EPIC] T27_PROMOTION_KERNEL_V5` — Part of #13473.
M0-05: Content bank drafts (approved claims only). Closes #13478.

## Goal

Draft post bank entries using only approved claim patterns. Each draft has claim/artifact/visual/CTA/metric fields. No publishing.

## Draft bank

| # | Claim | Artifact | Visual | CTA | Metric |
|---|-------|----------|--------|-----|--------|
| 1 | "84 formats in one vendor-neutral catalog." | arXiv:2606.09686 | Format catalog chart | View paper | Downloads |
| 2 | "6 bit-exact conformance packs." | conformance/FORMAT-SPEC-001.json | Pack fingerprint display | Download pack | Verifications |
| 3 | "Every pack has a SHA-256 fingerprint." | conformance/vectors/ | Fingerprint table | Verify fingerprint | Matches |
| 4 | "BF16: 21/21 vectors match ml_dtypes." | specs/numeric/gf16.t27 | Vector comparison grid | Run comparison | Match rate |
| 5 | "FP8 E4M3: 15/16 vectors match; the difference is documented." | specs/numeric/paper_claims.t27 | Error visualization | Read abs_error | Documented diffs |
| 6 | "We publish abs_error instead of hiding it." | specs/numeric/paper_claims.t27 | Error curve chart | Inspect vectors | Published errors |
| 7 | "One catalog generates Markdown, JSON, Python, Rust, C, and TypeScript." | specs/numeric/formats_catalog.t27 | Codegen output panel | Browse codegen | Languages |
| 8 | "No Vivado. Open tools only." | FPGA blink demo | Open toolchain screenshot | Build with yosys | Build success |
| 9 | "Every course widget is a compiled t27 spec." | docs/promotion/widget-inventory.md | Widget gallery | Open widget | Widgets compiled |
| 10 | "27 lessons, idea + widget + result." | docs/promotion/course-map.md | Course progress bar | Start lesson 1 | Lessons done |

## Rules

- Every draft uses an approved claim pattern from the epic
- No publishing in this task (M0 law: no X posts, no outbound email)
- Claims must stay inside approved patterns from the epic (arXiv:2606.09686 and arXiv:2606.05017 scope)
- On rate limit: mark `WAITING_RATE_LIMIT`, stop, do not switch to paid model

## Machine-readable bank

See `docs/promotion/content-bank.json` — same 10 drafts in JSON format.

## Linked assets

- `docs/promotion/README.md` — promotion kernel index
- Epic #13473 — T27_PROMOTION_KERNEL_V5
- Approved claim patterns (see README)

// phi^2 + 1/phi^2 = 3 | TRINITY
