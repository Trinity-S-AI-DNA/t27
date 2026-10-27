# Mutation test gaps of specs/analog/vacask_compare.t27, 2026-10-25

Issue #12182, part of epic #10486 (weak spots of the analog v0.1 release). The numbers before come from the census of
#12116 (section 3), measured 09:34-09:37 +07, exit 0, with `tri mutate spec --file specs/analog/vacask_compare.t27
--t27c <0.5.2 build> --jobs 3 --timeout 30`. This lane did not re-run the census. The tests below are checked by `tri
test` and by reading each mutant against its assert; the survivor gate was then run on this tree (section "Measured").

## Census (before)

| Run | Mutants | Killed | Survived | Hung | Unviable |
| --- | --- | --- | --- | --- | --- |
| Census of #12116, unmodified spec | 48 | 34 | 8 | 6 | 0 |
| After this change (gate run on this tree, measured) | 48 | 40 | 2 | 6 | 0 |

The measured row equals the 40/2/6/0 forecast made from reading the mutants; no number differs from it. The tests
appended to the spec add no mutants (the tool mutates the functions, not the tests), so the mutant count stays 48. The
line numbers are those of the census text; they hold on this tree too (the lane base rewrites asserts of the test
section only, which is below every mutated line), and the gate run confirms it: the 8 mutants it left unkilled are
the 8 accepted lines.

## The 8 survivors, by label

| Line | Function | Mutant | Label | What happens |
| --- | --- | --- | --- | --- |
| 173 | `vc_maxdiff` | `d > worst` to `>=` | equivalent | at `d == worst` the assignment writes the value already held; accepted |
| 185 | `vc_mindiff` | `d < best` to `<=` | equivalent | same reason; accepted |
| 181 | `vc_mindiff` | `a[0] - b[0]` to `a[0] + b[0]` | missing test | killed by the first new test, cases 1 and 4 |
| 184 | `vc_mindiff` | `a[i] - b[i]` to `a[i] + b[i]` | missing test | killed by the first new test, cases 2 and 3 |
| 193 | `vc_within` | `<= tol` to `< tol` | missing test | killed by the second new test |
| 198 | `vc_win` | `dp > dn` to `>=` | missing test | killed by the third new test |
| 227 | `vc_bracket_hi` | `i < n` to `i <= n` | missing test | killed by the fourth new test |
| 241 | `vc_monotone` | the `return false` guard dropped | missing test | killed by the fifth new test |

Eight survivors: 2 equivalent, 6 missing tests in 5 groups (181 and 184 share one cause).

The 6 hung mutants are drop-step mutants of loop counters (`i = i + 1` becomes `i = i`) at lines 174, 186, 208, 219,
229 and 243: each loop never ends, so no test can go red and only the clock ends the mutant. They are listed in the
accepted file with that reason, by construction, not by a run that was allowed to finish; the gate run saw all six
outlive `--timeout 30`.

## The asserts that kill each group

All five tests are appended after the last original test, on small arrays of exact binary fractions (every expected
value is a difference of two numbers the f64 holds without rounding). Every assert is a counted runtime assert on a
local `var` or a function result; none is on a module constant element.

- **Lines 181 and 184, `vc_mindiff`** (`the_least_difference_is_the_difference_not_the_sum_at_index_zero_and_at_a_later_index`, 5 asserts).
  Before, the only data had a[0] = 0.0 and b[0] = -1.05e-14, with the least difference at index 0 in every call. Case 1,
  a = 3, 10, 7 and b = 1, 4, 1: the least is 2 at index 0; the line-181 mutant starts at 4 and nothing later is below it.
  Case 4, a = 3, 10 and b = -1, 4: the least is 4; the line-181 mutant starts at 2. Case 2, a = 5, 8, 6 and b = 1, 7.5, 1:
  the least is 0.5 at index 1; the line-184 mutant sees sums 15.5 and 7, never below 4, and returns 4. Case 3,
  a = 4, 9, 2.25 and b = 1, 2, 2: the least is 0.25 at index 2; the line-184 mutant returns 3. A fifth assert (n = 1) pins that
  a single entry gives its own difference.
- **Line 193, `vc_within`** (`a_difference_exactly_on_the_tolerance_is_within_it`, 5 asserts). |1.5 - 1.0| = 0.5 against
  tol = 0.5 is true, in both argument orders, and |x - x| = 0 against tol = 0 is true; the mutant `<` returns false on all
  three. Two more asserts (tol 0.25 refuses, tol 0.75 accepts) show the bound is not always true.
- **Line 198, `vc_win`** (`a_tie_between_dp_and_dn_is_not_a_win_for_dp`, 5 asserts). `vc_win(x, x) == 0` with x in a
  variable; the mutant `>=` returns 1. The tie is also pushed through `vc_mismatch`: against flag 0 no mismatch, against flag
  1 one mismatch.
- **Line 227, `vc_bracket_hi`** (`the_upper_end_of_the_flip_bracket_does_not_read_one_past_the_first_n_runs`, 4 asserts).
  d = 30, 20, 10 and dp winning only at index 2: with n = 2 the result is 0; the mutant `i <= n` reads index 2 (a defined
  read, the arrays have 3 entries) and returns 10. With n = 3 the result is 10, and a dp win at index 1 gives 20 for n = 2 and 3.
- **Line 241, `vc_monotone`** (`a_winner_sequence_that_goes_dn_dp_dn_is_not_monotone`, 4 asserts). Winners dn, dp, dn
  must return false; without the guard the function returns true. Control asserts: dn, dp and dn, dp, dp are monotone;
  dp, dn is not.

## What was run

`tri test specs/analog/vacask_compare.t27 --t27c <older build tree>/target/release/t27c` (a pane of the T27 OS, load
6.94 at the start, 10:21 +07), exit 0:

- before this change (the lane base `1941409a64`; the count is from the run of the #12166 audit, not repeated here): 7 tests, 7 pass, 0 fail, 0 vacuous;
- after: 12 tests, 12 pass, 0 fail, `vacuous passes 0 of 12`. The five new tests count 5, 5, 5, 4 and 4 runtime asserts.

## Measured

The survivor gate ran on this tree on 2026-10-25, 10:26:31 to 10:29:16 +07, in a T27 OS pane with a master-built
`t27c` (master base `1c783b0b31`), `--jobs 3 --timeout 30`: 48 mutants, 40 killed (all 40 by a failing test, none by an
invariant), 2 survived, 6 hung, 0 unviable; "8 not killed (2 survived, 6 hung), 8 accepted by
docs/analog/mutation-accepted-vacask_compare.txt; 0 accepted line(s) now killed. Gate passed." So SC-001 and SC-003 are
measured and met. The seal `.trinity/seals/analog_AnalogVacaskCompare.json` was re-saved by `tri seals changed` with the
master-built `t27c` (11:13-11:14 +07); a second run left it unchanged, and no importer's seal moved.

## Not done

- The tool does not name which test kills a mutant (it reports "40 by a failing test"), so the "killed by the Nth new
  test" column above is from reading each mutant against its assert, not from the gate output.
- Strict coverage of `specs/analog/*.t27` (`tri coverage --strict`, SC-004) was not run locally; CI runs it.
- `tri test` for the counts in "What was run" used the older `t27c` of the build tree; the gate and the seal used the
  master-built one.
- The two equivalent survivors are classified by reading the function (the write at equality repeats the value held), not
  proved equivalent by a tool.
- No PR is opened by this lane.
