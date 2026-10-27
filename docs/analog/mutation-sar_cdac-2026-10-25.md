# Mutation census of specs/analog/sar_cdac.t27, 2026-10-25

Issue #12098, part of epic #10486 (weak spots of the analog v0.1 release). The tool, `tri mutate spec`, never edits the
spec, and the checkout stayed clean during the runs. Where each number comes from:

- **Before** (148 mutants, 122 killed, 13 survived, 12 hung, 1 unviable) is a census run in this lane on the unmodified
  spec: lane base master `6904fa623b`, master-built `t27c` (build `b9d83f9ef2`, seals stamp
  `built_by: t27c-bootstrap@0.5.2+19a89f1e3`), `--jobs 3 --timeout 30`, 04:28-04:32 +07, exit 0, load 5.86, 11 GiB
  free. The 13 survivor lines are read off that output.
- **After** is the gate run on the tree of this change, same binary and flags plus `--accepted`: 04:51-04:55 +07,
  exit 0, "Gate passed.", load 6.39, 10 GiB free. The tests appended to the spec add no mutants (the tool mutates the
  functions, not the tests), so the mutant count is 148 in both runs.

## What was run

Before, the census:

```
tri mutate spec --file specs/analog/sar_cdac.t27 --t27c <master-built t27c> --jobs 3 --timeout 30
```

After, the gate (the first seconds of that window are `tri seals changed`):

```
tri mutate spec --file specs/analog/sar_cdac.t27 --t27c <master-built t27c> --jobs 3 --timeout 30 \
  --accepted docs/analog/mutation-accepted-sar_cdac.txt
```

## Census

| Run | Mutants | Killed | Survived | Hung | Unviable |
| --- | --- | --- | --- | --- | --- |
| Before (unmodified spec, this lane) | 148 | 122 | 13 | 12 | 1 |
| After (this change) | 148 | 130 | 5 | 12 | 1 |

By kind (both runs): drop-guard 1, flip-cmp 25, drop-step 18, swap-arith 93, ret-default 11.

The gate line: 17 not killed (5 survived, 12 hung), 17 accepted by the file, 0 accepted lines now killed. The one
unviable mutant (`event_k` emptied, line 444: `zig: error: not yet implemented`) is rejected before any check ran; the
gate does not count it and the accepted file does not list it.

## The 13 survivors before, and what became of each

| Region (lines) | Survivors | Verdict | Outcome |
| --- | --- | --- | --- |
| `closed_k`, the left side sum `side_sum(...) + dummy` (517) | 1 | test gap: every test used a dummy unit of 0, so `+ dummy` and `- dummy` read the same | 1 killed |
| `closed_k`, the bridge factor `u * bridge / d` (525, 527) | 2 | test gap: every test used a bridge of 1 unit, so `* bridge` and `/ bridge` read the same | 2 killed |
| `worst_inl`, `worst_gain`, `worst_arith`, `worst_ng_vs_f64`, the record test `e > w.err + TOL` (581, 593, 609, 621; `>=` mutants) | 4 | test gap: no test built two codes with the same error, so "the first code of a tie wins" was not pinned | 4 killed |
| `worst_arith`, `e > w.err - TOL` (609) | 1 | test gap: no test had a grid where every error is exactly 0, where this mutant replaces the record at every code | 1 killed |
| `worst_ng_vs_f64`, `e > w.err - TOL_V` (621) | 1 | not constructed: it differs only for a code within 1e-16 V of the best, and the ngspice table is fixed data | 1 accepted |
| `worst_arith`, the code loop bound `c < CODES` (604) | 1 | equivalent: the extra code 1024 sets none of the ten plate bits, so it reads as code 0, whose error is not above the best | 1 accepted |
| `worst_fixed_vs_ng`, the record test (633; `>=` and `- TOL`) | 2 | not constructed: they differ only on an exact tie or a pair within TOL, which no stored-word set builds against the fixed ngspice table | 2 accepted |
| `ties_clear`, the edge test `tie_gap(..) <= 0.0001` (668) | 1 | equivalent: the gap is a multiple of 2^-54 and the f64 literal 0.0001 times 2^54 has a fraction, so the gap is never exactly 0.0001 (asserted in the last test) | 1 accepted |

