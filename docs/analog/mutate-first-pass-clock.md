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

## Measurement (2026-10-26, two paired runs on one Mac)

The command, the same in every run; only the `tri` binary differs (the driver of master before the wiring, and the
driver of the wiring), the master-built `t27c` and the accepted file are shared:

```
tri mutate spec --file specs/analog/linear_zoh.t27 --accepted docs/analog/mutation-accepted-linear_zoh.txt \
  --t27c <master-built t27c> --jobs 3 --timeout 45
```

| Pair | Run | Wall time | 1-min load at start | Mutants (killed / survived / hung / unviable) | Reruns | Exit |
|---|---|---|---|---|---|---|
| 1 | Before (driver of master before the wiring) | 708 s (11 min 48 s) | 30.54 | 223 / 2 / 27 / 0 | none (no first-pass clock) | 0 |
| 1 | After (this slice) | 401 s (6 min 41 s) | 8.18 | 223 / 2 / 27 / 0 | 3 | 0 |
| 2 | Before | 764 s (12 min 44 s) | 5.89 | 223 / 2 / 27 / 0 | none | 0 |
| 2 | After | 485 s (8 min 5 s) | 26.88 | 223 / 2 / 27 / 0 | 3 | 0 |

What the table does and does not show:

- The verdict is the same in all four runs. In pair 2 the two outputs differ by exactly one line, the new "First-pass
  clock" summary line.
- The wall time was shorter with the wiring in both pairs: by 307 s (43 %) in pair 1 and by 279 s (37 %) in pair 2.
  Both savings are about the size the summary line computes (915 job-s over 3 jobs is 305 s); that line is arithmetic
  on counts and is not a measurement, so the agreement is an observation, not a proof of cause.
- The load was not controlled: the loads at start differ inside each pair (pair 1: the before run started under the
  higher load; pair 2: the after run did), the 1-min load rose to 21-26 within minutes of any run starting, and one
  sample of 64.68 was seen during the pair-2 after run. Other sessions used the machine at the same time. Pair 1 started
  the wiring from a lower load and pair 2 from a higher one, and both show a saving, so the direction is the same under
  both orders; the size is not pinned down.
- Two pairs on one machine are not a benchmark. No general speed claim follows from them, and none is made: only that
  on this gate, in these four runs, the wall time was shorter with the first-pass clock and the verdict did not change.

The success criterion "same load class" of issue #12403 is therefore met only in part (the loads are written down, they
are not equal). A third pair with the loads matched would settle the size; the wiring stays on by default because the
direction held in both orders and the verdict never moved.

## Tests

`cli/tri/src/mutate.rs` carries one unit test of the driver side: a hung, unaccepted mutant is rerun; an accepted one is
not; a test binary that is missing is not rerun; and with no baseline the clock is `FLOOR_S`, capped by a smaller
`--timeout`. The rule itself is tested in `specs/tri/mutate/clock.t27`. After the transport was deleted the whole
`mutate::` test group of `cli/tri` passed in a T27 OS pane (41 passed, 0 failed).
