Loss L1 of `specs/tri/shootout/codegen_audit.t27` -- the conditional in the 200M
heartbeat loop -- written down to the emitted byte, with the conditions a
branchless form must satisfy, and the rule for reporting a noisy measurement
row. **No Rust file is touched, and this PR claims no measurement.** It is
opened so that a runner that is allowed to execute tests does execute them.

## What is here

| file | what it is |
|---|---|
| `specs/tri/shootout/l1_clamp.t27` (new) | the saturating wrap on its own, 3 functions, 5 tests pinning the edges of the wrap: 1, the limit, the limit + 1, the largest u64. The regression gate: the same values must hold after the transform |
| `specs/tri/t27b/ifconv_plan.t27` (new) | when an `if` may become a select, and in what order the emitter may write it: 17 functions, 19 tests. Nothing in `cli/t27b` calls it yet |
| `specs/tri/shootout/acceptance_matrix.t27` (new) | the acceptance axes, with the one-line spread rule written out as three functions and three tests (below). The file is new on this branch; master has no `acceptance_matrix.t27` |
| `.trinity/receipts/t27b-l1-clamp-2026-11-02/README.md` | the snapshot, the emitted assembly today, the hand-derived after-shape, the numeric edges, the prepared-not-applied Rust glue, and a correction owed to `codegen_audit.t27` |

## The correctness rule for if-conversion

Speculating an arm is sound exactly when its trap range is disjoint from the
guard interval of the side where it stopped running. `x - k` traps on
`[0, k-1]`, `x + k` on `[2^64 - k, max]`, and `k == 0` traps nowhere; each
comparison's guard is one interval, which is why `!=` is refused -- its true set
is two. The benchmark's clamp passes, because `t - 1` traps only at `t == 0`
and the arm it stopped running on is guarded by `[1001, max]`.

This is a rule over kinds and immediates, not a pattern match on this
benchmark. The negative controls are in the spec: a subtraction and an addition
whose trap falls inside the other side's guard are both refused, as are memory
arms, control-flow arms, non-integer values, a compound condition, and anything
over the select budget.

## A second condition, found while reading the emitted bytes

Soundness is not enough: the order the emitter writes the shape in is also part
of correctness, and it is not implied by anything above. t27b lowers the checked
subtraction to `subs x2, x1, #1` + `b.lo` and the guard to `cmp x1, #1000`.
Both write NZCV, and CSEL reads NZCV. So a select emitted after a speculated
`subs` chooses on the subtraction's flags: it takes the other arm, and nothing
traps. A silently wrong answer, which the trap-disjointness rule cannot catch.

`arm_clobbers_flags`, `clobbering_arms` and `emit_order_ok` make it checkable --
every flag-clobbering speculated arm before the guard compare, so that compare
is the last writer of NZCV before the selects -- with the wrong order as the
negative control.

The same reading corrects a register claim. The conversion needs **one** scratch
register, not two: `csel` takes a register number for its first source and 31 is
ZR, so `tick = 0` needs no `movz` and no register, and only the `1000` is
materialised. `cli/t27b/src/codegen.rs` lines 10-15 keep x8 as the operation
scratch and x16/x17 for materialised constants, so that one constant has a home
no arm's value is living in. `scratch_for_const` records it.

## The noisy-row rule

`acceptance_matrix.t27` used to say a noisy row "is re-run, never averaged in".
That permits re-running until the number looks quiet and then reporting the
quiet series. Replaced by `spread_status`, `may_run_extra_series` and
`union_spread_ms`: every run is kept; a row whose spread exceeds
`SPREAD_LIMIT_PCT` (5.0) of its own median is UNSTABLE and its conditions are
recorded; at most `EXTRA_SERIES_MAX` (1) further series, fixed in advance, may
be added and is reported beside the first; and two series are read as their
union, which can never be smaller than either.

The AOT receipt's own five runs are the test case:
`[341.655, 338.016, 349.013, 344.084, 324.901]`, median 341.655 ms, spread
24.112 ms = 7.06 percent. That row is therefore UNSTABLE. Adding an imagined
quieter second series leaves the union spread at 24.112, which is the arithmetic
that forbids reporting the quiet one alone.

## What has been run, measured

