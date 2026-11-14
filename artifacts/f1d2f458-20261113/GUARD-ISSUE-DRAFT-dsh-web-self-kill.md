# GUARD ISSUE (draft) — deepseek-ai/deepseek-harness issues disabled; file when issues re-enabled or via internal tracker

**Title:** Guard: a session's bash can SIGKILL its own host web process (incident 2026-11-13, port 3081)

## Incident summary
Autonomous web session `session-f1d2f458-eb2b-4d03-a1d3-92868ba7bbe3` (workdir `~/t27`) fought broken screenshot/CDP tooling and ran bash: "Find and kill dsh web processes"; `killall "BrowserOS neo"; ...`. The broad-pattern kill matched its own host (`apps/cli/lib/bin.js web --port 3081`). launchd KeepAlive restarted; an interactive browser tab kept feeding "дальше"; the durable session resumed and killed again. **7 PID changes in one evening**, each rotating the login token and breaking all open client tabs ("hung UI").

## Evidence
- Restart pattern: 7 tokens in ~90 min in `~/Library/Logs/t27os/dsh-web.out.log`; `launchctl print` `last exit code = -9` repeatedly.
- Session log: 28,119 events; reasoning "The kill command seems to have had an issue..." at turn ~119; 13,060 bash calls; 1,739 upstream 429s; 3 failed summarizations. Log kept at `~/.dsh/sessions/--Users-playra-t27--/session-f1d2f458-eb2b-4d03-a1d3-92868ba7bbe3/session.v4.jsonl.zstd` (19.6 MB).
- Feeder: 22 "дальше" messages, irregular gaps 0m..892m — interactive client, NOT cron (crontab, launchd agents, greps: zero hits).

## Requested guard (separate PR, not shipped during incident)
1. Session tool bash must not kill its own host web/server process: reject broad-pattern kills matching the host cmdline unless operator-allowlisted.
2. Explicit allowlist for managed restart (launchd unit), idempotent restart path.
3. Single-instance guard per port (fail loud on second bind).
4. Quarantined session dirs must not silently resume on boot (marker file).
5. Audit log of kill/restart attempts by sessions.

## Implementation sketch
At bash tool execution, resolve host process chain (server PID → argv); on kill-family commands, intersect target patterns with the host chain; typed rejection error naming the protected process. Lock file per port. `.quarantined` marker respected at boot.

Refs: `~/t27/HANDOVER-20261113-dsh-web-incident.md`; quarantine used: `~/.dsh/sessions/quarantine-20261113/` (session restored by owner after).
