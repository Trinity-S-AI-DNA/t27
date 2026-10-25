# Mutation census of specs/analog/sar_logic.t27, 2026-10-25

Issues #12237 (batch 1) and #12240 (batch 2), part of the sar_logic mutation work #12163 and of epic #10486 (weak
spots of the analog v0.1 release). This lane wrote all 26 missing tests (P1 to P13 in batch 1, P14 to P26 in batch 2),
the accepted file and this note. Where each number comes from:

- **Before** (677 mutants, 526 killed, 108 survived, 31 hung, 12 unviable) is the analog-loop census of the unmodified
  spec, 07:59-08:24 +07, `tri mutate spec` with t27c 0.5.2, exit 0. The spec is byte-identical between the census base
  035df5ce60 and this lane's base 368ce03b6d (`git diff` of the file is empty). The census ran in a pre-audit tree; no
  mutant was re-run in this lane. The classes of the 108 survivors are from reading the spec (and the converter
  `spice_convert.t27` where a test depends on it); nothing was classified by a tool.
- **After** is NOT measured. This lane did not run the survivor gate (the brief forbids it) and did not re-save the
  seal. The forecast, to be checked by the gate run that follows, is 677 mutants, 606 killed, 28 survived (27
  equivalent, 1 dead), 31 hung, 12 unviable, and the gate passing on the accepted file below.
- **Tests**: `tri test specs/analog/sar_logic.t27` in a T27 OS pane, a master-built `tri` and t27c, 2026-10-25 13:08 +07
  (load 22.7), end state of both batches: 44 tests (18 before, 26 new), 44 pass, 0 fail, 0 vacuous passes of 44,
  exit 0. After batch 1 alone it was 31 tests, all passing, 0 vacuous (12:18 +07, load 14).

Kinds in the census (all 677): drop-guard 57, flip-cmp 198, swap-logic 57, drop-step 64, swap-arith 249,
ret-default 52. Among the 108 survivors: drop-guard 13, drop-step 4, flip-cmp 53, swap-arith 27, swap-logic 11.

## The 108 survivors, and what became of each

