# T27 OS key orchestration, compared with Free Claude Code

Issue: gHashTag/t27#11224. Code: `specs/t27os/keys.t27` (decisions and data), the host side in
`keys_host.t27`, `pty_server.t27` and `glass_window.t27`. Stage 2 (a local router) is
`specs/t27os/router.t27`, decisions only. No key value appears in this file or in any spec; every test uses
fake values.

## Why

A pane that starts Claude Code inherited whatever provider the parent shell pointed at, and that provider
answered `429 Insufficient balance` or `Your organization has disabled Claude subscription access`. The pane
was dead until a person edited the environment by hand. The owner asked (2026-10-20) for key orchestration
like the tool Free Claude Code.

## The prior art, in our words

Free Claude Code (https://github.com/Alishahryar1/free-claude-code, page read 2026-10-20) is a local proxy
that makes Claude Code talk to many other providers. Its configuration, as the page describes it:

- one default model, plus an optional override for each tier (opus, sonnet, haiku, and a fourth tier); a tier
  left empty falls back to the default;
- a model is written as `provider/model`;
- each provider has a list of numbered keys that can be reordered; key 1 is used first, and a fallback entry
  may name a key number;
- an ordered fallback list: when a request still fails after its retries, the next entry is tried, so one
  request may spend the quota of more than one provider;
- an optional bearer token that protects the local proxy;
- an admin page where keys, models and fallbacks are set.

## The constraint that shapes our version

T27 OS cannot run a local HTTP proxy yet: t27b has no listening INET socket and no streamed HTTPS client
(issue #10902), so `router.t27` stops at decisions. But several providers serve the Anthropic protocol
themselves. Claude Code can talk to them directly with `ANTHROPIC_BASE_URL`, `ANTHROPIC_AUTH_TOKEN` and the
tier model variables. So the orchestration is done per pane, not per request:

1. At pane spawn, choose a profile (a provider, its base URL, its key variable, its tier models) and set that
   environment, and nothing else (`pick_profile`, `profile_var_*`, `claude_strips`).
2. When the pane shows a provider error, mark that provider bad for this pane and respawn the pane, which
   resumes its Claude Code session, on the next profile or the next key (`classify_line`, `fo_action`).

No proxy is involved.

## Comparison

| Free Claude Code idea | In T27 OS | Spec function | What stays blocked |
|---|---|---|---|
| One default model | Yes, per profile | `MODELS_DEFAULT`, `tier_model(pr, TI_DEFAULT)`, sets `ANTHROPIC_MODEL` | Nothing |
| Per-tier overrides (opus, sonnet, haiku) | Yes, as data, one column per tier; an empty field falls back to the default; the small-fast variable follows haiku | `MODELS_OPUS/SONNET/HAIKU`, `tier_model`, `profile_var_name/text` | A fourth tier (the one the OpenRouter and Kimi pages call fable) is not mapped; changing a tier is a data edit, not a screen |
| `provider/model` syntax | A profile is a provider; its models sit in the tier table | `PROFILE_NAMES`, `PROFILE_PROVIDER` | The combined one-line syntax; it exists in Free Claude Code because its proxy picks the upstream per request |
| Many providers | Four profiles today: z.ai, Anthropic direct, OpenRouter, Ollama cloud, over the nine key rows | `PROFILE_PROVIDER`, `PROFILE_BASE_URLS`, `PROFILE_KEY_VARS` | A provider that does not speak the Anthropic protocol needs the local router and `specs/provider/transform.t27` (#10902). DeepSeek, Moonshot and MiniMax speak it but need key rows of their own (see below) |
| Numbered keys, key 1 first | Naming and choice are decided: key 1 keeps the existing Keychain item, keys 2 and 3 add `#2`, `#3`; least-recently-used rotation with a rest time | `keychain_label_n`, `MAX_KEYS`, `rotate_pick`, `rotate_pick_except` | The KEYS tab still shows one row per provider and the host reads only key 1; storing and listing keys 2 and 3 is host work |
| A fallback entry names a key number | Decided: a throttled key (429) with a spare key swaps the key and keeps the profile | `fo_action` = `FA_RESPAWN_KEY`, `fo_target` | The host does not pass the key number to the respawn yet |
| Ordered fallback list after retries | Yes, per pane: the profile list is walked forward only | `next_profile`, `fo_action` = `FA_RESPAWN_NEXT`, `MAX_FAILOVERS` | Per request, inside one conversation turn: needs the local router. A respawn does not replay the request that failed; the person continues in the resumed session |
| Retries before the fallback | Not ours: Claude Code retries by itself, and the failover starts when the error text reaches the screen | `classify_line` | Nothing to build |
| Reasoning effort per route | No | none | Out of scope; not set by any profile |
| Bearer token for the local proxy | Not needed, there is no proxy | none | Needed with the router (it binds 127.0.0.1, `ROUTER_BIND`) |
| Admin page for keys | Yes: the KEYS tab, Keychain storage, masked display, a check button, a circuit breaker | `ky_*`, `mask_key`, `status_after_probe`, `circuit_fails`, `bad_mask` | The signed-in account keys on app.t27.ai cannot be spent from a pane: no proxy route exists server-side (see the MAP block in `keys.t27`) |
| Local models (LM Studio, llama.cpp, Ollama) | Ollama cloud only | profile `ollama-cloud` | Local Ollama needs no key, so the stored-keys mask cannot say it is available; a host fact (is the server up) must join the mask first |
| OAuth sign-in for some providers | No | none | Out of scope |

## The pane failover, step by step

All functions are pure; the host reads the screen and respawns.

1. `classify_line(line)` sorts one screen line into a class by plain byte matching: `disabled Claude
   subscription` is subscription disabled; `Insufficient balance` is out of balance (it wins over 429,
   because a provider sends both for an empty account); a standalone `429` on a line that also says
   "error" is rate limited; a standalone `401` or `403` there is login rejected; an `API Error` line that
   matches nothing is other; anything else is no error. A number glued to other digits does not count, and
   neither does "Edited 429 lines".
2. `fo_action(class, pr, stored, bad, done, alt)` answers keep, respawn on the next profile, respawn on the
   next key of the same profile, or give up. `pr` is the profile in use, or none for a pane that started
   without a key (where the "disabled Claude subscription access" line comes from). `alt` says another key of
   the provider is usable (`rotate_pick_except`, the failed key skipped).
3. `fo_bad(class, pr, bad)` is the new bad mask for this pane; `fo_target(...)` is the profile to respawn on.
4. `fo_why(...)` writes the line for the KEYS tab: the profile, the failure and the move, never a key value.

It cannot loop. The chain is walked forward only (`next_profile` returns a profile after the one that failed),
so each profile is tried at most once per pane. A key swap is counted in `done`, and `MAX_FAILOVERS` ends the
count even if the host always reports a spare key. After every profile has failed once, the answer is give up
and the pane stays where it is. A line that is no recognised failure keeps the pane as it is.

## Providers read on 2026-10-20

Only providers whose own pages document an Anthropic-protocol base URL are used. Model names are copied from
the pages, not guessed.

| Provider | Base URL | Key variable | Tier models shown | Page | In T27 OS |
|---|---|---|---|---|---|
| z.ai | `https://api.z.ai/api/anthropic` | `ANTHROPIC_AUTH_TOKEN` | A flash model for every tier is what the stored key answers (owner, 2026-10-19); the page lists larger ones for opus and sonnet | https://docs.z.ai/devpack/tool/claude | Profile `zai`, first in the chain |
| Anthropic direct | none (Claude Code's own) | `ANTHROPIC_API_KEY` | none set | Claude Code itself | Profile `anthropic-api`, second |
| OpenRouter | `https://openrouter.ai/api` | `ANTHROPIC_AUTH_TOKEN`, with `ANTHROPIC_API_KEY` set empty | opus, sonnet, haiku: the `~anthropic/claude-*-latest` aliases (`[1m]` on opus and sonnet) | https://openrouter.ai/docs/cookbook/coding-agents/claude-code-integration.md | Profile `openrouter`, third. The page warns that Claude Code is only guaranteed with Anthropic models there |
| Ollama cloud | `https://ollama.com` | `ANTHROPIC_AUTH_TOKEN`, with `ANTHROPIC_API_KEY` set empty | one cloud model, named on the page, used for every tier | https://docs.ollama.com/integrations/claude-code | Profile `ollama-cloud`, fourth |
| DeepSeek | `https://api.deepseek.com/anthropic` | `ANTHROPIC_AUTH_TOKEN` | one flash model for the default, opus and sonnet (with `[1m]`), the same without it for haiku and subagents | https://api-docs.deepseek.com/guides/coding_agents | Documented, no profile yet |
| Moonshot (Kimi) | `https://api.moonshot.ai/anthropic` | `ANTHROPIC_AUTH_TOKEN` | one model for default, opus and sonnet (with `[1m]`), a code model for haiku | https://platform.kimi.ai/docs/guide/claude-code-kimi | Documented, no profile yet |
| MiniMax | `https://api.minimax.io/anthropic` | `ANTHROPIC_AUTH_TOKEN` | one flash model for every tier (with `[1m]`) | https://platform.minimax.io/docs/token-plan/claude-code.md | Documented, no profile yet |
| Ollama, local | `http://localhost:11434` | token set to a fixed word, no real key | a local model, named on the page | https://docs.ollama.com/integrations/claude-code | Left out, see the comparison |

The three "no profile yet" rows need a provider row of their own in `keys.t27` (nine rows today). The KEYS
window sizes its arrays by nine and the PTY server sizes its key table by nine, so the new rows are a
change in those files too; the profile data is then one more field in each table in `keys.t27`.

## Defaults chosen, and what is left

- Order: z.ai first, as before; the others follow in the order of the table. A profile is used only once its
  provider's key is stored and has not failed its check. Nothing is enabled by this change on a Mac with no
  extra key stored.
- A Mac that already holds an OpenRouter or Ollama key (stored for another agent) now also gives Claude panes
  that profile after z.ai and the Anthropic API key.
- Host work, not decisions: call `classify_line` on the pane's error row only, never on the transcript body,
  because a diff that shows these phrases would otherwise trigger a failover; keep the per-pane `bad` mask and
  `done` count; pass the key number to the respawn; the window's start-up check loop covers the first two
  profiles only and should run to `PROFILE_COUNT`.
- Blocked: the local router (per-request fallback, translation for non-Anthropic providers, a bearer token for
  the proxy) until t27b has a listening socket and a streamed HTTPS client (#10902).
