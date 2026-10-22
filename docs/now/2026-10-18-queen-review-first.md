# NOW -- review_first.t27: no backlog dispatch while finished work waits in review (2026-10-18)

## specs/queen/review_first.t27 (Closes #10555)

- The owner's rule, 2026-10-18: the Queen takes nothing from the backlog while there is a queue in review.
- Measured 2026-10-18 17:09Z on api.t27.ai/queen/status: 69 of 70 workers busy; last hour 102 dispatched, 56 finished, 11 reviewed; reviewable 29, unreviewed 362. About nine bees start for every verdict.
- Rule 1: `in_review_queue()` is the review sweep's own filter (started, finished, review_state NULL or 'wait', not reaped, issue open) -- the same rows the status page counts as `reviewable`.
- Rule 2: `admission()` refuses a backlog candidate while that queue is longer than `REVIEW_QUEUE_MAX` (0); `round_refused()` says when the round owes the sentence `<n>` + `REFUSAL_TAIL`.
- Rule 3: `kind_of()` -- owner-assigned beats send-back beats backlog; rework and assigned are admitted at any queue length, so freed workers go to what the review asked for.
- Rule 4: the queue drains without a person, because every non-accept review outcome has a clock (review_valve.t27).
- Railway lab (t27c at 6ea5f5173, zig 0.16.0): 8 tests pass; seal verify all MATCH.
- Negative controls: 17 hand mutants (bound 0 -> 1, `<=` -> `<`, rework or assigned refused, kind order swapped, each sweep-filter clause dropped or flipped, refusal clauses flipped, code moved), 17 killed. Two are killed by the comptime invariants; the same two with the invariant neutralized are killed by named tests.
- Not here: the wiring into gHashTag/trios queen-tick.ts (candidate filter behind a flag), the Railway redeploy, the key vault.
