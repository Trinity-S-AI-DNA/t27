# Current Autonomous Task: Seal and land the 402 failover fix
**Objective**: Finish the PHI LOOP for the 402 screen-row classification in the t27os key failover laws.
**Background**: 2026-10-27 a turn died on ollama.com with HTTP 402 (usage credits gone, auto reload payment failed). The router and ladder laws already read 402 as a move-on refusal, but the wired screen watcher (keys.t27 classify_line / error_row) did not know the row shape, so no key rest and no respawn fired -- the pane sat dead and this ledger was left with an empty queue, which stalled the loop.
**Done (FAILOVER-402-CLASSIFY, local phases 1-4 + 7)**:
- specs/t27os/keys.t27: 402 classed FC_NO_BALANCE; error_start accepts "402:" rows; 6 new asserts; 113/113 green (t27b, 2531 runtime asserts)
- specs/t27os/pty_logic.t27: end-to-end test of the two-row shape ("This turn failed" + "402: {...}") -> FC_NO_BALANCE; 58/58 green
**Targets (FAILOVER-402-SEAL-LAND)**:
- Gen: regenerate the t27os sources from the specs with a master-built t27c (no fresh local t27c; prefer the lab executor over the owner's Mac)
- Seal: re-seal t27os_T27osKeys and t27os_T27osPtyLogic from that build
- Land: file the issue, commit on a branch, PR through the gates (no push without the owner's OK)
**Requirement**: Follow PHI LOOP for each target.
