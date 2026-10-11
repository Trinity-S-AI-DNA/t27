# NOW -- a Boundary line may name one item of its file (2026-10-11)

## specs/policy/issue_shape.t27 (Closes #9059; part of #9021)

- A Boundary line can now name a region: `- bootstrap/src/compiler.rs :: fn emit_instance`. The region is the lines of the single `fn NAME` item at the base commit the round reads. Comments and string literals are skipped, raw strings included.
- A name that is defined twice or not at all, or a body that does not close, resolves to nothing, and the line holds the whole file. An older reader takes only the path, so it also holds the whole file.
- `edit_inside` decides, per hunk of the bee's diff, whether the edit stayed inside its declared region. One hunk outside makes that file's edit whole again.
- On master cc19d1fcd the card resolves 1507 of 1598 `fn` names in compiler.rs, at about 9 ms per name.
- Estimate from issue text: 82 of the 125 issues blocked by one compiler.rs holder name a function there, and a greedy apart-batch admits 32.
- `t27c test-report`: 15 tests, 0 FAIL, 0 vacuous. 7 negative controls, each FAILs by name. `gen/c/policy/issue_shape.c` was regenerated.
