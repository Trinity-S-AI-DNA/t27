# Implement the 6 empty function bodies in specs/ml/rl/ppo_clip_loss.t27

## Context

`specs/ml/rl/ppo_clip_loss.t27` declares module `PpoClipLoss` in 140 lines. 6 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/ml/rl/ppo_clip_loss.t27 2>&1 | grep -c 'not yet implemented'
6
$ ./target/release/t27c parse specs/ml/rl/ppo_clip_loss.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 40 — `fn compute_ratio(new_log_prob: f32, old_log_prob: f32) -> f32`
2. line 45 — `fn clipped_surrogate(ratio: f32, advantage: f32, clip_epsilon: f32) -> f32`
3. line 50 — `fn policy_loss(ratios: []f32, advantages: []f32, clip_epsilon: f32) -> f32`
4. line 55 — `fn entropy_loss(entropies: []f32, entropy_coef: f32) -> f32`
5. line 60 — `fn value_loss(predicted_values: []f32, target_values: []f32, old_values: []f32, clip_coef: f32) -> f32`
6. line 65 — `fn total_loss(policy_loss_value: f32, value_loss_value: f32, entropy_loss_value: f32, value_loss_coef: f32) -> f32`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/ml/rl/ppo_clip_loss.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/ml/rl/ppo_clip_loss.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 6 names above still exist: `grep -cE '^\s*(pub )?fn (compute_ratio|clipped_surrogate|policy_loss|entropy_loss|value_loss|total_loss)\(' specs/ml/rl/ppo_clip_loss.t27` prints `6`
- 4. `grep -c '^\s*test "' specs/ml/rl/ppo_clip_loss.t27` prints at least `6` (today: 0)

## Boundary

specs/ml/rl/ppo_clip_loss.t27
