# Implement the 8 empty function bodies in specs/tri/pipeline/builder.t27

## Context

`specs/tri/pipeline/builder.t27` declares module `TriBuilder` in 106 lines. 8 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/pipeline/builder.t27 2>&1 | grep -c 'not yet implemented'
8
$ ./target/release/t27c parse specs/tri/pipeline/builder.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 24 — `fn init(capacity: usize, allocator: std.mem.Allocator) -> !Builder(T)`
2. line 29 — `fn empty() -> Builder(T)`
3. line 34 — `fn append(builder: *Builder(T), item: T, allocator: std.mem.Allocator) -> !void`
4. line 39 — `fn append_slice(builder: *Builder(T), slice: []const T, allocator: std.mem.Allocator) -> !void`
5. line 44 — `fn len(builder: Builder(T)) -> usize`
6. line 49 — `fn capacity(builder: Builder(T)) -> usize`
7. line 54 — `fn finish(builder: Builder(T), allocator: std.mem.Allocator) -> ![]T`
8. line 59 — `fn reset(builder: *Builder(T)) -> void`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/pipeline/builder.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/pipeline/builder.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (init|empty|append|append_slice|len|capacity|finish|reset)\(' specs/tri/pipeline/builder.t27` prints `8`
- 4. `grep -c '^\s*test "' specs/tri/pipeline/builder.t27` prints at least `8` (today: 0)

## Boundary

specs/tri/pipeline/builder.t27
