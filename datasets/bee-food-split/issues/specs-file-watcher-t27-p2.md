# Implement part 2 of 3 of the empty bodies in specs/file/watcher.t27 (8 functions)

## Context

`specs/file/watcher.t27` holds 20 functions declared with a body that contains no statement. It is split into 3 issues because one turn cannot carry them all.

**All 3 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/file/watcher.t27 2>&1 | grep -c 'not yet implemented'
12      # if it is not 12, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 65 — `fn wait(watcherID: WatcherID, timeout: u64) -> Result<WatchEvent?, FileError>`
2. line 83 — `fn set_filter(watcherID: WatcherID, filter: WatchFilter) -> Result<void, FileError>`
3. line 88 — `fn get_filter(watcherID: WatcherID) -> Result<WatchFilter, FileError>`
4. line 93 — `fn matches_filter(event: WatchEvent, filter: WatchFilter) -> bool`
5. line 110 — `fn get_backend() -> WatcherBackend`
6. line 115 — `fn backend_available(backend: WatcherBackend) -> bool`
7. line 131 — `fn set_debounce(watcherID: WatcherID, mode: DebounceMode, delay: u64) -> Result<void, FileError>`
8. line 136 — `fn get_debounce(watcherID: WatcherID) -> Result<(DebounceMode, u64), FileError>`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/file/watcher.t27 2>&1 | grep -c 'not yet implemented'` prints `4` (on arrival: 12)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/file/watcher.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (wait|set_filter|get_filter|matches_filter|get_backend|backend_available|set_debounce|get_debounce)\(' specs/file/watcher.t27` prints `8`

## Boundary

specs/file/watcher.t27
