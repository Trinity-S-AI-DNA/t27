# Metrics

Part of #13473. This card does not publish.

The executable check is `specs/automation/promotion_kernel.t27`
(`metric_known`, `WEEKLY_FIELD`).

The weekly report template is `docs/promotion/weekly-report-template.md`.
Its fields are the sixteen names of the epic final-report template, in
the same order as `WEEKLY_FIELD`.

## What is tracked

| Metric | State |
|--------|-------|
| impressions, 7-day | known, 809, cited from #13465 on 2026-11-09 |
| clicks | not measured |
| course starts | not measured |
| payments | not measured |
| paid checkout | off |

`metric_known` is true only for impressions. An unknown metric is left
blank in the weekly report. It is not written as 0.
