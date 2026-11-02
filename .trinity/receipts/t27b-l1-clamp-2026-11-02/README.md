# Receipt: the L1 clamp, site and prepared step (2026-11-02)

Loss L1 of `specs/tri/shootout/codegen_audit.t27` is the conditional in the 200M
heartbeat loop. This receipt records where it comes from, the assembly as t27b
emits it today, the transform and its correctness conditions, and the Rust glue
the transform would need. **Nothing here is a measurement of a change, and no
Rust file in this branch is touched.**

## Snapshot every line below is bound to

| what | value |
|---|---|
| worktree | `~/t27/.claude/worktrees/loving-kilby-0e85a7` |
| branch | `lane/t27b-floor-model` |
| HEAD when this receipt was written | `75fdb1f1fe859a06d7fc24414e8133d518d098f3` |
| `specs/tri/shootout/codegen_audit.t27` | sha256 `d66e075836097d8d...`, blob `9546cc2c0f01ef23` |
| `specs/tri/shootout/acceptance_matrix.t27` at that HEAD | sha256 `7a17bdec33728434...`, blob `64d5333dc58b530b` |
| `specs/tri/shootout/acceptance_matrix.t27` after this round's edit | sha256 `cf89bee8c554189d...`, 16 tests, uncommitted when written |
| `specs/tri/shootout/l1_clamp.t27` (new) | sha256 `96dc940bd28a8fe9...` |
| `specs/tri/t27b/ifconv_plan.t27` (new) as sections 1-8 read it | sha256 `8e4b07b8b1d0b860...`, 13 functions, 15 tests |
| `specs/tri/t27b/ifconv_plan.t27` after section 9's additions | sha256 `0e690948987d82d5...`, 17 functions, 19 tests |

Second snapshot, for the measured run in section 12 and the repair that follows
it. The run's numbers are bound to the first row here, not to the row above.

| what | value |
|---|---|
| HEAD the arm64 run executed | `389d6e26f61ddb63ab0dfd86cf1cae5527b18755` |
| `acceptance_matrix.t27` at that HEAD | sha256 `cf89bee8c554189d...`, 16 tests, the `Cell` name still in it |
| `acceptance_matrix.t27` after the `AcceptanceCell` rename | sha256 `a927eeaa843dc5a7...`, blob `36b6b2fd6cf7ca60`, 16 tests |
| HEAD after this round | `65c0819e13fd23a927f4627baed5a26df232727e` |
| base (merge-base with master) | `8ca1b0817e99818ef35354da1b270c001e865232` |
| `origin/master` when the reds were classified | `0215bd7223c646ceb57d1a468b16a263018aba3c` |

The rename is after the run, so section 12's table describes the `cf89bee8`
file. Nothing re-measures it here: a run of the renamed file does not exist yet,
and the only claim made about it is the one the type-name check itself makes.

Binaries used for every command in this receipt:

| binary | version | sha256 (first 16) | size | mtime |
|---|---|---|---|---|
| `~/t27/target/release/t27c` | t27c 0.5.2 (build 0, commit unknown) | `041fb69cd5fe45b2` | 10639984 B | 2026-11-01T17:40:05 |
| `~/t27/target/release/t27b` | t27b 0.1.0 | `ee89c7d27724dc11` | 3226800 B | 2026-11-01T20:19:20 |

Provenance of those binaries, as far as it is actually known: at the t27b
mtime the main clone's HEAD was `01f868a34a` on `spec/mycelium-zoom` (checked
out 20:11:48 +0700, next reflog entry 20:48:34). The build inputs were not
recorded, so this is the surrounding evidence and not a claim that the binary
was built from that commit. The main clone is now on `spec/integration`. The
worktree also holds a *different* `target/release/t27b` (sha256
`2a13b5e2128a716e`, built 2026-11-02 13:06 by `make t27os`); it was not used
here. An unchanged `cli/t27b` does not make either binary identical to one
built from master.

## 1. The site, exactly

`specs/tri/shootout/bench_audit.t27` lines 21-27, inside
`verdict_benchmark_200m`'s `while (i < CALLS_TARGET)`:

```
cur_tick = cur_tick + 1;
if (cur_tick > 1000) { cur_tick = 0; cur_prev = 1000; } else { cur_prev = cur_tick - 1; }
```

The branches come from two places, and both are in t27b, not in the spec:

| file:line | what it does |
|---|---|
| `cli/t27b/src/lower.rs:1923` | `NodeKind::StmtIf` -> `Stmt::If { cond, then, els }` |
| `cli/t27b/src/codegen.rs:694` | `Stmt::If` -> `branch(cond, l_else, false)`, the then arm, `jump(l_end)`, the else arm |
| `cli/t27b/src/codegen.rs:1315` | `ExprKind::Select` (an `if` used as a value) -> the same branch + jump, per arm |
| `cli/t27b/src/codegen.rs:1458` | `fn branch` -> `compare` then `bcond`: a conditional branch for every integer compare |

So rewriting the benchmark's `if` as a value expression would not remove the
branch: `Select` branches too. That is also why this is not a
benchmark-specific substitution -- the change is in the one place both shapes
pass through.

`specs/tri/t27b/a64.t27:632` already encodes `csel`, and it is generated into
`gen/rust/tri/t27b/a64.rs` (included by `#[path]` at `cli/t27b/src/a64.rs:14`).
`grep 'a64::csel' cli/t27b/src/codegen.rs` returns nothing: **no path in t27b
emits CSEL today.**

## 2. The minimal example and its assembly

