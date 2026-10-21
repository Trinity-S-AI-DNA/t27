# Implement part 3 of 3 of the empty bodies in specs/tools/registry.t27 (7 functions)

## Context

`specs/tools/registry.t27` holds 23 functions declared with a body that contains no statement. It is split into 3 issues because one turn cannot carry them all.

**All 3 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/tools/registry.t27 2>&1 | grep -c 'not yet implemented'
7      # if it is not 7, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 102 — `fn unregister_custom(registry: ToolRegistry, toolID: ToolID) -> Result<ToolDefinition?, ToolError>`
2. line 111 — `fn register_all(registry: ToolRegistry, tools: [ToolDefinition]) -> Result<u32, ToolError>`
3. line 116 — `fn set_all_permissions(registry: ToolRegistry, permissions: [ToolID: ToolPermission]) -> Result<void, ToolError>`
4. line 121 — `fn clear(registry: ToolRegistry) -> void`
5. line 130 — `fn find_by_description(registry: ToolRegistry, pattern: str) -> [ToolID]`
6. line 135 — `fn count(registry: ToolRegistry) -> u32`
7. line 140 — `fn is_empty(registry: ToolRegistry) -> bool`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/tools/registry.t27 2>&1 | grep -c 'not yet implemented'` prints `0` (on arrival: 7)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/tools/registry.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 7 names above still exist: `grep -cE '^\s*(pub )?fn (unregister_custom|register_all|set_all_permissions|clear|find_by_description|count|is_empty)\(' specs/tools/registry.t27` prints `7`

## Boundary

specs/tools/registry.t27
