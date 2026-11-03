# Work-control inventory, 2026-10-31 (Phase A, read-only)

Lane issue: #12980 under epic #12945 (T27 M, the Golden Builder). Phase A of
"Unified work control and dev-workspace UX": find out what runs, what waits and
what lies around, before any claim, bar, status split, notification or archive
is designed. Nothing was renamed, deleted, closed, checked out, reset or
restarted while this file was made.

Data as of 2026-10-31 07:04Z. Every number below comes from one-line `git`
and `gh` commands run from the inventory worktree on the owner's Mac; nothing
was built, tested or executed. Home paths are written as `~`.

Reading rules used throughout:

- `E#?` means no issue body, PR body or label confirmed an epic. It is a gap to
  close, not a guess to fill.
- `unknown` means the source could not be read. It is never green.
- A quiet agent is not abandoned: its row says what it waits on and when it was
  last confirmed.
- A branch whose PR was squash-merged is not lost even when its commits are not
  in master: the receipt is `gh pr list --state merged --head <branch>`.
- A session is confirmed by the app's session list (title, worktree, branch,
  PR, last activity); a worktree by `git status --porcelain` and
  `git rev-list --count`; a PR by `gh pr view`.

## 0. Counts

| What | Count | Source |
| --- | --- | --- |
| Local branches | 1884 | `git for-each-ref refs/heads` |
| Local branches not in origin/master | 1328 | `git branch --no-merged origin/master` |
| ... of which local-only (no origin twin) | 1241 (7251 commits) | for-each-ref join |
| ... of which with an origin twin | 87 | for-each-ref join |
| Remote branches | 2741 (1904 under `fed/`, 625 flat, 42 `pr/`, 41 `lane/`) | `git for-each-ref refs/remotes/origin` |
| Local branches with upstream | 1418 (1055 upstream gone) | `%(upstream:track)` |
| Branches ahead of their own non-master upstream (truly unpushed) | 12 branches, 42 commits | `git rev-list --count @{u}..HEAD` |
| Worktrees | 75 (all measured; the Codex tree answered late, see 1.3) | `git worktree list --porcelain` |
| Worktrees with uncommitted tracked changes | 14 | `git status --porcelain -uno` |
| Open PRs | 32 | `gh pr list --state open` |
| Sessions reachable from this family | 16 (9 linked + 7 seen in the full list at 06:52Z) | app session list |
| Ownerless live items (no session, no PR, or no issue) | 6 open issues + 27 open PRs + 1 session without an issue | sections 3.5 |
| Duplicate or superseded groups with evidence | 9 groups | section 3.6 |
| Unresolved epic mappings | 30 items (`E#?`) | section 3.8 |

Branches by prefix (local): `claude/` 633, flat 535, `lane/` 141, `fix/` 131,
`feat/` 83, `night/` 78, `spec/` 62, `rescue/` 38. Unmerged branches by month:
1283 in 2026-10, 17 in September, 24 in August, 1 in July, 3 in May. Unmerged
branches by size: 995 carry 1 to 3 commits, 241 carry 4 to 10, 78 carry 11 to
50, 14 carry more than 50.

## 1. Work map

Columns: epic | issue | title or type | owner session | branch / worktree |
PR + head SHA | local uncommitted / unpushed | state | depends on | next step |
last confirmation (time, source) | receipt.

State vocabulary: running, waiting (on CI, on a PR, on an answer), red (CI
failed, needs a fix), blocked (merge conflict or a dead base), needs decision,
quiet (owner session idle, work uncommitted), done.

### 1.1 Live items with an owner session

