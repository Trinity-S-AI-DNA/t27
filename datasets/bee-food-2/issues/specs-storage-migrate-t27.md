# Implement the 11 empty function bodies in specs/storage/migrate.t27

## Context

`specs/storage/migrate.t27` declares module `StorageMigrate` in 177 lines. 11 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/storage/migrate.t27 2>&1 | grep -c 'not yet implemented'
11
$ /Users/playom/t27/target/release/t27c parse specs/storage/migrate.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 34 — `fn version() -> Result<u32, StorageError>`
2. line 39 — `fn set_version(v: u32) -> Result<void, StorageError>`
3. line 44 — `fn apply(dir: str) -> Result<void, StorageError>`
4. line 49 — `fn apply_to(dir: str, target: u32) -> Result<void, StorageError>`
5. line 54 — `fn rollback(dir: str, target: u32) -> Result<void, StorageError>`
6. line 59 — `fn status(dir: str) -> Result<MigrationStatus, StorageError>`
7. line 64 — `fn register(step: MigrationStep) -> Result<void, StorageError>`
8. line 69 — `fn list_available() -> Result<[MigrationStep], StorageError>`
9. line 74 — `fn list_pending() -> Result<[MigrationStep], StorageError>`
10. line 83 — `fn migrate_1_legacy_projects(dir: str) -> Result<void, StorageError>`
11. line 89 — `fn migrate_2_session_diffs(dir: str) -> Result<void, StorageError>`

The file already has 7 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/storage/migrate.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/storage/migrate.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 11 names above still exist: `grep -cE '^\s*(pub )?fn (version|set_version|apply|apply_to|rollback|status|register|list_available|list_pending|migrate_1_legacy_projects|migrate_2_session_diffs)\(' specs/storage/migrate.t27` prints `11`
- 4. `grep -c '^\s*test "' specs/storage/migrate.t27` prints at least `18` (today: 7)

## Boundary

specs/storage/migrate.t27
