---
id: t27f
name: T27F Frontend
description: T27F is t27's frontend -- the one tool through which t27 shows anything visual, made through several tools (spec graph and outline, honeycomb map stills, the t27.ai graph page, particle reels and stills, the hot-reload stream, the claw/t27f check-run), with its surfaces, their status and the intent-to-surface routing in specs/t27f/frontend.t27. Its core is the visual spec of a .t27 file (gHashTag/trinity#1648, RFC gHashTag/trinity#1649). TRI CLAW serves its MCP tools. Use when the user says "t27f", "visual", "visual spec", "show / visualize / draw this spec", "reel", "still", or opens a .t27 file to see its structure. Also runs the topic-map application (gHashTag/trinity#1677) when the user gives a topic or a client brief.
---

# T27F Frontend

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

## What T27F is

**This section is the one home of the definition.** `CLAUDE.md`, `AGENTS.md`, `specs/OWNERS.md`,
the skill card `specs/skills/t27-t27f.t27` and the spec headers point here; they do not restate it.

T27F is **t27's frontend: its native visualization, made through several tools.** Everything t27
shows -- a spec as a diagram, a topic as a map, a lesson as a particle reel, a report as a still, a
pull request as a check-run -- is a T27F surface. Owner, 2026-10-18: everything visual is gathered
into T27F as one tool, and T27F goes into TRI CLAW.

**The law is `specs/t27f/frontend.t27`.** It lists the surfaces, what draws each today (LIVE, LAB,
PARTIAL, BLOCKED, SPEC), which surface answers an intent (`route`), the stand-in when the wanted one
cannot be made this turn (`pick`), the rule that nothing undrawn is claimed made (`claim_ok`), and
which surfaces TRI CLAW serves (`claw_serves`). Read the table there; this section does not copy it.
The rules of each surface stay in the spec that decides it.

The surfaces, by family:

