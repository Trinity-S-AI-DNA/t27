# Gateway adapter fix + repeat dry-run report — 2026-11-11

ORDER: T27_GATEWAY_ADAPTER_FIX_AND_INSTALL_V5. D1 and D2 FIXED, repeat
dry-run FULLY GREEN. Install: READY, but the install action itself could not
be executed in this session (see Wave 4).

## Wave 1-2 — adapter fixes (DONE, adapter-only)

Commit `22965a8ba6` on `lane/mcp-m0` (hooks own-language + secret-gate green,
no bypass): `cli/tri-mcp/src/main.rs`, 39 insertions / 2 deletions — within
the 40-added-line foreign budget.

- **D1:** `handle_tools_list` filters to exactly `gateway.status`,
  `dsh.status`, `cdp.observe`; legacy `tri_*` handlers remain transport-only
  and unlisted. No new tools; `policy.t27` untouched (seal hashes still
  match — the spec was not edited).
- **D2:** new `jsonrpc_error_code` helper: messages starting with "Unknown"
  map to JSON-RPC **-32601**, everything else to -32603. Unknown tools now
  surface as JSON-RPC errors, not text results.
- **Regression test:** `gateway_adapter_tests::adapter_d1_d2` —
  `cargo test -p tri-mcp`: 1 passed, 0 failed.

## Wave 3 — repeat dry-run (FULLY GREEN)

Same 8 requests, fresh binary, 2026-11-11T00:18:57+0700 -> 00:18:58+0700,
**exit=0**, stderr empty (`/tmp/gw_dryrun_out2.jsonl`, `/tmp/gw_dryrun2.err`):

| # | Request | Verdict |
|---|---|---|
| 1 | initialize | PASS (2024-11-05) |
| 2 | tools/list | PASS — exactly 3 gateway tools (D1 fixed) |
| 3 | gateway.status no args | DENY (deny-by-default; documented) |
| 4 | dsh.status loopback 3081 | PASS — ALLOW, connected:false |
| 5 | dsh.status 10.0.0.5 | PASS — structural DENY |
| 6 | cdp.observe loopback 9222 | PASS — ALLOW, connected:false |
| 7 | unknown tool | PASS — **-32601** (D2 fixed) |
| 8 | shell.run | PASS — refused, -32601 |

Probe on the new binary: `gateway.status` with `127.0.0.1:3081` ->
ALLOW, connected:false, policy_version 1. No live CDP/DSH connection occurred.

## Wave 4-5 — install gate and attempt (GATE PASSED, ACTION BLOCKED)

The gate is fully green and the owner approved observation-only install.
The action could not run here:

- The DSH `plugin_manager` has no mechanism to register an arbitrary local
  stdio binary; doing it properly means authoring a Cordis bundle package for
  the gateway.
- Bundle installation/approval actions in this session are auto-rejected
  (approval prompts are disabled) — a privileged session is required.

Install spec (ready to execute in a privileged session):
- plugin path: `/Users/playra/t27/.claude/worktrees/mcp-m0` (HEAD `22965a8ba6`)
- entrypoint: `target/release/tri-mcp` (stdio, line-delimited JSON-RPC 2.0)
- mode: observation-only; loopback 127.0.0.1 only (dsh 3081, cdp 9222..9332);
  deny-by-default; rate limit 30/60s; callers pass explicit loopback endpoints
- rollback: remove/disable the registered entry; the worktree is a lane
  branch, master untouched, no config persisted yet.

## Wave 6 — remaining gaps

1. Install execution in a privileged session (everything else is green).
2. Post-install verification checklist is the same 8-request dry-run.
3. Still open: M1.3 `owner-approved-foreign`; `synth_receipt.t27`
   restore/commit/park; swap 5.9/7.2 GB keeps OPENXC7/Docker HOLD.
