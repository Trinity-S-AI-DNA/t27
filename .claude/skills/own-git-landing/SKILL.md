---
id: own-git-landing
name: Own git landing
description: How every agent lands work -- verify locally, land on the local golden head, push to GitHub once per batch. Use before any git push, PR, update-branch, or re-run of checks; when CI is queued; when the owner says "GitHub is our bottleneck", "наш git", "golden chain", "батч".
---

# Own git landing (owner, 2026-10-20, #11275)

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

## The law

The rule is `specs/queen/golden_chain.t27`, section 6 (lane_ready, batch_from, when_to_push, lane_may,
gc_rerun_local, gc_bisect), over sections 1-5 (links, golden head by CAS, CARRY, gc_land). Do not
restate it; read it. Related: `specs/claw/push_gate.t27` (is a push worth its place in the queue),
`specs/claw/queue_cost.t27`, `specs/verified/reuse.t27` (verdict reuse), `specs/tri/land/land.t27`.
Why: a push starts every required check again at the back of an Actions queue of 800-900 runs (#10644,
#10588, #6595). Our git judges a change by its verdicts, locally.

## Practices

1. Verify locally first, in this order: `t27b test <spec>` on each changed spec and on the specs that
   import it; `t27c test-report <spec>` (fresh `ZIG_GLOBAL_CACHE_DIR`, delete after; 0 failed, 0
   vacuous); `t27c seal <spec> --save`; regenerate with `tri gen`, never hand-edit `gen/`.
2. Record the link: SPEC, GEN, SEAL, TEST, RECEIPT. A stage whose input key was judged golden before is
   REUSED, not re-run.
3. Land on the local golden head (`refs/golden/head`, GC_REF), only through gc_land (compare-and-swap on
   the parent). Never write the ref by hand.
4. Push to GitHub once per batch: up to 8 ready links, every 15 minutes (30 while the Actions queue
   holds 200 or more runs). One PR per batch, `Closes #N` for each issue in the commit messages (L1).
5. Merge master before pushing a shared file; derived paths (gen/, ledgers, figures) are regenerated on
   the new head, never merge-resolved.
6. A red batch is bisected here by replaying prefixes (gc_bisect); do not re-run GitHub to find it.

## Never

- Push per commit; push a link that is not golden; run `update-branch` loops.
- Re-run checks after a derived-only rebase (CARRIED or REGEN re-run nothing).
- Force-push, or move the head except by gc_land.
- Count a red as inherited unless the PR's failure set is a subset of master's.

## Traps

- `--auto` merge races a second push: run every gate before the first push, or the auto-merge lands
  commit 1 alone.
- Absolute `core.hooksPath` runs the main checkout's hooks in a worktree; use
  `git -c core.hooksPath=.githooks commit`.
- "fix #N" in a PR table closes the issue; cite as "see #N".
- Backticks inside `--body "..."` run in zsh; use `--body-file - <<'EOF'`.

## Commands, and what is missing

- `tri land [branch] [--plan|--check]` exists (specs/tri/land/land.t27, #11068): the GitHub leg for ONE
  branch (merge master, tests, seals, push, PR, auto-merge). Use it for the batch branch, once.
- `tri golden land [--plan]`, `tri golden push [--plan] [--queued N] --issue N`, `tri golden status` run the local leg
  (specs/tri/land/golden_land.t27, a thin host over golden_chain.t27; hosted by `cmd_golden` in cli/tri/src/main.rs, #11275).
  `land` reads the queue (local claude/evolve-* branches and `.trinity/golden/queue`), judges each link by `t27c seal --verify`
  and `t27b test` in a scratch worktree of its tip, and lands the ready ones on `refs/golden/head` by `git update-ref` with the
  old value (compare-and-swap); `--plan` writes no ref and prints the update-ref lines. `push` applies when_to_push to the links
  after `refs/golden/pushed` and, when due, cuts `claude/golden-<sha>` at the batch's last link and runs `tri land` on it once.
  A lane built on a head that has since moved is UNKNOWN_PARENT: `git merge refs/golden/head` into it first.
