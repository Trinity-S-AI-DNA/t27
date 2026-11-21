---
id: t27b-golden-spec
name: t27b-golden-spec
description: Lane manual for "T27B - GOLDEN SPEC" -- keeping t27b's AArch64 code quality measured and moving against gcc -O2 / zig ReleaseSafe through specs/tri/t27b/golden_spec.t27 (the SSOT) and golden_spec_roadmap.t27. Load when the owner says "T27B - GOLDEN SPEC", "golden spec", "t27b optimization", "bench_audit", "parity with gcc", "countdown", "B-wrap", "strength reduction", or resumes this lane from an exported session archive.
---

# T27B - GOLDEN SPEC

Scope is this lane only: t27b optimization quality and speed. GIVEL, admin, CRM, fleet
view and plugin work are other lanes; do not absorb their blockers.

## Where the truth lives

| What | File |
|---|---|
| SSOT: every optimization, its spec, PR, words saved, status | `specs/tri/t27b/golden_spec.t27` (STATUS TABLE) |
| What is next, in order (R0..R10) | `specs/tri/t27b/golden_spec_roadmap.t27` |
| Per-pass safety predicates | `specs/tri/t27b/*_plan.t27` (countdown, check_elide, cinc, ifconv, licm, peel) |
| Suite numbers (geomean, 20 tier-1 specs) | `docs/reports/t27b-bench-2026-10.md`, Results table |
| Pass code | `cli/t27b/src/lower.rs` (countdown, peel, check_elide), `codegen.rs` (ifconv) |
| Pass firing evidence | `T27B_TRACE_PASSES=1` -> `runs/t27b_pass_trace.log`, `runs/t27b_bench_audit.asm` |

## State (2026-11-19)

- Spec changes after #14095 (rules d/e, status rows, R8-R10) wait as an owner-landing
  patch on #14111.
- bench_audit hot loop: 10 words = gcc -O2 parity, soundly (first-iteration peel, #14002).
- Countdown rules (d) signed-bound guard and (e) wrap-mode step landed in code by #14095
  (fixes #14086: `while (i64 i < -5)` ran ~2^32 times); the predicates are in
  the #14111 patch as `countdown_bound_ok` / `countdown_step_ok` in countdown_plan.t27.
- Next: R9 B-wrap (`x * (2^k +- 1)` -> shifted add, wrap mode only), plan spec first.
  R10 B-trap is blocked until an overflow-equivalence proof exists.
- Suite gap: t27b trap is 3.4x gcc -O2 and 3.17x zig ReleaseSafe in instructions;
  trap/wrap is only 1.12x, so the gap is code quality, not the cost of checks.

## How a step lands

1. Plan spec first (`specs/tri/t27b/<name>_plan.t27`, owner-landed: see Traps) with tests and negative
   controls (a shape that must NOT fire). Test on the lab, not the Mac (the Mac load hook
   refuses builds and test runs).
2. The cli/t27b leg is net <= 0 hand-written lines, src and tests together (#8236). If it
   cannot be, park the branch as `rescue/<name>` and report; never add foreign lines over it.
3. Re-measure with a fresh binary and write the count into the golden_spec STATUS TABLE
   (roadmap R6). No count, no claim.
4. Commits say `Refs #N`; never `Closes` an epic (#12871, #13251). Re-seal changed specs
   with a master-built t27c (`.trinity/seals/t27b_*.json`).

## Traps this lane has paid for

- **Every `.t27` under `specs/tri/t27b/` is owner-only** (vf-write-gate, #13251, on master
  since #14083, 2026-11-17): the pre-commit hook refuses an agent commit there ("other"
  layer), and CI's landing role still denies it. Do not bypass it. Prepare the diff, check
  it (typecheck, test-report), post it on the issue as an owner-landing patch, and land
  only the agent-writable part (Rust net <= 0, skills, docs). This also covers
  `specs/tri/t27b/conformance/*.t27`.
- **An old session summary is not the state.** The 2026-11-18 archive said "merge of
  #14095 forbidden"; it had merged on 2026-11-17. Check `gh pr view` / `gh issue view
  -R Trinity-S-AI-DNA/t27` before acting on any summary.
- **`/tmp` artifacts do not survive.** Every harness, patch and draft spec that the
  archive listed under `/tmp` was gone. Put evidence under `runs/` in the lane branch.
- **Read numbers from the report table, not from a summary.** The archive called 1.12x
  "wrap vs gcc -O2"; the table says it is t27b trap / t27b wrap.
- **A `;` alone on a line** is a parse error at module level; blank comment lines are `; `.
- **bench_audit counts are trap mode.** Wrap-only gains (rule e, B-wrap) do not change the
  10-word count; record them as wrap savings.

End your report with the status-dashboard text block (skill status-dashboard).
