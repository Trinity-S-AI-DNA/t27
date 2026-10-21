# Implement the 9 empty function bodies in specs/tri/collections/array.t27

## Context

`specs/tri/collections/array.t27` declares module `TriArray` in 122 lines. 9 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/collections/array.t27 2>&1 | grep -c 'not yet implemented'
9
$ ./target/release/t27c parse specs/tri/collections/array.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 29 — `fn slice(arr: []const T, start: usize, end: usize) -> []const T`
2. line 34 — `fn slice_from(arr: []const T, start: usize) -> []const T`
3. line 39 — `fn first(arr: []const T) -> T`
4. line 44 — `fn last(arr: []const T) -> T`
5. line 49 — `fn is_empty(arr: []const T) -> bool`
6. line 54 — `fn contains(arr: []const T, item: T) -> bool`
7. line 59 — `fn index_of(arr: []const T, item: T) -> ?usize`
8. line 64 — `fn reverse(allocator: std.mem.Allocator, arr: []const T) -> []T`
9. line 69 — `fn concat(allocator: std.mem.Allocator, a: []const T, b: []const T) -> []T`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/collections/array.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/collections/array.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 9 names above still exist: `grep -cE '^\s*(pub )?fn (slice|slice_from|first|last|is_empty|contains|index_of|reverse|concat)\(' specs/tri/collections/array.t27` prints `9`
- 4. `grep -c '^\s*test "' specs/tri/collections/array.t27` prints at least `9` (today: 0)

## Boundary

specs/tri/collections/array.t27
