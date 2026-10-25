# Mutation census of the remaining analog specs, 2026-10-25 (partial: 11 of 17 specs written up here)

Issue #12116, part of epic #10486 (weak spots of the analog v0.1 release). The tool, `tri mutate spec`, never edits
the spec, and the checkout stayed clean during the run. This note is written while the census is unfinished, so a
reader sees what is measured and what is not: 11 of the 17 specs have a measured run and are written up below; 6 are
not measured. linear and linear_zoh were first measured on a tree that the later audit #12191 changed, so sections 9
and 10 give the re-run on master after the audit, and raw_writer (section 9) was measured on a tree that master has
not changed. Where each number comes from:

- **Run:** `tri mutate spec --file specs/analog/sar_logic.t27 --t27c <master-built t27c> --jobs 3 --timeout 30`, in
  a T27 OS shell cell on this Mac, started 07:59:23 +07 and finished 08:24:07 +07 (24 min 44 s of wall time), exit 0.
  Lane base `035df5ce60`, binary `t27c 0.5.2` built from master. No `--accepted` file: this is a census, not a gate.
- **Machine at the start:** load average 26.13 19.54 17.57, 7.4 GiB free on the data volume. At 08:25 the volume
  was down to 0.4 GiB and the run on the second spec was stopped by hand (kill threshold 2.5 GiB); see section 13.

## 1. sar_logic.t27 (677 mutants)

Follow-up issue for the survivors and hung mutants: #12163.

Operator classes of the 677: drop-guard 57, flip-cmp 198, swap-logic 57, drop-step 64, swap-arith 249, ret-default 52.

| Outcome | Count | Read off the output |
| --- | --- | --- |
| killed | 526 | all 526 by a failing test, 0 by an invariant at compile time |
| survived | 108 | drop-guard 13, drop-step 4, flip-cmp 53, swap-arith 27, swap-logic 11 |
| hung | 31 | all 31 are drop-step; the first two read are `j = j + 1` -> `j = j` and `i = i + 1` -> `i = i`: the tests outlived `--timeout 30` |
| unviable | 12 | all 12 are ret-default; the first two read (`sl_init`, `sl_links`, a `-> void` body emptied) give `zig: error: not yet implemented` |

The 108 survivors start at line 331 of the spec and fill every 50-line bucket up to 1050-1099 (3 to 14 per bucket).
The densest are 500-649 (38 survivors, around `sl_delay`, `sl_net`, `sl_init`, `sl_links`, `sl_loads`) and 750-849
(21, around `sl_run`, `sl_edges`, `sl_absdiff`). The first three, for orientation: line 331 `p < 2` -> `p <= 2`, line
335 `p < 3` -> `p <= 3`, line 340 the guard `if (p == 1) { return 250; }` dropped.

Classification of the 108 survivors, by reading and nothing run, is in `docs/analog/census-rest-2026-10-25/sar_logic-classification.md`:
27 equivalent, 80 test gap, 1 dead line, 0 undecided (27 + 80 + 1 = 108). The 80 test gaps are grouped into proposed tests P1 to P26,
each with the smallest assert and the mutants it kills; the expected values in that file are hand-derived and stay unverified until a
T27 OS pane runs them. The 31 hung and the 12 unviable are not classified there.

## 2. sar_trace_raw.t27 (42 mutants)

Run: same command and binary, started 09:08:39 +07, finished 09:10:53 +07 (2 min 14 s), exit 0, load 24.11 at the
start, 3.0 GiB free at the end. No follow-up issue is needed for this spec: nothing in it is a test gap.

Operator classes of the 42: drop-guard 2, flip-cmp 13, swap-logic 2, drop-step 6, swap-arith 15, ret-default 4.

| Outcome | Count | Read off the output |
| --- | --- | --- |
| killed | 38 | all 38 by a failing test, 0 by an invariant at compile time |
| survived | 1 | line 158, flip-cmp: `if (dd > r.worst) { r.worst = dd; }` -> `>=` |
| hung | 3 | all 3 are drop-step: line 100 `k = k + 1;`, line 160 `v = v + 1;`, line 162 `p = p + 1;` |
| unviable | 0 | |

Classification, read off the spec (`trace_readback`, lines 158-162, and `sar_trace_with`, line 100):

- The survivor is an **equivalent mutant**: when `dd == r.worst` the body assigns `r.worst = dd`, which is the value
  it already holds, so `>` and `>=` give the same state on every input.
- The three hung mutants are **loops the mutant made endless**: each dropped step is the increment of a `while`
  counter, so the loop never ends and the test outlives `--timeout 30`. The tool says as much in its own footer.
  The unmodified spec ran to the end in the same run (38 killed by failing tests, none by a timeout).

An accepted-list file for a future gate would hold the one survivor with the reason above; it is not added here,
because this note is a census and no gate runs on this spec yet.

## 3. vacask_compare (measured 09:34-09:37 +07, exit 0, 2 min 53 s)

48 mutants, 34 killed (all by a failing test), 8 survived, 6 hung, 0 unviable. Command: `tri mutate spec --file specs/analog/vacask_compare.t27 --t27c <0.5.2> --jobs 3 --timeout 30`.

The 6 hung are drop-step mutants of loop counters (`i = i + 1` becomes `i = i`), so each loop never ends; they are the mutant's hang, not a test gap.

| line | mutant | class | why |
| --- | --- | --- | --- |
| 173 | `d > worst` to `>=` | equivalent | at `d == worst` the assignment writes the value already held |
| 185 | `d < best` to `<=` | equivalent | same reason |
| 181 | `a[0] - b[0]` to `a[0] + b[0]` | test gap | the only data run through `vc_mindiff` has a[0] = 0.0 and b[0] = -1.05e-14, so the sum and the difference have the same magnitude |
| 184 | `a[i] - b[i]` to `a[i] + b[i]` | test gap | the minimum difference sits at index 0 in every call, so a wrong later difference is never the minimum |
| 193 | `<= tol` to `< tol` | test gap | no assert puts |x - y| exactly on the tolerance |
| 198 | `dp > dn` to `>=` | test gap | no assert for a tie (`vc_win(x, x) == 0`) |
| 227 | `i < n` to `i <= n` in `vc_bracket_hi` | test gap | no call where dp never wins (the loop then reads one past n) |
| 241 | drop the `return false` guard in `vc_monotone` | test gap | no call with a non-monotone winner sequence, so `vc_monotone` is never seen to return false |

Follow-up: one issue for the five test-gap groups (181 and 184 share a cause).

## 4. bench (measured 09:37-09:39 +07, exit 0, 2 min 0 s)

58 mutants, 55 killed (all by a failing test), 3 survived, 0 hung, 0 unviable.

| line | mutant | class | why |
| --- | --- | --- | --- |
| 258 | `tran > wall` to `>=` in `startup_permille` | equivalent | at `tran == wall` the original returns `ratio_permille(0, wall)`, which is 0, the same as the mutant's early return |
| 281 | `x < 0.0` to `<=` in `fabs64` | equivalent | at `x == 0.0` the mutant returns `0.0 - 0.0`, the same value |
| 287 | `a >= b` to `>` in `absdiff64` | equivalent | at `a == b` the fall-through returns `b - a`, which is 0, the same value |

No test gap, so no follow-up issue for this spec.

## 5. mos_psp103_gates (measured 09:43-09:48 +07, exit 0, 4 min 14 s)

94 mutants, 75 killed (all by a failing test), 10 survived, 9 hung, 0 unviable.

The 9 hung are all endless loops made by the mutant: 5 dropped counter steps (lines 197, 236, 250, 261, 295) and 4 bisection-loop conditions at lines 128 and 141 (`hi - lo > 1` to `hi + lo > 1` or `>= 1`, so the bracket never closes). No test gap.

| line | mutant | class | why |
| --- | --- | --- | --- |
| 190, 192, 195, 196, 235, 249, 260 | `<`/`>` to `<=`/`>=` in running min/max updates | equivalent | at equality the assignment writes the value already held |
| 248 | `g < 0.0` to `<=` in the absolute value | equivalent | at g = 0.0 the mutant computes 0.0 - 0.0, the same value |
| 233 | `k < GT_NV` to `<=` in `gt_tg_worst` | test gap (low value) | adds a 28th point after the 27 the doc comment names; the survivor means no test input has its largest resistance there, and no test pins the point count |
| 294 | `want >= 1e-9` to `>` in the hold check | test gap (low value) | no reference value sits exactly at 1 nA, so the boundary of the n9 count is never exercised |

