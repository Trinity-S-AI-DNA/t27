# Implement the 7 empty function bodies in specs/tri/io/filesystem.t27

## Context

`specs/tri/io/filesystem.t27` declares module `TriFilesystem` in 112 lines. 7 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/io/filesystem.t27 2>&1 | grep -c 'not yet implemented'
7
$ ./target/release/t27c parse specs/tri/io/filesystem.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 30 — `fn join(allocator: std.mem.Allocator, parts: [][]const u8) -> ![]u8`
2. line 35 — `fn basename(path: []const u8) -> []const u8`
3. line 40 — `fn dirname(path: []const u8) -> []const u8`
4. line 45 — `fn ext(path: []const u8) -> []const u8`
5. line 50 — `fn has_ext(path: []const u8, ext: []const u8) -> bool`
6. line 55 — `fn is_absolute(path: []const u8) -> bool`
7. line 60 — `fn normalize(allocator: std.mem.Allocator, path: []const u8) -> ![]u8`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/io/filesystem.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/io/filesystem.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 7 names above still exist: `grep -cE '^\s*(pub )?fn (join|basename|dirname|ext|has_ext|is_absolute|normalize)\(' specs/tri/io/filesystem.t27` prints `7`
- 4. `grep -c '^\s*test "' specs/tri/io/filesystem.t27` prints at least `7` (today: 0)

## Boundary

specs/tri/io/filesystem.t27
