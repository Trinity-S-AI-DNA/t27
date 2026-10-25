# What `t27c seal` refuses on the 16 analog milestone specs, 2026-10-25

Issue #12065, epic #10486, night loop tick 6. It fills the gap that the SC-002 row of `docs/analog/v0.1-release-notes.md` names: "A per-milestone `t27c seal` exit code was not collected for this release".

What this document does not say: nothing about speed (no time was recorded); nothing about whether a seal that passes is right (a seal records that the tests pass and the hashes were taken, not that the spec matches the circuit); nothing about the other 1,000-odd specs of the repository; nothing about the in-spec negative controls, which are a different object (section 1).

## 1. The question, and the reading taken

SC-002 of the epic reads: "a deck in the supported subset gives a spec whose tests pass with 0 vacuous, and `t27c seal` refuses each milestone's negative control".

The release notes use "negative control" for tests inside a spec that pass when a wrong variant fails (for example the ss tables on the tt top, the comparator without charge). Those controls are green inside a clean spec; `seal` is never asked to refuse them. What can be measured is the other reading, and this note takes it: when a milestone spec is wrong, does `t27c seal` refuse it? A wrong spec here is a spec with one planted fault, a one-line edit of a constant by one unit (one boolean operator for `sar_logic`).

Three forms of the command exist (`t27c seal --help`) and they behave differently:

- `t27c seal <spec>` (bare): prints the spec hash and four generator hashes.
- `t27c seal --save <spec>`: runs the spec's tests and the backends, then writes the seal; refuses when a test fails or a backend rejects the spec.
- `t27c seal --verify <spec>`: compares the current hashes with the saved seal.

## 2. Method

- A scratch checkout of master at `69eef64da1`, detached, never committed. A master-built `t27c` 0.5.2 (same commit).
- 16 patches of one line each. The lines were first chosen with `tri mutate plant` (one mutant on a copy, `t27c test-report` on baseline and mutant, the original hashed before and after).
- Each patch was applied alone (`git apply --unidiff-zero`), the three forms were run on that spec, and the tree was restored with `git checkout -- specs .trinity`. `git status --short` after each apply showed exactly one modified spec.
- The md5 of the 16 spec files before the first plant and after each of five batches is equal (`cmp` exit 0 five times).
- The runs were made in a T27 OS shell cell, not on the Mac shell, after the load (23) and the free disk (13 GB) were read.
- The clean column is the same three forms on the unmodified specs.

## 3. Results

Every row: the spec, the planted line (file under `specs/analog/` unless it says `fpga/`), and the exit code and line of each form on the faulty spec. "FAIL n of m" is `--save`'s own refusal line. Clean runs: `--save` exit 0 with "tests m/m pass", `--verify` exit 0 with "all hashes MATCH", bare exit 0, for all 16.

| spec (milestone) | planted fault | clean `--save` tests | `--save` | `--verify` | bare |
|---|---|---|---|---|---|
| linear (M0, M1) | linear:85 `ONE` 16777216 -> 16777217 | 14/14 | exit 1, FAIL 5 of 14 | exit 1 | exit 0 |
| linear_zoh (M1c) | linear_zoh:110 `PERIODS` 100 -> 101 | 15/15 | exit 1, FAIL 1 of 15 | exit 1 | exit 0 |
| fpga/analog_cell_ax7203 (M1b) | fpga/analog_cell_ax7203:77 `RLC_STATES` 3 -> 4 | 11/11 | exit 1, FAIL 4 of 11 | exit 1 | exit 0 |
| mos_table (M3b) | mos_table:155 `NCELL` 24 -> 25 | 21/21 | exit 1, FAIL 14 of 21 | exit 1 | exit 0 |
| mos_psp103 (M3c) | mos_psp103:204 `DC_POINTS` 49 -> 50 | 40/40 | exit 1, FAIL 6 of 40 | exit 1 | exit 0 |
| fpga/psp103_inv_ax7203 (M3d) | fpga/psp103_inv_ax7203:84 `STEPS_N` 400 -> 401 | 15/15 | exit 1, FAIL 1 of 15 | exit 1 | exit 0 |
| sar_cdac (M4a) | sar_cdac:119 `TOTAL_UNITS` 63 -> 64 | 13/13 | exit 1, FAIL 1 of 13 | exit 1 | exit 0 |
| sar_comparator (M4b) | sar_comparator:202 `SC_VTH_U` 5872026 -> 5872027 | 20/20 | exit 1, FAIL 11 of 20 | exit 1 | exit 0 |
| sar_comparator_q (M4b-q) | sar_comparator_q:122 `SQ_VDD` 33554432 -> 33554433 | 8/8 | exit 1, FAIL 4 of 8 | exit 1 | exit 0 |
| sar_switch (M4c) | sar_switch:175 `SW_VDDW` 33554432 -> 33554433 | 14/14 | exit 1, FAIL 6 of 14 | exit 1 | exit 0 |
| sar_logic (M4d) | sar_logic:386 `a == 0 or b == 0` -> `a == 0 and b == 0` | 17/17 | exit 1, FAIL 1 of 17 | exit 1 | exit 0 |
| sar_adc (M4e) | sar_adc:133 `CODES11` 2048 -> 2049 | 15/15 | exit 1, FAIL 1 of 15 | exit 1 | exit 0 |
| sar_kick_net (M4e) | sar_kick_net:159 `NS` 67 -> 66 | 23/23 | exit 1, FAIL 8 of 23 | exit 1 | exit 0 |
| fpga/sar_adc_ax7203 (SAR board top) | fpga/sar_adc_ax7203:87 `CONVS` 31 -> 32 | 13/13 | exit 1, FAIL 2 of 13 | exit 1 | exit 0 |
| accept_sar (acceptance) | accept_sar:138 `AS_N_BLOCKS` 13 -> 14 | 17/17 | exit 1, FAIL 1 of 17 | exit 1 | exit 0 |
| raw_writer (M5) | raw_writer:67 `RAW_BASE` 1000000000 -> 1000000001 | 18/18 | exit 1, FAIL 3 of 18 | exit 1 | exit 0 |

