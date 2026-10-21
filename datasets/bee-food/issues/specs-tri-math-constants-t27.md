# Implement the 12 empty function bodies in specs/tri/math/constants.t27

## Context

`specs/tri/math/constants.t27` declares module `TriConstants` in 127 lines. 12 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/math/constants.t27 2>&1 | grep -c 'not yet implemented'
12
$ ./target/release/t27c parse specs/tri/math/constants.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 34 — `fn max_path_len() -> usize`
2. line 39 — `fn max_line_len() -> usize`
3. line 44 — `fn max_args() -> usize`
4. line 49 — `fn max_env_vars() -> usize`
5. line 54 — `fn get_p_h_i() -> f64`
6. line 59 — `fn get_p_i() -> f64`
7. line 64 — `fn get_e() -> f64`
8. line 69 — `fn get_s_q_r_t2() -> f64`
9. line 74 — `fn get_s_q_r_t3() -> f64`
10. line 79 — `fn get_golden_ratio() -> f64`
11. line 84 — `fn get_system_limits() -> SystemLimits`
12. line 89 — `fn get_sacred_constants() -> SacredConstants`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/math/constants.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/math/constants.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 12 names above still exist: `grep -cE '^\s*(pub )?fn (max_path_len|max_line_len|max_args|max_env_vars|get_p_h_i|get_p_i|get_e|get_s_q_r_t2|get_s_q_r_t3|get_golden_ratio|get_system_limits|get_sacred_constants)\(' specs/tri/math/constants.t27` prints `12`
- 4. `grep -c '^\s*test "' specs/tri/math/constants.t27` prints at least `12` (today: 0)

## Boundary

specs/tri/math/constants.t27
