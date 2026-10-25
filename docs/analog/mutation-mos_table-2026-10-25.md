# Mutation test gaps of specs/analog/mos_table.t27, 2026-10-25

Issue #12317, part of epic #10486 (weak spots of the analog v0.1 release). This lane wrote batches 1 and 2 of the missing
tests (T1 to T12), the accepted file and this note. It did not run the mutation gate and did not save a seal: both are the
orchestrator's. Group K (48 mutants) and the dead line 835 are left to a later lane (batch 3, tests T13 to T16).
Where each number comes from:

- **Census** (632 mutants, 493 killed, 115 survived, 20 hung, 4 unviable) is the analog-loop census of the unmodified
  spec on 2026-10-25 (12:27 to 12:55 +07, 27 min 23 s), on a tree byte-identical to master 36c8d6a54f, run by
  `tri mutate spec`. The kinds among the 632: drop-guard 22, flip-cmp 64, swap-logic 5, drop-step 64, swap-arith 406,
  ret-default 71. The 115 survivors were classified by reading the spec after that census. No census mutant was re-run
  in this lane.
- **Tests** are measured: `tri test specs/analog/mos_table.t27` in a pane on this tree (base master 36c8d6a54f plus 365
  added lines, all after the last line of the old file, so no existing line moved; the diff against master has 0
  deleted lines): tests 33, pass 33, FAIL 0, vacuous passes 0 of 33 (run 14:49:06 to 14:49:13 +07, load average 16.44
  at the start and 20.53 at the end). The 21 existing tests are unchanged (5412 counted asserts); the 12 new tests count
  45 asserts, 5457 in all.
- **Kills** are measured one mutant at a time, by hand and not by the gate: a copy of the spec with the new tests, one
  `sed` edit that approximates the census mutant, `tri test` in a pane, the exit code. Exit 1 with a failing test is a
  kill. 41 mutants were run (the 41 census test gaps of groups A to J): 18 for batch 1 (14:14:12 to 14:17:25 +07, load
  37.35 at the start, 52.93 at the end) and 23 for batch 2 (14:39:49 to 14:41:52, load 29.17 at the start, 42.09 at the
  end). All 41 exited 1, each with the failing test named in the tables below. A drop-guard is the `if` commented out; a
  ret-default is the `return` line of the function replaced by `return 0;` (the census names the line of the function,
  the edit is on its return line); the other kinds change the operator or the operand the census names. These are my
  approximations of the census mutants, not byte copies of what `tri mutate` writes. Which assert inside the failing test
  went red is a reading of the mutated text against the assert, written next to each assert as a comment.
- **Expected values.** T1 to T10 are hand-derived from the function bodies and the table entries before the run; the
  derivation is in the comment above each test. T11 and T12 pin figures read from pane runs ("measured" in their
  comments): the lag of the K = 1 run in T11 and the slope words of T12.

## The 115 survivors, by label

The census labelled 25 equivalent (5 of them of the weaker kind, see below), 89 test gap and 1 dead line.

| Outcome | Mutants | Lines | Entry in the accepted file |
| --- | --- | --- | --- |
| equivalent, accepted | 20 | 196, 214, 384, 512, 563, 565, 569, 616, 623 (2), 631, 632, 639 (2), 640, 785, 829, 830, 879, 880 | yes, 18 lines |
| equivalent, weaker kinds, accepted | 5 | 624, 688, 875, 877, 894 | yes |
| dead line, kept | 1 | 835 | yes |
| test gap, killed by T1 to T12 | 41 | groups A to J below | no |
| test gap, group K, pending | 48 | 908 to 931 | no (batch 3 kills them) |

20 + 5 + 1 + 41 + 48 = 115. The weaker kinds are "below one ulp" (624, 875, 894), "inferred" (688) and "not
constructed" (877): none is proven equivalent, and each could become a test gap if a witness turns up.

### Test gaps killed by the new tests (41)

