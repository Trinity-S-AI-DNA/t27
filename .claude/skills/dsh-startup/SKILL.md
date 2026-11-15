---
name: dsh-startup
description: >-
  Local (Mac) DSH startup and focus: one launchd instance on 127.0.0.1:3080, read
  the ready URL from web-service.log without printing credentials, open via
  BrowserOS or system browser. Load with dev-monitor for full dev-mode law.
  Never start a second dsh web. For cloud agents load dsh-startup/CLOUD.md instead.
---

# DSH startup (local Mac)

Law: `specs/t27os/dev_monitor.t27`. Full dev-mode moves: `.claude/skills/dev-monitor/SKILL.md`.

## Preconditions

- **One instance only.** Launchd label `ai.t27.dsh-web` owns `127.0.0.1:3080`. If DSH is already running, do **not** start another `dsh web` or `pnpm dsh web`.
- **Port 3080** may show `deepseek-harness` / the dsh listener — that is expected. If another pid holds the port, stop and report (`S_PORT_HELD`); do not kill foreign processes.

## Discover (read-only)

```bash
launchctl print "gui/$(id -u)/ai.t27.dsh-web" 2>/dev/null | /usr/bin/grep -E 'state =|pid ='
lsof -nP -iTCP:3080 -sTCP:LISTEN
curl -s -o /dev/null -w '%{http_code}' http://127.0.0.1:3080/
```

Use `/usr/bin/grep` (not zsh aliased grep).

## Open the workspace (credential-safe)

The ready URL is printed once per start as `dsh web: http...` in `~/.dsh/web-service.log`. **It is a credential.**

- **Never** echo, `cat`, `tail`, screenshot, or paste the URL into chat, a brief, log artifact, or commit.
- **Safe:** count or filter only (`grep -c`, `grep -v` for secrets).
- Load into a shell variable and pass directly to `open` or BrowserOS — **do not print** `$DSH_URL`:

```bash
DSH_URL="$(/usr/bin/grep -o 'dsh web: http[^ ]*' ~/.dsh/web-service.log | tail -1 | cut -c10-)"
# Preferred: BrowserOS neo navigate (no credential in agent transcript)
# navigate "$DSH_URL"
open "$DSH_URL"
```

If `open` is unavailable, ask the owner to open the last ready URL themselves; report `degraded`.

## Model priority (local DSH keys)

Cheapest provider first; keys live in `~/.dsh/.credentials.yaml` on the Mac — **read names only**, never print values:

| Priority | Provider        | Env (names only)     |
|---------:|-----------------|----------------------|
| 1        | z.ai free       | `DSH_ZAI_KEY`        |
| 2        | NVIDIA NIM      | `NVIDIA_API_KEY_1`   |
| 3        | TeamoRouter     | `TEAMO_API_KEY`      |
| 4        | OpenRouter :free| `OPENROUTER_API_KEY` |
| --       | DeepSeek API    | UNFUNDED (402) — do not use |

Pass keys via process **env** at session start when the platform allows; never into prompts, PRs, or skills body.

## Free-first routing

Load `.claude/skills/free-first/SKILL.md` and `specs/claw/model_route.t27` before spawning panes or subagents for DSH-backed work.

## Traps

- `curl` GET without cookie → **401** is normal; it is not “sign-in required” by itself (see dev-monitor).
- Do not clone dsh outside `~/deepseek-harness`; do not `pnpm install` or change ports without owner ALLOW.
- Cloud agents: **this file does not apply** — use `CLOUD.md` in this folder.
