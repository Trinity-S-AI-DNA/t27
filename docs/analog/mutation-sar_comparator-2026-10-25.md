# Mutation census of specs/analog/sar_comparator.t27, 2026-10-25

Issue #12432, census 4 of 6 on issue #12241, part of epic #10486 (weak spots of the analog v0.1 release). This lane
wrote the missing asserts of groups A to M and the accepted file. Groups N and O and line 522 need a figure from the
lab: they are deferred to issue #12512 and sit in the accepted file as "deferred test gap" entries. The analog loop ran
the mutation gate on the committed tree and re-saved the seal (see "After"). Where each number comes from:

- **Before** (868 mutants, 659 killed, 98 survived, 78 hung, 33 unviable) is the analog-loop census of the unmodified
  spec, 18:57-19:27 +07, exit 0, 29 min 32 s, load 14.25 15.99 15.42 at the start, t27c 0.5.2. Kinds: drop-guard,
  flip-cmp, swap-logic, drop-step, swap-arith and ret-default. The 98 survivors were read against the spec lines in the
  classification of the census (42 equivalent, 55 test gaps in 15 groups A to O, 0 dead lines, 1 not decided).
- **Tests**: `tri test specs/analog/sar_comparator.t27 --t27c <release t27c>` in a T27 OS pane, 21:39-21:40 +07 (about
  one minute), 1-min load 27.4 at the start: 33 tests (20 before, 13 new), 33 pass, 0 fail, **0 vacuous passes of 33**,
  exit 0. Every new test executed runtime asserts: A 8, B 7, C 2, D 5, E 5, F 8, G 8, H 8, I 2, J 5, K 1, L 3, M 16.
- **After** (the gate on the committed tree, head bde919952c, before the deferred block was added): 868 mutants, 707
  killed, 49 survived, 79 hung, 33 unviable; 128 not killed, 121 accepted by the entries of the file, 0 accepted lines
  now killed, 7 not accepted (six keys, the seven deferred mutants below). `tri mutate spec ... --jobs 2 --timeout 45`
  in a T27 OS pane, 2026-10-25 23:40 to 2026-10-26 00:36 +07 (about 56 min), 1-min load 9.07 at the start (10.83 and
  11.70 for 5 and 15 min), exit 2 because of the 7 not accepted. The six deferred entries were added after that run, so
  the run with them in place was not repeated; exit 0 follows from the run's own list (the 7 are exactly the 6 keys now
  accepted as deferred) and is inferred, not measured. The census went 659 -> 707 killed and 98 -> 49 survivors.
- **Seal**: `AnalogSarComparator` re-saved with the master-built t27c (exit 0, verify 0); only
  `.trinity/seals/analog_AnalogSarComparator.json` changed.

## The 98 survivors, and what became of each

| Lines | Mutant | Verdict | Outcome |
| --- | --- | --- | --- |
| 272, 289, 292, 349, 420, 424, 425 x2, 436, 463, 474, 483, 497, 568, 570 flip, 645, 729, 782 x2, 825, 833, 835, 843, 844, 901, 903, 947, 967, 968, 1070 x3, 1216, 1383, 1420, 1432, 1436 to 1439, 1464, 1492 | ties, passes whose word is overwritten, a candidate compared with itself | equivalent (42 mutants) | accepted, 39 entries (the gate matches per line and kind) |
| 905 swap, 908, 910, 911 x2, 912 x4, 913 x2, 914 x2 | the swap branch of `sc_solve_f` (13 mutants) | test gap, group A | 12 killed by the permutation-matrix test (reading); 914 drop-step turns hung (accepted) |
| 695, 710, 713 x2, 742, 748 | the u32 measurement chain: last row, last interval, level on a sample, no crossing, `sc_after_clock(SC_NONE)`, equal outputs | test gap, group B | killed by the hand-built waveform test (reading) |
| 1473, 1474 x2 | `sc_never_rises` only asserted true | test gap, group C | killed by the last-step rise on node 2 (reading) |
| 445, 451 | `sc_pack(0, true)`, `sc_neg` | test gap, group D | killed by the direct sign-magnitude asserts (reading) |
| 1039, 1053, 1056 x2, 1070 swap-logic | the f64 measurement helpers | test gap, group E | killed by the f64 hand-built waveforms (reading) |
| 1250, 1253 | `sc_state_u` row base and word | test gap, group F | killed by `out[j*8+n] = j*100+n` (reading) |
| 570 swap-arith | upper clamp of `sc_apply_lim` | test gap, group G | killed by v = VDD - 10, x = +100 (reading) |
| 332, 334 | body slope in `sc_stamp_chan_w` | test gap, group H | killed by a = 3, b = 4, cb = 5 (reading) |
| 499, 905 flip | pivot tie in `sc_eliminate`, `sc_solve_f` | test gap, group I | killed by the equal-magnitude rows 0 and 1 (reading) |
| 1212, 1228, 1360 x2, 1371, 1463 | last-step helpers on hand-built arrays | test gap, group J | killed by the five direct asserts (reading) |
| 422, 425 `jd = jp + jn` | `sc_resid_at` read directly | test gap, group K | killed by the residual with a non-zero J- (reading) |
| 389, 391 x2 | `sc_stamp_cap` into a non-empty register | test gap, group L | killed by G+ 1000, J+ 5000 (reading) |
| 544, 933 | back substitution on a dirty x | test gap, group M | killed by x pre-filled with 5 (reading) |
| 1396, 1397, 1404 | `sc_headroom` with swap = true | test gap, group N | **deferred** (issue 12512): needs a lab figure; survived the gate |
| 1291, 1322 x2 | loose windows of the transposed control and `sc_gap_g` | test gap, group O | **deferred** (issue 12512): needs lab windows; survived the gate |
| 522 | `k < 9` to `k <= 9` in `sc_eliminate` | not decided | **deferred** (issue 12512): needs a lab trace; survived the gate |