`specs/tri/shootout/l1_clamp.t27` is that conditional with nothing around it:
`wrap_tick` (one arm), `wrap_prev` (two arms, one assignment each) and
`wrap_pair` (the benchmark's shape: two assignments in one arm). Its five tests
pin the edges 0, 1, 999, 1000, 1001, 1002 and u64 max, and they are the
regression gate: the same values must come out after the transform.

Commands, from the worktree root:

```
~/t27/target/release/t27c parse specs/tri/shootout/l1_clamp.t27     # rc 0
~/t27/target/release/t27b asm  specs/tri/shootout/l1_clamp.t27      # rc 0, 217 lines
```

The full output is `l1_clamp.before.asm` in this directory (sha256
`92713df74a025198...`). `wrap_pair`, which is the benchmark's shape:

```
_wrap_pair:  ; line 45
      50: aa0003e1  mov x1, x0
      54: d2800002  movz x2, #0x0
      58: f10fa03f  cmp x1, #1000        <-- the L1 chain starts
      5c: 54000089  b.ls 0x6c
      60: d2800001  movz x1, #0x0
      64: d2807d02  movz x2, #0x3e8
      68: 14000003  b 0x74
      6c: f1000422  subs x2, x1, #1
      70: 54000103  b.lo 0x90            <-- the L1 chain ends
      74: d2820011  movz x17, #0x1000
      78: 9bd17c28  umulh x8, x1, x17
      7c: 9b117c29  mul x9, x1, x17
      80: b5000088  cbnz x8, 0x90
      84: ab020120  adds x0, x9, x2
      88: 54000042  b.hs 0x90
      8c: d65f03c0  ret
      90: d4200020  brk #0x1
```

`0x58..0x70` is seven instructions and reproduces the loop's chain exactly:
`cmp / b.ls / movz / movz / b / subs / b.lo`. That is `L1_INSTRUCTIONS = 7` of
`codegen_audit.t27:138`, now visible as emitted bytes rather than as a count in
a spec. `0x74..0x8c` is the test harness's packing, not part of L1.

`t27b asm` resolves names (see `.trinity/receipts/typecheck-unknown-name-2026-11-02/`),
so rc 0 means every name in the spec exists. It is lowering, not execution: no
test in `l1_clamp.t27` has been run anywhere.

## 3. The transform

If-conversion of an `if`/`Select` whose arms are speculable: evaluate both arms
unconditionally into registers and choose with `csel`.

Hand-derived expectation for `wrap_pair`. **This is written by hand. No binary
emitted it, and no `t27b` in this repository can emit it today:**

```
      mov  x1, x0
      subs x9, x1, #1       ; the else arm, speculated; subs because the trap check reads its flags
      b.lo <trap>           ; fires only at x1 == 0 -- not in the then guard [1001, max]
      cmp  x1, #1000
      movz x10, #0x3e8      ; loop-invariant in the benchmark, hoistable out of the loop
      csel x2, x10, x9, hi  ; prev = (x1 > 1000) ? 1000 : x1 - 1
      csel x1, xzr, x1, hi  ; tick = (x1 > 1000) ? 0 : x1
```

The order is forced: `subs` sets the flags the trap check needs, so it has to
precede the `cmp` that the two `csel`s read.

| | chain length | branches in the chain | taken on the common path | instructions executed on the common path |
|---|---|---|---|---|
| today (emitted) | 7 | 2 | 1 | 4 |
| after (hand-derived) | 6 | 1 | 0 | 6 |
| after, with the `movz` hoisted | 5 | 1 | 0 | 5 |

"Common path" is `tick <= 1000`, which is 1000 of every 1001 iterations.

**Consequence inside the floor model of `codegen_audit.t27`, and only inside
it.** The model is `bound = max(taken / 1, branches / 2, instructions / 8) / unroll`
over `SHAPE_TODAY = 22 instructions / 9 branches / 5 taken` per call. Removing
this one taken branch gives 21 / 8 / 4, so
`max(4, 4, 2.625) = 4` cycles per call instead of 5: the model bound moves from
312.5 ms to 250 ms for 200M calls. The AOT median on the receipt is 344.08 ms.
**Both numbers are the model and the old receipt; this change has not been
built, run or timed. The first measured result comes later, on one allowed
machine, with the same inputs.**

### Correction owed to `codegen_audit.t27`

`codegen_audit.t27:139` says `L1_RECOVERABLE = 4  // sub + two csel replaces it;
one residual move stays`, i.e. 3 instructions left of 7. The shape above leaves
**6** (5 with the hoist), because the audit's count omits the `cmp`, the trap
check `b.lo`, and the `movz` of the constant 1000. So `L1_RECOVERABLE` is
optimistic by 2-3 instructions and must not be used as if it were derived.
Correcting it moves the model's other rows, so it belongs in its own change
with its own tests, not in this receipt. Recorded here so the number is not
quoted again as sound.

## 4. Correctness conditions, and where they are written down

`specs/tri/t27b/ifconv_plan.t27` (module `T27bIfconvPlan`) is the decision, in
t27, with 19 tests (15 when section 4 was first written, before section 9 added
the emission-order rule). `t27c parse` rc 0; `t27b asm` rc 0. The line count and
hash of that lowering live in section 6c's table, with the spec hash that
produced them, and are not repeated here. Not executed anywhere. Nothing in
`cli/t27b` calls it.

1. **Integers in registers only.** No float, no i128 (`all_int`).
2. **The arms touch nothing but registers.** `arm_speculable` accepts an absent
   arm, a constant, another local, `x - k` and `x + k`; it refuses a call, a
   load, a store, and everything else (`break`, `continue`, `return`, a nested
   `if`, a non-scalar).
3. **One compare.** `cond_is_simple`; an `and`/`or`/`!` chain is refused, because
   `fn branch` turns those into several branches with their own labels.
4. **Trap disjointness.** t27 arithmetic is checked, so a speculated `x - k` or
   `x + k` traps on part of the range. It may run unconditionally only when its
   trap range is disjoint from the guard interval of the side on which it
   *stopped* running:
   - trap ranges: `x - k` traps on `[0, k-1]`; `x + k` traps on `[2^64-k, max]`;
     `k == 0` traps nowhere.
   - guard intervals: `> c` is `[c+1, max]` (empty at `c == max`), `>= c` is
     `[c, max]`, `< c` is `[0, c-1]` (empty at `c == 0`), `<= c` is `[0, c]`,
     `== c` is `[c, c]`.
   - `!=` is refused: its true set is two intervals, and this step keeps one
     interval per side.
   - the clamp: trap `{0}` against the then guard `[1001, max]` -- disjoint, so
     the subtraction may be speculated.
5. **Budget**: at most 2 selects and at most 2 speculated ops. **These two
   numbers are chosen, not measured**; they hold the conversion to the shape the
   clamp needs until a measurement says more pays.

Negative controls in the spec (each refused, each as its own test):

| case | why it is refused |
|---|---|
| `if (t < 5) { p = 7; } else { p = t - 10; }` | `t - 10` traps on `[0,9]`, the else arm stopped running on `[0,4]`: `t = 3` returned 7 and would now trap |
| `if (t > 0) { p = 0; } else { p = t - 2; }` | trap `[0,1]` meets the then guard `[1, max]` at 1 |
| `if (t < 1000) { p = t + 1; } else { p = 0; }` | the then arm overflows at max, which is on the else side |
| an arm that calls (`verdict_fn`), loads, stores, or is anything else | not speculable |
| a non-integer value | `all_int` false |
| a compound condition | `cond_is_simple` false |
| `!=` as the condition | not a single interval |
| 3 selects, or 3 speculated ops | over the chosen budget |

Accepted cases that are not the benchmark, so the rule is not a special case:
the one-armed clamp; `if (t > 5) { p = 0; } else { p = t - 1; }`; `p = t - 0`
against any guard; a guard that is empty (`t > max`, `t < 0`), where the arm
already ran for every input; `== c` with two constant arms; and the mirror
`if (t > 1000) { p = t + 1; } else { p = 0; }`.

## 5. Numeric edges, including the one an assert cannot state

| input `t` | `wrap_tick` | `wrap_prev` | `wrap_pair` | after the transform |
|---|---|---|---|---|
| 0 | 0 | **traps** (`0 - 1`, checked) | **traps** | must still trap: the speculated `subs` keeps its `b.lo` |
| 1 | 1 | 0 | 4096 | unchanged |
| 2 | 2 | 1 | 2 * 4096 + 1 | unchanged |
| 999 | 999 | 998 | 999 * 4096 + 998 | unchanged |
| 1000 | 1000 | 999 | 1000 * 4096 + 999 | unchanged, the last value before the wrap |
| 1001 | 0 | 1000 | 1000 | unchanged, the first value after it |
| 1002 | 0 | 1000 | 1000 | unchanged |
| 18446744073709551615 | 0 | 1000 | 1000 | unchanged |

`wrap_tick` is asserted at every row but the first; `wrap_prev` at 1, 2, 1000,
1001, 1002 and max; `wrap_pair` at 1, 1000, 1001, max and 500. The remaining
cells of the table are read off the two one-line functions, not asserted. The
`t = 0` row is
here because an `assert` cannot state "this traps": the benchmark never reaches
it (`cur_tick` is incremented before the test), and the transform must not turn
it into a silent 2^64-1.

## 6. The Rust glue this needs -- prepared, NOT applied

The decision is in t27; the emitter is not. Three hand-written Rust edits would
be needed, and all three are inside the foreign-code gate:

1. `cli/t27b/src/codegen.rs`, in `Stmt::If` (line 694) and `ExprKind::Select`
   (line 1315): read each arm's kind and immediate, read the condition's
   comparison and constant, count the selects and speculated ops, ask the
   generated `may_if_convert`, and on `true` emit `a64::csel` per result
   instead of `branch` + `jump`. On `false` keep today's path byte for byte.
   Two constraints on that emission, both from section 9 and neither optional:
   the order is fixed -- every flag-clobbering speculated arm goes out **before**
   the guard compare, so that compare is the last writer of NZCV before the
   selects (`emit_order_ok`), and getting it wrong is a silently wrong answer,
   not a slower one; and a zero arm is `xzr` rather than a materialised
   constant, so the clamp needs one scratch register, not one per arm
   (`scratch_for_const`).
2. `cli/t27b/src/codegen.rs` or a new module: a `#[path = "../../../gen/rust/tri/t27b/ifconv_plan.rs"]`
   include, in the same shape as `cli/t27b/src/a64.rs:14` uses for `a64.rs`.
3. `cli/t27b/src/a64.rs`, the disassembler at line 209: it matches conditional
   select only as `w & 0x1FE0_0C00 == 0x1A80_0400`, which is the csinc/cset
   form. A real `csel` word (op2 = 00, `0x1A80_0000`) falls through to
   `.word 0x...`. Without about three more lines there, **the before/after
   assembly comparison that this transform has to be judged by cannot be read**.

Budget and gate, stated plainly rather than worked around:

- the own-language gate allows at most 40 added hand-written foreign lines per
  file and 80 per PR, **and** requires added `<=` deleted overall. The three
  edits add lines to two files, so each added line has to be paid for with a
  deletion in the same PR; that payment has not been found yet.
- editing Rust also needs the owner's `owner-approved-foreign` label on the PR.
  An agent never adds that label, and it cannot be obtained by changing how the
  diff is presented.

So the diff is described here and **not written into any `.rs` file in this
branch**. `git status` for this branch shows no file under `cli/`.

## 6a. Why this is L1 and not inlining

L1 does not wait for the inline work, and the inline work is not what is nearest.
Read from the closed PR's branch rather than from its description:

| what | state |
|---|---|
| PR #12961 | CLOSED without merge; label `owner-approved-foreign` still attached |
| its branch | `origin/epic2-inline-decision-glue`, head `966ff7e1f7`, unmerged |
| the glue it adds to `cli/t27b/src/codegen.rs` | +37 hand-written lines: a `#[path = "../../../gen/rust/tri/t27b/inline.rs"]` module and `fn try_inline`, which builds the facts, calls `should_inline_call`, and then **returns `false`** |
| contract drift | the glue builds a call site with `inside_loop`; master's `specs/tri/t27b/inline.t27` names that field `hot_loop`, so the two do not line up |
| unrelated deletions on the branch | `.claude/hooks/session-gate.sh` (-127) and `.trinity/check_startupclk.py` (-34) |

So the decision is computed and discarded: the glue has no runtime effect, and
restoring it would move no instruction. What is actually missing is an executor --
substituting a callee's body at the call site -- and that is a larger change than
the branchless clamp. Nothing of #12961 is restored here, and the inline row is
not treated as a prerequisite of L1.

## 6b. The noisy-row rule, now executable

`acceptance_matrix.t27` used to carry one sentence saying a noisy row "is
re-run, never averaged in". That sentence permits the thing that must not
happen: re-run until the number looks good, then report the quiet series. It is
replaced by a rule with three functions and three tests behind it:

| name | what it decides |
|---|---|
| `SPREAD_LIMIT_PCT = 5.0` | the spread, as a percent of the row's own median, above which a row is UNSTABLE |
| `EXTRA_SERIES_MAX = 1` | how many further series of the same size, fixed in advance, an UNSTABLE row may be given |
| `spread_status(spread, median)` | `ST_UNSTABLE` over the limit, and also when there is no median |
| `may_run_extra_series(status, extra_done)` | false for a stable row at any count, false once the allowance is used |
| `union_spread_ms(lo_a, hi_a, lo_b, hi_b)` | the spread of two series read together, which is their union and so never smaller than either |

The AOT receipt's own five runs are the test case:
`[341.655, 338.016, 349.013, 344.084, 324.901]`, median 341.655, spread 24.112 =
7.06 percent. The row is therefore UNSTABLE, reported with that spread; adding
an imagined quieter second series (341.0 to 342.0) leaves `union_spread_ms`
equal to 24.112, which is the arithmetic that forbids reporting the quiet series
alone. No run is dropped and no row is re-run to make it look stable.

Checked with the two commands the Mac hook allows, on the snapshot's binaries:

```
~/t27/target/release/t27c parse specs/tri/shootout/acceptance_matrix.t27    # rc 0
~/t27/target/release/t27b asm   specs/tri/shootout/acceptance_matrix.t27    # rc 0, 12176 asm lines
```

That is parse and lowering. None of the 16 tests has been executed.

## 6c. The two .asm artifacts are deliberately not committed

The full disassembly of both specs was written next to this file, and the
pre-commit gate refused it twice: `hand-written .asm is forbidden` (own-language)
and `more than 40 hand-written foreign lines added in one file`
(foreign-budget). The files are `t27b asm` output rather than hand-written, but
the gate cannot tell, and renaming them to an extension the gate does not check
would be getting past a refusal by changing the form of presentation. So they
stay out of the commit, and the part that carries the argument -- the
`wrap_pair` chain of section 2 -- is quoted in this file instead.

Each row names the sha256 of its **input** as well as its output, because an
`.asm` hash on its own goes stale the moment the spec is edited, and then the
reproduce command silently disagrees with the hash beside it. That happened
once here: `ifconv_plan.asm` was 1157 lines / `447fdb0b309ecc89...` from the
spec as section 4 first described it, and section 9's additions made both
numbers wrong. The input column is what makes that visible instead of silent.

| artifact | lines | sha256 | input spec, sha256 | how to reproduce it byte for byte |
|---|---|---|---|---|
| `l1_clamp.before.asm` | 217 | `92713df74a025198...` | `specs/tri/shootout/l1_clamp.t27`, `96dc940bd28a8fe9...` | `~/t27/target/release/t27b asm specs/tri/shootout/l1_clamp.t27` |
| `ifconv_plan.asm` | 1329 | `0179d4b597ac0f53...` | `specs/tri/t27b/ifconv_plan.t27`, `0e690948987d82d5...` | `~/t27/target/release/t27b asm specs/tri/t27b/ifconv_plan.t27` |

Both with the snapshot's `t27b` (sha256 `ee89c7d27724dc11`). If a committed
disassembly is wanted, it needs a path the gate accepts -- a generator with a
`.t27` source, not a rename.

## 7. What is not covered

- No test in `l1_clamp.t27` or `ifconv_plan.t27` has been executed anywhere.
  `t27c parse` and `t27b asm` are the only commands run, and the Mac hook allows
  only those two.
- No timing. The 250 ms figure is the model's bound, not a measurement, and the
  344.08 ms median is the older AOT receipt.
- The "after" assembly is hand-derived. Until the glue exists and a
  disassembler can print `csel`, there is no emitted after-shape to compare.
- Register allocation: answered in section 9 below. The shape needs one scratch
  register, not the two the hand-written listing in section 3 spends.

## 8. What this round committed, and where the branch stands

| commit | what |
|---|---|
| `cf503a566abd18ec24c1f88a6dfdc646616c7c9b` | the two new specs, the matrix's noisy-row rule, this receipt |
| `df4daec11661d0eeffefbf9a7cf049b0ca02a379` | `pr-body.md`, the text of the publication request |
| `d7a6f7b76e64807bd134cb444116dff76a24da50` | this section |
| `2109e3739a` and later | section 9 and anything after it. A commit cannot carry its own hash, so this table stops here rather than chasing the head; `git log --oneline origin/master..HEAD` is the current list |

`git merge-tree --write-tree HEAD origin/master` returns rc 0 against
`origin/master` = `0215bd7223c646ceb57d1a468b16a263018aba3c`, so GitHub can
squash the branch with no conflict. Every commit on the branch carries
`Refs #12877`, which is what the L1 check greps for.

Nothing is pushed. The branch is published, and the draft PR opened, only on the
owner's confirmation of the exact request recorded in `pr-body.md`.

Refs #12877.

## 9. The register question, answered -- and one scratch register, not two

Section 7 left the allocator unasked. It is answered here from t27b's own
register map and the `csel` encoder, without running anything.

The map is documented at `cli/t27b/src/codegen.rs` lines 10-15: x0-x7 arguments,
the result, and variable homes in leaf functions; **x8 the operation scratch**;
x9-x15 the expression temp stack, spilling to the frame from temp 7; **x16/x17
scratch for spilled temps and materialised constants**; x18 untouched; x19-x28
variable homes by loop-weighted use count. `wrap_pair` is a leaf, so its locals
live in x0-x7 -- which is what the emitted bytes show: x1 is `tick`, x2 is
`prev`, and x8/x9/x17 are already used as scratch by the multiply.

What the conversion actually needs at the selects:

| value | where it comes from | register cost |
|---|---|---|
| `tick = 0` | `csel rd, xzr, x1, hi` | **none** -- `csel` takes a register number for its first source and 31 is ZR (`specs/tri/t27b/a64.t27` `csel`, whose own comment says so). No `movz`, no register |
| `prev = 1000` | one `movz` into a scratch | one register, loop-invariant, hoistable |
| `prev = t - 1` | the speculated `subs` | none -- it may be written straight into `prev`'s home, which is what t27b already does at `0x6c` (`subs x2, x1, #1`) |

So **one** scratch register, and x8 or x16/x17 holds it. The listing in section 3
spends two (x9 for the speculated subtraction and x10 for the constant); x9 is
unnecessary, because `csel rd, rn, rm` is legal with `rd == rm`, and the
destination is written unconditionally in either case, so letting the
speculation land in `prev`'s home clobbers nothing the select does not
immediately overwrite. The instruction counts in section 3's table are
unaffected -- 6 instructions, 5 with the `movz` hoisted -- because dropping x9
removes no instruction, only a register.

### The flag order is now a rule with a negative control

Section 3 already said the order is forced, in prose: `subs` writes the flags
its trap check reads, so it must precede the `cmp` the selects read. This round
makes that checkable rather than asserted, because getting it backwards is not a
slower shape but a **silently wrong** one -- a select emitted after a speculated
`subs` chooses on the subtraction's flags, takes the other arm, and traps
nowhere. Nothing in the trap-disjointness rule catches that.

`specs/tri/t27b/ifconv_plan.t27` gained four functions and four tests:

| name | what it decides |
|---|---|
| `arm_clobbers_flags(kind)` | true for `AK_SUB_IMM` and `AK_ADD_IMM`, which lower to flag-setting `subs` / `adds`; false for a constant or a local move |
| `clobbering_arms(then, else)` | how many of the two arms write flags when speculated: 0, 1 or 2 |
| `emit_order_ok(total, before_cmp)` | the guard compare must be the last writer of NZCV before the selects, so every clobbering arm must already be emitted. `emit_order_ok(1, 0)` is the bug, and it is asserted false |
| `scratch_for_const(k)` | 0 for a zero arm, 1 otherwise -- the register finding above |

The spec is now 17 functions and 19 tests, sha256
`0e690948987d82d568495961b59936ec89d6d0844160a99e215105e957a0cd99`.
`t27c parse` rc 0; `t27b asm` rc 0, 1329 lines, empty stderr. That is parse and
lowering. **None of the 19 tests has been executed**, and nothing in `cli/t27b`
reads any of them.

The `pr-body.md` count was wrong and is corrected in the same commit: it claimed
16 functions and 14 tests where the file had 13 and 15.

## 10. Will a draft PR actually execute these files? Read, not assumed

The instruction was to read the workflows before asking for publication, because
opening a PR does not by itself guarantee that the tests run. Reading them
changed four things in the request.

**Both workflows do fire, and a draft does not stop them.** `spec-guards.yml`
and `t27b-native.yml` are both `on: pull_request` with a `paths:` list that
includes `specs/**`, and `grep -c draft` returns 0 in each, so neither job is
gated on `github.event.pull_request.draft`. That is the part of the request that
holds as written.

**The ratchet is three jobs, not one.** `t27b-native.yml` -- the file is named
`t27b-native.yml`, not `t27b-native-ratchet.yml` -- runs `t27b-native-linux`
(`ubuntu-24.04-arm`, the crate's own release test suite), `t27b-native-macos`
(`macos-15`, the MAP_JIT and Mach-O path) and `t27b-native-ratchet`. Only the
third reads these specs; the first two exercise the Rust crate, which this
branch does not touch. A publication request that names "the workflow" without
saying which job is not checkable.

**The ledger has no row for any of the five specs this branch touches, and that
is not news.** `grep -c '"path"'` on `docs/reports/t27b_expectations.json`
counts 2602 rows -- it names every spec, passing ones included -- and `grep -c`
for each of `acceptance_matrix.t27`, `l1_clamp.t27`, `ifconv_plan.t27`,
`codegen_audit.t27` and `analog/reference_receipt.t27` returns 0. The last two
are already on master without rows.

**Master's ratchet is already red.** Run `41401435133`, head `783f18b4f8`,
2026-11-02T06:40:46Z: the two crate jobs succeeded, `t27b-native-ratchet`
failed, with the summary line

```
tri t27b ratchet: red -- run 783f18b4f vs ledger 4125e2902:
UNEXPECTED FAILURE 0, UNEXPECTED PASS 0, UNLISTED 42, STALE 0,
OVER CAP 0, BAD REASON 0, MOVED 0, UNJUDGED 0, VACUITY MEASURED 0
```

so every finding is of one kind, and `codegen_audit.t27` and
`reference_receipt.t27` are two of the 42. This branch therefore does not turn
that job red, and `t27b-native.yml`'s own header states the consequence: the
ratchet is not required, and `ready` judges it by Q16, where a red blocks only
while master's run of it is green.

**Why `acceptance_matrix.t27` appears in none of it.** UNLISTED is raised only
when the reference *passes* a spec the ledger does not name
(`scripts/tri_loop/t27b.py:915`, guarded by `unlisted(reference, listed)` in
`t27b_rules.py:182`). A spec that does not exist on master cannot be run there
at all, and `git cat-file -e origin/master:specs/tri/shootout/acceptance_matrix.t27`
fails -- the file is new on this branch, which `pr-body.md` had listed as an
edit to an existing file.

What none of this establishes: that any test in these specs passes. The reading
settles only *where* each file would be executed and *which* red is
pre-existing. Nothing has been run.

**That last sentence is superseded by section 12, and the reading above needs
two corrections that only the run could expose.** Both are kept here rather than
edited away, because the wrong version is what the request was built on.

*First: `spec-guards` fires but executes none of these specs.* Section 10 reads
`paths: specs/**` as "the file is in the trigger list, so the job tests it".
Measured, the job's own selector decides that separately. `sg-full.log:6701-6702`:

```
frontier: reused 36 of 2935 (12 permille); rebuild: spec 63 closure 912 ...
RUN_ALL: a change outside specs/; corpus-ratchet runs the corpus
```

and `grep -c 'shootout\|ifconv_plan' target/ci/sg-full.log` returns **0**. A
change outside `specs/` -- here the `.trinity/receipts/**` files of this very
receipt -- makes the selector hand the corpus to another job and run nothing
itself. So "only the third reads these specs" was right about
`t27b-native.yml`'s three jobs, and `spec-guards` is not a second route to the
same answer, which is how I described it in chat. One job executed these specs,
not two.

*Second: the file list in this receipt and in `pr-body.md` names files that are
master's, not this branch's.* It was built with a two-dot diff,
`git diff --name-status origin/master..HEAD`, and `origin/master` has moved from
this branch's base `8ca1b0817e` to `0215bd7223`. A two-dot diff then reports
master's newer files as this branch's deletions, which is where
`D specs/analog/reference_receipt.t27` and `D docs/analog/reference-receipt-test-2026-11-02.md`
came from -- `git log -- <path>` shows no commit of mine for either. The
three-dot diff, `git diff --name-status origin/master...HEAD`, gives the real
set: **12 files, 8 added receipts, 3 added specs, 1 modified spec**, nothing
under `docs/analog/` or `specs/analog/`. Section 10's own argument about
`acceptance_matrix.t27` being new survives this (`git cat-file -e` was a direct
test of master, not a diff), but the count does not.

Refs #12877.

## 11. Which runner could execute the 16 + 13 tests: every route, checked

"Blocked on a runner" is a claim, so here is the claim's basis: five routes,
each checked with one command, on 2026-11-02.

| route | checked with | state |
|---|---|---|
| `t27-lanes/fs-lab.sh`, the route the Mac load hook's own refusal message names | `ls` | **does not exist.** The hook refuses a heavy command and points at a script the 2026-10-25 disk cleanup removed |
| `ssh t27c-lab` / `ssh t27b-lab`, the hosts the `t27b-loop` skill uses | `grep -i '^Host' ~/.ssh/config` | **not configured.** Five hosts, none of them a lab |
| this Mac | the hook itself | **refused**, by the owner's rule of 2026-10-09 after four watchdog panics. Not appealed, and not worked around |
| a T27 OS pane, the route earlier rounds of this program used to run 21/21 and 23/23 | `pgrep -fl t27os-pty`, then `T27OS_CTL=list` | **server live** (pid 30009, with glm-4.5-flash panes attached), **but the documented listing command is broken**: it produced no output for 120 s and started a *second* `t27os-pty`, which is the one thing the note on this trap says must never happen. The second process was stopped and only pid 30009 remains. Driving panes from this session is therefore unsafe until a listing command that does not spawn a server is found |
| GitHub Actions | section 10 | **available, and gated on the publication confirmation.** Both workflows fire on a draft PR; nothing else is needed |

So the answer to "which allowed arm64 runner" is: the only reachable one is CI,
and reaching it is the confirmation the request asks for. The other four are not
unwilling, they are measurably absent -- which is a different thing from the
word `WAITING_RESOURCE`, and it is why that word is not used here.

One correction to a note of my own, since it cost a process: the recorded
mitigation for this binary was "never run `t27os-pty` bare; use `T27OS_CTL=list`".
On the build running today, `T27OS_CTL=list` behaves like the bare command. The
mitigation is not a mitigation.

Refs #12877.

## 12. The run: what the arm64 runner actually executed

CI was the one reachable route (section 11), the publication was confirmed, and
the run finished. This section replaces the phrase "nothing has been run" with
numbers and the commands that read them.

**Identity of the run.** Workflow `t27b-native.yml`, run `41412031594`, triggered
by the pull request at head `389d6e26f61ddb63ab0dfd86cf1cae5527b18755`, branch
`lane/t27b-floor-model`. Three jobs: `t27b-native-linux` `125167018287`,
`t27b-native-macos` `125167017834`, `t27b-native-ratchet` `125167018191` --
`ubuntu-24.04-arm`, conclusion **success**. The reference verdicts come from
`t27c test-report` built inside that job against zig 0.16.0; the t27b verdicts
from the crate built in the same job. Ledger read: `4125e2902`. Both engines are
built from the branch's own tree, so neither is the Mac binary recorded in the
snapshot at the top of this file.

Read from the job's artifact, `t27b-native-run` id `13125729047`, 433 945 bytes
zipped, unpacked to `target/ci/nat/`:

```
jq -c '.results[]|select((.file//"")|test("shootout|ifconv_plan"))
  |{file,reference,t27b,tests,asserts,invariants,blockers}' target/ci/nat/run.json
```

| spec | reference | t27b | tests | asserts | invariants | blockers |
|---|---|---|---|---|---|---|
| `specs/tri/shootout/codegen_audit.t27` | pass | **pass** | **16** | 53 | 0 | none |
| `specs/tri/shootout/acceptance_matrix.t27` | pass | **pass** | **16** | 94 | 0 | none |
| `specs/tri/shootout/l1_clamp.t27` | pass | pass | 5 | 19 | 0 | none |
| `specs/tri/t27b/ifconv_plan.t27` | pass | pass | 19 | 81 | 0 | none |

56 tests, 247 runtime asserts, on both engines. Not one named verdict is
anything but `true`:

```
jq -r '.results[]|select((.file//"")|test("codegen_audit|acceptance_matrix"))
  |.file, ([.test_verdicts|to_entries[]|select(.value!=true)|.key]|tostring)'
     target/ci/nat/run.json
  ->  specs/tri/shootout/acceptance_matrix.t27   []
      specs/tri/shootout/codegen_audit.t27       []
```

**The request said 16 + 13; the measurement is 16 + 16.** `acceptance_matrix.t27`
carried 13 tests when the run was requested and carries 16 now -- section 9 and
the `AcceptanceCell` repair below both added cases. The number in the request is
not wrong, it is older than the file. Where this receipt says "the 16 + 13 tests"
in section 11, read 16 + 16, and the source of each number is this table, not a
recollection.

**No local run of these specs is recorded in this receipt, and none is claimed.**
There is no `t27b corpus` output for them on the Mac in `.trinity/`, because the
load rule forbids producing one. The arm64 run above is the only measurement
these four specs have. Any earlier sentence of mine that compared a local verdict
with this one had no receipt and is withdrawn rather than repaired.

**The ratchet's own verdict**, `native-ratchet.log:9559` and the job's notice:

```
tri t27b ratchet: green -- run 389d6e26f vs ledger 4125e2902:
UNEXPECTED FAILURE 0, UNEXPECTED PASS 0, UNLISTED 45, STALE 0,
OVER CAP 0, BAD REASON 0, MOVED 0, UNJUDGED 0, VACUITY MEASURED 0

t27b 2502 / reference 2647 native arm64 (+4 pass_vacuous, 5 outside the
reference; 2938 files; jit/interp mismatch 1; reference disagree 0);
ratchet green; corpus 512 s
```

All four of this branch's specs appear in that run as UNLISTED, each with the
same explanation: *the reference passes it and the ledger does not name it (t27b
pass)* (log lines 9547, 9553, 9554, 9556).

### 12a. Why this run is green at UNLISTED 45 and master's is red at 42

Not a rules change. `git diff --stat 8ca1b0817e origin/master -- scripts/tri_loop/
.github/workflows/t27b-native.yml docs/reports/t27b_expectations.json` prints
nothing. The difference is one flag the workflow passes only on a pull request:

```yaml
# .github/workflows/t27b-native.yml:261-268
PR_FLAG: ${{ github.event_name == 'pull_request' && '--pr' || '' }}
./scripts/tri t27b ratchet --run run.json $PR_FLAG --json > ratchet.json
```

and what it means, from the parser itself:

```python
# scripts/tri_loop/t27b.py:1020
ap.add_argument("--pr", action="store_true",
                help="a pull request: lag is pending, a runtime defect stays red")
```

Ledger lag is **pending** on a PR and **red** on a push to master. So the green
above is not evidence that the ledger is satisfied -- it is the rule that a lane
is not held responsible for rows master owes. Stated plainly, and not softened:
**landing this branch takes master's UNLISTED from 42 to 45 and fixes none of
it.** The three added rows (`acceptance_matrix.t27`, `l1_clamp.t27`,
`ifconv_plan.t27`) now have observed values and belong in
`docs/reports/t27b_expectations.json`; the ledger steward writes them from
master's next run, from the observed values only, and `max_not_pass` does not
rise for a passing row.

### 12b. The four red checks, separated by evidence rather than by assertion

Each failing step was read from its own log, and then the same workflow was read
on master's head. Three of the four fail there at a byte-identical step name.

| check | step that failed | master's own run | mine to fix |
|---|---|---|---|
| Corpus ratchet (type names) | "A type name may not gain a second definition" | `41403284655`, job `125139605389`, head `0215bd7223` | **in part -- see below** |
| spec-guards | "Does every seal still describe what the compiler emits" | `41401435163`, head `783f18b4f8` | no: `policy_PolicyOwnLanguage.json` is stale for `specs/policy/own_language.t27`, a file this branch does not touch |
| coverage | "Every seal still describes its spec" | `41403284547`, head `0215bd7223` | no: the same stale seal, 1 newly-not-holding seal |
| loop-tools-tracked | "No t27b spec moves without being named" | `41172959400`, head `4cfd2f2b6a` | no: one case, `ledger: every non-pass entry has a blocker and a known reason`, about `docs/reports/t27b_expectations.json` |

The type-name check is the one that separates cleanly, because it prints its own
arithmetic. Master's log:

```
ledger 141 name(s), observed 142
  + ParsedCommand  NEW conflict
```

this run's log:

```
ledger 141 name(s), observed 143
  + Cell  NEW conflict
  + ParsedCommand  NEW conflict
```

`ParsedCommand` is master's: its two definitions are
`specs/tri/collections/namespace.t27` and
`specs/port/trios/apps/trios-macos/rings/RUST-04/clade-improve/src/main.t27`, the
second added by `b91295e672` and present at this branch's base. `Cell` is the one
name this branch adds -- `pub const Cell = struct` at
`specs/tri/shootout/acceptance_matrix.t27:243`, colliding with
`bootstrap/tests/fixtures/generic_const/pos_04_no_pub.t27:3` and
`cli/t27b/tests/source.rs:1082`.

So the repair is exactly one rename and nothing else: the 17 bare `Cell` tokens
in `acceptance_matrix.t27` become `AcceptanceCell`, after which this check
returns to master's state -- still red, on `ParsedCommand`, which is not mine.
`CELLS` is deliberately **not** renamed: it is a const, not a type, and two
`const CELLS` already coexist on master (`specs/fpga/systolic.t27`,
`specs/queen/views.t27`) without a finding. Every reference to either name lives
inside this one file, which is why the rename needs no second edit.

The other three are not repaired here, and not because they are hard. Each is
another actor's open work -- the own-language seal is PR #13096, the
`ParsedCommand` pair is #13059/#13087 -- and a session merges or edits only a PR
it opened itself, even when that PR is the one holding its own gate.

### 12c. The rename held, and `loop-tools-tracked` has its own repair elsewhere

After the rename, head `59a386af50`, three of the four are gone and one remains:
`loop-tools-tracked` (run `41415969492`, job `125179777508`, 27 s). Its log is
the same shape as before -- 43 `ok` lines, one `FAIL ledger: every non-pass entry
has a blocker and a known reason`, and the cap line still passing at
`(140, cap 140)`. Three facts make it not this branch's, each checkable:

- `git diff <merge-base> HEAD -- docs/reports/t27b_expectations.json
  scripts/tri_loop/` prints nothing: the branch touches neither the ledger the
  case reads nor the checker that reads it.
- `gh run list -w loop-tools-gate -b master -L 4` is `failure` four times, back
  to 2026-10-27 (`41172959400` head `4cfd2f2b6a`, `41034049404`, `40512964239`,
  `40511652636`). The workflow runs on master only when its own paths move, which
  is why the newest master run is 10-31 rather than today.
- The repair exists and is green somewhere else: on PR #13097
  (`fix(t27b): regenerate steward.c so is_pass knows pass_ct; the ledger test asks
  is_pass`, head `0d9ae29cae`) `loop-tools-tracked` **passes** -- run
  `41403967388`, job `125141715680`. So the case is diagnosed and fixed in its own
  lane; duplicating it here would be a second home for the same fix.

One number from the same event is wrong and is worth recording so it is not
quoted later. The PR-dashboard bot's comment of 08:45:41Z reads
`#13099 ... 30 failing / 37 total`. Counted directly at that head,
`gh api repos/.../commits/65c0819e13/check-runs` gives **4 failure, 29 success, 4
cancelled, 4 skipped** -- 41 runs, 4 red. The bot's snapshot was taken while the
run was still starting; its counting method is not read here, and the direct
count is what section 12b is built on.

### 12d. At head `5f00009553` the four reds read exactly as master's

Section 12c was written while two of the four checks were still running, so it
reported one red where the finished set is four. The correction matters only in
one direction: with every check finished, **none of the four is this branch's
any more**, and each says so in master's own words.

| check | run at `5f00009553` | what it prints | master's own run |
|---|---|---|---|
| Corpus ratchet (type names) | `41416539691`, job `125181657226` | `ledger 141 name(s), observed 142` / `+ ParsedCommand  NEW conflict` | `41403284655`, head `0215bd7223` -- byte-identical two lines |
| coverage | `41416539406`, job `125181655035` | `FAIL: 1 seal(s) newly do not hold` / `policy_PolicyOwnLanguage.json  [stale]` | `41403284547`, same seal named |
| spec-guards | `41416539467`, job `125181655813` | `STALE generated-code hash : 1`, all four gen_hashes of the same seal (`gen_hash_rust sealed=6c3c4413c396 current=d9104fe0bd51`, and the zig/c/verilog rows) | `41401435163`, head `783f18b4f8` |
| loop-tools-tracked | `41416539485`, job `125181655709` | the 12c log shape, unchanged | `41172959400` + three older, and green in #13097 |

Two things are now established rather than argued:

- **The `Cell` rename held.** The type-name ratchet observed 143 names while
  `Cell` was in the tree and observes **142** now, and the only name it lists is
  `ParsedCommand`, which is #13059 / #13087's. The arithmetic closes: master
  observes 142 too.
- **The seal is not reachable from here.**
  `git diff <merge-base> HEAD -- specs/policy/ .trinity/seals/` prints nothing,
  so this branch changed neither `specs/policy/own_language.t27` nor any seal.
  Its repair is PR #13096 (`seals: re-seal policy_PolicyOwnLanguage from a
  master-built t27c`), and re-saving a seal here would need both another PR's
  file and a builder this worktree does not have. Not done.

So the event's three names are discharged without a fix, because there is
nothing here to fix: one red of mine existed, it was the `Cell` name, and it is
gone. The two review comments the same event carried are the PR-dashboard bot's
own PR-level comments (`6572951369`, `6573032868`), with no inline thread and
nothing asked of this branch; nothing was acted on, so nothing was replied to.
Their per-PR counts (`30 failing / 34 total`, then `28 failing / 33 total`,
four minutes apart, for a PR whose finished set is 4 red of 41) are two more
mid-run snapshots of the kind 12c already corrected.

Refs #12877.
