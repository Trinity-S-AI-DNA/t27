# GODMODE Migration Inventory

Phase 1 — Read-only. No code touched.  
Updated: 2026-11-08  
Exit: owner reviews; no migration action starts without approval.

---

## Sessions (Claude Code)

| Session | Branch | Status | Purpose |
|---|---|---|---|
| godmode-v1-executor-8d89e4 (this) | claude/godmode-ring-02-golden-chain | ACTIVE | Ring 2 GODMODE execution |
| queen-vertical-stack (other) | claude/queen-vertical-stack | OPEN | PR #13259 overlaps Ring 2+3 |

## Sessions (DSH)

Worktrees under `.agents/worktrees/`:
- dsh-5ed69103-50a5 (branch: dsh-5ed69103-50a5)
- dsh-9529fc7f-edb6 (branch: dsh-9529fc7f-edb6)
- dsh-a28ba37e-f95e (branch: dsh-a28ba37e-f95e, same commit as above)
- dsh-a697d470-48bf (branch: dsh-a697d470-48bf, same commit as above)

---

## Worktrees and branches

38 total worktrees. Noteworthy:

| Path | Branch | Notes |
|---|---|---|
| ~/t27 | merge-pr-12695 | main clone (not on master) |
| ~/t27-build | master | build worktree |
| ~/t27-master | (detached HEAD) | read-only master pin |
| .claude/worktrees/godmode-v1-executor-8d89e4 | claude/godmode-ring-02-golden-chain | this session |
| .claude/worktrees/admiring-feistel-551b81 | lane/t27f-006-wide-frames | T27F Ring 6 dup (rescue) |
| .agents/worktrees/dsh-* | dsh-* | 4 DSH agent worktrees |
| .agents/worktrees/t27os-browser-petal | tri-rewrite-rust-to-t27 | T27OS browser |
| ~/Documents/Codex/... | codex/wars-baseline-4328 | Codex worktree |

---

## Modified files in this session

- `specs/queen/golden_chain.t27` — section 7 added (Ring 2)
- `.trinity/seals/queen_QueenGoldenChain.json` — re-sealed (Ring 2)

---

## Open PRs (as of 2026-11-08)

| PR | Branch | Status | CI | Notes |
|---|---|---|---|---|
| #13304 | claude/course-v2-lesson-01 | open | unknown | Course Lesson 1 |
| #13301 | claude/godmode-ring-02-golden-chain | open | PENDING | Ring 2 (this session) |
| #13283 | ci/gate-fixes-unblock-merges | open | 9 fail | Fixes documented-commands + duplicate-bodies master reds |
| #13259 | claude/queen-vertical-stack | open | 5 fail | CONFLICT RISK: same files as #13301 |
| #13254 | claude/t27f-007-outfit-type | open | 9 fail | T27F-007 |
| #13250 | claude/t27f-006-frame-widths | open | 3 fail | T27F-006 |
| #13249 | claude/course-bench-still | open | 3 fail | Blog post |
| #13233 | claude/t27b-paper-numbers | open | CI pending | Paper numbers recompute |
| #13223 | lane/t27os-native-0.1.0-alpha.1 | open | unknown | T27OS native alpha.1 |
| #13221 | claude/cool-gauss-06c826 | open | unknown | T27F netlist_graph |

---

## Skills referenced by this session

- `work-cheap` — cost prompts for sub-agents
- `only-t27` — language policy
- `short-terminal-commands` — Bash command length rule
- `issue-shape` — issue gate (Boundary/Scenarios/Requirements/Criteria)
- `self-cleanup` — lane cleanup after merge
- `model-routing` — Sonnet/Opus routing

---

## Hooks (.claude/settings.json)

- 1 Stop hook present (exact command not shown)
- Mac-no-heavy hook: blocks t27c seal, builds, tests via Claude Code Bash

---

## Referenced environment variables (values not shown)

| Variable | Purpose | Used by |
|---|---|---|
| GH_TOKEN / GITHUB_TOKEN | GitHub API (gh CLI) | CI, pre-push hooks |
| T27OS_CTL | T27OS pane control | T27OS tools |
| MAC_HEAVY_OK | Override mac-no-heavy guard | Owner only |

---

## Migration risk classification

| Item | Class | Reason |
|---|---|---|
| Ring 2 PR #13301 | SAFE | Pure .t27 spec + seal, no migration |
| PR #13259 (queen-vertical-stack) | REVIEW | Overlaps Ring 2+3 files; must coordinate order |
| DSH worktrees (.agents/worktrees/dsh-*) | REVIEW | 4 active DSH sessions; unknown state |
| ~/t27 main clone on merge-pr-12695 | REVIEW | Not on master; may cause branch confusion |
| Phase 3 DSH migration | DO_NOT_MIGRATE | Requires explicit owner approval |
| Phase 4 external routing | DO_NOT_MIGRATE | Requires explicit owner approval |
| .claude/hooks.json migration | DO_NOT_MIGRATE | Requires explicit owner approval |

---

## Blockers requiring owner decision

1. **PR #13259 vs #13301 conflict**: Both touch `specs/queen/golden_chain.t27`.
   Owner must decide which PR to merge first (or close #13259 if superseded).

2. **Master reds** (emit-bitexact, duplicate-bodies, documented-commands):
   Pre-existing. PR #13283 is working on a fix. These may prevent auto-merge of #13301.

3. **Phase 3 migration**: No action until owner explicitly approves after reviewing this inventory.

4. **Phase 4 routing**: No action until owner explicitly approves.
