# Implement part 1 of 2 of the empty bodies in specs/git/diff.t27 (8 functions)

## Context

`specs/git/diff.t27` holds 15 functions declared with a body that contains no statement. It is split into 2 issues because one turn cannot carry them all.

**All 2 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/git/diff.t27 2>&1 | grep -c 'not yet implemented'
15      # if it is not 15, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 14 — `fn diff(cwd: str, ref: str) -> Result<[Item], GitError>`
2. line 19 — `fn diff_cached(cwd: str, ref: str?) -> Result<[Item], GitError>`
3. line 24 — `fn diff_files(cwd: str, ref: str, files: [str]) -> Result<[Item], GitError>`
4. line 29 — `fn diff_text(cwd: str, ref: str) -> Result<str, GitError>`
5. line 34 — `fn diff_file_text(cwd: str, ref: str, file: str) -> Result<str, GitError>`
6. line 43 — `fn stats(cwd: str, ref: str) -> Result<[Stat], GitError>`
7. line 48 — `fn stats_cached(cwd: str, ref: str?) -> Result<[Stat], GitError>`
8. line 58 — `fn stats_summary(cwd: str, ref: str) -> Result<StatsSummary, GitError>`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/git/diff.t27 2>&1 | grep -c 'not yet implemented'` prints `7` (on arrival: 15)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/git/diff.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (diff|diff_cached|diff_files|diff_text|diff_file_text|stats|stats_cached|stats_summary)\(' specs/git/diff.t27` prints `8`

## Boundary

specs/git/diff.t27
