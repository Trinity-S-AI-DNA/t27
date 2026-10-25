# Mutation test gaps of specs/analog/sar_switch.t27, 2026-10-25

Issue #12287, part of epic #10486 (weak spots of the analog v0.1 release). This lane wrote the missing asserts, the
accepted file and this note. The mutation gate and the seal were run afterwards by the analog loop (section "The gate
run"). Where each number comes from:

- **Census** (400 mutants, 306 killed, 53 survived, 33 hung, 8 unviable) is the analog-loop census of the unmodified
  spec on 2026-10-25, tree 4d03070df6 (it holds the spec of master f81a02f36a), run by `tri mutate spec`.
  The 53 survivors were classified by reading the spec after that census. No census mutant was re-run in this lane.
- **Tests** are measured: `tri test specs/analog/sar_switch.t27` in a pane on this tree (base master d5650360c1 plus
  294 added lines, all at the end of the spec, so no existing line moved): tests 22, pass 22, FAIL 0, vacuous passes
  0 of 22. The 14 existing tests are unchanged (230 counted asserts); the 8 new tests count 62 asserts, 292 in all.
- **Kills** are measured one mutant at a time, by hand and not by the gate: a copy of the spec with the new tests, one
  `sed` edit that makes the mutant of the census row, `tri test` in a pane, the exit code. Exit 1 with a failing test
  is a kill, exit 0 a survivor. 46 mutants were run this way (the 39 survivors the census labelled test gap and the 7
  it labelled below one ulp): 43 killed, 3 survive. The test that goes red is the one named in the tables below (a
  few mutants turn more than one test red, for example `s = s` on line 501 turns two). Which assert inside that test
  fails is a reading of the mutated text against the assert, written next to each assert as a comment.
- **Expected values** in the new asserts are not hand arithmetic: they are figures from pane runs on this tree (the
  windows of groups A and C, the counts of group B, the pinned maxima of group D, the sums of group E), written into
  the asserts and then checked by a green first run on the unmodified spec. The group F values are the correctly
  rounded sine and cosine at 0.05 and a series value at 0.4 (section "Group F").

Kinds among the 53 survivors: flip-cmp 21, swap-arith 21, swap-logic 5, drop-step 4, ret-default 2.

## The 53 survivors, by label

The census labelled 14 equivalent (7 of them "below one ulp", 1 "not constructed"), 39 test gap and 0 dead line. This
lane moved three of the test gaps to "not constructed" and "dead line", and killed the rest, including the seven
"below one ulp" ones:

| Outcome | Count | Lines | Entry in the accepted file |
| --- | --- | --- | --- |
| equivalent, accepted | 6 | 234, 236, 238, 510, 511, 772 | yes |
| not constructed, accepted | 3 | 467 (census: equivalent), 327 and 500 (census: test gap) | yes |
| dead line, kept | 1 | 792 (census: test gap, maybe a dead line) | yes |
| below one ulp in the census, killed | 7 | 400 (2), 406 (5) | no |
| test gap, killed | 36 | groups A to F below | no |

6 + 3 + 1 + 7 + 36 = 53. Killed in all: 7 + 36 = 43; not killed: 10.

### Test gaps and below-ulp rows, killed by the new tests (43)

| Group | Mutants | New test (the one that fails) | What fails on the mutant |
| --- | --- | --- | --- |
| A, the gap helpers had upper bounds only (3) | 822, 849 | `the_fixed_vs_real_gaps_are_pinned_from_both_sides` | `sw_gap_g` and `sw_gap_j` are above 0.0 at steps 6, 20, 100 and 333 of the Vin = 0.75 V run; the `ret-default` mutants return 0.0 |
| | 842 | same | the `SC_FG - n` read gives gg of about 2705, 2705, 3097 and 4059 at the four steps, above the new upper bounds (2600, 2400, 1750, 1950); the lower bounds (2100, 1800, 1300, 1450) bound the real value from below |
| B, the real twin's count and cap (3) | 515, 517 | `the_real_twin_counts_its_iterations_and_obeys_its_cap` | at k = 128 the maximum of `nit` is 86, the sum 439, 33 steps move twice or more and 302 at least once (measured on the original); by reading, `used = i` gives a maximum of 85 and a sum 137 lower (439 - 302), and the dropped counter leaves every count at 0 or 1 |
| | 497 | same | at k = 1 the maximum is exactly 1 and at k = 2 exactly 2; `i > k` lets one more iteration through |
| C, helpers at their boundary (4) | 659, 683, 709 | `the_test_helpers_are_exact_at_their_boundary` | `sw_cnt_gt([1, 64, 65], 3, 64)` is 1; `sw_argmax` of two equal largest gaps at 5 and 9 is 5; `sw_within(0.5, 0.0, 0.5, 0.0)` is true and 0.75 is outside |
| | 753 | `the_transposed_control_error_sits_in_its_window` | the transposed error is in 70..100 at step 20 and 7..10 at step 100; the `j / SC_NN + i` read gives about 4.3e4 at both |
| D, `sw_headroom` pinned on one stimulus (15) | 773 (11), 801 (2), 802, 807 | `the_headroom_maximum_is_pinned_on_three_crafted_stimuli` | the maximum on three crafted stimuli equals 78939255824449536 (a 0.2 V pulse at step 7 on a 1.3 V rail), 73891641910362112 (a step down to 0.2 V from step 331 on) and 337192909219837760 (a 1.3 V pulse at step 333 on a 0.2 V rail) to a relative 1e-12; each of the 15 mutants (the `or` chain of the step list, the row-sum terms and the start column) turns this test red |
| E, the damping ties and saturation (3) | 326 | `the_fixed_newton_is_pinned_on_two_flat_stalled_inputs` | on flat 1.8 V and 2.5 V inputs the sum of all 416 states (108596709576, 118642305180), the sum of the iteration counts (892, 1072), the steps at the cap K (4, 5) and at 1.8 V the sum of the residuals (368943) are pinned; the `r > rprev` mutant gives 108596703728 and 368592 at 1.8 V and 118642304328 at 2.5 V |
| | 501 (2) | `the_real_newton_damps_on_the_same_two_flat_inputs` | the real twin's iteration sums (893, 1069) and capped steps (4, 5) on the same inputs; `s = s` gives 882 and 1070 instead of 893 and 1069 (it also turns the count test of group B red); by reading, `s <= SW_SMAX` lets the counter reach 7 and index past the 7 entries of `SW_DAMP_F`, which stops the run |
| F, the series constants (8 + 7) | 400 (6), 406 (9) | `the_small_angle_series_hold_their_constants` | see "Group F" |

Counted asserts per new test (`tri test` counts a loop assert once per pass): 24, 12, 4, 4, 3, 7, 4, 4 in the order of
the tests in the spec (62 in all). Every value read from a table is read into a `var` first, so no assert is folded
at compile time and none is vacuous.

### Group D

The production stimulus lets step 5 alone set the maximum (the existing test holds it between 7.0e16 and 7.5e16; the
value is 7.23e16), so the other six measured steps, the `or` chain that lists them and the terms of the
back-substitution row sum never decided the result. The three crafted stimuli are a single 0.2 V pulse at step 7 on a
1.3 V rail, a step to 0.2 V from step 331 on, and a single 1.3 V pulse at step 333 on a 0.2 V rail. Which of the three
turns which of the 15 mutants red is not recorded: the test goes red as a whole (measured), the attribution by
stimulus is not.

### Group F

`sw_sin_small` and `sw_cos_small` are series in d2 = d * d. The stimulus is quantised at 2^-24 V, so an error below
about 5e-8 in sin or cos never reaches a state, and the constants were invisible. They are pinned by direct calls:

- at d = 0.05, the top of the claimed domain |d| < 0.05, the values are the correctly rounded sin and cos (the series
  truncation is about 1e-22 there), held to 4.2e-18 and 6.7e-17 (0.6 of an ulp);
- at d = 0.4, outside the claimed domain, every term is large enough to show: the value pinned is the series value, not
  sin 0.4: the true value plus the omitted terms (about +1.05e-12 for sin and -3.5e-14 for cos), held to 2.0e-16.

All 15 mutants of lines 400 and 406 (all `swap-arith`) turn the test red. Which assert goes red on which mutant is a
reading: the seven the census called below one ulp (400 x2 and 406 x5) can only be seen at d = 0.4. That is a pin
outside the domain the circuit reaches: those seven mutants stay invisible to the circuit and are killed only because
the function is pinned as a series. They get no entry; if the owner prefers to treat them as equivalent in the domain,
the entries would be `[swap-arith]` rows at 400 and 406 and the pin at 0.4 would go.

### Equivalent (6), accepted

| Line | Mutant | Why it is equivalent |
| --- | --- | --- |
| 234 | `j >= SW_JF + SW_R` to `>` in `sw_clk_at` | differs only at j = 336: the original returns 0; the mutant goes on with m = 8 and f = 0, `f < m` sets m to 0 and it returns 0 |
| 236 | `m > SW_R` to `>=` | at m = SW_R the mutant assigns SW_R to a variable that already holds SW_R |
| 238 | `f < m` to `<=` | at f = m the mutant assigns f to m, the same value |
| 510 | `z < 0.0` to `<=` in `sw_step_f` | at z = 0.0 the mutant assigns 0.0 to a variable that holds 0.0 (a negative zero compares equal to 0.0) |
| 511 | `z > SW_TOP_F` to `>=` | at z = SW_TOP_F the mutant assigns SW_TOP_F to a variable that holds it |
| 772 | `j <= 335` to `<` in `sw_headroom` | step 335 is not on the step list, so its pass only runs `sw_step`; `wide` is final after step 334 and the last result is not read |

### Not constructed (3), accepted

- **467** `... >= lim` to `>` in `sw_settled_f`: differs only when `fabs64(F) / J` equals `lim` (1e-6, or an integer
  plus 0.5) to the last bit of an f64. No test builds that, and it is not proven unreachable.
- **327** `s < SW_SMAX` to `<=` in `sw_step` (fixed point): differs only when the damping counter is already 6 and the
  residual does not fall; the mutant then shifts the move by 7 instead of 6. Pane probes on flat stalled inputs of
  1.8 V, 2.5 V and 3.0 V gave identical final states for the original and the mutant; the reading is that the stalled
  move is already 0 at both shifts. No witness found, so no test.
- **500** `r >= rprev` to `>` in `sw_step_f` (real twin): differs only on a tie of two f64 residuals, possible only
  when an iteration leaves the state unchanged. On the flat 1.8 V and 2.5 V inputs the iteration counts and the state
  sums are the same both ways, and the mutant passes all 22 tests. No witness found, so no test.

"No witness found" means not found on the inputs tried; it is not a proof that none exists.

### Dead line (1), kept

**792** `before * CELL_F` to `before / CELL_F` in `sw_headroom`: the product never sets the returned maximum on any
stimulus the tests use. If it did, the quotient would lower the pinned maximum and the test would fail; the mutant
passes all 22 tests, among them the production window and the three crafted stimuli pinned to a relative 1e-12. The line
stays in the code as the documented bound on the elimination. Whether some stimulus makes it the widest word is not proven; one that does would
turn it into a test gap.

### Hung (33) and unviable (8)

The 33 hung mutants are all `x = x + 1` turned into `x = x` in the counter of the `while` that encloses it, so the loop
never ends and no test can go red (lines 279, 293, 307, 337, 371, 381, 383, 427, 456, 468, 480, 513, 534, 543, 545,
554, 660, 671, 687, 700, 702, 743, 755, 757, 778, 787, 808, 811, 815, 831, 843, 858, 870). The 33 lines in the accepted
file are exactly the 33 hung lines of the census output (compared as lists). The four drop-step mutants that survived
are 501 (the damping counter of `sw_step_f`), 515 (the iteration count it hands back), 517 (its iteration counter) and
802 (the start column of the row sum in `sw_headroom`); the new tests kill all four.

Not every loop counter hangs: the census counts 48 drop-step mutants, of which 33 hang and 4 survived, so by
elimination 11 were killed by a failing test. Lines 270 (`n = n + 1` in `sw_stamp`) and 342 (`i = i + 1` in `sw_step`)
are in none of the hung, survivor and unviable lists, so they are among those 11. Why those loops end is not
established here.

The 8 unviable mutants are the `ret-default` mutant of a function with no return value (body emptied), which zig
answers with "not yet implemented": `sw_stamp_cfloat` 253, `sw_stamp` 265, `sw_rest` 350, `sw_make_vin` 423,
`sw_stamp_f` 441, `sw_run_f` 525, `sw_widen` 550, `sw_errors` 880. That the cause is on the emitter side and not in
this spec is not established here. By contrast `sw_gap_g` 822 and `sw_gap_j` 849 return a value, compiled and
survived; the new tests kill them.

## The accepted file

`docs/analog/mutation-accepted-sar_switch.txt` has 51 entries: 6 equivalent, 3 not constructed, 1 dead line (kept), 33
hung, 8 unviable, in the form `path:line [kind] note` that `tri mutate spec --accepted` reads. It has no entry for any
of the 43 mutants the tests kill. Reading the gate's rule (an entry is checked per line and kind, and it is stale only
when mutants of that line and kind ran and all were killed; unviable mutants are left out of that test), the 8
unviable entries are inert today, as in the other accepted files that carry them.

