# Implement part 1 of 3 of the empty bodies in specs/ml/optimizer/adamw.t27 (8 functions)

## Context

`specs/ml/optimizer/adamw.t27` holds 18 functions declared with a body that contains no statement. It is split into 3 issues because one turn cannot carry them all.

**All 3 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/ml/optimizer/adamw.t27 2>&1 | grep -c 'not yet implemented'
18      # if it is not 18, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 59 — `fn init(config: AdamWConfig, param_count: u32) -> AdamWState`
2. line 68 — `fn step(state: AdamWState, params: []gf16::GF16, grads: []gf16::GF16) -> OptimizerStepResult`
3. line 93 — `fn compute_bias_correction(beta: gf16::GF16, t: u64) -> gf16::GF16`
4. line 99 — `fn update_first_moment(m_prev: gf16::GF16, grad: gf16::GF16, beta1: gf16::GF16) -> gf16::GF16`
5. line 105 — `fn update_second_moment(v_prev: gf16::GF16, grad: gf16::GF16, beta2: gf16::GF16) -> gf16::GF16`
6. line 111 — `fn apply_weight_decay(params: []gf16::GF16, lr: gf16::GF16, weight_decay: gf16::GF16) -> []gf16::GF16`
7. line 117 — `fn compute_update(m: gf16::GF16, v: gf16::GF16, lr_t: gf16::GF16, epsilon: gf16::GF16) -> gf16::GF16`
8. line 123 — `fn get_effective_beta(config: AdamWConfig, beta_type: []const u8) -> gf16::GF16`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/ml/optimizer/adamw.t27 2>&1 | grep -c 'not yet implemented'` prints `10` (on arrival: 18)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/ml/optimizer/adamw.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (init|step|compute_bias_correction|update_first_moment|update_second_moment|apply_weight_decay|compute_update|get_effective_beta)\(' specs/ml/optimizer/adamw.t27` prints `8`

## Boundary

specs/ml/optimizer/adamw.t27
