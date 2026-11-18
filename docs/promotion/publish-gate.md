# Publish gate (M0-12)

Part of `[EPIC] T27_PROMOTION_KERNEL_V5` — Part of #13473.
M0-12: Publish gate (widget + screenshot + test + human approval). Closes #13485.

## Goal

Define the publish gate: first proof-based post only after widget + screenshot + test pass, and human approval is required.

## Publish gate rules

A promotional post may be published **only if all four conditions are met**:

| Condition | Required evidence | Check |
|-----------|-------------------|-------|
| Widget | Compiled t27 spec with visual output | Spec typechecks + tests pass |
| Screenshot | Generated artifact from widget pipeline | PPM/PNG file exists, >0 bytes |
| Test | Widget test suite passes | `t27c test` exit 0 |
| Human approval | Owner explicitly approves publication | Written approval with date |

If any condition is missing, the gate **refuses** publication. No exceptions.

## Gate workflow

```
1. Widget spec written and sealed
   └─→ t27c check passes? ──→ No: refuse
   └─→ Yes → t27c test passes? ──→ No: refuse
   └─→ Yes → Pipeline generates screenshot? ──→ No: refuse
   └─→ Yes → Human approval obtained? ──→ No: refuse
   └─→ Yes → GATE OPEN: publish allowed
```

## M0 constraints

- No X posts without owner approval
- No outbound email without owner approval
- No paid-token usage without owner approval
- On rate limit: mark `WAITING_RATE_LIMIT`, stop, do not switch to paid model

## Linked assets

- `docs/promotion/README.md` — promotion kernel index
- `specs/automation/promotion_kernel.t27` — proof-to-demand gate contract (#13465)
- Epic #13473 — T27_PROMOTION_KERNEL_V5

// phi^2 + 1/phi^2 = 3 | TRINITY
