# vf-write-gate-verify

Verified 2026-11-17 on `vf-p0-ci-fix` @ `1248d7000d` (VF P0 #14083). Proof over narration.

## Execution context

- **Lane worktree:** `/Users/playra/t27/.claude/worktrees/vf-p0-ci-fix`, branch `claude/vf-write-gate-p0`, HEAD `1248d7000d` (+3 over `origin/claude/vf-write-gate-p0` @ `23ac75eef5`).
- **Policy SSOT:** `specs/policy/vf_write_gate.t27` (HEAD); layer table + bootstrap paths in `specs/t27f/vf_guard.t27`.

## Landing role (CI)

`git show HEAD:specs/policy/vf_write_gate.t27` defines how CI picks the weakest role that still covers every added/modified `.t27` path:

| Mechanism | Spec lines (HEAD) |
|---|---|
| `ROLE_LANDING` string; CI sends `landing`, local lefthook stays `agent` | 25–28 |
| `header_has_landing` | 77–88 |
| `min_role_for_diff` (agent vs engine vs owner per path) | 93–131 |
| `effective_role` (landing → min role, else `parse_role`) | 134–138 |
| Tests: graph-only → agent; gate-bootstrap PR → engine; layout under landing still engine; contract → owner | 245–270 |
| Agent may edit bootstrap path `specs/policy/vf_write_gate.t27` alone | 273–276 |

**CI wiring (branch HEAD):** `spec-guards.yml` and `own-language.yml` use `printf 'landing\n--\n'` + `git diff --name-status` (not `VF_WRITE_ROLE`).

## Agent-only lefthook (local)

Local hooks **must not** read `VF_WRITE_ROLE`. Branch `lefthook.yml` always feeds `agent` before `--` (pre-commit `--cached`, pre-push `origin/master...HEAD`). Matches default `parse_role` when no header matches (48–63).

**Threat closed on branch:** exporting `VF_WRITE_ROLE=owner|engine` in the shell no longer changes hook input; merge enforcement is CI `landing` + unchanged agent default for human typos.

## GATE_BOOTSTRAP

Bootstrap paths let agent/engine land the gate itself without owner on check-layer paths:

- **Table:** `vf_guard.t27` — `GATE_BOOTSTRAP` / `GATE_BOOTSTRAP_COUNT` (policy + guard spec paths); `is_gate_bootstrap` / `gd_may_write` grant agent+engine on those paths only (105–180).
- **Landing composition:** `min_role_for_diff` calls `gd_may_write_bytes(..., GR_AGENT)` then `GR_ENGINE` (116–124 in `vf_write_gate.t27`); mixed PR with `vf_guard.t27` / `vf_storybook.t27` upgrades effective role to engine — see `landing_picks_engine_for_gate_bootstrap_pr` (252–256).

## Who may set role (summary)

| Surface | Role | Trusted? |
|---|---|---|
| lefthook `vf-write-gate` | literal `agent` | Yes (fixed stdin) |
| CI `own-language` / `spec-guards` | `landing` | Yes (GitHub runner) |
| `parse_role` | `agent` / `engine` / `owner` lines only | Typo → agent |

## Local refusal tests

**BLOCKED:** `t27c test-report` on owner Mac (load rule). Rely on spec `test` blocks in `vf_write_gate.t27` and `vf_guard.t27` until lab run.

## PR #14083 CI

Re-measure after owner push of squashed stack. Prior audit: `spec-guards` (`gates.c` drift), `cli-tri` (census), `scan` (`runs/ref_full*`), not vf logic.

## Issue #14080

`pull_request_target` `startup_failure`, 0 jobs; org Actions policy unreadable (HTTP 403) → **UNKNOWN** (issue body unchanged).

## Visual 0.2.0 / P1 #14008

Read-only prep only — **UNVERIFIED** (no `make t27os` / screenshot run on this verify).

## Squash prep (owner only — do not run in agent)

To fold the three commits ahead of remote into one staged tree, then commit once:

```bash
cd /Users/playra/t27/.claude/worktrees/vf-p0-ci-fix
git fetch origin claude/vf-write-gate-p0
git reset --soft origin/claude/vf-write-gate-p0
git commit -m "$(cat <<'EOF'
vf write gate: landing CI role, agent-only hooks, spec-guards stdin (Closes #13251)

EOF
)"
```

Full PR squash onto `origin/master` base (all five branch commits): use `git reset --soft origin/master` instead of the remote tip above, same `git commit`.

## Next executable tasks

1. Owner: squash + push when approved (no push without `PUSH_APPROVED`).
2. `tri census pin --bless`, regenerate `gen/c/claw/gates.c`, quarantine home paths in `runs/ref_full*.txt`.
3. P1: FR-001–003 in `glass_window.t27` (#14008) after P0 lands.