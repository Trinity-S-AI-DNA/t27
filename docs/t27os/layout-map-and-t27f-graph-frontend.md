# T27 OS layout map and T27F as a representation factory

Status: research and source audit, 2026-11-03. Nothing here is built or wired by this document. Refs #12779.

This file answers three questions in one place:

1. Which files decide layout and markup (verstka) of the T27 OS window, and where each decision lives.
2. What the T27F issues on GitHub say T27F is, what was delivered and what is missing.
3. How T27F becomes a smart graph frontend -- reframed by the owner on 2026-11-03 as a **factory of
   representations from a typed model**, not a set of hand-drawn screens -- after a source audit of what
   already exists.

The source audit (section 2) comes first because the owner's brief says to audit before building and to create
no duplicates.

---

## 0. Method and honesty rules

- Read-only pass over the worktree on branch `lane/t27os-dsh-panel` (HEAD `c9778a8567`). Paths are relative to the
  repository root. Counts: L = lines, T = `test` blocks. Line numbers drift while other writers edit; re-grep a name
  before relying on a line.
- `specs/t27os/glass_window.t27` is about 18269 lines and was never read whole: names were found with grep and read by
  line range.
- GitHub facts come from `gh` (titles, bodies, comments). "Delivered" means a merged PR names it; test counts are the
  PR's own claim and were not re-run here.
- Competitor facts come from vendor docs, repositories and release pages. A missing fact is marked UNVERIFIED, never
  filled from memory.
- The four research works the owner named (SUPPLE, Penrose, Vega-Lite, Draco) are summarised from their published
  abstracts as the owner cited them; they were not re-fetched in this pass.
- `find specs -name '*.t27'` counts 2944 files. That is corpus size, not the number of objects any generator supports.
- Labels used below: EXISTS (a spec with tests defines it), PARTIAL (it exists for one domain or one screen only),
  MISSING (no spec found), UNVERIFIED (not confirmed in a source read here).

---

## 1. The direction (owner, 2026-11-03, translated)

- We drifted from the goal into hand-drawing particular screens. What is needed is a **factory** of interfaces: a
  typed model goes in, representations come out by rule.
- The Observatory is a checking layer for what the factory produces. Adding three fields to the hand-drawn queen
  dashboard fixes one screen and nothing else.
- Agent instruction: change focus. T27F is a factory of representations from a typed model, not a set of hand-made
  screens. First run a source audit of the existing model / graph / view / layout / style / scene / interaction
  mechanisms and show paths, symbols, tests and gaps. Create no duplicates. Use Observatory A+ as a test of the
  shared pipeline, not as an extension of the hand-drawn dashboard. First result: one actor model -> three
  representations -> hot design switch -> one shared Scene IR -> render / hit-test / UiSnap. Then check reuse on
  specs, tests and issues. Implement everything in `.t27`, using published research ideas, without linking
  third-party UI libraries.
- The main change of plan: **the unit of development is a rule that turns an object into a representation, not a
  screen with elements placed in advance.**
- The owner marked their own table of "what exists" as not checked against the source. Section 2 replaces it.

Target pipeline as the owner drew it:

```
idea / .t27 / runtime events
  -> typed model                 (objects, kinds, labels, relations)
  -> intent                      (what the person wants to see or do)
  -> view grammar + design policy
  -> candidate generator + constraint check (hard filters, soft scores)
  -> layout / Scene IR
  -> render + interaction        (draw, hit-test, focus)
  -> Observatory                 (UiSnap from the same IR, pixel check, independent acceptance)
```

Five separate entities: model, intent, representation, layout, design. Changing the design leaves the data and the
representation alone; changing the representation keeps object identity. An AI may propose a configuration; what
it proposes stays labelled `suggested` until a rule or a gate accepts it.

---

## 2. Source audit: the chain, stage by stage

Answer in one line: **the chain "object type -> possible representations -> constraints -> composition -> geometry
-> interactive UI" exists only in fragments.** Every stage has some spec, but the stages are not joined, and the
native window joins none of them. The work is to extend these pieces, not to write a second set.

### 2.1 Table

