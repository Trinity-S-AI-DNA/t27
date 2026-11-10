// SPDX-License-Identifier: Apache-2.0
# OWNERS — specs/t27os/

## Primary

**S-Specs** — specs and standards, who owns ownership (letter 19, S). Owns the T27 OS instrument panel: 27 petals = 27 tools t27<letter>, glass_window, pty_server, queen, keys, evolve, domain_pulse, agent_parts, kanban, and the per-letter agent widgets under agents/. Default reviewer for structural changes in this tree; unowned paths in the panel go to S per specs/t27os/evolve.t27 line 17 ("unowned paths go to S").

## Dependencies

- specs/base/ — core types and operations SSOT for the language.
- specs/compiler/ — lexer, parser, types as `.t27` specifications (t27c lowers specs/t27os/*.t27 through this tree).
- specs/t27f/ — T27F frontend surfaces the panel visually (spec graph, outline, views).
- specs/queen/ — Queen scheduler and ring-packed event packing consumed by evolve.t27.

## Generates

- t27b builds specs/t27os/*.t27 to Mach-O objects for the T27 OS app.
- t27c emits gen/c, gen/zig, gen/rust, gen/verilog outputs for specs/t27os/*.t27 and its dependents.

## Notes

- Related: specs/t27os/agents/ (per-letter agent widgets), specs/t27f/ (T27F frontend), specs/queen/ (Queen scheduler).
- This directory had no OWNERS.md, making it an "unowned path" (G_UNOWNED) per specs/t27os/evolve.t27; the fix is this OWNERS.md (body_boundary = "OWNERS.md of the unowned paths", evolve.t27 line 236).
