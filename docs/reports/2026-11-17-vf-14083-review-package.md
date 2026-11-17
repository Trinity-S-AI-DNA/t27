# VF review package — gHashTag/t27 #14083

**OWNER_APPROVAL_BLURB**

**Commit (required):** `specs/policy/vf_write_gate.t27`, `gen/c/policy/vf_write_gate.c`, `lefthook.yml` — **optional:** `docs/reports/2026-11-17-vf-14083-review-package.md`, `.claude/skills/vf-write-gate-verify/SKILL.md`.

**Message:** `vf: literal lefthook roles and landing effective_role clamp refs #14083`

**Push:** Triggers CI on PR #14083 only; does **not** merge the PR or claim full merge-gate green.

**Owner phrase:** `commit push lefthook #14083`

Ledger date: 2026-11-17. Sources: `/tmp/vf_review_package_14083.txt`, `/tmp/vf_bundle_only.patch` (2785 bytes, 68 lines), `/tmp/vf_goal_owner_gate.txt`. No secrets.

## Git anchors

| Field | Value |
|-------|-------|
| Worktree | `/Users/playra/t27` |
| Branch | `vf-p0-ci-fix` |
| PR | #14083 (`claude/vf-write-gate-p0`) |
| Merge-base | `a1f4791d07e2fd9947a021f0747a34aa75ff1bee` (origin/master) |
| Branch HEAD | `912b81e4a33a240c7f896fbaaf6433f671749dc3` |

## Exact 3-file manifest (VF bundle IN)

1. `specs/policy/vf_write_gate.t27`
2. `gen/c/policy/vf_write_gate.c`
3. `lefthook.yml`

**OUT of bundle:** `specs/t27f/vf_guard.t27`, `.claude/skills/vf-write-gate-verify/SKILL.md`, all `specs/t27os/*` / browser lane (0.3.1–0.3.2).

Committed diff stat (`origin/master...HEAD`): 859 insertions across those three paths. Uncommitted WT delta on same paths: `effective_role` landing clamp, lefthook literal `printf` roles (no `VF_WRITE_ROLE` env), gen-c regen (+19/−7).

## vf_guard bootstrap-self-write

| Layer | Status |
|-------|--------|
| **FIXED on PR HEAD** | GATE_BOOTSTRAP semantics: agent+engine may write `vf_write_gate.t27` and `vf_guard.t27` (`is_gate_bootstrap`; test `gate_bootstrap_check_paths`). |
| **OUT of minimal bundle** | `vf_guard.t27` not in the three-file PR; reseal is follow-up, not P0 VF. |
| **Stale seal** | CI Seal Coverage run 43938970308: `vf_guard.t27` stale — hygiene failure, not vf_write_gate logic refusal. |
| **Local WT only** | Dirty tree may add `kit_iface.t27` to GD_PATHS — unrelated to bootstrap. |

## Test scope (13/0)

`MAC_HEAVY_OK=1 t27c test-report specs/policy/vf_write_gate.t27` → **13 pass / 0 fail** applies to **`vf_write_gate.t27` only**, not the full owner goal matrix in `vf_goal_owner_gate.txt`.

## R1 HEAD vs WT (lefthook)

**HEAD@912b81e4:** pre-commit sets `VF_WRITE_ROLE: agent` and pre-push sets `VF_WRITE_ROLE: landing`; both share `printf "${VF_WRITE_ROLE}\n--\n"`. **WT:** no `VF_WRITE_ROLE` key; literals `printf 'agent\n--\n'` / `printf 'landing\n--\n'`. Bypass closed locally, not on remote until bundle commit.

## vf_write_gate landing / effective_role

**FIXED in WT (uncommitted):** `effective_role` returns `GR_ENGINE` when landing would need `GR_OWNER`, so gate-bootstrap pairs clear while contract paths stay denied.

## Patch artifact

`PATCH_PATH=/tmp/vf_bundle_only.patch` — `PATCH_FILES` = the three IN paths above; 2785 bytes.

## Browser lane cross-ref (0.3.2)
Ledger: [`docs/reports/2026-11-17-browser-032-tick.md`](2026-11-17-browser-032-tick.md) (BROWSER_032_TICK).
**SPEC_GREEN** — loopback runner/host/bridge/framebuffer + `glass_window` green 2026-11-17 per tick §SPEC_GREEN; OUT of VF three-file IN.
**Host NEXT_BLOCKER** — owner relaunch rebuilt `target/t27os-local/T27OS.app` (stale in-memory vs post-`make` disk) per tick §RELAUNCH_STATUS; orthogonal to vf_write_gate WT.
Cross-ref only; browser host unblock is owner relaunch, not VF bundle scope. Browser tick does not close VF gates.

