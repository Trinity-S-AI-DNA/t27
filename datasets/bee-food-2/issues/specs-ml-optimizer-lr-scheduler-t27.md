# Implement the 7 empty function bodies in specs/ml/optimizer/lr_scheduler.t27

## Context

`specs/ml/optimizer/lr_scheduler.t27` declares module `LrScheduler` in 219 lines. 7 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/ml/optimizer/lr_scheduler.t27 2>&1 | grep -c 'not yet implemented'
7
$ /Users/playom/t27/target/release/t27c parse specs/ml/optimizer/lr_scheduler.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 46 — `fn init(config: SchedulerConfig) -> SchedulerState`
2. line 53 — `fn get_lr(state: SchedulerState) -> gf16::GF16`
3. line 62 — `fn step(state: SchedulerState) -> SchedulerState`
4. line 69 — `fn linear_warmup(step: u32, warmup_steps: u32, min_lr: gf16::GF16, max_lr: gf16::GF16) -> gf16::GF16`
5. line 76 — `fn cosine_decay(step: u32, total_steps: u32, max_lr: gf16::GF16, min_lr: gf16::GF16) -> gf16::GF16`
6. line 84 — `fn phi_cosine_decay(step: u32, total_steps: u32, max_lr: gf16::GF16, min_lr: gf16::GF16) -> gf16::GF16`
7. line 92 — `fn get_lr_at_step(config: SchedulerConfig, step: u64) -> gf16::GF16`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/ml/optimizer/lr_scheduler.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/ml/optimizer/lr_scheduler.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 7 names above still exist: `grep -cE '^\s*(pub )?fn (init|get_lr|step|linear_warmup|cosine_decay|phi_cosine_decay|get_lr_at_step)\(' specs/ml/optimizer/lr_scheduler.t27` prints `7`
- 4. `grep -c '^\s*test "' specs/ml/optimizer/lr_scheduler.t27` prints at least `7` (today: 0)

## Boundary

specs/ml/optimizer/lr_scheduler.t27
