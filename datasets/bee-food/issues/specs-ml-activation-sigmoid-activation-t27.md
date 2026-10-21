# Implement the 3 empty function bodies in specs/ml/activation/sigmoid_activation.t27

## Context

`specs/ml/activation/sigmoid_activation.t27` declares module `Sigmoid` in 62 lines. 3 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/ml/activation/sigmoid_activation.t27 2>&1 | grep -c 'not yet implemented'
3
$ ./target/release/t27c parse specs/ml/activation/sigmoid_activation.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 22 — `fn forward(x: f32) -> f32`
2. line 27 — `fn forward_batch(input: []f32) -> []f32`
3. line 32 — `fn derivative(sigmoid_x: f32) -> f32`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/ml/activation/sigmoid_activation.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/ml/activation/sigmoid_activation.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 3 names above still exist: `grep -cE '^\s*(pub )?fn (forward|forward_batch|derivative)\(' specs/ml/activation/sigmoid_activation.t27` prints `3`
- 4. `grep -c '^\s*test "' specs/ml/activation/sigmoid_activation.t27` prints at least `3` (today: 0)

## Boundary

specs/ml/activation/sigmoid_activation.t27