## The gate run

Measured on 2026-10-25, tree 07966d524e (this lane before the final rebase), run by
`tri mutate spec --file specs/analog/sar_switch.t27 --accepted docs/analog/mutation-accepted-sar_switch.txt --jobs 3
--timeout 45`: 14:49:42 to 15:07:09 (17 min 27 s), exit 0, "Gate passed." The machine load was 18.56 at the start and
rose above 60 for a few minutes in the middle (other sessions and the desktop app); no verdict was changed by hand.

| | mutants | killed | survived | hung | unviable |
|---|---|---|---|---|---|
| census, before this lane | 400 | 306 | 53 | 33 | 8 |
| gate, with the 8 new tests | 400 | 349 (349 by a failing test, 0 by an invariant) | 10 | 33 | 8 |

43 mutants are not killed (10 survived, 33 hung); all 43 are accepted by the file above, and 0 accepted lines are now
killed. The figures I predicted from the by-hand run (349 killed, 10 survived, 33 hung, 8 unviable) are exactly the
measured ones. The hung count is the census's 33, so the load spike turned no kill into a hang. The 10 survivors are
exactly lines 234, 236, 238, 510, 511, 772 (equivalent), 327, 467, 500 (no witness) and 792 (dead on the stimuli
tried); lines 400 and 406 are absent from the not-killed list, so their 15 mutants are killed.