| Stage | State | Where (path :: symbol) | Tests | Gap |
|---|---|---|---|---|
| Typed model, actors | EXISTS | `specs/t27f/work_graph.t27` :: `WorkGraph`, `apply_event` (186), `graph_digest` (316), `NODE_CAP` 2048, `EDGE_CAP` 8192; nodes SPEC/AGENT/TASK/ACTION/ARTIFACT, edges READS/ASSIGNED/PERFORMS/PRODUCES/VERIFIES/BLOCKS, spec_graph's 4 labels | 9 | Input is the replay of `specs/queen/actor_events.t27` plus tasks; no shared object contract with spec_graph |
| Typed model, specs | EXISTS | `specs/t27f/spec_graph.t27` :: `SpecGraph`, `MAX_NODES` 1024 (119), edges to 4096; labels exact / derived / suggested / unknown | yes | No producer of SUGGESTED (derive pass yields DERIVED and UNKNOWN only) |
| Common object contract | MISSING | units, actions, permissions appear only in `specs/t27f/ui_plan.t27` bindings | - | Two models, two kind enums, no shared "semantic object" record |
| Intent | PARTIAL | `specs/t27f/frontend.t27` :: `route(intent, host)` (154): 12 intents `I_*` -> 15 surfaces `SF_*`; `UiPlan.intent` is free text | 10 | Intent picks a surface (where), never a representation (what shape) |
| Representation vocabulary | EXISTS twice | `specs/t27f/ui_plan.t27` :: 6 `WIDGET_*`; `specs/t27f/viz_contract.t27` :: 13 `VIZ_REP_*` (`VIZ_REP_COUNT` 403: GRAPH, MATRIX, LIST, SOURCE, TRACE, INSPECTOR, STEPPER, CHOICE, BADGE, TIMELINE, CHART, MAP, TEXT) | 9 + 41 | Not joined; viz_contract's header says folding them is "a later catalog version, not a second catalog" (`CATALOG_VERSION` 1) |
| Type -> representation rule | PARTIAL (one domain) | `specs/t27f/spec_graph.t27` :: `node_widget(kind, label)` (484): a hand if-chain, MODULE/USE -> honeycomb card, CONST/FN -> formula view, TEST/INVARIANT -> run button, BENCH -> metrics table, SUGGESTED/UNKNOWN -> text block | yes | The only such rule in the corpus; `specs/t27f/petal_viz_plan.t27` :: `pvp_reps` (226) assigns reps per petal by hand, not from a type |
| Hard constraints | EXISTS | `ui_plan.t27` :: `validate_plan` (271) (catalog allowlist, `FORBIDDEN_NAMES`, caps, bindings, a11y labels, 12 x 8 grid overlap, `MAX_WIDGETS` 8); `scene.t27` :: `validate_scene` (243); `display_list.t27` :: `MAX_OPS_PER_FRAME` 128; `viz_contract.t27` :: `vh_validate` (555); `specs/t27os/ui_snapshot.t27` rules R1-R8 | many | Each validator guards its own IR; none ranks |
| Soft constraints / ranking | MISSING | grep for score / cost / weight finds only reel, lesson and still files | - | No way to prefer one valid representation over another |
| Composition | PARTIAL | `ui_plan.t27` grid 12 x 8 with spans set by hand; `viz_contract.t27` :: `VizHost` 16 nested slots, `vh_add` (486) with rects given by hand | - | ui_plan's header: "no solver turns an intent into a plan" |
| Geometry (layouts) | EXISTS, scattered | `spec_graph_view.t27` (column per kind, label-driven size and dash, UNKNOWN band); `graph_design.t27` :: `gd_mycelium_pos` (144), `gd_map_pos` (173); `specs/t27os/work_map.t27`; `specs/t27os/queen_layout.t27` (force, `QL_REPULSION` 2600); `queen_live_graph.t27` (bands); `queen_graph_plane.t27` (rows) | 10, 10, 20, 6, 6, 7 | Not behind one interface; two force layouts (queen_layout and the work-map mycelium) |
| Transitions | EXISTS, separate | `specs/t27os/graph_transition.t27` :: modes STATIC / SMOOTH / STAGED, `trs_tick` (129), event coalescing | 15 | Not wired to the window |
| Design policy | PARTIAL | `specs/t27os/hot_reload.t27` :: live table `THV_*` (15 values in permille: glass alphas, white and black levels) applied from a live `.t27` module without restart; `THEME_DARK` / `THEME_LIGHT` (400) as Core Image filters on the window's one layer (owner, 2026-11-02); `ui_plan.t27` :: `THEME_*` ids | 12 | Covers materials and a whole-window inversion. Graph palettes live elsewhere: `viz_contract.t27` `VIZ_*_RGB`, `graph_design.t27` state colours, `queen_live_graph.t27` palette, `specs/t27f/petal_glass.t27`, glass_window constants. No policy object the generator consults (density, label size, edge style, palette tokens) |
| Scene IR | EXISTS, small | `specs/t27f/scene.t27` :: 7 primitives, 5 style tokens, 8 fields, `apply_op` (189), `MAX_NODES` 32 ("a HUD, not a canvas"); `display_list.t27` :: 13 `DL_*` opcodes, `flatten_into` (178), pinned `HUD_SHA256` | 8 + 6 | Capped for a HUD; the native window does not import it (0 `use t27f::scene`) |
| Render | hand-drawn | `specs/t27os/glass_window.t27` :: `draw()` at its 64-parameter limit, plus `draw_*` per screen | 105 | Draws from its own rect maths, not from an IR |
| Hit-test and focus | EXISTS, unused | `viz_contract.t27` :: `vh_hit` (591), focus next/prev, `viz_apply` reducer (257) | 41 | 0 calls of `viz_apply`, `vh_hit` or `vh_add` in glass_window; the window's hit-tests are per-screen if/else fns |
| UiSnap (Observatory) | contract only | `specs/t27os/ui_snapshot.t27` (772 L committed): rules R1-R8, "no window fills a UiSnap yet" | 15 | No producer; it states the window has no component tree |