| Test | Group | Mutants killed (line, with a count where one line holds several) | What fails on the mutant |
| --- | --- | --- | --- |
| T1 `the_vds_factor_and_its_slope_factor_meet_at_one_sixteenth_with_the_series_below_and_the_table_above` | A | 363, 369 (2) | at x24 = 2^20 the table gives F = 1016418 and e^-x = 15760798, the series 1016490 and 15760726; at x24 = 1000 the table-only mutant gives 16776218 against 16776216 |
| T2 `the_rv_blend_reads_both_words_of_its_cell_and_the_current_rounds_its_x_to_the_nearest_unit` | B | 401, 413 | the rv words grow along the table, so a read of the same word twice gives 54000 against 54500; `- 2048` gives 62 against 63 |
| T3 `a_flat_log_table_gives_the_current_in_words_and_the_slope_through_the_tag_word_dispatch` | C | 458, 459 (2), 490, 496 | a flat log table gives id_log 237 (0, 236, 15565312 on the mutants), cur_of 237 and cur_gds 241; without the tag-word dispatch both read the table as a linear one |
| T4 `the_newton_step_clamps_to_the_supply_when_the_move_would_pass_it_and_does_not_when_it_would_not` | E | 569 (2) | the step from 2 cells under the supply with up = 5 cells returns VDD; both mutants return 28311552 |
| T5 `the_law_slope_is_zero_at_zero_drain_voltage_whatever_the_overdrive` | F | 631 (3) | law_gds(0.9, 0.0, 0.35) is 0.0; each mutant returns 5.5e-4 |
| T6 `the_monotone_check_reads_the_row_of_each_node_and_compares_both_neighbours` | G | 716, 717, 718 | three tables, each non-monotone in one place only (ta at node (3, 7) and tb at node (1, 0), both along Vgs; tc at node (0, 5), along Vds), and a ramp as control |
| T7 `monotone_along_compares_the_next_node_of_its_own_axis_and_a_tie_is_not_a_fall` | G | 736 (2), 738, 739 (2) | a ramp, an all-zero table (ties) and a table that rises along Vgs and falls along Vds, each read along both axes |
| T8 `the_runs_with_a_gate_dependent_charge_table_read_the_previous_gate_one_step_back_and_see_the_edge_at_step_200` | H | 764 (2), 847 (2) | a charge table that depends on the gate only: the output is 10485760 at step 199, 10387456 at step 200 and at step 399, in the fixed run and in the f64 run |
| T9 `the_f64_newton_step_adds_the_charge_plane_with_its_sign_at_the_mirrored_node_of_the_previous_and_the_present_gate` | I | 825 (3), 826 (3), 827 (2) | z = 4184078.978 to 0.001 on a plane of charge that rises along both axes |
| T10 `the_f64_newton_step_reads_the_drain_slope_of_the_charge_plane_at_the_mirrored_drain_voltage` | I | 827 (1: `VDD_F - y` to `VDD_F + y`) | z = 4184072.432 to 0.001 on a plane whose slope along Vds grows with the row |
| T11 `the_f64_run_makes_exactly_k_iterations_per_step_so_k_one_stays_far_from_the_converged_run` | J | 850 | the K = 1 run lags the K = 12 run by 898982.651 units at most; the mutant's K = 1 is the K = 2 run, 22752.730 units away |
| T12 `the_log_slope_is_pinned_bit_for_bit_at_zero_and_small_drain_voltage_and_on_a_table_whose_drain_slope_grows_with_the_gate` | D | 469, 471, 472, 474 | see "Group D" |

Mutants per test, in order: 3, 2, 5, 2, 3, 3, 5, 4, 8, 1, 1, 4 (41 in all); T9 and T10 together hold the 9 of group I
(825 x3, 826 x3, 827 x3). Counted asserts per new test, in the order of the spec: 3, 2, 4, 2, 2, 4, 5, 7, 1, 1, 6, 8 (45 in all). Every value read
from a table, and every argument, is read into a `var` first, so no assert is folded at compile time and none is vacuous.

Hand-mutation results worth keeping: each of the eight mutants of lines 825 and 826 and of the first two terms of 827
(`+ gdsf` to `- gdsf`, `VDD_F - a` to `VDD_F + a`) turned both T9 and T10 red, and the ninth (`VDD_F - y` to `VDD_F + y`
on line 827) turned only T10 red, which is why T10 exists. Every other mutant turned exactly the one test named in the
table.

### Group D

`gds_log` reads the Vds slope of the log word and subtracts two terms. Three of its four mutants (469, 471, 472) change a
rounding half of an argument from `+ k` to `- k`. At vd = 0 the u64 `(0 - 2048)` and the `E * F - 8388608` subtraction
underflow, and Zig in safe mode traps; the trap is reported as a FAIL of that one test, so T12 kills those three **by a
trap**, not by pinning a value at a rounding boundary. The asserts at vd = 0 (`p0`, `n0`) and at vd = 1000 (`p1`) are the
witnesses: 469 and 472 trap at vd = 0 only, 471 at vd = 0 and at vd = 1000 on `tabp`. Whether the gate's mutant build
traps the same way was not checked, because the gate was not run. The fourth mutant (474, `cx * nd1` to `cx / nd1`) is
killed by a value: on the cross-term table the mutant gives 0, 34 and 63 against 1, 56 and 9155 (measured).

