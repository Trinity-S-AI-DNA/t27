# Implement part 1 of 2 of the empty bodies in specs/auth/config.t27 (8 functions)

## Context

`specs/auth/config.t27` holds 13 functions declared with a body that contains no statement. It is split into 2 issues because one turn cannot carry them all.

**All 2 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/auth/config.t27 2>&1 | grep -c 'not yet implemented'
13      # if it is not 13, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 67 — `fn get(provider_id: str) -> Result<Info, AuthConfigError>`
2. line 72 — `fn all() -> Result<[str: Info], AuthConfigError>`
3. line 77 — `fn set(key: str, info: Info) -> Result<void, AuthConfigError>`
4. line 82 — `fn remove(key: str) -> Result<void, AuthConfigError>`
5. line 87 — `fn exists(provider_id: str) -> Result<bool, AuthConfigError>`
6. line 92 — `fn clear() -> Result<void, AuthConfigError>`
7. line 111 — `fn to_oauth(info: Info) -> Result<Oauth, AuthConfigError>`
8. line 116 — `fn to_api(info: Info) -> Result<Api, AuthConfigError>`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/auth/config.t27 2>&1 | grep -c 'not yet implemented'` prints `5` (on arrival: 13)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/auth/config.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (get|all|set|remove|exists|clear|to_oauth|to_api)\(' specs/auth/config.t27` prints `8`

## Boundary

specs/auth/config.t27
