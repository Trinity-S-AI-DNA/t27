# Issue Milestone Coverage Plan — T27_ISSUE_MILESTONE_COVERAGE_V1

Date: 2026-11-10. Inventory: `docs/coordination/issue-milestone-inventory.json` (7778 issues, all states).

## Milestone inventory (state=all)

| # | Title | State | Open | Closed |
|---|-------|-------|------|--------|
| 1 | EPOCH-01-HARDEN | closed | 0 | 20 |
| 2 | Phase 3 — Science Tests | closed | 0 | 3 |
| 3 | T27_MCP_RUNTIME_M0 | closed | 0 | 5 |
| 4 | M0_PROMOTION_KERNEL_SPRINT | open | 8 | 0 |

## Coverage picture

- TOTAL issues: 7778. With a milestone: 34. Unassigned: 7744.
- All 34 existing assignments are **already correct**: 18 closed hardening issues in #1, 3 closed science tests in #2, 5 closed M0-runtime issues in #3, 8 open sprint issues in #4. No issue sits on a wrong milestone; no open issue sits on a closed milestone.

## Proposed changes (2 issues)

| Issue | Current | Proposed | Reason | Confidence | Command |
|-------|---------|----------|--------|-----------|---------|
| 13465 T27_PROMOTION_KERNEL_V5 | none | M0_PROMOTION_KERNEL_SPRINT #4 | Open promotion-kernel epic — matches the #4 rule set | high | `gh issue edit 13465 --repo Trinity-S-AI-DNA/t27 --milestone "M0_PROMOTION_KERNEL_SPRINT"` |
| 13462 grok crm: T27L audit projection | none | M0_PROMOTION_KERNEL_SPRINT #4 | Open T27L audit issue — matches the #4 rule set | high | `gh issue edit 13462 --repo Trinity-S-AI-DNA/t27 --milestone "M0_PROMOTION_KERNEL_SPRINT"` |

## UNASSIGNED_CANDIDATE (7744 issues)

Every other unassigned issue (LSP epics ~100, spec-port backlog ~400, System pulse recurrences ~200,
analog/compiler/fpga/t27os/t27b work, closed history) has **no clear owner among the four milestones**
under the triage rules. Per the order: they stay UNASSIGNED_CANDIDATE. Do not guess. They are not moved.

## No-move guarantees

- No issue leaves a milestone. No milestone is created. No issue state, title, body, labels or
  assignees change. 8f70733e79 is untouched.

## Apply batch plan (after approval)

Batch 1 (2 issues, one batch, ≤25): 13465, 13462 via the commands above, then re-run the inventory
and verify: #4 open count 8 → 10, no other count changes.
