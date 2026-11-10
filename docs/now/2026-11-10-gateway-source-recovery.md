# Gateway source recovery report — 2026-11-10

ORDER: T27_GATEWAY_SOURCE_RECOVERY_V2. Verdict: the observation-only gateway
source was never lost — it was RECOVERED, not recreated.

## Wave 1 — artifact search (DONE)

- Commit `938d253ad2` (M1 gateway V2: MCP law in .t27) exists in the shared
  object database; commit `73f21c3b62` (gateway tools in tools/list, owner
  budget approval 2026-11-10) exists and is the HEAD.
- They live on branch `lane/mcp-m0`, checked out at
  `.claude/worktrees/mcp-m0`, worktree CLEAN (no uncommitted changes).
- Files present: `mcp/gateway/policy.t27` (143+ lines of law),
  `gen/rust/tri/gateway_policy.rs` (318 lines, committed with `-f`),
  `cli/tri-mcp/src/main.rs` (40-line transport adapter, mounted via
  `#[path = "../../../gen/rust/tri/gateway_policy.rs"]` — no decisions in Rust).
- Earlier alignment report wrongly named the path `/Users/playra/t27/mcp/gateway`;
  the canonical source is this branch/worktree.
- `/Users/playra/t27-mcp/server.mjs` NOT installed, NOT modified.

## Wave 2 — verification (DONE, no recreation needed)

- Seal `.trinity/seals/gateway_McpM1GatewayPolicy.json` (schema 2, ring 12,
  sealed 2026-11-10T16:08:12Z by t27c-bootstrap@0.5.2):
  `spec_hash sha256:c4f2de51…` == measured SHA-256 of `policy.t27` MATCH;
  `gen_hash_rust sha256:007f16dd…` == measured SHA-256 of `gateway_policy.rs` MATCH.
- Law content: exactly three tools (gateway.status, dsh.status, cdp.observe);
  deny-by-default table; loopback-only scopes (dsh 127.0.0.1:3081,
  cdp 127.0.0.1:9222..9332); rate limit denies the 31st request in 60s (30/60s);
  audit event with seven fields, no secret bytes; deny codes DENY_PROC_SPAWN
  (shell), DENY_FILE_WRITE, DENY_CREDENTIAL, DENY_BROWSER_ACT, DENY_SELF_MODIFY.

## Wave 3 — tests (DONE, both compilers green)

- `t27c test mcp/gateway/policy.t27`: 11/11 passed (0.17.1 toolchain).
- `t27b test mcp/gateway/policy.t27`: 11/11 passed + invariants 2 held, 0 broken.
- Required negative paths all covered by tests 3,4,5,6,9,10 + asserts at
  policy.t27:310-311: unknown tool DENY, non-loopback DENY, rate limit DENY_RATE,
  shell spawn DENY, file write DENY, credential access DENY, browser action DENY,
  policy self-modification DENY.

## Wave 4 — install readiness (PREPARED, install deferred to owner)

- Local plugin path: `/Users/playra/t27/.claude/worktrees/mcp-m0` (branch
  `lane/mcp-m0`, HEAD `73f21c3b62`); entrypoint: `cli/tri-mcp` (stdio JSON-RPC).
- Tool list: gateway.status, dsh.status, cdp.observe (schemas in main.rs:313-315).
- Dry-run evidence: from the sealed commit records — initialize OK,
  gateway.status ALLOW with connected=false (no live DSH/CDP), non-loopback
  dsh.status structurally DENIED, unknown method -32601, shell.run refused,
  cdp.observe loopback ALLOW. A FRESH dry-run binary was NOT built in this
  session: swap 16.9/17.4 GB used, load 30+ — build HOLD.
- NO install performed; waiting for owner approval in a privileged session.

## Remaining gaps

1. Fresh stdio dry-run re-execution (needs swap normalisation; a tri-mcp binary
   is not prebuilt in this worktree).
2. Owner approval for observation-only install.
3. Pre-existing 8 parse failures elsewhere: separate issues, untouched here.
4. Recovery-branch untracked `specs/fpga/synth_receipt.t27`: decision still open
   (restore/commit/park).
