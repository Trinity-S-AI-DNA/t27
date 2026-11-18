# T27 Promotion Kernel — M0 (docs/planning only)

This directory is the M0 foundation of **[EPIC] T27_PROMOTION_KERNEL_V5 (#13473)** and its sibling contract
`T27_PROMOTION_KERNEL_V5 (#13465)`. It holds only planning, docs and schema artifacts under
`docs/promotion/` and `specs/promotion/` — no X posts, no outbound email, no paid-token usage.

## Promotion kernel mission

Turn T27 research artifacts and the 27-lesson course widget engine into a proof-to-demand promotion engine:

> arXiv → measurable claims → conformance packs → open artifacts → course widgets → X demonstrations → GitHub issues → contributors → paid products → B2B

Sell **verifiable engineering**: build, verify, and run a ternary machine — from open tools to FPGA. Do **not** sell "AGI access".

Every publishable asset must contain all six:

1. one measurable claim
2. one artifact link
3. one reproducible path
4. one visual proof
5. one CTA
6. one metric to track

If any element is missing, the asset is **not publishable**.

## Approved claim patterns (stay inside these)

- "84 formats in one vendor-neutral catalog."
- "6 bit-exact conformance packs."
- "Every pack has a SHA-256 fingerprint."
- "BF16: 21/21 vectors match ml_dtypes."
- "FP8 E4M3: 15/16 vectors match; the difference is documented."
- "We publish abs_error instead of hiding it."
- "One catalog generates Markdown, JSON, Python, Rust, C, and TypeScript."
- "No Vivado. Open tools only."
- "Every course widget is a compiled t27 spec."

## M0 child deliverables

| # | Issue | Deliverable | Status |
|---|-------|-------------|--------|
| M0-01 | #13474 | `docs/promotion/baseline.md` + `baseline.json` | [done](#promotion-baseline) |
| M0-02 | #13475 | `docs/promotion/course-map.md` | [done](#course-map) |
| M0-03 | #13476 | `docs/promotion/widget-inventory.md` + `widget-inventory.json` | [done](#widget-inventory) |
| M0-04 | #13477 | `docs/promotion/content-calendar.md` | [done](#content-calendar) |
| M0-05 | #13478 | `docs/promotion/content-bank.md` + `content-bank.json` | [done](#content-bank) |
| M0-06 | #13479 | `docs/promotion/funnel.md` | [done](#funnel) |
| M0-07 | #13480 | `docs/promotion/metrics.md` + `weekly-report-template.md` | [done](#metrics) |
| M0-08 | #13481 | `docs/promotion/contributor-path.md` + `good-first-issue-template.md` | [done](#contributor-path) |
| M0-09 | #13482 | `docs/promotion/monetization-ladder.md` | [done](#monetization-ladder) |
| M0-10 | #13483 | `docs/promotion/t27l-promotion-schema.md` + `specs/promotion/t27l_promotion.t27` | [done](#t27l-audit-schema) |
| M0-11 | #13484 | `docs/promotion/self-improving-loop.md` | [done](#self-improving-loop) |
| M0-12 | #13485 | `docs/promotion/publish-gate.md` | [done](#publish-gate) |

The two contract specs live under `specs/`:

- `specs/automation/promotion_kernel.t27` — proof-to-demand gate (#13465)
- `specs/automation/grok_crm.t27` — `gc_event` T27L audit projection (#13462)
- `specs/promotion/t27l_promotion.t27` — T27L event schema for promotion_kernel (#13483)

## Constraints (M0 law, from the epic)

- `BOUNDED_CYCLES_ONLY`, `BUDGET_FIRST`, `FREE_MODELS_FIRST`
- No paid-token usage without explicit owner approval
- No outbound email, X post, merge, or cron without owner approval
- Zoho Mail / CRM / Browser OS are **later** milestones, not M0
- On rate limit: mark `WAITING_RATE_LIMIT`, save artifacts, do **not** switch to a paid model
- When code lands, only Boundary paths change

## Funnel (one CTA per step)

X → arXiv/GitHub → t27.ai course → widget → good first issue → contributor → paid product → B2B

---

<details>
<summary>Index anchors</summary>

- [Promotion baseline](#promotion-baseline): `docs/promotion/baseline.md`
- [Content calendar](#content-calendar): `docs/promotion/content-calendar.md`
- [Content bank](#content-bank): `docs/promotion/content-bank.md`
- [Course map](#course-map): `docs/promotion/course-map.md`
- [Funnel](#funnel): `docs/promotion/funnel.md`
- [Metrics tracking](#metrics): `docs/promotion/metrics.md`
- [Weekly report template](#weekly-report-template): `docs/promotion/weekly-report-template.md`
- [Publish gate](#publish-gate): `docs/promotion/publish-gate.md`
- [Widget inventory](#widget-inventory): `docs/promotion/widget-inventory.md`
- [Contributor path](#contributor-path): `docs/promotion/contributor-path.md`
- [Monetization ladder](#monetization-ladder): `docs/promotion/monetization-ladder.md`
- [T27L audit schema](#t27l-audit-schema): `docs/promotion/t27l-promotion-schema.md`
- [Self-improving loop](#self-improving-loop): `docs/promotion/self-improving-loop.md`

</details>

// phi^2 + 1/phi^2 = 3 | TRINITY
