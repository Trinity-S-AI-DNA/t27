# NOW -- self_health.t27: the Queen's doctor prescribes a treatment for every file-conflict jam (2026-10-11)

## specs/queen/self_health.t27 (Closes #9094, part of #9022)

- The owner asked for a "doctor bee". At 13:04Z 214 candidates stood on fileConflict, and every cause found so far had been found by a person reading replays.
- `prescribe()` takes one held path and returns one of: RELEASE, DERIVED_FREE, NARROW, NARROW_WAITS_CHECK, SPLIT, WAIT.
  - It reuses `holds_files_now` (queen/holds.t27) and `is_derived_path` (ci/derived_data.t27) by `use`.
  - NARROW only names the treatment. The region lines and the merge-side check stay in issue_shape.t27 and the batch chooser.
- `treat_before()` orders the jams biggest first. `conflict_starved()` is #9022's tick anomaly: lanes idle and fileConflict above 20% of candidates.
- Railway lab, `t27c test-report`: 11 tests, 0 FAIL, 0 vacuous, 2 invariants. All 9 mutants fail by name.
- Wiring is the Queen runtime's lane (#9021): run it every tick over the dispatch rows and apply what it prescribes.
