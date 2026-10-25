# Mutation census of the remaining analog specs, 2026-10-25 (partial: 2 of 17 specs)

Issue #12116, part of epic #10486 (weak spots of the analog v0.1 release). The tool, `tri mutate spec`, never edits
the spec, and the checkout stayed clean during the run. This note is written after the second spec finished and before
the other fifteen ran, so a reader sees what is measured and what is not. Where each number comes from:

- **Run:** `tri mutate spec --file specs/analog/sar_logic.t27 --t27c <master-built t27c> --jobs 3 --timeout 30`, in
  a T27 OS shell cell on this Mac, started 07:59:23 +07 and finished 08:24:07 +07 (24 min 44 s of wall time), exit 0.
  Lane base `035df5ce60`, binary `t27c 0.5.2` built from master. No `--accepted` file: this is a census, not a gate.
- **Machine at the start:** load average 26.13 19.54 17.57, 7.4 GiB free on the data volume. At 08:25 the volume
  was down to 0.4 GiB and the run on the second spec was stopped by hand (kill threshold 2.5 GiB); see section 3.

## 1. sar_logic.t27 (677 mutants)

Follow-up issue for the survivors and hung mutants: #12163.

Operator classes of the 677: drop-guard 57, flip-cmp 198, swap-logic 57, drop-step 64, swap-arith 249, ret-default 52.

| Outcome | Count | Read off the output |
| --- | --- | --- |
| killed | 526 | all 526 by a failing test, 0 by an invariant at compile time |
| survived | 108 | drop-guard 13, drop-step 4, flip-cmp 53, swap-arith 27, swap-logic 11 |
| hung | 31 | all 31 are drop-step; the first two read are `j = j + 1` -> `j = j` and `i = i + 1` -> `i = i`: the tests outlived `--timeout 30` |
| unviable | 12 | all 12 are ret-default; the first two read (`sl_init`, `sl_links`, a `-> void` body emptied) give `zig: error: not yet implemented` |

The 108 survivors start at line 331 of the spec and fill every 50-line bucket up to 1050-1099 (3 to 14 per bucket).
The densest are 500-649 (38 survivors, around `sl_delay`, `sl_net`, `sl_init`, `sl_links`, `sl_loads`) and 750-849
(21, around `sl_run`, `sl_edges`, `sl_absdiff`). The first three, for orientation: line 331 `p < 2` -> `p <= 2`, line
335 `p < 3` -> `p <= 3`, line 340 the guard `if (p == 1) { return 250; }` dropped.

## 2. sar_trace_raw.t27 (42 mutants)

Run: same command and binary, started 09:08:39 +07, finished 09:10:53 +07 (2 min 14 s), exit 0, load 24.11 at the
start, 3.0 GiB free at the end. No follow-up issue is needed for this spec: nothing in it is a test gap.

Operator classes of the 42: drop-guard 2, flip-cmp 13, swap-logic 2, drop-step 6, swap-arith 15, ret-default 4.

| Outcome | Count | Read off the output |
| --- | --- | --- |
| killed | 38 | all 38 by a failing test, 0 by an invariant at compile time |
| survived | 1 | line 158, flip-cmp: `if (dd > r.worst) { r.worst = dd; }` -> `>=` |
| hung | 3 | all 3 are drop-step: line 100 `k = k + 1;`, line 160 `v = v + 1;`, line 162 `p = p + 1;` |
| unviable | 0 | |

Classification, read off the spec (`trace_readback`, lines 158-162, and `sar_trace_with`, line 100):

- The survivor is an **equivalent mutant**: when `dd == r.worst` the body assigns `r.worst = dd`, which is the value
  it already holds, so `>` and `>=` give the same state on every input.
- The three hung mutants are **loops the mutant made endless**: each dropped step is the increment of a `while`
  counter, so the loop never ends and the test outlives `--timeout 30`. The tool says as much in its own footer.
  The unmodified spec ran to the end in the same run (38 killed by failing tests, none by a timeout).

An accepted-list file for a future gate would hold the one survivor with the reason above; it is not added here,
because this note is a census and no gate runs on this spec yet.

## 3. What is not claimed

- The other fifteen specs were not run: vacask_compare (started 09:10, 48 mutants listed, then stopped), bench,
  mos_psp103_inv20, mos_psp103_gates, sar_comparator_qi, linear, linear_zoh, sar_comparator_q, raw_writer,
  sar_switch, mos_table, accept_sar, sar_kick_net (started 08:24, then stopped), sar_comparator, mos_psp103.
  Their numbers are "not measured".
- The 108 sar_logic survivors are classified in #12163 by reading, not by this note; no test was added for them
  here. Each class needs a reading of the function, not a count.
- The 31 sar_logic hung mutants are not proven to be infinite loops; the tool reports only that the tests outlived
  30 s. For sar_trace_raw the three are read as endless loops from the dropped counter step, not re-run.
- No speed claim: 24 min 44 s and 2 min 14 s are two runs under load 24-46, not a rate.

## 4. Why the runs stopped

First stop, 08:25. Free space on the data volume fell from 7.4 GiB (07:59) to 0.4 GiB (08:25) while two other
trees on this Mac held a large part of it: `/private/tmp/opencode` 9.08 GiB (87 entries) and the user temp directory
`opencode` 8.81 GiB (131 entries), both read with `du -sk` at 08:31. Those belong to another tool's session and were
not touched.

Second stop, between 09:10 and 09:20. The third spec (vacask_compare) printed its mutant count and then no process was
left: the lane worktrees `lane-12070` (it held the `tri` and `t27c` binaries the run used) and `lane-12116`, with
about fifty other worktrees, were removed by something other than this loop, and the T27 OS app was relaunched around
09:18. The branch was already on origin, so no work was lost. The census resumes from a recreated lane with the
`tri` and `t27c` built in the main build tree (`t27c 0.5.2`, the same version as the first runs) when the volume has
more than 4 GiB free and the 1-minute load is under 30. At 09:25 the load was 80, so it did not start.