| Epic | Issue | Title / type | Owner session | Branch / worktree | PR + head | Local uncommitted / unpushed | State | Depends on | Next step | Last confirmation | Receipt |
| --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- | --- |
| E#12945 | #12976 | dev-monitor skill opens the authorized DSH once per dev entry (lane) | "dev-monitor skill: open the authorized DSH on dev entry" (running, started by the parent) | `claude/zen-matsumoto-037cfe` / `~/t27/.claude/worktrees/zen-matsumoto-037cfe` | #12979 `a4dd9dea94`, not draft, BLOCKED | 0 / 0 (worktree = PR head) | waiting on CI: 12 checks pending, 2 failed, 20 passed | master seals (#12972) if the 2 failures are the inherited `coverage` set | read the 2 failures once the 12 finish | 07:00Z, session list + `gh pr view` | PR body `Closes #12976`, `Refs #12945` |
| E#12945 | #12970 | 14-field result record spec and survey of seven published results | "result-record-spec" (running, started by the AI-HW-SW session) | `claude/evolve-result-record-12970` / `~/t27/.claude/worktrees/result-record-spec-608e45` | none | 0 / 1 commit `9ea74a1573` ahead of master, upstream is origin/master, so the commit is not on origin | running | none known | push the branch, open the PR | 07:04Z, worktree | issue body `Refs #12945` (via the epic list in #12945) |
| E#? | #? | AI-HW-SW co-design mail | "AI-HW-SW-CoDesign" (running) | `claude/ai-hw-sw-codesign-mail-edae50` / `~/t27/.claude/worktrees/ai-hw-sw-codesign-mail-edae50` | none | 0 / 0 (tree at master `006777702b`) | running, no issue found | unknown | the session names its issue or confirms it is mail only | 06:52Z, session list | none |
| E#? | #12975 (CLOSED, shape-refused) | OpenCode free pool as a second executor, law in .t27 | "DSH" session (running, group translated: "GitHub issues and release") | `claude/opencode-free-pool` / `~/t27/.claude/worktrees/claude-code-key-rotation-9c2748` | #12977 `6f20bbaa37`, draft, UNSTABLE | 0 (2 untracked) / 0 | red: 7 checks failed, 2 pending, 25 passed; issue is dead | #12975 reopened with a passing body (it `Refs #12974`, which `Refs #12945`) | fix the issue body first, then the 7 reds | 06:53Z, `gh pr view` | PR body `Refs #12975` only |
| E#12387 | #12966 | fpga-synth-ax7203: place-and-route never reaches a .bit (heap placer lane) | "Lane #12966: fpga-synth-ax7203 placement budget" (idle) | `claude/lane-12966-ax7203-placement-2737dc` / `~/t27/.claude/worktrees/lane-12966-ax7203-placement-2737dc` | #12969 draft, head `8d5616fd2e`, UNSTABLE | 0 / 1 commit (`628e19c93c` is 1 ahead of the PR head) | waiting on PR #12965 and on master seals (#12972); 6 checks failed, 11 skipped, 24 passed | #12965, #12972 | push the 1 commit; undraft after #12965 lands | 06:22Z, session list; hand-over note `lane-12966-handover.md` | PR body `Closes #12966`, `Refs #12387` |
| E#12387 | #12964 (CLOSED, shape-refused) | drop the duplicate OP_I64_ADD that reds emit-bitexact | "Fix duplicate OP_I64_ADD that reds emit-bitexact" (idle, started by lane-12952) | `claude/wizardly-easley-fb851e` / `~/t27/.claude/worktrees/keen-jemison-0ef16c` | #12965 `2a796f6681`, not draft, UNSTABLE | 0 / 0 | red: 9 checks failed, 4 skipped, 34 passed; issue is dead | master seals (#12972) for the inherited reds; #12964 body fix | reopen #12964 with a passing body or retarget the receipt | 07:00Z, session list | PR body `Closes #12964`, `Refs #12387` |
| E#? | #12963 | claw: regenerate the gate inventory after t27os workflows were removed | "Regenerate specs/claw/gates.t27 (gate-topology drift)" (idle, started by lane-12952) | worktree `~/t27/.claude/worktrees/youthful-sammet-d1cba8` is on `claude/fix-claw-gate-inventory-12963`; the session record says `claude/nostalgic-babbage-b0eefa` | #12967 `9527b65673`, UNSTABLE | 0 / 0 | red: 7 checks failed, 4 skipped, 27 passed | master seals (#12972) | confirm which branch the session owns; read the 7 reds | 06:06Z, session list | PR body `Closes #12963` |
| E#? | #12972 | Seals are stale on master: 126 gen-drift and 9 stale | "Re-save stale seals so coverage goes green" (idle, started by regenerate-gates) | `claude/ecstatic-mclean-c5cf93` / `~/t27/.claude/worktrees/ecstatic-mclean-c5cf93` | none | 128 modified seal files uncommitted / 0 commits, no upstream | quiet since 06:08Z; waits on unknown (no note left) | nothing known | commit the 128 seals, open the PR; this unblocks #12965, #12967, #12969, #12977, #12979 | 06:08Z, session list; 07:04Z worktree | none yet |
| E#? | #12968 (CLOSED, shape-refused) | re-bless the workflow census, 71 -> 70 | "Re-bless cli-tri workflow census to 70" (idle, started by regenerate-gates) | PR head was `claude/census-rebless-12968`; worktree `~/t27/.claude/worktrees/loving-kilby-0e85a7` sits on `claude/loving-kilby-0e85a7` `5ba817111f`, 1 commit ahead of master, no upstream | #12971 MERGED 2026-10-31 | 0 / 1 local commit on a side branch, not the PR head | done (merged into a dead issue) | none | decide whether `5ba817111f` is the same change (then the branch can be parked) | 06:20Z, session list | `gh pr view 12971` MERGED; issue #12968 stays CLOSED shape-refused |
| E#12945 | #12952 (CLOSED) | DSH best-practice adoption pipeline | "Lane #12952: DSH practice-adoption pipeline" (idle, started by the parent) | `claude/cool-fermi-286d59` / `~/t27/.claude/worktrees/wasm-doom-fpga-2ce0a0` | #12960 MERGED 2026-10-31 | 0 (1 untracked) / 1 commit not in master (squash) | done | none | worktree is a self-cleanup candidate for its own session only | 05:11Z, session list | `gh pr list --state merged --head claude/cool-fermi-286d59` -> 12960 |
| E#12173 (title match only; PR body not read) | #? | T27OS interactive visualizations MVP (the parent session) | "T27OS interactive visualizations MVP" (idle) | session record: `claude/t27os-interactive-viz-mvp-57fe52`; worktree `~/t27/.claude/worktrees/silly-thompson-a50ffa` is on `claude/viz-rollout-wave2` `36c8d6a54f` | #12193 MERGED | 0 (5 untracked) / 0 (`viz-rollout-wave2` has no PR but 0 commits outside master) | done; now orchestrating | none | none | 06:46Z, session list | `gh pr view 12193` MERGED |
| E#? | #12406 (CLOSED, shape-refused) | real-time fleet view at t27.ai/t27f/live/ | "Real-time fleet view at t27.ai/t27f/live/" (idle, started by the parent) | session record `claude/nervous-chaplygin-12302a`; worktree `~/t27/.claude/worktrees/t27-syntax-highlighting-db5100` is on `claude/t27f-black-backdrop-12406` `229ed2e8a5`, 5 ahead of master (squash) | #12383 MERGED; #12408 MERGED 2026-10-26 | 0 / 0 | done; the feed it reads (`/queen/public-activity`) answers 404 (#12978) | none | none in this lane; the 404 is #12978's | 2026-10-26, session list | `gh pr view 12408` MERGED |
| E#? | trinity#1733 (other repo) | GitHub issues and release (title translated from Russian) | "GitHub issues and release" (idle) | worktree `~/t27/.claude/worktrees/sad-kilby-2b6441` DETACHED at `05803d2acd` (master); PR is in gHashTag/trinity | trinity#1733 OPEN | 0 (5 untracked) / 0 | waiting on unknown: the trinity PR was not inspected | unknown | read trinity#1733 | 06:52Z, session list | none in this repo |
| E#? | #? | analog widget | "analog widget" (idle, started by the release session) | worktree not identified | #12025 MERGED | unknown | done | none | none | 06:52Z, session list | `gh pr view 12025` MERGED |
| E#12444 (skill `boilerplate-loop` names the epic) | #? | T27 boilerplate loop on the main clone | "T27 boilerplate loop" (flagged running, last activity 2026-10-25) | `pr-c-zig-like-plan` / `~/t27` (the main clone) | none | 101 modified tracked files / 16 commits ahead of master, no upstream | quiet 6 days; waits on unknown | nothing known | the session commits or parks its 16 commits as `rescue/`; the main clone should return to master | 2026-10-25, session list; 07:0xZ worktree | none |
| E#12945 | #12980 | this inventory (Phase A) | this session | `claude/quizzical-stonebraker-bfe908` / `~/t27/.claude/worktrees/quizzical-stonebraker-bfe908` | PR of this file | 1 file / 1 commit | running | none | PR, then Phase B | 07:04Z | commit `Closes #12980` |

