# NOW -- key_failover.t27: a bee whose key is refused moves to another key, then another pool (2026-10-17)

## specs/queen/key_failover.t27 (Closes #9993, part of #9976)

- Measured in the Railway logs, 2026-10-17 13:00-15:18Z: 17 of 93 bee turns on trios-bee-runner-2 ended after 0 steps.
  - At least 22 of them were z.ai 429 code 1305 (temporary overload), 16 in one minute at 14:08.
  - The cause: a bee is pinned to one key at dispatch, the SDK retries that key three times, and 33 other keys in two pools stand by.
- `error_kind()` classifies a refusal as transient, quota, auth or a bad request. The z.ai code wins over the HTTP status.
- `cooldown_seconds()` rests a transiently refused key for 15 s, doubled per strike, at most 240 s. A quota or auth refusal closes the key for 30 min.
- `next_free()` picks the next free key and never the one that just failed. `pool_overloaded()` moves to the next pool once 3 distinct keys said 1305.
- `step()` bounds a turn at 8 attempts. `charges_issue()` charges only a bad request, so an overloaded provider never retires the issue.
- Railway lab, `t27c test-report`: 9 tests, 112 runtime asserts, 0 FAIL, 0 vacuous, 4 invariants.
  - `tri mutate spec`: 92 of 93 mutants killed and 0 survived. The one hang is a mutant's own endless loop (`k = k`).
- Wiring into trios goes through `t27b ts` (#9994), not hand-written TypeScript.
