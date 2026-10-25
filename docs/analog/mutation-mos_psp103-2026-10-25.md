# Mutation test gaps of specs/analog/mos_psp103.t27, 2026-10-25

Issue #12434, part of epic #10486 (weak spots of the analog v0.1 release). `tri mutate spec` never edits the spec; this
note says which of its 72 survivors were test gaps, what kills each one, which turned out to be equivalent, and what
is still not measured. Where each number comes from:

- **Census** (359 mutants, 264 killed, 72 survived, 18 hung, 5 unviable) is the output kept by the analog loop in
  `h12/census-mos_psp103.out`, on the unmodified spec (2664 lines), lane base master `1b1a27d4d8`. The run time and the
  t27c version are not recorded in that output and are not claimed here. The classification of the 72 survivors is
  `h12/census-class-mos_psp103.md`; the line numbers below are the lines of that census and of this file (the tests
  are appended after line 2664, so no line moved).
- **Tests** are `tri test specs/analog/mos_psp103.t27` in a T27 OS pane, started at 21:26 +07 with load averages
  27.10 23.04 18.44: 51 tests (40 before, 11 new), 51 pass, 0 fail, 0 vacuous of 51 (every test executed asserts).
  The 11 new blocks hold 199 runtime asserts.
- **Gate** (the re-run of the census on the tree with the new tests, with the accepted file) has NOT been run. No
  mutant below is shown killed by a gate result; each claim "kills" is a hand check of the mutated line against the
  assert, nothing more.

## 1. Survivors by label

| Label | Mutants | Which |
| --- | --- | --- |
| Test gap, killed by the new tests | 42 | group A 32 (497-503 x14, 527-529 x6, 557 x2, 558 x2, 559, 560; 418, 421 x2, 422 x2, 425), group B 10 (347, 348 drop-step, 569 x2, 621, 622 x2, 625, 626 x2) |
| Equivalent, from the census | 22 | 643, 646, 689; 670, 671, 673; the 16 of `gate_step` (707, 709 x11, 711, 712, 713 x2) |
| Equivalent, found while writing the tests | 2 | 348 flip-cmp, 349 flip-cmp (section 3) |
| Equivalent, judgement call | 6 | group C: 637 x2, 640 x2, 687 x2 (section 4) |

42 + 22 + 2 + 6 = 72. The 18 hung and the 5 unviable are unchanged and are the same lines as in the census; the hung are
listed in `docs/analog/mutation-accepted-mos_psp103.txt` (the 5 unviable are named in its last comment).

## 2. The tests that kill the test gaps

All 11 blocks are appended after the last original test; no existing line, test or body changed. Every assert reads
`var` values (a constant assert is folded and counted as no assert). The bounds are copied from the spec's own constants,
never typed.

- **A1** `check_ok_refuses_each_measure_over_its_bound_and_accepts_each_at_its_bound` (16 asserts). Seven `var`
  measures, all 0.0; one at a time at twice its bound the check is false (kills the dropped guard of lines 497-503), at
  exactly its bound it is true (kills the `>=` of the same line).
- **A2** `check_q_ok_...` (7 asserts): the same for the three guards of lines 527-529.
- **A3** `q500_ok_...` (9 asserts), on a hand-built `Q500`: `up_rel` and `dn_rel` at twice B_Q_DELAY_REL refuse and at
  the bound pass (557, 558), `wave` at the bound B_Q_WAVE_ALL_UNITS passes (559, the tie), `res` = RES_BOUND + 1 refuses
  (560, the dropped guard).
- **A4** `cross_idx_...` (7 asserts), two samples, level 1.0, from 0 to 1: rising [0,2] gives 0.5 (418), rising [1,2]
  gives -1 (421, start on the level), rising [0,1] gives 1.0 (421, arrival on the level), falling [1,0] gives -1 and
  falling [2,1] gives 1.0 (422), a flat [0,0] gives -1 both ways (425).
- **B1** `hold_stats_counts_a_point_at_the_1e_6_floor` (6): the word 999999 is not counted, 1000000 is (kills the `>` of
  347: the f64 product of 1000000 pA and 1e-12 is exactly the double 1e-6, measured). `..._inside_the_1e_5_band_and_the_floors_are_nested`
  (3): a 2e-5 A point moves `n5` to 1 and leaves `n4` 0 (kills the dropped `n5` step of 348). `hold_stats_floors_nest_on_the_real_tt_sets`
  (16): on the midpoints and the R2 points of both tt devices `n5 > 0` and `n4 <= n5 <= n6 <= n` (the same mutant fails
  `n5 > 0`; the exact `n5` of these sets was not computed, so it is not pinned). The two one-pA-either-side tests
  (`..._1e_5_floor`, `..._1e_4_floor`, 6 asserts each) pin the floors from both sides (9999999 not counted, 10000001
  counted; 99999999 not counted, 100000001 counted); they do not kill a mutant of the census, they hold the floors in
  place.