`t27b-native-ratchet` (run `41412031594`, job `125167018191`, head
`389d6e26f6`, `ubuntu-24.04-arm`) finished **success**. Reference verdicts from
`t27c test-report` built in that job against zig 0.16.0, t27b verdicts from the
crate built in the same job, read out of the `t27b-native-run` artifact:

| spec | reference | t27b | tests | asserts | invariants | blockers |
|---|---|---|---|---|---|---|
| `specs/tri/shootout/codegen_audit.t27` | pass | pass | 16 | 53 | 0 | none |
| `specs/tri/shootout/acceptance_matrix.t27` | pass | pass | 16 | 94 | 0 | none |
| `specs/tri/shootout/l1_clamp.t27` | pass | pass | 5 | 19 | 0 | none |
| `specs/tri/t27b/ifconv_plan.t27` | pass | pass | 19 | 81 | 0 | none |

56 tests and 247 runtime asserts, on both engines, with no named verdict other
than `true` in any of the four files. The ratchet's own line:

```
tri t27b ratchet: green -- run 389d6e26f vs ledger 4125e2902: UNEXPECTED
FAILURE 0, UNEXPECTED PASS 0, UNLISTED 45, STALE 0, OVER CAP 0, BAD REASON 0,
MOVED 0, UNJUDGED 0, VACUITY MEASURED 0

t27b 2502 / reference 2647 native arm64 (+4 pass_vacuous, 5 outside the
reference; 2938 files; jit/interp mismatch 1; reference disagree 0)
```

Two claims of an earlier draft of this section are corrected by that run, not
edited away. **`codegen_audit.t27` is not blocked on the native engine** -- the
draft said its tests ran only on the reference path; measured, t27b passes it
with 16 tests, and the draft's claim carried no command. And **`spec-guards`
executes none of these specs**: its selector printed `RUN_ALL: a change outside
specs/; corpus-ratchet runs the corpus` (the `.trinity/receipts/**` files of this
PR are that change), and `grep -c 'shootout\|ifconv_plan'` over its full log
returns 0. One job executed these files, not two.

Also measured and worth stating against this PR rather than for it. The ratchet
is green here only because the workflow passes `--pr` on a pull request, which
the parser documents as *"a pull request: lag is pending, a runtime defect stays
red"* (`scripts/tri_loop/t27b.py:1020`). On a push to master the same lag is red,
and master's run of this job is red today at `UNLISTED 42` for exactly that
reason. **Landing this PR takes that count from 42 to 45 and fixes none of it.**
The three new specs now have observed values and the rows belong in
`docs/reports/t27b_expectations.json`, written from the observed values by the
ledger steward from master's next run; no row is guessed here, and
`max_not_pass` does not rise for a passing row.

The other three red checks fail on master at byte-identical step names -- the
stale `policy_PolicyOwnLanguage.json` seal (spec-guards `41401435163`, coverage
`41403284547`), and `ledger: every non-pass entry has a blocker and a known
reason` (loop-tools `41172959400`). Each is another actor's open work and none is
touched here. The one finding that *is* this branch's is the type-name ratchet:
master observes 142 conflicted names with `+ ParsedCommand`, this PR observed 143
with `+ Cell` as well, and `Cell` is the name `acceptance_matrix.t27` added. It is
renamed to `AcceptanceCell` in this PR -- 17 tokens, one file -- after which the
check returns to master's state. `CELLS` stays: it is a const, not a type, and two
`const CELLS` already coexist on master without a finding.

## Also recorded, so it is not re-derived

`codegen_audit.t27`'s `L1_RECOVERABLE = 4` is optimistic by 2 to 3 instructions:
the hand-derived branchless shape still needs the `cmp`, the trap check and one
`movz`, leaving 6 instructions (5 once the loop-invariant `movz` is hoisted), not
3. The receipt records the correction and why it is not applied in this PR --
changing that constant moves the model's other rows, and those are what the
existing 16-of-16 receipt is bound to.

The inline row is not a prerequisite of L1. On `origin/epic2-inline-decision-glue`
(head `966ff7e1f7`, PR closed unmerged) the glue builds the facts, calls
`should_inline_call` and then returns `false`, so the decision is computed and
discarded; its call-site field is `inside_loop` where master's `inline.t27` says
`hot_loop`; and the branch also deletes two unrelated files. What is missing
there is an executor, not glue. Nothing of it is restored here.

Refs #12877.

🤖 Generated with [Claude Code](https://claude.com/claude-code)
