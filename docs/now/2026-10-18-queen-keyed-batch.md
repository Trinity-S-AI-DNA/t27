# NOW -- keyed_batch.t27: the bee admission admits a batch of ready issues per queend choose (2026-10-18)

## specs/queen/keyed_batch.t27 (Closes #10033; epic #7851)

- Why: the actors trial of 2026-10-17 (#7851, comment 6237225782) started 2 bees in 20 minutes, against the loop's 18.5. The round readies about 826 candidates. The one admission actor then asked queend once per issue (`await deps.admit`, `queen-bee-actors.ts:298` at trios 42042a5), while the loop asks once per round.
- What: the admission now groups asks into one queend choose. keyed_guard.t27's answers stay the same; only the number of queend calls changes.
  - `joins_batch` and `answer_now`: exactly the asks `admit_answer` would send to queend join a batch, and the rest are answered at once.
  - `batch_goes_out`, `batch_take` and `chooses_for` (group commit): a batch goes out when no choose is in flight, with no timer, up to `BATCH_ASKS_MAX` = 1024, which is at least 826.
  - `new_lane_quota`: control.t27 `batch_quota`, the loop's cap.
  - `word_of`, `batch_answer` and `batch_grants`: queend's answer gives each issue PICK, LATER or NO. A lone ask answers as `admit_answer` does, and a batch grants exactly its quota.
  - `batch_wake_count`: a freed lane wakes every waiting issue into one batch.
  - `round_chooses` (the model): the measured round of 826 candidates on 47 lanes, with one pick in ten, takes 461 serial chooses and 1 batched.
  - `ask_wait_ms`, `ask_bound_ok` and `word_after_choose` (bounds): an ask's call must outlast two chooses. `askQueend` has no bound today. A choose that ends with no answer moves no lane.
- Lab evidence (Railway lab, master 9a7d54aaf tree, t27c 0.5.2):
  - `t27c test-report`: 13 of 13 tests pass, 0 vacuous, and 6 invariants are proved at comptime.
  - `tri mutate spec`: 85 of 89 mutants are killed. Of the other four:
    - 2 survivors are equivalent (`<` to `<=` where both branches return `BATCH_ASKS_MAX`);
    - 1 is a hang from the mutant's own endless loop;
    - 1 is unviable, because an invariant rejects it at comptime.
  - Hand constant plants: 10 of 10 are caught.
  - Wasm build (gen-c plus zig cc wasm32-wasi): reproducible, sha256 `1ed6bcb1...cfd8a`, 1187 bytes.
  - Seal: verified, all hashes match.
- Runtime: the thin TypeScript wiring is a gHashTag/trios issue. It covers a batch-shaped `deps.admit`, `planBatch` over one choose, `batch_wake_count`, and a bound on `askQueend`. The before/after numbers go on #7851.
