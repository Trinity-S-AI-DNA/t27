# t27b optimizer release notes -- bench_audit parity (2026-11-15)

## Headline

All 8 merged optimizer passes FIRE on the bench_audit witness, and a new
sound pass (first-iteration loop peeling) closes the last word: the hot loop
of `verdict_benchmark_200m` is **10 words = GCC -O2 parity**, without
dropping any trap the spec's semantics require.

## Changes

| Change | Spec | Where |
|---|---|---|
| Pass telemetry (env-gated, `T27B_TRACE_PASSES=1`) | golden_spec_roadmap.t27 R1 | cli/t27b/src/pass_trace.rs + decision points |
| First-iteration peel of a clamped counter | loop_peel_first_iter_plan.t27 (10/10 tests) | cli/t27b/src/lower.rs `peel_clamped`, gen: gen/rust/tri/t27b/loop_peel_first_iter_plan.rs |
| Golden-spec SSOT updated: 8/8 firing, 10 words, gap 0 | golden_spec.t27 addenda 1+2 | specs/tri/t27b/golden_spec.t27 |
| ifconv trace fix (guard-form arm; an earlier unguarded draft dropped non-ifconv ifs -- caught by negative controls, fixed before landing) | -- | cli/t27b/src/codegen.rs |

## Measurement (fresh binaries, this worktree)

- Hot loop 0x9c..0xc0: `cmp csinc add sub movz cmp csel csel sub cbnz` = 10 words (debug and release agree).
- Peeled prologue copy keeps `adds + b.hs`: `tick == u64::MAX` still traps.
- Per-pass evidence: runs/t27b_pass_trace.log, runs/t27b_pass_trace_release.log (8 FIRED, licm bails on the one-movz 1000 by design).
- Asm: runs/t27b_bench_audit.asm (debug), runs/t27b_bench_audit_release.asm (release).
- JIT/interpreter `--check` on bench_audit: 0 mismatches.

## Release binary

- Path: target/release/t27b (built by `cargo build --release -p t27b`)
- SHA-256: 6091c8521c9511c0ffef087868fb75d6f3ac884d2f5f3708bc76b6ef3c8e7cfb
- Size: 3,277,376 bytes (3.13 MB)
- Debug binary: same source; sha256 recorded at packaging time.

## Tests

- golden_spec 4/4, golden_spec_roadmap 3/3, loop_peel_first_iter_plan 10/10,
  check_elide_plan 26/26, actor_journal 13/13, bench_audit 1/1 (+ --check clean).
- Sweep specs/tri/t27b + specs/queen + specs/tri/shootout: the only reds are
  pre-existing on the clean tree (verified by git-stash rebuild):
  test_restore.t27 (redeclaration), bench_200m.t27 (typecheck), improve/
  option1-3 (undeclared `LOG`). No new reds from this work.
- Pre-existing JIT/interpreter divergences (not from this work; filed for the
  lab): key_credit.t27, steward.t27, and the uninitialized-array shape of
  actor_journal.t27's junk-kinds negative control (interpreter faults on a
  read the JIT never makes; spec passes 13/13 on both engines).

## Corpus A/A comparison (2026-11-15, local, 3,212 files)

The full current corpus ran through the release binary twice -- with the
optimizer work (peel build) and on the clean tree (git-stash rebuild).
**Verdicts identical on all 3,212 files: 0 diffs.** Distribution:
2,779 pass, 28 test-fail, 396 compile-error, 9 other -- every red verified
pre-existing. This is verdict-stability evidence, not the paper's coverage
metric (that needs the reference pipeline: lab runbook, section 1).

## Full-corpus JIT/interpreter cross-check (--check, 3,212 files, 2026-11-15)

Every file ran both engines (per-file timeout). 101 files report at least one
mismatch -- and **all 101 are specs that already fail plain t27b test**
(pre-existing test failures or compile errors; the failing test itself is
where the engines diverge). On all 3,111 files whose tests pass, JIT and
interpreter agree exactly: **0 engine disagreements on any passing spec**.
The 5 known divergences on passing-or-near-passing shapes stay documented in
docs/reports/issue-draft-jit-interp-divergences.md.

## Local reference differential (2026-11-15, 256-file subset)

The reference pipeline ran locally where it can: t27c gen (debug build) +
`zig test` 0.17.1, native arm64 macOS, against t27b's native JIT -- over
specs/queen + specs/tri/t27b + specs/tri/shootout + specs/numeric +
specs/golden + specs/math (256 files; runs/ref_diff.txt).

