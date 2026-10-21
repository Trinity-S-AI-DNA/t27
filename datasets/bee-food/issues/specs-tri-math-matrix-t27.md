# Implement the 7 empty function bodies in specs/tri/math/matrix.t27

## Context

`specs/tri/math/matrix.t27` declares module `TriMatrix` in 95 lines. 7 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/math/matrix.t27 2>&1 | grep -c 'not yet implemented'
7
$ ./target/release/t27c parse specs/tri/math/matrix.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 25 — `fn init(allocator: std.mem.Allocator, rows: usize, cols: usize) -> Matrix`
2. line 30 — `fn get(m: *Matrix, row: usize, col: usize) -> f64`
3. line 35 — `fn set(m: *Matrix, row: usize, col: usize, value: f64) -> void`
4. line 40 — `fn multiply(a: *Matrix, b: *Matrix, allocator: std.mem.Allocator) -> Matrix`
5. line 45 — `fn transpose(m: *Matrix, allocator: std.mem.Allocator) -> Matrix`
6. line 50 — `fn identity(allocator: std.mem.Allocator, size: usize) -> Matrix`
7. line 55 — `fn deinit(m: *Matrix) -> void`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/math/matrix.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/math/matrix.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 7 names above still exist: `grep -cE '^\s*(pub )?fn (init|get|set|multiply|transpose|identity|deinit)\(' specs/tri/math/matrix.t27` prints `7`
- 4. `grep -c '^\s*test "' specs/tri/math/matrix.t27` prints at least `7` (today: 0)

## Boundary

specs/tri/math/matrix.t27
