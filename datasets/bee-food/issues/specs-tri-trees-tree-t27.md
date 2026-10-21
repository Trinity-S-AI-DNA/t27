# Implement the 6 empty function bodies in specs/tri/trees/tree.t27

## Context

`specs/tri/trees/tree.t27` declares module `TriTree` in 84 lines. 6 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/trees/tree.t27 2>&1 | grep -c 'not yet implemented'
6
$ ./target/release/t27c parse specs/tri/trees/tree.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 25 — `fn leaf(value: T) -> Tree(T)`
2. line 30 — `fn branch(left: Tree(T), right: Tree(T)) -> Tree(T)`
3. line 35 — `fn is_leaf(tree: Tree(T)) -> bool`
4. line 40 — `fn height(tree: Tree(T)) -> usize`
5. line 45 — `fn size(tree: Tree(T)) -> usize`
6. line 50 — `fn inorder(tree: Tree(T)) -> []T`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/trees/tree.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/trees/tree.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 6 names above still exist: `grep -cE '^\s*(pub )?fn (leaf|branch|is_leaf|height|size|inorder)\(' specs/tri/trees/tree.t27` prints `6`
- 4. `grep -c '^\s*test "' specs/tri/trees/tree.t27` prints at least `6` (today: 0)

## Boundary

specs/tri/trees/tree.t27
