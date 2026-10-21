# Implement the 5 empty function bodies in specs/tri/collections/stack.t27

## Context

`specs/tri/collections/stack.t27` declares module `TriStack` in 70 lines. 5 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/collections/stack.t27 2>&1 | grep -c 'not yet implemented'
5
$ ./target/release/t27c parse specs/tri/collections/stack.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 22 — `fn empty() -> Stack(T)`
2. line 27 — `fn push(stack: Stack(T)) -> void`
3. line 32 — `fn pop(stack: Stack(T)) -> void`
4. line 37 — `fn peek(stack: Stack(T)) -> void`
5. line 42 — `fn is_empty(stack: Stack(T)) -> void`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/collections/stack.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/collections/stack.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 5 names above still exist: `grep -cE '^\s*(pub )?fn (empty|push|pop|peek|is_empty)\(' specs/tri/collections/stack.t27` prints `5`
- 4. `grep -c '^\s*test "' specs/tri/collections/stack.t27` prints at least `5` (today: 0)

## Boundary

specs/tri/collections/stack.t27
