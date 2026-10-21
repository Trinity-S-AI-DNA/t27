# Implement part 1 of 3 of the empty bodies in specs/tools/registry.t27 (8 functions)

## Context

`specs/tools/registry.t27` holds 23 functions declared with a body that contains no statement. It is split into 3 issues because one turn cannot carry them all.

**All 3 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/tools/registry.t27 2>&1 | grep -c 'not yet implemented'
23      # if it is not 23, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 14 — `fn create() -> ToolRegistry`
2. line 19 — `fn register(registry: ToolRegistry, tool: ToolDefinition) -> Result<void, ToolError>`
3. line 24 — `fn unregister(registry: ToolRegistry, toolID: ToolID) -> Result<ToolDefinition?, ToolError>`
4. line 29 — `fn get(registry: ToolRegistry, toolID: ToolID) -> Result<ToolDefinition?, ToolError>`
5. line 34 — `fn has(registry: ToolRegistry, toolID: ToolID) -> bool`
6. line 39 — `fn list(registry: ToolRegistry) -> [ToolID]`
7. line 44 — `fn list_by_category(registry: ToolRegistry, category: ToolCategory) -> [ToolID]`
8. line 49 — `fn list_allowed(registry: ToolRegistry) -> [ToolID]`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/tools/registry.t27 2>&1 | grep -c 'not yet implemented'` prints `15` (on arrival: 23)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/tools/registry.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (create|register|unregister|get|has|list|list_by_category|list_allowed)\(' specs/tools/registry.t27` prints `8`

## Boundary

specs/tools/registry.t27
