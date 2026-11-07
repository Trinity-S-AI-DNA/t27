# GODMODE FINAL ORDER, Phase 4b: free-first routing

Owner order, 2026-11-08: subagents spend free limits first, paid Claude is the
fallback, and when the free limit runs out the agent tells the owner in writing.
This block sits after Phase 4 in the FINAL ORDER and inherits its gate:
**policy is recorded here; no key, credential or external route is wired until
the owner approves Phase 4.** Status: `DO_NOT_MIGRATE` (wiring), `RECORDED` (policy).

The law stays in the existing specs (no second spec): tiers and roles in
`specs/claw/model_route.t27`, rungs in `specs/t27os/ladder.t27`, keys and
failure classes in `specs/t27os/keys.t27`. This file is the order and the gap
map, not a copy of the law.

---

## 1. The order as given

Model hierarchy:

| Rank | Tier | Use |
|---|---|---|
| 1 | FREE: NVIDIA NIM / z.ai GLM, round-robin over the free keys | bulk work |
| 2 | FREE: DeepSeek MCP | read-heavy, bounded |
| 3 | PAID: Claude Sonnet | planning, review, critical fixes |
| 4 | PAID: Claude Opus | architecture decisions, final review only |

Task-class routing:

| Task class | Primary | Fallback |
|---|---|---|
| Repo scan, call-path tracing, large diff first pass | NVIDIA NIM | DeepSeek MCP |
| Web research, summarization, draft docs, bulk classification | NVIDIA NIM | z.ai GLM |
| Architecture decision | Opus | Sonnet |
| Critical fix | Sonnet | Opus |
| Final review | Sonnet | Opus |

Owner notice: when every free key is rate-limited or refused (429/403/401), the
agent stops non-critical bulk work and writes the owner `FREE LIMIT EXHAUSTED`
with three options (a: slow down, fewer lanes and a longer interval; b: add
keys or a paid tier; c: switch bulk to paid Claude) and the status (queued,
done, budget spent). Bulk does not resume until the owner answers. Critical
classes (architecture, critical fix, final review) stay on paid Claude and are
never blocked by the notice.

Budget guard: each run has a cap on calls, tokens, seconds and dollars;
reaching any cap stops the run and notifies the owner.

Cost ledger row: `run_id, timestamp, task_class, route, model, key_alias,
input_tokens, output_tokens, cache_tokens, estimated_cost_usd, budget_status,
result_status, evidence_paths, free_tier_status (OK | RATE_LIMITED | EXHAUSTED),
owner_notified`. `key_alias` is a name, never a key value.

---

## 2. Corrections, measured or read from the docs (2026-11-08)

1. **`--fallback-model` cannot carry this chain.** Claude Code's flag takes a
   comma-separated list tried in order, capped at three models, and applies it to
   subagents (since v2.1.247; the switch is shown only when it happens). But it
   fires only on overload or non-retryable server errors; **rate-limit (429),
   auth and billing errors never trigger it**, and every entry is a model on the
   one configured Anthropic endpoint. NIM (chat-completions) and an MCP server
   are not models it can switch to. Sources: code.claude.com docs `cli-reference`,
   `model-config#fallback-model-chains`, `sub-agents`. Free-first therefore runs
   through our own router (`route()` with `TIER_PANE` in model_route.t27, the
   ladder and the key master), not through the CLI flag.
2. **NVIDIA NIM is not a usable free tier today.** The `nvidia` provider row and
   four nvapi keys exist; on 2026-10-26 `/v1/models` answered 200 and
   chat-completions answered 403 "Authorization failed" on three nemotron ids
   (`ladder.t27` test `the_nvidia_403_leaves_its_rung_unwired`; skill
   `access-map`). NVIDIA models are reachable only through OpenRouter `:free`.
3. **DeepSeek has a provider row and no profile**; the account is unfunded
   (t27#13034), and its ladder rung was removed 2026-11-01.
4. **z.ai is five rotating keys** in `ladder.t27`, not four.
5. **Rotating several free accounts of one provider** to multiply per-account
   limits may break that provider's terms. Each provider's terms are checked
   before a second account joins a rotation.

---

## 3. Gap map against the existing law

| Item | Exists | Missing | Home |
|---|---|---|---|
| (a) free-first order | `route()`, `base_tier()`, `is_waste()`; tiers OPUS/SONNET/PANE; escalate after 2 and 4 failures; owner flag goes to Opus; ladder rungs NVIDIA/ZAI/OLLAMA/OPENROUTER with `next_rung`, `ladder_exhausted` | no DeepSeek rung; NIM unwired; "Opus for architecture" exists only as owner flag or 4 failures | model_route.t27, ladder.t27 |
| (b) task classes | 9 roles in model_route.t27, table in skill `model-routing` | scan/trace/diff/research/summary/classify collapse into search/docs; no architecture role; no critical flag on fix | model_route.t27 (roles + a critical flag on `route()`) |
| (c) all-free exhausted | per-key cooldown, `FC_RATE_LIMITED`, failover actions, `FA_GIVE_UP`, `ladder_exhausted`, `names_refusal` | no all-free-exhausted predicate feeding routing; no bulk-vs-critical split; no "paused until owner answers" state; no owner-notice type; no `free_tier_status` enum | ladder.t27 (state), model_route.t27 (decision), keys.t27 (enum) |
| (d) run budget guard | `fits_budget()` (agent-provider-chain.t27, token reserve), a context-window guard, `cost_usd` in tri/agent/handoff.t27 | no run cap on calls/tokens/seconds/dollars; no stop-and-notify | model_route.t27, a pure `budget_status()` |
| (e) cost ledger row | `cost_usd` in handoff.t27; `specs/claw/ledger_union.t27` (not yet read) | no row with the fields above | read ledger_union.t27 first; else model_route.t27 |

Every touched spec needs a seal re-save from a master-built t27c
(`claw_ClawModelRoute.json`, `t27os_T27osKeys.json`).

---

## 4. Next step and requests

- **Spec work (after the owner approves Phase 4):** one issue, one lane: extend
  model_route.t27 with the task classes, the critical flag, `budget_status()`
  and the exhausted decision with its tests; add the exhausted state to
  ladder.t27 and the `free_tier_status` class to keys.t27; re-seal.
- **Owner:** approve Phase 4 wiring, or keep it held (default: held).
- **Owner:** NVIDIA inference access (the 403) is a plan question on the NVIDIA
  account; DeepSeek needs funding. Default: free bulk stays on z.ai and
  OpenRouter `:free`, and both stay unwired here until then.
