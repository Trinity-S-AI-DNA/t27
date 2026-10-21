# Implement the 10 empty function bodies in specs/git/status.t27

## Context

`specs/git/status.t27` declares module `GitStatus` in 188 lines. 10 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/git/status.t27 2>&1 | grep -c 'not yet implemented'
10
$ ./target/release/t27c parse specs/git/status.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 14 — `fn status(cwd: str) -> Result<[Item], GitError>`
2. line 19 — `fn is_clean(cwd: str) -> Result<bool, GitError>`
3. line 24 — `fn status_short(cwd: str) -> Result<str, GitError>`
4. line 29 — `fn status_long(cwd: str) -> Result<str, GitError>`
5. line 34 — `fn status_ignored(cwd: str) -> Result<[str], GitError>`
6. line 43 — `fn filter_by_status(items: [Item], kind: Kind) -> [Item]`
7. line 48 — `fn filter_by_pattern(items: [Item], pattern: str) -> [Item]`
8. line 72 — `fn count_by_kind(items: [Item]) -> (added: u32, deleted: u32, modified: u32)`
9. line 77 — `fn get_changed_files(items: [Item]) -> [str]`
10. line 82 — `fn has_changes(items: [Item]) -> bool`

The file already has 9 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/git/status.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/git/status.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 10 names above still exist: `grep -cE '^\s*(pub )?fn (status|is_clean|status_short|status_long|status_ignored|filter_by_status|filter_by_pattern|count_by_kind|get_changed_files|has_changes)\(' specs/git/status.t27` prints `10`
- 4. `grep -c '^\s*test "' specs/git/status.t27` prints at least `19` (today: 9)

## Boundary

specs/git/status.t27
