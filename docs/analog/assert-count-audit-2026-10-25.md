# Assert-count audit of 13 analog specs, 2026-10-25

Issue #12166, part of epic #10486 (weak spots of the analog v0.1 release). The defect, found in
`specs/analog/sar_kick.t27` and fixed there by PR #12153 (commit `7fefa30eaa`): an assert whose argument is a
constant table element, `assert_eq(TT50_N[24], 3)`, is folded by the compiler and `tri test` counts it as no assert at
all. The fix is to read the element into a `var` and assert on the var, `var n24 : u32 = TT50_N[24]; assert_eq(n24, 3);`.
This note records the same audit over the 13 other analog specs that carry pinned tables.

## What was run, and where each number comes from

Every number below is read from an output file of a `tri test` run, taken in T27 OS pane 4 (never on the Mac shell),
one command file per phase, each spec in its own `tri test` call with stdout and stderr to a file:

```
tri test specs/analog/<spec>.t27 --t27c <t27c>
```

`<tri>` and `<t27c>` are the binaries of the shared build tree (`tri` built Oct 23 17:46, `t27c` built Oct 24 09:59);
the commit they were built from is not recorded in this lane. The lane base is master `a6bdfefa5b`.

| Phase | Window (+07, 2026-10-25) | Load at start (1/5/15 min) | Free disk at start |
| --- | --- | --- | --- |
| Before (the 13 specs as on master) | 09:37-09:39 | 27.31 27.83 37.43 | 38 GiB |
| After (the 13 specs as in this change) | 09:51-09:53 | 13.46 16.05 24.30 | 40 GiB |
| Negative control, data side (12 scratch trees) | 09:58-09:59 | 16.15 15.42 20.83 | 38 GiB |
| Negative control, expected side (13 scratch trees) | 10:01-10:02 | 10.83 13.80 19.16 | 38 GiB |

The counted total of a spec is the sum of the per-test numbers under "runtime asserts executed, per test" in the
output. **The sums were added by hand from those lists, no script summed them.** Two checks guard the arithmetic: the
difference after minus before of each spec equals the number of asserts rewritten in it (below), and the sum of the 13
before-totals (86128) and of the 13 after-totals (86279) differ by 151, the number rewritten.

## Result per spec

"Candidates" is the line count of a read-only grep, a superset (it contains asserts the compiler counts already):

```
grep -nE '^[[:space:]]*assert[a-z_]*[ (].*[A-Z][A-Z0-9_]*\[' specs/analog/<spec>.t27
```

"Rewritten" is the number of asserts changed to read a var, and each of them is confirmed uncounted by the rise of the
counted total (see "Why these were uncounted"). Vacuous passes were 0 before and 0 after in every spec.

| Spec | Tests | Candidates | Rewritten | Counted before | Counted after | Delta |
| --- | --- | --- | --- | --- | --- | --- |
| `linear` | 14 | 13 | 13 | 132 | 145 | +13 |
| `linear_zoh` | 15 | 9 | 6 | 263 | 269 | +6 |
| `mos_psp103` | 40 | 9 | 1 | 21381 | 21382 | +1 |
| `mos_psp103_gates` | 9 | 13 | 10 | 134 | 144 | +10 |
| `mos_psp103_data` | 13 | 70 | 29 | 45766 | 45795 | +29 |
| `mos_psp103_50c_data` | 4 | 6 | 6 | 25 | 31 | +6 |
| `mos_table` | 21 | 4 | 4 | 5408 | 5412 | +4 |
| `sar_cdac` | 19 | 27 | 12 | 8520 | 8532 | +12 |
| `sar_comparator` | 20 | 11 | 11 | 2179 | 2190 | +11 |
| `sar_comparator_psp` | 16 | 24 | 16 | 276 | 292 | +16 |
| `sar_kick_net` | 24 | 43 | 8 | 1694 | 1702 | +8 |
| `sar_switch` | 14 | 40 | 23 | 207 | 230 | +23 |
| `vacask_compare` | 7 | 26 | 12 | 143 | 155 | +12 |
| Total | 216 | 295 | 151 | 86128 | 86279 | +151 |

