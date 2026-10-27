# Mutation census of specs/analog/sar_comparator_qi.t27, 2026-10-25

Issue #12186, part of epic #10486 (weak spots of the analog v0.1 release). This lane wrote the missing asserts and the
accepted file; the survivor gate was then run on this tree and the seal re-saved. Where each number comes from:

- **Before** (308 mutants, 256 killed, 20 survived, 18 hung, 14 unviable) is the analog-loop census of the unmodified
  spec, 09:48-10:00 +07, exit 0, 12 min 13 s, `tri mutate spec` 3 mutants at a time with zig `-j2` each. The census
  text does not record the t27c version. The classes of the 20 survivors are from reading the spec, `sq_jq` in
  `sar_comparator_q.t27`, `sc_add_j` and the sign-magnitude helpers in `sar_comparator.t27`, and the one other caller
  of `qi_scatter` (`board/sar_cmp_qi_unit.t27`); no mutant was re-run in this lane.
- **After** (308 mutants, 262 killed, 14 survived, 18 hung, 14 unviable) is measured: the survivor gate run on the tree
  of commit 04461096ed (this lane's tests, note and accepted file), in a T27 OS pane, 3 mutants at a time, timeout 30,
  a master-built t27c, master base 1c783b0b31, 2026-10-25 10:29:42 to 10:43:12 +07 (13 min 30 s). All 262 kills are by a
  failing test and none by an invariant at compile time. The gate line reads: 32 not killed (14 survived, 18 hung), 32
  accepted by `docs/analog/mutation-accepted-sar_comparator_qi.txt`, 0 accepted lines now killed; the gate passed.
  The measured numbers equal the forecast made before the run (308, 262, 14, 18, 14; 32 accepted lines alive, 0 now
  killed), so nothing in this note needed correcting beyond marking them measured.
- **Seal**: `.trinity/seals/analog_AnalogSarComparatorQi.json` was re-saved by `tri seals changed` with a master-built
  t27c; the save ran at 11:13-11:14 +07 on 2026-10-25, after the gate run (built_by `t27c-bootstrap@0.5.2+1941409a6`, tests 11 of 11);
  no importer seal moved. The 11 tests were also run once on their own with `tri test` (below).

Kinds in the census (before): drop-guard 5, flip-cmp 46, drop-step 43, swap-arith 193, ret-default 21.

## The 20 survivors, and what became of each

| Lines | Mutant | Verdict | Outcome |
| --- | --- | --- | --- |
| 142, 147 | `cn[e * QI_NC + QI_C_CUR]` to `- QI_C_CUR` (qi_reads_of, qi_mults_of) | equivalent: QI_C_CUR is 0 | accepted |
| 307, 362, 374 | `rec[o + QI_R_HI]` to `- QI_R_HI` (307, 374), `cn[oc + QI_C_CUR]` to `- QI_C_CUR` (362) | equivalent: QI_R_HI and QI_C_CUR are 0 | accepted |
| 176, 177, 179, 180, 185, 186 | `<` to `<=` and `>` to `>=` in qi_stat (185 is the `<=` one) | equivalent: at equality the assignment writes the value the word already holds | accepted |
| 293, 296 | `vg > vlo` and `vhi > vg` to `>=` in qi_eval | equivalent: at a tie both branches end at (x, xn) = (0, 0) | accepted |
| 185 | `st[14 + 2 * c]` to `st[14 - 2 * c]` in the condition of the least-word update | test gap, group A | 1 killed by the new test |
| 234, 235 | drop and flip of `if (k == 0)` and `if (k == 1)` in qi_ord | test gap, group B | 4 killed by the new test |
| 250 | `<=` to `<` in qi_accept | test gap, group C | 1 killed by the new test |
| 386 | drop `if (!jac) { return; }` in qi_scatter | dead line | kept, accepted |

Totals: 13 equivalent (accepted) + 6 test gap (killed) + 1 dead line (kept, accepted) = 20. The gate run agrees with
these counts: killed went from 256 to 262 and survivors from 20 to 14, and every one of the 32 mutants not killed (14
survived, 18 hung) is on an accepted line. The tool names no killing test, so which new assert kills which mutant stays
the by-hand reading below. The 18 hung mutants are all
`x = x + 1` turned into `x = x` in a loop counter (`qi_st_init` 155, `qi_stat` 187 and 190, `qi_state` 244, `qi_same`
258, `qi_eval` 305, `qi_charges` 402, 407, 413, `qi_assemble` 425, 431, 437, `qi_stamp` 452, `qi_step` 482 and 490,
`qi_run` 513, 526, 528); the loop never ends, so no test can go red and only the clock ends the mutant. They are
accepted by construction.

## The unviable 14, and the accepted file

`docs/analog/mutation-accepted-sar_comparator_qi.txt` has 32 entries: 13 equivalent, 1 dead line, 18 hung. The 14
unviable mutants are written there as comment lines, not as entries, which differs from the count in the issue
(46 entries, 14 of them unviable). The reason is how `tri mutate spec --accepted` reads the file
(`specs/tri/mutate/survivors.t27`, `cli/tri/src/mutate.rs`): unviable mutants (zig refuses them, 10 `ret-default` with
the body emptied and the 4 `swap-arith` of lines 501, 502, 504, 505 whose constant `SQ_OFF - SC_VCM` is negative) are
counted nowhere, and the existing accepted files (`sar_comparator_psp`, `sar_cdac`, `sar_kick`, `sar_switch_q`) do not
list them. An entry is checked per line and kind, and the gate exits 2 on an accepted line whose mutants are all killed.
Lines 501, 502, 504 and 505 each also carry a viable swap-arith mutant (`+ h` to `- h` and back) that is killed, as it
is in neither the survivors nor the hung list, so an entry for them would read as a stale line. The gate run reproduced
the same 14 unviable mutants and passed with the file as written (32 entries against 32 mutants not killed). That an
entry for lines 501, 502, 504 and 505 would make the gate exit 2 is still a reading of the source: that case was not run.

## The dead line (386)

`if (!jac) { return; }` in `qi_scatter` stays in the spec. No result depends on it: without it a jac-false call goes on
to the three `sq_jq` calls with slope words that are all 0 (`qi_eval` writes the slopes only with jac), which add
magnitude 0 to a register. It is listed in the accepted file as "dead line, kept: states that charge-only evaluations
skip the Jacobian". Keep or delete stays the owner's call; a board cost model that counted the 9 zero adds per device
would need the guard's absence stated.

## The 3 tests added (appended after the last original test, so no accepted line number moved)

Each assert carries a one-line comment naming the mutant it kills.

- **Group A**, `the_least_and_the_most_words_of_the_statistics_are_those_of_the_fifteen_devices_whichever_device_holds_the_least`
  (17 runtime asserts). It builds a 60-word counter array by hand: device 0 holds the least (current 2, charge 6,
  slope 8, other multiplications 12), devices 1 to 14 hold 4, 7, 16, 20. After `qi_st_init` and `qi_stat(&st, 0, &cn)`
  it asserts words 0 to 6 of block 0 (count 15, reads 510, least 20, most 35, multiplications 3062, least 122, most 210)
  and words 14 to 21 equal 2, 4, 6, 7, 8, 16, 12, 20, and that block 1 stays 0. Why the mutant `14 - 2 * c` fails: the
  condition then reads word 14, 12, 10, 8 for c = 0..3 while the write goes to 14, 16, 18, 20. For c = 1 it compares
  with word 12 (4294967295 after init), so every device passes and the last one (7) wins instead of the least (6): the
  assert on word 16 reads 7. For c = 2 and c = 3 the compared words (10 and 8) are 0, so nothing is ever written and
  words 18 and 20 stay 4294967295 where 8 and 12 are asserted. Qrd is 7 and not 6 on devices 1 to 14 on purpose: with
  6 the c = 1 mutant would write 6 anyway.
- **Group B**, `the_three_orders_of_the_devices_are_the_natural_the_reversed_and_a_fixed_shuffle_and_any_other_number_gives_the_shuffle`
  (8 runtime asserts). It binds `qi_ord(k)` for k = 0, 1, 2, 5 and asserts the first and the last word of each table
  (k = 0: 0 and 14; k = 1: 14 and 0; k = 2 and k = 5: 7 and 10). By hand: drop of 234 gives the shuffle at k = 0; flip of
  234 gives the shuffle at k = 0 and the natural order at k = 1 and 2; drop of 235 gives the shuffle at k = 1; flip of
  235 gives the shuffle at k = 1 and the reversed order at k = 2. Each is a first-word mismatch.
- **Group C**, `the_bar_is_inclusive_a_decision_time_exactly_half_a_percent_off_either_way_is_accepted`
  (5 runtime asserts). With variables, `qi_accept(201.0, 200.0)` and `qi_accept(199.0, 200.0)` are true and
  `qi_accept(201.1, 200.0)` and `qi_accept(198.9, 200.0)` are false; the original accepts the exact bar, the mutant `<`
  refuses it. The premise is its own assert: `fabs64(201.0 - 200.0) / 200.0 == QI_BAR`. **It held in the lab**
  (`tri test` passed with it in place): in f64, 1.0 / 200.0 compares equal to the literal QI_BAR, so the bar is
  inclusive at exactly half a percent and the test pins that.

`tri test specs/analog/sar_comparator_qi.t27 --t27c <build t27c>`: 11 tests pass (8 before), 0 fail, 0 vacuous passes of
11, exit 0 (10:21 +07). Asserts per new test: 17, 8, 5.

## Not claimed

- The gate was run once and the seal saved once; there is no second run to compare. The gate output file is in a
  scratch directory outside the repository and is not committed.
- `tri coverage --strict` was not run locally; CI runs it.
- The tool counts 262 kills by a failing test but names no killing test. That each of the 6 new asserts kills its
  mutant is shown by hand against the mutated line; the totals (256 to 262 killed, 20 to 14 survivors) agree with 6
  kills but do not say which assert made which.
- The 13 equivalent survivors are classified by reading the arithmetic, not proved equivalent by a tool; the 18 hung
  are classified by construction: the tool reports each as outliving the timeout, which is consistent with an endless
  loop but was not shown by a run that was let to finish.
- The 14 unviable mutants were never checked by any test. The 10 ret-default ones share one emitter-side message
  (`not yet implemented`, the same for every function with no return value), so it says nothing about the spec's tests.
- The earlier reading in the census note, that the last full evaluation of the last step carries the least counters
  (12 other multiplications, 8 slope reads) in the cost test, was inferred from the survival of `14 - 2 * c` and was not
  counted; the new Group A test does not depend on it, as it builds its counters by hand.
- Only `sar_comparator_qi.t27` was touched among the specs, with its seal; the unviable and hung classes of other
  analog specs are in their own lanes.