## Caveats (what is by hand or not established)

- **The equivalents** (234, 236, 238, 510, 511, 772) rest on reading the code, not on an exhaustive run.
- **327, 500, 467** have no witness; the entries say so. A witness would make each a test gap.
- **792** is dead on the stimuli tried, not for every stimulus.
- **The group F pin at d = 0.4** lies outside the claimed domain; see "Group F".
- **The windows of groups A and C** (gg, gj, the transposed error) come from a pane run read as a bucket about 7 % wide
  and then widened; they bound the real value on both sides but are not tight. The counts of group B, the maxima of
  group D and the sums of group E are exact figures of this tree's compiler; a change that moves f64 results at the
  last bit would move the group D and E figures first.
- **The flat inputs of group E** (1.8 V and 2.5 V) park the solve against a clamp so that the integer residual repeats;
  that reading explains why the damping counter rises there, but no per-iteration trace was printed.
- **The by-hand kill run** is not `tri mutate`: it uses `tri test` on an edited copy. A mutant that the gate would
  report as unviable or hung would look different, and all 46 here compiled and ran to a test report.

## Not done

- The seal of `analog_AnalogSarSwitch` was saved (tests 22 of 22) and `tri seals changed` passes after the rebase;
  see the PR for the final run.
- `tri coverage --strict` and the CI docs check were not run in this lane.
- 327 and 500 have no test (no witness); 792 is not pinned.
- The 33 hung lines were not re-run one by one: the gate ran them at the one `--timeout` 45 and each stayed hung, so
  they are clock results at that timeout, as in the other accepted files.
