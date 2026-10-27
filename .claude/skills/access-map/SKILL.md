---
id: access-map
name: Access map
description: Which model and API accesses an agent can reach -- what the T27 OS key master wires (providers, profiles, models, failure codes), what is known but NOT wired (NVIDIA NIM, DeepSeek, Moonshot, MiniMax, direct Gemini or OpenAI), how each one fails, and how to add one without ever handling a key value. Use before choosing a provider or model, when a pane prints 401/429/1113/1305, when the owner says "use every key we have", "nvidia", "what accesses do we have", or before telling the owner a provider is or is not available.
argument-hint: "[none | add <provider>]"
---

# Access map (owner request, 2026-10-25)

<!-- only-t27-rule -->
> **Only t27: everything is written in `.t27`.** (Owner hard rule 2026-10-05; budget #7371; restated in every skill per #7588.)
> - New logic starts as a `.t27` spec with `test` / `invariant` blocks. It reaches Rust, C, Zig, JS or Python only through `t27c gen-*`, and generated files are never hand-edited (L2).
> - Hand-written code in any other language is denied by the hooks and by CI: `.rs .py .ts .js .sh .zig .c .go .v .yml .toml`, Dockerfile, Makefile; the full list is in `specs/policy/own_language.t27`.
> - Existing foreign code is debt that only shrinks. Delete it, or replace it with a spec and its generated output.
> - Thin hand-written glue is capped at 40 added lines per file and 80 per PR. No label lifts the cap. Never split a PR to slip under it.
> - What gen cannot express yet is a compiler defect. File it on #5980 with a minimal `.t27` repro instead of writing the code by hand.
> - This rule overrides any step below that says to write a script or code in another language. The full text is in `AGENTS.md`, "Only t27".

The owner wants every access we have used to the maximum, and no agent breaking on one that is not there. This
page says what the specs wire, what they do not, and how each fails. It holds names and dated facts only: never a
key value, and the number of keys the owner has stored is not in the repo (the KEYS tab shows it). The tables are
in the specs, so this page points at them and does not copy them: `specs/t27os/keys.t27` (the key master),
`specs/queen/api_fleet.t27` (the dated route matrix), `specs/automation/queen-contributor-keys.t27` (the Queen's
server-side keys). The cost rules are skill `work-cheap`; the lane order is `worktree-lane`.

## What a pane can reach today (the key master, `keys.t27`)

- **Store:** macOS Keychain, service `ai.t27.os`, one account per key (`zai`, `zai#2` and so on). The window's
  KEYS tab adds a key and has the check button (`keys_probe.t27`). An agent never opens the Keychain.
- **Provider rows (9):** `PROVIDER_NAMES` = anthropic, openai, gemini, openrouter, zai, ollama, github, cursor,
  gonka. A row can hold a key without being a profile.
- **Profiles (4), the only ones a Claude Code pane can talk to:** `zai`, `anthropic-api`, `openrouter`,
  `ollama-cloud` (`PROFILE_NAMES`). The chain walks z.ai free first, then OpenRouter `:free`, then the paid ones.
  Each documents an Anthropic-protocol base URL on its own pages.
- **Models per profile:** `MODELS_DEFAULT` and `MODELS_SECOND` (and `MODELS_THIRD`, empty). Read them there; they
  change with each measurement.
- **Failover:** on 429, 1113, 1305 or 401/403 a pane rotates the key, then the model, then the profile
  (`fo_action`). If every key rests, the pane waits. Do not rotate by hand.

## Known but NOT wired (do not assume it works)

| Access | Why it is not in the chain | Status |
|---|---|---|
| **NVIDIA NIM**, direct (`integrate.api.nvidia.com/v1`) | Provider row `nvidia` added 2026-10-26 (keys.t27, 10 providers); 4 nvapi keys stored in the Keychain. Measured the same day: GET /v1/models 200 (85 models), chat-completions on three nemotron ids 403 "Authorization failed" -- the keys list the catalogue but their plan does not cover inference. Still NO profile: it speaks chat-completions, not the Anthropic protocol; a pane reaches NVIDIA through OpenRouter `nvidia/...:free` until the stage-2 router. |
| **NVIDIA models through OpenRouter** | Wired: the second OpenRouter model is `nvidia/nemotron-3.5-super-vl:free` | Measured 2026-10-20: HTTP 200, it thinks first, so `max_tokens` 16 gives empty text; the key master applies a floor (`model_min_tokens`). `nvidia/nemotron-3.5-lightning:free` returned no parsable body: dropped |
| DeepSeek, Moonshot (Kimi), MiniMax | Documented Anthropic-protocol pages exist, but each needs a provider row of its own (PROVIDER_COUNT 9 to 12, and the window and server size their tables by nine) | no profile; keys may exist on disk, which does not make them reachable |
| Direct Gemini, OpenAI, Cursor, Gonka, GitHub | Provider rows only; no profile | not used by a pane |
| `free-claude-code/*` routes | Third-party router | UNPROBED |
| Queen server-side keys (contributor keys) | Per the `keys.t27` header, no route lets a client spend a stored key | work on them is the Queen's and the contributor's (consent, `PROVIDER_FEE`); never move a key |

## How accesses fail (measured, dated; re-measure before leaning on a number)

- **429 / code 1302:** rate limit. At concurrency 5 most z.ai runs died: one call at a time per key.
- **1113:** "insufficient balance or no resource package": a paid model id on a free key. Not a dead key.
- **1305 / HTTP 529:** overload. The owner's finding of 2026-10-25 is that it is service noise on
  `glm-4.7-flash`, so it does not say the key lacks the model (the 2-of-7 reading in `keys.t27` is older).
- **401 / 403:** a fact about the route (model x plan) first, the key second. On 2026-10-25 five keys answered 401
  on one model and 200 on another in the same hour (`api_fleet.t27`). Re-probe the matrix before replacing keys.
- **A listed price of $0 is not an entitlement:** `glm-5.3-highspeed` was listed free and refused by the plan the
  same day.

## Rules for an agent

1. **Ask the key master, not the disk.** What is reachable is what the specs wire plus the keys the owner stored.
   Do not read `.env` files or the Keychain to find out; ask for the KEYS-tab check, or read the pane's status
   line (`rotation_status.t27`) and `T27OS_CTL=list` (high nibble of the flags is the pane's key number).
2. **Spread the load.** Free capacity is per key: let a long job take a pane on another key rather than queue
   behind a resting one, keep one call at a time per key, and put bounded work on the free tier first
   (`work-cheap` rule 1).
3. **Never name an unwired provider as available.** If a task needs NVIDIA direct, DeepSeek or the like, say
   "not wired, issue #<N>" and use the nearest wired route (OpenRouter `nvidia/...:free`, or z.ai).
4. **Do not point Claude Code at a chat-completions URL.** It sends the Anthropic protocol; a profile needs the
   provider's documented Anthropic-protocol base URL, or the local router.
5. **A key value never appears** in a brief, an issue, a PR, a comment or a transcript (`secret-hygiene`).

## Adding an access (the owner stores the key, an agent writes the spec)

1. The owner adds the key in the KEYS tab (Keychain account under `ai.t27.os`) and presses the check button.
   Nobody pastes a value into chat.
2. An agent opens an issue with the four headings (`issue-shape`), then edits the spec: a provider row, a
   profile with its documented base URL, the `MODELS_*` rows, and `assert` lines that pin them. Specs with
   `@extern` go through `t27b test`; the seals are re-saved from a master-built `t27c`.
3. Each model is measured with one tiny call per key by the KEYS-tab check, dated in the doc comment, and the
   table is ordered by that measurement, not by the provider's list.
4. Update this page only if a rule changed; the tables stay in the specs.

## Traps

- "We have a key" and "an agent can use it" are different facts: an NVIDIA key in a local file reaches nothing
  until a provider row, a profile or the router exists.
- A model that answers once is not verified: look for a parsable body and a floor on `max_tokens` (the thinking
  models answer empty text at 16).
- A copy of the matrix in a brief goes stale in a week. Point at the spec, and date any number you quote.
- Free is not unlimited: ask which limit applies (per key, per day) before building a loop on it.