What glass_window imports from `specs/t27f/` today: `viz_contract` 22 lines (palette, truth colours, refs, VizHost
type), `work_graph` 9 lines, `spec_graph` 1 line. It imports nothing from `ui_plan`, `scene`, `display_list` or
`graph_design`.

### 2.2 What this means for the factory

- The model exists twice (actors, specs) and is good: typed, labelled, pure, digest-pinned. The factory should read
  both through one small object contract, not replace either.
- The representation vocabulary exists twice. The rule says one catalog: merge `WIDGET_*` and `VIZ_REP_*` as
  `CATALOG_VERSION` 2 inside the existing files, as viz_contract already announces.
- The rule "type x intent -> representation" exists once, as `node_widget`. It is the seed: lift its if-chain into a
  rule table, and make `node_widget` read that table so there is one home.
- Hard constraints exist; soft ones do not. Soft scores are new work.
- The Scene IR exists but is sized for a HUD. Raising its caps (or a version 2 of the same vocabulary) is the way to
  a canvas; a second scene format would be a fork.
- Hit-test and focus already exist in VizHost and are pure. The window should call them instead of growing more
  if/else chains.
- The hot design switch exists at the material and whole-window level. The design policy the generator needs sits
  next to it (same live-module path), not in a new mechanism.

---

## 3. Proposal: the first factory slice

Unit of work: a **rule**, tested in `.t27`, that turns an object of a type, under an intent and a design policy, into
a representation candidate. A screen is the output of rules, never the unit.

### 3.1 First result (owner's order), mapped onto existing specs

| Step | Uses (exists) | New, small, in the existing home where possible |
|---|---|---|
| 1. One actor model | `work_graph.t27` from the `actor_events.t27` replay | an object view over `WorkGraph` nodes: kind, label, state, identity (no copy of the data) |
| 2. Three representations | `VIZ_REP_GRAPH`, `VIZ_REP_TIMELINE` (message sequence from deliver records, ordered by seq), `VIZ_REP_CHART` (load from the kernel mailbox counts in `specs/t27os/kernel.t27`) | a rule table: (object kind, intent) -> allowed reps, with a hard filter and a soft score per row; the exact event and mailbox fields are to be confirmed when the slice reads them |
| 3. Hot design switch | `hot_reload.t27` live table and theme filter | a design-policy record (palette tokens per state and truth label, label size, density, edge style) loaded through the same live module path; two policies in tests |
| 4. One Scene IR | `scene.t27` + `display_list.t27` | raise caps or version the vocabulary so a graph of the slice's size fits; identity of an object maps to a stable scene node id |
| 5. Render / hit-test / UiSnap from that IR | `display_list.flatten_into`, `viz_contract.vh_hit`, `ui_snapshot.t27` | one host consumer in the window that paints the display list; hit-test answered from the same scene rects; a UiSnap filled from the same IR |
| 6. Reuse check | `spec_graph.t27`, `spec_graph_view.t27` | run the same rule table on specs / tests / issues: `node_widget` becomes rows of the table |

Checks the slice must pass, as tests, before any picture is shown:

- Adding an object to the model changes the output only through the rules (no hand edit of a screen).
- Switching the design policy leaves the model digest and the chosen representations unchanged.
- Switching the representation keeps every object's identity (same scene node id, same selection).
- The same model, intent and policy give the same Scene IR digest twice.
- Hard constraints refuse an invalid candidate with a named error code; a soft score orders the valid ones.
- Negative control: a rule that would place a SUGGESTED fact as EXACT is refused.

A shared IR does not prove the pixels are right. The Observatory still needs a pixel check of the real window and an
acceptance by someone other than the author of the rule.

### 3.2 After the first slice

- Fold `petal_viz_plan.t27` hand-assigned reps into rule rows (a petal asks for an intent, the rules answer).
- Put the graph palettes (`VIZ_*_RGB`, `graph_design` state colours, `queen_live_graph` palette) behind the
  design-policy record; keep the values, move the home.
- Put the layouts behind one geometry interface (columns, layered map, mycelium spiral, force) chosen by rule, with a
  seed and layout version in the IR so a layout is reproducible.
- Wire `graph_transition.t27` between two Scene IR states (identity-preserving moves come from step 4).
- Window debts that block a clean consumer: `draw()` at 64 parameters, screen state as loose bools, two dock widths
  (section 4.4).

### 3.3 Capabilities the factory will need (requests, not work done here)

