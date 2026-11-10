# DSH startup (cloud agent)

# GOAL

Run free-model-backed tasks from a Cursor Cloud Agent or other remote session. **`127.0.0.1` and `~/.dsh/` on the owner's Mac are not reachable** — use external APIs and platform env only.

# PRIORITY

1. z.ai free (`DSH_ZAI_KEY`)
2. NVIDIA NIM (`NVIDIA_API_KEY_1`)
3. TeamoRouter (`TEAMO_API_KEY`)
4. OpenRouter `:free` (`OPENROUTER_API_KEY`)
5. DeepSeek API — **UNFUNDED (402)** — do not use

# KEY HANDLING

- Bind secrets in the Cloud Agent **environment** (Cursor secrets UI / `bind-app-secrets`), not in chat, brief, log, commit, or PR.
- On the Mac, current values live in `~/.dsh/.credentials.yaml` — **only the owner or a Mac session** may read them; cloud agents must not ask for pasted keys.
- Never print, copy, screenshot, or commit a secret (see `.claude/skills/secret-hygiene/SKILL.md`).

# USAGE

- Default to z.ai free for bounded tasks; escalate provider only when the prior tier is unavailable or fails validation.
- Judge free-model output with a **machine gate** (compiler, test, seal), not self-report.
- No silent paid fallback; no private repo source to an unapproved external model without policy.

# SAFETY

- No merge, publish, or window shell unless the task explicitly grants it.
- No tight polling — use `cursor-subscriptions` or a named timer (see subscribe skill).
- Merge only via `.claude/skills/merge-queue/SKILL.md` → `merge-verdict` spec (seven conditions, `M\t` line).

# NEXT REPORT

```text
PROVIDER:
MODEL:
ROUTE:
KEY_SOURCE: env|blocked|not_wired
DATA_ROUTE:
BLOCKERS:
```