Totals: 13 survivors, of which 8 killed and 5 accepted (the rows add up: 1 + 2 + 4 + 1 + 1 + 1 + 2 + 1 = 13; killed
1 + 2 + 4 + 1 = 8; accepted 1 + 1 + 2 + 1 = 5). The After run shows exactly the 5 accepted lines alive, which is the
check that the 8 others died. The per-mutant attribution to the test that killed it was not measured, only the totals
before and after.

The 12 hung mutants are drop-step mutants that make a loop endless (the unit-sum loop of `side_sum`, the plate loops of
`derive_k_top`, `closed_k` and `node_volts`, the code loops of the six sweep and worst-code functions, the word loop of
`mismatches`, the ratio loop of `ties_clear`), so no test can go red and only the clock ends the mutant. They are named
in the accepted file with that reason.

## The 6 tests added (appended after the last original test, so the accepted line numbers did not move)

- the closed form follows a bridge of two units and a dummy unit: eight pinned ratios (for example 33/1118 and 544/1118)
  and the matrix derivation compared on all 20 (28 runtime asserts);
- the worst-gain search keeps the first code of an exact tie: every stored word the same power of two, so codes 0 and
  1023 mirror each other and the first one wins (7 runtime asserts);
- the worst-INL search keeps the first code of an exact tie: two codes, 511 and 512, with the same error (7);
- the worst-arithmetic search keeps the first code of an exact tie and finds no worst code on a zero grid: the zero
  grid is where `w.err - TOL` would replace the record at every code (11);
- the search against ngspice keeps the first code of an exact tie (5);
- the rounding-tie gate reads a gap of one ten-thousandth as the edge and never meets it (6).

The ties are built where `TOL` is absorbed by rounding (the error is so large that `err + TOL == err`, or the same with
`TOL_V`, asserted in the tie tests), so `>` and `>=` read differently on a tie. Every expected value was worked out by hand before the run and the
tests passed on the first run (the first compile attempt failed only on a cast the test lowering reads as an integer
cast; the fix changed the expression, not a pinned value).

`tri test specs/analog/sar_cdac.t27`: 19 tests pass (13 before), 0 fail, 0 vacuous passes. The seal is re-saved with the
master-built `t27c` (`tri seals changed`: seal-save, seal-recheck, importers and importer-seal pass; its `seal` step
exits 1 on the stale seal before the save, as the tool expects).
`tri coverage 'specs/analog/*.t27' --strict`: 26 specs, `gaps 0 coverage 130/130`.

## Reading the accepted file

`docs/analog/mutation-accepted-sar_cdac.txt` follows `specs/tri/mutate/survivors.t27`: one `path:line [kind]` per
mutant with a free-text note after it; one entry covers all mutants of that kind on the line. With `--accepted` the tool
exits 2 on an unaccepted survivor or hung mutant, and also on an accepted line whose mutant is now killed, so the file
cannot go stale in either direction. A moved line leaves its mutant unaccepted: edit the spec above line 674 and
the file must be regenerated. New tests go at the end of the spec for that reason.

## Not claimed

- The 5 accepted survivors are classified by reading the arithmetic (each line carries it in the accepted file), not
  proved equivalent by a tool. The two "equivalent" notes (lines 604 and 668) rest on the arithmetic written there; the
  two "not constructed" notes say no test builds the tie, not that none exists.
- The 12 hung mutants are classified by construction (a loop step dropped), not by a run that was allowed to finish.
- Which test killed which of the 8 killed mutants was not measured.
- The tests pin the model's own tie rules and bounds; they do not compare the model with ngspice.
- Only `sar_cdac.t27` was gated here. `sar_comparator_psp.t27` has a census with survivors not yet pinned: #12099.
