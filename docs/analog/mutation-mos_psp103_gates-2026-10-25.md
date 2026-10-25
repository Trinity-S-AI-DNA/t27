# Mutation census of specs/analog/mos_psp103_gates.t27, 2026-10-25

Issue #12187, part of epic #10486 (weak spots of the analog v0.1 release). The tool, `tri mutate spec`, never edits
the spec. This note covers the census as it was measured, the two survivors that were classed as test gaps, and what
has not been run. Where each number comes from:

- **Census** (94 mutants, 75 killed, 10 survived, 9 hung, 0 unviable) is the run of the issue #12116 census on the
  unmodified spec: lane base master `035df5ce60`, the shared release build of `tri` and `t27c` (`t27c` 0.5.2),
  `--jobs 3 --timeout 30`, 09:43-09:48 +07 (4 min 14 s of wall time), exit 0, load averages 17.00 21.15 30.65 at the
  start. No `--accepted` file: it was a census, not a gate. The 10 survivor lines and the 9 hung lines are read off
  that output.
- **After** is `tri test` on the tree of this change (section 4). The mutation gate was not run on this tree.

## 1. Census

```
tri mutate spec --file specs/analog/mos_psp103_gates.t27 --t27c <master-built t27c> --jobs 3 --timeout 30
```

| Run | Mutants | Killed | Survived | Hung | Unviable |
| --- | --- | --- | --- | --- | --- |
| Census (unmodified spec) | 94 | 75 | 10 | 9 | 0 |

By kind: flip-cmp 20, drop-step 7, swap-arith 55, ret-default 12. All 75 kills are by a failing test, none by an
invariant at compile time.

## 2. The 10 survivors

| Line | Mutant | Class | Why |
| --- | --- | --- | --- |
| 190 | `v < one` to `<=` in `gt_levels` | equivalent | at equality the assignment writes the value `one` already holds |
| 192 | `v > zero` to `>=` in `gt_levels` | equivalent | same reason, for `zero` |
| 195 | `g > gap` to `>=` in `gt_levels` | equivalent | same reason, for `gap` |
| 196 | `0.0 - g > gap` to `>=` in `gt_levels` | equivalent | same reason, `gap` is rewritten with the value it holds |
| 235 | `r > w` to `>=` in `gt_tg_worst` | equivalent | same reason, for `w` |
| 248 | `g < 0.0` to `<=` in `gt_ron_gap` | equivalent | at `g == 0.0` the mutant computes `0.0 - 0.0`, the same value |
| 249 | `g > w` to `>=` in `gt_ron_gap` | equivalent | same reason, for `w` |
| 260 | `refs[k] > w` to `>=` in `gt_ref_max` | equivalent | same reason, for `w` |
| 233 | `k < GT_NV` to `<=` in `gt_tg_worst` | test gap | a 28th point (1.45 V) after the 27 the doc comment names: no test input had its largest resistance there, and no test pinned the point count |
| 294 | `want >= 1e-9` to `>` in `gt_hold_vsb` | equivalent on the pinned data | differs only at a reference word of exactly 1000 pA; none of the 1000 words of either device is 1000 |

The equivalence of the eight running-extremum and absolute-value lines is read off the code, not found by a tool.

## 3. The 9 hung

All nine are endless loops made by the mutant, so no test can go red and only the clock ends them. They are listed
in the accepted file with a reason each:

- Dropped counter steps (`k = k + 1` or `i = i + 1` becomes the same name): lines 197 (`gt_levels`), 236
  (`gt_tg_worst`), 250 (`gt_ron_gap`), 261 (`gt_ref_max`), 295 (`gt_hold_vsb`).
- Bisection loops of `gt_mid` (line 128) and `gt_nand_y` (line 141), `hi - lo > 1` to `hi - lo >= 1` or to
  `hi + lo > 1`: with `>=` the bracket cannot close when the else branch is taken (the midpoint is `lo` and `lo = m`
  moves nothing); with `hi + lo` the condition stays true once the bracket has closed at `lo >= 1`.

## 4. What was added and what it kills

Two tests, appended at the end of the spec (the tool mutates functions, not tests, so the 94 mutants and every
function line number are unchanged):

1. **`the_worst_resistance_is_the_largest_of_exactly_27_points_...`** kills line 233. With the real tables the
   resistance of the transmission gate falls over the last points (ngspice: 2776 ohm at 1.0 V, 1929 ohm at 1.40 V),
   so a 28th point is never the largest and the bound is unseen. A cell with the PMOS table all zero has its NMOS
   alone, and its resistance rises to the end of the sweep. The test builds that cell (`dead`, a zero table), asserts
   `gt_tg_v(26)` and `gt_tg_v(27)` as exact words, asserts the resistance at point 26 is between 10 Mohm and 100 Mohm
   and 1.5 times point 25, and asserts `gt_tg_worst(...) == r26` and `r27 > 3 * r26`. The mutant `k <= GT_NV` also
   reads point 27, so its `w` would be `r27` and `w == r26` fails. A last assert shows why the survivor existed: with
   the real tables the 28th point is below the worst of the 27. 9 runtime asserts counted.
2. **`no_held_out_reference_sits_exactly_at_1_nA_...`** pins the equivalence of line 294. It reads both pinned
   reference arrays into `var`s and counts the words equal to, at or above, and above 1000 pA (equal 0 for both
   devices, at-or-above equals above), the nearest word below and above 1000 (921 and 1038 for the NMOS, 872 and 1025
   for the PMOS), and asserts `gt_hold_vsb(...).n9` equals the count of words at or above 1000 pA for both devices.
   It does not kill the mutant (no input exists that could); it makes the acceptance checkable, because a future
   word of exactly 1000 pA in the data makes this test fail. 10 runtime asserts counted.

Result of `tri test specs/analog/mos_psp103_gates.t27` on this tree: 11 tests, 11 passed, 0 failed, 0 vacuous.

## 5. Decisions

- **Line 233: a test, not an acceptance.** The mutant reads one more point of a sweep whose size the doc comment
  states; an input that decides it exists (a cell whose resistance rises to the end), so a test is the right answer.
- **Line 294: accepted.** No input exists in the pinned data on which the two comparisons differ. Changing the
  function (for example by making the data injectable) would move the lines the other accepted entries key on.
  The acceptance sits on a stated fact about the data, and the second test fails if that fact stops holding.
- **Eight running-extremum lines and nine loop hangs: accepted** with the reasons above.

## 6. Not done

- **The mutation gate was not run** on this tree (`tri mutate spec --file specs/analog/mos_psp103_gates.t27
  --accepted docs/analog/mutation-accepted-mos_psp103_gates.txt`). The accepted file lists 18 lines: 8 equivalent,
  1 equivalent on the pinned data, 9 hung. Line 233 is deliberately not listed; if the new test does not kill it,
  the gate fails on it, and if it listed 233 the gate would fail on an accepted line that is now killed.
- **The kill of line 233 is read by hand**, not measured: the mutated line was checked against the asserts, the
  mutant was not run.
- **The seal was not re-saved** (`.trinity/seals/` untouched); the orchestrator re-saves it with a master-built
  `t27c`.
- **Strict coverage was not re-run** after the tests were added.
- **The equivalences and the hang reasons are classified by reading the code**, not by a tool; the census output
  itself only says that 9 mutants outlived `--timeout 30`.
- **Per-mutant attribution is not measured**: which test kills which of the 75 killed mutants is not recorded.
