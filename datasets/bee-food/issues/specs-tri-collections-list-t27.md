# Implement the 7 empty function bodies in specs/tri/collections/list.t27

## Context

`specs/tri/collections/list.t27` declares module `TriList` in 96 lines. 7 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/collections/list.t27 2>&1 | grep -c 'not yet implemented'
7
$ ./target/release/t27c parse specs/tri/collections/list.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 24 — `fn empty() -> List(void)`
2. line 29 — `fn cons(head: T) -> void`
3. line 34 — `fn head(list: List(T)) -> void`
4. line 39 — `fn tail(list: List(T)) -> void`
5. line 44 — `fn map(list: List(T)) -> void`
6. line 49 — `fn filter(list: List(T)) -> void`
7. line 54 — `fn fold(list: List(T)) -> void`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/collections/list.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/collections/list.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 7 names above still exist: `grep -cE '^\s*(pub )?fn (empty|cons|head|tail|map|filter|fold)\(' specs/tri/collections/list.t27` prints `7`
- 4. `grep -c '^\s*test "' specs/tri/collections/list.t27` prints at least `7` (today: 0)

## Boundary

specs/tri/collections/list.t27