## Measured 2026-11-17 afternoon

Remeasured on `/Users/playra/t27`, branch `vf-p0-ci-fix`, HEAD `912b81e4a33a240c7f896fbaaf6433f671749dc3`. No commit, no push. `gh pr view 14083` succeeded.

| Fact | Measurement |
|------|-------------|
| HEAD vs WT | All three bundle paths dirty. `git diff --stat HEAD` = 19 insertions, 7 deletions. |
| HEAD `VF_WRITE_ROLE` | Present. Pre-commit line 27 `agent`; pre-push line 49 `landing`; shared anchor line 32 `printf "${VF_WRITE_ROLE}\n--\n"`. |
| WT `VF_WRITE_ROLE` | Absent (count 0). Pre-commit `printf 'agent\n--\n'`; pre-push `printf 'landing\n--\n'`. |
| gen sha256 | HEAD `6a443cde22734e11fb6ff8ab326ef0553f1b08cbcd0f415b02bdafeaf7f67135`. WT `fe199d5b84a0e6bb7abf8f6e97b641b1d96edd7f8e1438122b0d3bf01763b39f`. Spec and `gen/c` diffs are the same `effective_role` clamp (`need == GR_OWNER` returns `GR_ENGINE`). |
| Committed vs master | `origin/master...HEAD` on the three paths: 859 insertions, 17 deletions. |
| test-report | `MAC_HEAVY_OK=1 ./target/release/t27c test-report specs/policy/vf_write_gate.t27` -> tests 13, pass 13, FAIL 0, rate 100.0%. Vacuous passes **unknown** (counting build: `assert_eq` not in source). |
| C smoke | Superseded by **Measured 2026-11-17 later** §B (`MAC_HEAVY_OK=1`: compile exit 0; `landing` + `visual_factory_contract.t27` stdin exit **1**, stdout `specs/t27f/visual_factory_contract.t27: contract`). |
| CI headers at HEAD | `own-language.yml` and `spec-guards.yml` use literal `printf 'landing\n--\n'` (no `VF_WRITE_ROLE`). |
| PR | OPEN. Title: `t27f: wire vf_guard write gate (P0, refs #13251)`. |

PR check rollup (head runs, completed 2026-11-16; read 2026-11-17 afternoon):

| Check | Conclusion | Run |
|-------|------------|-----|
| own-language | SUCCESS | 43938970788 |
| vf-diff | SUCCESS | 43938970462 |
| scan | SUCCESS | 43938970701 |
| cli-tri | SUCCESS | 43938970389 |
| check-linked-issue | SUCCESS | 43938965417 |
| spec-guards | FAILURE | 43938970537 |
| coverage | FAILURE | 43938970308 |
| emit-bitexact | FAILURE | 43938970853 |
| pr-dashboard | FAILURE | 43938970336 |
| t27b-native-ratchet | FAILURE | 43938970653 (linux job success; macos cancelled) |

spec-guards failed log: `python3 tools/check_seal_currency.py` prints `STALE generated-code hash     : 201`. No `VF write` line in that failed log. Browser lane remains cross-ref only and does not close these gates.

## Measured 2026-11-17 later (read-only; no commit, no push)

Worktree `/Users/playra/t27`, branch `vf-p0-ci-fix`. `MAC_HEAVY_OK=1` is the documented one-command override in `~/.claude/hooks/mac-no-heavy.py` (owner hook, 2026-10-09). Used only for the smoke compile and the two `t27c test-report` calls below.

### A. Issue #14080 exact failure

Issue still OPEN. `updatedAt` `2026-11-16T17:23:11Z`. Title claims every `pull_request_target` workflow has been `startup_failure` since 2026-11-16 16:01 UTC. Body says the org Actions policy is not readable with a member token (HTTP 403), so the cause is UNKNOWN.

Quoted UI lines (`gh run view`, fetched 2026-11-17). Each failed run has `jobs.total_count` 0. `gh run view 43913957841 --log-failed` printed `failed to get run log: log not found`. No workflow log line contains `403`. Org 403 stays UNCONFIRMED.

| Run | created_at | Workflow | Quote |
|-----|------------|----------|-------|
| 43902878339 | 2026-11-16T16:01:36Z | PR budget, PR #14042 | `This run likely failed because of a workflow file issue.` |
| 43913957841 | 2026-11-16T17:01:47Z | Issue Gate, PR #14078 | same sentence |
| 43918854613 | 2026-11-16T17:29:07Z | Issue Gate, PR #14077 | same sentence |

