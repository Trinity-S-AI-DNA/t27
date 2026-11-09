---
id: git-tri-defaults
name: git-tri-defaults
description: Commit and push inside the agent's own lane by default. Merge, publication, host files, mail, paid calls, and the window shell stay permission-gated. Load before a commit, a push, or a pull request.
argument-hint: "[commit|push|pr|status]"
---

# /git-tri-defaults -- commit, lane, report, pull request

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

The checked law is `specs/t27os/orchestrator.t27` (`oc_commit_ok`, `oc_branch_ok`, `oc_push_ok`, `oc_pr_ok`, `oc_needs_owner`). The queue, the blocker class, and the report call those functions. If this page and the spec disagree, the spec wins.

Agents on the git tri network speak one order: local commit, lane branch, report, pull request. A merge waits for the owner.

## Commit

- A commit inside the agent's own lane is the default. Do not ask permission for a normal commit.
- Commit only the files that belong to the current task.
- Use a clear conventional commit message.
- Never commit secrets, tokens, keys, or generated host binaries.

## Branch

- Work on the assigned lane branch.
- Do not switch to master.
- Do not force-push.
- Do not rebase shared branches.

## Push

- Push the lane branch by default after a successful local check.
- Do not push to master.
- Do not push if the working tree contains unrelated changes.

## Pull request

- Open a pull request only when the task is complete and the checks are green.
- Link the issue in the pull request body.
- Do not merge the pull request.
- Do not bypass required checks.

## Forbidden without owner approval

- merge
- publish to X
- send email
- paid API use
- new host binary
- new shell script
- window shell open
- test-report on this Mac
- a foreign host file

`oc_window_open` stays false. This skill does not open the window.
