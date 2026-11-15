# Contributor path

Part of #13473. Issue #13481. Status: DRAFT_UNSENT. This run filed no issue and sent no mail.

## Existing entry

`CONTRIBUTING.md` is the entry point already in the repo. It tells a contributor to read `SOUL.md`, `OWNERS.md`, and to reference a GitHub issue. The issue-shape sections a new issue needs are Boundary, User Scenarios, Requirements, and Success Criteria (`specs/policy/issue_shape.t27`, named from `AGENTS.md`).

## Good-first-issue label

Open issues with that label: 0.

Command: `gh api -X GET search/issues -f q='repo:Trinity-S-AI-DNA/t27 is:issue is:open label:good-first-issue' -f per_page=1 --jq .total_count`

No template file for that label existed under `docs/promotion/` before this pack. The proposed shape, not a filed issue, is `docs/promotion/good-first-issue-template.md`.

## Path

1. Read `CONTRIBUTING.md`.
2. Pick a lesson row in `docs/promotion/course-map.md` whose `frame_next` cell is not blank (eight rows; `specs/course/lesson_frames.t27` test `the_edit_changes_eight_frames_and_no_exempt_one`).
3. File an issue with the proposed template only after the issue-shape check the skill describes. This pack does not file it.
4. Open a pull request that references that issue.

## Asset fields

| Field | Value |
| --- | --- |
| claim | N/A |
| artifact | `CONTRIBUTING.md` and `docs/promotion/good-first-issue-template.md` |
| reproducible path | the label search above |
| visual | N/A |
| CTA | N/A -- not sent |
| metric | 0 open good-first-issue issues |
