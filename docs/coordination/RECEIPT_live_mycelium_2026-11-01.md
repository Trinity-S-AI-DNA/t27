# LIVE MYCELIUM enrichment — receipt (2026-11-01, Bangkok)

## Changed (uncommitted, review-ready)
- specs/t27os/queen_live_graph.t27 (+251): state classes
  working/connected/idle/blocked/stale/done (qlg_state_class, QLG_STALE_S=300,
  QLG_QUIET_S=600 quoted from specs/t27f/live_fleet.t27:86); honest stat line
  qlg_stat_line2; node label qlg_node_line; event ring QLEV_* (32 x 4 words,
  qlg_ev_add/at/line). 12 tests + 4 invariants.
- specs/t27os/glass_window.t27 (+324/-25): pane rows now carry issue
  (SST_PISSUE), model (lv_agent_model) and heartbeat (SST_PBEAT) from the
  session store at lanes_build — the source fix; band draws labels, honest
  counts, newest-first event strip, stale never counted live, click opens a
  detail card. Session store 560 -> 706 words, one registry. 102 tests.
- NOT ours / left untouched: bootstrap/src/use_resolve.rs had a pre-existing
  uncommitted var-splice change before this session.

## Measured
- t27b test queen_live_graph.t27 -> 12 passed, 0 failed, 4 invariants held
  (rerun by lead independently).
- t27b test glass_window.t27 -> 102 passed, 0 failed (rerun by lead).
- t27b test lanes_view.t27 -> 34 passed, 0 failed (regression).
- make t27os -> target/t27os/T27OS.app built and signed (build 7745).
- App launched; full-screen capture: /tmp/t27os_live_slice.png (2026-11-01
  17:53 +07). The agent could NOT view it (no image input in this model) —
  visual acceptance is the owner's check.

## Live-run attempt (owner directive 2026-11-01, points 4-5) — BLOCKED
- 18:23+07: app relaunched from build 7745, before/after full-screen captures
  /tmp/t27_before_*.png, /tmp/t27_after_182342.png.
- An automated keystroke send via the EXISTING osascript tool was attempted;
  no new implementation code was written. The attempt was stopped by the
  Accessibility blocker (osascript 1002, exit 1). Live events were NOT
  created, and nothing was substituted with fixture data. This is not
  equivalent to an owner's manual send.
- Consequence: the acceptance scenario (issue on node -> working -> verify ->
  blocker -> stale -> click details, ring wrap under real events) remains
  UNOBSERVED. pane_assign fires at the prefill send, which requires either
  Accessibility for keystrokes or a manual owner send into a pane.
- Distinction kept: everything observed so far is tests + build, NOT live
  execution. No fixture was presented as live.

## Commit
- Current local commit: 3519dc04b5 (amends 4079496de4 -> f888bc7dd3, both
  superseded same-change revisions). Published head of the branch is
  f3aad561ab ("wasm/doom: bless the duplicate-body ledger...", unrelated
  work that was already on origin/claude/doom-wasm-chain). Our LIVE MYCELIUM
  work is local-only, ahead 1, NOT pushed; no PR exists.
  Pre-commit gates: foreign-budget PASS, own-language PASS, secret-gate PASS
  (after gitleaks found at /opt/homebrew; first attempt failed only on
  missing PATH). Not pushed; PR not opened (no merge, per directive).
- Unrelated pre-existing changes preserved outside the commit:
  bootstrap/src/use_resolve.rs (var-splice suppression, rewritten mid-session
  by an out-of-team process — semantic change kept), untracked .trinity
  slices and litter untouched.

## Not yet measured (honest gaps)
1. The browser slice: t27.ai/t27f/live was unreachable from this session and,
   per specs/t27f/live_fleet.t27:7-18, the deployed page reads a recorded-log
   fixture, never live data — no comparable live slice exists on that side
   today.
2. The acceptance scenario (task appears on node -> working -> verify -> 
   blocker -> stale shown instead of live) was not exercised: no swarm task
   was run in a pane during this session. The hooks exist (spawn/start/
   verify/finish on pane lifecycle; stale via heartbeat past 300 s), but the
   end-to-end observation needs one real pane task under the running app.
3. SST_PISSUE has no writer yet (slots ready, default "-" ) — an issue number
   reaches a pane row only after the dispatcher writes it.

## Acceptance map (owner's 5-point scenario)
1. task on node -> SST_PISSUE writer (pending, item 3)
2. working state/action -> DONE (qlg_state_class + label + heartbeat)
3. verification in details and feed -> DONE (QLEV_VERIFY hook + detail card)
4. concrete blocker -> PARTIAL (BLOCKED state + card; blocker reason text
   comes with the issue writer)
5. stale shown, not live -> DONE (heartbeat > QLG_STALE_S -> STALE, excluded
   from live count)
