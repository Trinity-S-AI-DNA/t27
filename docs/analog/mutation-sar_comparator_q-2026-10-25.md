# Mutation census of specs/analog/sar_comparator_q.t27, 2026-10-25

Issue #12229, part of epic #10486 (weak spots of the analog v0.1 release). The first lane wrote the missing asserts of
groups A and B and the accepted file; groups C, D, E and the dead lines were left to the second issue #12230, whose
lane (branch `lane/analog-mut-sqq-cde-12230`) added three tests, decided the dead lines, and extended this note and
the accepted file; its parts are marked "12230" below. The analog loop ran the mutation gate on the tree of the first
lane (it passed, see "After" below) and re-saved the seal; the tree of 12230 has had both since (the gate passed on
it, see "After, tree of 12230" below, and the seal was re-saved). Where each number comes from:

- **Before** (336 mutants, 266 killed, 40 survived, 17 hung, 13 unviable) is the analog-loop census of the unmodified
  spec, 10:25-10:39 +07, exit 0, 14 min 13 s, `tri mutate spec` 3 mutants at a time with zig `-j2` each, t27c 0.5.2
  ("build 0 commit unknown"), tree at 035df5ce60. Kinds: drop-guard 20, flip-cmp 66, swap-logic 3, drop-step 34,
  swap-arith 181, ret-default 32. The spec in this lane's tree is byte-identical to master's, and the 40 survivor lines
  below were re-read against it in this lane; the first step of FR-001 (one gate run on the unmodified spec with the
  master-built compiler) was not done, so the census is the only count of the unmodified spec.
- **After, tree of 12229** (measured): 336 mutants, 278 killed (all 278 by a failing test, 0 by an invariant at compile time), 28
  survived, 17 hung, 13 unviable (278 + 28 + 17 + 13 = 336). Run: `tri mutate spec --file
  specs/analog/sar_comparator_q.t27 --accepted docs/analog/mutation-accepted-sar_comparator_q.txt --jobs 3 --timeout 30`
  with a master-built compiler (the build that also re-saved the seal), in a T27 OS pane, 11:49:10-12:02:56 +07 (13 min
  46 s), 1-min load 10.5 at the start, base `368ce03b6d`. The gate printed "45 not killed (28 survived, 17 hung), 45
  accepted by the file; 0 accepted line(s) now killed" and "Gate passed", exit 0. The 28 survivors are exactly the
  lines the table below marks equivalent (17), test gap of groups C, D, E (7, pending then, killed by 12230) or dead line (4); none of the lines of groups A and
  B (530, 533, 549, 555, 561, 562, 564) is among them. The forecast written before the run was the same four numbers.
- **After, tree of 12230** (measured): the same 336 mutants, gate on the lane tree at 9129e6e5ce (the spec and the
  accepted file are byte-identical to the merged tree; master was merged afterwards and changes neither). Run: `tri
  mutate spec --file specs/analog/sar_comparator_q.t27 --accepted docs/analog/mutation-accepted-sar_comparator_q.txt
  --jobs 2 --timeout 45` with the master-built compiler, in a T27 OS pane, 20:42:30-21:02:23 +07 (19 min 53 s), 1-min
  load 14.3 at the start. The gate printed "38 not killed (21 survived, 17 hung), 38 accepted by
  docs/analog/mutation-accepted-sar_comparator_q.txt; 0 accepted line(s) now killed" and "Gate passed", exit 0: the
  forecast of 38 / 21 / 17 held exactly. A first run of this gate (`--jobs 3`, started 20:28) was stopped at about
  mutant 190 by the kill rule when another session's seal sweep took the 1-min load to 71.7, so it gave no result.
- **Tests, tree of 12229**: `tri test specs/analog/sar_comparator_q.t27` with the old 0.5.2 binary in a T27 OS
  pane, last run 11:46 +07 (after the final comment edit; the 11:41 run before it read the same): 12 tests (8 before), 12 pass, 0 fail, **0 vacuous passes of 12**, exit 0.
  The same t27c prints the same version text as the census's ("t27c 0.5.2", "build 0 commit unknown"); whether the build
  is the same cannot be told from that text.
