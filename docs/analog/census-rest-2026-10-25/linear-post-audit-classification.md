# linear and linear_zoh after the audit: survivors and hung mutants, by class

In this file "the old file" and "the old rows" mean the pre-audit classification of the same specs (census on tree 035df5ce60). That file is not in the repo; this one cites its rows by line and kind and states a verdict again only where it reads it a second time.

## linear, post-audit census (run 11:23-11:36 +07, tree 1c783b0b31)

Compared against the pre-audit classification of linear (pre-audit run, tree 035df5ce60, same totals). Both raw outputs (this run and the pre-audit one) and the pre-audit classification file are kept outside the repo. Read-only work: spec text, the two raw outputs and a `git diff` of the two trees; nothing was built or run.

### 1. Comparison: 61 same, 0 differ

- The two raw outputs are byte-identical (`diff` of the two output files prints nothing). So the summary line (420 mutants, 316 killed, 61 survived, 38 hung, 5 unviable), the kind counts (drop-guard 10, flip-cmp 88, swap-logic 14, drop-step 45, swap-arith 229, ret-default 34) and every survivor, hung and unviable entry (line, kind, mutation text) are the same in both runs.
- The tree change is tests only: `git diff 035df5ce60 1c783b0b31 -- specs/analog/linear.t27` is 33 insertions and 13 deletions, all in the test blocks (lines 790-862 of the new file). The asserts on constant table elements (`RC_D[0]`, `RC_NV[0]`, `RLC_D[0]`, `RLC_D[3]`, `RLC_D[6]`, `LAD3_D[...]`, `LAD5_D[...]`) now read `var` copies first (the comment cites #12166). No line of the functions (lines 1-717) changed or moved, so every survivor keeps its line number, and the changed asserts read constants, not code the mutator edits, so killing none of the 61 is what the diff predicts.
- Result: every one of the 61 survivors is the survivor classified in the old file, so every old verdict stands as written and is not retyped here. The old caveat "any row may already be dead on master" is settled for this tree: no row moved.

### 2. Index: survivor -> old row

Rows follow the old file's table (its order). `n` is the number of mutants in the row; the kind is the census kind.

| this run: line [kind] | n | old row, old verdict |
| --- | --- | --- |
| 238, 247 [flip-cmp] | 2 | first row, equivalent |
| 291, 304 [flip-cmp] | 2 | second row, equivalent |
| 331, 332 [flip-cmp] | 2 | `>` to `>=` row, equivalent |
| 376 [flip-cmp] | 1 | equivalent |
| 393 [swap-arith] | 1 | equivalent |
| 461 [drop-step] | 1 | equivalent |
| 536, 625, 626, 686 [flip-cmp] | 4 | equivalent |
| 572 [flip-cmp] | 1 | equivalent |
| 574 [flip-cmp, swap-arith] | 2 | equivalent |
| 575, 578 [flip-cmp] | 2 | equivalent |
| 399, 401, 402 [swap-arith] | 3 | test gap A |
| 408 x2, 409 [swap-arith] | 3 | test gap A |
| 429 [swap-arith], 443 [drop-step] | 2 | test gap A |
| 432, 433 [swap-arith] | 2 | test gap A |
| 463 x3 [swap-arith] | 3 | test gap B |
| 468 [flip-cmp], 469, 470 [swap-arith] | 3 | test gap B |
| 463 [flip-cmp] | 1 | not decided |
| 309, 310, 311 [swap-arith] | 3 | test gap C |
| 573 [swap-arith], 578 x2 [swap-arith] | 3 | test gap C |
| 522 [ret-default], 529 [flip-cmp] | 2 | test gap D |
| 556, 559 [drop-step] | 2 | test gap D |
| 626 [swap-arith] | 1 | test gap D |
| 331, 332 [swap-logic] | 2 | test gap E |
| 364 [swap-logic], 366 [flip-cmp] | 2 | test gap E |
| 664 x3 [swap-arith] | 3 | test gap F |
| 702 [flip-cmp] | 1 | test gap F |
| 441 x3, 442 x4 [swap-arith] | 7 | dead line |

The n column sums to 61. Kinds among the 61: flip-cmp 20, swap-arith 33, swap-logic 3, drop-step 4, ret-default 1.

### 3. Second reading of the old verdicts against the spec text

No verdict changes. Refinements and fine print, each from the spec text unless marked:

- 575, 578 (`<=` to `<`): the old reason says a gap is a multiple of 2^-53. Read again, `f` near 0.5 lies in [0.25, 1), where `f - 0.5` is exact and `f` is a multiple of 2^-54 or coarser, so the gap is a multiple of 2^-54 at the finest. The conclusion stands: `TIE_GAP` as f64 is 7378697629483821 x 2^-66 (2^66 / 10^4 = 7378697629483820.6464, nearest integer 7378697629483821, odd, by hand), and an odd multiple of 2^-66 is not a multiple of 2^-54.
- 574: equivalence holds in real arithmetic (a shift by 1.0 is 2^24 whole LSBs, a sign change maps f to 1 - f); in f64 the two gaps can differ by rounding, so an input whose gap lies within about 1e-9 of `TIE_GAP` could flip. No table is there. Size of the band: not established.
- 393: the argument is IEEE round-to-nearest sign symmetry (every cell of the work matrix is negated, elimination divides row c by its own pivot, magnitudes drive the pivot choice and `residual`). That the symmetry holds for these operations is standard but was not run: not established.
- 331, 332 `or` (swap-logic): with `or`, a trailing `NODE_IN` also sets `top` to 15 (15 > top). In the four right tables the last element is `C(top, GND)`, so the final `top` is that node and the answer is right. In the mis-wired capacitor table of the negative-control test (`wc`, capacitor of node 3 moved to node 2) the 331 mutant ends with `top` 2 instead of 3 (hand trace), which the one-sided `mismatches > 0` does not notice.
- 626 `|` (peak): `(y | MAG) > peak` is true for any `peak` below MAG, so `peak` becomes the last sample. A 100-step RC run does not show it (the last sample is also the largest); the 200-step run does (see D below).
- 399-402, 408 `- col`: the mutated index can be `0 - n` or `0 - col`, which underflows `usize`. Whether the mutant build traps or wraps: not established; either way the changed cell or the trap fails the proposed asserts.
- 441, 442: unreachable for any table `table_ok` accepts (line 368 refuses an L with a terminal on `NODE_IN`), reachable by a direct `stamp` or `derive` call on a refused table. Verdict stays dead line (decision: delete, or lift the refusal). If the owner prefers to keep them, the hand-derived checks are in section 4.
- 463 `>=` (pivot tie): still not decided; a tie changes which row pivots, and the difference could only be rounding. Expected value not derived.

### 4. Hand-derived inputs for the six groups (the old file gave shapes; these are values)

All arithmetic is by hand, not run. Bind literal inputs to `var`s before asserting, as the audit did for table elements (#12166), so the compiler cannot fold the assert. A non-finite result (NaN, inf) makes a `<` assert false, so a mutant that divides by a zero pivot fails any of these.

**A: stamp shapes no table has** (kills 399, 401, 402, 408 x2, 409, 429, 432, 433, 443). Assert `table_ok` is true first.

- A1, floating capacitor: kinds V, R, C, R; a = IN, IN, 1, 2; b = GND, 1, 2, GND; values 0, 1000, 1e-9, 1000; dt 5e-8; ZS 1.0. With g = 0.001 and c = 1e-9 / 5e-8 = 0.02: A = [[g+c, -c], [-c, g+c]], C/dt = [[c, -c], [-c, c]], B = (g, 0); det = g (g + 2c). M = (c / (g + 2c)) [[1, -1], [-1, 1]] = (20/41) [[1, -1], [-1, 1]], N = ((g + c) / (g + 2c), c / (g + 2c)) = (21/41, 20/41). At 2^-24: 21/41 = 8593208.195, 20/41 = 8184007.805, both 0.305 from a tie. So `mismatches(2, ...)` is 0 against D = [NEG|8593208, NEG|8184008, NEG|8184008, NEG|8593208] and N = [8593208, 8184008]. Kills 399 (second `stamp_pair` call, cell (1,3) becomes (0,10)), 402 (cell (1,2) becomes (0,9)), 401 (`0 - 2 + 1` underflows).
- A2, three independent filters, to reach `stamp_src` at an index above 0: kinds V, R, R, R, C, C, C; a = IN, IN, 2, IN, 1, 2, 3; b = GND, 1, IN, 3, GND, GND, GND; values 0, 1000, 1000, 1000, 1e-9 x3; dt 5e-8; ZS 1.0 (n = 3). Each node is M0's RC with the same bump order (R then C), so M = diag(20/21), N = (1/21, 1/21, 1/21): `mismatches(3, ...)` is 0 against D = diag(NEG|798915) with zero off-diagonals and N = 798915 x3 (the words of `RC_D`, `RC_NV`). Kills 408 x2 (R(2, IN) puts B_1 at cell 17; `/ WIDE` lands on cell 6, `- col` on cell 5) and 409 (R(IN, 3) puts B_2 at cell 28; `/ WIDE` lands on cell 6). A resistor written (1, IN) alone would not kill the `/ WIDE` mutants (0 / WIDE = 0 * WIDE).
- A3, two inductors from node 2: kinds V, R, R, L, L; a = IN, IN, 1, 2, 2; b = GND, 1, 2, GND, GND; values 0, 1, 1, 1, 1; dt 1.0; ZS 1.0 (n = 4, unknowns v1, v2, xa, xb). Equations: 2 v1' - v2' = u; -v1' + v2' + xa' + xb' = 0; xa' = xa + v2'; xb' = xb + v2'. Solved: v2' = (u - 2xa - 2xb)/5, v1' = (3u - xa - xb)/5, xa' = (3xa - 2xb + u)/5, xb' = (-2xa + 3xb + u)/5. So M = rows [0, 0, -1/5, -1/5], [0, 0, -2/5, -2/5], [0, 0, 3/5, -2/5], [0, 0, -2/5, 3/5] and N = (3/5, 1/5, 1/5, 1/5); check A N = B: 2(3/5) - 1/5 = 1, then 0, 0, 0; DC point v1 = u/2, v2 = 0. Assert each `derive` entry to 1e-12. Kills 429 (second inductor gets ki = nn - 1), 443 (both inductors on one row), 432 and 433 (inductor a = node 2, so ix = 1; the RLC has ix 0).

**B: eliminate beyond stage 0** (kills 463 x3, 468, 469, 470). Build the work matrix by cell: A in columns 0-2, C/dt in 3-5, B in 6 (row stride 11).

- B1, swap at stage 1: A = [[1,0,0], [0,0,1], [0,1,0]], C/dt = I, B = (5, 7, 11). Trace: stage 0 no swap, stage 1 swaps rows 1 and 2 (|1| > |0|), all operations exact. Expected M = A, N = (5, 11, 7). Kills 463 `r * WIDE - c` (reads cell 21, an unused zero column, so no swap and a zero pivot), 469 and 470 (`c / WIDE` = 0 at c = 1: a wrong row is copied or overwritten, a zero pivot follows).
- B2, swap at stage 0 with nonzero cells below: A = [[1,0,0], [3,1,0], [2,0,1]], C/dt = 0, B = (1, 4, 3). Expected N = (1, 1, 1) to 1e-12 (x0 = 1, x1 = 4 - 3 = 1, x2 = 3 - 2 = 1). Kills 468: the extra swap exchanges cells (1,0) and (2,0) after the row swap, the multipliers become 2 and 1 instead of 1 and 2, and the result is (1/2, 5/2, 5/2).
- B3, pivot search compares against row 0 (463 `p / WIDE + c`): A = [[0,0,1], [1,1,0], [e,1,0]] with e = 2^-60, C/dt = 0, B = (5, 1, 2). Original: p = 1, swap rows 0 and 1, row 2 becomes [0, 1 - e = 1, 0 | 2 - e = 2] in f64, stage 1 swaps rows 1 and 2, row 0 becomes [1, 0, 0 | 1 - 2 = -1]: N = (-1, 2, 5) bit for bit. Mutant: r = 2 compares |e| with |w[0]| = 0 and picks pivot e: row 0 / e = [1, 2^60, 0 | 2^61], row 1 becomes [0, 1 - 2^60 = -2^60, 0 | 1 - 2^61 = -2^61] in f64, and N = (0, 2, 5).
- B4, pivot search against a wrong cell (463 `p * WIDE - c`): A = [[1,0,0], [0,1,1], [0,e,1]], e = 2^-60, C/dt = 0, B = (5, 1, 2). Original: pivot 1 at stage 1, row 2 becomes [0, 0, 1 - e = 1 | 2 - e = 2], row 1 becomes [0, 1, 0 | 1 - 2 = -1]: N = (5, -1, 2). Mutant (compares with the unused zero cell 10, so row 2 pivots): pivot e, row 1 = [0, 1, 2^60 | 2^61], row 2 = [0, 0, -2^60 | -2^61], N = (5, 0, 2).
- 463 `>=`: expected value not derived.

**C: tie_gap and ties_clear never see a tie** (kills 309, 310, 311, 573, 578 x2).

- `tie_gap(2.5 / SCALE)` is exactly 0.0 (x = 2.5, f = 0.5). Mutants: 309 gives a gap near 0.5 (x is about 9e-15), 310 gives |4.5 - 0.5| = 4.0, 311 gives |0.5 + 0.5| = 1.0.
- `ties_clear(2, mn)` is false for mn = zeros except mn[2] = 2^-25 (a tie at M[1][0]): the original sees it at i = 1, j = 0; 573 reads mn[0] there and says true. Same call with only mn[5] = 2^-25 (a tie at N_1): 578 `n / n + i` reads mn[2] and `n * n - i` reads mn[3], both zero, and say true. The zeros are clear in the original (tie_gap(0.0) = 0.5, the diagonal shift makes -1.0 whose gap is 0.5).

**D: the checkers need negative controls** (kills 522, 529, 556, 559, 626 `|`).

- `residual(w, sol_bad, 2)` equals 1.0, where `w` and `sol` are the solver test's and `sol_bad` is `sol` with cell 4 (N_0, 7) set to 8.0: row 1 of the B column gives -7 + 1 x 8 = 1, row 0 gives -5 + 1 x 5 = 0, the M columns are untouched. 522 returns 0.0; 529 skips the B column.
- `mismatches(RC_N, &m_half, &RC_D, &RC_NV)` equals 2, where `m_half` is the existing dt = 2.5e-8 derive: M = 0.04 / 0.041, so D = -1/41 (409200 at 2^-24, not 798915) and N = 1/41 (409200, not 798915): one D mismatch and one N mismatch. 556 and 559 each give 1.
- The 200-step RC run has `peak` 16649635 (the state after step 99, which is `m0_a`, the largest because the charge rises monotonically and then decays). 626 `|` gives the last sample, 126610 (`m0_b`).

**E: refusals** (kills 331 `or`, 332 `or`, 364, 366).

- `node_count` with a = [2, 1], b = [0, 0], ne = 2 is 2 (the 331 mutant gives 1); with a = [0, 0], b = [2, 1] it is 2 (the 332 mutant gives 1).
- `table_ok` is false for the RC table with V written (1, GND) (a = 1, IN, 1; b = GND, 1, GND) and for V written (IN, 1) (a = IN, IN, 1; b = 1, 1, GND); the `and` mutant of 364 needs both terminals wrong and says true for each.
- `table_ok` is false for the RC table with the resistor value 0.0 (values 0, 0.0, 1e-9); the `<` mutant of 366 says true.

**F: matmul and power_norm on hand matrices** (kills 664 x3, 702).

- `matmul(2, a, a, out)` with a = [1, 2, 3, 4] gives [7, 10, 15, 22]. Mutants: `acc -` gives -7; `a[i / n + k]` makes row 1 read row 0 (cell [1][0] = 7, not 15); `b[k / n + j]` makes column reads wrong (cell [0][0] = 3, not 7).
- `power_norm(1, [0.5], 3)` is 0.00390625 (0.5^8, exact). The 702 mutant makes a fourth squaring: 1.52587890625e-05.

**Alternative for the dead lines 441, 442** (the tables are refused by `table_ok`; this only pins the stamp if the owner keeps it): kinds V, L, R; ZS 1.0; dt 1.0; values 0, 1, 1. For L(IN, 1) with R(1, GND): A = [[1, -1], [1, 1]], C/dt = [[0, 0], [0, 1]], B = (0, 1); M = [[0, 1/2], [0, 1/2]], N = (1/2, 1/2). For L(1, IN): A = [[1, 1], [-1, 1]], B = (0, -1); M = [[0, -1/2], [0, 1/2]], N = (1/2, -1/2). Each is one `derive` call asserted to 1e-12; the 441 and 442 index mutants and the sign mutant (`0.0 + 1.0`) each change a cell.

### 5. Counts

| verdict | count | where |
| --- | --- | --- |
| EQUIVALENT | 18 | old file rows 1-10 (238, 247, 291, 304, 331 and 332 `>=`, 376, 393, 461, 536, 572, 574 x2, 575, 578 `<`, 625, 626 `>=`, 686) |
| TEST GAP | 35 | A 10, B 6, C 6, D 5, E 4, F 4 (6 groups) |
| DEAD LINE | 7 | 441 x3, 442 x4 |
| not decided | 1 | 463 `>=` |
| total survivors | 61 | the same 61 as the pre-audit run |

HUNG, 38: all `[drop-step]` mutants of the form `x = x + 1` to `x = x`, each marked "its tests outlived --timeout" in the census; the census footer says a hang in a loop the mutant made endless (a dropped step) is the mutant's. Read against the spec, each of the 38 lines (269, 281, 333, 344, 372, 381, 384, 445, 464, 472, 479, 488, 491, 493, 507, 510, 534, 537, 539, 557, 560, 576, 579, 602, 614, 617, 627, 629, 646, 648, 665, 668, 670, 684, 687, 700, 707, 709) is the counter of the `while` that encloses it and that tests it, so the loop never closes. The same 38 lines as the pre-audit run; the old file's by-function list is exact. Line 443 (`l = l + 1`) and 461 are not loop counters and are survivors.

UNVIABLE, 5: all `[ret-default]` of a function with no return value, body emptied, and every one reads "zig: error: not yet implemented": `step` (277), `bump` (392), `stamp_pair` (397), `stamp_src` (407), `matmul` (654). The same 5 as the pre-audit run. That the cause is on the emitter side and not in this spec: not established here (the old file reads it so from the same message in other censuses).

### 6. Not established

- Everything in section 4 is hand arithmetic from the spec's function bodies; none was run. Each expected value should be confirmed on the lab before the assert is written.
- That the mutant build traps on `usize` underflow, or wraps (section 3, and mutants 401 and 408 `- col`).
- The IEEE sign-symmetry argument for 393, and the width of the f64 boundary band for 574.
- Whether a pivot tie (463 `>=`) can change any bit of a solved matrix.
- Whether any other spec calls `stamp` or `derive` on a table `table_ok` refuses (a repo-wide search for callers of `AnalogLinear` was not completed), which would move 441 and 442 from dead line to test gap.

## linear_zoh, post-audit census (run 11:36-11:44 +07, tree 1c783b0b31)

Compared against the pre-audit classification of linear_zoh (pre-audit run, tree 035df5ce60, 10:15-10:25 +07). Both raw outputs (this run and the pre-audit one) and the pre-audit classification file are kept outside the repo. Same method as for linear: spec text, the two raw outputs and a `git diff` of the two trees; nothing was built or run.

### 1. Comparison: 30 same, 0 differ

- The 30-survivor block of the two raw outputs is byte-identical (same hash over lines 3-93 of both files), and so is the mutant-count line (252: drop-guard 1, flip-cmp 33, drop-step 34, swap-arith 171, ret-default 13). Only three things differ between the raw outputs, all about the hung: the summary line (193 killed, 29 hung becomes 195 killed, 27 hung), the HUNG header (29 becomes 27), and two HUNG entries that are gone (section 3).
- The tree change is tests only: `git diff 035df5ce60 1c783b0b31 -- specs/analog/linear_zoh.t27` is 13 insertions and 6 deletions in two hunks, lines 637-650 (test `the_rc_exact_constants_are_the_rounding_of_the_derived_ones`) and lines 714-725 (test `the_q20_and_tank_exact_constants_are_the_rounding_of_the_derived_ones`). The asserts on `ZRC_D[0]`, `ZRC_NV[0]`, `RC_NV[0]`, `ZRLC_D[3]`, `ZQ20_D[3]` and `ZTK_D[3]` now read `var` copies (the same #12166 fix as in linear). The highest mutant line is 550 and the tests start at 559, so every survivor keeps its line number, and the changed asserts read constant tables that no mutant touches.
- Result: every one of the 30 survivors is the survivor classified in the old file, so every old verdict stands as written and is not retyped here. No group of the old file is closed by this audit.

### 2. Index: survivor -> old row

| this run: line [kind] | n | old row, old verdict |
| --- | --- | --- |
| 157 [flip-cmp] | 1 | `near`, test gap E |
| 176 [flip-cmp] | 1 | `pade_expm` halving tie, equivalent |
| 433 [flip-cmp] | 1 | `fmax64`, equivalent |
| 193, 198, 211, 221, 233, 241 (`pade_expm`), 279, 284 (`zoh`) [flip-cmp] | 8 | test gap D |
| 390, 393, 401 (`grid_err`), 507, 510, 518 (`ring_f64`), 544 (`ring_fixed`) [flip-cmp] | 7 | test gap D |
| 388, 421 [flip-cmp] | 2 | test gap B |
| 453, 469 [swap-arith] | 2 | test gap A |
| 505, 541 [flip-cmp] | 2 | test gap C |
| 522, 548 [flip-cmp] | 2 | test gap C |
| 524 x2, 550 x2 [drop-step, swap-arith] | 4 | test gap C |

The n column sums to 30. Kinds among the 30: flip-cmp 24, swap-arith 4, drop-step 2.

### 3. What did change: two mutants moved from hung to killed (none of the 30)

- Pre-audit: 29 hung, namely 27 drop-step counters plus 452 and 468 (`const m : u32 = 1 << j;` to `1 >> j` in `rlc_ex_err` and `rc_ex_err`, both marked "its tests outlived --timeout"). This run: 27 hung, the same 27 drop-step lines (166, 184, 195, 202, 215, 217, 226, 228, 235, 243, 245, 258, 271, 274, 281, 286, 395, 398, 403, 410, 426, 512, 515, 520, 526, 546, 552); 452 and 468 are in neither list now, so a failing test killed them (195 = 193 + 2). They were never survivors, so no verdict is touched.
- Why they moved: not established. Facts: the code of both functions and of everything they call is identical in the two trees (the diff touches lines 637-725 only), and the test that reaches them (the grid test) is not in the diff. The runs differ in machine load at the start (load averages 13.13, 20.07, 19.87 pre-audit; 7.43, 8.60, 12.03 now). Candidate reading, not run: at j = 1 the mutant makes m = 0 and dt infinite; the loop `while (nrm > 0.5)` of `pade_expm` then halves an infinite norm, and its counter `s = s + 1` (line 178) is a u32, so the loop ends by overflow after 2^32 passes if the build traps overflow. That end arrives at a time that depends on load against `--timeout`: killed when the machine is quiet, hung when it is busy. The 27 drop-step hangs have counters that never change (`x = x`), so they never end at any load, which fits their staying hung. This reading also depends on the build trapping integer overflow, which is inferred (the old file inferred index traps from the same lists), not established.
- The old file's statement that the hang of 452 and 468 was "read off the code" is superseded by this measurement; whether a third run puts them back among the hung: not established.

### 4. Second reading of the old verdicts against the spec text

No verdict changes. Fine print:

- 176 (`nrm > 0.5` to `>=`), 2 equivalent: the label means equal in exact arithmetic and equal up to rounding in f64 (a tie takes one more halving and one more squaring). A test that pinned the bits of a non-dyadic result could tell the two apart; none does. Kept as equivalent, as the old file has it.
- 157: the old missing asserts hold by hand: `near(1.5, 1.0, 0.5)` is |0.5| <= 0.5 (true, the `<` mutant gives false) and `near(2.0, 2.0, 0.0)` is 0 <= 0 (true, the mutant gives false); every operand is exact in binary.
- 388, 421: with k = 1 the original visits one grid point and the mutant two, so `grid_err_rc(&mn, RC_DT, 1)` has `scale` equal to `an_rc(RC_DT)`, which is 1 - e^-0.05 (RC_DT over R C is 0.05 by the module's constants), and the mutant's `scale` is the larger of that and 1 - e^-0.1. Values 0.0488 and 0.0952 as in the old file; unrun.
- Group D (15 mutants) still depends on the build trapping out-of-range indexes; the survivor lists are identical, so the old inference (mutants 256, 263 and 266 are in neither list) is the same, still inferred and not established.
- Group C expected values for the `ring_fixed` rows (541, 548, 550) still come from a lab run; not derived here.

### 5. Counts

| verdict | count | where |
| --- | --- | --- |
| EQUIVALENT | 2 | 176, 433 |
| TEST GAP | 28 | A 2, B 2, C 8, D 15, E 1 (5 groups) |
| DEAD LINE | 0 | |
| not decided | 0 | |
| total survivors | 30 | the same 30 as the pre-audit run |

HUNG, 27 (was 29): 27 drop-step counters, section 3. UNVIABLE, 0 (unchanged).

## Summary: linear and linear_zoh after the audit (tree 1c783b0b31)

Group letters are per file (the A of linear is the stamp shapes, the A of linear_zoh is the exact-grid twin). "Open" means the group still has survivors in the post-audit run; later lanes are not in scope.

- linear: test-gap groups open A, B, C, D, E, F (35 mutants, none closed).
- linear: 18 equivalent, 7 dead line (441 x3, 442 x4), 1 not decided (463 `>=`).
- linear_zoh: test-gap groups open A, B, C, D, E (28 mutants, none closed).
- linear_zoh: 2 equivalent (176, 433), 0 dead line, 0 not decided.
