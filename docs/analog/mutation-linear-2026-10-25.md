# Mutation test gaps of specs/analog/linear.t27, 2026-10-25

Issue #12275, part of epic #10486 (weak spots of the analog v0.1 release). This lane wrote the missing asserts, the
accepted file and this note. It did not run the mutation gate and did not save a seal: both are the orchestrator's. The
orchestrator ran the gate on this tree afterwards; the result is in the section "Gate result". Where each number comes
from:

- **Census** (420 mutants, 316 killed, 61 survived, 38 hung, 5 unviable) is the analog-loop census of the unmodified
  spec on 2026-10-25, tree 1c783b0b31, a master-built compiler, run by `tri mutate spec`. The 61 survivors were
  re-read against the spec text after that census; every one of them is a survivor classified earlier, and no verdict
  changed. No mutant was re-run in this lane.
- **Tests** are measured: `tri test specs/analog/linear.t27` on this tree (base master f81a02f36a plus the 286 added
  lines) with an older release build of the compiler, which is enough for `tri test`: tests 26, pass 26, FAIL 0, vacuous
  passes 0 of 26. The 14 existing tests are unchanged (145 counted asserts); the 12 new tests count 98 asserts, 243 in
  all.
- **Which assert kills which mutant** is a by-hand reading of the mutated text against the assert, written next to each
  new assert as a comment. The gate measured the total, not this attribution: all 35 test-gap survivors are now killed,
  each by a failing test and none by an invariant (section "Gate result"). Which assert fails on which mutant stays a
  reading.

Kinds among the 61 survivors: flip-cmp 20, swap-arith 33, swap-logic 3, drop-step 4, ret-default 1.

## The 61 survivors, by label

| Label | Count | Lines |
| --- | --- | --- |
| equivalent | 18 | 238, 247, 291, 304, 331 `>=`, 332 `>=`, 376, 393, 461, 536, 572, 574 (2), 575, 578 `<`, 625, 626 `>=`, 686 |
| test gap | 35 | groups A to F below |
| dead line | 7 | 441 (3), 442 (4) |
| undecided | 1 | 463 `>` to `>=` |

18 + 35 + 7 + 1 = 61.

### Equivalent (18), accepted

| Line | Mutant | Why it is equivalent |
| --- | --- | --- |
| 238, 247 | `>=` to `>` in `delta_of`, `add_sm` | at equality both branches give magnitude 0, and `pack(0, s)` is 0 (zero is never negative) |
| 291, 304 | `x < 0.0` to `<=` in `fabs64`; `v < 0.0` to `<=` in `q_sm` | at 0 only the sign of a zero changes (nothing divides or orders by it); `q_sm` packs magnitude 0 either way |
| 331, 332 | `>` to `>=` in `node_count` | at equality `top` is set to the value it already holds |
| 376 | `id <= top` to `<` in `table_ok` | `top` is a node id present in some element, so id = top is always seen |
| 393 | `w + y` to `w - y` in `bump` | the whole work matrix is negated; row c is divided by its own pivot, and the pivot choice and `residual` take magnitudes |
| 461 | `r = c + 1` to `r = c` in `eliminate` | one extra comparison of row c against itself, and strictly greater is false |
| 536, 625, 626, 686 | `>` to `>=` in `residual`, `run` (err), `run` (peak), `norm_inf` | at equality the assignment writes the value already held |
| 572 | `j < n` to `<=` in `ties_clear` | the extra element is M[i + 1][0] or N_0, which the loops check anyway with the same (no) diagonal shift |
| 574 (2) | `i == j` to `!=`; `m - 1.0` to `m + 1.0` in `ties_clear` | a shift by 1.0 is a whole number of 2^-24 steps and a sign change maps the fraction f to 1 - f, so the gap is the same in real arithmetic |
| 575, 578 | `<=` to `<` against `TIE_GAP` in `ties_clear` (578 is the N term) | a gap is a multiple of 2^-54 at the finest, and `TIE_GAP` as f64 is an odd multiple of 2^-66, so a gap never equals it |

### Test gaps (35), killed by the new tests