- **Tests, tree of 12230**: `tri test specs/analog/sar_comparator_q.t27 --t27c <build>/target/release/t27c` from the
  lane directory, in a T27 OS pane, 19:44:48-19:44:59 +07 (11 s), 1-min load 25 at the start: **15 tests (12 before), 15
  pass, 0 fail, 0 vacuous passes of 15**, exit 0; the new tests ran 25, 6 and 5 runtime asserts.

## The 40 survivors, and what became of each

| Lines | Mutant | Verdict | Outcome |
| --- | --- | --- | --- |
| 162, 163 | `j - jf + 1` to `j + jf + 1`, `i >= SQ_R` to `i > SQ_R` (sq_clk_at) | equivalent: SQ_R = 1 | accepted |
| 166, 167 x3 | `j >= SQ_R` to `j > SQ_R`; `SQ_OFF + j * (VDD / SQ_R)` to `- j *`, `j /`, `VDD * SQ_R` (sq_clk_at) | equivalent: line 167 runs only at j = 0, where all four versions are SQ_OFF; at j = 1 the flip falls to SQ_OFF + VDD = SQ_VDD | accepted |
| 202 | `w >= SQ_QOFF` to `w > SQ_QOFF` (sq_qw) | equivalent: at the tie `sc_pack(0, .)` is 0 with either sign | accepted |
| 247, 251, 255 | `xn > 0` to `>= 0`, `x > VDD` to `>=`, `y > VDD` to `>=` (sq_qt) | equivalent: the added term has magnitude 0 | accepted |
| 347, 350 | `vg > vlo` to `>=`, `vhi > vg` to `>=` (sq_dev) | equivalent: at a tie x and xn are both 0 either way | accepted |
| 484 | `vdn > vdp` to `>=` (sq_rest) | equivalent: the same word | accepted |
| 548 | `tn < tp` to `<=` (sq_t_first) | equivalent: at a tie tn = tp | accepted |
| 586 | `dt * SQ_R / 2.0` to `dt / SQ_R / 2.0` (sq_node_at) | equivalent: (SQ_R as f64) is 1.0 | accepted |
| 602, 655 | `d > worst` to `>=`, `e > worst` to `>=` (sq_node_err, sq_jac_err) | equivalent: the assignment writes the held value | accepted |
| 530 | `while (j <= n)` to `j < n` (sq_cross) | test gap, group A | killed by the new test 1, assert a |
| 533 x2 | `y0 < lvl` to `<=`, `y1 >= lvl` to `y1 > lvl` (sq_cross) | test gap, group A | killed by test 1, asserts b and c |
| 549 | drop `if (t == SC_NONE) { return SC_NONE; }` (sq_t_first) | test gap, group A | killed by test 2, asserts t1 and t2 |
| 555 | `dp > dn` to `dp >= dn` (sq_sign) | test gap, group A | killed by test 2, assert q1 |
| 561 | `while (j <= n)` to `j < n` (sq_t_res) | test gap, group A | killed by test 3, assert a |
| 564 x2 | `y0 < SC_VCM` to `<=`, `y1 >= SC_VCM` to `y1 > SC_VCM` (sq_t_res) | test gap, group A | killed by test 3, asserts b and c |
| 562 x4 | the sample index of y0: `dp[j-1]` to `dp[j]`, to `dp[(j-1)/SC_NN]`; `dn[j-1]` to `dn[j]`, to `dn[(j-1)/SC_NN]` (sq_t_res) | test gap, group B | killed by test 4, the hand-built waveform and the long-run pin |
| 307, 311, 314, 315 | `x + SQ_DV <= VDD` to `<` (307), `y + ...` (311), `vsb + ...` (314), `vsb - SQ_DV` to `vsb + SQ_DV` (315), the one-sided differences of sq_fd at the supply edge | test gap, group C | killed by the 12230 test 5, shown red by hand (run 1 for 307, run 2 for 311, run 3 for 314, run 4 for 315) |
| 348, 351 | `vlo > SQ_OFF` to `>=`, `vhi < SQ_VDD` to `<=` (sq_dev) | test gap, group D (not equivalent: the body slope cb is non-zero at the kink, lab) | killed by the 12230 test 6, shown red by hand (run 1 for 348, run 2 for 351) |
| 369 | `kq = 1 - kind` to `1 + kind` (sq_dev, SQ_M_SWAPQ) | test gap, group E | killed by the 12230 test 7, shown red by hand (run 1) |
| 164 x3 | `SQ_VDD - i * (VDD / SQ_R)` to `+ i *`, `i /`, `VDD * SQ_R` (sq_clk_at) | dead line: line 163 returns for every i >= 1 at SQ_R = 1 | dead but kept (12230): accepted, the line stays |
| 388 | drop `if (!jac) { return; }` (sq_dev) | dead line: only skips work | dead but kept (12230): accepted, the line stays |

