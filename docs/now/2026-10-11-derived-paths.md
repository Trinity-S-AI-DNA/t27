# NOW -- which Boundary paths are derived records (2026-10-11)

## specs/ci/derived_data.t27 (Closes #9057; part of #9021)

- `derived_path_kind` maps a Boundary path to the derived record it names. `.trinity/seals/` and everything under it is P_SEAL. `docs/reports/t27b_expectations.json` is P_LEDGER. `bootstrap/stage0/FROZEN_HASH`, the digest of compiler.rs, is P_FROZEN_HASH. Every other path is a source.
- It is the host's marking for `gc_pair`'s GC_EDIT_DERIVED (queen/golden_chain.t27, #9029). The dispatcher lets two writes of one derived record run at once, behind TRIOS_QUEEN_DERIVED_FREE. On 2026-10-11 at 10:57Z, 13 issues were shut out only by a held `.trinity/seals/`.
- Left out on purpose: `docs/now/`, which is written once per pull request; `AGENTS.md`, a hand-written file; `gen/`, which belongs to l2_generation.
- On today's GitHub squash path nothing regenerates the ledger, so neighbouring ledger rows still meet in git's merge.
- `t27c test-report`: 27 tests, 0 FAIL, 0 vacuous. 5 negative controls, each FAILs by name.
