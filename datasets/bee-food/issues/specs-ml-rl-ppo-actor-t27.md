# Implement the 5 empty function bodies in specs/ml/rl/ppo_actor.t27

## Context

`specs/ml/rl/ppo_actor.t27` declares module `PpoActor` in 154 lines. 5 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/ml/rl/ppo_actor.t27 2>&1 | grep -c 'not yet implemented'
5
$ ./target/release/t27c parse specs/ml/rl/ppo_actor.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 43 — `fn forward_discrete(state: []const f32, params: []const f32, config: ActorConfig) -> []f32`
2. line 48 — `fn forward_continuous(state: []const f32, params: []const f32, config: ActorConfig) -> PolicyOutput`
3. line 53 — `fn sample_action(policy_output: PolicyOutput, action_space: ActionSpace, deterministic: bool) -> []f32`
4. line 58 — `fn log_prob(action: []f32, policy_output: PolicyOutput, action_space: ActionSpace) -> f32`
5. line 63 — `fn entropy(policy_output: PolicyOutput, action_space: ActionSpace) -> f32`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/ml/rl/ppo_actor.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/ml/rl/ppo_actor.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 5 names above still exist: `grep -cE '^\s*(pub )?fn (forward_discrete|forward_continuous|sample_action|log_prob|entropy)\(' specs/ml/rl/ppo_actor.t27` prints `5`
- 4. `grep -c '^\s*test "' specs/ml/rl/ppo_actor.t27` prints at least `5` (today: 0)

## Boundary

specs/ml/rl/ppo_actor.t27
