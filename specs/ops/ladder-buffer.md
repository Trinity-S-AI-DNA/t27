# Ladder Buffer — operational runbook

## Problem

`ladder-native` (127.0.0.1:3131) is a **single-worker** server. While one model
call is in flight it stops calling `accept()`. Slow rungs (nvidia 79–139 s,
zai 429 flaps + 8–36 s successes) fill the kernel accept queue, and every new
client — including `/health` — gets `connect ETIMEDOUT 127.0.0.1:3131`. A
restart does **not** fix it; the problem is structural.

## Solution

`ladder-buffer` sits in front of ladder-native on `127.0.0.1:3133`:

- instant `accept` for clients (kernel queue never fills)
- exactly one upstream socket at a time (matches what the ladder can do)
- extra requests queue **in memory** — nothing times out while waiting
- `/health` answers immediately (never blocks on a model call)
- connection-level retry only (never retries a model error the ladder returned)
- loopback only on both sides

## Files (runtime, outside the repo)

| File | Purpose |
|------|---------|
| `~/.t27os/bin/ladder-buffer.mjs` | Node.js proxy implementation |
| `~/Library/LaunchAgents/ai.t27.ladder-buffer.plist` | launchd service (`KeepAlive`) |
| `~/.t27os/ladder-buffer.log` | proxy logs |
| `~/.t27os/ladder-buffer.state` | advisory state (unused) |

The `.mjs` and `.plist` are **not in the git repo** (Only t27 rule: no new
foreign files). They are referenced by the spec
`specs/ops/ladder-buffer.t27`.

## DSH routing

The DSH provider overlay (`t27-godmode.patch.yml`) rewrites the `t27-ladder`
and `auto` `baseURL` from `http://127.0.0.1:3131/v1` to
`http://127.0.0.1:3133/v1`. Direct `nvidia` / `zai` routes are unchanged.

The overlay is applied as an extra patch layer:

```
dsh --profile web --patch <kit>/dsh/t27-godmode.patch.yml \
                 --patch <kit>/dsh/opencode-free.patch.yml --port 3080 --no-open
```

## Services

| Label | Port | Status |
|-------|------|--------|
| `ai.t27.ladder` | 3131 | ladder-native (single worker) |
| `ai.t27.ladder-buffer` | 3133 | buffering proxy |
| `ai.t27.dsh` | 3080 | DSH web service |
| `ai.t27.zen` | 3132 | OpenCode Zen facade |

Check with:

```
launchctl print gui/$(id -u)/ai.t27.ladder-buffer
curl -s http://127.0.0.1:3133/health   # instant, never blocks
```

## Verified chain

```
DSH :3080
  → t27-ladder
  → ladder-buffer :3133
  → ladder-native :3131
  → provider (nvidia / zai / openrouter / ollama)
```

E2E confirmed: `PROXY-OK`, HTTP 200, ~9.6 s through nvidia.

## Next steps (for the next agent)

1. **Add a queue cap + alerting** — `QUEUE_CAP` is currently `0` (unbounded).
   Set a cap and alert when `queueDepth` grows.
2. **Codify build/restore path** — `ladder-native` source needs a build path
   in the repo so we are not dependent on a binary.
3. **Multi-worker / async upstream** — the real fix for the single-worker
   ceiling (1 request / ~15 s).

## Logs

- Proxy: `~/.t27os/ladder-buffer.log`
- Ladder: `~/.t27os/ladder.log`
- DSH: `~/.dsh/dsh-web.log`

## Token

DSH token URL rotates on every restart:

```
grep -oE 'http://127\.0\.0\.1:3080/\?token=[A-Za-z0-9_-]+' ~/.dsh/dsh-web.log | tail -1
```
