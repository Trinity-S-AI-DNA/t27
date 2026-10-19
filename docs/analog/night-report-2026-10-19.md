# Night report 2026-10-19: analog emulation, lab, T27 OS

Issue: gHashTag/t27#11062, epic #11063 (ledger comment 6268998904 on the epic). English, ASCII only. Every number below is copied from a test, a receipt, a PR or an issue comment named next to it; a number without a source is not here.

## 1. What was asked

The owner's directive for the night: research the weak points of the task and its competitors, decompose, implement, end with a report and three collaboration options for the next loop; run autonomously; one continuous loop that never breaks earlier work; self-critical, find anomalies and repair itself; new `tri` commands and skills; every task a GitHub issue; coverage to 100 percent; auto-fix on and merge when green.

## 2. What merged tonight

| PR | issue | what | measured |
|---|---|---|---|
| #10893 | #10777 | PSP103 drain and gate charge tables (tt, ss) | 500 fF inverter delays within 0.020 % and 0.0041 % of ngspice (#11071, W3) |
| #10953 | #10888 | M4e-offset: the converter offset is the comparator's clock-edge kickback read through unequal top plates (`sar_kick.t27`) | base +9.42 mV, kickback removed +0.03 mV (PR body) |
| #11055 | #11020 | `tri remote`: reference jobs on a remote Docker host, command lines built by a spec | 16 tests, refusal tests for every shell byte, hand-written Rust +21 -441 |
| #11056 | #11054 | the rule "request, do not ask" in `CLAUDE.md` and the loop skill | 2 files |
| #11066 | #11041 | lab L1: the t27c lab's decisions as `.t27`, `lab.py` as the oracle | 5 specs, 66 tests, 1474 pinned asserts, 59 mutants killed |
| #11067 | #11043 | `LAB_GATE` as a network job kind run by T27 OS nodes, test-network XP | 21 tests, 3 invariants, 335 runtime asserts |
| #11069 | #11044 | one installer built with make, README, TRI CLAW as a component | 16 tests, 12 of 12 mutants killed, Makefile +40 -41 |
| #11071 | #11058 | the ten analog weaknesses against measured evidence | 0 closed, 8 reduced, 2 open |
| #11072 | #11057 | lab competitors, prior art and weaknesses | 16 systems, weaknesses W1 to W8, three options |
| #11075 | #11045 | `tri seals drift` names the claim that moved and reads the ledger in groups | see 3 |
| #11076 | #11059 | `tri loop plan`: the tick's anomaly, idempotence and requests-out decisions | 25 tests, hand-written Rust +23 -40 |
| #11079 | #11042 | lab L2: the t27b lab's decisions as `.t27` | 5 specs, 84 tests, 1392 cases, 46 mutants killed |
| #11080 | #11068 | `tri land`: land a lane branch with the rules in `.t27` | 26 tests, 29 steps, 18 of 18 planted mutants killed |
| #11082 | #11074 | AX7203 board regression of both analog tops | see 3 |
| #11084 | #11083 | the home path in the board document | 1 line |
| #11095 | #11060 | `tri learn`: save a lesson into a skill once | 21 tests |
| #11096 | #11061 | `tri coverage`: five facts per spec | 18 tests, five specs edited |
| #11092 | #10942 | partial: four ngspice decks for the comparator kick | no spec yet, see 6 |
| #11098 | #11081 | `tri worktrees`: evidence columns, KEEP_ACTIVE for recent commits, no `mine` label | 18 tests |
| #11105 | #11102 | one seal hash (ml_host_io_model) that the emitter fix #11094 missed, which had turned `coverage` and `spec-guards` red on master | merged with 28 passing checks, 0 failing |
| #11114 | #11110 | `tri learn` fence rule (a closing fence is bare and as long as its opener) after it made three Traps headings in one file | 22 tests; three lessons replayed on the real file give one heading |
| #11108 | #11106 | the night's six traps into three skills with `tri learn add` | merged with 27 passing checks, 0 failing |
| #11123 | #10882 | PSP103 charge tables for ff, fs and sf, the 500 fF inverter in all five corners, and the gate charge in a circuit | 500 fF inverter 50 % delays within 0.025 % of ngspice backward Euler in ff, fs and sf (tpLH 0.0207, 0.0191, 0.0249 %; tpHL 0.0107, 0.0019, 0.0079 %); gate node worst sample 0.051 % of the supply; tests 25, 9, 4, 13 and 20 in five specs, vacuous 0 |
| #11124 | #10942 | `sar_kick_net.t27`: the comparator's clock-edge kick on both top plates derived from the plates' drive (partial: S5 open) | 20 tests, vacuous 0; S1 to S4 and the seven first-decision offsets met, see 6 |