Totals: bare `seal` refuses 0 of 16; `seal --save` refuses 16 of 16; `seal --verify` refuses 16 of 16.

The failing tests, where there are few: `linear_zoh` fails `the_lc_tank_keeps_its_amplitude_after_100_periods_and_backward_euler_does_not`; `sar_cdac` fails `the_inventory_is_the_decks_63_units_in_two_arrays_of_31_and_one_bridge_unit`; `sar_adc` fails `the_ideal_sar_reads_every_bin_centre`; `sar_logic` fails `the_gate_logic_is_the_truth_table_of_each_kind`; `psp103_inv` fails `the_constants_are_the_specs`; `accept_sar` fails `the_thirteen_blocks_count_9_pass_0_fail_and_4_not_modelled_and_their_30_rows_26_pass_0_fail_and_4_not_modelled`; `sar_adc_ax7203` fails `the_roms_are_the_specs_tables_word_for_word` and `the_board_prints_31_passing_lines_then_goes_round_again`; `raw_writer` fails `the_expansion_of_powers_is_exact`, `f64_prints_like_c_printf_15e` and `the_ascii_file_is_ngspices_byte_for_byte`; `analog_cell` fails `the_circuit_facts_agree_with_the_cell`, `the_board_machine_runs_the_rlc_like_the_cell`, `the_run_starts_after_lock_and_button_and_restarts_on_the_button` and `the_board_prints_passing_receipts_for_both_circuits_in_turn`.

For all 16 faults the FAIL count of `--save` equals the count `tri mutate plant` reports from `t27c test-report` on a copy of the same one-line mutant. `--save` runs the same report (`bootstrap/src/main.rs`, `test_report::run` then `seal_verdict`), so the two cannot differ, and they do not. The plant run for `sar_adc_ax7203` used 31 -> 30 where the table uses 31 -> 32; both fail the same two tests.

## 4. Findings

1. The bare form never refuses. It exits 0 on all 16 faulty specs and prints five lines of hashes. A script or a person that runs `t27c seal <spec>` and reads exit 0 as "this spec is in order" is reading nothing: the tests are run only in the `--save` branch.
2. `--save` is the form that judges. 15 of the 16 faults are caught by one or more failing tests of the spec itself; the 16th (`analog_cell`) is caught by 4 tests. Seven of the 16 faults are caught by only one or two tests (`linear_zoh`, `sar_cdac`, `sar_logic`, `sar_adc`, `psp103_inv`, `accept_sar` by one; `sar_adc_ax7203` by two), so a second slip in the same area could remove the last guard.
3. `--verify` refuses all 16 by hash: the spec hash and all four generator hashes differ for every fault. That is a statement about the file changing, not about a test failing; a harmless edit (a comment) would refuse the same way.
4. Four more single-line faults survived `tri mutate plant`, which means no test pins that line (the baseline and the mutant both report all tests passing): `sar_kick_net.t27:153` `DR_UP : u8 = 1` -> 7; `sar_logic.t27:1093` `return 9999` -> `return 9998`; `sar_logic.t27:177` `SL_DC_LOAD_R : usize = 33380` -> 33381; `sar_logic.t27:183` `SL_TG_IN_LOAD : usize = 430` -> 431. No spec was edited here; they are a follow-up issue.
5. An earlier run planted all 16 faults at the same time and gave larger FAIL counts in 9 of the 16 rows (for example `accept_sar` 9 of 17, `mos_psp103` 31 of 40) and one compile refusal for `analog_cell` ("use of undeclared identifier 'ONE'" in the generated Zig) where one fault at a time gives 4 of 11 tests failing. 14 of the other 15 specs import `analog::linear` directly (`sar_logic` does not), so a fault in a dependency fails a spec whose own line is clean; the cause of the compile refusal was not investigated. That run is not used in the table. It would have overstated how many tests guard each line.

