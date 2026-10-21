# Implement part 2 of 2 of the empty bodies in specs/auth/config.t27 (5 functions)

## Context

`specs/auth/config.t27` holds 13 functions declared with a body that contains no statement. It is split into 2 issues because one turn cannot carry them all.

**All 2 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/auth/config.t27 2>&1 | grep -c 'not yet implemented'
5      # if it is not 5, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 121 — `fn to_well_known(info: Info) -> Result<WellKnown, AuthConfigError>`
2. line 126 — `fn from_oauth(oauth: Oauth) -> Info`
3. line 131 — `fn from_api(api: Api) -> Info`
4. line 136 — `fn from_well_known(well_known: WellKnown) -> Info`
5. line 141 — `fn get_type(info: Info) -> AuthType`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/auth/config.t27 2>&1 | grep -c 'not yet implemented'` prints `0` (on arrival: 5)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/auth/config.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 5 names above still exist: `grep -cE '^\s*(pub )?fn (to_well_known|from_oauth|from_api|from_well_known|get_type)\(' specs/auth/config.t27` prints `5`

## Boundary

specs/auth/config.t27
