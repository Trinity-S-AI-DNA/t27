# T27 OS agent homes: what each of the 27 petals shows today

Issue: Trinity-S-AI-DNA/t27#13864 (child of epic #13862), milestone M0. Read-only inventory.

- Read at SHA: 9fbcebdab0 (branch claude/agent-homes-27-petals-ae065e)
- Date: 2026-11-15
- Counts (from the tree): dedicated 8 (B G H K L M Q S), window 1 (C), stub 0 established, generic 18
  (A D E F I J N O P R T U V W X Y Z Ti). Total 27.
- Owner's figures versus the tree:
  - "8 dedicated pages B G H K L M Q S": HOLDS. `page_kind` returns a page for exactly letters 2, 7, 8, 11, 12, 13, 17, 19
    (specs/t27os/agent_parts.t27:1048-1057); test at agent_parts.t27:1078 names the same eight.
  - "CHAT (petal 3, C) is the window itself": HOLDS. `page_kind(3)` is PAGE_NONE; petal PETAL_CHAT draws `draw_chat`
    (specs/t27os/glass_window.t27:12409-12410), and `draw_home` prints "< CHAT" for it (glass_window.t27:5209).
  - "17 letters share one generic page" plus "FRONTEND (F) is a stub": the count of 18 non-dedicated, non-window letters
    holds (17 + F). That F is a STUB is NOT established: F has no page of its own (`page_kind(6)` = PAGE_NONE) and falls
    to the same generic page as the other 17; its artifact is specs/t27f/frontend.t27 (agent_parts.t27 TOOL_ARTIFACTS).
    Nothing read in the tree labels F "stub". Here F is counted as generic.
- Difference named: stub 0 (not 1), generic 18 (not 17), because F is not marked as a stub anywhere read.

Conventions: Home = AGENT_HOME index in specs/t27os/homes.t27:100-104 mapped through HOME_IDS (homes.t27:58). Realm =
petal / 9 (specs/t27f/petal_menu.t27:41, REALM_NAMES line 127: RAZUM 1-9, MATERIYA 10-18, DUKH 19-27). Names are
WORLD_NAMES (petal_menu.t27:123). Draw-fn line numbers are glass_window.t27 unless stated (G = specs/t27os/glass_window.t27).

| # | Letter | Name | Realm | Home | Current page | Draw fn (file:line) | Data source | Tests | Gaps | Priority |
|---|--------|------|-------|------|--------------|---------------------|-------------|-------|------|----------|
| 1 | A | ISA | RAZUM | queen | generic | G:5159 draw_home (dispatch G:5211) | NOT MEASURED | 0 (no page spec) | no own page; artifact specs/isa/t27a.t27 | P2 |
| 2 | B | BENCH | RAZUM | t27c | dedicated | G:7267 draw_bench_page (via G:10641) | ~/.t27os/bench/run.txt (G:7432) + bench scan feed (bench_tick G:7071, 10 s) | 29 (specs/t27os/bench.t27) | contract alignment NOT MEASURED | P0 |
| 3 | C | CHAT | RAZUM | t27c | window | G:12409 draw_chat (chat window; generic dashboard behind switch, G:5159) | PTY server ~/.t27os/pty.sock (G:5883); sessions | NOT MEASURED | page is the window, not an agent home | P2 |
| 4 | D | HEALTH | RAZUM | t27o | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact specs/t27mcp/dashboard.t27 | P2 |
| 5 | E | EXPERIENCE | RAZUM | queen | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact .trinity/experience | P2 |
| 6 | F | FRONTEND | RAZUM | t27c | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; "stub" not established; artifact specs/t27f/frontend.t27 | P2 |
| 7 | G | AGENTS | RAZUM | t27f | dedicated | G:10024 draw_agents_page (via G:10646) | swarm feed ~/.t27os/hive/feed.txt, devices.t27 lane rows (dv_lane_field G:10024 region; agent_parts.t27 comment) | 30 (specs/t27os/devices.t27, shared with H) | own-spec contract NOT MEASURED | P1 |
| 8 | H | HIVE | RAZUM | t27t | dedicated | G:10254 draw_hive_page (via G:10644) | ~/.t27os/hive/feed.txt (G:10718-10719; hive_tick G:8764, 60 s) | 30 (specs/t27os/devices.t27) | contract alignment NOT MEASURED | P1 |
| 9 | I | ISSUES | RAZUM | t27a | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact MCP t27_issues, tri issues | P2 |
| 10 | J | JOBS | MATERIYA | queen | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact specs/automation | P2 |
| 11 | K | KANBAN | MATERIYA | t27b | dedicated | G:7819 draw_kanban_page (via G:10642) | kanban.t27 loader kn_load (kanban_tick G:7635, 60 s); file path NOT MEASURED | 17 (specs/t27os/kanban.t27) | contract alignment NOT MEASURED | P1 |
| 12 | L | LOGS | MATERIYA | t27c | dedicated | G:8874 draw_logs_page (via G:10645) | ~/.t27os/loop/log.txt, tail 6 KB (G:8884), 30 s | 0 (no own spec; logic in glass_window.t27) | no own spec or test block | P1 |
| 13 | M | MAP | MATERIYA | queen | dedicated | G:9921 draw_gnn_page (via G:10648) | FIXTURE in specs/t27f/gnn_work_graph_viz.t27 (17 nodes, 17 edges) | 57 (specs/t27f/gnn_work_graph_viz.t27) | reads a fixture, not live data | P1 |
| 14 | N | PULSE | MATERIYA | t27t | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact docs/now, domain_pulse.t27 (11 tests, not wired to a page) | P2 |
| 15 | O | OUROBOROS | MATERIYA | queen | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact specs/tools/t27o | P2 |
| 16 | P | PULLS | MATERIYA | queen | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact tri pr, specs/claw | P2 |
| 17 | Q | QUEEN | MATERIYA | queen | dedicated | G:8515 draw_queen_page (via G:10643) | queen_board.t27 (qb_measured) + files under ~/.t27os queen dir (queen_tick G:7934, 60 s; QUEEN_DIR links/tests files) | 19 (specs/t27os/queen_board.t27) | contract alignment NOT MEASURED | P0 |
| 18 | R | ROADMAP | MATERIYA | t27c | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact HUD views research, roadmap | P2 |
| 19 | S | SPECS | DUKH | queen | dedicated | G:9129 draw_specs_viz_page (via G:10647) | FIXTURE tutorial_08_graph() of specs/t27f/spec_graph.t27 (15 nodes, 16 edges) via specs/t27f/petal_specs_viz.t27 | 84 (specs/t27f/petal_specs_viz.t27) | reads a fixed tutorial graph, not the open spec | P0 |
| 20 | T | TRAINER | DUKH | queen | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact specs/t27t/t27t.t27 | P2 |
| 21 | U | UNIVERSITY | DUKH | queen | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact the 7 courses in homes.t27 | P2 |
| 22 | V | VERDICTS | DUKH | t27c | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact t27c verdict, tri verdict | P2 |
| 23 | W | WIDGETS | DUKH | queen | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact specs/t27f/widget_factory.t27 | P2 |
| 24 | X | EXTERNAL | DUKH | t27o | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact HUD views providers, specs/t27os/keys.t27 | P2 |
| 25 | Y | YIELD | DUKH | queen | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact specs/depin/prove.t27 | P2 |
| 26 | Z | DOCS | DUKH | t27u | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact docs/, specs/blog | P2 |
| 27 | Ti | SECURITY | DUKH | queen | generic | G:5159 draw_home | NOT MEASURED | 0 (no page spec) | no own page; artifact specs/policy/secret_gate.t27 | P2 |

