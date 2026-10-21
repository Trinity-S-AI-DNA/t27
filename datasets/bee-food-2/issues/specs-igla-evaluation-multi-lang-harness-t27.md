# Implement the 4 empty function bodies in specs/igla/evaluation/multi_lang_harness.t27

## Context

`specs/igla/evaluation/multi_lang_harness.t27` declares module `IGLAMultiLangHarness` in 110 lines. 4 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/igla/evaluation/multi_lang_harness.t27 2>&1 | grep -c 'not yet implemented'
4
$ /Users/playom/t27/target/release/t27c parse specs/igla/evaluation/multi_lang_harness.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 33 — `fn evaluate(lang: LangTarget, prompt: str) -> EvalResult`
2. line 38 — `fn compile_check(lang: LangTarget, code: str) -> bool`
3. line 43 — `fn syntax_check(lang: LangTarget, code: str) -> bool`
4. line 48 — `fn benchmark_latency(lang: LangTarget, code: str) -> f64`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/igla/evaluation/multi_lang_harness.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/igla/evaluation/multi_lang_harness.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 4 names above still exist: `grep -cE '^\s*(pub )?fn (evaluate|compile_check|syntax_check|benchmark_latency)\(' specs/igla/evaluation/multi_lang_harness.t27` prints `4`
- 4. `grep -c '^\s*test "' specs/igla/evaluation/multi_lang_harness.t27` prints at least `4` (today: 0)

## Boundary

specs/igla/evaluation/multi_lang_harness.t27