Totals: 17 equivalent (15 lines) + 12 test gap of groups A and B (killed, by hand reading) + 7 test gap of groups C, D, E
(killed by 12230, shown red by hand) + 4 dead-line mutants (dead but kept, accepted) = 40. The tool
names no killing test, so "killed by" is a reading of the mutated line against the assert, not a measurement; for the 7
of 12230 it is backed by a hand run of each mutant (below), which is still not the mutation gate.

The 17 hung are all drop-step mutants of loop counters, `x = x + 1` turned into `x = x`, so each loop never ends: sq_charges
411, 416; sq_stamp 427, 433, 439; sq_step 465, 473; sq_run 511, 520, 522; sq_cross 537; sq_t_res 568; sq_node_err 603,
605; sq_jac_err 641, 656, 658. They are accepted by construction.

## The 4 tests added in 12229 (appended after the last original test, so no line of the spec moved)

The diff of the spec in 12229 is one hunk of 119 added lines after line 1231 and removes nothing. Each assert carries a comment
naming the mutant it kills. Expected words are hand arithmetic from the function bodies, except the one lab word of
group B.

- **Group A, sq_cross**, `the_crossing_search_takes_the_last_interval_and_a_sample_on_the_level_ends_a_crossing_but_does_not_start_one`
  (3 runtime asserts). Hand-built `out` of 3 rows of 8 words, all zero but the column SC_DP, level `lvl = SQ_OFF + SC_VCM`
  read into a `var`. (a) rows [lvl-4, lvl+4], n = 1: half way through the only interval, 2048 (half a step with 12
  fractional bits); the mutant `j < n` never enters the loop and says SC_NONE. (b) rows [lvl+5, lvl, lvl+5], n = 2: the
  signal touches the level and goes back up, SC_NONE; the mutant `y0 <= lvl` takes the second interval and says 4096.
  (c) rows [lvl-4, lvl, lvl+4], n = 2: the first interval ends on the level, 4096; the mutant `y1 > lvl` skips it and
  finds the second starting on the level, SC_NONE.
- **Group A, sq_t_first and sq_sign**, `the_decision_time_is_none_when_no_output_reaches_0_75_v_and_the_sign_of_equal_outputs_is_false`
  (7 runtime asserts). `sq_t_first(&out, 100)` on the +2 mV run of 100 steps (160 ps, outputs far below 0.75 V) is SC_NONE,
  and so is the same call on a hand-built waveform of zeros; the dropped guard would give SC_NONE less the half step 2048,
  4294965247. A positive control on a hand-built crossing gives 0. `sq_sign` with dp = dn in the last row is false (the
  mutant `>=` says true); one word more on dp is true, one word less is false. The last assert, on the run's fold
  variable, reads that the run happened.
- **Group A, sq_t_res**, `the_resolution_time_search_takes_the_last_interval_and_a_difference_on_the_level_ends_a_crossing_but_does_not_start_one`
  (3 runtime asserts). The same three cases on |dp - dn| through SC_VCM, with dn = 0 words: dp rows [VCM-8, VCM-4, VCM+4]
  give 4096 (the mutant `j < n` says SC_NONE); [VCM+5, VCM, VCM+5] give SC_NONE (the mutant `y0 <= SC_VCM` says 2048);
  [VCM-4, VCM, VCM+4] give 2048 (the mutant `y1 > SC_VCM` says SC_NONE). The half step 2048 is subtracted from each
  time, so the crossings are placed after it.
