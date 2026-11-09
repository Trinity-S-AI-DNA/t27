# Promotion baseline

Part of #13473. Issue #13474. Status: DRAFT_UNSENT. Nothing in this file was posted, scheduled, or mailed.

Measured at 2026-11-09T18:23:02Z (`date -u +%Y-%m-%dT%H:%M:%SZ`). Machine-readable copy: `docs/promotion/baseline.json`.

## GitHub

Repository `Trinity-S-AI-DNA/t27`. Master commit `116178f1d9ccec481ebc41d82cd7eb8d13d6ff2a`, commit time `2026-11-09 15:09:35 +0000` (`git log -1 --format='%H %ci' origin/master` after `git fetch origin master`).

| Field | Value | Command |
| --- | --- | --- |
| stars | 5 | `gh api repos/Trinity-S-AI-DNA/t27 --jq .stargazers_count` |
| forks | 4 | same call, `.forks_count` |
| open_issues_count | 2109 | same call, `.open_issues_count` (GitHub includes open pull requests) |
| open issues | 2041 | `gh api -X GET search/issues -f q='repo:Trinity-S-AI-DNA/t27 is:issue is:open' -f per_page=1 --jq .total_count` |
| open pull requests | 68 | `gh api -X GET search/issues -f q='repo:Trinity-S-AI-DNA/t27 is:pr is:open' -f per_page=1 --jq .total_count` |
| contributors | 31 | `gh api 'repos/Trinity-S-AI-DNA/t27/contributors?per_page=100&anon=true' --jq length` |
| repo pushed_at | 2026-11-09T18:19:51Z | same repo call, `.pushed_at` |

2041 + 68 = 2109, which matches `open_issues_count`. `pushed_at` and the master commit time are two different API fields; this file does not explain the gap.

## X

Handle named in `specs/automation/blog-share.t27`: `@t27_dev`.

Followers: unavailable. Impressions: unavailable.

`get_users_by_username` for `t27_dev` returned `client-not-enrolled`. No post was sent. No impression count exists in this run.

## Course

| Field | Value | Source |
| --- | --- | --- |
| lesson count in this repo | 27 | `specs/course/lesson_frames.t27` `LESSON_COUNT` |
| lesson count in the live course spec | 27 | `Trinity-S-AI-DNA/trinity` `apps/website/specs/course/course.t27`, blob sha `3b3aa5d21a1dc86876c5b873bf6d6b8cf8d20b21`, size 28305 (`gh api repos/Trinity-S-AI-DNA/trinity/contents/apps/website/specs/course/course.t27 --jq '{sha,size}'`) |
| course title | From zero to a chip: FPGA programming with t27 | `TITLE` in that blob |
| v2 task rows | 12 | `specs/course/fpga_course_v2.t27` `TASK_COUNT` |
| done / parked / claimed / open | 6 / 3 / 1 / 2 | `TASK_STATE` `[2, 2, 3, 1, 2, 2, 2, 2, 3, 3, 0, 0]` with `DONE=2`, `PARKED=3`, `CLAIMED=1`, `OPEN=0` |
| t27b blog URL | HTTP 404 | `curl -sS -o /tmp/blog-body.html -w '%{http_code}' https://t27.ai/blog/t27b-against-other-second-backends/` |

The map of the 27 lessons is `docs/promotion/course-map.md`. Lesson prose from the trinity blob was not copied: `specs/course/fpga_course_v2.t27` weak point W1 says lesson 1 and lesson 3 still name the wrong board.

## Conformance files, not the paper phrase

`ls conformance/vectors/*_conformance_v0.json | wc -l` printed 109. Issue #13473 lists the approved sentence "6 bit-exact conformance packs." That sentence was not remeasured here. The two numbers are different quantities.

## Promotional items

No item in this baseline is publishable. Drafts that quote the approved sentences are in `docs/promotion/content-bank.md`. Each one has claim, artifact, path, visual, CTA, and metric, and `published` is false.

| Field | This baseline |
| --- | --- |
| claim | N/A -- this file records reach, not a paper sentence |
| artifact | `docs/promotion/baseline.json` |
| reproducible path | the commands in the tables above |
| visual | N/A -- no screenshot taken |
| CTA | N/A -- not sent |
| metric | the GitHub counts above; X metrics unavailable |
