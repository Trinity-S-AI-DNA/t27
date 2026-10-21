# Implement the 10 empty function bodies in specs/tools/tri_to_t27_converter.t27

## Context

`specs/tools/tri_to_t27_converter.t27` declares module `TriToT27Converter` in 405 lines. 10 of its functions are declared with a body that holds no statement — a signature and a comment where the implementation belongs.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output, and not prose about an implementation.

## Current state — re-run these yourself

```
$ cd /Users/playom/t27
$ ./target/release/t27c gen specs/tools/tri_to_t27_converter.t27 2>&1 | grep -c 'not yet implemented'
10
$ ./target/release/t27c parse specs/tools/tri_to_t27_converter.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'
recovery-events: 0
declarations-swallowed: 0
lexer-discarded-chars: 0
```

Use `./target/release/t27c`, not the `t27c` on PATH: the PATH build prints no parse metrics at all, so the grep above returns nothing and an empty result reads as a pass.

## What to write

Keep every signature exactly as it is — the signature is the contract. Quoted verbatim from the file:

1. line 90 — `fn parse_tri_file(content: []const u8) -> TriSpec`
2. line 102 — `fn parse_type_line(line: []const u8) -> TriField`
3. line 110 — `fn convert_type(tri_type: []const u8) -> []const u8`
4. line 115 — `fn generate_t27(spec: TriSpec) -> []const u8`
5. line 128 — `fn route_file(source_path: []const u8) -> []const u8`
6. line 155 — `fn write_output(path: []const u8, content: []const u8) -> void`
7. line 162 — `fn run_migration(source_dir: []const u8, target_dir: []const u8) -> MigrationStats`
8. line 179 — `fn snake_to_pascal_case(name: []const u8) -> []const u8`
9. line 186 — `fn sanitize_description(desc: []const u8) -> []const u8`
10. line 191 — `fn is_valid_t27_syntax(content: []const u8) -> bool`

The file already has 0 `test` block(s). Add one for each function you implement; a body with nothing asserting on it is a claim, not a result.

## Acceptance criteria

- 1. `./target/release/t27c gen specs/tools/tri_to_t27_converter.t27 2>&1 | grep -c 'not yet implemented'` prints `0`
- 2. with criterion 1 satisfied, `./target/release/t27c parse specs/tools/tri_to_t27_converter.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 10 names above still exist: `grep -cE '^\s*(pub )?fn (parse_tri_file|parse_type_line|convert_type|generate_t27|route_file|write_output|run_migration|snake_to_pascal_case|sanitize_description|is_valid_t27_syntax)\(' specs/tools/tri_to_t27_converter.t27` prints `10`
- 4. `grep -c '^\s*test "' specs/tools/tri_to_t27_converter.t27` prints at least `10` (today: 0)

## Boundary

specs/tools/tri_to_t27_converter.t27
