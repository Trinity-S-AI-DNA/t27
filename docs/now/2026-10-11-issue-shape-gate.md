# NOW -- the issue-shape gate is wired: only the template's shape may stay open (2026-10-11)

## .github/workflows/needs-boundary.yml (Closes #7498)

- The owner asked (translated) for a hard block on creating issues any way but by the template. The rule had been in `specs/policy/issue_shape.t27` since #7505, but its workflow never landed, and templates bind only the web form: `gh issue create` and the REST API never see them.
- The daily labeller in `needs-boundary.yml` is now the gate: it runs on `issues: opened, edited, reopened`. Its plumbing is a one-line C main around the generated `check_issue()` (gen/c/policy/issue_shape.c).
  - A failing issue is closed as not planned and labelled `shape-refused`, with the missing sections as a comment.
  - An edit that passes reopens it; issues labelled epic or roadmap are skipped.
- `tools/queen/needs_boundary.py` (255 lines) is deleted: no new issue can lack a Boundary, so the backlog labeller has nothing left to find. Hand-written foreign lines in this PR: net -270.
- Measured on the Railway lab with the generated checker:
  - 931 of 980 open issues pass; 20 are epics or roadmap.
  - The 49 that fail are mostly design and epic issues without the epic or roadmap label.
  - Five plumbing cases were run against a stub `gh`: pass, fail and open, pass after a refusal, fail while closed, and a reopen of a refused issue. Each made exactly the expected calls.
- Not covered: issues opened by a workflow's GITHUB_TOKEN start no workflow, so the gate cannot see them. The Queen feeds are the only such openers, and they write the template.
