# M1.3 host-boundary budget amendment — DRAFT for owner approval — 2026-11-10

Lane: `lane/synth-spec-candidate-1` (worktree `.claude/worktrees/synth-spec-landing`).
Target file: `cli/tri/src/fpga_synth_spec.rs` (1064 lines, hand-written Rust call-site glue).

## Scope of the amendment (narrow)

This amendment applies to `cli/tri/src/fpga_synth_spec.rs` ONLY.

1. foreign-exceptions entry (one line, one comment):

    # owner 2026-11-10, M1.3 / milestone 4: call-site glue for the generated
    # synth spec mount; decisions stay in specs/fpga/synth_spec_pipeline.t27
    # (10/10 t27c+t27b) and specs/fpga/synth_spec_run.t27; contract tests in
    # the same PR. File-scoped; no other cli/tri/src path.
    cli/tri/src/fpga_synth_spec.rs

2. Generated `.t27` mount: the spec side stays in `specs/fpga/synth_spec_pipeline.t27`,
   `synth_spec_run.t27`, `synth_receipt.t27`; the Rust file is glue that calls into it.
   No second spec, no parallel math implementation (L2/SSOT rule).

3. Contract tests travel in the SAME commit: the 13/13 Rust tests (receipt schema,
   Yosys byte verification) plus the `.t27` gate suites.

4. The general 40/80 rule is NOT weakened: check_budget still applies to this file
   and every other foreign file; the entry grants path permission, not line budget.

## What the order authorizes vs what repo law requires

- The order authorizes PREPARING this amendment and then landing M1.3 in one commit
  after green gates.
- `tools/policy/foreign-exceptions.txt` law: an entry takes effect for local commits
  only once the OWNER merged it (a branch cannot list itself), and in CI the entry on
  the PR BASE must carry the `owner-approved-foreign` label. No entry lifts the 40/80
  budget.
- Therefore: the PR carries the entry + the file + tests in one commit and is marked
  `see` only until the owner adds the label / merges the entry PR. Landing is NOT
  self-serve. Owner decision requested: approve the entry above (or amend it).

## Pre-landing checklist

- [x] Recovery base green (commit 9dc4f9699d; t27c 10/10 + 3/3 + 3/3 + 2/2; t27b 10/10 + 3/3).
- [ ] Dirty foreign WIP in the lane worktree (`.trinity/seals/fpga_FpgaBoardReceipt.json`,
      `cli/tri/src/fpga.rs` modifications) must NOT enter the M1.3 commit; park or separate it first.
- [ ] Re-run 13/13 Rust tests on the final tree (deferred while swap 12.7/13.3 GB — build HOLD).
- [ ] One commit, no foreign WIP, full gate, PR carries the amendment entry and requests
      the `owner-approved-foreign` label.
