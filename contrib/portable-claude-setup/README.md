# Portable Claude Code profile

Dotfiles-style bundle for [Claude Code](https://www.anthropic.com/claude-code): **secrets live only in `~/.claude/.env`** (never committed); `settings.json` is generated/updated by a script. This GitHub repo contains **only this folder**, not your full `~/.claude` tree.

## Contents

| Path | Purpose |
|------|---------|
| `templates/settings.template.json` | Settings skeleton **without** tokens; paths are placeholders |
| `env.example` | Variables to copy into `~/.claude/.env` |
| `scripts/sync-settings-from-env.sh` | Merges `.env` into `settings.json` (`env.*` and optional MCP commands) |
| `scripts/apply-anthropic-token-from-env.sh` | Legacy name; calls `sync-settings-from-env.sh` |

## Clone from GitHub

```bash
git clone https://github.com/gHashTag/portable-claude-setup.git
cd portable-claude-setup
```

## New machine

1. Install `jq`, `bash`, `git`. Optionally [GitHub CLI](https://cli.github.com/) (`gh auth login`).

2. Create local Claude Code files:

   ```bash
   cp templates/settings.template.json ~/.claude/settings.json
   cp env.example ~/.claude/.env
   chmod 600 ~/.claude/.env
   ```

   Edit `~/.claude/.env` (tokens and real paths). Adjust `settings.json` for hooks, MCP, plugins if needed.

3. Inject secrets into `settings.json`:

   ```bash
   bash scripts/sync-settings-from-env.sh ~/.claude/.env ~/.claude/settings.json
   ```

4. After changing keys in `.env`, run the same command again (or add a shell alias).

## Multi-account Z.AI / `ZAI_KEY_N`

Set `ZAI_USE=2` and `ZAI_KEY_2=...`; otherwise the first non-empty `ZAI_KEY_1`, `ZAI_KEY_2`, … (numeric order) is used.

## Key rotation

There is none here, on purpose. Owner, 2026-10-18: every API key lives in one place, the Queen's
store, and is shared out to everyone who uses it (`specs/queen/key_vault.t27` in gHashTag/t27,
#10564). A numbered pool of private keys rotated on one machine is a second home for keys that
nobody else can see or share, so the rotation scripts that lived here were deleted. A consumer
that needs a model leases from the Queen (for the reviewer bees: `BEE_QUEEN_LEASE_URL` and
`BEE_QUEEN_LEASE_TOKEN`); keys you hold go into her store through the contributor-key flow.

## Security

- **Do not push** your entire `~/.claude` directory: it may include `history.jsonl`, plugin caches, and tracked `settings.json` with secrets.
- This repo should contain **only** templates and scripts—no `.env` or real tokens.

