# Mutation test gaps of specs/analog/accept_sar.t27, 2026-10-25

Issue #12431, part of epic #10486 (weak spots of the analog v0.1 release). This lane wrote the missing asserts, the
accepted file and this note. The orchestrator then ran the mutation gate (it passed, section "The accepted file") and
saved the seal (section "Not done"). Where each number comes from:

- **Census** (281 mutants, 218 killed, 58 survived, 4 hung, 1 unviable) is the analog-loop census of the unmodified
  spec on 2026-10-25, run by `tri mutate spec`; the census spec is commit 89a747eefc (1787 lines). The output records
  neither the run time nor the t27c version. The 58 survivors were classified by reading the spec after that census.
  The writing of the tests re-ran no census mutant; the gate run in section "The accepted file" re-ran all 281.
- **Tests** are measured: `tri test specs/analog/accept_sar.t27` in a T27 OS pane on this tree (base origin/master
  1b1a27d4d8 plus 209 added lines, all after line 1787, so no existing line moved): tests 26, pass 26, FAIL 0,
  vacuous passes 0 of 26.
  The 17 existing tests are unchanged (480 counted asserts); the 9 new tests count 69 asserts, 549 in all.
- **Kills** are measured one mutant at a time with `tri mutate plant` (rules: `specs/tri/mutate/plant.t27`), not by the
  gate: it plants one hand-written mutant on a copy of the spec, runs the tests on the copy, names the failing tests,
  and checks that the original file has the same sha256 before and after. 23 mutants were planted, one per census
  row of group A to I, each with the exact text of the census row (the text planted is shown on the planted line of the
  plant output): 23 killed, 0 survive. Every mutant turned red exactly the test named in the tables below. Which
  assert inside that test fails is a reading of the mutated text against the assert, written next to each assert as a
  comment.
- **Expected values** in the new asserts are not invented: the equality tests use a limit read into a `var` (the
  `fabs64` of linear.t27 is exact, so the value ties the limit to the last bit), the builders are fed hand-built
  structs whose other fields come from the real model run, the sums of group C are small integers, and groups H and
  I compare model results with each other or bound them by the ideal arithmetic of the formula. No lab value was
  needed and none was invented.

Kinds among the 58 survivors: flip-cmp 38, swap-logic 12, swap-arith 7, drop-guard 1.

## The 58 survivors, by label

The census labelled 34 equivalent (27 of them "not constructed"), 23 test gap and 1 dead line. This lane keeps those
labels and kills all 23 test gaps:

| Outcome | Count | Lines | Entry in the accepted file |
| --- | --- | --- | --- |
| equivalent, accepted (6 ties, 1 implied) | 7 | 485, 488, 705, 710 (flip-cmp), 711, 745, 851 | yes |
| not constructed, accepted | 27 | 473, 585, 593, 601, 685; 771, 780, 813 (2), 845 (3), 848 (3), 862 (3), 873, 874, 875 (4), 876, 885 (2) | yes |
| dead line, kept | 1 | 651 | yes |
| test gap, killed | 23 | groups A to I below | no |

7 + 27 + 1 + 23 = 58. Killed in all: 23; not killed: 35.

### Test gaps, killed by the new tests (23)

