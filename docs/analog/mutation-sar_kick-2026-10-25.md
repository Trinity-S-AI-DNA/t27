# Mutation census of specs/analog/sar_kick.t27, 2026-10-25

Issue #12097, part of epic #10486 (weak spots of the analog v0.1 release). The tool, `tri mutate spec`, never edits the
spec, and the checkout stayed clean during the runs. Where each number comes from:

- **Before** (94 mutants, 77 killed, 12 survived, 5 hung, 0 unviable) is a census run in this lane on the unmodified
  spec: lane base master `2e52102792`, master-built `t27c` (build `b9d83f9ef2`, seals stamp
  `built_by: t27c-bootstrap@0.5.2+19a89f1e3`), `--jobs 3 --timeout 30`, 04:04-04:07 +07, exit 0, load 9.96, 12 GiB
  free. The 12 survivor lines are read off that output.
- **After** is the gate run on the tree of this change, same binary and flags plus `--accepted`: 04:19-04:21 +07,
  exit 0, "Gate passed.". The tests appended to the spec add no mutants (the tool mutates the functions, not the
  tests), so the mutant count is 94 in both runs.

## What was run

Before, the census:

```
tri mutate spec --file specs/analog/sar_kick.t27 --t27c <master-built t27c> --jobs 3 --timeout 30
```

After, the gate (the first seconds of that window are `tri seals changed`):

```
tri mutate spec --file specs/analog/sar_kick.t27 --t27c <master-built t27c> --jobs 3 --timeout 30 \
  --accepted docs/analog/mutation-accepted-sar_kick.txt
```

## Census

| Run | Mutants | Killed | Survived | Hung | Unviable |
| --- | --- | --- | --- | --- | --- |
| Before (unmodified spec, this lane) | 94 | 77 | 12 | 5 | 0 |
| After (this change) | 94 | 81 | 8 | 5 | 0 |

By kind (both runs): flip-cmp 20, swap-logic 5, drop-step 9, swap-arith 48, ret-default 12.

The gate line: 13 not killed (8 survived, 5 hung), 13 accepted by the file, 0 accepted lines now killed.

## The 12 survivors before, and what became of each

| Region (lines) | Survivors | Verdict | Outcome |
| --- | --- | --- | --- |
| `rs_zero_ohm`, the row loop bound (168) | 1 | equivalent: the function returns at row 1 (the offset crosses the latch's own between the 1300 and 2592 ohm rows), so the extra row `i == NRS` is never read | 1 accepted |
| `rs_zero_ohm`, the bracket test `oa > lat && ob <= lat` (170; two mutants) | 2 | equivalent: the tie `oa == lat` or `ob == lat` is not met (9.4155 and 4.086 mV at row 0 and 1, then 4.086 and -0.8945 mV, against 0.153 mV) | 2 accepted |
| `rs_zero_ohm`, the "no row brackets" value (177) | 1 | equivalent: the line is never reached on the pinned data | 1 accepted |
| `late_errors`, the two error tests (255, 256) | 2 | test gap: no test put a threshold exactly on a pinned decision, so the rule "a 0 above, or a 1 at or below, the threshold is an error" was not pinned on its tie | 2 killed |
| `late_thr` and `pooled_min_errors`, the scan bound `j < SCAN_N` (275, 302) | 2 | equivalent: the extra point `j == SCAN_N` is -4.0 mV, where no index has fewer errors than its best (asserted) | 2 accepted |
| the same functions, the improvement test `e < best` (277, 305) | 2 | equivalent: at `e == best` the assignment writes the value `best` already holds | 2 accepted |
| `late_thr`, the bound of the run-finding loop `j < SCAN_N` (284) and the half width `SCAN_STEP * ((last - first) / 2)` (294; the mutant divides instead of multiplying) | 2 | test gap: every index a test read had records, so the fewest-errors run was never the whole grid, and neither the last grid point nor a wide half width was read | 2 killed |

Totals: 12 survivors, of which 4 killed and 8 accepted as equivalent (the rows add up: 1 + 2 + 1 + 2 + 2 + 2 + 2 = 12;
killed 2 + 2 = 4; accepted 1 + 2 + 1 + 2 + 2 = 8). The After run shows exactly the 8 accepted lines alive, which is
the check that the 4 others died. The per-mutant attribution to the test that killed it was not measured, only the
totals before and after.

The 5 hung mutants are drop-step mutants that make a loop endless (the row loop of `rs_zero_ohm`, the record loop of
`late_errors`, the two scan loops of `late_thr`, the scan loop of `pooled_min_errors`), so no test can go red and only
the clock ends the mutant. They are named in the accepted file with that reason. There are no unviable mutants.

## The 3 tests added (appended after the last original test, so the accepted line numbers did not move)

- a decision index without records has no errors and its threshold run is the whole scan grid (10 runtime asserts);
- a decision exactly on the threshold follows the rule: a 0 above, or a 1 at or below, is an error. Two pinned records
  are used as the threshold, with neighbours at plus and minus 1 nV, so the delta is +1 or 0 by the rule and cannot be
  met by accident (6 runtime asserts reported; the code has 10, the 4 that compare constant array elements are not in
  the count, and why was not established);
- the scan grid end and the series resistor bracket leave no tie or extra point to decide anything: they back the
  accepted notes of lines 168, 170, 177, 275 and 302 with strict assertions (11 runtime asserts).

`tri test specs/analog/sar_kick.t27`: 13 tests pass, 0 fail, 0 vacuous passes. The seal is re-saved with the
master-built `t27c` (`tri seals changed`: seal-save, seal-recheck, importers and importer-seal pass; its `seal` step
exits 1 on the stale seal before the save, as the tool expects).
`tri coverage 'specs/analog/*.t27' --strict`: 26 specs, `gaps 0 coverage 130/130`.

## Reading the accepted file

`docs/analog/mutation-accepted-sar_kick.txt` follows `specs/tri/mutate/survivors.t27`: one `path:line [kind]` per
mutant with a free-text note after it; one entry covers all mutants of that kind on the line. With `--accepted` the tool
exits 2 on an unaccepted survivor or hung mutant, and also on an accepted line whose mutant is now killed, so the file
cannot go stale in either direction. A moved line leaves its mutant unaccepted: edit the spec above line 306 and
the file must be regenerated. New tests go at the end of the spec for that reason.

## Not claimed

- The 8 accepted survivors are classified by reading the arithmetic (each line carries it in the accepted file), not
  proved equivalent by a tool; the tie-free brackets that back them are asserted in the last test on the pinned data
  only.
- The 5 hung mutants are classified by construction (a loop step dropped), not by a run that was allowed to finish.
- Which test killed which of the 4 killed mutants was not measured.
- The count of 6 runtime asserts for the second test against 10 written is unexplained; the test passes and is not
  vacuous either way.
- The tests pin the model's own tie rules and scan bounds; they do not compare the model with ngspice.
- Only `sar_kick.t27` was gated here. The other analog specs have a census with survivors not yet pinned: #12098
  (`sar_cdac`), #12099 (`sar_comparator_psp`).