Tail the same day, `gh run list -w issue-gate.yml -L 15`: 11 `success`, 4 `startup_failure`. The four failures are the older rows (16:20Z through 17:29Z). The latest five Issue Gate runs are all `success` (19:09Z through 20:38Z). PR #14083 run `43938965417` (`2026-11-16T19:24:13Z`) has `jobs.total_count` 1, job name `check-linked-issue`, `gh pr checks` conclusion `pass`. PR budget also succeeds again from `2026-11-16T18:08:00Z` (run `43925607120`). The issue body is stale against this tail. The issue stays open; this pass did not edit it.

### B. C smoke refusal

Documented one-liner is the CI step in `.github/workflows/own-language.yml` (and the same `printf | cc` in `spec-guards.yml`), not a second copy in the spec. Local replay with `MAC_HEAVY_OK=1`:

`cc -w -I . -x c -o target/vf_gate_smoke -` on the `check_vf_writes` driver: exit 0.

stdin `landing\n--\nM\tspecs/t27f/visual_factory_contract.t27\n` into that binary: stdout `specs/t27f/visual_factory_contract.t27: contract`, exit 1.

### C. P1 #14008 specs (not the host captures)

`MAC_HEAVY_OK=1 ./target/release/t27c test-report` on 2026-11-17:

| Spec | tests | pass | FAIL | vacuous |
|------|------:|-----:|-----:|---------|
| `specs/t27os/screenshot_pipeline.t27` | 9 | 9 | 0 | unknown (`assert_eq` not in source). 1 invariant. `SP_HOST_RENDERER = 0`. |
| `specs/t27os/frame_gate.t27` | 14 | 14 | 0 | unknown (same counting note). Includes `a_still_frame_is_skipped_until_forced` and `the_wait_grows_only_when_quiet_and_no_pane_is_live`. |

Host STILL captures were not re-run. Last measurement on #14008 (2026-11-16, master `32c616f248` and PR #14005 head `26b78ac660`): master 6 of 8 sha256-equal, PR head 3 of 8 equal. SC-001 asks for 8 of 8. Host is not 8/8. These spec reports do not move that score.

### D. 0.2.0 glass_window iteration without `~/.t27os/stable`

Read, not launched. The no-tamper channel is the dev bundle, not `make t27os` into `target/t27os/` (that path does not set `T27OS_CHANNEL`, so the identity stays production `ai.t27.os`).

- `specs/t27os/install.t27`: `CHANNEL_PROD_ID` `ai.t27.os`, `CHANNEL_DEV_ID` `ai.t27.os.dev`. Empty channel is production. Exact name `dev` is the only other accepted channel.
- `specs/t27os/plist_main.t27`: `T27OS_CHANNEL=dev` writes bundle id `ai.t27.os.dev` so LaunchServices keeps the dev app off the installed one.
- `specs/t27os/glass_window.t27`: `GW_DEV_HOME` `/.t27os-dev`. A bundle id containing `.dev` keeps window files under `<home>/.t27os-dev` instead of `<home>` (the tree that holds `~/.t27os/stable`).
- Repo skill `.claude/skills/t27os-dev-loop/SKILL.md`: DEV_UI is `target/t27os-local`, `T27OS_CHANNEL=dev`, bundle `ai.t27.os.dev`. It forbids opening `~/.t27os/stable`, `/Applications/T27OS.app`, and `target/t27os/` without the dev channel, and forbids repointing `~/.t27os/stable`.
- Window line 0.2.0 is the shipped window (`release.t27` / `.claude/skills/t27os-visual-iteration/SKILL.md`). A relaunch is still an owner step. This pass did not build or open the app.

### E. Skills checked (no new skill files)

