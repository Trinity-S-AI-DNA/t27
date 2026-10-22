# NOW -- a bee that left its declared region onto live work does not land (2026-10-11)

## specs/queen/golden_chain.t27 section 5 (Closes #9097; part of #9021)

- The merge side of `path :: fn NAME` (#9062). Admission compares declared regions; before an accept, every hunk of the bee's diff must lie inside a declared region of its file (`issue_shape.t27` `edit_inside`, asked by the host).
- `gc_merge_kind`: one hunk outside makes the file's edit EDIT_WHOLE again. `gc_pair` then asks the widened edit against every live edit, as admission does.
- `gc_region_step` folds the files: only an accept is gated; a widened edit that meets live work sends it back; one that meets nothing lands; an unread diff or base waits; a finding outranks a failed read. `gc_region_counts`: a send-back is charged to the issue, a wait is not.
- `t27c test-report` (t27c built from master ea1bccd77): 19 tests, 0 FAIL, 0 vacuous, 4 invariants. 8 mutants of the new rules each FAIL a test by name. The runtime wiring is in gHashTag/trios behind `TRIOS_QUEEN_REGIONS`, off by default.
