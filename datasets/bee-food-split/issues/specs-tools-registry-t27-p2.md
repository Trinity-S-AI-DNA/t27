# Implement part 2 of 3 of the empty bodies in specs/tools/registry.t27 (8 functions)

## Context

`specs/tools/registry.t27` holds 23 functions declared with a body that contains no statement. It is split into 3 issues because one turn cannot carry them all.

**All 3 parts name the same file, so they share one boundary and cannot run at the same time** — the swarm holds the file for whoever has it. They must be done in order, and the check below stops you if you arrived out of turn.

**`.t27` is the hand-authored source language.** `t27c` compiles it out to C, Rust, Verilog and Zig. You are writing source, not compiler output.

## Before you start

```
$ cd /Users/playom/t27
$ /Users/playom/t27/target/release/t27c gen specs/tools/registry.t27 2>&1 | grep -c 'not yet implemented'
15      # if it is not 15, this part is not yours yet — stop
```

## What to write

Keep every signature exactly as it is. Quoted verbatim:

1. line 54 — `fn list_dangerous(registry: ToolRegistry) -> [ToolID]`
2. line 63 — `fn set_permission(registry: ToolRegistry, toolID: ToolID, permission: ToolPermission) -> Result<void, ToolError>`
3. line 68 — `fn get_permission(registry: ToolRegistry, toolID: ToolID) -> Result<ToolPermission, ToolError>`
4. line 73 — `fn allow(registry: ToolRegistry, toolID: ToolID) -> Result<void, ToolError>`
5. line 78 — `fn deny(registry: ToolRegistry, toolID: ToolID) -> Result<void, ToolError>`
6. line 83 — `fn ask(registry: ToolRegistry, toolID: ToolID) -> Result<void, ToolError>`
7. line 92 — `fn register_custom(registry: ToolRegistry, tool: ToolDefinition) -> Result<void, ToolError>`
8. line 97 — `fn list_custom(registry: ToolRegistry) -> [ToolID]`

Add a `test` block for each function you implement.

## Acceptance criteria

- 1. `/Users/playom/t27/target/release/t27c gen specs/tools/registry.t27 2>&1 | grep -c 'not yet implemented'` prints `7` (on arrival: 15)
- 2. with criterion 1 satisfied, `/Users/playom/t27/target/release/t27c parse specs/tools/registry.t27 2>&1 | grep -E '^(recovery-events|declarations-swallowed|lexer-discarded-chars):'` reports 0, 0 and 0
- 3. with criterion 1 satisfied, all 8 names above still exist: `grep -cE '^\s*(pub )?fn (list_dangerous|set_permission|get_permission|allow|deny|ask|register_custom|list_custom)\(' specs/tools/registry.t27` prints `8`

## Boundary

specs/tools/registry.t27
