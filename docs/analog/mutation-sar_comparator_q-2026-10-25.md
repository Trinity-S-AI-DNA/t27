# Mutation census of specs/analog/sar_comparator_q.t27, 2026-10-25

Issue #12229, part of epic #10486 (weak spots of the analog v0.1 release). This lane wrote the missing asserts of
groups A and B and the accepted file; groups C, D, E and the dead lines are left to the second issue #12230. The
mutation gate was not run on this tree and the seal was not re-saved: both are the orchestrator's next steps. Where each
number comes from:

- **Before** (336 mutants, 266 killed, 40 survived, 17 hung, 13 unviable) is the analog-loop census of the unmodified
  spec, 10:25-10:39 +07, exit 0, 14 min 13 s, `tri mutate spec` 3 mutants at a time with zig `-j2` each, t27c 0.5.2
  ("build 0 commit unknown"), tree at 035df5ce60. Kinds: drop-guard 20, flip-cmp 66, swap-logic 3, drop-step 34,
  swap-arith 181, ret-default 32. The spec in this lane's tree is byte-identical to master's, and the 40 survivor lines
  below were re-read against it in this lane; no mutant was re-run, and the first step of FR-001 (one gate run on the
  unmodified spec with the master-built t27c) was not done, so there is no second count to set beside the census.
- **After** is a forecast, not a measurement: if the 12 mutants of groups A and B are killed, 278 killed, 28 survived
  (17 equivalent, 11 pending for #12230), 17 hung, 13 unviable (278 + 28 + 17 + 13 = 336), and the gate sees 45 not
  killed mutants against 41 accepted entries (15 + 9 + 17) that name all 45.
- **Tests**: `tri test specs/analog/sar_comparator_q.t27 --t27c /Users/playra/t27-build/target/release/t27c` in a T27 OS
  pane, last run 11:46 +07 (after the final comment edit; the 11:41 run before it read the same): 12 tests (8 before), 12 pass, 0 fail, **0 vacuous passes of 12**, exit 0.
  The same t27c prints the same version text as the census's ("t27c 0.5.2", "build 0 commit unknown"); whether the build
  is the same cannot be told from that text.

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
| 307, 311, 314, 315 | the one-sided differences of sq_fd at the supply edge | test gap, group C | pending: #12230 |
| 348, 351 | `vlo > SQ_OFF` to `>=`, `vhi < SQ_VDD` to `<=` (sq_dev) | test gap, group D | pending: #12230 |
| 369 | `kq = 1 - kind` to `1 + kind` (sq_dev, SQ_M_SWAPQ) | test gap, group E | pending: #12230 |
| 164 x3 | `SQ_VDD - i * (VDD / SQ_R)` to `+ i *`, `i /`, `VDD * SQ_R` (sq_clk_at) | dead line: line 163 returns for every i >= 1 at SQ_R = 1 | pending: #12230 (keep or delete) |
| 388 | drop `if (!jac) { return; }` (sq_dev) | dead line: only skips work | pending: #12230 (keep or delete) |

Totals: 17 equivalent (15 lines) + 12 test gap of groups A and B (killed, by hand reading) + 7 test gap of groups C, D, E
(pending) + 4 dead-line mutants (pending) = 40. Of the 11 pending, 7 are test gaps and 4 are the dead lines. The tool
names no killing test, so "killed by" is a reading of the mutated line against the assert, not a measurement.

The 17 hung are all drop-step mutants of loop counters, `x = x + 1` turned into `x = x`, so each loop never ends: sq_charges
411, 416; sq_stamp 427, 433, 439; sq_step 465, 473; sq_run 511, 520, 522; sq_cross 537; sq_t_res 568; sq_node_err 603,
605; sq_jac_err 641, 656, 658. They are accepted by construction.

## The 4 tests added (appended after the last original test, so no line of the spec moved)

The diff of the spec is one hunk of 119 added lines after line 1231 and removes nothing. Each assert carries a comment
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
run with `tri test specs/analog/sar_comparator_q.t27 --t27c /Users/playra/t27-build/target/release/t27c` in a T27 OS
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

## The accepted file

`docs/analog/mutation-accepted-sar_comparator_q.txt` has 41 entries: 15 equivalent, 9 pending, 17 hung. They name the 45
mutants that the gate will see as not killed once groups A and B are killed (17 equivalent, 11 pending, 17 hung). This
differs from the issue's "58 entries (17 equivalent, 11 pending, 17 hung, 13 unviable)" in two ways, both because of how
`tri mutate spec --accepted` reads the file (`cli/tri/src/mutate.rs`, `gate_counts`; `specs/tri/mutate/survivors.t27`):

- An entry is matched per line and kind, not per mutant: line 167 carries three swap-arith survivors and line 164 three,
  and one entry each names all three. So the 17 equivalent mutants are 15 entries and the 11 pending are 9.
- The 13 unviable mutants (8 ret-default of functions with no return value, answered `zig: error: not yet implemented`;
  5 swap-arith at lines 499, 500, 502, 503, 544 whose `SQ_OFF - SC_VCM` is -4194304, which u32 cannot hold) are counted
  nowhere by the gate, so they are comment lines with a one-line reason, as in the accepted file of
  `sar_comparator_qi` (whose gate run passed with the same arrangement). An entry for them could only matter in one
  way: the gate exits 2 on an accepted line whose viable mutants are all killed, and this lane did not run the gate to
  see which of those lines carry a viable mutant of the same kind. In `sar_comparator_qi` four of the five did.

## Not done, not claimed

- The mutation gate was not run (`tri mutate spec --file specs/analog/sar_comparator_q.t27 --accepted
  docs/analog/mutation-accepted-sar_comparator_q.txt --t27c <master-built t27c> --jobs 3 --timeout 30`), so SC-001 and
  the after-counts are unmeasured; the 12 kills are shown by reading each mutated line against its assert. If the gate
  says a mutant of group A or B survived, the assert is wrong and is to be fixed from the function body, not from the
  mutant.
- The seal of `AnalogSarComparatorQ` was not re-saved and `.trinity/seals/` was not touched (SC-004 is the
  orchestrator's). `tri coverage --strict` was not run locally; CI runs it.
- The census was not repeated with the master-built t27c (FR-001); the survivor lines were re-read against the spec in
  this tree and the spec is byte-identical to master's, but the counts are the census's.
- The 17 equivalent survivors are classified by reading the arithmetic and the helpers (`sc_pack(0, .)` is 0,
  `sc_sadd(q, 0)` is q), not proved equivalent by a tool; the 17 hung are classified by construction (the tool reports
  each as outliving `--timeout`, which is consistent with an endless loop but was not shown by a run that was let to
  finish).
- The 7 survivors of groups C, D, E and the 4 dead-line mutants are untouched and listed as "pending: #12230"; the
  group C, D and E figures in the accepted file are hand arithmetic from the census classification and were not run.
- Group B's shifts under the four mutants were not measured on the long run (above).
- Only `specs/analog/sar_comparator_q.t27` was touched among the specs.
