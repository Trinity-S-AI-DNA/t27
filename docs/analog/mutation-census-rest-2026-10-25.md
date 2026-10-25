# Mutation census of the remaining analog specs, 2026-10-25 (partial: 1 of 17 specs)

Issue #12116, part of epic #10486 (weak spots of the analog v0.1 release). The tool, `tri mutate spec`, never edits
the spec, and the checkout stayed clean during the run. This note is written after the first spec finished and before
the other sixteen ran, so a reader sees what is measured and what is not. Where each number comes from:

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

## 2. What is not claimed

- The other sixteen specs were not run: sar_kick_net (started, then stopped), sar_switch, sar_comparator,
  sar_comparator_q, accept_sar, bench, linear, linear_zoh, mos_psp103, mos_psp103_gates, mos_psp103_inv20,
  mos_table, raw_writer, sar_comparator_qi, sar_trace_raw, vacask_compare. Their numbers are "not measured".
- The 108 survivors are not classified (equivalent mutant, missing test, or dead code) and no test was added for
  them. Each class needs a reading of the function, not a count.
- The 31 hung mutants are not proven to be infinite loops; the tool reports only that the tests outlived 30 s.
- No speed claim: 24 min 44 s is one run under load 26-46 and falling disk, not a rate.

## 3. Why the run stopped

Free space on the data volume fell from 7.4 GiB (07:59) to 0.4 GiB (08:25) while two other trees on this Mac held
a large part of it: `/private/tmp/opencode` 9.08 GiB (87 entries) and the user temp directory `opencode` 8.81 GiB
(131 entries), both read with `du -sk` at 08:31. Those belong to another tool's session and were not touched. The
census resumes when the volume has more than 4 GiB free and the 1-minute load is under 30.
