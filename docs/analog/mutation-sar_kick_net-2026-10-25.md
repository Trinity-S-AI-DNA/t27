# Mutation test gaps of specs/analog/sar_kick_net.t27, 2026-10-25

Issue #12460, part of epic #10486 (weak spots of the analog v0.1 release). This lane writes the missing asserts for the
test gaps of the census. The first six tests (issue #12460) have expected values that follow from the function bodies by
hand arithmetic; the second six (issue #12494) carry lab values for the nine gaps that were deferred. **The mutation gate
has now been run twice on committed trees: the first run 2026-10-25 22:48 to 23:36 +07 at head 0022752f05, the second run
2026-10-26 01:11 to 01:51 +07 at head 0a308eb7fc with the lab-value tests. The seal is re-saved in the same PR.** Both
measured results are in the next section; the per-test "meant to kill" table further down was written before the first
run, and the first run agrees with it in the totals (33 more killed, 34 fewer survivors, the one expected hang), not
mutant by mutant.

## Where the numbers come from

- **Before** (727 mutants, 591 killed, 82 survived, 39 hung, 15 unviable) is the census of the unmodified spec (2183
  lines) by the analog loop, `tri mutate spec`, 2026-10-25. The census output does not record the `t27c` version or the
  run time, and neither is claimed here. Kinds: drop-guard 33, flip-cmp 123, swap-logic 19, drop-step 54, swap-arith
  432, ret-default 66.
- **After, first run (measured)**: `tri mutate spec --file specs/analog/sar_kick_net.t27 --accepted
  docs/analog/mutation-accepted-sar_kick_net.txt --jobs 2 --timeout 45` with the master-built `t27c`, 22:48:31 to
  23:36:06 +07, load 26 at the start, committed clean tree at head 0022752f05, accepted file as it stood before the nine
  deferred entries: **727 mutants, 624 killed (624 by a failing test, 0 at compile time), 48 survived, 40 hung, 15
  unviable**; 88 not killed, 79 accepted mutants matched, 0 accepted lines now killed, **9 not accepted**, exit 2. That
  is 591 -> 624 killed (+33), 82 -> 48 survived (-34) and 39 -> 40 hung (+1, the 770 `k = k` hang that the table below
  expects). The nine are exactly the nine deferred test gaps (618, 1047, 1054 and 1055 `[swap-arith]`, 1062, 1203,
  1343, 1469 `[drop-step]` and `[swap-arith]`), which the second run below covers. Before the run, `tri test
  specs/analog/sar_kick_net.t27` gave 30 tests, 30 pass, 0 fail, 0 vacuous passes (the six new tests execute 21, 5, 3,
  10, 2 and 4 runtime asserts). The spec change is a pure append: the diff is 161 added lines starting after old line
  2183, so no mutation line number has moved.
- **After, second run (measured)**: the same command (`tri mutate spec --file specs/analog/sar_kick_net.t27 --accepted
  docs/analog/mutation-accepted-sar_kick_net.txt --jobs 2 --timeout 45`) with the master-built compiler, 2026-10-26
  01:11:22 to 01:51:11 +07, clean tree at head 0a308eb7fc, load averages 8.91 10.03 11.34 at the start, the accepted
  file as it stood with its 77 entry lines (68 plus the nine deferred). The head holds the six lab-value tests of commit
  c033ef7e11, appended at the end of the spec after the last line of the first six tests (old line 2344), so no existing
  line moved. **727 mutants, 633 killed (633 by a failing test, 0 at compile time), 39 survived, 40 hung, 15
  unviable**; 79 not killed (39 + 40), all 79 matched by accepted entries, and **9 accepted lines now killed**, which
  are exactly the nine deferred entries: 618, 1047, 1054 and 1055 `[swap-arith]`, 1062 `[ret-default]`, 1203
  `[swap-arith]`, 1343 `[drop-guard]`, 1469 `[drop-step]` and `[swap-arith]`. The gate exits 2 on them: that is its
  "accepted but killed" signal, the instruction to delete those lines. Against the first run: 624 -> 633 killed (+9),
  48 -> 39 survived (-9), hung 40 and unviable 15 unchanged. Against the census: 591 -> 633 killed (+42), 82 -> 39
  survived (-43), 39 -> 40 hung (+1, the 770 hang). The seal re-save on the same spec after the run (01:54 +07, the
  minute is from the seal output file) printed tests 36/36 pass in the seal run (30 tests before, six new, 0 failed), and a
  verify with all five hashes MATCH; the seal file `.trinity/seals/analog_AnalogSarKickNet.json` is re-saved in this PR.
- **The nine deferred entries are deleted, and the file on disk was not gated again.** The second run had them in the
  file and reported them as accepted-but-killed, which is the signal to delete them; they are deleted, so the accepted
  file now has 68 entry lines. The gate was not run on the 68-entry file (about 40 minutes on a loaded machine). That it
  exits 0 follows from the rule that an accepted entry only filters its own (line, kind) key, and from every one of the
  79 not-killed mutants of the second run being matched by an entry that stays. It is **inferred, not measured**.

## The 82 survivors by label

| Label | Mutants | What became of them |
| --- | --- | --- |
| equivalent | 35 | 35 listed in `mutation-accepted-sar_kick_net.txt` (26 keys) |
| dead line | 4 | 709 and 710 in `build`, listed in the accepted file as dead lines, kept |
| test gap, group A | 33 | 32 addressed in the first run (see below); 1343 is now killed in the second run (measured: the 1343 `[drop-guard]` key is one of the nine accepted-but-killed lines) |
| test gap, group B | 9 | 2 addressed in the first run (`own_window`, line 1174); the other 7 (1047, 1054, 1055, 1062, 1203, 1469 x2) are now killed in the second run (measured: their keys are among the nine accepted-but-killed lines) |
| test gap, group C | 1 | 618 is now killed in the second run (measured: its key is one of the nine accepted-but-killed lines) |

35 + 4 + 33 + 9 + 1 = 82. All 43 test gaps are now addressed: 34 in the first run (32 of group A, 2 of group B), of which
33 were killed and 1 (770, `k = k`) turned into a hang, and the last 9 in the second run (1343 of group A, the 7 of group
B, 618 of group C), all nine killed. 34 + 9 = 43. What is left of the 82 is the 35 equivalent and the 4 dead-line
mutants: 35 + 4 = 39, the survivor count of the second run (the run matches them to the accepted file, 79 not killed
with the 40 hung, and does not print a per-mutant listing).

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

### The lab-value asserts of 12494

Six more tests are appended after the first six (commit c033ef7e11, issue 12494; spec lines 2349 to 2413). Their
expected values are not hand arithmetic: each was read on a T27 OS pane from the "runtime asserts executed" column of
`t27c test-report`, where a scratch spec burns one runtime assert per digit of the result, so nothing was computed in
another language. The key each test targets is taken from the comments in the commit; the gate prints the nine killed keys
(see the second run above) but not which test killed which, so the mapping below is the commit's own, backed by a hand
check on scratch copies outside the tree (18 single-operator mutants of the eight lines each failed exactly the intended
new test, and the unmutated copy passed 6 of 6), which is not the gate.

| Test (spec line) | Keys it targets | Lab value asserted |
| --- | --- | --- |
| moved_g, mean of the two off-diagonal entries (2349) | 618 `[swap-arith]` (the offsets, the sum and the halving) | `moved_g(0, 1)` = 0.3835865132499 and `moved_g(1, 1)` = 0.2880668831999 (microsiemens), window 1e-9; dir 1 reads the table entries 10 and 12 |
| quant_err_v, largest word error (2362) | 1047 (the step `/` to `*`), 1054 and 1055 `[swap-arith]` | `quant_err_v()` = 1.24948449226e-6 V, window 1e-9, the same value in lab runs 1 and 2; the step mutant gives 2.862113071454e-6; the largest error is not at the last sample (the N plate peaks at sample 64; at sample 66 the P plate is 1.080961045035e-6 and the N plate 1.196213842563e-6), so a comparison that always holds returns less |
| base_residual, rounding residual (2369) | 1062 `[ret-default]` | lab run 1 gave 2.326878824901e-11, asserted as a band, above 1e-12 and below 1e-10, because it is a rounding residual and not a value to pin |
| pulse_dq_g at the plates gate (2376) | 1203 `[swap-arith]` (`- q0` to `+ q0`) | `dq[0] == 0.0` exactly (q0 - q0), and `dq[66]` = 95.14411245675 fC, window 1e-4, with q0 = 2145.32581609 fC |
| dc_insitu, decision 9 against decision 10 (2386) | 1343 `[drop-guard]` (mode 9 falls through to the mode 10 tables) | conversion 0: 0.00685676 V (decision 9) and 0.00687964 V (decision 10); conversion 7: 0.00711316 V and 0.00714344 V; window 1e-9. The commit states that decision 9 and decision 10 differ at all eight conversions, smallest gap 2.84e-6 V, so 1343 is not an equivalent mutant |
| cgg_frac, interpolation (2404) | 1469 `[drop-step]` (`i + 1` to `i` reads sample 25 twice, 59.20668209839) and its `[swap-arith]` mutants | `cgg_frac(25.5)` = 59.21870498657 fF, the mean of `cgg_at` 25 (59.20668209839) and 26 (59.23072787475); `cgg_frac(24.25)` = 59.19559572601 fF, where the weights 0.75 and 0.25 are not symmetric; window 1e-6 |

## What the accepted file lists

`docs/analog/mutation-accepted-sar_kick_net.txt` has 68 entry lines: 26 equivalent keys (35 mutants), 2 dead-line keys
(709 and 710, 4 mutants) and 40 hung keys (the 39 measured plus the measured 770). It has no deferred test-gap keys any
more: the nine of issue 12494 were killed in the second run and are deleted. The 15 unviable mutants (every
`ret-default` of a function with no return value, `zig: error: not yet implemented`) are named in comment lines, as the
gate counts them nowhere. Three keys share a line with a mutant that the new tests kill and stay valid because an
equivalent mutant of the same key stays alive: 495 `flip-cmp` (the `drop-guard` is killed), 807 `flip-cmp` (`col != n` is
killed, `i != 0` and `i != 2` live) and 822 `flip-cmp` (`scale >= 0.0` is killed, `>= worst` lives).

## Not done

- **The swap block as test gap or dead line**: the judgement call is the owner's. The default taken here stands: it is a
  test gap, and the 3 x 3 test above kills it.
- **Owner decisions, not taken**: whether 709 and 710 are removed, kept or driven by a mixed-mode `kick_with` test with
  lab values (this would also settle 654 and 689); whether 1014 and 1265 are pinned by a tight window or stay equivalent
  to within the test windows.
- **Per-mutant kill claims: the nine are per key, the first 34 are not.** The second run prints which keys were killed
  only for accepted lines, and the nine deferred keys were accepted lines, so those nine kills are measured per key
  (618, 1047, 1054, 1055, 1062, 1203, 1343, 1469 x2). The 33 + 1 of the first run are still totals (+33 killed, -34
  survived, +1 hung, 0 stale accepted lines); that run did not print which asserts killed which mutant, so the mapping
  in the first table is read off the mutated lines. Which new test killed which of the nine is the commit's mapping and
  a hand check on scratch copies, not a gate output.
- **The 1343 teeth check** (do the two table expressions differ for at least one i) is covered by the issue-12494 test:
  the issue comment states that `dc_insitu(i, 9)` and `dc_insitu(i, 10)` differ for all 8 conversions (smallest gap
  2.84e-6 V), and the 1343 key is killed in the second run.
- **The confirmation gate on the 68-entry file is still to come.** Until a gate run on the file as it stands now exits
  0, "exit 0" for that file is inferred from the gate rule, not measured (see "Where the numbers come from").