Follow-up: one issue for the two low-value gaps, filed together with the others at the end of the census.

## 6. mos_psp103_inv20 (measured 09:39-09:43 +07, exit 0, 4 min 19 s)

101 mutants, 72 killed (all by a failing test), 23 survived, 5 hung, 1 unviable. Run: `tri mutate spec` on `specs/analog/mos_psp103_inv20.t27`, 3 at a time, zig -j2 each, t27c 0.5.2. Raw output: kept outside the repo. The classes below are from reading the spec and the census text; nothing was re-run.

Facts the table leans on: the gate `a` of every call is 0.0 or IV_VDD (25165824.0); the output `y` is fractional from the bisection or hand-fed (-1, 1, IV_VDD +- 1); a rounding is `(v + 0.5) as u32`, which truncates toward zero, so `(-0.5) as u32` is 0; the current at Vds = 0 is exactly 0 (`vds_factor(0)` is 0); the charge closure makes Qs(x, 0) = Qd(x, 0) exactly (the first word of every charge table is SQ_QOFF, 2097152); `cross_idx` returns -1.0 when there is no crossing and a found crossing is greater than 0.0 (sample 0 is the rest level, never on the threshold).

The 5 hung are all drop-step mutants of loop counters (lines 177, 202, 229, 281, 297: `k = k + 1` or `j = j + 1` becomes the name unchanged), so each loop never ends; they are the mutant's hang, not a test gap. No other hang.

The 1 unviable is the `[ret-default]` mutant of `iv_run` (line 209, body lines 210-231 emptied); zig answers `zig: error: not yet implemented`. The census text does not say why it fails to compile. The same message, for the same kind of mutant (a function with no return value, body emptied), stands in the census of `sar_cdac.t27:444` (`event_k`), `sar_comparator_psp.t27:137` (`sp_scale`) and `sar_logic.t27:584` (`sl_init`, with more in that file), so the cause is on the emitter side, not in this spec. Not a test gap, not a dead line.

