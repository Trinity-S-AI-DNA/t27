---
id: self-cleanup
name: self-cleanup
description: Clean up after your own work, every time, without being asked -- remove the lane worktrees whose PRs merged, the review checkouts and the per-lane Zig caches this session made, and never anything another session made. Load at the end of every loop tick, right after any PR of yours merges, and before starting a lane when free disk is under 8 GB.
argument-hint: "[--dry-run]"
---

# /self-cleanup -- a session removes what it made, and only that

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

Owner directive 2026-10-19: clean up yourself, always, and make it a skill. The night of 2026-10-18 filled the bench Mac's disk twice (228 GB at 95-99 %; each finished lane worktree is 300-650 MB, plus a Zig cache per lane) and the owner had to remove the trees by hand twice. Cleanup is part of the work, like the ledger row: a tick that merged something and left its trees behind is not finished.

The judgement is meant to move into `.t27` (#10805: a REMOVABLE verdict in `tri worktrees`). Until then this skill is the rule, applied by hand.

## What is yours

Only what this session created, known from its own record -- never from a path pattern, a branch prefix, or the mine/other column of `tri worktrees` (that column marks other sessions' trees "mine" too; on 2026-10-19 it listed another loop's `wt-b1`).

| Thing | How you know it is yours | Where |
|---|---|---|
| Lane worktree | the Agent result named its `worktreePath` and branch; the ledger row names the lane | `.claude/worktrees/agent-<id>` |
| Review checkout | you made it (`git worktree add ... $TMPDIR/rev-<lane>`) | `$TMPDIR/rev-*` and its `.git/worktrees/rev-*` entry |
| Per-lane Zig cache | the lane brief named it (`ZIG_GLOBAL_CACHE_DIR=$TMPDIR/zig-<lane>`) | `$TMPDIR/zig-<lane>`, `$TMPDIR/zig-<lane>-local` |
| Scratch files | written in this session's scratchpad | the scratchpad directory |

Write each one into the ledger row (or a list in the scratchpad) when you create it, so the next tick -- or the next session after a restart -- can find it.

## When

1. Right after a PR of yours merges or is closed: its lane worktree and cache.
2. At the end of every loop tick: review checkouts, finished caches, withdrawn scratch (corrupt captures, superseded drafts).
3. Before starting a lane: if `df -h /System/Volumes/Data` shows under 8 GB free, clean first; if it is still under 8 GB, start fewer lanes.

## The rule for a lane worktree

Remove it only when all of these hold; otherwise keep it and say why in the ledger:

- its PR is merged or closed (`merged_at` or `state` from the REST API for the branch -- not `git branch --merged`, which misses squash merges);
- `git -C <tree> status --short` prints nothing but `?? target` (the untracked link to the shared `target/`), or every modified file is byte-identical to `origin/master` or an older copy of it (a lane's stale skill copy);
- the lane has reported back (its task notification arrived), so no process is writing in it.

## How

The Bash sandbox cannot delete `.git/worktrees/<name>` (`Operation not permitted`), so `git worktree remove` fails there for lane trees. Run the removal in the session's Terminal tab (`run_in_terminal`, one line), then verify from Bash:

```
git -C "$(git rev-parse --git-common-dir)/.." worktree remove -f <tree>        # -f -f when a finished agent left its lock
[ -d <tree> ] && echo "still there" || echo removed
df -h /System/Volumes/Data | tail -1
```

- A review checkout in `$TMPDIR`: `git worktree remove --force $TMPDIR/rev-<lane>` from Bash; if its `.git/worktrees/rev-<lane>` entry stays behind, read that entry's `gitdir` file first, and only if it points at your `$TMPDIR/rev-<lane>`, remove that one entry from the Terminal tab.
- A Zig cache: `rm -rf $TMPDIR/zig-<lane> $TMPDIR/zig-<lane>-local` once its lane has reported.
- Lane briefs say: "delete your own `$TMPDIR` scratch and caches at the end; leave only the untracked `target` link".

## Never

- `git worktree prune`: it rewrites every session's entries at once.
- Another session's trees, `$TMPDIR` folders, or the shared caches (`$TMPDIR/zig-cache`, `$TMPDIR/zig-local`) while any lane may use them. When the disk is full because of someone else's files, name them to the owner with their sizes; do not delete them.
- A tree with uncommitted work that is not a copy of master, a tree whose PR is still open, or anything under the PDK (`~/IHP-Open-PDK`), the tool installs (`~/tools`), or the main checkout.
- `git stash` to "make a tree clean".

## Report

One line in the ledger row of the tick: what was removed (name, size), what was kept and why, and free disk before and after. Example: `cleanup: removed agent-abca (M4c, merged, 322 MB), rev-m3b entry; kept wt-b1 (not ours); free 7.2 -> 11 GB`.

## Related

- `tri worktrees [--size]` -- read-only view of every checkout's holdings; `tri lanes` -- a loop's open PRs and worktrees, judged.
- `.claude/skills/analog-emulation/SKILL.md` -- the loop tick that ends with this skill.
- `.claude/skills/model-routing/SKILL.md` -- how lanes are spawned (each in its own worktree).
