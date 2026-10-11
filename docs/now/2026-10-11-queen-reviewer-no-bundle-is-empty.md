# NOW -- a runner bee that committed nothing is an empty attempt after a 10-minute grace, not a 6-hour wait (2026-10-11)

## specs/queen/reviewer.t27 (Closes #9069; part of #9021, item 3)

- `no_bundle_is_empty(on_runner, bundle_stored, minutes_since_finished)`: a finished runner row with no stored bundle is an empty attempt once `NO_BUNDLE_GRACE_MINUTES` (10) have passed. Inside the grace it waits; a container row and a row whose bundle is stored are never decided here.
- Why: on 2026-10-11, 07:10-09:43Z, 425 of 480 reviewer visits answered `wait` because the diff of queen-N could not be read. Over 07:10-10:31Z, 15 runner rows got their bundle late, the latest 58 s after the first visit.
- Tests: 11 of 11 pass, 0 vacuous; 4 new tests with 14 runtime asserts, and 1 new invariant. 8 negative controls each FAIL or BLOCK test-report.
- The runtime that calls it, behind `TRIOS_QUEEN_NO_BUNDLE_EMPTY` (off by default), is a gHashTag/trios PR on `queen`.
