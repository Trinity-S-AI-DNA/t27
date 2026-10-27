# HANDOFF -- the ledger one session leaves for the next

`tri trigit handoff` reads this file and prints the records it holds;
`tri trigit handoff <agent> <task> <state> <issue> <note...>` appends one
record when the contract and the law of the states allow the move.

One record is one line, five fields, split by pipes:

    agent | task | state | issue | note

A field never holds a pipe. The state is one of `taken`, `doing`, `blocked`,
`done`: `done` is terminal, nothing jumps from `taken` to `done`, and a repeat
is not a move. What a record MEANS, why a move is refused, and the words of
the refusal are specs/tri/handoff/handoff.t27 (the contract) and
specs/tri/trigit/verbs.t27 section 8 (the verb). The row the T27OS makes of
the last record -- which session cell that task means -- is
specs/t27os/handoff_view.t27.

The file grows by appends carried in commits (issue #12286, FR-003 and
FR-006): history is never rewritten, and a session reads this ledger before
it reads anything else.

## Records

Buffy|viz-distill-graph|done|#12249|merged on master: 1119 nodes, 9577 edges, 0 errors
Buffy|trigit-handoff|doing|#12286|the verb, the contract and the T27OS row
