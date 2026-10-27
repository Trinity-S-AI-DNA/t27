# Mutation census of specs/analog/sar_comparator_psp.t27, 2026-10-25

Issue #12099, part of epic #10486 (weak spots of the analog v0.1 release). The tool, `tri mutate spec`, never edits the
spec, and the checkout stayed clean during the runs. Where each number comes from:

- **Before** (41 mutants, 26 killed, 9 survived, 5 hung, 1 unviable) is a census run in this lane on the unmodified
  spec: lane base master `41c281458b`, master-built `t27c` (build `b9d83f9ef2`, seals stamp
  `built_by: t27c-bootstrap@0.5.2+19a89f1e3`), `--jobs 3 --timeout 30`, 04:58-05:00 +07, exit 0, load 11.00, 8.5 GiB
  free. The 9 survivor lines are read off that output.
- **After** is the gate run on the tree of this change, same binary and flags plus `--accepted`: 05:07-05:09 +07,
  exit 0, "Gate passed.", load 9.08, 8.0 GiB free. The tests appended to the spec add no mutants (the tool mutates the
  functions, not the tests), so the mutant count is 41 in both runs.

## What was run

Before, the census:

```
tri mutate spec --file specs/analog/sar_comparator_psp.t27 --t27c <master-built t27c> --jobs 3 --timeout 30
```

After, the gate (the first seconds of that window are `tri seals changed`):

```
tri mutate spec --file specs/analog/sar_comparator_psp.t27 --t27c <master-built t27c> --jobs 3 --timeout 30 \
  --accepted docs/analog/mutation-accepted-sar_comparator_psp.txt
```

## Census

| Run | Mutants | Killed | Survived | Hung | Unviable |
| --- | --- | --- | --- | --- | --- |
| Before (unmodified spec, this lane) | 41 | 26 | 9 | 5 | 1 |
| After (this change) | 41 | 32 | 2 | 6 | 1 |

By kind (both runs): drop-guard 3, flip-cmp 8, drop-step 2, swap-arith 19, ret-default 9.

The gate line: 8 not killed (2 survived, 6 hung), 8 accepted by the file, 0 accepted lines now killed. The one unviable
mutant (`sp_scale` emptied, line 137: `zig: error: not yet implemented`) is rejected before any check ran; the gate does
not count it and the accepted file does not list it.

## The 9 survivors before, and what became of each

| Region (lines) | Survivors | Verdict | Outcome |
| --- | --- | --- | --- |
| `sp_first`, the guard `if (q == SC_NONE) { return -1.0; }` (158; drop-guard and `0.0 - 1.0` -> `0.0 + 1.0`) | 2 | test gap: every run in the tests reaches half the supply, so the "no crossing" branch was never taken | 2 killed |
| `sp_flip`, the first-branch result `return 0 - (2 * lo)` (204; `0 - ` -> `0 + `) | 1 | test gap: no test reached the first branch with a non-zero result (a flip of 0 reads the same as `0 + 0`), so the sign of that result was not pinned | 1 killed |
| `sp_abs_i`, the whole body and its guard `if (x < 0) { return (0 - x) }` (216 ret-default, 217 drop-guard, 217 `0 - x` -> `0 + x`) | 3 | test gap: only non-negative integers reached the absolute value | 3 killed |
| `sp_flip`, the first-branch loop `hi - lo > 1` -> `hi + lo > 1` (200) | 1 | test gap: the loop is entered with lo = 0, where `hi + lo` and `hi - lo` read the same; the mirrored pair enters with lo = 35031 and the mutant never ends | 1 now hung, accepted |
| `sp_res_max`, the record test `res[i] > worst` -> `>=` (179) | 1 | equivalent: the two differ only at `res[i] == worst`, where the assignment writes the value `worst` already holds | 1 accepted |
| `sp_abs_i`, the guard `x < 0` -> `x <= 0` (217) | 1 | equivalent: the two differ only at `x == 0`, where the branch returns `(0 - 0) as f64`, the same word as the fall-through | 1 accepted |