Every new test starts from a hand-built input; every expected value is hand arithmetic from the function bodies. The
first run on the unmodified spec was the check, and every value held on that first run: **no hand value had to be
corrected.**

| Group | Mutants | New test | The assert that fails on the mutant |
| --- | --- | --- | --- |
| A, stamp shapes no table has (10) | 399, 401, 402 | `a_floating_capacitor_stamps_both_of_its_nodes_in_both_blocks` | the six cells of M and N of V, R, C, R (a = IN, IN, 1, 2) equal 20/41, -20/41, -20/41, 20/41, 21/41, 20/41 to 1e-12, and `mismatches` against the stored words is 0: a lost, moved or doubled cell changes a value, and a singular matrix gives NaN |
| | 408 (2), 409 | `three_independent_filters_put_the_source_column_on_every_row` | with R(2, IN) and R(IN, 3) the source column lands on rows 1 and 2; M = diag(20/21) and N = 1/21 x3 to 1e-12 fail when it lands elsewhere (R(1, IN) alone would not kill the `/ WIDE` mutants: 0 / WIDE equals 0 * WIDE) |
| | 429, 443, 432, 433 | `two_inductors_on_one_node_get_one_row_each` | 20 cells of M and N of V, R, R, L, L to 1e-12: the second inductor on row 1 (429), one row each (443: both on one row gives a zero pivot, NaN), and the inductor with a = node 2 so that `ix` is 1 (432, 433; the RLC has `ix` 0) |
| B, eliminate beyond stage 0 (6) | 463 `r * WIDE - c`, 469, 470 | `a_swap_at_stage_1_exchanges_whole_rows` | a swap at stage 1 (rows 1 and 2): M = A and N = (5, 11, 7), bit for bit; the mutants read a zero column or copy a wrong row, and a zero pivot follows |
| | 468 | `a_swap_at_stage_0_leaves_the_cells_below_in_place` | N = (1, 1, 1) to 1e-12; the extra swapped cell exchanges (1, 0) and (2, 0) and gives (1/2, 5/2, 5/2) |
| | 463 `p / WIDE + c` | `the_pivot_search_compares_a_candidate_with_the_current_pivot_row` | with e = 2^-60, N = (-1, 2, 5) bit for bit; the mutant compares with row 0 and gives (0, 2, 5) |
| | 463 `p * WIDE - c` | `the_pivot_search_reads_the_pivot_cell_of_the_stage_column` | with e = 2^-60, N = (5, -1, 2) bit for bit; the mutant gives (5, 0, 2) |
| C, `tie_gap` and `ties_clear` never see a tie (6) | 309, 310, 311 | `the_tie_gap_of_an_exact_tie_is_zero_and_ties_clear_sees_a_tie_in_m_or_n` | `tie_gap(2.5 / SCALE)` is exactly 0.0 (x = 2.5, f = 0.5) and `tie_gap(0.0)` is 0.5; the mutants give about 0.5, 4.0 and 1.0 for the first |
| | 573, 578 (2) | same | `ties_clear(2, mn)` is false with only mn[2] = 2^-25 (a tie at M[1][0]; 573 reads mn[0] there) and false with only mn[5] = 2^-25 (a tie at N_1; `n / n + i` reads mn[2] and `n * n - i` reads mn[3], both zero) |
| D, the checkers need negative controls (5) | 522, 529 | `the_checkers_fail_on_a_wrong_answer_and_the_peak_is_the_largest_state` | `residual` of the solver test's system with N_0 changed from 7 to 8 equals 1.0 exactly (row 1 of the B column gives -7 + 8); a body returning 0.0 (522) or skipping the B column (529) gives 0.0 |
| | 556, 559 | same | `mismatches` of the dt = 25 ns derive against the stored RC words is 2 (one D and one N mismatch: 409200 at 2^-24, not 798915); each mutant drops one count and gives 1 |
| | 626 `\|` | same | in the 200-step RC run `peak` is 16649635 (the state after the 100th step) and the last state is 126610; the `\|` mutant takes the last sample as the peak |
| E, refusals (4) | 331 `or`, 332 `or` | `node_count_is_the_highest_free_node_whichever_terminal_holds_it` | `node_count` with a = [2, 1], b = [0, 0] is 2 (331 gives 1), and with a = [0, 0], b = [2, 1] is 2 (332 gives 1) |
| | 364, 366 | `table_ok_refuses_a_source_with_one_wrong_terminal_and_a_zero_value` | `table_ok` is false for the RC table with V written (1, GND) and with V written (IN, 1) (the `and` mutant of 364 needs both terminals wrong), and false with the resistor value 0.0 (the `<` mutant of 366 accepts it) |
| F, `matmul` and `power_norm` on hand matrices (4) | 664 (3) | `matmul_and_power_norm_are_right_on_hand_matrices` | `matmul` of [[1, 2], [3, 4]] with itself is [7, 10, 15, 22]: `acc -` gives -7, `a[i / n + k]` makes row 1 read row 0 (7, not 15), `b[k / n + j]` gives 3 at cell [0][0] (not 7) |
| | 702 | same | `power_norm(1, [0.5], 3)` is 0.00390625 (0.5^8, exact); the `<=` mutant squares a fourth time and gives 1.52587890625e-05 |

