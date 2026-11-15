# Lab runbook -- t27b release differential (8,344 tests + coverage)

Run on the lab host (the Mac does not run corpus sweeps). Every step prints
its own evidence; a step that prints nothing did not run.

## 0. Get the tree

Clone Trinity-S-AI-DNA/t27 and check out the release branch
(`release/t27b-optimizer-parity-2026-11-15`, pushed after owner approval; until
then, `git fetch <origin> && git checkout <commit>` on the lane branch). Build:

    cargo build --release -p t27b

Release binary SHA-256 must be
6091c8521c9511c0ffef087868fb75d6f3ac884d2f5f3708bc76b6ef3c8e7cfb
(the Mac build; the lab may differ -- record BOTH hashes and each one's counts;
the comparison is within one binary).

## 1. Differential corpus (t27b corpus --reference t27c)

Per docs/reports/t27b-bench-2026-10.md's harness commands:

    ./target/release/t27b corpus --reference <t27c-binary> specs/ > corpus.json

Accept:
- coverage >= 1,296/1,311 (98.9%) -- no coverage regression;
- reference_disagree == 0 on the 8,344 tests both ran -- no test disagreements.

## 2. JIT/interpreter cross-check (--check) on the same tree

    for f in specs/queen/*.t27 specs/tri/t27b/*.t27 specs/tri/shootout/*.t27; do
      ./target/release/t27b test --check "$f" || true
    done 2>&1 | tee check_sweep.log

Known pre-existing divergences (do not count as new): key_credit.t27,
steward.t27, the actor_journal.t27 uninitialized-array shape. Also pre-existing (verified on the clean tree): numeric/bigint.t27 and math/pellis_precision_verify.t27. Anything beyond
that list is a NEW finding: stop, save, report.

## 3. Per-pass firing evidence on bench_audit

    T27B_TRACE_PASSES=1 ./target/release/t27b asm specs/tri/shootout/bench_audit.t27 \
      > release.asm 2> trace.log

Accept: 8 distinct pass names with FIRED; hot loop 10 words
(cmp csinc add sub movz cmp csel csel sub cbnz); the peeled copy keeps
adds+b.hs.

## 4. Bit-exactness and determinism

    ./target/release/t27b build specs/tri/shootout/bench_audit.t27 --elf -o a.o
    ./target/release/t27b build specs/tri/shootout/bench_audit.t27 --elf -o b.o
    cmp a.o b.o && echo BITEXACT

## 5. Record

Append one T27L record per section to .trinity/agent_events.jsonl with the
lab's commit, binary hash and counts; commit corpus.json, check_sweep.log,
release.asm, trace.log beside the release notes
(docs/reports/t27b-optimizer-release-2026-11-15.md).

## Gate

The release tag is cut only when section 1 and 2 accept AND the owner
approves. No auto-push, no auto-tag.
