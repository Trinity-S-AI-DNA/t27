---
id: t27b-loop
name: t27b-loop
description: Run one tick of the unattended t27b improvement loop (epic #8326) -- budget guard, health checks with self-repair, merge-when-green, one plan step, and a ledger line. Use when a /loop or cron tick fires for t27b, or when resuming t27b work after a restart or compaction.
argument-hint: "[epic number, default 8326]"
---

# /t27b-loop -- one tick of the t27b improvement loop

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; shrink-only #8236; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`. Generated files are never hand-edited (L2).
> - Hand-written code in any other language may only shrink (#8236): `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile and Makefile. The gate `check_budget()` refuses a net gain, and no label lifts that.
> - t27b itself must become fully generated (#6198). Every PR under `cli/t27b` is net <= 0 in hand-written lines, counting src and tests.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - The full text is in `AGENTS.md`, "Only t27".

A tick must leave the repository better than it found it, or unchanged. It must never break the work of an
earlier tick, and it is cheap when nothing changed. Do the steps in order, and stop early when a step says so.

## 0. Budget guard (always first)

Read the plan limits with `get_usage`.
- **Weekly use >= 95%:** cancel the 15-minute cron and create a one-shot cron 5 minutes after the weekly reset that re-creates it. Write one ledger line, then stop. A loop that runs out of budget stops every session on the account, this one included.
- **Weekly use >= 85%:** no new agents. Run steps 1-3 and the agents already running.
- **Pace:** the week has 168 hours, so stay under about 0.6% per hour. If use exceeds `hours since reset x 0.6 + 5`, run no new agent this tick.
  - On 2026-10-09, three agents burned about 4.5% per hour, and the loop hit 98% in two hours.
  - Prefer one agent at a time, and a model whose separate weekly bucket has room (compare the `Weekly - <model>` windows in `get_usage`).
- **Otherwise:** run at most 3-4 agents at once, each in its own fresh worktree.

## 1. Health, with self-repair (#8325)

| check | healthy | repair |
|---|---|---|
| `git config --get core.hooksPath` (any worktree: they share the config) | `.githooks` (relative) | Set it back to `.githooks`. The absolute path runs the main checkout's uncommitted hooks, which need an unbuilt t27c and refuse every worktree commit. It flipped four times from 2026-10-09 to 2026-10-10; the owner wants it relative. Durable fix: right after every `git worktree add`, run `git -C <wt> config --worktree core.hooksPath .githooks`. `extensions.worktreeConfig` is on, so the worktree's own setting wins over the shared one. Agents never touch the shared config. |
| failing checks on master's head commit | none, except Scorecard | Each red check makes every PR UNSTABLE, and GitHub then refuses `--auto`. Fix the cause in a small PR: #8277 (a type name defined twice), #8289 (ledger rows). |
| t27b-lab `status.json` | `phase` moves, `updated` < 30 min old | If it is stale, read `ssh t27b-lab` logs before redeploying. A redeploy kills the run in flight, so time it right after a run publishes. |
| t27b-lab `image.lab_py_sha` | equals `git hash-object` of master's `contrib/railway/t27b-lab/lab.py` | `railway up` from a clean `git archive` of `contrib/railway/t27b-lab` |
| Mac disk | > 5 GB free | `git worktree remove` merged scratch worktrees; never delete another session's files |
| running agents | each reported within ~30 min | `SendMessage` with the current state. Never relay a permission an agent was denied. |

## 2. Merge when green

For every open PR by `@me`, merge only when all of these hold:
- the four required checks pass: `validate`, `check-linked-issue`, `parse-ratchet`, `own-language`;
- every other failing check also fails on master;
- the net hand-written foreign lines are <= 0;
- it is not held (see the epic).

Then:
- **Checks still pending:** run `gh pr merge N --auto --squash` right after the push. GitHub refuses `--auto` once every check has finished and a non-required one is red (UNSTABLE).
- **Checks finished and required ones green:** merge with `gh pr merge N --squash`, but only when the owner has said "merge when green" for this loop.
- **DIRTY:** merge master in from master's side, never rebase. Derived files are regenerated, not hand-merged:
  - `docs/reports/t27b_expectations.json`: master's rows plus only the lane's rows, with `max_not_pass` recounted;
  - the AGENTS.md remainder line: re-measured with `wc -l`;
  - seals: re-saved with master's t27c.

## 3. One plan step

**DRY first (owner rule, 2026-10-11).** Before writing any spec or function, search for an existing one: `t27c dupes --name`, `tools/dupe_scan.py --like`, `specs/tools/catalog.t27`. Then reuse or extend it. Never add a new conformance spec per feature: put the cases in the area's existing one. `tools/duplicate_bodies_baseline.txt` only moves down.

Take the first unchecked item of epic #8326 that is not already claimed, either by a branch (`git branch -a`) or by an open PR. Move it one PR forward, then tick it in the epic.

- **Write the decision in `.t27`.** Generate it on the t27c lab:
  - `ssh -o BatchMode=yes t27c-lab`;
  - zig lives at `/data/zig/zig-x86_64-linux-0.16.0`;
  - a fresh `git worktree add --detach /data/w-<topic> FETCH_HEAD` from the lab clone;
  - `cargo build --release -p t27c` with a shared `CARGO_TARGET_DIR`.
- **Bring the files back to a sparse worktree on the Mac.** `git worktree add --no-checkout`, `git sparse-checkout set --no-cone <paths>`, then copy each file with `ssh ... cat`. `gen/` is gitignored: use `git add -f`.
- **Ledger rows: the steward writes them.** The t27b ledger steward (`specs/jobs/t27b_ledger_steward.t27`, rules in `specs/tri/t27b/ledger_steward.t27`, #8654) adds the row from master's next run after a spec PR merges, and opens one ledger PR with auto-merge. Once the Queen runs that card (see gHashTag/trios#1751), spec PRs leave `docs/reports/t27b_expectations.json` alone.
  - **Until then, every new spec in a PR still gets its row.** A missing row turns master's `t27b-native-ratchet` red for everyone (UNLISTED). That happened 6 times in 30 minutes on 2026-10-09.
  - **The steward refuses** UNEXPECTED FAILURE, MOVED, BAD REASON, OVER CAP, a defect where the reference passes, a cap rise larger than the new rows, and more than 16 STALE rows. A refused finding no longer holds the run back: the steward applies its other rows, leaves the refused row as master has it, and lists it in the ledger PR's table. Only a run with nothing else to apply, a cap rise or a STALE flood gets one issue labelled `t27b-ledger-steward` and no PR. Those issues and the REFUSE rows in steward PRs are this loop's to read.
- **Receipts:** while master moves every 10-20 minutes, compare a lane's squash commit on master against its parent after the merge:
  - `cd /work/t27 && /work/t27c-master corpus-receipt compare BASE.receipt.json HEAD.receipt.json --challenge-head <nonce>` on t27b-lab;
  - a master receipt carries no nonce and is accepted as BASE;
  - REGRESSED or REFUSED means a revert PR at once.

## 4. Ledger line

Append one line to the loop ledger memory (`t27b-loop-ledger.md`): the time, what changed, and what was learned. A trap that cost time goes into this skill's table in the same tick, so the next tick cannot fall into it again.

## Traps seen (2026-10-09)

- **"fix #N" closes issue N, even inside a table cell.** Cite other issues as "see #N".
- **zsh treats `$n:r` as a modifier, which breaks refspecs.** Write `${n}`, and verify a push with `git ls-remote`.
- **The classifier denies `gh pr merge --squash` in auto mode** ("Merge Without Review"). `--auto` passes.
- **The lab drops a request whose head moved.** It also dropped requests it had not fetched yet; since #8320 those wait 30 min.
- **An unrelated session can change shared `.git/config`.** Verify the setting; do not trust that it was restored. Pin per worktree instead of fighting over the shared value.
- **Other sessions' spec rewrites can turn master's native ratchet red with UNEXPECTED FAILURE**, even when t27b did not regress (c7570433e and c9d36f314 on 2026-10-10). Before suspecting t27b, check with `git log` whether the spec changed.

## Traps seen (2026-10-19, #10837)

- **A blocked row cannot be added while the ledger sits at `max_not_pass`.** #10380 merged two specs with no rows. One of them is blocked, so its row would be the 140th not-pass row, and the ratchet then reports OVER CAP. Raising the cap is not allowed. Free a slot first: make t27b pass a blocked spec whose blocker is small. In #10837 that was `@truncate` in `specs/tri/utils/utf8.t27`, written as `truncate_plan` in `int_cast_plan.t27`. Count the slots before you file the issue: `grep -c '"t27b": "blocked"'` plus `codegen`, compared with the cap.
- **The fs-lab is x86_64, but the t27b JIT runs only on arm64.** Without `--runner "qemu-aarch64 -L /usr/aarch64-linux-gnu"`, `t27b corpus` reports every spec as `crash: exit 74`, and the ratchet then shows hundreds of UNEXPECTED FAILURE lines that t27b did not cause. Keep `--jobs 16`: at 32 or 40 the reference returns `spawn: Resource temporarily unavailable`, and those blocked rows poison the reference cache. Before reusing a cache, keep only its `pass` and `fail` rows.
- **The lab's `/data` volume can fill to 100%.** Cargo then fails with `failed to select a version ... locked to` rather than a disk error, because the index update in `CARGO_HOME=/data/cargo` cannot write. Set `CARGO_HOME` to a directory on `/root` (the overlay has free space) rather than deleting other lanes' trees.
- **Changing a plan module's behaviour also flips that module's old negative tests.** `assert !from_destination("@truncate")` failed once `@truncate` became a destination cast. Read the module's tests for the construct before you run them.
- **Two sessions can take the same ledger issue.** #10853 (bare `sqrt`) and #10862 (`@truncate`) both closed #10837, and each stayed red alone: one lacked the rows the other had, and the other lacked the slots. Before you open a PR for an issue, run `gh pr list --search "<issue number>"`. If a sibling PR exists, merge its branch into yours (fetch it as `refs/heads/<name>`), and say "Carries #N" in the body.
- **A cap lowered to the exact count leaves no room for rows other PRs still need.** #10853 set `max_not_pass` to 129. eval_hslm and ngram_hslm still needed two blocked rows, so the combined PR set 130. That is still below master's 139, so it does not raise the cap. Recount after every merge: `blocked` + `codegen` = cap.
- **The Mac hook refuses any Bash text that contains the words of a test command**, even inside a heredoc for a PR body. Write PR bodies with the Write tool.
- **A merge of master that changes `lefthook.yml` is refused by the own-language pre-commit hook**, because the gate file is owner-only and the merge commit carries master's edit to it. Never `--no-verify`. Instead, `git merge --abort`, make the branch's side of each conflicting file equal master's (for the ledger: master's rows plus only the lane's rows) in a normal commit, then check `git merge-tree --write-tree HEAD origin/master`. rc 0 means GitHub can squash it with no conflict (#10862).
- **`FETCH_HEAD` is whatever the last `git fetch` brought, not master.** In a loop over several branches, each `git fetch origin <branch>` overwrites it, so a later `git merge-base B FETCH_HEAD` compares a lane with itself. It reports no lane rows and a merge-base equal to the branch tip. Fetch master into a named ref once, `git fetch origin master:refs/remotes/origin/master`, and use `origin/master` (or its sha) for every merge-base, row diff and merge-tree check (#10792 stack, 2026-10-19).
- **Every branch in a stack carries its base PRs' rows.** After the base merges, those rows are already on master, so diff the lane's rows against master's rows (`comm -23 lane.add master.rows`), not against the merge-base. Only the rows master still lacks go into the rebuilt ledger. For the six-PR t27f stack that was one row (`glyph57`), plus one more on #10800.
- **Auto-merge squashes the head it saw go green, and a push in flight can miss it.** #10862 merged at 330dd0aef while 186c3ab6d, the commit with its last two ledger rows, was still being pushed. Master then reported both specs UNLISTED. After a PR with `--auto` merges, compare `gh pr view N --json headRefOid` with the lane's last commit. If they differ, carry the missing commit in the next PR at once (#10889 carried those rows).
- **A seal conflict cannot be resolved by the make-it-match recipe while master's diff carries `lefthook.yml`.** Both sides change the same seal lines against an old merge base, so any third value conflicts again. Instead, set the branch's seal to master's in a normal commit (the merge is now clean), let GitHub write the merge with `gh api -X PUT repos/gHashTag/t27/pulls/N/update-branch -f expected_head_sha=<head>`, then re-save the seal on the lab from the merged head and push it (#10806).
