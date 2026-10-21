# Implement the 8 empty function bodies in specs/tri/utils/bytes.t27

## Context

`specs/tri/utils/bytes.t27` declares module `TriBytes` in 105 lines. 8 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/utils/bytes.t27 2>&1 | grep -c 'not yet implemented'
8
$ ./target/release/t27c parse specs/tri/utils/bytes.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 23 — `fn empty() -> Bytes`
2. line 28 — `fn from_slice(slice: []const u8) -> Bytes`
3. line 33 — `fn clone(bytes: Bytes, allocator: std.mem.Allocator) -> !Bytes`
4. line 38 — `fn equals(a: Bytes, b: Bytes) -> bool`
5. line 43 — `fn slice(bytes: Bytes, start: usize, end: usize) -> Bytes`
6. line 48 — `fn concat(a: Bytes, b: Bytes, allocator: std.mem.Allocator) -> !Bytes`
7. line 53 — `fn index_of(bytes: Bytes, pattern: []const u8) -> ?usize`
8. line 58 — `fn split(bytes: Bytes, delimiter: u8, allocator: std.mem.Allocator) -> ![]Bytes`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/utils/bytes.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/utils/bytes.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (empty|from_slice|clone|equals|slice|concat|index_of|split)\(' specs/tri/utils/bytes.t27` prints `8`
- 4. `grep -c '^\s*test "' specs/tri/utils/bytes.t27` prints at least `8` (today: 0)

## Boundary

specs/tri/utils/bytes.t27
