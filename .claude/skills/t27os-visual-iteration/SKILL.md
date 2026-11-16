---
id: t27os-visual-iteration
name: T27 OS visual iteration
description: Ship-checked loop for the T27 OS window (0.2.0) and the native foundation line (0.3.0). Use when changing glass_window pixels, chasing UI baselines, or when the owner asks for screenshot proof, computer-use verification, or native acceptance runs.
---

# T27 OS visual iteration

Companion to `t27os-dev-loop` (edit/test/build) and repo `t27os` skill. This skill names **which product line** you are proving and how to close the loop without confusing window semver with native foundation semver.

## Version lines (do not mix)

| Line | What it is | Proof |
|------|------------|--------|
| **0.2.0** | Shipped window: `glass_window.t27`, `T27OS.app`, release card in `release.t27` | `t27b test` + macho link + codesign + screenshot |
| **0.3.0** | Native foundation: `native_host.t27`, `native_accept.t27` | `t27b test specs/t27os/native_host.t27` and `native_accept.t27` only — **no window** |

Window header must not be claimed as 0.3.0 until plist evidence exists (`t27os-dev-loop` VERSION_GATE).

## 0.2.0 loop (window pixels)

1. **STALE_UI_BASE** — Before build/open: `git fetch origin` and compare `HEAD` to `origin/master`. If the worktree is behind, refresh or label `STALE_UI_BASE=YES` in the proof line. Do not screenshot against a stale glass baseline.
2. **Edit** — `specs/t27os/glass_window.t27` only for product pixels unless the task names another spec. Geometry SSOT: `win_layout.t27` when the task is shell migration (P3).
3. **Test** — `~/t27/target/release/t27b test specs/t27os/glass_window.t27 --quiet`
4. **macho → ld → codesign** — Same as `t27os-dev-loop` §3–4: `t27b build … --macho`, `target/t27os-local`, `T27OS_CHANNEL=dev`, ad hoc `codesign --force --sign -`.
5. **Screenshot** — Cursor **computer-use** on your dev bundle only (`ai.t27.os.dev`); dark and light; store under `target/shots/` or task evidence path. If two dev apps are open, front yours first (see `t27os-dev-loop` §5).

**PROOF_LINE:** `binary_path=… CFBundleVersion=… git_HEAD=… origin/master=… STALE_UI_BASE=YES|NO`

## 0.3.0 loop (native acceptance, no window)

Run guide: `docs/t27os/run-native-0.3.0.md` (lands with PR #14078 / Refs #13212; until then read from that branch or draft in the PR).

```bash
cargo build --release -p t27b
# Optional: LM Studio at 127.0.0.1:1234 with a model loaded for provider tests
./target/release/t27b test specs/t27os/native_host.t27
./target/release/t27b test specs/t27os/native_accept.t27
```

Do not block vf_guard or kit P0 work on 0.3.0 unless the owner asks.

## vf_guard / agent lanes

Before new UI kit or layout work: agent role must be refused on layout/render paths (`specs/policy/vf_write_gate.t27`, `lefthook` `vf-write-gate`, CI step in `spec-guards.yml`). See ONE SHOT P0 on branch `claude/vf-write-gate-p0`.

## Skills layout (repo vs home)

- **Repo:** `/Users/playra/t27/.claude/skills/` — project skills (this file, `t27os-dev-loop`, `tri-pipeline`, …).
- **Home:** `~/.claude/skills/cursor-worker` — only on the owner's Mac user home; **do not duplicate** into the repo. If a lane needs the worker, symlink or reference the home skill in prose; optional note in repo `t27os` skill only.

## References

- `docs/t27os/run-local.md` — Mach-O product table and PTY socket
- `docs/t27os/run-native-0.3.0.md` — native 0.3.0 acceptance (when merged)
- `specs/t27f/vf_guard.t27` — layer table and `gd_may_write`
- `specs/policy/vf_write_gate.t27` — hook policy