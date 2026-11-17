# T27L audit event schema for promotion_kernel

Part of `[EPIC] T27_PROMOTION_KERNEL_V5` — Part of #13473.
M0-10: T27L audit event schema for promotion_kernel. Closes #13483.

## Goal

Define the T27L audit event schema for the promotion_kernel surface: events for draft, gate check, publish-approved, rate-limit wait.

## Constraints (M0 law)

- Docs/planning only — no X posts, no outbound email, no paid-token usage without owner approval.
- Free models first; on rate limit mark `WAITING_RATE_LIMIT` and stop (no paid fallback).
- Claims stay inside the approved patterns from the epic.
- Boundary: `docs/promotion/t27l-promotion-schema.md`, `docs/promotion/README.md`, `specs/promotion/t27l_promotion.t27`.

## Schema: t27l/v1 on surface promotion_kernel

Every promotion_kernel action emits exactly one `t27l/v1` line with these fields:

| Field | Type | Meaning |
|-------|------|---------|
| `v` | str | schema version, always `"t27l/v1"` |
| `surface` | str | always `"promotion_kernel"` |
| `event` | str | one of `draft`, `gate_check`, `publish_approved`, `rate_limit_wait` |
| `asset` | str | the promotional asset id being audited |
| `claim` | str | the measurable claim being made |
| `artifact` | str | link to the artifact |
| `path` | str | reproducible path |
| `visual` | str | visual proof |
| `cta` | str | call to action |
| `metric` | str | metric to track |
| `decision` | str | `approve` or `refuse` — the publish gate verdict |
| `reason` | str | why, when `decision == "refuse"` |
| `ts` | str | ISO-8601 UTC timestamp |

## Event reference

- `draft` — an asset is drafted; all six fields (claim/artifact/path/visual/cta/metric) are recorded, decision defaults to pending.
- `gate_check` — the publish gate runs: every listed promotional item must have claim/artifact/visual/CTA/metric fields or an explicit N/A with reason. Missing element → refuse.
- `publish_approved` — the gate passed and the asset is approved for the owner to publish (owner approval still required before any actual post).
- `rate_limit_wait` — a free-model rate limit was hit; mark `WAITING_RATE_LIMIT`, save artifacts, do **not** switch to a paid model.

## Publishable-asset rule (the gate)

Every draft promoted to `publish_approved` must contain all six:

1. one measurable claim
2. one artifact link
3. one reproducible path
4. one visual proof
5. one CTA
6. one metric to track

If any element is missing, the asset is **not publishable** and the gate emits `gate_check` with `decision == "refuse"`.

## What this schema does NOT do

- It does not post or transmit (the module is audit-only, see `specs/promotion/t27l_promotion.t27`).
- It holds no address and no secret in the payload.
- It does not switch to a paid model on rate limit.

## Linked assets

- `docs/promotion/README.md` — promotion kernel index
- `specs/promotion/t27l_promotion.t27` — the .t27 module that types this schema

// phi^2 + 1/phi^2 = 3 | TRINITY