## Shared generic page

- Selection: `draw_home` (specs/t27os/glass_window.t27:5159). For a letter `o`, `page_kind(o)` (agent_parts.t27:1048) is
  read at glass_window.t27:5211; when it is not PAGE_NONE (and the petal is not CHAT) `draw_tool_page`
  (glass_window.t27:10635, dispatch lines 10641-10648) draws the dedicated page. Otherwise, and also behind the
  "DASHBOARD >" switch of a dedicated page (glass_window.t27:5217-5221), the generic body of `draw_home` draws.
- What it shows (glass_window.t27:5240 onward): kicker "LETTER n OF 27 // REALM r // <home name>", an identity card
  (`draw_identity`: glyph, name, domain from `agent_domain` homes.t27:404), four gauges (home id, widgets count,
  surfaces count, loaded ratio), a segment bar, then a console block: the letter, its domain, a tile grid of the letter's
  instruments (widgets and surfaces) and its pinned widgets, plus a "+" tile. Letter C additionally gets a 2 x 2 chat
  block (BIG_LETTER = 3, dash.t27).
- What it reads: the instrument plan `plan(o)` = `parts_of_letter(o)` (specs/t27os/dash.t27:246), widgets and surfaces
  of the letter (homes.t27 tables), pins from ~/.t27os/pins (glass_window.t27:13507), widget load state
  (`dc[D_ST + widget]`) and the gallery cache. It reads no per-letter live feed.
- Spec tests behind it: specs/t27os/dash.t27 has 36 test blocks; agent_parts.t27 has 24.
- Number of instruments per letter: NOT MEASURED (would need evaluating `parts_of_letter` for each letter).

## Not established

- Whether F (FRONTEND) is a "stub" in the owner's sense: the tree shows only that it has no dedicated page.
- Per-letter instrument counts on the generic page (`widgets_of_letter`, `surfaces_of_letter` values) and which letters
  have empty consoles.
- Kanban page's input file path (kn_load) and G (AGENTS) page's exact feed file: read from comments only, not traced
  through the loaders.
- Test counts for the host draw code in glass_window.t27 itself (whole-file, not per page), and any visual or
  screenshot evidence of the pages. Nothing was run; this inventory is from static reading.
- Whether the dedicated pages meet the planned home contract of epic #13862 (no contract text was read; "Gaps"
  for P0/P1 pages are NOT MEASURED against it).
- specs/t27os/agents.t27 (4 tests) describes the coding-agent honeycomb; it was not shown to be the data source of
  page G, so it is not counted for G.
