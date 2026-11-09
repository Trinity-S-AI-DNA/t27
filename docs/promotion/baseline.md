# Promotion baseline

Part of #13473. This card does not publish and does not send mail.

The executable check is `specs/automation/promotion_kernel.t27`.
The same figures are in `docs/promotion/baseline.json`.

Captured 2026-11-10 (UTC).

## GitHub

Commands run that day:

- `gh api repos/Trinity-S-AI-DNA/t27` fields `stargazers_count` and `forks_count`
- `gh api search/issues?q=repo:Trinity-S-AI-DNA/t27+is:issue+is:open` field `total_count` (issues only; the repo `open_issues_count` also counts pull requests and was not used)
- `gh api repos/Trinity-S-AI-DNA/t27/contributors?per_page=100` length, and the same call with `anon=1`

| Field | Value |
|-------|-------|
| stars | 5 |
| forks | 4 |
| open issues | 2041 |
| named contributors | 6 |
| contributors including anonymous | 31 |

## X

Not re-measured. This agent has no X sign-in. The figures below are the citation in issue #13465 for @t27_dev, 7-day view, 2026-11-09.

| Field | Value | Known as a tracked metric |
|-------|-------|---------------------------|
| impressions | 809 | yes |
| profile visits | 2 | no, citation only |
| followers | 231 | no, citation only |
| clicks | not measured | no |
| course starts | not measured | no |
| payments | not measured | no |

Clicks are not stored as 0. `metric_known` is true only for impressions. Paid checkout is off.

## Course

`specs/course/lesson_frames.t27` `LESSON_COUNT` is 27. That is the planned frame count. A ready-lesson count is not measured (`COURSE_READY_KNOWN` is false).
