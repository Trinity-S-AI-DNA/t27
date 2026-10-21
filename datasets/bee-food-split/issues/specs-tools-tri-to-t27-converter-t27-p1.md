# Implement part 1 of 2 of the empty bodies in specs/tools/tri_to_t27_converter.t27 (8 functions)

## Context

`specs/tools/tri_to_t27_converter.t27` holds 10 functions declared with a body that contains no statement. It is split into 2 issues because one turn cannot carry them all.

**All 2 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/tools/tri_to_t27_converter.t27 2>&1 | grep -c 'not yet implemented'
10      # if it is not 10, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 90 — `fn parse_tri_file(content: []const u8) -> TriSpec`
2. line 102 — `fn parse_type_line(line: []const u8) -> TriField`
3. line 110 — `fn convert_type(tri_type: []const u8) -> []const u8`
4. line 115 — `fn generate_t27(spec: TriSpec) -> []const u8`
5. line 128 — `fn route_file(source_path: []const u8) -> []const u8`
6. line 155 — `fn write_output(path: []const u8, content: []const u8) -> void`
7. line 162 — `fn run_migration(source_dir: []const u8, target_dir: []const u8) -> MigrationStats`
8. line 179 — `fn snake_to_pascal_case(name: []const u8) -> []const u8`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/tools/tri_to_t27_converter.t27 2>&1 | grep -c 'not yet implemented'` prints `2` (on arrival: 10)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/tools/tri_to_t27_converter.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (parse_tri_file|parse_type_line|convert_type|generate_t27|route_file|write_output|run_migration|snake_to_pascal_case)\(' specs/tools/tri_to_t27_converter.t27` prints `8`

## Boundary

specs/tools/tri_to_t27_converter.t27
