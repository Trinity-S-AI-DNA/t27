# NOW -- a claim on the issue follows the work as it stands now (2026-10-11)

## specs/queen/holds.t27 (Closes #9111; part of #9021)

- 12:55Z, 89 open issues claimed. 16 had a bee at work; none was a dead worker. 9 were send-backs "expecting the same bee back", but no bee comes back once its turn has ended. 8 of those 9 were under the retry ceiling, waiting out an idle hour for nothing.
- 12:47Z, 374 issues counted `completed`. 2 had only pull requests closed unmerged (#4374, #6414): accepted work that will never land, and an issue never chosen again. 213 had no PR at all, because the export of their accepted heads has failed on every run since 10-08. They are not freed; the repair is the export.
- New rules:
  - `send_back_claim` frees a send-back under the ceiling.
  - `accept_claim` frees an accept whose branch has only closed-unmerged PRs. Open, merged, none, or unread keeps it done.
  - `held_stage` makes a freed claim hold no files.
- `t27c test-report`: 16 tests, 0 FAIL, 0 vacuous, 61 runtime asserts. 6 negative controls, each FAILs by name.
