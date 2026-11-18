# T27L promotion schema

Part of #13473. Issue #13483. Status: DRAFT_UNSENT. The spec does not emit a live event and does not post.

## Surface

Name: `promotion_kernel`.

Constant: `SURFACE` in `specs/promotion/t27l_promotion.t27`.

`REGISTERED_IN_T27L` is false. `specs/t27os/t27l.t27` on master `116178f1d` sets `TL_SURFACE_LAST` to 19. `tl_surface_name` returns `course_hud`, `dev_view`, `pane`, `viewer`, `neko`, `menu_l`, `browser`, `other`, `cost`, `system`, `agent`, `security`, `research`, `memory`, `time`, `ci`, `migration`, `visual`, `roadmap`. None of those strings is `promotion_kernel`. This pack does not edit `t27l.t27`.

## Actions

| constant | text |
| --- | --- |
| ACTION_DRAFT | draft |
| ACTION_GATE_CHECK | gate_check |
| ACTION_PUBLISH_APPROVED | publish_approved |
| ACTION_WAITING_RATE_LIMIT | WAITING_RATE_LIMIT |
| ACTION_REFUSED | refused |

`on_rate_limit(false)` returns `WAITING_RATE_LIMIT`. `on_rate_limit(true)` returns `refused`. A paid model is not a fallback.

## Gates

`asset_ready` is true only when claim, artifact, path, visual, CTA, and metric are all present.

`proof_ready` is true only when widget, screenshot, test pass, and human yes are all present (#13485).

`may_publish` is true only when both gates are true.

## Check

`./target/debug/t27c test-report specs/promotion/t27l_promotion.t27` on this run: tests 6, pass 6, FAIL 0, vacuous passes 0 of 6.

Seal: `.trinity/seals/promotion_promotion_t27l.json`, saved with `./target/debug/t27c seal specs/promotion/t27l_promotion.t27 --save`, then `--verify` printed `all hashes MATCH`.

## Asset fields

| Field | Value |
| --- | --- |
| claim | N/A |
| artifact | `specs/promotion/t27l_promotion.t27` |
| reproducible path | the test-report command above |
| visual | N/A |
| CTA | N/A -- not sent |
| metric | 6 tests, 0 failures, 0 vacuous |
