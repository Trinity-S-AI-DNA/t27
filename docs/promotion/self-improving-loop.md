# Self-improving loop

Part of #13473. Issue #13484. Status: specified, not started as a job. This file does not add a workflow, does not run cron, and does not call a paid model.

## Bound

One pass, then stop.

1. Read `docs/promotion/baseline.md` and `docs/promotion/weekly-report-template.md`.
2. If a metric has no command, write `not measured`. Do not write 0 in its place.
3. If the model or the API returns a rate limit, record `WAITING_RATE_LIMIT` (`on_rate_limit(false)` in `specs/promotion/t27l_promotion.t27`) and stop.
4. `on_rate_limit(true)` is `refused`. There is no paid fallback (#13473, `FREE_MODELS_FIRST`).
5. The only edit a pass may make before a named owner approval is under `docs/promotion/` and `specs/promotion/`. It does not post.

No cron card was added. Issue #12869 asks for a blog-steward workflow. That workflow is not this loop, and it was not added.

## Asset fields

| Field | Value |
| --- | --- |
| claim | N/A |
| artifact | this file and `specs/promotion/t27l_promotion.t27` |
| reproducible path | `t27c test-report` recorded in `t27l-promotion-schema.md` |
| visual | N/A |
| CTA | N/A -- not sent |
| metric | not measured -- the loop has not run |
