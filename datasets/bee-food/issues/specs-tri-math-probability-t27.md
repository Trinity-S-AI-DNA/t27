# Implement the 5 empty function bodies in specs/tri/math/probability.t27

## Context

`specs/tri/math/probability.t27` declares module `TriProbability` in 66 lines. 5 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/math/probability.t27 2>&1 | grep -c 'not yet implemented'
5
$ ./target/release/t27c parse specs/tri/math/probability.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 14 — `fn bernoulli(p: f64, rng: *std.Random.Default) -> bool`
2. line 19 — `fn binomial(n: usize, p: f64, rng: *std.Random.Default) -> usize`
3. line 24 — `fn poisson(lambda: f64, rng: *std.Random.Default) -> usize`
4. line 29 — `fn normal(mean: f64, std_dev: f64, rng: *std.Random.Default) -> f64`
5. line 34 — `fn exponential(lambda: f64, rng: *std.Random.Default) -> f64`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/math/probability.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/math/probability.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 5 names above still exist: `grep -cE '^\s*(pub )?fn (bernoulli|binomial|poisson|normal|exponential)\(' specs/tri/math/probability.t27` prints `5`
- 4. `grep -c '^\s*test "' specs/tri/math/probability.t27` prints at least `5` (today: 0)

## Boundary

specs/tri/math/probability.t27