Counted asserts per new test (`tri test` counts a loop assert once per pass): 10, 16, 23, 13, 4, 3, 3, 5, 5, 5, 4, 7 in
the order of the table above (the 12 tests of groups A1, A2, A3, B1, B2, B3, B4, C, D, E (two tests), F). Every table
element and every expected word is read into a `var` first, so no assert is folded at compile time and none is vacuous.

### Dead lines (7), kept

441 (3 mutants) and 442 (4) are the `NODE_IN` terminals of an inductor in `stamp`. They are reached only for an inductor
with a terminal on `NODE_IN`, which `table_ok` refuses at line 368 (a C or L may not touch `NODE_IN`), and no test calls
`stamp` or `derive` on such a table. The default is **keep** and list them in the accepted file as dead lines; deleting
them would move every line below and re-baseline the file, so that is a separate change on the owner's word. If the
owner prefers to keep and pin them instead, the issue holds the hand values (V, L, R; ZS 1.0; dt 1.0; values 0, 1, 1;
one `derive` call per orientation, asserted to 1e-12); that is not part of this change.

### Undecided (1)

463 `>` to `>=` in the pivot search. A tie of two candidates changes which row pivots, and the difference could only be
rounding. The expected value is not derived, so **no test was written**. A lab run of `eliminate` on a tied non-dyadic
column decides it: any differing bit makes it a test gap, none makes it equivalent. It is listed in the accepted file as
`pending: #12275`.

### Hung (38) and unviable (5)

The 38 hung mutants are all `x = x + 1` turned into `x = x` in the counter of the `while` that encloses it, so the loop
never ends and no test can go red; they are accepted by construction (lines 269, 281, 333, 344, 372, 381, 384, 445,
464, 472, 479, 488, 491, 493, 507, 510, 534, 537, 539, 557, 560, 576, 579, 602, 614, 617, 627, 629, 646, 648, 665, 668,
670, 684, 687, 700, 707, 709). The `l = l + 1` of line 443 and the `r = c + 1` of line 461 are not loop counters of a
`while` and are survivors, handled above.

The 5 unviable mutants are the `ret-default` mutant of a function with no return value (body emptied), which zig
answers with "not yet implemented": `step` 277, `bump` 392, `stamp_pair` 397, `stamp_src` 407, `matmul` 654. That the
cause is on the emitter side and not in this spec is not established here.

## The accepted file

`docs/analog/mutation-accepted-linear.txt` has 69 entries: 18 equivalent, 7 dead line (kept), 1 pending, 38 hung, 5
unviable, in the form `path:line [kind] note` that `tri mutate spec --accepted` reads. It has no entry for any of the
35 test-gap mutants. One check was made by comparing lists: the 38 hung lines in the file are exactly the 38 hung lines
of the census.

