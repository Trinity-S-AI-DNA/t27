# Sync audit: t27os <-> t27 (first run)

Run on 2026-10-23 from Gaia-MacBook-Air. Evidence: `t27` master at `a059416c55`, `t27os` master at `ffbb761`. Commands are recorded so the audit is reproducible.

## Method

For each of the three sync-critical facts named by the SSOT rule (PR t27os#2), read the value in both trees and compare. A fact that lives in only one tree is reported as "pointer-only" (no duplicate to drift) rather than "match".

## Results

### 1. Petal count

- t27: `specs/t27f/petal_menu.t27` line `pub const PETAL_COUNT : u32 = 27;` (also `PETALS_PER_REALM = 9`).
- t27os: `README.md` ("27-petal 999 triangle"), `docs/ARCHITECTURE.md` L1 row ("27 quads"), `specs/os/home.t27` comment ("owns the 27 quads").
- Verdict: MATCH (27).

### 2. specs/t27os file count (OS subsystem spec count)

- t27: `git ls-tree origin/master --name-only specs/t27os/` = 58 files.
- t27os: `docs/ARCHITECTURE.md` says "57 specs on master (2026-10-23)".
- Verdict: MISMATCH (doc stale by one). The extra file is `specs/t27os/labs.t27` (landed via t27#11784 after the doc was written). Action: update the doc line to 58. Filed as a follow-up.

### 3. Tool-letter registry (27-agent alphabet)

- t27: `specs/agents/<letter>.t27` for 27 letters (ORDINAL 1..27, Ti = 27) and `specs/t27os/agent_parts.t27` (each letter gets widgets; 13 letters have an honest 0 in `NO_WIDGETS`).
- t27os: no copy; `specs/os/home.t27` only names `HOME_SPEC = "gHashTag/t27:specs/t27f/petal_menu.t27"` and points at t27. Pointer-only, no duplicate to drift.
- Verdict: MATCH by absence (27 letters, 14 with widgets). If t27os ever gains its own copy, it must agree with 27.

### 4. Kernel agent cap

- t27: `specs/t27os/kernel.t27` defines `agent_cap(mem_bytes, free_bytes, ncpu, load1x100, on_battery) -> u32` (min of memory/cpu/LANE_HARD_MAX, halved on battery, never below 1) plus `cap_reason` and `may_spawn`.
- t27os: no duplicate; pointer-only.
- Swift copies (`trios/rings/SR-00/QueenDelegation.swift`, `queen-core/Sources/QueenPolicy/QueenDelegation.swift`) referenced by swarm-ops are NOT present in this checkout, so `maximumConcurrentWorkers` could not be compared locally. Deferred to the `policy-copies-agree` CI gate on a t27 PR.
- Verdict: MATCH by absence in t27os; Swift-copy agreement UNVERIFIED here (see follow-up).

### 5. Home surface status (t27os-owned)

- t27os `specs/os/home.t27`: `TARGET_COUNT = 2` (trios, ax7203), `TARGET_STATUS = [0, 0]` (both planned), `BOOT_COUNT = 3` (power, home, menu).
- t27: no counterpart (home surface is t27os-owned per the SSOT rule). Pointer check: t27os README and home.t27 both name `petal_menu.t27` as the home geometry owner. MATCH.

## Summary

| Fact | t27 | t27os | Verdict |
|---|---|---|---|
| Petal count | 27 | 27 (pointer) | match |
| specs/t27os count | 58 | 57 (doc) | doc stale |
| Tool-letter count | 27 | pointer | match |
| Kernel agent cap | formula in kernel.t27 | pointer | match; Swift copies unverified |
| Home targets | pointer | 2 planned | match |

## Follow-ups filed

- t27os: update ARCHITECTURE.md "57 specs" -> "58 specs" (labs.t27).
- t27os: add an automatic sync check between the two trees (the "automatic sync check" item under What's missing in t27os#3).
- t27: run `policy-copies-agree` to confirm the two `maximumConcurrentWorkers` copies agree with the spec cap.

## Rerun command

```
cd /Users/playom/t27 && git ls-tree origin/master --name-only specs/t27os/ | wc -l
grep -n "PETAL_COUNT" specs/t27f/petal_menu.t27
grep -n "TARGET_COUNT\|TARGET_STATUS\|BOOT_COUNT" /Users/playom/t27os-repo/specs/os/home.t27
```
## Claim-refs (mandatory step, t27os#3)

Practice followed this session: before taking each issue, read `refs/swarm/claims/*` on t27; after, push `refs/swarm/claims/<N>`. Claim refs written: 11793, 11795 (t27), 4 (t27os), 1717 (trinity). `tri lanes` run before cleanup (64 worktrees, ~30 stale) and after (24 worktrees). The rule is documented in t27 ARCHITECTURE.md (swarm git verbs section) and enforced by the Queen's dispatch (specs/queen/dispatch.t27).
