# NOW -- dispatch_exit.t27: a provider refusal never charges the issue (2026-10-17)

## specs/queen/dispatch_exit.t27 (Closes #10008, part of #9976)

- The bee dispatcher as keyed actors (gHashTag/trios `queen-bee-actors.ts`, `TRIOS_QUEEN_DISPATCH=actors`) asks this card whether an end counts against the issue.
  - Until now `counts_against_issue(DX_PROVIDER_ERROR)` was true, so three quota refusals in a row (`RUN_LIMIT`) gave the issue up.
  - A quota or an overload belongs to the key that served the turn. The next dispatch takes another key, and `key_failover.t27` `charges_issue` (#9993) charges only a bad request. The two cards now agree.
- Unchanged: a bad request (`HC_4XX`) still gives up at once, and a refusal that keeps repeating still ends at `STREAK_LIMIT` (6) with the backoff between tries.
- New test `a_provider_refusal_never_charges_the_issue`: 8 asserts, including that a timeout and an unknown end are still counted runs.
- Railway lab, master de2cff882:
  - `t27c test-report`: 11 tests, 0 FAIL, 0 vacuous, 4 invariants.
  - `tri mutate spec`: 81 of 84 mutants killed.
    - The 3 survivors are equivalent mutants in `restart_wait_seconds`, which this change does not touch: lines 192, 198 and 202 flip `>=` and `>` at a clamp, and at the boundary both branches return the same value (`LEASE_TTL_SECONDS - age` is 0; `wait` equals `BACKOFF_CAP`).
  - `dispatch_exit.wasm` (t27c gen-c + zig 0.16.0 cc wasm32-wasi -Oz -DNDEBUG reactor, every fn exported):
    - The old card rebuilds to `31c0e524...`, the hash pinned in gHashTag/trios `specs/PIN`, so the recipe is the one in production.
    - The new card builds to `b990a601e4c1806095688683e3c25f0b5176124d368dfd258fdcdd7934c56327`, the same hash on two builds.
- The TypeScript hosts read this card from the wasm, so trios takes the change by re-vendoring the `.t27`, the `.wasm` and the `specs/PIN` lines. There is no hand-written code.
