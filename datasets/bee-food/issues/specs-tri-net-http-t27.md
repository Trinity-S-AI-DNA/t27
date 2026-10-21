# Implement the 7 empty function bodies in specs/tri/net/http.t27

## Context

`specs/tri/net/http.t27` declares module `TriHttp` in 132 lines. 7 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tri/net/http.t27 2>&1 | grep -c 'not yet implemented'
7
$ ./target/release/t27c parse specs/tri/net/http.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 36 — `fn method_to_string(method: HttpMethod) -> []const u8`
2. line 41 — `fn status_from_code(code: u16) -> HttpStatus`
3. line 46 — `fn is_success(code: u16) -> bool`
4. line 51 — `fn is_redirect(code: u16) -> bool`
5. line 56 — `fn is_client_error(code: u16) -> bool`
6. line 61 — `fn is_server_error(code: u16) -> bool`
7. line 66 — `fn parse_url(allocator: std.mem.Allocator, url: []const u8) -> !Url`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tri/net/http.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tri/net/http.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 7 names above still exist: `grep -cE '^\s*(pub )?fn (method_to_string|status_from_code|is_success|is_redirect|is_client_error|is_server_error|parse_url)\(' specs/tri/net/http.t27` prints `7`
- 4. `grep -c '^\s*test "' specs/tri/net/http.t27` prints at least `7` (today: 0)

## Boundary

specs/tri/net/http.t27
