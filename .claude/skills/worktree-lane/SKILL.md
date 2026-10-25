---
id: worktree-lane
name: Worktree lane
description: Run one task as a lane -- a claimed issue, a worktree under ~/t27/.claude/worktrees, a model routed cheapest-first (free T27 OS pane, then Sonnet, then Opus), local verification, landing through trigit (golden head, one batch PR per window), then cleanup. Use when the owner says "launch this task through a worktree", "run it as a lane", "start the task with trigit", "worktree + trigit", or before handing a bounded task to a pane, a subagent or a side session.
argument-hint: "[issue number | new \"title\"]"
---

# Worktree lane (owner request, 2026-10-25)

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

One task, one issue, one worktree, one branch, one landing. This skill is the order of the steps; each rule
behind a step lives in one place and is pointed at, not copied: landing is `own-git-landing`
(`specs/queen/golden_chain.t27` section 6), the verbs are `specs/tri/trigit/verbs.t27`, the model order is
`specs/claw/model_route.t27` (skills `model-routing`, `cheap-model-routing`), a lane as a pane is
`t27os-orchestra`, removal is `self-cleanup` (`specs/tri/worktrees/reap.t27`).

## The steps

1. **Room first.** `df -g .` shows more than 5.4 GiB free (the app refuses a worktree below that) and `uptime`
   shows a load average under 40. Heavy builds, sweeps and benchmarks never run on the Mac: they go to the lab
   or a T27 OS pane (owner, 2026-10-09).
2. **Issue.** `gh issue list --search "<words>"` first; reuse an open issue. A new one has the four headings
   Boundary / User Scenarios / Requirements / Success Criteria (skill `issue-shape`), English, ASCII, filed with
   `--body-file`. Two issues whose Boundary names the same file run one after the other: give each its own
   files or a `path :: fn NAME` region.
3. **Claim.** `tri trigit claim <issue> --at <head-sha>` makes `refs/swarm/claims/<issue>` with an old value of
   zero, so of two lanes git accepts exactly one. A refusal means someone else holds it: take another issue.
   Look before acting: `tri trigit status` (claims, unpushed links).
4. **Worktree.** One home: `~/t27/.claude/worktrees/<slug>`, made by the app (EnterWorktree) or
   `git worktree add -b claude/evolve-<slug> <path> origin/master`. The branch name `claude/evolve-*` is what
   `tri golden land` reads. If the sandbox refuses the add, run it through a Terminal tab. No clone and no
   scratch tree in `~`, `/tmp` or `/private/tmp`.
5. **Bind.** In the worktree: `tri skill begin --issue <N> --desc "<one line>"`. Without the binding every
   commit is denied. If it answers "active skill already in progress: <other>", another lane holds the
   binding: do not run `tri skill end` on it (that breaks the other lane); commit through the plain path in
   step 10 or park the work.
6. **Brief.** The issue is the brief. Add to it, for any model that is not you: the four hard rules (only t27;
   no heavy runs on the Mac; English and ASCII on GitHub; never print or commit a secret), the files it may
   touch, the command that proves it done, and "do not commit, the orchestrator commits". A cheap model forgets
   a rule it was not told.
7. **Route the model, cheapest first.**
   - Attempts 1-2: a free T27 OS pane on z.ai (`glm-4.5-flash`), through the key master so a throttled key
     rotates by itself:
     `T27OS_CTL=spawn T27OS_AGENT=0 T27OS_CWD=<worktree> T27OS_MODEL=glm-4.5-flash ~/t27-build/target/t27os/t27os-pty`
     prints `spawned pane N`; send the brief with `T27OS_CTL=keys T27OS_PANE=N T27OS_KEYS='...'`. A free model
     is used only behind a gate that a machine judges (a compiler, a test), never on its own word.
   - After 2 failed attempts: a Sonnet 5.5 subagent (`model: "sonnet"`), which writes and returns a text report.
   - After 4, or on the owner's word: Opus 5.5.
   - The orchestrating session plans and reviews on Sonnet 5.5 and keeps the seals, the gates and the commits.
8. **Checkpoint while it runs.** `tri cell checkpoint --step "<what just worked>"` after each green step;
   `tri trigit beat` keeps the lane's heartbeat as a ref (`refs/swarm/alive/<host>`).
9. **Verify locally, in this order** (practice 1 of `own-git-landing`): `t27b test <spec>` on the changed spec
   and on the specs that import it, `t27c test-report <spec>` with 0 failed and 0 vacuous, `t27c seal <spec>
   --save` from a master-built binary, `tri gen` for generated files (never edit `gen/`). Specs with
   `@extern` cannot go through `t27c`; use `t27b test` and land them as a plain PR.
10. **Commit.** `tri cell seal && tri git commit --all -m "<what> (Closes #<N>)"` (trigit adds the `issue:N`
    prefix). If the installed `tri` has no `git` verb, use `git -c core.hooksPath=.githooks commit`. Every
    commit of the branch carries `Closes #N` or `Refs #N` (L1 greps each commit; "see #N" and "Part of #N"
    fail). An epic is only ever `Refs`.
11. **Land.** `tri golden land --plan` to see the link, then `tri golden land` (compare-and-swap on
    `refs/golden/head`), then `tri golden push --issue <N>` when the window is due: one batch PR per 15
    minutes, never a push per commit. A branch the golden chain cannot take lands with `tri land` (merge master,
    tests, seals, PR, auto-merge) once, after every gate has run locally.
12. **Release and clean up.** After the merge: `tri worktrees --reap-plan`, then skill `self-cleanup` removes
    the worktree only if it is clean, pushed (or parked as a local `rescue/<name>` ref, never pushed) and no
    process uses it. Kill the pane (`T27OS_CTL=kill T27OS_PANE=N`). End with a one-line `Requests out:`.

## Traps

- The installed `tri` can be older than the specs: `tri trigit` exists, but `tri golden land|push` and
  `tri git` may be missing or answer something else. Check `tri golden status` before relying on it; a fresh
  binary comes from the lab, not from a build on the Mac.
- A pane that prints `API Error: 401` or `429` is handled by the key master's failover (key, then model, then
  profile); do not edit keys by hand and never print one. If every key rests, the pane waits.
- `--auto` merge races a second push: run every gate before the first push.
- Absolute `core.hooksPath` runs the main checkout's hooks inside a worktree; use the `-c` form above.
- zsh: write `${b}` before a colon in a refspec; never put backticks inside a double-quoted `--body`.
- A stacked PR does not retarget when its base merges (`delete_branch_on_merge=false`): check the merge-base
  on master.
- A lane built on a golden head that has since moved is `UNKNOWN_PARENT`: `git merge refs/golden/head` first.