All 13 specs pass before and after (no failing test, no BLOCKED).

## Why these were uncounted

Each rewrite replaces one assert by one assert (the element is read into a var on its own line, the expected value is
the one the replaced assert already held, all 13 specs pass after). In every spec the counted number of each test rose
by exactly the number of asserts rewritten in that test, and the spec total rose by the number rewritten (the table
above, 151 in all). A rewrite of an assert that was counted already would have left its test number unchanged, so the
rise is the measurement that the old form was uncounted, assert by assert, not a reading of the grep list.

The comment "Variables, not constants: an assert on a constant table element is folded by the compiler and counts as no
assert at all (#12166)." stands once before the first rewrite of each test.

## Negative controls

Two controls per spec, each on a scratch COPY of `specs/analog/` (the lane tree was not touched). Each tree differs
from the lane spec in exactly one line (checked with `diff`; each pattern occurs once in its file).

**Data side**, the control the issue names: one element of the constant table is made wrong. Every run fails, in the
test that holds the rewritten assert that reads that element (the failing set also holds other tests wherever the
code under test reads the same table, so this control shows the test goes red, not that the rewritten assert is the
one that fired).

| Spec | Element changed | Failing tests | The test holding the rewritten assert |
| --- | --- | --- | --- |
| `mos_psp103_50c_data` | `TT50_N[624]` 1271348 to 1271349 | 3 of 4 | red |
| `vacask_compare` | `PRE_T_PS[5]` 100.0 to 102.0 | 1 of 7 | red |
| `mos_psp103_gates` | `GT_NG_NOR[0]` 2.883448431801e-06 to 2.983448431801e-06 | 2 of 9 | red |
| `linear` | `RC_NV[0]` 798915 to 798916 | 4 of 14 | red |
| `linear_zoh` | `ZRC_NV[0]` 818234 to 818235 | 4 of 15 | red |
| `mos_table` | `EXP2NEG[0]` 16777216 to 16777217 | 1 of 21 | red |
| `mos_psp103` | `TT_N[624]` (in `mos_psp103_data`) 1349912 to 1349913 | 2 of 40 | red |
| `mos_psp103_data` | the same word | 3 of 13 | red |
| `sar_cdac` | `K_Q[0]` 524801 to 524802 | 5 of 19 | red |
| `sar_comparator` | `SC_CAP[5]` 32 to 31 | 16 of 20 | red |
| `sar_comparator_psp` | `NG_MM[0]` 0.588023 to 5.588023 | 1 of 16 | red |
| `sar_kick_net` | `QG_MID[0]` 2097152 to 2097153 | 1 of 24 | red |
| `sar_switch` | `SW_CAP[0]` 39342 to 39343 | 8 of 14 | red |

**Expected side**, the sharper control: the expected value of ONE rewritten assert is made wrong and nothing else is
touched. Since every test passes on the unmutated tree, the one test that goes red is the one holding that line,
which shows the rewritten assert is live at run time. In all 13 runs exactly one test failed and it is the test that
holds the changed line (the test names were matched against the spec's `test` headers by line number).

| Spec | Line | Changed to | The only failing test |
| --- | --- | --- | --- |
| `mos_psp103_50c_data` | 227 | `assert_eq(n24, 4)` | `the_tables_are_monotone_and_vanish_at_vds_0` |
| `vacask_compare` | 314 | `assert_eq(ng_top2, 1)` | `every_winner_of_both_comparators_is_the_same_in_vacask_and_ngspice` |
| `mos_psp103_gates` | 337 | `assert(ng_nand3 > 0.0000027)` | `the_nand_levels_are_microvolts_from_the_rails_the_solver_brackets_its_roots_and_every_level_is_within_3_uV_of_ngspice` |
| `linear` | 794 | `assert_eq(rc_nv0, 798916)` | `the_rc_constants_are_the_rounding_of_the_derived_ones` |
| `linear_zoh` | 644 | `assert_eq(zrc_nv0, 818235)` | `the_rc_exact_constants_are_the_rounding_of_the_derived_ones` |
| `mos_table` | 1350 | `assert_eq(noq_0, 1)` | `the_charge_path_with_zero_charge_tables_is_the_current_only_path_bit_for_bit` |
| `mos_psp103` | 2050 | `assert_eq(tt_n_corner, 1349913)` | `the_log_tables_at_the_nodes_of_the_linear_grid_are_the_linear_tables_words_one_model` |
| `mos_psp103_data` | 8282 | `assert ff_n624 < tt_n624` | `the_accessors_return_their_own_corner_and_the_corners_differ` |
| `sar_cdac` | 701 | `assert_eq(BRIDGE_LINE, last4 + 2)` | `the_inventory_is_the_decks_63_units_in_two_arrays_of_31_and_one_bridge_unit` |
| `sar_comparator` | 1550 | `assert_eq(cap5, 33)` | `the_units_the_devices_and_the_circuit_are_as_stated` |
| `sar_comparator_psp` | 416 | `assert ng_res6 == 1.0` | `the_times_against_the_table_reference_with_the_bulks_tied_to_the_sources` |
| `sar_kick_net` | 1489 | `assert_eq(qg_mid0, 2097153)` | `the_pinned_data_is_what_the_decks_printed` |
| `sar_switch` | 990 | `assert_eq(dd5, SW_VOUT + 1)` | `the_units_the_devices_and_the_circuit_are_as_stated` |

## Observed, not changed

These asserts read no table element, so they are outside the boundary of the issue. The compiler folds them by the same
rule (a pure constant expression), so they are expected to be uncounted too. **That is the rule's prediction, not a
measurement: no run isolates them.** They are listed so a later lane can decide.

- `vacask_compare`: four scalar-constant asserts in the first test, and two asserts on the const local `dv_vk`.
- `mos_psp103_50c_data`: the assert on `R2_COUNT_50`.
- `linear`: `assert_eq(798915 * 21, ONE - 1)` (all literals) and the assert on the const local `zz` in the RLC test.
- `sar_cdac`: `assert_eq(CODES, 1024)`, `assert_eq(LSB * CODES, VDD)`, `assert_eq(VCM * 2, VDD)`.
- `sar_comparator`: the asserts on the const locals `dtc`, `bn`, `bp`.
- `mos_psp103_data`: six `PIN_*_RUN != PIN_TT_RUN` asserts in the ROM-choice test and six `PIN_*_LG` / `PIN_*_REF`
  asserts in the log-domain test, all on scalar constants.
- `sar_comparator_psp`: the assert on the const local `dv` (lines 527-529 on master).
- `sar_switch`: `assert_eq(SW_CB, 6032)`.
- `mos_psp103`, `mos_psp103_gates`, `linear_zoh`, `mos_table`, `sar_kick_net`: none found in the candidate lists.

## What is not claimed

- The grep finds an upper-case name followed by `[` on the line of an `assert`. An assert split over several lines, or
  a table whose name has lower-case letters, is not in the candidate lists, so the audit is complete only for the form
  the grep matches.
- Only 13 of the 151 rewritten asserts (one per spec) were individually driven red by the expected-side control; the
  other 138 are shown live by the rise of the counted number, not by a failing run of their own.
- The audit did not run the OLD folded form against a wrong value, so it says nothing about whether the old form
  refused it at compile time or let it through.
- The seals of the 13 specs were re-saved afterwards (10:23 +07, with a `t27c` built from master; an earlier save with
  a `t27c` older than master's gen-zig and gen-rust commits moved 20 seals and was discarded), and strict coverage of
  `specs/analog/*.t27` then read `gaps 0`, `coverage 130/130`. The mutation census of the specs that have an accepted
  file was not re-run. The edits sit below every accepted line
  of `sar_cdac` (last accepted line 669, first edit 698) and of `sar_comparator_psp` (last accepted line 217, first
  edit 414), so those `path:line` keys still name the same code; the other 11 specs have no accepted file.
- Speed is not claimed: the windows above are wall-clock times of runs on a loaded machine.
