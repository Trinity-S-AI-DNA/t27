# Implement part 1 of 1 of the empty bodies in specs/nn/hslm.t27 (4 functions)

## Context

`specs/nn/hslm.t27` holds 4 functions declared with a body that contains no statement. It is split into 1 issues because one turn cannot carry them all.

**All 1 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/nn/hslm.t27 2>&1 | grep -c 'not yet implemented'
4      # if it is not 4, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 360 — `fn ffn_backward_w2(grads: *HSLMWeights, layer_idx: usize) -> void`
2. line 368 — `fn gelu_backward(layer_idx: usize) -> void`
3. line 375 — `fn ffn_backward_w1(grads: *HSLMWeights, layer_idx: usize) -> void`
4. line 382 — `fn attention_backward(grads: *HSLMWeights, layer_idx: usize) -> void`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/nn/hslm.t27 2>&1 | grep -c 'not yet implemented'` prints `0` (on arrival: 4)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/nn/hslm.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 4 names above still exist: `grep -cE '^\s*(pub )?fn (ffn_backward_w2|gelu_backward|ffn_backward_w1|attention_backward)\(' specs/nn/hslm.t27` prints `4`

## Boundary

specs/nn/hslm.t27
