# Implement the 10 empty function bodies in specs/account/repo.t27

## Context

`specs/account/repo.t27` declares module `AccountRepo` in 196 lines. 10 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/account/repo.t27 2>&1 | grep -c 'not yet implemented'
10
$ ./target/release/t27c parse specs/account/repo.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 74 — `fn active() -> Result<Info?, AccountError>`
2. line 77 — `fn list() -> Result<[Info], AccountError>`
3. line 80 — `fn remove(account_id: AccountID) -> Result<void, AccountError>`
4. line 83 — `fn set_active(account_id: AccountID, org_id: OrgID?) -> Result<void, AccountError>`
5. line 86 — `fn get_row(account_id: AccountID) -> Result<AccountRow?, AccountError>`
6. line 89 — `fn persist_token(input: PersistTokenInput) -> Result<void, AccountError>`
7. line 92 — `fn persist_account(input: PersistAccountInput) -> Result<void, AccountError>`
8. line 95 — `fn get_state() -> Result<AccountState, AccountError>`
9. line 98 — `fn set_state(state: AccountState) -> Result<void, AccountError>`
10. line 101 — `fn clear_active() -> Result<void, AccountError>`

The file already has 9 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/account/repo.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/account/repo.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 10 names above still exist: `grep -cE '^\s*(pub )?fn (active|list|remove|set_active|get_row|persist_token|persist_account|get_state|set_state|clear_active)\(' specs/account/repo.t27` prints `10`
- 4. `grep -c '^\s*test "' specs/account/repo.t27` prints at least `19` (today: 9)

## Boundary

specs/account/repo.t27