| Group | Mutants | New test (the one that fails) | What fails on the mutant |
| --- | --- | --- | --- |
| A, the checks are closed bounds (8) | 270; 300, 305, 310; 380; 385 (2); 415 | `the_checks_accept_a_value_exactly_at_their_limit_and_refuse_one_just_past_it` | each check is fed its own limit, read into a `var` from the `L_*` constant, and must say yes; the `<=` or `>=` mutant says no. `as_within(0.25, 0.25)` and with -0.25 (270), `as_chk_tg_ron` at 30000, `as_chk_tg_worst` at 50000, `as_chk_tg_leak` at 1.0, `as_chk_clk_ratio` at 1.2, `as_chk_clk_stop` at 0.0 and at 30.0 (385 has two mutants, one for each end), `as_chk_int_err` at 64.0. Each check also refuses a value just past its limit, so the test does not pass by accepting everything |
| B, the sign check (1) | 335 | `the_comparator_sign_check_needs_dp_to_win_at_plus_2_mv_and_dn_at_minus_2_mv_both_together` | `as_chk_cmp_sign` is true only for (1, 0); (1, 1) and (0, 0) are refused, and so is (0, 1); the `or` mutant accepts (1, 1) and (0, 0) |
| C, the tally (1) | 915 | `the_tally_of_rows_adds_the_fail_count_of_a_block_and_the_tally_of_blocks_counts_a_failing_block_as_one_fail` | `as_tally(Blk{1,5,1}, Blk{2,3,4}, true)` is pass 3, fail 8, nm 5; the `t.fail - b.fail` mutant gives fail 2. 5 and 3 keep the subtraction from underflowing a `u32`, so the mutant fails on a value and does not trap. With `rows` false the same blocks give pass 1, fail 6, nm 1 |
| D, the comparator block (4) | 825, 826 (2), 827 | `the_comparator_block_refuses_a_gap_equal_to_its_bar_and_a_reset_that_is_not_flat` | a `Cmp` built from `as_cmp(0)` with one gap set to its bar (`SQ_BAR_LEVEL_V` for the levels and the reset, `GT_B_REL` for the delay) is pass 3, fail 1, nm 0 (the bar is strict: `g < bar`); the `<=` mutant passes the row. A reset left high (`rst_p_v` 1.5 V) with a gap of 0.0 is refused by `as_chk_cmp_reset`, so the row is FAIL; the `and` to `or` mutant at 826 passes it |
| E, the tgate block (3) | 789, 790, 791 | `the_tgate_block_measures_a_row_whose_gap_to_ngspice_equals_the_bar` | a `Tg` from the real `as_tgate(&TT_LG_N, &TT_LG_P, true)` with one gap set to the bar is pass 3, nm 0 (the bar is closed: `g <= bar`); the `<` mutant makes that row NOT MODELLED (nm 1). A gap of twice the bar is still nm 1 |
| F, the inverter delay row (2) | 756 (2) | `the_inverter_delay_row_refuses_a_gap_equal_to_its_bar_for_either_delay` | `as_blk_inv(0.75, InvDly{10.0, 10.0, g_fall, g_rise})`: with both gaps 0.0 it is pass 2, fail 0; with `g_fall` or `g_rise` equal to `GT_B_REL` it is pass 1, fail 1 (two literals, because the row is a conjunction); the `<=` mutant passes the row |
| G, the DFF levels row (1) | 804 | `the_dff_levels_row_is_a_pass_when_the_gap_to_ngspice_equals_the_bar` | `as_row_dff_levels(DqLv{worst_v: 1 uV, gap_v: GT_B_LEVEL_V, n_in: 8})` is AS_PASS; the `<` mutant gives AS_NM. Twice the bar is AS_NM in both |
| H, the trim decode (1) | 519 | `the_clock_trim_code_reaches_the_delay_through_bits_b_and_c_in_the_stimulus` | `as_clk_delay_ps(2)` differs from `as_clk_delay_ps(0)` and `as_clk_delay_ps(4)` differs from `as_clk_delay_ps(6)`, all four above 0. The mutant decodes code 2 as 000 and code 4 as 110, so the stimuli are the same pairs. Only the order of the comparison is asserted; no delay value is pinned |
| I, the ramp helper (2) | 708, 710 (swap-arith) | `the_ramp_helper_takes_the_smallest_step_between_conversions_only_and_in_both_directions` | a ramp from -1.4 V in steps of 2/7 V has a first code under 100 (ideal 69) and a step above 150 (ideal 195); with `i >= 0` the first code is counted as a step and the step falls under the first code. A descending ramp from 1.0 V has `first > last` and a step between 150 and 250; with `0.0 + d` the negative differences stay negative and the step is below 0 |

The counted asserts per new test are 17, 4, 6, 15, 7, 6, 2, 6, 6 in the order of the tests in the spec (69 in all).
Every value read from a constant is read into a `var` first, so no assert is folded at compile time and none is
vacuous.

