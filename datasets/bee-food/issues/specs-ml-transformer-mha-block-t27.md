# Implement the 9 empty function bodies in specs/ml/transformer/mha_block.t27

## Context

`specs/ml/transformer/mha_block.t27` declares module `MHABlock` in 382 lines. 9 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/ml/transformer/mha_block.t27 2>&1 | grep -c 'not yet implemented'
9
$ ./target/release/t27c parse specs/ml/transformer/mha_block.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 67 — `fn init(config: BlockConfig) -> BlockState`
2. line 76 — `fn forward(state: BlockState, input: []gf16::GF16) -> BlockForwardResult`
3. line 92 — `fn forward_with_post_ln(state: BlockState, input: []gf16::GF16) -> BlockForwardResult`
4. line 106 — `fn apply_residual_connection(x: []gf16::GF16, processed: []gf16::GF16, scale: gf16::GF16) -> []gf16::GF16`
5. line 112 — `fn phi_scaled_residual(x: []gf16::GF16, processed: []gf16::GF16) -> []gf16::GF16`
6. line 118 — `fn backward(state: BlockState, result: BlockForwardResult, grad_output: []gf16::GF16) -> BlockGradients`
7. line 133 — `fn get_parameter_count(config: BlockConfig) -> u64`
8. line 140 — `fn get_flops(config: BlockConfig, seq_len: u32) -> u64`
9. line 149 — `fn create_residual_connection(scale: gf16::GF16) -> ResidualConnection`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/ml/transformer/mha_block.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/ml/transformer/mha_block.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 9 names above still exist: `grep -cE '^\s*(pub )?fn (init|forward|forward_with_post_ln|apply_residual_connection|phi_scaled_residual|backward|get_parameter_count|get_flops|create_residual_connection)\(' specs/ml/transformer/mha_block.t27` prints `9`
- 4. `grep -c '^\s*test "' specs/ml/transformer/mha_block.t27` prints at least `9` (today: 0)

## Boundary

specs/ml/transformer/mha_block.t27
