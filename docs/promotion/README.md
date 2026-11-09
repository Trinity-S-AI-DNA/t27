# Promotion kernel, M0

Part of #13473. Planning and drafts only. Nothing in this directory was posted, scheduled, or mailed. The publish gate is `publish-gate.md`.

`docs/README.md` links this index because a new `docs/` directory has to be on that map.

## Deliverables

| Issue | File |
| --- | --- |
| #13474 | `baseline.md`, `baseline.json` |
| #13475 | `course-map.md` |
| #13476 | `widget-inventory.md`, `widget-inventory.json` |
| #13477 | `content-calendar.md` |
| #13478 | `content-bank.md`, `content-bank.json` |
| #13479 | `funnel.md` |
| #13480 | `metrics.md`, `weekly-report-template.md` |
| #13481 | `contributor-path.md`, `good-first-issue-template.md` |
| #13482 | `monetization-ladder.md` |
| #13483 | `t27l-promotion-schema.md`, `specs/promotion/t27l_promotion.t27` |
| #13484 | `self-improving-loop.md` |
| #13485 | `publish-gate.md` |

These issues stay open. Each child issue says a landing that finishes it changes only that issue's Boundary paths. This pack also edits `docs/README.md` and adds one spec plus its seal, so it does not claim to finish them.

## Other open social and media tasks

Measured with `gh search issues --repo Trinity-S-AI-DNA/t27 --state open` and `gh issue view`. Home skill files were checked with `test -e` on this VM: `/home/ubuntu/.claude` and `/home/ubuntu/.codex` are absent, so the Mac skill files named in the cloud brief are not here.

| Issue | State | Where it runs now | This run |
| --- | --- | --- | --- |
| #12630 tri blog | OPEN | `specs/tri/blog/blog.t27` absent (`test -f` exit 1). Built `tri` is not given a blog subcommand by this pack. | not started. Rust glue needs the owner-approved-foreign label the issue names. |
| #12866 t27b post on t27.ai | OPEN | `specs/blog/t27b_progress.t27` exists. `curl` of `https://t27.ai/blog/t27b-against-other-second-backends/` returned HTTP 404. | not published. |
| #12865 append 2026-10-28 lab entry | OPEN | The spec's newest entry is still 2026-10-05 (`DATE` `[20261004, 20261005]`). `curl` of `https://t27b-lab-production.up.railway.app/latest.json` returned HTTP 404. | not appended. The issue requires the lab summary object. |
| #12869 t27b blog steward cron | OPEN | No new cron card in this pack. | not added and not run. |
| #12631 epic progress card batch | OPEN | `specs/blog/epic_progress.t27` is the merge-train post, three entries, not the 37-card batch the issue describes. | not rewritten. Replacing that file would drop the existing post. |
| #13248 course bench-board post | OPEN | `specs/blog/course_bench_board.t27` absent. Trinity issue #1806 is OPEN (`gh issue view 1806 --repo Trinity-S-AI-DNA/trinity`). | not started. |
| #13273 queen stack progress card | OPEN | `specs/blog/queen_stack_progress.t27` absent. | not started. The issue also edits `AGENTS.md`, `access-map`, and `docs/t27os/keys-orchestration.md`. |
| #5629 blog to social networks | OPEN | Law in `specs/automation/blog-share.t27` version 5. Host named there is 999-multibots-telegraf, not this VM. | not posted. |
| #5626 first post through network APIs | OPEN | GitHub issue. | not posted. |
| #5625 connect Threads, Facebook, Instagram, LinkedIn, X | OPEN | GitHub issue. | accounts not connected. |
| #8318 blog-share v6 autopost | OPEN | Would post every new article to X, Telegram, and other networks. | not implemented. |
| #11620 Instagram reels via Graph API | OPEN | GitHub issue. | not published. |
| #10915 content factory epic | OPEN | `specs/t27f/post_caption.t27` present. Skill card `specs/skills/t27-social-pack.t27` present. Cover PR #10830 is MERGED (`gh issue view 10830`). | no new post. |
| #10560 lesson reels | OPEN | Epic. Publishing is out of its boundary until the owner says yes. | no render and no post. |
| #9063 course factory | OPEN | Epic. Course rows are copied into `course-map.md` only. | no new course page. |

`gh search issues` for `habr` in this repo, state open, limit 15, returned no rows. `content plan` returned no rows. `back-catalogue` and `back catalogue` returned no social-task title; the hits were unrelated. `notebooklm` returned port and CI titles, and the media back-catalogue skill file is absent, so that catalogue was not run. `terminal cast` returned #13267 and #11780; those titles are a queen ring and widget cards, and the terminal-cast skill file is absent, so no cast asset was made.

X handle `@t27_dev` is named in `specs/automation/blog-share.t27`. Followers and impressions are unavailable (`client-not-enrolled`). See `baseline.md`.
