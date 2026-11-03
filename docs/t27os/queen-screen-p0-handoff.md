# P0 handoff — the Queen screen from the view factory (2026-11-03)

From: lane `lane/t27os-dsh-panel`, worktree `~/.claude/worktrees/silly-thompson-a50ffa`, HEAD `fecaa9f40b`.
To: the P0 agent (the owner's next assignment).
Status of everything below: **candidates, not approved** (ui_snapshot.t27 R7 — the author does not approve).

## What P0 gets: a working render pipeline, not a dashboard to draw by hand

The audit (2026-11-03) found the factory's stages already exist as tested laws. The Queen screen
can be produced by the same chain the web graph page uses — **no new hand-drawn dashboard**:

```
domain model (already in the window: lanes table sst[SST_LV..], executor_registry rows,
              kernel rows, pane listing, plan.txt picks)
  → model fold        specs/t27f/work_graph.t27::apply_event  (or a thin adapter)
  → scene producer    specs/t27f/hud.t27::hud_scene           (WorkGraph → Scene, EXISTS)
                      specs/t27f/spec_scene.t27               (SpecGraph → Scene, NEW, first-7 window)
  → scene IR          specs/t27f/scene.t27 (32 slots, closed vocabulary, delta ops)
  → render            specs/t27f/display_list.t27::flatten_into → live + export  (EXISTS, wired)
  → observability     specs/t27os/ui_snapshot.t27 (UiSnap; glass_window ui_fill/ui_dump wired in
                      the worktree as the A+ candidate — see "unfinished" below)
  → interaction       specs/t27f/viz_hit.t27 (shared hit-test, NEW) + viz_contract VizState/viz_apply
  → representation    specs/t27f/viz_select.t27::vs_select (NEW: intent+shape → VIZ_REP composition,
                      hard refusals: hairball 3x, matrix 400 cap, provenance gate on SOURCE)
  → theme             specs/t27f/viz_theme.t27 (NEW: one home for all four palettes, values pinned
                      to the old literals — a visual no-op refactor with proof)
```

The template to copy for ANY new screen: `hud_scene` (work_graph → Scene) + `flatten_into`
(Scene → display list). A new screen is a new **producer function**, not a new draw function.

## Worktree-only components (master does NOT have them — cite worktree paths)

| Spec | Tests | SHA-16 (worktree HEAD) | Role for the Queen screen |
|---|---|---|---|
| specs/t27os/executor_registry.t27 | 5/5 | see `git rev-parse HEAD:specs/...` | evidence ladder + the five scoped counters + DSH/Queen-pane honest cards |
| specs/t27os/queen_layout.t27 | 6/6 | | deterministic force layout (reference's own constants) |
| specs/t27os/queen_messages.t27 | 7/7 | | live message flow: pane output → clean line, ring, pulse |
| specs/t27os/ui_snapshot.t27 | 15/15 | | Observatory contract incl. R7 author-approval ban, dash component ids |
| specs/t27os/shell_dock.t27 | 23/23 | | dock layout law (right dock, bottom terminal dock) |
| specs/t27os/work_map.t27 | 20/20 | | MYCELIUM/MAP/compact from one frozen dataset |
| specs/t27f/spec_scene.t27 | 3/3 | | **SpecGraph → Scene producer (the missing link, now present)** |
| specs/t27f/viz_select.t27 | 17/17 | | rep_for(intent, shape) selector with hard refusals |
| specs/t27f/viz_theme.t27 | 13/13 | | one theme (reproduces the four old palettes exactly) |
| specs/t27f/viz_hit.t27 | 17/17 | | shared hit-test over VizHost slots |
| specs/t27f/c1_candidate.t27 | 7/7 | | the C1 candidate/switch/rollback protocol on the fixture |

## Build and run (verified)

```bash
cd /Users/playra/t27/.claude/worktrees/silly-thompson-a50ffa
O=target/t27os-compare   # NEVER write target/t27os/T27OS.app — a window may be watching it
mkdir -p $O/T27OS.app/Contents/MacOS
target/release/t27b build specs/t27os/glass_window.t27 -o $O/glass_window.o --macho
# refusal check: nm -u $O/glass_window.o | grep -c .   (must be >= 1; was 96)
SDK=$(xcrun --show-sdk-path); SDKV=$(xcrun --show-sdk-version)
ld -arch arm64 -platform_version macos 11.0 $SDKV -syslibroot $SDK -e _main -lSystem \
  -lobjc -framework Foundation -framework AppKit -framework CoreGraphics -framework CoreText \
  -framework CoreFoundation -framework QuartzCore -framework Security -framework WebKit \
  -framework ImageIO -o $O/T27OS.app/Contents/MacOS/T27OS $O/glass_window.o
codesign --force --sign - $O/T27OS.app
# isolated run (fake HOME + symlinked socket + TAP_REPLAY seed):
HOME=/Users/playra/.t27os-compare-home ./target/t27os-compare/T27OS.app/Contents/MacOS/T27OS &
```

Last verified link+sign: 2026-11-03 ~01:0x, owner's own window SHA unchanged after builds
(`f6eaa6c33e7d5f56…`, checked before and after).

## Unfinished dependencies P0 must know

1. **glass_window.t27 is MODIFIED and UNCOMMITTED** in the worktree (the A+ Observatory wire:
   draw_qdash → tg slots → step pack → tb TBO_QM block → ui_fill; ui_fill takes `tb`; draw_qdash
   takes 14 args after the rec-param revert). It compiles green and is IN the compare bundle.
   Decision needed: commit as the A+ candidate or rework to generated-from-Scene observability
   (the owner prefers the latter as the direction; the hand-fill stays until then).
2. **The compare instance's server-probe gate**: with a fake HOME the window sees "SERVER RUNNING
   0 PANES" — the PTY status probe does not traverse the symlinked socket, so the message poll
   (tfd gate) stays closed and AGENT MESSAGES shows the honest empty line. One probe point to fix
   before the isolated instance can show live messages.
3. **Observatory capture is pending**: `~/.t27os-compare-home/.t27os/ui/want` was created but no
   snap.txt/snap.png pair appeared — the frame loop idle-waits and the want poll did not fire in
   the captured window of time. Moving the mouse or raising the window may wake it; untested.
   Until a pair exists, A+ stays "capture pending", not "done".
4. **spec_scene.t27 takes only the first 7 nodes** (scene MAX_NODES=32 minus groups/texts budget).
   A larger Queen screen needs the budget raised or a fold pass first.

## What P0 should reuse for the Queen screen (no hand-drawn dashboard)

1. **The model is already in the window**: lanes rows (`sst[SST_LV]`), executor_registry's five
   scoped counters, plan.txt picks (`QD_FEED_CMD` → tri-queen), pane listing (client_list).
   Fold them into a WorkGraph the way pty_live_graph.t27 does for tap events.
2. **Produce a Scene with hud_scene's pattern** (or a new producer beside it): status bar, one
   card per executor with its evidence level from executor_registry, one line per blocker with
   the missing input named — the owner's "Покажи связи / Почему зависла / Что изменилось" intent
   map picks the view; viz_select chooses the composition.
3. **Observatory comes free**: the Scene → display list path exists; extend ui_fill to read the
   Scene slots (or generate the UiSnap from the same producer output) — R1 holds because both
   come from one frame.
4. **The C1 protocol** (c1_candidate.t27) is the acceptance loop for any proposed change to the
   screen: baseline → one named change → checks → reviewer fixture → gate → rollback condition
   → restore → identity check.

## Explicitly NOT started (owner's hold, 2026-11-03)

- provenance u8-label through display_list + UiSnap (kept as a saved candidate)
- new experience_record.t27 contract (must first be reconciled with lane experience-gate @ c9edffb39f)
- B0 fix (t27c zig-codegen `_ = f(...)` → `const _ = ...`): reproducer and layer are pinned
  (t27c gen 0.5.2, spec f17af1fb10374045@main, error spec.zig:453 `const _ =`), fix not attempted
- the two flagged experience entries' quarantine check
- meta-loop (improvement-of-improvement) measurement

## Trust boundaries that hold for P0

- ui_snapshot R7: the author of a candidate never approves it.
- corpus_receipt / seals: accept/rollback evidence is hash-verified, not self-reported.
- The lanes table's "1 running" is a session state, not proof of a verified result.
- Fresh PTY output is observation, never a verified result (executor_registry ladder).

phi^2 + phi^-2 = 3 | TRINITY
