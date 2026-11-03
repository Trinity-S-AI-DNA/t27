# Receipt: `codegen_audit.t27` on the Zig path, 16 of 16 (2026-11-02)

The owner ran these two commands in their own terminal, from worktree
`loving-kilby-0e85a7`. The output reached the agent session a few turns late,
so the time of each run was not recorded. Which version of the spec ran is
known from the test names (see "Which version").

| binary | version | sha256 (first 16) |
|---|---|---|
| `~/t27/target/release/t27c` | t27c 0.5.2 (build 0, commit unknown), mtime 2026-11-01T17:40 | 041fb69cd5fe45b2 |
| `~/t27/target/release/t27b` | t27b 0.1.0, mtime 2026-11-01T20:19 | ee89c7d27724dc11 |

The zig binary that `t27c test-report` called was not recorded. The Mac hook
forbids the agent to run `zig`.

## Run 1: t27b refuses

```
~/t27/target/release/t27b test specs/tri/shootout/codegen_audit.t27
t27b: unsupported construct ExprArrayLiteral at line 387 (array literal with no result type)
t27b: unsupported construct ExprArrayLiteral at line 426 (array literal with no result type)
t27b: unsupported construct ExprArrayLiteral at line 436 (array literal with no result type)
```

No test executed. The cause was `pub const LOSSES = [ ... ]`, an array declared
without a type. Commit 73f626bae2 changed the declaration to
`pub const LOSSES : [5]Loss = [ ... ]`. On HEAD, `t27b asm` lowers the spec with
rc 0 (1930 asm lines, all 16 `test` blocks). That is lowering, not execution.

## Run 2: the Zig path passes

```
~/t27/target/release/t27c test-report specs/tri/shootout/codegen_audit.t27
  tests 16 / pass 16 / FAIL 0 / rate 100.0%
  vacuous passes 0 of 16 (2 comptime-verified)
```

| runtime asserts | test |
|---|---|
| 1 | census_partitions_the_loop_exactly |
| 0 (CT-VERIFIED) | loop_has_no_memory_traffic |
| 0 (CT-VERIFIED) | overflow_checks_are_a_branch_each |
| 2 | call_is_not_free |
| 2 | largest_loss_is_control_flow_not_the_call |
| 3 | recoverable_never_exceeds_what_is_there |
| 2 | measured_ipc_is_about_two_and_a_half |
| 2 | calls_per_second_matches_the_receipt |
| 2 | planned_six_instructions_beats_the_zig_rung |
| 3 | one_hundred_fifty_ms_needs_six_instructions_or_fewer |
| 1 | evidence_is_a_command_not_an_opinion |
| 4 | today_runs_within_ten_percent_of_its_own_floor |
| 5 | the_150_ms_rung_needs_unrolling_or_inlining |
| 4 | inlining_moves_the_floor_below_one_hundred_ms |
| 8 | closed_form_matches_the_loop_at_small_n |
| 2 | the_200m_answer_is_whether_tick_equals_prev |

## Which version

Tests 12 to 14 carry their old names. Commit 73f626bae2 renamed them to
`model_puts_today_within_ten_percent_of_its_bound`,
`model_says_the_150_ms_rung_needs_unrolling_or_inlining` and
`model_puts_the_inlined_bound_below_one_hundred_ms`. So both runs used the file
as it was before 73f626bae2:

| commit | blob | sha256 of the file (first 16) |
|---|---|---|
| 5e7ff40da8 = 75039dbf07 (same file) | 4edb38dfbcb898a99595e1d9f9f5ea82245df183 | 59881a37ea73097d |
| HEAD (73f626bae2 onward) | 9546cc2c0f01ef2303ded31ed7279770ea015f4f | not run |

## What this shows, and only this

- The 16 tests of that version pass on the Zig path, with no vacuous pass.
- t27b refused that version before running anything.
- These tests check the arithmetic of a model: an instruction census and a
  branch-bound floor. They do not measure the compiler. "Within ten percent of
  its own floor" holds inside the model and is not a measured limit.

## Not covered

- HEAD's version, which differs by 32 added and 24 removed lines, on either path.
- t27b executing any test of this spec.
- `specs/tri/shootout/acceptance_matrix.t27`, which has not run anywhere.

Refs #12877.