- **Group B**, `the_resolution_time_reads_both_outputs_of_the_row_before_the_crossing_and_the_long_run_pins_it_to_the_word`
  (3 runtime asserts). First a hand-built waveform in which dp and dn both move: rows (dp, dn) = (VCM+20, 100),
  (VCM+160, 200), (VCM+440, 400), so |dp - dn| = VCM-80, VCM-40, VCM+40 and the crossing is in the second interval
  (j = 2), where the indices `(j-1)/SC_NN` of the two swap-arith mutants differ from `(j-1)*SC_NN` (at j = 1 they are
  equal). The original gives 4096 (y0 = VCM-40, y1 = VCM+40, frac 2048, 4096 + 2048 - 2048). By hand, the four mutants
  of line 562 give: `dp[j]` reads y0 = VCM+240, not under the level, SC_NONE; `dp[(j-1)/SC_NN]` reads dp0, y0 = VCM-180,
  frac 3351, time 5399; `dn[j]` reads y0 = VCM-240, frac 3510, time 5558; `dn[(j-1)/SC_NN]` reads dn0, y0 = VCM+60, not
  under the level, SC_NONE. All four differ from 4096. Then the long run (the +2 mV input, jf = 1250, SQ_NL steps)
  pins `sq_t_res(&out, SQ_NL)` with `assert_eq` on the lab word below.

## The lab value of group B

`sq_t_res` on the long run is **2664251** (650.452 steps, 1040.72 ps at 1.6 ps a step), read in the lab on the spec
as it stands (the functions untouched, only tests appended). How: the existing window `tr > 1040.6`, `tr < 1040.8` is
the words 2663936 to 2664448; ten temporary tests, not committed, each asserted one bit (0 to 9) of `sq_t_res - 2663936`;
run with `tri test specs/analog/sar_comparator_q.t27` and the old 0.5.2 binary in a T27 OS
pane before the final run, bits 0, 1, 3, 4, 5, 8 passed and bits 2, 6, 7, 9 failed, giving 315, so 2663936 + 315 =
2664251; one temporary test asserted that the word of the first 700 steps equals the word of the full run, and it
passed. The pinned `assert_eq(tl, 2664251)` then passed on the full run (the final run above), which also rules out
any bit above the tenth. The probe tests were removed before the commit.

What the inference in the issue got right and wrong: the crossing is in the interval that ends at step 651
(2666299 words before the half step is taken off = 650 steps + 3899 of 4096), at 0.952 of the interval, against the
inferred 0.94; so frac can rise by at most 0.048 step = 0.077 ps, against the inferred bound of 0.096 ps (the bound
holds, and is tighter). The shifts of the four mutants on the long run were **not measured**: the mutants were not run,
and the 0.04 to 0.1 ps in the issue stays an inference. The hand-built waveform above is what decides all four by
arithmetic, independent of the long run; the long-run pin is the part the issue asked for, and that it moves under
each of the four mutants is expected (a wrong y0 changes frac by whole words out of 4096) but not shown.

## The 3 tests added in 12230 (appended after the tests of 12229, so no line above them moved)

The diff of the spec in 12230 is one hunk of 114 added lines after line 1350 and removes nothing; the three tests are
tests 5, 6 and 7 of the file's 15 (the 12 before them are unchanged). `tri test` runs one test per process and drops
stderr, so a failing assert shows FAIL and no value; the expected values below were therefore read in the lab through
temporary tests (not committed): a relation assert per case for C and D, one assert per bit for the fold of E, brackets
for the rest, then the final tests pinned them and passed (run of 19:44:48, above).

- **Group C, sq_fd**, `the_one_sided_difference_is_forward_up_to_the_word_one_millivolt_under_the_supply_and_the_top_word_reads_the_backward_one_for_both_kinds`
  (25 runtime asserts, a loop over the NMOS and the PMOS kind). The tie of `sq_fd` is the word `w = VDD - SQ_DV =
  25149440`, where the probe point is exactly VDD and the forward difference is the wanted one. For each kind, on each
  axis put the variable on `w` (gate drive x with y = 12582912 and vsb = 4194304; Vds y with x = 16777216; source-bulk
  vsb with x = y = 16777216) and call `sq_fd` directly with `i0 = sq_i(kind, x, y, vsb)`. Asserted: `sq_fd` equals the
  forward form written out (`dpos` of the two currents), forward is not the backward form (the mutants of 307, 311 and
  314 take the backward form at the tie), and forward is positive. Lab: all three hold for both kinds. Then axis 2 at
  `vsb = VDD`, where the probe point is above VDD and the backward difference is the original's: `sq_fd` equals the
  backward form, it is positive, and `sq_i(kind, x, y, VDD + SQ_DV)` equals `i0` because `lim` clamps to VDD, so the
  mutant `vsb + SQ_DV` of 315 reads the same current twice and its difference is 0.
