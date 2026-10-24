# Mutation census of specs/analog/sar_adc.t27, 2026-10-25

Issue #12096, part of epic #10486 (weak spots of the analog v0.1 release). The tool, `tri mutate spec`, never edits the
spec, and the checkout stayed clean during the runs. Where each number comes from:

- **Before** (98 mutants, 76 killed, 14 survived, 8 hung, 0 unviable, and the 14 survivor lines) is the census recorded in
  issue #12096: measured 2026-10-25 on master `b9d83f9ef2` with the master-built `t27c`, `--jobs 4 --timeout 30`, exit 0.
  It was not re-run in this lane, so the survivor kinds on lines 413 to 445 below are read off the source lines, not off
  the census output.
- **After** is read from two runs in this lane, on the tree of this change (lane base master `6e8e0212bd`; no commit
  between `b9d83f9ef2` and it touches `specs/analog/sar_adc.t27`, read with `git log`), with the master-built `t27c`
  (`built_by` of the re-saved seal ends in `+19a89f1e3`): a census and the gate.

## What was run

After, the census (03:57-04:00 +07, exit 0):

```
tri mutate spec --file specs/analog/sar_adc.t27 --t27c <master-built t27c> --jobs 3 --timeout 30
```

After, the gate (04:02-04:06 +07, exit 0, "Gate passed."; the first minute of that window is `tri seals changed`):

```
tri mutate spec --file specs/analog/sar_adc.t27 --t27c <master-built t27c> --jobs 3 --timeout 30 \
  --accepted docs/analog/mutation-accepted-sar_adc.txt
```

## Census

| Run | Mutants | Killed | Survived | Hung | Unviable |
| --- | --- | --- | --- | --- | --- |
| Before (master, as recorded in the issue) | 98 | 76 | 14 | 8 | 0 |
| After (this change, both runs agree) | 98 | 85 | 5 | 8 | 0 |

By kind (read from the After runs): drop-guard 4, flip-cmp 30, swap-logic 5, drop-step 15, swap-arith 36, ret-default 8.

The gate line: 13 not killed (5 survived, 8 hung), 13 accepted by the file, 0 accepted lines now killed.

## The 14 survivors before, and what became of each

| Region (lines) | Survivors | Verdict | Outcome |
| --- | --- | --- | --- |
| `ideal_code`, the clamps (239, 240; two mutants on each line) | 4 | one mutant on each line is the flip-cmp (`<` to `<=`, `>=` to `>`) and is equivalent: at `x == 0.0` the fall-through cast gives 0, the value the clamp returns, and at `x == 2047.0` it gives 2047. The other mutant on each line (a dropped guard, by the shape of the line) was a test gap: no original test fed a voltage below the first code or above the last | 2 killed, 2 accepted (the two flip-cmp, still alive in the After runs) |
| `miss_margins` (370) | 1 | equivalent: at a margin equal to `worst` the assignment writes the value `worst` already holds | 1 accepted |
| `err_range` (387, 388) | 2 | equivalent: at an error equal to `lo` or `hi` the assignment writes the value already held | 2 accepted |
| `fit_grid`, the loop bounds `while (a < FIT_NA)` and `while (b < FIT_NB)` (413, 417) | 2 | test gap: nothing read the grids at their last point or one past it | 2 killed |
| `fit_cp`, the loop bound (439), the `hits >= best.hits` test (443) and the three mutants of the improvement test `hits > best.hits \|\| v < best.viol` (445) | 5 | test gap: the choice of the first point of most matches and least margin was never pinned against a hand scan | 5 killed |

Totals: 14 survivors, of which 9 killed and 5 accepted as equivalent (the rows add up: 4 + 1 + 2 + 2 + 5 = 14; killed
2 + 2 + 5 = 9; accepted 2 + 1 + 2 = 5). The After runs show exactly the 5 accepted lines alive, which is the check that
the 9 others died. The per-mutant attribution to the test that killed it was not measured, only the totals before and
after.

The 8 hung mutants are drop-step mutants that make a loop endless (the decision loop of `follow`, the word loop of
`words_of`, the conversion loops of `match_count_p`, `miss_margins` and `err_range`, the offset and Cp loops of
`fit_grid`, the Cp loop of `fit_cp`), so no test can go red and only the clock ends the mutant. They are named in the
accepted file with that reason. There are no unviable mutants.

## The 3 tests added (appended after the last original test, so the accepted line numbers did not move)

- the ideal quantizer clamps below the first code and above the last (7 asserts);
- `fit_cp` returns the first point of most matches and least margin, equal to a hand scan of the 41 Cp points, on every
  one of the 41 offsets of the fit grid (164 runtime asserts);
- a model one step past the end of a fit grid is not found, and one in the middle of an interval is found at its first
  point; the 31 inputs are built by bisection on the model itself, a low-edge and a high-edge placement alternating, and
  bracketed by 80 mV so that a 2 % gain error cannot slip through (290 runtime asserts).

`tri test specs/analog/sar_adc.t27`: 18 tests pass, 0 fail, 0 vacuous passes. The seal is re-saved with the
master-built `t27c` (`tri seals changed`: seal-save, seal-recheck, importers and importer-seal pass).
`tri coverage 'specs/analog/*.t27' --strict`: 26 specs, `gaps 0 coverage 130/130`.

## Reading the accepted file

`docs/analog/mutation-accepted-sar_adc.txt` follows `specs/tri/mutate/survivors.t27`: one `path:line [kind]` per mutant
with a free-text note after it; one entry covers all mutants of that kind on the line. With `--accepted` the tool exits
2 on an unaccepted survivor or hung mutant, and also on an accepted line whose mutant is now killed, so the file cannot
go stale in either direction. A moved line leaves its mutant unaccepted: edit the functions above line 833 and the file
must be regenerated. New tests go at the end of the spec for that reason.

## Not claimed

- The Before census is the issue's, not a run of this lane; the kinds of the survivors on lines 413 to 445 are read off
  the source, not off the census output.
- The 5 accepted survivors are classified by reading the arithmetic (each line carries it in the accepted file), not
  proved equivalent by a tool.
- The 8 hung mutants are classified by construction (a loop step dropped), not by a run that was allowed to finish.
- The construction test pins the loop bounds and the tie rule of the fit on the model itself; it does not compare the
  model with ngspice.
- Only `sar_adc.t27` was gated here. The other analog specs have a census with survivors not yet pinned: #12097
  (`sar_kick`), #12098 (`sar_cdac`), #12099 (`sar_comparator_psp`).
