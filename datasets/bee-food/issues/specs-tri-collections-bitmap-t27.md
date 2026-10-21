# Implement the 10 empty function bodies in specs/tri/collections/bitmap.t27

## Context

`specs/tri/collections/bitmap.t27` declares module `TriBitmap` in 100 lines. 10 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/collections/bitmap.t27 2>&1 | grep -c 'not yet implemented'
10
$ ./target/release/t27c parse specs/tri/collections/bitmap.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 23 — `fn init(capacity: usize, allocator: std.mem.Allocator) -> !Bitmap`
2. line 28 — `fn get(bitmap: Bitmap, index: usize) -> bool`
3. line 33 — `fn set(bitmap: *Bitmap, index: usize) -> void`
4. line 38 — `fn clear(bitmap: *Bitmap, index: usize) -> void`
5. line 43 — `fn flip(bitmap: *Bitmap, index: usize) -> void`
6. line 48 — `fn set_all(bitmap: *Bitmap) -> void`
7. line 53 — `fn clear_all(bitmap: *Bitmap) -> void`
8. line 58 — `fn count(bitmap: Bitmap) -> usize`
9. line 63 — `fn find_first(bitmap: Bitmap) -> ?usize`
10. line 68 — `fn find_last(bitmap: Bitmap) -> ?usize`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/collections/bitmap.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/collections/bitmap.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 10 names above still exist: `grep -cE '^\s*(pub )?fn (init|get|set|clear|flip|set_all|clear_all|count|find_first|find_last)\(' specs/tri/collections/bitmap.t27` prints `10`
- 4. `grep -c '^\s*test "' specs/tri/collections/bitmap.t27` prints at least `10` (today: 0)

## Boundary

specs/tri/collections/bitmap.t27