- **Group D, sq_dev**, `the_body_slope_is_left_out_where_the_source_sits_exactly_on_its_rail_word_for_the_nmos_and_the_pmos`
  (6 runtime asserts). Two direct calls of `sq_dev` with the charge off (`SQ_M_NOCHARGE`) and `jac` true, so only the
  Jacobian stamp runs: NMOS 2 with its source on exactly `SQ_OFF`, and PMOS 7 with its source on exactly `SQ_VDD`.
  At that tie the source-bulk voltage is 0, `forward` is set and the body slope `cb` stays 0, so the stamped entry
  (the hi row of the low column for the NMOS, the hi-hi entry for the PMOS) holds, in magnitude, the gate plus drain
  slopes `m * (a + b)` only; the mutants 348 and 351 compute `cb = m * sq_fd(..., 2)` and stamp it as well. Asserted for each device:
  `cb > 0` (the lab shows the body slope is **not** zero at these inputs, so 348 and 351 are genuine test gaps and
  not equivalent mutants), `a + b > 0`, and the register entry equals `m * (a + b)` computed from `sq_fd`. The two
  words are on no run (the low end of the NMOS devices that reach `SQ_OFF` is a rail column, which `sc_add_j` drops),
  hence direct calls.
- **Group E, sq_dev under SQ_M_SWAPQ**, `the_swapped_charge_tables_run_is_pinned_by_its_fold_its_residual_and_its_first_output_time`
  (5 runtime asserts). `sq_run(16777, false, SQ_K, SQ_NS, SQ_M_SWAPQ, 0, 0, ...)`, the control that reads each device's
  charge from the other kind's tables (`kq = 1 - kind`; the mutant `1 + kind` gives `kq = 2` for the PMOS). Pinned by
  three things the lab read: the largest residual `r4`, above 4.2e9 and not above 4.27e9 words; the first output time
  `t4` above 7.8 and below 7.9 ps; and the fold of the run, `0xE704BBC6BF4F491C`. The fold was read one bit at a time
  (64 temporary tests, a pass means the bit is set; 35 bits set, 29 clear); the residual and the time by bracketing.
  A mutated `kq` changes the run, hence the fold, and the pin is exact.

The lab commands were all the same, from the lane directory, in a T27 OS pane: `<build>/target/release/tri test
specs/analog/sar_comparator_q.t27 --t27c <build>/target/release/t27c`, where `<build>` is the master-built tree.
Two probe runs (19:34 and 19:39, with the temporary tests; the first showed that no value reaches the report, hence the
relation and bit asserts) were followed by the final run at 19:44:48-19:44:59.

### The 7 mutants, run by hand

Not the mutation gate (these hand runs came before the gate run of "After, tree of 12230"): one edit of the mutated line in the lane's spec,
`tri test` as above in a T27 OS pane, then the spec restored from the final copy (checked byte-equal afterwards). Each
run takes 7 to 9 s.

| Run | Mutated lines | Time (+07) | Tests red | Others |
| --- | --- | --- | --- | --- |
| 1 | 307 `x + SQ_DV <= VDD` to `<`, 348 `vlo > SQ_OFF` to `>=`, 369 `1 - kind` to `1 + kind`, all three at once | 19:45:35 | 5 (C), 6 (D), 7 (E) | 12 green |
| 2 | 311 `y + SQ_DV <= VDD` to `<`, 351 `vhi < SQ_VDD` to `<=`, both at once | 19:46:13 | 5 (C), 6 (D) | 13 green |
| 3 | 314 `vsb + SQ_DV <= VDD` to `<` | 19:46:53 | 5 (C) | 14 green |
| 4 | 315 `vsb - SQ_DV` to `vsb + SQ_DV` | 19:47:33 | 5 (C) | 14 green |

