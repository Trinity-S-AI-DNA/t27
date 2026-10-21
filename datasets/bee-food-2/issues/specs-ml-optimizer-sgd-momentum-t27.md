# Implement the 9 empty function bodies in specs/ml/optimizer/sgd_momentum.t27

## Context

`specs/ml/optimizer/sgd_momentum.t27` declares module `SgdMomentum` in 363 lines. 9 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/ml/optimizer/sgd_momentum.t27 2>&1 | grep -c 'not yet implemented'
9
$ /Users/playom/t27/target/release/t27c parse specs/ml/optimizer/sgd_momentum.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 52 — `fn init(config: SgdMomentumConfig, param_count: u32) -> SgdMomentumState`
2. line 59 — `fn step(state: SgdMomentumState, params: []gf16::GF16, grads: []gf16::GF16) -> OptimizerStepResult`
3. line 76 — `fn compute_velocity(velocity: gf16::GF16, grad: gf16::GF16, momentum: gf16::GF16, dampening: gf16::GF16) -> gf16::GF16`
4. line 82 — `fn nesterov_update(param: gf16::GF16, velocity: gf16::GF16, grad: gf16::GF16, lr: gf16::GF16, momentum: gf16::GF16) -> gf16::GF16`
5. line 88 — `fn standard_update(param: gf16::GF16, velocity: gf16::GF16, lr: gf16::GF16) -> gf16::GF16`
6. line 94 — `fn apply_weight_decay(params: []gf16::GF16, grads: []gf16::GF16, weight_decay: gf16::GF16) -> []gf16::GF16`
7. line 100 — `fn phi_damped_momentum(base_momentum: gf16::GF16) -> gf16::GF16`
8. line 106 — `fn get_effective_momentum(config: SgdMomentumConfig) -> gf16::GF16`
9. line 113 — `fn zero_grad(state: SgdMomentumState) -> SgdMomentumState`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/ml/optimizer/sgd_momentum.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/ml/optimizer/sgd_momentum.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 9 names above still exist: `grep -cE '^\s*(pub )?fn (init|step|compute_velocity|nesterov_update|standard_update|apply_weight_decay|phi_damped_momentum|get_effective_momentum|zero_grad)\(' specs/ml/optimizer/sgd_momentum.t27` prints `9`
- 4. `grep -c '^\s*test "' specs/ml/optimizer/sgd_momentum.t27` prints at least `9` (today: 0)

## Boundary

specs/ml/optimizer/sgd_momentum.t27
