# Handoff: t27b AArch64 golden_spec correction

**Date:** 2026-11-10  
**Branch:** `lane/golden-spec-derived`  
**PR:** Trinity-S-AI-DNA/t27#13498  
**Issue:** Trinity-S-AI-DNA/t27#13453  
**Head commit:** `b46f825d81`

---

## What was done

Corrected the measurement claim in `specs/tri/t27b/golden_spec.t27`:

| Field | Old value | New value |
|---|---|---|
| Hot-loop word count | 11 (projected) | 21 (measured, fresh binary 2026-11-10) |
| `merged_word_savings()` | `return 6` | `return 0` |
| Test assertions | `assert == 6` × 3 | `assert == 0` × 3 |

The 11-word figure was an intermediate projection (after check_elide+cinc, pre-ifconv). The fresh binary on `lane/golden-spec-derived` with all 8 PRs merged shows the naive 21-word form because **none of the 8 optimizations fire on bench_audit**.

## Why optimizations don't fire

`verdict_benchmark_200m` calls `verdict_fn` via `bl 0x0` (not inlined). The check_elide/cinc/ifconv/countdown passes target the inlined form. The hot loop (offsets `0x54..0xa4`) is the naive 21-word sequence:

```
mov(1) mov(2) bl(3) mov(4) adds(5) b.hs(6) adds(7) b.hs(8) cmp(9) b.ls(10)
movz(11) movz(12) b(13) subs(14) b.lo(15) adds(16) b.hs(17) movz(18) movk(19) cmp(20) b.lo(21)
```

GCC -O2 produces 10 words (gap = 11). This is a **regression vs the design** — a follow-up issue is needed.

## State on GitHub

- Branch pushed, all 4 required CI checks green (validate, check-linked-issue, parse-ratchet, own-language).
- **Blocker:** `require_extra_approval_for_unattributed_changes: true` (ruleset ID 14813367). The `Co-Authored-By: Claude Sonnet 4.6 <noreply@anthropic.com>` attribution resolves to external GitHub user `claude`. Owner must click **Approve** once on the PR page.
- Non-required failing checks (`t27b-native-ratchet`, `emit-bitexact`, `spec-guards`, `duplicate-bodies`, `coverage`) are all pre-existing master reds unrelated to this PR.

## For the next agent

1. **To merge:** owner clicks Approve on Trinity-S-AI-DNA/t27#13498. Auto-merge fires immediately.
2. **Follow-up:** file a Trinity-S-AI-DNA/t27 issue: "Why do check_elide/cinc/ifconv/countdown not fire on bench_audit after all 8 PRs merged?" Evidence: `verdict_fn` not inlined (called via `bl`); hot loop = naive form.
3. **Worktree:** branch `lane/golden-spec-derived` (clone from Trinity-S-AI-DNA/t27 and check it out).
4. **Key file:** `specs/tri/t27b/golden_spec.t27` lines 96-102 (measurement comment) and 163-187 (functions + tests).
5. **Tests pass:** `./target/release/t27b test specs/tri/t27b/golden_spec.t27` → PASS 3/3 (run from worktree root or with full path from main clone).

## Related PRs (all merged before this branch)

| PR | Optimization | Status |
|---|---|---|
| #13397 | check_elide (a)+(b) | MERGED |
| #13403 | check_elide (c) | MERGED |
| #13415 | cinc peephole | MERGED |
| #13383 | ifconv | MERGED |
| #13422 | countdown | MERGED |
| #13425 | licm_body_const | MERGED |
| #13452 | countdown liveness guards | MERGED |
