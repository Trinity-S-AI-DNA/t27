# NOW -- golden_chain.t27: the pipeline's own version control, land on verdicts, not on a queue (2026-10-11)

## specs/queen/golden_chain.t27 (Closes #9027)

- The owner asked for "our own git to manage the pipeline, and a golden chain pipeline". This card is the land side. A link is one change plus its stage verdicts SPEC -> GEN -> SEAL -> TEST -> RECEIPT. A stage is golden when it passed, or when its key was judged golden before.
  - The golden head moves only forward, by a compare-and-swap on the parent id.
  - A link built on an older head is carried past what landed since. Derived paths never conflict. Source edits conflict only when actors.t27 `edits_meet` says their regions meet. An edit under the other side's import re-judges from GEN.
  - `gc_land` lands a whole queue in one pass.
- Reused, not restated: actors.t27 `edits_meet` and the EDIT_* kinds. Also named, but not imported: reuse.t27/frontier.t27 (verdict keys), corpus_merkle.t27 (ids), and l2_generation.t27 `is_gen` plus derived_data.t27 (what is derived). A redundant "mixed derived" branch was deleted, because `edits_meet` already fails closed on an unknown kind; a mutant proved it dead.
- `t27c test-report` on the Railway lab: 12 tests, 0 FAIL, 0 vacuous, 3 invariants. All 12 mutants FAIL by name:
  - each carry rule dropped;
  - the parent check removed;
  - the since-window ignored;
  - worst-of turned to best-of;
  - the head taken from the first link;
  - chain order unchecked;
  - an unknown parent read as the head.
- Real data, the last 300 master commits, file level:
  - 72% of touched paths are derived (gen/, seals, ledger and figure JSON, baselines);
  - of 2945 commit pairs at most 10 apart, 537 share a path, and only 149 (5.1%) share a source path;
  - so 72% of overlaps are derived-only, and the golden chain lands them without a conflict or a re-run.
- Not yet: line ranges and imports were not in that measurement, and there is no benchmark against the GitHub loop (see #7851). The merger wiring comes behind a flag.