Totals: 42 equivalent + 49 test gaps of groups A to M + 6 test gaps of groups N and O (deferred) + 1 not decided
(deferred) = 98. The gate run after the tests measured 49 survivors: 42 equivalent plus the 7 deferred mutants, so the 49
test gaps of groups A to M are measured as killed (the survivor count dropped from 98 to 49 and the gate lists no other
line). The tool still names no killing test, so the per-line "killed by" column is a reading of the mutated line against
the assert; each expected word is hand arithmetic from the arrays the test builds, traced against the function bodies,
and the unmutated tree passes all of them.

The 78 hung are all drop-step mutants of loop counters, `x = x + 1` turned into `x = x`, so each loop never ends. They are
accepted by construction. Line 914 (the swap loop counter k of `sc_solve_f`) was a survivor in the census because no
test took the swap branch; the group A test now runs that branch, so the mutant hangs, and the accepted file lists it
with the hung ones (79 hung entries). The 33 unviable are all ret-default (body emptied, `zig: error: not yet
implemented`) and are named in comment lines at the end of the accepted file.

## What each new test asserts

All asserts sit on `var` bindings, because an assert on a constant expression is folded by the compiler and counts for
nothing. The tests are appended after the last existing test, so no line above moves.

- **A** `the_real_solver_swaps_rows_on_a_permutation_matrix`: a(i, 7 - i) = 1.0, b(i) = xt(7 - i) with xt = (100, -200,
  300, -400, 500, -600, 700, -800); `sc_solve_f` must return x = xt exactly.
- **B** `the_u32_measurement_chain_...`: `sc_pick` reads dp = 777 and |dp - dn| = 677 in row 140; `sc_cross` gives 571392
  (crossing in the last interval), 4096 (level on sample 1) and SC_NONE (touch from above); `sc_after_clock(SC_NONE)` is
  SC_NONE; `sc_sign` is false for equal outputs.
- **C** `never_rises_says_false_...`: node 2 is 1 only in step 140, so `sc_never_rises(&out, 2)` is false; node 3 stays
  true. The optional `!sc_never_rises(&out, SC_DP)` on the +2 mV run was not added (it needs a full run).
- **D** `the_sign_magnitude_contract_...`: `sc_pack(0, true)` is 0; `sc_neg` is true for a packed negative 5 and false for
  a positive 5; `sc_mag` gives 5.
- **E** `the_real_measurement_helpers_...`: `sc_pick_f` 777.0; `sc_cross_f` 139.5, 1.0 and -1.0; `sc_t_first_f` 1.5 on a
  dp series that never crosses next to a dn that crosses at 9.5.
