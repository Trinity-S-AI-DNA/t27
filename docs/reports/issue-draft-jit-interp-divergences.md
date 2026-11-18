# Issue draft: JIT/interpreter divergences on the current tree (pre-existing)

## Boundary

t27b's two execution engines disagree on 5 spec files (verified 2026-11-15,
release binary, and re-verified on the clean tree by git-stash rebuild -- none
are caused by the peel pass or the telemetry):

| File | Shape | Observation |
|---|---|---|
| specs/queen/key_credit.t27 | lease/identity | JIT and interpreter verdicts differ (pre-existing) |
| specs/tri/t27b/steward.t27 | delta paths | 5 divergent tests on the clean tree, 1 on the current tree |
| specs/queen/actor_journal.t27 | uninitialized `[256]u64` local | interpreter faults "8-byte read at stack+0x808 never written", JIT returns 0; spec passes 13/13 on BOTH engines |
| specs/numeric/bigint.t27 | bignum paths | many divergent tests on the clean tree (49 lines) |
| specs/math/pellis_precision_verify.t27 | precision replay | diverges on the clean tree |

## User scenario

A spec author runs `t27b test --check` to get the two-engine agreement seal
and cannot tell whether a mismatch is their bug or a known engine gap.

## Requirements

1. Each engine gap gets a named defect: which side is wrong (JIT or
   interpreter) is decided by the spec's own semantics, not by majority.
2. `--check` output gains a KNOWN-DIVERGENCE list (file+test names) so a
   clean sweep can fail on anything outside the list (the lab runbook
   already keeps this list by hand: docs/reports/t27b-release-lab-runbook.md
   section 2).
3. The actor_journal shape reduces to a rule: an uninitialized aggregate read
   is either a defined trap in both engines or zero-init in both; the current
   split (JIT zero, interpreter fault) must resolve one way.

## Success criteria

- `t27b test --check` over specs/queen + specs/tri/t27b + specs/tri/shootout +
  specs/numeric + specs/golden + specs/math (239 files) reports 0 mismatches
  outside the KNOWN-DIVERGENCE list;
- the list shrinks (each fixed entry is removed in the fixing PR);
- no corpus coverage change.

## Next report

Owner-assigned lane; free model; per-file bisection (JIT vs interpreter) as
separate small issues.
