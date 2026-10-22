---
id: issue-shape
name: Issue Shape
description: How to file a gHashTag/t27 issue that the issue-shape gate keeps open -- the four sections it requires (Boundary, User Scenarios, Requirements, Success Criteria), the Boundary path trap, checking a body with the generated check_issue() on the Railway lab before `gh issue create`, and how to recover when the gate has already closed an issue as `shape-refused`. Use before filing or editing any gHashTag/t27 issue, when an issue was closed seconds after filing, or when the user asks why an issue was closed "as not planned".
---

# Issue Shape

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

## Source of truth

**`specs/policy/issue_shape.t27` is the rule** (owner rule 2026-10-07, #7498). `t27c gen-c`
lowers it to `gen/c/policy/issue_shape.c`, and `.github/workflows/needs-boundary.yml` feeds every
new or edited issue body into its `check_issue()`. While any check fails, the workflow closes the
issue as not planned, labels it `shape-refused` and comments the failing lines. This skill is a
reading guide to that spec. **Where the two disagree, the spec wins**: fix this file, not the
issue. The spec's header still names the workflow `issue-shape.yml`; the file in the tree is
`needs-boundary.yml`.

Issues labelled `epic` or `roadmap` are containers and are skipped.

## What the gate wants

Read the spec's header comment for the exact rule. In practice, a body that passes has, in this
order, with these English spellings (the gate accepts no other language):

1. Free prose first: summary, repro, tables, anything that names files in passing.
2. `## Boundary` -- one path per line, `- specs/x.t27`. A line counts when one of its tokens holds
   a `/` or ends in a dot plus 1 to 10 word characters. To name one region of a file, write
   `- path :: fn name`. A heading with no path line under it counts as missing.
3. `## User Scenarios` -- at least one `**Given** ... **when** ... **then** ...` line.
4. `## Requirements` -- numbered obligations, `- FR-001: ... MUST ...`.
5. `## Success Criteria` (or `## Acceptance Criteria`, `## Acceptance`, `## Done when`) with at
   least one outcome a machine can settle: a number with a unit (`0 files`, `200 ms`), a
   backticked file with a listed extension (`` `x.rs` ``, `` `x.md` ``), `exit 0`, or a backticked
   `cargo` / `git` / `make` command. Note that `.t27` is not in the extension list, so a
   backticked `.t27` path alone does not settle it.

## The Boundary trap

The Boundary section runs from `## Boundary` to the next line that starts with `## `. **Every
path token inside it is a file the Queen reserves for this issue**, prose or not. A sentence
under the list such as "`bootstrap/src/compiler.rs` is existing Rust" reserves that file again,
and a remark that mentions some other path ("unlike `specs/foo.t27`") reserves a file the task
never touches. Put such prose above `## Boundary`, in the summary. Keep the section to path
lines.

## Check the body before filing

Never file blind. Write the body to a file with the editor tool (a heredoc that contains a
compiler command trips the Mac load hook), then judge it with the same generated C the workflow
uses -- on the Railway lab, never on the Mac (AGENTS.md and the lab rule: builds run on the lab):

```bash
~/t27-lanes/lab-up.sh ./body.md /root/body.md
```

On the lab, in a t27 checkout at `origin/master`, the harness is the one in
`.github/workflows/needs-boundary.yml`: include `gen/c/policy/issue_shape.c`, read stdin, call
`check_issue(buf, n, out, out_cap)`, print `out`, and exit non-zero when the returned count of
missing checks is above 0. Copy the harness from the workflow at the time you run it rather than
from here, so the two cannot drift. Exit 0 and an empty report mean the gate will keep the issue
open. Run it once on a body you know is wrong (drop `## Boundary`) as a negative control, so a
harness that always prints nothing cannot fool you.

## When the gate already closed it

An issue closed with `shape-refused` is not lost and does not need a new number. Fix the body,
check it as above, then:

```bash
gh issue edit <N> -R gHashTag/t27 --body-file body.md
```

The edit re-runs the workflow (it listens on `opened`, `edited` and `reopened`); a passing body
removes `shape-refused` and reopens the issue. Do not reopen it by hand while the body still
fails: the `reopened` event runs the check and closes it again. Do not file a duplicate.

## Checklist before `gh issue create`

- English, ASCII headings exactly as above.
- Prose with file paths sits above `## Boundary`.
- Every file the task may touch is a path line under `## Boundary`, and nothing else is.
- Scenarios use Given/when/then; requirements use FR-NNN and MUST.
- The criteria section has a machine-settled outcome.
- `check_issue()` on the lab exits 0, and the negative control fails.
