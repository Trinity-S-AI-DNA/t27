# NOW -- a dispatch holds the files of its issue as it stands now (2026-10-11)

## specs/queen/holds.t27 (Closes #9033; part of #9021)

- At 10:57Z the round chose nothing while 63 of 70 lanes were idle. 122 candidates were shut out only by `bootstrap/src/compiler.rs`, held by the bee of #8660, an issue closed at 10:56:59Z. A closed issue's work is never reviewed or published, so its hold protected nothing.
- New rule, `holds_files_now`: when the round has read the whole open-issue list, a closed issue holds nothing, and an issue whose Boundary is gone holds nothing. If the list is unread, the stage rule decides as before.
- New rule, `held_part`: a stored path holds only where the current Boundary still names it. A narrowed directory holds the narrower path (#8721: `.trinity/seals/` down to its seal files). A path the Boundary added holds nothing, because the bee was briefed with the stored set.
- `t27c test-report`: 11 tests, 0 FAIL, 0 vacuous, 40 runtime asserts. 5 negative controls, each FAILs by name.
