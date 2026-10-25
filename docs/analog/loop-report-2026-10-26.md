# Analog emulation improvement loop: end-of-loop report, 2026-10-26

Epic: #10486. Issue of this report: #12555. English, ASCII only.

Window: from the v0.1 release (tag `analog-v0.1.0`, 2026-10-24) to 2026-10-26 05:35 +07, which is rows 29 to 136 of the epic ledger. "Row N" below means row N of that ledger (the table of ticks kept as a comment on the epic). Every figure is copied as written in its source; a figure that was inferred or forecast says so, and a saving that was never timed is labelled "estimate". The ledger is the only record of the loop's day-to-day work, so most sources below are ledger rows.

## 1. Summary

- The loop spent the window on one question: do the tests of the analog specs notice a wrong line? The mutation gate (`tri mutate spec`) was run on 17 of 17 analog specs (census note, PR #12501, issue #12241).
- For the big specs the gate now exits 0 on a final accepted-mutants file: sar_comparator (868 mutants), sar_kick_net (727), sar_logic (677), accept_sar (281), mos_psp103 (359), among others (section 3).
- The seal command was measured against 16 planted faults: bare seal refuses 0 of 16, `--save` and `--verify` refuse 16 of 16 (row 37, #12065).
- Strict coverage of the analog specs went from gaps 5 (145 of 150) to gaps 0 (150 of 150) (rows 33 and 34).
- Open: #12116 stays open because SC-002 is not met to the letter for two specs; the owner has several decisions due on or after 2026-10-27, each with a default (section 4).
- Every merge since row 91 followed the required-checks rule, with a basis comment posted first.
- v0.1 keeps its recorded slowdown (about 170000 to 196000 times slower than real time, #11313); this window adds no emulation speed claim.
- Sections 5 and 6 give three cooperation variants and three speed-up variants, each with a default.

## 2. What was done, per area

### 2.1 Mutation testing of the analog specs

- Method: a census first (measure how many mutants the tests kill), then a classification of every survivor, then a follow-up issue shaped to pass the issue-shape gate (four headings, numbered FR requirements with "must", countable success criteria).
- Classes used: equivalent (no test can tell), test gap (a test can be written), dead line (no input reaches it), hung (an endless loop), not constructed (a witness input is still needed), undecided (needs a lab trace).
- Fix: tests are appended after the last test of the spec; the model lines are not edited. The seal is re-saved with a master-built compiler, and an accepted-mutants file per spec records what stays alive and why.
- Gate rule: exit 2 when a not-killed mutant is not accepted, or when an accepted line is now killed. An entry is stale when every viable mutant of its key was killed; unviable mutants are ignored in that lookup (row 115, found by reading the gate source).
- Census of 17 of 17 specs is on master in `docs/analog/mutation-census-rest-2026-10-25.md` (PR #12501 and the refresh PR #12552 for #12543). #12116 is checked against it: SC-001 met (17 specs, five counts each); SC-002 not met to the letter (row 135).
- Per-spec results with the PR that landed each are in section 3.

### 2.2 Converter and comparator blocks

- sar_comparator (868 mutants): census 98 survivors; groups A to M closed by arithmetic tests, survivors 98 to 49 (#12432, PR #12513, row 122), then the last seven mutants (groups N and O and line 522) by five lab-figure tests (#12512, PR #12540). The gate shows none of the 56 survivors of groups A to O left (row 128).
- sar_kick_net (727 mutants): six tests by hand arithmetic (#12460, PR #12496); nine lab-figure mutants closed by six tests, 36 of 36 pass (#12494, PR #12531). The 79 left are accepted.
- sar_logic (677), accept_sar (281), mos_psp103 (359), sar_comparator_q, sar_switch_q, sar_adc, sar_kick, sar_dff_q: tests appended, gates passed (section 3).
- Board, SC-003 of the v0.1 epic: the second bitstream set for ss, ff, fs and sf read PASS on every well-formed line (#12050, row 36, PR #12063). With the tt receipt of 2026-10-20, all five corners now have a board receipt.
- Strict coverage: five specs lacked a negative control; #12041 (PR #12046) added planted-fault tests, giving gaps 0 (row 34).
- Not changed: the acceptance result of v0.1 (9 of 13 blocks PASS, 4 NOT MODELLED) and every model line.

### 2.3 Reference chain

- #12027 (pin the reference chain): requirement 3 was measured read-only through the hosting service's tree interface. The 17 PSP103 model files and the corner library at pin 21133f21 equal the development branch (cb743f75) byte for byte; the main branch (5e6d592e) differs on `Changelog` and `PSP103_module.include`. Recorded in `docs/analog/reference-chain.md` (row 30, PR #12032).
- The receipts part of #12027 and #12028 (the three gate-level blocks of the golden deck) were "not started; blocked on a pane" (row 29); the deck lines are not on the loop's machine (row 32). No model was written from memory.
- Competitor delta: section 12 of `docs/analog/competitors.md` (PR #12029, row 30). This report says nothing about competitors beyond what that note records.

### 2.4 The mutation driver and its tools

- First-pass clock (#12345, PR #12346, row 88): `specs/tri/mutate/clock.t27`, 24 tests, strict coverage 25 of 25. It is a rule as a spec; it is wired nowhere.
- Slice 2 (#12403) would wire it into the driver. It is +40 / -8 hand lines, net +32, and the foreign-line rule refuses a diff that adds more than it deletes; no label lifts that (rows 95 and 98). Parked.
- `tri coverage` prints `STALE-T27C?` when only the Rust hash differs, so a stale compiler is no longer read as a broken spec (#12071, PR #12072, rows 39 to 41).
- Delta runs (`--fn`) and the load and disk kill rule are in force (section 6).
- #12539: the driver turns any other compiler answer into a silent unviable (two extra unviable mutants in the second sar_comparator gate came from missing cache files, row 128). Parked on the same foreign-line question.
- Five master-wide reds that blocked the loop's own PRs were read in their logs and fixed by small PRs: #12358/#12404, #12417/#12421, #12488/#12489, #12510/#12511, #12514/#12515 (rows 100 to 123).

### 2.5 Skills and ledger discipline

- Skill lessons landed as dated bullets in the analog-emulation skill: PR #12391 (ticks 61-65), #12475 (nine bullets), #12517, #12542 and #12554.
- Every task took a claim comment first; every merge took a basis comment first (head sha, four required checks, each other red read in its log); every tick ends with "Requests out" (who, what, default, due).
- Correction rows instead of rewrites (rows 36, 39, 115). Wording drift was hunted: two accepted-file headers and one note section that still said the gate was not run (#12518, PR #12519) and four more headers that forecast a result their notes record as measured (#12521, PR #12530).
- Measured, forecast and inferred are kept apart. Three inferred exits (rows 112, 119, 128) were each replaced by a measured one (rows 114, 130, 133).

## 3. What was measured

| Figure | Result | Source |
|---|---|---|
| v0.1 acceptance, 13 blocks (unchanged) | 9 PASS, 0 FAIL, 4 NOT MODELLED; 26 of 30 rows | v0.1-release-notes.md |
| Seal against one planted fault in each of 16 specs | bare refuses 0 of 16; `--save` 16 of 16; `--verify` 16 of 16 | row 37, #12065, seal-refusal-2026-10-25.md |
| Strict coverage of the analog specs | gaps 5, 145 of 150, then gaps 0, 150 of 150 | rows 33, 34, #12041 |
| Stale compiler against master-built compiler | seal 16 of 30, gaps 14, 136 of 150, against 30 of 30, gaps 0, 150 of 150 | row 39, #12070 |
| Reference files at pin 21133f21 | 17 model files plus corner library equal dev cb743f75 | row 30, PR #12032 |
| Board, five corners (SC-003) | ss 1216 of 1216, ff 1218 of 1218, fs 1219 of 1219, sf 1217 of 1217 | row 36, #12050 |
| sar_logic, 677 mutants | census 526 killed, 108 survived, 31 hung, 12 unviable; gate 608, 26, 31, 12; 57 accepted; exit 0 | rows 60, 124, #12163 |
| sar_comparator, 868 mutants | census 659, 98, 78, 33; final gate 714, 42, 79, 33; 121 accepted; exit 0 | rows 101, 133, #12512, PR #12540 |
| sar_kick_net, 727 mutants | census 591, 82, 39, 15; final gate 633, 39, 40, 15; 79 accepted; exit 0 | rows 107, 130, #12494, PR #12531 |
| accept_sar, 281 mutants | census 218, 58, 4, 1; gate 241, 35, 4, 1; 39 accepted; exit 0 | rows 99, 114, #12431 |
| mos_psp103, 359 mutants | census 264, 72, 18, 5; gate 306, 30, 18, 5; 48 accepted; exit 0 | rows 102, 111, #12434 |
| mos_table, 632 mutants | gate 583, 25, 20, 4; 45 accepted; exit 0 (fifth run) | row 95, #12317 |
| raw_writer, 526 mutants | census 412, 83, 31, 0 (exit 0); gate 474, 21, 31, 0; 52 accepted; gate passed | row 96, mutation-raw_writer-2026-10-25.md |
| linear, 420 mutants | census 316, 61, 38, 5; gate 351, 26, 38, 5; 64 accepted; exit 0 | rows 71, 82 |
| Delta gate, one function of linear_zoh | 3 mutants in 11 s, exit 0 | row 81 |
| Delta gate, `--fn sl_init` of sar_logic | 17 mutants: 12 killed, 1 survived, 3 hung, 1 unviable; 14:05 to 14:08 | row 83 |
| Gate time, sar_kick_net | 39 min 49 s first gate; 38 min 05 s confirmation gate | rows 126, 130 |
| Gate time, sar_comparator | 49 min 19 s first gate; 48 min 30 s confirmation gate | rows 128, 133 |
| Lab-figure tests | #12494: 36 of 36 pass; #12512: 38 of 38 pass | rows 126, 128 |
| First-pass clock spec | 24 tests, coverage 25 of 25, 20 of 24 own mutants killed (4 equivalent) | row 88, PR #12346 |
| Slice 2 of #12403 | +40 / -8 = net +32 hand lines, refused | row 98 |
| Free disk during the census | 7.4 GiB at 07:59 fell to 0.4 GiB at 08:25 | row 60 |
| v0.1 slowdown, recorded only | 17.0 to 19.6 us per 100 ps step; 170000 to 196000 times slower than real time | v0.1-release-notes.md, #11313 |

Reading the gate figures: killed, survived, hung, unviable. "Census" is the first run before tests were added; "gate" is the run on the final accepted file. All gate runs used `tri mutate spec` with `--jobs 2` or `--jobs 3` and `--timeout 30` or `--timeout 45`, as each row states.

## 4. Weak spots that remain

| Item | Reference | Blocked on | Default if nobody answers |
|---|---|---|---|
| SC-002 of #12116 not met to the letter: sar_trace_raw (1 survivor, 3 hung) and bench (3 survivors) have follow-up none | #12116, row 135 | the owner | close after 2026-10-27 with the deviation written in the closing comment |
| Slice 2 of the first-pass clock, net +32 hand lines | #12403, row 98 | the owner (two options in the issue) | parked, revisit 2026-10-27 |
| Silent unviable in the compiler-answer handling | #12539, row 128 | the owner (same question) | parked |
| An owner choice recorded in the issue | #12069, row 41 | the owner | option c on 2026-10-27 |
| Reference chain receipts and the three gate-level blocks | #12027, #12028, rows 29, 32 | a machine with the reference toolchain and the golden deck, which the loop does not hold | not started |
| The zig cache folder was emptied during a gate (61.5 GiB, link inferred from time and message) | #12539, row 128 | the cache cleaner on the owner's machine, which the loop does not hold | the loop keeps free disk above 10 GiB during gates and treats a cache-error unviable as noise |
| 27 accept_sar survivors "not constructed" | #12431, row 110 | nothing; a witness input for each is needed | left accepted |
| linear.t27: line 463 undecided, dead lines 441 and 442; each needs a gate of about 48 minutes | row 136 | nothing; judged low priority in row 136 | left accepted |
| raw_writer dead lines 683 and 954 | mutation-raw_writer-2026-10-25.md | the owner decides | kept and accepted |
| Inherited reds on master: five stale seals (#12509, PR #12495); `same` duplicate group (#12498, PR #12499); `put_lit` group (#12419, PR #12429); 25 parse-no-discard rows (#12497, PR #12500) | rows 131, 134, 136 | other sessions | documented as inherited; the required checks decide |
| Board: comparator step (phase 2b, #11380) not built; phase 2a unit routed at 59.46 MHz and FAILS at 100 MHz; four blocks NOT MODELLED | v0.1-release-notes.md | a board and cable on a machine the loop does not hold | v0.1 receipts stand |
| Bare seal runs no tests; two single-line faults change no output of the model | #12065, #12067, seal-refusal note section 7 | nothing; recorded | `--save` and `--verify` are the refusing forms |

## 5. Three cooperation variants

These are real alternatives. A changes how work lands, B adds an agent, C adds an outside block. The loop does not need an answer to continue.

### A. The owner keeps merging by the required-checks rule

- Who does what: the loop merges a PR when the four required checks (`validate`, `check-linked-issue`, `parse-ratchet`, `own-language`) are green. It reads every other red in its log, shows it inherited, and posts a basis comment before the merge. The owner may veto.
- Owner time: none unless a veto; a veto is one comment (estimate: under 5 minutes).
- Default if nobody answers: the rule stands from 2026-10-26 12:00 +07 (rows 91, 120 to 136).

### B. A second agent takes the FPGA board lane

- Who does what: a second agent, on a machine with the board, the cable and the reference toolchain, takes the board items: the comparator step on the board (#11380), the receipts of #12027 and the gate-level blocks of #12028. The loop keeps the mutation and tool work and hands over the notes of section 2.2 and 2.3.
- Owner time: name the agent and the machine (estimate: 15 minutes).
- Default if nobody answers: parked. The v0.1 board receipts stand as recorded and the loop does not touch the board lane.

### C. The external team supplies a block and a criterion

- Who does what: the external team names the next block and the check that enforces it, as it did for the 13 blocks of v0.1. The loop turns it into a spec with tests and a gate, and runs the acceptance evaluator on it. The state of the exchange is kept in the epic ledger (row 136 and later), not in this report.
- Owner time: read the team's answer and say yes or no to sending a reply (estimate: 10 minutes).
- Default if nobody answers: no reply is sent, and the loop continues with its own blocks and models nothing from memory.

## 6. Three Pareto speed-up variants

These concern the mutation gate and the loop, not the emulation. Each removes one step; evidence is a measured ledger figure unless it says "estimate".

### P1. Delta gate while iterating, one full gate at landing

- Step removed: the full-spec gate after each test-only edit.
- Evidence: `tri mutate spec --file linear_zoh.t27 --fn fmax64 --jobs 1` ran 3 mutants in 11 s with exit 0 (row 81), where the full 252-mutant gate was recorded as about 10 minutes (row 81, approximate). A sar_logic function run took 17 mutants in about 3 minutes, 14:05 to 14:08 (row 83). Full gates of the large specs took 38 min 05 s to 49 min 19 s (section 3).
- Risk: accepted lines outside the function are ignored (row 81), so a delta gate cannot see an effect in another function. The landing gate stays.

### P2. Skip the confirmation gate when only gate-named entries were deleted

- Step removed: the second full gate after the entries that the first gate reported as "now killed" are deleted.
- Evidence: the skipped runs were 38 min 05 s for sar_kick_net (row 130) and 48 min 30 s for sar_comparator (row 133). Both returned exit 0, equal to what the first gate plus the deleted entries implied (rows 126 to 133).
- Risk: an inferred exit 0 can be wrong. The first accept_sar gate exited 2 because an entry for an unviable mutant counted as killed, which only a re-run showed (rows 112 to 115). The sar_comparator confirmation also moved two unviable mutants (33 against 35) because of cache errors (row 133, #12539). Until a confirmation gate has run, the PR says `Refs`, not `Closes` (rows 119, 128).

### P3. First-pass clock with one rerun of the hung set

- Step removed: waiting out the full timeout on every hung mutant.
- Evidence: for one gate, 27 hung x 45 s = 1215 of about 2445 job-seconds, about half (row 87). This is arithmetic, not a timing: estimate. The rule is `specs/tri/mutate/clock.t27` (PR #12346, row 88); slice 2 is not wired.
- Risk: a clock set too short turns slow-but-finishing mutants into hung, which changes the counts and breaks the accepted files, because those are written against one timeout (row 87). It also needs the owner's call on the +32 hand lines of #12403.

Already in force, not a variant: start a gate only below load 15 (below 30 with `--timeout 45`), and stop it above load 60 or below 2.5 GiB free disk. The mos_table gate did not finish on its first four runs (rows 89 to 94) and passed on the fifth, at load 9.3 (row 95).

## 7. Sources

- The epic ledger, rows 29 to 136, kept as a comment on #10486: every row cited above.
- Epic #10486; issues #12116, #12241, #12027, #12028, #12041, #12050, #12065, #12067, #12069, #12070, #12071, #12163, #12317, #12345, #12403, #12431, #12432, #12434, #12460, #12494, #12512, #12539, #12543, #12555.
- Pull requests #12029, #12032, #12046, #12063, #12072, #12085, #12102, #12108, #12153, #12346, #12401, #12459, #12468, #12482, #12496, #12501, #12513, #12531, #12540, #12552.
- Notes in `docs/analog/`: `v0.1-release-notes.md`, `seal-refusal-2026-10-25.md`, `reference-chain.md`, `competitors.md` (pointer only), `mutation-census-rest-2026-10-25.md`, `mutation-raw_writer-2026-10-25.md` and the other `mutation-*` notes and accepted-mutants files named by spec.
- Not used: any web source, any note outside the list above.
