# NOW -- key_failover.t27 wired into the live Queen dispatcher (2026-10-18)

## gHashTag/trios PR 2045 (Closes #10403)

- Where the live dispatcher picks a key (gHashTag/trios `trios/agent-server`, branch `queen`, `apps/server/src/api/services/`):
  - `dispatchBee` (queen-dispatch.ts) calls `resolveWorkerProvider(takenKeyIndices, afterKeyIndex, runtime)`, then `endpointPoolProvider`, then `availableKeyIndex`.
  - A key is full when the count of `takenKeyIndices` equal to its durable index reaches the lane count; pool key = `(pool.number - 1) * 10_000 + position`.
  - `drain` ends every bee, in the Queen process and in the runner alike: it reads the stream error, classifies quota, then "ended unexpectedly", then `closeDispatch`.
  - Runner lanes are `RUNNER_KEY_BASE (100_000_000) + id`: a separate key space, not operator keys.
- Wired (behind `TRIOS_QUEEN_KEY_FAILOVER=on`; off is the previous behaviour):
  - `drain`: 429, 5xx or Z.ai 1302/1305 closes the row as `provider overloaded (http N | zai code N)`, chosen by `error_kind`.
  - `dispatchBee`: a key inside `cooldown_seconds` counts as full, so the next bee takes the next key.
  - `beeEndOfRow`: the label ends as `EF_PROVIDER`, never charged to the issue.
  - The Z.ai quota-code `Set` in TS is deleted; the card's `is_quota_code` replaces it.
- Vendored byte-identical: `key_failover.t27` sha256 `822367357fc6b487c1c7fb7ac8369165e6a7e6636fd9b0dcafec29e9a6274301`; `key_failover.wasm` (831 bytes) sha256 `aa4fdd776ccf74ecff59cee1c5284db16ead03e477afd9988409920afa0b6b4f`, the same hash on two builds.
- Hand-written TS: 70 added, 70 deleted, no file over 40, no new foreign file.
- Railway lab (`t27c test-report`, master eb425e6ac):
  - Vendored card: 9 tests pass, 0 vacuous.
  - Negative control 1, `next_free` allowed to return the key that failed (the old behaviour): `the_next_key_is_never_the_one_that_failed` FAILS.
  - Negative control 2, 429 and 1302/1305 not transient: `the_code_wins_over_the_status` and `a_status_without_a_code_falls_in_its_kind` FAIL.
- Not verified: the changed TypeScript has not been executed. The lab has no bun and the trios tests are `bun test`; CI on the trios PR is the first run.
- Parked, with reasons in #10403: the pool-level next-pool move, and `keyed_batch` admission (live only on the actors path, which is off in production, and not a thin edit).
