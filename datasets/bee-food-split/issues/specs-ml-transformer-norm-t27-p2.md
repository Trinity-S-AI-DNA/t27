# Implement part 2 of 2 of the empty bodies in specs/ml/transformer/norm.t27 (3 functions)

## Context

`specs/ml/transformer/norm.t27` holds 11 functions declared with a body that contains no statement. It is split into 2 issues because one turn cannot carry them all.

**All 2 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/ml/transformer/norm.t27 2>&1 | grep -c 'not yet implemented'
3      # if it is not 3, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 152 — `fn phi_init_beta(hidden_size: u32) -> []gf16::GF16`
2. line 159 — `fn phi_normalize(x: gf16::GF16, mean: gf16::GF16, var: gf16::GF16) -> gf16::GF16`
3. line 166 — `fn create_strands(hidden_size: u32, gamma: []gf16::GF16, beta: []gf16::GF16) -> []Strand`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/ml/transformer/norm.t27 2>&1 | grep -c 'not yet implemented'` prints `0` (on arrival: 3)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/ml/transformer/norm.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 3 names above still exist: `grep -cE '^\s*(pub )?fn (phi_init_beta|phi_normalize|create_strands)\(' specs/ml/transformer/norm.t27` prints `3`

## Boundary

specs/ml/transformer/norm.t27
