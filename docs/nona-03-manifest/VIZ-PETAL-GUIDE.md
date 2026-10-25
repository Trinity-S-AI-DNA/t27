# VIZ-PETAL-GUIDE -- adding the visualization for one more petal

**Bucket:** `docs/nona-03-manifest/` (cross-cutting tooling).
**Spec (SSOT):** [`specs/t27f/viz_contract.t27`](../../specs/t27f/viz_contract.t27) (the contract) and
[`specs/t27f/petal_viz_plan.t27`](../../specs/t27f/petal_viz_plan.t27) (the plan).
**Issue:** #12173 (epic), success criterion SC-007.

This guide is a map, not a copy. The contract and the plan decide; the guide names them and says
what they decide. It repeats none of their values, so it cannot go stale when they change. When you
need a code, a bit, a list or a count, open the spec.

## What a petal visualization is

One spec per petal under `specs/t27f/`, drawn through `viz_contract.t27`: one state (`VizState`),
one reducer (`viz_apply`), several views of the same entities that all read that state and so stay
linked, and a truth badge that says whether the drawing may be presented as a result
(`viz_may_claim`). The method follows Distill's *A Gentle Introduction to Graph Neural Networks*
(distill.pub/2021/gnn-intro): explore, change an input, watch the intermediate steps. That is the
inspiration for the method, not the design; the design is the contract's. The finished example is
[`specs/t27f/petal_specs_viz.t27`](../../specs/t27f/petal_specs_viz.t27), the reference petal.

## 1. Where do I start

- **Find your row** in `petal_viz_plan.t27`. Its arrays are indexed by petal: `PVP_QUESTION` (what
  the picture answers), `PVP_SOURCE` (the existing spec it is drawn from), `PVP_REPS` (the view
  kinds), `PVP_TRUTH` (the best truth that source backs), `PVP_STATUS` and `PVP_REASON`. The
  readers are `pvp_question`, `pvp_source`, `pvp_reps`, `pvp_truth`, `pvp_status`, `pvp_reason`.
  The plan's header says which index is which petal; `specs/t27f/petal_menu.t27` owns the letters.
- **A deferred row needs a source first.** `PVP_STATUS` is `PVP_DEFERRED`, `PVP_SOURCE` is empty
  and `PVP_REASON` says why. Do not draw such a petal from an invented sample and call it a result;
  the plan's header forbids it. File the source-spec work as its own issue, then come back.
- **Name the file** `specs/t27f/petal_<name>_viz.t27`, module `t27f_petal_<name>_viz`, as the
  reference is `petal_specs_viz.t27` with module `t27f_petal_specs_viz`.
- **Pick a unique name prefix `P<letter>V_`** (and `p<letter>v_` for functions). The reference uses
  `PSV_` / `psv_` and the plan uses `PVP_` / `pvp_`. Prove yours is free before you write a line:
  `grep -rIn 'PGV_' specs | head -3` (with your letter) must print nothing.

## 2. What do I write

Open with the header comment the reference has: the question, the data source and the truth it
backs, the layers, what is integer-only, and what is **not claimed**. Use `; --` as a separator,
never a bare `;` on a line of its own (it can swallow the `module` line). Then the parts, in the
order the reference has them:

1. **State.** A struct holding one `VizState` plus whatever small extra the question needs. Build
   it with `viz_init`, then `VIZ_EV_LOAD` and `VIZ_EV_LOADED` through `viz_apply` (`psv_open`). Every
   change goes through `viz_apply`. If the question needs an input the contract lacks, add one
   petal event numbered above `VIZ_EV_COUNT` and a petal reducer that forwards every contract event
   to `viz_apply` (`PSV_EV_TOGGLE_EDGE` and `psv_apply`). No view keeps its own hover, selection or
   step.
2. **Entities.** Each is a `viz_ref(kind, index)` of a `VIZ_ENT_*` kind. Give the focus order as a
   function from focus index to ref (`psv_focus_ref`) and make activation the same as a click
   (`psv_activate`). The entities come from the source spec's own `pub fn`s through `use`
   (`tutorial_08_graph` from `spec_graph.t27`, the box geometry from `spec_graph_view.t27`). Never
   copy a table, struct or function body out of the source.
3. **Petal logic**, only if the question needs it (the reference computes a spread over its
   graph). Integer only, like the reducer, so it can lower to the FPGA framebuffer later.