Repo `.claude/skills/`: `issue-shape` (four-heading gate before any issue edit), `foreign-budget-discipline` (`lefthook.yml` is foreign; 40-per-file / 80-per-PR and added<=deleted before a bundle commit), `merge-verdict` (no MERGE_NOW while required checks are red; owner has not approved merge), `merge-verdict-issue-gate` (#14080 ledger; do not treat a missing `check-linked-issue` as a lane YAML bug).

Home only, not in the repo tree: `~/.claude/skills/only-t27` (do not hand-write a new smoke driver; the CI one-liner was replayed), `~/.claude/skills/tri-ci-fix` (its steps amend and force-push; do not run them on this task).

### F. Not one global CI block

`gh pr checks 14083` on 2026-11-17: `vf-diff` pass (run `43938970462`), `own-language` pass (`43938970788`), `check-linked-issue` pass (`43938965417`). `spec-guards` fail (run `43938970537`). Re-read of that failed log: `2026-11-16T19:27:52.3018047Z   STALE generated-code hash     : 201`. The same log grep found no `VF write` line. Other reds on the same head remain `coverage`, `emit-bitexact`, `pr-dashboard`, and `t27b-native-ratchet` (`t27b-native-linux` pass, `t27b-native-macos` fail 15m33s on this read). Those reds are separate from the vf-diff pass and from the closed startup_failure window above. #14083 is not blocked by a single gate.

## OWNER_DECISION_PACKET

Ledger: 2026-11-17 coordinator pass. No commit, no push, no `gh issue edit`.

### Bundle manifest (exact three paths)

1. `specs/policy/vf_write_gate.t27`
2. `gen/c/policy/vf_write_gate.c`
3. `lefthook.yml`

### Working tree delta vs branch HEAD (`git diff --stat HEAD` on bundle only)

```
 gen/c/policy/vf_write_gate.c   |  6 +++++-
 lefthook.yml                   | 14 +++++++++-----
 specs/policy/vf_write_gate.t27 |  6 +++++-
 3 files changed, 19 insertions(+), 7 deletions(-)
```

(Command: `git diff --stat HEAD -- specs/policy/vf_write_gate.t27 gen/c/policy/vf_write_gate.c lefthook.yml` from `/Users/playra/t27`, branch `vf-p0-ci-fix`, HEAD `912b81e4a33a240c7f896fbaaf6433f671749dc3`.)

### Commit message draft (owner applies; agent must not commit without phrase)

```
vf: literal lefthook roles and landing effective_role clamp refs #14083
```

### What PR head should prove after owner push

| Proof target | Remote today (912b81e4) | After bundle lands |
|--------------|------------------------|---------------------|
| **lefthook** pre-commit | `VF_WRITE_ROLE: agent` + `printf "${VF_WRITE_ROLE}\n--\n"` (HEAD lines 27-33) | No `VF_WRITE_ROLE` key; `printf 'agent\n--\n'` (WT lines 26-32) |
| **lefthook** pre-push | `VF_WRITE_ROLE: landing` + shared anchor expands env (HEAD lines 49-50, anchor 32) | No `VF_WRITE_ROLE`; `printf 'landing\n--\n'` (WT lines 47-53) |
| **effective_role** | Landing header could demand `GR_OWNER` on gate-bootstrap PRs | `need == GR_OWNER` returns `GR_ENGINE` in spec + gen/c (WT; `vf_write_gate.t27` ~134-138) |
| **CI vf landing step** | Already literal `printf 'landing\n--\n'` in workflow yaml | Unchanged; should stay green (`vf-diff`, own-language vf step) |

### Expected CI after push (VF story vs hygiene)

| Check | Expectation after bundle | Notes |
|-------|--------------------------|-------|
| `vf-diff` | Stay **green** | Already SUCCESS run 43938970462 on current head |
| `own-language` (vf step) | Stay **green** | Literal landing at `.github/workflows/own-language.yml` L45-46 |
| `spec-guards` vf step | Stay **green** (no `VF write` failure) | Literal landing at `spec-guards.yml` L184-185; current fail is **201 stale hashes**, not vf gate stdout |
| `spec-guards` (full job) | Likely **still red** | `STALE generated-code hash : 201` until separate hygiene land |
| `coverage`, `emit-bitexact`, `pr-dashboard`, `t27b-native-ratchet` | Likely **still red** | Separate from VF P0 bundle per report Measured table |

**VF P0 narrative flip:** R1 closed on GitHub (literal hooks) + landing bootstrap behavior on remote; vf-diff/own-language vf steps remain proofs; full merge gate stays open on 201-stale and sibling reds.

### Mandatory CI enforcement gap (honest split)

**GitHub Actions (committed on PR head; WT = HEAD for these files):**

- `.github/workflows/own-language.yml` L45-46: `printf 'landing\n--\n'` then `git diff --name-status "$BASE...$HEAD"` into `check_vf_writes` smoke binary. No `VF_WRITE_ROLE`.
- `.github/workflows/spec-guards.yml` L184-185: same landing literal and diff feed. No `VF_WRITE_ROLE`.

**Local lefthook (bundle delta only on WT):**

| Location | HEAD@912b81e4 | WT (uncommitted) |
|----------|---------------|------------------|
| pre-commit env | L27 `VF_WRITE_ROLE: agent` | L26: only `VF_WRITE_DIFF: --cached` |
| pre-commit stdin | L32 `printf "${VF_WRITE_ROLE}\n--\n"` | L31 `printf 'agent\n--\n'` |
| pre-push env | L49 `VF_WRITE_ROLE: landing` | L47: only `VF_WRITE_DIFF: origin/master...HEAD` |
| pre-push stdin | anchor uses `${VF_WRITE_ROLE}` | L52 `printf 'landing\n--\n'` |

**Split:** CI on current PR head already enforces landing with a **literal** printf in workflow yaml (cannot inherit shell `export VF_WRITE_ROLE=landing`). **Lefthook on remote HEAD still expands `${VF_WRITE_ROLE}`**, so inherited-shell bypass remains on developer machines until the bundle is pushed. WT matches CI semantics locally; remote PR diff does not yet.

### D follow-up: glass_window 0.2.0 dev iteration (minimal owner checklist)

Source: `.claude/skills/t27os-dev-loop/SKILL.md` section 0 (VERSION_GATE) and sections 3-4. Shipped window semver remains **0.2.0** (`release.t27` / visual-iteration skill); dev copy is **not** `~/.t27os/stable`.

1. **Channel:** Build and launch only **DEV_UI** -- `T27OS_CHANNEL=dev`, bundle id `ai.t27.os.dev`, output tree `target/t27os-local` (forbid `~/.t27os/stable`, `/Applications/T27OS.app`, and `target/t27os/` without dev channel).
2. **Test:** `~/t27/target/release/t27b test specs/t27os/glass_window.t27 --quiet` (read `target/test.out`; rc alone lies).
3. **Build:** `t27b build specs/t27os/glass_window.t27 -o target/t27os-local/glass_window.o --macho`, then link/sign with `T27OS_CHANNEL=dev` plist step into `target/t27os-local/T27OS.app`.
4. **Relaunch:** Kill only **your** worktree `T27OS` pid; `open -n --env T27OS_PAGE=<petal> target/t27os-local/T27OS.app` (do not repoint stable).
5. **Proof line:** Paste `binary_path`, `bundle_id`, `CFBundleVersion`, `git_HEAD`, `STALE_UI_BASE=YES|NO` per skill section 0.

### #14008 -- owner path to SC-001 (8/8 STILL sha256)

Issue **t27os window: T27OS_STILL frames are not still** (`specs/t27os/glass_window.t27` boundary only). Success criterion SC-001: on **one build**, pages home/C/F/Q in dark and light, **two captures 3 s apart after `T27OS_STILL_READY`**, equal sha256 on **two separate launches** (8 of 8). Measured 2026-11-16: master 6/8, PR #14005 head 3/8; live pane / lefthook banner still reaches the frame under STILL. Owner would: merge FR-001..003 behavior (no wall-clock or live-pane paint after first complete frame, emit `T27OS_STILL_READY`, spec tests for STILL hold rules), build **dev or release** binary from that `glass_window.t27`, then run the issue shot recipe twice per case (`T27OS_PAGE`, `T27OS_THEME`, `T27OS_STILL=1`, wait for ready line, capture at T and T+3s, compare sha256). Specs `screenshot_pipeline.t27` (9/0) and `frame_gate.t27` (14/0) passed locally 2026-11-17 but do not replace host 8/8 (`report` section C). This is P1; not in the VF three-file bundle.

### Spec test gap (lefthook literals)

**OPEN:** `specs/policy/vf_write_gate.t27` has 13 tests for roles, landing, and gate-bootstrap paths; **no test** asserts that hook stdin must use literal `agent`/`landing` lines (no `${VF_WRITE_ROLE}` expansion contract). R1 is proven by `lefthook.yml` diff review and manual zsh bypass reasoning, not by `t27c test-report`. No new spec added this pass (economy).

### Coordinator summary -- top two owner phrases

1. **"commit push lefthook #14083"** (or equivalent) -- lands the 3-file WT bundle; closes R1 on GitHub, syncs `effective_role` clamp on remote, re-runs CI so PR head matches local bypass closure; unlocks objective 1, 2, 6 without claiming full merge green.
2. **"relaunch dev app"** (after `make` / dev-loop build) -- unblocks 0.2.0 visual proof on `target/t27os-local` with `T27OS_CHANNEL=dev`; orthogonal to VF bundle but required for STALE_UI and #14008 host work (`vf_goal_owner_gate.txt` item 8-9).

---

## Objective completion audit (goal matrix)

Source bullets: `/tmp/vf_goal_owner_gate.txt` objectives 1-12 + `GOAL_COMPLETE`. Evidence pointers from this report and measured commands unless noted.

| # | Objective | Status | Evidence pointer |
|---|-----------|--------|------------------|
| 1 | VF write gate spec + gen/c + lefthook on **remote** PR head | **PARTIAL** | Three paths exist on HEAD@912b81e4 with old lefthook + no WT clamp; fix is **only** in uncommitted WT (+19/-7, `git diff --stat HEAD` above). |
| 2 | R1 close `${VF_WRITE_ROLE}` bypass on GitHub | **OPEN** (remote) / **PROVEN** (local WT) | HEAD lefthook L27-33, L49-50 vs WT L31-32, L52; report section R1. |
| 3 | R5 vf_write_gate test-report 13/0 | **PROVEN** | `MAC_HEAVY_OK=1 ./target/release/t27c test-report specs/policy/vf_write_gate.t27` -- report Measured table. |
| 4 | CI vf paths (own-language vf step, vf-diff) | **PROVEN** @912b81e4 | PR checks runs 43938970788, 43938970462 -- report Measured table. |
| 5 | Full PR merge gate green | **OPEN** | spec-guards 201 stale, coverage, emit-bitexact, pr-dashboard, t27b-native-ratchet red -- report Measured table. |
| 6 | Mandatory CI re-run post-uncommitted bundle | **OPEN** | No push; owner phrase required -- `vf_goal_owner_gate.txt` L41-42. |
| 7 | #14080 startup_failure / org 403 | **PARTIAL** | Issue Gate tail green; org 403 UNCONFIRMED -- report section A. |
| 8 | #14008 STILL + host 8/8 | **OPEN** | Issue body 6/8 and 3/8; SC-001 -- report section C and #14008 paragraph above. |
| 9 | DEV_UI at `target/t27os-local` | **PROVEN** | `vf_goal_owner_gate.txt` PROOF_LINE; exclude 0.3.1 from VF bundle. |
| 10 | vf-write-gate-verify skill alignment | **PARTIAL** | Untracked/optional; not in 3-file IN list -- report Exact manifest. |
| 11 | Land / merge #14083 | **OPEN** | `AWAITING_PUSH_APPROVAL` -- goal file L56-57. |
| 12 | GOAL_COMPLETE | **OPEN** | goal file L59-60; blocked by 1, 5, 6, 8, 11. |

## GOAL_REMAINING

Ledger: coordinator pass 2026-11-17. OPEN/PARTIAL rows only; one next action each (max 8).

- **1 + 2 (PARTIAL / OPEN remote):** Owner commits and pushes the 3-file WT bundle (`vf_write_gate.t27`, `gen/c`, `lefthook.yml`) so GitHub PR diff has literal `printf 'agent'/'landing'` and no `VF_WRITE_ROLE` env keys.
- **6 (OPEN):** After owner phrase **commit push lefthook #14083**, poll `gh pr checks 14083` for own-language and spec-guards vf landing steps on the new head.
- **5 (OPEN):** After bundle push, run tri-ci-fix batch on reds: spec-guards 201 stale hashes, coverage (`vf_guard` seal), emit-bitexact, pr-dashboard, t27b-native-ratchet (measured red 2026-11-17).
- **7 (PARTIAL):** Owner documents org Actions 403 on #14080 or closes the issue when Issue Gate tail-green plus zero-job startup_failure window is accepted as sufficient.
- **7 (PARTIAL), paste draft:** `docs/reports/2026-11-17-14080-issue-update-draft.md` — owner copies the comment block onto #14080 (close or historical retitle).
- **8 (OPEN):** Land P1 #14008 FR-001..003 on `glass_window.t27` (STILL ready, live-pane isolation), rebuild dev app, re-run SC-001 host 8/8 sha256 capture recipe (not in VF P0 bundle).
- **10 (PARTIAL):** Skill documented at `.claude/skills/vf-write-gate-verify/SKILL.md` (frontmatter, in-repo report pointer, MAC_HEAVY_OK=1 commands, lefthook `grep VF_WRITE_ROLE` preflight, own-language.yml C smoke, no-push / no tri-ci-fix). Optional commit with VF bundle or separate skills land.
- **11 (OPEN):** Merge #14083 only after explicit owner push approval (`AWAITING_PUSH_APPROVAL`) and merge-verdict green required checks -- do not merge on vf-diff alone.
- **Preflight OK:** Working-tree `lefthook.yml` has zero `VF_WRITE_ROLE` matches (`grep -n VF_WRITE_ROLE lefthook.yml` exit 1); `git diff HEAD -- lefthook.yml` removes env keys and uses literal `printf 'agent\n--\n'` / `printf 'landing\n--\n'` (WT matches intended R1 closure locally; remote HEAD still has `${VF_WRITE_ROLE}` until push).

## Goal source (in-repo)

Canonical copy of `/tmp/vf_goal_owner_gate.txt` objectives 1-12 (2026-11-17). Ledger must not depend on `/tmp`.

```
VF P0 GOAL -- gHashTag/t27 #14083 -- OWNER GATE
Branch: vf-p0-ci-fix @ 912b81e4a33a240c7f896fbaaf6433f671749dc3
Land state: AWAITING_PUSH_APPROVAL

1. VF write gate spec + gen/c + lefthook on remote PR head
   PARTIAL | Commit+push 3-4 file bundle (vf_write_gate.t27, gen/c, lefthook.yml, optional vf_guard/skill); exclude 0.3.1 t27os version specs.

2. R1 -- close ${VF_WRITE_ROLE} lefthook bypass on GitHub
   OPEN (remote) / PROVEN (local WT) | Same push as (1); verify PR diff shows literal agent/landing printf, no VF_WRITE_ROLE env.

3. R5 -- vf_write_gate local test-report 13/0
   PROVEN | MAC_HEAVY_OK=1 t27c test-report specs/policy/vf_write_gate.t27 -> 13 pass / 0 fail.

4. CI -- vf gate paths (own-language vf step, vf-diff)
   PARTIAL @912b81e4 | After (1): poll gh pr checks 14083 -- own-language + vf-diff must stay SUCCESS.

5. CI -- full PR merge gate green
   OPEN | After push: tri-ci-fix batch -- spec-guards 201, coverage vf_guard seal, emit-bitexact, pr-dashboard, t27b-native-ratchet.

6. CI mandatory re-run post-uncommitted bundle
   OPEN | Owner phrase "commit push lefthook #14083" then mandatory poll of own-language + spec-guards vf landing step.

7. #14080 Issue Gate startup_failure / org 403 root cause
   PARTIAL | Last 5 Issue Gate runs green; org Actions 403 remains UNKNOWN -- owner documents or closes #14080 when org fact known.

8. #14008 STILL FR-001..003 + host 8/8 sha captures
   OPEN | P1 lane on glass_window.t27: T27OS_STILL_READY, live-pane isolation; not in VF P0 bundle.

9. DEV_UI app at target/t27os-local (ai.t27.os.dev)
   PROVEN | Built this tick; launch with open -n --env T27OS_PAGE=<petal> target/t27os-local/T27OS.app.

10. vf-write-gate-verify skill + evidence alignment
    PARTIAL | Untracked skill dir; optional same commit as (1) or separate skills land.

11. Goal land / merge #14083
    OPEN | AWAITING_PUSH_APPROVAL -- no agent push without owner word.

12. GOAL_COMPLETE
    OPEN | Not asserted; see objectives 1, 5, 6, 8, 11. (Source: GOAL_COMPLETE: no)
```

## LAND_READY_VERIFY

Timestamp: **2026-11-17 ~13:45 +7**. Verifier: vf-write-gate-verify skill. CWD `/Users/playra/t27`, branch `vf-p0-ci-fix`, HEAD `912b81e4a33a240c7f896fbaaf6433f671749dc3`. No commit, no push, no issue edit. Browser specs and `vf_guard.t27` untouched.

| Step | Command / check | Expected | Result | Exit |
|------|-----------------|----------|--------|------|
| 1 Preflight | `grep -n VF_WRITE_ROLE lefthook.yml` | zero matches (grep exit 1) | **PASS** — no output, `grep_exit=1` | 1 |
| 2 Lefthook WT | `git diff HEAD -- lefthook.yml` | literal `printf 'agent\n--\n'` / `printf 'landing\n--\n'`; no `VF_WRITE_ROLE` env | **PASS** — diff matches skill policy | 0 |
| 3 test-report | `MAC_HEAVY_OK=1 ./target/release/t27c test-report specs/policy/vf_write_gate.t27` | 13 pass / 0 fail | **PASS** — 13/0, rate 100.0% | 0 |
| 4 C smoke compile | `own-language.yml` one-liner → `target/vf_gate_smoke` | compile exit 0 | **PASS** | 0 |
| 5 C smoke run | stdin `landing\n--\nM\tspecs/t27f/visual_factory_contract.t27\n` | stdout `specs/t27f/visual_factory_contract.t27: contract`, exit 1 | **PASS** | 1 |
| 6 Bundle stat | `git diff --stat HEAD` on 3-file IN | spec + gen/c + lefthook only | **PASS** — 3 files, 19 insertions, 7 deletions; gen/c sha256 `fe199d5b84a0e6bb7abf8f6e97b641b1d96edd7f8e1438122b0d3bf01763b39f` | 0 |

**Verdict:** All verifier steps **PASS**. Working-tree VF bundle is **land-ready** pending owner phrase only (`commit push lefthook #14083`). Remote PR HEAD still carries `${VF_WRITE_ROLE}` until that push; CI merge gate on published HEAD remains partial per GOAL_REMAINING.

## CI_POLL

Polled **2026-11-17 13:47 +7**. Commands: `gh pr checks 14083`, `gh pr view 14083 --json headRefOid,updatedAt`. Head OID **unchanged** vs report anchor (`912b81e4a33a240c7f896fbaaf6433f671749dc3`). `updatedAt` `2026-11-16T19:24:09Z`.

| Check | Conclusion | Duration |
|-------|------------|----------|
| coverage | fail | 3m26s |
| emit-bitexact | fail | 2m22s |
| pr-dashboard | fail | 24s |
| spec-guards | fail | 3m35s |
| t27b-native-ratchet | fail | 9m50s |
| t27b-native-macos | fail | 15m33s |
| Check L1 TRACEABILITY | pass | 1m29s |
| Conflict markers (no unresolved merges) | pass | 23s |
| Corpus ratchet (expected-failure ledger) | pass | 16m23s |
| Damage negatives (false-positive fixtures) | pass | 14s |
| Documented t27c subcommands exist | pass | 1m23s |
| Every source file is reachable from its crate root | pass | 1m36s |
| L1 Traceability Summary | pass | 3s |
| No test binary shares one scratch directory | pass | 1m49s |
| Verilog widths (absurd-width ledger) | pass | 3m9s |
| check-linked-issue | pass | 5s |
| claw-criteria | pass | 20s |
| claw-criteria-post | pass | 16s |
| claw-sarif | pass | 19s |
| claw-suggest | pass | 4s |
| cli-tri | pass | 2m7s |
| duplicate-bodies | pass | 18s |
| fix-carries-source | pass | 19s |
| gate-topology | pass | 12s |
| gitleaks | pass | 29s |
| own-language | pass | 1m0s |
| parse-ratchet | pass | 1m23s |
| phi-loop-check | pass | 15s |
| scan | pass | 14s |
| t27b-native-linux | pass | 56s |
| untrusted-input | pass | 37s |
| validate | pass | 25s |
| vf-diff | pass | 59s |
| withdrawn-live | pass | 20s |
| claw-batch | skip | 0 |
| claw-merge | skip | 0 |
| claw-t27f | skip | 0 |
| resync | skip | 0 |

**Delta vs prior report rollup (§Measured 2026-11-17 afternoon):** Same head; same five reds (`coverage`, `emit-bitexact`, `pr-dashboard`, `spec-guards`, `t27b-native-ratchet`); vf trio (`vf-diff`, `own-language`, `check-linked-issue`) still green. **New detail this poll:** `t27b-native-macos` listed **fail** 15m33s (rollup had noted macos cancelled under ratchet only). No new pass/fail flips on listed merge blockers.

## POST_PUSH_CHECKLIST

After owner phrase `commit push lefthook #14083` (ordered):

1. `git add specs/policy/vf_write_gate.t27 gen/c/policy/vf_write_gate.c lefthook.yml` (optional: `docs/reports/2026-11-17-vf-14083-review-package.md`, `.claude/skills/vf-write-gate-verify/SKILL.md`).
2. `git commit -m "vf: literal lefthook roles and landing effective_role clamp refs #14083"` (hooks on; no `--no-verify`).
3. `git push -u origin vf-p0-ci-fix`.
4. `gh pr checks 14083 --watch`; then `gh pr view 14083 --json headRefOid,statusCheckRollup` — head OID must change from `912b81e4a33a240c7f896fbaaf6433f671749dc3`.
5. **Obj 1 + 2 proved:** GitHub PR diff — no `VF_WRITE_ROLE`; `lefthook.yml` uses literal `printf 'agent\n--\n'` (pre-commit) and `printf 'landing\n--\n'` (pre-push); spec + gen/c carry landing `effective_role` clamp.
6. **Obj 6 proved:** fresh runs **`own-language`** and **`vf-diff`** SUCCESS on new OID; optional log grep on **`spec-guards`** vf landing step — no `VF write` denial (201 stale line may still appear).
7. **Not goal complete:** **`STALE generated-code hash : 201`**, other merge reds (`coverage`, `emit-bitexact`, `pr-dashboard`, `t27b-native-ratchet`), or **merging #14083** without explicit owner merge approval.