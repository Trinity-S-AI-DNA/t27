# Implement part 3 of 3 of the empty bodies in specs/file/watcher.t27 (4 functions)

## Context

`specs/file/watcher.t27` holds 20 functions declared with a body that contains no statement. It is split into 3 issues because one turn cannot carry them all.

**All 3 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/file/watcher.t27 2>&1 | grep -c 'not yet implemented'
4      # if it is not 4, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 145 — `fn watch_batch(watcherID: WatcherID, paths: [str], recursive: bool) -> Result<void, FileError>`
2. line 150 — `fn unwatch_batch(watcherID: WatcherID, paths: [str]) -> Result<void, FileError>`
3. line 168 — `fn get_stats(watcherID: WatcherID) -> Result<WatcherStats, FileError>`
4. line 173 — `fn reset_stats(watcherID: WatcherID) -> Result<void, FileError>`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/file/watcher.t27 2>&1 | grep -c 'not yet implemented'` prints `0` (on arrival: 4)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/file/watcher.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 4 names above still exist: `grep -cE '^\s*(pub )?fn (watch_batch|unwatch_batch|get_stats|reset_stats)\(' specs/file/watcher.t27` prints `4`

## Boundary

specs/file/watcher.t27