- Pan, zoom and scroll input reaching the spec (host externs), so navigation lives in `.t27`.
- Level of detail and an idle-redraw fix: whole-window fills measured at 34-78% idle CPU (t27#12410).
- gen-js host fns for a browser host without hand JS (t27#10295).
- A native smoke test for ObjC calls that t27c and t27b cannot verify (PR 13135).
- The TRI CLAW `/mcp` mount of the t27 tools (t27#10554, 999#4015), so chat-side drawings come from the same IR.

### 3.4 Anti-goals

- No third-party UI library linked into T27 OS; reuse algorithms and open formats only (DOT, JSON Canvas, ELK JSON).
- No second catalog, second scene format or second force layout.
- No new hand-drawn screen as a deliverable; Observatory A+ is a test of the pipeline.
- No AI-made structure shown as fact: it stays `suggested` until a rule or gate accepts it.

---

## 4. T27 OS layout and markup inventory

Frame: one native macOS window built by t27b into an arm64 Mach-O (`Makefile` target `t27os`); AppKit,
CoreGraphics and CoreText through `@extern`; a 2x backing bitmap, scene point = view point; one `draw()` paints
everything; hit-tests are pure fns over the same scene numbers. No font, icon or theme asset files exist; all type is
system fonts through CoreText. `gen/` holds no t27os output.

All paths in 4.1-4.3 are under `specs/t27os/` unless a prefix is given.

### 4.1 The host and the layout law

| Path | L | T | Role | Geometry-deciding names |
|---|---|---|---|---|
| glass_window.t27 | ~18269 | 105 | host + layout law + hit-test + draw + theme in one module; ~78 `@extern` lines | W_PT 960, H_PT 540, TITLE_H 34, AREA_TOP 35, WS_TOP 35, SIDE_W 272, COMPACT_BELOW 760, PANEL_W 400, SW_MAX 1920, SH_MAX 1200, CH_MAX 1300, BAR_H 26, HEADER_H 44; fns `on_glass`, `scene`, `workspace`, `content_right`, `draw`, `main` |
| layout.t27 | 90 | 4 | row helper: next x, centre line, baseline, pixel snap | CAP_SHARE .70, ROW_ICON_H 11, ROW_GAP 8 |
| shell_dock.t27 | ~840 | 23 | Queen right dock + bottom terminal dock as shell law; push vs over from the target width; animation | SD_CENTER_MIN_W 480, SD_DUR .24, SD_RIGHT_MIN/DEFAULT/MAX 280/360/560, bottom MIN/DEFAULT/FRAC 120/260/.6; `sd_sync`, `sd_layout`, `sd_settled`, `sd_chat_rects`, `sd_term_rect`, `sd_ctx_*` |
| queen.t27 | 707 | 12 | Queen right panel geometry and hit-tests | QP_W 300, QP_GOD_H 44, QP_BTN_H 26; `qp_left`, `on_panel` |
| queen_chats.t27 | 284 | 5 | chat column inside the Queen dock | RC_TERM_TOP 44, RC_IN_H 40, QC_W 84 |
| queen_layout.t27 | 258 | 6 | force layout for the Queen graph | QL_REPULSION 2600, QL_SPRING_REST 70 |
| windows.t27 | 503 | 7 | agent-window rows in the side column | ROW_H 28, AGENT_ROW_H 38 |
| statusbar.t27 | 860 | 12 | status bar and header strip hit-tests | SB_H 24, STRIP_H 16 |
| term_pane.t27 | 824 | 21 | terminal grid from a rect | TP_FONT 12, TP_CELL_W 7.22, TP_LINE_H 15, cols 20..400, rows 4..120 |
| window_keep.t27 | 146 | 5 | window frame kept between runs | WK_OLD_W/H 960 x 540 |
| frame_gate.t27 | 299 | 13 | when a frame is drawn | FG_IDLE_S .5, FG_ANIM_S 1/60 |
| text_cache.t27 | 82 | 4 | cache of shaped CoreText lines | |
| hot_reload.t27 | ~460 | 12 | binary swap, live THEME table, dark/light filter | THV_* (15), THEME_DARK/LIGHT |

### 4.2 Screens and panels (layout + hit-test per screen; drawing stays in the window)

dash.t27 (1871 L, 38 T, widget-tree packer), homes.t27 (1190, 27), agent_parts.t27 (1212, 24), pins.t27 (433, 8),
kanban.t27 (974, 16), lanes_view.t27 (1289, 34), orbit_view.t27 (768, 12), devices.t27 (1730, 30), queen_dash.t27
(758, 8), queen_board.t27 (1194, 18), bench.t27 (1649, 28), profile.t27 (1832, 32), doctor.t27 (801, 20), keys.t27
(3981, 115), release.t27, ui_snapshot.t27 (772, 15), executor_registry.t27 (window imports one fn), work_map.t27
(1113, 20, not imported by the window), graph_transition.t27 (431, 15), graph_k_hop.t27 (313, 8),
queen_messages.t27 (318, 6), queen_graph_plane.t27 (307, 7), queen_live_graph.t27 (257, 6), live_tap / live_data /
live_petal_plan / pty_live_graph (feed plumbing). Terminal: vt.t27 (1238, 26), term_input.t27 (1136, 31),
pty_client / pty_proto / pty_sys (no geometry). Host bridges with `@extern` (no layout decisions): profile_host,
keys_host, keysync_host, gh_signin_host, editors_host, pty_sys.

Shared vocabulary in `specs/t27f/` that the window imports: petal_menu (27 petals, 9 per realm), petal_glass (glass
constants), gnn_work_graph_viz (GNN_FRAME 1900 x 1000), petal_specs_viz, viz_contract, formula_orbit, work_graph.
Not imported by the window: ui_plan, scene (+ the scene_* family), frontend, spec_graph_view, frame_render (5 x 7
raster font), petal_viz_plan, live_glue_layout, graph_design.

Docs and skills: `docs/t27os/run-local.md`, `docs/nona-03-manifest/VIZ-PETAL-GUIDE.md`, `.claude/skills/t27os/SKILL.md`
(matte glass rule, petal table, home-logo geometry), `.claude/skills/t27f/SKILL.md` (what T27F is).

### 4.3 Import layers of the window

```
L0 vocabulary from specs/t27f/   petal_menu, petal_specs_viz, gnn_work_graph_viz, viz_contract, formula_orbit, work_graph, petal_glass
L1 layout law                    layout, queen, queen_layout, shell_dock, window_keep, text_cache, frame_gate, windows, statusbar, homes
L2 state                         hot_reload, ui_snapshot, live_data, queen_chats, queen_messages, queen_live_graph, queen_graph_plane, ...
L3 screens (layout + hit-test)   dash, queen_dash, queen_board, doctor, devices, bench, lanes_view, release, keys, profile, kanban, pins, term_pane, vt, ...
L4 draw                          all inside glass_window.t27 (draw_titlebar, draw_chat, draw_header, draw_home, draw_queen_panel, draw_term, draw, ...)
L5 host                          AppKit / CoreGraphics / CoreText through @extern
```

The window imports about 50 modules and no layout spec imports the window: the monolith is a sink. 11 t27os specs
`use t27f::`.

### 4.4 Where is X decided (glass_window.t27 unless named)

| X | Where | Rule |
|---|---|---|
| Window size | W_PT 960, H_PT 540; `setContentMinSize:` 480 x 320; frame kept by window_keep.t27 | the 480 minimum equals SD_CENTER_MIN_W |
| Scene cap | SW_MAX 1920, SH_MAX 1200, CH_MAX 1300; fns `scene`, `view_to_scene` | capped at or above COMPACT_BELOW 760 |
| Breakpoint | COMPACT_BELOW 760; `workspace(sw, dash)` | EXPANDED / DASHBOARD / COMPACT |
| Side column | `side_w`, SIDE_W 272 (also in dash.t27) | |
| Content column | `content_x`, `content_right`, `content_w`; READABLE_W 900, BODY_PAD 24 | |
| Title bar and safe area | TITLE_H 34, AREA_TOP 35, WS_TOP 35; BRAND_CLEAR 12 | three names, two values |
| Home comb (petals) | `hex_r`, `hex_c`, `comb_cell`, `card_at`; HEX_GAP 10 | 27 petals from petal_menu.t27 |
| Screen switching | loose bools in `main`: docked, dash, qp, dp, pfp, kyp | no single screen value |
| Queen right dock | queen.t27 QP_W 300 + shell_dock.t27 (since `c9778a8567`): `main` keeps `swf` (scene) and `sw = swf - push` (centre); packed words TPG_SD_RW 281, TPG_SD_PUSH 282 | dock lies over the centre (OVER) when the centre would fall under 480 pt |
| Bottom terminal dock | shell_dock.t27 `sd_term_rect`; the older `draw_term` path | law written, not drawn yet |
| Panels | qp (queen.t27), dp (doctor.t27), kyp (keys.t27), pfp (profile.t27) | each panel snaps the Queen dock shut |
| Terminal grid | term_pane.t27 `tp_cols`, `tp_rows` | cols = f(width), rows = f(height) |
| Fonts | `mk_line` (NSFont system or monospaced, then CTLine); WEIGHT_*; TP_FONT 12 | sizes are literals at call sites, no type scale |
| Materials | `on_glass`, `tint_alpha`, `surface_alpha*`, `draw_ground`, `draw_backdrop`; THV_* live table | matte glass only at panel width; full window and full screen opaque black |
| Animation | ANIM_FRAMES 75, SPRING_K 28, SPRING_DAMP .86; frame_gate.t27; shell_dock SD_DUR .24 | two timing systems: spring + frame count, and ease-out over a duration |
| Hit-testing | per-screen pure fns (`on_*`, `card_at`, `sb_item_at`, `row_at`, `item_at`, `dash_hit`, `lv_row_at`, `ky_hit`, `prf_*`, `tp_chip_hit`) | draw and hit share constants, not one rect list |

Geometry tests in the window: title-bar controls clear the traffic lights; the window folds below 760; panel width is
glass and full width is opaque; an open panel takes its width from the terminal; title-bar icons share one size and
step; profile boxes hit their own control; the header cluster clears the brand; a helper samples the terminal rect for
window controls. Small test homes for layout law: shell_dock (23), layout (4), queen (12), term_pane (21), dash (38).

### 4.5 Duplicates, conflicts and debts (facts)

1. `draw()` has exactly 64 parameters, the t27b limit; new layout inputs must be packed into existing words.
2. `main` is about 3000 lines: window creation, event loop, per-frame scene, every screen flag.
3. The window is one module of about 18269 lines: host, layout law, hit-tests and drawing together.
4. Packed state `tgv: [TPG_N]u64` with TPG_N 288; adding words grows the array.
5. Wide draw fns: draw_keys_panel 26 params, draw_chat 21, draw_home 20, draw_queen_panel 19, ui_fill 17.
6. Screen state is loose bools, not one value.
7. Two dock widths: QP_W 300 (queen.t27) vs SD_RIGHT_DEFAULT 360 (shell_dock.t27). In stage A the `sd_layout` result
   is discarded on purpose: `draw_rc` is still hardwired to a QP_W dock, so the dock keeps its look while its left
   edge follows the animated width (`dsw = swf + (QP_W - rwa)`). Stage B must give the dock one width law.
8. Constants with two homes: SIDE_W 272 (window, dash.t27); BORDER_WHITE .14, HIGHLIGHT_WHITE .08, CARD_R 12,
   CARD_FILL .06, CARD_EDGE .10 (window, specs/t27f/petal_glass.t27); 960 x 540 (window, window_keep, petal_glass);
   35 vs 34 (AREA_TOP / WS_TOP vs TITLE_H); 760 (COMPACT_BELOW, CW_MAX which only a test reads, a literal on the viz
   virtual stage); 480 (min size, SD_CENTER_MIN_W).
9. Type has no scale: sizes are literals at call sites.
10. Viz pages carry hardcoded pixel geometry (the 760 virtual stage, 0.85 x font size captions, GNN_FRAME / GRAPH /
    PANEL).
11. Layout law the window does not read: work_map.t27, most of executor_registry.t27, ui_plan.t27, scene.t27.
12. Geometry tests live in the 18269-line module, so every layout test compiles the monolith.

Stale text: `shell_dock.t27` lines 38-39 still say the window does not import the module (it does since
`c9778a8567`); a stray `use t27os::petal_centre_unused` near line 1000 of the window names a module that does not
exist.

---

## 5. T27F on GitHub

Corpus: every issue or PR whose title contains "T27F": 314 rows (t27 269, trinity 30, t27-mcp 11, t27os 2, trios 1,
999-multibots-telegraf 1). The org was renamed: gHashTag/{t27, trinity, trios, t27-mcp} redirect to
Trinity-S-AI-DNA/*. There is no `t27f` label. Short names: `t27#N`, `tr#N` (trinity), `mcp#N` (t27-mcp).

**Trap:** the issue-shape gate (`specs/policy/issue_shape.t27`) closes issues that lack its four sections, so CLOSED
is not proof of delivery. t27#11337, #11506 and #11599 are closed by the gate and have no PR.

### 5.1 Definition and owner corrections (translated)

- tr#1648 (open epic): native generative UI -- intent -> system description -> domain model -> renderer-neutral UI IR
  -> renderer -> verify loop. Changing view, theme or zoom never mutates world state.
- tr#1649 (open RFC): the visual spec of `.t27` -- graph IR, labels exact / derived / suggested / unknown, read-only
  viewer first.
- 2026-10-17, t27#10016: T27F is the visual spec of `.t27`, not the topic-to-value engine (tr#1677 is an application).
- 2026-10-18, t27#10537: T27F is t27's native visualization through several tools in every home; adds
  `specs/t27f/frontend.t27`.
- 2026-10-20, t27#11262 / #11275: matte glass in a window, all black in full screen; no overlapping buttons; tabs
  look like the chat screen.
- 2026-10-25, t27#12370: black background everywhere, palette in one place. t27#12164: see live what each agent does.
- 2026-10-26, t27#12690: the dashboard shows the actors' online work like t27.ai/t27f/live.
- 2026-11-01, t27#13044: bring a physics model into T27 OS through the live knowledge graph.

### 5.2 Delivered (a merged PR names it)

- Graph core: spec_graph, spec_graph_view, spec_graph_export (1024 nodes / 4096 edges), spec_graph_derive,
  spec_graph_outline, spec_graph_fold (wrap, kind fold, expand, hit test), spec_deps, ui_plan, frontend.
- MCP tools t27f_graph, t27f_outline, t27f_view; `/t27f/graph/` on the site.
- Live layer (epic t27#10266): work_graph, scene, motion, display_list, hud, a11y_mirror, live_stream, face,
  export_video, frame_render; the site's `/t27f/live/` replay at 60 fps.
- Viz layer (epic t27#12173): viz_contract, petal viz specs, gnn_work_graph_viz, black palette.
- Mycelium: graph_transition, graph design system (MYCELIUM / MAP), a native map prototype (PRs 13123, 13133, 13135).
- Media surfaces: reels, deck, particle engine, factories, captions. Gate: the `claw/t27f` check-run.

### 5.3 Missing or open

1. One renderer-neutral draw path: ui_plan has no renderer; display_list exists but the window pages draw their own
   boxes; the site and the MCP SVG use other paths.
2. Layout stability between revisions and a generic layered layout proven on real data.
3. Navigation: pan, zoom, search, filter, minimap, level of detail beyond kind fold.
4. Scale: fixed arrays; the GNN page and heat bench are fixtures of 17 and 10 nodes.
5. Real data on screen: one tutorial spec, a fixed work graph, a replay of 24 recorded events.
6. Diff and time: no graph diff between revisions, no timeline scrubber.
7. Inference: no producer of SUGGESTED.
8. Editor bridge: LSP diagram (t27#12253) open.
9. Host plumbing needs t27 capabilities, not hand code (t27#10295, PR 13135, mcp#27, tr#1717 loader cap).
10. Performance: 34-78% idle CPU (t27#12410); no click-to-paint latency measured natively.
11. Accessibility of the graph itself: no screen-reader mirror named.
12. Process: stale umbrella t27#11038; the TRI CLAW mount (t27#10554, 999#4015) blocks chat-side drawings.

### 5.4 Contradictions and open questions

- Reels: "applications on top" (t27#10016, #11038) vs a T27F surface (t27#10537, frontend.t27).
- tr#1648 says start from the React/TypeScript frontend; only-t27 and t27#12780 point to t27-compiled hosts.
- Palette: site colours with green accent (t27#10213) vs pure black (t27#12406); the later wins.
- Which renderer is the native graph renderer (site wasm page, window drawer, Core Animation in PR 13135)?
- Live transport: poll (t27#11450), SSE (live_stream) or WebSocket (t27#11599)?
- Is the display list the IR the window pages should draw (t27#12199)? Section 3 answers yes, as the owner's order
  implies; the decision is not yet recorded in an issue.

---

## 6. Competitors

45 products in seven groups were read for what makes a graph UI "smart". None of their code can be linked into T27 OS
(only-t27); what transfers is a published algorithm, an open format or a UX idea.

### 6.1 Matrix (condensed)

Columns: SZ semantic zoom; AL auto layout; NB neighbourhood expansion; QS query by selection; SV saved views; PV
per-fact provenance; AI; TS time scrubbing; DF diff. y = documented, p = partial, n = no, ? = UNVERIFIED.

| Product | SZ | AL | NB | QS | SV | PV | AI | TS | DF |
|---|---|---|---|---|---|---|---|---|---|
| Obsidian Graph + Canvas | ? | y | y | p | p | n | n | y | n |
| Kumu | ? | ? | y | y | y | p | n | ? | p |
| Tana | n | n | n | y | p | n | p | n | n |
| Miro | ? | ? | n | p | p | n | y | ? | ? |
| Structurizr (C4) | p | y | p | y | y | p | n | n | n |
| Mermaid / D2 | n | y | n | n | n | n | n | n | n |
| Cytoscape.js | ? | y | y | y | n | n | n | n | n |
| Cosmograph / cosmos.gl | ? | y | ? | p | ? | n | y | y | n |
| ELK / dagre / Graphviz / d3-force | n | y | n | n | n | n | n | n | n |
| React Flow | ? | p | n | n | ? | n | n | n | n |
| Gephi | n | y | n | y | ? | n | n | y | n |
| Neo4j Bloom | ? | ? | p | y | p | n | p | ? | n |
| Graphistry | ? | y | ? | y | ? | n | p | ? | n |
| JetBrains | n | y | p | p | n | n | n | n | y |
| Sourcegraph | n | n | ? | p | ? | p | ? | n | n |
| CodeScene | n | n | p | p | n | y | ? | y | p |
| LangGraph Studio | n | ? | n | n | p | p | n | y | n |
| ComfyUI | n | n | n | n | p | y | n | n | n |
| Unreal Blueprints | n | n | n | n | n | ? | n | p | ? |

The full 45-row, 13-column matrix and per-product notes were kept in the working notes of this pass; the rows above
are the ones that carry a mechanism used below. Reading: no product has per-fact provenance as a uniform first-class
feature, and none combines it with neighbourhood expansion, query, diff and auto layout on one typed model. That is
the open space T27F's four labels fill.

### 6.2 Ten mechanisms worth taking (ranked)

1. A per-fact trust label drawn the same way in every view (CodeScene coupling source, Sourcegraph precise vs
   fallback, ComfyUI recipe in the artefact). T27F's exact / derived / suggested / unknown is stronger: "exact" is
   tied to the compiler, and unknown is a node, not a gap.
2. One model, many scoped views (Structurizr). In T27F a view is a filter over labels, derived, never drawn by hand.
3. Neighbourhood expansion with depth and path focus (Obsidian local graph, Kumu focus, JetBrains paths). T27F keeps
   suggested edges collapsed behind a count and unknown ends as stubs.
4. Stable, deterministic, pinned layout (Graphviz dot, ELK layered, D2 TALA pins). Seed and layout version live in
   the IR; a pinned position is user data, a computed one is derived.
5. Semantic zoom with a trust floor: far = modules from exact edges, mid = functions and tests, near = source.
6. Query and selection as one live thing (Tana live search, Cytoscape selectors, Cosmograph lasso), including
   queries over the label itself.
7. Saved, named views in an open text format (Structurizr DSL, JSON Canvas). In T27F a saved view is a `.t27` file
   that records the IR version and shows "stale" when the spec moved.
8. Time axis and diff (Obsidian animate, CodeScene trends, Gephi dynamic graphs): a diff that tells an exact edge
   removed from a suggested one going away.
9. Runtime overlay on the static structure (LangGraph Studio, Unreal wires, ComfyUI re-run of changed nodes): an
   agent's claimed result stays suggested until a gate makes it exact.
10. A docked shell around the canvas with one inspector (VS Code side bars, Zed layouts, Cursor agents window). The
    repo already has the law for it in `specs/t27os/shell_dock.t27`.

### 6.3 Ten traps

1. Hairballs: a force layout of a whole dataset becomes a ball; cure is a depth-limited local view and filters first.
2. Layout jitter: force engines move on restart; layered engines re-flow on upgrade (Mermaid moved its default from
   dagre to ELK).
3. Lost context when drilling down: keep the parent visible, faded, with a path.
4. Fake AI confidence: AI clusters and summaries shown as organisation, with no proof pointer.
5. The picture drifts from the thing: hand-made boards; static generated diagrams with no confidence.
6. Widget pages instead of a graph: the shape the owner rejected; the graph owns the screen.
7. Dependence on an unstable vendor or licence; formats (JSON Canvas, DOT, ELK JSON) are safer than code.
8. Layout engines with unequal features decide which view features can be promised.
9. Debug overlays that exist only at run time; keep observations as labelled facts with a timestamp.
10. Rendering ceiling: SVG and DOM canvases choke past a few thousand items; level of detail first, GPU second.

UNVERIFIED items left open: Obsidian minimap and label fade; Sigma label LOD; yFiles folding and bundling; Neo4j
Bloom perspectives; Zed dock positions; Sourcegraph UI labelling; CodeSee product status; and several more of the
same kind.

---

## 7. Research behind the factory (named by the owner)

| Work | Idea taken | Where it lands in the pipeline |
|---|---|---|
| SUPPLE (Gajos and Weld, 2004; journal version 2010) | UI generation as a search for the representation that minimises a cost under device and user constraints | candidate generator + soft scores (section 3.1 step 2) |
| Penrose (Ye et al., SIGGRAPH 2020) | separate what (Substance), how (Style) and the types (Domain); layout as constrained optimisation | model vs design policy vs geometry kept as separate entities |
| Vega-Lite (Satyanarayan et al., 2017) | a grammar of graphics with encodings and selections; a small spec expands to a full view | the rule table and the representation vocabulary as a grammar, selections as shared state |
| Draco (Moritz et al., 2018) and Draco 2 (2023) | design knowledge as hard constraints (validity) and soft constraints (weighted preferences) | hard filters already exist (validators); soft scores are the new part |

These are cited as ideas to re-specify in `.t27`; no code from them is used.

---

## 8. Open questions, with the default taken

| Question | Default until decided |
|---|---|
| Where does the rule table live? | a new small spec in `specs/t27f/` next to viz_contract, importing VIZ_REP_* and both models' kinds; `node_widget` reads it |
| Catalog merge now or later? | `CATALOG_VERSION` 2 in the existing ui_plan / viz_contract files, inside the first slice |
| Scene IR: raise caps or version? | version the vocabulary (same primitives) with larger caps; the HUD keeps version 1 limits |
| Which three representations first? | graph, message sequence, load (owner's order) |
| Where does the design policy live? | next to hot_reload.t27's live table, loaded by the same live-module path |
| Who accepts the Observatory output? | someone other than the rule's author, plus a pixel check of the real window |

---

## 9. Appendix: sources

- Specs read for the audit: `specs/t27f/{ui_plan, scene, display_list, viz_contract, graph_design, work_graph,
  frontend, petal_viz_plan, spec_graph, spec_graph_view}.t27`, `specs/t27os/{work_map, graph_transition, ui_snapshot,
  hot_reload, glass_window, shell_dock, queen_layout, queen_live_graph, queen_graph_plane, kernel}.t27`,
  `specs/queen/actor_events.t27`.
- GitHub: titles, bodies and comments of the 314 T27F rows listed in section 5, read with `gh`.
- Competitors: vendor docs, repositories and release pages, read 2026-11-03.
- Not re-run here: any test count quoted from a PR; any competitor fact marked UNVERIFIED; the four research works.
