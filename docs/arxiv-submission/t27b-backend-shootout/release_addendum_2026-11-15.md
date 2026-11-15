# Release addendum -- t27b optimizer: bench_audit parity (2026-11-15)

Not part of the paper: the paper's numbers stay pinned at commit 9a0f4c394
(paper law, Section "State after the snapshot" and LIMITATIONS.md 18). This
addendum records what changed AFTER the snapshot, with its own measurement.

## Result

The hot loop of `verdict_benchmark_200m`
(specs/tri/shootout/bench_audit.t27) is 10 AArch64 words -- **parity with
GCC -O2 on this witness** -- reached soundly:

- all 8 merged passes (check_elide a/b/c #13397 #13403, cinc #13415,
  ifconv #13383, countdown #13422 #13452, licm #13425, small-callee inline)
  fire on the witness (per-pass FIRED/BAIL evidence via T27B_TRACE_PASSES=1);
- the 2026-11-10 "0 of 8 fire, 21 words" regression did not reproduce and is
  closed (golden_spec.t27 addendum 1);
- the last word required a NEW pass: first-iteration loop peeling
  (specs/tri/t27b/loop_peel_first_iter_plan.t27, 10/10 tests). GCC reaches
  10 words only through C's undefined overflow; t27b's checked u64 keeps the
  first-iteration trap and peels it instead: the peeled prologue copy keeps
  `adds + b.hs`, the clamp `if (x > 1000) { x = 0 }` gives the invariant
  x <= 1000 at the loop head, and the in-loop add relaxes to wrapping.

## Measurement

Debug and release binaries agree: hot loop 0x9c..0xc0 =
`cmp csinc add sub movz cmp csel csel sub cbnz` = 10 words.
Artifacts: runs/t27b_bench_audit{,_release}.asm, runs/t27b_pass_trace{,_release}.log.
Release binary SHA-256: 6091c8521c9511c0ffef087868fb75d6f3ac884d2f5f3708bc76b6ef3c8e7cfb
(3,277,376 bytes).

## Effect on the paper's claims

- "code at the level of an unoptimising compiler" stays true as a snapshot
  statement and becomes conservative for the current tree on this witness;
- no paper table changes; the addendum travels with the release, and the
  NEXT paper revision re-measures (its law: a fresh binary, a fresh count).

## Verification status

- Local: golden_spec 4/4, peel plan 10/10, check_elide 26/26,
  actor_journal 13/13, bench_audit 1/1 (+ JIT/interpreter --check clean);
  sweep reds are pre-existing (proved by git-stash rebuild).
- Pending: the 8,344-test differential corpus run on the lab host
  (docs/reports/t27b-release-lab-runbook.md); owner approval before any tag
  or publication.