The 5 unviable entries differ from the other accepted files (`sar_comparator_qi`, `sar_comparator_psp`, `sar_cdac`,
`sar_kick`, `sar_switch_q`), which name unviable mutants only in comment lines. They are written as entries here
because the issue counts them in the 69. Reading `gate_counts` in `cli/tri/src/mutate.rs`: an entry is checked per line
and kind, and it is stale (exit 2, "ACCEPTED BUT KILLED") only when mutants of that line and kind ran and all were
killed; unviable mutants are left out of that test, and each of the five lines carries only its one `ret-default`
mutant, so the five entries are inert today. The gate run confirmed this reading: it passed with the file unchanged, so
the five entries stay (section "Gate result").

## Gate result

Run by the orchestrator on this tree on 2026-10-25, 13:14:15 to 13:27:00 (+07), with the master-built compiler, jobs 3,
timeout 30: `tri mutate spec --file specs/analog/linear.t27 --accepted docs/analog/mutation-accepted-linear.txt`, exit
0, "Gate passed".

- **Mutants:** 420, of which 351 killed (all by a failing test, 0 by an invariant), 26 survived, 38 hung, 5 unviable.
- **Against the census of the unmodified spec** (316 killed, 61 survived, 38 hung, 5 unviable) the added tests turned
  35 survivors into kills: 316 + 35 = 351 and 61 - 35 = 26. The 26 survivors match the 18 equivalent, 7 dead line and 1
  undecided mutants of the tables above (18 + 7 + 1 = 26).
- **Survivor gate:** 64 mutants not killed (26 survived, 38 hung), 64 accepted by the accepted file, 0 accepted lines now
  killed. No survivor is unaccepted and no entry is stale.
- **Unviable entries:** the 5 entries (lines 277, 392, 397, 407, 654, kind `ret-default`) are kept. As predicted from
  reading `gate_counts`, the gate ignores an entry whose mutants were all unviable, and it passed with the file
  unchanged, so the five entries are inert. They become a signal if the emitter later lets one of those mutants run and
  a test kills it: the entry is then stale and the gate fails.

## Caveats (what is by hand or not established)

- **393** rests on IEEE round-to-nearest sign symmetry (negating every cell leaves the elimination bit for bit). That
  is standard but was not run: not established.
- **574** is equivalent in real arithmetic. In f64 an input whose gap lies within about 1e-9 of `TIE_GAP` could flip;
  no table is there, and the width of the band is not established.
- **575 and 578** rest on `TIE_GAP` as f64 being 7378697629483821 x 2^-66, an odd multiple (by hand), and on a gap being
  a multiple of 2^-54 at the finest (f lies in [0.25, 1), where `f - 0.5` is exact).
- **331 and 332 `or`** (group E): the trace is by hand. With `or`, a free terminal sets `top` at every element, so
  `top` ends as the last free terminal, and a trailing `NODE_IN` would set it to 15; in the four right tables the last
  element is the highest node, which is why no earlier test saw it.
- **399-402, 408 `- col`**: the mutated index can be `0 - n` or `0 - col`, which underflows `usize`. Whether the mutant
  build traps or wraps is not established; either way the changed cell or the trap fails the new asserts.
- **441 and 442**: whether any other spec calls `stamp` or `derive` on a table `table_ok` refuses is not established. A
  repo-wide search for callers of `AnalogLinear` was not completed. A caller would move these seven from dead line to
  test gap.
- **The by-hand kill reading** of every group is the per-assert attribution only. The gate measured that all 35
  test-gap survivors are killed, each by a failing test; it did not name the assert. The group D assert on `peak`
  (16649635 and 126610) and the group A and B values passed on the unmodified spec at the first run, so the expected
  values are right; that a given assert is the one that fails on a given mutant is the reading above.

## Not done

- The seal of `AnalogLinear` was re-saved by `tri seals changed` (seal-save and seal-recheck exit 0, importer seals exit
  0 with nothing else changed) and `t27c seal specs/analog/linear.t27 --verify` exits 0; `tri test` on the spec gives
  26/26 with 0 vacuous.
- 463 `>=` has no test (no expected value is derived).
- `tri coverage --strict` and the CI docs check were not run in this lane; the orchestrator runs coverage separately.
- The dead lines 441 and 442 are not removed and not pinned.