## 5. What this does not say

- The 16 faults are one chosen line per spec. This is not a mutation score; the four survivors in finding 4 show that other lines are not guarded.
- A refusal by `--verify` is a hash comparison, not a test verdict (finding 3).
- The clean `--verify` and `--save` runs were made on master `69eef64da1`. Master has since moved (#12062 reseals 247 seals); this note does not check whether the 16 seals changed there.
- A clean `--save` rewrites tracked seal JSON (metadata such as the sealing time; 43 changed lines over 16 files in the earlier run). It was restored with `git checkout`, not examined and not committed. A reseal of these 16 belongs to its own change.
- No time was recorded for any run and none is claimed.
- One host, one scratch checkout, one day.

## 6. The SC-002 row

Proposed text for the row of `docs/analog/v0.1-release-notes.md` (the release notes are edited after #12063 lands, because the lines next to the row differ on that branch):

> SC-002 met for `t27c seal --save` and `--verify`, not met for the bare `t27c seal <spec>`: with one planted single-line fault in each of the 16 milestone specs, one fault at a time, `--save` refuses 16 of 16 on failing tests of the spec and `--verify` refuses 16 of 16 on a hash mismatch; the bare form runs no tests and exits 0 on all 16 (`docs/analog/seal-refusal-2026-10-25.md`, #12065). Of four further single-line faults that no test caught, two are now caught (#12067) and two (a step of 1 in `SL_DC_LOAD_R` and in `SL_TG_IN_LOAD`) change no output of the integer-ps model (the note, section 7).

## 7. Finding 4 after #12067 (2026-10-25)

Seven plants, one at a time, with the master-built compiler (`tri mutate plant --t27c <master-built t27c>`; the tool hashes the original before and after, and it was unchanged each time). Tests of the changed specs: `sar_kick_net` 23 -> 24, `sar_logic` 17 -> 18, all pass; the coverage row of both specs reads gaps 0, seal MATCH.

| Line | Plant | Result | Killed by |
|---|---|---|---|
| `sar_kick_net.t27:153` | `DR_UP : u8 = 1` -> 7 | KILLED | `the_five_drive_mode_codes_are_the_dense_table_zero_to_four_and_up_is_the_pmos_driver_by_its_number` |
| `sar_logic.t27:1093` | `return 9999` -> `9998` (rise) | KILLED | `a_chain_error_asked_for_an_edge_the_run_never_made_is_exactly_9999_for_the_fall_and_for_the_rise` |
| `sar_logic.t27:1085` | `return 9999` -> `9998` (fall; not in the first list) | KILLED | the same test |
| `sar_logic.t27:177` | `SL_DC_LOAD_R` 33380 -> 33381 and -> 33379 | SURVIVED both | none |
| `sar_logic.t27:183` | `SL_TG_IN_LOAD` 430 -> 431 and -> 429 | SURVIVED both | none |

The second sources of the three new pins: the arrays of `sar_kick_net` start as `[0] ** 10`, so idle must be code 0, and the five codes are a strictly ascending chain below 5; `p_drive`'s doc comment gives UP through the pmos and DOWN through the nmos; for the 9999 marker, the doc comments of `sl_chain_err_f` and `sl_chain_err_r` ("9999 when there is no such edge") and the run's own edge counts (4 falls and 5 rises, so p = 4 names edges that are not there).

Why the two load constants are not pinned. `sl_arc` is `d0 + k * load / 10000` in whole picoseconds, with k from 246 to 2159, so a step of 1 in the load moves an arc by k / 10000 = 0.02 to 0.22 ps and reaches an output only when it crosses a whole picosecond. Measured on a scratch copy with `t27b test` (two probe tests that sum raw edge times in ps, not committed): the first sums the eleven CK11..CK1 rise times with weights 1 to 11 and reads 22924022; the second sums, over the 21 calibration chains with weights 1 to 21, the fourth clkc fall plus three times the fifth clkc rise and reads 352034858. With `SL_TG_IN_LOAD` 431, `SL_DC_LOAD_R` 33381 and 33379 both sums are identical to the digit, so no output these probes read can tell them apart; with `SL_TG_IN_LOAD` 429 the first sum is 22923967 (55 lower) and the second is unchanged, but no test has a bound that tight (all 18 tests still pass, row above). The probes read the CK edges and the calibration chains; the other edges the 18 tests read were not summed, and the plant tool's SURVIVED on all four plants is the check for those. These are equivalent mutants of the model as it is built, not missing tests. A test that wrote 33380 or 430 as its own expected value would only copy the line. To pin them for real the load would have to become a parameter of the model (then a fit property against ngspice's CK edges could be tested), which changes a constant-bearing function and is outside #12067's boundary.
