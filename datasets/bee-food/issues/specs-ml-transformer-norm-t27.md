# Implement the 11 empty function bodies in specs/ml/transformer/norm.t27

## Context

`specs/ml/transformer/norm.t27` declares module `LayerNorm` in 477 lines. 11 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/ml/transformer/norm.t27 2>&1 | grep -c 'not yet implemented'
11
$ ./target/release/t27c parse specs/ml/transformer/norm.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 82 — `fn init(config: NormConfig) -> NormState`
2. line 95 — `fn forward(state: NormState, input: []gf16::GF16) -> ForwardResult`
3. line 107 — `fn forward_strand(strand: Strand, input: []gf16::GF16) -> ForwardResult`
4. line 114 — `fn backward(state: NormState, result: ForwardResult, grad_output: []gf16::GF16) -> NormGradients`
5. line 122 — `fn backward_strand(strand: Strand, result: ForwardResult, grad_output: []gf16::GF16) -> NormGradients`
6. line 128 — `fn update(state: NormState, grads: NormGradients, lr: gf16::GF16) -> NormState`
7. line 135 — `fn normalize_vector(x: []gf16::GF16, eps: gf16::GF16) -> []gf16::GF16`
8. line 145 — `fn phi_init_gamma(hidden_size: u32) -> []gf16::GF16`
9. line 152 — `fn phi_init_beta(hidden_size: u32) -> []gf16::GF16`
10. line 159 — `fn phi_normalize(x: gf16::GF16, mean: gf16::GF16, var: gf16::GF16) -> gf16::GF16`
11. line 166 — `fn create_strands(hidden_size: u32, gamma: []gf16::GF16, beta: []gf16::GF16) -> []Strand`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/ml/transformer/norm.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/ml/transformer/norm.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 11 names above still exist: `grep -cE '^\s*(pub )?fn (init|forward|forward_strand|backward|backward_strand|update|normalize_vector|phi_init_gamma|phi_init_beta|phi_normalize|create_strands)\(' specs/ml/transformer/norm.t27` prints `11`
- 4. `grep -c '^\s*test "' specs/ml/transformer/norm.t27` prints at least `11` (today: 0)

## Boundary

specs/ml/transformer/norm.t27
