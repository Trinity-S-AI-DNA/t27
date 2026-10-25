# Mutation census of specs/analog/mos_psp103_inv20.t27, 2026-10-25

Issue #12185, part of epic #10486 (weak spots of the analog v0.1 release). The tool, `tri mutate spec`, never edits the
spec. Where each number comes from:

- **Before** (101 mutants, 72 killed, 23 survived, 5 hung, 1 unviable) is the census of the unmodified spec, lane base
  master `de7c038e2d`, `tri mutate spec`, 3 mutants at a time, zig `-j2` each, t27c 0.5.2, 09:39-09:43 +07, exit 0.
  The 23 survivor lines are read off that output; the classes below come from reading the spec and the census text.
- **After** (80 killed, 15 survived, 5 hung, 1 unviable) is the expected count, **not a measurement**: the gate has not
  been run on the changed spec. The tests appended to the spec add no mutants (the tool mutates functions, not tests).

## The 23 survivors, and what became of each

| Lines | Survivors | Label | Outcome |
| --- | --- | --- | --- |
| 127, 134, 146 (x word), 154 (x2 word) | 4 | test gap, group A: the gate-drive rounding `(v + 0.5) as u32` of `iv_nmos` and `iv_pmos` | killed by test 1 |
| 223 | 1 | test gap, group B: `k < n` to `<=` in `iv_run` writes one sample too many | killed by test 3 |
| 236 (drop-guard and `0.0 - 1.0` to `0.0 + 1.0`) | 2 | test gap, group C: the no-crossing return of `iv_delay_ps` was never taken | killed by test 3 |
| 293 | 1 | test gap, group D: `k <= n` to `<` in `iv_kick` leaves out the last sample | killed by test 4 |
| 126, 143, 146 (`>=`, `+`, xn), 154 (`>=`, xn2), 173, 199, 201, 236 (`<=`), 280, 295, 296 | 14 | equivalent | accepted |
| 176 | 1 | was undecided; equivalent, shown by test 5 | accepted |

Totals: 8 killed (4 + 1 + 2 + 1), 15 accepted (14 + 1); 8 + 15 = 23. The 5 hung mutants (177, 202, 229, 281, 297) are
drop-step mutants of loop counters (`k = k + 1` or `j = j + 1` becomes the name unchanged), so each loop never ends and
no test can go red; they are named in the accepted file with that reason. The 1 unviable mutant is the `[ret-default]`
of `iv_run` (line 209): zig answers `zig: error: not yet implemented`. The same message stands for the same kind of
mutant (a function with no return value, its body emptied) in the census of `sar_cdac`, `sar_comparator_psp` and
`sar_logic`, so the cause is on the emitter side and not in this spec; the gate counts an unviable mutant nowhere, so it
has no entry.

## The 5 tests added (appended after the last original test, so the accepted line numbers did not move)

1. `one_word_of_gate_drive_moves_the_integer_current_somewhere_and_each_device_reads_the_rounded_word` (group A, 8
   runtime asserts). A first version asserted at the round points 0.75 V and 1 V and failed: the integer current is read
   from a log word of 2^-16 octave, so one word of gate drive (6e-8 V) moves it only where the read crosses a step of
   that word, and at those points the neighbouring word read the same current. The test now searches a grid of the
   output (step 4001 words) for the first point where the current of the word and of the word below it differ, and
   asserts that the device returns the integer of the rounded word there; the mutant, which reads the word below,
   returns the other integer. One loop per mutant: 127 (gate at the supply, NMOS), 134 (gate at 0, NMOS below ground),
   146 (gate at 0, PMOS under the supply), 154 (gate at the supply, PMOS over it; a gate at 0 would read past the tables'
   edge, where the current is held and one word changes nothing). Each loop also asserts that a point was found.
2. `at_the_rail_the_second_branch_of_each_device_reads_the_same_charge_and_no_current_as_the_first` (1 assert: eight
   ties counted). At y == 0 (NMOS) and y == Vdd (PMOS), for the gate words 0 and Vdd, the source closure `iv_qs` equals
   the drain table read `sq_qt`, and the current is 0 in both. It backs the equivalence of lines 126 and 143; it does not
   kill a mutant.
3. `an_edge_run_writes_n_samples_after_the_rest_level_and_a_trace_that_never_crosses_has_no_delay` (groups B and C, 10
   asserts). B: after `iv_run(25000, IV_DT_FS, edge, 200, 0, &out)` the cell `out[201]` is still 0.0 and `out[200]` is
   written, for both edges (the mutant at 223 writes `out[201]`). C: a flat 0.0 trace and a flat supply trace give -1.0
   from `iv_delay_ps` for both edges at 500 fs (a dropped guard answers -0.5, and `0.0 + 1.0` answers +1); a positive
   control, a step from 0 to the supply between samples 99 and 100, gives 49.75 ps.
4. `a_kick_of_one_20_ps_step_is_the_last_sample_so_the_scan_to_index_n_reads_it` (group D, 4 asserts). With a step of
   20 ps the run is one step (n = 1), so the scan reads `out[1]` only; the mutant reads nothing and returns the rest
   level (about 3e-8 V rise, 1.5 V fall). Bands from a ladder of bounds run on 2026-10-25: the rise kick is in
   (-0.15, -0.1) V, the fall kick in (1.6, 1.7) V.
5. `the_rest_bisections_never_land_on_a_zero_current_so_the_strict_and_the_loose_test_take_the_same_branch` (line 176, 2
   asserts). An inline copy of the `iv_rest` bisection for the two gates it is called with (the supply and 0) counts the
   midpoints whose current is exactly 0.0: none, so `>` and `>=` take the same branch at every step; and the copy ends
   on the root `iv_rest` returns. Line 176 is therefore equivalent on this spec's inputs.

`tri test specs/analog/mos_psp103_inv20.t27` (`--t27c` the master-built binary), 10:34 +07, exit 0: 15 tests (the 10
original and these 5) pass, 0 fail, 0 vacuous passes; the 5 added tests execute 8, 1, 10, 4 and 2 runtime asserts.

## Reading the accepted file

`docs/analog/mutation-accepted-mos_psp103_inv20.txt` follows `specs/tri/mutate/survivors.t27`: one `path:line [kind]`
with a free-text note; one entry covers all mutants of that kind on the line. With `--accepted` the tool exits 2 on an
unaccepted survivor or hung mutant, and on an accepted line whose mutants are all killed. Lines 146 and 154 keep a
`swap-arith` entry: the mutant of the gate-drive rounding on each is killed now, but an equivalent `swap-arith` mutant
(146: `+` and xn; 154: xn2) stays alive on the same line, so the entry is not stale. A moved line leaves its mutant
unaccepted: edit the spec above line 563 and the file must be regenerated.

## Not done, not claimed

- The mutation gate has not been run on the changed spec, so the After counts (80 / 15 / 5 / 1) and "8 killed" are
  expected from reading each mutated line against its assert, not measured. Which test kills which mutant was not
  measured either.
- The seal is not saved (`.trinity/seals/` was not touched); a `tri seals changed` is still owed before landing.
- The 14 equivalent survivors are classified by reading the arithmetic (each entry carries it), not proved equivalent by
  a tool; the rail ties (126, 143) and line 176 are backed by asserts on this spec's own inputs only.
- The 5 hung mutants are classified by construction (a loop step dropped), not by a run that was allowed to finish.
- The unviable mutant at 209 is classified by the shared message with three other specs, not by reading the emitter.
- Group A is checked at the first grid point where the integer differs, not at every gate value the runs use; the
  tests pin the model's own rounding and tie rules, they do not compare the model with ngspice.
