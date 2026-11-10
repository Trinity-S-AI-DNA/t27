// SPDX-License-Identifier: Apache-2.0
# OWNERS — specs/t27f/

## Primary

**T-Queen** — T27F, t27's frontend for showing anything visual (spec diagrams,
map stills, particle reels, hot-reload stream, t27f graph), managed under the
Queen/Per-viewer domain per the `specs/OWNERS.md` subtree table
(`specs/t27f/` → **T-Queen**). Its spec surface lives in `specs/t27f/`:
`spec_graph.t27`, `frontend.t27`, `petal_menu.t27`, `spec_graph_export.t27`,
`spec_graph_derive.t27`, `spec_graph_view.t27`, `spec_graph_fold.t27`,
`spec_graph_outline.t27` and `spec_deps.t27` (modules `t27f_spec_graph`,
`t27f_spec_graph_export`, `t27f_spec_graph_derive`, `t27f_spec_graph_view`,
`t27f_spec_graph_fold`, `t27f_spec_graph_outline`, `t27f_spec_deps`).

## Dependencies

- `specs/t27os/` — T27 OS instrument panel surfaces that consume T27F views.
- `specs/queen/` — Queen orchestration whose views are drawn by T27F.
- `specs/compiler/` — `.t27` language front end whose specs T27F visualizes.

## Generates

T27F visual surfaces served via t27-mcp and app.t27.ai: graph pages
(`t27.ai/t27f/graph/`, Spec Explorer at app.t27.ai/game/specs), map stills,
particle reels and stills, hot-reload events (`t27f` events from
`node server.mjs --watch` in gHashTag/t27-mcp), and the `claw/t27f` check-run
(changed specs judged by their seals).

## Notes

- No hand-written code outside `.t27` specs (Only t27 rule): generation goes
  through `t27c` exporters (`spec_graph_export.t27`,
  `spec_graph_derive.t27`) — see `.claude/skills/t27f/SKILL.md` ("What T27F
  is").
- Every number shown by a T27F surface carries its source: measured, derived,
  or "not measured" — never 0 (T27 OS bench law).