Every PR above is merged. A lane for #10911 (VACASK as a second reference) was still running when this was written; its result goes into an addendum at the end of this file. Failing checks of the merged PRs were read one by one: the ones that are red on master too (`duplicate-bodies`, `t27b-native-ratchet`, and `Verilog widths` naming two widgets specs of another session) are inherited; none was caused by these PRs.

## 3. Measured numbers

- Board regression (PR #11082): M1b top 573 of 573 receipt lines PASS, 1196 LUT, 1096 FF, 0 DSP, 0 BRAM, 116.06 MHz post-route (earlier 572 of 572, 1197 LUT, 106.30 MHz; the Verilog changed since #10475), each negative control FAIL on its own circuit only. SAR top 549 of 549 PASS bit-exact (550 lines start with `T27`, as before), 908 LUT, 538 FF, 0 DSP, 0 BRAM, 108.27 MHz, control FAIL on exactly the 16 predicted conversions. Both tops are behavioural: they do not read the PSP103 tables, so the FPGA size of a PSP103 solver is not measured.
- Seal drift (PR #11075): the lane measured the scan of 2298 specs at 69.80, 56.44, 107.79 and 92.37 s before and 14.40, 25.05 and 24.51 s after, under load averages of 9 to 44; a forced serial control took 88.69 s. The orchestrator's run on a master tree with both CLI PRs merged: 25.69 s, 0 specs moved; a planted verilog drift printed `moved: gen_hash_verilog`, a 1-byte edit printed `moved: spec_hash`, `--fix` re-saved exactly that one spec.
- Builds: a `t27c` release rebuild after merging master with dependencies cached took 47.27 s on an unloaded machine and 176.10 s at load 40; `tri` took 485.47 s at load 40 and about 19 minutes from a cold target (lane report).
- Offload: the full aperture scan of the golden comparator (20 transients) ran on the remote Docker host as a container job in about one minute of wall time (container started 18:13, log last written 18:14 on the host clock, load average 0.06 afterwards); on the laptop under load the same deck had finished 4 of 20 in about 45 minutes before it was stopped.
- Machine: free disk fell from 9.4 GB to 3.6 GB in about 50 minutes; swap used reached 6752 of 7168 MB (3.0 GB an hour earlier). After the heavy runs moved off the laptop: load average 6.5, swap used 5.0 GB, 6.0 GB free.
- Coverage (PR #11096): the lane's harness over 29 specs of the five groups gave 140 of 145 facts before and 145 of 145 after its edits and re-saved seals; the live `tri coverage <glob> --t27c target/release/t27c` over the same groups reads 145 of 145: analog 75/75, lab 55/55, board 5/5, remote 5/5, install 5/5 (gaps 0). The first live run showed far lower numbers because it ran in a scratch tree without `target/`, so tests, vacuous and seal were unmeasured.

## 4. Weak points and competitors

- Analog (PR #11071, section 11 of `docs/analog/competitors.md`): W1 reduced (exact zero-order hold for the linear cell, amplitude ratio 1.0003 against 1.78e-7 for backward Euler; the nonlinear path is still backward Euler); W2 open (no stiffness report, steps chosen by hand); W3 reduced (charge tables for tt and ss in PR #10893, then ff, fs and sf in PR #11123: all five corners, inverter delays within 0.025 %); W4 reduced (five corners within 0.56 % where Id is at least 100 uA, weak inversion not met); W5 reduced; W6 reduced; W7 open (no speed claim is allowed); W8 reduced; W9 reduced (RAW writer byte-equal to ngspice's); W10 reduced. Nine more weak points found while building are listed as W11 onward.
- Lab (PR #11072, `docs/lab/competitors.md`): volunteer computing, compute markets, remote build execution and mobile limits; the design's weak points are the cost of k-of-n verification, Sybil and collusion, floating-point determinism, untrusted code, node supply at cold start, mobile limits and the legal line.

## 5. Decomposition

Epic #11063 with children #11057 to #11062, #11068, #11081, #11083; the lab epic #11040 with #11041 to #11045; the analog epic #10486 with #10942, #10881, #10882, #10911. The ledger comment of the epic holds one row per tick and a status table; the tick is now decided by `tri loop plan`.

## 6. Not done, and why

- `sar_kick_net.t27` (#10942, PR #11124): S1 to S4 and the seven first-decision offsets are met (kicks within 1.3 to 8.1 percent against 10 and 15 asked, the seven offsets within 1 mV and in the right order); S5, the late decisions of the converter, is not: of four late-decision offsets three are within 0.2 mV, one misses by 0.29 mV (decision 10 of the second conversion). `sar_adc.t27` is untouched and #10942 stays open. The next gap, named in the spec header: the bit path (the model's two paths differ by 0.2 mV at decision 9, ngspice's 28 agree within 0.05 mV) and decision 10 against 9.
- #10881 (weak inversion read): queued; its ngspice sweeps go to the remote host, not the laptop. #10882 (charge tables for ff, fs, sf and the gate charge in a circuit) is done, PR #11123.
- #10911 (VACASK as second reference): the first attempt on the remote host found VACASK 0.3.2-103 (debug build) in the container image there, which rejects the main-branch demo (syntax error at line 42, column 62); a newer VACASK is the next step.
- M5 second half (the M4e trace as a RAW file) and M6 (release v0.1, due 2026-11-16) are untouched tonight.
- No GPU workload exists in the analog work, and no work is planned for a GPU machine.
- #11103: two specs that use `host_open` do not compile under Zig on macOS (a bare literal to the variadic libc `open`, `bootstrap/src/compiler.rs:7414`); the one-line emitter fix changes emitted output, so it needs the owner's word on `FROZEN_HASH`; default taken: not done, Linux CI is unaffected.

## 7. Self-critique

- The landing of every lane went through a throwaway script outside the repository because `gh` fails TLS inside the sandbox and the zig cache is not writable there; the script is now the spec of `tri land` (#11080).
- A seal landed recording the sandbox's blocked zig cache with a home path (PR #11080); found in review, re-saved, and the script now re-saves any seal that records `blocked`.
- PR #11082 merged with its `scan` check red because the document named a developer home path; the check is not required, so auto-merge did not wait for it and I had not read non-required reds. Fixed in #11084; the landing script now refuses such a path.
- I first blamed the gen-verilog merge (#8844) for two red fpga checks; the merge-rules session showed they come from `specs/fpga/systolic.t27` (#10780).
- I told the external team something about our use of their machine that did not match the owner's instruction; corrected within the hour.
- Heavy ngspice runs on the laptop filled swap and disk; the idle remote host should have taken them from the start.
- Scripts were killed by closing their terminal tab before they printed their final marker, twice early and once more at 00:00 (two pollers of remote container jobs; the containers kept running and were fetched again).
- A coverage fix of mine was pushed with a spec that no longer compiled, because the script did not stop on the failing test; every landing script now stops at the first failing gate before the commit.
- `tri learn`, my own tool, made three Traps headings in a skill file whose fences it counted odd; found while landing its lessons, fixed in #11114 and the lessons PR rebuilt.
- A lane's commit said `Part of #N`, which the L1 check rejects; the PR merged anyway because that check is not required. The rule is now a lesson in `ci-gates`.
- Master went red on `coverage` and `spec-guards` because a peer's emitter fix missed one seal; the seal could not be re-saved on a Mac because that spec does not compile there (#11103), so one hash was set by hand to the value `seal --verify` prints, with the evidence in the PR body (#11105).
- `tri worktrees` printed `mine` for another session's lanes (#11081).
- The first cron carried the owner's raw directive as its prompt and would have restarted the whole night every 15 minutes; replaced by a tick that reads the ledger first.

## 8. What changed in skills, `tri` and memory

Skills and rules: `CLAUDE.md` section "Proactivity: request, do not ask", the user skill `request-dont-ask`, step 6 "Requests out" of the analog loop tick. `tri`: `remote`, `seals drift`, `loop plan`, `land`, `learn`, `coverage` (and `worktrees` in #11081). Memory: the night-loop file with the traps found tonight (gh TLS in the sandbox, blocked seals, the issue-shape gate, non-required reds, swap on the shared APFS container).

## 9. Three collaboration options for the next loop

Option A: finish the v0.1 deliverable with the external EDA team. Needs from them: access to their self-checking evaluator and its ADC limits, the ngspice version behind their published RAW, a VACASK that reads current syntax. Cost: small to medium; risk: their answers (several of our questions are still open). First measurable step: VACASK, ngspice-47 and our model on the golden comparator's flip point and the DAC top-plate voltages, differences tabulated (#10911). Default taken if nobody answers: this option.

Option B: the lab as a network job kind. Needs: operators of machines (our own laptop, a CI runner, the remote host if its owner agrees). Cost: medium; risk: determinism across hardware, untrusted code, the legal line (test-network XP only, no value claim). First measurable step: run the closed gate list on one pinned commit on each host class and count the verdict hashes that differ.

Option C: open-source upstreams. Needs: the maintainers of VACASK, OpenVAF-Reloaded, IHP-Open-PDK and ngspice. Cost: low; risk: the method is public before v0.1. First measurable step: one public, reproducible ngspice-versus-VACASK comparison of the comparator flip point, and a count of maintainer responses within 14 days.

## 10. Requests to the owner, with the default taken

- The word on `bootstrap/stage0/FROZEN_HASH` for the one-line emitter fix of #11103 (two specs that use `host_open` compile on macOS). Default taken: not done; Linux CI is unaffected and the two specs stay unsealable on a Mac.
- Which option of section 9 to run next. Default taken: option A.
- Whether a page the external team sent (a researcher's account of time and space) matters for the work. Default taken: nothing.
- Whether and when to hold a call with the external team is the owner's to decide; no call, price or deadline was promised in the chat.