By reading: 27 equivalent, 80 test gap, 1 dead line, 0 undecided. Of the 80 test gaps, the 13 tests of batch 1 (P1 to
P13) kill 61 (a distinct count: survivor #8, the tgate pass row at line 359, is named by both P3 and P11) and the 13
tests of batch 2 (P14 to P26) kill the other 19. "#N" is the Nth survivor in census order. The mutant column follows
the census text.

| Lines | Mutant | Verdict | Outcome |
| --- | --- | --- | --- |
| 331, 335 | `p < 2` and `p < 3` to `<=` in sl_pin_load (#1, #2) | test gap | 2 killed by P6 |
| 340 | drop `if (p == 1) { return 250; }` (#3) | test gap | killed by P6 |
| 349 (2) | drop `if (p < 4) { return 250; }`, and `p < 4` to `p <= 4` (#4, #5) | test gap | 2 killed by P6 |
| 356 (2) | NAND row of sl_row: drop `if (kind == G_NAND) { return 1 + p; }`, and `1 + p` to `1 - p` (#6, #7) | test gap: no deck builds a NAND | 2 killed by P11 |
| 359 | drop `if (p == 2) { return 8; }` in sl_row, the tgate pass row (#8) | test gap | killed by P3 and P11 |
| 415 | drop `if (kind == G_INV) { return sl_not(v0); }` in sl_want (#9) | equivalent: falls through to the same `return sl_not(v0)` at line 422 | accepted |
| 417 | drop `if (kind == G_NAND)` in sl_want (#10) | test gap: no deck builds a NAND | killed by P11 |
| 448 | `j < sl_n_in(kind)` to `<=` in sl_trigger (#11) | test gap | killed by P20 |
| 449 (2) | `hit == NONE and` to `or`, and `ms[base + j]` to `ms[base - j]` in sl_trigger (#12, #13) | test gap | 2 killed by P3 |
| 453 | `hit == 1` to `hit != 1` in sl_trigger (#14) | test gap | killed by P3 |
| 459, 465 | `age <= ...` to `<` in sl_h_get (#15, #16) | equivalent: the tables are continuous at their points | accepted |
| 480 | `c > t` to `c >= t` in sl_age (#17) | test gap | killed by P21 |
| 488 | `j < sl_n_in(...)` to `<=` in sl_high_in (#18) | equivalent for the only caller (caveat below) | accepted |
| 516 | `ask == SL_S1` to `!=` in sl_delay (#19) | test gap | killed by P1 |
| 517 (8) | the tgate strength-only rule for a 0 in sl_delay: line dropped, four `==` to `!=`, three `and`/`or` swaps (#20-#27) | test gap | 8 killed by P1 |
| 539 | `load - 150` to `load + 150` in sl_delay (#28) | test gap | killed by P1 and P3 |
| 548 | `sl_age(ms, cell - 1, t)` to `sl_age(ms, cell, t)` (#29) | test gap | killed by P22 |
| 556 (3) | `fed - 1` to `fed`, `fed - 1` to `fed + 1`, and `- sl_arc(..)` to `+ sl_arc(..)` (#30-#32) | test gap | 3 killed by P7 |
| 561 | drop `if (d < 1) { return 1; }` (#33) | dead line | kept, accepted |
| 561 | `d < 1` to `d <= 1` (#34) | equivalent: both return 1 | accepted |
| 570 | `n < gt[GT_NNET]` to `<=` in sl_net (#35) | test gap | killed by P23 |
| 587, 592, 597, 609, 630 | `<=` in the loops of sl_init (587, 592, 597), sl_links (609) and sl_loads (630) (#36, #37, #40, #43, #48) | equivalent: spare or ghost slots nothing reads | accepted |
| 593, 594 | `ms[SL_DRV + g]` and `ms[SL_PT + g]` to `-` in sl_init (#38, #39) | test gap | 2 killed by P8 |
| 598 (2) | `g * 7 + j` to `g / 7 + j` and to `g * 7 - j` in sl_init (#41, #42) | test gap | 2 killed by P8 |
| 610, 611 | `ms[SL_CELL + g]` and `ms[SL_FED + g]` to `-` in sl_links (#44, #45) | test gap | 2 killed by P16 |
| 613 | `h < gt[GT_NGATE]` to `<=` in sl_links (#46) | test gap | killed by P24 |
| 616 | `ms[SL_FED + g] = h + 1` to `= h` in sl_links (#47) | test gap | killed by P7 |
| 631 (2), 632 (2) | the write to `SL_LDR`/`SL_LDF` with `-`, and the read from `SL_EXTL` with `-`, in sl_loads (#49-#52) | test gap | 4 killed by P5 |
| 636 | `g < gt[GT_NGATE]` to `<=` in sl_loads (#53) | test gap | killed by P14 |
| 639, 641 (2) | `p < 4` to `p <= 4`, `p < sl_n_in` to `<=`, and `kind == G_TGATE and p == 3` to `or` (#54-#56) | equivalent | accepted |
| 652 | `ms[SL_EXTL + net]` to `ms[SL_EXTL - net]` in sl_ext_load (#57) | test gap | killed by P5 |
| 666, 667 | `i < NWATCH` and `NLOG < SL_LOGMAX` to `<=` in sl_set (#58, #59) | test gap | 2 killed by P15 (and #58 also by P14 and P26) |
| 680, 687 | `ms[SL_EXT + n]` to `-` in sl_drive, `ms[b + SL_S0]` to `-` in sl_resolve (#60, #61) | equivalent (#60 under a contract, below; SL_S0 is 0) | accepted |
| 689 | `ms[b + SL_X]` to `ms[b - SL_X]` in sl_resolve (#62) | test gap | killed by P25 |
| 717, 718 | `ask == cur` to `!=`, and `ms[SL_PT + g]` to `-` in sl_sched (#63, #64) | test gap | 2 killed by P17 |
| 730, 739 | `<=` in the loops of sl_sched and sl_step (#65, #67) | equivalent | accepted |
| 731 | `g * 7 + j` to `g * 7 - j` in sl_sched (#66) | test gap | killed by P3 |
| 744 (2) | the maturity test of sl_step with its first, and then its second, `and` read as `or` (#68, #69) | test gap | 2 killed by P13 |
| 745 | `ms[SL_TCH + g]` to `ms[SL_TCH - g]` in sl_step (#70) | test gap | killed by P13 |
| 752, 754 | `n < NNET` and `c < 6` to `<=` in the counter clear of sl_step (#71, #72) | equivalent | accepted |
| 761 | `g < gt[GT_NGATE]` to `<=` in sl_step (#73) | test gap | killed by P14 |
| 763 (2) | `o * 6` to `o / 6` and `+ ms[SL_DRV + g]` to `-` on the read side of the counter (#74, #75) | equivalent | accepted |
| 767, 768, 774, 791 | `<=` and `ms[SL_EXT - n]` in sl_step, `ms[SL_PT + g] <= nxt` in sl_run (#76-#79) | equivalent (#77 under the contract) | accepted |
| 794 (2) | `nxt > t_end` to `>=`, and `NSTEP >= SL_MAXSTEP` to `>` in sl_run (#80, #81) | test gap | 2 killed by P9 |
| 798 | `si < nst` to `si <= nst` in sl_run (#82) | test gap | killed by P9 |
| 803 | the step counter `NSTEP + 1` dropped to `NSTEP` in sl_run (#83) | test gap | killed by P9 |
| 816, 829 | `i < NLOG` to `<=` in sl_edges and sl_edge (#84, #86) | test gap | 2 killed by P10 |
| 818, 831 | `ms[row] >= tmin` to `>` in sl_edges and sl_edge (#85, #87) | test gap | 2 killed by P10 |
| 842 | `a > b` to `a >= b` in sl_absdiff (#88) | equivalent: both give 0 at a == b | accepted |
| 848 (3), 853 (3) | `<=` to `<` on the permille limit, `and` to `or`, and `<=` to `<` on the ps limit, in sl_within and sl_within_fall (#89-#94) | test gap | 6 killed by P2 |
| 875 | `pos + i < dst.len` to `<=` in sl_cat (#95) | test gap | killed by P4 |
| 914 | `i < 11` to `<=` in sl_ids (#96) | equivalent: ids[20] is overwritten at line 925 | accepted |
| 932 (2), 1026 (2) | the refusal guard `ERR_LINE != 0 or n == 0` dropped, or `or` read as `and`, in sl_build and sl_chain_build (#97, #98, #103, #104) | test gap | 4 killed by P4 |
| 939 | `i < 12` to `<=` in sl_build (#99) | test gap | killed by P26 |
| 973, 974 | the two edge-count guards of sl_order_ok dropped (#100, #101) | test gap | 2 killed by P12 |
| 983 | the order guard `(ri < rj) != (...)` of sl_order_ok dropped (#102) | test gap | killed by P12 |
| 1047, 1052 | `SL_H_REST + SL_H_FALL0` to `-`, and `t + 654` to `t - 654` in sl_chain_stim (#105, #106) | test gap | 2 killed by P19 |
| 1085, 1093 | `e < tr` to `<=` in sl_chain_err_f (1085) and `e < tf` to `<=` in sl_chain_err_r (1093) (#107, #108) | test gap | 2 killed by P18 |

Totals: 27 equivalent (accepted) + 1 dead line (kept, accepted) + 61 test gaps killed by batch 1 + 19 test gaps
killed by batch 2 = 108. The census cannot say which assert kills which mutant, so the by-hand reading below stands
in; the gate run that follows is the check.

## The hung 31, the unviable 12, and the accepted file

`docs/analog/mutation-accepted-sar_logic.txt` has 58 entry lines: 26 equivalent, 1 dead line, 31 hung. The issue
counts mutants and not lines: 27 equivalent + 1 dead + 31 hung + 12 unviable = 71. The 71 mutants are not 71 lines for
two reasons.

- The 12 unviable mutants (all `ret-default`, a body emptied, which zig refuses with `not yet implemented`: lines 584,
  607, 627, 651, 656, 662, 679, 712, 737, 782, 903, 1009) are written as comment lines, not as entries.
  `tri mutate spec --accepted` (`specs/tri/mutate/survivors.t27`, `cli/tri/src/mutate.rs`) counts unviable mutants
  nowhere, and the accepted files of the other analog specs do not list them either.
- The gate matches an entry by path, line and kind and it names every mutant with that key, so the two equivalent
  mutants of line 763 (#74 `o / 6`, #75 `- DRV`, both `swap-arith`) are one entry.

The 31 hung mutants are all `x = x + 1` turned into `x = x` in a loop counter (lines 450, 472, 490, 575, 578, 589, 599,
601, 620, 622, 633, 645, 647, 674, 732, 749, 756, 758, 764, 771, 776, 792, 800, 819, 835, 877, 923, 941, 975, 984,
986); the loop never ends, so no test can go red and only the clock ends the mutant. They are accepted by
construction, as in the other analog notes. The counter and function of each is read from the spec, not run.

The 80 killed survivors (61 of batch 1, 19 of batch 2) are not in the file, because an accepted line whose mutants
are all killed makes the gate exit 2 as a stale line. In batch 1 the 19 survivors that batch 2 targets were listed as
pending, as they were still alive; batch 2 killed them and removed those lines, so the file now holds only what stays
alive on purpose. If the gate run finds one of the 19 still alive, its line goes back into the file as an equivalent
entry with the finding, as the brief requires. No (line, kind) key is shared between an accepted entry and a killed
survivor; the keys of the hung drop-step lines do not meet the four drop-step survivors (548, 556, 616, 803).

## The dead line (561)

`if (d < 1) { return 1; }` at the end of `sl_delay` can never run, by reading: `d` is a usize, the smallest unscaled arc
intercept is 17 ps (row 8, rising), the history scales (only on a nor3 that drives a cell) keep a ratio of at least
about 0.22 on loads of 33380 or more, and the inverter-after-cell terms add 345 or 31. The line stays in the spec:
deleting it would move every accepted line below it (and the seal moves anyway), and keep or delete is the owner's
call. Its sibling mutant `d <= 1` (#34) is equivalent for the same reason.

## The 13 tests of batch 1, P1 to P13 (appended after the last original test, so no accepted line number moved)

The diff is 559 added lines after original line 1686 and no removed line (one hunk). 187 runtime asserts in all. Every
expected value was hand-derived from the spec before it was run. One was wrong in the first run (P9, a node value after
the run) and was corrected from the bodies of `sl_run` and `sl_step`, not from the mutant. A trivially true assert in P7
was removed.

| Test | Asserts | Kills |
| --- | --- | --- |
| P1 `a_tgate_that_only_changes_strength_takes_1_ps_and_one_that_changes_value_takes_its_arc` | 17 | #19-#27 (lines 516, 517), and #28 |
| P2 `the_ck_bounds_hold_exactly_at_their_edges_and_need_the_permille_limit_and_the_ps_limit_together` | 14 | #89-#94 (848, 853) |
| P3 `the_tgate_pass_arc_and_the_enable_arc_time_the_two_edges_of_a_node_it_drives` | 8 | #8, #12, #13, #14, #28, #66 |
| P4 `a_work_buffer_too_small_for_the_netlist_is_refused_by_both_builders_and_sl_cat_stops_at_the_end_of_its_buffer` | 13 | #95, #97, #98, #103, #104 |
| P5 `the_capacitance_the_top_adds_to_a_port_reaches_both_load_tables_of_its_net` | 9 | #49-#52, #57 |
| P6 `each_pin_of_each_gate_kind_loads_its_net_by_the_measured_amount_and_an_output_pin_loads_nothing` | 23 | #1-#5 |
| P7 `the_inverter_after_the_delay_cell_takes_the_slow_edge_delay_of_the_cells_own_trim_code` | 13 | #30, #31, #32, #47 |
| P8 `sl_init_leaves_every_gate_slot_unknown_and_nothing_pending_for_the_last_gate_as_for_the_first` | 27 | #38, #39, #41, #42 |
| P9 `sl_run_applies_only_the_edges_it_is_given_includes_the_end_time_and_stops_at_the_step_cap` | 19 | #80-#83 |
| P10 `the_log_readers_find_nothing_in_an_empty_log_and_count_an_edge_at_exactly_the_time_asked_for` | 12 | #84-#87 |
| P11 `a_nand_gate_takes_arc_rows_1_and_2_and_drives_its_truth_table_and_a_tgate_pass_takes_row_8` | 14 | #6, #7, #8, #10 |
| P12 `sl_order_ok_refuses_a_ck_that_never_rises_one_that_falls_and_two_that_rise_in_the_wrong_order` | 6 | #100, #101, #102 |
| P13 `a_matured_change_stamps_its_gate_only_between_two_known_values_and_clears_the_stamp_out_of_unknown` | 12 | #68, #69, #70 |

How the groups reach their mutants, in one line each:

- **P1**: calls `sl_delay` directly on a tgate with hand-chosen drives. A tgate that changes only strength (weak 1 to
  strong 1, weak 0 to strong 0, and back) takes 1 ps; one that changes value, including a release to a 0 or a 1, takes
  its enable arc (78 ps falling, 137 ps rising on a load of 1000). Every comparison and `and`/`or` of lines 516 and 517
  changes one of these cases; the load of 1000, not 1300, also pins line 539.
- **P2**: `sl_within` and `sl_within_fall` at exactly the permille limit and exactly the ps limit, with the other
  limit met and not met. The two bounds are an `and`: `or` accepts a value that fails one of them, `<` refuses the
  exact edge.
- **P3**: a tgate pass arc and its enable arc on a node it drives, timing the two edges: the q rise lands at 137 ps
  (enable arc, rising) and the fall at 10064 (pass arc, row 8, falling, 64 ps after d falls at 10000). The survivors
  named give 78 ps (10078) instead, or 153 ps for the load. It fixes row 8 (line 359), the `hit` scan of
  `sl_trigger` (449, 453), the tgate's own load of 150 (539) and the remembered inputs of `sl_sched` (731).
- **P4**: a work buffer too small for the netlist is refused by both builders (`ERR_LINE` is 2), and `sl_cat` stops at
  the end of its buffer. With the refusal taken, `n > 0`, so the `or n == 0` half of the guard cannot be told apart;
  `and` in its place does not refuse, which is what #98 and #104 show.
- **P5, P6**: direct state checks of the two load tables after a top-level capacitance on a port (600 on `vout`, 1300 on
  `ck1`), and the measured load of every pin of every gate kind (an output pin loads 0).
- **P7**: the inverter after the delay cell takes the slow-edge delay of the cell's own trim code; the `fed - 1` and
  the arc term of line 556, and the `h + 1` of `sl_links` (616), change it.
- **P8**: `sl_init` leaves every gate slot unknown and nothing pending for the last gate as for the first.
- **P9**: `sl_run` applies only the edges it is given, includes the end time, and stops at the step cap.
- **P10**: the log readers on an empty log and on an edge at exactly the time asked for.
- **P11**: a NAND gate takes arc rows 1 and 2 and its truth table (no deck builds a NAND, so these lines were reached
  by nothing), and a tgate pass takes row 8.
- **P12**: `sl_order_ok` on a hand-made log: a CK that never rises, one that falls, two that rise in the wrong order.
- **P13**: `sl_step` bookkeeping on a hand-made state: a matured change stamps its gate only between two known values
  and clears the stamp out of unknown.

After batch 1 alone, `tri test specs/analog/sar_logic.t27` gave 31 tests pass (18 before), 0 fail, 0 vacuous of 31,
exit 0.

### Hand-mutant spot checks of batch 1 (extra evidence, not the gate)

Eighteen hand-made mutants (M1 to M18) were applied one at a time to the lines of P3, P4, P7, P8, P9, P11 (line 359),
P12 (974, 983) and P13 (745), each reverted after its run, and each was killed by the test that was meant to kill it.
The individual results are not kept in a file. These were NOT spot-checked: P1 (516, 517, 539), P2 (848, 853), P5, P6,
P10, P11 (356, 417), P12 line 973 and P13 line 744. For those the kill rests on the by-hand reading and on the
census's own mutants being the same edits.

## The 13 tests of batch 2, P14 to P26 (appended after P13, so no line moved)

The diff against batch 1 is 426 added lines after line 2245 (the end of P13) and no removed line (one hunk); against
the unmodified spec it is 985 added lines. 140 runtime asserts in all. Each assert that kills a survivor has a comment
naming the survivor, its line and what the mutant does. Every expected value was hand-derived from the function bodies
before the first run.

| Test | Asserts | Kills |
| --- | --- | --- |
| P14 `an_empty_row_after_the_last_gate_is_not_counted_as_a_load_nor_as_a_driver_of_net_0` | 16 | #53 (636), #73 (761) |
| P15 `the_log_keeps_to_the_nets_that_were_watched_and_stops_at_its_capacity` | 15 | #58 (666), #59 (667) |
| P16 `sl_links_wipes_both_link_slots_of_every_gate_before_it_looks_for_a_link` | 7 | #44 (610), #45 (611) |
| P17 `a_change_that_goes_back_to_what_the_gate_drives_withdraws_the_pending_edge_of_that_gate_only` | 10 | #63 (717), #64 (718) |
| P18 `a_chain_edge_exactly_on_the_valid_edge_that_caused_it_is_a_real_edge_with_a_delay_of_zero_and_not_a_missing_one` | 4 | #107 (1085), #108 (1093) |
| P19 `the_pulse_train_stimulus_puts_valids_first_fall_after_the_rest_and_ck1_654_ps_after_the_last_rise` | 20 | #105 (1047), #106 (1052) |
| P20 `the_trigger_of_a_gate_reads_its_input_pins_and_never_its_output_pin` | 9 | #11 (448) |
| P21 `the_age_of_an_output_that_changed_at_this_very_ps_is_zero_and_not_the_long_rest` | 5 | #17 (480) |
| P22 `the_nor3_that_drives_the_delay_cell_reads_the_age_of_the_delay_cells_output_and_not_of_the_gate_after_it` | 17 | #29 (548) |
| P23 `sl_net_finds_a_net_by_its_name_and_answers_none_for_a_name_no_net_has` | 8 | #35 (570) |
| P24 `the_inverter_after_a_nor3_that_has_two_inputs_high_scales_the_delay_cell_by_the_nor3_and_not_by_an_empty_row` | 14 | #46 (613) |
| P25 `a_net_with_an_unknown_driver_resolves_to_unknown_whatever_it_held` | 7 | #62 (689) |
| P26 `the_sequencer_builder_watches_the_eleven_ck_nets_and_clkc_and_no_other_net` | 8 | #99 (939) |

How the groups reach their mutants, in one line each:

- **P14**: a one-inverter deck whose output `y` is the first port, so `y` is net 0, the net the empty row after the last
  gate points at. `y` loads nothing (LDR and LDF of `y` are 0, `a` loads 1000), rises 48 ps after `a` falls, falls at
  2064, and is a clean 1 (the `<=` of `sl_step` would meet the empty row's strong 0 with the inverter's strong 1 and
  make X). The premises (`sl_kind` of row 1 is an inverter, its pins are net 0) are asserted too.
- **P15**: `sl_set` on a hand-made state logs only a watched net (net 3, not net 0), logs time, net and value, ignores a
  repeat, writes the last row when the count is 639, and writes nothing and counts nothing at the capacity of 640.
- **P16**: `sl_links` on a deck with no delay cell, after numbers were left in both link tables of both gates: all four
  slots read 0.
- **P17**: an inverter that is gate 1 (a dummy gate 0 comes first, so a slot before gate 1's is told apart): `a` rises at
  1000, goes back at 1030, and the pending fall of 1064 is withdrawn on gate 1's slot. One fall at 2064 is logged.
- **P18**: a hand-made log of CLKC; `sl_chain_err_f` and `sl_chain_err_r` on an edge exactly on its cause give 1000 permille
  (0 ps against 100 ps), not the 9999 of a missing edge; two controls with an edge after its cause give 500 and 750.
- **P19**: `sl_chain_stim` with 11 pulses: 32 edges with CK1, 31 without; Valid's first fall at 51100, the last rise at
  499700, CK1 at 500354 (654 ps after), the last fall at 519900.
- **P20**: `sl_trigger` on the hold deck: the output pin (`y` of the inverter, `q` of the tgate) differs from its remembered
  value and is not a trigger; a changed input (`d`) is found, as pin 2.
- **P21**: `sl_age` at the very ps of the stamp is 0, one ps later 1, a future stamp and no stamp are the long rest.
- **P22**: the clock chain of `sl_chain_build`: the nor3 reads the age of the delay cell (gate 1), 5000 ps, so its rise is
  the history-scaled 6008 ps and not the settled 7390 ps that the age of the inverter after it (never changed) would give.
- **P23**: `sl_net` finds a net by name, answers NONE for a missing name, and answers NONE for the empty name (the row after
  the last net has an empty name, which is as long as the empty name).
- **P24**: the clock chain with the nor3 output as the first port (net 0): the delay cell's feeder is the nor3, and with two
  nor3 inputs high the cell rises in 8504 ps (806 / 1000 of 10552); with one high it rises in 10552 ps.
- **P25**: `sl_resolve` of a net with only an unknown driver gives X whatever it held, with four controls.
- **P26**: `sl_build` watches 12 nets (CK11 to CK1 and clkc) and no thirteenth; net 0 (VDD) is driven high at 0 and has no
  logged edge, and the log holds exactly the edges of the 12.

`tri test specs/analog/sar_logic.t27`, end state (batches 1 and 2): see the Tests line at the top.

### Hand-mutant spot checks of batch 2 (extra evidence, not the gate)

All 19 mutants of batch 2 were applied one at a time to the spec (each by a single-line edit, taken from the census
text, copied from the file kept before the run), `tri test` was run on each, and the file was restored from the kept copy
and compared byte by byte. Each of the 19 gave exit 1 with the intended test as the only FAIL, with one exception that
is itself evidence: the mutant at line 666 (#58) turns P14 and P26 red as well as P15. The tool names only the failed
test, not the assert, so the claim is "killed by that test", and the assert comment names which assert is meant to do
it. This is not the survivor gate: the gate runs all 677 mutants with its own timeouts.

## What the survivors say about the spec

Found by reading; none of it is changed by this lane.

- **Line 561** is dead (above). **Line 415** duplicates the fall-through at line 422.
- **Lines 356 and 417**, the NAND row and arm of `sl_want`: no deck contains a NAND (line 16 says neither block uses it,
  and line 1168 asserts that the NAND count of one deck is 0). P11 tests them directly; the alternative is to drop the
  NAND kind from this model.
- **Line 744**, `was != now`: redundant by reading. Whenever PT[g] is set (`sl_sched` sets it only when the wanted value
  differs from DRV[g]), PV[g] differs from DRV[g], and DRV[g] changes only at line 746, which clears PT[g]; so at
  maturity the two always differ. P13 tests the bookkeeping, not that claim.
- **Lines 273 and 274**, `SL_NETS` = 128 and `SL_GATES` = 160, are defined and used nowhere, and the converter's
  tables hold 256 nets and 256 gates. `sar_logic.t27` has no capacity check: a deck beyond 128 nets or 160 gates writes
  into the next table of `ms` without a message.
- **Ghost row**: the loops that stop at `g < NGATE` or `n < NNET` rely on the row after the last gate of a zeroed `gt`
  being an inverter on net 0. That is harmless while net 0 is VDD or a stimulus input (every deck in the file) and wrong
  when net 0 is a gate's output (#53 and #73 show it, and P14 and P24 build exactly that deck; the loop bounds at 636,
  761 and 613 are the three places a `<=` reaches the row).
- **Lines 932 and 1026**: the `or n == 0` half of the refusal guard is redundant on a refusal, since the refusal text is
  written; whether an accepted deck could give `n == 0` was not checked.
- **Line 1052**: the doc comment says CK1 rises 654 ps after the last Valid rise. With `t - 654` the stimulus array is no
  longer sorted by time, which breaks the contract of `sl_run` (the doc comment at lines 780-781). No test pins the
  array before P19 (batch 2), which now asserts the time of every edge it names.
- **Lines 516 and 517**, the tgate strength-only rule, and **line 359**, the pass arc, were reached by no asserted delay
  before P1 and P3 (inferred from the survivors, not traced).

## Not claimed

- The survivor gate was not run in this lane, so the forecast above (606 killed, 28 survived, 31 hung, 12 unviable)
  is not a measurement: FR-001 (a census re-run on the unmodified tree) and SC-001 (the gate on the changed tree) are
  open. The 30 s mutant timeout was not re-measured with the 26 added tests, which lengthen the suite; a slower suite
  can turn a killed mutant into a hung one, which the gate would show.
- The seal of the spec was not re-saved (FR-003): the spec text changed, so its seal in `.trinity/seals/` is stale until
  a master-built t27c saves it. `tri coverage --strict` and the CI checks were not run locally.
- Both batches are written (#12237 and #12240), but the 19 survivors of batch 2 are removed from the accepted file on
  the strength of the by-hand reading and the 19 single-mutant runs above, not of a gate run. If the gate finds one
  still alive, its line is added back as an equivalent entry with the finding.
- The 27 equivalent survivors are classified by reading the arithmetic, not proved equivalent by a tool. Three carry a
  condition: #18 (line 488) holds only for the call `sl_delay` makes on a converter deck, and #60 and #77 (lines 680,
  768) hold while an input has no driver in the circuit (the contract at line 678). The loop-bound `<=` equivalents
  (lines 587, 592, 597, 609, 630, 639, 641, 730, 739, 752, 754, 767, 774) assume a deck below 128 nets and 160 gates
  and a zeroed `gt`; a deck at capacity was not traced. If one of them is not equivalent after all, a later test kills
  it and the gate reports that accepted line as stale.
- The 31 hung mutants are classified by construction: the tool reports each as outliving the timeout, which is
  consistent with an endless loop but was not shown by a run that was let to finish. The 12 unviable mutants were never
  checked by any test; they share one emitter-side message (`not yet implemented`) that says nothing about the spec.
- The expected values in the new tests were hand-derived from the spec's arc table and loads; all of them now pass,
  which confirms the derivation for those asserts and says nothing about any other value. Two derivations slipped
  before they ran: P9 (a node value after the run, found on the first run) and P18 (the design had 0 permille for an
  edge on its cause; `sl_err_pm` divides a 0 ps delay against the nominal and gives 1000, fixed from the function body
  before the first run). Every other value of batch 2, including 6008, 7390, 8504 and 10552 of P22 and P24, matched.
- The tool says which test failed, not which assert. A mutant that fails the intended test is shown killed by that test,
  and the claim that it is the assert whose comment names it is by reading.
- The run-level effect of #106 (`t - 654`) was not re-derived, and #29 (P22) is claimed only as far as that test's own
  assert.
- Only `specs/analog/sar_logic.t27` was touched among the specs. No PR was opened; this issue is a part of #12163 and
  does not close it.