### 1.2 Open PRs with no session behind them (27)

| Epic | Issue | PR | Branch | Head / state | Author | CI | State | Next step |
| --- | --- | --- | --- | --- | --- | --- | --- | --- |
| E#? | #9633 (CLOSED, not merged) | #9636 | `codex/ci-gen-w-9633` | BLOCKED | dmitrii-f-t27 | 6 failed, 21 passed | blocked: its `Closes` target is already closed | decide: close the PR or re-file the issue |
| E#? | #12034 (CLOSED, shape-refused), `Refs #11560` | #12035 | `t27os-line-upstream` | DIRTY | dmitrii-f-t27 | 7 failed, 26 passed | blocked: merge conflict, dead issue | decide: rebase or close |
| E#5980 | none (`Refs #5980`) | #12695 | `parser/single-element-bracket-literal` | DIRTY | gHashTag | 10 failed, 35 passed | blocked: merge conflict | rebase |
| E#? | none | #12811 | `hosting/railway-retired` | BLOCKED | gHashTag | 12 failed, 22 passed | red, no issue | needs an issue or a close |
| E#? | #12835 (CLOSED, shape-refused) | #12901 | `xilinx7-packet-constants-pinned` | UNSTABLE | gHashTag | 7 failed, 28 passed | red, dead issue | reopen #12835 with a passing body |
| E#? | 21 port issues, e.g. #12573 (trios merkle.rs) | #12917 .. #12937 | `port/*` | UNSTABLE | dmitrii-f-t27 | 7 failed each (Corpus ratchet, emit-bitexact, gate-topology, coverage, spec-guards, Verilog widths, cli-tri on #12917) | red; the 7 failures are master's own (#12959, #12972) | land master's fixes, then re-run; map the port issues to an epic |
| E#? | none | #12961 | `epic2-inline-decision-glue` | DIRTY | gHashTag | 22 failed, 2 cancelled, 49 passed | blocked: conflict, no issue | needs an issue or a close |