4. **One view per rep kind in your plan row.** Each view has a layout function and an inverse hit
   test (`psv_graph_hit`, `psv_matrix_hit`, `psv_list_hit`). The contract classifies each rep kind
   as a control, a view or the badge (`viz_rep_is_control`, `viz_rep_is_view`): controls write the
   state, the badge reports it, views draw entities.
5. **Highlights, all from the one state.** Every view asks `viz_highlight(state, ref)`, then adds
   the petal's own relation rule on top (`psv_node_hl_in`). A node hovered in one view lights in
   all of them in the same frame.
6. **The inspector:** everything about the entity under the pointer, else the selected one
   (`psv_inspect_value`, `psv_inspect_hops` and the line readers beside them).
7. **Host embedding.** A `VizHost` built with `vh_blank` and `vh_add` in depth-first order and
   checked by `vh_validate` (`psv_host`), plus one function that returns the rep bit set the petal
   composes (`psv_reps`, `psv_host_reps`). To nest a widget inside another, `vh_embed` copies its
   host, never its logic.

**Why nothing is copied.** The duplicate-bodies ratchet (`tools/dupe_scan.py`, CI job
`duplicate-bodies` in `.github/workflows/dupe-ratchet.yml`) fails a new group of identical function
bodies or a group that grew, and the repo forbids a second spec where one exists (CLAUDE.md,
"Proactivity"). Before writing a helper, ask what exists:
`python3 tools/dupe_scan.py --name <fn>` or `--like <spec>`. All of it is `.t27`: no JS or Python
probe, not even for an expected value (AGENTS.md, "Only t27").

## 3. What must I test

A test that cannot fail is not a test: the `corpus-ratchet` workflow refuses a new spec test with
no assertion (`tools/check_assertionless_spec_tests.py`). Write each oracle apart from the code it
checks. The categories, with the reference test to read for each:

- **Views stay in sync.** A hit in any view lands in the one state and every view's highlight reads
  it (`a_click_in_any_view_goes_through_the_one_reducer`,
  `selection_wins_over_hover_and_a_hovered_end_beats_a_related_one`).
- **State transitions.** Every event the petal handles, including its own; a change bumps the
  revision once and an event on a view that is not ready changes nothing
  (`contract_events_pass_through_the_one_reducer`,
  `toggle_ignores_an_unknown_edge_and_a_view_that_is_not_ready`).
- **Agreement with the source spec.** Compute the expected value from the source spec's functions
  inside the test and compare it with what your view shows. Where the petal computes something new,
  add a second computation that shares no code with the first
  (`the_value_reaches_a_node_exactly_when_it_is_within_that_many_hops`: `psv_hops` against
  `psv_trace`). Values you read by hand from a fixture are written in the test, not computed
  (`PSV_HAND_DEGREE`).
- **The truth badge on both sides of the claim rule.** Assert `viz_may_claim` is true for the truth
  your row promises (read it with `pvp_truth`) when that truth can claim, and false for the
  degraded state (fixture only, cached, failed), and that the petal moves between the two
  (`toggling_an_edge_flips_the_truth_to_demo_and_back`,
  `a_reload_keeps_a_truth_that_cannot_claim_anyway`).