Runs 3 and 4 are single-line. Runs 1 and 2 put two or three mutants in at once to save runs, so there the red tests are
attributed to a line by the input each test reaches (test 5 calls `sq_fd` only, on the tie words; test 6 calls `sq_dev`
at the rail words, on no run; test 7 is the only SWAPQ run), not by a run per line. The 12 and 13 other tests stayed
green in those runs, so none of the three mutants moved anything else.

## The accepted file

`docs/analog/mutation-accepted-sar_comparator_q.txt` has 34 entries: 15 equivalent, 2 dead but kept, 17 hung. They name
the 38 mutants that the gate should see as not killed on the tree of 12230 (17 equivalent, 4 dead but kept, 17 hung).
The file of 12229 had 41 entries (15 equivalent, 9 pending, 17 hung; 45 mutants); 12230 removed the 7 pending entries
of the test gaps (307, 311, 314, 315, 348, 351, 369), which the new tests kill (a killed mutant behind an entry would
make the gate exit 2 as a stale line), and turned the 2 entries of the dead lines (164, 388) from pending into "dead but
kept" with their reasons. No "pending" remains in the file. The gate on the tree of 12230 printed "38 not killed (21
survived, 17 hung), 38 accepted by the file; 0 accepted line(s) now killed" (measured, see "After, tree of 12230").
The file differs from the issue's "58 entries (17 equivalent, 11 pending, 17 hung, 13 unviable)" in two ways, both
because of how `tri mutate spec --accepted` reads the file (`cli/tri/src/mutate.rs`, `gate_counts`;
`specs/tri/mutate/survivors.t27`):

- An entry is matched per line and kind, not per mutant: line 167 carries three swap-arith survivors and line 164 three,
  and one entry each names all three. So the 17 equivalent mutants are 15 entries and the 4 dead-line mutants are 2.
- The 13 unviable mutants (8 ret-default of functions with no return value, answered `zig: error: not yet implemented`;
  5 swap-arith at lines 499, 500, 502, 503, 544 whose `SQ_OFF - SC_VCM` is -4194304, which u32 cannot hold) are counted
  nowhere by the gate, so they are comment lines with a one-line reason, as in the accepted file of
  `sar_comparator_qi` (whose gate run passed with the same arrangement). The gate run on the tree of 12229 passed with
  the file of 12229 (41 entries), which has the same arrangement, so it holds for this spec too; the file of 12230
  has been through the gate and passed.

## Not done, not claimed

- The tool names no killing test, so "killed by test N, assert x" in the table is a reading of the mutated line
  against the assert; what the gate shows is that the 12 mutants of groups A and B are no longer among the survivors
  (40 survivors before, 28 after, and none of the 12 lines is listed).
- The seal of `AnalogSarComparatorQ` was re-saved by the loop with the master-built compiler after the branch of 12229
  was brought up to master, and again on the tree of 12230 after master was merged into the lane (`t27c seal --save`,
  then `--verify` exit 0, and `tri seals changed` CHECKED); `tri coverage --strict` was not run locally, CI runs it.
- The mutation gate on the tree of 12230 passed (above). For its 7 test-gap mutants the evidence is the gate (they are
  no longer among the survivors), the `tri test` run of the tree (15 of 15 pass, 0 vacuous) and four hand runs that
  each turned the mutated line(s) red.
- The census was not repeated with the master-built compiler on the unmodified spec (FR-001); the before-counts are the
  census's, the after-counts are the gate's.
- The 17 equivalent survivors are classified by reading the arithmetic and the helpers (`sc_pack(0, .)` is 0,
  `sc_sadd(q, 0)` is q), not proved equivalent by a tool; the 17 hung are classified by construction (the tool reports
  each as outliving `--timeout`, which is consistent with an endless loop but was not shown by a run that was let to
  finish).
- The 4 dead-line mutants (164 x3, 388) are kept, with the spec lines untouched and the reason "dead but kept" in the
  accepted file. Line 388 (`if (!jac) { return; }`) is in fact testable by a direct `sq_dev` call with `jac` false that
  reads `reg` afterwards, and the test was not added: the brief for 12230 kept the line as dead and the entry says so.
  Line 164 needs a second `SQ_R`, which the spec does not have.
- Group B's shifts under the four mutants were not measured on the long run (above).
- Only `specs/analog/sar_comparator_q.t27` was touched among the specs.