### 1.3 Worktrees (75; all measured)

Format of the measurement: branch, head, tracked dirty files, commits ahead of
origin/master, upstream, commits ahead of upstream. "ahead of master" with a
MERGED PR means squash: not lost.

Session worktrees under `~/t27/.claude/worktrees/` (30): see 1.1 for the 12
with a session. The rest:

| Worktree | Branch | Head | Dirty | Ahead master | PR | Reading |
| --- | --- | --- | --- | --- | --- | --- |
| boiler-f-12450 | `claude/boiler-f-12450` | `1b1a27d4d8` | 0 | 0 | none | "claw: add the t27os workflow row to the gate inventory, 68 rows" 2026-10-25; content is in master; same subject as #12967 |
| epic-posts-rule-12629 | `lane/epic-posts-rule-12629` | `021291cfb9` | 0 | 2 | #12634 MERGED | done |
| gen-c-arr | `fix/gen-c-arr-fresh` | `f1fb27891d` | 2 | 0 | none | "the seal rides with its spec, the autofix law" 2026-10-26; 2 uncommitted files, ownerless |
| keys-merge | `feat/keys-merge` | `adad7b6197` | 27 | 0 | #12755 CLOSED | superseded keys lane, 27 dirty files, ownerless |
| keys-pr | `feat/keys-tab` | `cbd55e9483` | 29 | 0 | none | superseded keys lane, 29 dirty files, ownerless |
| lsp-L01 .. lsp-L05 | `claude/lsp-L01` .. `L05` | various | 0 | 1,1,1,2,0 | none | five LSP lanes, no PR, no upstream, ownerless (plus `claude/lsp-epic`, no PR) |
| mystifying-bhabha-d40f6d | `claude/mystifying-bhabha-d40f6d` | `f1fb88cc6d` | 0 | 0 | none | "re-ledger t27b_T27bWasm" 2026-10-24; content in master |
| pr-fix | DETACHED | `3c082cc752` | 0 | 6 | none | 6 commits on no branch: at risk, ownerless |
| reds-fix | `issue12705-patha` | `1e23fc3dc9` | 1 | 0 | none | #12705 CLOSED shape-refused; 1 dirty file |
| silly-thompson-a50ffa/target/lb | DETACHED | `dfae0aa3a6` | 0 | 0 | none | nested worktree inside a build dir |
| t27os-live-12779 | `lane/queen-band-graph` | `0132bc95c5` | 95 | 17 | none | 17 commits + 95 dirty files, no PR; same area merged via #12791/#12808; ownerless |
| wasm-b2-pnr-12456 | `lane/wasm-b2-pnr-12456` | `f4106fe950` | 0 | 7 | #12624 MERGED | done |

