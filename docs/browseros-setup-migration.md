# T27 BrowserOS + Ladder + Bridge -- setup snapshot for machine migration

Snapshot date: 2026-11-02. Source machine: Mac (playra). Everything below was
verified with live tests the same day. Secrets are intentionally absent: key
values stay in the Keychain and ~/.fcc/.env on each machine.

## 1. T27 Ladder -- model router (port 3131)

- File: ~/.t27os/bin/ladder.mjs (V1+V2+V3: sticky rungs, hedged serve).
- Supervisor: launchd ai.t27.ladder (~/Library/LaunchAgents/ai.t27.ladder.plist,
  KeepAlive).
- Patch 2026-11-02: added GET /v1/models route returning one honest id
  glm-5.3-flash (owned_by: t27-ladder). PORT stays 3131.
- Upstream rungs: 0) Z.ai https://api.z.ai/api/coding/paas/v4/chat/completions
  (glm-5.3-flash / glm-4.7-flash / glm-4.5-flash; keys from
  ~/.dsh/.credentials.yaml key DSH_ZAI_POOL), 1) local Ollama 127.0.0.1:11434
  (gemma4:31b-cloud), 2) DeepSeek api.deepseek.com (T27_DEEPSEEK_KEY).
- Migration: copy ladder.mjs + plist; launchctl load -w; verify
  curl http://127.0.0.1:3131/v1/models (200) and POST /v1/chat/completions (200).
- Rollback: remove the GET /v1/models branch, then
  launchctl kickstart -k gui/$(id -u)/ai.t27.ladder.

## 2. FCC bridge -- OpenAI/Anthropic translator (port 8091)

- File: ~/.local/lib/fcc-chat-bridge/bridge.py (stdlib only).
- Fix 2026-11-02: TOKEN no longer defaults to "freecc"; when BRIDGE_TOKEN is
  unset it reads ANTHROPIC_AUTH_TOKEN from ~/.fcc/.env (the same allowed store
  fcc-server on 8082 uses). The value is never logged.
- Pre-fix backup: bridge.py.pre-token-fix.bak.
- Supervisors: launchd ai.fcc.chatbridge; upstream fcc-server on 8082
  (uv tool free-claude-code, launchd ai.fcc.server).
- Route note: the bridge accepts any path ending in /chat/completions, so
  Base URL http://127.0.0.1:8091/v1 is valid for POST. There is NO
  GET /v1/models on the bridge (404 {"detail":"Not Found"}): the UI Test
  button can fail while chat still works.
- FCC Local model: zai/glm-5.3-flash (catalog via GET /admin/api/models on
  8082). Model gpt-6-luna routes to nvidia_nim and 504s (NIM keys list the
  catalog but do not serve inference; see access-map).
- Diagnostics: ~/.fcc/logs/server.log (APITimeoutError = upstream provider
  down; launchctl kickstart -k gui/$(id -u)/ai.fcc.server restarts it).
- Migration: copy bridge.py; ensure ~/.fcc/.env exists (created by fcc first
  run; owner enters secrets personally); launchctl load -w.
- 2026-11-03: the FCC Local card was removed from BrowserOS by the owner
  (gpt-6-luna hit the dead nvidia_nim). The bridge and its fix keep running
  on 8091; restore with the two lines above if ever needed.

## 3. BrowserOS LLM provider card (Settings -> AI & Agents)

```text
Provider: OpenAI Compatible
Name:     T27 Ladder
Base URL: http://127.0.0.1:3131/v1
Model:    glm-5.3-flash
API Key:  ladder            (the ladder does not check Authorization)
Images:   off
Default:  yes (Set as default)
```

Post-entry checks: Test button (ladder log shows rung=0 200), a plain chat
turn, and an Agent Mode task (open file, read marker, return value).

Known UI bug (2026-11-02): after Save/Test the "Configure New Provider"
dialog stays open and empty while the save itself SUCCEEDS. Close with
Escape or a page reload; trust the "Your providers" list, not the dialog.
Extreme case: the tab crashes with "Something went wrong, React error
#185" (maximum update depth). Closing the tab or about:blank -> back
recovers it; saved providers are never lost.

## 4. BrowserOS Chat & Hub (Settings -> Chat, web chat links)

Custom provider:

```text
Name: T27 Ledger
URL:  https://t27.ai
```

Same dialog bug applies; verify via the "Configured AI Providers" list.

## 5. DSH web (port 3080) -- login and its traps

- Service: pnpm -C ~/deepseek-harness dsh web ... (stdout/stderr log at
  ~/.dsh/web-service.log).
- The login token is printed there as
  "dsh web: http://127.0.0.1:3080/?token=..." and CHANGES on every restart.
- Trap 1 (SameSite): the dsh-auth cookie is SameSite=Strict. Opening the
  token URL from a file:// page fails: the follow-up GET / goes out without
  the cookie. Open the token URL directly in the address bar, or redirect
  from an http://127.0.0.1 origin.
- Trap 2: never swap 127.0.0.1 for localhost; the cookie is host-bound.
- Trap 3: never paste the token URL into chats or screenshots. After a
  successful login a ~30-day cookie carries access; the token is not needed
  again until the next restart.

## 6. Copy checklist

```text
~/.t27os/bin/ladder.mjs                   copy (already patched, /v1/models)
~/Library/LaunchAgents/ai.t27.ladder.plist    copy
~/.local/lib/fcc-chat-bridge/bridge.py    copy (already token-fixed)
~/Library/LaunchAgents/ai.fcc.chatbridge.plist    copy
~/Library/LaunchAgents/ai.fcc.server.plist        copy (fcc-server 8082)
~/.dsh/.credentials.yaml                  secrets: owner moves personally
~/.fcc/.env                               secrets: owner moves personally
Keychain (ai.t27.os Safe Storage etc.)    secrets: owner moves personally
```

Order on the new machine: ladder -> fcc-server -> bridge -> BrowserOS cards
(sections 3-4 by hand) -> dsh web -> login per section 5.

## 7. Quick post-migration check

```bash
curl -s http://127.0.0.1:3131/v1/models          # 200
curl -s http://127.0.0.1:3131/health             # ladder: up
curl -s http://127.0.0.1:8082/health             # fcc: healthy
curl -s http://127.0.0.1:8091/health             # bridge: healthy
launchctl list | grep -E "ladder|fcc"            # three jobs alive
```

Synthetic UI-test markers (no personal data): T27_BROWSEROS_OK, T27_FCC_OK,
T27_PAGE_MARKER_* (page t27-browseros-agent-test.html, ~/t27 root; kept as a
local artifact, not committed -- see section 8).

## 8. Local artifacts of this setup (not committed)

- t27os-ladder-models-patch.mjs -- test copy of the ladder (PORT 3132, never
  use as production; production ladder already carries the route).
- t27-browseros-agent-test.html -- Agent Mode marker test page.
