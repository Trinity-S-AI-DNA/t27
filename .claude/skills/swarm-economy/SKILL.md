---
name: swarm-economy
description: Run T27 OS work cheaply when the weekly Claude limit is low -- free-model actors and bees do the work, Claude only lands, watches and unblocks; hourly pulse of bee load and token stock on the dashboard so nothing idles. Use when the owner says "economically", "few tokens left", "through our actors", or when a loop must watch that everyone works at full capacity.
---

# Swarm economy (owner, 2026-10-20)

Owner: "fix everything through our actors, economically -- few tokens left on the weekly limit";
"switch on a cron that watches that everyone works at maximum efficiency"; "show bee load and token
stock online on the dashboard so there is no idle time". Deadline set by the owner: Saturday 2026-10-24.

## Who does what

| Work | Who | Model / cost |
|---|---|---|
| Specs, ports, fixes, tests | bees, Queen lanes, T27 OS panes (OpenCode, Crush, Codex) | free tiers via the key master: glm-4.5-flash (7 keys), OpenRouter `:free` |
| Landing, reseal, merge, screenshots, answers | the Claude orchestrator | Claude, short turns |
| New Claude subagents | nobody, unless 2 free-tier attempts failed on the same task | -- |

1. Remaining work becomes **4-heading issues** (Boundary / User Scenarios / Requirements / Success
   Criteria) under the epics in AGENTS.md "Swarm focus". One issue = one lane = one letter.
2. The orchestrator never re-reads large files to answer a status question: the pulse answers it.
3. A Claude turn that only waits is a wasted turn: use notifications, never polling loops.

## Pulse (no LLM)

`pulse.sh` (this folder) collects, without any model call:

- bees running / queued (GitHub Actions runs on gHashTag/t27), PRs opened and merged in 24 h;
- swarm issues waiting;
- token stock: count of Keychain items per provider under service `ai.t27.os` -- **presence only, the
  script never reads a key value** (`security find-generic-password` without `-w`);
- agent processes on this Mac, free disk.

It writes `measures.json`; the orchestrator uploads it to the Mission Control dashboard dataset
`measures` (artifact LvTojtwf5fatVpxwkSzunZ). The first rows are live, labelled `live HH:MM`.

## Hourly watch (cron, minute 17)

1. Run `pulse.sh` in a terminal tab; upload; update the dataset.
2. **Idle rule:** queued == 0 and swarm issues waiting > 0 -> capacity is idle: start one free-tier lane
   on the oldest waiting issue (T27 OS pane through the key master), never a Claude subagent.
3. **Keys rule:** z.ai keys < 3 or OpenRouter == 0 -> request the owner for keys (exact value to
   provide, once), meanwhile route all lanes to the remaining tier.
4. **Disk rule:** free disk < 8 GB -> start no build (cargo, t27b app, nextpnr); remove own merged
   worktrees first (skill self-cleanup).
5. End the tick with one line: state, numbers, action taken, `Requests out:`.

## Traps

- A cron fire is itself a Claude turn: keep the tick to the collector + at most one action.
- `gh` search with `#` inside quotes can undercount; count swarm issues by epic number in the body.
- The sandbox blocks the Keychain and `gh` writes: run the collector in a terminal tab.

Port target: this collector is plumbing; its rules (idle, keys, disk thresholds) belong in
`specs/t27os/evolve.t27` as tested fns -- see the swarm issues under gHashTag/t27#11275.
