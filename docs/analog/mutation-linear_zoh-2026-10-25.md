# Mutation test gaps of specs/analog/linear_zoh.t27, 2026-10-25

Issue #12276, part of epic #10486 (weak spots of the analog v0.1 release); the census that found the gaps is #12116.
This note covers the tests appended to the spec, the accepted file `docs/analog/mutation-accepted-linear_zoh.txt`, and
what has not been done. Where the numbers come from:

- **Census** (252 mutants, 195 killed, 30 survived, 27 hung, 0 unviable; the 29 hung of the first census became 27 after
  the constant-element audit of #12166, the 30 survivors being byte-identical in both) is copied from the issue.
- **Gate** (`tri mutate spec --file specs/analog/linear_zoh.t27 --accepted docs/analog/mutation-accepted-linear_zoh.txt
  --jobs 3 --timeout 45`, T27 OS pane, 14:36 to 14:49 +07 = 13 min 35 s, load average 17.98 at the start): 252 mutants,
  223 killed (195 + the 28 test gaps), 2 survived (176 and 433, both equivalent and accepted), 27 hung (all accepted),
  0 unviable. The gate exited 2 on one line only: accepted line 177 (`nrm / 2.0` to `nrm * 2.0`) was killed, so it was
  removed from the accepted file and left as a comment there, with the reason. The full gate was not repeated on the
  final accepted file; the removal cannot change a verdict, since the 29 other not-killed mutants are all accepted.
- **Tests** are the lane's own: `tri test specs/analog/linear_zoh.t27 --t27c <compiler>` in a T27 OS pane, 12:24 +07,
  exit 0, load average 13.75 at the start, 21 GiB free. The compiler is the shared release build of the compiler (the
  lane brief calls it slightly old), not a fresh master build. Lane base: master `f81a02f36a`.

## The 30 survivors, labelled

| Line | Mutant | Label | Killed by (test, assert) |
| --- | --- | --- | --- |
| 157 | `<=` to `<` in `near` | test gap, group E | `near_holds_at_its_exact_tolerance`: `near(1.5, 1.0, 0.5)` and `near(2.0, 2.0, 0.0)` are true (the mutant gives false for both) |
| 176 | `>` to `>=` in the halving loop of `pade_expm` | equivalent | none; accepted file |
| 193, 198, 211, 221, 233, 241 | `<` to `<=` in the loops of `pade_expm` | test gap, group D | `the_five_state_exponential_and_hold_stay_inside_their_arrays`: `pade_expm` at n = 5 on diag(-1 .. -5), the diagonal equals exp(-1) .. exp(-5) and two off-diagonal cells are 0 |
| 279, 284 | `<` to `<=` in the copy loops of `zoh` | test gap, group D | the same test: `zoh` at n = 5 with dt = 1 and b = 1, the diagonal of M is exp(-l) and N_i is (1 - exp(-l)) / l |
| 390, 393, 401 | `<` to `<=` in the loops of `grid_err` | test gap, group D | `the_two_state_circuit_in_five_states_gives_the_two_state_runs`: `grid_err` on the 2-state circuit embedded in 5 states equals the 2-state run (`err` and `scale`) |
| 507, 510, 518 | `<` to `<=` in the loops of `ring_f64` | test gap, group D | the same test: `ring_f64` embedded in 5 states equals the 2-state run (`scheme` and `analytic`) |
| 544 | `<` to `<=` in the copy loop of `ring_fixed` | test gap, group D | the same test: `ring_fixed` embedded in 5 states equals the 2-state run |
| 388, 421 | `t < k` to `t <= k` in `grid_err`, `grid_err_rc` | test gap, group B | `a_one_step_run_has_exactly_one_grid_point`: at k = 1 the RC scale equals `1.0 - @exp(0.0 - 0.05)` (0.0488, bounds 0.0487 .. 0.0489) and the RLC scale equals the larger of `an_v` and `an_z` at RLC_DT to 1e-15 |
| 433 | `a > b` to `a >= b` in `fmax64` | equivalent | none; accepted file |
| 453, 469 | `RLC_DT / m` and `RC_DT / m` to `*` in `rlc_ex_err`, `rc_ex_err` | test gap, group A | `the_exact_grids_are_the_grids_of_their_backward_euler_twins`: for j = 1, 2 and r = 100, 5 and 0.5, `rlc_ex_err(r, j, STEPS).scale == rlc_be_err(r, j, STEPS).scale`, and `rc_ex_err(j, STEPS).scale == rc_be_err(j, STEPS).scale` |
| 505 | `t < k` to `t <= k` in `ring_f64` | test gap, group C | `the_f64_ring_window_is_the_last_stamps_of_the_run`: one-state system a = 1.0001, `scheme / exp(k ln a)` is 1 to 1e-7 (the mutant gives a^(k+1), ratio 1.0001) |
| 522 | `t + w >= k` to `>` in `ring_f64` | test gap, group C | the same test: a = 0.9999, `scheme / exp((k - w + 1) ln a)` is 1 to 1e-7 (the mutant loses stamp 12440 and gives ratio 0.9999) |
| 524 (drop-step and swap-arith) | `(t + 1) * dt` to `t * dt` and `(t - 1) * dt` in `ring_f64` | test gap, group C | the same test: r = -0.5, `analytic / abs(1 - an_v(r, k dt))` is 1 to 1e-9 (the growing ring peaks on stamp 12566; stamp sets ending one or two earlier lose about 2e-3, hand arithmetic) |
| 541 | `t < k` to `t <= k` in `ring_fixed` | test gap, group C | `the_fixed_point_ring_window_is_the_last_stamps_of_the_run`: with N = -1 word the largest `abs(1 - x)` in the window is 12566 words above 1 (the mutant reads 12567) |
| 548 | `t + w >= k` to `>` in `ring_fixed` | test gap, group C | the same test: with N = +1 word the largest `1 - x` is 12440 words below 1 (the mutant reads 12441) |
| 550 (drop-step and swap-arith) | the analytic stamp, as 524 | test gap, group C | the same test: r = -0.5, `analytic / abs(1 - an_v(r, k dt))` is 1 to 1e-9 |

Counts: 2 equivalent (176, 433); 28 test gaps (group A 2, B 2, C 8, D 15, E 1); 0 dead lines; 0 undecided; 0 unviable.
The accepted file holds 29 active entries: the 2 equivalent survivors and the 27 hung drop-step mutants, each with a one-line
reason read off its function. Group D has 15 further lines in the accepted file, all commented out ("pending: #12276"):
see "Group D and the trap" below.

## The 7 tests added (appended after the last original test, so no mutant line moved)

| Test | Runtime asserts counted | Group |
| --- | --- | --- |
| `near_holds_at_its_exact_tolerance` | 3 | E |
| `a_one_step_run_has_exactly_one_grid_point` | 7 | B |
| `the_exact_grids_are_the_grids_of_their_backward_euler_twins` | 8 | A |
| `the_f64_ring_window_is_the_last_stamps_of_the_run` | 12 | C |
| `the_fixed_point_ring_window_is_the_last_stamps_of_the_run` | 6 | C |
| `the_five_state_exponential_and_hold_stay_inside_their_arrays` | 17 | D |
| `the_two_state_circuit_in_five_states_gives_the_two_state_runs` | 9 | D |

`tri test specs/analog/linear_zoh.t27`: 22 tests (15 existing, 7 new), 22 pass, 0 fail, `vacuous passes 0 of 22`. The
7 new tests add 62 counted runtime asserts. No existing line was edited (the diff of the spec is 218 added lines, 0 removed).
Every module-constant table element used as an assert operand is first read into a `var` (#12166); the one place the new
tests read `ZTK_D` and `ZTK_NV` is an assignment into a local array, not an assert.

## Values: the first run was the check

Every expected value was hand arithmetic from the function bodies (the issue says so, and the ring_fixed rows were marked
"needs a lab value"). The first `tri test` run on the unmodified spec passed all 22 tests, so no hand value had to be
corrected, and no value was taken from a mutant. Two points the run settled:

- The `ring_fixed` pins (12440 and 12566 words) were derived by hand rather than taken from a lab run: with D = 0 and a
  one-word N the integer step gives x_t = +t words (or -t words for N = NEG | 1), so 1 - x falls by one word per step and
  the largest value in the window sits on the first stamp 12440 (the second on the last, 12566). The run confirms them;
  the command is the `tri test` line above.
- Equality asserts on f64 (`==` on the `scale` and `err` of two runs, and on the 5-state against the 2-state runs) held
  bit for bit, so no tolerance was needed. The loose bounds (`want1 > 3.0`, `0.2 < want2 < 0.3`, `4.5 < want3 < 5.0`,
  `g5.scale > 1.1`, `p5.analytic > 0.2111`, `scheme / analytic > 1.0002`) only guard against a degenerate run; they pin
  nothing the mutants depend on.

## Hand check of the kills

Each target mutant was read against its assert (not run; the gate is the orchestrator's). The decisive numbers:
a = 1.0001 gives a mutant peak a^(k+1), 1.0001 times the pin; a = 0.9999 gives a^(k-w+2), 0.9999 times the pin; the
growing tank at r = -0.5 peaks within half a step of stamp 12566 (0.41 steps before the continuous extremum at 200 half
damped periods) and stamp k-1 or k-2 is 1.41 or 2.41 steps off, which costs about 2e-3 in relative terms against a
tolerance of 1e-9; one extra grid point doubles the one-step RLC scale and moves the RC scale from 0.0488 to 0.0952.

## Group D and the trap

Group D needs the build to trap an index one past the end of an array. At census time that was inferred only (from the
absence of mutant 256 in the survivor and hung lists). Two further pieces of evidence now exist, neither of which is a gate run:

- A scratch spec (never committed; a slice parameter over a 5-word array, a loop with `i <= n`, plus a control loop with
  `i < n`) was run with `tri test` in the pane at 12:26 +07, exit 1: the control test passed and the one-past-the-end test
  FAILED at run time. So the test build traps this.
- `tri mutate spec` compiles each mutant with the same `zig test` line as the test build (no optimization flag, read in
  `cli/tri/src/mutate.rs`), and neither `bootstrap/src` nor `cli/tri/src` contains `setRuntimeSafety`, `ReleaseFast` or
  `ReleaseSmall`. So the mutants should trap the same way.

The gate run (gate 12276c, above) confirmed it: no group D mutant is among the 2 survivors or the 27 hung, so all 15
are killed. The commented lines in the accepted file for them stay comments (an accepted line whose mutant is killed
fails the gate, `specs/tri/mutate/survivors.t27`).

Mutant 177 is a second case of the same kind as 452 and 468 below: `nrm * 2.0` drives nrm to infinity, the halving loop
then runs until the u32 counter overflows, and whether that ends before the clock depends on load. The first gate run
(timeout 30, high load) listed it as hung; gate 12276c (timeout 45, load 18) killed it. That is a reading of the source
plus two verdicts, not a trace of the overflow.

## Caveats of the classification

- 176 is equivalent in exact arithmetic and equal up to rounding in f64 (a tie takes one more halving and one more
  squaring); a test that pins the bits of a non-dyadic result could tell the two apart. The nilpotent test matrix is
  dyadic, so there the result is bit-identical. 433 differs only when a equals b, and then both are the same number.
- The ring extrema (at 200 half damped periods, 100 periods = 12565.5 to 12566.5 steps from `ring_k` = 12566) are hand
  arithmetic. The growing-ring pin passed at 1e-9, so the window maximum for r = -0.5 is on stamp 12566. Which of stamps
  12440 and 12441 holds the maximum of the decaying tank is not decided here, and 522 and 548 are killed either way,
  because the one-state systems put the maximum on a stamp that is known.
- 0.0488 is now pinned by a passing assert (1e-12 against `1.0 - exp(-0.05)`). The mutant's values (0.0952 for the RC; for
  group A the longer horizons, 1.0 against 1 - e^-10 for the RC) are from e^-0.1 and e^-10 by hand and unrun.
- Mutants 452 and 468 (`1 << j` to `1 >> j` in `rlc_ex_err` and `rc_ex_err`): between the pre-audit and the post-audit
  runs they moved from hung to killed (29 hung became 27, 193 killed became 195), while the code of both functions
  and of everything they call is identical in the two trees and the test that reaches them is not in the diff. The cause is
  not established; the runs differ in machine load at the start (load averages 13.13, 20.07, 19.87 before; 7.43, 8.60,
  12.03 after). A candidate reading, not run: at j = 1 the mutant makes m = 0 and dt infinite, and the halving loop of
  `pade_expm` then ends only by overflow of its u32 counter if the build traps integer overflow, which arrives in a time
  that depends on load against `--timeout`; that depends on overflow trapping, which is inferred, not established. They
  were never survivors, so no verdict is touched. They are not in the accepted file; add them as hung only if the gate
  run names them as hung.

## Not done

- The full gate was not repeated after line 177 left the accepted file (see "Gate" above); mutants 452 and 468 were not
  named by the gate as hung or survived, so they stay out of the accepted file as the caveat above says.
- The seal of `AnalogLinearZoh` is re-saved with the tests; it is checked again by `tri seals changed` after the rebase on
  master, before the push.
- `tri coverage 'specs/analog/*.t27' --strict` was not run here; CI runs it (SC-007).
