# GitHub automation inventory (T27_GITHUB_AUTOMATION_V1)

Measured 2026-11-10 via gh api.

## Repository

- `Trinity-S-AI-DNA/t27`, default branch `master`, discussions enabled.

## Issues and milestones

- Open issues: 2020.
- Milestones: EPOCH-01-HARDEN (closed, 20/20), Phase 3 Science Tests (closed,
  3/3), T27_MCP_RUNTIME_M0 (closed, 5/5), M0_PROMOTION_KERNEL_SPRINT (open,
  10 open / 0 closed). Due slots are order, not calendar days
  (`specs/automation/t27_plan.t27`: DUE_IS_ORDER).

## Existing labels (mapping targets for Area/Priority)

- Priority: `P0`, `P1`, `high-priority`, `priority/critical`, `priority/high`,
  `priority/medium`, `priority/low`.
- Area: `area/compiler`, `area/host`, `area/infra`, `area/rtl`, `area/specs`,
  plus `compiler`, `t27os`, `docs`, `numeric`, `theory`, `tooling`,
  `observability`, `ux-docs-task`.
- State: `blocked`, `in-work`, `status/blocked-hardware`, `no-auto-merge`,
  `epic`, `initiative`, `lane`, `one-shot`, `queen-shaped`.

## Discussions

- 0 discussions. Categories exist: Announcements, General, Ideas, Polls,
  Q&A, Show and tell. Missing vs target set: `Design proposals` (Idea format);
  the REST/GraphQL API cannot create discussion categories, so it is a
  one-click manual step.

## Projects v2

- `gh project list` is BLOCKED for the current token: missing scopes
  `read:project` / `write:project`. Everything in WAVE 2-4 (project creation,
  fields, views, auto-add) requires `gh auth refresh -s project` by the owner.

## New labels created by this setup

- `triaged`, `needs-owner`, `converted-to-issue` (discussion triage);
  `duplicate` and `wontfix` already existed.