- **Spec diagram (the core).** Every t27 specification has a first-class visual form, an
  interactive, inspectable diagram (epic gHashTag/trinity#1648, RFC gHashTag/trinity#1649). Drawn as
  SVG by `t27f_graph`, as text by `t27f_outline`, and on t27.ai/t27f/graph/ and the Spec Explorer.
  - **One spec -> one synchronized source model -> many views**: source, architecture, data flow,
    dependencies, tests/benchmarks, docs. Every view is derived from the real spec (AST/IR plus
    metadata), never a hand-drawn picture that can drift from the code.
  - **Graph IR**: versioned, typed nodes and edges; the full lists are in #1649 and
    `specs/t27f/spec_graph.t27`.
  - **Every node carries one inference label**: `exact`, `derived`, `suggested` (a model's
    explanation, visibly marked) or `unknown`. A model's guess is never drawn as a verified property.
  - **Bidirectional source mapping**, deterministic generation, graph diffs between revisions.
  - **Read-only viewer first.** GF16 (`specs/numeric/gf16.t27`) is a *candidate* pilot (#1651).
  - **The pipeline (#1648)**: intent -> system description -> domain model -> renderer-neutral UI
    plan (trusted widget ids only, `specs/t27f/ui_plan.t27`) -> T27F renderer -> verification. A model
    never ships code to the renderer.
- **Maps.** The 27-cell honeycomb (`specs/t27f/reel.t27`, look `deck.t27`): stills by `t27f_view`
  today; the video and live player are spec only.
- **Particle reels and stills.** The token_floor world (`token_floor*.t27`, `lesson_reel.t27`,
  `reel_particles*.t27`, `particle_engine.t27`): every pixel is a sum a spec computes and every motion
  a step of an integrator. Rendered on the Railway lab; a still is one frame of a reel. T27F Live
  (`particle_field.t27`) is the same physics driven by an agent's events, spec only.
- **Live editing and review.** The hot-reload stream (`hot.t27`, `build.t27`), the `claw/t27f`
  check-run the TRI CLAW App posts on a pull request (`specs/claw/t27f_gate.t27`), the PR preview
  (`specs/claw/preview.t27`).
- **Health.** The MCP dashboard (`specs/t27mcp/dashboard.t27`, tool `t27_dashboard`).

**In TRI CLAW.** `specs/t27mcp/service.t27` gives every `t27_` and `t27f_` tool to the t27 home and
mounts it into TRI CLAW's `/mcp`. `frontend.t27` checks that every T27F tool lands there and names
the drawing set TRI CLAW's chat agent carries (`claw_visual_tool`), next to the coder persona's
`CODER_TOOLS`. Whether the deployed `/mcp` mounts them today is not measured.

**What T27F is not:** it is not the topic-to-value engine. gHashTag/trinity#1677 (topic or client
brief -> 27-lesson map -> Bee -> verified artifact) is an *application*: its decisions live in
`specs/t27f/topic_engine.t27`, and what it shows is drawn through T27F's map surface. T27F is also
not one diagram: an answer asked for "a T27F visual" picks the surface `frontend.t27` routes it to.
Owner corrections: 2026-10-17 (not the topic engine), 2026-10-18 (the frontend, through several
tools, reels included).

## What exists and what does not

The status of every surface is `SURFACE_STATUS` in `specs/t27f/frontend.t27`. The rows below give
the diagram core in detail, then the other surfaces and the topic-map application.

| Layer | Where | State |
|---|---|---|
| Spec -> graph IR schema (the core) | `specs/t27f/spec_graph.t27` (t27#10020) | spec with tests: node / edge kinds, labels, spans, `validate_graph`, line -> node mapping, node -> ui_plan widget, one hand-read fixture (`specs/tutorial/08_modules_and_arrays.t27`) |
| Graph view: layout of a spec graph | `specs/t27f/spec_graph_view.t27` (t27#10034) | spec with tests: one column per node kind, size / colour / dash / row order from the inference label, UNKNOWN nodes alone in a band, every edge with a source line, node -> widget through `node_widget`; numbers only, laid out on the hand-read fixture; drawn by the MCP tool `t27f_graph` (gHashTag/t27-mcp#9) |
| Graph view for big specs: wrap, fold, expand, hit | `specs/t27f/spec_graph_fold.t27` (t27#10218) | spec with tests: a kind column taller than N = 12 rows wraps into sub-columns so the canvas aspect is near phi (identical to the view when nothing wraps); a fold to one box per kind with the exact count and one arc per pair of kinds with the exact edge count, each labelled by its weakest member; `expand_graph` = one kind plus its direct neighbours, copied verbatim with an origin map; `wrap_hit` / `wrap_hit_line_start` / `wrap_hit_line_end` take a click to a node and its lines; numbers only, the view's colours are not touched |
| Dependencies view: the specs around one spec | `specs/t27f/spec_deps.t27` (t27#10680) | spec with tests: reads `t27c use-edges --specs-dir specs`; layer 0 is the focus, +k the specs k imports away (right), -k the specs k importers away (left), each spec once in its shortest-path layer; every edge-list line between two drawn specs is one `exact` edge, a repeated line drawn once; `cycle=1` when the focus imports itself or is imported by a spec in its own import closure; a walk past the caller's room or depth is refused whole (`refused\tfull` / `refused\tdeep`), never cut. A `use` line t27c does not resolve is not in the edge list, so `deps_unknown` (t27#10702) reads the focus source and writes each such line as `unknown\tpath`, mirroring `use_resolve.rs` (whole path, then one fallback level) without looking for a file; on master f33f269d5 that was 23 lines in 13 specs, byte for byte the `t27c gen` warnings. `deps_why a b` (t27#10705) writes one shortest chain of edge-list lines from a to b, each `exact`, a tie taken in edge-list order, `none` when a does not reach b, refused whole when it does not fit; `specs/t27f/why_card.t27` (t27#10716) draws that chain as a 1080x1350 social card, one exact box per spec, and draws nothing for a chain that is not one. Limits: the view flags only the focus, and `deps_cycles` (t27#10693) lists every spec on a cycle, each once, refused whole (`cap + 1`) if anything does not fit; a spec on a cycle through the focus is drawn on both sides; no host draws it yet. Corpus on master e607228f7: 806 specs, 2461 edge lines, nothing refused at depth 32, deepest walks 6, one real cycle (igla race `backend` <-> `rtl`), which `deps_cycles` lists exactly on master f7efbc232 and lists as empty with #10689 |
| Spec -> graph IR exporter | `specs/t27f/spec_graph_export.t27` (t27#10162) | spec with tests: `export_graph(path, src)` is a deterministic byte scanner over the source, `export_verdict` is `validate_graph` of the result. One node per top-level declaration (a leading `pub ` is allowed), spans to the matching closing brace, every node and edge EXACT or (unclassifiable) OPAQUE + UNKNOWN, CALLS / USES / CONSTRAINS edges with the line of the use, refusal past 1024 nodes or 4096 edges. On the tutorial 08 snapshot it equals the hand-read fixture on all 14 declaration nodes and all 9 EXACT edges; it does not emit the fixture's `std.testing` UNKNOWN node, nor its 7 DERIVED edges (4 data flow, 3 tested_by), because a scanner never derives; the derive pass below adds them. Limits: textual matching without scope or type resolution, no block comments, no struct/enum bodies, no cross-spec `use` resolution, capacity 1024 nodes and 4096 edges, u16 indices (all 2225 specs in specs/ fit, largest 775 nodes and 3194 edges after derive; anything past the cap is refused, never truncated), takes a `[]const u8` slice rather than `[65536]u8` plus a length. AST/IR audit is trinity#1650, pilot choice #1651 |
| Graph IR derive pass | `specs/t27f/spec_graph_derive.t27` (t27#10191) | spec with tests: `derive_graph(g, src)` appends what the exporter must not claim and changes nothing it holds. R1 DERIVED `tested_by` = every EXACT CALLS edge from a test, invariant or bench to a fn read backwards (no line). R2 DERIVED `data_flow` = a const or `use` that an EXACT USES edge ties to a fn and that is named inside one of the fn's return statements (from `return` to the next `;`), on the line of the first such name. R3 UNKNOWN opaque node = an undeclared qualified call target of three or more segments (`std.testing.expectEqual(` names `std.testing`), spanning the lines that call through it. Exporter plus derive equal the tutorial 08 fixture as sets: 15 of 15 nodes, 16 of 16 edges, no special case. Limits: names matched as text; call results and flow through locals are not derived; R3 needs 3 or more segments (two cannot be told from a method call); a graph that does not validate, or a source of another line count, comes back unchanged; past 1024 nodes or 4096 edges the result is refused with the too-many verdict. Not wired to a host: the MCP tool does not call it |
| UI plan IR + trusted widget catalog | `specs/t27f/ui_plan.t27` (t27#9962) | spec with tests; no renderer draws a plan yet |
| Honeycomb focus+context geometry | `specs/t27f/reel.t27` | spec; stills drawn by the MCP tool `t27f_view` |
| Renderer in the Trinity UI | gHashTag/trinity, t27.ai/t27f/graph/ and the Spec Explorer graph layer | live for the spec graph; it ignores a `src=` parameter, and no page draws a UI plan yet |
| Surface: particle reels and stills | `specs/t27f/token_floor*.t27`, `lesson_reel.t27`, `reel_particles*.t27` | rendered on the Railway lab (lesson 1 EN and RU); no renderer in the repo, no MCP tool yet |
| Surface: honeycomb map video and deck | `specs/t27f/reel.t27`, `deck.t27` | spec; stills only, through `t27f_view` |
| Application: topic map (#1677) | `specs/t27f/topic_engine.t27`, `topic_reel.t27` | spec; drawn through the map surface; procedure below |
| Application: content factory (epic t27#10560) | `specs/t27f/reel_factory.t27` (t27#10561) | spec with tests: an item's code and title, read keyword by keyword, give 11 features. Those pick 1 of 12 scene archetypes (best pair first, cap 4, no repeat within 2 items, then the least-used generic scene) plus an accent, a motion, a layout and 8 output formats. R3 (t27#10571) draws it: `scene_raster`, `scene_lex`, `scene_bg` plus one `scene_<archetype>.t27` per archetype (code, graph, tests, trit, loop, branch, data, links, guard, title, task, map), each `<name>_px(t, acc, n, x, y)` over the lesson's own content on a 360 x 640 canvas, dispatched by `scene_still.pixel`; `still_gate.t27` renders the 27 course-0 stills (108 x 192 luma), scores all 351 pairs with a block SSIM and refuses above 850 permille, measured worst pair lessons 8 and 20 at 607. `lesson_reel` still draws token_floor (R4). SSIM on luma sees layout and contrast, not meaning. The floor modulo is `fmod` (t27b rejects `@mod`). Lessons: read the plan's spread over the whole course before trusting it. A greedy walk in course order gave 7 lessons one scene. Code counts alone handed lesson 8 the tests scene over "A test block". Gate: `tri factory`. The plan prints from a separate module, `specs/t27f/reel_factory_plan.t27` (t27#10573): `t27c gen` it, then run the zig (t27c has no `run`). Names come from the factory's comma lists, so the law module keeps no `main`. Traps: the lab's `/root/t27` t27b can predate #10455 and refuse `std.debug.print` that master accepts, so build a fresh t27b in its own worktree with its own `CARGO_TARGET_DIR` before you call a print unsupported. `t27c seal --save` needs zig on PATH (`/opt/zig`). A base64 line fetched back from fs-lab merges with the Railway warning, so wrap it as `B64:...:END`. A new `tri` arm must start its own line (`  name)`) or the documented-commands checker misses it |
| Related | trinity#1647 dashboards, #1652 MVP research, #1653 three skills, #1655 evolution runtime | open |

When asked "how is T27F going", report against the core rows first, then the other surfaces.
Progress on a reel or on the topic map is not progress on the diagram core.

**Trap: a name declared in two specs cannot be imported (t27#10556).** Measured on t27c 0.5.2:
`use t27f::lesson_reel::phrase_start` returns `UNRESOLVED ... ambiguous, not spliced`, because
`token_floor_mux.t27` declares a private `fn phrase_start`. The same happens to `cta_from` and
`voice_fits`. The explicit path does not help. Before you plan a new module on top of a reel spec,
grep `specs/` for every name it would import. If any of them collides, put the new code inside the
spec that owns those names, under a prefix such as `beat_`. The beat clock (t27#10596, PR #10598)
lives in `lesson_reel.t27` for this reason.

**Cutting video at a ms time: land on a frame, then measure the runs (t27#10603, PR #10605).**

ffmpeg `xfade` starts on the first frame whose time is at or after `offset`. This is how to get the cut you planned:

- Floor the cut to a base frame: `f = (c - half) * fps / 1000`.
- Print the offset as `trunc(f * 1000 / fps)` ms. Rounding up misses frame f on 2 frames in 3.
- Put every input through `fps=30,settb=AVTB` so the offset is not rounded again.
- Chained `xfade` offsets are absolute reel times when each clip starts at its own frame 0.

To check a graph without looking at pictures:

1. Feed it solid `color=` inputs at `s=2x2`.
2. Write `-f rawvideo -pix_fmt rgb24`.
3. Count the runs with `od -An -tu1 -w12 -v | cut -c1-12 | uniq -c`.

The runs are the scene lengths and must equal the spec's frame counts. Expect one extra frame when the offset falls on a whole ms.

Lesson main functions read `std.c.getenv`, so build them on the lab with `zig build-exe -lc`.

**Adding a mode to a graph printer without moving the old ones (t27#10609, PR #10610).**

- Pass the old callers a flag (`graph(..., false)`) instead of copying the printer.
- Give the input numbers a function with a test that every input is used exactly once, in each layout.
- Prove the old output did not move: on the lab, build the old spec (from `git show <ref>:path`) and the new one, run both on a grid of lessons, languages and voices, and `cmp -s` every graph. PR #10610 compared 96 graphs and 0 differed.
- For a large frame, render lossless (`-c:v libx264 -qp 0`) and crop a 2x2 pixel outside every text box (`crop=2:2:x:y,format=rgb24`). The solid runs must still equal the spec's frame counts.

Traps from this tick:

- An untyped array literal in a test (`var a = [_]usize{...}`) becomes a Zig tuple, which cannot be indexed at runtime ("tuple field index must be comptime-known"). Declare `var a : [8]usize = undefined;` and fill it in a loop.
- `fs-lab.sh "echo $b64a ... ; echo $b64b ..."` with two large files exits 0 and writes nothing. Upload one file per call and compare md5 on both sides.
- zsh does not word-split `set -- $p`. Write one call per file.

**Keeping overlays off a crossfade (t27#10614, PR #10627).**

- Work in whole frames, not seconds. Give each scene a function for the frames it owns outside its crossfades, clip each overlay to it, and print exactly those frames as `between(n,lo,hi)` terms joined with `+`. A test checks that the printed frames equal the frames `between(t,a,b)` showed before.
- Drop a piece that has no room; never squeeze it. Name the cost in a test: count what is dropped, assert that each drop is on a crossfade, and assert that every phrase keeps at least one caption.
- To check the pixels, render lossless from solid-colour clips. Crop a full-width band 40 rows tall over the text, split the raw file per frame, and count the distinct colours per frame with `od -An -tu1 -w3 -v | sort -u | wc -l`. Crossfade frames must show 1. The frames with text are the control: they must be many.

Traps from this tick:

- A minimum-length rule silently drops spoken captions. A short chunk that straddles a crossfade leaves two pieces under the minimum, and the voice speaks words that are never shown. A grid test that says "no overlay on a crossfade" stays green while this happens. Only a count of what was dropped finds it.
- Lab scripts run under `sh`, which is dash: `$((10#$x))` fails with "expecting EOF". Strip leading zeros with `sed 's/^0*//'` and default with `${i:-0}`.
- The Mac hook reads the whole Bash command. A heredoc PR body that names the report command gets refused. Write the body with the Write tool and pass `--body-file`.

**Sweeping every crossfade frame, not one band of one lesson (t27#10651).** A band check on one lesson proves one lesson. To cover every lesson, run a whole-frame sweep:

1. For each lesson, language, voice and orientation, print the graph and write the text files it reads (`textfile=` gives the directory).
2. Take each `offset=` and compute the crossfade's first frame in integer ms: ceil(offset x 30).
3. Append `;[v]select='between(n\,s\,s+15)+...',scale=W/4:H/4:flags=area,format=rgb24[w]` to the graph.
4. Map `[w]` to `-fps_mode passthrough -f rawvideo`. Map `[a]` to `-f null -`, because the graph must still consume its audio.
5. Feed it three solid `color=` clips, split the raw file per frame and count the distinct colours of each frame.

Every frame must have exactly 1 colour: area scaling turns a blend of two solid clips into one colour, so anything drawn on top adds more. Run the parent binary as the negative control; it must flag frames.

Measured for mode 5 with PR #10647's binary: 27 lessons x 2 languages x 3 voices x 2 orientations is 324 renders and 10368 crossfade frames, and 0 frames were flagged. The #10632 binary was flagged on all 162 tall renders, 1350 frames in total. On voices 1300/4200/6800 and 700/4200/8000 it flags frames 20..31: the last card sits over crossfade 1. On 900/2600/4100 it flags only frame 31. A control that flags a single frame catches bugs that a check of one hand-picked frame would miss. The sweep takes about 5 s per render, 40 min in total.

Traps from this tick (ffmpeg on the lab):

- Pass `-nostdin`. Under `nohup ... < script`, ffmpeg reads the script as keyboard commands ("Parse error"), and the render stops after 16 or 17 frames.
- The lab cgroup has `pids.max` 1000, and a graph with many `drawtext` filters starts 400 to 700 threads. Cap them with `-threads 2 -filter_threads 2 -filter_complex_threads 2` and `-threads 1` on the output. Never run two renders at once. "ff_frame_thread_encoder_init failed" means the cap was hit. Look for a stuck ffmpeg and kill it by pid.
- lavfi rejects `d=.900`. Print the duration as `$((m/1000)).$(printf %03d $((m%1000)))`.
- `pkill -f` on the loop script leaves its child renders running. Find them with `ps -eo pid,args | grep '[s]h xf'` and kill them by pid.
- Through `fs-lab.sh`, `pkill -f 'why_seal'` also matches the ssh command line that carries it and kills your own session; the output comes back empty. Bracket the first letter (`pkill -f '[w]hy_seal'`) so the pattern does not match itself. A killed session leaves the detached ratchets it started running, so several end up writing one log; list them with `ps -eo pid,etimes,args | grep '[c]orpus-only'` before you read a verdict.

**Holding the address when the voice runs long (t27#10629).** If an overlay must stay on screen for a minimum time and must also stay off a crossfade, then the crossfade has to move; the overlay cannot. Work out in frames the latest cut that still leaves the last scene its minimum: cut 1 is capped at `BEAT_CUT1_LAST_MS` = 13249. Prove it is the latest with a one-ms-later control. Only the voices the reel speeds up are affected, so the `cmp` diff is limited to those voices and to crossfade 1's offset. The cost is real: the crossfade starts up to 184 ms under the task's last word, and lesson 1's measured voice (1300/4200/6800) is one of the affected voices. Write a test that bounds that overlap.

- Before trusting a "fast voice" grid result, work out which fixture voices are fast. `voice_room_ms(3)` is 12250 ms, so the measured lesson 1 voice (raw 12300) is already sped up. A change aimed at "fast voices" therefore changes the reel everyone actually sees.

**Letting a caller reach a printer in another spec (t27#10628).** When an overlay's `enable=` is printed in an imported spec, the importing spec cannot clip it. Split the printer into a form that takes the window (`print_alpha(lo, hi)`, `print_rise_from(o, lo, dy)` in token_floor_mux) and the old form as a thin wrapper that passes the spec's own window (`print_fade(c)`, `print_rise(o, c, dy)`). Then run `cmp` on the imported spec's own graph: it must be byte-identical (token_floor_mux: 28336 bytes, 0 differences). Run `cmp` on every mode of the importer that does not use the window too (modes 3 and 4: 0 of 162 graphs differ).

- Move the ramp along with the window. A card's rise and fade-in start at its first kept frame (`beat_first_lo`), and its fade-out ends at its last kept frame (`beat_last_hi`). If you clip only `enable=`, the card pops in fully risen on the close beat's first frame, because the ramp started on frame 392, under the crossfade.
- A window can end past the reel. `CARD_ON` gives the last card frames 392..450, but the reel's last frame is 449. Clamp the window (`card_hi`) and assert the clamp in a test.

## Show a spec (the core: exporter for what it reads, by hand for the rest)

1. Load the spec: `mcp__t27__t27_spec` with `path` and `source: true`, or the file from a
   gHashTag/t27 checkout. Say which one you used.
2. List nodes and edges from the source only: module, imports (`use`), constants, functions,
   `test` / `invariant` / `bench` blocks, calls between them, which tests exercise which function.
   Give every node and edge its line span and its label (`exact` / `derived` / `suggested` /
   `unknown`). The kinds, labels and rules are those of `specs/t27f/spec_graph.t27`; read them
   there, and check the reading against its `validate_graph` rules (its `tutorial_08_graph`
   fixture is a worked example).
3. Show it as a table (node, kind, span, label) and, if useful, a diagram whose every arrow is a
   row of that table. Nothing is drawn that the table does not back.
4. Say plainly that this is a manual reading unless a graph came out of `export_graph`
   (`specs/t27f/spec_graph_export.t27`), with `derive_graph` (`specs/t27f/spec_graph_derive.t27`)
   on top. The exporter reads declarations, spans and exact CALLS / USES / CONSTRAINS edges only,
   within 1024 nodes and 4096 edges; the derive pass adds DERIVED tested_by and data_flow edges
   and UNKNOWN opaque nodes for undeclared qualified call targets, by its three stated rules.
   Anything the rules do not reach (SUGGESTED explanations, call-result flow, anything larger)
   is still a manual reading, and no host runs either pass yet. Name that gap rather than hide it.
5. For the specs **around** a spec (what it imports, who imports it), run `t27c use-edges
   --specs-dir specs` on the lab and lay it out with `deps_view` in `specs/t27f/spec_deps.t27`.
   Trap: `tsort` prints loops on **stderr** and still prints an order on stdout, so "tsort gave
   an order" is not "acyclic". An earlier issue body said there was no cycle, and the
   spec's corpus run found igla race `backend.t27` <-> `rtl.t27`. Read stderr, or use the
   spec's `cycle=` flag. That cycle is broken in #10689. Since then, a loop on stderr is a new
   regression.
6. Breaking a cycle the view finds. First count the names that cross the back edge. In #10689,
   one invariant used one function, so the invariant moved to the side that already had the
   import, and nothing was copied. Then compare `test-report` against **master**, not against
   "passes". The igla race specs are BLOCKED at a zig error on master, so the right claim is
   "the same verdict as before", not "the tests pass".
   Then run the corpus ratchet on the lab (`t27c suite --repo-root . --ratchet --corpus-only`).
   `test-report` cannot see text moving between two specs that the ledger pins. #10689 moved an
   invariant the parser discards, so 15 tokens moved from `rtl` to `backend`. The ratchet went
   red (`DISCARD WORSENED` +15 on one spec, `IMPROVED` -15 on the other), and the gate merged it
   red anyway (#10695, fixed in #10699). To re-pin, run `--bless-expectations` to measure, then
   copy only the moved numbers into master's file: the bless also deletes the
   `_why_entries_fell` history field.
7. Before you open a PR that adds a spec, run `python3 tools/dupe_scan.py` on the lab. The
   issue gate auto-merges non-draft PRs even when a check is red: #10681 merged with a copy of
   `lit_len` that was red on `duplicate-bodies` until #10688. Reuse the existing body through
   `use`, and do not `--bless` a copy you made by accident.

## Visual in every output (Claude Code and TRI CLAW)

**The rule is `specs/t27mcp/visual_output.t27`** (its `RULE_CLAUDE_CODE` and `RULE_TRI_CLAW` lines
are what each host reads). **Its particle-still layer is `specs/claw/visual_output.t27`.** This
section is the order of steps; which answer carries which visual and how it reaches each host are
the specs', not repeated here.

1. Decide the answer's kind (`K_*` in the t27mcp spec). A one-sentence reply or a question back
   goes without a visual.
2. Spec, flow or numbers: draw from t27f tool rows (`t27f_outline`, `t27f_graph`, `t27f_view`),
   labelled, with the legend and the source file, as `answer_verdict` checks.
3. A report of a piece of work: a particle still (`takes_still`). Render the reel on the Railway lab
   (never the Mac), take the frame at `STILL_MS` with `ffmpeg -ss 9 -frames:v 1`, and bring it into
   the session folder (`out-reels/`, gitignored, never `git add`).
4. Anything not made: say which and why in the answer. Never show an older file as this turn's.

## Application: topic map (gHashTag/trinity#1677)

Use this part when the user gives a topic or a client brief, not a spec.

**Its rules live in one place: `specs/t27f/topic_engine.t27`.** This section is the order of
operations around it. It never restates a constant, a vocabulary or a gate; it reads them. If
this text and the spec disagree, the spec wins and this text is the bug.

### 0. Load the engine

Read the spec through the t27 MCP server, which is already installed in Claude Code:

- `mcp__t27__t27_spec` with `path: "specs/t27f/topic_engine.t27"` and `source: true`.
- Until the spec is published to t27.ai, the MCP answers "not found": read the file from a
  gHashTag/t27 checkout instead. Say which one you used.

From the source, take: `LENS_NAMES`, `LENS_QUESTIONS`, `STAGE_NAMES`, the four `*_WORDS`
vocabularies and `KEYWORD_MATCH`, the five `*_BEES` lists, `MAX_ACTIVE_BEES`, `GAP_BEE`, and the
functions `intake_verdict`, `map_verdict`, `initial_status`, `ground`, `route`, `dispatch_allowed`,
`bee_target`, `required_evidence`, `evidence_accepted`, `verify`, `counts_as_progress`,
`needs_human_approval`, `action_allowed`, `after_action`. Evaluate them as written; they are small and total.

Everything in `DETERMINISTIC_STEPS` is decided by those functions, never by your judgement.
You write only what `LLM_WRITES` names.

### 1. Intake

Ask for, or take from the message: the topic or brief, and its sources (links, files, notes).
Apply `intake_verdict(has_topic, source_count)`:

- `INTAKE_EMPTY`: ask for the topic and stop.
- `INTAKE_NO_SOURCES`: continue, but every lesson starts as a GAP. Say so up front.
- `INTAKE_OK`: continue.

### 2. Route

Count the brief's hits against each `*_WORDS` list exactly as `KEYWORD_MATCH` says. Show the four
counts and the words that hit, then apply `route(content, engineering, research, sales)`. Name the domain and the Bee list
it allows. Do not override the result; if it looks wrong, the vocabulary in the spec is what needs a
PR.

### 3. Three lenses

Write one short paragraph per lens, answering that lens's question from `LENS_QUESTIONS`, using only
the sources. A claim with no source is not written; it becomes a gap.

### 4. The 27-lesson map

Print a table of 27 rows, lens-major (`step_lens`, `step_stage`): step, lens, stage, lesson title,
source, status.

- For each lesson, `grounded` means at least one source from intake supports it. Sources you find
  yourself are not intake; they arrive through the Bee task in step 5 and `ground`.
- A GAP row keeps its lesson title (titles are in `LLM_WRITES`); its source cell is empty.
- Status from `initial_status(grounded, first_grounded)`; `first_grounded` is the grounded lesson
  with the lowest step. A GAP row's status cell says **GAP: needs a source on <what>** and nothing
  else. Never invent filler to make the map look complete.
- Count grounded lessons per lens and report `map_verdict(...)`.

### 5. One Bee task

Apply `bee_target(open_lessons)`. `TARGET_OPEN_LESSON`: take the first OPEN lesson.
`TARGET_FIRST_GAP` (nothing is OPEN, as when intake was `INTAKE_NO_SOURCES`): take the first GAP, the
capability is `GAP_BEE`, and the task is to find a source for it; once one is attached, apply
`ground`. `active_bees` counts the Bees of this journey still working; at the start it is 0. Check
`dispatch_allowed(active_bees)`. Choose one capability from the
domain's Bee list and write one task for it:

- goal (one sentence), input (the lesson and its sources), output (the artifact),
- the evidence that will verify it: `required_evidence(stage, domain)`.

In this phase the Bee is you, in this session, or a delegated Agent if the user asks for it. Never
start more than `MAX_ACTIVE_BEES`, and never one per lesson.

### 6. Draft, then verify

Produce the artifact as a DRAFT and label it as one. Collect the evidence (a source link, a passing
test, a review, the client's approval). Apply `verify(DRAFT, stage, domain, evidence)`.

- Only VERIFIED or PUBLISHED counts: `counts_as_progress`. Report the progression event as
  `+POINTS_PER_VERIFIED_LESSON` point, and nothing else. `REWARD_IS_MONEY` is false: never describe
  points as money, tokens, payouts or value.
- `opens_next(DRAFT)` is true, so the learner may move on while the draft waits for evidence.

### 7. Approval gate

Before any action, check `needs_human_approval(action)` and `action_allowed(status, action,
approved)`.

- Publish, send to a client, any payment, anything destructive: ask the user in chat and wait for a
  clear yes. A yes covers that one action only.
- Publishing needs a VERIFIED item. A draft is never published. Sending to a client also takes a
  DRAFT, because a client approval is the only evidence that verifies a sales or content DELIVER
  lesson; the draft stays a DRAFT (`after_action`) until that approval comes back.

### 8. Keep the three layers apart

Keep drafts, verified results and published items under separate headings (`layer_of`). If the
journey is saved, save it as a `.t27` data module, not JSON. Saving follows the user's instruction.

## Application: content factory, beats (t27#10584)

`specs/t27f/reel_beats.t27` plans every reel of course 0 as three beats: open, main and close.
Main is `arche(t)` from `reel_factory.t27`. Open is the best other construct, with a generic
fallback. Close is a rotating generic. No two reels share the ordered pair (open, main), and
neighbouring reels differ in every beat.

Before you change a uniqueness rule, measure what it can draw from:

- **Count the pool before you promise uniqueness.** 21 of 27 course-0 lessons have content only
  for graph and tests among the constructs. This is measured, not a detection bug: course 0's
  snippets have no `while` or `if`. Six archetypes make only 15 unordered pairs for 27 reels,
  so an unordered rule fails 4 of 7 tests. Ordered pairs (30) hold. Print the content matrix
  (lesson by archetype `content()`) first.
- **A generated printer needs a test block.** `t27c gen` emits `const std = @import("std")`
  only when the module has a test. Without one, `std.debug.print` fails in zig with "use of
  undeclared identifier 'std'". Give a lab-only printer one trivial `test`.
- **Keep scratch printers on the lab.** Copy them into `specs/` for `t27c gen`, then `rm` them
  in the same command. Never commit them.
- **Write PR bodies with the Write tool.** The Mac hook matches words in a
  shell heredoc, such as a PR body that mentions tests. Write the body with the Write tool, then
  pass `--body-file`.

### Preview the factory's stills as a video (t27#10571)

The owner asks to see intermediate videos, not only numbers. Show one every time a renderer changes.

1. **Write a lab-only printer.** It needs a `main` that loops `t` over `TOPICS`. For each lesson it
   takes `arche`, `accent`, `motion`, `layout` and `content(t, a)` from `reel_factory`. It then
   prints a plain PPM (`P3`, 360 x 640) of `scene_still.pixel(...)` through `scene_raster`'s
   `red`, `green` and `blue`. Give it one trivial `test`.
2. **Generate and build.** Run `t27c gen` on it, then `zig build-exe -lc -O ReleaseFast`.
3. **Run it.** `./d 2> all.ppm`: `std.debug.print` writes to stderr. All 27 stills take 4 s and
   56 MB.
4. **Split and convert.** Split the dump on each `P3` line with awk. Convert each part to PNG with
   ffmpeg, which reads P3.
5. **Make the video and the sheet.**
   - Video: `ffmpeg -framerate 1.25 -i s%02d.png` with
     `-vf scale=720:1280:flags=neighbor,fps=30,format=yuv420p` and libx264. This gives 21.6 s.
   - Contact sheet: `-vf scale=120:213,tile=9x3 -frames:v 1`.
6. **Fetch them.** Bring both back wrapped as `B64:...:END`, and compare sha256 on both ends.
   Put them in the session's notes folder, never in the repo or `out-reels/`.

Measured at 355d44a39: each still has 9 to 14 flat colours, and the PNGs are 4 to 32 KB.

**Traps:**

- **A Mac session may not read images.** A hook can block image reads because the upstream
  provider refuses them. Check the stills by numbers (colour count, file size, ffprobe) and open
  the file for the owner in the file pane.
- **WIP commits can sink a PR.** A WIP commit whose message names `#N` without a linking keyword
  fails L1. `l1-traceability.yml` checks every non-merge commit, and the history cannot be
  rewritten. Carry the work instead:
  1. Make a new branch from master.
  2. `git read-tree -m -u HEAD <merged-branch>`.
  3. Make one commit with `Closes #N`, open a new PR, and close the old one with a pointer.
- **Merge in the right direction.** Merging master into an old branch can trip the pre-commit
  `foreign-budget` hook on master's own Rust lines (t27#10414). Merge the old branch into a
  master-based branch instead. The result still fast-forwards the old branch.

## Application: idea factory, the OS particle design system (t27#10771)

The owner wants one factory that unpacks any idea in any style preset, with every element made
of particles, and runnable in the OS itself and on FPGA. Two specs do this:

- **`specs/t27f/particle_ds.t27` is the design system.** A preset is a column in integer tables:
  `BG`, `COL_A/B/C`, `RADIUS`, `GAIN`, `SPRING`, `DAMP`, `LAW`, `FORCE`, `KICK`, `CARD`. The five
  are noir, neon, trit, paper and rain. **Add a preset by adding a column, never a branch in code.**
- **`specs/t27f/idea_factory.t27` is the factory**, `(IDEA, PRESET) -> RGB24 frames`. It wraps
  (at most 9 characters per line, 4 lines per scene, `|` cuts a scene), lays out in glyph57's 5x7 font,
  assigns particles to glyph sub-slots and the card border, and closes on `CTA`.

**Keep it FPGA-shaped.** Use Q8 fixed point. The per-step and per-pixel paths use only add,
multiply, compare, table lookup (`kt` kernel ROM, `tt` tone ROM) and power-of-two division.
Divide by a variable at most once per scene cut, never per pixel. The seals then carry
`gen_hash_verilog`. That proves Verilog is emitted, not that it was synthesised. Say so.

**Render the preset sweep on the lab:**

1. For each preset, `sed` the `PRESET` constant. Also rename the module (for example
   `idea_factory_p`), so the gen output does not overwrite the committed module's.
2. Run `t27c gen`, then `zig build-exe -O ReleaseFast`.
3. Run `./f 2>&1 1>fo.txt | ffmpeg -f rawvideo -pix_fmt rgb24 -s 1080x1920 -r 30 -i - ...`
   (libx264, crf 19). The frames go to stderr. Send stdout to a file, **never to `/dev/null`**:
   the lab replays `/dev/null` writes.
4. Concatenate the five, scale to 720 and make a tile sheet. Fetch them as `B64:...:END` and
   check sha256 on both ends.

Measured: each preset takes 14 to 35 s for 216 frames, with 3072 particles.

**Check legibility by numbers, because the Mac cannot read images.** Crop the settled frame at
t = 2.2 s with ffmpeg and read `signalstats` YAVG for three regions: the text block, a gap row
between lines, and the empty bottom. On a dark preset you want block > gap > bottom (rain:
44.8 > 25.4 > 19.0). On paper the order flips (block 187 < gap 211). A flat triple means the
particles never settled into letters.

**Traps:**

- **Read the `test-report` tail, not a grep.** A grep for pass and fail counts miscounts the
  summary lines (`tests N`, `pass N`, `FAIL 0`, `vacuous passes 0 of N`).
- **Rename loop variables by line range.** A rename that only changes the declaration leaves the
  body pointing at the old name. Give each top-level loop in `main` its own name (`ia`, `ib`,
  `ic`).
- **Base the branch on master's tip before the PR.** If the base PR (R3) merged first, reset the branch onto
  `origin/master`. Untracked new specs survive the reset.

### Russian text: UTF-8 and the 5x7 font (t27#10788)

`specs/t27f/glyph57.t27` gives the factory Cyrillic. It decodes UTF-8 (`cp_len`, `cp_at`,
`cp_count`), folds lower case to capitals, and stores a 5x7 font. The font holds ASCII 32..95
and only the 21 Cyrillic capitals that have no Latin twin. The 11 look-alikes (A, Ve, Ie, Ka, Em,
En, O, Er, Es, Te, Ha) and Yo reuse the Latin glyph through the 32-entry `CYR_MAP`.

- **Write non-ASCII text as a `u8` array, never as `"\xNN"`.** L3 keeps sources ASCII. The t27c
  lexer lowers only `\n`, `\t`, `\\` and `\"`, and keeps `\xD0` as four characters, so a
  `\xNN` test fails with no error. Use `pub const RU_IDEA : [80]u8 = [208, 167, ...];` and pass
  `RU_IDEA[0..80]` to a `[]const u8` parameter. `specs/tri/utils/terminal.t27` records the same
  trap for `\x1b`.
- **Count widths in characters, keep positions in bytes.** Wrapping compares `cp_count` with
  `MAXC`. The line start and end stay byte offsets, and the layout loop steps by
  `bi = bi + cp_len(t, bi)`. An ASCII idea's numbers do not change, so the old tests still pin
  the old layout.
- **Guard the font with a duplicate-bitmap test.** Every pair of stored glyphs must differ, and
  every look-alike must light exactly its Latin twin's pixels.
- **Render a Russian idea without touching the committed constant.** In the lab copy, `sed`
  `reel_frames(IDEA)` and `reel_cell(IDEA,` to use `RU_IDEA[0..80]`, next to the `PRESET` and
  module renames.

## Check the whole corpus for import cycles (t27#10693)

dependency-cruiser's `no-circular` and madge's `--circular` list every module on a cycle.
`deps_cycles` in `specs/t27f/spec_deps.t27` gives the same answer for the `use-edges` list.
No t27c subcommand runs a main (t27#10692), so the only host today is a scratch test on the
Railway lab. Do not commit it.

1. On the lab, write `t27c use-edges --specs-dir specs` to a file **before** you put the
   scratch spec in `specs/`. Otherwise its own `use` lines join the list, which is harmless but
   changes the count.
2. Build the scratch spec from three parts:
   - a head: `use policy::text_lines; use ci::affected; use claw::t27f_gate; use
     t27f::spec_deps;`, then a test that opens `const g : str = "`;
   - the edges, converted with `sed 's/\t/\\t/g; s/$/\\n/' | tr -d '\n'` (in an `fs-lab.sh`
     string the backslashes are doubled again);
   - a tail that closes with `$";` and asserts on the result.
   Room: the 2462-line corpus fits `set` 16384, `done` 65536 and `out` 4096.
3. Assert `w <= cap` first, so that a refusal is told apart from a wrong list. Then run a
   **negative control**: the same edges with the opposite expectation must FAIL. On master
   f7efbc232 the list was exactly `backend.t27` and `rtl.t27`. With #10689 it was empty, and
   the control that expected the pair failed.
4. Remove the scratch spec, and `git checkout` any spec you swapped in, before you seal
   anything.

### Check that the unknown imports match t27c (t27#10702)

`deps_unknown` has an oracle. `t27c gen <spec>` prints `warning: <spec>:<line>: use X resolves
to no spec` on stderr for every `use` line it drops (#7176), so the two lists must agree byte
for byte.

1. On the lab, before any scratch spec is in `specs/`, write three files:
   - the gen warnings over every spec (`t27c gen $f > o.zig 2>> warn.txt` over
     `git ls-files 'specs/*.t27'`; never write to or read from `/dev/null` there);
   - the `use-edges` list;
   - every `use` line as `spec<TAB>line` (`grep -E '^[[:space:]]*use '`).
   Check that none of them holds `"`, `\` or `$` before you embed it.
2. The scratch spec copies each use line into a small buffer and calls `deps_unknown` with that
   line's spec as the focus. It prefixes each output line with the spec and expects
   `spec<TAB>unknown<TAB>X` in `git ls-files` order, which is the order gen ran in.
3. Controls: the same list minus one line must FAIL, and so must `w == 0`. On master f33f269d5
   it was 23 lines in 13 specs (#7176 had counted 119 in 70), and both controls failed.
4. All 13 of those specs fail `t27c test-report` for other reasons before the dropped `use`
   matters (`std::*` imports, an undeclared `gf32_from_f64`, `Float`, Rust syntax). Repairing a
   dropped import alone therefore fixes none of them. Classify a spec before you promise a fix.

### Check one chain per reached spec (t27#10705)

`deps_why a b` answers the question that `cargo tree -i`, `npm explain` and `pnpm why` answer:
why does a import b. It writes one shortest chain of edge-list lines. The corpus check holds it
to three properties:

1. Over the `use-edges` list, call `deps_why focus x` for every spec x the focus reaches. The
   chain must have exactly `layer(x)` hops, every hop must be an edge-list line, and each hop
   must start where the last one ended: at the focus first, at x last. Count the specs checked
   and compare with `deps_count` of the walk, so no spec is skipped.
2. Control: expect one hop more (`zz_check(1)`). It must FAIL.
3. Mutation: drop the last edge from the chain. Two unit tests must FAIL.

Build the scratch spec by concatenation (head of the template, the escaped edges, then the
tail). A `sed s#@EDGES@#...#` with 2466 lines fails with "Argument list too long". A focus with
no outgoing edge, such as `verified/reuse`, checks only itself, so `> 1` fails for it; that is
the probe, not the function.

### Draw the chain as a card (t27#10716)

`specs/t27f/why_card.t27` turns the chain into an ffmpeg graph. Its main reads the edge list on
stdin, plus `WHY_A`, `WHY_B` and `WHY_FONT`. Skill card: `t27/why-card`. On the lab:

1. `t27c gen specs/t27f/why_card.t27 > why_card.zig`, then compile it with zig and `-lc`.
2. `t27c use-edges --specs-dir specs | WHY_A=... WHY_B=... WHY_FONT=/root/tf/fonts/jbm-all-800.ttf ./why_card 2> g.txt`.
3. Render with `ffmpeg -nostdin -threads 2 -filter_threads 1 -filter_complex_script g.txt -map '[v]' -frames:v 1 card.png`.
4. Check pixels, never by eye. Crop one pixel (`crop=1:1:x:y`) to rgb24 and compare: box fill
   at (106, top+6), stroke at (101, top+48), fill at both inner edges (110 and 970) so a name
   never reaches the stroke, backdrop in each gap, the green connector at x=540. Count
   `ff ff ff` in each box's text band so every name is drawn, with 0 below the column. Add one
   deliberately wrong expectation that must MISS.

On 8aec8e259 (2474 edge lines): from `why_card` 20 chains drew, up to 7 boxes (to `verified/device_dna`, 6 hops),
and all 62 pixel checks held.

Traps:

- **Do not import a print module for one constant.** `use t27f::lesson_reel::FONT` pulls
  lesson_reel's whole scene tree into the build, and zig stops on `FLOOR_KEY` undeclared. Pass
  the value in (`WHY_FONT`) instead of copying it.
- **`t27c gen` drops an open slice end** (t27#6250): `buf[n..].ptr` becomes `buf[n].ptr`, and
  zig says "type 'u8' does not support field access". Write `buf[n..cap].ptr`.
- **A count check can hide behind a later check.** `k != hops` mutated to `k > hops` survived
  until a fixture had fewer edges than `hops=` *and* still ended at B. Give every refusal a
  fixture that only that refusal catches.

### Draw who uses a spec as a card (t27#10720)

`specs/t27f/deps_card.t27` draws one spec's neighbourhood. Its first three direct importers sit
above it and its first three direct imports below it, with `+N more` past three and the
transitive counts in the footer. A gold `ON AN IMPORT CYCLE` line appears when `deps_on_cycle`
holds. It imports why_card's `card_*` helpers rather than copying them, and its main reads
`DEPS_SPEC` and `DEPS_FONT`. Skill card: `t27/deps-card`. To build and render it, follow the
why-card recipe above with `deps_card` in place of `why_card`.

Pixel checks per row:

- fill at (106, y+6);
- stroke at (101, y+32);
- fill at both inner edges (110 and 970);
- backdrop in the gap at (300, y+70).

Rows start at y = 280 + 76k above the focus and 820 + 76k below it. Then check:

- the focus fill at (112, 650) and its 8 px stroke at (103, 688);
- the connector at (540, 630) above and (540, 745) below, or the backdrop at those points when a
  band is empty;
- the white-pixel count in every row's text band.

On 255910235 (2483 edge lines, 813 specs named):

- 783 cards drew. `base/types` is the hub: 378 direct importers, reached by 400.
- 30 were refused (all drawn since t27#10728, below):
  - 29 because the spec's own name is over 58 bytes (`port/trios/...`);
  - 1 because a shown neighbour's name is.
- No spec in the corpus is on an import cycle.
- 70 of 70 pixel checks held on the hub, leaf and mid cards.

Traps:

- **A shell helper that sets `tag` clobbers the caller's `tag`.** POSIX `sh` has no `local`:
  `rows()` setting `tag=$4` sent every later check to `up.png`, which does not exist. Name each
  helper's variables apart.
- **`sort -t "$TAB" -k3,3n -r` sorts ascending.** A key with its own ordering flag does not
  inherit `-r`, so write `-k3,3nr`. In `sh`, `$'\t'` is two characters; use `TAB=$(printf '\t')`.
- **A one-byte-short fit test cannot tell `w > cap` from `w > cap + k`.** Both return `cap + 1`
  there. The original and the mutant only differ when the room is 2..k+1 bytes short, so the
  test that kills the mutant is three bytes short, not ten.
- **Bands were in edge-list order, so a hub showed `account/*` first.** Since t27#10762 (below)
  they are ranked by how many specs import each row. Even so, do not caption the first three as
  the most important: "most imported" is the only measure the card has.

#### Long names are abbreviated, not refused (t27#10728)

A shown name of at most 58 bytes is drawn byte for byte. A longer one is drawn as its first
segment, `/...`, and the longest run of whole trailing segments that fits
(`port/.../rings/CR-CHAT-03/src/confirmation_tag_chain`). When no whole run fits, it is drawn as
`...` and its last 55 bytes. Only drawn bytes are checked, and the text size follows the drawn
length. `why_card` is unchanged and still refuses a name over 58 bytes.

On 5440d35e1 the sweep compares the new binary with master's, card by card:

- 30 cards are newly drawn, 783 are byte-identical to master, and 0 are lost or refused;
- 0 changed cards and 0 newly drawn cards have no `...`, and 0 identical cards have one;
- 29 abbreviated forms are drawn: 27 `/.../` and 2 `...`, the longest 58 bytes;
- the helpers card (29 importers), a `/.../` focus and a `...` focus passed every pixel check,
  with 0 white pixels outside x 100..980.

Lab traps from this run:

- **`pkill -f dn_lab.sh` kills the fs-lab command that runs it**, because the pattern is in that
  command's own line. Write `pkill -f '[d]n_lab.sh'`, in an fs-lab call that names the script
  nowhere else (see #10762's traps).
- **An untracked seal in the lab checkout makes `git checkout` fail, and the script then tests the
  old commit.** Have the script take the commit it must test and stop with `WRONG COMMIT` when
  `HEAD` differs. Once the PR adding a seal merges, that seal is tracked, so `rm -f` of it deletes
  a tracked file: restore it with `git checkout --` before sealing.
- **Count a kill only on `^ ?FAIL [a-z]`.** The `FAIL 0` summary line otherwise counts as a failing
  test, and a surviving mutant reads `KILLED by 1: 0`.
- **A `<` mutant of a 58-byte bound needs an unquotable byte that only the whole-name check
  sees.** A 58-byte name with `:` in the middle is `true` under the mutant, because the dots form
  hides that byte, and `false` under the rule.

#### Bands rank the most imported first (t27#10762)

Each band draws its specs by how many distinct specs import each of them in the whole edge list,
highest first. A repeated edge line counts once, and equal scores keep the edge-list order, so a
band of equal scores draws exactly what it drew before. The subtitle says
`most imported first in each band`. The direct counts, the transitive counts and `+N more` do not
change: only which names are drawn, and in what order. "Most imported" is the only measure; the
ranking is quadratic in the edge list, and the hub is the largest band measured (539 ms, 378 importers).

On 7853e2ef4 the sweep compares the new binary with the #10728 binary, card by card:

- 124 cards are reordered, 689 are the same, and 0 are lost or refused;
- 0 cards have a count that differs, and 0 drawn rows score higher than the row above them
  (1647 rows checked; 3 rows have abbreviated names, so their scores are unknown);
- the hub `base/types` now shows `compiler/lexer`, `compiler/parser` and `igla/race/bram_weights`
  in place of three `account/*` specs, and both it and `analog/linear` passed every pixel check;
- 21 of 21 mutants fail a named test, including the tie order, the repeated line, out-degree in
  place of in-degree, and the hidden-name check run in edge-list order.

Traps from this run:

- **ffmpeg splits drawtext on `:` and `,` even inside the quoted text.** A subtitle with `:` made
  ffmpeg refuse the whole graph ("Both text and text file provided"), and every test still passed,
  because the tests compare the graph text and do not run ffmpeg. Every fixed text now passes
  `dcard_text_ok`, which has a test and a negative control. Only a render catches this class.
- **`pkill -f '[r]k.sh'` still kills the fs-lab command if that same command line also says
  `/root/rk.sh`.** The bracket trick only hides the pattern from itself. Run the pkill in its own
  fs-lab call, then send and start the script in the next one.
- **`seal --verify` does not check the closure.** After a master merge changes an imported spec,
  `--verify` still says MATCH, and frontier refuses the seal (#10749). Re-save the seal after any merge
  that touches the closure.

#### Long names are abbreviated, not refused (t27#10728)

A shown name of at most 58 bytes is drawn byte for byte. A longer one is drawn as its first
segment, `/...`, and the longest run of whole trailing segments that fits
(`port/.../rings/CR-CHAT-03/src/confirmation_tag_chain`). When no whole run fits, it is drawn as
`...` and its last 55 bytes. Only drawn bytes are checked, and the text size follows the drawn
length. `why_card` is unchanged and still refuses a name over 58 bytes.

On 5440d35e1 the sweep compares the new binary with master's, card by card:

- 30 cards are newly drawn, 783 are byte-identical to master, and 0 are lost or refused;
- 0 changed cards and 0 newly drawn cards have no `...`, and 0 identical cards have one;
- 29 abbreviated forms are drawn: 27 `/.../` and 2 `...`, the longest 58 bytes;
- the helpers card (29 importers), a `/.../` focus and a `...` focus passed every pixel check,
  with 0 white pixels outside x 100..980.

Lab traps from this run:

- **`pkill -f dn_lab.sh` kills the fs-lab command that runs it**, because the pattern is in that
  command's own line. Write `pkill -f '[d]n_lab.sh'`.
- **An untracked seal in the lab checkout makes `git checkout` fail, and the script then tests the
  old commit.** Have the script take the commit it must test and stop with `WRONG COMMIT` when
  `HEAD` differs. Once the PR adding a seal merges, that seal is tracked, so `rm -f` of it deletes
  a tracked file: restore it with `git checkout --` before sealing.
- **Count a kill only on `^ ?FAIL [a-z]`.** The `FAIL 0` summary line otherwise counts as a failing
  test, and a surviving mutant reads `KILLED by 1: 0`.
- **A `<` mutant of a 58-byte bound needs an unquotable byte that only the whole-name check
  sees.** A 58-byte name with `:` in the middle is `true` under the mutant, because the dots form
  hides that byte, and `false` under the rule.

## What this skill does not claim

The graph IR is a schema (`specs/t27f/spec_graph.t27`) and `specs/t27f/spec_graph_export.t27` turns source into it, but only exact declarations, spans and edges within the IR's capacity, with `specs/t27f/spec_graph_derive.t27` adding the derived and unknown part by stated rules, and no host calls either yet; no renderer draws a T27F view in the Trinity UI, no
runtime dispatches Bees from a spec, and nothing publishes on its own. If a step needs one of
them, say that it is missing; do not simulate it.
