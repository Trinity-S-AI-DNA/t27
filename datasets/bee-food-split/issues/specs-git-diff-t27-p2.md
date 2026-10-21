# Implement part 2 of 2 of the empty bodies in specs/git/diff.t27 (7 functions)

## Context

`specs/git/diff.t27` holds 15 functions declared with a body that contains no statement. It is split into 2 issues because one turn cannot carry them all.

**All 2 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/git/diff.t27 2>&1 | grep -c 'not yet implemented'
7      # if it is not 7, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 68 — `fn get_files_changed(items: [Item]) -> [str]`
2. line 73 — `fn count_files_by_status(items: [Item]) -> (added: u32, deleted: u32, modified: u32)`
3. line 78 — `fn find_changes_in_path(items: [Item], path: str) -> [Item]`
4. line 83 — `fn find_changes_in_patterns(items: [Item], patterns: [str]) -> [Item]`
5. line 88 — `fn has_changes_in_path(items: [Item], path: str) -> bool`
6. line 107 — `fn parse_hunks(diff_text: str) -> Result<[Hunk], GitError>`
7. line 112 — `fn count_hunks(diff_text: str) -> u32`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/git/diff.t27 2>&1 | grep -c 'not yet implemented'` prints `0` (on arrival: 7)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/git/diff.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 7 names above still exist: `grep -cE '^\s*(pub )?fn (get_files_changed|count_files_by_status|find_changes_in_path|find_changes_in_patterns|has_changes_in_path|parse_hunks|count_hunks)\(' specs/git/diff.t27` prints `7`

## Boundary

specs/git/diff.t27
