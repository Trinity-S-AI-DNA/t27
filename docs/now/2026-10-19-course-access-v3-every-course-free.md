# NOW -- course-access VERSION 3: every course free for everyone (2026-10-19)

## specs/automation/course-access.t27 (Closes #10798)

- Why: owner, 2026-10-19 -- "make the courses free for everyone! our task is to grow the language". A course behind the club is a reader the language does not get; the club stays the bot's own subscription for generations.
- `step_of(opened)` returns `STEP_FREE` for every rung (v2: free for three, then paid). `LADDER_RUNGS` counts the rungs that differ before `step_of` repeats its last: 4 in v2, 1 now.
- `STEP_BOT`, `STEP_PAID`, `may_open`, `stays_open`, `mint_ok`, `key_days` unchanged: a key already minted keeps opening what it opened. `lesson_event`, `tells_owner`, `may_report` do not depend on the paid rung and are unchanged; `SITE_CHECKOUT` and `CONTENT_HIDDEN` stay false.
- Tests: the old paid-ladder asserts now assert free; a new loop pins `step_of(n) == STEP_FREE` for n = 0..255 (767 runtime asserts); a new test keeps a held paid key opening a paid step. VERSION 3 with a dated header note and its test.
- t27c (bootstrap code unchanged since 239b4b059), zig 0.16.0: test-report 18 of 18 pass, 0 vacuous. Negative control: the v2 `step_of` body turns 3 tests red.
- `scripts/t27-logic.mjs` (999) lowers 9 fns, 0 not; in node `stepOf(1..10)` is all 0. Seal saved.
- Not here: re-vendoring into 999 render and trinity (`courseLadder.ts`). A course stored on a paid rung under v2 still closes by `stays_open` when the club ends; a host that must keep it open re-rungs it with `step_of`.