### Equivalent (25), accepted

All 25 rest on reading the code, not on an exhaustive run. The per-line reason is in the note of each entry of the
accepted file, which is the one place for it. The 20 mutants of the first row of the table above all differ from the
original only where the two compared values are equal, and there both sides return or assign the same value. The five of the weaker kind are stated as such in their entries: 624 may differ in the
last bit (the products associate differently), 688, 875 and 877 are inferred from the code and not run, 894 adds a term
of about 1e-22.

### Dead line (1), kept

**835** `ret-default` of `newton_iter_f`: the function has no caller anywhere in `specs/`, so no test can see its result.
A relation test (T16, pending) would call it directly with the T4 inputs in f64 and turn this entry into a stale one.

### Hung (20) and unviable (4)

The 20 hung mutants are all `x = x + 1` turned into `x = x` in a loop counter, so the loop never ends and no test can go
red (lines 584, 601, 652, 654, 667, 669, 702, 704, 719, 721, 741, 743, 770, 786, 852, 855, 881, 884, 897, 902). The
census machine was busy when it started (load averages 7.63, 15.30, 16.53), which may have inflated the count.

The 4 unviable mutants are the `ret-default` mutant of a function with no return value (body emptied), which zig answers
with "not yet implemented": `build_table` 645, `run_tabf_q` 841, `run_tabf` 860, `run_ana` 866. That the cause is on the
emitter side and not in this spec is not established here.

## The accepted file

`docs/analog/mutation-accepted-mos_table.txt` has 48 entries in the form `path:line [kind] note` that
`tri mutate spec --accepted` reads: 18 equivalent lines (20 mutants), 5 of the weaker kind, 1 dead line (kept), 20 hung,
4 unviable. It has no entry for any of the 41 mutants the tests kill, and none for group K. Reading the gate's rule (an
entry is checked per line and kind, and it is stale only when all mutants of that line and kind ran, none was unviable
and all were killed; unviable mutants are counted nowhere), the 4 unviable entries are inert today. Lines 569 and 631
carry an entry for an equivalent flip-cmp mutant while T4 and T5 kill other mutants of the same line; those killed
mutants are of other kinds, or (631) of the same kind as a surviving one, so neither entry is stale.

Predicted gate figures, not measured: 632 mutants, 534 killed (493 + 41), 74 survived (115 - 41: the 25 equivalent, the
dead line and the 48 of group K), 20 hung, 4 unviable; 26 survivors and 20 hung accepted, 0 accepted lines now killed.
The gate would still exit 2 because of the 48 unaccepted survivors of group K, until batch 3 lands; no entry is written
for them, since they are test gaps and not acceptable.

## Caveats (what is by hand or not established)

- **The equivalents** rest on reading the code. 688, 875 and 877 are inferred and not run; 624 and 894 are below one ulp.
- **The hand mutants** are approximations of the census mutants (see "Where each number comes from"). They are not
  `tri mutate`: a mutant the gate would report as unviable or hung would look different, and all 41 here compiled and ran
  to a test report.
- **Group D** (469, 471, 472) is killed by an underflow trap, not by a bit pin; see "Group D".
- **T11 and T12 pin figures of this tree's compiler** to a unit (T11) and bit for bit (T12). A change that moves f64
  results in the last bit would move T11 first; a change in the rounding of the log reader would move T12.
- **T9 and T10** use bounds of 0.001 on a value near 4.18e6. The hand-mutation runs showed that all nine mutants of group
  I turn at least one of the two tests red; by how much each moves z was not recorded.
- **The bound 100000.0 of T11** is a threshold between the measured K = 1 lag (898982.651) and K = 2 lag (22752.730),
  chosen by me.

## Not done

- The mutation gate was not run on this tree (`tri mutate spec --file specs/analog/mos_table.t27 --accepted
  docs/analog/mutation-accepted-mos_table.txt`); the figures above are predicted.
- No seal was saved and `.trinity/seals/` is untouched. Whether the appended tests change the sealed hash was not
  checked here.
- `tri coverage` and the CI docs check were not run in this lane.
- Group K (T13 to T15, 48 mutants of the f64 reader `idf_log` and `cellf_w`, lines 908 to 931) and T16 (the relation
  test for `newton_iter_f`, line 835) are not written.
- The 20 hung lines were not re-run; they are the census's.
