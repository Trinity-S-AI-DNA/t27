# Content calendar

Part of #13473. This card does not publish.

The executable check is `specs/automation/promotion_kernel.t27`
(`day_claim`, `day_pillar`, `day_publishable`, `weekly_posts`).

## 10-day cycle

Each day names a claim id and a pillar tag. `publishable_days` is 0.
A visual does not make a day publishable.

| Day | Claim | Pillar | Publishable |
|-----|-------|--------|-------------|
| 0 | 84 formats (refused) | proof | no |
| 1 | BF16 21/21 (refused) | proof | no |
| 2 | FP8 15/16 (refused) | artifact | no |
| 3 | paper row 11, printed 12 clusters (refused) | artifact | no |
| 4 | paper row 10, 109 records (fits, still a draft) | course | no |
| 5 | measured 13 clusters (fits, still a draft) | course | no |
| 6 | 84 formats (refused) | github | no |
| 7 | 84 formats (refused) | github | no |
| 8 | 84 formats (refused) | cta | no |
| 9 | 84 formats (refused) | cta | no |

Pillar tags: proof, artifact, course, github, cta. Two days each.

Days 0, 1, 2, 6, 7, 8 and 9 stay on refused claims so the cycle does not
invent a new slogan. Days 4 and 5 name claims the gate marks `fit`, and
they stay drafts.

## Weekly cadence

One review of the ten drafts. `weekly_posts` is 0. Nothing is scheduled
and nothing is posted.
