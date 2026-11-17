---
name: vf-write-gate-verify
description: >-
  Verify VF write gate (#14083): lefthook literal roles, vf_write_gate 13/0,
  C smoke via own-language.yml. Use for vf-p0-ci-fix bundle verify; no push
  without owner phrase; never tri-ci-fix force-push on this lane.
---

# vf-write-gate-verify

Verified 2026-11-17 on playra-mac (verifier lane VF_GATE_P0). Proof over narration.

**Ledger (in-repo):** `docs/reports/2026-11-17-vf-14083-review-package.md` (OWNER_DECISION_PACKET, GOAL_REMAINING, C smoke section B). Do not treat `/tmp/vf_*` copies as SSOT.

## Agent gates (mandatory)

- **No git push** and **no bundle commit** unless the owner gives an explicit phrase (coordinator default: **commit push lefthook #14083**). `AWAITING_PUSH_APPROVAL` until then.
- **Do not run** home skill `tri-ci-fix` on this lane (its flow amends and **force-pushes**). Hygiene reds (spec-guards 201, coverage, etc.) are owner/coordinator batch after bundle lands, not agent force-push.

## Execution context

- **Worker (local process):** `cursor-agent … worker start --name playra-mac --worker-dir /Users/playra/t27` (pid observed via `ps`, not cursor.com UI).
- **Agent CWD (this verifier):** `/Users/playra/t27` (main clone); branch `vf-p0-ci-fix`, HEAD `912b81e4a33a240c7f896fbaaf6433f671749dc3` (PR #14083 head at verify).
- **Worktree:** `~/t27/.claude/worktrees/vf-p0-ci-fix` may be **absent**; run verify on the main clone checked out to the same branch (or `git show vf-p0-ci-fix:…`).
- **Legacy lane:** `claude/vf-write-gate-p0` / `~/t27/.claude/worktrees/vf-write-gate-p0` may also be gone — do not assume a worktree path.

## Policy SSOT (branch `vf-p0-ci-fix` @ `912b81e4a3`)

| Piece | Location |
|---|---|
| Diff gate | `specs/policy/vf_write_gate.t27` — `ROLE_LANDING` (25–28), `parse_role` (48–63), `min_role_for_diff` (93–131), `effective_role` (134–138), `check_vf_writes` (142–195) |
| Bootstrap paths | `specs/t27f/vf_guard.t27` — `GATE_BOOTSTRAP` (107–111), `gd_may_write` bootstrap branch (173–180) |
| Local hooks | `lefthook.yml` — pre-commit: literal `printf 'agent\n--\n'` + `VF_WRITE_DIFF: --cached` (no `VF_WRITE_ROLE` env; bypass closed); pre-push: literal `printf 'landing\n--\n'` + `VF_WRITE_DIFF: origin/master...HEAD` (no `VF_WRITE_ROLE` env; bypass closed) |
| CI | `own-language.yml` + `spec-guards.yml` — hardcoded `printf 'landing\n--\n'` + diff (no env; landing always) |

## Landing role (CI)

See `vf_write_gate.t27` HEAD: `header_has_landing` (77–88), tests `landing_picks_engine_for_gate_bootstrap_pr` (252–256), `agent_allowed_gate_bootstrap_check` (273–276).

## Owner VF bundle (uncommitted 2026-11-17, no push)

Stage **only** these paths for #14083 (leave t27os version specs out):

1. `specs/policy/vf_write_gate.t27`
2. `gen/c/policy/vf_write_gate.c`
3. `lefthook.yml`
4. `.claude/skills/vf-write-gate-verify/SKILL.md` (optional; same commit or separate skills land)

Commit message draft and delta stats: report **OWNER_DECISION_PACKET** (not `/tmp`).

**Exclude:** `specs/t27os/native_accept.t27`, `native_version.t27`, `release.t27`, `statusbar.t27` (0.3.1 lane).

## Landing / owner fix (spec + gen/c, uncommitted)

Under `header_has_landing`, `effective_role` no longer returns bare `min_role_for_diff`: when `need == GR_OWNER`, return `GR_ENGINE` so gate-bootstrap files clear landing while `visual_factory_contract` and other owner paths stay denied (`landing_denies_contract_without_owner`).

After `MAC_HEAVY_OK=1 ./target/release/t27c gen-c specs/policy/vf_write_gate.t27`, **gen_hash** (sha256 of `gen/c/policy/vf_write_gate.c`):

- HEAD@`912b81e4`: `6a443cde22734e11fb6ff8ab326ef0553f1b08cbcd0f415b02bdafeaf7f67135`
- Working tree: `fe199d5b84a0e6bb7abf8f6e97b641b1d96edd7f8e1438122b0d3bf01763b39f`

Commit **spec + gen/c together** (L2).

## Lefthook preflight (literal roles)

Before claiming R1 closed locally:

```text
grep -n VF_WRITE_ROLE lefthook.yml   # must exit 1 on working tree (zero matches)
git diff HEAD -- lefthook.yml          # pre-commit: printf 'agent\n--\n'; pre-push: printf 'landing\n--\n'
```

Remote PR HEAD@912b81e4 still has `${VF_WRITE_ROLE}` until the owner bundle push.

## test-report (2026-11-17, playra-mac)

```text
MAC_HEAVY_OK=1 ./target/release/t27c test-report specs/policy/vf_write_gate.t27
```

Expected: **13 / 0** (100%). Heavy runs require `MAC_HEAVY_OK=1` (owner hook `~/.claude/hooks/mac-no-heavy.py`).

## C smoke (CI one-liner; section B of in-repo report)

SSOT: `.github/workflows/own-language.yml` lines 45-47 (`printf` driver piped to `cc -w -I . -x c`; then `printf 'landing\n--\n'` + PR diff into the binary). Mirror in `spec-guards.yml` L184-185. Do not hand-write a second driver (`only-t27`).

Local replay (same semantics as CI; wrap with `MAC_HEAVY_OK=1` if the Mac heavy hook applies):

1. Build smoke binary from `gen/c/policy/vf_write_gate.c` using the **same** one-line `main` as the workflow step (see yaml L45).
2. Feed stdin: `landing\n--\n` + `M\tspecs/t27f/visual_factory_contract.t27\n`.

Compile: exit **0**. Refusal: stdout `specs/t27f/visual_factory_contract.t27: contract`, exit **1** (measured 2026-11-17 in the in-repo report section B).

## Local hooks (role wiring)

- **pre-commit:** `VF_WRITE_DIFF: --cached`; stdin header is **literal** `printf 'agent\n--\n'` (no `VF_WRITE_ROLE` in hook env — inherited-shell override **CLOSED**).
- **pre-push:** `VF_WRITE_DIFF: origin/master...HEAD`; stdin header is **literal** `printf 'landing\n--\n'` (no `VF_WRITE_ROLE` in hook env — inherited-shell override **CLOSED**).
- **CI:** both workflows hardcode `landing\n--\n` so PR diffs match the owner/landing merge gate, not the developer’s shell.
- **Hardening status:** bundle above is **local only** until owner push on #14083.

## HEAD `${VF_WRITE_ROLE}` vs working-tree literals (zsh bypass)

On **HEAD** (`git show HEAD:lefthook.yml`), both vf steps set `VF_WRITE_ROLE` in hook `env` and build stdin with `printf "${VF_WRITE_ROLE}\n--\n"` inside a `{ … }` block. Any **inherited** `VF_WRITE_ROLE` in the parent shell is expanded **before** lefthook runs the command, so `export VF_WRITE_ROLE=landing` before `git commit` can feed **landing** into the pre-commit gate while the hook still advertises `VF_WRITE_ROLE: agent` in YAML.

**Working tree (uncommitted):** literals only — pre-commit `printf 'agent\n--\n'`, pre-push `printf 'landing\n--\n'`; no `VF_WRITE_ROLE` env keys. Bypass **closed** locally; **not published** until `lefthook.yml` is committed and pushed on #14083.

**R5 (local vf spec tests):** **PROVEN** on working tree — `test-report` **13/0** after landing/owner `effective_role` + `gen-c` refresh (see above). Prior Zig harness miss was stale gen/c; fixed by regen, not lab-only.

## Auth boundary (static audit 2026-11-17)

| Layer | Status | Evidence |
|---|---|---|
| **Local hooks** | **PROVEN** (working tree, uncommitted) | Literal `agent` / `landing` in `lefthook.yml`; bundle list above. HEAD@`912b81e4` still had `${VF_WRITE_ROLE}` + shared anchor. |
| **R1 remote auth** | **PARTIAL — needs push** | Owner phrase `commit push lefthook #14083`; until push, PR head bypass remains open on remote and auth boundary is not **PROVEN** on #14083. |
| **CI merge gate** | **PROVEN** @ `912b81e4` | `git show 912b81e4:.github/workflows/own-language.yml` and `spec-guards.yml` — `printf 'landing\n--\n'` only; vf steps green on run ids in `CI_EVIDENCE@912b81e4` below. |

## Threat model (remaining)

1. **Rename gap:** `kind_checked` ignores `D` and `R*` — extend parser + tests.
2. **Bypass:** `--no-verify` / `LEFTHOOK=0` forbidden by law; CI is merge gate.
3. **Hook role bypass:** **CLOSED** on working-tree `lefthook.yml` — pre-commit literal `agent\n--\n`, pre-push literal `landing\n--\n` (no env indirection); CI hardcodes `landing`.

## Local refusal tests (this Mac)

**R5 PROVEN:** `test-report` **13/0** on refreshed spec + `gen/c/policy/vf_write_gate.c` (2026-11-17). Embedded CC gate smoke: contract path refused under landing header.

**Static proof (spec tests on branch):** `agent_denied_layout`, `default_role_is_agent`, `ignores_deletions`, `unlisted_path_denied_for_agent` in `specs/policy/vf_write_gate.t27`; layer matrix tests in `specs/t27f/vf_guard.t27`.

**Staged gap:** pre-commit uses `git diff --cached`; uncommitted layout edits are invisible until `git add`. Pre-push uses `origin/master...HEAD` (full branch delta).

**Verifier crash:** Hook `main` returns `check_vf_writes != 0`; `cc` failure aborts before gate — not a silent pass.

## CI_EVIDENCE@912b81e4

PR #14083 head `912b81e4a33a240c7f896fbaaf6433f671749dc3` (verifier polls 2026-11-17, no push/merge).

| Check | Result | Run id |
|---|---|---|
| own-language | **PASS** (vf step) | `43938970788` |
| vf-diff | **PASS** | `43938970462` |
| scan | **PASS** | `43938970701` |
| spec-guards | **FAIL** — `STALE generated-code hash : 201`; `check_seal_currency` (step ~146) exits **before** *VF write gate -- landing role* (~178); vf landing step does not run (no `VF write` in log) | `43938970537` |

**CI enforcement verdict:** **PARTIAL** — own-language + vf-diff prove the vf path; spec-guards red is unrelated 201 stale seals, not a vf refusal.

## PR #14083 CI (gh, read-only)

Run `43928840687` (2026-11-16T18:26:33Z):

| Check | Result |
|---|---|
| check-linked-issue | **pass** |
| vf-diff | pass |
| spec-guards | **fail** (committed `gen/c/claw/gates.c` drift vs regenerated) |
| cli-tri | **fail** (`tri census pin --gate`: workflow step count 343→345) |
| scan | **fail** (hardcoded `/Users/playra/` in `runs/ref_full*.txt`) |
| own-language | pending at snapshot |
| emit-bitexact, corpus, coverage, t27b-native | pending |

## Issue #14080

- **Symptom:** `pull_request_target` workflows with `conclusion: startup_failure`, **0 jobs** (`gh api …/runs/43913957841/jobs` → `total_count: 0`).
- **UI message:** "This run likely failed because of a workflow file issue" (`gh run view 43913957841`).
- **Last success before wave:** Issue Gate success at 2026-11-16T18:08Z on branch `docs/merge-verdict-startup-failure-trap`; PR #14083 Issue Gate **succeeded** at 18:26Z — not universally broken at verify time.
- **Org root cause:** issue body states org Actions policy **HTTP 403** with member token → **UNKNOWN** (not guessed).
- **Measured 2026-11-17 tail:** last 15 Issue Gate runs → **11** `success` / **4** `startup_failure`; latest failure **43918854613** (`t27os/logger-black-uikit-on-d`); latest vf-p0 success **43938965417**; issue body may be **stale vs tail**; org 403 still **UNKNOWN**.

## Visual build loop (0.2.0 / `glass_window.t27`)

From `docs/t27os/run-local.md` (no build run on Mac — load rule):

1. Ancestry: branch contains `specs/t27os/glass_window.t27` closure; version string `0.2.0` pinned in `specs/t27os/statusbar.t27` tests.
2. One-shot: `make t27os` → `target/t27os/T27OS.app`, `target/t27os/t27os-pty`.
3. Hot loop: `target/release/t27b watch specs/t27os/glass_window.t27 -o target/t27os/glass_window.o --macho --then 'make t27os'`.
4. Launch: PTY server first, then `open target/t27os/T27OS.app` — **avoid writing** `~/.t27os/stable` (updates tree under `~/.t27os/updates/` is release machinery).
5. Read-only stable check: `ls ~/.t27os` only if needed; do not run publish/swap.

**Screenshot instability (P1 context):** #14008 — `T27OS_STILL=1` frames still capture live pane noise; boundary `specs/t27os/glass_window.t27` only.

## P1 #14008 STILL (not vf P0)

P1 #14008 STILL prep lives in T27L seq 16 `goal_p1_still_fr` (`~/.t27os/logs/t27l.jsonl`).
Not vf P0 — keep this skill on vf landing/agent hooks (#14083); do not fold STILL implementation here.
Key gap: `specs/t27os/frame_gate.t27` — live panes keep short event waits; STILL frames still capture pane noise.

## P1 #14008 preparation

- **Contract:** `specs/t27os/screenshot_pipeline.t27` — renderer → writer → non-empty artifact; merge needs green checks + owner approval; `SP_TRI_VISUAL_UPDATE_BUILT = 0`.
- **Test plan (from issue):** 8 cases × 2 captures 3s apart after `T27OS_STILL_READY`; SC-003 spec tests no vacuous pass.
- **Write paths:** Primary edit surface `specs/t27os/glass_window.t27` (STILL path). **Not** in `vf_guard` `GD_PATHS` — vf-write-gate does **not** classify `glass_window.t27` (agent-allowed at vf layer; other gates apply).

## cursor-agent worker CLI (verified snippet)

```
agent worker start --worker-dir <path>   # workspace root exposed to cloud agents
--name <name>                            # default hostname; playra-mac in use
```

Full help: `cursor-agent worker --help` on this machine (2026-11-17).

## Next executable tasks

1. On PR branch: `tri census pin --bless` for workflow step census; regenerate/commit `gen/c/claw/gates.c` to fix spec-guards.
2. Remove or quarantine `/Users/playra/` paths from `runs/ref_full*.txt` for scan gate.
3. ~~Document pre-commit/pre-push literal roles vs CI hardcoded `landing`~~ — done in this skill + working-tree `lefthook.yml`.
4. Optional: extend `check_vf_writes` for rename lines `R*`.
5. P1: implement FR-001–003 in `glass_window.t27` (#14008), then screenshot pipeline wiring.