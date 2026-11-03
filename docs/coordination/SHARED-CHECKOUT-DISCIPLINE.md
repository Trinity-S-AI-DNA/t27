# Shared checkout discipline (owner-approved, 2026-11-03)

One shared checkout, many concurrent lanes. The index, the lock and the
staged set are shared state; without a discipline, lanes destroy each
other's work while each one believes it is doing the right thing. This
page is the rule's one home. The compact restatement lives in the
"Shared checkout discipline" section of the `own-git-landing` skill.

## Evidence base (2026-11-03, one session, one checkout)

All four are measured incidents, not hypotheticals.

1. A lane's bare `git commit -m ...` (no pathspec) swallowed and
   conflicted with another lane's staged dataset; two pre-commit runs
   then failed with `more hand-written foreign lines added than deleted`
   because foreign files from a foreign lane sat in the shared index.
2. A lane's `git add` ran while another lane held `.git/index.lock`; a
   stale-looking lock was deleted while its owning process was alive,
   and the index race left a staged set half-written (436 files instead
   of ~880). Unreferenced worktree changes were later lost.
3. An untracked dataset directory (inputs + manifests) was destroyed
   mid-session by concurrent git operations; recovery required
   hash-based mapping (`pid = sha1(rel)[:12]`) plus full re-emission.
4. Two pre-commit runs refused with net-positive foreign budgets caused
   by OTHER lanes' staged files, not by the committing lane's own diff.

## Rules

a. On the shared checkout, ALWAYS pathspec-commit:
   `git commit -- <your paths>`. A bare commit takes whatever sits in
   the shared index, which at any moment holds other lanes' work.
b. NEVER `git add` without a pathspec. `git add <your paths>` only;
   `git add -A` / `git add .` stage every lane's work into your commit.
c. NEVER delete `.git/index.lock`. If a commit fails on a lock, wait
   for the owning process to finish, or use a pathspec commit after
   confirming the owner is gone. Deleting the lock of a live process
   is what corrupted the index on 2026-11-03.
d. Untracked work you cannot afford to lose must be committed (or
   moved into a lane worktree) BEFORE another lane starts git work.
   Untracked files have no protection: no index entry, no HEAD copy,
   no ref to recover from.
e. Heavy and parallel lanes work in `.claude/worktrees` lanes. The
   shared checkout is for landing, not for concurrent staging.
f. Foreign-budget arithmetic is per-commit and counts whatever is
   staged. The pathspec commit is the isolation tool: it builds the
   commit from HEAD plus your named paths only, so another lane's
   staged files never enter your budget or your commit.

## Recovery playbook

The dataset race above (incident 3) was recovered deterministically:
hash-mapping (`pid = sha1(rel)[:12]`) to rebuild which input belonged
to which pair, then full re-emission against one verification HEAD.
The pattern, with the audit trail, is documented in
`dataset/igla-coder/v0.4/README.md` ("Rebuild event, 2026-11-03").
Read it before attempting any recovery of destroyed untracked state;
guessing at a mapping is how data is lost twice.
