# MCP Connector M0 — Inventory (WAVE 1)

Measured 2026-11-10, host `playra-mac` (Asia/Bangkok). Every fact below was
measured with the named command; nothing is assumed. Issue: #13583.

## 1. .t27 runtime

- Compiler: `t27c` built from master `116178f1d9` at
  `~/t27/target/release/t27c` (`cargo build --release -p t27c`; workspace
  member `bootstrap/`, binary lands at the workspace root).
- Commands used by M0: `t27c check <spec>` (typecheck), `t27c seal <spec>
  --save|--verify` (seal hashes into `.trinity/seals/<module>.json`),
  `t27c test-report <spec>` (per-test pass/fail table).
- Capability model: specs generate backends via `t27c gen-*` (L2); specs with
  `@extern` (T27 OS host files) are NOT compiled by `t27c` and are out of scope
  for M0 — every M0 spec is extern-free and self-contained.
- Existing MCP family: `specs/mcp/tool_registry.tri` (ring 105-004) defines
  `ToolDescriptor`, `RegistryError`, `InvocationRecord` — the M0 contract
  reuses its descriptor shape (id, schema hash) without duplicating the
  registry; M0 adds the authorization layer, not a second registry.

## 2. DSH endpoint / auth

- DeepSeek Harness Web GUI answers at `http://127.0.0.1:3081`;
  `curl /api/health` returned `unauthorized` (HTTP-level auth gate) — the API
  is reachable on loopback and refuses unauthenticated reads. Good default.
- Session identity comes from environment (`DSH_SESSION_ID`, `DSH_PROFILE`,
  `DSH_HOME`, `DSH_WEB_URL` observed set in the session env).
- M0 contract: the adapter takes its token from the environment only
  (`T27_DSH_TOKEN`), never logs it, never writes it to a spec or audit record;
  the audit log stores an args hash, not arguments.

## 3. CDP endpoint

- BrowserOS neo 155.0.8318.26 is running (`pgrep`), but its main process
  carries NO `--remote-debugging-port` and no `--user-data-dir` override
  (checked via `ps` over the browser main processes).
- Port 9222 is closed (`curl http://127.0.0.1:9222/json/version` fails).
- M0 therefore defines the observer against a DEDICATED Chrome profile that
  the user starts with
  `--remote-debugging-port=127.0.0.1:PORT --user-data-dir=<scratch>`; the
  observer binds to `127.0.0.1` only and never to the user's daily profile.
  Until that profile runs, `browser.observe` returns
  `CDP_UNREACHABLE` (fail-closed, no retry storm).

## 4. Agent permission matrix (M0 defaults)

| Operation | Read | Write | Confirmation |
|---|---:|---:|---|
| DSH/agent status | yes | no | no |
| Task list/report | yes | no | no |
| Create bounded task | no | yes | dry-run first |
| Stop agent | no | yes | yes |
| Shell run | no | yes | yes + allowlist |
| Browser observe | yes | no | no |
| Browser act | no | yes | yes |
| Push/merge/close issue | no | no | owner-only, never via M0 |

- Allowlists live in `specs/mcp/policy.t27` as data and are deny-by-default:
  repos `gHashTag/t27` (this repo, `Trinity-S-AI-DNA/t27` on GitHub),
  branches `lane/*` and `claude/*`, paths under `specs/mcp/`, `docs/mcp/`,
  `tests/`; commands `status`, `list`, `report`; network egress limited to
  `127.0.0.1` (DSH GUI port 3081, CDP observer port); `sudo` denied outright.
- Worktree discipline (owner rule 2026-10-25): this lane works in
  `~/t27/.claude/worktrees/mcp-m0`, branch `lane/mcp-m0`; no clone or scratch
  tree outside `~/t27`.

## 5. Facts that bound the design

- `;` comments are legal ONLY at module level in a `.t27` spec (PolicyBudgetDiscipline,
  trap #2) — no comments inside test/fn bodies.
- Foreign hand-written code is denied by hooks/CI; every M0 artefact is
  `.t27` or `.md`. The `tests/` the ORDER mentions are the in-spec `test`
  blocks (L4), plus `docs/mcp/` prose.
- Disk at lane start: 15 GiB free (> 5.4 GiB a worktree needs).
