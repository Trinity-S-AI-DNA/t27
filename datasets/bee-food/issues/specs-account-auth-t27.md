# Implement the 9 empty function bodies in specs/account/auth.t27

## Context

`specs/account/auth.t27` declares module `AccountAuth` in 277 lines. 9 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/account/auth.t27 2>&1 | grep -c 'not yet implemented'
9
$ ./target/release/t27c parse specs/account/auth.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 15 — `fn token(account_id: AccountID) -> Result<AccessToken?, AccountError>`
2. line 20 — `fn refresh(account_id: AccountID) -> Result<AccessToken, AccountError>`
3. line 25 — `fn login(server: str) -> Result<Login, AccountError>`
4. line 30 — `fn poll(login: Login) -> Result<PollResult, AccountError>`
5. line 35 — `fn logout(account_id: AccountID) -> Result<void, AccountError>`
6. line 44 — `fn orgs(account_id: AccountID) -> Result<[Org], AccountError>`
7. line 55 — `fn orgs_by_account() -> Result<[AccountOrgs], AccountError>`
8. line 60 — `fn config(account_id: AccountID, org_id: OrgID) -> Result<[str: any]?, AccountError>`
9. line 75 — `fn get_user(account_id: AccountID) -> Result<UserInfo, AccountError>`

The file already has 14 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/account/auth.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/account/auth.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 9 names above still exist: `grep -cE '^\s*(pub )?fn (token|refresh|login|poll|logout|orgs|orgs_by_account|config|get_user)\(' specs/account/auth.t27` prints `9`
- 4. `grep -c '^\s*test "' specs/account/auth.t27` prints at least `23` (today: 14)

## Boundary

specs/account/auth.t27