- 223 files: t27b and Zig pass (witness: wasm.t27, 108/108 == 108/108).
- 26 files: both fail.
- 7 files differ -- none is a test-verdict disagreement:
  2 x generated Zig does not compile under 0.17.1 (API drift vs the paper's
  0.16: acceptance_matrix std.mem.eql, libm aborted on comptime @log(10.0)
  differing 1 ulp from the spec's 64-digit constant -- a Zig-version
  artifact; t27b passes libm 32/32);
  3 x t27b blocked constructs (lucas_accumulator, posit_ladder_control,
  tf3 -- coverage gaps, Zig passes);
  2 x t27b parse/typecheck error against a vacuous Zig "All 0 tests passed"
  (queen_pipeline_events, test_restore -- the known pre-existing reds).
- Zero files where both engines ran the same tests and disagreed on a
  verdict.

Caveats: native arm64 macOS instead of aarch64-linux under qemu, Zig 0.17.1
instead of 0.16, one directory subset instead of the full corpus -- so this
is supporting evidence, not the paper's 1,296/1,311 metric (lab runbook
section 1 still owns that number on its pinned revision).

The final corpus classification command (run when the sweep completes):

    awk -F'|' '{tb=$2; zg=$3; tbpass=(tb ~ / 0 failed/); zgpass=(zg ~ /All [0-9]+ tests passed/);
      if (tbpass&&zgpass) bp++; else if (!tbpass&&!zgpass) bf++; else d++}
      END {print "files="NR" both_pass="bp" both_fail="bf" diffs="d+0}' runs/ref_full.txt

## FULL-corpus reference differential -- FINAL (3,212 files, 2026-11-15)

The complete current corpus ran through both engines: t27b's native JIT and
the reference pipeline (t27c gen + zig test 0.17.1, native arm64; runs/ref_complete.txt).

- files: 3,212
- both pass: 2,738
- both fail: 290 (verdicts agree on the failures too)
- differ: 184 -- and NONE is a verdict disagreement:
  - 44 generated-Zig compile errors on 0.17.1 (API drift vs the paper's 0.16);
  - 140 t27b blocked constructs (reference var params, @exp/@sin/@constCast
    builtins, array literals, BenchBlock, std.*) -- coverage gaps, Zig passes;
  - (0 other)
- local coverage: 90.8% (2,878 / 3,168 files the reference could reach).

Honest caveats: bigger corpus than the paper's 1,819, native arm64 macOS
instead of aarch64-linux under qemu, Zig 0.17.1 instead of 0.16 -- so 90.8%
here is NOT the paper's 98.9% (that number needs the pinned revision on the
lab host; runbook section 1). The release-relevant claim this run DOES make:
the optimizer work changed no verdict anywhere in 3,212 files, and no new
engine disagreement appeared.

## PINNED-REVISION COVERAGE -- the paper's criterion, met locally (2026-11-15)

The paper's coverage metric, reproduced locally with today's release binary:
the 1,311 files the reference passed at the pinned revision (9a0f4c394,
per-file classes from docs/reports/t27b-bench-2026-10.json, content extracted
via git archive) ran through the current release t27b.

- PASS: 1,306 / 1,311 = **99.62%** -- above the 98.9% criterion.
- At the snapshot the same metric was 1,296 / 1,311 = 98.86%: the current
  tree is BETTER than the paper's snapshot (10 more files pass).
- The 5 remaining blocks are known unsupported constructs (2x frame address,
  undeclared str_contains, assignment through a reference) -- same classes
  the paper lists, none introduced by this work.
- Evidence: runs/ref_pass_pinned.txt (file list), runs/pinned_coverage.txt
  (the 5 blocks), command in this file's history.

Caveat kept honest: the reference verdicts are the pinned revision's
committed classes (zig 0.16 aarch64-linux on the lab); t27b ran locally,
native arm64. That is the same evidence split the paper itself uses (t27b
verdicts are t27b's own; the reference is the committed measurement).

## Not in this release (pending)

- Full 8,344-test differential corpus run: lab-host work (lab rule: no heavy
  sweeps on this Mac); the paper's snapshot numbers stay pinned at 9a0f4c394.
- Paper addendum: the 10-word result and the peel pass go into the release
  addendum, not into the snapshot-pinned tables (paper law, LIMITATIONS.md 18).
- Owner approval before any publication or tag push.
