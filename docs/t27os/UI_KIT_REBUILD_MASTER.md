# T27OS UI Kit Rebuild: master index

Status date: 2026-11-16. Epic: Trinity-S-AI-DNA/t27#13426. This file is an index. Each fact
has one home (an issue, a spec or a PR); this file points at it and copies no rule or spec text.
A source that was not found is written as "not found".

## 1. What the rebuild is

- Rebuild T27OS as a UI Kit first system: one reproducible stack from object graph to a saved
  screenshot, with no new hand-written pixel layout (epic #13426, rules 1 to 5 in its body).
- The graph-first engine it stands on is epic #13251 (VfGraph is the object graph; pixels are a
  derived layer).
- What T27F is: `.claude/skills/t27f/SKILL.md`, section "What T27F is"; the surface/status table
  is `specs/t27f/frontend.t27` (SURFACE_NAMES, SURFACE_STATUS, SURFACE_MAKER).

## 2. The stack and who owns each layer

| Layer | Owner file(s) |
|---|---|
| T27B (compiler backend, runs the specs) | epic #13251 "Correction" paragraph; no UI spec of its own |
| UI Kit | `specs/t27f/ui_kit.t27` (kit_button, card, input, badge, nav, table, text, container), `specs/t27f/vf_uikit.t27`, `specs/t27f/vf_storybook.t27`, `specs/t27f/stories/` |
| Tokens and theme | `specs/t27f/vf_tokens.t27`, `specs/t27f/vf_theme.t27`, `specs/t27f/vf_style.t27` |
| VfGraph (object graph) and layout | `specs/t27f/visual_factory_contract.t27`, `specs/t27f/vf_layout.t27`, `specs/t27f/vf_meaning.t27` |
| Type and render | `specs/t27f/vf_type.t27`, `specs/t27f/vf_card.t27`, `specs/t27f/vf_render.t27`, `specs/t27f/display_list.t27` |
| T27F (frontend catalog) | `specs/t27f/frontend.t27` |
| T27OS screens | `specs/t27os/glass_window.t27`, `specs/t27os/vf_story_win.t27`, `specs/t27f/petal_menu.t27` |
| Screenshot | `specs/t27f/vf_snapshot.t27`, `specs/t27os/vf_card_main.t27`, `specs/t27os/vf_frame_main.t27`, shot plan `specs/tri/shot.t27` (not merged, PR #14015) |
| Visual regression | `specs/t27f/vf_diff.t27`, `specs/t27f/vf_regress.t27`, guard `specs/t27f/vf_guard.t27` |
| T27L (UI iteration records) | `specs/t27f/ui_history.t27` (typed record), `specs/promotion/t27l_promotion.t27`, schema doc `docs/promotion/t27l-promotion-schema.md` |

### specs/t27f by family (161 entries, names only)

- UI Kit and style (vf_*, ui_*): ui_kit, ui_plan, ui_targets, ui_history (+ log, s01..s05),
  vf_brand, vf_card, vf_card_fit, vf_card_tok, vf_diff, vf_extent, vf_guard, vf_layout, vf_meaning,
  vf_pixel, vf_proportion, vf_regress, vf_render, vf_ring1, vf_scale, vf_snapshot, vf_storybook,
  vf_style, vf_text_measure, vf_theme, vf_tokens, vf_type, vf_uikit, vf_wide,
  visual_factory_contract, viz_contract.
- Petal menu: petal_menu, petal_menu_wasm, petal_glass, petal_*_viz (frontend, health, isa, issues,
  jobs, pulls, pulse, security, specs, university, verdicts, widgets), petal_viz_plan.
- Netlist views: netlist_* (about 15 renderer-neutral specs).
- Spec graph: spec_graph*, spec_deps, deps_card, work_graph*, gnn_work_graph_viz.
- Scenes and stills: scene*, still_gate, still_text_main, frame_*, display_list.
- Reels and particles: reel*, particle_*, topic_*, lesson_reel, export_video, post_caption.
- Live and hot: live_*, hot, measure_live.
- Fonts: outfit_atlas, outfit_b_data, outfit_r_data, glyph57.
- Catalog and law: frontend, build, deck, hud, motion, quest, face, sieve_design_law, token_floor*.
- Directory: stories/ (3 entries).

Other UI Kit specs outside t27f: `specs/t27os/vf_card_main.t27`, `vf_frame_main.t27`,
`vf_story_main.t27`, `vf_story_win.t27`, `vf_type_main.t27`; a `specs/uikit/` tree (stories,
netlist renderer) exists on master.

### Surface law
`specs/t27f/frontend.t27` holds the surface and status table; read it there. Not copied here.

## 3. Milestones, as the epic #13426 defines them

Quoted from the epic body (table "Milestones"):

| # | Name | Epic status |
|---|---|---|
| M0 | Foundation: tokens, components, screenshot pipeline | 80% done |
| M1 | Window Shell: title bar, sidebar, content area from UI Kit | not started |
| M2 | Live Work Graph: agents, lanes, tasks, PRs online | not started |
| M3 | Queen Dashboard: GOD MODE, AGENTS, LANES via UI Kit | not started |
| M4 | Petal Menu: 27 petals via display list, not pixel math | not started |
| M5 | Cleanup: delete replaced code, connect vf_guard hook | not started |

M0 gaps listed in the epic: (a) `kit_text` and `kit_container` missing; (b) no T27L log schema for
UI events; (c) no saved M0 screenshot artifact.

Where each stands (evidence checked 2026-11-16):

| Item | State | Evidence / issue |
|---|---|---|
| M0 kit_text, kit_container | done on master | #13427 closed; both are in `ui_kit.t27`. Open PR #13513 would remove them again (not ours, untouched) |
| M0 saved screenshot, VR record, T27L record | done | PR #13661 merged 2026-11-14: `runs/m0/` (card frames, diffs, `t27l_m0.json`) |
| M0 pixel diff host program | open | #13662 (label M0) |
| M0 visual specs reportable on the VPS toolchain | open | #13664 (named in an epic comment; not read in this pass) |
| M0 T27L "schema for UI events" | partly | typed record `ui_history.t27`; records exist (section 6); not judged against the epic wording |
| M1 Window Shell | not started as defined; related work open | shell epic #13895; header and tab work PR #13897 (open) |
| M2 Live Work Graph | not started | #13423 open |
| M3 Queen Dashboard | not started | no issue found in the list collected |
| M4 Petal Menu | not started as defined; related open | #13299 open; names merged in #14005 |
| M5 Cleanup | not started | none |

Related work that exists on master but is not mapped to M1..M5 by the epic: the Control Center
view and its wiring (PRs #13777, #13798, #13802) and the object-per-PR migration of kit primitives
(PRs #13776 to #13797, #13915, #13920, #13922). The epic comment of 2026-11-14 (owner order
2026-11-15) sets the migration order: tokens, header, layout, surface, gallery.

## 4. Issues

| # | Title | State | What it decides |
|---|---|---|---|
| 13426 | epic: T27OS UI Kit Rebuild V1 | open | Milestones M0..M5, non-negotiable rules, Definition of Done |
| 13251 | epic(t27f): graph-first UI engine | open | VfGraph is the object graph; theme swap changes tokens only; rings 1..10 |
| 13401 | t27f: T27 OS panel components in ui_kit with stories | open | Adds panel components and one story per state (gate 4) |
| 13417 | t27os: agent_registry_view, AN_SLUGS + VfGraph | open | 27 domains as nodes, no hand layout in glass_window |
| 13423 | t27os: Live Work Graph | open | One screen for all agent work through UI Kit and T27F |
| 13399 | policy: ui_raw_values scan | closed | Literal colour calls counted as a ratchet (gates 2, 3) |
| 13400 | claw: cost_ledger_scan | open | Paid calls checked against the pane-first escalation law (gate 1) |
| 13280 | t27f: text wrap for VF_TEXT in vc_render_s | open | Wrap long text nodes in the card renderer |
| 13264 | queen: ring 4 Style tokens | open | Chain labels read existing T27F tokens only |
| 13299 | T27OS UI: 27-petal menu through one display list | open | Petal menu drawn from a display list, not direct path math |
| 13416 | t27os: registry view, AN_SLUGS + 7 new domains | closed | Ring 2 registry card |
| 13895 | [EPIC] T27 OS Shell | open | Sidebar, agent tabs, panels, window manager, Team runtime |
| 13662 | t27f: host program for vf_diff | open | Pixel diff of two frames (M0) |
| 14008 | T27OS_STILL frames are not still | open | Live pane content reaches the frame; hashes not stable |
| 14011 | tri: shot plan and settle verdict | open | `specs/tri/shot.t27`, PR #14015 (branch `claude/tri-shot-14011`) |
| 14013 | t27c: use scanner drops names after first `;` | open | Imported helpers: one statement per line until fixed |
| 14014 | t27os: identity.t27 does not compile | open | Module alias A undeclared; seal cannot be remade |
| 14004 | spec(t27os): petal and domain names equal the owner table | closed | One source for names; merged as PR #14005 (labelled shape-refused by the gate; the PR landed) |

Child issues named in epic comments, not in the collected list: #13427 (closed), #13428
(evidence, PR #13661 merged), #13663, #13664, #13748 to #13753, #13852, #13874, #13914, #13918,
#13919.

## 5. Merged PRs carrying UI Kit work (search over titles, 2026-11-16)

| PR | Merged | Title |
|---|---|---|
| 13324 | 2026-11-09 | renderers draw the UI Kit frame in TH_T27 + Outfit |
| 13328 | 2026-11-08 | style guide and UI kit values for the course HUD |
| 13337 | 2026-11-08 | course: HUD on vf_style/vf_uikit, frame diff |
| 13342 | 2026-11-08 | policy: own-language entries for the course HUD browser glue |
| 13371 | 2026-11-09 | t27os: UI kit ring 1, window geometry and theme switch |
| 13671 | 2026-11-14 | t27os: story host and first UI Kit story frames, 68 of 70 |
| 13676 | 2026-11-14 | t27f: kit variant and state reach the renderer, gallery sheets |
| 13699 | 2026-11-14 | uikit: design tokens + primary button story spec |
| 13703 | 2026-11-14 | uikit: Badge Primary story IDs and interaction spec |
| 13710 | 2026-11-14 | uikit: single .t27 SSOT, catalog + native tokens wired to FRONTEND petal |
| 13733 | 2026-11-14 | vf_story_win: UIKit storybook window/input adapter |
| 13776 to 13787 | 2026-11-14 | ui-kit objects: registry, control center shell, theme pill, plate, container, text, button, light dot, bracket frame, avatar, lamp, god mode toggle |
| 13796, 13797 | 2026-11-14 | ui-kit: card, badge |
| 13798, 13802 | 2026-11-14 | ui-kit: Control Center view, wired into GW page |
| 13806 | 2026-11-14 | one theme-role swap in the theme contract |
| 13811 | 2026-11-14 | path imports for stories, stories_badge, catalog |
| 13813 | 2026-11-14 | uikit shots plan spec |
| 13856, 13876 | 2026-11-15 | Plate story, bracket frame story |
| 13881, 13884, 13908 | 2026-11-15 | NETLIST constructor view-wave, stories, renderer |
| 13915, 13920, 13922 | 2026-11-15 | StatCell, HeroCard, SegmentBar objects |
| 13952, 13961 | 2026-11-15 | NETLIST stories T27L scaffold and 14 screenshots |
| 13661 | 2026-11-14 | M0 evidence (`runs/m0/`; title not in the title search, found via the epic comment) |
| 14005 | 2026-11-16 | window petal and domain names equal the owner table (25bb529264) |

Open (list only, none touched): #13513 `split/t27f-foundation` (drops text and container from the
kit; conflicts with the M0 list), #13974 `uikit/log-row-13969`, #13897
`claude/agent-header-tabs-13893`, #13815 `tri-uikit-shots-13814`, #14015
`claude/tri-shot-14011`. The search also returned #13962, #13686, #13694, which are not UI Kit
work.

## 6. Evidence on disk

Tracked in the repo (paths relative to the repo root; sha256 first 16 hex):

| Path | Bytes | sha256 |
|---|---|---|
| `runs/m0/card_375_t0.png` | 15676 | 4b099d1f1f08e654 |
| `runs/m0/card_375_t1.png` | 15885 | 696e7b6fca418869 |
| `runs/m0/m0_card_v1.ppm` | 1080015 | 3c2a3f45d9ebf696 |
| `runs/m0/diff_t0_base_after.png` | 1147 | 4532e45aba578e21 |
| `runs/m0/diff_t0_vs_t1.png` | 15496 | 0efb9a6b1d753b70 |
| `runs/m0/kit/gallery/overview.png` | 376943 | d63bb159ee6e69a8 |

Also tracked: `runs/m0/kit/` (34 stories, each `sNN_t0.png` and `sNN_t1.png`, plus gallery sheets),
`runs/netlist_card/` (gallery and sheet frames), `runs/netlist_stories/` (14 jpg frames).
Nothing tracked is named `petal_menu` or `QD_RV` apart from specs and a seal.
Images were not opened in this pass; judged by size and hash only.

Local build output, not committed (`target/vr-14004/`, settled frames of the #14004 before/after
run; 38 files in `before/`, 52 in `after/`):

| Case | before bytes / sha | after bytes / sha |
|---|---|---|
| home-dark | 204662 / 9c6768ee | 204662 / 9c6768ee (same) |
| home-light | 153862 / ee93a332 | 181133 / 2ab1468e |
| C-dark | 942857 / d64f6592 | 927637 / 9073814e |
| C-light | 843687 / bac6989c | 927832 / bf00c688 |
| F-dark | 315092 / 8dec234f | 312642 / 2b5d4017 |
| F-light | 310127 / 51773ac8 | 309698 / 684fc681 |
| Q-dark | 420322 / 5b2e3596 | 422728 / 2ae4d641 |
| Q-light | 395678 / b0a93d4d | 389595 / 8feba85b |

`target/win-master/` holds the test window bundle `T27OSTest.app`, `glass_window.o` (1651944
bytes) and `run.log` (6232 bytes); no image files.

T27L records (UI iterations): `runs/m0/t27l_m0.json` (1 record, issue #13428),
`runs/m0/kit/t27l_kit.json` (1 record, issue #13677), `runs/netlist_card/t27l_card.json`
(1 record, issue 13673), `runs/netlist_stories/t27l_netlist_stories.json` (1 record, issue 13950).
Other T27L files: seals `.trinity/seals/promotion_promotion_t27l.json`,
`.trinity/seals/t27mcp_McpT27lSurface.json`, `.trinity/seals/t27os_T27osT27l.json`.

Not found: `/tmp/t27f_architecture_report.md` (the file does not exist on this machine).

## 7. Known gaps, stated plainly

- No pixel diff host program: #13662. Visual regression so far is byte or hash comparison.
- Still frames are not deterministic: #14008 (live pane content reaches the frame).
- The shot plan and settle verdict as a spec is in work: #14011, PR #14015 (open).
- The `use` scanner drops names after the first semicolon on a line: #14013 (workaround: one
  statement per line in imported helpers).
- `specs/t27os/identity.t27` does not compile; its seal cannot be remade: #14014.
- Purpose, layer and register of the 27 domains were not supplied; the window draws names only.
- Old names remain in `specs/agents/*.t27`, `queen_dash.t27` `QD_PIN_AGENTS` and prose in
  `homes.t27` (epic comment of 2026-11-16).
- PR #13513 (open, another session) would remove `kit_text` and `kit_container`, which M0 needs.
- `tri visual-update` was not built when the M0 status comment was written (2026-11-14); not
  re-checked here.

## 8. Latest state

Petal and domain names equal the owner table: merged as PR #14005 (25bb529264), issue #14004
closed. A test window was built from that commit; the owner looked at it on 2026-11-16 and
confirmed the new names are on screen. Pixel diff (#13662) is still open.

## 9. Next measurable step for M0

Take #13662, the first open M0 item: a host program for `vf_diff` that reads two frames and
prints the differing pixel count and bounding box. Measure: run it on
`runs/m0/m0_card_v1.ppm` against itself (expect 0 differing pixels) and against a frame of the
other theme (expect more than 0), then record the result in a T27L record.
