# Funnel

Part of #13473. Issue #13479. Status: DRAFT_UNSENT. One CTA is written per step. None was sent.

The step order is the funnel sentence in #13473: X, then arXiv/GitHub, then the t27.ai course, then a widget, then a good first issue, then a contributor, then a paid product, then B2B.

| step | CTA text (not sent) | artifact | metric |
| --- | --- | --- | --- |
| 1. X | Open the paper from the post. No post exists in this run. | N/A -- X followers unavailable, see baseline.md | not measured |
| 2. arXiv/GitHub | Read https://arxiv.org/abs/2606.09686 and the repo Trinity-S-AI-DNA/t27. | those two URLs | GitHub stars 5, forks 4, see baseline.md |
| 3. t27.ai course | Open the course spec recorded in course-map.md. | trinity course blob 3b3aa5d21a1dc86876c5b873bf6d6b8cf8d20b21 | lesson count 27 |
| 4. widget | Use one lesson widget id from course-map.md. | course-map.md | gallery membership not measured |
| 5. good first issue | Use the proposed template in good-first-issue-template.md. This run filed no issue. | docs/promotion/good-first-issue-template.md | open issues labelled good-first-issue: 0 |
| 6. contributor | Read CONTRIBUTING.md, then open a pull request that names an issue. | CONTRIBUTING.md | contributors 31, see baseline.md |
| 7. paid product | N/A -- #13473 names the step and names no product and no price | N/A | not measured |
| 8. B2B | N/A -- #13473 names the step and names no offer | N/A | not measured |

Paid and B2B stay empty on purpose. Inventing a price or a product name is not this task.

## Asset fields for the funnel page itself

| Field | Value |
| --- | --- |
| claim | N/A |
| artifact | this file |
| reproducible path | baseline commands plus `gh api -X GET search/issues -f q='repo:Trinity-S-AI-DNA/t27 is:issue is:open label:good-first-issue' -f per_page=1 --jq .total_count` which printed 0 |
| visual | N/A |
| CTA | N/A -- the step texts above were not sent |
| metric | the counts named in the table |
