Loss L1 of `specs/tri/shootout/codegen_audit.t27` -- the conditional in the 200M
heartbeat loop -- written down to the emitted byte, with the conditions a
branchless form must satisfy, and the rule for reporting a noisy measurement
row. **No Rust file is touched, and this PR claims no measurement.** It is
opened so that a runner that is allowed to execute tests does execute them.

## What is here

| file | what it is |
|---|---|
| `specs/tri/shootout/l1_clamp.t27` (new) | the saturating wrap on its own, 3 functions, 5 tests pinning the edges of the wrap: 1, the limit, the limit + 1, the largest u64. The regression gate: the same values must hold after the transform |
| `specs/tri/t27b/ifconv_plan.t27` (new) | when an `if` may become a select, 16 functions, 14 tests. Nothing in `cli/t27b` calls it yet |
| `specs/tri/shootout/acceptance_matrix.t27` | the one-line spread rule replaced by three functions and three tests (below) |
| `.trinity/receipts/t27b-l1-clamp-2026-11-02/README.md` | the snapshot, the emitted assembly today, the hand-derived after-shape, the numeric edges, the prepared-not-applied Rust glue, and a correction owed to `codegen_audit.t27` |

## The correctness rule for if-conversion

Speculating an arm is sound exactly when its trap range is disjoint from the
guard interval of the side where it stopped running. `x - k` traps on
`[0, k-1]`, `x + k` on `[2^64 - k, max]`, and `k == 0` traps nowhere; each
comparison's guard is one interval, which is why `!=` is refused -- its true set
is two. The benchmark's clamp passes, because `t - 1` traps only at `t == 0`
and the arm it stopped running on is guarded by `[1001, max]`.

This is a rule over kinds and immediates, not a pattern match on this
benchmark. The negative controls are in the spec: a subtraction and an addition
whose trap falls inside the other side's guard are both refused, as are memory
arms, control-flow arms, non-integer values, a compound condition, and anything
over the select budget.

## The noisy-row rule

`acceptance_matrix.t27` used to say a noisy row "is re-run, never averaged in".
That permits re-running until the number looks quiet and then reporting the
quiet series. Replaced by `spread_status`, `may_run_extra_series` and
`union_spread_ms`: every run is kept; a row whose spread exceeds
`SPREAD_LIMIT_PCT` (5.0) of its own median is UNSTABLE and its conditions are
recorded; at most `EXTRA_SERIES_MAX` (1) further series, fixed in advance, may
be added and is reported beside the first; and two series are read as their
union, which can never be smaller than either.

The AOT receipt's own five runs are the test case:
`[341.655, 338.016, 349.013, 344.084, 324.901]`, median 341.655 ms, spread
24.112 ms = 7.06 percent. That row is therefore UNSTABLE. Adding an imagined
quieter second series leaves the union spread at 24.112, which is the arithmetic
that forbids reporting the quiet one alone.

## What has and has not been run

Run locally, on the binaries named in the receipt's snapshot table:
`t27c parse` and `t27b asm` on each spec, every one rc 0. Those are the only two
commands the local hook allows, and they are parse and lowering, not execution.

**No test in these specs has been executed anywhere.** That is what this PR is
for. Two jobs here do execute them:

- `spec-guards`, step "Affected specs -- select with specs/ci/affected.t27, test
  only those": runs `t27c test-report <spec> --specs-dir specs` on each changed
  spec against zig 0.16.0 on `ubuntu-24.04`, and prints `<file>: head H, base B
  (failing tests)`. For a new file the base is 0, so any failing test turns the
  step red and names the file.
- `t27b-native-ratchet` on `ubuntu-24.04-arm`: `t27b corpus specs --json` with
  `--reference ./target/release/t27c`, so every spec runs on both the t27b JIT
  and the t27c + zig reference. Per-file counts land in the `t27b-native-run`
  artifact (`run.json`: `tests`, `invariants`, `asserts`, `blockers` for each
  engine). It gives counts, not per-test names, and no per-file exit code.

Expected, and not a defect of this PR: the four specs have no rows in
`docs/reports/t27b_expectations.json`, so the ratchet will report them UNLISTED
and go red. The rows are deliberately absent -- a row written before the run
would be a guess at the verdict. They are added from the observed values once
this run reports, and that must happen before any merge, because a missing row
turns master's ratchet red for everyone.

## Also recorded, so it is not re-derived

`codegen_audit.t27`'s `L1_RECOVERABLE = 4` is optimistic by 2 to 3 instructions:
the hand-derived branchless shape still needs the `cmp`, the trap check and one
`movz`, leaving 6 instructions (5 once the loop-invariant `movz` is hoisted), not
3. The receipt records the correction and why it is not applied in this PR --
changing that constant moves the model's other rows, and those are what the
existing 16-of-16 receipt is bound to.

The inline row is not a prerequisite of L1. On `origin/epic2-inline-decision-glue`
(head `966ff7e1f7`, PR closed unmerged) the glue builds the facts, calls
`should_inline_call` and then returns `false`, so the decision is computed and
discarded; its call-site field is `inside_loop` where master's `inline.t27` says
`hot_loop`; and the branch also deletes two unrelated files. What is missing
there is an executor, not glue. Nothing of it is restored here.

Refs #12877.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
