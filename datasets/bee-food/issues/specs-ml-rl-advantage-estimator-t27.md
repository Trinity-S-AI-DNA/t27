# Implement the 4 empty function bodies in specs/ml/rl/advantage_estimator.t27

## Context

`specs/ml/rl/advantage_estimator.t27` declares module `Advantage` in 127 lines. 4 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/ml/rl/advantage_estimator.t27 2>&1 | grep -c 'not yet implemented'
4
$ ./target/release/t27c parse specs/ml/rl/advantage_estimator.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 39 — `fn compute_td_residual(reward: f32, value: f32, next_value: f32, gamma: f32, done: bool) -> f32`
2. line 44 — `fn compute_gae(rewards: []f32, values: []f32, dones: []bool, gamma: f32, lambda_: f32) -> []f32`
3. line 49 — `fn compute_returns(rewards: []f32, dones: []bool, last_value: f32, gamma: f32) -> []f32`
4. line 54 — `fn normalize_advantages(advantages: []f32) -> []f32`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/ml/rl/advantage_estimator.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/ml/rl/advantage_estimator.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 4 names above still exist: `grep -cE '^\s*(pub )?fn (compute_td_residual|compute_gae|compute_returns|normalize_advantages)\(' specs/ml/rl/advantage_estimator.t27` prints `4`
- 4. `grep -c '^\s*test "' specs/ml/rl/advantage_estimator.t27` prints at least `4` (today: 0)

## Boundary

specs/ml/rl/advantage_estimator.t27
