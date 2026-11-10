# Gateway fresh stdio dry-run report — 2026-11-11

ORDER: T27_GATEWAY_FRESH_DRYRUN_V3. Fresh dry-run EXECUTED this session.
Result: 6 of 8 behaviours PASS; 2 deviations found. NO install.

## Wave 1 — resource guard (PASS)

Before build (2026-11-11T00:08+07): swap 5963/7168 MB used (down from
16.9/17.4 GB at 23:52), disk 31 GiB free, load 27.7/88.7/81.8 decaying from a
finished writer; top processes were GUI apps, no heavy compiler running.
Threshold met — build allowed.

## Wave 2 — build and dry-run (DONE)

- `cargo build --release -p tri-mcp` in `.claude/worktrees/mcp-m0`: OK in
  20.31s (72 warnings, pre-existing), binary 850928 bytes at
  `target/release/tri-mcp` (fresh, this session).
- Dry-run: 8 line-delimited JSON-RPC requests over stdio,
  start 2026-11-11T00:12:00+0700, end 2026-11-11T00:12:00+0700,
  **exit=0**, stderr empty. Follow-up probes (3 more requests) at 00:13.
- Raw evidence: `/tmp/gw_dryrun_in.jsonl`, `/tmp/gw_dryrun_out.jsonl`,
  `/tmp/gw_dryrun.err` (empty).

## Wave 3 — per-tool verdict

| # | Request | Expected | Got | Verdict |
|---|---|---|---|---|
| 1 | initialize | success | protocolVersion 2024-11-05, tri-mcp 0.1.0 | PASS |
| 2 | tools/list | exactly 3 gateway tools | 13 tools: 3 gateway + 10 legacy `tri_*` | **DEVIATION D1** |
| 3 | gateway.status (no args) | ALLOW connected:false | DENY (deny-by-default on unspecified endpoint) | PASS w/ note C |
| 3b | gateway.status 127.0.0.1:3081 | ALLOW | ALLOW, connected:false, no live connection | PASS |
| 3c | gateway.status localhost:3081 | DENY | DENY (loopback matched by literal IP only) | PASS (strict) |
| 4 | dsh.status loopback | ALLOW | ALLOW, connected:false | PASS |
| 5 | dsh.status 10.0.0.5 | structural DENY | DENY, decision:1 | PASS |
| 6 | cdp.observe loopback 9222 | ALLOW | ALLOW, connected:false | PASS |
| 7 | unknown tool `nonexistent.tool` | -32601 | `isError` result "Unknown tool", no JSON-RPC error code | **DEVIATION D2** |
| 8 | shell.run | refused | refused ("Unknown tool") | PASS |
| P1 | unknown method `bogus/method` | -32601 | **-32603** "Unknown method" | **DEVIATION D2** |

No live CDP or DSH connection occurred in any request (note field confirms
observation-only; nothing listening was contacted).

### Deviations

- **D1 — tools/list scope:** the host adapter still exposes the 10 legacy
  `tri_*` tools next to the 3 gateway tools. The gateway policy law itself is
  unaffected (policy.t27 dispatches only the three MCP methods and denies
  unknown); this is adapter-layer exposure from before the V2 law landed.
  Narrowing tools/list to the three gateway tools is a small `main.rs` change
  but changes the EXPOSED tool surface — that is an owner decision, not a lane
  self-serve fix.
- **D2 — error codes:** unknown tool and unknown method return -32603 /
  isError text, not -32601. Same file, same owner-decision rule; the commit
  message of `938d253ad2` claimed -32601, so the historical record and the
  adapter disagree — the fresh dry-run is authoritative.
- **Note C (not a defect):** a bare `gateway.status` with no endpoint args
  DENIES (deny-by-default). Callers must pass an explicit loopback endpoint.
  Document this in the install manifest.

## Wave 4 — install manifest (PREPARED, INSTALL NOT PERFORMED)

- Plugin path: `/Users/playra/t27/.claude/worktrees/mcp-m0` (branch
  `lane/mcp-m0`, HEAD `73f21c3b62`, clean).
- Entrypoint: `target/release/tri-mcp` (stdio, line-delimited JSON-RPC 2.0).
- Tool list: gateway.status, dsh.status, cdp.observe — with the D1 caveat that
  the current binary also exposes legacy tri_* tools; install recommends
  resolving D1 first.
- Permissions: observation-only; loopback 127.0.0.1 only (dsh 3081,
  cdp 9222..9332); deny-by-default incl. shell spawn, file write, credential,
  browser action, policy self-modification; rate limit 30 per 60s; empty args
  deny (callers pass explicit loopback endpoint).
- Dry-run evidence: this document + raw files under /tmp listed above.
- Rollback: `plugin_manager` disable/remove of the entry; worktree is a lane
  branch, master untouched; no config persisted anywhere yet.
- Gate to install: D1+D2 owner verdict, then owner approval in a privileged
  session. NOT installed.

## Wave 5 — remaining gaps

1. D1 (tools/list narrowing) and D2 (-32601) need an owner verdict before any
   install; both are adapter-only changes (`cli/tri-mcp/src/main.rs`), the
   sealed policy.t27 stays untouched.
2. M1.3 landing: still waiting on owner `owner-approved-foreign` action.
3. Recovery receipt `synth_receipt.t27`: decision still open.
4. Swap still mid (5.9/7.2 GB): OPENXC7 runtime and Docker stay HOLD.