- **B2** `scale_q_rounds_half_up_and_multiplies_by_num_over_den` (2 asserts that count 625 matching words each): the
  pattern {1000000, 30, 29} scaled by 1/20 gives {50000, 2, 1} (the `- den / 2` of 569 gives {49999, 1, 0}); the
  pattern {7, 8, 9} scaled by 2/3 gives {5, 5, 6} (`src / num` gives {1, 1, 1}, `- den / 2` gives {4, 5, 5}).
- **B3** `hold_corner_lin_is_hold_stats_on_the_arrays_of_its_kind_in_every_corner` (121 asserts): for the five corners,
  both devices and the three kinds, `hold_corner_lin` equals `hold_stats` called by hand on the arrays of that kind
  (field by field on `abs_a`, `rel9`, `n6`), and no two of the three point sets give the same `abs_a` for either device
  in any corner (the teeth: a dropped or inverted kind test reads another set and the equality fails). Kills 621, 622 x2,
  625, 626 x2. The teeth asserts passed, so the three sets do differ in all five corners.

## 3. Two flips that turned out equivalent (348, 349)

Group B4 of the issue asked for the lab check "does `10000000 * 1e-12` equal the literal 0.00001?". It does not: a
temporary t27 test (written, run, removed again) showed that the f64 product for 10000000 pA, and likewise for
100000000 pA against 0.0001, is strictly under the literal, while for 1000000 pA it equals 1e-6 exactly. One pA is a step
of 1e-12, which is far larger than an ulp at 1e-5 (about 1.7e-21), so the next word, 10000001, is over the literal by
about 1e-12 and no u32 word is a tie. `>=` and `>` therefore agree on every word at 1e-5 and 1e-4, and the flips of
348 and 349 cannot be killed by any input; they move to the accepted file. 347 stays a test gap and is killed.

## 4. Group C is a judgement call (637 x2, 640 x2, 687 x2), decided: equivalent

The six mutants move the probe offset of `slope_cells_ok` (Vgs `(cx * 977 + 31) % CELL`, Vds `(cy * 4093 + 17) % CELL`)
and of `gmf_cells_ok` (Vds `(cy + 0.37) * CELL_F`). The identity those functions assert (the interpolant's movement across
a cell equals the slope read off the same four nodes) holds at every position inside a cell, so a correct reader passes
at any offset, the count of ok cells stays 576, and the result of the function does not change. The offsets are `const`
locals, not arguments, so pinning them would need them pulled out into `pub fn`s: a body change and a re-seal, which
this lane may not do. They are recorded as judged-equivalent, one line each, in the accepted file.

The other reading: a test gap of low value (a reader that is wrong only at some fraction of a cell could pass with the
offsets collapsed to a node or to one fixed point). To take it, pull the three offsets out into small `pub fn`s, assert
their values by hand (Vgs offset of cell 0 is 31 words, of cell 1 is 1008 words), and re-seal. Nothing here needs a
lab value, so no survivor was left undecided.

## 5. Not done, and not proven

- The mutation gate has not been run on this tree and the seal of the spec has not been re-saved; the orchestrator does
  both. The accepted file lists 48 mutants (30 survivors, 18 hung) that the gate must find unchanged; if one of the
  accepted lines is reported killed, the file needs that entry removed.
- Each kill above is a hand check of the mutated line against the assert (the mutants were not applied). The tests
  pass on the original spec; whether each fails on its mutant is for the gate to show.
- Inferred, not computed: `mag_diff` at most 1 for 643 and 646, the rounding-noise bound for 689, and the "to within the
  last ulp" equivalence of the 16 `gate_step` mutants (the evidence is their survival under windows of 10 units and the
  hangs at 707 and 715).
- Not pinned: the exact `n5` of the tt sets (the nesting and `n5 > 0` are asserted instead), the `gate_step` mutants
  (they would need a bit-exact fold of the gate run, which the spec does not have; the owner's call), and the six probe
  offsets of group C (section 4).
