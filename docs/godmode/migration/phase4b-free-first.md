# GODMODE FINAL ORDER, Phase 4b: free-first routing

Owner order, 2026-11-08: subagents spend free limits first, paid Claude is the
fallback, and when the free limit runs out the agent tells the owner in writing.
This block sits after Phase 4 in the FINAL ORDER and inherits its gate. The
owner approved Phase 4 / 4b on 2026-11-08. Status: `APPROVED` (spec work),
`BLOCKED` (on #13204, below), `DO_NOT_MIGRATE` (keys: no key value is read,
moved or written by this work).

The law stays in the existing specs (no second spec). **The free-first walk
already has a home: `specs/t27os/lagger.t27` (#13204)** -- cost-ordered path
table, free paths first, paid only with the owner's grant, `lg_free_exhausted`,
and a fail-closed 503 naming every tried path. It is commit 9e9029cbdf on the
local branch `feat/lagger-cost-free-first` (worktree `lagger-cost`, another
session's, 2026-11-06), not pushed, no PR. Task roles stay in
`specs/claw/model_route.t27`. This file is the order and the gap map, not a
copy of the law.

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
   through our own router (lagger.t27 paths, `route()` with `TIER_PANE` in
   model_route.t27, the ladder and the key master), not through the CLI flag.
2. **NVIDIA NIM is not a usable free tier today.** The `nvidia` provider row and
   four nvapi keys exist; on 2026-10-26 `/v1/models` answered 200 and
   chat-completions answered 403 "Authorization failed" on three nemotron ids
   (`ladder.t27` test `the_nvidia_403_leaves_its_rung_unwired`; skill
   `access-map`). NVIDIA models are reachable only through OpenRouter `:free`.
3. **DeepSeek has a provider row and no profile**; the account is unfunded
   (t27#13034), and its ladder rung was removed 2026-11-01.
4. **z.ai is five rotating keys** in `ladder.t27`, not four. #13204 measured
   them on 2026-11-05: key 1 at its weekly limit (429/1310, reset 2026-11-08
   14:35:31), keys 2-5 with no balance (429/1113). The only free path that
   answered that day was the local LM Studio model.
5. **Rotating several free accounts of one provider** to multiply per-account
   limits may break that provider's terms. Each provider's terms are checked
   before a second account joins a rotation.

---

## 3. Gap map against the existing law

| Item | Exists | Missing | Home |
|---|---|---|---|
| (a) free-first order | lagger.t27 (#13204, unpushed): path table, `lg_next_path`, paid grant; model_route.t27 `route()` tiers OPUS/SONNET/PANE, escalate after 2 and 4 failures, owner flag to Opus | lagger.t27 not on master; "Opus for architecture" exists only as owner flag or 4 failures | lagger.t27, model_route.t27 |
| (b) task classes | 9 roles in model_route.t27, table in skill `model-routing` | scan/trace/diff/research/summary/classify collapse into search/docs; no architecture role; no critical flag on fix | model_route.t27 (roles + a critical flag on `route()`) |
| (c) all-free exhausted | lagger.t27 `lg_free_exhausted`, fail-closed 503 naming tried paths; per-key cooldown, `FC_RATE_LIMITED`, `FA_GIVE_UP` in keys.t27 | no bulk-vs-critical split; no "paused until owner answers" state; no owner-notice text with the three options; no `free_tier_status` enum | lagger.t27 |
| (d) run budget guard | `fits_budget()` (agent-provider-chain.t27, token reserve), a context-window guard, `cost_usd` in tri/agent/handoff.t27 | no run cap on calls/tokens/seconds/dollars; no stop-and-notify | lagger.t27, a pure `budget_status()` |
| (e) cost ledger row | `cost_usd` in handoff.t27; `specs/claw/ledger_union.t27` (not yet read) | no row with the fields above | GODMODE Ring 6 (#13266) `specs/queen/cost_measure.t27`; Ring 6 FR-004 keeps model_route.t27 and ladder.t27 untouched there |

Every touched spec needs a seal re-save from a master-built t27c
(`t27os_T27osLagger.json`, `claw_ClawModelRoute.json`).

---

## 4. Next step and requests

- **Owner approved Phase 4 / 4b on 2026-11-08 ("approv").**
- **Blocker:** #13204 (lagger.t27) is another session's unpushed work. This
  session does not push or take over that branch without the owner's word.
- **Then, one lane on top of lagger.t27:** owner-notice text and the
  bulk-paused state in lagger.t27, the critical classes that bypass the pause,
  `budget_status()`; task classes and the critical flag in model_route.t27;
  re-seal both. Cost ledger and measurement stay with GODMODE Ring 6
  (#13266, `cost_measure.t27`).
- **Owner:** NVIDIA inference access (the 403) is a plan question on the NVIDIA
  account; DeepSeek needs funding. Default: free bulk stays on z.ai and
  OpenRouter `:free`, and both stay unwired here until then.
