# Mutation test gaps of specs/analog/sar_kick_net.t27, 2026-10-25

Issue #12460, part of epic #10486 (weak spots of the analog v0.1 release). This lane writes the missing asserts for the
test gaps of the census whose expected values follow from the function bodies by hand arithmetic. **The mutation gate was
run on the committed tree (22:48 to 23:36 +07, head 0022752f05) and the seal is re-saved in the same PR.** The measured
result is in the next section; the per-test "meant to kill" table further down was written before the run, and the run
agrees with it in the totals (33 more killed, 34 fewer survivors, the one predicted hang), not mutant by mutant.

## Where the numbers come from

- **Before** (727 mutants, 591 killed, 82 survived, 39 hung, 15 unviable) is the census of the unmodified spec (2183
  lines) by the analog loop, `tri mutate spec`, 2026-10-25. The census output does not record the `t27c` version or the
  run time, and neither is claimed here. Kinds: drop-guard 33, flip-cmp 123, swap-logic 19, drop-step 54, swap-arith
  432, ret-default 66.
- **After** (measured): `tri mutate spec --file specs/analog/sar_kick_net.t27 --accepted
  docs/analog/mutation-accepted-sar_kick_net.txt --jobs 2 --timeout 45` with the master-built `t27c`, 22:48:31 to
  23:36:06 +07, load 26 at the start, committed clean tree at head 0022752f05, accepted file as it stood before the nine
  deferred entries: **727 mutants, 624 killed (624 by a failing test, 0 at compile time), 48 survived, 40 hung, 15
  unviable**; 88 not killed, 79 accepted mutants matched, 0 accepted lines now killed, **9 not accepted**, exit 2. That
  is 591 -> 624 killed (+33), 82 -> 48 survived (-34) and 39 -> 40 hung (+1, the predicted 770 `k = k`). The nine are
  exactly the nine "lab value first" mutants below. Before the run, `tri test specs/analog/sar_kick_net.t27` gave 30
  tests, 30 pass, 0 fail, 0 vacuous passes (the six new tests execute 21, 5, 3, 10, 2 and 4 runtime asserts). The spec
  change is a pure append: the diff is 161 added lines starting after old line 2183, so no mutation line number has
  moved.
- **The gate with the nine deferred entries has not been re-run** (about 50 minutes on a loaded machine). That it exits 0
  follows from the rule that an accepted entry only filters its own (line, kind) key, and from the nine keys being the
  only unaccepted ones in the measured run; the next gate run on this file confirms it. It is stated here as inferred,
  not measured.

## The 82 survivors by label

| Label | Mutants | What became of them |
| --- | --- | --- |
| equivalent | 35 | 35 listed in `mutation-accepted-sar_kick_net.txt` (26 keys) |
| dead line | 4 | 709 and 710 in `build`, listed in the accepted file as dead lines, kept |
| test gap, group A | 33 | 32 addressed here (see below); 1343 is not done (needs a lab teeth check) |
| test gap, group B | 9 | 2 addressed here (`own_window`, line 1174); 7 need a lab value first and are not done |
| test gap, group C | 1 | 618, not done (lab value first) |

35 + 4 + 33 + 9 + 1 = 82. Of the 43 test gaps, 34 are addressed (32 of group A, 2 of group B): 33 are meant to be
killed by the new asserts, and 1 (770, `k = k`) turns into a hang. The 9 not done are 618, 1047, 1054 `+`, 1055 `+`,
1062, 1203, 1343 and 1469 x2; 34 + 9 = 43.

## The assert that kills each test-gap group

