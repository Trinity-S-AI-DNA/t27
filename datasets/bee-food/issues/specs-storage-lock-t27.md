# Implement the 11 empty function bodies in specs/storage/lock.t27

## Context

`specs/storage/lock.t27` declares module `StorageLock` in 178 lines. 11 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/storage/lock.t27 2>&1 | grep -c 'not yet implemented'
11
$ ./target/release/t27c parse specs/storage/lock.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 29 — `fn acquire_read(key: str) -> Result<bool, StorageError>`
2. line 36 — `fn acquire_write(key: str) -> Result<bool, StorageError>`
3. line 41 — `fn release(key: str) -> Result<void, StorageError>`
4. line 47 — `fn try_lock(key: str, mode: LockMode, timeout_ms: u64) -> Result<bool, StorageError>`
5. line 52 — `fn is_locked(key: str) -> Result<bool, StorageError>`
6. line 57 — `fn get_lock_state(key: str) -> Result<LockState?, StorageError>`
7. line 62 — `fn wait_for_read(key: str) -> Result<void, StorageError>`
8. line 67 — `fn wait_for_write(key: str) -> Result<void, StorageError>`
9. line 76 — `fn cleanup_expired() -> Result<void, StorageError>`
10. line 81 — `fn get_all_locks() -> Result<[LockState], StorageError>`
11. line 86 — `fn force_release(key: str) -> Result<void, StorageError>`

The file already has 9 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/storage/lock.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/storage/lock.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 11 names above still exist: `grep -cE '^\s*(pub )?fn (acquire_read|acquire_write|release|try_lock|is_locked|get_lock_state|wait_for_read|wait_for_write|cleanup_expired|get_all_locks|force_release)\(' specs/storage/lock.t27` prints `11`
- 4. `grep -c '^\s*test "' specs/storage/lock.t27` prints at least `20` (today: 9)

## Boundary

specs/storage/lock.t27