A correction to the classification file, not to a class: for 708 it says both ramps start at code about 512 with a step
of about 154; that is the report-like ramp (`as_ramp_report_like`). The literal ramp (`as_ramp_literal`, from -1.0 V)
starts at code 333 with a step of about 198, which the existing asserts at lines 1677-1679 pin. Both start well above
the smallest step, which is why the `i >= 0` mutant survived: its first "difference" was larger than every real step.
The -1.4 V ramp of group I is the one that tells it apart.

### Equivalent (7), accepted

Six are ties that assign the value the variable already holds; one is implied for all inputs by the next line.

| Line | Mutant | Why it is equivalent |
| --- | --- | --- |
| 485 | `g_lo > g_lv` to `>=` in `as_cmp` | at a tie the mutant assigns `g_lv = g_lo`, the same value |
| 488 | `g_rn > g_rv` to `>=` | at a tie the mutant assigns `g_rv = g_rn`, the same value |
| 705 | `e > worst` to `>=` in `as_ramp` | at a tie the mutant assigns `worst = e`, the same value |
| 710 | `d < 0.0` to `<=` | at d = 0.0 the mutant computes `0.0 - 0.0`, which is 0.0 |
| 711 | `d < smallest` to `<=` | at a tie the mutant assigns `smallest = d`, the same value |
| 745 | `g_in > g_leak` to `>=` in `as_tgate` | at a tie the mutant assigns `g_leak = g_in`, the same value |
| 851 | `as_chk_sw_droop(d_lo) and as_chk_sw_droop(d_hi)` to `or` in `as_blk_sw_q` | line 852 also requires `as_within(d_lo, 1 uV)` and `as_within(d_hi, 10 uV)`, each tighter than the 5 mV droop limit, so the row is the same for all inputs |

Line 710 holds two survivors: the `flip-cmp` is accepted here, the `swap-arith` is group I and is killed.

### Not constructed (27), accepted

No input is known that tells the mutant from the original. Not proven equivalent; a witness would turn each into a test
gap. 22 of them sit in a builder that takes no argument (`as_blk_nand`, `as_blk_nor`, `as_blk_dff`, `as_blk_switch`,
`as_blk_clock`, `as_blk_sar_logic`) or only the two negative-control knobs (`as_blk_sw_q`): it makes one model run, so
an appended test cannot move its guard, and each guard has the same truth value in that run (for example `slow_ps > 0`
to `>= 0` on a `usize`, which is a tautology, and the run gives about 16145 ps). The other 5 are inferred from the
survival of the census.

- **473** `dp > dn` to `>=` in `as_cmp`: differs only at dp == dn. In the +2 mV run dp is 1.49933 V and dn 2.26 mV.
  Only the NOCHARGE mode (both outputs near 0.786 V) might tie, and its `win_p` is never asserted.
- **585** `n < 11` to `<=` in `as_sar_run`: the extra pass n = 11 asks for the same 11 ones as n = 10 and sets no flag.
- **593** `tr + 18000` to `tr - 18000` in `as_sar_run`: the second `sl_run` asks for an earlier time and is a no-op, so
  the hold check re-reads the state of the edge check.
- **601** `t11 != NONE and t1 != NONE` to `or`: both edges are found in both runs (swapped and not).
- **685** `before == 0 and reset == 0` to `or` in `as_dff_set_reset`: the only knob is `swapped`, and both have the same
  truth value in both runs.
- **771, 780, 813 (2), 845 (3), 848 (3), 862 (3), 873, 874, 875 (4), 876, 885 (2)**: the builders above. Their gaps are
  not at a tie (nand 2.36 uV and nor 1.04 uV against a 10 uV bar; the hold gaps +2.4 and +9.4 percent against 10;
  `fast` 9.822 ns), the flags are all true in the one run, and `slow / fast` to `slow * fast` (875) passes either way
  because `as_chk_clk_ratio` is one-sided (`r >= 1.2`; the ratio is 1.64 and the product 158.6).

"No witness" means not found by reading the spec and the figures its tests pin; it is not a proof that none exists, and
nothing in this lane was run to look for one.

### Dead line (1), kept

