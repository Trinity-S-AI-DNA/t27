# `tri mutate spec`: the first-pass clock, wired (slice 2 of #12345)

Issue #12403, part of #12345. The rule is law in `specs/tri/mutate/clock.t27` (slice 1); this note records how the
driver in `cli/tri/src/mutate.rs` uses it. The driver holds no constant and no formula of its own: it calls the
generated functions `first_pass_s`, `rerun_at_cap` and `job_s_saved` (`gen/rust/tri/mutate/clock.rs`). Change the
rule in the spec and regenerate; never edit the Rust.

## What the driver does

1. The baseline (the unmutated spec) is lowered, compiled and run once at `--timeout`, as before. Its compiled test
   binary is then run once more and timed; that net test time, rounded up to whole seconds, is the baseline of the
   rule. (Timing the whole baseline step would count `t27c gen` and the compile, which a mutant's hang clock does not
   cover.)
2. Every mutant's test run gets `first_pass_s(baseline, --timeout)` instead of `--timeout`. The `t27c gen` and the
   compile step keep the full `--timeout`, as before.
3. A test run that hangs on the first-pass clock and that the accepted file does not name is run once more at the full
   `--timeout` (`rerun_at_cap`). A hung mutant the accepted file names is not rerun. "Names" is read by the existing
   `gate_counts` (survivors.t27), so the accepted-file reading has one home.
4. A mutant that hangs while compiling (a comptime quota that never ends) leaves no test binary; it is neither counted
   nor rerun, because a longer test clock would not help it.
5. If the first-pass clock is not shorter than `--timeout` there is nothing to save, so nothing is rerun or counted.
6. The summary prints one line: the first-pass clock, the number of reruns and `job_s_saved`. It is arithmetic on counts
   (hung mutants, accepted hits, the two clocks), not a measurement.

The gate verdict is unchanged: a hung mutant that is still hung after its rerun fails the gate (exit 2) unless the
accepted file names it.

## The `--lab` transport is gone

The same change deletes the `--lab` and `--lab-wait` flags of `tri mutate spec` and the code behind them (`LabEnv`,
`LabRun`, the upload, probe and launch helpers, `lab_mutate_spec`, and their tests). They drove a remote lab service that
the project no longer runs (the lab moved to T27 OS), and no caller was left: `git grep` finds none outside `mutate.rs`,
the `lab.t27` comments and old `docs/now` notes. The module `lab` (`zig_j`, `lab_jobs`, the exit codes) stays, because
local runs use it; so `specs/tri/mutate/lab.t27` and its seal are untouched, and the regenerated `lab.rs` is byte-identical
to the committed one. `--jobs` now defaults to 4 on every run.

The deletion pays the foreign-line budget of this wiring (lefthook `foreign-budget`: added lines must not exceed deleted
lines): 40 lines added to `mutate.rs`, 973 deleted.

## Known limits

- With no usable baseline (0 s) the clock is `FLOOR_S`, capped by `--timeout`; it is not the full cap.
- An accepted hung entry that is really a slow kill is found only at the cap: it is not rerun, so it stays a hang.
  `clock.t27` states the same limit.
- `job_s_saved` multiplies counts the caller supplies; its precondition (hung x cap below 2^32) is the spec's.
- Every number in the rule is proposed, not measured on t27 (`clock.t27`, last paragraph of its header).

## Measurement (not part of the change that wires the clock)

No speed claim is made here. This change wires the rule and deletes dead code; the before/after run is recorded on
issue #12403 and in this table by the run that owns the machine. A claim needs both runs, their command and the load at
the start:

```
tri mutate spec --file specs/analog/linear_zoh.t27 --t27c <master-built t27c> --jobs 3 --timeout 45
```

| Run | Wall time | Load at start | Mutants (killed / survived / hung / unviable) | Reruns |
|---|---|---|---|---|
| Before (master driver) | not yet measured | | | |
| After (this slice) | not yet measured | | | |

If the measured wall time is not shorter, the wiring must be switched off by default (issue #12403, Success Criteria)
and this note must say so.

## Tests

`cli/tri/src/mutate.rs` carries one unit test of the driver side: a hung, unaccepted mutant is rerun; an accepted one is
not; a test binary that is missing is not rerun; and with no baseline the clock is `FLOOR_S`, capped by a smaller
`--timeout`. The rule itself is tested in `specs/tri/mutate/clock.t27`. After the transport was deleted the whole
`mutate::` test group of `cli/tri` passed in a T27 OS pane (41 passed, 0 failed).