Lane folders outside the app (`~/t27-lanes/*`, 12; `~/t27-wt-*`, 8;
`~/t27-build`): 17 of 21 sit on a branch whose PR is MERGED (chore/kv-reseal
#12386, night/189 #12402, night/194 #12414, night/188 #12397, night/209 #12472,
feat/preflight-push-gate #12398, lane/wasm-fb-12203 #12270, lane/wasm-loader-12201
#12278, lane/t27os-release-verify #12671, lane/queen-api-dispatch #12662,
lane/t27os-feed #12393, lane/seals-r6 #12812, lane/t27f-replay-today #12715,
lane/wasm-carrier #12308, feat/wasm32-floor-locals #12378 CLOSED,
lane/reseal-kv-storage #12433 CLOSED, fix/gate-topology #12392 CLOSED).
Exceptions: `~/t27-lanes/repair-actor` (`repair-actor`, 6 dirty, 0 ahead, no PR,
same subject as merged #12398), `~/t27-lanes/seal-sweep` (`t27os-seal-sweep-2`,
4 ahead, no PR), `~/t27-lanes/night-209` (1 unpushed commit over a merged PR),
`~/t27-wt-news` (1 dirty file), `~/t27-build` (master, clean),
`~/t27-lanes/census-tree` (DETACHED `89a747eefc`, clean).

Agent litter (not t27 lanes; listed, not touched): `/private/tmp/floor13-master-check`,
`/private/tmp/t27-pareto-cycle`, about 16 `/private/var/.../opencode/...` trees
(DETACHED; two of them `tmpvar` at `0132bc95c5` with 12 dirty files = the same
tip as `lane/queen-band-graph`), `~/.codex/worktrees/0c4d/t27` (DETACHED
`305c336818`, 10 dirty), `~/.copilot/session-state/.../pr-11420-worktree`
(`fix/11420-ci-20261021`, 19 dirty, 3 ahead, no PR),
`~/.local/share/opencode/worktree/751937/prfix` (`pr-l1-fix`, 3 ahead, no PR),
`~/Documents/Codex/2026-09-01/new-chat-2/work/t27-wars-baseline-4328`
(`codex/wars-baseline-4328`, no PR, no upstream; `git status` answered after about
an hour: 7 dirty paths with untracked counted, 33 commits ahead of master, last
commit 2026-05-24).

## 2. Lists

### 2.1 Active (running now)

- #12976 dev-monitor skill: PR #12979, 12 checks pending (session running).
- #12970 result record spec: 1 commit ahead, not pushed, no PR (session running).
- AI-HW-SW co-design mail: clean tree, no issue, no PR (session running).
- #12975 OpenCode free pool: draft PR #12977 red; issue closed shape-refused (DSH session running).
- T27 boilerplate loop on `~/t27`: flagged running, last activity 2026-10-25, 101 dirty files, 16 unpushed commits.

### 2.2 Waiting on CI (checks still pending)

- #12979 (12 pending, 2 failed).
- #12977 (2 pending, 7 failed).

### 2.3 Red CI (every check finished, some failed; a fix, not a wait)

- #12965 (9), #12967 (7), #12969 draft (6), #12901 (7), the 21 port PRs #12917..#12937 (7 each), #9636 (6), #12811 (12).
- The common failures are master's own: `coverage` (stale seals, #12972), `Corpus ratchet` (#12959), `gate-topology` (#12963), `emit-bitexact` (#12964 / PR #12965). Until those land, no PR in this list can go green on its own.

### 2.4 Blocked and on what

- #12969 waits on #12965 (same cell) and on master seals (#12972).
- #12965, #12967, #12977, #12979 wait on master seals (#12972) for their inherited reds.
- #12972 itself is quiet: 128 re-saved seals sit uncommitted in `ecstatic-mclean-c5cf93`; nothing is pushed.
- DIRTY (merge conflict): #12035, #12695, #12961.
- Dead base: #9636 closes #9633 which is already CLOSED; #12035, #12901, #12965, #12977 close or reference issues that are CLOSED shape-refused.

### 2.5 Needs an owner decision

1. #12941, the owner's own example epic, is CLOSED shape-refused. Reopen with the `epic` label (the gate skips epics) or re-file.
2. L1 receipts to dead issues: #12971 merged with `Closes #12968` (shape-refused), #12965 says `Closes #12964` (shape-refused), #12977 says `Refs #12975` (shape-refused), #12901 says `Refs #12835` (shape-refused), #9636 says `Closes #9633` (closed, not merged). Rule needed: a PR may not reference a closed issue as its receipt; fix the bodies and let the gate reopen them.
3. The main clone `~/t27` is on `pr-c-zig-like-plan` with 101 dirty files and 16 unpushed commits while a session is flagged running on it. Decide: that session commits and parks, or the owner does.
4. 21 `port/*` PRs by `dmitrii-f-t27` (a second account) with no epic: keep and map to an epic, or close.
5. `fed/` prefix: 1904 remote branches, meaning unknown. Keep, or sweep by rule.
6. Stale bound: 360 min (`queen/claim_msg`) vs 120 min (`queen/worktree_keep`, `t27os/night_take`, `queen/dispatch`). One constant.
7. Three giant orphan branches with no PR: `fix/hello-world-count` (1204 commits over a 2026-04-04 base), `402-failover-fix` (714), `acceleration-v9-final` (535). Rescue refs or delete; nothing here decides.
8. Five session records name a branch other than the one their worktree is on (parent, regenerate-gates, lane-12952, real-time fleet, DSH). Which is the owner's truth: the app record or the worktree.

### 2.6 Ownerless

Open issues with no session, branch or PR: #12973 (owner inbox), #12974
(archive), #12951 (Matryoshka lane), #12962 (token-efficiency baseline), #12959
(corpus ratchet red), #12963's sibling drift issues are covered by #12967.
Open PRs with no session: the 27 in 1.2. A running session with no issue: AI-HW-SW
co-design mail. Branches with commits and no PR, no session: `pr-fix` (6,
detached), `claude/lsp-L01..L05` + `claude/lsp-epic`, `lane/queen-band-graph`
(17), `t27os-seal-sweep-2` (4), `d96` (3 over a merged base), `fixs20-queen-8265`
(2 over a merged base), `rename-10175` (6), `rescue/pr11801-master-merge` (6),
`fix/11420-ci-20261021` (3), `pr-l1-fix` (3), `codex/wars-baseline-4328`, and the
1241 unmerged local-only branches at large (995 of them 1 to 3 commits).

### 2.7 Possible duplicates and superseded work (with evidence)

1. Keys lane: `claude/t27os-keys-a7d243` (#10959 CLOSED), `-v2` (#10972 CLOSED), `-v3` (#10977 CLOSED), `-v4` (#11228 MERGED; `rb-t27os-keys-v4` same sha `a44d116573`); `feat/keys-12753-x9` (no PR) vs `-x7` (#12774 MERGED) and `-final-x3` (#12781 MERGED); worktrees `keys-pr` (29 dirty) and `keys-merge` (27 dirty, #12755 CLOSED). Superseded: the merged ones carry the work.
2. Seal refresh: `ecstatic-mclean-c5cf93` (128 uncommitted seals, #12972) vs `seals/corpus-refresh` (#12626 CLOSED DIRTY), `seals/corpus-refresh-v2` (#12698 CLOSED), `seals/queue-drain-refresh` (#12623 CLOSED), `t27os-seal-sweep-2` (4 ahead, no PR). Fourth or fifth attempt at the same fix; none landed.
3. Gate inventory: #12967 (`claude/fix-claw-gate-inventory-12963`) vs `claude/boiler-f-12450` ("add the t27os workflow row to the gate inventory, 68 rows", no PR, content in master) vs `fix/gate-topology` (#12392 CLOSED, worktree `~/t27-wt-news` 1 dirty).
4. Queen band graph: `lane/queen-band-graph` (17 ahead, 95 dirty, no PR; two opencode trees at the same tip) vs `lane/queen-graph-plane` (#12808 MERGED) vs `lane/queen-live-mycelium-12780` (#12791 MERGED). Same screen; the merged pair likely supersedes the 17.
5. Preflight gate: `repair-actor` (no PR, "the nine local gates as one command") vs `feat/preflight-push-gate` (#12398 MERGED).
6. Auto-reporter issues "The swarm is not answering": #12902, #12938, #12939, #12978, all from `/queen/status` 404. One cause, four issues.
7. #12971 (MERGED, census 71 -> 70) and the `loving-kilby-0e85a7` commit `5ba817111f` (1 ahead, no PR): same change twice, or a follow-up; unknown.
8. `rescue/pr11801-master-merge` (6 commits over `feat/lab-t27os-host-11800`, whose PR #11801 MERGED): rescue of a merge, or leftover.
9. `d96` (3 over `perf/pr-rebase-bot-v2`, #12696 MERGED; "crons: split, the card and its exceptions entry land now") and `fixs20-queen-8265` (2 over `queen-8265`, #8442 MERGED 2026-10-12): follow-ups that never got a PR.

### 2.8 Unpushed local work

Branches ahead of their own non-master upstream (12 branches, 42 commits):
`claude/lane-12966-ax7203-placement-2737dc` 1, `d96` 3, `fixs20-queen-8265` 2,
`lane/queen-graph-plane` 9, `lane/queen-live-mycelium-12780` 1,
`night/209-lab-sched-specs` 1, `opcode-pins` 2, `rel-10262-hud-a11y` 2,
`rel-face-ledger` 1, `rename-10175` 6, `rescue/pr11801-master-merge` 6,
`site-givel-map` 8. Of these, `lane/queen-graph-plane`, `lane/queen-live-mycelium-12780`,
`opcode-pins`, `rel-10262-hud-a11y`, `site-givel-map` have MERGED PRs (#12808,
#12791, #12852, #11782, #12805): the extra commits are post-merge or merge
commits, probably safe; the others have no PR.

Uncommitted tracked changes (14 worktrees): `~/t27` 101, `ecstatic-mclean-c5cf93`
128, `t27os-live-12779` 95, `keys-pr` 29, `keys-merge` 27, `.copilot/...
pr-11420-worktree` 19, two opencode `tmpvar` trees 12 each, `.codex/.../t27` 10,
`~/t27-lanes/repair-actor` 6, `gen-c-arr` 2, `reds-fix` 1, `~/t27-wt-news` 1.
Also `result-record-spec-608e45` 1 commit not on origin, `loving-kilby-0e85a7`
1 commit on no upstream, `pr-fix` 6 commits detached.

### 2.9 Unresolved epic mappings (`E#?`)

- Issues: #12972, #12963, #12968, #12964 (PR says `Refs #12387`; the issue is dead), #12975 (chain #12975 -> #12974 -> #12945 unconfirmed by label), #12959 (`Refs #12953`, closed; epic #12387 by the body of #12959, to confirm), the 21 port issues (#12573 and siblings: no labels, no epic refs), #9633, #12034 (`Refs #11560`, not checked), #12835.
- PRs with no issue: #12811, #12961.
- Sessions: AI-HW-SW co-design mail, analog widget (epic #10486 is CLOSED), real-time fleet (epic #10266 plausible, unconfirmed), "GitHub issues and release" (trinity#1733), the parent (#12193's issue not read).
- Branches: every ownerless branch in 2.6.

### 2.10 Sources that stayed unknown, and why

- `tri status`, `tri pr-state`, `tri golden status`: no built binary in the worktree; building is forbidden on this Mac.
- Queen `/queen/status` and `/queen/public-activity`: 404 (#12978, #12939); `specs/t27f/live_fleet.t27` reads the latter.
- `fed/` remote prefix (1904 branches): sampled one, no documentation found.
- Cloud sessions: not listed by the app's local session list.
- DSH state and tokens: `~/.dsh/` was never read (owner rule); DSH availability is "unknown" here, the dev-monitor lane (#12976) is the probe.
- `~/Documents/Codex/.../t27-wars-baseline-4328`: `git status` answered after about an hour (first pass, untracked counted); the tracked-only second pass does not include it.
- Branch-to-PR check covered 200 of 1725 local-only branches; 1525 unchecked.
- Per-PR failing check names were read for one port PR (#12917) and the five session PRs' counts; not for all 32.
- Sessions outside this family: seen once (06:52Z) through the full list with title, branch, PR, state only.
- gHashTag/trinity, trinity-fpga, igla, t27os, trios: not inspected.
- Which checks are required on master: `unknown` at 07:04Z; re-read at 07:40Z from the
  active ruleset `t27-master-protection`: validate, check-linked-issue, parse-ratchet,
  own-language (4). The other check names on a PR are not required.

## 3. Proposed chat titles (rename_pending; nothing renamed)

Format: `<emoji> E#<epic> . #<issue> . <short outcome>`. This file is ASCII, so
each emoji is written as its name in colons and the middle dot as a period; the
ASCII form is the only committed form; the Unicode title is rendered from it (emoji
from its name, period to a middle dot). Session ids and model names are left out on
purpose.

| Session (by current title) | Proposed title | Flag |
| --- | --- | --- |
| dev-monitor skill: open the authorized DSH on dev entry | :telescope: E#12945 . #12976 . dev-monitor opens the authorized DSH once per dev entry; PR 12979 in CI | rename_pending |
| Fix duplicate OP_I64_ADD that reds emit-bitexact | :jigsaw: E#12387 . #12964 . duplicate OP_I64_ADD dropped; PR 12965 red on 9 checks, issue shape-refused | rename_pending |
| T27OS interactive visualizations MVP | :control_knobs: E#12173 . PR 12193 . viz MVP merged; orchestrating work control (Phase A #12980) | rename_pending; issue number not read |
| Lane #12966: fpga-synth-ax7203 placement budget | :compass: E#12387 . #12966 . AX7203 heap placer PnR; draft PR 12969 waits on 12965 | rename_pending |
| Re-bless cli-tri workflow census to 70 | :white_check_mark: E#? . #12968 . workflow census 71 -> 70 merged (PR 12971); issue shape-refused | rename_pending |
| Re-save stale seals so coverage goes green | :pushpin: E#? . #12972 . 128 stale seals re-saved locally, uncommitted, no PR | rename_pending |
| Regenerate specs/claw/gates.t27 (gate-topology drift) | :door: E#? . #12963 . gates.t27 inventory regenerated; PR 12967 red on 7 checks | rename_pending |
| Lane #12952: DSH practice-adoption pipeline | :books: E#12945 . #12952 . DSH practice-adoption pipeline merged (PR 12960) | rename_pending |
| Real-time fleet view at t27.ai/t27f/live/ | :satellite: E#? . #12406 . live fleet view merged (PR 12383, 12408); feed answers 404 | rename_pending |
| AI-HW-SW-CoDesign | :envelope: E#? . #? . AI-HW-SW co-design mail; no issue, no PR, clean tree | rename_pending; needs an issue |
| result-record-spec | :triangular_ruler: E#12945 . #12970 . 14-field result record spec committed, not pushed | rename_pending |
| DSH | :bucket: E#? . #12975 . OpenCode free pool as second executor; draft PR 12977 red, issue shape-refused | rename_pending |
| GitHub issues and release (translated) | :rocket: E#? . trinity#1733 . release PR open in gHashTag/trinity; t27 tree detached and clean | rename_pending |
| analog widget | :microscope: E#? . #? . analog widget merged (PR 12025) | rename_pending; issue not read |
| T27 boilerplate loop | :spaghetti: E#12444 . #? . boilerplate loop on the main clone; 101 dirty files, 16 unpushed commits, quiet since 2026-10-25 | rename_pending |
| this session | :card_index_dividers: E#12945 . #12980 . Phase A work-control inventory filed | rename_pending |

## 4. Plan for Phases B-E and archive

Every phase extends a spec that already exists. No second ledger, no second
router, no new core. Each line names the spec, the first spec change and the
test that proves it.

### Phase B: owner claim

- Extends `specs/queen/claim_msg.t27` (#12582: `refs/swarm/claims/<issue>`
  old-value-zero push, issue comment, heartbeat; CM_FREE / CM_HELD / CM_STALE)
  and the `claimed` flag of `specs/queen/tasks.t27`.
- Record: `issue -> owner_session -> scope -> claimed_at -> last_confirmed_at`.
  One owning lane per issue is the existing compare-and-swap on the claim ref;
  the new fields ride in the ref's blob and the issue comment.
- First spec change: a `ClaimCard` record with those five fields, an invariant
  `one_owner_per_issue`, and a test where the second claim on one issue returns
  CM_HELD and leaves `owner_session` unchanged. `last_confirmed_at` replaces
  `cm_idle_minutes` as the stale input; CM_STALE_MINUTES and
  `worktree_keep` CLAIM_STALE_S become one constant (decision 2.5.6).
- Scope is the Boundary of the issue (`specs/policy/issue_shape.t27` already
  reserves those paths), so no new field is invented for it.

### Phase C: dev-workspace UI

- Extends `specs/t27os/lanes_view.t27` (#11397, the one row model: LVF_SRC,
  LVF_ISSUE, LVF_MODEL, LVF_LIGHT, LVF_STATE, LVF_ELAPSED, LVF_TLEN, LVF_DEPTH;
  feed `~/.t27os/lanes/lanes.txt`) and the T27F surface list in
  `specs/t27f/frontend.t27`; the dashboard verdicts stay in
  `specs/queen/dashboard.t27`.
- Top bar `T27 DEV`: `DSH: available | authentication required | unavailable`
  from the probe states of `specs/t27os/dev_monitor.t27` (#12976, in flight);
  `Work: running . waiting on CI . blocked . needs decision` as counts over
  LVS_* states plus the ClaimCard; `Data as of <time>` from the feed's mtime,
  LV_NOT_MEASURED when the feed is absent (never green).
- First spec change: two columns LVF_EPIC and LVF_OWNER on the lane row, a
  `DevBar` record with the three DSH states and four counts, and a test that
  renders all three DSH states and shows LV_NOT_MEASURED when the feed is
  missing. Filters by epic, owner and state are predicates over the row.

### Phase D: separate statuses

- Extends `specs/queen/claim.t27` (CL_RUNNING .. CL_CANCELLED = execution;
  V_* 11 codes = review), `specs/queen/golden_chain.t27` (GC_SPEC .. GC_DONE =
  merge), `specs/queen/views.t27` (DONE_FROM_ACCEPTED = acceptance) and
  `specs/verified/payout.t27` (payment).
- Record `WorkStatus { execution_status, review_status, merge_status,
  acceptance_status, payment_status }`, each field typed by its existing enum;
  `done` is derived only from acceptance (views.t27 already says so).
- First spec change: the views.t27 row carries the five-tuple, `epic_complete`
  derives from acceptance alone, and a test shows a merged PR with no acceptance
  is not done.

### Phase E: notifications

- Extends `specs/queen/events.t27` (cursor bus, KEEP_SECONDS) and the event
  kinds of `.trinity/events/akashic-log.jsonl` (`task.started`,
  `task.blocked`, `task.completed`, `handoff.request` ...), fenced by the
  epochs of `specs/queen/control.t27`; `NOTIFY_IS_A_HINT` from
  `specs/queen/waits.t27` stays the law.
- Emit only on a WorkStatus field transition; nothing on heartbeats.
- First spec change: EV_STATUS_CHANGED with `(issue, field, from, to, at)` and a
  test that an unchanged status emits nothing and a changed one emits exactly
  one event. The owner inbox (#12973) is a reader of this event, not a new bus.

### Archive

- Extends `specs/queen/worktree_keep.t27` (WT_KEEP default, `archive_verified`,
  ST_UNKNOWN) and the task states of `specs/queen/tasks.t27` (#12974), with the
  branch sweep rule of `crons/t27-branch-sweep.t27` (record orphans before any
  deletion) and the `rescue/<name>` ref convention.
- Archive only after a verified terminal state: PR merged by REST `merged_at`
  (the self-cleanup rule), or issue CLOSED with a `Closes` receipt in a merged
  commit, or the owner's explicit word. A quiet agent (BEE_QUIET_SECONDS) is
  "quiet, waits on X", never archived for silence. History is never deleted:
  a removed worktree leaves its branch, a removed branch leaves a `rescue/` ref
  or a sweep record.
- First spec change: a test `quiet_is_not_abandoned` in worktree_keep.t27 (a
  tree idle past CLAIM_STALE_S with an open PR stays WT_KEEP) and a test that
  `archive_verified` is false while `merged_at` is unknown.

### Order and dependencies

B first (every later phase keys on owner and claimed_at), then D (statuses),
then C (the bar reads B and D), then E (events on D's transitions), then the
archive (reads D's terminal states). The 2.5 decisions that gate the start:
one stale constant (6) and the dead-issue receipt rule (2).
