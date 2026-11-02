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

## What has and has not been run

Run locally, on the binaries named in the receipt's snapshot table:
`t27c parse` and `t27b asm` on each spec, every one rc 0. Those are the only two
commands the local hook allows, and they are parse and lowering, not execution.

**No test in these specs has been executed anywhere.** That is what this PR is
for. Two jobs here do execute them:

- `spec-guards` (`.github/workflows/spec-guards.yml`), step "Affected specs --
  select with specs/ci/affected.t27, test only those": runs
  `t27c test-report <spec> --specs-dir specs` on each changed spec against zig
  0.16.0 on `ubuntu-24.04`, and prints `<file>: head H, base B
  (failing tests)`. For a new file the base is 0, so any failing test turns the
  step red and names the file.
- `t27b-native-ratchet`, the third job of `.github/workflows/t27b-native.yml`
  (the other two are `t27b-native-linux` and `t27b-native-macos`, which run
  `cargo test -p t27b`, not these specs), on `ubuntu-24.04-arm`:
  `t27b corpus specs --json` with
  `--reference ./target/release/t27c`, so every spec runs on both the t27b JIT
  and the t27c + zig reference. Per-file counts land in the `t27b-native-run`
  artifact (`run.json`: `tests`, `invariants`, `asserts`, `blockers` for each
  engine). It gives counts, not per-test names, and no per-file exit code.

Which engine runs which file matters for reading the result. The ratchet job
runs every spec twice, but `t27b corpus` reports `codegen_audit.t27` as
`blocked` on the native engine, so **its 13 tests are executed by the reference
path and not by the t27b JIT**: the verdict to read for that file is
`spec-guards`' `t27c test-report`, and the ratchet's native column for it is a
blocker count, not a pass count. `l1_clamp.t27`, `ifconv_plan.t27` and
`acceptance_matrix.t27` have no such report yet, because they have never been
run anywhere.

Ledger rows, stated as measured rather than predicted. Five of the specs this
branch touches have no row in `docs/reports/t27b_expectations.json`: the three
new ones, plus `codegen_audit.t27` and `specs/analog/reference_receipt.t27`,
which already had no row before this branch existed. And master's
`t27b-native-ratchet` is **already red** without this branch: run
`41401435133` on head `783f18b4f8` (2026-11-02T06:40:46Z) reports
`UNLISTED 42` with every other finding kind at 0 -- no unexpected failure, no
unexpected pass, no stale row, nothing over cap -- and `codegen_audit.t27` and
`reference_receipt.t27` are two of those 42. So this PR does not turn that job
red; it adds 3 names to a list of 42. The workflow's own header states the rule
that follows from it: the ratchet is not required, and `ready` judges it by Q16,
where a red here blocks only while master's run of it is green. It is not.

The rows are still deliberately absent, for the reason that has not changed: a
row written before the run is a guess at the verdict, and the ratchet exists to
catch exactly that. They are written from the observed values once a run
reports. What that does **not** buy, and the earlier draft of this paragraph
claimed it did, is a green ratchet -- adding three rows leaves master's other 39
unlisted specs exactly where they are.

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