- **Loading, empty and error.** Drive `VIZ_EV_LOAD`, `VIZ_EV_LOADED` with no items
  (`VIZ_LC_EMPTY`), `VIZ_EV_FAILED` (`VIZ_LC_ERROR`) and `VIZ_EV_RETRY`; check that interaction is
  ignored until the view is ready, and that each lifecycle has something to draw. The reference
  has no test of its own for EMPTY or ERROR (as merged in #12193): do not copy that gap.
- **Keyboard focus and reduced motion.** `VIZ_EV_FOCUS_NEXT` walks your entities in your focus
  order and wraps; `VIZ_EV_ACTIVATE` on the focused item equals a click
  (`focus_walks_nodes_then_edges_and_activate_selects_the_focused_item`); `VIZ_EV_BACK` clears the
  selection before it closes (`viz_back_closes`); after `VIZ_EV_MOTION`, `viz_motion_ms` is 0 for
  every animation your petal declares. The reference has no reduced-motion test either.
- **Reps equal the plan row.** Compare the petal's rep set and its host's with `pvp_reps(<index>)`
  imported from the plan (`use t27f::petal_viz_plan::pvp_reps`), never with a pasted number.
  Assert `viz_plan_ok`, and `viz_is_linked` when the row has two or more views
  (`the_host_and_the_plan_use_the_same_reps_and_pass_the_plan_check`). Your petal imports the plan;
  the plan must not import your petal, which would force a plan edit in your PR.
- **Host embedding.** `vh_validate` accepts the host; `vh_hit` returns each slot at a point inside
  it (`the_host_validates_and_places_each_view_in_its_slot` shows both); `vh_focus_next` follows
  slot order; the host still validates after `vh_embed` into another. The reference tests neither
  of the last two (as merged in #12193): cover them.
- **Invariants.** An `invariant` lowers to Zig comptime with a backward-branch quota of 1000, and
  one blown quota blocks the whole spec's report while `typecheck` stays quiet. No loops in
  invariants; put the iteration in a test.

## 4. How do I check it

- Build the compiler once: `cargo build --release -p t27c`. The binary is `./target/release/t27c`.
  While you change the compiler itself, use the faster builds in
  [CONTRIBUTING.md, Build speed](../../CONTRIBUTING.md#build-speed-the-t27c-inner-loop). A spec-only
  change does not need a rebuild.
- **Light:** `./target/release/t27c typecheck specs/t27f/petal_<name>_viz.t27`. Types only; it runs
  no test.
- **Heavy, the score CI uses:**
  `./target/release/t27c test-report specs/t27f/petal_<name>_viz.t27 --specs-dir specs`.
  Spec Guards reads its `FAIL` count and any `BLOCKED` line. Run it in a T27 OS pane or a Terminal
  tab, one spec at a time, never as a background build on a laptop.
- **Optional second witness:** `t27b test specs/t27f/petal_<name>_viz.t27`. If it disagrees with
  `t27c`, one of them is wrong; say so in the PR instead of choosing the answer you like.
- `python3 tools/dupe_scan.py` is the duplicate-bodies gate, run locally.

## 5. How does it land

- **File a child issue** of epic #12173 with four headings, because
  [`specs/policy/issue_shape.t27`](../../specs/policy/issue_shape.t27) closes an issue without them:
  `## Boundary` listing the file path (`- specs/t27f/petal_<name>_viz.t27`), `## User Scenarios`
  (Given/When/Then), `## Requirements` (`FR-001: ... MUST ...`) and `## Success Criteria` with an
  outcome a machine can settle (a test count, `exit 0`, a backticked command). Give the issue its
  own file: two issues that name the same file run one after the other.
- **Every commit carries `Closes #<child>` or `Refs #12173`.** The L1 check
  (`.github/workflows/l1-traceability.yml`) reads each commit, and "see #N" or "Part of #N" fails.
  Never put `Closes` on the epic.
- **Seals.** A new petal spec lands without a seal, and so did the contract, the plan and the
  reference: none of the three has a file under `.trinity/seals/`.
  Once a spec is sealed (with a master-built `t27c`, CLAUDE.md section 2, "Seals"), re-seal it after
  every edit: a seal that no longer matches its spec fails `seal-coverage`
  (`tools/check_seal_coverage.py`).
- **Before each push, merge `origin/master` and re-run `test-report`.** Spec Guards scores the pull
  request merged with master, not your branch alone. Real example: master gave petal G a bespoke
  glass-window page, and the plan's host-page test failed only on the merged tree. Push once; with
  auto-merge on, a second push can land the first commit alone.
- **Do not edit the plan row in the same PR as a petal.** Parallel petal PRs would conflict on the
  same arrays. Any change to a plan row goes in its own plan PR, after yours.
- `tri land [--issue N]` is the landing command (CLAUDE.md section 2, "Landing").

## 6. Drawing it on screen

The host is the T27 OS glass window, [`specs/t27os/glass_window.t27`](../../specs/t27os/glass_window.t27).
Wiring a petal into it is a **separate step after the petal spec merges**: its own child issue,
its own PR. The file uses `@extern`, so `t27c` cannot compile it. Test it with
`t27b test specs/t27os/glass_window.t27` and build the app with `make t27os`; `tri land` refuses
such a branch, so it lands as a plain PR (CLAUDE.md section 2). The host folds `VizState.revision`
into its frame key and redraws exactly when something changed (contract layer L4). A row rises to
LIVE only when a host feeds it; read the plan's header on truth before you claim it.
