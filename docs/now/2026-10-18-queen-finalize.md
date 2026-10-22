# NOW -- finalize.t27: how a bee run or a claw check run ends, what it keeps and what it owes (2026-10-18)

## specs/queen/finalize.t27 (Closes #10424)

- Idea from REA (morluto/rea, rea-agents 6.6 and 6.7, MIT); no code taken.
- Measured 2026-10-06, the bee-runner disk jam: both 49 GB runner volumes had about 6.8 GB free, bee merges were 0 for about eight hours.
  - Leaked finished worktrees (about 294 MB each; 80 and 98 removed by hand) were listed nowhere as owed.
- Rule 1: `observations_kept()` and `entry_after_finalize()` keep every completed observation for every outcome; `report_kind()` says partial, never empty.
- Rule 2: `takes_primary()` keeps the first failure or cancel; a cleanup error is never primary. `cancel_reason_after()` keeps the exact reason that dispatch_exit's DX_SHUTDOWN folds away.
- Rule 3: `entry_after_check()` keeps an entry pending, with its owner, until a read says the resource is gone; `reap_action()` releases a worktree only on worktree_keep's `reap_verdict` proof. This list is what ai.t27.runner-reap should read.
- Rule 4: `shutdown_step()` waits for in-flight probes up to PROBE_GRACE_MS (3000 ms) on top of drain's `exit_step()`; a cut probe answers unknown.
- Rule 5: `final_status()` keeps unknown unknown; `check_conclusion()` posts neutral for cancelled or unknown, never failure.
- Railway lab, `t27c test-report`: 26 tests, 0 FAIL, 0 vacuous, 6 invariants; `spec-status` IMPLEMENTED; seal verify all MATCH.
- Not here: wiring into gHashTag/trios and the reaper itself.