- **F** `the_state_of_a_fixed_point_run_...`: `sc_state_u(&out, 12, &v)` gives 1200 + n for n = 0 to 7.
- **G** `the_newton_move_is_held_at_vdd_from_above`: v = VDD - 10, x = +100, so `sc_apply` gives VDD at every node.
- **H** `the_channel_stamp_carries_the_body_slope`: a = 3, b = 4, cb = 5 gives 12 at reg[65], reg[9] (NMOS) and at
  reg2[0], reg2[72] (PMOS), with 4, 4 and 4, 3 at the neighbours.
- **I** `the_pivot_tie_goes_to_the_first_largest_row`: rows 0 and 1 tie in column 0 (opposite signs); word 0 stays CELL
  after `sc_eliminate` and stays 4.0 after `sc_solve_f`.
- **J** `the_last_step_helpers_...`: `sc_mirror_dev_u` 7, `sc_mirror_dev_f` 5.0, `sc_dev_prefix` 9.0, `sc_peak_f` 5.0,
  `sc_peak` 5, each with the only non-zero word in the last step (or word 20 for the prefix).
- **K** `the_residual_reads_a_non_zero_j_minus_diagonal`: `sc_resid_at(&reg, 1)` is 2097152.
- **L** `the_capacitor_stamp_adds_into_a_register_that_is_not_empty`: G+ 1020, G- 8, J+ 2102152.
- **M** `back_substitution_overwrites_a_dirty_solution_vector`: with x pre-filled with 5, every x(i) is 7, fixed-point and
  f64.

## The accepted file

`docs/analog/mutation-accepted-sar_comparator.txt` has 124 entries: 39 equivalent (naming the 42 equivalent mutants), 79
hung and 6 deferred test gaps (issue 12512, naming the 7 deferred mutants; they keep the gate green without claiming
equivalence, as in `mutation-accepted-sar_kick_net.txt`). It is read per line and kind (`cli/tri/src/mutate.rs`, `gate_counts`; `specs/tri/mutate/survivors.t27`), which
has three consequences, each written in the header of the file:

- One entry covers every mutant of its line and kind: 782 (2 mutants) and 1070 flip-cmp (3) are one entry each.
- 425 [swap-arith] covers two census mutants, the equivalent `jp + jn > 1` and the group K mutant `jd = jp + jn`. The
  entry stays valid because the equivalent one still survives; the group K mutant is killed by the new test.
- 570 swap-arith (group G) and 1070 swap-logic (group E) are test gaps on a line that also carries an equivalent
  mutant of another kind; they have no entry, and an entry for them would make the gate exit 2 ("accepted but
  killed") once the new tests kill them.

The 33 unviable mutants are counted nowhere by the gate, so they are comment lines at the end of the file.

## Not done, not claimed

- The gate was run once, on the committed tree before the deferred block was added (see "After"); the run with the
  deferred block in place was not repeated, so exit 0 of the final tree is inferred from that run's list, not measured.
  The census itself was not repeated; the gate run is the only measurement of the new kill counts.
- All seven open mutants are tracked in issue #12512 and listed as "deferred test gap (12512)" entries; each needs a lab
  figure, and none was guessed:
- **Line 522** (`k < 9` to `k <= 9` in the row update of `sc_eliminate`) is not decided: it needs a lab trace of the words at
  the extra pass and, if a matrix with a non-zero column-0 residue exists, an assert; otherwise an equivalent entry.
- **Group N** (1396, 1397, 1404, `sc_headroom` with swap = true): the figure of `sc_headroom(&t, SC_H2, true, SC_K,
  &SC_DV_M)` is a lab measurement and was not guessed. Line 1404 (the last step dropped) goes to the equivalent block if the
  lab finds no stimulus that puts the maximum in step 140.
- **Group O** (1291, 1322 x2): the two-sided windows for the `wrong` figure of the transposed control and the `wg` figure of
  `sc_gap_g` come from lab values and were not guessed. These mutants stay survivors until then.
- The 42 equivalent survivors are classified by reading the arithmetic, not proved equivalent by a tool; the 349 and 825
  reason (symmetric conductance on the same four entries at vd = vs) is an inference from the stamp. The 79 hung are
  classified by construction (the tool reports each as outliving `--timeout`, which is consistent with an endless loop
  but was not shown by a run that was let to finish).
- 914 [drop-step] was a survivor and became a hung mutant once the group A test runs the swap branch; the gate counted
  79 hung against 78 in the census, which is consistent with that reading (the single mutant is not named by the tool).
- Only `specs/analog/sar_comparator.t27` was touched among the specs.
