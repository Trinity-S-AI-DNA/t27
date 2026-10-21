# Implement part 1 of 2 of the empty bodies in specs/ml/transformer/norm.t27 (8 functions)

## Context

`specs/ml/transformer/norm.t27` holds 11 functions declared with a body that contains no statement. It is split into 2 issues because one turn cannot carry them all.

**All 2 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/ml/transformer/norm.t27 2>&1 | grep -c 'not yet implemented'
11      # if it is not 11, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 82 — `fn init(config: NormConfig) -> NormState`
2. line 95 — `fn forward(state: NormState, input: []gf16::GF16) -> ForwardResult`
3. line 107 — `fn forward_strand(strand: Strand, input: []gf16::GF16) -> ForwardResult`
4. line 114 — `fn backward(state: NormState, result: ForwardResult, grad_output: []gf16::GF16) -> NormGradients`
5. line 122 — `fn backward_strand(strand: Strand, result: ForwardResult, grad_output: []gf16::GF16) -> NormGradients`
6. line 128 — `fn update(state: NormState, grads: NormGradients, lr: gf16::GF16) -> NormState`
7. line 135 — `fn normalize_vector(x: []gf16::GF16, eps: gf16::GF16) -> []gf16::GF16`
8. line 145 — `fn phi_init_gamma(hidden_size: u32) -> []gf16::GF16`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/ml/transformer/norm.t27 2>&1 | grep -c 'not yet implemented'` prints `3` (on arrival: 11)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/ml/transformer/norm.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (init|forward|forward_strand|backward|backward_strand|update|normalize_vector|phi_init_gamma)\(' specs/ml/transformer/norm.t27` prints `8`

## Boundary

specs/ml/transformer/norm.t27