Totals: 9 survivors, of which 6 killed, 1 turned into a hung mutant and 2 accepted (the rows add up: 2 + 1 + 3 + 1 + 1 + 1
= 9; killed 2 + 1 + 3 = 6; hung 1; accepted 1 + 1 = 2). The After run shows exactly the 2 accepted lines alive and the
hung count 5 -> 6, which is the check that the others changed as listed. The per-mutant attribution to the test that
killed it was not measured, only the totals before and after.

The 6 hung mutants after are: the two drop-step mutants that make a loop endless (the word loop of `sp_scale` at 141 and
the step loop of `sp_res_max` at 180), and the four loop-bound mutants of the two bisections in `sp_flip` (`hi - lo > 1`
flipped to `>= 1` or swapped to `hi + lo > 1` at 200; the same two at 208). A bisection step that cannot move the
bracket never ends, so no test can go red and only the clock ends the mutant. They are named in the accepted file with
the reason.

## The 3 tests added (appended after the last original test, so the accepted line numbers did not move)

- the times are minus one where no output reaches half the supply: an all-zero output reads -1 from both the first
  crossing and the resolution time; one output that jumps from 0 to the supply at the last sample reads 131.5 steps
  (5 runtime asserts);
- the absolute value of a signed integer changes only the negative side: -5 and 5 give 5, 0 gives 0, -140000 gives
  140000 (4);
- the mirrored unequal pair moves the flip point to the other side: with the multiplier widths (36, 35) the second
  input wins at d = 0 and the flip is +70064; with (35, 36) the first wins at d = 0 and the flip is -70062. The two flips
  have the opposite sign, their sum is within 4 of 0, and the magnitude of the mirrored one is 2.1 % below the
  71580 reference (12).

The mirror pair is the second source for the first-branch result: the first branch is reached only when the first
input wins at d = 0, and only the (35, 36) pair of this test reaches it with a non-zero result. The two flip values were measured
once in this lane (probe tests that assert one candidate each, then removed; `assert_eq` does not print the values) and
are pinned as the exact words 70064 and -70062; their sum, 2, is the model's own asymmetry between the two bisection
branches, measured and not explained here, and not a claim about ngspice.

`tri test specs/analog/sar_comparator_psp.t27`: 16 tests pass (13 before), 0 fail, 0 vacuous passes. The seal is
re-saved with the master-built `t27c` (`tri seals changed`: seal-save, seal-recheck, importers and importer-seal pass;
its `seal` step exits 1 on the stale seal before the save, as the tool expects).
`tri coverage 'specs/analog/*.t27' --strict`: 26 specs, `gaps 0 coverage 130/130`.

## Reading the accepted file

`docs/analog/mutation-accepted-sar_comparator_psp.txt` follows `specs/tri/mutate/survivors.t27`: one `path:line [kind]`
per mutant with a free-text note after it; one entry covers all mutants of that kind on the line. With `--accepted` the
tool exits 2 on an unaccepted survivor or hung mutant, and also on an accepted line whose mutant is now killed, so the
file cannot go stale in either direction. A moved line leaves its mutant unaccepted: edit the spec above line 690 and
the file must be regenerated. New tests go at the end of the spec for that reason.

## Not claimed

- The 2 accepted survivors are classified by reading the arithmetic (each line carries it in the accepted file), not
  proved equivalent by a tool.
- The 6 hung mutants are classified by construction (a loop that cannot move its bracket), not by a run that was
  allowed to finish.
- Which test killed which of the 6 killed mutants was not measured.
- The pinned flips 70064 and -70062 are the model's own words at this seal, not ngspice values; the tests compare only
  the mirror (opposite sign, sum within 4 steps, magnitude 2.1 % under the reference).
- Only `sar_comparator_psp.t27` was gated here. Other analog specs are covered by their own lanes (#12096, #12097,
  #12098).
