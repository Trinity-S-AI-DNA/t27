# Implement part 1 of 3 of the empty bodies in specs/file/watcher.t27 (8 functions)

## Context

`specs/file/watcher.t27` holds 20 functions declared with a body that contains no statement. It is split into 3 issues because one turn cannot carry them all.

**All 3 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/file/watcher.t27 2>&1 | grep -c 'not yet implemented'
20      # if it is not 20, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 21 — `fn create() -> Result<WatcherID, FileError>`
2. line 26 — `fn watch(watcherID: WatcherID, path: str, recursive: bool) -> Result<void, FileError>`
3. line 31 — `fn unwatch(watcherID: WatcherID, path: str) -> Result<void, FileError>`
4. line 36 — `fn close(watcherID: WatcherID) -> Result<void, FileError>`
5. line 41 — `fn is_active(watcherID: WatcherID) -> Result<bool, FileError>`
6. line 46 — `fn get_watched_paths(watcherID: WatcherID) -> Result<[str], FileError>`
7. line 55 — `fn next(watcherID: WatcherID) -> Result<WatchEvent?, FileError>`
8. line 60 — `fn poll(watcherID: WatcherID) -> Result<[WatchEvent], FileError>`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/file/watcher.t27 2>&1 | grep -c 'not yet implemented'` prints `12` (on arrival: 20)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/file/watcher.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (create|watch|unwatch|close|is_active|get_watched_paths|next|poll)\(' specs/file/watcher.t27` prints `8`

## Boundary

specs/file/watcher.t27
