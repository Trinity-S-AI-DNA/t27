# Mutation gates of sar_trace_raw and bench -- receipts, 2026-11-02 (issue #13093)

Binary: the master-worktree build (t27c 0.5.2, sha256 4dcd5cf240b4a4167a15f28850dd026ab7e64e977df593b89cb1f93069cfe467; commit unknown). Tree: lane/analog-receipt-schema, head 12d51cd10a + the two accepted files of this commit. Guards held: load 19.34 at the start (under 40), about 19 GiB free (over 4 GB). Command shape, both runs:

    tri mutate spec --file specs/analog/<spec>.t27 --t27c <t27c> --jobs 3 --timeout 30 --accepted docs/analog/mutation-accepted-<spec>.txt --fail-on-survived

## sar_trace_raw (exit 0)

First-pass clock 10 s of --timeout 30 s: 0 rerun(s), 60 job-s saved. Compile answers: 0 retried, 0 unviable. Survivor gate: 4 not killed (1 survived, 3 hung), 4 accepted by docs/analog/mutation-accepted-sar_trace_raw.txt; 0 accepted line(s) now killed. Gate passed.

## bench (exit 0)

First-pass clock 10 s of --timeout 30 s: 0 rerun(s). Compile answers: 0 retried, 0 unviable. Survivor gate: 3 not killed (3 survived, 0 hung), 3 accepted by docs/analog/mutation-accepted-bench.txt; 0 accepted line(s) now killed. Gate passed.

## What this closes

FR-001 and FR-002 of #13093: the two accepted files exist, with every not-killed census mutant listed (1 equivalent for sar_trace_raw; 3 equivalent for bench; the 3 hung of sar_trace_raw are hung by construction -- dropped counter steps) and none of the listed lines now killed. FR-003's clean-run control was green earlier the same day (7/7 and 14/14, 0 vacuous). SC-002 of #12116 is met for these two specs; the v0.1.2 notes can drop their "censused and not gated" caveat in their own docs change.

No speed, power or completion-percentage claim anywhere in the receipts.
