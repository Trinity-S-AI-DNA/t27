# NOW -- `tri pr preflight`: the rules a PR must pass before it opens (2026-10-18)

## Why (Refs #10190)

About 190 pull requests were open, and most were red. `specs/policy/pr_budget.t27`
(#10070, #10087) caps how many may be open at once. This is the other half: a
branch is checked before `gh pr create`, so a bee PR does not open on a defect
its author could have seen.

## What

- `specs/tri/pr/preflight.t27` owns every rule, every verdict (PASS, FAIL, UNREAD,
  NOT RUN) and every message. 25 tests, 0 vacuous, 1 comptime invariant.
- The evidence is data: each command the spec names runs once, per changed spec
  where the rule is per-spec. Its output is framed as
  `@@ preflight <step> <entry> <exit> <len>`, and the spec reads the frames. A
  frame whose length does not hold is UNREAD, never PASS.
- `tri pr preflight [--quick]` runs the spec's commands, frames their output and
  prints the spec's verdict. Exit codes: 0 every rule passed, 1 a rule FAILed,
  2 evidence could not be read. `--quick` leaves out the corpus ratchet. The
  publisher gate runs it in the bee's worktree, with origin/master merged in,
  and skips a branch that did not exit 0.
- **The glue landed in #11126.** `preflight` in `cli/tri/src/prcheck.rs` is 40 lines
  (a `#[path]` mount of `gen/rust/tri/pr/preflight.rs`, the `Preflight` subcommand and the
  loop). It loops to the spec's STEPS and RULES, so a step added to the spec runs without a
  glue change. It was parked until then because `check_budget` refuses any net growth of
  hand-written foreign lines (owner rule 2026-10-09) and no label lifts it; it landed
  net-zero, paid by deleting the 52 lines of `inflight.rs` that `cargo check -p tri` listed
  as never used (`Merge`, `merge_outcome` and their three unit tests). The Queen's side,
  about 30 lines in `tools/queen/publish.py`, is not part of that change.

The rules are changes, base, check, tests, seal, types, dupes, ledgers, suite,
census, budget (the own-language foreign-line budget, #11125), devhome (the
developer-home-path scan, #11125) and kept. Only a branch's own defects fail it:

- dupes compares `dupe_scan.py` on the branch with the same scan on a clean
  origin/master worktree. A finding master already prints is not the branch's.
- suite fails only when the corpus ratchet names a changed spec.
- ledgers fails only for a changed spec's row.

## Replay (lab, parked glue, PR head with origin/master merged, full run)

| PR | preflight | agrees with CI |
|---|---|---|
| #8879 | suite, kept, dupes FAIL on lotus.t27 | Corpus ratchet; 12 of 26 tests deleted |
| #9548 | suite, ledgers, seal FAIL on stage.t27 | Corpus ratchet, t27b UNLISTED |
| #9463 | dupes, ledgers, seal FAIL | duplicate-bodies (4 new copies), t27b UNLISTED |
| #9485 | base FAIL: add/add conflict with master | not mergeable |
| #10056 | ledgers, seal FAIL on reset_queue.t27 | t27b UNLISTED; its Corpus ratchet red is hslm.t27, inherited |
| #10185 | seal FAIL on own_model.t27 | CI's reds are inherited; the missing seal is stricter than CI |

## Open

- `tools/queen/publish.py` does not call `tri pr preflight` yet; the publisher gate that
  skips a branch that did not exit 0 is still to land, net-zero as well.
- t27b's JIT runs on arm64 only and the lab is x86_64, so this PR's own
  `t27b_expectations.json` row is left to the tool after an arm64 run measures it.
- cli-tri (Rust build) and "documented t27c subcommands" are outside the spec rules.