All six tests are appended at the end of the spec (lines 2184 to 2344). Every assert reads a `var`, because an assert on
a constant is folded at compile time and counts as no assert (#12166). Every expected value is hand arithmetic from the
function body and from arrays the test builds; none is a measured or typed-in number.

| Test (spec line) | Mutants it is meant to kill | The assert and why the mutant fails it |
| --- | --- | --- |
| the solve swaps a zero pivot row (2187) | 760 `r / WA`; 764; 766 x2; 767 x2; 768 x4; 769 x2; 770 x2 (14; the `k = k` one as a hang, the `k = k - 1` one as a kill) | A 3 x 3 system built by hand in the 34-stride arrays: `g[0][0] = 1`, `g[1][1] = -2`, `g[1][2] = 1`, `g[2][1] = 1`, `g[2][2] = -3`, `c[1][1] = 2`, `c[2][2] = 3`, n = 3, h = 1.0, bsel = 0. Then A = G + C/h = [[1,0,0],[0,0,1],[0,1,0]], and at column 1 the pivot is row 2, so the swap runs (the pinned 34-node network never enters it). After `solve_mn` the A part is the identity, the C part is zero except `aug[1*69+5] = 3.0` and `aug[2*69+4] = 2.0`, and the b column is [1, 1, 0]. A mutant that does not swap leaves a zero pivot (NaN or inf through the rest), one that swaps the wrong cells or not the b column puts a row in the wrong place, and `k = k` loops forever |
| the solve residual (2231) | 796 `ret-default`; 801; 806; 807 `col != n`, outer `\|\|`, inner `&&`; 822 `scale >= 0.0`, `\|\|`, `*` (9) | Five cases on the same system, each one entry of `aug` corrupted: S0 unchanged gives 0.0; S1 `aug[0*69+6] = 5.0` gives 4.0 (the b column is checked, against its own scale 1.0); S2 `aug[1*69+5] = 7.0` gives 4/3 within 1e-12 (the scale of C column 2 is 3.0, from row 2, not row 0); S3 `aug[1*69+3] = 1.0` gives 0.0 (C column 0 has scale 0 and is skipped, a division by zero would give inf); S4 `aug[1*69+4] = 8.0`, `aug[1*69+5] = 6.0` gives 4.0 (the largest ratio 8/2 = 4 is kept; the `*` mutant gives 16) |
| clampf (2283) | 495 `drop-guard` | `clampf(-1.0, 0.0, 1.0)` is 0.0 (the mutant falls through to -1.0), `clampf(0.5, ..)` is 0.5, `clampf(2.0, ..)` is 1.0 |
| row_of (2296) | 1991 `ret-default`; 1992, 1993, 1994 drop-guard and `i == k` to `!=` (7) | `row_of(0)`, `row_of(1)`, `row_of(2)`, `row_of(3)` equal the names V_BASE, V_PLATES, V_NPLATE, V_PPLATE, and the four are pairwise different, so a constant row or a mis-dispatched index fails |
| sample_floor (2321) | 1450 `ret-default` | `sample_floor(T01_K)` is 15 and `sample_floor(T075_K)` is 53 (15.069 and 53.18 cut to the sample at or before) |
| own_window (2332) | 1174 drop-guard and `3 * cv` to `3 / cv` (2) | with cv = 2 and m = 0 the slot is 6 (the drop-guard gives 8, `3 / cv` gives 1); cv = 1 gives 3 at mode 0, 4 at mode 9 and 5 at mode 10 (a mutant that works for cv = 1 only is caught by cv = 2) |

The 770 `k = k - 1` mutant makes the usize counter underflow; it is read as killed (a trap in a safe build, or failing
asserts in a wrapping one). The 770 `k = k` mutant becomes a hang once the swap runs; it is listed as a hang in the
accepted file. The hang count rose from 39 to 40 in the run, which fits; the census output does not name the mutant, so
that it is exactly the 770 one is read from the count, and the `k = k - 1` behaviour (trap or failing asserts) is derived
from the loop, not observed.

## What the accepted file lists

`docs/analog/mutation-accepted-sar_kick_net.txt` has 77 entry lines: 26 equivalent keys (35 mutants), 2 dead-line keys
(709 and 710, 4 mutants), 40 hung keys (the 39 measured plus the measured 770), and 9 deferred test-gap keys (the nine
mutants of "Not done" below, one each, issue 12494). The 15 unviable mutants (every
`ret-default` of a function with no return value, `zig: error: not yet implemented`) are named in comment lines, as the
gate counts them nowhere. Three keys share a line with a mutant that the new tests kill and stay valid because an
equivalent mutant of the same key stays alive: 495 `flip-cmp` (the `drop-guard` is killed), 807 `flip-cmp` (`col != n` is
killed, `i != 0` and `i != 2` live) and 822 `flip-cmp` (`scale >= 0.0` is killed, `>= worst` lives).

## Not done

- **Lab value first** (as the issue says, and none was invented): 618 (`moved_g(dir, 1)` mean, needs the two table
  entries), 1047 (the step `h` in `quant_err_v`, needs the window around the true figure), 1054 and 1055 `[swap-arith]`
  (where the largest difference sits), 1062 (`base_residual` above 1e-12, needs its value), 1203 (`q0`), 1469 x2
  (`cgg_at` at 19, 20 and 21). These eight mutants stay survivors (as does 1343 below). They were the 9 NOT ACCEPTED lines
  of the measured run; this PR lists them in the accepted file under "Deferred test gaps" so that the gate reports the
  rest of the file, and follows them in **issue 12494**. They are open items, not understood ones: the label is
  "deferred", not "equivalent".
- **1343**: the assert that `dc_insitu(i, 9)` and `dc_insitu(i, 10)` equal their own table expressions has teeth only if
  the lab shows the two expressions differ for at least one i; not written.
- **The swap block as test gap or dead line**: the judgement call is the owner's. The default taken here stands: it is a
  test gap, and the 3 x 3 test above kills it.
- **Owner decisions, not taken**: whether 709 and 710 are removed, kept or driven by a mixed-mode `kick_with` test with
  lab values (this would also settle 654 and 689); whether 1014 and 1265 are pinned by a tight window or stay equivalent
  to within the test windows.
- **Per-mutant kill claims are not individually measured.** The run gives totals (+33 killed, -34 survived, +1 hung) and
  0 stale accepted lines, which fits the table above; it does not print which asserts killed which mutant, so the
  mapping in the table is read off the mutated lines. The gate with the nine deferred entries is inferred, not re-run
  (see "Where the numbers come from").