| line | mutant | class | why |
| --- | --- | --- | --- |
| 126 | `y >= 0.0` to `>` in `iv_nmos` | equivalent | at y == 0.0 the other branch is taken with d = 0.0, so x, e and the charge are the same (the closure Qs = Qd at Vds = 0 is exact) and the current is 0.0 - 0.0 in place of 0.0 |
| 127 | `a + 0.5` to `a - 0.5` in `iv_nmos` | test gap (group A, low value) | at a = IV_VDD the drive word is 25165824 in the original and 25165823 in the mutant (at a = 0.0 both are 0); the existing call `n4 = iv_nmos(1.0, gate_hi, 0)` reaches it but the assert is `n4.cur_iu > 30.0`. Missing assert: the integer current of a saturated point, `iv_nmos(12582912.0, IV_VDD, 0).cur_iu == sq_i(0, 25165824, 12582912, 0) as f64`, in the jump test |
| 134 | `a + d + 0.5` to `a + d - 0.5` in `iv_nmos` | test gap (group A, low value) | for every d >= 0.5 the word differs by 1; the kick goes to d near 6.4e6 words, so the real runs reach it, and no assert reads the integer. Missing assert: `iv_nmos(0.0 - 8388608.0, 0.0, 0).cur_iu == 0.0 - sq_i(0, 8388608, 8388608, 0) as f64`, in the jump test |
| 143 | `y < IV_VDD` to `<=` in `iv_pmos` | equivalent | at y == IV_VDD the first branch is taken with e = 0, x and xn the same as y - a gives, so the charge is the same by the exact closure and the current is 0 (Vds = 0) in both |
| 146 | `IV_VDD > a` to `>=` | equivalent | the two branches differ only at a == IV_VDD, where both leave x = 0 and xn = 0 |
| 146 | `IV_VDD - a` to `IV_VDD + a` | equivalent | the branch is taken only at a = 0.0 (a is 0.0 or IV_VDD), where the sum and the difference are equal |
| 146 | `IV_VDD - a + 0.5` to `- 0.5` (x) | test gap (group A, low value) | at a = 0.0 the word is 25165824 in the original and 25165823 in the mutant; the call `p2 = iv_pmos(top_above, gate_lo, 0)` and every gate-low run reach it, no assert reads the integer. Missing assert: `iv_pmos(12582912.0, 0.0, 0).cur_iu == 0.0 - sq_i(1, 25165824, 12582912, 0) as f64`, in the jump test |
| 146 | `a - IV_VDD + 0.5` to `- 0.5` (xn) | equivalent | the else branch is taken only at a == IV_VDD, where a - IV_VDD is 0.0 and both +0.5 and -0.5 truncate to 0 |
| 154 | `y > a` to `>=` | equivalent | the branches differ only at y == a, which with y >= IV_VDD and a in {0.0, IV_VDD} means y == a == IV_VDD, where both leave x2 = 0 and xn2 = 0 |
| 154 | `y - a + 0.5` to `- 0.5` (x2) | test gap (group A, low value) | for y > a the word differs by 1 (the fall's kick over the supply, and `p2`, `p4` of the jump test, reach it); no assert reads the integer. Missing assert: `iv_pmos(IV_VDD + 8388608.0, 0.0, 0).cur_iu == sq_i(1, 25165824 + 8388608, 8388608, 0) as f64`, in the jump test |
| 154 | `a - y + 0.5` to `- 0.5` (xn2) | equivalent | the else branch is reached only at y == a == IV_VDD, where a - y is 0.0 and both truncate to 0 |
| 173 | `k < IV_ITERS` to `<=` in `iv_rest` | equivalent | a 65th halving of a bracket that is 17 * 2^22 words wide before 64 halvings (about 4e-12 words after) moves the root by under 2e-12 words; the rest bar is 17 words. At worst it flips a one-word rounding read at an exact half-word root, which no assert sees |
| 176 | `n.cur_iu > 0.0` to `>=` in `iv_rest` | not decided | the current is an integer, so a midpoint can land on an exact 0 and the two ends of that plateau (at most about one word apart) are then the two answers. For the gate-high rest the survival shows there is no zero plateau (the mutant would end near -0.5 and fail `hi >= 0.0`). For the gate-low rest `iv_rest(0.0)` whether the PMOS and NMOS integers cancel exactly at a visited midpoint cannot be read off the code; either way the shift is at most one word against the bar `vdd - lo < 17.0`. Leans equivalent; one lab run (print the 64 currents of `iv_rest(0.0)`) decides |
| 199 | `k < IV_ITERS` to `<=` in `iv_step` | equivalent | same reason as line 173: a 65th halving moves the root by under 2e-12 words, far under every bar (1 percent of ngspice, 0.1 percent between steps) |
| 201 | `g > 0.0` to `>=` in `iv_step` | equivalent | g rises with y at a slope of at least 1 (the term `y - y0`; the node-equation test), so a tie at a midpoint is the root itself and both directions converge to it within the last bracket |
| 223 | `k < n` to `<=` in `iv_run` | test gap (group B) | the mutant runs step n and writes `out[n + 1]`, which `iv_delay_ps`, `iv_kick` and `iv_wave_gap` never read (they stop at index n). Missing assert: the cell after the run stays 0.0, `out[201] == 0.0` after the first `iv_run(25000, IV_DT_FS, 0, 200, 0, &out)` of the 25 fF test (the array starts as all 0.0) |
| 236 | drop `if (c < 0.0) { return 0.0 - 1.0; }` and `0.0 - 1.0` to `0.0 + 1.0` in `iv_delay_ps` | test gap (group C) | no test feeds `iv_delay_ps` a run that does not cross 0.75 V, so the -1 path is never taken; with the guard dropped the function returns `-1.0 * dt_fs / 1000.0`, which is -1.0 only at dt_fs = 1000 (`IV_DT_FS`). Missing assert: on a flat all-0.0 array, `iv_delay_ps(&flat, 0, 200, 500) == 0.0 - 1.0` (500, not 1000, so the dropped guard shows too), in the 25 fF test or a block of its own |
| 236 | `c < 0.0` to `<=` | equivalent | a found crossing is strictly greater than 0.0 (sample 0 is the rest level, not on the threshold), so c is never exactly 0.0 |
| 280, 295, 296 | `d > worst`, `v < m`, `v > m` to `>=`, `<=`, `>=` | equivalent | at equality the assignment writes the value already held |
| 293 | `k <= n` to `<` in `iv_kick` | test gap (group D) | the mutant leaves out the last sample; at dt = 500 the extreme is at index 1, so the last sample (index 40) is never the extreme. Missing assert: a step so large that the run is one step, `iv_kick(0, 20000, 0)` (n = 1), must be below 0.0 (the original returns out[1], the mutant returns the rest level out[0], about 3e-8 V), in the waveform-and-kick test; the expected bound comes from the lab, not from this note |

Counts of the 23 survivors: 14 equivalent (126, 143, 146 `>=`, 146 `+`, 146 xn, 154 `>=`, 154 xn2, 173, 199, 201, 236 `<=`, 280, 295, 296), 8 test gap (group A: 127, 134, 146 x, 154 x2; group B: 223; group C: 236 drop-guard and 236 `+ 1.0`; group D: 293), 0 dead line, 1 not decided (176).

Not run, so not decided: for group A the rounded word is shown to differ (by 1 of 25165824) at the real gate values; how many IUNIT the current moves by is not run (at the jump test's own inputs, 1 word of Vds, the effect is under one IUNIT and the test's bars are 2 and 30 IUNIT, so a saturated point is the place to look; if the integer read does not move even there, those four turn equivalent at the table's resolution).

Follow-up: one issue for the four test-gap groups (A: the gate-drive rounding in `iv_nmos` and `iv_pmos`, four mutants, low value; B: the extra step of `iv_run`; C: the no-crossing return of `iv_delay_ps`; D: the last sample of `iv_kick`), with the one undecided mutant (line 176) added as a single lab run; the expected values of the new asserts come from the lab, not from this note.

## 7. sar_comparator_qi (measured 09:48-10:00 +07, exit 0, 12 min 13 s)

308 mutants, 256 killed (all by a failing test), 20 survived, 18 hung, 14 unviable. Kinds: drop-guard 5, flip-cmp 46, drop-step 43, swap-arith 193, ret-default 21. Run: `tri mutate spec` on `specs/analog/sar_comparator_qi.t27`, 3 at a time, zig -j2 each; the census text does not record the t27c version. Raw output: kept outside the repo. The classes below are from reading the spec (this lane), `sq_jq` in `sar_comparator_q.t27`, `sc_add_j` and the sign-magnitude helpers in `sar_comparator.t27`, and the one other caller of `qi_scatter` in `board/sar_cmp_qi_unit.t27`; nothing was re-run.

Facts the table leans on: `QI_C_CUR` (line 106) and `QI_R_HI` (line 89) are both 0, so `+ 0` and `- 0` index the same word; `qi_stat` keeps, per block b (0 for the full evaluations, 7 for the charge-only ones), b+0 the count, b+1 the sum of reads, b+2 the least reads, b+3 the most reads, b+4 the sum of multiplications, b+5 the least, b+6 the most, and for block 0 only the least word 14 + 2c and the most word 15 + 2c of counter c (c = 0..3: current reads, charge reads, slope reads, other multiplications); `qi_st_init` puts 4294967295 into the least words (2, 5, 9, 12, 14, 16, 18, 20); within a step `qi_charges` (block 1) runs before the iterations (block 0); the only assert that reads words 14..21 is in the cost test, after both 438-step runs; `qi_ord` is called only in the order test (pe = [0,1,1,2,2], ps = [1,0,2,0,2], and o = 1, 2), whose every assert says the result equals the natural-order one, which holds for any order; the other tests pass `&QI_ORD_REV` and `&QI_ORD_MIX` directly; `qi_scatter` with jac false is called by `qi_charges` (its `reg` is a local that is never read) and by `spec_matrix` of the board unit (which reads `reg`, but a jac-false record holds 0 in every slope word, because `qi_eval` zeroes its slot and writes the slopes only with jac); `sq_jq` with slopes 0 adds magnitude 0 to a register (`sc_mag(0)` is 0, `sc_flip` keeps the magnitude, `sc_add_j` ignores a row or column at or above SC_NN); the tests of `qi_accept` feed 1.004, 0.996, 1.006, 0.994 times the reference, the charge-removed run and the two real runs (about 0.0007 and 0.0001 percent off).

The 18 hung are all drop-step mutants of loop counters (`x = x + 1` becomes `x = x`), so each loop never ends; they are the mutant's hang, not a test gap. Grouped by function: `qi_st_init` (155), `qi_stat` (187, 190), `qi_state` (244), `qi_same` (258), `qi_eval` (305), `qi_charges` (402, 407, 413), `qi_assemble` (425, 431, 437), `qi_stamp` (452), `qi_step` (482, 490), `qi_run` (513, 526, 528). No hang is of another kind.

The 14 unviable are of two kinds. 10 are the `[ret-default]` mutant of a function with no return value, body emptied, and zig answers `zig: error: not yet implemented`: `qi_st_init` (151), `qi_stat` (168), `qi_state` (240), `qi_eval` (269), `qi_scatter` (370), `qi_charges` (395), `qi_assemble` (419), `qi_stamp` (446), `qi_update` (459), `qi_iter` (469). It is the same message for the same kind of mutant as in the censuses of `mos_psp103_inv20`, `sar_cdac`, `sar_comparator_psp` and `sar_logic`, so the cause is on the emitter side, not in this spec; not a test gap, not a dead line. The other 4 are `[swap-arith]` in `qi_run` (lines 501, 502, 504, 505, `SQ_OFF + SC_VCM` becomes `SQ_OFF - SC_VCM`): SQ_OFF is 8388608 and SC_VCM is 12582912, so the constant difference is -4194304, which u32 cannot hold, and zig refuses it at compile time (`overflow of integer type 'u32' with value '-4194304'`); the type rejects the mutant, not a gap.

| line | mutant | class | why |
| --- | --- | --- | --- |
| 142, 147 | `cn[e * QI_NC + QI_C_CUR]` to `- QI_C_CUR` in `qi_reads_of`, `qi_mults_of` | equivalent | QI_C_CUR is 0, so both indices are `e * QI_NC` (the same swap on the non-zero offsets QI_C_QRD, QI_C_SLP, QI_C_MUL is not among the survivors, so it was killed) |
| 307, 362, 374 | `rec[o + QI_R_HI]` to `- QI_R_HI` (307, 374), `cn[oc + QI_C_CUR]` to `- QI_C_CUR` (362) | equivalent | QI_R_HI and QI_C_CUR are 0, the word index is unchanged; the neighbouring offsets (QI_R_LO and the rest) were killed |
| 176, 177, 179, 180, 185, 186 | `<` to `<=` (176, 179, 185) and `>` to `>=` (177, 180, 186) in `qi_stat` | equivalent | at equality the assignment writes the value already held (`st[b+2] = rd` when `rd == st[b+2]`, and the same for the other five) |
| 185 | `st[14 + 2 * c]` to `st[14 - 2 * c]` in the condition of the least-word update of `qi_stat` | test gap (group A) | the condition reads word 14, 12, 10, 8 (c = 0..3) while the write goes to 14, 16, 18, 20, so for c >= 1 the "least" is compared with a block-1 word and the word becomes the last value that passed that test, not the least. It survives because the one reader (cost test, `st[14] = 2`, `st[16] = 6`, `st[18] = 8`, `st[20] = 12`) looks only after the whole 876-step accumulation: c = 1 is the constant 6 in mode 0; for c = 3 the test `cv < st[8]` is always true once the first charge-only block has run (st[8] is at least 90, cv at most 20), so the mutant's word is the last evaluation's count, and that it still reads 12 means the last full evaluation of the last step carries 12 (and 8 slope reads for c = 2), the least; inferred from the survival, not run. Missing assert: in the shape test, which already builds a counter array by hand, call `qi_st_init` and then `qi_stat(&st, 0, &cn)` on a 60-word `cn` whose device 0 holds the least (2, 6, 8, 12) and whose devices 1 to 14 hold larger words (for example 4, 6, 16, 20), and assert `st[14..21]` equal the least and the most (2, 4, 6, 6, 8, 16, 12, 20); on a fresh `st` the mutant never writes words 18 and 20 (st[10] and st[8] are 0), so they stay 4294967295 |
| 234, 235 | drop the guard `if (k == 0) { return &QI_ORD_NAT; }`, flip it to `k != 0`; drop the guard `if (k == 1) { return &QI_ORD_REV; }`, flip it to `k != 1` in `qi_ord` | test gap (group B) | each makes `qi_ord` return another table for some k (drop 234: k=0 gives the shuffle; flip 234: k=1 and 2 give the natural order, k=0 the shuffle; drop 235: k=1 gives the shuffle; flip 235: k=1 gives the shuffle, k=2 the reversed). The order test calls `qi_ord` for the five pairs and for o = 1, 2, but asserts only that the result equals the natural-order result, which is true for every order, so the wrong table passes (and so would the same table five times). Missing assert: in the order test, bind `const o0 : []const u32 = qi_ord(0)`, `o1 = qi_ord(1)`, `o2 = qi_ord(2)` and assert the first and last words: o0 is 0 and 14, o1 is 14 and 0, o2 is 7 and 10 (the words of QI_ORD_NAT, QI_ORD_REV, QI_ORD_MIX); the first word of o0 and o1 alone kills all four |
| 250 | `<=` to `<` in `qi_accept` | test gap (group C, low value) | the two differ only when `fabs64(t_ps - ref_ps) / ref_ps` equals QI_BAR exactly, and no test input is there (0.4 and 0.6 percent off, and the runs, are far from 0.5). An exact input exists: with variables, `qi_accept(201.0, 200.0)` has difference 1.0 and 1.0 / 200.0 is the nearest f64 to 0.005, as the literal QI_BAR is, so the original accepts and the mutant refuses (same for 199.0). Missing assert: both, in the "THE BAR SEES WHAT IT SHOULD" lines of the charge-removed test, true for both; the equality of that quotient and QI_BAR is by IEEE rounding, not run, so the lab confirms it first, and it also pins the bar as inclusive ("within the bar") |
| 293, 296 | `vg > vlo` to `>=` (293, NMOS), `vhi > vg` to `>=` (296, PMOS) in `qi_eval` | equivalent | at a tie the original takes the else branch with `xn = vlo - vg = 0` (or `vg - vhi = 0`) and the mutant takes the first with `x = 0`; x and xn both start at 0, so both end at (0, 0) |
| 386 | drop `if (!jac) { return; }` in `qi_scatter` | dead line | without the guard a jac-false call goes on to the three `sq_jq` calls with slope words that are all 0 (`qi_eval` writes the slopes only with jac), which add magnitude 0 to a register, so `reg` is unchanged in both callers (in `qi_charges` it is a local nobody reads, in the board unit's `spec_matrix` it is read and stays as it was). The line only skips work; no result depends on it. Keep or delete is the owner's call (it states that a charge-only evaluation does not touch the Jacobian, and a board's cost model would count the 9 zero adds per device if it were gone) |

Counts of the 20 survivors: 13 equivalent (142, 147, 176, 177, 179, 180, 185 `<=`, 186, 293, 296, 307, 362, 374), 6 test gap (group A: 185 `- 2 * c`; group B: 234 drop, 234 flip, 235 drop, 235 flip; group C: 250), 1 dead line (386), 0 not decided.

Not run, so inferred: for group A, that the last full evaluation of the last step carries the least counters (12 other multiplications, 8 slope reads) is read off the survival and the always-true condition, not counted; for group C, that `1.0 / 200.0` compares equal to QI_BAR in f64 is the IEEE argument, to be confirmed by one lab run before the assert is written. Neither changes the class.

Follow-up: one issue for the three test-gap groups (A: the least and most words of `qi_stat` checked on a hand-built counter array whose least is not in the last device, one mutant; B: the three tables of `qi_ord`, four mutants; C: the bar's exact boundary in `qi_accept`, one mutant, low value), so 3 groups and 6 mutants; the expected values of A and B are chosen by hand in the test (the counter words, the words of the QI_ORD tables), the one value for C comes from the lab. The dead line (386) needs a decision, not an assert: keep it as the statement that charge-only evaluations skip the Jacobian, or delete it.

## 8. sar_comparator_q (measured 10:25-10:39 +07, exit 0, 14 min 13 s)

336 mutants, 266 killed (all by a failing test), 40 survived, 17 hung, 13 unviable. Kinds: drop-guard 20, flip-cmp 66, swap-logic 3, drop-step 34, swap-arith 181, ret-default 32. Run: `tri mutate spec` on `specs/analog/sar_comparator_q.t27`, 3 at a time, zig -j2 each; the census ran with t27c 0.5.2, an older build than the tree's own, and the tree was at commit 035df5ce60. Raw output: kept outside the repo. The classes below are from reading the spec (this lane), `sar_comparator.t27` (the device tables SC_DV_*, `sc_add_j`, the sign-magnitude helpers) and `mos_table.t27` (`lim`, `dpos`, `mag_diff`, `lg_vsb_*`); nothing was re-run.

Facts the table leans on: SQ_R (line 140) is 1, so both clock edges are one step, and SQ_VDD - SQ_OFF is VDD (33554432 - 8388608 = 25165824), so the ramp ends are the rails; `sq_clk_at` can therefore reach line 167 only at j = 0 and line 164 never; `sc_pack(0, .)` is 0 (zero is normalised) and `sc_sadd(q, 0)` is q, so a term of magnitude 0 added to a charge changes nothing; the fold pins `sp`, `sm` and `s` hash every sample of three runs (+h and -h short, and the long run with jf = 1250), so a mutant that changes one sample of those runs is killed, but the sweep, the free start and the negative-control runs are not fold-pinned; `sc_add_j` ignores a row or column at or above SC_NN, so a slope whose column is a rail (GND, VDD, the clock, the inputs) is dropped; `lg_vsb_cell` and `lg_vsb_frac` read `lim(vsb)`, which clamps vsb to the supply; the only callers of `sq_cross`, `sq_t_first`, `sq_sign` and `sq_t_res` are asserts on real runs (lines 953-1159), never on a hand-built waveform, and the only reader of `sq_t_res` is the window `tr > 1040.6`, `tr < 1040.8` (lines 1081-1082), 0.2 ps wide against 0.01 ps for the decision time of the neighbouring tests; `sq_charges` passes `sq_dev` a local `reg` that nobody reads, and `sq_dev` has no other caller; the SWAPQ control (lines 1148-1155) asserts only `r4 > 4000000000` and that `s3 != s4` and `s4 != s5` (lines 1172-1173), so it pins neither the residual figure nor the time that the header (line 89) states.

The 17 hung are all drop-step mutants of loop counters (`x = x + 1` becomes `x = x`), so each loop never ends; they are the mutant's hang, not a test gap. Grouped by function: `sq_charges` (411, 416), `sq_stamp` (427, 433, 439), `sq_step` (465, 473), `sq_run` (511, 520, 522), `sq_cross` (537), `sq_t_res` (568), `sq_node_err` (603, 605), `sq_jac_err` (641, 656, 658). No hung mutant is of another kind.

The 13 unviable are of two kinds. 8 are the `[ret-default]` mutant of a function with no return value, body emptied, and zig answers `zig: error: not yet implemented`: `sq_acc` (270), `sq_jadd` (280), `sq_jq` (285), `sq_dev` (323), `sq_charges` (406), `sq_stamp` (421), `sq_iter` (448), `sq_rest` (482); the same message for the same kind of mutant as in the censuses of `sar_comparator_qi` and `sar_logic`, so the cause is on the emitter side, not in this spec. The other 5 are `[swap-arith]` in `sq_run` (499, 500, 502, 503) and `sq_t_first` (544), `SQ_OFF + SC_VCM` becomes `SQ_OFF - SC_VCM`: the constant difference is -4194304, which u32 cannot hold, and zig refuses it at compile time (`overflow of integer type 'u32' with value '-4194304'`); the type rejects the mutant, not a gap.

| line | mutant | class | why |
| --- | --- | --- | --- |
| 162, 163 | `j - jf + 1` to `j + jf + 1` (162), `i >= SQ_R` to `i > SQ_R` (163) in `sq_clk_at` | equivalent | with SQ_R = 1 the value i is at least 1 on this path (j >= jf), so `i >= SQ_R` is always true and both versions of 162 return SQ_OFF; the flip of 163 lets i = 1 fall to line 164, which gives SQ_VDD - 1 * (VDD / 1) = SQ_OFF, the same word, and every i > 1 returns SQ_OFF in both |
| 164 | `SQ_VDD - i * (VDD / SQ_R)` to `+ i *`, to `- i /`, to `VDD * SQ_R` (3 mutants) | dead line | at SQ_R = 1 the guard on line 163 returns for every i >= 1, so line 164 never runs; it is the falling ramp for SQ_R > 1 (a rise of several steps), kept for the parameter. Keep or delete is the owner's call; if SQ_R stays a constant 1 the line is dead weight, and a test of it would need a second SQ_R, which the spec does not have |
| 166, 167 | `j >= SQ_R` to `j > SQ_R` (166), `SQ_OFF + j * (VDD / SQ_R)` to `- j *`, to `j /`, to `VDD * SQ_R` (167, 3 mutants) | equivalent | with SQ_R = 1 the guard of 166 returns SQ_VDD for every j >= 1, so line 167 runs only for j = 0, where each of the four versions is SQ_OFF (0 * x, 0 / x and a subtracted 0 are all 0); the flip of 166 lets j = 1 fall to 167, which gives SQ_OFF + 1 * VDD = SQ_VDD, the same word |
| 202 | `w >= SQ_QOFF` to `w > SQ_QOFF` in `sq_qw` | equivalent | at w = SQ_QOFF the original packs magnitude 0 with sign `pmos`, the mutant packs magnitude SQ_QOFF - w = 0 with sign `!pmos`; `sc_pack(0, .)` is 0 either way |
| 247, 251, 255 | `xn > 0` to `xn >= 0` (247, always true on a u32), `x > VDD` to `x >= VDD` (251), `y > VDD` to `y >= VDD` (255) in `sq_qt` | equivalent | at xn = 0, x = VDD and y = VDD the branch is entered by the mutant and adds a term of magnitude `(sc_mag(s) * 0) >> CELL_BITS` = 0 (`sc_pack(0, .)` is 0), and `sc_sadd(q, 0)` is q; `sq_slope` has no side effect, so nothing else differs |
| 307, 311, 314, 315 | `x + SQ_DV <= VDD` to `<` (307), `y + SQ_DV <= VDD` to `<` (311), `vsb + SQ_DV <= VDD` to `<` (314), `vsb - SQ_DV` to `vsb + SQ_DV` (315) in `sq_fd` | test gap (group C, low value) | 307, 311, 314 differ from the original only on the exact tie `x + SQ_DV = VDD`, that is x (or y, or vsb) = VDD - SQ_DV = 25149440: there the original takes the forward difference and the mutant the backward one, and no run puts a gate drive, a Vds or a source-bulk voltage on that one word. 315 runs only when `vsb + SQ_DV > VDD`: the original reads the current at `vsb - SQ_DV`, the mutant at `vsb + SQ_DV`, which `lim` clamps to the supply, and for vsb = VDD the mutant reads the current at VDD, so its difference is `dpos(i0, i0)` = 0 while the original's is positive (the current falls with vsb); no run has a source-bulk voltage within 1 mV of the supply (hand arithmetic, not run). Missing assert: a direct `sq_fd(kind, x, y, vsb, i0, axis)` at x = 25149440 (axis 0), y = 25149440 (axis 1), vsb = 25149440 and vsb = VDD (axis 2), with i0 = `sq_i(kind, x, y, vsb)`, asserted against `dpos(sq_i(kind, x + SQ_DV, y, vsb), i0)` and the other forms written out in the test; the lab confirms that forward and backward values differ at the chosen words before the assert is written |
| 347, 350 | `vg > vlo` to `vg >= vlo` (347, NMOS), `vhi > vg` to `vhi >= vg` (350, PMOS) in `sq_dev` | equivalent | at a tie the original takes the else branch with `xn = vlo - vg = 0` (or `vg - vhi = 0`) and the mutant takes the first with `x = 0`; x and xn both start at 0, so both end at (0, 0) |
| 348, 351 | `vlo > SQ_OFF` to `>=` (348, NMOS), `vhi < SQ_VDD` to `<=` (351, PMOS) in `sq_dev` | test gap (group D, low value) | at the tie `vlo = SQ_OFF` (or `vhi = SQ_VDD`) both versions give vsb = 0, but the original sets `forward` and so leaves the body slope cb at 0, while the mutant computes `cb = m * sq_fd(..., 2)` and stamps it. In the runs that changes nothing: for the NMOS devices 4, 9..12 the low end is GND, a rail, and `sc_add_j` drops a column at or above SC_NN, and no run puts a node end exactly on the word SQ_OFF or SQ_VDD with a device conducting (the tie at rest, net2 and net3 at SQ_VDD under the PMOS 7 and 8, has the device off, where the body slope is below one word; inferred from the survival, not run). Missing assert: a direct `sq_dev` call with jac true whose low end is a node at exactly SQ_OFF (device 2 with net1 = SQ_OFF and ap = SQ_OFF + 6291456, gate VDN high) and, for 351, device 7 with net3 = SQ_VDD and an low, asserting the sum of the row of `reg` for that device, J[hi][hi] + J[hi][lo] + a = 0 or the cb word itself; the lab confirms that cb is non-zero there before the assert is written |
| 369 | `kq = 1 - kind` to `1 + kind` in `sq_dev` (SQ_M_SWAPQ) | test gap (group E, low value) | for kind 0 both give 1; for kind 1 the mutant gives kq = 2, so `sq_qd_tab(2)` and `sq_qg_tab(2)` still return the PMOS tables but `pm = (kq == 1)` is false and the PMOS charges are read with the NMOS sign. The SWAPQ negative control is a different wrong run, and it passes: the test asserts `r4 > 4000000000` (true for both) and `s4` unequal to `s3` and `s5` (true for both), never the 4.2e9 units and the 7.8 ps that the header states, nor a fold of the run. Missing assert: `assert_eq` on `s4` (the fold, value from the lab) or `r4` and the first-output time within a window around the header's figures; the lab confirms the figures first |
| 388 | drop `if (!jac) { return; }` in `sq_dev` | dead line | without the guard a jac-false call goes on to compute the slopes and the three `sq_jq` calls into `reg`; the one caller with jac false is `sq_charges`, whose `reg` is a local that nobody reads, and `sq_dev` has no other caller. The line only skips work; no result depends on it. Keep or delete is the owner's call (it states that a charge-only evaluation does not touch the Jacobian) |
| 484 | `vdn > vdp` to `vdn >= vdp` in `sq_rest` | equivalent | at vdn = vdp the original keeps top = vdp, the mutant takes top = vdn, the same word |
| 530, 533 | `while (j <= n)` to `j < n` (530), `y0 < lvl and y1 >= lvl` to `y0 <= lvl and ...` and to `... and y1 > lvl` (533, 2 mutants) in `sq_cross` | test gap (group A, high value) | `sq_cross` is called only on real runs (through `sq_t_first`), where the crossing is neither in the last interval, nor on a sample that sits exactly on the level. Missing asserts, on a hand-built `out` of 8 words per row with `sel` = SC_DP and lvl = SQ_OFF + SC_VCM: rows [lvl-4, lvl+4] with n = 1 return 2048 (the original; the mutant 530 returns SC_NONE); rows [lvl+5, lvl, lvl+5] with n = 2 return SC_NONE (the original; the mutant `y0 <= lvl` returns 4096); rows [lvl-4, lvl, lvl+4] with n = 2 return 4096 (the original; the mutant `y1 > lvl` returns SC_NONE); hand arithmetic, not run |
| 548 | `tn < tp` to `tn <= tp` in `sq_t_first` | equivalent | at a tie the original keeps t = tp, the mutant takes t = tn, and tn = tp |
| 549 | drop `if (t == SC_NONE) { return SC_NONE; }` in `sq_t_first` | test gap (group A, high value) | every test calls `sq_t_first` on a run that crosses 0.75 V, so t is never SC_NONE; without the guard the function returns SC_NONE - 2048 = 4294965247 (the half step of SQ_R = 1 subtracted from 4294967295). Missing assert: `sq_t_first(&out, 100)` on the +h run (160 ps, before the decision, the outputs are below 0.75 V at 50 to 500 ps) equals SC_NONE |
| 555 | `dp > dn` to `dp >= dn` in `sq_sign` | test gap (group A, high value) | the two differ only when the outputs are equal; both sign asserts (lines 956, 971) have a clear winner. Missing assert: on a hand-built `out` with dp = dn in the last row, `sq_sign` is false (the mutant says true) |
| 561, 564 | `while (j <= n)` to `j < n` (561), `y0 < SC_VCM and y1 >= SC_VCM` to `y0 <= SC_VCM and ...` and to `... and y1 > SC_VCM` (564, 2 mutants) in `sq_t_res` | test gap (group A, high value) | same three cases as `sq_cross`, on |dp - dn|: the real run crosses 0.75 V in the interior, on no sample. Missing asserts, on a hand-built `out` with dn = 0 words and dp = the listed values: dp rows [VCM-8, VCM-4, VCM+4] with n = 2 return 4096 (the original; the mutant 561 returns SC_NONE); rows [VCM+5, VCM, VCM+5] with n = 2 return SC_NONE (the original; the mutant `y0 <= SC_VCM` returns 2048); rows [VCM-4, VCM, VCM+4] with n = 2 return 2048 (the original; the mutant `y1 > SC_VCM` returns SC_NONE); hand arithmetic, not run (the half step 2048 is subtracted, so the crossing must lie after it) |
| 562 | the sample index of y0 in `sq_t_res`: `dp[j-1]` to `dp[j]` (drop-step), `dp[j-1]` to `dp[(j-1)/SC_NN]` (swap-arith), `dn[j-1]` to `dn[j]` (drop-step), `dn[j-1]` to `dn[(j-1)/SC_NN]` (swap-arith); 4 mutants | test gap (group B, high value) | y0 enters only the interpolation `frac = (SC_VCM - y0) / (y1 - y0)` of the crossing interval, not the test of the crossing for a clear-cut run, so a wrong y0 moves the resolution time by a fraction of one step. The time sits at 650.94 steps (1040.7 ps) and the interval fraction is about 0.94 (hand arithmetic, not run), so frac can rise by at most 0.06 step = 0.096 ps (it cannot pass 1) and the two `dp[j]`, `dn[j]` mutants move it by a fraction of a step's change of |dp - dn|, about 0.04 to 0.1 ps; the window `tr > 1040.6`, `tr < 1040.8` is 0.2 ps wide, wide enough to hold such a shift when the true value is near its middle, and the run is not fold-pinned for this function. Missing assert: narrow the window to 0.01 ps around the value (as the decision-time asserts are) or pin the integer `sq_t_res(&out, SQ_NL)` with `assert_eq`; the value comes from the lab |
| 586 | `dt * SQ_R / 2.0` to `dt / SQ_R / 2.0` in `sq_node_at` | equivalent | (SQ_R as f64) is 1.0, so `dt * 1.0` and `dt / 1.0` are the same f64 |
| 602, 655 | `d > worst` to `>=` (602, `sq_node_err`), `e > worst` to `>=` (655, `sq_jac_err`) | equivalent | at d = worst the mutant assigns worst = d, and d = worst, so the maximum is unchanged |

Counts of the 40 survivors: 17 equivalent (162, 163, 166 and 167 x3 = 6; 202; 247, 251, 255; 347, 350; 484; 548; 586; 602, 655), 19 test gap (group A: 530, 533 x2, 549, 555, 561, 564 x2 = 8; group B: 562 x4 = 4; group C: 307, 311, 314, 315 = 4; group D: 348, 351 = 2; group E: 369 = 1), 4 dead line (164 x3, 388), 0 not decided.

Not run, so inferred: for group B, the position of the crossing in its interval (frac about 0.94) and the 0.04 to 0.1 ps shifts are read from the test's 1040.5 ps comment and the 1.6 ps step, not computed from the samples; the bound of 0.096 ps is the part that does not depend on them; for group A, the expected words (2048, 4096, SC_NONE, 4294965247) are hand arithmetic from the function bodies; for groups C, D and E, the lab must confirm that the forward and backward differences, the cb word and the SWAPQ fold actually differ at the chosen inputs, before an assert is written; for the equivalent class, that `sc_sadd(q, 0)` is q and `sc_pack(0, .)` is 0 is read from the helpers in `sar_comparator.t27`, not run. None of these changes a class, except that a lab finding that cb is zero at the kink in group D would move 348 and 351 to equivalent.

Follow-up (one issue per test-gap group, highest value first; none filed here):

1. "analog: sq_cross, sq_t_first, sq_sign, sq_t_res on hand-built waveforms (last interval, level on a sample, no crossing, equal outputs)" -- group A, kills 8 mutants (530, 533 x2, 549, 555, 561, 564 x2); the expected words are chosen by hand in the test, no lab value is needed.
2. "analog: pin the resolution time of the long run in sar_comparator_q (narrow the 0.2 ps window or assert_eq the integer)" -- group B, kills 4 mutants (562 x4); one value from the lab.
3. "analog: sq_fd one-sided differences at the supply edge (x, y, vsb = VDD - SQ_DV and vsb = VDD)" -- group C, kills 4 mutants (307, 311, 314, 315), low value; direct calls, the inputs from the lab.
4. "analog: body slope at the kink vsb = 0 in sq_dev (low end exactly on SQ_OFF, high end exactly on SQ_VDD)" -- group D, kills 2 mutants (348, 351), low value; may turn out equivalent if the lab finds cb zero.
5. "analog: pin the SWAPQ negative control of sar_comparator_q (fold s4 or the header's residual and time)" -- group E, kills 1 mutant (369), low value; the value from the lab.

The two dead-line decisions (164 x3: the falling ramp for SQ_R > 1; 388: the jac-false early return) need an owner's call, keep or delete, not an assert.

## 9. raw_writer (measured 10:39-10:55 +07, exit 0, 15 min 50 s)

526 mutants, 412 killed (all by a failing test), 83 survived, 31 hung, 0 unviable. Kinds: drop-guard 44, flip-cmp 148, swap-logic 32, drop-step 80, swap-arith 187, ret-default 35. Run: `tri mutate spec` on `specs/analog/raw_writer.t27`, 3 at a time, zig -j2 each; the census ran with t27c 0.5.2, an older build than the tree's own, and the tree was at commit 035df5ce60. Raw output: kept outside the repo. The spec is unchanged between the census base and master (a `git diff` of the path is empty), so the line numbers are the census's. The classes are from reading the spec; nothing was re-run. The 83-row table (line, mutant, class, why), the proposed tests and the proposed issues are in `docs/analog/census-rest-2026-10-25/raw_writer-classification.md`.

Facts the table leans on: `raw_sig` works in two scratch buffers, `a` (90 limbs) and `d` (810 digits), that every test and the writer reuse, so after a long number the cells d[16] and d[17] hold stale digits; the test buffers are oversized and zero-filled and no test fills a buffer exactly or overflows one, so an off-by-one at the end of the data is invisible unless it changes a result; the binary byte-for-byte test calls `raw_at_hex` only positively, where its ASCII twin changes one byte and asserts the compare turns false, so a `raw_at_hex` that returns true whenever the bytes fit passes every assert in it; the multi-line `if (...) { inf.err = ...; return inf; }` blocks of `raw_check` are not mutated by the census, so they cannot be listed and their absence is not a finding. The mutants that read or write one cell past the end are killed only if the build traps on it; that the tests run as Zig Debug tests, where an out-of-range index, an unsigned underflow or a divide by zero panics, is inferred from earlier censuses, not read (group J below rests on it).

The 31 hung are 30 drop-step mutants of loop counters and cursors (`x = x + 1` becomes `x = x`, lines 136, 176, 235, 242, 280, 309, 364, 368, 380, 391, 477, 500, 546, 549, 562, 609, 629, 650, 658, 668, 690, 719, 722, 725, 804, 859, 877, 888, 919, 939) and 1 flip-cmp (line 139, `while (carry > 0)` to `>= 0`, which cannot end); each loop never ends, so they are the mutant's hang, not a test gap.

Counts of the 83 survivors: 19 equivalent (79; 157, 163, 167; 228; 240; 297; 333, 338, 347; 645; 665; 683 flip; 690; 719; 720 flip; 722; 723 flip; 917), 62 test gap in ten groups (below), 2 dead line (683 drop, 954), 0 not decided (19 + 62 + 2 = 83).

| Group | Mutants | What is missing | Basis of the expected values |
| --- | --- | --- | --- |
| A | 6 | `raw_at_hex` negative control and exact-size cases (867 x3, 873 x2, 876) | hand-derived; the lab confirms them first |
| B | 14 | malformed variable lines for `raw_check_vars` (715 x2, 717 x2, 720 x2, 723 x7, 726); six hand-built lines | hand-derived; the lab confirms them first |
| C | 2 | `raw_put_f64` of zero and `raw_from_bits` of the non-finite patterns (301, 355) | hand-derived; the lab confirms them first |
| D | 8 | header counts and numbers at their limits (`raw_uint`, `raw_count_line`; 624 x2, 628, 629, 630, 636, 638 x2) | hand-derived; the lab confirms them first |
| E | 7 | ASCII body shape, truncated and malformed point lines (`raw_check_ascii`; 685 x2, 688, 691 x2, 701 x2); four tiny hand-built bodies | hand-derived error codes; the lab confirms them first |
| F | 11 | reader helpers on tight ranges (`raw_at`, `raw_eol`, `raw_has`, `raw_uint`, `raw_value_ok`); low value | hand-derived |
| G | 2 | `raw_sig` must not read stale scratch digits (230, 233); two calls with pre-set d[16] and d[17] | hand-derived; the lab confirms them first |
| H | 1 | the scale of the RLC current column (938) | needs a lab value: the peak current of 5.5 mA is an estimate from the circuit, not from the trace |
| I | 2 | `raw_sig` carry-out of sixteen 9s (258 x2) | needs a lab value: a double whose 16 rounded digits are all 9; none is named, it is a lab search |
| J | 9 | exact-size and short-input calls of the writers (140, 270, 387 drop, 543 x2, 560, 571, 715 flip, 873 drop); low value | inferred: rests on the trapping Debug build; if the build does not trap, these 9 rows become equivalent (28 equivalent, 53 test gap) |

Not run, so inferred: every missing-assert value is hand arithmetic from the function bodies (digit counts, byte lengths such as 13 for a variable line and 21 for the zero text, the error code each mutant would return), labelled hand-derived and unverified; the lab must confirm each before an assert is written. The equivalent class rests on reading the helpers (`raw_pow(base, 0)` is 1, `raw_mul` by 1 changes nothing, `raw_eol` does not read when from >= n, the high bit of m at be = 1), not on running them. A lab finding that a group F or G assert cannot be made to differ would move that row to equivalent.

Follow-up: one issue per group, highest value first; the groups are filed as #12273 (groups A to D) and #12274 (groups E to J and the two dead lines). The two dead-line decisions (683: the early `at >= n` return that line 685 already covers; 954: the fold guard in `raw_rlc_file`, which no call can reach) need an owner's call, keep or delete, not an assert; the default taken is to keep both lines and list them until the owner decides.

## 10. linear and linear_zoh after the audit (re-run on master 1c783b0b31)

Both specs were first measured on the census base 035df5ce60 (linear_zoh 10:15-10:25 +07), before the audit #12191 changed their tests. They were run again on master 1c783b0b31: linear at 11:23-11:36 +07, linear_zoh at 11:36-11:44 +07. The raw outputs and the pre-audit classifications are kept outside the repo; the reading of the second runs is in `docs/analog/census-rest-2026-10-25/linear-post-audit-classification.md`, and it is the text of that file, not a new run, that decides every class below. In that file "the old file" is the pre-audit classification.

Counts are mutants: killed / survived / hung / unviable.

| Spec | Post-audit run | Pre-audit run | Kinds |
| --- | --- | --- | --- |
| linear | 316 / 61 / 38 / 5 of 420 | the same | drop-guard 10, flip-cmp 88, swap-logic 14, drop-step 45, swap-arith 229, ret-default 34 |
| linear_zoh | 195 / 30 / 27 / 0 of 252 | 193 / 30 / 29 / 0 | drop-guard 1, flip-cmp 33, drop-step 34, swap-arith 171, ret-default 13 |

What the audit changed in the two specs: for linear, tests only (a `git diff` of the two trees for the path is 33 insertions and 13 deletions, all in the test blocks, lines 790-862 of the new file; the asserts on constant table elements now read `var` copies first, and the comment cites #12166); no line of the functions (lines 1-717) changed or moved, so every survivor keeps its line number, and the changed asserts read constants, not code the mutator edits, so killing none of the 61 is what the diff predicts. For linear_zoh, the diff touches lines 637-725 only. The two raw outputs of linear are byte-identical; the 30-survivor block of the two raw outputs of linear_zoh is byte-identical. Result: every survivor of the post-audit runs is a survivor of the pre-audit runs (61 and 30), so every old verdict stands and no class was reopened.

| Spec | Equivalent | Test gap | Dead line | Not decided | Survivors |
| --- | --- | --- | --- | --- | --- |
| linear | 18 | 35 in six groups (A 10, B 6, C 6, D 5, E 4, F 4) | 7 (441 x3, 442 x4) | 1 (463 `>=`) | 61 |
| linear_zoh | 2 (176, 433) | 28 in five groups (A 2, B 2, C 8, D 15, E 1) | 0 | 0 | 30 |

Group letters are per file (the A of linear is the stamp shapes, the A of linear_zoh is the exact-grid twin). "Open" means the group still has survivors in the post-audit run: all six groups of linear and all five of linear_zoh are open, none closed. Later lanes are not in scope.

The hung and the unviable. linear: 38 hung, all drop-step mutants of the form `x = x + 1` to `x = x`, each the counter of the `while` that encloses it, so the loop never closes (the same 38 lines as in the pre-audit run); 5 unviable, all `[ret-default]` of a function with no return value (`step` 277, `bump` 392, `stamp_pair` 397, `stamp_src` 407, `matmul` 654), each reading "zig: error: not yet implemented"; that the cause is on the emitter side and not in this spec is not established here. linear_zoh: 27 hung, all drop-step counters (the same 27 lines as before), 0 unviable.

Two mutants of linear_zoh moved from hung to killed: 452 and 468 (`const m : u32 = 1 << j;` to `1 >> j` in `rlc_ex_err` and `rc_ex_err`). Before: 29 hung, namely the 27 counters plus these two; now 27 hung and 195 killed (193 + 2). They were never survivors, so no verdict is touched. Why they moved is not established. Facts: the code of both functions and of everything they call is identical in the two trees, and the test that reaches them (the grid test) is not in the diff; the machine load at the start differed (load averages 13.13, 20.07, 19.87 for the pre-audit run, 7.43, 8.60, 12.03 for the post-audit run). A candidate reading, not run and unverified: at j = 1 the mutant makes m = 0 and dt infinite, the loop `while (nrm > 0.5)` of `pade_expm` halves an infinite norm, and its u32 counter `s = s + 1` ends the loop by overflow after 2^32 passes if the build traps overflow, an end that arrives at a time which depends on the load against `--timeout`; that also rests on the inferred trapping build. Whether a third run puts them back among the hung: not established.

Not established: every expected value in the file is hand arithmetic from the function bodies and none was run, so each should be confirmed on the lab before an assert is written; that the mutant build traps on `usize` underflow or out-of-range indexes (linear mutants 401 and 408, `- col`; linear_zoh group D, 15 mutants); the IEEE sign-symmetry argument for 393 and the width of the f64 boundary band for 574; whether a pivot tie (463 `>=`) can change any bit of a solved matrix; whether any other spec calls `stamp` or `derive` on a table that `table_ok` refuses (a repo-wide search for callers of `AnalogLinear` was not completed), which would move 441 and 442 from dead line to test gap; the linear_zoh group C expected values for the `ring_fixed` rows (541, 548, 550), which still come from a lab run and were not derived.

Follow-up: #12275 (linear, groups A to F) and #12276 (linear_zoh, groups A to E). The dead lines 441 and 442 of linear are unreachable for any table that `table_ok` accepts and need a decision, not an assert: delete them or lift the refusal; if the owner prefers to keep them, the hand-derived checks are in section 4 of the classification file.

## 11. Where each written-up spec stands

Counts are mutants: killed / survived / hung / unviable. "Census" is the first run (tree `035df5ce60`) for every spec except linear and linear_zoh, whose census cell is the re-run on master `1c783b0b31` after the audit (section 10); "gate" is the survivor gate of the pull request for the follow-up issue, run on the lane rebased on master `1c783b0b31`.

| Spec | Census | Follow-up | State |
| --- | --- | --- | --- |
| sar_logic | 526 / 108 / 31 / 12 of 677 | #12163 | classified by reading (27 / 80 / 1); no test added |
| sar_trace_raw | 38 / 1 / 3 / 0 of 42 | none | nothing to fix: 1 equivalent, 3 endless loops |
| vacask_compare | 34 / 8 / 6 / 0 of 48 (pre-audit tree) | #12182 (PR #12204) | gate on master: 40 / 2 / 6 / 0; the 6 test gaps are killed, 2 equivalent and 6 hung are accepted |
| bench | 55 / 3 / 0 / 0 of 58 | none | 3 equivalent, no test gap |
| mos_psp103_gates | 75 / 10 / 9 / 0 of 94 (pre-audit tree) | #12187 (PR #12206) | gate on master: 76 / 9 / 9 / 0 |
| mos_psp103_inv20 | 72 / 23 / 5 / 1 of 101 | #12185 (PR #12207) | gate on master: 81 / 14 / 5 / 1; see the line below the table |
| sar_comparator_qi | 256 / 20 / 18 / 14 of 308 | #12186 (PR #12205) | gate on master: 262 / 14 / 18 / 14 |
| sar_comparator_q | 266 / 40 / 17 / 13 of 336 | to be filed | 17 equivalent, 19 test gap in five groups, 4 dead line; nothing run |
| raw_writer | 412 / 83 / 31 / 0 of 526 | #12273, #12274 | 19 equivalent, 62 test gap in ten groups (J inferred, H and I need a lab value), 2 dead line; nothing run |
| linear | 316 / 61 / 38 / 5 of 420 (post-audit tree, same as pre-audit) | #12275 | 18 equivalent, 35 test gap in six groups, 7 dead line, 1 not decided; nothing run |
| linear_zoh | 195 / 30 / 27 / 0 of 252 (post-audit tree; pre-audit 193 / 30 / 29 / 0) | #12276 | 2 equivalent, 28 test gap in five groups, 0 dead line; 452 and 468 moved from hung to killed, cause not established; nothing run |

mos_psp103_inv20, in words: of the 23 survivors, 9 are killed by the five added tests and 14 are accepted (13 equivalent mutants in 12 entries, plus line 176). Line 173, which the first draft of the note had accepted as equivalent, is killed by the exact-mirror test, so its accepted entry was removed; the first gate run exited 2 on that stale entry and the second run (11:13-11:16 +07) passed. The forecast written before the run was 80 / 15 / 5 / 1.

## 12. What is not claimed

- **Six specs have no measured run:** sar_switch (a run started and was killed by the load rule, section 13), mos_table, accept_sar, sar_kick_net (727 mutants; an earlier run was stopped at 08:25 and its output file is a stub), sar_comparator and mos_psp103. Their numbers are "not measured".
- **linear and linear_zoh were measured twice.** The first runs were on the census base `035df5ce60`; `bf66e458aa` (#12191, 151 asserts on constant table elements now count) changed both specs afterwards, so section 10 uses the re-runs on master `1c783b0b31`. The same commit changed vacask_compare, mos_psp103_gates, mos_table, sar_comparator, sar_kick_net, sar_switch and mos_psp103, and `7fefa30eaa` (#12153) changed sar_kick. Sections 3 and 5 are therefore the pre-audit counts of vacask_compare and mos_psp103_gates; the gate rows of section 11 are the current ones. The re-runs found the same survivors as the first runs (61 and 30), so the classes are the first runs' classes, read a second time; the only counts that moved are the killed and the hung of linear_zoh (193 to 195 and 29 to 27, section 10), and why they moved is not established.
- **raw_writer was measured on an unchanged tree** (a `git diff` of its path between the census base and master is empty) and is written up in section 9. Its test-gap groups H and I need a value from the lab, and group J is inferred: it rests on the premise that the tests run in a build that traps on an out-of-range index, an underflow and a divide by zero, which nothing here confirms.
- The specs this note classifies on the census base (sar_logic, sar_trace_raw, bench, mos_psp103_inv20, sar_comparator_qi, sar_comparator_q, raw_writer) are byte-identical between the census base and master (checked with `git diff --quiet` on these paths) except two: master has since gained only added lines in mos_psp103_inv20 (150, #12207) and in sar_comparator_qi (99, #12205), the tests of the follow-up issues. The survivor lines in sections 6 and 7 are the census's, and the gate rows of section 11 are the current state.
- **Classes come from reading, not from a run:** the 108 sar_logic survivors and the 40 sar_comparator_q survivors were read off the spec by subagents and spot-checked by the orchestrator; the 83 raw_writer survivors and the 91 survivors of linear and linear_zoh (61 and 30) were classified the same way, and their notes record no spot-check. The expected values of the proposed asserts are hand-derived and stay unverified until a T27 OS pane runs them. An "equivalent" is an argument, not a proof by a tool.
- **Hung is not proven endless.** The tool reports only that the tests outlived `--timeout 30`. Each hung mutant listed is a dropped counter step or a bisection condition that never closes, which is endless by construction, and none was re-run with a longer limit. The one measured exception is in section 10: two linear_zoh mutants (452, 468) were hung in the first run and killed in the second, so a hang is a statement about one run, and a hung mutant that is not a dropped counter step is not shown to be endless.
- **Compiler and load.** Every census run on the census base used one `t27c`, version 0.5.2; the compiler binary of the two re-runs on master (linear, linear_zoh) is not recorded in their notes. The 0.5.2 build is an older build than the tree's own (it predates the gen-zig and gen-rust commits now on master); no verdict was re-run with a current binary. A compiler build from another session ran beside the middle runs, and the 1-minute load was 24 to 46 during them, so a mutant that was only slow could have been counted as hung. Under a current binary exactly the spec seals of the follow-up lanes move, which is a separate check from the mutant verdicts.
- No speed claim: the wall times are single runs under load, not a rate.

## 13. Why the runs stopped

First stop, 08:25. Free space on the data volume fell from 7.4 GiB (07:59) to 0.4 GiB (08:25) while two other trees on this Mac held a large part of it: `/private/tmp/opencode` 9.08 GiB (87 entries) and the user temp directory `opencode` 8.81 GiB (131 entries), both read with `du -sk` at 08:31. Those belong to another tool's session and were not touched. The run on sar_kick_net was stopped by hand (kill threshold 2.5 GiB).

Second stop, between 09:10 and 09:20. The third spec (vacask_compare) printed its mutant count and then no process was left: the lane worktrees `lane-12070` (it held the `tri` and `t27c` binaries the run used) and `lane-12116`, with about fifty other worktrees, were removed by something other than this loop, and the T27 OS app was relaunched around 09:18. The branch was already on origin, so no work was lost. The census resumed at 09:34 from a recreated lane with the `tri` and `t27c` built in the main build tree, when the volume had more than 4 GiB free and the load had fallen; vacask_compare, bench, mos_psp103_inv20, mos_psp103_gates, sar_comparator_qi and sar_comparator_q were then measured, mostly one after another.

Third stop, 11:08. The 1-minute load, driven by compiler, synthesis and `claude -p` jobs of other sessions, rose to 75 to 116 at 11:07 and 11:08. The kill rule (load above 60) ended the census pane while sar_switch was running; it left a start time, the load at the start and a partial output, and no result is claimed for it. By 11:13 the load was 4.5 and the follow-up gates and seals ran.

After the third stop, linear (11:23-11:36 +07) and linear_zoh (11:36-11:44 +07) were re-run on master `1c783b0b31`, one after the other (section 10). The 1-minute load at the start of the linear_zoh run was 7.43 (5-minute 8.60, 15-minute 12.03); the load at the start of the linear run is not recorded in its note. raw_writer was measured earlier, between the second and the third stop (10:39-10:55 +07, section 9).

Resuming: when the 1-minute load is under 15 and the volume has more than 4 GiB free, the census is run on master for the six unmeasured specs, one pane at a time. Each result is posted on #12116 before the next run starts.