**651** `if (r1 == NONE) { return 0; }` in `as_clk_delay_ps`: r1 is found for trim codes 0 and 7, the only codes any
caller passed before this lane, and for 2, 4 and 6, which the group H test now passes (each delay is above 0, so the
guard did not fire). Without the guard a missing edge would return `NONE - SL_NG_CLKS_F`. Keep or delete is the owner's
call, not an assert; the accepted file holds it as a dead line.

### Hung (4) and unviable (1)

The 4 hung mutants are all `x = x + 1` turned into `x = x` in the counter of the `while` that encloses it, so the loop
never ends, outlived the census's `--timeout`, and no test can go red: 441 (`as_inv_vth_of`, `k`), 542 (`as_thermo`,
`j`), 595 (`as_sar_run`, `n`) and 714 (`as_ramp`, `i`). The similar drop-steps at 528 (`as_stim`) and 552 (`as_high`) were
killed, probably by an out-of-range index or an overflow trap; that is inferred, not read from the output.

The 1 unviable mutant is 631, `[swap-arith]` in `as_clk_golden`: `f11 - (80100 + 10 * 40000)` becomes
`f11 - (80100 - 10 * 40000)`. The constant is -319900, which `usize` cannot hold, and zig refuses it at compile time:
the type rejects the mutant, it is not a gap. The gate lists it as "UNVIABLE -- not killed" and does not count it among
the 39 not killed.

## The accepted file

`docs/analog/mutation-accepted-accept_sar.txt` has 39 entries: 7 equivalent, 27 not constructed, 1 dead line (kept), 4
hung, in the form `path:line [kind] note` that `tri mutate spec --accepted` reads, one entry per mutant (the accepted
file of linear.t27 does the same). It has no entry for any of the 23 mutants the tests kill, and none for the unviable
mutant of line 631.

**Measured gate run** (`tri mutate spec --file specs/analog/accept_sar.t27 --accepted
docs/analog/mutation-accepted-accept_sar.txt`, the master-built compiler, `--jobs 2 --timeout 45`, 2026-10-25
22:21-22:42 +07, 20 minutes, 1-minute load 25-44): 281 mutants, 241 killed (all 241 by a failing test, 0 by an invariant
at compile time), 35 survived, 4 hung, 1 unviable; 39 not killed, 39 accepted, 0 accepted lines now killed; "Gate
passed", exit 0. The census had 218 killed, so the 23 new kills are exactly the 23 test gaps.

**The first gate run** (21:56-22:19, same options and tree, with a 40th entry for line 631) ended with exit 2: the
gate reported "1 accepted line(s) now killed" for `accept_sar.t27:631 [swap-arith]`. An entry for an unviable mutant
counts as killed, not as ignored, so unviable mutants must not be listed in the accepted file; the entry was removed
and the second run passed. An earlier reading of the gate's rule in this note (unviable entries are inert) was wrong
for this mutant.

## Caveats (what is by hand or not established)

- **The equivalents and the not-constructed rows** rest on reading the code and on figures in the census and in the
  header comments of the spec, not on a run.
- **Groups H and I** assert relations (different, above 0, under 100, above 150, between 150 and 250). They kill the
  mutants but pin no delay and no code; a lab run could pin one value later. The bounds of group I come from the
  ideal arithmetic `dv * 1023 / 1.5 + 1024` and are checked only by a green run, the real codes were not printed.
- **The plant is not the gate.** It ran the tests of the copy with the pane's `t27c`, which is slightly older than
  master's; a mutant the gate would report as hung or unviable would look different. All 23 compiled and ran to a test
  report.
- **Line numbers** are those of the census (spec of 1787 lines). `git diff 89a747eefc HEAD -- specs/analog/accept_sar.t27`
  is empty, so the base of this lane holds the census spec unchanged, and the added tests start after line 1787.

## Not done

- The seal of `analog_AnalogAcceptSar` was saved with the master-built compiler after the gate passed (save, verify and
  `tri seals changed` all exit 0); the new tests change the test count of the module.
- `tri coverage --strict` and the CI docs check were not run in this lane; CI decides.
- 651 keep or delete is the owner's call.
- The 27 "not constructed" survivors are accepted on reading the code, not on a constructed input; the gate only
  confirms that they survive.
