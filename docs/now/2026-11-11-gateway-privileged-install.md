# Gateway privileged install report — 2026-11-11

ORDER: T27_GATEWAY_PRIVILEGED_INSTALL_V6. INSTALL EXECUTED AND VERIFIED:
the observation-only gateway is live in the DSH profile as server
`t27gateway` with exactly three tools.

## Wave 1 — resource guard (PASS, 00:30+07)

Swap 9217/10240 MB, load 6.79/23.46/40.39 (1-min calm), disk 25 GiB free.
No heavy writer (1-min load single digits). Proceed.

## Wave 2 — install spec (DONE: minimal local Cordis bundle)

- Bundle: `/Users/playra/t27/.claude/local-bundles/t27-gateway-mcp/`
  (`@local/t27-gateway-mcp` 1.0.0, private, no npm publish, no secrets).
  Files: `package.json`, `cordis.patch.yml`, localized `locale/*.json`
  (7 languages), `icon.svg` (template copy).
- Patch inserts `@deepseek-ai/dsh-mcp-client`, serverName `t27gateway`,
  transport **stdio**, command
  `/Users/playra/t27/.claude/worktrees/mcp-m0/target/release/tri-mcp`,
  cwd the mcp-m0 worktree, failOnStartupError true.
- Install: `plugin_manager install_bundle` -> **application: applied**,
  exit 0. Only warning: unrelated account-controller pending.
- Binary unchanged: HEAD `22965a8ba6` (D1+D2 fixes), policy.t27 seal intact.

## Wave 3 — post-install dry-run (through the INSTALLED server)

- Registration: `Tool.listTools` shows exactly three gateway tools:
  `mcp__t27gateway__gateway_status_*`, `mcp__t27gateway__dsh_status_*`,
  `mcp__t27gateway__cdp_observe_*`. MCP handshake (initialize + tools/list)
  succeeded — that is what registers tools.
- `dsh.status` non-loopback (10.0.0.5:3081): **DENY surfaced end-to-end**
  ("gateway policy denied", decision:1) — proves live round-trip through the
  installed plugin and the loopback-only law.
- `gateway.status`, `dsh.status` loopback, `cdp.observe` loopback: called
  through the installed server, no error (ALLOW path). Note: harness surfaced
  ALLOW results as "no model-visible content" — a display quirk of the
  harness MCP client, not a gateway defect; the same binary returns full JSON
  over raw stdio (verified twice: 00:12 and 00:18 dry-runs, both exit=0).
- Unknown tool / shell.run: the harness blocks unknown tool names before they
  reach the server; the -32601 behavior is verified at the stdio level in the
  repeat dry-run and by the `adapter_d1_d2` regression test.
- No live CDP/DSH connection occurred.

## Wave 4 — rollback (documented)

1. `plugin_manager` `set_bundle` `@local/t27-gateway-mcp` enabled:false (or
   `remove_bundle`) in a privileged session.
2. That restores the prior profile state — the bundle is additive; it touched
   nothing else (no unrelated DSH config modified).
3. Verify by `Tool.listTools`: the three `mcp__t27gateway__*` tools disappear.
4. Recovery and M1.3 work are untouched by design (separate branches).

## Remaining gaps

1. ALLOW-path text visibility inside the harness MCP client (cosmetic; raw
   stdio evidence is the authority).
2. Still open: M1.3 follow-ups (report says LANDED as `b693c56d2b` — not
   verified in this session), `synth_receipt.t27` decision, swap for
   OPENXC7/Docker.
