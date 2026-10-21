# Implement part 3 of 3 of the empty bodies in specs/ml/optimizer/adamw.t27 (2 functions)

## Context

`specs/ml/optimizer/adamw.t27` holds 18 functions declared with a body that contains no statement. It is split into 3 issues because one turn cannot carry them all.

**All 3 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/ml/optimizer/adamw.t27 2>&1 | grep -c 'not yet implemented'
2      # if it is not 2, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 566 — `fn get_effective_betas(variant: PhiVariant) -> (gf16::GF16, gf16::GF16, gf16::GF16)`
2. line 572 — `fn amsgrad_update(v: gf16::GF16, v_max: gf16::GF16) -> gf16::GF16`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/ml/optimizer/adamw.t27 2>&1 | grep -c 'not yet implemented'` prints `0` (on arrival: 2)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/ml/optimizer/adamw.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 2 names above still exist: `grep -cE '^\s*(pub )?fn (get_effective_betas|amsgrad_update)\(' specs/ml/optimizer/adamw.t27` prints `2`

## Boundary

specs/ml/optimizer/adamw.t27
