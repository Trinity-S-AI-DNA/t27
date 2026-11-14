# M1 Gateway V1 — observation-only control plane

ORDER: T27_M1_GATEWAY_V1 (2026-11-10). The law is
[`mcp/gateway/policy.t27`](policy.t27); the audit-event schema is
[`mcp/gateway/schema.md`](schema.md). This README is the surface map, not a
second law.

## Tools (V1: read-only, exactly three)

| Tool | Answers | Endpoint |
|---|---|---|
| `gateway.status` | policy version, uptime counters, deny-table digest | loopback only |
| `dsh.status` | DSH health read, proxied | `127.0.0.1:3081` (auth-gated; measured 2026-11-10) |
| `cdp.observe` | target list, URL read, screenshot, AX/DOM snapshot | `127.0.0.1:9222..9332` (dedicated observer profile) |

There is **no fourth tool**. The spec's tool table has three rows and the
gate answers `DENY` for any index outside it.

## Explicitly absent in V1

- No write tools of any kind (M0's write tools live in
  `specs/mcp/t27_dsh_mcp.t27` and stay behind its dry-run/confirmation gate;
  the gateway does not expose them).
- No shell execution (`DENY_PROC_SPAWN`, op 10).
- No arbitrary file access (`DENY_FILE_WRITE`, op 11).
- No credential access (`DENY_CREDENTIAL`, op 12): no secret, token or
  endpoint credential is read, stored or proxied by the gateway; DSH
  authentication stays with the human-side session, outside gateway state.
- No browser mutation (`DENY_BROWSER_ACT`, op 13).
- No policy self-modification (`DENY_SELF_MODIFY`, op 14): the gateway
  process cannot change this spec at runtime — every runtime decision is one
  of the spec's verdicts (`ALLOW`, `DENY`, `DENY_RATE`) and no branch of
  `gate()` reads caller-controlled policy state.

## Cross-cutting rules

- Loopback only: both endpoint checks refuse anything that is not
  `127.0.0.1` on the named ports.
- Rate limit: at most 30 requests in any 60 s window per tool
  (`RATE_MAX`, `RATE_WINDOW_S`); the 31st request gets `DENY_RATE`.
- Every decision — allow or refusal — emits one audit event with seven
  fields (see schema.md); the request is hashed (FNV-1a 32 in u64 lanes, no
  wrapping arithmetic), never carried.

## Verification

```sh
./target/release/t27c test mcp/gateway/policy.t27
./target/release/t27b test mcp/gateway/policy.t27
```

Measured 2026-11-10: `t27c` 8/8 pass, `t27b` 8/8 pass + 2 invariants held.
